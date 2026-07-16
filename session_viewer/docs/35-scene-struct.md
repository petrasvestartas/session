# 35 Scene struct — the app layer takes shape

> **Big picture.** *Phase 4 closes.* The viewer loads real files now, but 34b left the Session walk
> inside `Gpu` — the engine knows what a `Mesh` is. That blocks everything ahead: reconcile (38)
> diffs Sessions, picking (42) resolves guids, undo (51) snapshots objects — all *document* work that
> must live above the GPU. This lesson draws the architectural line the rest of the course builds on:
> **app owns the document, engine owns the device.**

Since lesson 34, `Gpu::new` does two unrelated jobs: it stands up the wgpu device, and it walks a
`Session` — matching every `Geometry` variant, calling `push_mesh`, running the line/point adapters.
This lesson splits them. A new `Scene` in the app layer becomes the one place that knows about
`Session`, `Mesh`, guids, and visibility, and it hands `Gpu` a single flat `ArenaUpload`; `Gpu` goes
back to knowing only `RenderVertex`, `Instance`, and buffers — the same "distribute, don't smash" move
lesson 13 made for the camera.

<svg viewBox="0 0 680 200" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="before: Gpu::new walks the Session; after: Scene walks it and hands Gpu a flat ArenaUpload" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="150" y="16" fill="#888" text-anchor="middle">before — engine/gpu.rs</text>
  <rect x="30" y="26" width="240" height="120" fill="none" stroke="#3a3a3a"/>
  <text x="150" y="48" fill="#d7dae0" text-anchor="middle">Gpu::new</text>
  <text x="150" y="70" fill="#666" text-anchor="middle">Session · match Geometry</text>
  <text x="150" y="88" fill="#666" text-anchor="middle">push_mesh · adapters</text>
  <text x="150" y="106" fill="#666" text-anchor="middle">color · visibility</text>
  <text x="150" y="130" fill="#6fb3ff" text-anchor="middle">device · surface · pipelines</text>
  <text x="300" y="90" fill="#6fb3ff" font-size="16" text-anchor="middle">▶</text>
  <text x="480" y="16" fill="#888" text-anchor="middle">after</text>
  <rect x="360" y="26" width="240" height="56" fill="none" stroke="#6fb3ff"/>
  <text x="480" y="46" fill="#d7dae0" text-anchor="middle">app::Scene</text>
  <text x="480" y="64" fill="#666" text-anchor="middle">session · guid_to_row · hidden</text>
  <text x="480" y="98" fill="#6fb3ff" text-anchor="middle">↓ ArenaUpload</text>
  <text x="480" y="112" fill="#555" text-anchor="middle">verts vids idx objects_base segments glyphs</text>
  <rect x="360" y="122" width="240" height="48" fill="none" stroke="#3a3a3a"/>
  <text x="480" y="142" fill="#d7dae0" text-anchor="middle">engine::Gpu</text>
  <text x="480" y="160" fill="#6fb3ff" text-anchor="middle">device · surface · pipelines · buffers</text>
  <text x="480" y="192" fill="#555" text-anchor="middle">no Session · no Mesh · no BRep named here</text>
</svg>

## Why

Every later phase needs the document side on its own: reconcile (38) diffs `Session` by guid, undo
(51) snapshots objects, picking (41-46) resolves a guid from a ray hit. All of that lives above
`Gpu`, so the ownership has to move now — before more code piles onto the wrong side. The litmus test
for this lesson: afterward, `grep -rn "Session\|Mesh\|BRep" src/engine/` is **empty**.

## Files we touch

```
src/engine/gpu/mod.rs        # Instance/CylinderSegment/GlyphPoint go pub; ArenaUpload; Gpu::new stops walking the Session
src/engine/gpu/adapters.rs   # DELETED — the geometry→row converters move up to the app layer
src/app/scene.rs             # NEW — Scene { session, order, guid_to_row, hidden } + Scene::build → ArenaUpload
src/app/mod.rs               # `pub mod scene;` beside 34's `pub mod persistence;`
src/state.rs                 # State gains `scene: Scene`; builds the ArenaUpload, passes it into Gpu::new
```

