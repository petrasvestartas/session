# 35 Scene struct — the document comes back

> **Big picture.** Since 34e the viewer has NO document. Each `Session` is parsed, walked into
> flat GPU tables, and **thrown away** — deliberately, to survive nine 120k-object files at once.
> That was right for the stress wall and is wrong for everything ahead: picking (42) must answer
> "which OBJECT did the ray hit" (a guid), undo (51) must snapshot geometry, save (39) must write
> a `.pb` back — all of that needs the real `Session` alive in memory. This lesson brings it back
> and draws the line the rest of the course builds on: **app owns the document, engine owns the
> device.**

## What holds the geometry now? (read this first)

Four types, and which one is "the truth" at each stage:

| type | where | what it is | lifetime |
|---|---|---|---|
| `Session` | kernel (`session_rust`) | THE document: `lookup: HashMap<guid → Geometry>` — the exact same data structure as `session_py`/`session_cpp`; what `.pb`/`.json` files serialize | 34e: dies right after the walk. **35: lives forever, inside `Scene`** |
| `SceneTables` | engine (34e) | flat GPU tables for ONE walked file, anonymous — no guids, no way back to objects | **deleted this lesson** |
| `Scene` | app (**NEW**) | `Session` + the viewer-only bookkeeping the kernel must not carry: row order, `guid_to_row`, `hidden` | owned by `State`, lives as long as the viewer |
| `ArenaUpload` | engine (**NEW**) | `SceneTables` reborn as a one-way HANDOFF: `Scene::build()` produces it, `Gpu::new` consumes it | one call |

So nothing exotic replaces `Session` — `Session` IS the data structure, kept alive as
`state.scene.session`. `Scene` exists only because row order / hidden / (later) selection are
*viewer* state, not *document* state: they must not leak into the kernel type that three
languages share and that serializes to disk. When later lessons say "the document", they mean
`scene.session`; when they say "row" or "instance id", they mean the GPU-side index that
`scene.guid_to_row` translates guids into.

<svg viewBox="0 0 680 200" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="before: Sessions are dropped after walking; after: Scene keeps the Session and hands Gpu a flat ArenaUpload" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="150" y="16" fill="#888" text-anchor="middle">34e — no document</text>
  <rect x="30" y="26" width="240" height="120" fill="none" stroke="#3a3a3a"/>
  <text x="150" y="48" fill="#d7dae0" text-anchor="middle">fetch → parse → walk</text>
  <text x="150" y="70" fill="#c66" text-anchor="middle">Session DROPPED</text>
  <text x="150" y="88" fill="#666" text-anchor="middle">SceneTables (anonymous)</text>
  <text x="150" y="130" fill="#6fb3ff" text-anchor="middle">Gpu walks + merges + draws</text>
  <text x="300" y="90" fill="#6fb3ff" font-size="16" text-anchor="middle">▶</text>
  <text x="480" y="16" fill="#888" text-anchor="middle">35 — app owns the document</text>
  <rect x="360" y="26" width="240" height="56" fill="none" stroke="#6fb3ff"/>
  <text x="480" y="46" fill="#d7dae0" text-anchor="middle">app::Scene</text>
  <text x="480" y="64" fill="#666" text-anchor="middle">session (KEPT) · guid_to_row · hidden</text>
  <text x="480" y="98" fill="#6fb3ff" text-anchor="middle">↓ ArenaUpload</text>
  <text x="480" y="112" fill="#555" text-anchor="middle">verts vids idx objects_base segments glyphs</text>
  <rect x="360" y="122" width="240" height="48" fill="none" stroke="#3a3a3a"/>
  <text x="480" y="142" fill="#d7dae0" text-anchor="middle">engine::Gpu</text>
  <text x="480" y="160" fill="#6fb3ff" text-anchor="middle">device · surface · pipelines · buffers</text>
  <text x="480" y="192" fill="#555" text-anchor="middle">engine code names no Session / Mesh / BRep</text>
</svg>

**The 34e stress wall retires here.** The wall proved the renderer; every lesson from now on
works on ONE document (multi-document comes back much later). That means: one URL again, no grid
merge loop, no `STRESS_GRID` — and the walk moves from `Gpu::walk_session` into `Scene::build`,
almost line-for-line.

## Files we touch

