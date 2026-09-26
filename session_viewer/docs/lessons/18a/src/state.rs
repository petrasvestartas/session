// --8<-- [start:001-state]
use crate::app::scene::Scene; // register:scene
use crate::app::scene::FileDoc; // register:scene
use crate::app::selection::{ControlId, Controls, SelectionMode}; // register:selection
use crate::app::walk::encode::FACING_UNKNOWN; // register:selection
use crate::camera::Camera; // register:camera
use crate::engine::gpu::glyphs::GlyphRows; // register:controls
use crate::engine::gpu::pick::PickMode; // register:pick
use crate::engine::gpu::segments::SegRows; // register:controls
use crate::engine::gpu::{CylinderSegment, GlyphPoint}; // register:controls
// --8<-- [start:004-use-frame]
use crate::engine::gpu::FrameInput; // register:render
// --8<-- [end:004-use-frame]
// --8<-- [start:002-use-gpu]
use crate::engine::gpu::Gpu; // register:gpu
// --8<-- [end:002-use-gpu]
use crate::engine::gpu::Pick; // register:pick
// --8<-- [start:002-use-clock]
use crate::engine::performance::now_ms; // register:gpu
// --8<-- [end:002-use-clock]
use crate::engine::performance::heap_mb; // register:scene
mod cloud_query; // register:cloud_query
mod features; // register:features
mod text; // register:text
use features::Features; // register:features
use std::sync::Arc;
use winit::window::Window;


/// Everything the viewer holds: window, GPU, camera, scene, selection.
pub struct State {
    pub window: Arc<Window>,                // the winit window on the canvas
// --8<-- [start:002-field]
    pub gpu: Gpu,                           // device, buffers, pipelines; register:gpu
// --8<-- [end:002-field]
    pub camera: Camera,                     // the view; register:camera
    load_camera: crate::camera::CameraPose, // the view before loading started; register:fit
    pub scene: Scene,                       // the loaded documents; register:scene
    pub needs_frame: bool,                  // draw again on the next redraw
    pub interacting: bool,                  // a drag or pinch is in progress; register:input
    dirty: bool,                            // the picture changed
    last_frame_ms: f64,                     // when the last frame was drawn
    last_resize_ms: f64,                    // when the last resize was applied
    pub selection: SelectionMode,           // object, edge, face or control points; register:selection
    pub selection_tool: crate::app::selection::SelectionTool, // what a click selects; register:selection
    controls: Controls,                     // control points of the selected object; register:controls
    requested: PickMode,                    // what the pending pick looks for; register:pick
    pub(crate) additive_selection: bool,    // Shift held: add to the selection; register:selection
    selection_order: Vec<u32>,              // selected rows in pick order; register:selection
    highlighted: Vec<u32>,                  // rows highlighted, when several are selected; register:selection
    pub selection_radius_css: f64,          // click tolerance in CSS pixels; register:pick
    show_selected_names: bool,              // name label on the selection, T toggles; register:scene_text
    pub(crate) features: Features,          // what each feature keeps, features.rs; register:features
}
impl State {
    /// Open the GPU and upload the scene.
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
// --8<-- [start:002-open]
        let t0 = now_ms(); // register:gpu
        let mut gpu = Gpu::new(window.clone()).await?; // register:gpu
// --8<-- [end:002-open]
        let mut scene = Scene::new(); // register:scene
        scene.upload_to(&mut gpu); // the scene rows go to the GPU once; register:scene
// --8<-- [start:002-log]
        log::info!("gpu init {:.0} ms", now_ms() - t0); // register:gpu
// --8<-- [end:002-log]
        let camera = Camera::new(); // register:camera
        Ok(Self {
            window,
// --8<-- [start:002-init]
            gpu, // register:gpu
// --8<-- [end:002-init]
            load_camera: camera.pose(), // register:fit
            camera,                     // register:camera
            scene,                      // register:scene
            needs_frame: true,
            interacting: false, // register:input
            dirty: true,
            last_frame_ms: 0.0,
            last_resize_ms: f64::NEG_INFINITY,
            selection: SelectionMode::Object, // register:selection
            selection_tool: crate::app::selection::SelectionTool::default(), // register:selection
            controls: Controls::default(),    // register:controls
            requested: PickMode::Object,      // register:pick
            additive_selection: false,        // register:selection
            selection_order: Vec::new(),      // register:selection
            highlighted: Vec::new(),          // register:selection
            selection_radius_css: 6.0,        // register:pick
            show_selected_names: true,        // register:scene_text
            features: Features::default(),    // register:features
        })
    }

