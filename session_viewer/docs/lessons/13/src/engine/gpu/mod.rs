// --8<-- [start:modules]
// One line per file of this folder; `pub` lets code outside `gpu` reach it.
// A line tagged `register:<name>` is a registration: each later lesson adds its own line to lists like this one.
pub mod arena; // register:arena
pub mod backdrop; // register:backdrop
pub mod buffers; // register:buffers
pub mod cloud; // register:cloud
pub mod device; // register:device
pub mod faces; // register:faces
pub mod frame; // register:frame
pub mod glyphs; // register:glyphs
pub mod hull; // register:hull
pub mod instance; // register:instance
pub mod lod; // register:lod
pub mod objects;
pub mod slots; // register:slots

pub mod lane; // register:lane
pub mod pass; // register:pass
pub(crate) mod patch; // register:patch
pub mod pick; // register:pick
pub mod present; // register:present
pub mod render; // register:render
pub mod segments; // register:segments
pub mod splat; // register:splat
pub mod targets; // register:targets
pub mod text; // register:text
pub mod text_outline; // register:text_outline
pub mod upload; // register:upload
pub mod vectors; // register:vectors
pub mod view; // register:view
// --8<-- [end:modules]

// --8<-- [start:uses]
use crate::engine::performance::Performance;
use crate::engine::pipelines::{Layouts, Target};
use session_rust::{AABB, Point};

use arena::ArenaLane; // register:meshes
use backdrop::BackdropLane;
use buffers::GpuCtx;
use cloud::CloudLane; // register:clouds
use device::DeviceSetup;
use frame::FrameUniforms;
use glyphs::GlyphLane; // register:markers
use lane::{Lane, RowLane};
use objects::InkScene; // register:ink
use objects::InstanceTable;
use pass::Pass;
use pick::Picker; // register:shell
use segments::SegmentLane; // register:strokes
use splat::Splat; // register:clouds
use targets::Targets;

pub use cloud::{CloudDraw, LodNode, NO_NORMALS}; // register:clouds
pub use frame::FrameInput;
pub use glyphs::GlyphPoint; // register:markers
pub use instance::Instance;
pub use objects::{ObjectRow, Rebase};
pub use pick::Pick; // register:shell
pub use segments::CylinderSegment; // register:strokes
pub use upload::Upload;
pub use view::View;
// --8<-- [end:uses]

// --8<-- [start:gpu-struct]
/// Everything on the GPU: the device, the frame and one field per lane.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // the canvas; None when headless
    pub ctx: GpuCtx,                             // device and queue
    pub config: wgpu::SurfaceConfiguration,      // canvas size and format
    pub layouts: Layouts,                        // shared bind group layouts
    pub frame: FrameUniforms,                    // per-frame uniform buffers
    pub targets: Targets,                        // depth and color textures
    pub view: View,                              // display settings
    pub objects: InstanceTable,                  // one row per object
    pub backdrop: BackdropLane,                  // background and grid
    pub arena: ArenaLane,                        // meshes; register:meshes
    pub segments: SegmentLane,                   // lines; register:strokes
    pub glyphs: GlyphLane,                       // markers and dots; register:markers
    pub controls: GlyphLane,                     // control point dots; register:shell
    pub control_net: SegmentLane,                // control polygon lines; register:shell
    pub text: text::TextLane,                    // labels; register:text
    pub selection_revision: u64,                 // bumps on every selection change
    pub logical_size: [f64; 2],                  // canvas size in CSS pixels
    pub cloud: CloudLane,                        // point cloud buffers; register:clouds
    registered: Vec<Box<dyn RowLane>>,           // lanes from lane::REGISTRY
    passes: Vec<Box<dyn Pass>>,                  // passes from pass::PASSES, in frame order
    dead: patch::Counts, // editable rows retired and not yet reclaimed; register:patch
    dead_points: u32,    // cloud points retired and not yet reclaimed; register:clouds
    pub splat: Splat,    // point cloud drawing; register:clouds
    pub pick: Picker,    // reads object ids under the cursor; register:shell
    pub performance: Performance, // frame timing
    pub bounds: AABB,                     // world box of everything uploaded
    device_type: wgpu::DeviceType,        // discrete, integrated or CPU
    pub failure: std::sync::Arc<std::sync::Mutex<Option<String>>>, // first GPU error
}
// --8<-- [end:gpu-struct]

