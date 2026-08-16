# 34b Load a Session II — walk it into the GPU tables

> **Big picture.** 34a proved bytes → `Session`; the GPU still draws five hand-made meshes. This
> lesson deletes that demo: a `match` over every `Geometry` variant walks the real file into the
> arena/segment/glyph tables from 30–32. The payoff is the whole point of Phase 4 — a 42,232-object
> technical drawing renders with the *same six draw calls* as the five-mesh demo. Only the row
> counts grow.

The lesson-30 arena loop only ever saw `Mesh`. A real `Session` is heterogeneous —
`session.lookup: HashMap<String, Geometry>` mixes Mesh, BRep, Line, Polyline, Point, Plane, OBB,
PointCloud in one file — so the loop grows a `match`, and lines/points get their own adapters for the
first time.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the Session lookup map is matched by geometry type into the arena, segment, glyph and instance tables" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="26" width="200" height="34" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="110" y="47" fill="#d7dae0" text-anchor="middle">lookup{ guid → Geometry }</text>
  <rect x="250" y="26" width="230" height="34" fill="none" stroke="#3a3a3a"/>
  <text x="365" y="47" fill="#d7dae0" text-anchor="middle">match: Mesh·BRep / Line·Polyline / Point</text>
  <rect x="520" y="26" width="150" height="34" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="595" y="41" fill="#d7dae0" text-anchor="middle" font-size="10">arena · segments</text>
  <text x="595" y="53" fill="#d7dae0" text-anchor="middle" font-size="10">glyphs · instances[]</text>
  <g stroke="#6fb3ff" stroke-width="1.5">
    <line x1="210" y1="43" x2="246" y2="43" marker-end="url(#ah34b)"/>
    <line x1="480" y1="43" x2="516" y2="43" marker-end="url(#ah34b)"/>
  </g>
  <text x="595" y="86" fill="#888" text-anchor="middle">→ draws unchanged since 31/32</text>
  <defs>
    <marker id="ah34b" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
      <path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/>
    </marker>
  </defs>
</svg>

## Files we touch

```
src/engine/gpu.rs → gpu/mod.rs   # mechanical split, content unchanged
src/engine/gpu/adapters.rs       # NEW — Line/Polyline/Point → CylinderSegment/GlyphPoint
src/engine/gpu/mod.rs            # Gpu::new takes &Session; the demo loop becomes the Geometry match
src/lib.rs                       # the F-to-fit handler reads real scene bounds
src/state.rs                     # pass 34a's parsed session into Gpu::new
```

## Step 1 — split `gpu.rs` into a directory, add the adapters

`gpu.rs` (~830 lines) is about to grow a `match` over `Geometry`'s 11 variants; the archive hit the
same wall and split its gpu module the same way.

**1a. Rename `src/engine/gpu.rs` → `src/engine/gpu/mod.rs`, content unchanged.** `engine/mod.rs`'s
`pub mod gpu;` needs no edit — Rust resolves `mod gpu;` to either `gpu.rs` or `gpu/mod.rs`.

**1b. Create `src/engine/gpu/adapters.rs`** — pure `Type → GPU row` converters, ported from the
archive's `adapters.rs`:

```rust
//! Session geometry → GPU rows. `CylinderSegment`/`GlyphPoint` are private to `gpu/mod.rs`, but
//! Rust visibility is "this module and its descendants" — adapters.rs is a child of gpu, so it
//! sees them through a plain `use super::…`, no `pub` needed on either struct.

use super::{CylinderSegment, GlyphPoint};
use session_rust::{Line, Point, Polyline};

pub fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment {
    CylinderSegment { p0: l.start().to_f32(), radius: 0.0, p1: l.end().to_f32(),
        instance_id, color: l.linecolor.to_f32() }
}

pub fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment> {
    let pts = pl.get_points();
    let color = pl.linecolor.to_f32();
    pts.windows(2).map(|w| CylinderSegment {
        p0: w[0].to_f32(), radius: 0.0, p1: w[1].to_f32(), instance_id, color,
    }).collect()
}

pub fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint {
    GlyphPoint { center: p.to_f32(), radius: 0.0, color: p.pointcolor.to_f32(),
        instance_id, _pad: [0; 3] }
}
```

