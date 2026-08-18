# 35b The mesh edge lane — a pen that survives the surface it draws on

## Goal

Draw a mesh's edges as **camera-facing rectangles** — two triangles an edge instead of a
tessellated tube — at the correct pen width, with occlusion that holds at every zoom, every
angle and every pen width. And, just as important, understand *why* the obvious fixes fail, so
the next person does not spend a day rediscovering it.

> **Big picture.** 31 gave edges a 3D tube: twelve triangles per edge, and the tube's radius
> lifts the ink off the surface it decorates, so it never loses the depth test. It looks right
> and it costs 90× the geometry it decorates. A flat rectangle is 2 triangles — but a flat
> rectangle *lies in the plane through the edge*, and at a convex edge that plane cuts into the
> wedge the two faces form. Half the pen ends up inside the solid. Everything in this lesson
> follows from that single sentence.

<svg viewBox="0 0 680 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a tube bulges toward the eye by its radius so it is never buried; a flat quad lies in the plane through the edge and half its width sits inside the wedge the two faces form" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="170" y="16" fill="#888" text-anchor="middle">tube — proud by r in every direction</text>
  <path d="M60 150 L280 90" stroke="#3a3a3a" stroke-width="1"/>
  <path d="M60 150 L280 190" stroke="#3a3a3a" stroke-width="1"/>
  <circle cx="60" cy="150" r="16" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <text x="98" y="150" fill="#6fb3ff">r</text>
  <text x="170" y="120" fill="#666" text-anchor="middle">face A</text>
  <text x="170" y="182" fill="#666" text-anchor="middle">face B</text>
  <text x="170" y="205" fill="#5c5" text-anchor="middle">nothing can bury it</text>

  <text x="510" y="16" fill="#888" text-anchor="middle">flat quad — a PLANE through the edge</text>
  <path d="M400 150 L620 90" stroke="#3a3a3a" stroke-width="1"/>
  <path d="M400 150 L620 190" stroke="#3a3a3a" stroke-width="1"/>
  <line x1="384" y1="150" x2="416" y2="150" stroke="#c66" stroke-width="3"/>
  <text x="510" y="120" fill="#666" text-anchor="middle">face A</text>
  <text x="510" y="182" fill="#666" text-anchor="middle">face B</text>
  <text x="510" y="205" fill="#c66" text-anchor="middle">both halves cut INTO the solid</text>
</svg>

## Files we touch

| file | change |
|---|---|
| `src/engine/gpu/mod.rs` | `LineStyle`, `CylinderSegment` repacked 48 → 40 B, `Instance.extent`/`.spacing`, `eye_from_view_proj` |
| `src/app/scene.rs` | `pack_rgba`, `oct16`, `pack_facing`, `mesh_spacing`, adjacency in `push_mesh` |
| `src/shaders/ribbon.wgsl` | the whole flat lane: width, near-plane clip, facing cull, plane hug, density LOD |
| `src/shaders/sphere.wgsl` | vertex markers: the same hug, drawn last, their own cull and LOD |
| `src/shaders/cylinder.wgsl` | the tube lane reads the same repacked table |
| `src/selftest.rs`, `examples/selftest.rs` | `VIEWER_ORBIT`, `VIEWER_ZOOM`, manifest loading, table footprint |

---

## Step 1 — two lanes over one table

Both lanes read the **same** `CylinderSegment` rows, so switching costs one branch at the draw
site and nothing in memory.

**Find** in `src/engine/gpu/mod.rs`, near the other GPU enums:

```rust
pub struct Gpu {
```

**Add above it:**

```rust
/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
/// switching costs one branch at the draw site and nothing in memory.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: 12 triangles, and the radius lifts the ink off the surface.
    Tubes,
    /// A camera-facing quad per edge: 6 vertices, the flat lane's own shader.
    Flat,
}
```

At the draw site in `encode_frame`, the two lanes differ by one `match`:

```rust
match self.line_style {
    LineStyle::Tubes => {
        pass.set_pipeline(&self.pipelines.cylinder);
        pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.pipe_count);
    }
    LineStyle::Flat => {
        pass.set_pipeline(&self.pipelines.ribbon_solid);
        pass.draw(0..6 * self.pipe_count, 0..1);   // vid/6 picks the row
    }
}
```

Bind **L** in `lib.rs` to flip it. Keep the tube lane forever: it is *real geometry*, so it is
the only ground truth you can hold a screen-space construction against. Every width bug below
was caught by measuring the flat lane against it.