// --8<-- [start:002-size]
    /// Width over height of the canvas.
    pub fn aspect(&self) -> f64 { // register:gpu
        self.gpu.config.width.max(1) as f64 / self.gpu.config.height.max(1) as f64
    }

    /// Canvas size in device pixels.
    pub fn viewport(&self) -> (f64, f64) { // register:gpu
        (self.gpu.config.width as f64, self.gpu.config.height as f64)
    }
// --8<-- [end:002-size]

    /// Something changed: drop pending picks, draw again.
    pub fn touch(&mut self) {
        self.cancel_cloud_query(); // register:cloud_query
        self.gpu.pick.cancel(); // register:pick
        self.dirty = true;
        self.needs_frame = true;
    }
}
// --8<-- [end:001-state]

// --8<-- [start:004-render]
/// Background color.
const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

/// Orbit step per frame in `?spin=1` mode.
const SPIN_STEP: f32 = 0.004; // register:spin

impl State {
    /// Draw one frame; a still scene asks for no more.
    pub fn render(&mut self) {
        self.before_picks(); // register:features
        self.follow_logical_size();

        if self.gpu_failed() {
            return;
        }

        self.take_picks(); // register:pick
        self.needs_frame = false;
        self.after_picks(); // register:features
        self.spin(); // register:spin
        let now_ms = now_ms();
        let rebase = self.rebase(now_ms); // register:anchor
        let input = FrameInput {
            view_proj: self.camera.view_proj_anchored(self.aspect(), &rebase.anchor), // register:anchor
            clear: CLEAR,
            now_ms,
        };
        // redraw reasons
        self.dirty |= rebase.moved; // register:anchor
        self.dirty |= self.gpu.view.perf; // register:perf
        self.dirty |= self.gpu.view.spin; // register:spin
        self.dirty |= self.gpu.performance.rough() && !self.interacting; // register:perf

        let mut dropped = false;
        // starts false; a later feature ORs in its own reason on a line of its own, so this line never changes
        let mut waiting = false;
        waiting |= self.cloud_query_awaiting_gpu(); // a point-cloud query waits for its answer; register:cloud_query

        if self.dirty && !waiting {
            let gap = now_ms - self.last_frame_ms; // time since the last frame
            self.last_frame_ms = now_ms;
            self.gpu.performance.interacting = self.interacting; // register:perf
            let drawn = self.gpu.present(&input); // encode time, None when the frame was dropped
            self.reduce_if_slow(); // register:perf
            dropped = drawn.is_none() && self.gpu.surface.is_some(); // try again next frame
            self.dirty = dropped;
            self.perf_frame(gap, drawn); // register:perf
        }

        self.pick_frame(dropped, &input); // register:pick
        // reasons to draw again; a drag frame is redrawn in full once the drag ends
        self.needs_frame |= dropped;
        self.needs_frame |= rebase.pending; // register:anchor
        self.needs_frame |= self.gpu.pick.busy(); // register:pick
        self.needs_frame |= self.gpu.view.perf; // register:perf
        self.needs_frame |= self.gpu.view.spin; // register:spin
        self.needs_frame |= self.gpu.performance.rough(); // register:perf
        #[cfg(target_arch = "wasm32")] // register:inspection
        crate::app::inspection::publish(self); // register:inspection
    }

    /// A GPU error from the last frame stops drawing; a lost device reloads the page.
    fn gpu_failed(&mut self) -> bool {
        let failure = match self.gpu.failure.lock() {
            Ok(failure) => failure.clone(),
            Err(_) => None,
        };
        let Some(message) = failure else {
            return false;
        };

        #[cfg(target_arch = "wasm32")] // register:recovery
        if crate::app::route::recover_from_device_loss(&message) { // register:recovery
            self.needs_frame = false;
            return true;
        }

        crate::app::feedback::error(&message);
        self.cancel_cloud_query(); // register:cloud_query
        self.gpu.pick.cancel(); // register:pick
        self.needs_frame = false;
        true
    }
}
// --8<-- [end:004-render]

