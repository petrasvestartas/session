//! session_viewer — a browser-only (WebGPU/wgpu + winit) 3D viewer, grown one documented chapter
//! at a time. The module layout mirrors `session_viewer_archive` (engine → app → ui layers); see
//! that crate's `ARCHITECTURE.md` for the full map. Target: the browser canvas (wasm32) only —
//! the default build target is pinned in `.cargo/config.toml`, so there are no `cfg` gates.
//!
//! Chapter 1: a window that clears the screen.
//!   lib.rs        — the winit/browser shell: create the canvas window, run the event loop
//!   state.rs      — `State`, where the layered stack is wired
//!   engine/gpu.rs — `Gpu`, the wgpu device/queue/surface (lowest layer)

mod engine;
mod state;
mod camera;
pub mod math;
pub mod app; // App layer for file loading
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest; // headless render harness - see src/selftest.rs

pub use state::State;
use crate::app::persistence;
#[cfg(target_arch = "wasm32")]
use crate::app::scene::{auto_grid, Manifest};
#[cfg(target_arch = "wasm32")]
use crate::app::live::LiveSource;
#[cfg(target_arch = "wasm32")]
use crate::{camera::View, app::scene::Scene};

/// WHERE A SCENE COMES FROM. Three routes, and the URL alone decides which one runs:
///
///   * **no query, on localhost** - `view_local.toml` and `pb/view_local.pb`, both served by
///     `trunk serve` out of `assets/`. NOTHING is fetched over the network, so a dev server
///     works offline and shows the geometry being edited rather than what is published. It is
///     the ONLY local scene; there is deliberately no second one to drift out of date.
///   * **no query, anywhere else** - the LIVE source (`app/live.rs`): `view_live.toml` and the
///     files it names, read from the `session-viewer-data` bucket on R2 and re-read every poll.
///     This is what the deployed page shows.
///   * **`?scene=<path>`** - that manifest AND its files from R2, never from this origin. Every
///     scene but the local one lives in the bucket, so a named scene is the same bytes opened
///     from a laptop or from the deployed page.
///
/// `?data=<https base>` overrides where the `.pb` come from and `?data=off` forces this origin;
/// `?live=` overrides the live source.
///
/// This returns the second: `Some(path)` when the page asked for one, `None` otherwise. A page
/// that asks for neither (`?live=off` with no `?scene=`) comes up as an empty grid — there is no
/// built-in scene to fall back to, on purpose, because a hard-coded third answer is how the two
/// real ones drifted out of sync in the first place.
///
/// The value is a path under `assets/`, exactly like a manifest's own `file` entries. It is
/// rejected unless it stays inside that tree: an absolute URL, a scheme, or any `..` segment
/// would let a page point the viewer at another origin.
/// The one scene a local `trunk serve` shows, and the only manifest ever read from this origin.
/// Its geometry is `pb/view_local.pb`, uploaded nowhere - it is the local working copy.
#[cfg(target_arch = "wasm32")]
const LOCAL_SCENE: &str = "view_local.toml";

/// A scene named by the PATH instead of the query: `/view_lines` means `?scene=view_lines.toml`.
///
/// The whole app is one page, so any path serves the same `index.html` - trunk's dev server does
/// it already, and GitHub Pages needs `404.html` to be a copy of it (the deploy workflow makes
/// one). That leaves the path free to name a scene, which is the short URL worth typing.
///
/// Only the LAST segment is read, so it works at the site root and under `/session/` alike. The
/// same rules as `?scene=` apply: nothing that could point the viewer at another origin.
pub(crate) fn path_scene() -> Option<String> {
    let path = web_sys::window()?.location().pathname().ok()?;
    let last = path.rsplit('/').next()?.to_string();
    let safe = !last.is_empty()
        && !last.ends_with(".html")
        && !last.contains(':')
        && last != ".."
        && !last.starts_with('.');
    safe.then_some(last)
}

