# 34 Load a Session — the kernel file format arrives

Every object on screen so far came from Rust code written by hand. This lesson deletes that: the
five-mesh demo is replaced by a real `.pb`/`.json` file — fetched over HTTP (wasm32 has no
filesystem) and walked, geometry-type by geometry-type, into the same arena/segment/glyph tables
lessons 30–32 already built.

## Why

`session_rust::Session` is already the kernel's file format — C++, Python and Rust all dump/load
the identical bytes through `pb_dumps`/`pb_loads` (protobuf) and `file_json_dump`/`file_json_load`
(JSON); the CI minitests round-trip thousands of them. Reading one browser-side is one call,
`Session::pb_loads(&bytes)`. The real work is the last mile: the lesson-30 arena loop only ever
saw `Mesh`. A real `Session` is heterogeneous — `session.lookup: HashMap<String, Geometry>` mixes
Mesh, BRep, Line, Polyline, Point, Plane, OBB, PointCloud in one file — so the loop grows a `match`
over every variant, and lines/points get their own adapters for the first time.

<svg viewBox="0 0 680 190" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="bytes fetched over HTTP become a Session, whose lookup map is matched by geometry type into the arena, segment, glyph and instance tables" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g stroke="#6fb3ff" stroke-width="1.5" fill="none">
    <rect x="10"  y="24" width="90"  height="34"/>
    <rect x="150" y="24" width="230" height="34"/>
    <rect x="430" y="24" width="230" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="55"  y="45">bytes</text>
    <text x="265" y="45">Session::pb_loads / jsonload</text>
    <text x="545" y="45">lookup{ guid → Geometry }</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.5">
    <line x1="100" y1="41" x2="146" y2="41" marker-end="url(#ah34)"/>
    <line x1="380" y1="41" x2="426" y2="41" marker-end="url(#ah34)"/>
    <path d="M545,58 V95 H265 V121" fill="none" marker-end="url(#ah34)"/>
  </g>
  <text x="120" y="34" fill="#666" font-size="9">fetch</text>
  <g stroke="#3a3a3a" fill="none">
    <rect x="70"  y="125" width="390" height="34"/>
  </g>
  <text x="265" y="146" fill="#d7dae0" text-anchor="middle">match: Mesh·BRep / Line·Polyline / Point → adapters</text>
  <g stroke="#6fb3ff" stroke-width="1.5">
    <line x1="460" y1="142" x2="500" y2="142" marker-end="url(#ah34)"/>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.5" fill="none">
    <rect x="504" y="125" width="160" height="34"/>
  </g>
  <text x="584" y="141" fill="#d7dae0" text-anchor="middle" font-size="10">arena · segments</text>
  <text x="584" y="153" fill="#d7dae0" text-anchor="middle" font-size="10">glyphs · instances[]</text>
  <text x="584" y="178" fill="#888" text-anchor="middle">→ GPU (draws unchanged since 31/32)</text>
  <defs>
    <marker id="ah34" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
      <path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/>
    </marker>
  </defs>
</svg>

## Files we touch

```
Cargo.toml                       # web-sys: Request, RequestInit, RequestMode, Response (fetch API)
index.html                       # Trunk copy-file — fixture .pb bytes served next to the wasm
src/app/persistence.rs           # NEW — fetch_bytes (fetch API) + session_from_bytes (.pb/.json dispatch)
src/engine/gpu.rs → gpu/mod.rs   # mechanical split, content unchanged
src/engine/gpu/adapters.rs       # NEW — Line/Polyline/Point → CylinderSegment/GlyphPoint
src/lib.rs                       # mod app; the F-to-fit handler reads real scene bounds
src/state.rs                     # the demo hook — fetch + parse before Gpu::new
```

`app/` is new territory: `engine/mod.rs`'s own doc comment says app-specific code lives in `app/`,
"never here" — this is the first lesson that needs it. Everything else (pipelines, shaders,
the arena/segment/glyph buffers) is untouched; only their *source* changes, from a hardcoded `Vec`
to a parsed file.

## Step 1 — fetch bytes: wasm has no `std::fs` — `src/app/persistence.rs`

