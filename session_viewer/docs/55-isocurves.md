# 55 Iso-curves — the lines that make a surface read

> **Big picture.** *Phase 4b.* Shading alone doesn't say "surface" — a smooth gray blob could be
> anything. What makes CAD surfaces legible is their **linework**: the boundary curves and the u/v
> iso-parameter lines (the grid every Rhino surface wears). Today a surface wears the WRONG
> linework: `push_mesh` gives it the mesh treatment — tubes along its tessellation's triangle
> edges — which is exactly the look that says "mesh". This is a short lesson because the
> infrastructure already exists: iso lines are more `point_at` samples through 31's tube path,
> cached WITH the tessellation so they share one lifecycle.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a curved surface patch with boundary curves and interior u v iso lines sampled from point_at" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <path d="M 40,90 C 120,40 220,40 300,80 L 290,110 C 210,72 130,72 55,115 Z" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <path d="M 60,100 C 130,55 220,52 292,88" fill="none" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 50,105 C 125,62 215,58 296,95" fill="none" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 110,63 L 100,98" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 200,52 L 195,86" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <text x="170" y="118" fill="#888" text-anchor="middle" font-size="10">boundary (dark) + u/v iso lines (light) — parameter-space lines, not triangle edges</text>
  <g transform="translate(380,18)">
    <text x="0" y="14" fill="#d7dae0">iso lines = point_at along one FIXED parameter</text>
    <text x="0" y="32" fill="#666" font-size="10">u = ¼, ½, ¾ (v sweeps) and v = ¼, ½, ¾ (u sweeps)</text>
    <text x="0" y="56" fill="#d7dae0">cached with the tessellation — one lifecycle</text>
    <text x="0" y="74" fill="#666" font-size="10">tubes in the SOLID lane: they protrude, no z-fight</text>
  </g>
</svg>

## Files we touch

```
src/app/scene.rs   # surface_linework; tess_cache widens to (mesh, linework);
                   # push_mesh gains an Edges flag so surfaces can suppress triangle wireframe
```

## Step 1 — extract: `surface_linework`