```
src/engine/gpu/mod.rs   # Step 1: SceneTables/walk_session/push_mesh OUT, ArenaUpload IN,
                        #         row structs go pub, Gpu::new takes the upload
src/engine/gpu/adapters.rs  # Step 1g: DELETED — converters move to scene.rs
src/app/scene.rs        # Step 2: NEW — Scene { session, order, guid_to_row, hidden } + build()
src/app/mod.rs          # Step 3: pub mod scene;
src/state.rs            # Step 4: one URL, State.scene, fetch → Scene → build → Gpu
index.html              # Step 5 (optional): drop the 8 stress-wall copy-file lines
```

> ⚠ Nothing compiles between Step 1 and the end of Step 4 — the walk is changing owners. Type it
> all, then `cargo check`.

## Step 1 — `Gpu` forgets the document: `src/engine/gpu/mod.rs`

Nine edits, strictly top to bottom.

**1a. Imports.** Find at the top:

```rust
use adapters::{line_to_segment, point_to_glyph, polyline_to_segments, encode_width};
use bytemuck::Zeroable;
use session_rust::{Mesh, Xform, RenderVertex, Point, Geometry};
use session_rust::mesh::ColorMode;
```

Replace all four lines with (also delete the `mod adapters;` line just above them):

```rust
use bytemuck::Zeroable;
use session_rust::{Xform, RenderVertex, Point};
```

