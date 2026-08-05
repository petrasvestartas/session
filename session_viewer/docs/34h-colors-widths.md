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
    <text x="500" y="80" fill="#888">selection reuses tint (45)</text>
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
with the same line: `final = row_color × instances[id].color`. Selection (lesson 45) will recolor an
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
untouched files stay bit-identical. The future thickness slider (47) writes `LineUniform.thickness`
and every user width scales with it, Rhino-style.

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

`cargo check` in both `session_rust` (native) and `session_viewer` (wasm). Then two gates:

**1. Regression — nothing moved.** `floor_model.pb` and the stress wall must look pixel-identical:
every default is white-tint × row-color and `radius 0.0`.

**2. Positive — user colors/widths appear.** Write a fixture with every channel exercised. Create
`session_rust/examples/colors_widths.rs` with exactly this content, then run
`cargo run --example colors_widths` from `session_rust/`:

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

Add a `copy-file` link for it in `index.html` (same pattern as 34e Step 5), point
`DEMO_SESSION_URLS` (34e) at just this file — `&["session_data/colors_widths.pb"]` —
and check, left to right: box 1 shows six distinct flat face colors (not white — the FACECOLORS bug
is dead), box 2 shows the vertex gradient AND gradient-colored dots, box 3 is indistinguishable from
before, the polyline is red at 5× thickness, the point is a fat black dot (4× — the width lane on
a glyph). Then swap the URL list back.

## Recap

```
Ch 34b: session → tables; colors were whatever happened to reach the rows.
Ch 34h: RESOLVE COLORS/WIDTHS ONCE, CPU-SIDE. Row color = the user's color (precedence:
        color_mode gates FACECOLORS/POINTCOLORS — auto-seeded vecs mean nothing; linecolors ride
        edges_with_colors; surfacecolor bakes into the BRep mesh). Instance.color = WHITE TINT,
        multiplied in all the shaders (selection's channel, lesson 45). Width = multiplier in the
        radius sign lane (0 default / negative px-multiplier / positive world) — width==1.0 encodes
        0.0, defaults bit-identical. to_render grows a FACECOLORS branch (duplicated verts, flat
        color; Rust-only bridge). Dots: pointcolors when user-set, dark constant otherwise —
        every-vertex-dots policy unchanged.
```

## Next

`35-scene-struct.md` — the walk (now carrying the full color/width resolution) moves out of `Gpu`
into an app-layer `Scene`. Later consumers of today's channels: 45 selection (writes the tint),
47 thickness slider (scales every multiplier width for free), PointCloud lesson (per-point colors +
`point_size` via a repurposed `Instance._pad[0]`), 63 BRep per-face colors (maps
`BRepFace.facecolor` onto the mesh's facecolors — rides today's lane, but needs face-run tracking in
`brep.rs`: a 3-language kernel change, flag it then).