`Point::to_f32() -> [f32; 3]` and `Color::to_f32() -> [f32; 4]` are the same casts 31/32 already used.

> The archive also glyphs every line/polyline **endpoint** — skipped on purpose. On the stress file
> (Step 6) that would add **84,464** extra sphere instances (~12M triangles) for zero visual value at
> that density. It's a selection/hover affordance — reintroduce it in the picking lessons, scoped to
> the hovered object only.

## Step 2 — walk the session: `src/engine/gpu/mod.rs`

**2a. Add the `session` parameter.** Find
`pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {` and
thread the `Session` through (34a's `State::new` already has it in hand):

```rust
    pub async fn new(window: std::sync::Arc<winit::window::Window>,
                     session: &session_rust::Session) -> anyhow::Result<Self> {
```

and in `state.rs`, change the call to match:

```rust
        let gpu = Gpu::new(window.clone(), &session).await?;   // was: Gpu::new(window.clone())
```

**2b. Replace the whole scene-building block** — lesson 30's hardcoded `objects: Vec<(Mesh, Xform,
[f32; 4])>` together with the loop that filled `verts`/`vids`/`idx`/`instances`, plus 31/32's
`segments`/`glyphs` collection (same loop, same variables — one wholesale swap). **32b's bunny
point-cloud block goes too** (the `read_xyz_from_str` load and its extra `instances` row) — it's demo
content like the five meshes, and the object counts below only match without its +1 row. Keep an
empty `let points: Vec<CloudPoint> = Vec::new();` in its place — the cloud pipeline stays wired,
`storage_buffer`'s zero-size guard pads the upload, and `point_count = 0` draws nothing until the
`PointCloud` variant is wired in a later lesson. Add `mod adapters;` and extend the top-of-file
import first:

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

        // Each object's PLACEMENT lives in its xform — the kernel convention on EVERY type
        // (stored coordinates are local, xform is the pending placement; `transform()` bakes it
        // and resets to identity). `to_render()`/`start()`/`get_points()` all read the stored
        // coordinates and ignore the xform, so the xform IS the instance model — the
        // cylinder/sphere shaders apply instances[instance_id].model to p0/p1/center exactly like
        // the mesh path. objects_base keeps the TRUE placement; lesson 33's rebuild_instances
        // rebases model+color against the camera origin every frame.
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
                    // b.xform, NOT bm.xform — mesh() builds via Mesh::from_polylines, whose
                    // xform is ALWAYS identity; the BRep's placement lives on the BRep itself.
                    let bm = b.mesh();
                    objects_base.push((b.xform.clone(), b.surfacecolor.to_f32()));
                    push_mesh(&bm, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::Line(l) => {
                    objects_base.push((l.xform.clone(), l.linecolor.to_f32()));
                    segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    objects_base.push((pl.xform.clone(), pl.linecolor.to_f32()));
                    segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    objects_base.push((p.xform.clone(), p.pointcolor.to_f32()));
                    glyphs.push(point_to_glyph(p, ri));
                }
                // later lessons — the match MUST stay exhaustive over all 11 variants
                // (no wildcard: adding a kind should force a compile error here, not a silent skip).
                // PointCloud is skipped even though 32b's CloudPoint infra exists — wiring it is
                // deliberate later work, not an oversight.
                Geometry::Plane(_) | Geometry::OBB(_) |
                Geometry::PointCloud(_) | Geometry::Element(_) |
                Geometry::NurbsCurve(_) | Geometry::NurbsSurface(_) => {}
            }
        }

        // Initial instance rows from the true placements;
        // 33's rebuild_instances rebases each frame.
        let mut instances: Vec<Instance> = objects_base.iter()
            .map(|(m, c)| Instance { model: m.to_f32(), color: *c, flags: 0, _pad: [0; 3] })
            .collect();

        let segment_count = segments.len() as u32;   // BEFORE padding — the real draw-call count
        let glyph_count = glyphs.len() as u32;

        // A real file isn't the five-mesh demo: a pure line drawing (Step 6) has ZERO mesh verts,
        // a pure mesh file has zero segments. wgpu buffers can't be zero-sized, so pad the CPU
        // side with one placeholder — *_count above already captured the true number, so an empty
        // category still draws NOTHING, it just doesn't crash the buffer upload.
        if instances.is_empty() { instances.push(Instance { model: Xform::identity().to_f32(),
            color: [0.5, 0.5, 0.5, 1.0], flags: 0, _pad: [0; 3] }); }
        if verts.is_empty()     { verts.push(RenderVertex::zeroed()); vids.push(0);
            idx.extend_from_slice(&[0, 0, 0]); }
        if segments.is_empty()  { segments.push(CylinderSegment::zeroed()); }
        if glyphs.is_empty()    { glyphs.push(GlyphPoint::zeroed()); }

        let arena_index_count = idx.len() as u32;
        log::info!("session '{}': {} objects, {} arena verts, {} segments, {} glyphs",
            session.name, instances.len(), verts.len(), segments.len(), glyphs.len());
```