// --8<-- [start:lane-list]
// A macro that takes a macro: `lane_list!(shared, self)` becomes `shared!(self; frame, objects, backdrop)`, so every loop below walks one list.
/// Every lane, once. Adding one means writing its `Lane` impl and one line here.
macro_rules! lane_list {
    ($apply:ident, $g:ident) => {
        $apply!($g;
            frame,             // register:frame
            objects,
            backdrop,          // register:backdrop
            arena,             // register:meshes
            segments,          // register:strokes
            glyphs,            // register:markers
            controls,          // register:shell
            control_net,       // register:shell
            text,              // register:text
            cloud,             // register:clouds
            splat,             // register:clouds
            pick,              // register:shell
        )
    };
}

/// The lanes, shared.
macro_rules! shared {
    // `$(...),*` repeats once per name; `as &dyn Lane` views each field through the Lane trait
    ($g:ident; $($lane:ident),* $(,)?) => { [$(&$g.$lane as &dyn Lane),*] };
}

/// The lanes, mutable. Each field is borrowed once, so `ctx` stays free.
macro_rules! owned {
    ($g:ident; $($lane:ident),* $(,)?) => { [$(&mut $g.$lane as &mut dyn Lane),*] };
}
// --8<-- [end:lane-list]

// --8<-- [start:gpu-bytes]
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
        for pass in &self.passes {
            let (b, t) = pass.bytes();
            buffers += b;
            textures += t;
        }
        let pixels = u64::from(self.config.width) * u64::from(self.config.height);
        let samples = u64::from(self.targets.samples);
        // per sample: 4 color + 4 depth + 4 triangle id; at 1x no color copy
        let frame_textures =
            pixels * if samples > 1 { samples * 12 } else { 8 } + if samples > 1 { 8 } else { 32 };
        (buffers, textures + frame_textures)
    }
// --8<-- [end:gpu-bytes]

// --8<-- [start:gpu-new]
    /// Open the GPU for a window.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), (size.width, size.height)).await
    }

    /// Open the GPU with no window, drawing into a texture.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, (width, height)).await
    }
// --8<-- [end:gpu-new]

// --8<-- [start:gpu-build]
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
        let arena = ArenaLane::new(&ctx, &layouts, target); // register:meshes
        let objects = InstanceTable::new(&ctx, &layouts);
        let backdrop = BackdropLane::new(&ctx, &layouts, target);
        let segments = SegmentLane::new(&ctx, &layouts, target); // register:strokes
        let glyphs = GlyphLane::new(&ctx, &layouts, target); // register:markers
        let controls = GlyphLane::new(&ctx, &layouts, target); // register:shell
        let control_net = SegmentLane::new(&ctx, &layouts, target); // register:shell
        let text = text::TextLane::new(&ctx, target); // register:text
        let cloud = CloudLane::new(&ctx); // register:clouds
        let splat = Splat::new(&ctx, &layouts, target, cloud.buffers()); // register:clouds
        // each registered lane builds itself through its `make` function
        let registered = lane::REGISTRY
            .iter()
            .map(|lane| (lane.make)(&ctx, &layouts, target))
            .collect();
        let passes = pass::PASSES.iter().map(|make| make(&ctx, target)).collect();

        log::info!(
            "viewer init OK - surface {}x{}, format {:?}",
            config.width,
            config.height,
            config.format
        );
        crate::engine::performance::mark("gpu built");
        let mut gpu = Self {
            surface,
            ctx,
            config,
            layouts,
            frame,
            targets,
            view: View::from_env(),
            objects,
            backdrop,
            arena,       // register:meshes
            segments,    // register:strokes
            glyphs,      // register:markers
            controls,    // register:shell
            control_net, // register:shell
            text,        // register:text
            selection_revision: 0,
            logical_size: [size.0 as f64, size.1 as f64],
            cloud, // register:clouds
            registered,
            passes,
            dead: patch::Counts::default(), // register:patch
            dead_points: 0,                 // register:clouds
            splat,                          // register:clouds
            pick: Picker::new(),            // register:shell
            performance: Performance::new(),
            bounds: AABB::empty(),
            device_type,
            failure,
        };
        gpu.rebind_ink(); // register:ink
        Ok(gpu)
    }
// --8<-- [end:gpu-build]

// --8<-- [start:set-scene]
    /// Append one upload to every lane.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena); // register:meshes
        self.segments.append(&self.ctx, &self.layouts, &up.seg); // register:strokes
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph); // register:markers
        for lane in &mut self.registered {
            lane.on_append(&self.ctx, &self.layouts, up);
        }

        self.append_cloud(up); // register:clouds
        self.bounds.union_with(&up.bounds);
        self.log_scene(); // register:clouds
        self.retarget(false);
        self.rebind_ink(); // register:ink
    }

    /// Current color format and sample count.
    pub(super) fn target(&self) -> Target {
        Target {
            format: self.config.format,
            samples: self.targets.samples,
        }
    }
