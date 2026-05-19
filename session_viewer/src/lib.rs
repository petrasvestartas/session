use std::sync::Arc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    window::Window,
};

/// This will store the state of our application.
///
/// - `Arc`: https://doc.rust-lang.org/std/sync/struct.Arc.html
/// - `winit::window::Window`: https://docs.rs/winit/0.30/winit/window/struct.Window.html
pub struct State {
    window: Arc<Window>,
}

impl State {
    /// Create a new `State`.
    ///
    /// - `anyhow::Result`: https://docs.rs/anyhow/1.0/anyhow/type.Result.html
    /// - async fn: https://doc.rust-lang.org/std/keyword.async.html
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self { window })
    }

    /// Called when the window is resized.
    ///
    /// - Resize event: https://docs.rs/winit/0.30/winit/event/enum.WindowEvent.html#variant.Resized
    pub fn resize(&mut self, _width: u32, _height: u32) {}

    /// Request the window to redraw.
    ///
    /// - `Window::request_redraw`: https://docs.rs/winit/0.30/winit/window/struct.Window.html#method.request_redraw
    pub fn render(&mut self) {
        self.window.request_redraw();
    }
}

// We need to tell winit how to use our `State` struct as the application state.
// The state variable stores State struct as an option.
pub struct App {
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(event_loop: &EventLoop<State>) -> Self {
        Self {
            proxy: Some(event_loop.create_proxy()),
            state: None,
        }
    }

    // To run all the application we use this as the main entry point.
    pub fn run() -> anyhow::Result<()> {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
        let event_loop = EventLoop::with_user_event().build()?;
        let app = App::new(&event_loop);
        event_loop.spawn_app(app);
        Ok(())
    }
}

impl ApplicationHandler<State> for App {

    // It defines attributes about the window including web.
    // We use those attributes to create the window.
    // We create a future that creates our State struct.
    // On web we run the future asynchronously which sends the results to the user_event function.
    // The user_event function serves as a landing point for State future.
    // Resumed isn't async so we need to offload the future and send the results somewhere.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        const CANVAS_ID: &str = "canvas";

        let window = wgpu::web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
        let html_canvas_element = canvas.unchecked_into();

        let window_attributes = Window::default_attributes()
            .with_canvas(Some(html_canvas_element));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Run the future asynchronously and use the proxy to send results to the event loop.
        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(async move {
                assert!(proxy
                    .send_event(
                        State::new(window)
                            .await
                            .expect("Unable to create canvas!")
                    )
                    .is_ok());
            });
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // This is where proxy.send_event() ends up.
        event.window.request_redraw();
        event.resize(
            event.window.inner_size().width,
            event.window.inner_size().height,
        );
        self.state = Some(event);
    }

    // This is where we can process events such as keyboard inputs, and mouse movements.
    // As well as other events when the window want to draw or is resized.
    // We can call the methods we defined on `State` here.
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => state.render(),
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
                ..
            } => match (code, key_state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                _ => {}
            },
            _ => {}
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().unwrap_throw();
    Ok(())
}
