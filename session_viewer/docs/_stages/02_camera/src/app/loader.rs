//! The async loader (wasm): bring the canvas up EMPTY. Touches no GPU.

use std::sync::Arc;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use crate::{Msg, State};

/// Start-up: the empty canvas.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    let state = State::new(window).await.expect("State init failed");
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));
}
