//! `Gpu` - the lowest layer of the viewer: the floor (surface, device, layouts, frame
//! uniforms, targets, view knobs, the object table) and the lanes, one file each. This file
//! builds the struct, appends an upload and keeps the lanes' targets current; the frame list
//! is `render.rs`, presenting is `present.rs`.

pub mod arena;
pub mod backdrop;
pub mod buffers;
pub mod device;
pub mod frame;
pub mod glyphs;
pub mod instance;
pub mod objects;
pub mod present;
pub mod render;
pub mod segments;
pub mod targets;
pub mod upload;
pub mod view;

use crate::engine::pipelines::{Layouts, Target};
use crate::math::Aabb;
use session_rust::Point;

use arena::ArenaLane;
use backdrop::BackdropLane;
use buffers::GpuCtx;
use device::DeviceSetup;
use frame::FrameUniforms;
use glyphs::GlyphLane;
use objects::InstanceTable;
use segments::SegmentLane;
use targets::Targets;

pub use frame::FrameInput;
pub use glyphs::GlyphPoint;
pub use instance::Instance;
pub use objects::{ObjectRow, Rebase};
pub use segments::CylinderSegment;
pub use upload::Upload;
pub use view::View;

/// Everything on the GPU side of the viewer: the floor, then one field per lane.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>,
    pub ctx: GpuCtx,
    pub config: wgpu::SurfaceConfiguration,
    pub layouts: Layouts,
    pub frame: FrameUniforms,
    pub targets: Targets,
    pub view: View,
    pub objects: InstanceTable,
    pub backdrop: BackdropLane,
    pub arena: ArenaLane,
    pub segments: SegmentLane,
    pub glyphs: GlyphLane,
    /// The world box of everything uploaded; the camera fits it and the inside test reads it.
    pub bounds: Aabb,
}

impl Gpu {
    /// The stack over a canvas window.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), (size.width, size.height)).await
    }

    /// Negotiate the device, make every layout, buffer, bind group and pipeline, start empty.
    async fn build(window: Option<std::sync::Arc<winit::window::Window>>, size: (u32, u32)) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, size).await?;
        let ctx = GpuCtx { device, queue };
        let size = (config.width, config.height);
        let target = Target { format: config.format, samples: 1 };

        let layouts = Layouts::new(&ctx.device);
        let frame = FrameUniforms::new(&ctx, &layouts, size);
        let targets = Targets::new(&ctx, size, config.format, target.samples);
        let objects = InstanceTable::new(&ctx, &layouts);
        let backdrop = BackdropLane::new(&ctx, &layouts, target);
        let arena = ArenaLane::new(&ctx, &layouts, target);
        let segments = SegmentLane::new(&ctx, &layouts, target);
        let glyphs = GlyphLane::new(&ctx, &layouts, target);

        log::info!("viewer init OK - surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self {
            surface,
            ctx,
            config,
            layouts,
            frame,
            targets,
            view: View::from_env(),
            objects,
            backdrop,
            arena,
            segments,
            glyphs,
            bounds: Aabb::empty(),
        })
    }

    /// Append one upload to every lane. Every table is a DELTA; a bind group is rebuilt only
    /// when its buffer grew. An MSAA flip rebuilds the targets and every pipeline.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);
        self.bounds.union(&up.bounds);

        log::info!(
            "scene: {} objects, {} verts, {} pipes, {} ribbons, {} markers, {} dots",
            self.objects.len(), self.arena.vert_count(), self.segments.pipe_count(), self.segments.ribbon_count(),
            self.glyphs.sphere_count(), self.glyphs.dot_count()
        );
        self.retarget(false);
    }

    /// The pass target the lanes are built for now.
    fn target(&self) -> Target {
        Target { format: self.config.format, samples: self.targets.samples }
    }

    /// Bring the targets to the sample count the scene and canvas call for: on a change every
    /// lane's pipelines follow; `resized` remakes the targets even at the same count.
    fn retarget(&mut self, resized: bool) {
        let samples = self.msaa_now();
        let flip = samples != self.targets.samples;
        if flip || resized {
            self.targets = Targets::new(&self.ctx, (self.config.width, self.config.height), self.config.format, samples);
        }
        if flip {
            let target = self.target();
            self.backdrop.retarget(&self.ctx, &self.layouts, target);
            self.arena.retarget(&self.ctx, &self.layouts, target);
            self.segments.retarget(&self.ctx, &self.layouts, target);
            self.glyphs.retarget(&self.ctx, &self.layouts, target);
            log::info!("msaa: {}x", samples);
        }
    }

    /// The sample count for what is ON the GPU now: 4x only with solid geometry (faces,
    /// pipes, spheres) and a canvas MSAA can afford; a pure sheet stays at 1x.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.face_count() > 0 || self.segments.pipe_count() > 0 || self.glyphs.sphere_count() > 0;
        Targets::samples_for(solid, self.config.width * self.config.height, self.view.msaa_forced)
    }

    /// The anchor the instance table is rebased about. `now` is the frame's one timestamp (ms).
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self.objects.rebase_anchor(&self.ctx, origin, view_dist, now);
        rebase
    }

    /// Reconfigure the surface and remake every size-bound target.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        if let Some(s) = &self.surface {
            s.configure(&self.ctx.device, &self.config);
        }
        self.retarget(true);
    }

    /// Forget every lane's rows so the next upload writes from row 0; capacity stays.
    pub fn reset(&mut self) {
        self.objects.reset();
        self.arena.reset();
        self.segments.reset();
        self.glyphs.reset();
        self.bounds = Aabb::empty();
    }

    /// Forget every lane's rows AND hand the memory back, CPU mirrors and GPU buffers alike.
    pub fn release(&mut self) {
        self.objects.release(&self.ctx, &self.layouts);
        self.arena.release(&self.ctx);
        self.segments.release(&self.ctx, &self.layouts);
        self.glyphs.release(&self.ctx, &self.layouts);
        self.bounds = Aabb::empty();
        self.retarget(false);
    }
}

/// Every lane's shaders, for the mirror tests: a lane joins by adding its `SHADERS` here.
#[cfg(test)]
pub(crate) fn lane_shaders() -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    out.extend_from_slice(backdrop::SHADERS);
    out.extend_from_slice(arena::SHADERS);
    out.extend_from_slice(segments::SHADERS);
    out.extend_from_slice(glyphs::SHADERS);
    out
}
