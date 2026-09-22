use super::decode::session_from_bytes;
use super::fetch::{fetch_bytes, sleep_ms};
use super::live::LiveSource;
use super::manifest::Manifest;
use super::route::AUTO_GRID;
use super::route::{SceneRoute, join, knob_u32, named_scene, scene_route};
use super::scene::{FileDoc, Scene, SheetInit, StreamedInit};
use super::stream::{
    CloudFields, SheetFields, cloud_fields, cloud_lod, fetch_colors, fetch_positions,
    fetch_sheet_slice, sheet_fields,
};
use super::walk::cloud::StreamRows;
use super::walk::sheet::SheetRows;
use crate::engine::performance::now_ms;
use crate::{CloudChunk, Msg, SheetChunk, State};
use session_rust::Xform;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;

/// Points a streamed cloud reads before its first frame.
const STREAM_PREFIX_POINTS: u32 = 2_000_000;

/// Points per follow-up slice.
const STREAM_CHUNK_POINTS: u32 = 2_000_000;

/// Most streamed points on the page, `?points=` overrides.
const STREAM_MAX_POINTS: u32 = 6_000_000;

/// Files this large are always streamed.
const STREAM_MIN_BYTES: u64 = 64 * 1024 * 1024;

/// Fewest points a streamed cloud gets, even over budget.
const STREAM_MIN_PREFIX: u32 = 250_000;

/// Segments a sheet reads before its first frame.
const SHEET_PREFIX_SEGMENTS: u32 = 500_000;

/// Segments per follow-up slice.
const SHEET_CHUNK_SEGMENTS: u32 = 500_000;

/// Most sheet segments on the page, `?segments=` overrides.
const SHEET_MAX_SEGMENTS: u32 = 3_000_000;

thread_local! {
    /// Sends messages into the event loop.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };

    /// Streamed points loaded so far.
    static RESIDENT: Cell<u32> = const { Cell::new(0) };

    /// Sheet segments loaded so far.
    static SHEET_RESIDENT: Cell<u32> = const { Cell::new(0) };

    /// Bumped when the scene is cleared; old stream tasks stop.
    static GENERATION: Cell<u32> = const { Cell::new(0) };

    /// Bumped on every reload request; older loads give up.
    static LOAD_GENERATION: Cell<u64> = const { Cell::new(0) };
}

/// Clear the scene and stop every stream.
fn clear_scene() {
    GENERATION.set(GENERATION.get().wrapping_add(1));
    RESIDENT.set(0);
    SHEET_RESIDENT.set(0);
    post(Msg::Clear);
}

/// Send one message to the event loop; false when it is gone.
pub(super) fn post(msg: Msg) -> bool {
    PROXY.with_borrow(|proxy| {
        proxy
            .as_ref()
            .is_some_and(|proxy| proxy.send_event(msg).is_ok())
    })
}

/// The point ceiling, from `?points=` or the default.
fn max_points() -> u32 {
    knob_u32("points").unwrap_or(STREAM_MAX_POINTS)
}

/// Points still allowed.
fn budget_left() -> u32 {
    max_points().saturating_sub(RESIDENT.get())
}

/// Count `n` points as loaded.
fn budget_spend(n: u32) {
    RESIDENT.set(RESIDENT.get().saturating_add(n));
}

/// The segment ceiling, from `?segments=` or the default.
fn max_segments() -> u32 {
    knob_u32("segments").unwrap_or(SHEET_MAX_SEGMENTS)
}

/// Segments still allowed.
fn sheet_budget_left() -> u32 {
    max_segments().saturating_sub(SHEET_RESIDENT.get())
}

/// Count `n` segments as loaded.
fn sheet_budget_spend(n: u32) {
    SHEET_RESIDENT.set(SHEET_RESIDENT.get().saturating_add(n));
}

