//! session_viewer - a browser-only (WebGPU/wgpu + winit) 3D viewer, grown one documented chapter
//! at a time. This file is the shell only: the canvas window, the event loop and the `Msg`
//! handlers, each delegating to `State`. Loader: `app/loader.rs`; bindings: `app/input.rs`.

mod engine;
mod state;
mod camera;
pub mod math;
pub mod app;
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest; // headless render harness - see src/selftest.rs

pub use state::State;
use crate::app::scene::{CloudBegin, FileDoc};
use crate::math::Aabb;

/// Async init -> event-loop messages. `Ready` carries the `State` built around an EMPTY scene;
/// each `File` is one more parsed document appended live; the `Cloud*` messages are a streamed
/// cloud's slices; `Clear` drops the documents, keeping `State` (see `loader::reload_scene`).
pub enum Msg {
    Ready(Box<State>),
    File(FileDoc),
    CloudBegin(CloudBegin),
    CloudPos(Vec<f32>),
    CloudCol(Vec<u32>),
    CloudEnd(Aabb),
    Clear,
}

#[cfg(target_arch = "wasm32")]
use {
    std::sync::Arc, wasm_bindgen::prelude::*, wasm_bindgen::JsCast, winit::application::ApplicationHandler,
    winit::event::{ElementState, WindowEvent}, winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys}, winit::window::{Window, WindowId}, crate::app::{input::Input, loader},
};

/// The winit application handler: owns the viewer `State` once async init completes, the
/// gesture state, and whether the camera has framed geometry yet.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<EventLoopProxy<Msg>>,
    input: Input,
    fitted: bool, // first geometry fits the camera; everything later only grows the extent
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
        let app = App { proxy: Some(event_loop.create_proxy()), state: None, input: Input::new(), fitted: false };
        event_loop.spawn_app(app);
        Ok(())
    }

    /// `Ready`: adopt the State, size it to the canvas, draw - the scene is still empty.
    fn adopt(&mut self, mut state: State) {
        let (w, h) = desired_canvas_size().unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
        state.resize(w, h);
        state.window.request_redraw();
        self.state = Some(state);
    }

    /// The one place a frame is asked for: whenever the handler that just ran left
    /// `needs_frame` set. Render on demand - a still scene asks for nothing.
    fn request_if_needed(&self) {
        if let Some(state) = &self.state {
            if state.needs_frame {
                state.window.request_redraw();
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler<Msg> for App {
    /// Bind to the `#canvas` element and start the loader; `State` comes back as `Msg::Ready`.
    /// `State::new` is async and winit's `resumed` is not - the documented wasm pattern.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() { return; }
        let canvas = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let attrs = Window::default_attributes().with_canvas(Some(canvas));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        if let Some(proxy) = self.proxy.take() {
            loader::keep_proxy(&proxy);
            wasm_bindgen_futures::spawn_local(loader::boot(window, proxy));
        }
    }

    /// Every message after `Ready` drives `State`; each one changes the scene, so each one
    /// leaves `needs_frame` set. The first document (or a finished scan) fits the camera;
    /// later ones only grow its extent.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
        let msg = match msg { Msg::Ready(state) => return self.adopt(*state), other => other };
        let Some(state) = &mut self.state else { return };
        match msg {
            Msg::Ready(_) => {}
            Msg::Clear => state.scene.clear(&mut state.gpu),
            Msg::File(doc) => {
                state.append(doc);
                if self.fitted { state.camera.grow_extent(state.gpu.bounds.min, state.gpu.bounds.max) } else { state.fit_all() }
                self.fitted = true;
            }
            Msg::CloudBegin(c) => state.cloud_begin(c),
            Msg::CloudPos(pos) => state.gpu.cloud_pos(&pos), // the cloud grows on screen as it arrives
            Msg::CloudCol(col) => state.gpu.cloud_col(&col),
            Msg::CloudEnd(local) => { state.cloud_end(&local); self.fitted = true; }
        }
        state.needs_frame = true;
        self.request_if_needed();
    }

    /// Redraw and resize here; keys and the mouse go to `Input`, which says whether anything
    /// changed. A frame is requested only when something did (`request_if_needed`).
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };
        let changed = match event {
            WindowEvent::CloseRequested => { event_loop.exit(); false }
            WindowEvent::RedrawRequested => {
                // Make the GPU surface match the canvas's real pixel size before drawing:
                // a cheap check every frame, a reconfigure only on a genuine change.
                if let Some((w, h)) = desired_canvas_size() {
                    if (w, h) != (state.gpu.config.width, state.gpu.config.height) {
                        state.resize(w, h);
                    }
                }
                if let Err(e) = state.render() { log::error!("render: {e}"); }
                false // `render` decides on its own whether the next frame is due
            }
            WindowEvent::Resized(_) => true, // the canvas changed; the redraw above resizes the surface
            WindowEvent::KeyboardInput { event, .. } => {
                event.state == ElementState::Pressed && !event.repeat && self.input.key(state, event.logical_key.as_ref())
            }
            other => self.input.mouse(state, &other),
        };
        if changed { state.needs_frame = true; }
        self.request_if_needed();
    }
}

/// The canvas's pixel size (CSS size × device-pixel-ratio), or `None` if zero or unavailable.
#[cfg(target_arch = "wasm32")]
fn desired_canvas_size() -> Option<(u32, u32)> {
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
