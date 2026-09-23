// --8<-- [start:step-19a]
//! One place for what the viewer knows between frames, so no pass has to reach into another pass's data.
use crate::app::scene::{FileDoc, Scene, StreamedInit};
use crate::app::selection::SelectionMode;
use crate::app::walk::cloud::StreamRows;
use crate::camera::Camera;
use crate::engine::gpu::pick::PickMode;
use crate::engine::gpu::{FrameInput, Gpu, Pick};
use crate::engine::performance::{heap_mb, now_ms};
use crate::engine::text::{TextLabel, TextPlacement};
use session_rust::AABB;
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
    pub window: Arc<Window>, // the winit window on the canvas
    pub gpu: Gpu, // device, buffers, pipelines
    pub camera: Camera, // the view
    pub scene: Scene, // the loaded documents
    pub needs_frame: bool, // draw again on the next redraw
    dirty: bool, // the picture changed
    last_frame_ms: f64,
    pub selection: SelectionMode, // object, edge, face or control points
    requested: PickMode, // what the pending pick looks for
    pub selection_radius_css: f64, // click tolerance in CSS pixels
    scene_labels: Vec<TextLabel>,
    show_selected_names: bool, // name label on the selection, T toggles
}

// --8<-- [end:step-19a]
// --8<-- [start:step-19b]
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
            camera: Camera::new(), // default view
            scene,
            needs_frame: true,
            dirty: true,
            last_frame_ms: 0.0,
            selection: SelectionMode::Object,
            requested: PickMode::Object,
            selection_radius_css: 6.0,
            scene_labels: Vec::new(), // no text yet
            show_selected_names: true,
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

    /// Start a streamed point cloud; returns its slot.
    pub fn add_streamed(&mut self, init: StreamedInit) -> usize {
        let idx = self.scene.add_streamed_cloud(init, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
        idx
    }

// --8<-- [end:step-19b]
    // --8<-- [start:step-19c]
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

    /// Remove every document; camera and GPU stay.
    pub fn clear(&mut self) {
        self.selection = SelectionMode::Object;
        self.scene_labels.clear();
        self.scene.clear(&mut self.gpu);
        self.touch();
    }

    /// Fit the camera to everything loaded.
    pub fn fit_all(&mut self) {
        let b = &self.gpu.bounds;
        log::info!("fit: bounds {} aspect {:.3}", b.str(), self.aspect());
        self.camera.fit(&self.gpu.bounds, self.aspect());
        self.touch();
    }

    /// Fit the camera to the selection, or to everything.
    pub fn fit_selected_or_all(&mut self) {
        let row = match self.scene.selected {
            Some(row) => self.gpu.objects.row_bounds(row),
            None => None,
        };
        let Some(b) = row else {
            self.fit_all();
            return;
        };
        log::info!(
            "fit selected: bounds {} aspect {:.3}",
            b.str(),
            self.aspect()
        );
        self.camera.fit(&b, self.aspect());
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
    }

// --8<-- [end:step-19c]
    // --8<-- [start:step-19d]
    /// Forward a canvas resize to the GPU layer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        self.gpu.logical_size = self.logical_size();
        self.touch();
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
        self.request_selection(x, y, false);
        // --8<-- [end:step-19d]
    // --8<-- [start:step-19e]
    }

    /// Something changed: drop pending picks, draw again.
    pub fn touch(&mut self) {
        self.gpu.pick.cancel();
        self.dirty = true;
        self.needs_frame = true;
    }

    /// Select one row, or nothing.
    pub fn select(&mut self, row: Option<u32>) {
        self.selection = SelectionMode::Object;
        self.gpu.controls.reset();
        self.gpu.control_net.reset();
        self.gpu.segments.set_edge(&self.gpu.ctx, None);
        self.gpu.splat.set_controls(None);

        if let Some(old) = self.scene.selected.take() {
            self.gpu.set_selected(old, false);
        }
        // --8<-- [end:step-19e]
// --8<-- [start:step-19f]

        if let Some(r) = row {
            self.gpu.set_selected(r, true);
        }

        self.scene.selected = row;
        self.update_label();
        self.touch();
    }

    /// H: hide the selection.
    pub fn hide_selected(&mut self) {
        let Some(row) = self.scene.selected else {
            return;
        };
        let Some(guid) = self.scene.identity_of(row) else {
            return;
        };
        self.select(None);
        self.scene.hidden.insert(guid); // by id, so a rebuild keeps it hidden
        self.gpu.set_hidden(row, true);
        self.touch();
    }

    /// S: show everything hidden.
    pub fn show_all(&mut self) {
        for row in self.scene.hidden_rows() {
            self.gpu.set_hidden(row, false);
        }

        self.scene.hidden.clear();
        self.touch();
    }

    /// A pick answer arrived: select what it hit.
    fn apply_pick(&mut self, pick: Option<Pick>) {
        match self.requested {
            PickMode::Edge => {
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
                }

                return;
            }
            PickMode::Object => {}
        }

        // nothing hit: clear, unless Shift is adding
        let Some(p) = pick else {
            log::info!("pick: nothing");
            self.select(None);
            // --8<-- [end:step-19f]
            // --8<-- [start:step-19g]
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

                let toggle = if self.scene.selected == Some(hit.row) {
                    None
                } else {
                    Some(hit.row)
                };
                self.select(toggle);
            }
            None => log::info!("pick: row {} sub {} (no document)", p.row, p.sub),
        }
    }

    /// Draw one frame; a still scene asks for no more.
    pub fn render(&mut self) {
        let logical = self.logical_size();

        // the CSS size changed: control dots keep their pixel size
        if logical != self.gpu.logical_size {
            self.gpu.logical_size = logical;
            self.touch();
        }

        // a GPU error from the last frame
        let failure = match self.gpu.failure.lock() {
            Ok(failure) => failure.clone(),
            Err(_) => None,
        };

        if let Some(message) = failure {
            crate::app::feedback::error(&message);
            self.gpu.pick.cancel();
            self.needs_frame = false;
            return;
        }

        // apply a pick answer first, so this frame shows it
        if let Some(pick) = self.gpu.pick.poll() {
            self.apply_pick(pick);
        }

        self.needs_frame = false;
        // --8<-- [end:step-19g]
// --8<-- [start:step-19h]

        if self.gpu.view.spin {
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

        if self.dirty {
            let gap = now_ms - self.last_frame_ms;
            self.last_frame_ms = now_ms;
            let drawn = self.gpu.present(&input); // encode time, None when the frame was dropped
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

    /// CSS size from the canvas; native size is physical.
    fn logical_size(&self) -> [f64; 2] {
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

    /// One gesture, one pick pass; Ctrl never selects.
    pub fn request_selection(&mut self, x: u32, y: u32, edge: bool) {
        self.gpu.pick.cancel();
        let mode = if edge {
        // --8<-- [end:step-19h]
            // --8<-- [start:step-19i]
            PickMode::Edge
        } else {
            PickMode::Object
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

    /// Esc: leave control points, keep the object selected.
    pub fn escape_selection(&mut self) {
        let parent = self.selection.escape();
        self.select(parent);
        self.status("");
    }

    /// T: show or hide the name label on the selection.
    pub fn toggle_selected_names(&mut self) {
        self.show_selected_names = !self.show_selected_names;
        self.update_label();
        self.touch();
    }

    /// Prepare one source-document title when its CAD rows arrive.
    fn annotate_document(&mut self, first_row: usize) {
        let mut bounds = AABB::empty();

        for index in first_row..self.scene.object_count() {
            let row = index as u32;
            if matches!(
                self.scene.geometry(row),
                Some(session_rust::Geometry::BRep(_) | session_rust::Geometry::NurbsSurface(_))
            ) && let Some(object) = self.gpu.objects.row_bounds(row)
            {
                bounds.union_with(&object);
            }
        }

        if !bounds.is_valid() {
            return;
        }

        let Some(document) = self.scene.docs.last() else {
            return;
        };
        let mut anchor = label_center(&bounds);
        anchor[2] = bounds.max_point()[2] + bounds.diagonal() * 0.08;
        self.scene_labels.push(nameplate(
        // --8<-- [end:step-19i]
            // --8<-- [start:step-19j]
            self.scene.docs.len() as u32,
            document.name.clone(),
            anchor,
        ));
    }

    /// Document titles plus the selected name, same color.
    fn update_label(&mut self) {
        let mut labels = self.scene_labels.clone();

        if self.show_selected_names
            && let Some(row) = self.scene.selected
            && let Some(bounds) = self.gpu.objects.row_bounds(row)
        {
            // Text ID 0 belongs to selection; document IDs start at 1, independent of rows.
            labels.push(nameplate(
                0,
                self.scene.object_name(row).to_string(),
                label_center(&bounds),
            ));
        }

        if let Err(error) = self.gpu.text.set_labels(labels) {
            self.status(&format!("Text: {error}"));
        }
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

/// Center an annotation in the source object bounds.
fn label_center(bounds: &AABB) -> [f64; 3] {
    let mut center = [0.0; 3];

    for (axis, coordinate) in center.iter_mut().enumerate() {
        *coordinate = (bounds.min_point()[axis] + bounds.max_point()[axis]) * 0.5;
    }

    center
}
// --8<-- [end:step-19j]
// --8<-- [start:step-19k]

/// Match production padding, rounded caps and white glyph style.
fn nameplate(id: u32, text: String, world: [f64; 3]) -> TextLabel {
    let scale = if id == 0 { 0.75 } else { 1.0 };
    let line_height = 26.0 * scale;
    let vertical_padding = 4.0 * scale;
    // a cap at each end keeps the text inside
    let horizontal_padding = if id == 0 {
        line_height * 0.5 + vertical_padding
    } else {
        6.0 * scale
    };
    TextLabel {
        id,
        text,
        font_size: 18.0 * scale,
        line_height,
        color: [255; 4],
        placement: TextPlacement::Nameplate {
            world,
            padding: [horizontal_padding, vertical_padding],
            rounded: id == 0, // the title gets round corners
        },
        clip: None,
    }
}
// --8<-- [end:step-19k]