## Step 2 — the row: 48 → 40 bytes, and where the adjacency lives

**Replace** `CylinderSegment` in `src/engine/gpu/mod.rs` with:

```rust
pub struct CylinderSegment{
    // FLAT f32s, not `[f32; 3]`: WGSL aligns `vec3<f32>` to 16, so a struct containing one is
    // padded to a multiple of 16 - this table was 48 B and could not have been 40 whatever else
    // was packed. Scalars align to 4, so the stride is the honest sum of the fields.
    pub p0: [f32; 3],   // 12 B
    pub radius: f32,    // 4 B - 0.0 = screen-constant px; > 0 = world-mm override
    pub p1: [f32; 3],   // 12 B
    pub instance_id: u32,  // 4 B
    pub color: u32,     // 4 B - RGBA8, low byte red. Was `[f32; 4]` carrying 8-bit colour.
    pub facing: u32,    // 4 B - two oct16 adjacent face normals
}                       // 40 B
```

Packing the colour paid for `facing` **and** took 8 B off every row: on the bunny that is
104,288 edges, 4.0 MB where 48 B would have been 4.8.

The `facing` word is two octahedral normals, 16 bits each — about 1.4°, when all that is ever
asked of them is the **sign** of a dot product. In `src/app/scene.rs`:

```rust
fn oct16(n: &Vector) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();
    if !(l > 0.0) { return None; }
    let (mut x, mut y) = (n[0] / l, n[1] / l);
    if n[2] < 0.0 {
        // signNotZero, NOT signum. `f64::signum(0.0)` is 0.0, which folds (0,0,-1) onto (0,0) -
        // the code for (0,0,+1) - so the two poles collide. On an axis-aligned box that is the
        // top and bottom faces, i.e. most of its edges.
        let s = |v: f64| if v < 0.0 { -1.0 } else { 1.0 };
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * s(x), (1.0 - ax) * s(y));
    }
    let q = |v: f64| (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff;
    Some(q(x) | q(y) << 8)
}

/// "No adjacency, always draw". It CANNOT be 0: (0,0) is the honest encoding of +Z.
pub const FACING_UNKNOWN: u32 = u32::MAX;
```

> **The bug that hid for a day.** Both of those comments are scar tissue. `signum(0.0) == 0.0`
> made ±Z encode identically, and that collision landed on an all-zeros sentinel — so the facing
> test was silently inert for most of a box's edges, and an experiment that depended on it
> "proved" nothing. If you take one habit from this lesson: **a sentinel must be a value the
> encoder can never produce.**

In `push_mesh`, fill it from the halfedge the kernel already has:

**Find:**

```rust
    let edges = m.edges_with_colors();
```

**Add below it:**

```rust
    // Face normals once for the whole mesh, so the per-edge lookup is two map reads.
    let fnormals = m.face_normals();
```

and inside the edge loop, before the `segments.push`:

```rust
        let f = m.edge_faces(a, b).unwrap_or_default();
        let facing = pack_facing(
            f.first().and_then(|&k| fnormals.get(&k).cloned()),
            f.get(1).and_then(|&k| fnormals.get(&k).cloned()),
        );
```

A naked edge (the bunny has 223 of them) gets its single normal duplicated — a boundary edge is
visible whenever its one face is, which needs no special case in the shader.

## Step 3 — the width was twice the pen, twice over

**NDC spans [-1, 1] across `vp_h` pixels, so one NDC unit is `vp_h/2` px, not `vp_h`:**

```
y_ndc = (y_eye / d) * cot(fovy/2)        px = y_ndc * vp_h/2 = y*cot*vp_h / (2*d)
```

The lane divided by `vp_h`. And separately used `thickness` — documented as an on-screen
**width** — as a **half**-width. Same factor twice.

**Replace** the width helper in `src/shaders/ribbon.wgsl`:

```wgsl
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius > 0.0){
        if (line.ortho_h > 0.0){
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }
        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }
    return line.thickness * 0.5 * select(1.0, -radius, radius < 0.0);
}
```

How to know you got it right: **measure against the tube lane**, which is real geometry of
radius `r` and cannot be argued with. Before this a mesh edge measured 8 px flat against 4 px as
a tube; after, 4 against 4. This is also why the depth artifacts below were so violent — the
wedge is proportional to band *width*, so a pen at twice its size fights twice as hard.

## Step 4 — the quad is a TRAPEZOID, so the width cannot be a varying

