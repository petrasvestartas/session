//! `Gpu` - the lowest layer of the viewer: the floor (surface, device, layouts, frame
//! uniforms, targets, view knobs, the object table) and the lanes, one file each. This file
//! builds the struct, appends an upload and keeps the lanes' targets current; the frame list
//! is `render.rs`, presenting is `present.rs`, picking is `pick.rs`.

pub mod arena;
pub mod backdrop;
pub mod buffers;
pub mod cloud;
pub mod device;
pub mod faces;
pub mod frame;
pub mod glyphs;
pub mod instance;
pub mod lod;
pub mod objects;
pub mod pick;
pub mod present;
pub mod render;
pub mod segments;
pub mod splat;
pub mod surface_outline;
pub mod targets;
pub mod text;
pub mod text_outline;
mod triangle_tiles;
pub mod upload;
pub mod view;

use crate::engine::performance::Performance;
use crate::engine::pipelines::{Layouts, Target};
use crate::math::Aabb;
use session_rust::Point;

use arena::ArenaLane;
use backdrop::BackdropLane;
use buffers::GpuCtx;
use cloud::CloudLane;
use device::DeviceSetup;
use frame::FrameUniforms;
use glyphs::GlyphLane;
use objects::{InkScene, InstanceTable};
use pick::Picker;
use segments::SegmentLane;
use splat::Splat;
use targets::Targets;

pub use cloud::{CloudDraw, LodNode, NO_NORMALS};
pub use frame::FrameInput;
pub use glyphs::GlyphPoint;
pub use instance::Instance;
pub use objects::{ObjectRow, Rebase};
pub use pick::Pick;
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
    pub controls: GlyphLane,
    pub control_net: SegmentLane,
    /// The move/rotate/scale widget, drawn by the same two lane types the control net uses:
    /// three arms and their balls are strokes and markers, so the widget costs no shader and
    /// no pipeline of its own.
    pub gizmo_arms: SegmentLane,
    pub gizmo_dots: GlyphLane,
    pub text: text::TextLane,
    pub selection_outline: surface_outline::SurfaceOutline,
    pub solid_outline: surface_outline::SurfaceOutline,
    /// Counts selection flag changes: part of the coverage masks' cache key.
    pub selection_revision: u64,
    pub logical_size: [f64; 2],
    pub cloud: CloudLane,
    pub splat: Splat,
    pub pick: Picker,
    pub performance: Performance,
    /// The world box of everything uploaded; the camera fits it and the inside test reads it.
    pub bounds: Aabb,
    /// What class of GPU is drawing; the antialiasing budget is spent against it.
    device_type: wgpu::DeviceType,
    pub failure: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