fn scene_url() -> Option<String> {
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

/// A NAMED scene: its URL in the bucket, and the base its `file` entries hang off.
///
/// A named scene lives in the bucket ALWAYS - never on this origin - so it is the same bytes
/// from a laptop and from the deployed page. The bucket is FLAT: one object per name, no
/// prefixes, so `?scene=view_meshes.toml` is the whole path and a manifest names its geometry
/// the same way.
#[cfg(target_arch = "wasm32")]
fn named_scene(path: &str) -> (String, String) {
    // A BARE NAME MEANS `scenes/`. Every named scene lives in that one folder, so
    // `?scene=view_lines.toml` and `?scene=scenes/view_lines.toml` are the same scene. Without
    // this the short form asks the bucket ROOT, gets a 404, and the page comes up empty for a
    // URL with nothing visibly wrong with it.
    // `.toml` is implied, so `/view_lines` and `?scene=view_lines` both work.
    let path = if path.contains('.') { path.to_string() } else { format!("{path}.toml") };
    let path = if path.contains('/') { path } else { format!("scenes/{path}") };
    (persistence::asset_url(&path), persistence::data_base())
}

/// The manifest to load and the base its `file` entries hang off, or `None` when this page has
/// neither route (deployed, no `?scene=` - the live source answers instead).
///
/// A manifest and its files ALWAYS share a base: mixing them is how a local manifest ends up
/// naming bucket geometry it cannot have meant.
#[cfg(target_arch = "wasm32")]
fn scene_route() -> Option<(String, String)> {
    // `?scene=` first - an explicit query beats the address bar's path.
    if let Some(path) = scene_url().or_else(path_scene) {
        return Some(named_scene(&path));
    }
    crate::app::live::page_is_local().then(|| (LOCAL_SCENE.to_string(), String::new()))
}

/// Async init - event-loop messages.
/// `Ready` carries the State built around the first file
/// pixes in 2s, each file is one more parsed document appended live.
pub enum Msg {
    Ready(Box<State>),
    File(String, session_rust::Session, session_rust::Xform, f32, bool),
    /// Drop the current documents, keeping `State` - see [`reload_scene`].
    Clear,
    /// Frame the camera on what is loaded now - sent after a live swap, whose files replace a
    /// scene the camera was fitted to.
    Fit,
    /// The next chunk of a streamed cloud: its index in `Scene::streamed`, the points, their
    /// colours, and the point the cloud is resident up to once this is appended.
    CloudChunk(usize, Vec<f32>, Vec<u32>, u32),
    /// A streamed cloud that is NOT the first file on screen. It cannot be added by the loader
    /// the way the first one is, because its index in `Scene::streamed` - which every later
    /// chunk is addressed by - exists only once the live scene has taken it.
    StreamedCloud(Box<StreamedInit>),
}

/// Everything `Scene::add_streamed_cloud` needs for one cloud, plus what `stream_rest` needs to
/// carry on fetching it. Boxed into `Msg::StreamedCloud` so a scene can hold any number of
/// streamed clouds instead of the first one only.
pub struct StreamedInit {
    pub name: String,
    pub url: String,
    pub place: session_rust::Xform,
    pub positions: Vec<f32>,
    pub colors: Vec<u32>,
    pub lod: persistence::CloudLod,
    pub resident: u32,
    pub total: u32,
    pub point_px: f32,
    pub col_at: u64,
}

thread_local! {
    /// A proxy kept past start-up so [`reload_scene`] can post files into the
    /// running event loop. `resumed` takes `self.proxy`, so without this copy
    /// there is no way back into the app once it is going.
    static RELOAD_PROXY: std::cell::RefCell<Option<winit::event_loop::EventLoopProxy<Msg>>> =
        const { std::cell::RefCell::new(None) };
}

/// Reload the scene in place: same canvas, same camera, new geometry.
///
/// The page calls this after rewriting a `.pb` (see the docs' Thebe cells) so an
/// edit redraws the MODEL instead of restarting the viewer - reloading the
/// iframe would rebuild the WebGPU device and throw away the view you had
/// framed. A named `url` is read from the bucket, exactly as `?scene=` is;
/// with no argument the page reloads whatever route it came up on.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn reload_scene(url: Option<String>) {
    let proxy = RELOAD_PROXY.with(|slot| slot.borrow().clone());
    let Some(proxy) = proxy else {
        log::warn!("reload_scene: viewer is not running yet");
        return;
    };
    // A manifest and its files share a base, so the route decides both together.
    let route = match url {
        Some(path) => Some(named_scene(&path)),
        None => scene_route(),
    };
    let Some((url, base)) = route else {
        log::warn!("reload_scene: no manifest given and this page has no scene route - nothing to reload");
        return;
    };
    wasm_bindgen_futures::spawn_local(async move {
        let _ = proxy.send_event(Msg::Clear);
        load_manifest(url, base, move |name, session, place, px, only| {
            let _ = proxy.send_event(Msg::File(name, session, place, px, only));
        })
        .await;
    });
}

