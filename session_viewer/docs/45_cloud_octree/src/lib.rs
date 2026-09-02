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
pub mod app; // App layer for file loading
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest; // headless render harness - see src/selftest.rs

pub use state::State;
use crate::camera::View;
use crate::app::persistence;
use crate::app::scene::{auto_grid, Manifest, Scene};

// The scene: which sheets, and where each one sits.
// Fetched at runtime, so re-arringing the scene is a text edit in assets/scenes, not rebuild (app/scene.rs)
const DEMO_SCENE_URL: &str = "scenes/bunny_drawings.toml";
// const DEMO_SCENE_URL: &str = "scenes/cloud_mix.toml"; // was bunny_drawings.json

/// The manifest to load: `?scene=<path under assets/>` when the page supplies one, else
/// [`DEMO_SCENE_URL`]. One build can therefore serve many scenes - the docs embed a single
/// 7.7 MB wasm in an iframe per example and vary only the query string.
///
/// The value is a path under `assets/`, exactly like a manifest's own `file` entries. It is
/// rejected unless it stays inside that tree: an absolute URL, a scheme, or any `..` segment
/// would let a page point the viewer at another origin.
fn scene_url() -> String {
    fn from_query() -> Option<String> {
        let search = web_sys::window()?.location().search().ok()?;
        let raw = search.strip_prefix('?')?;
        let value = raw
            .split('&')
            .find_map(|pair| pair.strip_prefix("scene="))?;
        let decoded = js_sys::decode_uri_component(value).ok()?.as_string()?;
        let safe = !decoded.is_empty()
            && !decoded.starts_with('/')
            && !decoded.contains("//")
            && !decoded.contains(':')
            && !decoded.split('/').any(|seg| seg == "..");
        safe.then_some(decoded)
    }
    from_query().unwrap_or_else(|| DEMO_SCENE_URL.to_string())
}

/// Async init - event-loop messages.
/// `Ready` carries the State built around the first file
/// pixes in 2s, each file is one more parsed document appended live.
pub enum Msg {
    Ready(Box<State>),
    CloudBegin(String, session_rust::Xform, u32, f32),
    CloudPos(Vec<f32>),
    CloudCol(Vec<u32>),
    CloudEnd([f32; 3], [f32; 3]),
    File(String, session_rust::Session, session_rust::Xform, f32, bool),
    /// Drop the current documents, keeping `State` - see [`reload_scene`].
    Clear,
}

thread_local! {
    /// A proxy kept past start-up so [`reload_scene`] can post files into the
    /// running event loop. `resumed` takes `self.proxy`, so without this copy
    /// there is no way back into the app once it is going.
    static RELOAD_PROXY: std::cell::RefCell<Option<winit::event_loop::EventLoopProxy<Msg>>> =
        const { std::cell::RefCell::new(None) };
}

/// Reload the scene in place: same canvas, same camera, new geometry.
///
/// The page calls this after rewriting a `.pb` (see the docs' Thebe cells) so an
/// edit redraws the MODEL instead of restarting the viewer - reloading the
/// iframe would rebuild the WebGPU device and throw away the view you had
/// framed. `url` is a manifest path under `assets/`, as with `?scene=`.
#[wasm_bindgen]
pub fn reload_scene(url: Option<String>) {
    let proxy = RELOAD_PROXY.with(|slot| slot.borrow().clone());
    let Some(proxy) = proxy else {
        log::warn!("reload_scene: viewer is not running yet");
        return;
    };
    let url = url.unwrap_or_else(scene_url);
    wasm_bindgen_futures::spawn_local(async move {
        let _ = proxy.send_event(Msg::Clear);
        load_manifest(url, move |name, session, place, px, only| {
            let _ = proxy.send_event(Msg::File(name, session, place, px, only));
        })
        .await;
    });
}