impl Gpu {
    /// Owned buffers and framebuffer texture arithmetic, not a physical VRAM measurement.
    /// Glyphon's private atlas/instance capacities and browser swapchain allocations are separate.
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let (splat_buffers, splat_textures) = self.splat.allocated_bytes();
        let (pick_buffers, pick_textures) = self.pick.allocated_bytes();
        let (mut outline_buffers, mut outline_textures) = self.selection_outline.allocated_bytes();
        let (buffers, textures) = self.solid_outline.allocated_bytes();
        outline_buffers += buffers;
        outline_textures += textures;
        let buffers = self.arena.allocated_bytes()
            + self.segments.allocated_bytes()
            + self.glyphs.allocated_bytes()
            + self.controls.allocated_bytes()
            + self.control_net.allocated_bytes()
            + self.gizmo_arms.allocated_bytes()
            + self.gizmo_dots.allocated_bytes()
            + self.cloud.allocated_bytes()
            + self.objects.allocated_bytes()
            + self.frame.allocated_bytes()
            + self.text.allocated_bytes()
            + splat_buffers
            + pick_buffers
            + outline_buffers;
        let pixels = u64::from(self.config.width) * u64::from(self.config.height);
        let samples = u64::from(self.targets.samples);
        // 16 B a sample is the three physical attachments together: 4 for the MSAA colour, 4
        // for depth32, 8 for the Rgba16Float gradient. At 1x there IS no MSAA colour target -
        // the pass draws into the swapchain, which this process does not own - so a pixel
        // costs 12. The constant that follows is the 1x1 placeholder pair `Targets::new` makes
        // at the OTHER sample count: 4 x 12 while we are at 1x, 12 while we are at 4x.
        let frame_textures = pixels * if samples > 1 { samples * 16 } else { 12 }
            + if samples > 1 { 12 } else { 48 };
        (
            buffers,
            frame_textures
                + self.arena.tiles.allocated_bytes().1
                + splat_textures
                + pick_textures
                + self.text.texture_bytes()
                + outline_textures,
        )
    }

    /// The stack over a canvas window.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), (size.width, size.height)).await
    }

    /// The same stack with no window and no surface, rendering into an offscreen texture.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, (width, height)).await
    }

    /// Negotiate the device, make every layout, buffer, bind group and pipeline, start empty.
    async fn build(
        window: Option<std::sync::Arc<winit::window::Window>>,
        size: (u32, u32),
    ) -> anyhow::Result<Self> {
        let DeviceSetup {
            surface,
            device,
            queue,
            config,
            device_type,
            failure,
        } = device::open(window, size).await?;
        let ctx = GpuCtx { device, queue };
        let size = (config.width, config.height);
        let target = Target {
            format: config.format,
            samples: 1,
        };

        let layouts = Layouts::new(&ctx.device);
        let frame = FrameUniforms::new(&ctx, &layouts, size);
        let targets = Targets::new(&ctx, size, config.format, target.samples);
        let arena = ArenaLane::new(&ctx, &layouts, target);
        let objects = InstanceTable::new(
            &ctx,
            &layouts,
            &InkScene {
                targets: &targets,
                tiles: &arena.tiles,
            },
        );
        let backdrop = BackdropLane::new(&ctx, &layouts, target);
        let segments = SegmentLane::new(&ctx, &layouts, target);
        let glyphs = GlyphLane::new(&ctx, &layouts, target);
        let controls = GlyphLane::new(&ctx, &layouts, target);
        let control_net = SegmentLane::new(&ctx, &layouts, target);
        let gizmo_arms = SegmentLane::new(&ctx, &layouts, target);
        let gizmo_dots = GlyphLane::new(&ctx, &layouts, target);
        let text = text::TextLane::new(&ctx, target);
        let selection_outline = surface_outline::SurfaceOutline::new(
            &ctx,
            target,
            surface_outline::OutlineKind::Selected,
        );
        let solid_outline = surface_outline::SurfaceOutline::new(
            &ctx,
            target,
            surface_outline::OutlineKind::AllSolids,
        );
        let cloud = CloudLane::new(&ctx);
        let splat = Splat::new(&ctx, &layouts, target, cloud.buffers());

        log::info!(
            "viewer init OK - surface {}x{}, format {:?}",
            config.width,
            config.height,
            config.format
        );
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
            controls,
            control_net,
            gizmo_arms,
            gizmo_dots,
            text,
            selection_outline,
            solid_outline,
            selection_revision: 0,
            logical_size: [size.0 as f64, size.1 as f64],
            cloud,
            splat,
            pick: Picker::new(),
            performance: Performance::new(),
            bounds: Aabb::empty(),
            device_type,
            failure,
        })
    }

    /// Append one upload to every lane. Every table is a DELTA; a bind group is rebuilt only
    /// when its buffer grew. An MSAA flip rebuilds the targets and every pipeline.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);
        if self.cloud.append(&self.ctx, &up.cloud) {
            self.splat
                .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        }
        self.splat.invalidate();
        self.bounds.union(&up.bounds);

        log::info!(
            "scene: {} objects, {} verts, {} pipes, {} ribbons, {} markers, {} dots, {} points",
            self.objects.len(),
            self.arena.vert_count(),
            self.segments.pipe_count(),
            self.segments.ribbon_count(),
            self.glyphs.sphere_count(),
            self.glyphs.dot_count(),
            self.cloud.point_count
        );
        self.retarget(false);
        self.rebind_ink();
    }

    /// Group 2 for ink is rebuilt whenever the depth targets or the tile pool moved.
    fn rebind_ink(&mut self) {
        self.objects.rebind_ink(
            &self.ctx,
            &self.layouts,
            &InkScene {
                targets: &self.targets,
                tiles: &self.arena.tiles,
            },
        );
    }

    /// The pass target the lanes are built for now.
    fn target(&self) -> Target {
        Target {
            format: self.config.format,
            samples: self.targets.samples,
        }
    }

    /// Bring the targets to the sample count the scene and canvas call for: on a change every
    /// lane's pipelines follow; `resized` remakes the targets even at the same count.
    fn retarget(&mut self, resized: bool) {
        let samples = self.msaa_now();
        let flip = samples != self.targets.samples;
        if flip || resized {
            self.targets = Targets::new(
                &self.ctx,
                (self.config.width, self.config.height),
                self.config.format,
                samples,
            );
            self.rebind_ink();
        }
        if flip {
            let target = self.target();
            self.backdrop.retarget(&self.ctx, &self.layouts, target);
            self.arena.retarget(&self.ctx, &self.layouts, target);
            self.segments.retarget(&self.ctx, &self.layouts, target);
            self.glyphs.retarget(&self.ctx, &self.layouts, target);
            self.controls.retarget(&self.ctx, &self.layouts, target);
            self.control_net.retarget(&self.ctx, &self.layouts, target);
            self.gizmo_arms.retarget(&self.ctx, &self.layouts, target);
            self.gizmo_dots.retarget(&self.ctx, &self.layouts, target);
            self.text.retarget(&self.ctx, target);
            self.selection_outline.retarget(&self.ctx, target);
            self.solid_outline.retarget(&self.ctx, target);
            self.splat.retarget(&self.ctx, &self.layouts, target);
            log::info!("msaa: {}x", samples);
        }
    }

    /// How many pixels this adapter carries at 4x; the canvas is sized to fit inside it.
    pub fn msaa_budget(&self) -> Option<u32> {
        Targets::msaa_budget(self.device_type)
    }

    /// The sample count for what is ON the GPU now: 4x only with solid geometry (faces,
    /// pipes, spheres), or imported sheet vectors, on a canvas within the memory budget.
    /// Pure analytic strokes/clouds stay at 1x; vector lettering needs coverage samples.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.face_count() > 0
            || self.arena.sheet_count() > 0
            || self.segments.pipe_count() > 0
            || self.glyphs.sphere_count() > 0;
        Targets::samples_for(
            solid,
            self.config.width * self.config.height,
            self.view.msaa_forced,
            self.msaa_budget(),
            self.config.width as f32 / self.logical_size[0].max(1.0) as f32,
        )
    }

    /// The anchor the instance table is rebased about. A rebase moves every model, so the
    /// point pass is stale. `now` is the frame's one timestamp (ms).
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self
            .objects
            .rebase_anchor(&self.ctx, origin, view_dist, now);
        if rebase.moved {
            self.splat.invalidate();
        }
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
        self.splat.resize();
        self.pick.resize();
    }

    /// Forget every lane's rows so the next upload writes from row 0; capacity stays.
    pub fn reset(&mut self) {
        self.objects.reset();
        self.arena.reset(&self.ctx);
        self.segments.reset();
        self.glyphs.reset();
        self.controls.reset();
        self.control_net.reset();
        self.gizmo_arms.reset();
        self.gizmo_dots.reset();
        self.text.reset();
        self.selection_outline.reset();
        self.solid_outline.reset();
        self.pick.cancel();
        self.segments.set_edge(&self.ctx, None);
        self.cloud.reset();
        self.splat.invalidate();
        self.bounds = Aabb::empty();
    }

    /// Forget every lane's rows AND hand the memory back, CPU mirrors and GPU buffers alike.
    pub fn release(&mut self) {
        self.objects.release(&self.ctx, &self.layouts);
        self.arena.release(&self.ctx);
        self.segments.release(&self.ctx, &self.layouts);
        self.glyphs.release(&self.ctx, &self.layouts);
        self.controls.release(&self.ctx, &self.layouts);
        self.control_net.release(&self.ctx, &self.layouts);
        self.gizmo_arms.release(&self.ctx, &self.layouts);
        self.gizmo_dots.release(&self.ctx, &self.layouts);
        self.text.release(&self.ctx);
        self.selection_outline.reset();
        self.solid_outline.reset();
        self.pick.cancel();
        self.segments.set_edge(&self.ctx, None);
        self.cloud.release(&self.ctx);
        self.splat.release();
        self.splat
            .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        self.bounds = Aabb::empty();
        self.retarget(false);
        self.rebind_ink();
    }

    /// Flip the selection flag on one object row.
    pub fn set_selected(&mut self, row: u32, on: bool) {
        self.selection_revision = self.selection_revision.wrapping_add(1);
        self.segments.set_selected(row, on);
        self.selection_outline.set_selected(row, on);
        self.objects
            .set_flag(&self.ctx, row, Instance::FLAG_SELECTED, on);
        self.splat.invalidate();
    }

    /// Hide or show `row`. Every lane's vertex stage parks a hidden row outside the clip
    /// volume, so it leaves the picture and the ID pass together; clouds are dropped on the
    /// CPU when the splat records are rebuilt, hence the invalidate.
    pub fn set_hidden(&mut self, row: u32, on: bool) {
        self.objects
            .set_flag(&self.ctx, row, Instance::FLAG_HIDDEN, on);
        self.splat.invalidate();
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