/// Start the viewer, load the first scene, then keep polling.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    PROXY.with_borrow_mut(|slot| *slot = Some(proxy.clone()));
    let state = match State::new(window, Scene::new()).await {
        Ok(state) => state,
        Err(error) => {
            super::feedback::error(&format!(
                "Unable to start WebGPU: {error}. Use a browser with an available WebGPU adapter, then reload."
            ));
            return;
        }
    };
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));

    let mut live = LiveSource::from_query();
    let mut loaded = false;

    if let Some(src) = live.as_mut() {
        log::info!("live: watching {} every {:.0} ms", src.url, src.poll_ms);
        loaded = post_live(src).await;
    }

    if !loaded && let Some(route) = scene_route() {
        load_route(&route, None).await;
    }

    let Some(mut src) = live else { return };

    loop {
        sleep_ms(src.tick_ms).await;
        post_live(&mut src).await;
    }
}

/// One live poll; true when the scene was replaced.
async fn post_live(src: &mut LiveSource) -> bool {
    let generation = LOAD_GENERATION.get();
    let Some(docs) = src.check().await else {
        return false;
    };

    if stale_load(generation) {
        return false;
    }

    let texts = src.texts();

    if docs.is_empty() && texts.is_empty() {
        return false;
    }

    clear_scene();

    for doc in docs {
        post(Msg::File(doc));
    }

    post(Msg::Texts(texts));
    post(Msg::Fit);
    super::feedback::status("");
    true
}

/// Load a named scene, or reload the page's own, keeping the camera.
#[wasm_bindgen]
pub fn reload_scene(url: Option<String>) {
    let route = match url {
        Some(path) => Some(named_scene(&path)),
        None => scene_route(),
    };
    let Some(route) = route else {
        log::warn!("reload_scene: this page has no scene route - nothing to reload");
        return;
    };
    let generation = LOAD_GENERATION.get().wrapping_add(1);
    LOAD_GENERATION.set(generation);
    wasm_bindgen_futures::spawn_local(load_replacement(route, generation));
}

/// Load a route as a replacement.
async fn load_replacement(route: SceneRoute, generation: u64) {
    load_route(&route, Some(generation)).await;
}

/// True when a newer load has started since.
fn stale_load(generation: u64) -> bool {
    LOAD_GENERATION.get() != generation
}

/// Fetch the manifest; a missing `.toml` falls back to `.yaml`.
async fn fetch_manifest(route: &SceneRoute) -> Result<Vec<u8>, String> {
    match fetch_bytes(&route.manifest).await {
        Err(error) if error.starts_with("HTTP 404") && route.manifest.ends_with(".toml") => {
            let yaml = format!("{}.yaml", route.manifest.trim_end_matches(".toml"));
            super::feedback::status("Opening the current YAML scene for this TOML bookmark");
            fetch_bytes(&yaml).await
        }
        result => result,
    }
}

