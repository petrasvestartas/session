//! `State` - the viewer itself: the layers (`gpu`, `scene`, `camera`) and the shell state:
//! `needs_frame`, the demand for a redraw, and `dirty`, whether the picture changed (a pick
//! on a still scene needs the loop, not a colour frame). Higher layers drive lower ones,
//! never the other way round.

use crate::app::scene::{FileDoc, Scene, StreamedInit};
use crate::app::selection::{ControlId, Controls, SelectionMode};
use crate::app::walk::cloud::StreamRows;
use crate::app::walk::encode::FACING_UNKNOWN;
use crate::camera::Camera;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::pick::PickMode;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use crate::engine::gpu::{FrameInput, Gpu, Pick};
use crate::engine::performance::{heap_mb, now_ms};
use crate::engine::text::{TextLabel, TextPlacement};
use std::sync::Arc;
use winit::window::Window;

/// Background colour of every frame.
const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

/// Orbit step per frame in `?spin=1` mode.
const SPIN_STEP: f32 = 0.004;

/// Own the window, rendering stack and source selection, coordinating frame and query lifetimes.
pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
    pub scene: Scene,
    /// Something changed since the last frame; the shell asks for a redraw when it sees this.
    pub needs_frame: bool,
    /// The picture changed: the redraw presents a colour frame. A pending pick alone does not.
    dirty: bool,
    last_frame_ms: f64,
    pub selection: SelectionMode,
    controls: Controls,
    requested: PickMode,
    pub selection_radius_css: f64,
    /// Document annotations are prepared only when documents change, never on camera motion.
    scene_labels: Vec<TextLabel>,
    /// A view preference retained across selections and scene reloads; T toggles it.
    show_selected_names: bool,
    cloud_query: Option<crate::app::cloud_query::Query>,
    #[cfg(target_arch = "wasm32")]
    query_generation: u64,
}

impl State {
    /// Wire the stack around `scene` (usually empty; the loader posts documents afterwards).
    pub async fn new(window: Arc<Window>, mut scene: Scene) -> anyhow::Result<Self> {
        let t0 = now_ms();
        let mut gpu = Gpu::new(window.clone()).await?;
        scene.upload_to(&mut gpu);
        log::info!("gpu init {:.0} ms", now_ms() - t0);
        Ok(Self {
            window,
            gpu,
            camera: Camera::new(),
            scene,
            needs_frame: true,
            dirty: true,
            last_frame_ms: 0.0,
            selection: SelectionMode::Object,
            controls: Controls::default(),
            requested: PickMode::Object,
            selection_radius_css: 6.0,
            scene_labels: Vec::new(),
            show_selected_names: true,
            cloud_query: None,
            #[cfg(target_arch = "wasm32")]
            query_generation: 0,
        })
    }

    /// The surface's width over its height (never the window's, which is 0x0 on the web).
    pub fn aspect(&self) -> f64 {
        self.gpu.config.width.max(1) as f64 / self.gpu.config.height.max(1) as f64
    }

    /// The surface size in physical pixels.
    pub fn viewport(&self) -> (f64, f64) {
        (self.gpu.config.width as f64, self.gpu.config.height as f64)
    }

    /// Append one parsed document: walk it into the tables, upload the delta.
    pub fn append(&mut self, doc: FileDoc) {
        let t0 = now_ms();
        let first_row = self.scene.object_count();
        self.scene.add_file(doc);
        let t1 = now_ms();
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

    /// Replace authored scene text after the corresponding manifest/geometry revision is ready.
    pub fn set_texts(&mut self, texts: Vec<crate::app::manifest::TextItem>) {
        self.scene.texts = texts;
        self.update_label();
        self.include_text_bounds();
        self.touch();
    }

    /// A streamed cloud's first slice; returns the slot later slices address.
    pub fn add_streamed(&mut self, init: StreamedInit) -> usize {
        let idx = self.scene.add_streamed_cloud(init, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
        idx
    }

    /// One more slice of streamed cloud `idx`.
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

    /// Drop every document; the canvas, device and camera stay.
    pub fn clear(&mut self) {
        self.selection = SelectionMode::Object;
        self.controls = Controls::default();
        self.scene_labels.clear();
        self.scene.clear(&mut self.gpu);
        self.touch();
    }

    /// Fit the camera around everything loaded so far.
    pub fn fit_all(&mut self) {
        let b = &self.gpu.bounds;
        log::info!(
            "fit: bounds {:?} .. {:?} aspect {:.3}",
            b.min,
            b.max,
            self.aspect()
        );
        self.camera.fit(&self.gpu.bounds, self.aspect());
        self.touch();
    }

    /// Fit the camera to the selected object's world box; falls back to `fit_all` when
    /// nothing is selected or the selection has no volume.
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
            "fit selected: bounds {:?} .. {:?} aspect {:.3}",
            b.min,
            b.max,
            self.aspect()
        );
        self.camera.fit(&b, self.aspect());
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
    }

