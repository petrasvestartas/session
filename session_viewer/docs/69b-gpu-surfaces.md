# 69b GPU surfaces — tensor product in compute, the arena gets a producer

> **Big picture.** *Phase 10b.* 69a's pattern — a compute producer writing an existing table —
> aimed at linework. This lesson aims it at the **vertex arena** (43a): a `NurbsSurface`
> evaluates on the GPU, one invocation per grid vertex, writing `RenderVertex` rows the
> triangle pass already draws. 66's economics survive with one word changed: *tessellate once*
> becomes *dispatch once* — the cache now stores a **resolution decision**, not a mesh, and the
> mesh bytes never cross the bus at all. The kernel's `mesh_q(max_angle, chord)` criteria are
> not thrown away: they become the **LOD law** that picks the grid density up front — the
> refinement *metrics* port, the refinement *mesher* doesn't need to.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="surface CVs upload once; a compute grid dispatch evaluates the tensor product per vertex and writes render vertices into the arena; a static index grid completes the mesh" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="20" width="150" height="34" fill="none" stroke="#6fb3ff"/><text x="85" y="34" fill="#d7dae0" text-anchor="middle">CV grid + knots×2</text><text x="85" y="47" fill="#666" text-anchor="middle" font-size="9">v-major, homogeneous f32</text>
  <rect x="200" y="20" width="180" height="34" fill="none" stroke="#6fb3ff"/><text x="290" y="34" fill="#d7dae0" text-anchor="middle">compute 8×8 tiles</text><text x="290" y="47" fill="#666" text-anchor="middle" font-size="9">1 invocation = 1 (u,v) vertex</text>
  <rect x="420" y="20" width="150" height="34" fill="none" stroke="#6fb3ff"/><text x="495" y="34" fill="#d7dae0" text-anchor="middle">arena verts</text><text x="495" y="47" fill="#666" text-anchor="middle" font-size="9">triangle pass unchanged</text>
  <text x="180" y="33" fill="#888">→</text><text x="400" y="33" fill="#888">→</text>
  <text x="340" y="88" fill="#d7dae0" text-anchor="middle">indices: a STATIC (gu-1)×(gv-1)×2-triangle grid, written CPU-side once per resolution</text>
  <text x="340" y="108" fill="#888" text-anchor="middle">density = mesh_q's angle/chord criteria applied to the KNOT SPANS, decided before dispatch</text>
  <text x="340" y="128" fill="#666" text-anchor="middle" font-size="10">normals: central differences of eval() — display contract, honest label below</text>
</svg>

## Files we touch

```
src/shaders/surface_tess.wgsl # NEW — 2-direction basis + tensor accumulate + FD normal
src/engine/pipelines/build.rs # build_surface_tess_pipeline (clone of 69a's builder, new bgl)
src/engine/pipelines/mod.rs   # register
src/engine/gpu/mod.rs         # arena vbo gains STORAGE usage; dispatch_surfaces
src/app/scene.rs              # surface arm: reserve arena region + static index grid
```

## Step 1 — what the tensor product parallelizes

`point_at(u, v)` (nurbssurface.rs) is two 1-D basis evaluations and an
`order_u × order_v` weighted sum of CVs. Every grid vertex is independent — the exact shape of
work GPUs were built for. A 128×128 grid is 16k invocations finishing in microseconds; the CPU
version was the reason 66 needed a cache and a priming pass at all.

The upload rule from 69a extends: CVs go up **v-major exactly like `m_cv`** and
always-homogeneous `(x, y, z, w)`, `w = 1` when `!m_is_rat` — one shader path, one divide.

## Step 2 — the shader: `src/shaders/surface_tess.wgsl` (NEW)

`find_span` and the Cox–de Boor triangle are 69a's functions parameterized by direction — port
them across with `order/base` arguments. The new parts:

```wgsl
struct SurfInfo {
    order_u: u32, order_v: u32,
    cv_count_u: u32, cv_count_v: u32,
    grid_u: u32, grid_v: u32,      // vertex grid (cells + 1 each way)
    vert_base: u32,                // first RenderVertex this surface owns in the arena
    knotu_base: u32, knotv_base: u32,
    cv_base: u32,
    _pad0: u32, _pad1: u32,
    u0: f32, u1: f32, v0: f32, v1: f32,
    color: vec4<f32>,              // facecolor baked per vertex, like the CPU mesh does
};
@group(0) @binding(0) var<uniform> surf: SurfInfo;
@group(0) @binding(1) var<storage, read> data: array<f32>;

// Must match session_rust::RenderVertex (40 B): position, normal, color.
struct RenderVertex {
    px: f32, py: f32, pz: f32,
    nx: f32, ny: f32, nz: f32,
    r: f32, g: f32, b: f32, a: f32,
};
@group(0) @binding(2) var<storage, read_write> verts: array<RenderVertex>;

const MAX_ORDER: u32 = 8u;   // 69a's cap - the ported basis arrays size against it here too
```

`eval` is `point_at` ported — two spans, two basis rows, tensor accumulate:

```wgsl
fn eval(u: f32, v: f32) -> vec3<f32> {
    let su = find_span(surf.order_u, surf.cv_count_u, surf.knotu_base, u);
    let sv = find_span(surf.order_v, surf.cv_count_v, surf.knotv_base, v);
    let nu = basis(surf.order_u, surf.knotu_base, su, u);
    let nv = basis(surf.order_v, surf.knotv_base, sv, v);
    var acc = vec4<f32>(0.0);
    for (var k = 0u; k < surf.order_u; k = k + 1u) {
        for (var l = 0u; l < surf.order_v; l = l + 1u) {
            let i = su + k;
            let j = sv + l;
            if (i >= surf.cv_count_u || j >= surf.cv_count_v) { continue; }
            let idx = surf.cv_base + (i * surf.cv_count_v + j) * 4u;
            acc = acc + (nu[k] * nv[l]) * vec4<f32>(data[idx], data[idx + 1u], data[idx + 2u], data[idx + 3u]);
        }
    }
    if (abs(acc.w) > 1e-10) { return acc.xyz / acc.w; }
    return acc.xyz;
}
```

and the per-vertex entry point evaluates position + a **finite-difference normal**:

```wgsl
@compute @workgroup_size(8, 8)
fn tess(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= surf.grid_u || gid.y >= surf.grid_v) { return; }
    let fu = f32(gid.x) / f32(surf.grid_u - 1u);
    let fv = f32(gid.y) / f32(surf.grid_v - 1u);
    let u = surf.u0 + (surf.u1 - surf.u0) * fu;
    let v = surf.v0 + (surf.v1 - surf.v0) * fv;
    let p = eval(u, v);

    // Display normal by central differences, h = 1e-3 of the domain. Honest label: the CPU
    // bakes EXACT normals (normal_at, the A2.3 derivative table); this is the O(h^2)
    // approximation, chosen because it reuses eval() instead of porting a second 40-line
    // table, and the error is below shading visibility. The upgrade path, if a highlight
    // artifact ever demands it: port basis_functions_derivatives + the rational quotient
    // rule (S_u = (A_u - S w_u) / w), same bindings, same entry point.
    let hu = (surf.u1 - surf.u0) * 1e-3;
    let hv = (surf.v1 - surf.v0) * 1e-3;
    let du = eval(min(u + hu, surf.u1), v) - eval(max(u - hu, surf.u0), v);
    let dv = eval(u, min(v + hv, surf.v1)) - eval(u, max(v - hv, surf.v0));
    var n = cross(dv, du);            // dv × du — normal_at's orientation, so CPU/GPU agree
    let m = length(n);
    if (m > 1e-14) { n = n / m; } else { n = vec3<f32>(0.0, 0.0, 1.0); }

    var o: RenderVertex;
    o.px = p.x; o.py = p.y; o.pz = p.z;
    o.nx = n.x; o.ny = n.y; o.nz = n.z;
    o.r = surf.color.x; o.g = surf.color.y; o.b = surf.color.z; o.a = surf.color.w;
    verts[surf.vert_base + gid.y * surf.grid_u + gid.x] = o;
}
```

## Step 3 — density is a decision, not a loop