/// Load every item of a manifest; a reload swaps the scene only once complete.
async fn load_route(route: &SceneRoute, replacement: Option<u64>) {
    let generation = match replacement {
        Some(generation) => generation,
        None => LOAD_GENERATION.get(),
    };
    let mut pending = Vec::new(); // staged items of a reload
    let mut failed = false;
    let budget = scene_budget_bytes(); // whole-file bytes allowed
    let mut spent = 0u64;
    let mut skipped: Vec<String> = Vec::new();
    let mut staged_points = 0u32;
    let mut staged_segments = 0u32;
    let t0 = now_ms();
    let bytes = match fetch_manifest(route).await {
        Ok(b) => b,
        Err(e) => {
            super::feedback::status(&format!("Cannot fetch the scene manifest: {e}"));
            return;
        }
    };

    if stale_load(generation) {
        return;
    }

    let manifest = match Manifest::parse(&bytes) {
        Ok(m) => m,
        Err(e) => {
            super::feedback::status(&format!("Cannot read the scene manifest: {e}"));
            return;
        }
    };
    log::info!("scene '{}': {} items", manifest.name, manifest.items.len());

    let mut files = 0u32;

    for item in &manifest.items {
        if item.file.ends_with(".pb") {
            files += 1;
        }
    }

    let files = files.max(1);
    let share = (max_points() / files).max(STREAM_MIN_PREFIX);

    for (i, item) in manifest.items.iter().enumerate() {
        let url = join(&route.base, &item.file);
        let place = manifest.place(i, AUTO_GRID);
        let point_px = item.point_size as f32;

        if url.ends_with(".pb") {
            let slot = Placement {
                name: manifest.name_of(i, &item.file),
                place: place.clone(),
                point_px,
            };
            let remaining = if replacement.is_some() {
                max_points().saturating_sub(staged_points)
            } else {
                budget_left()
            };

            if let Some(init) =
                stream_prefix(&url, &slot, share.min(remaining.max(STREAM_MIN_PREFIX))).await
            {
                if stale_load(generation) {
                    return;
                }

                if init.resident == 0 {
                    failed = true;
                    continue;
                }

                if replacement.is_some() {
                    staged_points = staged_points.saturating_add(init.resident);
                    pending.push(PendingDocument::Streamed(Box::new(init)));
                } else {
                    budget_spend(init.resident);
                    post(Msg::StreamedCloud(Box::new(init)));
                }

                continue;
            }

            let remaining = if replacement.is_some() {
                max_segments().saturating_sub(staged_segments)
            } else {
                sheet_budget_left()
            };

            if let Some(init) = sheet_prefix(&url, &slot, remaining).await {
                if stale_load(generation) {
                    return;
                }

                if init.resident == 0 {
                    failed = true;
                    continue;
                }

                if replacement.is_some() {
                    staged_segments = staged_segments.saturating_add(init.resident);
                    pending.push(PendingDocument::Sheet(Box::new(init)));
                } else {
                    sheet_budget_spend(init.resident);
                    post(Msg::Sheet(Box::new(init)));
                }

                continue;
            }
        }

        // skip a file the device cannot hold
        let length = super::fetch::content_length(&url).await.unwrap_or(0);

        if spent + length > budget {
            log::warn!(
                "skipped '{}': {} MB over the {} MB scene budget",
                item.file,
                length >> 20,
                budget >> 20
            );
            skipped.push(format!("{} ({} MB)", item.file, length >> 20));
            continue;
        }

        spent += length;
        let f0 = now_ms();
        let bytes = match fetch_bytes(&url).await {
            Ok(b) => b,
            Err(e) => {
                super::feedback::status(&format!("Unable to load {}: {e}", item.file));
                failed = true;
                continue;
            }
        };
        let n = bytes.len();
        let f1 = now_ms();
        let session = match session_from_bytes(&url, bytes).await {
            Ok(session) => session,
            Err(error) => {
                super::feedback::status(&format!("Cannot decode {}: {error}", item.file));
                failed = true;
                continue;
            }
        };

        if stale_load(generation) {
            return;
        }

        if session.lookup.is_empty() {
            log::warn!("'{}' holds no geometry ({n} bytes); skipped", item.file);
            failed = true;
            continue;
        }

        let name = manifest.name_of(i, &session.name);
        log::info!(
            "loaded '{name}': {} objects, {n} bytes | fetch {:.0} ms, parse {:.0} ms",
            session.lookup.len(),
            f1 - f0,
            now_ms() - f1
        );
        let doc = FileDoc {
            name,
            session: Rc::new(session),
            place,
            point_px,
            display_only: item.display_only,
        };

        if replacement.is_some() {
            pending.push(PendingDocument::Whole(doc));
        } else {
            post(Msg::File(doc));
        }
    }

    if stale_load(generation) {
        return;
    }

    if replacement.is_some() {
        if failed {
            super::feedback::status(
                "Scene replacement failed; the last valid scene is still visible",
            );
            return;
        }

        clear_scene();
        budget_spend(staged_points);
        sheet_budget_spend(staged_segments);

        for document in pending {
            match document {
                PendingDocument::Whole(doc) => {
                    post(Msg::File(doc));
                }
                PendingDocument::Streamed(stream) => {
                    post(Msg::StreamedCloud(stream));
                }
                PendingDocument::Sheet(sheet) => {
                    post(Msg::Sheet(sheet));
                }
            }
        }
    }

    post(Msg::Texts(manifest.texts));
    post(Msg::Fit);

    if !failed {
        super::feedback::status(&skipped_notice(&skipped, budget));
    }

    log::info!(
        "scene posted {:.0} ms after the manifest fetch",
        now_ms() - t0
    );
}