// --8<-- [start:04a-tail]
impl State {
    /// Add one loaded document to the scene.
    pub fn append(&mut self, doc: FileDoc, source: Option<String>) {
        let t0 = now_ms();
        let first_row = self.scene.row_count(); // rows before this document
        let index = self.scene.docs.len();
        crate::app::fonts::need_names(&doc.name, &doc.session); // register:loading
        self.scene.add_file(doc);
        let t1 = now_ms();
        // only the new rows go to the GPU
        self.scene.upload_to(&mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.annotate_document(first_row); // register:scene_text
        self.release_display_only(index, first_row, source); // register:release

        self.update_label(); // register:scene_text
        log::info!(
            "appended: walk {:.0} ms, upload {:.0} ms | {} docs | memory observation {:.0} MiB",
            t1 - t0,
            now_ms() - t1,
            self.scene.docs.len(),
            heap_mb()
        );
        self.touch();
    }

    /// Remove every document; camera and GPU stay.
    pub fn clear(&mut self) {
        self.load_camera = self.camera.pose(); // remember the view
        self.selection = SelectionMode::Object;
        self.gpu.arena.source_faces.select(&self.gpu.ctx, None);
        self.controls = Controls::default();
        self.scene.clear(&mut self.gpu);
        self.touch();
    }

    /// Fit the camera, unless the user already moved it.
    pub fn fit_loaded(&mut self) {
        // unchanged since loading started?
        if self.camera.pose() == self.load_camera {
            self.fit_all();
        }
    }

    /// Shrink the scene box to what is left, after objects were deleted or moved.
    pub(crate) fn refresh_bounds(&mut self) {
        if !self.scene.bounds_stale {
            return;
        }

        self.scene.bounds_stale = false;
        self.gpu.bounds = self.gpu.objects.live_world_bounds();
        self.include_text_bounds(); // register:scene_text
    }

    /// Fit the camera to everything loaded.
    pub fn fit_all(&mut self) {
        self.refresh_bounds();
        let b = &self.gpu.bounds;
        log::info!("fit: bounds {} aspect {:.3}", b.str(), self.aspect());
        self.camera.fit(&self.gpu.bounds, self.aspect());
        self.touch();
    }

    /// Fit the camera to the selection, or to everything.
    pub fn fit_selected_or_all(&mut self) {
        // the box of every selected row
        // `filter_map` keeps the rows that have a box and unwraps them in one step
        let mut bounds = self
            .selected_rows()
            .into_iter()
            .filter_map(|row| self.gpu.objects.row_bounds(row));
        let Some(mut b) = bounds.next() else {
            self.fit_all();
            return;
        };
        for next in bounds {
            b.union_with(&next);
        }
        log::info!(
            "fit selected: bounds {} aspect {:.3}",
            b.str(),
            self.aspect()
        );
        self.camera.fit(&b, self.aspect());
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
    }

    // A `const` inside `impl` is an associated constant, read as `Self::RESIZE_HOLD_MS`.
    /// Minimum time between two resizes.
    const RESIZE_HOLD_MS: f64 = 100.0;

    /// Resize the GPU targets; false when asked too soon after the last one.
    pub fn resize(&mut self, width: u32, height: u32) -> bool {
        let now = now_ms();

        // a window drag resizes every frame; wait between remakes
        if now - self.last_resize_ms < Self::RESIZE_HOLD_MS {
            return false;
        }

        self.last_resize_ms = now;
        self.gpu.resize(width, height);
        self.gpu.logical_size = self.logical_size();
        self.upload_controls(); // register:controls
        self.touch();
        true
    }

    /// Set the point size of clouds.
    pub fn set_cloud_size(&mut self, size: f32) {
        self.gpu.view.cloud_size = size.clamp(0.25, 8.0);
        self.touch();
    }

    /// P: switch between shaded faces and x-ray.
    pub fn toggle_xray(&mut self) {
        self.gpu.view.opacity = if self.gpu.view.opacity > 0.0 {
            0.0
        } else {
            1.0
        };
        self.touch();
    }

    /// Ask what is under a pixel; the answer comes in a later frame.
    pub fn request_pick(&mut self, x: u32, y: u32) {
        self.request_selection(x, y, false, false);
    }

    /// The picture changed: draw on the next redraw.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn request_frame(&mut self) {
        self.dirty = true;
        self.needs_frame = true;
    }


