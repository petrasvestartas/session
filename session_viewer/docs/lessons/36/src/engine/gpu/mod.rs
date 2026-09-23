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

pub(crate) mod patch;
pub mod pick;
pub mod present;
pub mod render;
pub mod segments;
pub mod splat;
// --8<-- [start:step-13a]
mod ssao;
// --8<-- [end:step-13a]
pub mod surface_outline;
pub mod targets;
pub mod text;
pub mod text_outline;
mod triangle_tiles;
pub mod ui;
pub mod upload;
pub mod view;
mod widget;
mod widget_mesh;

use crate::engine::performance::Performance;
use crate::engine::pipelines::{Layouts, Target};
use session_rust::{AABB, Point};

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

/// Everything on the GPU: the device, the frame and one field per lane.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // the canvas; None when headless
    pub ctx: GpuCtx, // device and queue
    pub config: wgpu::SurfaceConfiguration, // canvas size and format
    pub layouts: Layouts, // shared bind group layouts
    pub frame: FrameUniforms, // per-frame uniform buffers
    pub targets: Targets, // depth and color textures
    pub view: View, // display settings
    pub objects: InstanceTable, // one row per object
    pub backdrop: BackdropLane, // background and grid
    // --8<-- [start:step-13b]
    ssao: Option<ssao::Ssao>,
    // --8<-- [end:step-13b]
    pub arena: ArenaLane, // meshes
    pub segments: SegmentLane, // lines
    pub glyphs: GlyphLane, // markers and dots
    pub controls: GlyphLane, // control point dots
    pub control_net: SegmentLane, // control polygon lines
    pub widget: widget::Widget, // gumball mesh, own depth
    pub ui: Option<ui::Ui>,
    pub text: text::TextLane, // labels
    pub selection_outline: surface_outline::SurfaceOutline, // outline around the selection
    pub solid_outline: surface_outline::SurfaceOutline, // outline around every solid
    pub selection_revision: u64, // bumps on every selection change
    pub logical_size: [f64; 2], // canvas size in CSS pixels
    pub cloud: CloudLane, // point cloud buffers
    pub splat: Splat, // point cloud drawing
    pub pick: Picker, // reads object ids under the cursor
    pub performance: Performance, // frame timing
    pub bounds: AABB, // world box of everything uploaded
    device_type: wgpu::DeviceType, // discrete, integrated or CPU
    pub failure: std::sync::Arc<std::sync::Mutex<Option<String>>>, // first GPU error
}