/// One staged item of a reload.
enum PendingDocument {
    Whole(FileDoc),              // a decoded file
    Streamed(Box<StreamedInit>), // a cloud's first slice
    Sheet(Box<SheetInit>),       // a sheet's first slice
}

/// Name and placement of a streamed document.
struct Placement {
    name: String,  // display name
    place: Xform,  // world placement
    point_px: f32, // point size, clouds only
}

/// Read a cloud's first `share` points by range; None when it should load whole.
async fn stream_prefix(url: &str, slot: &Placement, share: u32) -> Option<StreamedInit> {
    let (name, place, point_px) = (slot.name.as_str(), slot.place.clone(), slot.point_px);
    let mut fields = cloud_fields(url).await?;

    if fields.count <= STREAM_PREFIX_POINTS && fields.coords_len < STREAM_MIN_BYTES {
        return None;
    }

    let lod = cloud_lod(url, &mut fields).await?;
    let resident = STREAM_PREFIX_POINTS.min(share).min(fields.count);
    let Some(positions) = fetch_positions(url, &fields, 0, resident).await else {
        log::warn!(
            "'{name}': the prefix range read failed - the cloud stays off screen (a whole decode would take {:.0} MB)",
            fields.coords_len as f64 / 1.048576e6
        );
        return Some(StreamedInit {
            name: name.to_string(),
            url: url.to_string(),
            place,
            rows: StreamRows {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            lod,
            col_at: fields.colors_at,
            fields,
            resident: 0,
            point_px,
        });
    };
    let (colors, col_at) = fetch_colors(url, &fields, fields.colors_at, resident)
        .await
        .unwrap_or((Vec::new(), fields.colors_at));
    log::info!(
        "streamed '{name}': {resident} of {} points on screen, {} nodes",
        fields.count,
        lod.len()
    );
    Some(StreamedInit {
        name: name.to_string(),
        url: url.to_string(),
        place,
        rows: StreamRows { positions, colors },
        lod,
        fields,
        resident,
        point_px,
        col_at,
    })
}

/// `name` in the same folder as `url`.
fn sibling(url: &str, name: &str) -> String {
    let dir = url.rfind('/').map_or(0, |at| at + 1);
    format!("{}{name}", &url[..dir])
}

/// Read a sheet's first `share` segments by range; None when not a sheet file.
async fn sheet_prefix(url: &str, slot: &Placement, share: u32) -> Option<SheetInit> {
    let name = slot.name.as_str();
    let fields = sheet_fields(url).await?;
    let meta_url = (!fields.meta.is_empty()).then(|| sibling(url, &fields.meta));
    let mut resident = SHEET_PREFIX_SEGMENTS.min(share).min(fields.count);
    let rows = match fetch_sheet_slice(url, &fields, 0, resident).await {
        Some(rows) if resident > 0 => rows,
        _ => {
            log::warn!(
                "'{name}': no sheet prefix - {resident} of {} segments allowed (?segments= to raise the ceiling) or the range read failed",
                fields.count
            );
            resident = 0;
            SheetRows {
                positions: Vec::new(),
                colors: Vec::new(),
                widths: Vec::new(),
                ids: Vec::new(),
            }
        }
    };
    log::info!(
        "sheet '{name}': {resident} of {} segments on screen, {} entities",
        fields.count,
        fields.entities
    );
    Some(SheetInit {
        name: name.to_string(),
        url: url.to_string(),
        meta_url,
        place: slot.place.clone(),
        rows,
        fields,
        resident,
    })
}

/// Where a sheet's streaming continues.
pub struct SheetCursor {
    pub idx: usize,          // the sheet's slot in the scene
    pub url: String,         // the sheet file
    pub fields: SheetFields, // array positions in the file
    pub from: u32,           // next segment to read
}

/// Keep reading a sheet's slices in the background.
pub fn spawn_sheet_rest(cursor: SheetCursor) {
    wasm_bindgen_futures::spawn_local(sheet_rest(cursor));
}

/// The slice loop behind `spawn_sheet_rest`.
async fn sheet_rest(c: SheetCursor) {
    let (url, idx, fields) = (c.url, c.idx, c.fields);
    let generation = GENERATION.get();
    let mut at = c.from;

    while at < fields.count {
        if GENERATION.get() != generation {
            return;
        }

        let left = sheet_budget_left();

        if left == 0 {
            log::info!(
                "'{url}': {at} of {} segments resident - at the page's segment ceiling (?segments= to raise it)",
                fields.count
            );
            return;
        }

        let to = (at + SHEET_CHUNK_SEGMENTS.min(left)).min(fields.count);
        sheet_budget_spend(to - at);
        let Some(rows) = fetch_sheet_slice(&url, &fields, at, to).await else {
            if GENERATION.get() == generation {
                SHEET_RESIDENT.set(SHEET_RESIDENT.get().saturating_sub(to - at));
            }

            super::feedback::status("A sheet range failed; reload to retry the missing data");
            return;
        };

        if GENERATION.get() != generation {
            return;
        }

        if !post(Msg::SheetChunk(SheetChunk { idx, rows, to })) {
            return;
        }

        at = to;
    }
}

/// Where a cloud's streaming continues.
pub struct StreamCursor {
    pub idx: usize,          // the cloud's slot in the scene
    pub url: String,         // the cloud file
    pub fields: CloudFields, // array positions in the file
    pub from: u32,           // next point to read
    pub col_at: u64,         // byte position of its colour
}

/// Keep reading a cloud's slices in the background.
pub fn spawn_stream_rest(cursor: StreamCursor) {
    wasm_bindgen_futures::spawn_local(stream_rest(cursor));
}

/// The slice loop behind `spawn_stream_rest`.
async fn stream_rest(c: StreamCursor) {
    let (url, idx, fields) = (c.url, c.idx, c.fields);
    let generation = GENERATION.get();
    let mut col_at = c.col_at;
    let mut at = c.from;

    while at < fields.count {
        if GENERATION.get() != generation {
            return;
        }

        let left = budget_left();

        if left == 0 {
            log::info!(
                "'{url}': {at} of {} points resident - at the page's point ceiling (?points= to raise it)",
                fields.count
            );
            return;
        }

        let to = (at + STREAM_CHUNK_POINTS.min(left)).min(fields.count);
        budget_spend(to - at);
        let Some(positions) = fetch_positions(&url, &fields, at, to).await else {
            if GENERATION.get() == generation {
                RESIDENT.set(RESIDENT.get().saturating_sub(to - at));
            }

            super::feedback::status("A point-cloud range failed; reload to retry the missing data");
            return;
        };
        let (colors, next) = fetch_colors(&url, &fields, col_at, to - at)
            .await
            .unwrap_or((Vec::new(), col_at));
        col_at = next;

        if GENERATION.get() != generation {
            return;
        }

        if !post(Msg::CloudChunk(CloudChunk {
            idx,
            rows: StreamRows { positions, colors },
            to,
        })) {
            return;
        }
        at = to;
    }
}

/// Bytes a scene may load: `?budget=` or 16 MB per GB.
fn scene_budget_bytes() -> u64 {
    if let Some(mb) = crate::engine::gpu::view::knob("VIEWER_BUDGET", "budget")
        && let Ok(mb) = mb.parse::<u64>()
    {
        return mb << 20;
    }

    let gigabytes = web_sys::window()
        .map(|window| window.navigator())
        .and_then(|navigator| js_sys::Reflect::get(&navigator, &"deviceMemory".into()).ok())
        .and_then(|value| value.as_f64())
        .unwrap_or(4.0);
    ((gigabytes * 16.0) as u64) << 20
}

/// The message naming skipped files, if any.
fn skipped_notice(skipped: &[String], budget: u64) -> String {
    if skipped.is_empty() {
        return String::new();
    }

    format!(
        "Skipped over the {} MB scene budget: {}. Add ?budget=<MB> to raise it.",
        budget >> 20,
        skipped.join(", ")
    )
}