/// Fetch a manifest and hand every parsed file to `emit`, in manifest order.
///
/// Shared by start-up and [`reload_scene`]; the only difference between them is
/// that start-up builds `State` around the first file, so it cannot use this
/// directly for that one.
#[cfg(target_arch = "wasm32")]
async fn load_manifest<F>(url: String, base: String, mut emit: F)
where
    F: FnMut(String, session_rust::Session, session_rust::Xform, f32, bool),
{
    let manifest_bytes = persistence::fetch_bytes(&url).await.unwrap_or_default();
    let Some(manifest) = Manifest::parse(&manifest_bytes) else {
        log::error!("cannot read the scene manifest at {url}");
        return;
    };
    let count = manifest.items.len();
    for (i, item) in manifest.items.iter().enumerate() {
        let file = persistence::join(&base, &item.file);
        let bytes = persistence::fetch_bytes(&file).await.unwrap_or_default();
        let session = persistence::session_from_bytes_chunked(&file, &bytes).await;
        if session.lookup.is_empty() {
            continue;
        }
        let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
        let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
        emit(name, session, place, item.point_size as f32, item.display_only);
    }
}

#[cfg(target_arch = "wasm32")]
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseScrollDelta, WindowEvent, MouseButton},
    keyboard::{Key, NamedKey},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
#[cfg(target_arch = "wasm32")]
use crate::app::touch::{Act, Touches};

// ── Browser event loop ──────────────────────────────────────────────────────
// State::new is async; winit's `resumed` is not, so we create the window, kick off async init,
// and deliver the finished State back as a user event (winit's documented wasm pattern).
/// The winit application handler: owns the viewer `State` once async init completes,
/// and tracks the mouse-orbit / modifier state between events — and, on a touchscreen,
/// the fingers (`app/touch.rs`), which winit delivers on their own event and never as a mouse.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<winit::event_loop::EventLoopProxy<Msg>>,
    orbiting: bool,
    panning: bool,
    last_cursor: (f64, f64),
    ctrl: bool,
    touch: Touches,
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
        use winit::platform::web::EventLoopExtWebSys;
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
        let app = App {
            proxy: Some(event_loop.create_proxy()),
            state: None,
            orbiting: false,
            panning: false,
            last_cursor: (0.0, 0.0),
            ctrl: false,
            touch: Touches::new(),
         };
        event_loop.spawn_app(app);
        Ok(())
    }
}

/// How many points a streamed cloud brings down before the file is on screen.
///
/// A prefix, not the whole cloud: the points are stored in octree order, so the first N are the
/// coarse levels and the file opens at a correct low detail whatever its size. 2 M points is
/// 48 MB of coordinates and about 16 MB of colours - bounded, where decoding a 431 MB scan whole
/// peaks near a gigabyte and the tab dies.
///
/// The REST follows: `stream_rest` keeps fetching a chunk at a time until the cloud is whole.
/// This number only decides how soon something is on screen, never how much ends up there.
#[cfg(target_arch = "wasm32")]
const STREAM_PREFIX_POINTS: u32 = 2_000_000;

/// Points per follow-up chunk. Same size as the prefix: each chunk is two range requests and one
/// GPU append, and the frames between them stay interactive.
#[cfg(target_arch = "wasm32")]
const STREAM_CHUNK_POINTS: u32 = 2_000_000;

/// Hard ceiling on how many points a streamed cloud may make resident, `?points=` to change it.
///
/// The GPU lanes cost 20 bytes a point (12 position, 4 colour, 4 normal) and `append_rows` grows
/// them by DOUBLING with a GPU-side copy, so the peak during a chunk is about three times the
/// steady size. Streaming a 13.8 M cloud to completion measured ~280 MB steady and killed the
/// GPU process on top of the render targets. 6 M points is ~120 MB steady, ~360 MB peak.
///
/// A cloud past the ceiling stays a coarse-level prefix, which is a correct cloud, not a
/// truncated one - the octree stores points coarsest-first. Raising the ceiling raises the
/// crash risk with it; the way to see a big cloud in full detail is `?lod=4`, which draws the
/// nodes the camera can actually resolve instead of all of them.
#[cfg(target_arch = "wasm32")]
const STREAM_MAX_POINTS: u32 = 6_000_000;

