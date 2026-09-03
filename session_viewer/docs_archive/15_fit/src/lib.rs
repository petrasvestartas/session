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

pub use state::State;
use crate::camera::View;

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseScrollDelta, WindowEvent, MouseButton};
use winit::keyboard::{Key, NamedKey};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

const SCENE_MIN: [f32; 3] = [-0.7, -0.4, -0.3];
const SCENE_MAX: [f32; 3] = [ 0.7,  0.5,  0.3];


// ── Browser event loop ──────────────────────────────────────────────────────
// State::new is async; winit's `resumed` is not, so we create the window, kick off async init,
// and deliver the finished State back as a user event (winit's documented wasm pattern).
pub struct App {
    state: Option<State>,
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    orbiting: bool,
    last_cursor: (f64, f64),
    ctrl: bool,
}

impl App {
    pub fn run() -> anyhow::Result<()> {
        use winit::platform::web::EventLoopExtWebSys;
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<State>::with_user_event().build()?;
        let app = App { 
            proxy: Some(event_loop.create_proxy()), 
            state: None,
            orbiting: false,
            last_cursor: (0.0, 0.0),
            ctrl: false,
         };
        event_loop.spawn_app(app);
        Ok(())
    }
}

impl ApplicationHandler<State> for App {

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
                let state = State::new(window).await.expect("State init failed");
                let _ = proxy.send_event(state);
            });
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut state: State) {
        let (w, h) = desired_canvas_size()
            .unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
        state.resize(w, h);
        state.window.request_redraw();
        self.state = Some(state);
    }

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
                        Key::Character("f" | "F") => {
                            let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64; 
                            state.camera.fit(SCENE_MIN, SCENE_MAX, aspect);
                        }
                        _ => {}

                    }
                }
            }

            WindowEvent::MouseInput {state: btn, button: MouseButton::Right, ..} => {
                self.orbiting = btn == ElementState::Pressed; // hold RMB to orbit
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.orbiting {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    if self.ctrl {
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
                state.camera.zoom(amount);
            }
            WindowEvent::ModifiersChanged(mods)=>{
                self.ctrl = mods.state().control_key();
            }
            _ => {},
        }
    }



}

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

#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
