use std::sync::Arc;

// Docs (v0.30): https://docs.rs/winit/0.30/winit/
use winit::{
    application::ApplicationHandler,   // winit::application::ApplicationHandler
    event::*,                           // everything in winit::event
    event_loop::{ActiveEventLoop, EventLoop}, // two items from winit::event_loop
    keyboard::{KeyCode, PhysicalKey},   // two from winit::keyboard
    window::Window                      // winit::window::Window
};


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;


/// Application context holding the window handle.
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