    /// Forward a canvas resize to the GPU layer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        self.gpu.logical_size = self.logical_size();
        self.upload_controls();
        self.touch();
    }

    /// The global cloud point-size scale, clamped.
    pub fn set_cloud_size(&mut self, size: f32) {
        self.gpu.view.cloud_size = size.clamp(0.25, 8.0);
        self.touch();
    }

    /// Ask what is under pixel (x, y); the answer lands in a later frame (`apply_pick`).
    pub fn request_pick(&mut self, x: u32, y: u32) {
        self.request_selection(x, y, false);
    }

    /// The picture changed: the next redraw presents it.
    pub fn touch(&mut self) {
        self.cancel_cloud_query();
        self.gpu.pick.cancel();
        self.dirty = true;
        self.needs_frame = true;
    }

    /// Make `row` the selection (or none), moving the highlight.
    pub fn select(&mut self, row: Option<u32>) {
        self.selection = SelectionMode::Object;
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
        self.update_label();
        self.touch();
    }

    /// Toggle selected-object annotations without changing source selection or document titles.
    pub fn toggle_selected_names(&mut self) {
        self.show_selected_names = !self.show_selected_names;
        self.update_label();
        self.touch();
    }

    /// Hide the selection - `H`. The guid goes in the hide set as well as the row's flag, so
    /// a `rebuild` (an edit commit) re-applies it when the rows come back. A live reload
    /// takes the `clear` path instead and starts a fresh scene with nothing hidden.
    pub fn hide_selected(&mut self) {
        let Some(row) = self.scene.selected else {
            return;
        };
        let Some(guid) = self.scene.identity_of(row) else {
            return;
        };
        self.select(None);
        self.scene.hidden.insert(guid);
        self.gpu.set_hidden(row, true);
        self.touch();
    }

    /// Show everything hidden so far - `S`.
    pub fn show_all(&mut self) {
        for row in self.scene.hidden_rows() {
            self.gpu.set_hidden(row, false);
        }
        self.scene.hidden.clear();
        self.touch();
    }

    /// A pick came back: log what it hit and select it (clicking the selection clears it).
    fn apply_pick(&mut self, pick: Option<Pick>) {
        #[cfg(target_arch = "wasm32")]
        if self.cloud_query_awaiting_gpu() {
            self.apply_cloud_query_pick(pick);
            return;
        }
        match self.requested {
            PickMode::Edge => {
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
        let Some(p) = pick else {
            log::info!("pick: nothing");
            self.select(None);
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

    /// Draw ONE frame and never ask for the next: a still scene costs nothing after this.
    /// The shell asks again when `needs_frame` is set - by an input, a message, a resize, a
    /// throttled re-anchor still due, a pick in flight, or continuous mode. A pick that came
    /// back is applied first, so the same frame presents its highlight; a pick requested on
    /// a still scene runs alone, with no colour frame to wait behind.
    pub fn render(&mut self) {
        let logical = self.logical_size();
        if logical != self.gpu.logical_size {
            self.gpu.logical_size = logical;
            self.upload_controls();
            self.touch();
        }
        let failure = match self.gpu.failure.lock() {
            Ok(failure) => failure.clone(),
            Err(_) => None,
        };
        if let Some(message) = failure {
            crate::app::feedback::error(&message);
            self.cancel_cloud_query();
            self.gpu.pick.cancel();
            self.needs_frame = false;
            return;
        }
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
        self.dirty |= rebase.moved || self.gpu.view.perf || self.gpu.view.spin;

        let mut dropped = false;
        if self.dirty && !self.cloud_query_awaiting_gpu() {
            let gap = now_ms - self.last_frame_ms;
            self.last_frame_ms = now_ms;
            let drawn = self.gpu.present(&input);
            dropped = drawn.is_none() && self.gpu.surface.is_some();
            self.dirty = dropped;
            if let (true, Some(encode_ms)) = (self.gpu.view.perf, drawn) {
                self.perf_line(gap, encode_ms);
            }
        }
        if !dropped && let Some(at) = self.gpu.pick.take_pending() {
            self.gpu.pick_frame(&input, at);
        }
        self.needs_frame |= dropped
            || rebase.pending
            || self.gpu.pick.busy()
            || self.gpu.view.perf
            || self.gpu.view.spin;
        #[cfg(target_arch = "wasm32")]
        crate::app::inspection::publish(self);
    }

    /// CSS dimensions come from the actual canvas; native harness dimensions are physical.
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

    /// A pointer gesture chooses one pass; Ctrl never also performs ordinary selection.
    pub fn request_selection(&mut self, x: u32, y: u32, edge: bool) {
        self.cancel_cloud_query();
        self.gpu.pick.cancel();
        #[cfg(target_arch = "wasm32")]
        if !edge && self.start_cloud_query(x, y) {
            return;
        }
        let mode = if edge {
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
        let scale = f64::from(self.gpu.config.width) / logical[0];
        self.gpu
            .pick
            .configure(mode, self.selection_radius_css, scale);
        self.gpu.pick.request(x, y);
        self.needs_frame = true;
    }

    /// Enable source controls once for the sole selected parent, retaining the source geometry.
    pub fn enable_controls(&mut self) {
        let Some(parent) = self.scene.selected else {
            self.status("Select one object before pressing F10");
            return;
        };
        if matches!(self.selection, SelectionMode::Controls { parent: active, .. } if active == parent)
        {
            return;
        }
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
        self.selection.enable_controls(Some(parent), controls.cloud);
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

    /// Escape clears targets before retaining the active parent as an ordinary selection.
    pub fn escape_selection(&mut self) {
        let parent = self.selection.escape();
        self.select(parent);
        self.status("");
    }

    /// Upload only temporary source controls; repeated F10 never appends duplicate markers.
    fn upload_controls(&mut self) {
        self.gpu.controls.reset();
        self.gpu.control_net.reset();
        let SelectionMode::Controls {
            parent, selected, ..
        } = self.selection
        else {
            return;
        };
        let scale = f64::from(self.gpu.config.width) / self.logical_size()[0];
        let mut glyphs = GlyphRows::default();
        for control in &self.controls.points {
            let color = if Some(control.id) == selected {
                [1.0, 1.0, 0.0, 1.0]
            } else {
                [0.15, 0.35, 0.9, 1.0]
            };
            glyphs.dots.push(GlyphPoint {
                center: render_position(control.position),
                radius: -3.5 * scale as f32,
                color,
                instance_id: parent,
                facing: FACING_UNKNOWN,
                facing_ext: [FACING_UNKNOWN; 2],
            });
        }
        let mut segments = SegRows::default();
        for &[start, end] in &self.controls.links {
            segments.ribbons.push(CylinderSegment {
                p0: render_position(self.controls.points[start].position),
                p1: render_position(self.controls.points[end].position),
                radius: 0.0,
                color: 0xffcc8866,
                instance_id: parent,
                facing: FACING_UNKNOWN,
            });
        }
        self.gpu
            .controls
            .append(&self.gpu.ctx, &self.gpu.layouts, &glyphs);
        self.gpu
            .control_net
            .append(&self.gpu.ctx, &self.gpu.layouts, &segments);
    }

    /// Resolve only controls belonging to the still-active parent, never an old pick's parent.
    fn apply_control(&mut self, pick: Pick, cloud: bool) {
        let SelectionMode::Controls { parent, .. } = self.selection else {
            return;
        };
        if pick.row != parent {
            return;
        }
        let id = if cloud {
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
        self.status(&format!("Selected {id:?}"));
        self.touch();
    }

    /// A source-query page owns GPU readback while ordinary color rendering is paused.
    fn cloud_query_awaiting_gpu(&self) -> bool {
        match &self.cloud_query {
            Some(query) => query.awaiting_gpu,
            None => false,
        }
    }

    /// Locate the streamed descriptor belonging to the active source parent.
    fn streamed_slot(&self, parent: u32) -> Option<usize> {
        for (slot, cloud) in self.scene.streamed.iter().enumerate() {
            if cloud.row == parent {
                return Some(slot);
            }
        }
        None
    }

    /// Superseding input invalidates both HTTP callbacks and GPU readback for the query.
    fn cancel_cloud_query(&mut self) {
        if self.cloud_query.take().is_some() {
            self.gpu.pick.cancel();
            self.upload_controls();
            self.status("Point query cancelled because the view or selection changed");
        }
    }

    /// F10 on a streamed cloud queries source ranges independently of display residency.
    #[cfg(target_arch = "wasm32")]
    fn start_cloud_query(&mut self, x: u32, y: u32) -> bool {
        use crate::app::cloud_query::{Query, QueryView};
        let SelectionMode::Controls {
            parent,
            cloud: true,
            ..
        } = self.selection
        else {
            return false;
        };
        let Some(slot) = self.streamed_slot(parent) else {
            return false;
        };
        self.gpu.pick.cancel();
        self.query_generation = self.query_generation.wrapping_add(1);
        let cloud = &self.scene.streamed[slot];
        let projection = self
            .camera
            .view_proj_anchored(self.aspect(), &session_rust::Point::new(0.0, 0.0, 0.0));
        let scale = f64::from(self.gpu.config.width) / self.logical_size()[0];
        let view = QueryView {
            matrix: crate::math::mat_mul(&projection.m, &cloud.place),
            size: [
                f64::from(self.gpu.config.width),
                f64::from(self.gpu.config.height),
            ],
            at: [x, y],
            radius: (self.selection_radius_css * scale).ceil().clamp(1.0, 128.0) + 3.5 * scale,
        };
        self.cloud_query = Some(Query::new(self.query_generation, cloud, view));
        self.gpu.pick.start_source_query();
        self.advance_cloud_query();
        true
    }

    /// Fetch one page, or resolve the final winner only after all eligible pages finished.
    #[cfg(target_arch = "wasm32")]
    fn advance_cloud_query(&mut self) {
        let Some(query) = self.cloud_query.as_mut() else {
            return;
        };
        query.awaiting_gpu = false;
        query.candidates.clear();
        if let Some(page) = query.next_page() {
            let progress = format!(
                "Checking source points: {} / {} (display LOD remains bounded)",
                query.checked, query.total
            );
            crate::app::cloud_query::fetch_page(query, page);
            self.status(&progress);
        } else if let Some(best) = query.best {
            crate::app::cloud_query::resolve_id(query, best);
            self.status("All eligible source points checked; resolving original point ID…");
        } else {
            self.cloud_query = None;
            self.gpu.pick.cancel();
            self.status("No visible source point in the selection window");
        }
        self.upload_controls();
    }

    /// A range callback belongs to exactly one camera/scene/parent generation.
    #[cfg(target_arch = "wasm32")]
    pub fn cloud_query_batch(&mut self, batch: crate::app::cloud_query::Batch) {
        let Some(query) = self.cloud_query.as_mut() else {
            return;
        };
        if query.id != batch.query || query.cancelled.get() {
            return;
        }
        let (candidates, revision) = match batch.result {
            Ok(result) => result,
            Err(error) => {
                self.cloud_query = None;
                self.gpu.pick.cancel();
                self.upload_controls();
                self.status(&format!("Point query failed: {error}"));
                return;
            }
        };
        query.checked += batch.count;
        query.revision = revision;
        if candidates.is_empty() {
            self.advance_cloud_query();
            return;
        }
        query.candidates = candidates;
        query.awaiting_gpu = true;
        let parent = query.parent;
        let at = query.view.at;
        let scale = f64::from(self.gpu.config.width) / self.logical_size()[0];
        let query = self.cloud_query.as_ref().unwrap();
        let mut glyphs = GlyphRows::default();
        for candidate in &query.candidates {
            glyphs.dots.push(GlyphPoint {
                center: render_position(candidate.position),
                radius: -3.5 * scale as f32,
                color: [0.15, 0.35, 0.9, 1.0],
                instance_id: parent,
                facing: FACING_UNKNOWN,
                facing_ext: [candidate.local, FACING_UNKNOWN],
            });
        }
        self.gpu.controls.reset();
        self.gpu
            .controls
            .append(&self.gpu.ctx, &self.gpu.layouts, &glyphs);
        // These candidates are ID targets only. The normal frame never presents this page.
        self.requested = PickMode::Controls {
            parent,
            cloud: false,
        };
        self.gpu
            .pick
            .configure(self.requested, self.selection_radius_css, scale);
        self.gpu.pick.request(at[0], at[1]);
        self.needs_frame = true;
    }

    /// Fold this page's GPU-visible answer, then continue through every remaining node.
    #[cfg(target_arch = "wasm32")]
    fn apply_cloud_query_pick(&mut self, pick: Option<Pick>) {
        let Some(query) = self.cloud_query.as_mut() else {
            return;
        };
        // This is the winner of the accumulated physical source-point depth, including
        // previous pages. Its exact source position is range-read after the final page.
        query.best = match pick {
            Some(pick) if pick.row == query.parent && pick.sub < query.fields.count => {
                Some(pick.sub)
            }
            _ => None,
        };
        self.advance_cloud_query();
    }

    /// Keep the actual chosen source position visible, even when it is outside the resident LOD.
    #[cfg(target_arch = "wasm32")]
    pub fn cloud_query_resolved(&mut self, resolved: crate::app::cloud_query::Resolved) {
        let Some(query) = self.cloud_query.as_ref() else {
            return;
        };
        if query.id != resolved.query || query.cancelled.get() {
            return;
        }
        let query = self.cloud_query.take().unwrap();
        let (source, position) = match resolved.result {
            Ok(result) => result,
            Err(error) => {
                self.gpu.pick.cancel();
                self.upload_controls();
                self.status(&format!("Point query failed: {error}"));
                return;
            }
        };
        let Some(best) = query.best else { return };
        let id = ControlId::Point(source);
        self.selection = SelectionMode::Controls {
            parent: query.parent,
            selected: Some(id),
            cloud: true,
        };
        self.controls.points = vec![crate::app::selection::Control { id, position }];
        self.gpu.splat.set_point(None);
        self.upload_controls();
        self.status(&format!(
            "Selected source point {source} (row {}); all {} eligible points checked",
            best, query.total
        ));
        self.touch();
    }

    /// Keep one readable source-document title above each loaded CAD group.
    /// Only the newly appended rows are visited; large scenes are not scanned on selection.
    fn annotate_document(&mut self, first_row: usize) {
        let mut bounds = crate::math::Aabb::empty();
        for index in first_row..self.scene.object_count() {
            let row = index as u32;
            if matches!(
                self.scene.geometry(row),
                Some(session_rust::Geometry::BRep(_) | session_rust::Geometry::NurbsSurface(_))
            ) && let Some(object) = self.gpu.objects.row_bounds(row)
            {
                bounds.union(&object);
            }
        }
        if !bounds.is_finite() {
            return;
        }
        let Some(document) = self.scene.docs.last() else {
            return;
        };
        let mut anchor = label_center(&bounds);
        anchor[2] = f64::from(bounds.max[2]) + f64::from(bounds.diagonal()) * 0.08;
        self.scene_labels.push(nameplate(
            self.scene.docs.len() as u32,
            document.name.clone(),
            anchor,
        ));
    }

    /// Source-document text and the selected object's centered name share a fixed white style.
    fn update_label(&mut self) {
        let mut labels = self.scene_labels.clone();
        for (index, text) in self.scene.texts.iter().enumerate() {
            labels.push(TextLabel {
                id: (self.scene.docs.len() + index + 1) as u32,
                text: text.text.clone(),
                font_size: 18.0,
                line_height: 26.0,
                color: [255; 4],
                placement: TextPlacement::WorldPlane {
                    world: text.at,
                    right: text.right,
                    up: text.up,
                    world_height: text.height,
                },
                clip: None,
            });
        }
        if self.show_selected_names
            && !matches!(self.selection, SelectionMode::Controls { .. })
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

    /// Include authored text planes in camera fitting using shaped extents and their fixed frame.
    fn include_text_bounds(&mut self) {
        for run in &self.gpu.text.document.runs {
            let TextPlacement::WorldPlane {
                world,
                right,
                up,
                world_height,
            } = run.label.placement
            else {
                continue;
            };
            let unit = world_height / f64::from(run.label.font_size);
            let mut width = 0.0f64;
            let mut height = 0.0f64;
            for line in run.buffer.layout_runs() {
                width = width.max(f64::from(line.line_w) * unit);
                height = height.max(f64::from(line.line_top + line.line_height) * unit);
            }
            let padding = world_height * 0.125;
            for x in [-padding, width + padding] {
                for y in [-padding, height + padding] {
                    let mut point = [0.0f32; 3];
                    for axis in 0..3 {
                        point[axis] = (world[axis] + right[axis] * x - up[axis] * y) as f32;
                    }
                    if point.into_iter().all(f32::is_finite) {
                        self.gpu.bounds.grow(point);
                    }
                }
            }
        }
    }

    /// Non-disruptive interaction feedback also remains available to native diagnostics.
    fn status(&self, message: &str) {
        #[cfg(target_arch = "wasm32")]
        if let Some(window) = web_sys::window()
            && let Some(document) = window.document()
            && let Some(status) = document.get_element_by_id("viewer-status")
        {
            status.set_text_content(Some(message));
        }
        log::info!("{message}");
    }

    /// The `?perf=1` line: frame number, gap since the previous frame, encode time, heap.
    #[cfg(target_arch = "wasm32")]
    fn perf_line(&self, gap_ms: f64, encode_ms: f64) {
        let line = format!(
            "f{} gap {gap_ms:.0} enc {encode_ms:.1} ms wasm capacity {:.0} MiB",
            self.gpu.performance.frames,
            heap_mb()
        );
        crate::engine::performance::perf_line(&line);
    }

    /// Natively the perf line goes nowhere (the harness prints its own numbers).
    #[cfg(not(target_arch = "wasm32"))]
    fn perf_line(&self, _gap_ms: f64, _encode_ms: f64) {}
}

#[cfg(target_arch = "wasm32")]
impl State {
    /// Read-only source-control detail for the opt-in browser inspection fixture.
    pub fn inspected_controls(&self) -> Vec<serde_json::Value> {
        let mut points = Vec::with_capacity(self.controls.points.len());
        for point in &self.controls.points {
            points.push(serde_json::json!({"id": point.id, "position": point.position}));
        }
        points
    }
}

/// Convert a retained source position at the temporary GPU-control upload boundary.
fn render_position(position: [f64; 3]) -> [f32; 3] {
    [position[0] as f32, position[1] as f32, position[2] as f32]
}

/// Center a viewer annotation in the object's world-space bounds before camera projection.
fn label_center(bounds: &crate::math::Aabb) -> [f64; 3] {
    let mut center = [0.0; 3];
    for (axis, coordinate) in center.iter_mut().enumerate() {
        *coordinate = (f64::from(bounds.min[axis]) + f64::from(bounds.max[axis])) * 0.5;
    }
    center
}

/// White-on-black annotations; selection ID 0 uses 75% of the document title size.
fn nameplate(id: u32, text: String, world: [f64; 3]) -> TextLabel {
    let scale = if id == 0 { 0.75 } else { 1.0 };
    let line_height = 26.0 * scale;
    let vertical_padding = 4.0 * scale;
    // A full cap radius at each end keeps the entire text line inside the straight section.
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
            rounded: id == 0,
        },
        clip: None,
    }
}