    /// Select one row, or nothing.
    pub fn select(&mut self, row: Option<u32>) {
        let row = row.filter(|row| self.scene.selectable(*row));

        for old in self.highlighted.drain(..) {
            self.gpu.set_selected(old, false);
        }

        // back to plain object mode, no edge, face or controls
        self.selection = SelectionMode::Object;
        self.gpu.arena.source_faces.select(&self.gpu.ctx, None);
        self.controls = Controls::default();
        self.gpu.controls.reset();
        self.gpu.control_net.reset();
        self.gpu.segments.set_edge(&self.gpu.ctx, None);
        self.gpu.splat.set_controls(None);

        if let Some(old) = self.scene.selected.take() {
            self.gpu.set_selected(old, false);
        }

        if let Some(r) = row {
            self.gpu.set_selected(r, true);
        }

        self.scene.selected = row;
        self.selection_order = row.into_iter().collect();
        self.update_label(); // register:scene_text
        self.touch();
    }

    /// Every selected row.
    pub(crate) fn selected_rows(&self) -> Vec<u32> {
        if self.highlighted.is_empty() {
            self.scene.selected.into_iter().collect()
        } else {
            self.highlighted.clone()
        }
    }

    /// The selected rows in the order they were picked.
    pub(crate) fn ordered_rows(&self) -> Vec<u32> {
        ordered(&self.selection_order, &self.selected_rows())
    }

    /// Select several rows, added to the selection or replacing it.
    pub(crate) fn select_rows(&mut self, rows: Vec<u32>, additive: bool) {
        let mut selected = if additive {
            self.ordered_rows()
        } else {
            Vec::new()
        };
        // skip rows that cannot be selected or are hidden
        selected.extend(rows.into_iter().filter(|r| {
            self.scene.selectable(*r)
                && self
                    .scene
                    .identity_of(*r)
                    .is_some_and(|id| !self.scene.hidden.contains(&id))
        }));
        let order = selected.clone();
        // `dedup` only drops neighbours that repeat, so sort first
        selected.sort_unstable();
        selected.dedup();
        self.select(None);
        self.selection_order = ordered(&order, &selected);
        self.scene.selected = selected.first().copied(); // the first is the main one
        for &row in &selected {
            self.gpu.set_selected(row, true);
        }
        if selected.len() > 1 {
            self.highlighted = selected;
        }
        self.update_label(); // register:scene_text
        self.touch();
    }

    /// Select what a viewport click on `row` reaches: its whole group, when it is in one.
    pub(crate) fn select_picked(&mut self, row: u32, additive: bool) {
        // CLICK_ROWS is a hook list in features.rs: an array of functions, one per feature; the first answer wins
        let rows = features::CLICK_ROWS
            .iter()
            .find_map(|widen| widen(self, row))
            .unwrap_or_else(|| vec![row]);
        self.select_rows(rows, additive);
    }

    /// T: show or hide the name label on the selection.
    pub fn toggle_selected_names(&mut self) {
        self.show_selected_names = !self.show_selected_names;
        self.update_label(); // register:scene_text
        self.touch();
    }

    /// H: hide the selection.
    pub fn hide_selected(&mut self) {
        // several rows selected
        if !self.highlighted.is_empty() {
            let rows = std::mem::take(&mut self.highlighted);

            for row in &rows {
                self.gpu.set_selected(*row, false);
            }

            return;
        }

        let Some(row) = self.scene.selected else {
            return;
        };
        let Some(guid) = self.scene.identity_of(row) else {
            return;
        };
        self.select(None);
        self.scene.hidden.insert(guid); // by id, so a new row of it stays hidden
        self.gpu.set_hidden(row, true);
        self.update_label(); // register:scene_text
        self.touch();
    }

    /// S: show everything hidden.
    pub fn show_all(&mut self) {
        for row in self.scene.hidden_rows() {
            self.gpu.set_hidden(row, false);
        }

        self.scene.hidden.clear();
        self.update_label(); // register:scene_text
        self.touch();
    }

    /// Apply a pick answer first, so this frame shows it.
    fn take_picks(&mut self) {
        if let Some(pick) = self.gpu.pick.poll() {
            self.apply_pick(pick);
        } else {
            self.cloud_query_lost(); // register:cloud_query
        }
    }

    /// A pending pick draws its own id frame.
    fn pick_frame(&mut self, dropped: bool, input: &FrameInput) {
        if !dropped && let Some(at) = self.gpu.pick.take_pending() {
            self.gpu.pick_frame(input, at);
        }
    }