<svg viewBox="0 0 620 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="lookup entries in map order compact into objects_base rows; skipped variants leave no row, so instance_id indexes the compacted row not the map slot" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="120" y="16" fill="#888" text-anchor="middle">session.lookup (map order)</text>
  <text x="500" y="16" fill="#888" text-anchor="middle">objects_base / instances[]</text>
  <!-- lookup rows -->
  <g>
    <rect x="30" y="28" width="180" height="24" fill="none" stroke="#6fb3ff"/>
    <text x="40" y="44" fill="#d7dae0">Mesh</text>
    <rect x="30" y="58" width="180" height="24" fill="none" stroke="#555"/>
    <text x="40" y="74" fill="#666">Plane</text><text x="196" y="74" fill="#e06c6c" text-anchor="end">✗</text>
    <rect x="30" y="88" width="180" height="24" fill="none" stroke="#6fb3ff"/>
    <text x="40" y="104" fill="#d7dae0">Line</text>
    <rect x="30" y="118" width="180" height="24" fill="none" stroke="#555"/>
    <text x="40" y="134" fill="#666">NurbsSurface</text><text x="196" y="134" fill="#e06c6c" text-anchor="end">✗</text>
    <rect x="30" y="148" width="180" height="24" fill="none" stroke="#6fb3ff"/>
    <text x="40" y="164" fill="#d7dae0">Point</text>
  </g>
  <!-- compacted rows -->
  <g>
    <rect x="410" y="28" width="180" height="24" fill="none" stroke="#5bbf87"/>
    <text x="420" y="44" fill="#d7dae0">ri 0 · Mesh</text>
    <rect x="410" y="88" width="180" height="24" fill="none" stroke="#5bbf87"/>
    <text x="420" y="104" fill="#d7dae0">ri 1 · Line</text>
    <rect x="410" y="148" width="180" height="24" fill="none" stroke="#5bbf87"/>
    <text x="420" y="164" fill="#d7dae0">ri 2 · Point</text>
  </g>
  <g stroke="#5bbf87" stroke-width="1.2">
    <line x1="210" y1="40" x2="406" y2="40" marker-end="url(#ah34bc)"/>
    <line x1="210" y1="100" x2="406" y2="100" marker-end="url(#ah34bc)"/>
    <line x1="210" y1="160" x2="406" y2="160" marker-end="url(#ah34bc)"/>
  </g>
  <text x="310" y="194" fill="#888" text-anchor="middle">ri = objects_base.len(), advanced only when a row is pushed — skipped variants leave no hole</text>
  <defs>
    <marker id="ah34bc" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
      <path d="M0,0 L6,3 L0,6 Z" fill="#5bbf87"/>
    </marker>
  </defs>
</svg>

`segment_count`/`glyph_count` feed the exact same fields 31/32 already added to `Gpu` — only their
*source* changed. Everything from `instance_buffer`/`segment_buffer`/`glyph_buffer` creation downward
is untouched.