/// Fetch a manifest and hand every parsed file to `emit`, in manifest order.
///
/// Shared by start-up and [`reload_scene`]; the only difference between them is
/// that start-up builds `State` around the first file, so it cannot use this
/// directly for that one.
async fn load_manifest<F>(url: String, mut emit: F)
where
    F: FnMut(String, session_rust::Session, session_rust::Xform, f32, bool),
{
    let manifest_bytes = persistence::fetch_bytes(&url).await.unwrap_or_default();
    let Some(manifest) = Manifest::parse(&manifest_bytes) else {
        log::error!("cannot read the scene manifest at {url}");
        return;
    };
    let count = manifest.items.len();
    for (i, item) in manifest.items.iter().enumerate() {
        let bytes = persistence::fetch_bytes(&item.file).await.unwrap_or_default();
        let session = persistence::session_from_bytes_chunked(&item.file, &bytes).await;
        if session.lookup.is_empty() {
            continue;
        }
        let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
        let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
        emit(name, session, place, item.point_size as f32, item.display_only);
    }
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
    fitted: bool, // first geometry fits the camera; everything later only grows the extent
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
            fitted: false,
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
            RELOAD_PROXY.with(|slot| *slot.borrow_mut() = Some(proxy.clone()));
            wasm_bindgen_futures::spawn_local(async move {

                // Manifest, then the files - pipelined
                // fetsch_start is eager: the brwoser request for file n+1 is in flight while file n parses
                // and progressive - ready after the first file, every later streams in as a Msg::File
                let t0 = crate::engine::performance::now_ms();
                let scene_url = scene_url();
                let manifest_bytes = persistence::fetch_bytes(&scene_url).await.unwrap_or_default();
                let manifest = Manifest::parse(&manifest_bytes).unwrap_or_else(|| panic!("cannot read the scene manifest at {scene_url}"));
                log::info!("scene '{}': {} items", manifest.name, manifest.items.len());
                let count = manifest.items.len();
                // The canvas and the GPU come up FIRST, empty. A streamed cloud writes into
                // GPU buffers, so the GPU has to exist before the first byte of geometry is
                // fetched - and as a bonus the viewport is live immediately, not after a parse.
                let state = State::new(window.clone(), Scene::new()).await.expect("State init failed");
                log::info!("canvas live {:.0}ms after manifest fetch", crate::engine::performance::now_ms() - t0);
                let _ = proxy.send_event(Msg::Ready(Box::new(state)));

                // whole-file prefetch skips `stream` items - starting a plain GET on a 431 MB
                // scan would pull the entire body
                let prefetch = |it: &crate::app::scene::Item| (!it.stream).then(|| persistence::fetch_start(&it.file));
                let mut next = manifest.items.first().and_then(prefetch);
                for (i, item) in manifest.items.iter().enumerate() {
                    let f0 = crate::engine::performance::now_ms();
                    let cur = next.take();
                    next = manifest.items.get(i + 1).and_then(prefetch);
                    // ── STREAMING PATH ─────────────────────────────────────────────────
                    // A `stream` cloud never becomes a kernel object and never exists whole
                    // in wasm memory. Two small Range reads find the packed arrays; the
                    // coords run then arrives in 8 MB slices, each converted, handed to the
                    // GPU and dropped.
                    if item.stream {
                        let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
                        let named = if item.name.is_empty() { item.file.clone() } else { item.name.clone() };
                        let Some(f) = persistence::cloud_fields(&item.file).await else {
                            log::warn!("'{}': stream requested but no Range-addressable cloud found - skipped", named);
                            continue;
                        };
                        log::info!("streaming '{}': {} points | coords {:.0} MB + colours {:.0} MB",
                            named, f.count, f.coords_len as f64 / 1048576.0, f.colors_len as f64 / 1048576.0);
                        let _ = proxy.send_event(Msg::CloudBegin(named.clone(), place, f.count, item.point_size as f32));

                        // 8 MB, rounded DOWN to a whole number of points: a slice boundary can
                        // then never fall inside a point, let alone inside one of its doubles.
                        const SLICE: u64 = (8 * 1024 * 1024 / 24) * 24;
                        let (mut at, mut left) = (f.coords_at, f.coords_len);
                        let (mut lo, mut hi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);

                        // PIPELINED, and this is the whole performance story of the loader:
                        // `fetch_range(..).await` resolves off network I/O, so it cannot
                        // resume until the current FRAME is done - a sequential loop pays one
                        // frame per slice. Keeping slice n+1 in flight while slice n converts
                        // hides the round trip AND the frame behind work we had to do anyway.
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
                            // A real macrotask between slices. With a warm cache the fetch
                            // promises resolve as MICROtasks, which never let the browser paint -
                            // the same freeze the sliced prost parse exists to avoid.
                            persistence::next_tick().await;
                        }
                        if let Some(col) = persistence::cloud_colors(&item.file, f.colors_at, f.colors_len, f.count).await {
                            let _ = proxy.send_event(Msg::CloudCol(col));
                        }
                        let _ = proxy.send_event(Msg::CloudEnd(lo, hi));
                        log::info!("streamed '{}' in {:.0}ms", named, crate::engine::performance::now_ms() - f0);
                        continue;
                    }
                    let bytes = match cur {
                        Some(Ok(f)) => persistence::fetch_finish(f).await.unwrap_or_default(),
                        _ => Vec::new(),
                    };
                    let f1 = crate::engine::performance::now_ms();
                    let session = persistence::session_from_bytes_chunked(&item.file, &bytes).await;
                    let name = if item.name.is_empty() {
                        session.name.clone()
                    } else {
                        item.name.clone()
                    };
                    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), bytes.len(), f1 - f0, crate::engine::performance::now_ms() - f1);
                    if session.lookup.is_empty() {
                        continue; // failed fetch - skipped file
                    }
                    let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
                    let _ = proxy.send_event(Msg::File(name, session, place, item.point_size as f32, item.display_only));

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
                state.window.request_redraw(); // the scene is still empty - the first file fits
                self.state = Some(state);
            }
            Msg::Clear => {
                if let Some(state) = &mut self.state {
                    state.scene.clear(&mut state.gpu);
                    state.window.request_redraw();
                }
            }
            Msg::File(name, session, place, cloud_px, display_only) => {
                let Some(state) = &mut self.state else {
                    return;
                };
                let t0 = crate::engine::performance::now_ms();
                state.scene.add_file(name, session, place, cloud_px, display_only);
                let t1 = crate::engine::performance::now_ms();
                state.scene.upload_to(&mut state.gpu);
                if self.fitted {
                    state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);
                } else {
                    let s = state.window.inner_size();
                    let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                    state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                    self.fitted = true;
                }
                log::info!("appended: walk {:.0}ms · upload {:.0}ms | {} docs | heap {:.0} MB",
                    t1 - t0, crate::engine::performance::now_ms() - t1, state.scene.docs.len(),
                    crate::engine::performance::heap_mb());
                state.window.request_redraw();
            }
            // A cloud, streaming. Nothing here holds points: begin_cloud reserves the GPU
            // range from a count that is already known, each slice is written and dropped,
            // and the CPU keeps a name, a count and one instance row.
            Msg::CloudBegin(name, place, count, px) => {
                let Some(state) = &mut self.state else { return };
                let row = state.scene.begin_cloud(name, place, count, px);
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
                    let (mut wlo, mut whi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
                    for c in 0..8u32 {
                        let corner = [
                            if c & 1 == 0 { lo[0] } else { hi[0] },
                            if c & 2 == 0 { lo[1] } else { hi[1] },
                            if c & 4 == 0 { lo[2] } else { hi[2] },
                        ];
                        let w = crate::app::scene::xform_point(&slot.place.m, corner);
                        for k in 0..3 { wlo[k] = wlo[k].min(w[k]); whi[k] = whi[k].max(w[k]); }
                    }
                    state.gpu.grow_scene(wlo, whi);
                    state.scene.grow_bounds(wlo, whi);
                }
                // a finished scan is the dominant geometry - refit around everything so far
                let s = state.window.inner_size();
                let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                self.fitted = true;
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
                        Key::Named(NamedKey::Space) => {
                            let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                            state.camera.toggle_projection_framed(state.gpu.scene_min, state.gpu.scene_max, aspect);
                        }
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
                        // Q / W / E hide a whole KIND of thing, so an overlap can be taken
                        // apart by eye: points, then lines and polylines, then mesh/BRep edges.
                        // E is the one to reach for on a model that draws its own outlines as
                        // polylines: the mesh topology gives every edge a second time, and two
                        // strokes a fraction of a pixel apart read as one thick ragged line.
                        Key::Character("q" | "Q") => {
                            state.gpu.show_points = !state.gpu.show_points;
                            log::info!("points: {}", state.gpu.show_points);
                            state.window.request_redraw();
                        }
                        Key::Character("w" | "W") => {
                            state.gpu.show_lines = !state.gpu.show_lines;
                            log::info!("lines: {}", state.gpu.show_lines);
                            state.window.request_redraw();
                        }
                        Key::Character("e" | "E") => {
                            state.gpu.show_mesh_edges = !state.gpu.show_mesh_edges;
                            log::info!("mesh edges: {}", state.gpu.show_mesh_edges);
                            state.window.request_redraw();
                        }
                        Key::Character("l" | "L") => {
                            use crate::engine::gpu::LineStyle;
                            state.gpu.line_style = match state.gpu.line_style {
                                LineStyle::Tubes => LineStyle::Flat,
                                LineStyle::Flat => LineStyle::Tubes,
                            };
                            log::info!("line style: {:?}", state.gpu.line_style);
                        }
                        // live cloud point size
                        Key::Character("[") => {
                            state.gpu.cloud_size = (state.gpu.cloud_size - 0.25).max(0.25);
                            log::info!("cloud size scale: x{}", state.gpu.cloud_size);
                        }
                        Key::Character("]") => {
                            state.gpu.cloud_size = (state.gpu.cloud_size + 0.25).min(8.0);
                            log::info!("cloud size scale: x{}", state.gpu.cloud_size);
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