Under perspective the two ends project to different widths, so half-width is a function of the
along-coordinate — which over a trapezoid is **projective, not affine**. Hand the rasterizer a
per-vertex `hw` and each of the quad's two triangles builds its own affine approximation; they
agree only on the diagonal they share, and the seam shows as a **triangular bite** out of the
band along that diagonal.

**Change** the varyings:

```wgsl
    // Half-width in px at each END, both FLAT. Never interpolated.
    @location(4) @interpolate(flat) hw0: f32,
    @location(5) @interpolate(flat) hw1: f32,
```

and resolve per fragment, at the SDF's own along-parameter `h`:

```wgsl
 fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), hairline_fade(raw));
 }
```

Exact, and independent of how the quad happens to be triangulated. The centreline depth gets the
same treatment (`zend`).

## Step 5 — clip against the near plane before dividing by w

This lane projects by hand, and a hand divide is only valid **in front of the eye**. The old
`c.xy / max(abs(c.w), 1e-6)` does not clip a vertex behind the eye — it **mirrors** it through
the screen centre, and the quad splays off across the model.

**Add** before the screen-space mapping:

```wgsl
    // In CLIP space `z - w` is linear along the segment and the near plane is exactly z - w = 0
    // (reverse-Z depth z/w = 1), visible side <= 0. Closed form, no uniform, and it needs to know
    // neither the near distance nor the scene scale.
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;
    if (f0 > 0.0 && f1 > 0.0){ return dead_vertex(); }
    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);
```

The tube lane never had this bug because the hardware clips real geometry for you.

## Step 6 — hidden edges never reach the rasterizer

An edge belongs to two faces. If **both** turn away, it is inside the solid.

```wgsl
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if (facing == FACING_UNKNOWN){ return true; }
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}
```

The eye it needs is recovered from the view-projection alone — **the eye is the one point that
projects to nothing**, where clip x, y and w all vanish. Three rows, one 3×3 solve
(`eye_from_view_proj`), so it works for any caller including the headless harness. It must be
the real eye, not a constant forward direction: at 60° FOV a constant is off by 30° at the frame
corner, and near-silhouette edges are exactly the ones that would flip.

> **`FLAG_INSIDE`.** From inside a solid *every* face points away, so this cull would delete the
> whole object the moment the camera crosses a face. A per-edge test cannot tell "far side of the
> solid" from "eye inside it" — that difference is global, so it rides the instance row as a
> per-frame CPU flag from the object's world AABB.

## Step 7 — the depth trade, and why no offset can win

Here is the load-bearing paragraph of the whole lesson.

A band's depth is its **centreline's** — one value across the whole width. The depth test runs
**per fragment**. At screen distance `d` from the centreline the adjacent face has already risen
toward the eye by `d · tan θ`, θ being the angle between that face's normal and the view ray. So
the offset the ink needs is proportional to the **pen width**, and it is **unbounded** as the
face turns edge-on.

And the trade is symmetric: any offset large enough to clear a pen makes the two faces meeting at
an edge fight each other over a band of the same width. One artifact converts into the other.

| attempt | what happens |
|---|---|
| constant ink lift toward the camera | clears mild cases; a grazing face needs an unbounded value |
| relative face push (`clip.xy*K, clip.z, clip.w*K`) | same limitation — it is a constant |
| hardware `DepthBiasState` slope_scale | works, but `constant`'s units on a float depth format are implementation-defined |
| `dpdx/dpdy` slope bias in the face shader | at the strength that kills the wedge, 16 px slivers along every shared edge |
| per-edge secant lift `r·tan θ` | correct law, still a race against the same unbounded quantity |

**If a constant is being tuned, the model is wrong.** That is the tell.

## Step 8 — the fix: the ink HUGS the surface it decorates

Stop choosing a distance. The adjacent faces are **planes**, their normals are already in the
table, and a plane's depth at a pixel is closed form. Write, per fragment, the depth of whichever
front-facing adjacent plane is nearer here, one epsilon in front.

Build the planes in **clip space**, as the homogeneous join of three transformed points:

```wgsl
// The plane three clip-space points span, as four signed 3x3 minors (each a dot with a cross).
// No matrix inverse and no normalize - the fragment's solve divides the overall scale back out.
fn join3(a: vec4<f32>, b: vec4<f32>, c: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        dot(a.yzw, cross(b.yzw, c.yzw)),
        -dot(a.xzw, cross(b.xzw, c.xzw)),
        dot(a.xyw, cross(b.xyw, c.xyw)),
        -dot(a.xyz, cross(b.xyz, c.xyz)),
    );
}
```

