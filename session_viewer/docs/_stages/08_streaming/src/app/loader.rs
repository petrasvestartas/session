//! The async loader (wasm): bring the canvas up EMPTY, then post every document to the
//! event loop as a `Msg` - whole files through `decode`, big clouds a slice at a time
//! through `stream`. Touches no GPU.

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use session_rust::Xform;
use crate::engine::performance::now_ms;
use crate::{CloudChunk, Msg, State};
use super::decode::session_from_bytes;
use super::fetch::fetch_bytes;
use super::manifest::Manifest;
use super::route::{join, knob_u32, scene_route, SceneRoute};
use super::scene::{FileDoc, Scene, StreamedInit};
use super::stream::{cloud_fields, cloud_lod, fetch_colors, fetch_positions, CloudFields};
use super::route::AUTO_GRID;
use super::walk::cloud::StreamRows;

/// Points a streamed cloud brings down before it is on screen: the octree's coarse levels,
/// so the file opens at a correct low detail whatever its size.
const STREAM_PREFIX_POINTS: u32 = 2_000_000;

/// Points per follow-up slice.
const STREAM_CHUNK_POINTS: u32 = 2_000_000;

/// Hard ceiling on resident streamed points across the whole page (`?points=` to change):
/// 16 B a point on the GPU plus the growth slack, and a 14 M cloud killed the GPU process.
const STREAM_MAX_POINTS: u32 = 6_000_000;

/// Files at least this large open by range even without a count-based reason.
const STREAM_MIN_BYTES: u64 = 64 * 1024 * 1024;

/// The smallest prefix a streamed cloud gets even past the ceiling: the coarsest octree levels,
/// so the cloud is on screen and correct, just sparse.
const STREAM_MIN_PREFIX: u32 = 250_000;

thread_local! {
    /// The start-up proxy, kept so the stream tasks can post messages.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };
    /// Points resident across every streamed cloud on the page: the ceiling is a scene budget.
    static RESIDENT: Cell<u32> = const { Cell::new(0) };
    /// Bumped on every `Clear`: a stream task from an older scene stops at its next slice.
    static GENERATION: Cell<u32> = const { Cell::new(0) };
}

/// Post one message into the running event loop; false when the loop is gone.
fn post(msg: Msg) -> bool {
    PROXY.with(|p| p.borrow().as_ref().map(|proxy| proxy.send_event(msg).is_ok())).unwrap_or(false)
}

/// The resident ceiling for this page load.
fn max_points() -> u32 {
    knob_u32("points").unwrap_or(STREAM_MAX_POINTS)
}

/// Points the scene may still make resident.
fn budget_left() -> u32 {
    RESIDENT.with(|r| max_points().saturating_sub(r.get()))
}

/// Book `n` points against the budget.
fn budget_spend(n: u32) {
    RESIDENT.with(|r| r.set(r.get().saturating_add(n)));
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

    let files = manifest.items.iter().filter(|i| i.file.ends_with(".pb")).count().max(1) as u32;
    let share = (max_points() / files).max(STREAM_MIN_PREFIX);
    for (i, item) in manifest.items.iter().enumerate() {
        let url = join(&route.base, &item.file);
        let place = manifest.place(i, AUTO_GRID);
        let point_px = item.point_size as f32;
        if url.ends_with(".pb") {
            let slot = Placement { name: manifest.name_of(i, &item.file), place: place.clone(), point_px };
            if let Some(init) = stream_prefix(&url, &slot, share).await {
                post(Msg::StreamedCloud(Box::new(init)));
                continue;
            }
        }
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

/// Where a streamed cloud goes: its document name, placement and point size.
struct Placement {
    name: String,
    place: Xform,
    point_px: f32,
}

/// Try to open a cloud by RANGE: `None` means the file carries no octree or is small enough
/// to decode whole. Three small reads locate everything, one read brings the prefix - at most
/// `share` points (the budget split over the scene's files, so the first cloud cannot take it
/// all), never fewer than `STREAM_MIN_PREFIX`.
async fn stream_prefix(url: &str, slot: &Placement, share: u32) -> Option<StreamedInit> {
    let (name, place, point_px) = (slot.name.as_str(), slot.place.clone(), slot.point_px);
    let fields = cloud_fields(url).await?;
    if fields.count <= STREAM_PREFIX_POINTS && fields.coords_len < STREAM_MIN_BYTES {
        return None;
    }
    let lod = cloud_lod(url, &fields).await?;
    let resident = STREAM_PREFIX_POINTS.min(share).min(fields.count).min(budget_left().max(STREAM_MIN_PREFIX));
    let Some(positions) = fetch_positions(url, &fields, 0, resident).await else {
        log::warn!("'{name}': the prefix range read failed - the cloud stays off screen (a whole decode would take {:.0} MB)", fields.coords_len as f64 / 1.048576e6);
        return Some(StreamedInit { name: name.to_string(), url: url.to_string(), place, rows: StreamRows { positions: Vec::new(), colors: Vec::new() }, lod, fields, resident: 0, point_px, col_at: fields.colors_at });
    };
    let (colors, col_at) = fetch_colors(url, &fields, fields.colors_at, resident).await.unwrap_or((Vec::new(), fields.colors_at));
    budget_spend(resident);
    log::info!("streamed '{name}': {resident} of {} points on screen, {} nodes", fields.count, lod.len());
    Some(StreamedInit { name: name.to_string(), url: url.to_string(), place, rows: StreamRows { positions, colors }, lod, fields, resident, point_px, col_at })
}

/// Where a streamed cloud continues: its slot, its file layout, the next point and where the
/// colour run continues.
pub struct StreamCursor {
    pub idx: usize,
    pub url: String,
    pub fields: CloudFields,
    pub from: u32,
    pub col_at: u64,
}

/// Fetch the rest of a streamed cloud, a slice at a time, posting each one; spawned once the
/// scene has given the cloud its slot. Stops when the scene it belongs to is cleared.
pub fn spawn_stream_rest(cursor: StreamCursor) {
    wasm_bindgen_futures::spawn_local(stream_rest(cursor));
}

/// The slice loop behind `spawn_stream_rest`.
async fn stream_rest(c: StreamCursor) {
    let (url, idx, fields) = (c.url, c.idx, c.fields);
    let generation = GENERATION.with(|g| g.get());
    let mut col_at = c.col_at;
    let mut at = c.from;
    while at < fields.count {
        if GENERATION.with(|g| g.get()) != generation {
            return;
        }
        let left = budget_left();
        if left == 0 {
            log::info!("'{url}': {at} of {} points resident - at the page's point ceiling (?points= to raise it)", fields.count);
            return;
        }
        let to = (at + STREAM_CHUNK_POINTS.min(left)).min(fields.count);
        budget_spend(to - at);
        let Some(positions) = fetch_positions(&url, &fields, at, to).await else { return };
        let (colors, next) = fetch_colors(&url, &fields, col_at, to - at).await.unwrap_or((Vec::new(), col_at));
        col_at = next;
        if GENERATION.with(|g| g.get()) != generation {
            return;
        }
        if !post(Msg::CloudChunk(CloudChunk { idx, rows: StreamRows { positions, colors }, to })) {
            return;
        }
        at = to;
    }
}

