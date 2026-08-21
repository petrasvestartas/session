# 69c GPU trimming — the CDT stays home, the fragment decides

> **Big picture.** *Phase 10b.* This is the lesson where a CPU algorithm does **not** port —
> and knowing why is the content. 69's trimmed surfaces mesh through a Bowyer–Watson
> constrained Delaunay triangulation (`nurbssurface_trimmed.rs`): insert a point, find the
> cavity, re-triangulate, repeat — each step depends on the last. That is a *sequential*
> algorithm; GPU Delaunay is a research topic, not a viewer feature. The GPU-shaped answer
> (Guthe 2005; Schollmeyer & Fröhlich 2009; stencil trimming before both) is to **stop
> polygonizing the trim region entirely**: tessellate the FULL UV rectangle with 69b's dumb
> grid, and decide *per fragment* whether its `(u, v)` is inside the trim loops — a winding
> test, the same rule 2D vector graphics fill with. Concave boundaries and holes cost zero
> special cases: parity handles both. The price is `discard` in the fragment shader — the
> early-Z killer the flat-lines rework taught us to fear — so a compute pre-pass **classifies
> cells** and only the cells the boundary actually crosses ever pay it.

<svg viewBox="0 0 680 168" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a uv grid over a concave trim loop with a hole; cells are classified inside, outside, or boundary; inside cells draw plain, outside cells collapse, boundary cells clip per fragment by winding" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(16,14)">
    <g stroke="#333">
      <path d="M0,0 h128 M0,16 h128 M0,32 h128 M0,48 h128 M0,64 h128 M0,80 h128 M0,96 h128"/>
      <path d="M0,0 v96 M16,0 v96 M32,0 v96 M48,0 v96 M64,0 v96 M80,0 v96 M96,0 v96 M112,0 v96 M128,0 v96"/>
    </g>
    <path d="M 8,88 C 30,10 100,4 122,40 L 112,88 C 80,64 40,72 8,88 Z" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <ellipse cx="56" cy="42" rx="14" ry="9" fill="none" stroke="#6fb3ff" stroke-width="1.6"/>
    <text x="64" y="114" fill="#888" text-anchor="middle">UV domain: loops from mesh_q's OWN discretizer</text>
  </g>
  <g transform="translate(200,20)" font-size="11">
    <rect x="0" y="0" width="12" height="12" fill="#274b27"/><text x="20" y="10" fill="#d7dae0">IN — plain triangles, early-Z intact, no discard</text>
    <rect x="0" y="24" width="12" height="12" fill="#1a1a1a" stroke="#444"/><text x="20" y="34" fill="#d7dae0">OUT — collapsed to degenerate in compute, zero fragments</text>
    <rect x="0" y="48" width="12" height="12" fill="#5b3b1a"/><text x="20" y="58" fill="#d7dae0">BOUNDARY — the only cells that run the winding test + discard</text>
    <text x="0" y="86" fill="#888">crossing parity: odd = inside — concave and holes both correct, no cases</text>
    <text x="0" y="106" fill="#888">boundary cell count grows like the PERIMETER, not the area:</text>
    <text x="0" y="122" fill="#888">the discard tax is O(edge pixels), everything else stays fast-path</text>
  </g>
</svg>

## Files we touch

```
src/shaders/trim_classify.wgsl # NEW — per-CELL in/out/boundary classification (compute)
src/shaders/triangle.wgsl      # trimmed variant fs: winding test on boundary fragments
src/engine/pipelines/build.rs  # build_trim_classify_pipeline + the trimmed triangle pipeline
src/app/scene.rs               # trimmed arm: 69b grid + loop upload; kill_outside in compute
```

## Step 1 — the loops come from the code 69 already trusts

`mesh_q` step 1 discretizes each UV loop (`m_outer_loop`, `m_inner_loops` — 2D `NurbsCurve`s)
into a polygon, adaptively refining where the surface bends. **Reuse it verbatim** — CPU-side,
at upload, the same `disc_loop` produces `Vec<[f64; 2]>` per loop; flatten to f32 pairs and
ship. The GPU consumes *polygons*, never pcurves: all NURBS math stays in one place (69a's
shader), and the loop budget is exactly what the CDT would have used for its boundary.

