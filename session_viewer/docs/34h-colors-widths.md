# 34h Colors & widths — honor what the user set

> **Big picture.** The kernel lets a user color a mesh four ways (`objectcolor`, per-vertex
> `pointcolors`, per-face `facecolors`, per-edge `linecolors`) and give every curve/point a `width` —
> but the viewer ignores most of it: `triangle.wgsl` throws away the per-vertex color `to_render()`
> carefully bakes, a FACECOLORS mesh renders white (the kernel's own `pipe` primitive ships that way
> — a live bug), and no `width` field is ever read. This lesson installs ONE resolution rule and one
> width encoding, with defaults bit-identical to today's pixels.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="kernel color channels resolve CPU-side into per-row colors, multiplied by a per-object tint in every shader" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g stroke="#6fb3ff" stroke-width="1.5" fill="none">
    <rect x="10"  y="20" width="200" height="90"/>
    <rect x="260" y="20" width="180" height="42"/>
    <rect x="260" y="72" width="180" height="38"/>
    <rect x="490" y="40" width="180" height="50"/>
  </g>
  <g fill="#d7dae0">
    <text x="20" y="38">objectcolor · pointcolors</text>
    <text x="20" y="54">facecolors · linecolors</text>
    <text x="20" y="70">width(s) — per class</text>
    <text x="20" y="98" fill="#888">color_mode = user-set signal</text>
    <text x="270" y="38">resolve CPU-side →</text>
    <text x="270" y="54">row.color / row.radius</text>
    <text x="270" y="92" fill="#888">instance = white tint</text>
    <text x="500" y="60">shader: row × tint</text>
    <text x="500" y="80" fill="#888">selection reuses tint (50)</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.5">
    <line x1="210" y1="55" x2="256" y2="47" marker-end="url(#ah34c)"/>
    <line x1="440" y1="47" x2="486" y2="60" marker-end="url(#ah34c)"/>
    <line x1="440" y1="91" x2="486" y2="75" marker-end="url(#ah34c)"/>
  </g>
  <defs>
    <marker id="ah34c" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
      <path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/>
    </marker>
  </defs>
</svg>

## The two rules

**Colors resolve on the CPU at table-build time.** Every draw path already has a per-row color slot
(`RenderVertex.color`, `CylinderSegment.color`, `GlyphPoint.color`, `CloudPoint.color`) — the real
color lives there. `Instance.color` becomes a pure **tint**, white by default, and every shader ends
with the same line: `final = row_color × instances[id].color`. Selection (lesson 70) will recolor an
object by writing its tint — no row re-upload.

Precedence, first satisfied rule wins (`color_mode` is THE "user set it" signal — the Mesh vecs are
auto-seeded white/black on `add_vertex`/`add_face`, so a non-empty vec means nothing):

| primitive → row | user-set | default |
|---|---|---|
| mesh faces → `RenderVertex.color` | FACECOLORS+coverage → per-face flat; POINTCOLORS+coverage → per-vertex | `objectcolor` |
| mesh edges → `CylinderSegment.color` | `linecolors[i]` (already works) | black |
| mesh vertex dots → `GlyphPoint.color` | POINTCOLORS+coverage → `pointcolors[i]` | `[0.1,0.1,0.1,1]` |
| line/polyline → `CylinderSegment.color` | `linecolor` (already works) | black |
| point → `GlyphPoint.color` | `pointcolor` (already works) | black (34d changed the kernel default from blue, ×3 languages) |
| brep → mesh lanes | `surfacecolor` as the built mesh's objectcolor | black |

**Width is a multiplier**, encoded in the `radius` field every segment/glyph already carries — zero
layout churn:

```
radius == 0.0 → px = LineUniform.thickness            (global default — unchanged)
radius <  0.0 → px = LineUniform.thickness * (-radius) (kernel width lands here)
radius >  0.0 → world-units radius                     (unchanged — 34f's paper-space lane)
```

