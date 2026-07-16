# 62 Iso-curves — the lines that make a surface read

> **Big picture.** *Phase 10.* Shading alone doesn't say "surface" — a smooth gray blob could be
> anything. What makes CAD surfaces legible is their **linework**: the boundary curves and the u/v
> iso-parameter lines (the grid every Rhino surface wears). This is a short lesson because the
> infrastructure already exists: iso-curves are just more `point_at` samples through 31's tube path —
> the same recipe as 60's curve body, evaluated on the surface.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a curved surface patch with boundary curves and interior u v iso lines sampled from point_at" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <path d="M 40,90 C 120,40 220,40 300,80 L 290,110 C 210,72 130,72 55,115 Z" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <path d="M 47,102 C 125,55 217,55 295,95" fill="none" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 105,63 L 96,96" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 175,52 L 172,85" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 245,60 L 248,93" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <text x="170" y="30" fill="#888" text-anchor="middle">boundary (thick) + iso lines (thin)</text>
  <g transform="translate(380,26)">
    <text x="0" y="10" fill="#d7dae0">u-iso: fix u, sample point_at(u, v) over v</text>
    <text x="0" y="30" fill="#d7dae0">v-iso: fix v, sample point_at(u, v) over u</text>
    <text x="0" y="54" fill="#666" font-size="10">boundary = the u/v domain edges (always)</text>
    <text x="0" y="70" fill="#666" font-size="10">interior = quarter lines (¼, ½, ¾) by default</text>
    <text x="0" y="90" fill="#888" font-size="10">tubes protrude (31) → no z-fighting, no bias</text>
  </g>
</svg>

## Files we touch

```
src/app/scene.rs   # iso extraction beside the surface build arm (61); cached with the tessellation
```

## Step 1 — extract: `src/app/scene.rs`

Fix one parameter, sample the other — the boundary is the domain's edge values, interior lines the
quarter fractions. Emitted as `CylinderSegment`s on the surface's row, boundary slightly darker so
edges read over the iso grid:

```rust
const ISO_FRACS: [f64; 3] = [0.25, 0.5, 0.75];     // interior lines per direction
const ISO_SAMPLES: usize = 48;                     // samples along each line

/// Boundary + iso segments for one surface, in LOCAL space (they ride the same instance row and
/// xform as the tessellated body, so transforms move body and lines together for free).
fn surface_linework(ns: &NurbsSurface, ri: u32) -> Vec<CylinderSegment> {
    let (u0, u1) = ns.domain(0).unwrap();
    let (v0, v1) = ns.domain(1).unwrap();
    let mut segs = Vec::new();
    let mut line = |fixed_u: Option<f64>, fixed_v: Option<f64>, color: [f32; 4]| {
        let mut prev: Option<Point> = None;
        for i in 0..=ISO_SAMPLES {
            let t = i as f64 / ISO_SAMPLES as f64;
            let (u, v) = match (fixed_u, fixed_v) {
                (Some(u), None) => (u, v0 + (v1 - v0) * t),
                (None, Some(v)) => (u0 + (u1 - u0) * t, v),
                _ => unreachable!(),
            };
            if let Some(p) = ns.point_at(u, v) {
                if let Some(q) = &prev {
                    segs.push(CylinderSegment { p0: q.to_f32(), radius: 0.0, p1: p.to_f32(),
                                                instance_id: ri, color });
                }
                prev = Some(p);
            }
        }
    };
    let edge = [0.10, 0.10, 0.10, 1.0];            // boundary: near-black, like mesh edges (31)
    // interior: lighter, reads as structure not silhouette
    let iso  = [0.35, 0.35, 0.38, 1.0];
    line(Some(u0), None, edge); line(Some(u1), None, edge);
    line(None, Some(v0), edge); line(None, Some(v1), edge);
    for f in ISO_FRACS {
        line(Some(u0 + (u1 - u0) * f), None, iso);
        line(None, Some(v0 + (v1 - v0) * f), iso);
    }
    segs
}
```

(`ns.domain(dir)` returns `Option<(f64, f64)>` and `point_at(u, v)` → `Option<Point>` — the same
calls the kernel's own BRep bounding-box sampler uses, so the unwraps above are safe for any valid
surface; keep the `if let` guards regardless.)

## Step 2 — cache it with the mesh: `src/app/scene.rs`

Sampling 10 lines × 48 points is cheap but not free — and it changes exactly when the tessellation
does. Widen 61's cache entry to carry both:

```rust
    pub tess_cache: HashMap<String, (Mesh, Vec<CylinderSegment>)>,   // was: HashMap<String, Mesh>
    // 61's surface_mesh fills both: (ns.mesh(), surface_linework(ns, ri))
```

The build arm (61 Step 2) appends the cached linework to `segments` after `push_mesh` — note
`push_mesh` already emits the *mesh's* edge tubes; for surfaces, suppress those (pass a flag or use a
`push_mesh_faces_only` variant) so the surface wears its **iso lines**, not its tessellation
triangles' wireframe. That substitution — parameter-space lines instead of triangle edges — is
exactly what visually separates "a surface" from "a mesh" in every CAD viewer.

## Step 3 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- The surface now wears a dark boundary and a light 3×3 interior grid that **follows the curvature**
  (the lines are geodesics of the parameterization, not screen-space overlays — orbit and watch them
  foreshorten correctly).
- No tessellation wireframe on the surface — triangle edges are the mesh look, iso lines are the
  surface look, and the difference is instantly readable next to a real mesh box.
- Zoom to a grazing angle: **no z-fighting flicker** between lines and skin — the tubes protrude
  (31's whole design); there is no depth-bias knob to tune because none is needed.
- Gumball-drag the surface: lines and skin move as one (same row, same xform — local-space linework,
  Step 1's parenthetical) and the perf HUD stays flat (cached, 61's rule).

## Recap

```
Ch 61: surfaces — tessellate once, matrices forever.
Ch 62: LINEWORK. surface_linework: boundary (domain edges, near-black) + interior iso lines (¼ ½ ¾
       per direction, lighter) sampled point_at along one fixed parameter — 48 samples/line → 31's
       tubes, LOCAL space on the surface's own row (transforms carry them free). Cached WITH the
       tessellation (one invalidation story). Surfaces suppress push_mesh's triangle-edge tubes —
       iso lines are what makes a surface read as a surface, not a mesh. Tubes protrude → no
       z-fight, no bias. Short lesson, old infrastructure — that's the compounding paying out.
```

Edited: `app/scene.rs` (`surface_linework`, widened `tess_cache`, faces-only mesh push for surfaces).

## Next

`63-brep.md` — the boundary representation: multiple faces + shared edges as **one object**. It's
been half-supported since 34 (tessellated and drawn); now it gets the 61 treatment — cached
tessellation, matrix-only transforms — and its edge curves drawn properly, so picking any face
selects the whole solid.