> **Precision caveat (33).** Rebasing the instance *model* keeps meshes (local vertices + an xform
> placement) solid at any distance. But a Line/Polyline/Point writes its coordinates straight into the
> segment/glyph buffers, already f32 — a drawing authored millions of units from the origin loses
> precision at build time. These fixtures sit near the origin; the real fix (subtract the origin in
> f64 *before* filling those buffers) is a later concern — flagged, not silently skipped.
>
> The placement half is already handled: Line/Polyline/Point push their own `xform` as the instance
> model (same as Mesh/BRep), so placed linework renders correctly *and* gets the f64 rebase through
> `objects_base`. Only the baked-in coordinate precision above remains a later concern.

**2c. Add `push_mesh`** near `unit_cylinder`/`unit_sphere` at the bottom of the file — 31's and 32a's
per-object loop bodies, factored so `Mesh` and `BRep` (which becomes a `Mesh` via `.mesh()`) share it.
The glyph loop stays 32a's `vertices()` — **every vertex gets a dot, always** (viewer policy: display
quality never degrades; a naked-only variant was considered and rejected). If sphere glyphs ever get
heavy at this density, the lever is a cheaper primitive — 32b's 3-vertex billboards draw the same
dots for ~2% of the triangles — never hiding them:

```rust
fn push_mesh(m: &Mesh, ri: u32, verts: &mut Vec<RenderVertex>, vids: &mut Vec<u32>,
             idx: &mut Vec<u32>, segments: &mut Vec<CylinderSegment>,
             glyphs: &mut Vec<GlyphPoint>) {
    let base = verts.len() as u32;
    let rm = m.to_render();
    for v in &rm.vertices { verts.push(*v); vids.push(ri); }
    for &i in &rm.indices { idx.push(base + i); }

    for (a, b, col) in m.edges_with_colors() {
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();
        segments.push(CylinderSegment { p0: pa.to_f32(), radius: 0.0, p1: pb.to_f32(),
            instance_id: ri, color: col.to_f32() });
    }
    for vk in m.vertices() {
        let p = m.vertex_point(vk).unwrap();
        glyphs.push(GlyphPoint { center: p.to_f32(), radius: 0.0, color: [0.1, 0.1, 0.1, 1.0],
            instance_id: ri, _pad: [0; 3] });
    }
}
```

## Step 3 — scene bounds, so `F` fits real data: `gpu/mod.rs` + `src/lib.rs`

`F` has fit a hardcoded `SCENE_MIN`/`SCENE_MAX` since lesson 15 — sized for the demo. A loaded file
needs the real extent.

**3a. In `gpu/mod.rs`, right after the empty-buffer guards** (Step 2b), fold the min/max of every
vertex, segment endpoint, and glyph centre — one cheap pass, no BVH (that's 40's job; this is just a
camera target):

```rust
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
```

Store them on `Gpu` — add `pub scene_min: [f32; 3], pub scene_max: [f32; 3],` to the struct (next to
`arena_index_count`) and to the `Ok(Self { … })` initializer.

**3b. In `src/lib.rs`, drop the hardcoded constants** (`const SCENE_MIN`/`SCENE_MAX` near the top)
and **point the `F` handler at the real bounds**:

```rust
                        Key::Character("f" | "F") => {
                            let aspect = state.gpu.config.width as f64 /
                                state.gpu.config.height as f64;
                            state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                        }
```

`Camera::fit(min, max, aspect)` is unchanged (lesson 15) — only where the box comes from changes.

## Step 4 — run: the first real fixture

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

`floor_model.pb` (a compas_tf timber floor, 3.0 MB) is 491 objects — verified by loading it through
`session_rust` directly: **201 Mesh + 290 Polyline**, nothing else. One file, both adapter paths:
meshes tessellate into the arena (5,650 verts) with edges as segments (15,095) and one glyph per
vertex (~5,650); polylines add 1,800 more segments. Console:

```
session 'floor_model': 491 objects, 5650 arena verts, 16895 segments, 5650 glyphs
perf: 60.0 fps | 16.67 ms | 5 draws | 491 objects
```