thread_local! {
    /// Points resident across EVERY streamed cloud on the page. The ceiling below is a scene
    /// budget, not a per-cloud one: three scans streaming to 6M points each is 18M points and
    /// the GPU process dies, which is the crash the ceiling exists to prevent. Clouds share it
    /// in load order, so the first ones are dense and the last ones stay coarse - and a coarse
    /// prefix is a correct cloud, not a truncated one.
    static STREAM_RESIDENT: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// Points the scene may still make resident.
#[cfg(target_arch = "wasm32")]
fn stream_budget_left() -> u32 {
    STREAM_RESIDENT.with(|r| stream_max_points().saturating_sub(r.get()))
}

/// Book `n` points against the budget.
#[cfg(target_arch = "wasm32")]
fn stream_budget_spend(n: u32) {
    STREAM_RESIDENT.with(|r| r.set(r.get().saturating_add(n)));
}

/// `?points=` - the resident ceiling for this page load.
#[cfg(target_arch = "wasm32")]
fn stream_max_points() -> u32 {
    web_sys::window()
        .and_then(|w| w.location().search().ok())
        .and_then(|q| q.split(['?', '&']).find_map(|p| p.strip_prefix("points=").map(str::to_string)))
        .and_then(|v| v.parse().ok())
        .unwrap_or(STREAM_MAX_POINTS)
}

/// Try to open a cloud by RANGE instead of decoding it whole. `None` means the file carries no
/// octree, or is small enough not to bother, and the caller should take the normal path.
///
/// Three small reads locate everything (`persistence::cloud_lod`), then one read brings the
/// prefix. Nothing here decodes a protobuf message: `coords` is a packed double array, so the
/// bytes ARE the numbers.
#[cfg(target_arch = "wasm32")]
async fn stream_cloud(url: &str) -> Option<(Vec<f32>, Vec<u32>, persistence::CloudLod, u32, u32, u64)> {
    let (fields, lod) = persistence::cloud_lod(url).await?;
    let resident = STREAM_PREFIX_POINTS.min(fields.count);
    if fields.count <= resident && fields.coords_len < 64 * 1024 * 1024 {
        return None; // small enough that the whole-file path costs nothing
    }
    let raw = persistence::fetch_range(url, fields.coords_at, resident as u64 * 24).await.ok()?;
    let positions = persistence::positions_from(&raw);
    // Colours are packed VARINTS, so unlike coords they cannot be sliced - they decode from the
    // start, which is exactly what a prefix needs. 0-255 each, so 2 bytes a channel is generous.
    let want = ((resident as u64) * 4 * 2).min(fields.colors_len);
    let (colors, col_at) = persistence::cloud_colors_from(url, fields.colors_at, want, resident)
        .await
        .unwrap_or((Vec::new(), fields.colors_at));
    stream_budget_spend(resident);
    log::info!("streamed '{url}': {resident} of {} points on screen ({:.0} MB of {:.0} MB), {} nodes",
        fields.count, raw.len() as f64 / 1.048576e6, fields.coords_len as f64 / 1.048576e6, lod.len());
    Some((positions, colors, lod, resident, fields.count, col_at))
}

/// Fetch the rest of a streamed cloud, a chunk at a time, and post each one into the running
/// event loop. Spawned after `Msg::Ready`, so the file is already on screen and every chunk
/// only makes it denser.
///
/// One `await` per chunk hands the browser back its main thread between fetches, which is what
/// keeps the viewer interactive while a 316 MB scan is still coming down.
#[cfg(target_arch = "wasm32")]
async fn stream_rest(url: String, idx: usize, from: u32, total: u32, mut col_at: u64) {
    let Some((fields, _)) = persistence::cloud_lod(&url).await else { return };
    let col_end = fields.colors_at + fields.colors_len;
    let mut at = from;
    while at < total {
        // Re-read the budget every chunk: the other clouds of this scene are spending it too.
        let left = stream_budget_left();
        if left == 0 {
            log::info!("'{url}': {at} of {total} points resident - the scene is at its point \
                        ceiling (?points= to change); the rest of the octree stays on the server");
            return;
        }
        let to = (at + STREAM_CHUNK_POINTS.min(left)).min(total);
        let n = (to - at) as u64;
        let Ok(raw) = persistence::fetch_range(&url, fields.coords_at + at as u64 * 24, n * 24).await else { return };
        let positions = persistence::positions_from(&raw);
        // Colours resume at the boundary the previous chunk stopped on, so each chunk reads only
        // its own bytes. 8 bytes a point is generous for four 0-255 varints.
        let want = (n * 4 * 2).min(col_end.saturating_sub(col_at));
        let (colors, next) = persistence::cloud_colors_from(&url, col_at, want, n as u32)
            .await
            .unwrap_or((Vec::new(), col_at));
        col_at = next;
        let sent = RELOAD_PROXY.with(|p| {
            p.borrow().as_ref().map(|proxy| proxy.send_event(Msg::CloudChunk(idx, positions, colors, to)).is_ok())
        });
        if sent != Some(true) { return } // the loop is gone - stop pulling bytes into a dead tab
        stream_budget_spend(to - at);
        at = to;
    }
}

/// Start-up path for the pinned local scene: the `?scene=` manifest under `assets/`. Reached only
/// when the live source declined the page, which `?scene=` itself is one of the two ways to do.
/// Builds `State` around the first file that loads and streams the rest as `Msg::File`. Returns
/// whether a `State` was sent.
#[cfg(target_arch = "wasm32")]
async fn local_scene(proxy: &winit::event_loop::EventLoopProxy<Msg>, window: Arc<Window>, scene_url: &str, base: &str) -> bool {
    // fetch_start is eager: the browser request for file n+1 is in flight while file n parses,
    // and progressive - ready after the first file, every later one streams in as a Msg::File
    let t0 = crate::engine::performance::now_ms();
    // The manifest comes from the SAME place as the files it names. Fetching it page-relative
    // while its entries resolve to the bucket is how `?scene=scenes/lidar14.toml` 404s against
    // an origin that was never meant to hold data.
    // AS GIVEN: `scene_route` already resolved this - the local scene against this origin, a
    // named scene against the bucket. Re-resolving here sent `view_local.toml` to R2 (404).
    // The fetch error is REPORTED, not swallowed. Defaulting to empty bytes here turns "that
    // URL 404s" into "missing field `items`", which sends you looking at the manifest instead of
    // at the address it was asked for.
    let manifest_bytes = match persistence::fetch_bytes(scene_url).await {
        Ok(b) => b,
        Err(e) => {
            log::error!("cannot fetch the scene manifest at {scene_url}: {}",
                        e.as_string().unwrap_or_else(|| format!("{e:?}")));
            return false;
        }
    };
    let manifest = match Manifest::parse_verbose(&manifest_bytes) {
        Ok(m) => m,
        Err(e) => { log::error!("cannot read the scene manifest at {scene_url}: {e}"); return false; }
    };
    log::info!("scene '{}': {} items", manifest.name, manifest.items.len());
    let count = manifest.items.len();
    let mut sent_ready = false;
    for (i, item) in manifest.items.iter().enumerate() {
        let f0 = crate::engine::performance::now_ms();
        let file = persistence::join(base, &item.file);
        let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));

        // PROBE BEFORE FETCHING. A cloud whose file carries an octree opens by RANGE, and the
        // probe is three reads totalling under a megabyte - so a 431 MB scan is never pulled
        // down to discover it did not need to be. Asking after the fetch would download the
        // file to learn it was avoidable.
        if let Some((pos, col, lod, resident, total, col_at)) = stream_cloud(&file).await {
            let name = if item.name.is_empty() { item.file.clone() } else { item.name.clone() };
            // A LATER streamed cloud goes into the scene that is already on screen. Its index in
            // `Scene::streamed` - what every chunk of it is addressed by - is only decided there,
            // so the running loop adds it and spawns its own `stream_rest`.
            if sent_ready {
                let _ = proxy.send_event(Msg::StreamedCloud(Box::new(StreamedInit {
                    name, url: file.clone(), place, positions: pos, colors: col, lod,
                    resident, total, point_px: item.point_size as f32, col_at,
                })));
                continue;
            }
            let mut scene = Scene::new();
            let idx = scene.streamed.len();
            scene.add_streamed_cloud(name, file.clone(), place, pos, col, &lod, resident, total, item.point_size as f32);
            let state = State::new(window.clone(), scene).await.expect("State init failed");
            let _ = proxy.send_event(Msg::Ready(Box::new(state)));
            sent_ready = true;
            // The rest follows in the background - the cloud on screen is a prefix until it does.
            let (url, t) = (file.clone(), total);
            wasm_bindgen_futures::spawn_local(async move { stream_rest(url, idx, resident, t, col_at).await });
            continue;
        }

        let bytes = persistence::fetch_bytes(&file).await.unwrap_or_default();
        let f1 = crate::engine::performance::now_ms();
        let session = persistence::session_from_bytes_chunked(&file, &bytes).await;
        let name = if item.name.is_empty() {
            session.name.clone()
        } else {
            item.name.clone()
        };
        log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), bytes.len(), f1 - f0, crate::engine::performance::now_ms() - f1);
        if session.lookup.is_empty() {
            continue; // failed fetch - skipped file
        }
        if !sent_ready {
            sent_ready = true;
            let mut scene = Scene::new();
            scene.add_file(name, session, place, item.point_size as f32, item.display_only);
            let state = State::new(window.clone(), scene).await.expect("State init failed");
            log::info!("first file on screen {:.0}ms after manifest fetch", crate::engine::performance::now_ms() - t0);
            let _ = proxy.send_event(Msg::Ready(Box::new(state)));
        } else {
            let _ = proxy.send_event(Msg::File(name, session, place, item.point_size as f32, item.display_only));
        }

    }
    // `Ready` framed the FIRST file only; frame everything the manifest listed. Without this a
    // scene whose files spread out - clouds at -8000, sheets at +14100 - comes up zoomed onto
    // whichever file happened to load first, with the rest off screen and apparently missing.
    if sent_ready { let _ = proxy.send_event(Msg::Fit); }
    sent_ready
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler<Msg> for App {

    /// Bind to the `#canvas` element and kick off async `State` init (delivered back via `user_event`).
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;

        if self.state.is_some() { return; }

        let canvas = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let attrs = Window::default_attributes().with_canvas(Some(canvas));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());

        if let Some(proxy) = self.proxy.take() {
            RELOAD_PROXY.with(|slot| *slot.borrow_mut() = Some(proxy.clone()));
            wasm_bindgen_futures::spawn_local(async move {

                // LIVE DATA FIRST. The page reads a manifest straight out of the R2 bucket -
                // no build, no deploy, no workflow between an upload and this page. When it is
                // readable and lists loadable files, that is the scene. When it is not (bucket
                // unreachable, nothing loads), the
                // built-in demo scene is shown and the poll loop below picks the live data up as
                // soon as it appears.
                let mut live = LiveSource::from_query();
                let mut sent_ready = false;
                if let Some(src) = live.as_mut() {
                    log::info!("live: watching {} every {} ms", src.label, src.poll_ms);
                    if let Some(manifest) = src.fetch_manifest().await {
                        for item in src.load_all(&manifest).await {
                            if !sent_ready {
                                sent_ready = true;
                                let mut scene = Scene::new();
                                scene.add_file(item.name, item.session, item.place, item.point_size, item.display_only);
                                let state = State::new(window.clone(), scene).await.expect("State init failed");
                                let _ = proxy.send_event(Msg::Ready(Box::new(state)));
                            } else {
                                let _ = proxy.send_event(Msg::File(item.name, item.session, item.place, item.point_size, item.display_only));
                            }
                        }
                        // `Ready` framed the first file only; frame everything the manifest listed.
                        if sent_ready { let _ = proxy.send_event(Msg::Fit); }
                    }
                }
                // The other routes: a named scene from the bucket, or - on a dev server with no
                // query at all - the one local scene. `scene_route` decides which, and hands back
                // the base its files hang off so both come from the same place.
                if !sent_ready && let Some((url, base)) = scene_route() {
                    sent_ready = local_scene(&proxy, window.clone(), &url, &base).await;
                }
                // Neither source produced geometry - a missing branch, a mid-push commit, a bad
                // `?scene=`, or a page that asked for neither. Come up as an empty grid anyway:
                // the window, the GPU device and the camera are built ONCE, and a scene arriving
                // later (the poll below, or `reload_scene`) is `Clear` + `File` + `Fit` on top of
                // them. A viewer that renders nothing until it has geometry is a viewer that
                // renders nothing when the geometry is late.
                if !sent_ready {
                    let state = State::new(window.clone(), Scene::new()).await.expect("State init failed");
                    let _ = proxy.send_event(Msg::Ready(Box::new(state)));
                }
                let Some(mut src) = live else { return };
                // Poll: a changed manifest re-fetches every listed file; the scene is swapped
                // only when at least one of them loaded, so a broken push never blanks the page.
                loop {
                    persistence::sleep_ms(src.poll_ms).await;
                    let Some(manifest) = src.fetch_manifest().await else { continue };
                    let items = src.load_all(&manifest).await;
                    if items.is_empty() { continue; }
                    let _ = proxy.send_event(Msg::Clear);
                    for item in items {
                        let _ = proxy.send_event(Msg::File(item.name, item.session, item.place, item.point_size, item.display_only));
                    }
                    let _ = proxy.send_event(Msg::Fit);
                }
            });
        }
    }

    /// `Ready`: adopt the State built around the first file, size it, fit the camera, draw.
    /// `File` append one more document - walk it into the shared tabkles, re-upload, redraw.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg){
        match msg {
            Msg::Ready(state) => {
                let mut state = *state;
                let (w, h) = desired_canvas_size()
                    .unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
                state.resize(w, h);
                let aspect = w as f64 / h as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                state.window.request_redraw();
                self.state = Some(state);
            }
            Msg::Clear => {
                if let Some(state) = &mut self.state {
                    state.scene.clear(&mut state.gpu);
                    state.window.request_redraw();
                }
            }
            Msg::Fit => {
                if let Some(state) = &mut self.state {
                    // The surface size, not `window.inner_size()` - that is 0x0 on the web.
                    let aspect = state.gpu.config.width.max(1) as f64 / state.gpu.config.height.max(1) as f64;
                    state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                    state.window.request_redraw();
                }
            }
            Msg::File(name, session, place, cloud_px, display_only) => {
                let Some(state) = &mut self.state else {
                    return;
                };
                let t0 = crate::engine::performance::now_ms();
                state.scene.add_file(name, session, place, cloud_px, display_only);
                let t1 = crate::engine::performance::now_ms();
                state.scene.upload_to(&mut state.gpu);
                state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);
                log::info!("appended: walk {:.0}ms · upload {:.0}ms | {} docs | heap {:.0} MB",
                    t1 - t0, crate::engine::performance::now_ms() - t1, state.scene.docs.len(),
                    crate::engine::performance::heap_mb());
                state.window.request_redraw();
            }
            Msg::StreamedCloud(init) => {
                let Some(state) = &mut self.state else { return };
                let init = *init;
                // The index the live scene gives it, NOT the one the loader guessed: every
                // `Msg::CloudChunk` for this cloud is addressed by it.
                let idx = state.scene.streamed.len();
                let (url, resident, total, col_at) = (init.url.clone(), init.resident, init.total, init.col_at);
                state.scene.add_streamed_cloud(init.name, init.url, init.place, init.positions,
                    init.colors, &init.lod, init.resident, init.total, init.point_px);
                state.scene.upload_to(&mut state.gpu);
                state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);
                log::info!("streamed cloud {idx} appended: {resident} of {total} points | heap {:.0} MB",
                    crate::engine::performance::heap_mb());
                state.window.request_redraw();
                wasm_bindgen_futures::spawn_local(async move { stream_rest(url, idx, resident, total, col_at).await });
            }
            Msg::CloudChunk(idx, positions, colors, to) => {
                let Some(state) = &mut self.state else { return };
                let n = positions.len() / 3;
                state.scene.extend_streamed_cloud(idx, positions, colors, to);
                state.scene.upload_to(&mut state.gpu);
                state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);
                let total = state.scene.streamed.get(idx).map_or(0, |s| s.total);
                log::info!("cloud chunk: +{n} points, {to} of {total} resident | heap {:.0} MB",
                    crate::engine::performance::heap_mb());
                state.window.request_redraw();
            }
        }
    }

    /// Handle one window event: redraw, resize, keyboard view shortcuts, and mouse orbit/pan/zoom.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = match &mut self.state { Some(s) => s, None => return };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // `?spin=1`: orbit a little every frame - a moving-camera benchmark that needs no
                // input device (frame line in the top-left corner, `?perf=1`).
                if spin_mode() { state.camera.orbit(0.004, 0.0); }
                // Before drawing, make the GPU surface match the canvas's real pixel size.
                // Cheap check every frame; reconfigure only on a genuine change.
                if let Some((w, h)) = desired_canvas_size() {
                    if (w, h) != (state.gpu.config.width, state.gpu.config.height) {
                        state.resize(w, h);
                    }
                }
                if let Err(e) = state.render() { log::error!("render: {e}"); }
            }
            WindowEvent::KeyboardInput { event, ..} => {
                if event.state == ElementState::Pressed && !event.repeat{
                    match event.logical_key.as_ref() {
                        Key::Named(NamedKey::Space) => {
                            let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                            state.camera.toggle_projection_framed(state.gpu.scene_min, state.gpu.scene_max, aspect);
                        }
                        Key::Character("1") => state.camera.set_view(View::Front),
                        Key::Character("2") => state.camera.set_view(View::Back),
                        Key::Character("3") => state.camera.set_view(View::Left),
                        Key::Character("4") => state.camera.set_view(View::Right),
                        Key::Character("5") => state.camera.set_view(View::Top),
                        Key::Character("6") => state.camera.set_view(View::Bottom),
                        Key::Character("7") => state.camera.set_view(View::Iso),
                        Key::Character("c" | "C") => state.camera.reset(),
                        // L toggles how the SOLID lane draws mesh/BRep edges: real 3D tubes, or
                        // camera-facing flat quads through the flat lane's own shader. Same
                        // instance table either way, so it is a free A/B at any zoom.
                        // Q / W / E hide a whole KIND of thing, so an overlap can be taken
                        // apart by eye: points, then lines and polylines, then mesh/BRep edges.
                        // E is the one to reach for on a model that draws its own outlines as
                        // polylines: the mesh topology gives every edge a second time, and two
                        // strokes a fraction of a pixel apart read as one thick ragged line.
                        Key::Character("q" | "Q") => {
                            state.gpu.show_points = !state.gpu.show_points;
                            log::info!("points: {}", state.gpu.show_points);
                            state.window.request_redraw();
                        }
                        Key::Character("w" | "W") => {
                            state.gpu.show_lines = !state.gpu.show_lines;
                            log::info!("lines: {}", state.gpu.show_lines);
                            state.window.request_redraw();
                        }
                        Key::Character("e" | "E") => {
                            state.gpu.show_mesh_edges = !state.gpu.show_mesh_edges;
                            log::info!("mesh edges: {}", state.gpu.show_mesh_edges);
                            state.window.request_redraw();
                        }
                        Key::Character("l" | "L") => {
                            use crate::engine::gpu::LineStyle;
                            state.gpu.line_style = match state.gpu.line_style {
                                LineStyle::Tubes => LineStyle::Flat,
                                LineStyle::Flat => LineStyle::Tubes,
                            };
                            log::info!("line style: {:?}", state.gpu.line_style);
                        }
                        // live cloud point size
                        Key::Character("[") => {
                            state.gpu.cloud_size = (state.gpu.cloud_size - 0.25).max(0.25);
                            log::info!("cloud size scale: x{}", state.gpu.cloud_size);
                        }
                        Key::Character("]") => {
                            state.gpu.cloud_size = (state.gpu.cloud_size + 0.25).min(8.0);
                            log::info!("cloud size scale: x{}", state.gpu.cloud_size);
                        }
                        Key::Character("f" | "F") => {
                            let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                            state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                        }
                        _ => {}

                    }
                }
            }

            WindowEvent::MouseInput {state: btn, button: MouseButton::Right, ..} => {
                self.orbiting = btn == ElementState::Pressed; // hold RMB to orbit
            }
            WindowEvent::MouseInput {state: btn, button: MouseButton::Middle, ..} => {
                self.panning = btn == ElementState::Pressed; // hold MMB to plan (CAD standard)
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.orbiting || self.panning {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    if self.panning || self.ctrl {
                        state.camera.pan(dx, dy);
                    } else {
                        state.camera.orbit(dx, dy)
                    };
                }
                self.last_cursor = (position.x, position.y);
            }
            WindowEvent::MouseWheel {delta, ..} => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 100.0,
                };
                // Zoom toward the curson - the point under the nouse stays put
                let vp = (state.gpu.config.width as f64, state.gpu.config.height as f64);
                state.camera.zoom_at(amount, self.last_cursor, vp);
            }
            WindowEvent::ModifiersChanged(mods)=>{
                self.ctrl = mods.state().control_key();
            }
            // A touchscreen. One finger orbits, two pan and pinch, a double tap fits - the same
            // four moves the right button, the middle button, the wheel and F give a mouse
            // (app/touch.rs). Rendering is continuous, so the redraw here is belt and braces.
            WindowEvent::Touch(t) => {
                let vp = (state.gpu.config.width as f64, state.gpu.config.height as f64);
                let dpr = device_pixel_ratio();
                match self.touch.event(&mut state.camera, &t, vp, dpr) {
                    Act::None => {}
                    Act::Moved => state.window.request_redraw(),
                    Act::Fit => {
                        state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, vp.0 / vp.1);
                        state.window.request_redraw();
                    }
                }
            }
            _ => {},
        }
    }

}