`width == 1.0` (every kernel default) encodes as `0.0` (34f's `encode_width` already does this), so
untouched files stay bit-identical. The future thickness slider (60) writes `LineUniform.thickness`
and every user width scales with it, Rhino-style.

> **Superseded in Part 3 (2026-08-11):** the `1.0 → 0.0` special case turned out to be lossy — PDF
> widths became absolute mm, and a real 0.35 mm pen (the old multiplier 1.0) silently collapsed to
> "unset". `encode_width` now encodes every `w > 0` as `-(w)`; safe because all four flat shaders
> compute `mult = select(1.0, -radius, radius < 0.0)`, so `0.0` and `-1.0` render identically.

## Files we touch

```
session_rust/src/render_mesh.rs   # Step 1: to_render() FACECOLORS branch (Rust-only, no py/cpp port)
src/engine/gpu/adapters.rs        # Step 2: point widths enter the encoding
src/engine/gpu/mod.rs             # Step 3: imports; walk pushes WHITE TINT + BRep bake; push_mesh
src/shaders/triangle.wgsl         # Step 4a: resurrect the baked vertex color
src/shaders/ribbon.wgsl           # Step 4b: negative-radius decode + tint multiply
src/shaders/glyph.wgsl            # Step 4c: same
src/shaders/cylinder.wgsl         # Step 4d: same (SOLID parity)
src/shaders/sphere.wgsl           # Step 4d: same
src/shaders/point.wgsl            # Step 4e: tint multiply

session_rust/examples/colors_widths.rs   # Verify V1: the fixture (NEW file) — V2 runs it
session_viewer/index.html         # Verify V3: copy-file link for the fixture .pb
src/state.rs                      # Verify V4: DEMO_SESSION_URLS → the fixture (temporary)
```

Part 2 (below) takes the same rules to nine real drawings and touches these as well:

```
src/shaders/ribbon.wgsl · glyph.wgsl     # Step 5: HAIRLINE_MIN_ALPHA floor
src/engine/pipelines/build.rs · mod.rs   # Step 6: MSAA per scene, `samples` threaded through
src/engine/gpu/mod.rs                    # Steps 6,7,9,10: msaa_for, INK_DEPTH_PREPASS, curve arm, hidden wireframe
session_rust/src/line.rs                 # Step 8a: xform was dropped by pb_dumps/pb_loads
src/camera.rs · src/state.rs             # Step 8b: anchor units (metres vs mm) — zoom-in clipping
src/engine/gpu/adapters.rs               # Step 9: nurbscurve_to_segments
session_data/pdf_to_session.py           # Step 11: PDF → paths JSON (extraction only)
session_rust/examples/pdf_build.rs       # Step 11: JSON → .pb — CDT, groups, protobuf (NEW file)
session_data/import_drawings.sh          # Step 11: both stages for all nine sheets (NEW file)
session_rust/examples/combined_scene.rs  # Verify: grid-place the nine sheets into one .pb (NEW file)
session_rust/examples/probe_scene.rs     # Verify: what a .pb really holds (NEW file)
```

## Step 1 — FACECOLORS in `to_render`: `session_rust/src/render_mesh.rs`

(The KERNEL crate, not the viewer — but Rust-only: `session_py`/`session_cpp` have no `to_render`,
so no 3-language port.) `to_render` shares vertices between faces, but a shared vertex can only
carry ONE color — flat per-face color needs its own vertices, three per triangle.

**Find the `has_point_colors` gate and the `let mut vertices` line that follows it:**

```rust
        let has_point_colors =
            self.color_mode == crate::mesh::ColorMode::POINTCOLORS && point_colors.len() == keys.len();

        let mut vertices: Vec<RenderVertex> = Vec::with_capacity(keys.len());
```

**Insert the FACECOLORS early-return between them** (after the gate, before `let mut vertices`):

```rust
        // FACECOLORS: flat per-face color needs duplicated vertices — a shared vertex can only
        // carry one color. Same gate style as pointcolors: the MODE is the user-set signal.
        // (facecolors is private to mesh.rs; from this sibling module use get_facecolors().)
        let has_face_colors = self.color_mode == crate::mesh::ColorMode::FACECOLORS
            && self.get_facecolors().len() == self.face.len();
        if has_face_colors {
            let face_colors = self.get_facecolors();
            let mut vertices: Vec<RenderVertex> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();
            let mut face_keys: Vec<usize> = self.face.keys().copied().collect();
            face_keys.sort_unstable();
            for (fi, fk) in face_keys.iter().enumerate() {
                let c = &face_colors[fi];
                let color = [c.r, c.g, c.b, 1.0];
                let mut tris: Vec<[usize; 3]> = Vec::new();
                if let Some(cached) = self.triangulation.get(fk) {
                    tris.extend_from_slice(cached);
                }
                if tris.is_empty() {
                    let vs = &self.face[fk];
                    if vs.len() < 3 { continue; }
                    for i in 1..(vs.len() - 1) { tris.push([vs[0], vs[i], vs[i + 1]]); }
                }
                for tri in &tris {
                    if tri.iter().any(|vk| !self.vertex.contains_key(vk)) { continue; }
                    for &vk in tri {
                        let v = &self.vertex[&vk];
                        let nx = v.attributes.get("nx").copied().unwrap_or(0.0);
                        let ny = v.attributes.get("ny").copied().unwrap_or(0.0);
                        let nz = v.attributes.get("nz").copied().unwrap_or(0.0);
                        indices.push(vertices.len() as u32);
                        vertices.push(RenderVertex {
                            position: [v.x as f32, v.y as f32, v.z as f32],
                            normal: [nx as f32, ny as f32, nz as f32],
                            color,
                        });
                    }
                }
            }
            return RenderMesh { vertices, indices };
        }
```

The shared-vertex path below stays untouched for OBJECTCOLOR/POINTCOLORS. Memory grows only for
meshes actually in FACECOLORS mode. Check it: `cargo check --lib` in `session_rust/`.

## Step 2 — point widths: `src/engine/gpu/adapters.rs`

34f already landed `encode_width` and wired `line_to_segment`/`polyline_to_segments`. One piece
remains — points. In `point_to_glyph`, find:

```rust
        center: p.to_f32(),
        radius: 0.0,
```

Replace the radius line with:

```rust
        radius: encode_width(p.width),
```

## Step 3 — the walk pushes a TINT: `src/engine/gpu/mod.rs`

**3a. Imports.** At the top of the file, find:

```rust
use adapters::{line_to_segment, point_to_glyph, polyline_to_segments};
use bytemuck::Zeroable;
use session_rust::{Mesh, Xform, RenderVertex, Point, Geometry};
```

Replace with (`encode_width` joins the list; `ColorMode` is NOT re-exported at the crate root —
it lives in the `mesh` module):

```rust
use adapters::{line_to_segment, point_to_glyph, polyline_to_segments, encode_width};
use bytemuck::Zeroable;
use session_rust::{Mesh, Xform, RenderVertex, Point, Geometry};
use session_rust::mesh::ColorMode;
```

**3b. Every `t.objects.push((xform, color))` becomes white tint** — the row colors now carry the
real color, the instance slot is the modulation channel. In `walk_session`'s match, find the five
renderable arms:

```rust
                Geometry::Mesh(m) => {
                    t.objects.push((m.xform.clone(), m.objectcolor().to_f32()));
                    push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.segments, &mut t.glyphs);
                }
                Geometry::BRep(b) => {
                    let bm = b.mesh();
                    t.objects.push((b.xform.clone(), b.surfacecolor.to_f32()));
                    push_mesh(&bm, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.segments, &mut t.glyphs);
                }
                Geometry::Line(l) => {
                    t.objects.push((l.xform.clone(), l.linecolor.to_f32()));
                    t.segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    t.objects.push((pl.xform.clone(), pl.linecolor.to_f32()));
                    t.segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    t.objects.push((p.xform.clone(), p.pointcolor.to_f32()));
                    t.glyphs.push(point_to_glyph(p, ri));
                }
```

Replace all five with (BRep additionally BAKES its surfacecolor into the built mesh, so
`to_render` carries it into the vertex rows):

```rust
                Geometry::Mesh(m) => {
                    t.objects.push((m.xform.clone(), [1.0; 4]));   // was m.objectcolor() — to_render bakes it
                    push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.segments, &mut t.glyphs);
                }
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());    // bake surfacecolor into the built mesh
                    t.objects.push((b.xform.clone(), [1.0; 4]));
                    push_mesh(&bm, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.segments, &mut t.glyphs);
                }
                Geometry::Line(l) => {
                    t.objects.push((l.xform.clone(), [1.0; 4]));   // linecolor already rides the segment row
                    t.segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    t.objects.push((pl.xform.clone(), [1.0; 4]));
                    t.segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    t.objects.push((p.xform.clone(), [1.0; 4]));
                    t.glyphs.push(point_to_glyph(p, ri));
                }
```

(`rebuild_instances` is unchanged — it now streams the tint.)

**3c. `push_mesh` — per-edge widths.** In `push_mesh` (bottom of `gpu/mod.rs`), find the edge
loop:

```rust
    for (a, b, col) in m.edges_with_colors(){
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();
        segments.push(
            CylinderSegment{
                p0: pa.to_f32(),
                radius: 0.0,
                p1: pb.to_f32(),
                instance_id: ri,
                color: col.to_f32()
            }
        )
    }
```

Replace with an indexed loop (`m.widths()` is seeded in the same discovery order
`edges_with_colors()` walks — kernel-guaranteed alignment):

```rust
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
```

**3d. `push_mesh` — honest dots.** Directly below, find the dot loop:

```rust
    for vk in m.vertices(){
        let p = m.vertex_point(vk).unwrap();
        glyphs.push(
            GlyphPoint { 
                center: p.to_f32(), 
                radius: 0.0, 
                color: [0.1, 0.1, 0.1, 1.0], 
                instance_id: ri, 
                _pad: [0;3] }
        );
    }
```

Replace with:

```rust
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
```

## Step 4 — six shaders, one rule each (both linework modes stay honest)

**4a. `src/shaders/triangle.wgsl`** — in `vs_main`, find:

```wgsl
    o.color = inst.color.rgb; // Set the color
```

Replace — this single line resurrects everything `to_render` bakes (objectcolor, pointcolors, and
now facecolors):

```wgsl
    o.color = in.color.rgb * inst.color.rgb; // baked base color × instance tint (white today)
```

**4b. `src/shaders/ribbon.wgsl`** (default edges, 34f) — two edits. Find:

```wgsl
    var px = line.thickness;
    if (seg.radius > 0.0) {
```

Give it the px-multiplier lane (negative radii — 3D files' widths; planar sheets already
converted theirs to the world lane in `walk_session`):

```wgsl
    let mult = select(1.0, -seg.radius, seg.radius < 0.0);
    var px = line.thickness * mult;
    if (seg.radius > 0.0) {
```

Then find `o.color = seg.color;` and replace with:

```wgsl
    o.color = seg.color * instances[seg.instance_id].color;
```

**4c. `src/shaders/glyph.wgsl`** (default dots, 34f) — the same two changes. Find:

```wgsl
    var px = line.thickness;
    if (g.radius > 0.0) {
```

Replace with:

```wgsl
    let mult = select(1.0, -g.radius, g.radius < 0.0);
    var px = line.thickness * mult;
    if (g.radius > 0.0) {
```

Then `o.color = g.color;` → `o.color = g.color * instances[g.instance_id].color;`

**4d. SOLID parity — `cylinder.wgsl` + `sphere.wgsl`** (so `LINEWORK_SOLID = true` honors the
same widths/tints). In `cylinder.wgsl`, find (note: no spaces around the `>`):

```wgsl
    let r = select(screen_radius(clip_c.w, line), seg.radius, seg.radius>0.0);
```

Replace with:

```wgsl
    let mult = select(1.0, -seg.radius, seg.radius < 0.0);
    let r = select(screen_radius(clip_c.w, line) * mult, seg.radius, seg.radius>0.0);
```

Then `o.color = seg.color;` → `o.color = seg.color * instances[seg.instance_id].color;`

In `sphere.wgsl`, find:

```wgsl
    let base = screen_radius(clip_c.w, line) * 1.0; // sphere inflation radius
```

Replace with:

```wgsl
    let mult = select(1.0, -g.radius, g.radius < 0.0);
    let base = screen_radius(clip_c.w, line) * mult; // sphere inflation radius
```

(the `let r = select(base, g.radius, g.radius > 0.0);` line below it stays), then
`o.color = g.color;` → `o.color = g.color * instances[g.instance_id].color;`

**4e. `src/shaders/point.wgsl`** — find `o.color = p.color;` and replace with:

```wgsl
    o.color = p.color * instances[p.instance_id].color;
```

(tint wiring only; per-cloud `point_size` waits for the PointCloud lesson).

## Verify

`cargo check` in both `session_rust` (native) and `session_viewer` (wasm) — both must be
warning-free. `encode_width` and `ColorMode` are unused-import warnings until Step 3c/3d land, so a
clean build is itself the check that Step 3 is complete.

Then two gates. The first is free; the second needs a fixture, built in five ordered steps below.

**Gate 1 — regression, nothing moved.** `floor_model.pb` and the stress wall must look
pixel-identical: every default is white-tint × row-color and `radius 0.0`.

**Gate 2 — positive, user colors/widths appear.** Four files, in this order (V1 → V5). The `.pb`
must exist on disk *before* `trunk serve` runs — V3's `copy-file` link is resolved at build time,
so skipping V2 fails the whole build with:

```
error getting canonical path for ".../session_data/colors_widths.pb"
No such file or directory (os error 2)
```

### V1 — the fixture: create `session_rust/examples/colors_widths.rs`

A NEW file (no anchor — nothing exists yet), with exactly this content. Every line matters: the
three `s.add_*` calls and the final `s.pb_dump` are what actually writes the file — a fixture that
builds the geometry but never adds/dumps it produces nothing.

```rust
use session_rust::{Session, Mesh, Polyline, Point, Color, Xform};

fn main() {
    let mut s = Session::new("colors_widths");
    let palette = Color::palette();                                // 12 spectral colors

    let mut m1 = Mesh::create_box(400.0, 400.0, 400.0);            // FACECOLORS — 6 faces
    m1.set_facecolors((0..6).map(|i| palette[i * 2].clone()).collect());
    m1.xform = Xform::translation(-600.0, 0.0, 0.0);               // placement = instance model (34b)

    let mut m2 = Mesh::create_box(400.0, 400.0, 400.0);            // POINTCOLORS gradient — 8 verts
    let n = m2.number_of_vertices();
    m2.set_pointcolors((0..n).map(|i| Color::new(i as f32 / n as f32, 0.2, 1.0 - i as f32 / n as f32, 1.0)).collect());
    // m2 stays at the origin

    let mut m3 = Mesh::create_box(400.0, 400.0, 400.0);            // control — unchanged look
    m3.xform = Xform::translation(600.0, 0.0, 0.0);

    let mut pl = Polyline::new(vec![
        Point::new(-600.0, -600.0, 0.0),
        Point::new(600.0, -600.0, 0.0),
        Point::new(600.0, 600.0, 200.0),
    ]);
    pl.linecolor = Color::red();
    pl.width = 5.0;                                                // 5× the global thickness

    let mut p = Point::new(0.0, -800.0, 0.0);
    p.width = 4.0;                                                 // fat dot, 4× the global px

    s.add_mesh(m1, None);
    s.add_mesh(m2, None);
    s.add_mesh(m3, None);
    s.add_polyline(pl, None);
    s.add_point(p, None);
    s.pb_dump("../session_data/colors_widths.pb");
}
```

(`Mesh::create_box`, not `Mesh.create_box` — `::` is Rust's path separator. And `.collect()` closes
the *iterator*, outside the `map`: `(0..6).map(|i| palette[i * 2].clone()).collect()`.)

### V2 — RUN it (this is what creates the `.pb`)

From `session_rust/`, not from the viewer:

```bash
cd session_rust
cargo run --example colors_widths
ls ../session_data/colors_widths.pb     # must exist before V3/V5
```

It prints nothing. The path in `pb_dump` is relative to the crate you run from — that is why it
reads `../session_data/`.

### V3 — publish the file: `session_viewer/index.html`

Find the LAST `copy-file` link (the block of `../session_data/draw_*.pb` lines) and add one more
directly below it:

```html
   <link data-trunk rel="copy-file" href="../session_data/colors_widths.pb" data-target-path="session_data"/>
```

### V4 — load only it: `session_viewer/src/state.rs`

At the top of the file, find `const DEMO_SESSION_URLS` (34e) and TEMPORARILY replace its whole list
with the one fixture:

```rust
const DEMO_SESSION_URLS: &[&str] = &["session_data/colors_widths.pb"];
```

### V5 — look at it

`trunk serve` in `session_viewer/`, then check, left to right: box 1 shows six distinct flat face
colors (not white — the FACECOLORS bug is dead), box 2 shows the vertex gradient AND
gradient-colored dots, box 3 is indistinguishable from before, the polyline is red at 5× thickness,
the point is a fat black dot (4× — the width lane on a glyph).

Then **restore** the V4 list to what it was (V3's link can stay — it costs one small file).

---

# Part 2 — the same rules against a REAL drawing

The fixture proves the plumbing. Nine architectural PDFs prove whether it survives contact with
production data — half a million objects, plot pens measured in hundredths of a millimetre, and
colour that a viewer can lose in four different places. Everything below came out of pointing the
viewer at those sheets and fixing what was wrong. Four of the fixes are in code you already typed.

> **Big picture.** A drawing is not a scene with a few coloured boxes. It is 600k hairlines whose
> colour reads as *white paper* unless the fade has a floor, ink whose front-to-back order is
> decided by a HashMap unless something writes depth, and geometry that vanishes as you zoom in
> unless camera-relative rendering actually runs. Also: text, fills and CAD layers, none of which
> the importer used to carry.

## Step 5 — the hairline floor: `src/shaders/ribbon.wgsl`, `src/shaders/glyph.wgsl`

34f's hairline rule says a sub-pixel pen renders 1 px wide with *proportional opacity*, so apparent
weight stays continuous under zoom. That is right for one line and catastrophic for a sheet.

Do the arithmetic on real data. Those PDFs plot with 0.09, 0.14, 0.28 and 0.43 mm pens. Fit a
2400 mm wide sheet into a 1265 px canvas and one drawing unit is about half a pixel, so the *widest*
common pen is 0.14 px — `fade = 0.14 / 0.5 = 0.28`. Every line on the sheet draws at under 30%
alpha over a 0.9 grey background. The ink is black and dark red; what you see is pale grey. The
colour was never lost — it was faded away.

In **`ribbon.wgsl`**, find the `LineUniform` struct's closing brace and the `struct VsOut` that
follows it. Insert between them:

```wgsl
// Sub-pixel pens never fade below this: 0 = original continuous fade, 1 = always solid 1px.
const HAIRLINE_MIN_ALPHA = 0.5;
```

Then in `vs_main`, find:

```wgsl
    var fade = 1.0;
    if (px < 0.5) {
        fade = px / 0.5;
        px = 0.5;
    }
```

and change ONE line — `fade = px / 0.5;` becomes:

```wgsl
        fade = max(px / 0.5, HAIRLINE_MIN_ALPHA);
```

Do exactly the same twice in **`glyph.wgsl`**: the same `const` above its `struct VsOut`, and the
same `max(...)` inside its `if (px < 0.5)` block. A dot and the line it terminates must agree about
weight, or the same width reads as two different weights.

CAD's own answer to a sub-pixel pen is a solid 1 px hairline — `HAIRLINE_MIN_ALPHA = 1.0` is that,
exactly, at the cost of the thin/thick hierarchy that makes an architectural sheet readable. 0.5
keeps both: colour legible, hierarchy intact.

## Step 6 — MSAA is a property of the PASS: `src/engine/pipelines/build.rs`, `mod.rs`, `gpu/mod.rs`

The obvious question, once a sheet is on screen: can linework run at 1× (it antialiases itself in
the shader) while meshes run at 4×? No. Sample count belongs to the render *pass* — every pipeline
drawn into it must agree, and the depth attachment too. Mixing means two passes plus a manual depth
resolve, because a multisampled depth buffer cannot be read by a 1-sample pass.

So choose per SCENE. In `build.rs`, delete the constant:

```rust
pub const MSAA_SAMPLES: u32 = 1;
```

and give **every** `build_*_pipeline` function a `samples: u32` parameter (right after
`device: &wgpu::Device`), replacing each `count: MSAA_SAMPLES` with `count: samples`. Ten builders,
one mechanical edit each. `Pipelines::new` takes `samples: u32` too and forwards it.

In `gpu/mod.rs`, `create_depth_view` and `create_msaa_view` take `samples` as well (`sample_count:
samples`), `Gpu` stores `pub samples: u32` so `resize` can rebuild both targets at the right count,
and the rule itself goes next to them:

```rust
    /// MSAA sample count for a scene. It cannot be chosen per lane: sample count belongs to the
    /// render PASS, and every pipeline drawn into a pass must match it, so 1x linework and 4x
    /// solids in one frame would need two passes and a depth resolve between them. Pick per scene
    /// instead - hard-edged geometry (triangles, tubes, spheres) is the only thing MSAA smooths,
    /// while ribbons and dots antialias themselves in the shader. A 2D sheet therefore pays
    /// nothing, and a model with meshes gets clean silhouettes.
    fn msaa_for(files: &[SceneTables]) -> u32 {
        let solid = files.iter().any(|f| !f.verts.is_empty() || !f.pipes.is_empty() || !f.spheres.is_empty());
        if solid { 4 } else { 1 }
    }
```

One more consequence, in `clear`'s render pass: a 1-sample attachment must NOT carry a resolve
target. Find the `color_attachments` line and make both fields conditional:

```rust
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: if self.samples > 1 { &self.msaa_view } else { &view },
                    resolve_target: if self.samples > 1 { Some(&view) } else { None },
```

Measured on the nine sheets: **9 fps → 50 fps**. That is the whole edit.

## Step 7 — flat ink writes no depth: `src/engine/gpu/mod.rs`

Both flat pipelines set `depth_write_enabled: false` — deliberately, because a blended AA feather
that wrote depth would block later ink at the same depth and leave halos at every line crossing.
The consequence is that ink-vs-ink order is decided by *draw order alone*, and draw order is
`session.lookup.values()` — a HashMap walk. A dot sits on top of the polyline it belongs to at every
camera angle, and a polyline hides behind a drawing that is metres further away.

The fix is a depth-only prepass: the same geometry, a binary coverage test at half alpha, colour
masked off, depth written. `ribbon.wgsl` and `glyph.wgsl` each grow an `fs_depth` entry point, and
`build_ink_depth_pipeline` builds both (note the colour target with `write_mask:
wgpu::ColorWrites::empty()` — Dawn rejects an empty target list against a colour pass), while the
colour pipelines switch to `depth_compare: GreaterEqual` so each survives its own prepass.

It costs a second full pass over every ribbon and dot, so it is a flag, at the top of `gpu/mod.rs`:

```rust
/// Depth prepass for the FLAT lane, so flat ink occludes flat ink (a dot behind a polyline
/// loses to it) instead of pure draw order deciding - and draw order here is HashMap order,
/// so without it "who is in front" is effectively random. Costs a SECOND full pass over every
/// ribbon/dot; set false to trade correct ink ordering for that frame time back.
const INK_DEPTH_PREPASS: bool = true;
```

> **Default flipped in Part 3:** on the ten-sheet scene the second pass doubles the frame for ink
> that is all coplanar anyway — the flag ships `false` now; set it back for 3D-heavy scenes.

guarding both prepass draws:

```rust
            if INK_DEPTH_PREPASS && self.segment_count > self.pipe_count {
```

Coplanar ink — nearly all of a drawing sheet — is unaffected either way: same depth, so
`GreaterEqual` lets every line paint, and order stays painter's order where that is the correct
answer.

## Step 8 — two bugs that only a real file exposes

**8a. `Line::pb_dumps` threw away the xform.** `session_rust/src/line.rs` wrote `xform: None` into
the proto and never read one back, even though `line.proto` has the field and both C++ and Python
serialize it. Lines are ~90% of a drawing (40,814 of 43,844 objects in one sheet), so laying nine
sheets out in a grid moved the polylines and left every line stacked at the origin. In `pb_dumps`,
find the object-level `xform: None,` (the one after `name:` — NOT the two inside the `start`/`end`
points, which are plain coordinates) and replace it with:

```rust
            xform: Some(crate::proto::Xform {
                guid: self.xform.guid().to_string(),
                name: self.xform.name.clone(),
                matrix: self.xform.m.iter().map(|&v| v as f64).collect(),
            }),
```

and in `pb_loads`, after the `if let Some(color) = proto.linecolor { ... }` block, add the mirror:

```rust
        if let Some(xform) = proto.xform {
            line.xform.set_guid(xform.guid);
            line.xform.name = xform.name;
            for (i, val) in xform.matrix.iter().enumerate() {
                if i < 16 {
                    line.xform.m[i] = *val as f64;
                }
            }
        }
```

The lesson underneath: C++ is ground truth and Rust had drifted from it silently, because no test
round-trips a *non-identity* xform. A `to_proto/from_proto` test that only ever sees an identity
matrix cannot tell the difference between "serialized" and "reconstructed by the constructor".

**8b. Camera-relative rendering had never actually run.** The camera keeps `target`/`distance` in
**metres** (`fit` multiplies by `unit.to_meters()`), while the instance table's translations are in
world **millimetres**. `rebase_anchor` was handed metres and subtracted them straight from
millimetres — a 1000× mismatch — and its threshold (`REANCHOR_DIST = 1e5`, documented as mm) was
compared against a metre-scale drift, i.e. a 100 km trigger. The anchor therefore stayed at
`(0,0,0)` forever and every vertex kept its full world magnitude.

You see it as geometry that *disappears* when you zoom in. At a 1 mm view the near/far planes are
~10 µm and ~10 mm apart while the f32 mvp differences numbers around 47,000: roughly 4 mm of error
in view-space z, so the computed depth falls outside the frustum and is clipped.

In `camera.rs`, `origin()` returns the target in WORLD units and `view_proj_anchored` converts the
anchor back to metres itself — one unit for the anchor everywhere, converted where the scale
already lives:

```rust
    pub fn origin(&self) -> Point{
        let s = self.unit.to_meters();
        Point::new(self.target[0] / s, self.target[1] / s, self.target[2] / s)
    }

    pub fn distance_world(&self) -> f64 {
        self.distance / self.unit.to_meters()
    }
```

```rust
        let a = self.unit.to_meters();
        let anchor = Point::new(anchor[0] * a, anchor[1] * a, anchor[2] * a); // world -> metres
```

and the threshold becomes a quarter of the view distance, clamped — zoomed out, panning must not
trigger constant 52 MB rebuilds; zoomed in, it must re-anchor before coordinates grow back:

```rust
        let thresh = (view_dist * 0.25).clamp(REANCHOR_MIN, REANCHOR_MAX);
```

(`state.rs` passes it: `self.gpu.rebase_anchor(&origin, self.camera.distance_world())`.) While you
are in `camera.rs`, delete the `.clamp(0.2, 100.0)` still sitting in `fit()` — 34g removed that
clamp from zoom precisely because it culled fitted scenes.

## Step 9 — curves finally draw: `src/engine/gpu/adapters.rs`, `mod.rs`

`walk_session`'s match had `Geometry::NurbsCurve(_)` in the do-nothing arm, so ~7% of every drawing
was silently invisible — and after the importer work below, *most* of it would be. A curve is a
polyline by the time the GPU sees it, so it rides the FLAT lane.

In `adapters.rs`, add `NurbsCurve` to the `use session_rust::{...}` line, then add above
`point_to_glyph`:

```rust
/// A curve becomes a polyline of ribbon segments. Sample count follows the curve's SIZE, not a
/// fixed number: a PDF sheet is mostly 1-2 mm glyph outlines (4 segments is already smoother than
/// a pixel) next to metre-long arcs (which need ~50), and a flat count would either shatter the
/// budget or visibly facet the big ones.
pub fn nurbscurve_to_segments(c: &NurbsCurve, instance_id: u32) -> Vec<CylinderSegment>{
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for i in 0..c.m_cv_count {
        if let Some(cv) = c.cv(i) {
            let w = if c.m_is_rat && cv.len() > 3 && cv[3] != 0.0 { cv[3] } else { 1.0 };
            for k in 0..3 { lo[k] = lo[k].min(cv[k] / w); hi[k] = hi[k].max(cv[k] / w); }
        }
    }
    if lo[0] > hi[0] { return Vec::new(); }
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);

    let (t0, t1) = c.domain();
    let color = c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let radius = encode_width(c.width);
    let pts: Vec<[f32; 3]> = (0..=n)
        .map(|i| c.point_at(t0 + (t1 - t0) * i as f64 / n as f64).to_f32())
        .collect();
    pts.windows(2).map(|w| CylinderSegment{
        p0: w[0],
        radius,
        p1: w[1],
        instance_id,
        color,
    }).collect()
}
```

Note it reads `linecolors` (plural) — a curve carries a vec, not the single `linecolor` a
line/polyline has, and an empty vec means black.

In `mod.rs`, add `nurbscurve_to_segments` to the `use adapters::{...}` line, delete
`Geometry::NurbsCurve(_) |` from the ignore arm, and add a real arm above `Geometry::Point(p)`:

```rust
                // Curves ride the FLAT lane too - sampled to segments, they ARE polylines by
                // the time the GPU sees them. A PDF sheet is mostly these (every bezier, and
                // every glyph outline once fonts are flattened to paths).
                Geometry::NurbsCurve(c) => {
                    t.objects.push((c.xform.clone(), [1.0; 4]));
                    t.segments.extend(nurbscurve_to_segments(c, ri));
                }
```

## Step 10 — hidden wireframe: `src/engine/gpu/mod.rs`

`push_mesh` emits a cylinder per edge and a sphere per vertex, unconditionally. That is right for a
box and disastrous for the next step, where every letter of text becomes a small mesh: each glyph
would be outlined in tubes and dotted at every vertex.

Rule: **edge width 0 means hidden**. It is safe because `widths()` is empty unless someone called
`set_linecolors`, and the existing `.unwrap_or(1.0)` keeps every ordinary mesh exactly as it was.

In `push_mesh`, find the `for (i, (a, b, col)) in m.edges_with_colors()` loop and insert above it:

```rust
    // Edge width 0 = HIDDEN wireframe. A mesh only has explicit widths if someone called
    // set_linecolors, so the 1.0 default below leaves every ordinary mesh untouched - but a
    // triangulated PDF fill (a letter, a poché region) asks for no wireframe at all, and without
    // this every glyph would render outlined in tubes and dotted at each vertex.
    // A single width BROADCASTS to every edge - one entry instead of one per edge, which for
    // thousands of small glyph meshes is the difference between a lean .pb and a fat one.
    let width_at = |i: usize| -> f64 {
        let w = m.widths();
        if w.len() == 1 { w[0] } else { w.get(i).copied().unwrap_or(1.0) }
    };
    let hidden = |i: usize| width_at(i) == 0.0;
```

then, as the loop's first statement, `if hidden(i) { continue }`, and use the broadcast everywhere a
width is read: `radius: encode_width(width_at(i)),` in the edge loop, `let w = width_at(i);` in the
`vwidth` loop below it (guarded by the same `if hidden(i) { continue }`), and a dot loop that skips
a vertex with no surviving edge — replace
`radius: encode_width(vwidth.get(&vk).copied().unwrap_or(1.0)),` with `radius: encode_width(vw),`
after adding, as the dot loop's first line:

```rust
        let Some(&vw) = vwidth.get(&vk) else { continue };
```

## Step 11 — the importer, in two stages: `pdf_to_session.py` + `pdf_build.rs`

> **Superseded.** This two-stage Python+Ghostscript importer was replaced by a pure-Rust MuPDF
> device, which after a spell as a standalone `session_pdf`/`session_io` crate now lives in the
> kernel as `session_rust/src/pdf.rs` — Part 3 below records the final state. The extraction
> lessons here (white knockouts, size-adaptive flattening, fills as meshes, layers as groups) all
> carried over; only the machinery changed.

What a PDF sheet actually contains, measured with PyMuPDF on `30700 Querschnitt G-G.pdf`:

| feature | count | before |
|---|---|---|
| stroke paths | 57,292 | imported |
| fill paths | 1,348 (4,803 once fonts are flattened) | drawn as hollow outlines |
| text | 543 spans, 3,651 chars | **absent** |
| white knockout paths | 1,017 | imported as phantom rectangles |
| named CAD layers (OCG) | 33 | discarded |
| page box | 2979 × 2526 pt | absent |
| dashes / images / annotations | none / 0 / 0 | — |

**Python extracts, Rust builds.** PyMuPDF is the only PDF reader we have, so extraction stays in
Python — but nothing else does. The first version triangulated the fills in Python and wrote the
`.pb` there too: **3 min 11 s for one sheet**. The same work in Rust, with the rayon-parallel
`from_polygon_with_holes_many` and the Rust protobuf writer, is **1.8 s**; nine sheets import in
1m43s instead of most of an hour. Python now emits a `.paths.json` of primitives and stops.

```
pdf_to_session.py   PDF ─► strokes / curves / fills(loops) / layers / page   ─► <stem>.paths.json
pdf_build.rs        JSON ─► CDT (parallel) + Session + tree groups + protobuf ─► <stem>.pb
import_drawings.sh  both stages for all nine sheets
```

**11a. White is a knockout, not ink.** A white path on white paper is a mask box — behind text,
behind title-block fields. Imported, it is ~1,100 phantom rectangles floating over the drawing.

```python
def is_white(c):
    return c is not None and min(float(v) for v in c[:3]) >= 0.99
```

Applied to the fill colour and the stroke colour *separately* — one path can be both.

**11b. Text.** `page.get_drawings()` returns *paths*; text lives in a layer it cannot see.
Ghostscript rewrites every glyph as outlines, and — the useful surprise — emits them as **fill
paths** while preserving all 33 OCG layers:

```python
    r = subprocess.run(["gs", "-q", "-o", tmp.name, "-sDEVICE=pdfwrite", "-dNoOutputFonts", src],
                       capture_output=True)
```

Note what this costs: the font is gone. A glyph arrives as contours, with no character code, no
family, no size — solid letters you cannot re-typeset. Real text needs a `Text` type in the kernel
(3 languages + proto), a font system in the viewer, and `page.get_text("dict")` in the importer;
`InstanceRef` (`session_rust/src/instance_ref.rs`, not yet a `Geometry` variant) is the natural
middle step — one mesh per unique character, one placement per occurrence.

**11c. Flatten a fill contour by its own size.** Every extra vertex on a glyph is also a face, a
halfedge and a triangulation entry in the `.pb` — mesh serialization is where a drawing's file size
now lives. A fixed 6 samples per bezier cost 619 MB across nine sheets; sizing each cubic by its
control-polygon length cost 444 MB for the same picture:

```python
def bez_steps(a, c1, c2, b):
    d = (math.dist(a, c1) + math.dist(c1, c2) + math.dist(c2, b))
    return max(2, min(12, int(math.sqrt(d / BEZ_CHORD))))
```

**11d. Fills become meshes — in Rust.** Text and poché are one problem: closed contours, where a
break in the item chain starts a new one (that is how a glyph's counter, the hole in `o`, `a`, `e`,
arrives; 1–4 contours per letter). `pdf_build.rs` triangulates them all at once:

```rust
    let inputs: Vec<Vec<Vec<Point>>> = paths.fills.iter()
        .map(|f| f.loops.iter().map(|lp| points(lp)).collect())
        .collect();
    let meshes = Mesh::from_polygon_with_holes_many(inputs, true, true);
```

then per mesh:

```rust
        m.set_objectcolor(color(f.c));
        // A fill is flat colour: drop the auto-seeded per-vertex/per-face vecs, which would
        // otherwise dominate the .pb for thousands of glyph meshes.
        m.clear_pointcolors();
        m.clear_facecolors();
        // ONE transparent, zero-width entry: the viewer broadcasts a single width to every edge,
        // and reads width 0 as "no wireframe" - a letter renders solid, not outlined and dotted.
        m.set_linecolors(vec![Color::new(0.0, 0.0, 0.0, 0.0)], vec![0.0]);
```

`true, true` = pick the border by largest bbox (PDF does not guarantee contour order) and run
rayon. Note `set_linecolors` does *not* change `color_mode` — the fill colour stays with
`objectcolor`, which is the precedence table at the top of this lesson doing its job on data nobody
hand-authored. The single-entry width is what Step 10's `width_at` broadcasts.

**11e. Layers and the page edge.** One tree group per CAD layer, passed as the `parent` argument
every `add_*` already accepts:

```rust
    let mut group = |s: &mut Session, i: usize| {
        groups.entry(i)
            .or_insert_with(|| s.add_group(paths.layers.get(i).map(|x| x.as_str()).unwrap_or("0 unlayered")))
            .clone()
    };
```

plus a closed polyline of the page box in a `page` group — so a sheet's extents are the PAPER, not
its ink, and sheets whose content sits off-centre stop looking mis-placed next to each other.

Result across the nine sheets: **3,373–14,423 fill meshes each** (text + poché), 1–33 layer groups,
20,613 knockouts dropped from the worst sheet, and 572,097 objects in the merged scene under 194
groups.

## Verify — Part 2

```bash
cd session_data && ./import_drawings.sh          # ~11s per sheet: 4.6s extract + 1.8s build
cd ../session_rust && cargo run --release --example probe_scene -- ../session_data/draw_pf_he.pb
cargo run --release --example combined_scene     # nine sheets, grid-placed, one .pb
```

`probe_scene` must show `mesh` among the types, `moved=<all>` (8a), and bounds equal to the page
box, not the ink box. Then copy the result into `session_viewer/dist/session_data/` — **trunk only
re-copies assets when it rebuilds**, and a stale `dist` looks exactly like a fix that did not work.

On screen: labels and dimensions are solid black text, poché is filled, no white rectangles, no
wireframe outlining the glyphs, a rectangle around each sheet, ink in its real colours (Step 5), and
the dot at a polyline's end no longer floats in front of it from every angle (Step 7).

## Still missing

Honest list, all deliberate: `PointCloud`, `NurbsSurface`, `Plane`, `OBB` and `Element` are still
do-nothing arms in `walk_session`. Dash patterns have no kernel representation (none of these sheets
use them). Fills are lit by the 3D key/fill lights rather than drawn flat, and the triangle pipeline
does not blend, so a translucent fill renders opaque. Text is glyph outlines, not an SDF atlas —
fine at 3,651 characters per sheet, wrong at 100,000. And hatching arrives from these PDFs
pre-exploded (56,889 single-segment paths, median 3.2 pt), so a hatch *shader* would have nothing to
shade: that only pays off for hatch entities our own kernel authors, and it needs filled regions
first — which Step 11c just built.

## Recap

```
Ch 34b: session → tables; colors were whatever happened to reach the rows.
Ch 34h: RESOLVE COLORS/WIDTHS ONCE, CPU-SIDE. Row color = the user's color (precedence:
        color_mode gates FACECOLORS/POINTCOLORS — auto-seeded vecs mean nothing; linecolors ride
        edges_with_colors; surfacecolor bakes into the BRep mesh). Instance.color = WHITE TINT,
        multiplied in all the shaders (selection's channel, lesson 70). Width = multiplier in the
        radius sign lane (0 default / negative px-multiplier / positive world) — width==1.0 encodes
        0.0, defaults bit-identical. to_render grows a FACECOLORS branch (duplicated verts, flat
        color; Rust-only bridge). Dots: pointcolors when user-set, dark constant otherwise —
        every-vertex-dots policy unchanged.

Ch 34h pt2: THE SAME RULES, NINE REAL SHEETS. Hairline fade gets a FLOOR (a 0.09mm plot pen is
        sub-pixel at every zoom; unfloored it reads as white paper). MSAA is chosen per SCENE, not
        per lane — sample count belongs to the PASS (9→50 fps). Flat ink writes no depth, so
        ink-vs-ink order is HashMap order until INK_DEPTH_PREPASS pays for a second pass. Two
        silent defects: Line::pb_dumps dropped xform (Rust only — C++/py were right; nine sheets
        stacked at the origin), and camera-relative anchoring mixed metres with millimetres so it
        NEVER RAN (geometry clipped away when zooming in). NurbsCurve finally draws, sampled by
        size. Mesh edge width 0 = hidden wireframe — which lets PDF fills and TEXT arrive as
        triangulated meshes (gs -dNoOutputFonts turns glyphs into fill paths), with CAD layers as
        tree groups and the page box as a polyline.
```

---

# Part 3 — the importer rewritten, the load path attacked (2026-08-11)

A record of what changed after Part 2, in the order it landed. These steps are already in the
repo — this is the map of WHERE, not instructions to retype.

## 3.1 The importer lives in the kernel: `session_rust/src/pdf.rs`

The pure-Rust MuPDF device that replaced Step 11's Python+Ghostscript pair spent a while as its
own crate (`session_pdf`, renamed `session_io`) on the theory that every foreign format should
funnel through one converter writing `.pb`. **That theory was measured and dropped.** Handing bulk
geometry to a kernel as protobuf costs *3–4× the parse it was meant to share* — 160k vertices of
OBJ parse in 757 ms, then serialize+deserialize adds 2965 ms on top — and inflates 12.9 MB of OBJ
into 83 MB of `.pb`. Parsing was never the bottleneck; rebuilding the geometry twice more is.

So ordinary formats (obj, xyz, ply) are parsed **natively in each kernel**, and the importer moved
into `session_rust` as `pdf.rs`, exposing `session_rust::pdf::import_pdf(src, stem, page)` plus a
`pdf_import` bin. `session_io` is gone.

PDF keeps one special property that earns the split *within* the crate: `mupdf-sys` compiles
MuPDF's own C sources, which **cannot build for wasm32** — the viewer's target. So the module and
its bin sit behind an optional `pdf` feature, off by default:

```toml
[features]
default = []
pdf = ["dep:mupdf", "dep:earcutr"]

[[bin]]
name = "pdf_import"
path = "src/bin/pdf_import.rs"
required-features = ["pdf"]
```

```rust
#[cfg(feature = "pdf")]
pub mod pdf;
```

`cargo check --target wasm32-unknown-unknown` therefore pulls no MuPDF at all and the viewer builds
exactly as before; `import_drawings.sh` passes `--features pdf` for the native tool. The viewer
still *loads* sheets as `.pb` — it cannot run the importer itself, because a browser has no
filesystem and MuPDF's C sources have no wasm target.

## 3.2 Correct reading (`session_rust/src/pdf.rs`)

- **Dash patterns import dashed.** mupdf hands a custom device the raw `dashes()`/`dash_phase()`
  (user-space, scaled by `ctm.expansion()` like the width); the pattern is walked by arc length
  over the flattened chain and each ON-run becomes its own stroke. A dashed cubic is flattened —
  it cannot stay an analytic NurbsCurve.
- **The nonzero winding rule.** `fill_path` was ignoring its `even_odd` flag; `islands()` now
  classifies by winding number when it is false (a same-orientation nested contour is REDUNDANT,
  not a hole — parity punched wrong holes). Glyph outlines fill nonzero, per spec.
- **Close-after-curve.** `Seg::Close` closes to the tracked subpath START; the old chain-reset on
  `Seg::Curve` dropped the closing edge (`draw_pb_haus25` gained 43 polylines back).
- **Nothing drops silently.** Raster images (with placed mm² area), gradient shadings and
  no-outline glyphs (Type 3) are counted and WARNED about; an empty space-glyph outline is not a
  failure. `create_dir_all` replaced the `is_dir()` gate on the font dump.
- **Widths are absolute mm.** `PEN_REF` (0.35) deleted; PDF width 0 → 0.1 mm hairline. The real
  pen table survives into the file — `0.14×6177, 0.28×48332, 0.37×769, 0.51×925, 0.71×686` on one
  sheet — and the viewer's planar lane treats `-w` as full mm width (`radius = -w * 0.5`).

## 3.3 The 90× blow-up, halved (importer perf)

- **Glyph cache**: outline + islands + earcut ONCE per `(font, gid)` in glyph space; each
  occurrence only transforms cached verts by its text matrix. The same `e` was being outlined and
  triangulated thousands of times.
- **Merged fills**: ONE mesh per `(layer, rgba)` — 5142 meshes → 11 on one sheet. Per-object cost
  (2 guids, graph node, tree node, proto framing) is what the .pb and the parse were paying.
- Corpus: **781 → 486 MB (−38%)**; import ≈1-3 s/sheet; zero dropped/bad fills on all nine.

## 3.4 The viewer load path

- **Pipelined fetch, window of 2** (`app/persistence.rs` + `state.rs`): `fetch_start()` builds the
  request and wraps the browser promise — which is EAGER; only the Rust await is lazy — so file
  N+1 downloads while file N parses. Measured: every fetch fully hidden (77-304 ms inside
  ~1-2 s parses). Do NOT fetch all up-front: raw bytes for ten sheets would blow the wasm heap.
- **`INK_DEPTH_PREPASS = false`** (see the note in Step 7).
- **Kernel serialization stopped double-encoding** (after the Xform refactor landed): every
  geometry type got `to_proto()`/`from_proto()` split out of `pb_dumps`/`pb_loads`, and
  `Session::pb_dumps`/`pb_loads` use them directly — the old path encoded every object to bytes
  and decoded it again, in BOTH directions. minitest 718/718; byte-identical output.
- **Placement moved into the Session** (the Xform refactor, a parallel effort): geometry types
  lost their `xform` member; transforms live in `session.xforms` (`set_xform`/`world_xform`), and
  `walk_session` resolves them in ONE pass via `session.world_xforms()` — the per-object form
  rescans the tree each call. Per-sheet placement stays in `assets/scenes/drawings.toml`.

Measured end-to-end, ten sheets, 486 MB: **28.6 s → 24.3 s** load (parse −30%), then 165 fps at
744k objects over 7 draw calls. The remaining 24 s is ~11 s parse + ~11 s walk — which is what
lesson 35 restructures (the walk moves into `Scene`, and the first sheet stops waiting for the
other nine).

## Next

`35-scene-struct.md` — the walk (now carrying the full color/width resolution) moves out of `Gpu`
into an app-layer `Scene`, and loading turns PROGRESSIVE: first sheet on screen in ~2 s, the rest
streaming in behind it. Later consumers of today's channels: 45 selection (writes the tint),
47 thickness slider (scales every multiplier width for free), PointCloud lesson (per-point colors +
`point_size` via a repurposed `Instance._pad[0]`), 63 BRep per-face colors (maps
`BRepFace.facecolor` onto the mesh's facecolors — rides today's lane, but needs face-run tracking in
`brep.rs`: a 3-language kernel change, flag it then).