The three points are the two endpoints (both lie on both faces) plus one stepped
`cross(n, edir) * elen` off the midpoint. Near-plane clipping is irrelevant to them: a clip-space
point with `w < 0` is still algebraically on the plane.

Then the fragment solves it:

```wgsl
//     pl.x*nx + pl.y*ny + pl.z*nz + pl.w = 0   =>   nz = -(pl.x*nx + pl.y*ny + pl.w) / pl.z
```

Three rules make it work:

1. **`max()` against the centreline, never `min`.** A plane must not pull ink *behind* the
   centreline, or a silhouette edge loses its outer half.
2. **Back-facing planes are skipped.** Past the silhouette they continue through space that is
   not the object, and would drag the ink forward over things it should not cover.
3. **The epsilon is derived, not tuned** — `HUG_ABS + HUG_PIX·slope + HUG_REL·rise`: float
   disagreement between the plane solve and the face rasterizer; the plane's own ndc-z change per
   pixel (under MSAA the ink solves once at the pixel centre while the face holds a depth *per
   sample* — `glPolygonOffset`'s slope term, in closed form); and a fraction of the local rise,
   which covers the oct16 normals' 1.4° quantization.

**The way to see what this is doing:** it makes the flat band compute, per fragment, the depth
the *tube* would have had. The tube gets that from geometry; the ribbon gets it from algebra.

## Step 9 — vertex markers are the topmost ink

Markers ride the same hug. Two rules beyond it:

- **Draw them LAST of the solid lane, and compare `GreaterEqual`.** Drawn first they must win
  *strictly*, because the band testing `GreaterEqual` against them takes any tie. Drawn last they
  only have to match — a strictly weaker condition.
- **Bound the band's own epsilon.** The band references its centreline depth *at the fragment*,
  and a fragment on the disc is up to one marker radius along the band from the vertex, where
  that centreline has moved by the plane's screen slope times the distance:

```wgsl
            let band_span = slope_px * (in.px + 0.5);
            let eps = HUG_ABS + HUG_PIX * slope_px
                + HUG_REL * (abs(zp - z_band) + band_span) + SPHERE_TIE;
```

A trihedral corner needs **three** face pairs, not one — a marker hugging only the widest
incident edge's two faces still loses a sector of its disc to the third face's band. `GlyphPoint`
carries `facing` + `facing_ext[2]`, up to six incident normals.

## Step 10 — density: the part that is not a depth problem at all

Zoom out and a dense mesh goes *see-through*. It is tempting to read that as another depth bug.
It is not, and no depth fix touches it: 104,288 edges and 35,947 markers at **screen-constant**
width over a bunny 100 px tall is ink on every pixel several times over, and a thin feature's
front and back land within a pixel where 4× MSAA resolves both. That is why you can see the
inside of an ear through its near side.

A 2 px pen does not shrink with the model. So stop drawing wires once they fall below the
density the screen can carry.

**Add** to `src/shaders/ribbon.wgsl`, after the width is known:

```wgsl
const WIRE_MIN_PX = 2.5;

    // Below the density threshold this wire is noise, not information.
    if (seg.facing != FACING_UNKNOWN && len < WIRE_MIN_PX){
        return dead_vertex();
    }
```

Measured on the edge itself, so it needs no per-object data. A marker cannot measure its own
length, so it uses the object's vertex **spacing** — `extent / sqrt(vertices)`, computed in
`mesh_spacing` and shipped on the instance row — projected the same way a world radius is:

```wgsl
    let sp = instances[g.instance_id].spacing;
    if (sp > 0.0 && line.ortho_h <= 0.0
        && sp * line.proj_y * line.vp_h * 0.5 / max(clip.w, 1e-6) < MARKER_MIN_PX) {
        return dead_dot();
    }
```

Free-standing linework is exempt on both counts (`facing == FACING_UNKNOWN`, `spacing == 0`): a
short polyline segment is a real line the user drew, and a drawing is full of them.

**And one more unbounded quantity while you are here.** The lift is a fraction of *eye depth*, so
world lift = lift × eye depth grows with camera distance while an object's size does not. On a
1000 mm box with a 2 px pen it exceeds the box at **242 m** for a band and **91 m** for a
marker — ordinary zoom-out. `lift_capped` clamps it to a tenth of the object's world AABB
diagonal, which the CPU already computes for `FLAG_INSIDE`.

## Step 11 — the harness, and the acceptance test that ends the argument

None of the above can be judged by eye. Give `selftest` three knobs — `VIEWER_ORBIT`,
`VIEWER_ZOOM`, and a `.json` argument it resolves as a **scene manifest the way the browser
does** — and one comparison:

> Render with the ink's depth test on, and again with `VIEWER_NO_DEPTH=1` forcing `Always`. On
> genuinely visible edges the two must match. They may differ only where an edge is truly
> occluded.

That number is the bug. It went **1804 → 12** differing px of 675,000 at zoom 19.

```bash
cargo build --release --example selftest --target x86_64-unknown-linux-gnu
VIEWER_W=900 VIEWER_H=750 VIEWER_ZOOM=19 VIEWER_ORBIT="10,-8" \
  ./target/x86_64-unknown-linux-gnu/release/examples/selftest out.ppm assets/scenes/bunny.json
```

Two more measurements worth keeping in the harness, both one line of output:

- **table footprint** before upload — bunny: 1.4 MB verts + 0.8 indices + 4.0 edges + 1.6 markers
  = **7.7 MB**
- **staged RSS** — 17.2 MB file → +74.1 MB decode+build → +17.1 MB walk = **108.5 MB**. The
  connectivity, not the render data, is the whole cost: ~3.4 KB per vertex, which is the
  `HashMap<usize, HashMap<usize, Option<usize>>>` halfedge with one allocation per vertex.

## Faster loading

`trunk build` is a **debug** wasm build. Release is 7.1 MB against 10.8, and `session_viewer`'s
own walk — `push_mesh` over 104,288 edges — stops running unoptimized. (`[profile.dev.package."*"]
opt-level = 3` already optimizes *dependencies*, so the kernel parse was never the slow part.)

```bash
trunk serve --release     # not just `trunk serve`
```

## Verify

- **L** toggles tubes ↔ flat. On a free-standing polyline the two lanes are pixel-identical; on a
  box edge they agree to a pixel. If flat is twice as wide, Step 3 is missing.
- Zoom into a box corner: bands solid, no wedge of surface inside them, the vertex marker a clean
  disc.
- Zoom out on the bunny: a smooth shaded surface, not speckle you can see through.
- A 2D drawing sheet is untouched — 52,244 ink px before and after every step here.

## Reference — checking what you typed

The end state of this lesson **is** the current `session_viewer/src`. Every step above landed as
its own commit on `origin/pointcloud-memory`, so you can diff your own typing against the exact
change rather than against a finished file:

```bash
git log --oneline main..origin/pointcloud-memory   # the whole arc, newest first
git show cd4af476 -- session_viewer/src/shaders/ribbon.wgsl   # e.g. the 2x width fix
```

| step | commit | what it changed |
|---|---|---|
| 1 — two lanes, L to toggle | `eee380c3` | `LineStyle`, the draw-site match |
| 3 — width 2x, and the oct poles | `cd4af476` | `half_width_px`, `oct16` signNotZero, all-ones sentinel |
| 4 — trapezoid width | `c9dfddf6` | `hw0`/`hw1` flat + `resolve_width` |
| 5 — near-plane clip | `c9dfddf6` | clip against `z - w = 0` |
| 2, 6 — 40 B row + facing cull | `ff86cfab` | repacked `CylinderSegment`, `edge_faces_camera`, `eye_from_view_proj` |
| 7, 8 — the hug | `6c8f8c50` | `join3`, `ink_depth`, the derived epsilon, `FLAG_INSIDE` |
| 9 — markers on top | `2fbd7cd2` | draw last + `GreaterEqual`, the `band_span` bound |
| 10 — density LOD + lift cap | `4f1ae1d1` | `WIRE_MIN_PX`, `MARKER_MIN_PX`, `lift_capped` |
| 11 — harness | `662c099f`, `e13ab7b3` | `VIEWER_ORBIT`/`VIEWER_ZOOM`, manifest loading |

Two commits in that range are **dead ends kept on purpose** — `b50e78fd` (lift + hardware slope
bias) and `04b17444` (the `dpdx/dpdy` slope bias, reverted because at the strength that kills the
wedge it puts 16 px slivers along every shared edge). If you find yourself reinventing either,
their messages say what happened.

## Next

[`36-raw-cloud-lane.md`](36-raw-cloud-lane.md) — meshes now have a lane that scales. Point clouds
do not: 35 routes them into the flat glyph dots, which is the right answer for a demo cloud and
the wrong one for a 13.8M-point scan. 36 gives dense clouds their own lane, and 37–39 take the
loading peak apart the way this lesson took the pen apart.