Boundary (the domain's four edges, near-black) plus a light 3×3 interior grid, sampled
`point_at(u, v)` along one fixed parameter. Kernel honesty: on a surface, `domain(dir)` and
`point_at(u, v)` return `Option` — a degenerate surface answers `None`, and the extractor just
yields no lines for it. Beside `nurbscurve_to_segments`:

```rust
const ISO_FRACS: [f64; 3] = [0.25, 0.5, 0.75]; // interior lines per direction
const ISO_SAMPLES: usize = 48;                 // samples per line

/// Boundary + iso lines for one surface, LOCAL space, instance stamped at push time.
fn surface_linework(s: &NurbsSurface) -> Vec<CylinderSegment> {
    let mut out = Vec::new();
    let (Some((u0, u1)), Some((v0, v1))) = (s.domain(0), s.domain(1)) else { return out };
    let dark  = pack_rgba([0.05, 0.05, 0.05, 1.0]);
    let light = pack_rgba([0.35, 0.35, 0.35, 1.0]);
    // one polyline along a fixed u (v sweeps) or fixed v (u sweeps)
    let mut line = |fix: f64, u_fixed: bool, color: u32| {
        let mut prev: Option<[f32; 3]> = None;
        for i in 0..=ISO_SAMPLES {
            let t = i as f64 / ISO_SAMPLES as f64;
            let (u, v) = if u_fixed { (fix, v0 + (v1 - v0) * t) } else { (u0 + (u1 - u0) * t, fix) };
            let Some(p) = s.point_at(u, v) else { prev = None; continue };
            let p = p.to_f32();
            if let Some(q) = prev {
                out.push(CylinderSegment { p0: q, radius: 0.0, p1: p,
                                           instance_id: 0, color, facing: FACING_UNKNOWN });
            }
            prev = Some(p);
        }
    };
    line(u0, true, dark); line(u1, true, dark);   // boundary
    line(v0, false, dark); line(v1, false, dark);
    for f in ISO_FRACS {                          // interior grid
        line(u0 + (u1 - u0) * f, true, light);
        line(v0 + (v1 - v0) * f, false, light);
    }
    out
}
```

`radius: 0.0` = the screen-constant global pen (31's convention), and the segments go to the
**pipes** lane — the SOLID lane — deliberately: flat ribbons would z-fight the skin they lie on,
while tubes protrude by construction (31's whole design). No depth-bias knob exists because none
is needed.

## Step 2 — cache it with the mesh: widen the entry

Linework shares the tessellation's lifecycle exactly (born with the shape, dies on reshape), so it
shares the cache entry. **Find** (44):

```rust
    pub tess_cache: HashMap<String, Mesh>, // per-guid COLORED tessellation; BReps join in 46
```

**Replace with:**

```rust
    /// Per-guid (colored tessellation, linework) — both LOCAL, both born once per shape.
    /// Linework carries instance_id 0 in the cache; the walk stamps the real row at push.
    pub tess_cache: HashMap<String, (Mesh, Vec<CylinderSegment>)>,
```

and in the surface arm, the closure fills both halves and the push site consumes both — the mesh
now passes `Edges::Suppress` (Step 3), and the linework is stamped with this walk's row:

```rust
                Geometry::NurbsSurface(s) => {
                    let (sm, lw) = self.tess_cache.entry(guid.clone()).or_insert_with(|| {
                        let mut m = s.mesh();
                        if let Some(c) = s.facecolors.first() {
                            m.set_objectcolor(c.clone());
                        }
                        (m, surface_linework(s))
                    });
                    let b = push_mesh(
                        sm,
                        ri,
                        vb,
                        &mut t.verts,
                        &mut t.vids,
                        &mut t.idx,
                        &mut t.pipes,
                        &mut t.spheres,
                        Edges::Suppress          // iso lines below, not triangle wireframe
                    );
                    t.pipes.extend(lw.iter().map(|seg| {
                        let mut seg = *seg;      // cache holds instance 0 — stamp THIS row
                        seg.instance_id = ri;
                        seg
                    }));
                    t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, sm.number_of_vertices()));
                }
```

(Why instance 0 in the cache: the same surface guid can be walked into different rows across
rebuilds — rows are positional — so the cached copy must stay row-free and the push site owns
the stamp.)

## Step 3 — `Edges::Suppress`: one flag, never a fork

`push_mesh` currently ALWAYS runs its edge machinery (the `edges_with_colors` walk, width map,
tubes, vertex dots — unless a LOD or print-fill early-out fires). Surfaces need the vertex/index
push and the bounds, and none of the decoration. One new parameter, never a copied function:

```rust
pub enum Edges { Draw, Suppress }
```

Widen the signature (`edge_mode: Edges` last — the local `let edges = m.edges_with_colors();`
already owns the short name), and **find** the dense-mesh early-out:

```rust
    if rm.indices.len() / 3 > MESH_RAW_MIN {
        return None;
    }
```

**Add directly above it:**

```rust
    // Surfaces (44/46/47) draw parameter-space linework instead of triangle wireframe -
    // suppress the decoration, KEEP the bounds (the dense early-out below predates the
    // bounds return and still drops them; suppression must not repeat that).
    if matches!(edge_mode, Edges::Suppress) {
        return local_bounds;
    }
```

Every existing call site (the Mesh arm, the BRep arms, the Element arms) passes `Edges::Draw` —
mechanical, the compiler lists them. Note what the comment calls out: the dense-LOD path
`return None`s even though `local_bounds` is computed — dense meshes lose their recorded bounds
today. Suppression deliberately does better; the dense path keeps its behavior (changing it is a
one-word fix the day something needs dense-mesh bounds).

## Step 4 — verify

```bash
cargo check --target wasm32-unknown-unknown --lib
```

- The 44 surface fixture now wears a dark boundary and a light 3×3 interior grid that **follows
  the curvature** (the lines are lines OF the parameterization — orbit and watch them foreshorten
  correctly), and **no tessellation wireframe** — put a mesh box beside it: triangle edges say
  "mesh", iso lines say "surface", instantly.
- Zoom to a grazing angle: **no z-fighting flicker** between lines and skin (tubes protrude —
  31's design; there is no bias to tune).
- The linework logs no extraction cost on a second walk of the same guid — cached with the
  tessellation, one lifecycle.

## Recap

```
Ch 44: surfaces — tessellate once, matrices forever.
Ch 45: LINEWORK. surface_linework: boundary (domain edges, near-black) + interior iso lines
       (¼ ½ ¾ per direction, lighter), point_at along one fixed parameter, 48 samples/line —
       Option-honest (degenerate surfaces yield nothing) → tables.pipes (SOLID lane: flat
       segments would z-fight the skin; tubes protrude), LOCAL space, instance 0 in cache and
       the row stamped at push (rows are positional across rebuilds). Cached WITH the
       tessellation — tess_cache widens to (Mesh, Vec<CylinderSegment>), one lifecycle, one
       eviction story. push_mesh gains Edges::Suppress — a FLAG, never a forked function —
       returning local_bounds (and exposing that the dense-LOD path drops bounds today; known,
       left). Density knobs are constants; scale them by span counts (43's rule) when they
       become settings — a density change is a cache clear away, never a per-frame cost.
```

Edited: `app/scene.rs` (`surface_linework`, widened `tess_cache`, `Edges` flag on `push_mesh`,
`Edges::Draw` at the existing call sites).

## Next

`56-brep.md` — the boundary representation: multiple faces + shared edges as **one object**.
It's been drawing since the first walk (`b.mesh()` → `push_mesh`, three call sites) — now it
gets the 44 treatment: cached tessellation — and its edge curves drawn properly from the
kernel's real curve network.
