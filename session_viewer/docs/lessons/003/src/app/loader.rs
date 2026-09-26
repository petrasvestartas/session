// --8<-- [start:001-boot]
use crate::{Msg, State};
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;

thread_local! {
    /// Sends messages into the event loop.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };
}

/// Start the viewer, load the first scene, then keep polling.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    PROXY.with_borrow_mut(|slot| *slot = Some(proxy.clone()));
    let state = match State::new(window).await {
        Ok(state) => state,
        Err(error) => {
            super::feedback::error(&format!(
                "Unable to start WebGPU: {error}. Use a browser with an available WebGPU adapter, then reload."
            ));
            return;
        }
    };
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));

}
// --8<-- [end:001-boot]