// --8<-- [end:set-scene]

// --8<-- [start:retarget]
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
            self.rebind_ink(); // register:ink
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
        for pass in &mut self.passes {
            pass.on_retarget(ctx, layouts, target);
        }
    }
// --8<-- [end:retarget]

// --8<-- [start:msaa]
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
        let mut solid = false;
        solid |= self.live_faces() > 0; // register:meshes
        solid |= self.live_sheet() > 0; // register:meshes
        solid |= self.live_pipes() > 0; // register:strokes
        solid |= self.live_spheres() > 0; // register:markers
        Targets::samples_for(
            solid,
            self.config.width * self.config.height,
            self.view.msaa_forced,
            self.msaa_budget(),
            self.config.width as f32 / self.logical_size[0].max(1.0) as f32,
        )
    }
// --8<-- [end:msaa]

// --8<-- [start:resize]
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
        self.splat.resize(); // register:clouds
        self.pick.resize(); // register:shell
    }
// --8<-- [end:resize]

// --8<-- [start:reset]
    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        let ctx = &self.ctx;
        for lane in lane_list!(owned, self) {
            lane.on_reset(ctx);
        }
        for lane in &mut self.registered {
            lane.on_reset(ctx);
        }
        for pass in &mut self.passes {
            pass.on_reset(ctx);
        }
        self.segments.set_edge(&self.ctx, None); // register:strokes
        self.bounds = AABB::empty();
        self.dead = patch::Counts::default(); // register:patch
        self.dead_points = 0; // register:clouds
    }
// --8<-- [end:reset]

// --8<-- [start:release]
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
        for pass in &mut self.passes {
            pass.on_release(ctx, layouts);
        }
        self.segments.set_edge(&self.ctx, None); // register:strokes
        self.rebind_cloud(); // a freed cloud buffer needs a new bind group; register:clouds
        self.bounds = AABB::empty();
        self.dead = patch::Counts::default(); // register:patch
        self.dead_points = 0; // register:clouds
        self.retarget(false);
        self.rebind_ink(); // register:ink
    }
}
// --8<-- [end:release]

// --8<-- [start:lane-shaders]
/// Every lane's shader sources, for the tests.
#[cfg(test)]
pub(crate) fn lane_shaders() -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    out.extend_from_slice(backdrop::SHADERS);
    out.extend_from_slice(arena::SHADERS); // register:meshes
    out.extend_from_slice(segments::SHADERS); // register:strokes
    out.extend_from_slice(glyphs::SHADERS); // register:markers
    out.extend_from_slice(vectors::SHADERS); // register:strokes
    out
}
// --8<-- [end:lane-shaders]

// --8<-- [start:03-rows]
// --8<-- [start:scene-edits]
impl Gpu {
    /// Grow the scene box to include object `row`.
    pub fn grew_bounds(&mut self, row: u32) {
        if let Some(box_) = self.objects.row_bounds(row) {
            self.bounds.union_with(&box_);
        }
    }

    /// Move the scene origin near the camera when it drifted far.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self
            .objects
            .rebase_anchor(&self.ctx, origin, view_dist, now);

        if rebase.moved {
            self.splat.invalidate(); // register:clouds
        }

        rebase
    }

    /// Take the editable rows retired but not yet reclaimed, for the live counts.
    pub(crate) fn set_dead(&mut self, dead: patch::Counts, points: u32) {
        self.dead = dead;
        self.dead_points = points; // register:clouds
    }
// --8<-- [end:scene-edits]

// --8<-- [start:row-edits]
    // Every edit below writes one 96-byte row and never the geometry: selecting a mesh of a million triangles is one small write.
    /// Select or deselect object `row`.
    pub fn set_selected(&mut self, row: u32, on: bool) {
        self.selection_revision = self.selection_revision.wrapping_add(1);
        self.segments.set_selected(row, on); // register:strokes
        for pass in &mut self.passes {
            pass.on_select(row, on);
        }
        self.objects
            .set_flag(&self.ctx, row, Instance::FLAG_SELECTED, on);
        self.splat.invalidate(); // register:clouds
    }

    /// Set the face or edge color of object `row`; None restores its own.
    pub fn set_object_color(&mut self, row: u32, edge: bool, color: Option<[u8; 3]>) {
        self.objects.set_color(&self.ctx, row, edge, color);
        self.splat.invalidate(); // register:clouds
    }

    // Hidden, not freed: the row stays on the GPU, so showing it again is one more write.
    /// Hide or show object `row`.
    pub fn set_hidden(&mut self, row: u32, on: bool) {
        self.objects
            .set_flag(&self.ctx, row, Instance::FLAG_HIDDEN, on);
        self.splat.invalidate(); // register:clouds
    }
}
// --8<-- [end:row-edits]
// --8<-- [end:03-rows]

