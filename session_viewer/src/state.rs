//! `State` - the viewer itself: the layers (`gpu`, `scene`, `camera`) and the shell state:
//! `needs_frame`, the demand for a redraw, and `dirty`, whether the picture changed (a pick
//! on a still scene needs the loop, not a colour frame). Higher layers drive lower ones,
//! never the other way round.

use std::sync::Arc;
use winit::window::Window;
use crate::app::scene::{FileDoc, Scene, StreamedInit};
use crate::app::walk::cloud::StreamRows;
use crate::camera::Camera;
use crate::engine::gpu::{FrameInput, Gpu, Pick};
use crate::engine::performance::{heap_mb, now_ms};

/// Background colour of every frame.
const CLEAR: wgpu::Color = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

/// Orbit step per frame in `?spin=1` mode.
const SPIN_STEP: f32 = 0.004;

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
}

impl State {
    /// Wire the stack around `scene` (usually empty; the loader posts documents afterwards).
    pub async fn new(window: Arc<Window>, mut scene: Scene) -> anyhow::Result<Self> {
        let t0 = now_ms();
        let mut gpu = Gpu::new(window.clone()).await?;
        scene.upload_to(&mut gpu);
        log::info!("gpu init {:.0} ms", now_ms() - t0);
        Ok(Self { window, gpu, camera: Camera::new(), scene, needs_frame: true, dirty: true, last_frame_ms: 0.0 })
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
        self.scene.add_file(doc);
        let t1 = now_ms();
        self.scene.upload_to(&mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        log::info!("appended: walk {:.0} ms, upload {:.0} ms | {} docs | heap {:.0} MB", t1 - t0, now_ms() - t1, self.scene.docs.len(), heap_mb());
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
        self.scene.extend_streamed_cloud(idx, rows, to, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        log::info!("cloud slice: {to} points resident | heap {:.0} MB", heap_mb());
        self.touch();
    }

    /// Drop every document; the canvas, device and camera stay.
    pub fn clear(&mut self) {
        self.scene.clear(&mut self.gpu);
        self.touch();
    }

    /// Fit the camera around everything loaded so far.
    pub fn fit_all(&mut self) {
        let b = &self.gpu.bounds;
        log::info!("fit: bounds {:?} .. {:?} aspect {:.3}", b.min, b.max, self.aspect());
        self.camera.fit(&self.gpu.bounds, self.aspect());
        self.touch();
    }

    /// Fit the camera to the selected object's world box; falls back to `fit_all` when
    /// nothing is selected or the selection has no volume.
    pub fn fit_selected_or_all(&mut self) {
        let row = self.scene.selected.and_then(|r| self.gpu.objects.row_bounds(r));
        let Some(b) = row else {
            self.fit_all();
            return;
        };
        log::info!("fit selected: bounds {:?} .. {:?} aspect {:.3}", b.min, b.max, self.aspect());
        self.camera.fit(&b, self.aspect());
        self.touch();
    }

    /// Forward a canvas resize to the GPU layer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        self.touch();
    }

    /// The global cloud point-size scale, clamped.
    pub fn set_cloud_size(&mut self, size: f32) {
        self.gpu.view.cloud_size = size.clamp(0.25, 8.0);
        self.touch();
    }

    /// Ask what is under pixel (x, y); the answer lands in a later frame (`apply_pick`).
    pub fn request_pick(&mut self, x: u32, y: u32) {
        self.gpu.pick.request(x, y);
        self.needs_frame = true;
    }

    /// The picture changed: the next redraw presents it.
    pub fn touch(&mut self) {
        self.dirty = true;
        self.needs_frame = true;
    }

    /// Make `row` the selection (or none), moving the highlight.
    pub fn select(&mut self, row: Option<u32>) {
        if let Some(old) = self.scene.selected.take() {
            self.gpu.set_selected(old, false);
        }
        if let Some(r) = row {
            self.gpu.set_selected(r, true);
        }
        self.scene.selected = row;
        self.touch();
    }

    /// Hide the selection - `H`. The guid goes in the hide set as well as the row's flag, so
    /// a `rebuild` (an edit commit) re-applies it when the rows come back. A live reload
    /// takes the `clear` path instead and starts a fresh scene with nothing hidden.
    pub fn hide_selected(&mut self) {
        let Some(row) = self.scene.selected else { return };
        let Some(guid) = self.scene.guid_of(row) else { return };
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
        let Some(p) = pick else {
            log::info!("pick: nothing");
            self.select(None);
            return;
        };
        match self.scene.resolve(p, &self.gpu) {
            Some(hit) => {
                match &hit.point {
                    Some(pt) => log::info!("pick: '{}' {} row {} point {} id {} at ({:.1}, {:.1}, {:.1})", hit.doc, hit.guid, hit.row, pt.local, pt.id, pt.position[0], pt.position[1], pt.position[2]),
                    None => log::info!("pick: '{}' {} row {}", hit.doc, hit.guid, hit.row),
                }
                let toggle = if self.scene.selected == Some(hit.row) { None } else { Some(hit.row) };
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
        if let Some(pick) = self.gpu.pick.poll() {
            self.apply_pick(pick);
        }
        self.needs_frame = false;
        if self.gpu.view.spin {
            self.camera.orbit(SPIN_STEP, 0.0);
        }
        let now_ms = now_ms();
        let origin = self.camera.origin();
        let rebase = self.gpu.rebase_anchor(&origin, self.camera.distance_world(), now_ms);
        let view_proj = self.camera.view_proj_anchored(self.aspect(), &rebase.anchor);
        let input = FrameInput { view_proj, clear: CLEAR, now_ms };
        self.dirty |= rebase.moved || self.gpu.view.perf || self.gpu.view.spin;

        let mut dropped = false;
        if self.dirty {
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
        self.needs_frame |= dropped || rebase.pending || self.gpu.pick.busy() || self.gpu.view.perf || self.gpu.view.spin;
    }

    /// The `?perf=1` line: frame number, gap since the previous frame, encode time, heap.
    #[cfg(target_arch = "wasm32")]
    fn perf_line(&self, gap_ms: f64, encode_ms: f64) {
        let line = format!("f{} gap {gap_ms:.0} enc {encode_ms:.1} ms heap {:.0} MB", self.gpu.performance.frames, heap_mb());
        crate::engine::performance::perf_line(&line);
    }

    /// Natively the perf line goes nowhere (the harness prints its own numbers).
    #[cfg(not(target_arch = "wasm32"))]
    fn perf_line(&self, _gap_ms: f64, _encode_ms: f64) {}
}
