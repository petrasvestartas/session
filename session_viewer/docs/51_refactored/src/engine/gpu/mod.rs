//! `Gpu` - the lowest layer of the viewer (ARCHITECTURE.md §1): the floor (surface, `GpuCtx`,
//! layouts, pipelines, frame uniforms, targets, view), the four row families, the two point
//! lanes and the splatter over them - one file each. This file only builds the struct, appends
//! an upload, and keeps the splat groups current; the frame list is `render.rs`.

pub mod arena;
pub mod backdrop;
pub mod buffers;
pub mod cloud;
pub mod device;
pub mod frame;
pub mod glyphs;
pub mod instance;
pub mod objects;
pub mod present;
pub mod render;
pub mod segments;
pub mod splat;
pub mod stream;
pub mod targets;
pub mod upload;
pub mod view;

use crate::engine::performance::Performance;
use crate::engine::pipelines::{Layouts, Pipelines, Target};
use crate::math::Aabb;
use session_rust::Point;

use buffers::GpuCtx;
use cloud::CloudLane;
use device::DeviceSetup;
use frame::FrameUniforms;
use glyphs::GlyphLane;
use segments::SegmentLane;
use splat::{PixelBufs, Splat, SplatCx};
use stream::StreamLane;
use targets::Targets;

pub use arena::Arena;
pub use cloud::{CloudDraw, LodNode};
pub use frame::FrameInput;
pub use glyphs::GlyphPoint;
pub use instance::Instance;
pub use objects::{InstanceTable, Rebase};
pub use segments::{CylinderSegment, LineStyle};
pub use upload::Upload;
pub use view::View;

/// Everything on the GPU side of the viewer, 17 fields: the floor, the families, the lanes.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // Screen to draw pixels on; None when headless.
    pub ctx: GpuCtx,                         // Device (makes resources) + queue (submits work).
    pub config: wgpu::SurfaceConfiguration,  // Settings for Surface: size, pixel format
    /// Layouts survive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    pub layouts: Layouts,
    pub pipelines: Pipelines,
    pub frame: FrameUniforms,                // mvp / line / cloud uniforms + this frame's eye and ortho
    pub targets: Targets, // depth + MSAA colour at the sample count this scene chose (see `msaa_now`)
    /// The runtime knobs: what to show, ink style, cloud/EDL/LOD scalars, pen weight.
    pub view: View,
    /// The object rows: instances, their f64 translations, the bounded rows, the re-anchor, the inside test.
    pub objects: InstanceTable,
    /// The mesh arena: one vertex table, three index runs (faces, sheet fills, lettering).
    pub arena: Arena,
    /// The segment family: pipes (solid lane) and ribbons (flat lane) over one row layout.
    pub segments: SegmentLane,
    /// The glyph family: spheres (solid lane markers) and dots (flat lane) over one row layout.
    pub glyphs: GlyphLane,
    /// The walked cloud lane: three point tables, one draw per cloud, the octree nodes.
    pub cloud: CloudLane,
    /// The stream lane: clouds written slice by slice, never held on the CPU.
    pub stream: StreamLane,
    /// The compute splatter over both lanes: pixel buffers, record slots, the static-skip key.
    pub splat: Splat,
    pub performance: Performance,
    pub bounds: Aabb,
}

