pub mod arena;
pub mod backdrop;
pub mod buffers;
pub mod clip;
pub mod cloud;
pub mod device;
pub mod faces;
pub mod frame;
pub mod glyphs;
pub mod hull;
pub mod instance;
pub mod instanced;
pub mod lod;
pub mod objects;

pub mod lane;
pub(crate) mod patch;
pub mod pick;
pub mod present;
pub mod render;
pub mod segments;
pub mod splat;
mod ssao;
pub mod surface_outline;
pub mod targets;
pub mod text;
pub mod text_outline;
pub mod timing;
mod triangle_tiles;
pub mod ui;
pub mod upload;
pub mod vectors;
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
use lane::{Lane, RowLane};
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
    ssao: Option<ssao::Ssao>, // ambient occlusion, when on
    ssao_pipes: [Option<ssao::SsaoPipelines>; 2], // ambient pipelines at 1x and 4x, kept when off
    pub arena: ArenaLane, // meshes
    pub segments: SegmentLane, // lines
    pub glyphs: GlyphLane, // markers and dots
    pub controls: GlyphLane, // control point dots
    pub control_net: SegmentLane, // control polygon lines
    pub widget: widget::Widget, // gumball mesh, own depth
    pub ui: Option<ui::Ui>, // egui overlay
    pub text: text::TextLane, // labels
    pub selection_outline: surface_outline::SurfaceOutline, // outline around the selection
    pub solid_outline: surface_outline::SurfaceOutline, // outline around every solid
    pub selection_revision: u64, // bumps on every selection change
    pub logical_size: [f64; 2], // canvas size in CSS pixels
    pub cloud: CloudLane, // point cloud buffers
    registered: Vec<Box<dyn RowLane>>, // lanes from lane::REGISTRY
    dead: patch::Counts, // editable rows retired and not yet reclaimed
    dead_points: u32, // cloud points retired and not yet reclaimed
    pub splat: Splat, // point cloud drawing
    pub clip: clip::Clip, // clipping planes and their section caps
    pub pick: Picker, // reads object ids under the cursor
    pub performance: Performance, // frame timing
    pub timer: Option<timing::PassTimer>, // GPU time per pass, when a bench installs it
    pub bounds: AABB, // world box of everything uploaded
    device_type: wgpu::DeviceType, // discrete, integrated or CPU
    pub failure: std::sync::Arc<std::sync::Mutex<Option<String>>>, // first GPU error
}

/// Every lane, once. Adding one means writing its `Lane` impl and one line here.
macro_rules! lane_list {
    ($apply:ident, $g:ident) => {
        $apply!($g;
            frame,             // register:frame
            objects,           // register:objects
            backdrop,          // register:backdrop
            arena,             // register:arena
            segments,          // register:segments
            glyphs,            // register:glyphs
            controls,          // register:controls
            control_net,       // register:control_net
            widget,            // register:widget
            text,              // register:text
            selection_outline, // register:selection_outline
            solid_outline,     // register:solid_outline
            cloud,             // register:cloud
            splat,             // register:splat
            clip,              // register:clip
            pick,              // register:pick
            ssao               // register:ssao
        )
    };
}

/// The lanes, shared.
macro_rules! shared {
    ($g:ident; $($lane:ident),*) => { [$(&$g.$lane as &dyn Lane),*] };
}

/// The lanes, mutable. Each field is borrowed once, so `ctx` stays free.
macro_rules! owned {
    ($g:ident; $($lane:ident),*) => { [$(&mut $g.$lane as &mut dyn Lane),*] };
}