**1a. Add the fetch-API features to `Cargo.toml`**, in the existing `web-sys` feature list (find the
`features = [` array ending in `"Performance"]`):

```toml
web-sys = { version = "0.3", features = [
    "Document",
    "Window",
    "Element",
    "HtmlCanvasElement",
    "EventTarget",
    "Event",
    "CanvasRenderingContext2d",
    "ImageData",
    "Location",
    "Performance",
    "Request",
    "RequestInit",
    "RequestMode",
    "Response"] }
```

**1b. Create `src/app/persistence.rs`.** Two functions: get bytes, then hand them to `Session`'s
own loaders — the SAME `pb_loads`/`file_json_loads` every other language's minitest already proves
round-trip correctly, just fed bytes/a string instead of a filepath:

```rust
//! Session loading — the kernel file format arrives. wasm32 has no filesystem, so the fetch API
//! is the only way to reach a `.pb`/`.json` file (std::fs is not an option here).

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use session_rust::Session;

/// GET `url` (Trunk-served, same origin as the page) and return the raw bytes.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// `.pb` → prost, `.json` → serde — dispatched on `url`'s extension. Both loaders already exist
/// on `Session` (used by every language's minitest); a failed/empty fetch degrades to
/// `Session::default()` — an empty scene, not a panic.
pub fn session_from_bytes(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        Session::file_json_loads(&String::from_utf8_lossy(bytes))
    } else {
        Session::pb_loads(bytes).unwrap_or_default()
    }
}
```

> `Session::pb_load(path)`/`file_json_load(path)` (no trailing `s`) read from `std::fs` — those
> panic on wasm32. The `_loads` pair (bytes/string in, no path) is the browser-safe half of the
> exact same API and is what every language's `file_json_dump`/`file_json_load` minitest already
> exercises.
>
> **New ground, checked against the pinned crate:** neither `session_viewer_archive` nor today's
> `session_viewer` has ever fetched anything — the archive's own `ARCHITECTURE.md` lists file
> loading as a documented but unbuilt "Phase 1" (`app/persistence.rs`, `<input type="file">` →
> `ArrayBuffer` → `pb_loads`). The signatures above (`RequestInit::new`/`.method`/`.mode`,
> `Request::new_with_str_and_init`, `Window::fetch_with_request`, `Response::array_buffer`) were
> confirmed against `web-sys 0.3.99` in this crate's own `Cargo.lock` — not just recalled from
> memory — but the *flow* itself (async fetch → bytes → `Session`) is new and still wants a real
> `trunk serve` + browser click-through before you trust it in anger.
>
> A user-driven alternative sketched by that same roadmap: `<input type="file">` + a
> `web_sys::FileReader` (`readAsArrayBuffer`, callback-based — bridge it into a future with a
> oneshot channel) feeding the same `session_from_bytes`. Left for the file-menu lesson; the fetch
> path above is what the fixtures and the stress gate need today.

## Step 2 — serve the fixtures: `index.html`

Trunk only ships what `index.html` tells it to. **Add two `copy-file` links** next to the existing
`data-trunk rel="rust"` link — `session_data/` is a sibling of `session_viewer/`, and Trunk resolves
`href` relative to `index.html` at build time (a `..` is fine; it never reaches the browser):

```html
  <link data-trunk rel="rust" data-target-name="session_viewer" data-wasm-opt="0"/>
  <link data-trunk rel="copy-file" href="../session_data/floor_model.pb" data-target-path="session_data"/>
  <link data-trunk rel="copy-file" href="../session_data/30700_querschnitt_gg.pb" data-target-path="session_data"/>
  <canvas id="canvas"></canvas>
```

With `Trunk.toml`'s `public_url = "./"`, these land at `dist/session_data/*.pb` and are reachable
at runtime as `session_data/floor_model.pb` — the URL Step 6 fetches.

## Step 3 — split `gpu.rs` into a directory, add the adapters: `gpu/mod.rs` + `gpu/adapters.rs`

`gpu.rs` (392 lines) is about to grow a `match` over `Geometry`'s nine variants; the archive hit the
same wall and split `gpu_session.rs` into `engine/gpu/{types,session,geometry,adapters,…}.rs`. Do
the small, mechanical piece of that now.

