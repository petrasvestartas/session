use crate::app::scene::{FileDoc, Scene, SheetInit, StreamedInit};
use crate::app::selection::{ControlId, Controls, SelectionMode};
use crate::app::walk::cloud::StreamRows;
use crate::app::walk::encode::FACING_UNKNOWN;
use crate::app::walk::sheet::SheetRows;
use crate::camera::Camera;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::pick::PickMode;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use crate::engine::gpu::{FrameInput, Gpu, Pick};
use crate::engine::performance::{heap_mb, now_ms};
mod cloud_query;
// --8<-- [start:step-38a]
mod drawing;
// --8<-- [end:step-38a]
pub mod edit;
mod panel;
mod sheet_query;
mod splitting;
mod text;
use std::sync::Arc;
use winit::window::Window;

/// Background color.
const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

/// Orbit step per frame in `?spin=1` mode.
const SPIN_STEP: f32 = 0.004;

/// Everything the viewer holds: window, GPU, camera, scene, selection.
pub struct State {
    pub window: Arc<Window>,                                // the winit window on the canvas
    pub gpu: Gpu,                                           // device, buffers, pipelines
    pub camera: Camera,                                     // the view
    pub scene: Scene,                                       // the loaded documents
    pub needs_frame: bool,                                  // draw again on the next redraw
    pub interacting: bool,                                  // a drag or pinch is in progress
    dirty: bool,                                            // the picture changed
    last_frame_ms: f64,                                     // when the last frame was drawn
    last_resize_ms: f64,                                    // when the last resize was applied
    pub selection: SelectionMode,                           // object, edge, face or control points
    pub selection_tool: crate::app::selection::SelectionTool, // what a click selects
    hierarchy: crate::app::hierarchy::Hierarchy,
    pending_split: Option<splitting::Pending>,              // a split waiting for its cutter
    // --8<-- [start:step-38b]
    pub(crate) draft: Option<drawing::Draft>,               // a shape being drawn
    pub(crate) snap_enabled: bool,
    controls: Controls,                                     // control points of the selected object
    requested: PickMode,                                    // what the pending pick looks for
    pub(crate) additive_selection: bool,                    // Shift held: add to the selection
    // --8<-- [end:step-38b]
    pub selection_radius_css: f64,                          // click tolerance in CSS pixels
    show_selected_names: bool,                              // name label on the selection, T toggles
    cloud_query: Option<crate::app::cloud_query::Query>,    // a point-cloud pick in flight
    #[cfg(target_arch = "wasm32")]
    query_generation: u64,                                  // counts cloud queries, old answers dropped
    sheet_query: Option<crate::app::sheet_query::Query>,    // a sheet pick in flight
    sheet_generation: u64,                                  // counts sheet queries, old answers dropped
    pub gizmo: Option<crate::app::gizmo::Gizmo>,            // the move/rotate/scale widget
    dragging: Option<edit::GizmoDrag>,                      // a gizmo drag in progress
    control_drag: Option<edit::ControlDrag>,                // a control point drag in progress
}

impl State {
    /// Open the GPU and upload the scene.
    pub async fn new(window: Arc<Window>, mut scene: Scene) -> anyhow::Result<Self> {
        let t0 = now_ms();
        let mut gpu = Gpu::new(window.clone()).await?;
        // the scene rows go to the GPU once
        scene.upload_to(&mut gpu);
        log::info!("gpu init {:.0} ms", now_ms() - t0);
        Ok(Self {
            window,
            gpu,
            camera: Camera::new(), // the default view
            scene,
            needs_frame: true,
            interacting: false,
            dirty: true,
            last_frame_ms: 0.0,
            last_resize_ms: f64::NEG_INFINITY,
            selection: SelectionMode::Object,
            selection_tool: crate::app::selection::SelectionTool::default(),
            hierarchy: Default::default(),
            pending_split: None,
            // --8<-- [start:step-38c]
            draft: None,
            snap_enabled: true,
            controls: Controls::default(),
            requested: PickMode::Object,
            additive_selection: false,
            // --8<-- [end:step-38c]
            selection_radius_css: 6.0,
            show_selected_names: true,
            cloud_query: None,
            #[cfg(target_arch = "wasm32")]
            query_generation: 0,
            sheet_query: None,
            sheet_generation: 0,
            gizmo: None,
            dragging: None,
            control_drag: None,
        })
    }

    /// Width over height of the canvas.
    pub fn aspect(&self) -> f64 {
        self.gpu.config.width.max(1) as f64 / self.gpu.config.height.max(1) as f64
    }