impl Gpu {
    /// Set up the five wgpu objects, in order: Instance → Surface → Adapter → Device + Queue → configure.
    /// The scene starts empty - every upload, including the first file, goes through `set_scene`
    /// (progressive loading calls it once per appended file). One code path, not two.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), size.width.max(1), size.height.max(1)).await
    }

    /// Same stack with no window and no surface, rendering into an offscreen texture. Exists so
    /// a shader can be checked against a PNG on this machine instead of against the user's eyes.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, width.max(1), height.max(1)).await
    }

    /// The shared constructor: negotiate the device, make every layout, buffer, bind group and
    /// pipeline, and start with an empty scene.
    async fn build(
        window: Option<std::sync::Arc<winit::window::Window>>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, (width, height)).await?;
        let ctx = GpuCtx { device, queue };

        // Depth and MSAA - the empty scene starts flat (1x); set_scene flips to 4x when the
        // first solid geometry arrives (`Targets::samples_for`).
        let samples = 1;
        let targets = Targets::new(&ctx, &config, samples);

        // Every bind-group layout, once; pipelines and bind groups are made from these.
        let layouts = Layouts::new(&ctx.device);
        let frame = FrameUniforms::new(&ctx, &layouts, (config.width, config.height));

        // The four row families start as one zeroed row each: wgpu cannot bind a 0-byte
        // buffer, and every length is 0, so the first frame draws nothing. The loader calls
        // set_scene the moment the first file's tables exist.
        let objects = InstanceTable::new(&ctx, &layouts);
        let arena = Arena::new(&ctx);
        let segments = SegmentLane::new(&ctx, &layouts);
        let glyphs = GlyphLane::new(&ctx, &layouts);

        // The walked cloud lane - empty until set_scene fills it from the upload.
        let cloud = CloudLane::new(&ctx);

        // The stream lane: its own buffers, grown for real by `StreamLane::begin`.
        let stream = StreamLane::new(&ctx);

        // The compute splatter over both lanes: framebuffer-sized per-pixel buffers and one
        // record slot per lane, bound over the lanes' placeholder buffers for now.
        let splat_cx = SplatCx { ctx: &ctx, layouts: &layouts, frame: &frame };
        let splat = Splat::new(&splat_cx, (config.width, config.height), cloud.buffers(), stream.buffers());

        // Pipelines - render and compute, one set per sample count.
        let pipelines = Pipelines::new(&ctx.device, Target { format: config.format, samples }, &layouts);

        // Output
        log::info!("viewer init OK — surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self {
            surface,
            ctx,
            config,
            layouts,
            pipelines,
            frame,
            targets,
            view: View::from_env(),
            objects,
            arena,
            segments,
            glyphs,
            cloud,
            stream,
            splat,
            performance: Performance::new(),
            bounds: Aabb { min: [0.0; 3], max: [0.0; 3] },
        })
    }

    /// Append one upload to every family - called once per file while progressive loading
    /// appends. Every table but `obj` is a DELTA: only this file's rows travel, and a bind group
    /// is rebuilt only when the buffer behind it grew. An MSAA flip (first solid file after
    /// flat-only ones) also rebuilds the targets and every pipeline: sample count belongs to the PASS.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);

        if self.cloud.append(&self.ctx, &up.cloud) {
            self.rebind_splat();
        }
        self.splat.invalidate();

        if up.bounds.is_finite() { // an empty upload (the State boots before any file) knows no box
            self.bounds = up.bounds;
        }

        log::info!(
            "scene: {} objects {} arena verts {} segments ({} pipes) {} glyphs ({} spheres) {} cloud points",
            self.objects.len(), self.arena.vert_count(), self.segments.pipe_count() + self.segments.ribbon_count(), self.segments.pipe_count(),
            self.glyphs.sphere_count() + self.glyphs.dot_count(), self.glyphs.sphere_count(), self.cloud.point_count
        );

        self.retarget(false);
    }

    /// Bring the targets to the sample count the scene and canvas call for now: on a change
    /// every pipeline follows (the count belongs to the PASS); `resized` remakes the targets
    /// even at the same count, since they are sized to the surface.
    fn retarget(&mut self, resized: bool) {
        let samples = self.msaa_now();
        let flip = samples != self.targets.samples;
        if flip || resized {
            self.targets = Targets::new(&self.ctx, &self.config, samples);
        }
        if flip {
            self.pipelines = Pipelines::new(&self.ctx.device, Target { format: self.config.format, samples }, &self.layouts);
            log::info!("msaa: {}x", samples);
        }
    }

    /// The anchor the instance table is rebased about - see `InstanceTable::rebase_anchor`.
    /// A rebase moves every instance model, so the splats are stale. `now` is the frame's one
    /// timestamp (ms), read once by the caller.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self.objects.rebase_anchor(&self.ctx, origin, view_dist, now);
        if rebase.moved {
            self.splat.invalidate();
        }
        rebase
    }

    /// Re-point the splat groups at the current buffers - a lane grew or the canvas resized.
    fn rebind_splat(&mut self) {
        let cx = SplatCx { ctx: &self.ctx, layouts: &self.layouts, frame: &self.frame };
        self.splat.rebind(&cx, self.cloud.buffers(), self.stream.buffers());
    }

    /// Grow the scene box by a streamed cloud's world-space AABB, so the camera can fit it.
    pub fn grow_scene(&mut self, world: &Aabb) {
        if !world.is_finite() { return }
        // an empty scene starts with a zero box; the first cloud replaces it
        if self.bounds.min[0] >= self.bounds.max[0] {
            self.bounds = *world;
            return;
        }
        self.bounds.union(world);
    }

    /// Reconfigure the surface and recreate the depth + MSAA targets for a new canvas size.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            if let Some(s) = &self.surface { s.configure(&self.ctx.device, &self.config); }
            self.retarget(true);
            self.splat.pixels = PixelBufs::new(&self.ctx, (width, height));
            self.rebind_splat();
            self.splat.invalidate();
        }
    }

    /// Forget every family's rows, so the next upload writes from row 0 again. Every lane
    /// appends, so a rebuild has to rewind every lane - leaving one set would append the
    /// re-walked scene BEHIND the copy already there. Capacity stays: a rebuild costs no allocation.
    pub fn reset_arena(&mut self) {
        self.objects.reset();
        self.arena.reset();
        self.segments.reset();
        self.glyphs.reset();
        self.cloud.reset();
    }

    /// Forget every family's rows AND hand their memory back, CPU mirrors and GPU buffers alike:
    /// one-row placeholders again, as `build` made them. `reset_arena` keeps capacity for a
    /// rebuild; a cleared scene has nothing to rebuild and must not stay resident.
    pub fn release(&mut self) {
        self.objects.release(&self.ctx, &self.layouts);
        self.arena.release(&self.ctx);
        self.segments.release(&self.ctx, &self.layouts);
        self.glyphs.release(&self.ctx, &self.layouts);
        self.cloud.release(&self.ctx);
        self.rebind_splat();
        self.splat.invalidate();
    }

    /// MSAA sample count for the scene NOW. It cannot be chosen per lane: the count belongs to
    /// the render PASS, so it is picked per scene from what is ON THE GPU (an upload is a delta;
    /// reading it thrashed 4x -> 1x -> 4x on every cloud append). Solid = the faces run, pipes or
    /// spheres - the vertex count would make a pure sheet (fills only) pay for 4x it cannot use.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.face_count() > 0 || self.segments.pipe_count() > 0 || self.glyphs.sphere_count() > 0;
        Targets::samples_for(solid, self.config.width * self.config.height, self.view.msaa_override)
    }
}