**3a. Rename `src/engine/gpu.rs` → `src/engine/gpu/mod.rs`, content unchanged.** `engine/mod.rs`'s
`pub mod gpu;` needs no edit — Rust resolves `mod gpu;` to either `gpu.rs` or `gpu/mod.rs`.

**3b. Create `src/engine/gpu/adapters.rs`** — pure `Type → GPU row` converters, ported from
`session_viewer_archive/src/engine/gpu/adapters.rs`'s `line_to_segment` / `polyline_to_segments`,
trimmed to what Step 4 calls (endpoint glyphs are discussed, not wired — see the callout below):

```rust
//! Session geometry → GPU rows. `CylinderSegment`/`GlyphPoint` are private to `gpu/mod.rs`, but
//! Rust visibility is "this module and its descendants" — adapters.rs is a child of gpu, so it
//! sees them through a plain `use super::…`, no `pub` needed on either struct.

use super::{CylinderSegment, GlyphPoint};
use session_rust::{Line, Point, Polyline};

pub fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment {
    CylinderSegment { p0: l.start().to_f32(), radius: 0.0, p1: l.end().to_f32(), instance_id, color: l.linecolor.to_f32() }
}

pub fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment> {
    let pts = pl.get_points();
    let color = pl.linecolor.to_f32();
    pts.windows(2).map(|w| CylinderSegment {
        p0: w[0].to_f32(), radius: 0.0, p1: w[1].to_f32(), instance_id, color,
    }).collect()
}

pub fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint {
    GlyphPoint { center: p.to_f32(), radius: 0.0, color: p.pointcolor.to_f32(), instance_id, _pad: [0; 3] }
}
```

`Point::to_f32() -> [f32; 3]` and `Color::to_f32() -> [f32; 4]` are the same casts lesson 31/32
already used for mesh edges/vertices — reused verbatim here for standalone geometry.

> The archive also glyphs every line/polyline **endpoint** (`line_endpoint_glyphs`,
> `polyline_endpoint_glyphs`) — skipped here on purpose. Measured on the stress-gate file (Step 8):
> 40,814 lines + 1,418 polylines would add **84,464** extra sphere instances (144 tris each ≈ 12M
> triangles) for zero visual value at that density. That's a selection/hover affordance, not a
> default-on one — reintroduce it in the picking lesson, scoped to the hovered/selected object only.

## Step 4 — walk the session into the arena + instance table: `src/engine/gpu/mod.rs`

**4a. Add the `session` parameter.** Find `pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {` and thread a `Session` through:

```rust
    pub async fn new(window: std::sync::Arc<winit::window::Window>, session: &session_rust::Session) -> anyhow::Result<Self> {
```

