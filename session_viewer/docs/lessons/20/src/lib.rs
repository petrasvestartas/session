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
    engine::performance::mark("wasm entry"); // a named point on the browser's performance timeline; register:frame
    start(); // open the window and the event loop; register:shell
    Ok(())
}
// --8<-- [end:000-entry]

// `macro_rules!` makes a macro, code that writes code; it must come before the `mod` lines that use it.
/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader { // register:shaders
    // `$name:literal` matches one string literal, such as "background.wgsl".
    ($name:literal) => {
        // `include_str!` pastes the file into the binary at compile time; OUT_DIR is the folder build.rs wrote.
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

// --8<-- [start:001-modules]
// --8<-- [start:002-engine]
// `mod engine;` makes src/engine/mod.rs part of this crate.
mod engine; // register:gpu
// --8<-- [end:002-engine]

mod camera; // register:camera

pub mod app;

#[cfg(target_arch = "wasm32")] // register:text_quality
pub mod text_quality; // register:text_quality

mod state;

use crate::app::scene::FileDoc; // register:scene
pub use state::State;
// --8<-- [end:001-modules]

// --8<-- [start:001-messages]
/// Messages the async loader sends to the event loop.
pub enum Msg {
    Ready(Box<State>),                              // GPU is up, here is the state
    File(FileDoc, Option<String>), // one loaded file; a display-only one names its file; register:scene
    Texts(Vec<app::manifest::TextItem>), // text labels to place; register:scene_text
    Clear,                         // empty the scene; register:scene
    Fit,                           // frame the camera on everything; register:scene
    StreamedCloud(Box<StreamedInit>), // a point cloud starts streaming; register:stream
    CloudChunk(CloudChunk),        // more points arrived; register:stream
    CloudQueryBatch(app::cloud_query::Batch), // points asked for on click; register:cloud_query
    CloudQueryResolved(app::cloud_query::Resolved), // those points answered; register:cloud_query
    Sheet(Box<SheetInit>),         // a drawing sheet starts streaming; register:sheets
    SheetChunk(SheetChunk),        // more segments arrived; register:sheets
    SheetEntity(app::sheet_query::Resolved), // a picked sheet entity answered; register:sheets
    CancelPointer,                 // the browser lost the pointer; register:input
    Fonts(Vec<Vec<u8>>),           // the whole label fonts, main font first; register:loading
}
// --8<-- [end:001-messages]

// --8<-- [start:001-app]
#[cfg(target_arch = "wasm32")]
use {
    crate::app::input::Input, // register:input
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
    input: Input,                       // mouse and key gestures; register:input
    pointer_cancellation: Option<app::input::PointerCancellation>, // browser pointer-lost listener; register:input
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
            input: Input::new(),        // register:input
            pointer_cancellation: None, // register:input
        };
        // a browser loop cannot block: `spawn_app` hands the app over and returns at once
        event_loop.spawn_app(app);
        Ok(())
    }

    /// Take the ready state, size it to the canvas, draw.
    fn adopt(&mut self, mut state: State) {
// --8<-- [start:005-adopt]
        fit_canvas(&mut state); // register:resize
// --8<-- [end:005-adopt]
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
            self.listen_pointer(canvas.clone(), &proxy); // register:input
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
            Msg::Clear => state.clear(), // register:scene
            Msg::Fit => state.fit_loaded(), // register:scene
            Msg::File(doc, source) => state.append(doc, source), // register:scene
            Msg::Fonts(faces) => self.use_fonts(faces),   // register:loading
            Msg::Texts(texts) => state.set_texts(texts),  // register:scene_text
            Msg::StreamedCloud(init) => start_stream(state, init), // register:stream
            Msg::CloudChunk(c) => state.extend_streamed(c.idx, c.rows, c.to), // register:stream
            Msg::CloudQueryBatch(batch) => state.cloud_query_batch(batch), // register:cloud_query
            Msg::CloudQueryResolved(resolved) => state.cloud_query_resolved(resolved), // register:cloud_query
            Msg::Sheet(init) => start_sheet(state, init), // register:sheets
            Msg::SheetChunk(c) => state.extend_sheet(c.idx, c.rows, c.to), // register:sheets
            Msg::SheetEntity(resolved) => state.sheet_entity(resolved), // register:sheets
            Msg::CancelPointer => self.pointer_lost(), // register:input
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
            WindowEvent::KeyboardInput { event, .. } => self.key(&event), // register:keys
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
        changed |= self.input.mouse(state, event); // register:input
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

    // after a GPU-loss reload, show the notice
    if let Some(notice) = app::route::adopt_recovery() { // register:recovery
        app::feedback::status(notice);
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

// --8<-- [start:005-redraw]
        if page_hidden() || desired_canvas_size().is_none() { // register:resize
            return false;
        }

        if resize_held(state) { // register:resize
            state.needs_frame = true;
            return false;
        }
// --8<-- [end:005-redraw]

        state.render();
        false
    }
}
// --8<-- [end:004-redraw]
// --8<-- [start:005-resize]
/// Match the canvas pixel size.
#[cfg(target_arch = "wasm32")]
fn fit_canvas(state: &mut State) {
    if let Some((w, h)) = desired_canvas_size() {
        let _ = state.resize(w, h);
    }
}

/// Resize first; true when a resize not ready yet holds the frame.
#[cfg(target_arch = "wasm32")]
fn resize_held(state: &mut State) -> bool {
    match desired_canvas_size() {
        Some((w, h)) if (w, h) != (state.gpu.config.width, state.gpu.config.height) => {
            !state.resize(w, h)
        }
        _ => false,
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
// --8<-- [end:005-resize]
// --8<-- [start:04a-tail]
#[cfg(target_arch = "wasm32")]
impl App {
    /// A key press: first press only, and only while the canvas has focus.
    fn key(&mut self, event: &winit::event::KeyEvent) -> bool {
        let Some(state) = &mut self.state else {
            return false;
        };
        viewer_focused()
            && event.state == ElementState::Pressed
            && !event.repeat
            && self.input.key(state, event.logical_key.as_ref())
    }
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
            state.use_fonts(faces); // register:scene_text
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Listen for the browser taking the pointer away.
    fn listen_pointer(&mut self, canvas: web_sys::HtmlCanvasElement, proxy: &EventLoopProxy<Msg>) {
        match app::input::PointerCancellation::new(canvas, proxy.clone()) {
            Ok(listener) => self.pointer_cancellation = Some(listener),
            Err(error) => log::warn!("Cannot register pointer cancellation: {error:?}"),
        }
    }

    /// The browser lost the pointer: end every gesture.
    fn pointer_lost(&mut self) {
        let Some(state) = &mut self.state else { return };
        self.input.cancel();
        state.touch();
    }
}

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

/// Replace the scene with a saved one.
#[cfg(target_arch = "wasm32")]
fn open_saved(state: &mut State, scene: Box<app::scene::Scene>) {
    state.clear();
    state.scene = *scene;
    state.scene.upload_to(&mut state.gpu);
    state.scene.restore_text_visibility(&mut state.gpu); // register:scene_text
    state.update_label(); // the saved texts reach the GPU; register:scene_text
    state.fit_all();
    state.touch();
    app::feedback::status("Session opened");
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Listen to the hidden input that raises the phone keyboard.
    fn listen_agent(&mut self, canvas: web_sys::HtmlCanvasElement, proxy: &EventLoopProxy<Msg>) {
        match app::agent::CommandAgent::new(canvas, proxy.clone()) {
            Ok(agent) => self.agent = Some(agent),
            Err(error) => log::warn!("Cannot register the command agent: {error:?}"),
        }
    }

    /// Phone keys become key presses.
    fn agent_keys(&mut self, event: app::agent::AgentEvent) {
        let Some(state) = &mut self.state else { return };

        if let Some(ui) = self.ui.as_mut() {
            for key in ui.agent(event) {
                self.input
                    .key(state, winit::keyboard::Key::Character(key.as_str()));
            }
        }

        state.touch();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod selftest;
// --8<-- [end:04a-tail]