impl Gpu {
    /// Bytes reserved on the GPU: (buffers, textures).
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let mut buffers = 0;
        let mut textures = 0;
        for lane in lane_list!(shared, self) {
            let (b, t) = lane.bytes();
            buffers += b;
            textures += t;
        }
        for lane in &self.registered {
            let (b, t) = lane.bytes();
            buffers += b;
            textures += t;
        }
        let pixels = u64::from(self.config.width) * u64::from(self.config.height);
        let samples = u64::from(self.targets.samples);
        // per sample: 4 color + 4 depth + 4 triangle id; at 1x no color copy
        let frame_textures = pixels * if samples > 1 { samples * 12 } else { 8 }
            + if samples > 1 { 8 } else { 32 };
        (buffers, textures + frame_textures)
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
        crate::engine::performance::mark("device ready");
        let ctx = GpuCtx::new(device, queue);
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
        let clip = clip::Clip::new(target);
        let registered = lane::REGISTRY
            .iter()
            .map(|lane| (lane.make)(&ctx, &layouts, target))
            .collect();

        log::info!(
            "viewer init OK - surface {}x{}, format {:?}",
            config.width,
            config.height,
            config.format
        );
        crate::engine::performance::mark("gpu built");
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
            ssao: None,
            ssao_pipes: [None, None],
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
            registered,
            dead: patch::Counts::default(),
            dead_points: 0,
            splat,
            clip,
            pick: Picker::new(),
            performance: Performance::new(),
            timer: None,
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
        for lane in &mut self.registered {
            lane.on_append(&self.ctx, &self.layouts, up);
        }

        // a moved cloud buffer needs a new bind group
        if self.cloud.append(&self.ctx, &up.cloud) {
            self.splat
                .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        }

        self.splat.invalidate();
        self.bounds.union_with(&up.bounds);