> **5 draws, not 6 — and guard the data-driven passes.** With the bunny gone the cloud table is
> empty, and a zero-count draw earns a Dawn "vertex/instance count of 0 is unusual" console warning.
> In `clear()`, wrap each pass whose count comes from FILE DATA in a skip guard:
>
> ```rust
> if self.segment_count > 0 { /* the whole Edges block, draws += 1 inside */ }
> if self.glyph_count > 0 { /* the Spheres block */ }
> if self.point_count > 0 { /* the Points block */ }
> ```
>
> floor_model exercises segments+glyphs (→ 5 draws; the sixth returns when PointCloud is wired), but
> a pure line drawing has zero glyphs and a pure cloud file zero segments — the guards cover every
> shape of file. The other three passes need none: background/grid are the viewport itself (fixed
> counts — a guard would hide the grid), and the arena can't reach zero (the padding guard pushed
> `[0,0,0]` into `idx`, so it draws one degenerate zero-area triangle — nonzero count, no warning).
> Rule: guard counts that come from data; fixed and padded counts can't go to zero.

Press `F` — the whole floor fits. Draw count: constant no matter the file — 5 passes (background,
grid, arena, edges, glyphs), never per-object.

## Step 5 — the stress gate: a real PDF, converted to curves

Swap `DEMO_SESSION_URL` to `"session_data/30700_querschnitt_gg.pb"` — a real technical drawing
(`30700 Querschnitt G-G.pdf`) converted by `session_data/pdf_to_session.py`: **40,814 Line + 1,418
Polyline**, 17.5 MB, no meshes at all.

```
session 'my_session': 42232 objects, 1 arena verts, 51166 segments, 1 glyphs
perf: __._ fps | __.__ ms | 4 draws | 42232 objects   (no glyphs → the sphere pass skips itself)
```

(`1 arena verts` / `1 glyphs` are Step 2's placeholders — legitimately empty categories.) **51,166
cylinder segments, ONE `draw_indexed`** — lesson 31's whole point proven at scale. Record the fps,
press `F`, orbit: if it isn't interactive, frustum culling (41) is the designed next lever.

> **Coverage honesty.** Between them the two fixtures exercise Mesh, Polyline and Line — but no
> fixture contains a `BRep` or a standalone `Point`, so those two branches (and `point_to_glyph`)
> compile here but run **untested**. The first BRep-bearing file will exercise them; until then treat
> those paths as written-to-spec, not verified.

## Recap

```
Ch 34a: bytes → Session, proven by a console count.
Ch 34b: SESSION → TABLES. gpu.rs becomes gpu/mod.rs + gpu/adapters.rs (Line/Polyline/Point →
        CylinderSegment/GlyphPoint). The lesson-30 loop becomes a match over Geometry's 11 variants:
        Mesh/BRep share push_mesh (arena verts + edge segments + every-vertex glyphs); placement =
        EVERY object's own xform (kernel convention — brep.xform, NOT the built mesh's:
        from_polylines returns identity); ri = objects_base
        row, not lookup index, so skipped variants leave no hole. Empty-buffer guards pad what a
        real file legitimately lacks; *_count captured before padding keeps empty categories
        drawing nothing. F fits real scene bounds. Verified: floor_model.pb (491 objects, both
        adapter paths) and the STRESS GATE (42,232 objects, 51,166 segments, ONE draw_indexed).
```

Edited: `src/engine/gpu.rs` → `gpu/mod.rs` (session-driven build loop, `push_mesh`, guards, scene
bounds), `gpu/adapters.rs` (NEW), `src/lib.rs` (`F` reads real bounds), `src/state.rs` (passes
`&session` into `Gpu::new`).

## Next

`34c-anchor-rebase.md` — the first real file at scale exposes lesson 33's per-frame rebase as a
300ms CPU wall; the floating anchor fixes it. The 34c–34h arc (anchor → proper reader →
multi-file grid → flat linework → camera UX → colors/widths) hardens the viewer against half a
million objects before `35-scene-struct.md` moves the walk into the app layer.