`Zeroable` stays — the padding guards still call `CylinderSegment::zeroed()` etc.
(`RenderVertex` is a kernel type too, but it's the agreed wire format — it stays.)

**1b. `SceneTables` dies.** Find:

```rust
/// Grid floor for load testing: at least STRESS_GRID² cells, cycling the loaded files.
const STRESS_GRID: u32 = 3;
```

and **delete from that doc comment down to and including the closing `}` of
`pub struct SceneTables { … }`** — the next line you KEEP is `pub struct Gpu {`.

**1c. The field grows a flag.** In `pub struct Gpu`, find:

```rust
    objects_base: Vec<(Xform, [f32; 4])>, // TRUE world model+color; isntance[] is rebased from this
```

Replace with (`Scene` supplies a hidden bit per object now):

```rust
    objects_base: Vec<(Xform, [f32; 4], u32)>, // TRUE world model+color+flags; instances[] is rebased from this
```

**1d. The signature.** Find:

```rust
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        files: &[SceneTables]) -> anyhow::Result<Self> {
```

Replace the parameter line so it reads:

```rust
    pub async fn new(
        window: std::sync::Arc<winit::window::Window>,
        upload: ArenaUpload) -> anyhow::Result<Self> {
```

**1e. The tables come from the upload.** Scroll to the six `let mut …` declarations (after the
Time-uniform block). **Delete everything from**

```rust
        let mut verts: Vec<RenderVertex> = Vec::new(); // slot 0 - every mesh's vertices, concatenated
```

**down to and including the end of the `let mut instances` statement** — its last line is:

```rust
        }).collect();
```

(that deletion swallows all six declarations, the whole grid merge loop with its `scene_min`/
`scene_max`/`cells`/`cols` machinery, and the old instances build). The next line you KEEP is
`let segment_count = segments.len() as u32; // Before padding…`. **In the hole, insert:**

```rust
        let ArenaUpload { verts, vids, idx, objects_base, segments, glyphs } = upload;
        let (mut verts, mut vids, mut idx) = (verts, vids, idx);
        let (mut segments, mut glyphs) = (segments, glyphs);

        // Initial instance rows from the TRUE transforms; clear()'s rebuild_instances (33)
        // rebases model+color as the camera origin moves. objects_base stays the source of truth.
        let mut instances: Vec<Instance> = objects_base.iter()
        .map(|(m, c, f)| Instance {
            model: m.to_f32(),
            color: *c,
            flags: *f,
            _pad: [0; 3]
        }).collect();
```

Everything below — `let segment_count…`, `let glyph_count…`, `let points…`, the four
`is_empty()` padding guards — is untouched.

**1f. Bounds return, grid log dies.** Find:

```rust
        let arena_index_count = idx.len() as u32;

        log::info!("grid: {} cells x {} files: {} objects, {} arena verts, {} segments, {} glyphs",
            cells, files.len(), instances.len(), verts.len(), segments.len(), glyphs.len());
```

Replace with (the `scene_min`/`scene_max` fold lived in `walk_session` since 34e — it comes back
here, over the upload's tables):

```rust
        let arena_index_count = idx.len() as u32;

        // Bounding Box - over the upload's tables (was walk_session's job in 34e)
        let mut scene_min = [f32::INFINITY; 3];
        let mut scene_max = [f32::NEG_INFINITY; 3];
        for v in &verts { for k in 0..3 {
            scene_min[k] = scene_min[k].min(v.position[k]);
            scene_max[k] = scene_max[k].max(v.position[k]);
        } }
        for s in &segments { for p in [s.p0, s.p1] { for k in 0..3 {
            scene_min[k] = scene_min[k].min(p[k]);
            scene_max[k] = scene_max[k].max(p[k]);
        } } }
        for g in &glyphs { for k in 0..3 {
            scene_min[k] = scene_min[k].min(g.center[k]);
            scene_max[k] = scene_max[k].max(g.center[k]);
        } }

        log::info!("scene: {} objects, {} arena verts, {} segments, {} glyphs",
            instances.len(), verts.len(), segments.len(), glyphs.len());
```

From here to the end of `new()` nothing changes — the buffers fill from the upload's Vecs
exactly as they filled from the locally-built ones.

**1g. `walk_session` dies** (it moves to `Scene::build`, Step 2 — keep a copy to paste from).
Find:

```rust
    /// One file → compact tables. Called from state.rs BEFORE Gpu::new, so the parsed
    /// Session (and its bytes) can be dropped before the next file is fetched.
    pub fn walk_session(session: &session_rust::Session) -> SceneTables {
```

and **delete from that doc comment down to and including the fn's closing `}`** — the last lines
of the deletion are the 34f planar block and:

```rust
        t
    }
```

The next thing you KEEP is the `/// The anchor the instance table is rebased about.` doc comment
on `rebase_anchor`. **Also delete `src/engine/gpu/adapters.rs` entirely** (the file) — its three
converters and `encode_width` reappear in `scene.rs`.

**1h. `rebuild_instances` learns the third tuple element.** Find:

```rust
        for (i, (model, color)) in self.objects_base.iter().enumerate() {
```

Replace with (the flag is set once at build; rebasing never touches it, so hidden stays hidden
through every camera-origin rebase):

```rust
        for (i, (model, color, _)) in self.objects_base.iter().enumerate() {
```

**1i. The row structs go `pub`, `ArenaUpload` appears.** At the bottom of the file, find
`struct Instance {` and make it pub with the flag const — the whole block becomes:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    model: [f32; 16], // 64 B - column-major, from Xform::to_f32()
    color: [f32; 4], // 16 B
    flags: u32, // 4 B - bit 0 reserved for FLAG_SELECTED (45)
    _pad: [u32; 3], // 12 B - pad the row to 96 B (storage array stride)
}

impl Instance {
    /// Row is skipped by the draw (46). Bit 0 is reserved for FLAG_SELECTED (45).
    pub const FLAG_HIDDEN: u32 = 1 << 1;
}
```

(fields stay private — only `Gpu` builds `Instance` rows; `Scene` just names the flag).
Then find `struct CylinderSegment{` and `struct GlyphPoint{` further down and put `pub` on the
struct keyword **and on every field** of both — `Scene` constructs them field-by-field across
the module boundary, so one private field is a compile error there. Keep each field's comment;
only the visibility changes:

```rust
pub struct CylinderSegment{
    pub p0: [f32; 3],   // …
    pub radius: f32,    // …
    pub p1: [f32; 3],   // …
    pub instance_id: u32,  // …
    pub color: [f32; 4],  // …
}
```

```rust
pub struct GlyphPoint{
    pub center: [f32; 3], // …
    pub radius: f32, // …
    pub color:  [f32; 4],
    pub instance_id: u32, // …
    pub _pad: [u32; 3], // …
}
```

Then insert `ArenaUpload` right after the `GlyphPoint` block (before the
`// Points inscribed in circles used for pointclouds` comment):

```rust
/// Everything `Gpu` needs to fill its buffers, built by `app::scene::Scene`. `objects_base`
/// holds the TRUE per-object transform + color + flags — `Gpu` builds the instance rows from it
/// and rebases them every frame (33). No Mesh, no Session, no wgpu type on the app side of this line.
pub struct ArenaUpload {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub objects_base: Vec<(Xform, [f32; 4], u32)>,   // true model, color, flags
    pub segments: Vec<CylinderSegment>,
    pub glyphs: Vec<GlyphPoint>,
}
```

**1j. `push_mesh` dies** (it names `Mesh` — it moves to `scene.rs`). Near the bottom, find:

```rust
fn push_mesh(
```

and **delete the whole function** down to and including its closing `}` — the next thing you
KEEP is the `/// A read-only storage buffer that is never zero-sized…` doc comment on
`storage_buffer`.

## Step 2 — `Scene` owns the document: `src/app/scene.rs` (NEW FILE)

Create the file with exactly this content. Read it as three parts: the struct (Session + the
bookkeeping), `build()` (34e's walk with a `flags` third tuple element and 34f's planar pass at
the end), and the converters (34's `adapters.rs` + `push_mesh`, bodies unchanged — only
`pub`/`pub(super)` dropped, they're file-local now):

```rust
//! `Scene` — the open DOCUMENT. It owns the kernel `Session` (the same guid → Geometry
//! data structure session_py/session_cpp use) plus the viewer-only bookkeeping the kernel
//! must not know about: row order, guid→row map, hidden set. Everything document-shaped
//! (reconcile 38, picking 42, undo 51) talks to THIS type; `Gpu` only ever sees the flat
//! `ArenaUpload` that `build()` emits.

use std::collections::{HashMap, HashSet};
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, Xform, RenderVertex};
use session_rust::mesh::ColorMode;
use crate::engine::gpu::{Instance, CylinderSegment, GlyphPoint, ArenaUpload};

pub struct Scene {
    pub session: Session,                // THE document — kernel type, source of truth
    order: Vec<String>,                  // renderable guids in fixed row order
    pub guid_to_row: HashMap<String, u32>,
    pub hidden: HashSet<String>,
}

impl Scene {
    pub fn new(session: Session) -> Self {
        let mut order = Vec::new();
        let mut guid_to_row = HashMap::new();
        // session.order() is the kernel's CANONICAL order — deterministic across runs and
        // languages; Scene keeps the renderable subset of it.
        for guid in session.order() {
            if matches!(&session.lookup[&guid], Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::Line(_) |
                              Geometry::Polyline(_) | Geometry::Point(_)) {
                guid_to_row.insert(guid.clone(), order.len() as u32);
                order.push(guid);
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
                    // white TINT (34h) — the real colors ride the rows; placement = xform
                    objects_base.push((m.xform.clone(), [1.0; 4], flags));
                    push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());   // 34h's surfacecolor bake
                    objects_base.push((b.xform.clone(), [1.0; 4], flags));
                    push_mesh(&bm, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::Line(l) => {
                    objects_base.push((l.xform.clone(), [1.0; 4], flags));
                    segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    objects_base.push((pl.xform.clone(), [1.0; 4], flags));
                    segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    objects_base.push((p.xform.clone(), [1.0; 4], flags));
                    glyphs.push(point_to_glyph(p, ri));
                }
                _ => {} // Scene::new only put renderable guids into order
            }
        }

        // 34f's paper-space lane, moved here with the walk: planar (z ≡ 0) sheets get
        // world-mm lineweights; 3D files keep screen-constant px.
        let (mut zmin, mut zmax) = (f32::INFINITY, f32::NEG_INFINITY);
        for s in &segments {
            zmin = zmin.min(s.p0[2].min(s.p1[2]));
            zmax = zmax.max(s.p0[2].max(s.p1[2]));
        }
        if zmin.is_finite() && (zmax - zmin).abs() < 1e-3 {
            for s in &mut segments {
                s.radius = if s.radius < 0.0 { -s.radius * 0.5 } else { 0.5 };
            }
        }

        ArenaUpload { verts, vids, idx, objects_base, segments, glyphs }
    }
}

// ── geometry → GPU-row converters, moved up from engine/gpu (34's adapters.rs + push_mesh) ──
// They name Mesh/Line/Point — document types — which is exactly why they live in the app layer.

fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment{
    CylinderSegment {
        p0: l.start().to_f32(),
        radius: encode_width(l.width),
        p1: l.end().to_f32(),
        instance_id,
        color: l.linecolor.to_f32(),
    }
}

fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment>{
    let pts = pl.get_points();
    let color = pl.linecolor.to_f32();
    pts.windows(2).map(|w| CylinderSegment{
        p0: w[0].to_f32(),
        radius: encode_width(pl.width),
        p1: w[1].to_f32(),
        instance_id,
        color,
    }).collect()
}

fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint{
    GlyphPoint {
        center: p.to_f32(),
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id,
        _pad: [0; 3],
    }
}

/// Kernel width (dimensionless, default 1.0) → the radius encoding's NEGATIVE lane (px
/// multiplier); 0.0 = plain global default. `Scene::build` flips negatives into the POSITIVE
/// (world-mm) lane for planar 2D drawings — paper-space lineweights that scale with zoom.
fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 { -(w as f32) } else { 0.0 }
}

fn push_mesh(
    m: &Mesh,
    ri: u32,
    verts: &mut Vec<RenderVertex>,
    vids: &mut Vec<u32>,
    idx: &mut Vec<u32>,
    segments: &mut Vec<CylinderSegment>,
    glyphs: &mut Vec<GlyphPoint>
){
    let base = verts.len() as u32;
    let rm = m.to_render();
    for v in &rm.vertices{
        verts.push(*v);
        vids.push(ri);
    }
    for &i in &rm.indices{
        idx.push(base+i);
    }

    for (i, (a, b, col)) in m.edges_with_colors().into_iter().enumerate(){
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();
        segments.push(
            CylinderSegment{
                p0: pa.to_f32(),
                radius: encode_width(m.widths().get(i).copied().unwrap_or(1.0)),
                p1: pb.to_f32(),
                instance_id: ri,
                color: col.to_f32()
            }
        )
    }

    // Dots honor user-set pointcolors; the auto-seeded white vec is filtered by the MODE gate.
    // m.vertices() is sorted — the same order to_render indexes pointcolors by.
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();
    for (i, vk) in m.vertices().into_iter().enumerate(){
        let p = m.vertex_point(vk).unwrap();
        glyphs.push(
            GlyphPoint {
                center: p.to_f32(),
                radius: 0.0,                       // no per-vertex width exists in the kernel
                color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
                instance_id: ri,
                _pad: [0;3] }
        );
    }
}
```

Two things to notice while typing:

- `Scene::new` filters the kernel's `session.order()` — the CANONICAL object order (the typed
  `objects` vectors walked in one fixed sequence), identical across runs and across the three
  languages — down to the renderable subset. The row a guid gets here is the row it keeps until
  a full rebuild; that stability is what `guid_to_row` promises to picking/selection later, and
  38 teaches incremental reconcile on top of it.
- 34e's `walk_session` derived planarity from the per-file `min[2]/max[2]` extents; those fields
  are gone, so `build()` derives it from the segments' z directly — same answer for every
  drawing sheet (all ink at z ≡ 0).

## Step 3 — register the module: `src/app/mod.rs`

The file is one line. Make it two:

```rust
pub mod persistence;
pub mod scene;
```

## Step 4 — `State` holds the document: `src/state.rs`

**4a. Imports + one URL + the field.** Find:

```rust
use crate::engine::gpu::Gpu;
use crate::camera::Camera;
use crate::app::persistence;
use crate::engine::performance::now_ms;

// Runtime fetch paths — each must match an index.html copy-file target (data-target-path + filename).
const DEMO_SESSION_URLS: &[&str] = &[
    "session_data/30700_querschnitt_gg.pb",
    "session_data/draw_pb_haus25.pb",
    "session_data/draw_pc_gru_og2.pb",
    "session_data/draw_pd_treppenhaus04.pb",
    "session_data/draw_pe_schalungsbild.pb",
    "session_data/draw_pf_he.pb",
    "session_data/draw_pi_laengsschnitt.pb",
    "session_data/draw_pj_grundriss_og2.pb",
    "session_data/draw_pj_treppenhaus_a.pb",
];

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
}
```

Replace the whole block with:

```rust
use crate::engine::gpu::Gpu;
use crate::camera::Camera;
use crate::app::persistence;
use crate::app::scene::Scene;
use crate::engine::performance::now_ms;

// Runtime fetch path — must match an index.html copy-file target (data-target-path + filename).
// The 34e stress wall is retired: from here on the viewer holds ONE document.
const DEMO_SESSION_URL: &str = "session_data/floor_model.pb";

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    pub camera: Camera,
    pub scene: Scene, // the DOCUMENT (kernel Session + row/hidden bookkeeping)
}
```

**4b. The loader.** In `State::new`, find 34e's loop (everything from `let t0 = now_ms();`
through the `Ok(Self {…})` line):

```rust
        let t0 = now_ms();
        let mut files = Vec::new();
        for url in DEMO_SESSION_URLS {
            let f0 = now_ms();
            let bytes = persistence::fetch_bytes(url).await.unwrap_or_default();
            let f1 = now_ms();
            let session = persistence::session_from_bytes(url, &bytes);
            log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms",
                session.name, session.lookup.len(), bytes.len(), f1 - f0, now_ms() - f1);
            if !session.lookup.is_empty() {
                files.push(Gpu::walk_session(&session)); // failed fetch = skipped file
            }
            // `session` + `bytes` DROP here — peak memory holds one parsed file, not all nine
        }
        let t1 = now_ms();
        let gpu = Gpu::new(window.clone(), &files).await?;
        log::info!("{} files | load {:.0}ms · gpu {:.0}ms", files.len(), t1 - t0, now_ms() - t1);
        Ok(Self {window, gpu, camera: Camera::new() })
```

Replace with — the crucial difference from 34e is the comment on the `Scene::new` line:

```rust
        let t0 = now_ms();
        let bytes = persistence::fetch_bytes(DEMO_SESSION_URL).await.unwrap_or_default();
        let session = persistence::session_from_bytes(DEMO_SESSION_URL, &bytes);
        log::info!("loaded '{}': {} objects, {} bytes in {:.0}ms",
            session.name, session.lookup.len(), bytes.len(), now_ms() - t0);
        let scene = Scene::new(session);          // the Session LIVES ON — Scene owns it
        let upload = scene.build();               // document → flat GPU tables
        let gpu = Gpu::new(window.clone(), upload).await?;
        Ok(Self {window, gpu, camera: Camera::new(), scene })
```

`resize`/`render` are untouched — `render()` still does 34c's anchor dance against `gpu` and
`camera` only.

## Step 5 (optional) — slim the dist: `index.html`

The 8 `draw_p*.pb` copy-file lines from 34e now only bloat every `trunk` build (~200MB of
fixtures nothing loads). Delete them; KEEP `floor_model.pb` (today's URL) and
`30700_querschnitt_gg.pb` (handy for re-testing paper widths — point `DEMO_SESSION_URL` at it
any time).

## Verify

`trunk serve` → the floor model renders exactly as before: same colors (white tint × row colors,
34h), same thin edges, same vertex dots, `F` still fits. Swap `DEMO_SESSION_URL` to
`"session_data/30700_querschnitt_gg.pb"` and the drawing still comes up dark-red with paper-space
pen weights — `Scene::build` carried the 34f lane over. Then the litmus test:

```bash
grep -rn "Session\|Mesh\|BRep" src/engine/ | grep -v "//"
```

Empty — the only survivors are two comments. `engine/` now compiles against `Xform`,
`RenderVertex`, `Point`, and its own `Instance`/`CylinderSegment`/`GlyphPoint`/`ArenaUpload` —
nothing that knows what a document is. And the document is BACK: in the browser console,
`state.scene.session` holds every object by guid, ready for picking, undo, and save.

## Recap

```
Ch 34e: STREAM, WALK, DROP — fast, but the viewer held no document at all (just GPU tables).
Ch 35:  THE DOCUMENT RETURNS. Scene (app layer) OWNS the kernel Session — the same guid→Geometry
        structure as session_py/session_cpp — plus viewer-only bookkeeping: order (fixed rows),
        guid_to_row, hidden. Scene::build = 34's walk + 34h tints + 34f paper lane, emitting one
        flat ArenaUpload; Gpu::new consumes it and returns to pure device/surface/pipelines/
        buffers (33's rebase kept, now over (Xform, color, flags)). Row structs go pub (wire
        formats stay in engine); converters move up (they name Mesh/Line). The stress wall
        retires — one document from here on. Litmus: engine/ names no Session/Mesh/BRep in code.
```

Edited: `engine/gpu/mod.rs` (imports trimmed, `SceneTables`+`walk_session`+`push_mesh`+merge loop
deleted, `ArenaUpload` + pub row structs + `FLAG_HIDDEN`, `objects_base` gains flags),
`engine/gpu/adapters.rs` (DELETED), `app/scene.rs` (NEW), `app/mod.rs` (+`pub mod scene;`),
`state.rs` (one URL, `State.scene`, fetch → Scene → build → Gpu), `index.html` (optional slim).

## Next

`36-scene-bvh.md` — `Scene` now has a fixed, ordered object list; the next lesson gives it a
broad-phase AABB BVH over their world boxes. One BVH, reused by frustum culling (37), picking
(42), and box-select (45) — the "one acceleration structure, many uses" principle, and the reason
the object list had to stabilize here first.