        log::debug!(
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
            self.rebuild_pipelines();
            log::info!("msaa: {}x", samples);
        }
    }

    /// Every lane builds its pipelines again for the current target; the cache compiles only new ones.
    fn rebuild_pipelines(&mut self) {
        let target = self.target();
        let ctx = &self.ctx;
        let layouts = &self.layouts;
        for lane in lane_list!(owned, self) {
            lane.on_retarget(ctx, layouts, target);
        }
        for lane in &mut self.registered {
            lane.on_retarget(ctx, layouts, target);
        }
    }

    /// Pixels this GPU can afford at 4x MSAA.
    pub fn msaa_budget(&self) -> Option<u32> {
        Targets::msaa_budget(self.device_type)
    }

    /// Pick the MSAA sample count again, after live rows came or went.
    pub(crate) fn refresh_samples(&mut self) {
        self.retarget(false);
    }

    /// MSAA samples for the current scene: 4x only with solid geometry.
    fn msaa_now(&self) -> u32 {
        let solid = self.live_faces() > 0
            || self.live_sheet() > 0
            || self.live_pipes() > 0
            || self.live_spheres() > 0;
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

    /// Take the editable rows retired but not yet reclaimed, for the live counts.
    pub(crate) fn set_dead(&mut self, dead: patch::Counts, points: u32) {
        self.dead = dead;
        self.dead_points = points;
    }

    /// Solid face indices still drawn.
    pub(crate) fn live_faces(&self) -> u32 {
        self.arena.face_count().saturating_sub(self.dead.faces)
    }

    /// Sheet fill and lettering indices still drawn.
    pub(crate) fn live_sheet(&self) -> u32 {
        self.arena
            .sheet_count()
            .saturating_sub(self.dead.print + self.dead.text)
    }

    /// Pipe rows still drawn.
    pub(crate) fn live_pipes(&self) -> u32 {
        self.segments.pipe_count().saturating_sub(self.dead.pipes)
    }

    /// Ribbon rows still drawn, sheet segments included.
    pub(crate) fn live_ribbons(&self) -> u32 {
        self.segments.ribbon_count().saturating_sub(self.dead.ribbons)
    }

    /// Marker rows still drawn.
    pub(crate) fn live_spheres(&self) -> u32 {
        self.glyphs.sphere_count().saturating_sub(self.dead.spheres)
    }

    /// Dot rows still drawn.
    pub(crate) fn live_dots(&self) -> u32 {
        self.glyphs.dot_count().saturating_sub(self.dead.dots)
    }

    /// Cloud points still drawn.
    pub(crate) fn live_points(&self) -> u32 {
        self.cloud.point_count.saturating_sub(self.dead_points)
    }

    /// Overwrite one object's rows in every editable lane, from the rows `at`.
    pub(crate) fn write_rows(&mut self, at: patch::Counts, up: &Upload) {
        self.arena.patch(&self.ctx, at, &up.arena);
        self.segments.patch(&self.ctx, at, &up.seg);
        self.glyphs.patch(&self.ctx, at, &up.glyph);

        for (i, lane) in self.registered.iter_mut().enumerate() {
            lane.write_at(&self.ctx, &self.layouts, at.lanes[i], up);
        }

        self.objects.geometry_changed();
    }

    /// Hand `count` rows of `lane` from `first` to the hidden object row `sink`.
    pub(crate) fn kill_rows(&mut self, lane: patch::LaneId, first: u32, count: u32, sink: u32) {
        use patch::LaneId;

        if count == 0 {
            return;
        }

        match lane {
            LaneId::Verts | LaneId::Faces | LaneId::Print | LaneId::Text | LaneId::Sources => {
                self.arena.kill(&self.ctx, lane, first, count, sink)
            }
            LaneId::Pipes | LaneId::Ribbons => {
                self.segments.kill(&self.ctx, lane, first, count, sink)
            }
            LaneId::Spheres => self.glyphs.kill(&self.ctx, true, first, count, sink),
            LaneId::Dots => self.glyphs.kill(&self.ctx, false, first, count, sink),
            LaneId::Registered(i) => {
                self.registered[i as usize].kill(&self.ctx, first, count, sink)
            }
        }

        self.objects.geometry_changed();
    }

    /// Turn `count` face, print or text indices from `first` into empty triangles on `vertex`.
    pub(crate) fn degenerate_rows(
        &mut self,
        lane: patch::LaneId,
        first: u32,
        count: u32,
        vertex: u32,
    ) {
        self.arena.degenerate(&self.ctx, lane, first, count, vertex);
        self.objects.geometry_changed();
    }

    /// Free the editable lanes; object rows, clouds, sheets, text and controls stay.
    pub(crate) fn release_editable(&mut self) {
        let ctx = &self.ctx;
        let layouts = &self.layouts;
        self.arena.release(ctx);
        self.segments.release_editable(ctx, layouts);
        self.glyphs.release(ctx, layouts);

        for lane in &mut self.registered {
            lane.on_release(ctx, layouts);
        }

        self.dead = patch::Counts::default();
        self.objects.geometry_changed();
        self.rebind_ink();
    }

    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        let ctx = &self.ctx;
        for lane in lane_list!(owned, self) {
            lane.on_reset(ctx);
        }
        for lane in &mut self.registered {
            lane.on_reset(ctx);
        }
        self.segments.set_edge(&self.ctx, None);
        self.bounds = AABB::empty();
        self.dead = patch::Counts::default();
        self.dead_points = 0;
    }

    /// Forget every row and free the buffers.
    pub fn release(&mut self) {
        let ctx = &self.ctx;
        let layouts = &self.layouts;
        for lane in lane_list!(owned, self) {
            lane.on_release(ctx, layouts);
        }
        for lane in &mut self.registered {
            lane.on_release(ctx, layouts);
        }
        self.segments.set_edge(&self.ctx, None);
        // a freed cloud buffer needs a new bind group
        self.splat
            .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        self.bounds = AABB::empty();
        self.dead = patch::Counts::default();
        self.dead_points = 0;
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

    /// Set the face or edge color of object `row`; None restores its own.
    pub fn set_object_color(&mut self, row: u32, edge: bool, color: Option<[u8; 3]>) {
        self.objects.set_color(&self.ctx, row, edge, color);
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
    out.extend_from_slice(vectors::SHADERS);
    out
}