    /// Canvas size in device pixels.
    pub fn viewport(&self) -> (f64, f64) {
        (self.gpu.config.width as f64, self.gpu.config.height as f64)
    }

    /// Add one loaded document to the scene.
    pub fn append(&mut self, doc: FileDoc) {
        let t0 = now_ms();
        let first_row = self.scene.object_count(); // rows before this document
        self.scene.add_file(doc);
        let t1 = now_ms();
        // only the new rows go to the GPU
        self.scene.upload_to(&mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.annotate_document(first_row);
        // the layer panel lists the new rows
        self.refresh_layers();
        self.update_label();
        log::info!(
            "appended: walk {:.0} ms, upload {:.0} ms | {} docs | memory observation {:.0} MiB",
            t1 - t0,
            now_ms() - t1,
            self.scene.docs.len(),
            heap_mb()
        );
        self.touch();
    }

    /// Replace the scene's text labels.
    pub fn set_texts(&mut self, texts: Vec<crate::app::manifest::TextItem>) {
        self.scene.set_texts(texts, &mut self.gpu);
        self.update_label();
        self.touch();
    }

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

    /// Remove every document; camera and GPU stay.
    pub fn clear(&mut self) {
        self.cancel_split();
        self.cancel_gesture();
        self.hierarchy = Default::default();
        self.selection = SelectionMode::Object;
        self.sheet_query = None;
        self.gpu.arena.source_faces.select(&self.gpu.ctx, None);
        self.controls = Controls::default();
        self.scene.clear(&mut self.gpu);
        self.place_gizmo(None);
        self.refresh_layers();
        self.touch();
    }

    /// Fit the camera to everything loaded.
    pub fn fit_all(&mut self) {
        let b = &self.gpu.bounds;
        log::info!("fit: bounds {} aspect {:.3}", b.str(), self.aspect());
        self.camera.fit(&self.gpu.bounds, self.aspect());
        self.touch();
    }

    // --8<-- [start:step-38d]
    /// Fit the camera to the selection, or to everything.
    pub fn fit_selected_or_all(&mut self) {
        // the box of every selected row
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
        // --8<-- [end:step-38d]
        log::info!(
            "fit selected: bounds {} aspect {:.3}",
            b.str(),
            self.aspect()
        );
        self.camera.fit(&b, self.aspect());
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
    }

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
        self.upload_controls();
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

    /// Something changed: drop pending picks, draw again.
    pub fn touch(&mut self) {
        self.cancel_cloud_query();
        self.gpu.pick.cancel();
        self.dirty = true;
        self.needs_frame = true;
    }

    /// Select one row, or nothing.
    pub fn select(&mut self, row: Option<u32>) {
        self.cancel_split();
        let row = row.filter(|row| self.scene.selectable(*row));
        self.cancel_gesture();

        // unhighlight the old selection
        for old in self.hierarchy.selected.drain(..) {
            self.gpu.set_selected(old, false);
        }

        // back to plain object mode, no edge, face or controls
        self.selection = SelectionMode::Object;
        self.sheet_query = None;
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
        // --8<-- [start:step-38e]
        self.refresh_layers();
        self.place_gizmo(row);
        self.update_label();
        self.touch();
    }

    /// Every selected row.
    pub(crate) fn selected_rows(&self) -> Vec<u32> {
        if self.hierarchy.selected.is_empty() {
            self.scene.selected.into_iter().collect()
        } else {
            self.hierarchy.selected.clone()
        }
    }

    /// Select several rows, added to the selection or replacing it.
    pub(crate) fn select_rows(&mut self, rows: Vec<u32>, additive: bool) {
        let mut selected = if additive {
            self.selected_rows()
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
        selected.sort_unstable();
        selected.dedup();
        self.select(None);
        self.scene.selected = selected.first().copied(); // the first is the main one
        for &row in &selected {
            self.gpu.set_selected(row, true);
        }
        if selected.len() > 1 {
            self.hierarchy.selected = selected;
        }
        self.place_gizmo(self.scene.selected);
        self.refresh_layers();
        self.update_label();
        self.touch();
    }

    /// T: show or hide the name label on the selection.
// --8<-- [end:step-38e]
    pub fn toggle_selected_names(&mut self) {
        self.show_selected_names = !self.show_selected_names;
        self.update_label();
        self.touch();
    }

    /// H: hide the selection.
    pub fn hide_selected(&mut self) {
        // several rows selected
        if !self.hierarchy.selected.is_empty() {
            let rows = std::mem::take(&mut self.hierarchy.selected);

            for row in &rows {
                self.gpu.set_selected(*row, false);
            }

            self.set_rows_hidden(&rows, true);
            return;
        }

        let Some(row) = self.scene.selected else {
            return;
        };
        let Some(guid) = self.scene.identity_of(row) else {
            return;
        };
        self.select(None);
        self.scene.hidden.insert(guid); // by id, so a rebuild keeps it hidden
        self.gpu.set_hidden(row, true);
        self.refresh_layers();
        self.update_label();
        self.touch();
    }

    /// S: show everything hidden.
    pub fn show_all(&mut self) {
        for row in self.scene.hidden_rows() {
            self.gpu.set_hidden(row, false);
        }

        self.scene.hidden.clear();
        self.refresh_layers();
        self.update_label();
        self.touch();
    }

    /// A pick answer arrived: select what it hit.
    fn apply_pick(&mut self, pick: Option<Pick>) {
        // a split is waiting for its cutter
        if self.pending_split.is_some() {
            if let Some(pick) = pick {
                self.pick_split_cutter(pick.row);
            }

            return;
        }

        let pick = pick.filter(|pick| self.scene.selectable(pick.row));

        // a point-cloud query takes the answer
        #[cfg(target_arch = "wasm32")]
        if self.cloud_query_awaiting_gpu() {
            self.apply_cloud_query_pick(pick);
            return;
        }

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
                    self.place_gizmo(Some(pick.row));
                    self.status(&format!("Edge {edge} selected"));
                } else if self.requested == PickMode::Component
                    && let Some(pick) = pick
                    && let Some((address, source)) =
                        self.gpu.arena.source_faces.source(pick.row, pick.sub)
                {
                    self.select(Some(source.parent));
                    self.gpu.set_selected(source.parent, false);
                    self.gpu.selection_outline.set_selected(source.parent, true);
                    self.selection = SelectionMode::Face {
                        parent: source.parent,
                        face: source.face,
                    };
                    self.gpu
                        .arena
                        .source_faces
                        .select(&self.gpu.ctx, Some(address));
                    self.place_gizmo(Some(source.parent));
                    self.status(&format!("Face {} selected", source.face));
                }
                return;
            }
            PickMode::Controls { parent, cloud } => {
                if let Some(pick) = pick
                    && pick.row == parent
                {
                    self.apply_control(pick, cloud);
                }

                return;
            }
            PickMode::Object => {}
        }

