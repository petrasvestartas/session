//! `State` - the viewer itself: the layers (`gpu`, `scene`, `camera`) and the shell state:
//! `needs_frame`, the demand for a redraw, and `dirty`, whether the picture changed (a pick
//! on a still scene needs the loop, not a colour frame). Higher layers drive lower ones,
//! never the other way round.

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
pub mod edit;
mod sheet_query;
mod text;
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
    /// A drag or pinch is in progress (set by the input layer).
    pub interacting: bool,
    /// The picture changed: the redraw presents a colour frame. A pending pick alone does not.
    dirty: bool,
    last_frame_ms: f64,
    pub selection: SelectionMode,
    controls: Controls,
    requested: PickMode,
    pub selection_radius_css: f64,
    /// A view preference retained across selections and scene reloads; T toggles it.
    show_selected_names: bool,
    cloud_query: Option<crate::app::cloud_query::Query>,
    #[cfg(target_arch = "wasm32")]
    query_generation: u64,
    sheet_query: Option<crate::app::sheet_query::Query>,
    sheet_generation: u64,
    /// The move/rotate/scale widget, present only while a row with a box is selected.
    pub gizmo: Option<crate::app::gizmo::Gizmo>,
    /// The drag in progress, holding what the gesture is measured FROM.
    dragging: Option<edit::GizmoDrag>,
    /// A control point being dragged, which is a different gesture from a placement drag.
    control_drag: Option<edit::ControlDrag>,
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
            interacting: false,
            dirty: true,
            last_frame_ms: 0.0,
            selection: SelectionMode::Object,
            controls: Controls::default(),
            requested: PickMode::Object,
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
        self.scene.set_texts(texts, &mut self.gpu);
        self.update_label();
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

    /// A sheet's first slice; returns the slot later slices address.
    pub fn add_sheet(&mut self, init: SheetInit) -> usize {
        let idx = self.scene.add_sheet(init, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
        idx
    }

    /// One more slice of sheet `idx`.
    pub fn extend_sheet(&mut self, idx: usize, rows: SheetRows, to: u32) {
        self.scene.extend_sheet(idx, rows, to, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        log::info!(
            "sheet slice: {to} segments resident | heap {:.0} MB",
            heap_mb()
        );
        self.touch();
    }

    /// Drop every document; the canvas, device and camera stay.
    pub fn clear(&mut self) {
        self.selection = SelectionMode::Object;
        self.sheet_query = None;
        self.gpu.arena.source_faces.select(&self.gpu.ctx, None);
        self.controls = Controls::default();
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

    /// `P`: shaded faces <-> x-ray. In x-ray every mesh, NURBS and BRep face is gone and only
    /// its edges, points and text remain; lines and points are never affected.
    pub fn toggle_xray(&mut self) {
        self.gpu.view.opacity = if self.gpu.view.opacity > 0.0 {
            0.0
        } else {
            1.0
        };
        self.touch();
    }

    /// Ask what is under pixel (x, y); the answer lands in a later frame (`apply_pick`).
    pub fn request_pick(&mut self, x: u32, y: u32) {
        self.request_selection(x, y, false, false);
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
        self.place_gizmo(row);
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
        self.update_label();
        self.touch();
    }

    /// Show everything hidden so far - `S`.
    pub fn show_all(&mut self) {
        for row in self.scene.hidden_rows() {
            self.gpu.set_hidden(row, false);
        }
        self.scene.hidden.clear();
        self.update_label();
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
            PickMode::Edge | PickMode::Component => {
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
                if let Some(entity) = hit.entity {
                    self.apply_sheet_pick(hit.row, entity);
                    return;
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
            self.gpu.performance.interacting = self.interacting;
            let drawn = self.gpu.present(&input);
            if self.gpu.performance.take_slow_interaction()
                && crate::engine::gpu::view::device_pixel_ratio() > 1.0
            {
                crate::engine::gpu::view::reduce_for_slow_frames();
                log::warn!(
                    "slow interaction frames; rendering at device scale 1 without antialiasing"
                );
                self.status("Slow frames: rendering at device scale 1 without antialiasing");
            }
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

    /// Ask what is under pixel (x, y) in ONE pass, chosen by the modifiers the click carried:
    /// `edge` is Ctrl and picks the source edge, `face` is Ctrl+Shift and picks the source face
    /// with edges still winning. Both false is the ordinary object pass, or the control-point
    /// pass while F10 controls are up. Ctrl never also performs ordinary selection.
    pub fn request_selection(&mut self, x: u32, y: u32, edge: bool, face: bool) {
        self.cancel_cloud_query();
        self.gpu.pick.cancel();
        #[cfg(target_arch = "wasm32")]
        if !edge && self.start_cloud_query(x, y) {
            return;
        }
        let mode = if face {
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
            // A control dot tags the top two bits `01` and carries its index in the low 30;
            // `Pick::sub` documents the whole scheme. Anything else here is not a dot.
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

    /// Non-disruptive interaction feedback also remains available to native diagnostics.
    fn status(&self, message: &str) {
        crate::app::feedback::status(message);
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
pub(crate) fn render_position(position: [f64; 3]) -> [f32; 3] {
    [position[0] as f32, position[1] as f32, position[2] as f32]
}