/// `?spin=1`: turn the camera a little every frame, a moving-camera benchmark needing no input.
#[cfg(target_arch = "wasm32")]
fn spin_mode() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| web_sys::window().and_then(|w| w.location().search().ok()).is_some_and(|s| s.contains("spin=1")))
}

/// Physical pixels per CSS pixel — 1 on a desktop monitor, 2-4 on a phone, and it changes under
/// the page when the browser is zoomed or the window is dragged to another screen, so it is read
/// per event rather than cached. Touch positions arrive multiplied by it (`app/touch.rs`).
#[cfg(target_arch = "wasm32")]
fn device_pixel_ratio() -> f64 {
    web_sys::window().map(|w| w.device_pixel_ratio()).filter(|d| *d > 0.0).unwrap_or(1.0)
}

/// The canvas's pixel size (CSS size × device-pixel-ratio), or `None` if zero or unavailable.
#[cfg(target_arch = "wasm32")]
fn desired_canvas_size() -> Option<(u32, u32)> {
    use wasm_bindgen::JsCast;
    let win = web_sys::window()?;
    let dpr = win.device_pixel_ratio();
    let canvas = win.document()?
        .get_element_by_id("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
    let w = (canvas.client_width()  as f64 * dpr).round() as u32;
    let h = (canvas.client_height() as f64 * dpr).round() as u32;
    (w > 0 && h > 0).then_some((w, h))
}

/// wasm entry point: install the panic hook and run the app.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
