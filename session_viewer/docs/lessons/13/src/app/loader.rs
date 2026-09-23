// --8<-- [start:step-12a]
//! Loads a scene file and turns it into GPU rows a chunk at a time, so the page never freezes.
use super::scene::{FileDoc, Scene, StreamedInit};
use super::stream::CloudFields;
use super::walk::cloud::StreamRows;
// --8<-- [end:step-12a]
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
    // --8<-- [start:step-12b]
    if super::route::query("scene").as_deref() == Some("stream-test.yaml") {
        return streamed_fixture().await;
    }

// --8<-- [end:step-12b]
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

// --8<-- [start:step-12c]
/// Read a display prefix; keep every source row for F10.
async fn streamed_fixture() -> Result<(), String> {
    let base = super::route::query("data").ok_or("local source URL is missing")?;

    if !base.starts_with("http://127.0.0.1:") && !base.starts_with("http://localhost:") {
        return Err("The tutorial source fixture must use a local HTTP server".to_string());
    }

    let url = format!("{}/cloud.pb", base.trim_end_matches('/'));
    let mut fields = super::stream::cloud_fields(&url)
        .await
        .ok_or("invalid cloud envelope")?;
    let lod = super::stream::cloud_lod(&url, &mut fields)
        .await
        .ok_or("invalid source node table")?;
    let resident = fields.count.min(250_000);
    let positions = super::stream::fetch_positions(&url, &fields, 0, resident)
        .await
        .ok_or("invalid source positions")?;
    let (colors, col_at) = if fields.colors_len == 0 {
        (Vec::new(), fields.colors_at)
    } else {
        super::stream::fetch_colors(&url, &fields, fields.colors_at, resident)
            .await
            .ok_or("invalid source colors")?
    };
    post(Msg::StreamedCloud(Box::new(StreamedInit {
        name: "Source query fixture".to_string(), // the scene's title
        url,
        place: Xform::identity(),
        rows: StreamRows { positions, colors },
        lod,
        fields,
        resident,
        point_px: 3.0, // point size in CSS pixels
        col_at,
    })));
    Ok(())
}

/// Where a cloud's streaming continues.
// --8<-- [end:step-12c]
pub struct StreamCursor {
    pub idx: usize, // the cloud's slot in the scene
    pub url: String, // the cloud file
    pub fields: CloudFields, // array positions in the file
    pub from: u32, // next point to read
    pub col_at: u64, // byte position of its colour
}

/// Fixed display residency; F10 reads the source.
pub fn spawn_stream_rest(_cursor: StreamCursor) {}