impl Gpu {
    /// Bytes reserved on the GPU: (buffers, textures).
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
            + self.widget.allocated_bytes().0
            + self.cloud.allocated_bytes()
            + self.objects.allocated_bytes()
            + self.frame.allocated_bytes()
            + self.text.allocated_bytes()
            + splat_buffers
            + pick_buffers
            // --8<-- [start:step-13c]
            + outline_buffers
            + if self.ssao.is_some() { 144 } else { 0 };
            // --8<-- [end:step-13c]
        let pixels = u64::from(self.config.width) * u64::from(self.config.height);
        let samples = u64::from(self.targets.samples);
        // per sample: 4 color + 4 depth + 8 gradient; at 1x no color copy
        let frame_textures = pixels * if samples > 1 { samples * 16 } else { 12 }
            + if samples > 1 { 12 } else { 48 };
        (
            buffers,
            frame_textures
                + self.widget.allocated_bytes().1
                + self.arena.tiles.allocated_bytes().1
                + splat_textures
                + pick_textures
                + self.text.texture_bytes()
                // --8<-- [start:step-13d]
                + outline_textures
                + self.ssao.as_ref().map_or(0, ssao::Ssao::texture_bytes),
                // --8<-- [end:step-13d]
        )
    }

    /// Open the GPU for a window.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), (size.width, size.height)).await
    }

    /// Open the GPU with no window, drawing into a texture.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, (width, height)).await
    }

    /// Open the device and create every lane, empty.
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
        // start without MSAA; retarget flips it later
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
        let widget = widget::Widget::new(&ctx, target);
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
            // --8<-- [start:step-13e]
            ssao: None,
            // --8<-- [end:step-13e]
            arena,
            segments,
            glyphs,
            controls,
            control_net,
            widget,
            ui: None,
            text,
            selection_outline,
            solid_outline,
            selection_revision: 0,
            logical_size: [size.0 as f64, size.1 as f64],
            cloud,
            splat,
            pick: Picker::new(),
            performance: Performance::new(),
            bounds: AABB::empty(),
            device_type,
            failure,
        })
    }

    /// Append one upload to every lane.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);

        // a moved cloud buffer needs a new bind group
        if self.cloud.append(&self.ctx, &up.cloud) {
            self.splat
                .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        }

        self.splat.invalidate();
        self.bounds.union_with(&up.bounds);

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

    /// Grow the scene box to include object `row`.
    pub fn grew_bounds(&mut self, row: u32) {
        if let Some(box_) = self.objects.row_bounds(row) {
            self.bounds.union_with(&box_);
        }
    }

    /// Rebuild the ink bind group after targets or tiles moved.
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

    /// Current color format and sample count.
    fn target(&self) -> Target {
        Target {
            format: self.config.format,
            samples: self.targets.samples,
        }
    }

    /// Remake targets and pipelines when the sample count changes.
    fn retarget(&mut self, resized: bool) {
        let samples = self.msaa_now();
        let flip = samples != self.targets.samples;

        if flip || resized {
            self.targets.destroy();
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
            self.widget.retarget(&self.ctx, target);
            self.text.retarget(&self.ctx, target);
            self.selection_outline.retarget(&self.ctx, target);
            self.solid_outline.retarget(&self.ctx, target);
            self.splat.retarget(&self.ctx, &self.layouts, target);
            log::info!("msaa: {}x", samples);
        }
    }

    /// Pixels this GPU can afford at 4x MSAA.
    pub fn msaa_budget(&self) -> Option<u32> {
        Targets::msaa_budget(self.device_type)
    }

    /// MSAA samples for the current scene: 4x only with solid geometry.
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

    /// Move the scene origin near the camera when it drifted far.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self
            .objects
            .rebase_anchor(&self.ctx, origin, view_dist, now);

        if rebase.moved {
            self.splat.invalidate();
        }

        rebase
    }

    /// Resize the canvas and every texture that follows it.
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

    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        self.objects.reset();
        self.arena.reset(&self.ctx);
        self.segments.reset();
        self.glyphs.reset();
        self.controls.reset();
        self.control_net.reset();
        self.widget.clear();
        self.text.reset();
        self.selection_outline.reset();
        self.solid_outline.reset();
        self.pick.cancel();
        self.segments.set_edge(&self.ctx, None);
        self.cloud.reset();
        self.splat.invalidate();
        self.bounds = AABB::empty();
    }

    /// Forget every row and free the buffers.
    pub fn release(&mut self) {
        self.objects.release(&self.ctx, &self.layouts);
        self.arena.release(&self.ctx);
        self.segments.release(&self.ctx, &self.layouts);
        self.glyphs.release(&self.ctx, &self.layouts);
        self.controls.release(&self.ctx, &self.layouts);
        self.control_net.release(&self.ctx, &self.layouts);
        self.widget.clear();
        self.text.release(&self.ctx);
        self.selection_outline.reset();
        self.solid_outline.reset();
        self.pick.cancel();
        self.segments.set_edge(&self.ctx, None);
        self.cloud.release(&self.ctx);
        self.splat.release();
        self.splat
            .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        self.bounds = AABB::empty();
        self.retarget(false);
        self.rebind_ink();
    }

    /// Select or deselect object `row`.
    pub fn set_selected(&mut self, row: u32, on: bool) {
        self.selection_revision = self.selection_revision.wrapping_add(1);
        self.segments.set_selected(row, on);
        self.selection_outline.set_selected(row, on);
        self.objects
            .set_flag(&self.ctx, row, Instance::FLAG_SELECTED, on);
        self.splat.invalidate();
    }

    // --8<-- [start:step-13f]
    /// Set the face or edge color of object `row`; None restores its own.
    pub fn set_object_color(&mut self, row: u32, edge: bool, color: Option<[u8; 3]>) {
        self.objects.set_color(&self.ctx, row, edge, color);
        // --8<-- [end:step-13f]
        self.splat.invalidate();
    }

    /// Hide or show object `row`.
    pub fn set_hidden(&mut self, row: u32, on: bool) {
        self.objects
            .set_flag(&self.ctx, row, Instance::FLAG_HIDDEN, on);
        self.splat.invalidate();
    }
}

/// Every lane's shader sources, for the tests.
#[cfg(test)]
pub(crate) fn lane_shaders() -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    out.extend_from_slice(backdrop::SHADERS);
    out.extend_from_slice(arena::SHADERS);
    out.extend_from_slice(segments::SHADERS);
    out.extend_from_slice(glyphs::SHADERS);
    out
}
