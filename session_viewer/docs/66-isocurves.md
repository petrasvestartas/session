# 66 Iso-curves — the lines that make a surface read

> **Big picture.** *Phase 10.* Shading alone doesn't say "surface" — a smooth gray blob could be
> anything. What makes CAD surfaces legible is their **linework**: the boundary curves and the u/v
> iso-parameter lines (the grid every Rhino surface wears). This is a short lesson because the
> infrastructure already exists: iso-curves are just more `point_at` samples through 31's tube path —
> the same recipe as 64's curve body, evaluated on the surface.

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
src/app/scene.rs   # iso extraction beside the surface build arm (65); cached with the tessellation
```

## Step 1 — extract: `src/app/scene.rs`

Fix one parameter, sample the other — the boundary is the domain's edge values, interior lines the
quarter fractions. Emitted as `CylinderSegment`s destined for the **solid pipe lane**
(`tables.pipes` — 3D linework must protrude off the skin; the flat `segments` lane draws at surface
depth and would z-fight it), boundary slightly darker so edges read over the iso grid. Cached
linework carries `instance_id: 0` — the push site stamps the real row:

```rust
const ISO_FRACS: [f64; 3] = [0.25, 0.5, 0.75];     // interior lines per direction
const ISO_SAMPLES: usize = 48;                     // samples along each line

/// Boundary + iso segments for one surface, in LOCAL space (they ride the same instance row and
/// placed frame as the tessellated body, so transforms move body and lines together for free).
/// instance_id stays 0 in the cache — stamped with the real row at push time.
fn surface_linework(ns: &NurbsSurface) -> Vec<CylinderSegment> {
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
                                                instance_id: 0, color });
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
does. Widen 65's cache entry to carry both, and widen `surface_mesh` (61 Step 1) to fill both halves
on first use. **Find 65's `tess_cache` field and `surface_mesh`, replace with:**

```rust
    pub tess_cache: std::collections::HashMap<String, (Mesh, Vec<CylinderSegment>)>,   // was: <String, Mesh>

    /// 65's surface_mesh, widened: the cache entry is now (mesh, linework) — both built once.
    fn surface_mesh(&mut self, guid: &str) -> Option<&(Mesh, Vec<CylinderSegment>)> {
        if !self.tess_cache.contains_key(guid) {
            let ns = self.docs.iter().find_map(|d| match d.session.lookup.get(guid) {
                Some(Geometry::NurbsSurface(ns)) => Some(ns),   // a lookup variant — no
                _ => None,                                      // collection scan needed
            })?;
            self.tess_cache.insert(guid.to_string(), (ns.mesh(), surface_linework(ns)));
        }
        self.tess_cache.get(guid)
    }
```

Now the surface arm in `add_file`'s walk (61 Step 2) unpacks the tuple, pushes the mesh **faces
only**, and appends the cached iso lines to `tables.pipes` — the **solid** lane, stamping the row as
they go. **Find 65's surface arm in `add_file` and replace its body with:**

```rust
                Geometry::NurbsSurface(ns) => {
                    t.objects.push((placed, surface_color(ns), flags));
                    // warmed by 65's priming pass (top of add_file — see below)
                    if let Some((m, linework)) = self.tess_cache.get(&guid) {
                        // NO edge tubes …
                        push_mesh_faces_only(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                            &mut t.spheres);
                        // …iso lines instead — SOLID lane, real row stamped now
                        t.pipes.extend(linework.iter()
                            .map(|s| CylinderSegment { instance_id: ri, ..*s }));
                    }
                }
```

Pushing the linework into `segments` instead would put it in the flat lane — ribbons drawn *at* the
skin's depth, z-fighting it at every grazing angle and falsifying the no-bias claim this lesson
makes below. 3D surface linework always rides `pipes`.

> **Keep 65's priming pass — it runs at the top of `add_file` now** (the walk lives there since
> 36). The walk loop reads the *warmed* cache; it can't call `surface_mesh` itself (that's
> `&mut self` — the E0502 case 61 solved with a separate pass). 65's
> `for guid in &ns_guids { self.surface_mesh(guid); }` runs just before the walk, unchanged — and
> it's exactly why the cache stores `instance_id: 0`: the priming pass runs before this doc's
> objects have rows (and can't know rows for docs not yet walked at all), so only the push site can
> stamp `ri`.

(Two one-word ripples from the widened cache type: 65's pick arm now casts against
`self.tess_cache[guid].0`, and 65's world-box reads — if you pointed any at the
cache — gain the same `.0`.)

`push_mesh_faces_only` is `push_mesh(m, ri, verts, vids, idx, pipes, spheres)` with one thing
removed: drop the `pipes` parameter and the loop that pushes edge `CylinderSegment`s — keep the
arena vertex/index work, the `vwidth` pass, and the dot loop verbatim. The `vwidth` map is built
from the edge widths and *gates* the dot loop — delete it along with the edge loop and every
boundary dot disappears. Surfaces then wear their **iso lines**, not their tessellation
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
- Gumball-drag the surface: lines and skin move as one (same row, same placed frame — local-space
  linework, Step 1's parenthetical) and the perf HUD stays flat (cached, 65's rule).

## Recap

```
Ch 65: surfaces — tessellate once, matrices forever.
Ch 66: LINEWORK. surface_linework: boundary (domain edges, near-black) + interior iso lines (¼ ½ ¾
       per direction, lighter) sampled point_at along one fixed parameter — 48 samples/line →
       tables.pipes (the SOLID lane — flat segments would z-fight the skin), LOCAL space, cached
       with instance_id 0 and the row stamped at push (the priming pass predates rows). Cached WITH
       the tessellation (one invalidation story). Surfaces suppress push_mesh's triangle-edge tubes —
       iso lines are what makes a surface read as a surface, not a mesh. Tubes protrude → no
       z-fight, no bias. Short lesson, old infrastructure — that's the compounding paying out.
```

Edited: `app/scene.rs` (`surface_linework`, widened `tess_cache`, faces-only mesh push for surfaces).

## Next

`67-brep.md` — the boundary representation: multiple faces + shared edges as **one object**. It's
been half-supported since 34 (tessellated and drawn); now it gets the 61 treatment — cached
tessellation, matrix-only transforms — and its edge curves drawn properly, so picking any face
selects the whole solid.