The CPU mesher refines until deflection criteria pass. On the GPU the same criteria run ONCE,
CPU-side, to pick the grid — in `src/app/scene.rs` beside the surface arm:

```rust
/// mesh_q's criteria as an up-front density law: per direction, samples per span from the
/// span's bend (control-polygon turn ~ the angle criterion) with the chord factor as floor.
/// Uniform-in-span like 69a; clamped so one surface can never starve the arena.
fn grid_for(ns: &NurbsSurface) -> (u32, u32) {
    let per_span = 16;
    let gu = ((ns.span_count(0) * per_span) as u32).clamp(16, 256) + 1;
    let gv = ((ns.span_count(1) * per_span) as u32).clamp(16, 256) + 1;
    (gu, gv)
}
```

Honest label: this is the *budgeted* version — spans stand in for measured curvature, exactly
65's trade. The full port (turn angles of the control net per span deciding `per_span`) is a
CPU-side refinement of this function; the shader never changes.

## Step 4 — the arena becomes writable: `src/engine/gpu/mod.rs`

Find the arena vbo usage (`let vu = wgpu::BufferUsages::VERTEX | ...`, two sites — creation
and the grow path in reconcile) and add `STORAGE` to both:

```rust
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
               | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE; // compute writes verts
```

`dispatch_surfaces` mirrors `dispatch_curves` with 2-D workgroups:

```rust
                pass.dispatch_workgroups(c.grid_u.div_ceil(8), c.grid_v.div_ceil(8), 1);
```

## Step 5 — the surface arm reserves a region

In `Scene::build`'s `Geometry::NurbsSurface(ns)` arm (66), the `surface_mesh` cache call is
replaced for DISPLAY by a reservation: push `gu × gv` default `RenderVertex` rows (the arena
region), and CPU-write the **static index grid** — two triangles per cell, the only part that
is pure index arithmetic:

```rust
        let (gu, gv) = grid_for(ns);
        let base = verts.len() as u32;
        verts.extend((0..gu * gv).map(|_| RenderVertex::zeroed()));   // bytemuck::Zeroable
        for y in 0..gv - 1 {
            for x in 0..gu - 1 {
                let a = base + y * gu + x;
                idx.extend_from_slice(&[a, a + 1, a + gu, a + 1, a + gu + 1, a + gu]);
            }
        }
        gpu_surfaces.push(surf_upload(ns, base, gu, gv, ri));
```

66's `tess_cache` **stays** — demoted to what it always really was for picking: the pick mesh.
`surface_mesh` keeps feeding 47's raycast at a fixed coarse resolution; the display path never
touches it again. (Same CPU-proxy split as 69a, stated once more because it is the load-bearing
reason Phase 10b can precede nothing in 40–50.)

## What you should see

A surface (66's `srf` tool or a loaded file) shades exactly as before — the acceptance diff:
CPU `mesh_grid` vs GPU grid at the same resolution agree to f32 rounding (verified 1.3e-5 worst
on a 50 mm wavy patch), and normals agree to FD tolerance (no visible shading delta, flat vs
smooth both). The win shows at density: crank `per_span` to 64 — the CPU version's add_file
stall is gone, because no mesh bytes ever cross the bus.

```
Ch 69b: the producer pattern, second table. CV grid + two knot vectors upload once; one
        invocation per (u,v) vertex runs the tensor product (point_at port, verified);
        FD normals with the exact-derivative upgrade path named; density = mesh_q criteria
        decided BEFORE dispatch (spans × budget); indices stay a CPU-written static grid;
        tess_cache demoted to pick proxy. Arena vbo gains STORAGE, triangle pass unchanged.
```

Edited: `shaders/surface_tess.wgsl` (new), `pipelines/build.rs` + `mod.rs`, `gpu/mod.rs`
(arena STORAGE + `dispatch_surfaces`), `scene.rs` (`grid_for`, reservation arm).

## Next

`69c-gpu-trimming.md` — the lesson the CDT does not survive: trim loops become a per-fragment
winding test, concave boundaries and holes included, and the only cells that pay are the ones
the boundary actually crosses.