    /// A pick answer arrived: select what it hit.
    fn apply_pick(&mut self, pick: Option<Pick>) {
        // a feature waiting for this answer takes it: a drag, a tool, a split, a point-cloud query
        for take in features::TAKE_PICK {
            if take(self, pick) {
                return;
            }
        }

        let pick = pick.filter(|pick| self.scene.selectable(pick.row));

        match self.requested {
            PickMode::Edge | PickMode::Component => {
                // an edge hit wins over a face hit
                if let Some(pick) = pick
                    && let Some(edge) = self.scene.edge_at(pick)
                {
                    self.select(Some(pick.row));
                    self.gpu.set_selected(pick.row, false);
                    self.selection.select_edge(pick.row, edge);
                    self.gpu
                        .segments
                        .set_edge(&self.gpu.ctx, Some((pick.row, edge)));
                    self.status(&format!("Edge {edge} selected"));
                    return;
                }

                self.pick_face(pick); // register:scene_text
                return;
            }
            PickMode::Controls { parent, cloud } => {
                if let Some(pick) = pick
                    && pick.row == parent
                {
                    self.apply_control(pick, cloud); // register:controls
                }

                return;
            }
            PickMode::Object => {}
        }

        // nothing hit: clear, unless Shift is adding
        let Some(p) = pick else {
            log::info!("pick: nothing");
            if !self.additive_selection {
                self.select(None);
            }
            return;
        };

        match self.scene.resolve(p, &self.gpu) {
            Some(hit) => {
                match &hit.point {
                    Some(pt) => log::info!(
                        "pick: '{}' {} row {} point {} id {} at ({:.1}, {:.1}, {:.1})",
                        hit.doc,
                        hit.guid,
                        hit.row,
                        pt.local,
                        pt.id,
                        pt.position[0],
                        pt.position[1],
                        pt.position[2]
                    ),
                    None => log::info!("pick: '{}' {} row {}", hit.doc, hit.guid, hit.row),
                }

                // a sheet entity, not an object
                if let Some(entity) = hit.entity {
                    return;
                }

                self.select_picked(hit.row, self.additive_selection);
            }
            None => log::info!("pick: row {} sub {} (no document)", p.row, p.sub),
        }
    }

    /// The CSS size changed: control dots keep their pixel size.
    fn follow_logical_size(&mut self) {
        let logical = self.logical_size();

        if logical != self.gpu.logical_size {
            self.gpu.logical_size = logical;
            self.upload_controls(); // register:controls
            self.touch();
        }
    }


    /// Canvas size in CSS pixels; device pixels natively.
    pub(crate) fn logical_size(&self) -> [f64; 2] {
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window()
            && let Some(document) = window.document()
            && let Some(canvas) = document.get_element_by_id("canvas")
        {
            return [
                f64::from(canvas.client_width().max(1)),
                f64::from(canvas.client_height().max(1)),
            ];
        }

        [
            f64::from(self.gpu.config.width),
            f64::from(self.gpu.config.height),
        ]
    }

    /// Ask what is under a pixel: an object, an edge (Ctrl) or a face (Ctrl+Shift).
    pub fn request_selection(&mut self, x: u32, y: u32, edge: bool, face: bool) {
        // a split or a tool wants a plain object
        let mut splitting = false;
        let face = !splitting
            && (face || self.selection_tool == crate::app::selection::SelectionTool::Face);
        let edge = !splitting
            && (edge || self.selection_tool == crate::app::selection::SelectionTool::Edge);
        self.cancel_cloud_query(); // register:cloud_query
        self.gpu.pick.cancel();

        // a point cloud answers by its own query
        let mut queried = false;
        queried |= self.start_cloud_pick(x, y, splitting || edge); // register:cloud_query

        if queried {
            return;
        }

        // which kind of pick the id frame runs
        let mode = if splitting {
            PickMode::Object
        } else if face {
            PickMode::Component
        } else if edge {
            PickMode::Edge
        } else {
            match self.selection {
                SelectionMode::Controls { parent, cloud, .. } => {
                    PickMode::Controls { parent, cloud }
                }
                _ => PickMode::Object,
            }
        };
        self.requested = mode;
        let logical = self.logical_size();
        let scale = f64::from(self.gpu.config.width) / logical[0]; // device pixels per CSS pixel
        self.gpu
            .pick
            .configure(mode, self.selection_radius_css, scale);
        self.gpu.pick.request(x, y);
        self.needs_frame = true;
    }

