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
                let mut next = manifest.items.first().map(|it| persistence::fetch_start(&it.file));
                let mut sent_ready = false;
                for (i, item) in manifest.items.iter().enumerate() {
                    let f0 = crate::engine::performance::now_ms();
                    let cur = next.take();
                    next = manifest.items.get(i + 1).map(|it| persistence::fetch_start(&it.file));
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
                    if !sent_ready {
                        sent_ready = true;
                        let mut scene = Scene::new();
                        scene.add_file(name, session, place, item.point_size as f32, item.display_only);
                        let state = State::new(window.clone(), scene).await.expect("State init failed");
                        log::info!("first file on screen {:.0}ms after manifest fetch", crate::engine::performance::now_ms() - t0);
                        let _ = proxy.send_event(Msg::Ready(Box::new(state)));
                    } else {
                        let _ = proxy.send_event(Msg::File(name, session, place, item.point_size as f32, item.display_only));
                    }

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
                state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);
                log::info!("appended: walk {:.0}ms · upload {:.0}ms | {} docs | heap {:.0} MB",
                    t1 - t0, crate::engine::performance::now_ms() - t1, state.scene.docs.len(),
                    crate::engine::performance::heap_mb());
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
