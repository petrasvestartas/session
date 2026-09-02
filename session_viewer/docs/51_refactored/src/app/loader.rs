//! The async manifest loader: fetch the manifest, bring the canvas up EMPTY, then post every
//! item to the event loop as a `Msg` - whole files through `decode` (prefetched one ahead),
//! `stream` items through `stream` as 8 MB slices. Touches no GPU. `reload_scene` re-enters
//! here from the page with the `State` kept.

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use session_rust::Xform;
use crate::{Msg, State};
use crate::engine::performance::now_ms;
use crate::math::Aabb;
use super::decode::session_from_bytes_chunked;
use super::fetch::{fetch_bytes, fetch_finish, fetch_range_finish, fetch_range_start, fetch_start, next_tick, Fetch};
use super::manifest::{Item, Manifest};
use super::scene::{CloudBegin, FileDoc, Scene};
use super::stream::{cloud_fields, positions_from, CloudFields, ColorRun, SLICE_BYTES};

/// The scene when the page names none: fetched at runtime, so re-arranging it is a text edit
/// in assets/scenes, not a rebuild.
const DEMO_SCENE_URL: &str = "scenes/bunny_drawings.toml";

/// The manifest to load: `?scene=<path under assets/>` when the page supplies one, else
/// [`DEMO_SCENE_URL`]. One build can therefore serve many scenes - the docs embed a single
/// 7.7 MB wasm in an iframe per example and vary only the query string.
///
/// The value is a path under `assets/`, exactly like a manifest's own `file` entries. It is
/// rejected unless it stays inside that tree: an absolute URL, a scheme, or any `..` segment
/// would let a page point the viewer at another origin.
fn scene_url() -> String {
    /// The `scene=` query value, when present and inside `assets/`.
    fn from_query() -> Option<String> {
        let search = web_sys::window()?.location().search().ok()?;
        let raw = search.strip_prefix('?')?;
        let value = raw
            .split('&')
            .find_map(|pair| pair.strip_prefix("scene="))?;
        let decoded = js_sys::decode_uri_component(value).ok()?.as_string()?;
        let safe = !decoded.is_empty()
            && !decoded.starts_with('/')
            && !decoded.contains("//")
            && !decoded.contains(':')
            && !decoded.split('/').any(|seg| seg == "..");
        safe.then_some(decoded)
    }
    from_query().unwrap_or_else(|| DEMO_SCENE_URL.to_string())
}

thread_local! {
    /// A proxy kept past start-up so [`reload_scene`] can post files into the
    /// running event loop. `resumed` takes `self.proxy`, so without this copy
    /// there is no way back into the app once it is going.
    static RELOAD_PROXY: std::cell::RefCell<Option<winit::event_loop::EventLoopProxy<Msg>>> =
        const { std::cell::RefCell::new(None) };
}

/// Keep the start-up proxy so [`reload_scene`] can post into the running event loop.
pub fn keep_proxy(proxy: &EventLoopProxy<Msg>) {
    RELOAD_PROXY.with(|slot| *slot.borrow_mut() = Some(proxy.clone()));
}

/// Reload the scene in place: same canvas, same camera, new geometry.
///
/// The page calls this after rewriting a `.pb` (see the docs' Thebe cells) so an
/// edit redraws the MODEL instead of restarting the viewer - reloading the
/// iframe would rebuild the WebGPU device and throw away the view you had
/// framed. `url` is a manifest path under `assets/`, as with `?scene=`.
#[wasm_bindgen]
pub fn reload_scene(url: Option<String>) {
    let proxy = RELOAD_PROXY.with(|slot| slot.borrow().clone());
    let Some(proxy) = proxy else {
        log::warn!("reload_scene: viewer is not running yet");
        return;
    };
    let url = url.unwrap_or_else(scene_url);
    wasm_bindgen_futures::spawn_local(async move {
        let _ = proxy.send_event(Msg::Clear);
        load_manifest(url, &proxy).await;
    });
}

