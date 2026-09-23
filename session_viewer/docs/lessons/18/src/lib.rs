pub mod app;
mod camera;
mod engine;
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest;
mod state;
#[cfg(target_arch = "wasm32")]
pub mod text_quality;

use crate::app::scene::{FileDoc, StreamedInit};
use crate::app::walk::cloud::StreamRows;
pub use state::State;

/// One more slice of streamed cloud `idx`.
pub struct CloudChunk {
    pub idx: usize, // which cloud
    pub rows: StreamRows, // the new points
    pub to: u32, // rows loaded so far
}

/// Messages the async loader sends to the event loop.
pub enum Msg {
    Ready(Box<State>), // GPU is up, here is the state
    File(FileDoc),
    Texts(Vec<app::manifest::TextItem>), // text labels to place
    Clear, // empty the scene
    Fit, // frame the camera on everything
    StreamedCloud(Box<StreamedInit>),
    CloudChunk(CloudChunk), // more points arrived
    CloudQueryBatch(app::cloud_query::Batch), // points asked for on click
    CloudQueryResolved(app::cloud_query::Resolved), // those points answered
    CancelPointer, // the browser lost the pointer
}

#[cfg(target_arch = "wasm32")]
use {
    crate::app::{input::Input, loader},
    std::sync::Arc,
    wasm_bindgen::JsCast,
    wasm_bindgen::prelude::*,
    winit::application::ApplicationHandler,
    winit::event::{ElementState, WindowEvent},
    winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    winit::window::{Window, WindowId},
};

/// The winit application: owns the state and the gestures.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>, // everything drawn, once the GPU is up
    proxy: Option<EventLoopProxy<Msg>>, // sends messages into the loop
    input: Input, // mouse and key gestures
    pointer_cancellation: Option<app::input::PointerCancellation>, // browser pointer-lost listener
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
        // log::info! goes to the browser console
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
        let app = App {
            proxy: Some(event_loop.create_proxy()),
            state: None,
            input: Input::new(),
            pointer_cancellation: None,
        };
        event_loop.spawn_app(app);
        Ok(())
    }

    /// Take the ready state, size it to the canvas, draw.
    fn adopt(&mut self, mut state: State) {
        // match the canvas pixel size
        if let Some((w, h)) = desired_canvas_size() {
            state.resize(w, h);
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

#[cfg(target_arch = "wasm32")]
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
            match app::input::PointerCancellation::new(canvas, proxy.clone()) {
                Ok(listener) => self.pointer_cancellation = Some(listener),
                Err(error) => log::warn!("Cannot register pointer cancellation: {error:?}"),
            }

            // async: GPU setup, then Msg::Ready
            wasm_bindgen_futures::spawn_local(loader::boot(window, proxy));
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
            Msg::Fit => state.fit_all(),
            Msg::File(doc) => state.append(doc),
            Msg::Texts(texts) => state.set_texts(texts),
            Msg::StreamedCloud(init) => {
                // add the first rows, keep loading the rest
                let (url, fields, from, col_at) = (
                    init.url.clone(),
                    init.fields.clone(),
                    init.resident,
                    init.col_at,
                );
                let idx = state.add_streamed(*init);
                loader::spawn_stream_rest(loader::StreamCursor {
                    idx,
                    url,
                    fields,
                    from,
                    col_at,
                });
            }
            Msg::CloudChunk(c) => state.extend_streamed(c.idx, c.rows, c.to),
            Msg::CloudQueryBatch(batch) => state.cloud_query_batch(batch),
            Msg::CloudQueryResolved(resolved) => state.cloud_query_resolved(resolved),
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

                if let Some((w, h)) = desired_canvas_size()
                    && (w, h) != (state.gpu.config.width, state.gpu.config.height)
                {
                    state.resize(w, h);
                }

                state.render();
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

/// Browser entry point.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    // panics print to the console
    console_error_panic_hook::set_once();

    // the text-quality page runs its own code
    if let Some(window) = web_sys::window()
        && let Some(document) = window.document()
        && document.get_element_by_id("text-quality-canvas").is_some()
    {
        return Ok(());
    }

    if let Err(error) = App::run() {
        app::feedback::error(&format!("Cannot start the viewer: {error}"));
    }

    Ok(())
}