    /// Esc: the first cancels a command and keeps the selection, the next one clears it.
    pub fn escape(&mut self) {
        let mut cancelled = false;

        if !cancelled {
            self.escape_selection();
        }
    }

    /// Esc: leave control points, keep the object selected.
    pub fn escape_selection(&mut self) {
        let parent = self.selection.escape();
        self.select(parent);
        self.status("");
    }

    /// Show a message in the status line.
    fn status(&self, message: &str) {
        crate::app::feedback::status(message);
    }

    /// The `?perf=1` line after a drawn frame.
    fn perf_frame(&self, gap: f64, drawn: Option<f64>) {
        if let (true, Some(encode_ms)) = (self.gpu.view.perf, drawn) {
            self.perf_line(gap, encode_ms);
        }
    }

    /// Slow even at the top drag tier: drop to device scale 1 and no antialiasing.
    fn reduce_if_slow(&mut self) {
        if self.gpu.performance.take_slow_interaction()
            && (crate::engine::gpu::view::device_pixel_ratio() > 1.0
                || self.gpu.targets.samples > 1)
        {
            crate::engine::gpu::view::reduce();
            self.gpu
                .resize(self.gpu.config.width, self.gpu.config.height);
            log::warn!("slow interaction frames; rendering at device scale 1 without antialiasing");
            self.status("Slow frames: rendering at device scale 1 without antialiasing");
        }
    }

    /// The `?perf=1` line: frame number, gap, encode time, heap.
    #[cfg(target_arch = "wasm32")]
    fn perf_line(&self, gap_ms: f64, encode_ms: f64) {
        let line = format!(
            "f{} gap {gap_ms:.0} enc {encode_ms:.1} ms wasm capacity {:.0} MiB",
            self.gpu.performance.frames,
            heap_mb()
        );
        crate::engine::performance::perf_line(&line);
    }

    // Two versions of one function, chosen by `#[cfg]`: natively it does nothing.
    /// Natively the perf line goes nowhere.
    #[cfg(not(target_arch = "wasm32"))]
    fn perf_line(&self, _gap_ms: f64, _encode_ms: f64) {}
}

// A second `impl State` block, compiled only for the browser build.
#[cfg(target_arch = "wasm32")]
impl State {
    /// The control points as JSON, for the inspection tests.
    pub fn inspected_controls(&self) -> Vec<serde_json::Value> {
        let mut points = Vec::with_capacity(self.controls.points.len());

        for point in &self.controls.points {
            points.push(serde_json::json!({"id": point.id, "position": point.position}));
        }

        points
    }
}

/// A position as the f32 the GPU takes.
pub(crate) fn render_position(position: [f64; 3]) -> [f32; 3] {
    [position[0] as f32, position[1] as f32, position[2] as f32]
}

/// `selected` in the pick order `order`, first picks first; rows the order misses come last.
fn ordered(order: &[u32], selected: &[u32]) -> Vec<u32> {
    let mut left: std::collections::HashSet<u32> = selected.iter().copied().collect();
    let mut rows: Vec<u32> = Vec::with_capacity(selected.len());

    for row in order.iter().chain(selected) {
        if left.remove(row) {
            rows.push(*row);
        }
    }

    rows
}

#[cfg(test)]
mod order_tests {
    use super::ordered;

    /// Picks keep their order; a dropped row goes, an unknown one comes last.
    #[test]
    fn the_selection_keeps_pick_order() {
        assert_eq!(ordered(&[9, 2, 5], &[2, 5, 9]), vec![9, 2, 5]);
        assert_eq!(ordered(&[9, 2, 5], &[2, 9]), vec![9, 2]);
        assert_eq!(ordered(&[9, 2], &[1, 2, 9]), vec![9, 2, 1]);
        assert_eq!(ordered(&[3, 3, 1], &[1, 3]), vec![3, 1]);
    }
}

