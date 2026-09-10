//! session_viewer - a browser-only (WebGPU/wgpu + winit) CAD viewer over `session_rust`.
//! This file is the shell only: the canvas window, the event loop and the `Msg` handlers,
//! each delegating to `State`. Loading is `app/loader.rs`; bindings are `app/input.rs`.

pub mod app;
mod camera;
mod engine;
pub mod math;
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest;
mod state;
#[cfg(target_arch = "wasm32")]
pub mod text_quality;

use crate::app::scene::{Doc, SheetInit, StreamedInit};
use crate::app::walk::cloud::StreamRows;
use crate::app::walk::sheet::SheetRows;
pub use state::State;

/// One more slice of streamed cloud `idx`: its rows and the point the cloud is resident up to.
pub struct CloudChunk {
    pub idx: usize,
    pub rows: StreamRows,
    pub to: u32,
}

/// One more slice of sheet `idx`: its rows and the segment the sheet is resident up to.
pub struct SheetChunk {
    pub idx: usize,
    pub rows: SheetRows,
    pub to: u32,
}

/// Async loader -> event-loop messages. `Ready` carries the `State` built around an empty
/// scene; everything after it changes the scene in place.
pub enum Msg {
    Ready(Box<State>),
    File(Doc),
    Texts(Vec<app::manifest::TextItem>),
    Clear,
    Fit,
    StreamedCloud(Box<StreamedInit>),
    CloudChunk(CloudChunk),
    CloudQueryBatch(app::cloud_query::Batch),
    CloudQueryResolved(app::cloud_query::Resolved),
    Sheet(Box<SheetInit>),
    SheetChunk(SheetChunk),
    SheetEntity(app::sheet_query::Resolved),
    CancelPointer,
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

/// The winit application handler: owns `State` once async init completes, and the gestures.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<EventLoopProxy<Msg>>,
    input: Input,
    pointer_cancellation: Option<app::input::PointerCancellation>,
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
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
        if let Some(state) = &self.state
            && state.needs_frame
        {
            state.window.request_redraw();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler<Msg> for App {
    /// Bind to the `#canvas` element and start the loader; `State` comes back as `Msg::Ready`.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() || self.proxy.is_none() {
            return;
        }
        let Some(canvas) = viewer_canvas() else {
            app::feedback::error("The viewer canvas is missing or invalid");
            return;
        };
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
            wasm_bindgen_futures::spawn_local(loader::boot(window, proxy));
        }
    }

    /// Every message after `Ready` changes the scene, so each one leaves `needs_frame` set.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
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
            Msg::Sheet(init) => {
                let (url, fields, from) = (init.url.clone(), init.fields.clone(), init.resident);
                let idx = state.add_sheet(*init);
                loader::spawn_sheet_rest(loader::SheetCursor {
                    idx,
                    url,
                    fields,
                    from,
                });
            }
            Msg::SheetChunk(c) => state.extend_sheet(c.idx, c.rows, c.to),
            Msg::SheetEntity(resolved) => state.sheet_entity(resolved),
            Msg::CancelPointer => {
                self.input.cancel();
                state.touch();
            }
        }
        self.request_if_needed();
    }

    /// Redraw and resize here; keys and the mouse go to `Input`, which says whether anything
    /// changed. A frame is requested only when something did.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };
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

/// The shell owns one canvas; no browser event handler searches unrelated page elements.
#[cfg(target_arch = "wasm32")]
fn viewer_canvas() -> Option<web_sys::HtmlCanvasElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id("canvas")?
        .dyn_into()
        .ok()
}

/// Keyboard shortcuts apply only while the viewer canvas owns browser focus.
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

/// Hidden tabs stop issuing rendering work; winit resumes redraw delivery when visible.
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

/// The canvas's pixel size (CSS size x device-pixel-ratio, capped by `?dpr=`), or `None` if
/// zero or unavailable.
#[cfg(target_arch = "wasm32")]
fn desired_canvas_size() -> Option<(u32, u32)> {
    let dpr = engine::gpu::view::device_pixel_ratio();
    let canvas = viewer_canvas()?;
    let w = (canvas.client_width() as f64 * dpr).round() as u32;
    let h = (canvas.client_height() as f64 * dpr).round() as u32;
    (w > 0 && h > 0).then_some((w, h))
}

/// wasm entry point: install the panic hook and run the app.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
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
