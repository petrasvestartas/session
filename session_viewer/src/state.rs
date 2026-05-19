use std::sync::Arc; // Smart pointer

#[cfg(target_arch = "wasm32")]
use winit::event;
// Docs (v0.30): https://docs.rs/winit/0.30/winit/
use winit::{
    application::ApplicationHandler,   // winit::application::ApplicationHandler
    event::*,                           // everything in winit::event
    event_loop::{ActiveEventLoop, EventLoop}, // two items from winit::event_loop
    keyboard::{KeyCode, PhysicalKey},   // two from winit::keyboard
    window::Window                      // winit::window::Window
};


// Allows Rust to interact with JavaScript when compiled to WebAssembly.
// Imports a winit extension trait that adds browser-specific functionality to the event loop.
#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen::prelude::*, 
    winit::platform::web::EventLoopExtWebSys
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
        Ok(Self {
            window,
        })
    }

    /// Called when the window is resized.
    ///
    /// - Resize event: https://docs.rs/winit/0.30/winit/event/enum.WindowEvent.html#variant.Resized
    pub fn resize(&mut self, _width: u32, _height: u32) {

    }

    /// Request the window to redraw.
    ///
    /// - `Window::request_redraw`: https://docs.rs/winit/0.30/winit/window/struct.Window.html#method.request_redraw
    pub fn render(&mut self){
        self.window.request_redraw();
    }

}

// We need to tell winit how to use our `State` struct as the application state.
// The state variable stores State struct as an option.
pub struct App{
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")] 
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
    }
}

impl ApplicationHandler<State> for App{

    fn resumed(&mut self, event_loop: &ActiveEventLoop){

        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window  = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to await the window creation.
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        // Run the future asynchronously and use the proxy to send results to the event loop.
        {
            if let Some(proxy) = self.proxy.take(){
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(proxy.
                        send_event(
                            State::new(window)
                            .await
                            .expect("Unable to create canvas!")
                        )
                        .is_ok());
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State){
        // This is where proxy.send_event() ends up.
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height
            );
        }
        self.state = Some(event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId,  event: WindowEvent, ) { 
        // TODO: Next we'll talk about window_event
        // https://sotrh.github.io/learn-wgpu/beginner/tutorial1-window/#the-code
    }

} 