(`gpu/mod.rs` keeps the directory 34 introduced — 35 only empties it of `adapters.rs`, it doesn't
collapse back to a single `gpu.rs`; the directory is where 37+'s pipeline/buffer submodules land.)

`RenderVertex`, `Instance`, `CylinderSegment`, `GlyphPoint` (the byte layouts the shaders read) stay
defined in `engine/gpu/mod.rs` — they're wire formats, not document types. Only the *decisions and the
geometry walk* move out: which object gets which row, which color, which visibility bit, and the
`Mesh`/`Line`/`Point` → GPU-row conversion itself.

## Step 1 — the boundary types go `pub`, `ArenaUpload` carries the whole upload: `src/engine/gpu/mod.rs`

`Scene` will build the GPU rows now, so the three row structs (private since 31/32/34) become `pub`,
and `Instance` gains the hidden bit. `ArenaUpload` is the one type that crosses from the app layer
into `Gpu` — it holds *everything* the arena/segment/glyph passes need.

**1a. Make `Instance`, `CylinderSegment`, and `GlyphPoint` `pub`** (find each at the bottom of the
file) and add the flag const to `Instance`:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    model: [f32; 16],
    color: [f32; 4],
    flags: u32,
    _pad: [u32; 3],
}

impl Instance {
    /// Row is skipped by the draw (46). Bit 0 is reserved for FLAG_SELECTED (45).
    pub const FLAG_HIDDEN: u32 = 1 << 1;
}
```

(`CylinderSegment` and `GlyphPoint`: just add `pub` to the `struct` keyword — the fields can stay
private; `Scene` constructs them field-by-field, so also `pub` each field, or add a small
`CylinderSegment::new`/`GlyphPoint::new`. Making the fields `pub` is the smaller edit and matches how
`ArenaUpload` exposes its own.)

**1b. Add `ArenaUpload`** right after the structs — the complete, wgpu-free, Session-free boundary:

```rust
/// Everything `Gpu` needs to fill its buffers, built by `app::scene::Scene`. `objects_base` holds
/// the TRUE per-object transform + color + flags — `Gpu` builds the instance rows from it and
/// rebases them every frame (33). No Mesh, no Session, no wgpu type on the app side of this line.
pub struct ArenaUpload {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub objects_base: Vec<(Xform, [f32; 4], u32)>,   // true model, color, flags
    pub segments: Vec<CylinderSegment>,
    pub glyphs: Vec<GlyphPoint>,
}
```

## Step 2 — `Gpu::new` stops walking the Session: `src/engine/gpu/mod.rs`

**2a. Trim the import** — `Gpu` no longer names a document type:

```rust
use session_rust::{Xform, RenderVertex, Point};   // was: + Session, Geometry, Mesh, BRep, Color
```

**2b. Delete the whole 34 walk** — the `objects_base`/`verts`/`vids`/`idx`/`segments`/`glyphs`
declarations, the `for geom in session.lookup.values()` match, and the `push_mesh` function at the
bottom (it names `Mesh` — it moves to `Scene` in Step 3). At the top, drop 34's three walk-only
lines — `mod adapters;`, `use adapters::{…};`, `use session_rust::{Geometry, Session};` — and delete
`gpu/adapters.rs` itself. **Keep `use bytemuck::Zeroable;`** — the padding guards below still call
`RenderVertex::zeroed()` etc. Replace the walk with a destructure of the parameter, then build the
instance rows from `objects_base` exactly as 33 did:

```rust
let ArenaUpload { verts, vids, idx, objects_base, segments, glyphs } = upload;

// Initial instance rows from the TRUE transforms; clear()'s rebuild_instances (33) rebases
// model+color as the camera origin moves. objects_base stays the source of truth.
let mut instances: Vec<Instance> = objects_base.iter()
    .map(|(m, c, f)| Instance { model: m.to_f32(), color: *c, flags: *f, _pad: [0; 3] })
    .collect();
