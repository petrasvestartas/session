use super::scene::{FileDoc, Scene};
use super::stream::CloudFields;
use crate::{Msg, State};
use session_rust::{Session, Xform};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;

thread_local! {
    /// Sends messages into the event loop.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };
}

/// The query-result delivery interface.
pub(super) fn post(message: Msg) -> bool {
    PROXY.with_borrow(|proxy| post_with_proxy(proxy, message))
}

/// Hand the result to the event loop.
fn post_with_proxy(proxy: &Option<EventLoopProxy<Msg>>, message: Msg) -> bool {
    match proxy {
        Some(proxy) => proxy.send_event(message).is_ok(),
        None => false,
    }
}

/// Own the proxy for asynchronous source-query messages.
fn retain_proxy(slot: &mut Option<EventLoopProxy<Msg>>, proxy: &EventLoopProxy<Msg>) {
    *slot = Some(proxy.clone());
}

/// Start the viewer, load the first scene, then keep polling.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    PROXY.with_borrow_mut(|slot| retain_proxy(slot, &proxy));
    let state = match State::new(window, Scene::new()).await {
        Ok(state) => state,
        Err(error) => {
            super::feedback::error(&format!("Cannot initialize WebGPU: {error}"));
            return;
        }
    };
    post(Msg::Ready(Box::new(state)));

    if let Err(error) = fixture().await {
        super::feedback::error(&error);
        return;
    }

    post(Msg::Fit);
    super::feedback::status("");
}

/// Only the explicit local streaming test selects the ranged source fixture.
async fn fixture() -> Result<(), String> {
    let session = Session::pb_loads(include_bytes!("../../assets/pb/interaction.pb"))
        .map_err(fixture_error)?;
    post(Msg::File(FileDoc {
        name: "Interaction fixture".to_string(), // the scene's title
        session: Rc::new(session),
        place: Xform::identity(),
        point_px: 0.0,
        display_only: false,
    }));
    Ok(())
}

/// A decode failure is reported, not hidden.
fn fixture_error(error: Box<dyn std::error::Error>) -> String {
    format!("Bundled interaction fixture: {error}")
}

/// Where a cloud's streaming continues.
pub struct StreamCursor {
    pub idx: usize,          // the cloud's slot in the scene
    pub url: String,         // the cloud file
    pub fields: CloudFields, // array positions in the file
    pub from: u32,           // next point to read
    pub col_at: u64,         // byte position of its colour
}

/// Fixed display residency; F10 reads the source.
pub fn spawn_stream_rest(_cursor: StreamCursor) {}
