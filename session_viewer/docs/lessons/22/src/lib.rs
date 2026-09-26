#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Browser entry point.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    // panics print to the console
    console_error_panic_hook::set_once();
    engine::performance::mark("wasm entry"); // register:frame
    start(); // register:shell
    Ok(())
}

// --8<-- [start:01]
/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader {
    ($name:literal) => {
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

mod engine;
// --8<-- [end:01]

// --8<-- [start:02]
mod camera;
// --8<-- [end:02]

// --8<-- [start:06]
pub mod app;
// --8<-- [end:06]

// --8<-- [start:11]
#[cfg(target_arch = "wasm32")]
pub mod text_quality;
// --8<-- [end:11]

// --8<-- [start:12]
mod state;

use crate::app::scene::FileDoc;
pub use state::State;

/// Messages the async loader sends to the event loop.
pub enum Msg {
    Ready(Box<State>),                              // GPU is up, here is the state
    File(FileDoc, Option<String>), // one loaded file; a display-only one names its file
    Texts(Vec<app::manifest::TextItem>), // text labels to place; register:scene_text
    Clear,                         // empty the scene
    Fit,                           // frame the camera on everything
    StreamedCloud(Box<StreamedInit>), // a point cloud starts streaming; register:stream
    CloudChunk(CloudChunk),        // more points arrived; register:stream
    CloudQueryBatch(app::cloud_query::Batch), // points asked for on click; register:cloud_query
    CloudQueryResolved(app::cloud_query::Resolved), // those points answered; register:cloud_query
    Sheet(Box<SheetInit>),         // a drawing sheet starts streaming; register:sheets
    SheetChunk(SheetChunk),        // more segments arrived; register:sheets
    SheetEntity(app::sheet_query::Resolved), // a picked sheet entity answered; register:sheets
    CancelPointer,                 // the browser lost the pointer
    Hydrated(Box<app::scene::Hydrated>), // a released document's objects are back; register:editing
    Fonts(Vec<Vec<u8>>),           // the whole label fonts, main font first; register:loading
}

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

/// The winit application: owns the state and the gestures.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,               // everything drawn, once the GPU is up
    proxy: Option<EventLoopProxy<Msg>>, // sends messages into the loop
    input: Input,                       // mouse and key gestures
    pointer_cancellation: Option<app::input::PointerCancellation>, // browser pointer-lost listener
    ui: Option<app::ui::Ui>,            // the egui panels; register:egui
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
            ui: None,    // register:egui
        };
        event_loop.spawn_app(app);
        Ok(())
    }

    /// Take the ready state, size it to the canvas, draw.
    fn adopt(&mut self, mut state: State) {
        // match the canvas pixel size
        if let Some((w, h)) = desired_canvas_size() {
            let _ = state.resize(w, h);
        }

        self.adopt_panels(&mut state); // register:egui
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
            Msg::Hydrated(back) => state.hydrated(*back), // register:editing
            Msg::Fonts(faces) => self.use_fonts(faces),   // register:loading
            Msg::Texts(texts) => state.set_texts(texts),  // register:scene_text
            Msg::StreamedCloud(init) => start_stream(state, init), // register:stream
            Msg::CloudChunk(c) => state.extend_streamed(c.idx, c.rows, c.to), // register:stream
            Msg::CloudQueryBatch(batch) => state.cloud_query_batch(batch), // register:cloud_query
            Msg::CloudQueryResolved(resolved) => state.cloud_query_resolved(resolved), // register:cloud_query
            Msg::Sheet(init) => start_sheet(state, init), // register:sheets
            Msg::SheetChunk(c) => state.extend_sheet(c.idx, c.rows, c.to), // register:sheets
            Msg::SheetEntity(resolved) => state.sheet_entity(resolved), // register:sheets
            Msg::CancelPointer => {
                state.cancel_gesture(); // register:editing
                self.input.cancel();
                state.touch();
            }
        }

        self.request_if_needed();
    }

    /// Handle one window event: redraw, resize, key or mouse.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let taken = self.panels_take(&event); // the panels get the event first; register:egui
        let Some(state) = &mut self.state else { return };

        // true when the scene must be drawn again
        let changed = match event {
            _ if taken => false, // register:egui
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
                    // panels lay out, then the scene draws; register:egui
                    let repaint = self.ui.as_mut().is_some_and(|ui| ui.frame(state)); // register:egui
                    state.render();
                    repaint_if(state, repaint); // register:egui
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
// --8<-- [end:12]

// --8<-- [start:14]
#[cfg(target_arch = "wasm32")]
impl App {
    /// Keep the whole fonts for the page's life, shared by the labels and the panels.
    fn use_fonts(&mut self, faces: Vec<Vec<u8>>) {
        let Some(state) = &mut self.state else { return };
        let faces: Vec<&'static [u8]> = faces
            .into_iter()
            .map(|face| &*Box::leak(face.into_boxed_slice()))
            .collect();

        if let Ok(faces) = <[&'static [u8]; 3]>::try_from(faces) {
            state.use_fonts(faces); // register:scene_text
            self.panel_fonts(faces); // register:egui
        }
    }
}
// --8<-- [end:14]

// --8<-- [start:15]
use crate::app::scene::StreamedInit;
use crate::app::walk::cloud::StreamRows;

/// The next slice of streamed cloud `idx`.
pub struct CloudChunk {
    pub idx: usize,       // which cloud
    pub rows: StreamRows, // the new points
    pub to: u32,          // rows loaded so far
}

/// Add a streamed cloud's first rows and keep loading the rest.
#[cfg(target_arch = "wasm32")]
fn start_stream(state: &mut State, init: Box<StreamedInit>) {
    let (url, fields, from, col_at) = (
        init.url.clone(),
        init.fields.clone(),
        init.resident,
        init.col_at,
    );
    let idx = state.add_streamed(*init);
    app::loader::spawn_stream_rest(app::loader::StreamCursor {
        idx,
        url,
        fields,
        from,
        col_at,
    });
}
// --8<-- [end:15]

// --8<-- [start:19]
use crate::app::scene::SheetInit;
use crate::app::walk::sheet::SheetRows;

/// The next slice of sheet `idx`.
pub struct SheetChunk {
    pub idx: usize,      // which sheet
    pub rows: SheetRows, // the new segments
    pub to: u32,         // segments loaded so far
}

/// Add a sheet's first segments and keep loading the rest.
#[cfg(target_arch = "wasm32")]
fn start_sheet(state: &mut State, init: Box<SheetInit>) {
    let (url, fields, from) = (init.url.clone(), init.fields.clone(), init.resident);
    let idx = state.add_sheet(*init);
    app::loader::spawn_sheet_rest(app::loader::SheetCursor {
        idx,
        url,
        fields,
        from,
    });
}
// --8<-- [end:19]

// --8<-- [start:22]
#[cfg(target_arch = "wasm32")]
impl App {
    /// The egui panels and their GPU painter.
    fn adopt_panels(&mut self, state: &mut State) {
        self.ui = Some(app::ui::Ui::new(&state.window, state.logical_size()[0]));
        state.gpu.ui = Some(engine::gpu::ui::Ui::new(
            &state.gpu.ctx,
            state.gpu.config.format,
        ));
    }

    /// The panels take the fonts too.
    fn panel_fonts(&mut self, faces: [&'static [u8]; 3]) {
        if let Some(ui) = self.ui.as_mut() {
            ui.use_fonts(faces);
        }
    }

    /// The panels get the event first; true when they took it.
    fn panels_take(&mut self, event: &WindowEvent) -> bool {
        let Some(state) = &mut self.state else {
            return false;
        };
        let Some(ui) = self.ui.as_mut() else {
            return false;
        };
        let (mut consumed, repaint) = ui.event(&state.window, event);

        // keys reach the viewer unless a text field or a menu has them; the number box from its click on
        if matches!(event, WindowEvent::KeyboardInput { .. }) {
            consumed = app::ui::keys_taken() || state.number_box_open();
        }

        if repaint {
            state.request_frame();
        }

        // a command following a left drag, e.g. a lasso, keeps the pointer over panels too
        let held = self.input.tool_held()
            && matches!(
                event,
                WindowEvent::CursorMoved { .. }
                    | WindowEvent::MouseInput {
                        button: winit::event::MouseButton::Left,
                        ..
                    }
            );

        if consumed && !held {
            // a release inside a panel ends any viewer drag
            if matches!(
                event,
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    ..
                } | WindowEvent::Touch(winit::event::Touch {
                    phase: winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled,
                    ..
                })
            ) {
                self.input.cancel();
                state.cancel_gesture();
            }

            return true;
        }

        false
    }
}

/// The panels asked for another frame.
#[cfg(target_arch = "wasm32")]
fn repaint_if(state: &mut State, repaint: bool) {
    if repaint {
        state.request_frame();
    }
}
// --8<-- [end:22]