**4b. Replace the whole scene-building block** — lesson 30's hardcoded `objects: Vec<(Mesh, Xform,
[f32; 4])>` together with the loop that filled `verts`/`vids`/`idx`/`instances`, plus lesson 31/32's
`segments`/`glyphs` collection (same loop, same variables — this is one wholesale swap, not three
separate edits). Add `mod adapters;` and extend the top-of-file import first:

```rust
mod adapters;
use adapters::{line_to_segment, point_to_glyph, polyline_to_segments};
use bytemuck::Zeroable;
use session_rust::{Geometry, Session};
```

Then the new loop:

```rust
        let mut verts: Vec<RenderVertex> = Vec::new();
        let mut vids: Vec<u32> = Vec::new();
        let mut idx: Vec<u32> = Vec::new();
        let mut segments: Vec<CylinderSegment> = Vec::new();
        let mut glyphs: Vec<GlyphPoint> = Vec::new();
        let mut objects_base: Vec<(Xform, [f32; 4])> = Vec::with_capacity(session.lookup.len());

        // Each object's PLACEMENT lives in its xform — `to_render()` reads the stored vertices and
        // ignores the xform, so the xform IS the instance model (identity for standalone lines/points,
        // whose segment/glyph coordinates are already world). objects_base keeps the TRUE placement;
        // lesson 33's rebuild_instances rebases model+color against the camera origin every frame.
        // `ri` is the row in objects_base, NOT the lookup index — so skipped variants (Plane/OBB/…)
        // leave no hole for the shader's instances[instance_id] to read wrong.
        for geom in session.lookup.values() {
            let ri = objects_base.len() as u32;
            match geom {
                Geometry::Mesh(m) => {
                    objects_base.push((m.xform.clone(), m.objectcolor().to_f32()));
                    push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::BRep(b) => {
                    let bm = b.mesh();
                    objects_base.push((bm.xform.clone(), b.surfacecolor.to_f32()));
                    push_mesh(&bm, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::Line(l) => {
                    objects_base.push((Xform::identity(), l.linecolor.to_f32()));
                    segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    objects_base.push((Xform::identity(), pl.linecolor.to_f32()));
                    segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    objects_base.push((Xform::identity(), p.pointcolor.to_f32()));
                    glyphs.push(point_to_glyph(p, ri));
                }
                Geometry::Plane(_) | Geometry::OBB(_) | Geometry::PointCloud(_) | Geometry::Element(_) => {}  // next lesson
            }
        }

        // Initial instance rows from the true placements; 33's rebuild_instances rebases each frame.
        let mut instances: Vec<Instance> = objects_base.iter()
            .map(|(m, c)| Instance { model: m.to_f32(), color: *c, flags: 0, _pad: [0; 3] })
            .collect();

        let segment_count = segments.len() as u32;   // BEFORE padding — the real draw-call count
        let glyph_count = glyphs.len() as u32;

        // A real file isn't the five-mesh demo: a pure line drawing (Step 8) has ZERO mesh verts,
        // a pure mesh file has zero segments. wgpu buffers can't be zero-sized, so pad the CPU
        // side with one placeholder — *_count above already captured the true number, so an empty
        // category still draws NOTHING, it just doesn't crash the buffer upload.
        if instances.is_empty() { instances.push(Instance { model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, _pad: [0; 3] }); }
        if verts.is_empty()     { verts.push(RenderVertex::zeroed()); vids.push(0); idx.extend_from_slice(&[0, 0, 0]); }
        if segments.is_empty()  { segments.push(CylinderSegment::zeroed()); }
        if glyphs.is_empty()    { glyphs.push(GlyphPoint::zeroed()); }

        let arena_index_count = idx.len() as u32;
        log::info!("session '{}': {} objects, {} arena verts, {} segments, {} glyphs",
            session.name, instances.len(), verts.len(), segments.len(), glyphs.len());
```

`segment_count`/`glyph_count` feed the exact same fields lessons 31/32 already added to `Gpu` — only
their *source* changed. Everything from `instance_buffer`/`segment_buffer`/`glyph_buffer` creation
downward is untouched.

> **Precision caveat (33).** Rebasing the instance *model* keeps meshes (local vertices + an xform
> placement) solid at any distance. But a Line/Polyline/Point writes its coordinates straight into the
> segment/glyph buffers, already f32 — so a drawing authored millions of units from the origin loses
> precision at build time, before any per-frame rebase can help. These fixtures sit near the origin,
> so it doesn't bite; the real fix (subtract the origin in f64 *before* filling those buffers, and
> rebuild them when it moves) is a later concern — flagged, not silently skipped.

**4c. Add `push_mesh`** near `unit_cylinder`/`unit_sphere` at the bottom of the file — this is
lesson 31's and 32's per-object loop bodies, unchanged, just factored into a function so `Mesh` and
`BRep` (which becomes a `Mesh` via `.mesh()`) share it instead of duplicating both blocks per arm:

```rust
fn push_mesh(m: &Mesh, ri: u32, verts: &mut Vec<RenderVertex>, vids: &mut Vec<u32>, idx: &mut Vec<u32>,
             segments: &mut Vec<CylinderSegment>, glyphs: &mut Vec<GlyphPoint>) {
    let base = verts.len() as u32;
    let rm = m.to_render();
    for v in &rm.vertices { verts.push(*v); vids.push(ri); }
    for &i in &rm.indices { idx.push(base + i); }

    for (a, b, col) in m.edges_with_colors() {
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();
        segments.push(CylinderSegment { p0: pa.to_f32(), radius: 0.0, p1: pb.to_f32(), instance_id: ri, color: col.to_f32() });
    }
    for vk in m.naked_vertices(true) {
        let p = m.vertex_point(vk).unwrap();
        glyphs.push(GlyphPoint { center: p.to_f32(), radius: 0.0, color: [0.1, 0.1, 0.1, 1.0], instance_id: ri, _pad: [0; 3] });
    }
}
```

## Step 5 — scene bounds, so `F` fits real data: `gpu/mod.rs` + `src/lib.rs`

`F` has fit a hardcoded `SCENE_MIN`/`SCENE_MAX` since lesson 15 — sized for the five-mesh demo. A
loaded file has no relation to those numbers, so `F` needs the real extent of what just got built.

**5a. In `gpu/mod.rs`, right after the empty-buffer guards** (Step 4b), fold the min/max of every
vertex, segment endpoint, and glyph centre — one cheap pass, no BVH (that's lesson 36's job for
culling; this is just a camera target):

```rust
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];
        for v in &verts { for k in 0..3 { scene_min[k] = scene_min[k].min(v.position[k]); scene_max[k] = scene_max[k].max(v.position[k]); } }
        for s in &segments { for p in [s.p0, s.p1] { for k in 0..3 { scene_min[k] = scene_min[k].min(p[k]); scene_max[k] = scene_max[k].max(p[k]); } } }
        for g in &glyphs { for k in 0..3 { scene_min[k] = scene_min[k].min(g.center[k]); scene_max[k] = scene_max[k].max(g.center[k]); } }
```

Store `scene_min`/`scene_max` on `Gpu` — add `pub scene_min: [f32; 3], pub scene_max: [f32; 3],` to
the struct (next to `arena_index_count`) and to the `Ok(Self { … })` initializer.

**5b. In `src/lib.rs`, drop the hardcoded constants** (`const SCENE_MIN`/`SCENE_MAX` near the top)
and **point the `F` handler at the real bounds**:

```rust
                        Key::Character("f" | "F") => {
                            let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                            state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                        }
```

`Camera::fit(min: [f32;3], max: [f32;3], aspect: f64)` is unchanged (lesson 15) — only where the
box comes from changes.

## Step 6 — wire the load: `src/lib.rs` + `src/state.rs`

**6a. In `lib.rs`, declare the new module** next to the existing three:

```rust
mod engine;
mod state;
mod camera;
mod app;   // ← ADD — the first app-layer file (engine/mod.rs said this was coming)
```

**6b. In `state.rs`, fetch before building the GPU state.** `State::new` has been `async` since the
very first window chapter (the wasm init pattern) — awaiting the fetch here is free, no new plumbing:

```rust
use crate::app::persistence;

const DEMO_SESSION_URL: &str = "session_data/floor_model.pb";

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let bytes = persistence::fetch_bytes(DEMO_SESSION_URL).await.unwrap_or_default();
        let session = persistence::session_from_bytes(DEMO_SESSION_URL, &bytes);
        let gpu = Gpu::new(window.clone(), &session).await?;
        Ok(Self { window, gpu, camera: Camera::new() })
    }
    // resize / render unchanged
}
```

A failed fetch (offline, 404) degrades to `bytes = vec![]` → `pb_loads` errors → `Session::default()`
— an empty scene draws (Step 4's padding guards), it does not panic.

## Step 7 — run: the first real fixture

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

`floor_model.pb` (a compas_tf timber floor, 3.0 MB) is 491 objects — verified by loading it through
`session_rust` directly: **201 Mesh + 290 Polyline**, nothing else. That single file exercises both
adapter paths at once: meshes tessellate into the arena (5,650 verts) with their edges as segments
(15,095) and boundary vertices as glyphs (373); polylines add 1,800 more segments. Console:

```
session 'floor_model': 491 objects, 5650 arena verts, 16895 segments, 373 glyphs
perf: 60.0 fps | 16.67 ms | 6 draws | 491 objects
```

Press `F` — the whole floor fits, not the old five-mesh demo box. Draw count is unchanged from 32
— the same 6 calls (background, grid, triangles, cylinders, spheres, billboards) fire every frame
regardless of how much each table holds, this file included — only the row counts inside each
buffer grew.

## Step 8 — the stress gate: a real PDF, converted to curves

Swap `DEMO_SESSION_URL` to `"session_data/30700_querschnitt_gg.pb"` — `30700 Querschnitt G-G.pdf`,
a real technical drawing, converted by `session_data/pdf_to_session.py` (lines, polylines, béziers →
curves; no meshes at all). Verified the same way: **40,814 Line + 1,418 Polyline**, 17.5 MB.

```
session 'my_session': 42232 objects, 1 arena verts, 51166 segments, 1 glyphs
perf: __._ fps | __.__ ms | 6 draws | 42232 objects
```

(`1 arena verts` / `1 glyphs` are Step 4's placeholder — this file has no meshes and no bare
`Point`s, so both categories are legitimately empty.) **51,166 cylinder segments, ONE
`draw_indexed`** — this is lesson 31's whole point proven at scale: the draw count doesn't move,
only the segment table's row count does. Record the fps this logs at, then press `F`: the entire
drawing should frame, and orbit/pan should stay interactive at this density. If it doesn't, the
next lever is exactly what Phase 5 is for — frustum culling (lesson 37) trims the segment table to
what's on screen instead of uploading all 51k every frame.

> `session_tests/` (the roadmap's original pointer) holds the Vue viewer's per-language *test
> report* JSON, not scene fixtures — real loadable `.pb`/`.json` dumps live in `session_data/`,
> which is what Steps 7–8 use. `session_data/closest_point.pb` was tried first as a smaller fixture
> and rejected: it fails to decode against the current `session_rust` proto schema (an older dump,
> pre-dating a schema change) — a real gotcha worth knowing about before picking a fixture blind.

## Recap

```
Ch 30-32: the arena/segment/glyph tables, built from FIVE HARDCODED meshes.
Ch 34:    the SAME tables, now built from a REAL FILE. `app/persistence.rs` fetches bytes (wasm32
          has no std::fs) and hands them to `Session::pb_loads`/`file_json_loads` — the identical
          entry points every language's minitest round-trips. `gpu.rs` becomes `gpu/mod.rs` +
          `gpu/adapters.rs` (Line/Polyline/Point → CylinderSegment/GlyphPoint, ported from the
          archive); the lesson-30 loop grows a `match` over `Geometry`'s 9 variants, with Mesh/BRep
          reusing 31/32's edge+glyph extraction via a shared `push_mesh`. Each object's placement
          lives in its xform (to_render ignores it), so the instance model IS mesh.xform (identity for
          standalone lines/points), rebased every frame by 33; instance_id is the objects_base row,
          not the lookup index, so skipped variants leave no hole. Verified on two real files: floor_model.pb (491
          objects, both adapter paths) and the STRESS GATE, a PDF technical drawing converted to
          curves (42,232 objects, 51,166 segments, ONE draw_indexed). `F` now fits the loaded scene's
          real bounds, not a hardcoded box.
```

Edited: `Cargo.toml` (fetch-API web-sys features), `index.html` (Trunk `copy-file` fixtures),
`src/app/persistence.rs` (NEW — `fetch_bytes` + `session_from_bytes`), `src/engine/gpu.rs` →
`src/engine/gpu/mod.rs` (session-driven build loop, `push_mesh`, empty-buffer guards, scene bounds),
`src/engine/gpu/adapters.rs` (NEW — `line_to_segment`/`polyline_to_segments`/`point_to_glyph`),
`src/lib.rs` (`mod app;`, `F` reads real bounds), `src/state.rs` (fetch + parse before `Gpu::new`).

## Next

`35-scene-struct.md` — the mesh list currently lives inside `Gpu`; ARCHITECTURE.md's target has it
in `app/scene.rs` (`Scene { session, guid → row map, visibility, … }`) with `Gpu` back to pure
device/surface/pipeline ownership. Reloading a *second* file, hiding an object, or editing one in
place all need that `guid → row` map this lesson didn't build — Session objects flow in today, but
nothing remembers *which row* a given `guid` landed in once the load is done.