// Control point = a point that shapes a curve, surface or mesh; moving it reshapes the geometry.
// Control net = the thin lines joining neighbouring control points.
impl State {
    /// F10: show the control points of the selected object.
    pub fn enable_controls(&mut self) {
        let Some(parent) = self.scene.selected else {
            self.status("Select one object before pressing F10");
            return;
        };

        // already on
        // `matches!` tests a pattern, here with an `if` guard, and returns a bool
        if matches!(self.selection, SelectionMode::Controls { parent: active, .. } if active == parent)
        {
            return;
        }
        // a released document comes back first
        let mut released = false;

        if released {
            return;
        }

        // the points come from the source geometry
        let controls = match self.scene.geometry(parent) {
            Some(geometry) => Controls::from_geometry(geometry),
            None if self.streamed_slot(parent).is_some() => Controls::cloud(), // register:cloud_query
            None => {
                self.status("Source controls are unavailable for this display-only object");
                return;
            }
        };

        if !controls.cloud && controls.points.is_empty() {
            self.status("This object has no selectable source controls");
            return;
        }

        self.selection.enable_controls(Some(parent), controls.cloud);
        self.gpu.arena.source_faces.select(&self.gpu.ctx, None);
        self.gpu.segments.set_edge(&self.gpu.ctx, None);
        self.gpu.set_selected(parent, false);
        // `then_some` turns true into Some(parent) and false into None
        self.gpu
            .splat
            .set_controls(controls.cloud.then_some(parent));
        self.controls = controls;
        self.upload_controls(); // register:controls
        self.update_label(); // register:scene_text
        self.status("Control points: click to select; Esc to leave");
        self.touch();
    }

    /// Send the control dots and their links to the GPU.
    pub(crate) fn upload_controls(&mut self) {
        // start empty, so nothing doubles
        self.gpu.controls.reset();
        self.gpu.control_net.reset();
        let SelectionMode::Controls {
            parent, selected, ..
        } = self.selection
        else {
            return;
        };
        let scale = f64::from(self.gpu.config.width) / self.logical_size()[0]; // device pixels per CSS pixel
        let mut glyphs = GlyphRows::default();

        // one dot per control point
        for control in &self.controls.points {
            let color = if Some(control.id) == selected {
                [1.0, 1.0, 0.0, 1.0] // yellow: selected
            } else {
                [0.15, 0.35, 0.9, 1.0] // blue
            };
            glyphs.dots.push(GlyphPoint {
                center: render_position(control.position),
                radius: -3.5 * scale as f32, // negative: a pixel size, not a world size
                color,
                instance_id: parent,
                facing: FACING_UNKNOWN,          // no face orientation
                facing_ext: [FACING_UNKNOWN; 2], // no neighbour orientation
            });
        }

        let mut segments = SegRows::default();

        // one thin line per link between control points
        for &[start, end] in &self.controls.links {
            segments.ribbons.push(CylinderSegment {
                p0: render_position(self.controls.points[start].position),
                p1: render_position(self.controls.points[end].position),
                radius: 0.0,
                color: 0xffcc8866,
                instance_id: parent,
                facing: FACING_UNKNOWN, // no face orientation
            });
        }

        self.gpu
            .controls
            .append(&self.gpu.ctx, &self.gpu.layouts, &glyphs);
        self.gpu
            .control_net
            .append(&self.gpu.ctx, &self.gpu.layouts, &segments);
    }

    /// A control point was clicked: select it.
    fn apply_control(&mut self, pick: Pick, cloud: bool) {
        let SelectionMode::Controls { parent, .. } = self.selection else {
            return;
        };

        if pick.row != parent {
            return;
        }

        let id = if cloud {
            // a cloud point: find its source id
            let Some((owner, local)) = self.gpu.cloud.row_of(pick.sub) else {
                return;
            };

            if owner != parent {
                return;
            }

            self.gpu.splat.set_point(Some(pick.sub));
            let Some(source) = self.scene.point_at(parent, local) else {
                self.status("Source point ID unavailable; no local display ID was substituted");
                return;
            };
            ControlId::Point(source.id)
        } else {
            // a control dot: top two bits 01, index in the rest
            if pick.sub & 0xc000_0000 != 0x4000_0000 {
                return;
            }

            let Some(control) = self.controls.points.get((pick.sub & 0x3fff_ffff) as usize) else {
                return;
            };
            control.id
        };
        self.selection = SelectionMode::Controls {
            parent,
            selected: Some(id),
            cloud,
        };
        self.upload_controls(); // register:controls
        self.status(&format!("Selected {id:?}"));
        self.touch();
    }
}

use crate::app::scene::StreamedInit;
use crate::app::walk::cloud::StreamRows;

