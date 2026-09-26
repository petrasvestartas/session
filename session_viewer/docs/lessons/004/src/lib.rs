// --8<-- [start:000-entry]
// `#[cfg(...)]` keeps the next item only when the condition holds: here, only in the browser build.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// The browser runs this once the module has loaded.
#[cfg(target_arch = "wasm32")]
// wasm-bindgen writes the JavaScript glue around the module; `start` makes that glue call this function.
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    // a panic then prints its message to the browser console instead of a bare `unreachable`
    console_error_panic_hook::set_once();
    start(); // open the window and the event loop; register:shell
    Ok(())
}
// --8<-- [end:000-entry]

// --8<-- [start:001-modules]
// --8<-- [start:002-engine]
// `mod engine;` makes src/engine/mod.rs part of this crate.
mod engine; // register:gpu
// --8<-- [end:002-engine]

pub mod app;

mod state;

pub use state::State;
// --8<-- [end:001-modules]

// --8<-- [start:001-messages]
/// Messages the async loader sends to the event loop.
pub enum Msg {
    Ready(Box<State>),                              // GPU is up, here is the state
}
// --8<-- [end:001-messages]

// --8<-- [start:001-app]
#[cfg(target_arch = "wasm32")]
use {
    std::sync::Arc,
    wasm_bindgen::JsCast,
    winit::application::ApplicationHandler,
    winit::event::{ElementState, WindowEvent},
    winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    winit::window::{Window, WindowId},
};

/// The application: winit owns the event loop and calls its methods with every event.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,               // everything drawn, once the GPU is up
    proxy: Option<EventLoopProxy<Msg>>, // sends messages into the loop
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
        // log::info! goes to the browser console
        console_log::init_with_level(log::Level::Info).ok();
        // `with_user_event` lets our own `Msg` values travel through the loop beside the window events.
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
        let app = App {
            proxy: Some(event_loop.create_proxy()),
            state: None,
        };
        // a browser loop cannot block: `spawn_app` hands the app over and returns at once
        event_loop.spawn_app(app);
        Ok(())
    }

    /// Take the ready state, size it to the canvas, draw.
    fn adopt(&mut self, mut state: State) {
        state.window.request_redraw();
        self.state = Some(state);
    }

    /// Ask for a redraw only when something changed.
    fn request_if_needed(&self) {
        if let Some(state) = &self.state
            && state.needs_frame
        {
            state.window.request_redraw();
        }
    }
}
// --8<-- [end:001-app]

// --8<-- [start:001-events]
#[cfg(target_arch = "wasm32")]
// winit calls `resumed` once, `user_event` for each `Msg` and `window_event` for each input or redraw.
impl ApplicationHandler<Msg> for App {
    /// Bind the window to the page canvas and start loading.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // runs once
        if self.state.is_some() || self.proxy.is_none() {
            return;
        }

        let Some(canvas) = viewer_canvas() else {
            app::feedback::error("The viewer canvas is missing or invalid");
            return;
        };
        // the winit window is the page canvas
        let attrs = Window::default_attributes().with_canvas(Some(canvas.clone()));
        let window = match event_loop.create_window(attrs) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                app::feedback::error(&format!("Cannot initialize the viewer window: {error}"));
                return;
            }
        };

        if let Some(proxy) = self.proxy.take() {
            // async: GPU setup, then Msg::Ready
            wasm_bindgen_futures::spawn_local(app::loader::boot(window, proxy)); // register:boot
        }
    }

    /// Apply one loader message to the scene.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
        // Ready is the only message without a state yet
        let msg = match msg {
            Msg::Ready(state) => return self.adopt(*state),
            other => other,
        };
        let Some(state) = &mut self.state else { return };

        match msg {
            Msg::Ready(_) => {}
        }

        self.request_if_needed();
    }

    /// Handle one window event: redraw, resize, key or mouse.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {

        if self.state.is_none() {
            return;
        }

        // true when the scene must be drawn again
        let changed = match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                false
            }
// --8<-- [start:004-events]
            WindowEvent::RedrawRequested => self.redraw(), // register:redraw
            WindowEvent::Resized(_) => true, // register:redraw
// --8<-- [end:004-events]
            other => self.mouse(&other),
        };

        if changed && let Some(state) = &mut self.state {
            state.touch();
        }

        self.request_if_needed();
    }
}
// --8<-- [end:001-events]

// --8<-- [start:001-mouse]
#[cfg(target_arch = "wasm32")]
impl App {
    /// A pointer event; true when the picture changed.
    fn mouse(&mut self, event: &WindowEvent) -> bool {
        let Some(state) = &mut self.state else {
            return false;
        };
        let mut changed = false;
        changed
    }
}
// --8<-- [end:001-mouse]

// --8<-- [start:001-canvas]
/// The page element with id `canvas`.
#[cfg(target_arch = "wasm32")]
fn viewer_canvas() -> Option<web_sys::HtmlCanvasElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id("canvas")?
        .dyn_into()
        .ok()
}
// --8<-- [end:001-canvas]

// --8<-- [start:001-start]
/// Start the viewer, unless this is the text-quality page.
#[cfg(target_arch = "wasm32")]
fn start() {
    // the text-quality page runs its own code
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && document.get_element_by_id("text-quality-canvas").is_some()
    {
        return;
    }

    if let Err(error) = App::run() {
        app::feedback::error(&format!("Cannot start the viewer: {error}"));
    }
}
// --8<-- [end:001-start]

// --8<-- [start:004-redraw]
#[cfg(target_arch = "wasm32")]
impl App {
    /// The browser asked for a frame: resize first, then draw; a resize not ready yet holds the frame.
    fn redraw(&mut self) -> bool {
        let Some(state) = &mut self.state else {
            return false;
        };

        state.render();
        false
    }
}
// --8<-- [end:004-redraw]