// --8<-- [start:04a-editable-rows]
// --8<-- [start:editable-rows]
impl Gpu {
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

    /// Overwrite one object's rows in every editable lane, from the rows `at`.
    pub(crate) fn write_rows(&mut self, at: patch::Counts, up: &Upload) {
        self.arena.patch(&self.ctx, at, &up.arena);
        self.segments.patch(&self.ctx, at, &up.seg); // register:strokes
        self.glyphs.patch(&self.ctx, at, &up.glyph); // register:markers

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
            LaneId::Pipes => self.segments.kill(&self.ctx, lane, first, count, sink), // register:strokes
            LaneId::Ribbons => self.segments.kill(&self.ctx, lane, first, count, sink), // register:strokes
            LaneId::Spheres => self.glyphs.kill(&self.ctx, true, first, count, sink), // register:markers
            LaneId::Dots => self.glyphs.kill(&self.ctx, false, first, count, sink), // register:markers
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
        self.segments.release_editable(ctx, layouts); // register:strokes
        self.glyphs.release(ctx, layouts); // register:markers

        for lane in &mut self.registered {
            lane.on_release(ctx, layouts);
        }

        self.dead = patch::Counts::default(); // register:patch
        self.objects.geometry_changed();
        self.rebind_ink(); // register:ink
    }
}
// --8<-- [end:editable-rows]
// --8<-- [end:04a-editable-rows]

// --8<-- [start:04b-ink]
// --8<-- [start:ink]
// --8<-- [start:stroke-rows]
impl Gpu {
    /// Pipe rows still drawn.
    pub(crate) fn live_pipes(&self) -> u32 {
        self.segments.pipe_count().saturating_sub(self.dead.pipes)
    }

    /// Ribbon rows still drawn, sheet segments included.
    pub(crate) fn live_ribbons(&self) -> u32 {
        self.segments
            .ribbon_count()
            .saturating_sub(self.dead.ribbons)
    }
}
// --8<-- [end:stroke-rows]

// --8<-- [start:rebind-ink]
impl Gpu {
    /// Rebuild the ink bind group after targets or tiles moved.
    fn rebind_ink(&mut self) {
        self.objects.rebind_ink(
            &self.ctx,
            &self.layouts,
            &InkScene {
                targets: &self.targets,
            },
        );
    }
}
// --8<-- [end:rebind-ink]
// --8<-- [end:ink]
// --8<-- [end:04b-ink]

// --8<-- [start:04c-markers]
// --8<-- [start:markers]
// --8<-- [start:marker-rows]
impl Gpu {
    /// Marker rows still drawn.
    pub(crate) fn live_spheres(&self) -> u32 {
        self.glyphs.sphere_count().saturating_sub(self.dead.spheres)
    }

    /// Dot rows still drawn.
    pub(crate) fn live_dots(&self) -> u32 {
        self.glyphs.dot_count().saturating_sub(self.dead.dots)
    }
}
// --8<-- [end:marker-rows]
// --8<-- [end:markers]
// --8<-- [end:04c-markers]

// --8<-- [start:04d-clouds]
// --8<-- [start:clouds]
// --8<-- [start:cloud-rows]
impl Gpu {
    /// Append one upload's point clouds.
    fn append_cloud(&mut self, up: &Upload) {
        // a moved cloud buffer needs a new bind group
        if self.cloud.append(&self.ctx, &up.cloud) {
            self.splat
                .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        }

        self.splat.invalidate();
    }

    /// Log what the scene holds after an upload.
    fn log_scene(&self) {
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
    }

    /// Bind the cloud buffers again after they moved or were freed.
    fn rebind_cloud(&mut self) {
        self.splat
            .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
    }

    /// Cloud points still drawn.
    pub(crate) fn live_points(&self) -> u32 {
        self.cloud.point_count.saturating_sub(self.dead_points)
    }
}
// --8<-- [end:cloud-rows]
// --8<-- [end:clouds]
// --8<-- [end:04d-clouds]
