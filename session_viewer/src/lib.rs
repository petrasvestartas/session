//! session_viewer — a browser-only (WebGPU/wgpu + winit) 3D viewer, grown one documented chapter
//! at a time. The module layout mirrors `session_viewer_archive` (engine → app → ui layers); see
//! that crate's `ARCHITECTURE.md` for the full map. Target: the browser canvas (wasm32) only —
//! the default build target is pinned in `.cargo/config.toml`, so there are no `cfg` gates.
//!
//! Chapter 1: a window that clears the screen.
//!   lib.rs        — the winit/browser shell: create the canvas window, run the event loop
//!   state.rs      — `State`, where the layered stack is wired
//!   engine/gpu.rs — `Gpu`, the wgpu device/queue/surface (lowest layer)

mod engine;
mod state;
mod camera;
mod app; // App layer for file loading
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest; // headless render harness - see src/selftest.rs

pub use state::State;
use crate::camera::View;
use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest, Scene};

// The scene: which sheets, and where each one sits.
// Fetched at runtime, so re-arringing the scene is a text edit in assets/scenes, not rebuild (app/scene.rs)
const DEMO_SCENE_URL: &str = "scenes/meshes.json";

/// Async init - event-loop messages.
/// `Ready` carries the State built around the first file
/// pixes in 2s, each file is one more parsed document appended live.
pub enum Msg {
    Ready(Box<State>),
    File(String, session_rust::Session, session_rust::Xform),
    /// A cloud is about to stream in: name, placement, and the EXACT point count - known from
    /// the file's packed-double length prefix before a single point has been fetched.
    CloudBegin(String, session_rust::Xform, u32),
    /// One slice of positions / the colour run, on their way to GPU memory. These Vecs are the
    /// only copy of that data that ever exists on the CPU, and they die in the handler.
    CloudPos(Vec<f32>),
    CloudCol(Vec<u32>),
    /// Done, with the cloud's local-space AABB for the camera fit.
    CloudEnd([f32; 3], [f32; 3]),
}

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent, MouseButton};
use winit::keyboard::{Key, NamedKey};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