/// Fetch a manifest and post every parsed file as a `Msg::File`, in manifest order - the
/// [`reload_scene`] path: no prefetch, no streaming, the `State` already exists.
async fn load_manifest(url: String, proxy: &EventLoopProxy<Msg>) {
    let manifest_bytes = fetch_bytes(&url).await.unwrap_or_default();
    let Some(manifest) = Manifest::parse(&manifest_bytes) else {
        log::error!("cannot read the scene manifest at {url}");
        return;
    };
    for (i, item) in manifest.items.iter().enumerate() {
        let bytes = fetch_bytes(&item.file).await.unwrap_or_default();
        let session = session_from_bytes_chunked(&item.file, bytes, item.display_only).await;
        if session.lookup.is_empty() {
            continue;
        }
        let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
        let place = manifest.place(i, [0.0, 0.0]);
        let _ = proxy.send_event(Msg::File(FileDoc { name, session, place, point_px: item.point_size as f32, display_only: item.display_only }));
    }
}

/// Start-up: manifest, `State` around an EMPTY scene (posted as `Msg::Ready`), then every item
/// in manifest order.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    let t0 = now_ms();
    let scene_url = scene_url();
    let manifest_bytes = fetch_bytes(&scene_url).await.unwrap_or_default();
    let manifest = Manifest::parse(&manifest_bytes).unwrap_or_else(|| panic!("cannot read the scene manifest at {scene_url}"));
    log::info!("scene '{}': {} items", manifest.name, manifest.items.len());

    // The canvas and the GPU come up FIRST, empty. A streamed cloud writes into GPU buffers, so
    // the GPU has to exist before the first byte of geometry is fetched - and as a bonus the
    // viewport is live immediately, not after a parse.
    let state = State::new(window, Scene::new()).await.expect("State init failed");
    log::info!("canvas live {:.0}ms after manifest fetch", now_ms() - t0);
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));

    // Pipelined: `fetch_start` is eager, so file n+1 is in flight while file n parses; and
    // progressive: every file streams in as its own `Msg` the moment it is ready.
    let mut next = manifest.items.first().and_then(prefetch);
    for (i, item) in manifest.items.iter().enumerate() {
        let cur = next.take();
        next = manifest.items.get(i + 1).and_then(prefetch);
        let place = manifest.place(i, [0.0, 0.0]);
        if item.stream {
            stream_item(&proxy, item, place).await;
        } else {
            whole_item(&proxy, item, cur, place).await;
        }
    }
}

/// The whole-file prefetch; `stream` items are skipped - a plain GET on a 431 MB scan would
/// pull the entire body.
fn prefetch(it: &Item) -> Option<Result<Fetch, JsValue>> {
    (!it.stream).then(|| fetch_start(&it.file))
}

/// A `stream` cloud never becomes a kernel object and never exists whole in wasm memory: two
/// small Range reads find the packed arrays, then the coords run and the colours run stream.
async fn stream_item(proxy: &EventLoopProxy<Msg>, item: &Item, place: Xform) {
    let f0 = now_ms();
    let named = if item.name.is_empty() { item.file.clone() } else { item.name.clone() };
    let Some(f) = cloud_fields(&item.file).await else {
        log::warn!("'{}': stream requested but no Range-addressable cloud found - skipped", named);
        return;
    };
    log::info!("streaming '{}': {} points | coords {:.0} MB + colours {:.0} MB",
        named, f.count, f.coords_len as f64 / 1048576.0, f.colors_len as f64 / 1048576.0);
    let _ = proxy.send_event(Msg::CloudBegin(CloudBegin { name: named.clone(), place, count: f.count, px: item.point_size as f32 }));
    let local = stream_coords(proxy, &item.file, &f).await;
    stream_colors(proxy, &item.file, &f).await;
    let _ = proxy.send_event(Msg::CloudEnd(local));
    log::info!("streamed '{}' in {:.0}ms", named, now_ms() - f0);
}

