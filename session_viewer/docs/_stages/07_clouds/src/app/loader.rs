//! The async loader (wasm): bring the canvas up EMPTY, then post every document to the
//! event loop as a `Msg` - whole files through `decode`. Touches no GPU.

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use crate::engine::performance::now_ms;
use crate::{Msg, State};
use super::decode::session_from_bytes;
use super::fetch::fetch_bytes;
use super::manifest::Manifest;
use super::route::{join, scene_route, SceneRoute};
use super::scene::{FileDoc, Scene};
use super::route::AUTO_GRID;

thread_local! {
    /// The start-up proxy, kept so the loader can post messages.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };
}

/// Post one message into the running event loop; false when the loop is gone.
fn post(msg: Msg) -> bool {
    PROXY.with(|p| p.borrow().as_ref().map(|proxy| proxy.send_event(msg).is_ok())).unwrap_or(false)
}

/// Start-up: the empty canvas, then the URL's route.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    PROXY.with(|p| *p.borrow_mut() = Some(proxy.clone()));
    let state = State::new(window, Scene::new()).await.expect("State init failed");
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));
    load_route(&scene_route()).await;
}

/// Fetch a manifest and post every item, in manifest order, then a `Fit`.
async fn load_route(route: &SceneRoute) {
    let t0 = now_ms();
    let bytes = match fetch_bytes(&route.manifest).await {
        Ok(b) => b,
        Err(e) => {
            log::error!("cannot fetch the scene manifest: {e}");
            return;
        }
    };
    let manifest = match Manifest::parse(&bytes) {
        Ok(m) => m,
        Err(e) => {
            log::error!("cannot read the scene manifest at {}: {e}", route.manifest);
            return;
        }
    };
    log::info!("scene '{}': {} items", manifest.name, manifest.items.len());

    for (i, item) in manifest.items.iter().enumerate() {
        let url = join(&route.base, &item.file);
        let place = manifest.place(i, AUTO_GRID);
        let point_px = item.point_size as f32;
        let f0 = now_ms();
        let bytes = match fetch_bytes(&url).await {
            Ok(b) => b,
            Err(e) => {
                log::warn!("'{}' could not be fetched ({e}); skipped", item.file);
                continue;
            }
        };
        let n = bytes.len();
        let f1 = now_ms();
        let session = session_from_bytes(&url, bytes).await;
        if session.lookup.is_empty() {
            log::warn!("'{}' holds no geometry ({n} bytes); skipped", item.file);
            continue;
        }
        let name = manifest.name_of(i, &session.name);
        log::info!("loaded '{name}': {} objects, {n} bytes | fetch {:.0} ms, parse {:.0} ms", session.lookup.len(), f1 - f0, now_ms() - f1);
        post(Msg::File(FileDoc { name, session: Rc::new(session), place, point_px, display_only: item.display_only }));
    }
    post(Msg::Fit);
    log::info!("scene posted {:.0} ms after the manifest fetch", now_ms() - t0);
}