// ── Browser event loop ──────────────────────────────────────────────────────
// State::new is async; winit's `resumed` is not, so we create the window, kick off async init,
// and deliver the finished State back as a user event (winit's documented wasm pattern).
/// The winit application handler: owns the viewer `State` once async init completes,
/// and tracks the mouse-orbit / modifier state between events.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<winit::event_loop::EventLoopProxy<Msg>>,
    orbiting: bool,
    panning: bool,
    last_cursor: (f64, f64),
    ctrl: bool,
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
        use winit::platform::web::EventLoopExtWebSys;
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
        let app = App {
            proxy: Some(event_loop.create_proxy()),
            state: None,
            orbiting: false,
            panning: false,
            last_cursor: (0.0, 0.0),
            ctrl: false,
         };
        event_loop.spawn_app(app);
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler<Msg> for App {

    /// Bind to the `#canvas` element and kick off async `State` init (delivered back via `user_event`).
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;

        if self.state.is_some() { return; }

        let canvas = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let attrs = Window::default_attributes().with_canvas(Some(canvas));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());

        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(async move {

                // Manifest, then the files.
                //
                // The canvas and the GPU come up FIRST, empty. A streamed cloud writes into GPU
                // buffers, so the GPU has to exist before the first byte of geometry is fetched -
                // and as a bonus the viewport is live immediately instead of after a parse.
                let t0 = crate::engine::performance::now_ms();
                let manifest_bytes = persistence::fetch_bytes(DEMO_SCENE_URL).await.unwrap_or_default();
                let manifest = Manifest::parse(&manifest_bytes).unwrap_or_else(|| panic!("cannot read the scene manifest at {DEMO_SCENE_URL}"));
                log::info!("scene '{}': {} items", manifest.name, manifest.items.len());
                let count = manifest.items.len();

                let state = State::new(window.clone(), Scene::new()).await.expect("State init failed");
                log::info!("canvas live {:.0}ms after manifest fetch", crate::engine::performance::now_ms() - t0);
                let _ = proxy.send_event(Msg::Ready(Box::new(state)));

                for (i, item) in manifest.items.iter().enumerate() {
                    let f0 = crate::engine::performance::now_ms();
                    let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
                    let named = if item.name.is_empty() { item.file.clone() } else { item.name.clone() };

                    // ── STREAMING PATH ──────────────────────────────────────────────────────
                    // A cloud-only .pb never becomes a kernel object and never exists whole in
                    // wasm memory. Two small Range reads find the packed arrays; the coords run
                    // then arrives in 8 MB slices, each converted, handed to the GPU and dropped.
                    if let Some(f) = persistence::cloud_fields(&item.file).await {
                        log::info!("streaming '{}': {} points | coords {:.0} MB + colours {:.0} MB",
                            named, f.count, f.coords_len as f64 / 1048576.0, f.colors_len as f64 / 1048576.0);
                        let _ = proxy.send_event(Msg::CloudBegin(named.clone(), place, f.count));

                        // 8 MB, rounded DOWN to a whole number of points: a slice boundary can
                        // then never fall inside a point, let alone inside one of its doubles.
                        // 8 MB, rounded DOWN to a whole number of points: a slice boundary can
                        // then never fall inside a point, let alone inside one of its doubles.
                        const SLICE: u64 = (8 * 1024 * 1024 / 24) * 24;
                        let (mut at, mut left) = (f.coords_at, f.coords_len);
                        let (mut lo, mut hi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);

                        // PIPELINED, and this is the whole performance story of the loader.
                        // `fetch_range(..).await` is itself a yield: the promise resolves off
                        // network I/O, so it cannot resume until the current FRAME is done. Eleven
                        // sequential slices therefore cost eleven frames - invisible at 100 fps,
                        // and 5.5 s when sheets are parsing and frames run 500-1100 ms. That, not
                        // the explicit yields, is why a 3.0 s stream became 7.5 s in the mixed
                        // scene. Keeping slice n+1 in flight while slice n converts hides the
                        // round trip AND the frame behind work we had to do anyway.
                        let mut inflight = if left > 0 {
                            persistence::fetch_range_start(&item.file, at, SLICE.min(left)).ok()
                        } else {
                            None
                        };
                        while let Some(f_in) = inflight.take() {
                            let n = SLICE.min(left);
                            at += n;
                            left -= n;
                            // next one on the wire BEFORE we spend time on this one
                            inflight = if left > 0 {
                                persistence::fetch_range_start(&item.file, at, SLICE.min(left)).ok()
                            } else {
                                None
                            };
                            let Ok(raw) = persistence::fetch_range_finish(f_in).await else { break };
                            let pos = persistence::positions_from(&raw);
                            drop(raw);
                            for q in pos.chunks_exact(3) {
                                for k in 0..3 { lo[k] = lo[k].min(q[k]); hi[k] = hi[k].max(q[k]); }
                            }
                            let _ = proxy.send_event(Msg::CloudPos(pos));
                        }
                        if let Some(col) = persistence::cloud_colors(&item.file, f.colors_at, f.colors_len, f.count).await {
                            let _ = proxy.send_event(Msg::CloudCol(col));
                        }
                        let _ = proxy.send_event(Msg::CloudEnd(lo, hi));
                        log::info!("streamed '{}' in {:.0}ms", named, crate::engine::performance::now_ms() - f0);
                        continue;
                    }

                    // ── WHOLE-FILE PATH ─────────────────────────────────────────────────────
                    // Everything that is not a lone point cloud still goes through prost.
                    let bytes = match persistence::fetch_start(&item.file) {
                        Ok(f) => persistence::fetch_finish(f).await.unwrap_or_default(),
                        _ => Vec::new(),
                    };
                    let f1 = crate::engine::performance::now_ms();
                    let nbytes = bytes.len();
                    let session = persistence::session_from_bytes_chunked(&item.file, bytes).await;
                    let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
                    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms",
                        name, session.lookup.len(), nbytes, f1 - f0, crate::engine::performance::now_ms() - f1);
                    if session.lookup.is_empty() {
                        continue; // failed fetch - skipped file
                    }
                    let _ = proxy.send_event(Msg::File(name, session, place));
                }

            });
        }
    }

    /// `Ready`: adopt the State built around the first file, size it, fit the camera, draw.
    /// `File` append one more document - walk it into the shared tabkles, re-upload, redraw.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg){
        match msg {
            Msg::Ready(state) => {
                let mut state = *state;
                let (w, h) = desired_canvas_size()
                    .unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
                state.resize(w, h);
                let aspect = w as f64 / h as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                state.window.request_redraw();
                self.state = Some(state);
            }
            // A cloud, streaming. Nothing here holds points: begin_cloud reserves the GPU
            // range from a count that is already known, each slice is written and dropped, and
            // the CPU keeps a name, a count and one instance row.
            Msg::CloudBegin(name, place, count) => {
                let Some(state) = &mut self.state else { return };
                let row = state.scene.begin_cloud(name, place, count);
                state.scene.upload_to(&mut state.gpu); // pushes the instance row
                state.gpu.cloud_begin(count, row);
            }
            Msg::CloudPos(pos) => {
                let Some(state) = &mut self.state else { return };
                state.gpu.cloud_pos(&pos);
                state.window.request_redraw(); // the cloud grows on screen as it arrives
            }
            Msg::CloudCol(col) => {
                let Some(state) = &mut self.state else { return };
                state.gpu.cloud_col(&col);
                state.window.request_redraw();
            }
            Msg::CloudEnd(lo, hi) => {
                let Some(state) = &mut self.state else { return };
                // lo/hi are the cloud's LOCAL box; place it before it can fit the camera.
                if let Some(slot) = state.scene.clouds.last() {
                    if let Some((xf, _, _)) = state.scene.tables.objects.get(slot.instance as usize) {
                        let (mut wlo, mut whi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
                        for c in 0..8u32 {
                            let corner = [
                                if c & 1 == 0 { lo[0] } else { hi[0] },
                                if c & 2 == 0 { lo[1] } else { hi[1] },
                                if c & 4 == 0 { lo[2] } else { hi[2] },
                            ];
                            let w = crate::app::scene::xform_point(xf, corner);
                            for k in 0..3 { wlo[k] = wlo[k].min(w[k]); whi[k] = whi[k].max(w[k]); }
                        }
                        state.gpu.grow_scene(wlo, whi);
                        state.scene.grow_bounds(wlo, whi);
                    }
                }
                let s = state.window.inner_size();
                let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                state.window.request_redraw();
            }
            Msg::File(name, session, place) => {
                let Some(state) = &mut self.state else {
                    return;
                };
                let t0 = crate::engine::performance::now_ms();
                state.scene.add_file(name, session, place);
                let t1 = crate::engine::performance::now_ms();
                state.scene.upload_to(&mut state.gpu);
                log::info!("appended: walk {:.0}ms · upload {:.0}ms | {} docs",
                    t1 - t0, crate::engine::performance::now_ms() - t1, state.scene.docs.len());
                state.window.request_redraw();
            }
        }
    }

    /// Handle one window event: redraw, resize, keyboard view shortcuts, and mouse orbit/pan/zoom.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = match &mut self.state { Some(s) => s, None => return };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // Before drawing, make the GPU surface match the canvas's real pixel size.
                // Cheap check every frame; reconfigure only on a genuine change.
                if let Some((w, h)) = desired_canvas_size() {
                    if (w, h) != (state.gpu.config.width, state.gpu.config.height) {
                        state.resize(w, h);
                    }
                }
                if let Err(e) = state.render() { log::error!("render: {e}"); }
            }
            WindowEvent::KeyboardInput { event, ..} => {
                if event.state == ElementState::Pressed && !event.repeat{
                    match event.logical_key.as_ref() {
                        Key::Named(NamedKey::Space) => state.camera.toggle_projection(),
                        Key::Character("1") => state.camera.set_view(View::Front),
                        Key::Character("2") => state.camera.set_view(View::Back),
                        Key::Character("3") => state.camera.set_view(View::Left),
                        Key::Character("4") => state.camera.set_view(View::Right),
                        Key::Character("5") => state.camera.set_view(View::Top),
                        Key::Character("6") => state.camera.set_view(View::Bottom),
                        Key::Character("7") => state.camera.set_view(View::Iso),
                        Key::Character("c" | "C") => state.camera.reset(),
                        // L toggles how the SOLID lane draws mesh/BRep edges: real 3D tubes, or
                        // camera-facing flat quads through the flat lane's own shader. Same
                        // instance table either way, so it is a free A/B at any zoom.
                        Key::Character("l" | "L") => {
                            use crate::engine::gpu::LineStyle;
                            state.gpu.line_style = match state.gpu.line_style {
                                LineStyle::Tubes => LineStyle::Flat,
                                LineStyle::Flat => LineStyle::Tubes,
                            };
                            log::info!("line style: {:?}", state.gpu.line_style);
                        }
                        Key::Character("f" | "F") => {
                            let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                            state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                        }
                        _ => {}

                    }
                }
            }

            WindowEvent::MouseInput {state: btn, button: MouseButton::Right, ..} => {
                self.orbiting = btn == ElementState::Pressed; // hold RMB to orbit
            }
            WindowEvent::MouseInput {state: btn, button: MouseButton::Middle, ..} => {
                self.panning = btn == ElementState::Pressed; // hold MMB to plan (CAD standard)
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.orbiting || self.panning {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    if self.panning || self.ctrl {
                        state.camera.pan(dx, dy);
                    } else {
                        state.camera.orbit(dx, dy)
                    };
                }
                self.last_cursor = (position.x, position.y);
            }
            WindowEvent::MouseWheel {delta, ..} => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 100.0,
                };
                // Zoom toward the curson - the point under the nouse stays put
                let vp = (state.gpu.config.width as f64, state.gpu.config.height as f64);
                state.camera.zoom_at(amount, self.last_cursor, vp);
            }
            WindowEvent::ModifiersChanged(mods)=>{
                self.ctrl = mods.state().control_key();
            }
            _ => {},
        }
    }



}

/// The canvas's pixel size (CSS size × device-pixel-ratio), or `None` if zero or unavailable.
#[cfg(target_arch = "wasm32")]
fn desired_canvas_size() -> Option<(u32, u32)> {
    use wasm_bindgen::JsCast;
    let win = web_sys::window()?;
    let dpr = win.device_pixel_ratio();
    let canvas = win.document()?
        .get_element_by_id("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
    let w = (canvas.client_width()  as f64 * dpr).round() as u32;
    let h = (canvas.client_height() as f64 * dpr).round() as u32;
    (w > 0 && h > 0).then_some((w, h))
}

/// wasm entry point: install the panic hook and run the app.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
