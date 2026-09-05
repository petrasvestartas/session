//! session_viewer - a browser-only (WebGPU/wgpu + winit) CAD viewer over `session_rust`.
//! This file is the shell only: the canvas window, the event loop and the `Msg` handlers,
//! each delegating to `State`. Loading is `app/loader.rs`.

mod engine;
mod state;
pub mod app;

pub use state::State;

/// Async loader -> event-loop messages. `Ready` carries the `State` built around an empty
/// scene.
pub enum Msg {
    Ready(Box<State>),
}

#[cfg(target_arch = "wasm32")]
use {
    crate::app::loader,
    std::sync::Arc,
    wasm_bindgen::prelude::*,
    wasm_bindgen::JsCast,
    winit::application::ApplicationHandler,
    winit::event::WindowEvent,
    winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    winit::window::{Window, WindowId},
};

/// The winit application handler: owns `State` once async init completes.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<EventLoopProxy<Msg>>,
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
        let app = App { proxy: Some(event_loop.create_proxy()), state: None };
        event_loop.spawn_app(app);
        Ok(())
    }

    /// `Ready`: adopt the State, size it to the canvas, draw.
    fn adopt(&mut self, mut state: State) {
        if let Some((w, h)) = desired_canvas_size() {
            state.resize(w, h);
        }
        state.window.request_redraw();
        self.state = Some(state);
    }

    /// The one place a frame is asked for: whenever a handler left `needs_frame` set.
    fn request_if_needed(&self) {
        if let Some(state) = &self.state && state.needs_frame {
            state.window.request_redraw();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler<Msg> for App {
    /// Bind to the `#canvas` element and start the loader; `State` comes back as `Msg::Ready`.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let attrs = Window::default_attributes().with_canvas(Some(canvas));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(loader::boot(window, proxy));
        }
    }

    /// The one message, `Ready`: the loader hands over the State.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
        match msg {
            Msg::Ready(state) => self.adopt(*state),
        }
    }

    /// Redraw and resize here. A frame is requested only when something changed.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };
        let changed = match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                false
            }
            WindowEvent::RedrawRequested => {
                if let Some((w, h)) = desired_canvas_size() && (w, h) != (state.gpu.config.width, state.gpu.config.height) {
                    state.resize(w, h);
                }
                state.render();
                false
            }
            WindowEvent::Resized(_) => true,
            _ => false,
        };
        if changed {
            state.needs_frame = true;
        }
        self.request_if_needed();
    }
}

/// The canvas's pixel size (CSS size x device-pixel-ratio), or `None` if zero or unavailable.
#[cfg(target_arch = "wasm32")]
fn desired_canvas_size() -> Option<(u32, u32)> {
    let win = web_sys::window()?;
    let dpr = win.device_pixel_ratio();
    let canvas = win.document()?.get_element_by_id("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
    let w = (canvas.client_width() as f64 * dpr).round() as u32;
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
