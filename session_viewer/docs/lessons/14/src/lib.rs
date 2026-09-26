// --8<-- [start:entry]
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
    engine::performance::mark("wasm entry"); // a named point on the browser's performance timeline; register:frame
    start(); // open the window and the event loop; register:shell
    Ok(())
}
// --8<-- [end:entry]

// --8<-- [start:01-first-frame]
// --8<-- [start:shader-macro]
// `macro_rules!` makes a macro, code that writes code; it must come before the `mod` lines that use it.
/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader {
    // `$name:literal` matches one string literal, such as "background.wgsl".
    ($name:literal) => {
        // `include_str!` pastes the file into the binary at compile time; OUT_DIR is the folder build.rs wrote.
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

// `mod engine;` makes src/engine/mod.rs part of this crate.
mod engine;
// --8<-- [end:shader-macro]
// --8<-- [end:01-first-frame]

// --8<-- [start:02-camera]
// --8<-- [start:camera-mod]
mod camera;
// --8<-- [end:camera-mod]
// --8<-- [end:02-camera]

// --8<-- [start:06-app]
// --8<-- [start:app-mod]
pub mod app;
// --8<-- [end:app-mod]
// --8<-- [end:06-app]

// --8<-- [start:11-text-quality]
// --8<-- [start:text-quality-mod]
#[cfg(target_arch = "wasm32")]
pub mod text_quality;
// --8<-- [end:text-quality-mod]
// --8<-- [end:11-text-quality]

// --8<-- [start:12-shell]
// --8<-- [start:state-msg]
mod state;

use crate::app::scene::FileDoc;
pub use state::State;

/// Messages the async loader sends to the event loop.
pub enum Msg {
    Ready(Box<State>),                              // GPU is up, here is the state
    File(FileDoc, Option<String>), // one loaded file; a display-only one names its file
    Clear,                         // empty the scene
    Fit,                           // frame the camera on everything
    CancelPointer,                 // the browser lost the pointer
    Fonts(Vec<Vec<u8>>),           // the whole label fonts, main font first; register:loading
}
// --8<-- [end:state-msg]

// --8<-- [start:app-struct]
#[cfg(target_arch = "wasm32")]
use {
    crate::app::input::Input,
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
    input: Input,                       // mouse and key gestures
    pointer_cancellation: Option<app::input::PointerCancellation>, // browser pointer-lost listener
}
// --8<-- [end:app-struct]

// --8<-- [start:app-run]
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
            input: Input::new(),
            pointer_cancellation: None,
        };
        // a browser loop cannot block: `spawn_app` hands the app over and returns at once
        event_loop.spawn_app(app);
        Ok(())
    }

    /// Take the ready state, size it to the canvas, draw.
    fn adopt(&mut self, mut state: State) {
        // match the canvas pixel size
        if let Some((w, h)) = desired_canvas_size() {
            let _ = state.resize(w, h);
        }

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
// --8<-- [end:app-run]

// --8<-- [start:app-events]
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
            match app::input::PointerCancellation::new(canvas.clone(), proxy.clone()) {
                Ok(listener) => self.pointer_cancellation = Some(listener),
                Err(error) => log::warn!("Cannot register pointer cancellation: {error:?}"),
            }

            // async: GPU setup, then Msg::Ready
            wasm_bindgen_futures::spawn_local(app::loader::boot(window, proxy)); // register:loading
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
            Msg::Clear => state.clear(),
            Msg::Fit => state.fit_loaded(),
            Msg::File(doc, source) => state.append(doc, source),
            Msg::Fonts(faces) => self.use_fonts(faces),   // register:loading
            Msg::CancelPointer => {
                self.input.cancel();
                state.touch();
            }
        }

        self.request_if_needed();
    }

    /// Handle one window event: redraw, resize, key or mouse.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };

        // true when the scene must be drawn again
        let changed = match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                false
            }
            WindowEvent::RedrawRequested => {
                if page_hidden() || desired_canvas_size().is_none() {
                    return;
                }

                // resize first; a resize not ready yet holds the frame
                let held = match desired_canvas_size() {
                    Some((w, h)) if (w, h) != (state.gpu.config.width, state.gpu.config.height) => {
                        !state.resize(w, h)
                    }
                    _ => false,
                };

                if held {
                    state.needs_frame = true;
                } else {
                    state.render();
                }

                false
            }
            WindowEvent::Resized(_) => true,
            WindowEvent::KeyboardInput { event, .. } => {
                // first press only, and only while the canvas has focus
                viewer_focused()
                    && event.state == ElementState::Pressed
                    && !event.repeat
                    && self.input.key(state, event.logical_key.as_ref())
            }
            other => self.input.mouse(state, &other),
        };

        if changed {
            state.touch();
        }

        self.request_if_needed();
    }
}
// --8<-- [end:app-events]

// --8<-- [start:canvas]
/// The page element with id `canvas`.
#[cfg(target_arch = "wasm32")]
fn viewer_canvas() -> Option<web_sys::HtmlCanvasElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id("canvas")?
        .dyn_into()
        .ok()
}

/// True while the canvas has keyboard focus.
#[cfg(target_arch = "wasm32")]
fn viewer_focused() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };

    match document.active_element() {
        Some(element) => element.id() == "canvas",
        None => false,
    }
}

/// True while the browser tab is hidden.
#[cfg(target_arch = "wasm32")]
fn page_hidden() -> bool {
    let Some(window) = web_sys::window() else {
        return true;
    };

    match window.document() {
        Some(document) => document.hidden(),
        None => true,
    }
}

/// The canvas size in device pixels, `None` when zero.
#[cfg(target_arch = "wasm32")]
fn desired_canvas_size() -> Option<(u32, u32)> {
    let dpr = engine::gpu::view::device_pixel_ratio();
    let canvas = viewer_canvas()?;
    let w = (canvas.client_width() as f64 * dpr).round() as u32;
    let h = (canvas.client_height() as f64 * dpr).round() as u32;
    (w > 0 && h > 0).then_some((w, h))
}
// --8<-- [end:canvas]

// --8<-- [start:start]
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

    // after a GPU-loss reload, show the notice
    if let Some(notice) = app::route::adopt_recovery() {
        app::feedback::status(notice);
    }

    if let Err(error) = App::run() {
        app::feedback::error(&format!("Cannot start the viewer: {error}"));
    }
}
// --8<-- [end:start]
// --8<-- [end:12-shell]

// --8<-- [start:14-fonts]
// --8<-- [start:use-fonts]
#[cfg(target_arch = "wasm32")]
impl App {
    /// Keep the whole fonts for the page's life, shared by the labels and the panels.
    fn use_fonts(&mut self, faces: Vec<Vec<u8>>) {
        let Some(state) = &mut self.state else { return };
        let faces: Vec<&'static [u8]> = faces
            .into_iter()
            // `Box::leak` hands the bytes a 'static lifetime: they are never freed, which suits fonts kept for the page's life
            .map(|face| &*Box::leak(face.into_boxed_slice()))
            .collect();

        if let Ok(faces) = <[&'static [u8]; 3]>::try_from(faces) {
        }
    }
}
// --8<-- [end:use-fonts]
// --8<-- [end:14-fonts]