## Step 2 — cell classification: `src/shaders/trim_classify.wgsl` (NEW)

One invocation per CELL of 69b's grid, once per (re)tessellation:

```wgsl
struct TrimInfo {
    cells_u: u32, cells_v: u32,   // cell grid = vertex grid - 1 each way
    loop_count: u32,
    class_base: u32,              // first cell slot this surface owns in classes[]
    u0: f32, u1: f32, v0: f32, v1: f32,
};
@group(0) @binding(0) var<uniform> trim: TrimInfo;
// loops[] = flat (u,v) pairs; ranges[i] = (first index, count) of loop i. Loop 0 is the
// outer boundary, the rest are holes - winding treats them identically.
@group(0) @binding(1) var<storage, read> loops: array<vec2<f32>>;
@group(0) @binding(2) var<storage, read> ranges: array<vec2<u32>>;
@group(0) @binding(3) var<storage, read_write> classes: array<u32>; // 0 out, 1 in, 2 boundary

// Crossing parity: odd = inside. A hole's loop flips the parity back out - concave outers
// and any number of holes are correct with no special cases, exactly like a 2D vector fill.
fn inside(p: vec2<f32>) -> bool {
    var crossings = 0u;
    for (var l = 0u; l < trim.loop_count; l = l + 1u) {
        let first = ranges[l].x;
        let count = ranges[l].y;
        for (var i = 0u; i < count; i = i + 1u) {
            let a = loops[first + i];
            let b = loops[first + ((i + 1u) % count)];
            if ((a.y > p.y) != (b.y > p.y)) {
                let x = a.x + (p.y - a.y) / (b.y - a.y) * (b.x - a.x);
                if (x > p.x) { crossings = crossings + 1u; }
            }
        }
    }
    return (crossings & 1u) == 1u;
}

// Conservative segment-vs-cell-rect test: bbox reject, then "do the rect's corners straddle
// the segment's line". Conservative is the correct direction - a false BOUNDARY only costs
// that cell the fragment test; a false IN/OUT would draw or drop wrongly.
fn seg_hits_rect(a: vec2<f32>, b: vec2<f32>, lo: vec2<f32>, hi: vec2<f32>) -> bool {
    if (max(a.x, b.x) < lo.x || min(a.x, b.x) > hi.x
     || max(a.y, b.y) < lo.y || min(a.y, b.y) > hi.y) { return false; }
    let d = b - a;
    let s0 = d.x * (lo.y - a.y) - d.y * (lo.x - a.x);
    let s1 = d.x * (lo.y - a.y) - d.y * (hi.x - a.x);
    let s2 = d.x * (hi.y - a.y) - d.y * (hi.x - a.x);
    let s3 = d.x * (hi.y - a.y) - d.y * (lo.x - a.x);
    let all_pos = s0 > 0.0 && s1 > 0.0 && s2 > 0.0 && s3 > 0.0;
    let all_neg = s0 < 0.0 && s1 < 0.0 && s2 < 0.0 && s3 < 0.0;
    return !(all_pos || all_neg);
}

@compute @workgroup_size(8, 8)
fn classify(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= trim.cells_u || gid.y >= trim.cells_v) { return; }
    let du = (trim.u1 - trim.u0) / f32(trim.cells_u);
    let dv = (trim.v1 - trim.v0) / f32(trim.cells_v);
    let lo = vec2<f32>(trim.u0 + du * f32(gid.x), trim.v0 + dv * f32(gid.y));
    let hi = lo + vec2<f32>(du, dv);
    var boundary = false;
    for (var l = 0u; l < trim.loop_count && !boundary; l = l + 1u) {
        let first = ranges[l].x;
        let count = ranges[l].y;
        for (var i = 0u; i < count; i = i + 1u) {
            let a = loops[first + i];
            let b = loops[first + ((i + 1u) % count)];
            if (seg_hits_rect(a, b, lo, hi)) { boundary = true; break; }
        }
    }
    var kind = 0u;
    if (boundary) {
        kind = 2u;
    } else if (inside((lo + hi) * 0.5)) {   // no edge crosses: the centre speaks for the cell
        kind = 1u;
    }
    classes[trim.class_base + gid.y * trim.cells_u + gid.x] = kind;
}
```