/// The coords run in 8 MB slices, each converted, posted and dropped; returns the cloud's own
/// box. PIPELINED, and this is the loader's whole performance story: `fetch_range(..).await`
/// resolves off network I/O and cannot resume until the current FRAME is done, so a sequential
/// loop pays a frame per slice - slice n+1 in flight while n converts hides both.
async fn stream_coords(proxy: &EventLoopProxy<Msg>, url: &str, f: &CloudFields) -> Aabb {
    // Rounded DOWN to a whole number of points: a slice boundary can then never fall inside a
    // point, let alone inside one of its doubles.
    const SLICE: u64 = (SLICE_BYTES / 24) * 24;
    let (mut at, mut left) = (f.coords_at, f.coords_len);
    let (mut lo, mut hi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    let mut inflight = if left > 0 {
        fetch_range_start(url, at, SLICE.min(left)).ok()
    } else {
        None
    };
    while let Some(f_in) = inflight.take() {
        let n = SLICE.min(left);
        at += n;
        left -= n;
        // next one on the wire BEFORE we spend time on this one
        inflight = if left > 0 {
            fetch_range_start(url, at, SLICE.min(left)).ok()
        } else {
            None
        };
        let Ok(raw) = fetch_range_finish(f_in).await else { break };
        let pos = positions_from(&raw);
        drop(raw);
        for q in pos.chunks_exact(3) {
            for k in 0..3 { lo[k] = lo[k].min(q[k]); hi[k] = hi[k].max(q[k]); }
        }
        let _ = proxy.send_event(Msg::CloudPos(pos));
        // A real macrotask between slices: with a warm cache the fetch promises resolve as
        // MICROtasks, which never let the browser paint - the freeze the sliced parse avoids.
        next_tick().await;
    }
    Aabb { min: lo, max: hi }
}

/// The colours run in the same 8 MiB slices, the same pipelining; one `Msg::CloudCol` per
/// slice, the split varint at each boundary carried by `ColorRun`. The GPU rows fill in behind
/// the positions as each slice lands, at the offset `StreamLane::push_col` keeps.
async fn stream_colors(proxy: &EventLoopProxy<Msg>, url: &str, f: &CloudFields) {
    let (mut at, mut left) = (f.colors_at, f.colors_len);
    let mut run = ColorRun::new(f.count);
    let mut inflight = if left > 0 {
        fetch_range_start(url, at, SLICE_BYTES.min(left)).ok()
    } else {
        None
    };
    while let Some(f_in) = inflight.take() {
        let n = SLICE_BYTES.min(left);
        at += n;
        left -= n;
        inflight = if left > 0 {
            fetch_range_start(url, at, SLICE_BYTES.min(left)).ok()
        } else {
            None
        };
        let Ok(raw) = fetch_range_finish(f_in).await else { break };
        let col = run.decode(&raw);
        drop(raw);
        if !col.is_empty() {
            let _ = proxy.send_event(Msg::CloudCol(col));
        }
        next_tick().await;
    }
}

/// One whole file: finish its prefetch, decode in chunks, post it as a `Msg::File`.
async fn whole_item(proxy: &EventLoopProxy<Msg>, item: &Item, fetched: Option<Result<Fetch, JsValue>>, place: Xform) {
    let f0 = now_ms();
    let bytes = match fetched {
        Some(Ok(f)) => fetch_finish(f).await.unwrap_or_default(),
        _ => Vec::new(),
    };
    let f1 = now_ms();
    let nbytes = bytes.len();
    let session = session_from_bytes_chunked(&item.file, bytes, item.display_only).await;
    let name = if item.name.is_empty() {
        session.name.clone()
    } else {
        item.name.clone()
    };
    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), nbytes, f1 - f0, now_ms() - f1);
    if session.lookup.is_empty() {
        return; // failed fetch - skipped file
    }
    let _ = proxy.send_event(Msg::File(FileDoc { name, session, place, point_px: item.point_size as f32, display_only: item.display_only }));
}