impl State {
    /// Start a streamed point cloud; returns its slot.
    pub fn add_streamed(&mut self, init: StreamedInit) -> usize {
        let idx = self.scene.add_streamed_cloud(init, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
        idx
    }

    /// Add more points to streamed cloud `idx`.
    pub fn extend_streamed(&mut self, idx: usize, rows: StreamRows, to: u32) {
        self.scene
            .extend_streamed_cloud(idx, rows, to, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        log::info!(
            "cloud slice: {to} points resident | heap {:.0} MB",
            heap_mb()
        );
        self.touch();
    }
}

impl State {
    /// A display-only document keeps its rows and tree, not its objects.
    pub(super) fn release_display_only(
        &mut self,
        index: usize,
        first_row: usize,
        source: Option<String>,
    ) {
        if let Some(url) = source {
            self.scene.release(index, first_row as u32, url);
        }
    }
}

impl State {
    /// A face pick in Component mode: select the face.
    fn pick_face(&mut self, pick: Option<Pick>) {
        if self.requested == PickMode::Component
            && let Some(pick) = pick
            && let Some((address, source)) = self.gpu.arena.source_faces.source(pick.row, pick.sub)
        {
            self.select(Some(source.parent));
            self.gpu.set_selected(source.parent, false);
            self.gpu
                .pass_mut::<crate::engine::gpu::surface_outline::Outline>()
                .selection
                .set_selected(source.parent, true);
            self.selection = SelectionMode::Face {
                parent: source.parent,
                face: source.face,
            };
            self.gpu
                .arena
                .source_faces
                .select(&self.gpu.ctx, Some(address));
            self.status(&format!("Face {} selected", source.face));
        }
    }
}

use crate::app::scene::SheetInit;
use crate::app::walk::sheet::SheetRows;

impl State {
    /// Start a streamed sheet; returns its slot.
    pub fn add_sheet(&mut self, init: SheetInit) -> usize {
        let idx = self.scene.add_sheet(init, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
        idx
    }

    /// Add more segments to sheet `idx`.
    pub fn extend_sheet(&mut self, idx: usize, rows: SheetRows, to: u32) {
        self.scene.extend_sheet(idx, rows, to, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        log::info!(
            "sheet slice: {to} segments resident | heap {:.0} MB",
            heap_mb()
        );
        self.touch();
    }
}

impl State {
    /// Element Features On|Off: draw the features inside each element; `None` toggles.
    pub fn show_attributes(&mut self, value: Option<bool>) -> bool {
        let show = value.unwrap_or(!self.scene.attributes);
        self.scene.attributes = show;
        self.select(None);

        if !self.scene.rewalk_editable(&mut self.gpu) {
            self.resume_after(hydrate::Resume::Rewalk);
        }

        self.place_gizmo(None);
        self.refresh_layers();
        self.update_label();
        self.touch();
        show
    }
}

impl State {
    /// Make elements slightly see-through the first time they arrive.
    fn dim_elements(&mut self, first_row: usize) {
        // an opacity was already chosen
        if self.features.opacity_chosen || self.gpu.view.opacity < 1.0 {
            return;
        }

        // does the new document have elements?
        let elements = (first_row..self.scene.row_count()).any(|row| {
            let row = row as u32;
            matches!(
                self.scene
                    .geometry(row)
                    .or_else(|| self.scene.instance_definition(row)),
                Some(session_rust::Geometry::Element(_))
            )
        });

        if elements {
            self.gpu.view.opacity = ELEMENT_OPACITY;
            self.features.opacity_chosen = true;
        }
    }

    /// Opacity <value>: 0 is x-ray, 1 is solid.
    pub fn set_opacity(&mut self, value: f32) {
        self.gpu.view.opacity = value.clamp(0.0, 1.0);
        self.features.opacity_chosen = true;
        self.touch();
    }
}

impl State {
    /// `?spin=1`: orbit a little every frame.
    fn spin(&mut self) {
        if self.gpu.view.spin {
            self.cancel_cloud_query(); // register:cloud_query
            self.camera.orbit(SPIN_STEP, 0.0);
        }
    }
}

impl State {
    /// The point GPU rows are measured from, moved when the camera drifted far.
    fn rebase(&mut self, now_ms: f64) -> crate::engine::gpu::Rebase {
        self.camera.grow_extent(&self.gpu.bounds);
        let origin = self.camera.origin();
        self.gpu
            .rebase_anchor(&origin, self.camera.distance_world(), now_ms)
    }
}
// --8<-- [end:04a-tail]