(`class` is a WGSL reserved word — hence `kind`. Found the honest way.)

Cost sanity: `cells × loop_edges` at classification time — a 128² grid against a 500-edge loop
is 8M cheap tests, once per re-tessellation, in a dispatch that runs beside 69b's. Never per
frame.

## Step 3 — outside cells die in compute, boundary cells carry a flag

Extend 69b's `tess` entry (or a small follow-up pass): after writing the vertex, each OUT
cell's two triangles are collapsed by writing degenerate indices — or simpler and index-buffer
untouched: the vertex shader reads `classes[]` and any vertex of an OUT cell collapses
outside NDC — the ribbon lane's `dead_vertex` trick, same reason — so the triangle clips to
zero fragments. IN cells draw the plain fast path. BOUNDARY cells pass
their `(u, v)` varying to the fragment stage with a flag.

The fragment side lives in a **trimmed variant** of the triangle pipeline (same shader file,
new entry `fs_trimmed`): boundary fragments run `inside(vec2(u, v))` — the same winding
function, now per pixel — and `discard` when outside. The pipeline is bound only for trimmed
surfaces' arena ranges, so meshes and untrimmed surfaces never pay a branch. `discard` here is
confined exactly like the flat-lane prepass confined blending: to the pixels where geometry
genuinely cannot answer without it.

Honest label: the trim edge is as crisp as the loop polygon — sub-pixel at mesh_q's own
discretization budget — but it is not *watertight geometry*; a boolean export still uses the
CPU CDT. Display and modeling truth split here on purpose, the same split as 69a/69b's pick
proxies.

## Step 4 — the trimmed arm: `src/app/scene.rs`

69's arm called `mesh_render` and pushed a mesh. Now it goes through 69b's reservation
(the underlying `m_surface` gives the grid), plus the loop upload:

In the trimmed branch of the `all_objects()` walk (69 made it the single registration
point), the `mesh_render` push becomes:

```rust
    trimmed_arm => {
        let (gu, gv) = grid_for(&ts.m_surface);
        // 69b's reservation + index grid, verbatim …
        // … then the loops, discretized by the SAME adaptive rule mesh_q uses:
        let (loops, ranges) = trim_loops_upload(ts);   // Vec<[f32;2]>, Vec<(u32,u32)>
        gpu_trims.push(trim_upload(loops, ranges, base_cells, gu - 1, gv - 1));
    }
```

`mesh_render` still runs once, coarse, for the pick proxy — 69's every-map discipline
(`all_objects()`) is untouched; only the display bytes moved.

## What you should see

69's test set is the acceptance: a surface with a **circular hole** and one with a **concave
outer boundary**. Side by side against the CPU build (env-flag the old path), the silhouettes
match to sub-pixel; the interior shading is identical (same 69b vertices). Then the perf claim:
orbit a 256² grid trimmed surface and watch the frame cost sit at the untrimmed surface's cost
— the boundary cells are a thin ring, and only they run the winding test. Zoom until one
boundary cell fills the screen: THAT is the worst case, one cell's fragments against the loop
edges, and it is still flat-frame.

```
Ch 69c: the CDT does not port (sequential by construction) and does not need to: full-rect
        grid + per-fragment winding = trimming as 2D vector fill. Parity handles concave +
        holes with zero cases. Compute classifies cells IN (fast path) / OUT (dead) /
        BOUNDARY (the only discard pixels - perimeter-sized tax). Loops discretized by
        mesh_q's own step-1 code, uploaded once. Display/modeling split stated: crisp trim
        on screen, CDT remains the export/boolean truth.
```

Edited: `shaders/trim_classify.wgsl` (new), `shaders/triangle.wgsl` (`fs_trimmed`),
`pipelines/build.rs` + `mod.rs`, `scene.rs` (trimmed arm reroute + loop upload).

## Next

`69d-gpu-brep.md` — faces are trimmed surfaces, edges are curves, and the CPU mesher's hardest
labor — watertight shared-edge matching — turns out to be a non-requirement for display.