        // nothing hit: clear, unless Shift is adding
        let Some(p) = pick else {
            log::info!("pick: nothing");
            // --8<-- [start:step-38f]
            if !self.additive_selection {
                self.select(None);
            }
            // --8<-- [end:step-38f]
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
                    self.apply_sheet_pick(hit.row, entity);
                    return;
                }

                // --8<-- [start:step-38g]
                self.select_rows(vec![hit.row], self.additive_selection);
                // --8<-- [end:step-38g]
            }
            None => log::info!("pick: row {} sub {} (no document)", p.row, p.sub),
        }
    }

    /// Draw one frame; a still scene asks for no more.
    pub fn render(&mut self) {
        self.upload_gizmo();
        let logical = self.logical_size();

        // the CSS size changed: control dots keep their pixel size
        if logical != self.gpu.logical_size {
            self.gpu.logical_size = logical;
            self.upload_controls();
            self.touch();
        }

        // a GPU error from the last frame
        let failure = match self.gpu.failure.lock() {
            Ok(failure) => failure.clone(),
            Err(_) => None,
        };

        if let Some(message) = failure {
            // a lost device reloads the page
            #[cfg(target_arch = "wasm32")]
            if crate::app::route::recover_from_device_loss(&message) {
                self.needs_frame = false;
                return;
            }

            crate::app::feedback::error(&message);
            self.cancel_cloud_query();
            self.gpu.pick.cancel();
            self.needs_frame = false;
            return;
        }

        // apply a pick answer first, so this frame shows it
        if let Some(pick) = self.gpu.pick.poll() {
            self.apply_pick(pick);
        } else if self.cloud_query_awaiting_gpu() && !self.gpu.pick.busy() {
            self.cloud_query = None;
            self.gpu.pick.cancel();
            self.upload_controls();
            self.status("Point query failed during GPU readback; click to retry");
        }

        self.needs_frame = false;

        if self.gpu.view.spin {
            self.cancel_cloud_query();
            self.camera.orbit(SPIN_STEP, 0.0);
        }

        let now_ms = now_ms();
        self.camera.grow_extent(&self.gpu.bounds);
        let origin = self.camera.origin();
        // the point GPU rows are measured from
        let rebase = self
            .gpu
            .rebase_anchor(&origin, self.camera.distance_world(), now_ms);
        let view_proj = self
            .camera
            .view_proj_anchored(self.aspect(), &rebase.anchor);
        let input = FrameInput {
            view_proj,
            clear: CLEAR,
            now_ms,
        };
        self.dirty |= rebase.moved || self.gpu.view.perf || self.gpu.view.spin; // redraw reasons

        let mut dropped = false;

        if self.dirty && !self.cloud_query_awaiting_gpu() {
            let gap = now_ms - self.last_frame_ms; // time since the last frame
            self.last_frame_ms = now_ms;
            self.gpu.performance.interacting = self.interacting;
            let drawn = self.gpu.present(&input); // encode time, None when the frame was dropped

            // slow frames: drop to device scale 1 and no antialiasing
            if self.gpu.performance.take_slow_interaction()
                && (crate::engine::gpu::view::device_pixel_ratio() > 1.0
                    || self.gpu.targets.samples > 1)
            {
                crate::engine::gpu::view::reduce();
                self.gpu
                    .resize(self.gpu.config.width, self.gpu.config.height);
                log::warn!(
                    "slow interaction frames; rendering at device scale 1 without antialiasing"
                );
                self.status("Slow frames: rendering at device scale 1 without antialiasing");
            }

            dropped = drawn.is_none() && self.gpu.surface.is_some(); // try again next frame
            self.dirty = dropped;

            if let (true, Some(encode_ms)) = (self.gpu.view.perf, drawn) {
                self.perf_line(gap, encode_ms);
            }
        }

        // a pending pick draws its own id frame
        if !dropped && let Some(at) = self.gpu.pick.take_pending() {
            self.gpu.pick_frame(&input, at);
        }

        // reasons to draw again
        self.needs_frame |= dropped
            || rebase.pending
            || self.gpu.pick.busy()
            || self.gpu.view.perf
            || self.gpu.view.spin;
        #[cfg(target_arch = "wasm32")]
        crate::app::inspection::publish(self);
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
        let splitting = self.pending_split.is_some(); // a split wants a plain object
        let face = !splitting
            && (face || self.selection_tool == crate::app::selection::SelectionTool::Face);
        let edge = !splitting
            && (edge || self.selection_tool == crate::app::selection::SelectionTool::Edge);
        self.cancel_cloud_query();
        self.gpu.pick.cancel();

        // a point cloud answers by its own query
        #[cfg(target_arch = "wasm32")]
        if !splitting && !edge && self.start_cloud_query(x, y) {
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

    /// F10: show the control points of the selected object.
    pub fn enable_controls(&mut self) {
        let Some(parent) = self.scene.selected else {
            self.status("Select one object before pressing F10");
            return;
        };

        // already on
        if matches!(self.selection, SelectionMode::Controls { parent: active, .. } if active == parent)
        {
            return;
        }
        // the points come from the source geometry
        let controls = match self.scene.geometry(parent) {
            Some(geometry) => Controls::from_geometry(geometry),
            None if self.streamed_slot(parent).is_some() => Controls {
                cloud: true,
                ..Controls::default()
            },
            None => {
                self.status("Source controls are unavailable for this display-only object");
                return;
            }
        };

        if !controls.cloud && controls.points.is_empty() {
            self.status("This object has no selectable source controls");
            return;
        }

        // the gizmo would take the same clicks
        self.place_gizmo(None);
        self.selection.enable_controls(Some(parent), controls.cloud);
        self.gpu.arena.source_faces.select(&self.gpu.ctx, None);
        self.gpu.segments.set_edge(&self.gpu.ctx, None);
        self.gpu.set_selected(parent, false);
        self.gpu
            .splat
            .set_controls(controls.cloud.then_some(parent));
        self.controls = controls;
        self.upload_controls();
        self.update_label();
        self.status("Control points: click to select; Esc to leave");
        self.touch();
    }

    /// Esc: leave control points, keep the object selected.
    pub fn escape_selection(&mut self) {
        let parent = self.selection.escape();
        self.select(parent);
        self.status("");
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
                facing: FACING_UNKNOWN, // no face orientation
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
        self.upload_controls();
        self.place_gizmo(Some(parent));
        self.status(&format!("Selected {id:?}"));
        self.touch();
    }

    /// Show a message in the status line.
    fn status(&self, message: &str) {
        crate::app::feedback::status(message);
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

    /// Natively the perf line goes nowhere.
    #[cfg(not(target_arch = "wasm32"))]
    fn perf_line(&self, _gap_ms: f64, _encode_ms: f64) {}
}

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