```

Store `objects_base` onto `self` in the `Ok(Self { … })` initializer, exactly where 33 already put
it. Two tiny 33 edits fall out of the new third tuple element (the hidden flag `Scene` now supplies):

```rust
    // struct Gpu — 33's field gains the flag:
    objects_base: Vec<(Xform, [f32; 4], u32)>,        // was (Xform, [f32; 4])

    // rebuild_instances — ignore the flag on rebase (it's set once, at build):
    for (i, (model, color, _)) in self.objects_base.iter().enumerate() {   // was (model, color)
```

That's the whole reconciliation: `rebuild_instances` still writes only `model` + `color`, never
`flags`, so a hidden object stays hidden through every camera-origin rebase. `Scene` sets the flag
once, in `build()`; `Gpu::new` reads it into the initial `Instance` row above (`flags: *f`).

**2c. Keep the counts, padding, and scene-bounds from 34** — they belong here now (Gpu owns the
wgpu "buffers can't be zero-sized" constraint). Right after the destructure:

```rust
let segment_count = segments.len() as u32;   // BEFORE padding — the real draw-call count
let glyph_count = glyphs.len() as u32;
let (mut verts, mut vids, mut idx) = (verts, vids, idx);
let (mut segments, mut glyphs) = (segments, glyphs);
if instances.is_empty() { instances.push(Instance { model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, _pad: [0; 3] }); }
if verts.is_empty()     { verts.push(RenderVertex::zeroed()); vids.push(0); idx.extend_from_slice(&[0, 0, 0]); }
if segments.is_empty()  { segments.push(CylinderSegment::zeroed()); }
if glyphs.is_empty()    { glyphs.push(GlyphPoint::zeroed()); }
let arena_index_count = idx.len() as u32;
```

The `scene_min`/`scene_max` fold (34, Step 5a) reads `verts`/`segments`/`glyphs` — unchanged, still
right here. Everything from `instance_buffer`/`segment_buffer`/`glyph_buffer` creation downward is
untouched; the buffers just fill from the upload instead of a locally-built `Vec`.

**2d. Change the signature** — take the upload, not a `Session`:

```rust
pub async fn new(window: std::sync::Arc<winit::window::Window>, upload: ArenaUpload) -> anyhow::Result<Self> {
```

## Step 3 — `Scene` owns the document: `src/app/scene.rs` (NEW)

`Scene` is the one place `Session`/`Mesh`/`Line` are visible from here on. It captures the guid order
**once** (a `HashMap` iterates unordered, so the row a guid gets here is the row it keeps until a full
rebuild — 38 teaches incremental reconcile), then `build()` is 34's walk verbatim, moved up.

```rust
use std::collections::{HashMap, HashSet};
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, Xform, RenderVertex};
use crate::engine::gpu::{Instance, CylinderSegment, GlyphPoint, ArenaUpload};

pub struct Scene {
    pub session: Session,
    order: Vec<String>,                  // renderable guids in fixed row order
    pub guid_to_row: HashMap<String, u32>,
    pub hidden: HashSet<String>,
}

impl Scene {
    pub fn new(session: Session) -> Self {
        let mut order = Vec::new();
        let mut guid_to_row = HashMap::new();
        for (guid, geom) in &session.lookup {
            if matches!(geom, Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::Line(_) | Geometry::Polyline(_) | Geometry::Point(_)) {
                guid_to_row.insert(guid.clone(), order.len() as u32);
                order.push(guid.clone());
            }
        }
        Self { session, order, guid_to_row, hidden: HashSet::new() }
    }

    /// The lesson-34 walk, moved out of `Gpu`. Emits the TRUE per-object transform (placement lives
    /// in `mesh.xform`; `to_render` ignores it) — 33's rebuild_instances rebases it every frame.
    /// `ri` is the objects_base row, never the lookup index, so nothing reads a stale instance row.
    pub fn build(&self) -> ArenaUpload {
        let mut verts: Vec<RenderVertex> = Vec::new();
        let mut vids: Vec<u32> = Vec::new();
        let mut idx: Vec<u32> = Vec::new();
        let mut segments: Vec<CylinderSegment> = Vec::new();
        let mut glyphs: Vec<GlyphPoint> = Vec::new();
        let mut objects_base: Vec<(Xform, [f32; 4], u32)> = Vec::with_capacity(self.order.len());

        for (ri, guid) in self.order.iter().enumerate() {
            let ri = ri as u32;
            let flags = if self.hidden.contains(guid) { Instance::FLAG_HIDDEN } else { 0 };
            match &self.session.lookup[guid] {
                Geometry::Mesh(m) => {
                    objects_base.push((m.xform.clone(), m.objectcolor().to_f32(), flags));
                    push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::BRep(b) => {
                    let bm = b.mesh();
                    objects_base.push((bm.xform.clone(), b.surfacecolor.to_f32(), flags));
                    push_mesh(&bm, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::Line(l) => {
                    objects_base.push((Xform::identity(), l.linecolor.to_f32(), flags));
                    segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    objects_base.push((Xform::identity(), pl.linecolor.to_f32(), flags));
                    segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    objects_base.push((Xform::identity(), p.pointcolor.to_f32(), flags));
                    glyphs.push(point_to_glyph(p, ri));
                }
                _ => {}
            }
        }
        ArenaUpload { verts, vids, idx, objects_base, segments, glyphs }
    }
}
```

**Bring the converters over from 34** — `push_mesh` (was in `engine/gpu/mod.rs`) and
`line_to_segment` / `polyline_to_segments` / `point_to_glyph` (were in the deleted
`engine/gpu/adapters.rs`) drop straight into `scene.rs` as free functions, **bodies unchanged**. They
name `Mesh`/`Line`/`Point`, so the app layer is exactly where they belong now — that's what makes the
litmus grep pass.

> **Static vs. dynamic.** `build()` runs once, at startup — the whole scene bakes into one upload,
> like lesson 30's arena. Toggling `hidden` today needs a full rebuild-and-reupload; incremental sync
> (touch only the rows that changed) is 38's job, once there's an actual edit to react to.

## Step 4 — wire it into `State`: `src/app/mod.rs`, `src/lib.rs`, `src/state.rs`

**4a. `src/app/mod.rs`** — add the module beside 34's:

```rust
pub mod persistence;
pub mod scene;   // ← ADD
```

**4b. `src/state.rs`** — `State` gains a `scene` field; `Scene::build` needs no camera (33's per-frame
rebase in `clear()` owns the origin), so the order is just fetch → scene → upload → gpu:

```rust
use crate::app::scene::Scene;

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
    pub scene: Scene,   // ← ADD — the document; everything else is unchanged from 34
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let camera = Camera::new();
        // 34's loader, unchanged — only its destination moves (into Scene, not straight to Gpu).
        let bytes = crate::app::persistence::fetch_bytes(DEMO_SESSION_URL).await.unwrap_or_default();
        let session = crate::app::persistence::session_from_bytes(DEMO_SESSION_URL, &bytes);
        let scene = Scene::new(session);
        let upload = scene.build();
        let gpu = Gpu::new(window.clone(), upload).await?;
        Ok(Self { window, gpu, camera, scene })
    }
    // resize / render unchanged — render() still reads gpu.config and camera.view_proj/origin
}
```

## Step 5 — run and check the litmus test

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Identical pixels to 34 — the floor model, then the line-heavy stress file, both draw exactly as
before; meshes, lines, and points all survive the move (that's the bug this refactor exists to *not*
introduce). Then the real test:

```bash
grep -rn "Session\|Mesh\|BRep" src/engine/
```

Empty. `engine/` now compiles against `RenderVertex`, `Xform`, and its own `Instance`/`CylinderSegment`/
`GlyphPoint`/`ArenaUpload` — nothing that knows what a document is.

## Recap

```
Ch 33: camera-relative — instance-row translations −= origin (f64) before the f32 cast.
Ch 34: Load a Session — Gpu::new walks a real .pb/.json into the arena/segment/glyph tables.
Ch 35: SPLIT. The ENTIRE 34 walk (all Geometry variants, push_mesh + line/point adapters) moves out
       of Gpu into app/scene.rs::Scene, which owns Session + guid_to_row + hidden and emits one flat
       ArenaUpload (verts, vids, idx, objects_base, segments, glyphs). objects_base carries the TRUE
       transform/color/hidden-bit; Gpu builds the Instance rows from it, KEEPS 33's per-frame rebase,
       applies the empty-buffer guards + scene bounds, and otherwise returns to pure device/surface/
       pipelines/buffers — like 13's camera extraction. The three GPU-row structs go pub (wire
       formats stay in engine); the geometry converters go up to app (they name Mesh/Line). Litmus:
       engine/ names no Session, Mesh, or BRep.
```

Edited: `engine/gpu/mod.rs` (`Instance`/`CylinderSegment`/`GlyphPoint` → `pub` + `FLAG_HIDDEN`, new
`ArenaUpload`, `Gpu::new` takes it instead of a `Session`, walk + `push_mesh` + `mod adapters` removed),
`engine/gpu/adapters.rs` (DELETED — moved up), `app/scene.rs` (NEW — `Scene`, `Scene::build`, the
converters), `app/mod.rs` (`pub mod scene;`), `state.rs` (`State.scene`, fetch → scene → upload → gpu).

## Next

`36-scene-bvh.md` — `Scene` now has a fixed, ordered object list; the next lesson gives it a
broad-phase AABB BVH over their world boxes. One BVH, reused by frustum culling (37), picking (42),
and box-select (45) — the "one acceleration structure, many uses" principle, and the reason the object
list had to stabilize here first.
