# 47 Sub-object picking — vertex, edge, or face

> **Big picture.** *Phase 7.* Object-level picking is enough to *select*; editing needs the *part* —
> the vertex you drag, the edge you bevel, the face you extrude. Sub-object resolution is what makes
> the gumball (52+) and control-point editing (77) possible later; the screen-pixel radius trick it
> introduces is also exactly how 44 picks lines and points at all.

42 answers *which mesh*. Editing needs *which part*: drag a **vertex**, bevel an **edge**, extrude a
**face**. From the same click, this lesson resolves the hit down to one sub-object and returns a
`SubHit { row, guid, kind }` the gumball and edit tools act on (the sub-object key rides *inside* `kind`).

The resolution is **screen-space, with a priority order**. A vertex is a point and an edge is a line —
in 3D a ray almost never hits either exactly, so "did I click the vertex?" is really "is the vertex
within a few *pixels* of the cursor?" Project the candidates to the screen, measure pixel distance, and
prefer the most specific: **vertex beats edge beats face**. If the cursor is within the radius of a
vertex, that's the pick; else the nearest edge; else the face the ray landed on.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the hit mesh's vertices and edges are projected to the screen; if the cursor is within pixel radius of a vertex it wins, else the nearest edge, else the face" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <circle cx="120" cy="70" r="4" fill="#6fb3ff"/><circle cx="300" cy="50" r="4" fill="#6fb3ff"/><circle cx="260" cy="120" r="4" fill="#6fb3ff"/>
  <line x1="120" y1="70" x2="300" y2="50" stroke="#4a4a4a"/><line x1="300" y1="50" x2="260" y2="120" stroke="#4a4a4a"/><line x1="260" y1="120" x2="120" y2="70" stroke="#4a4a4a"/>
  <circle cx="128" cy="66" r="14" fill="none" stroke="#6fb3ff" stroke-dasharray="3 3"/><text x="128" y="40" fill="#d7dae0" text-anchor="middle">cursor near vertex → VERTEX</text>
  <text x="120" y="86" fill="#666">v</text>
  <g transform="translate(360,0)">
    <text x="20" y="30" fill="#888">priority (most specific wins):</text>
    <text x="30" y="52" fill="#d7dae0">1. vertex — cursor within R px of a projected vertex</text>
    <text x="30" y="72" fill="#d7dae0">2. edge — cursor within R px of a projected edge segment</text>
    <text x="30" y="92" fill="#d7dae0">3. face — the face the ray actually hit</text>
    <text x="20" y="122" fill="#666">R ≈ 8 px; same screen-space test 44 uses for thin geometry</text>
  </g>
</svg>

## Files we touch

```
src/engine/pick.rs   # project_to_screen(view_proj, origin, world_pt, viewport) — the forward of 41
src/app/pick.rs      # SubKind { Vertex/Edge/Face }, SubHit { row, guid, kind }
src/app/scene.rs     # resolve_subobject(guid, hit, cursor, view_proj, origin, viewport) → SubHit
```

## Step 1 — project a world point to the screen: `src/engine/pick.rs`

The exact inverse of 45's unproject: world → camera-relative (`− origin`) → clip (`view_proj`) → NDC →
pixels. Returns `None` when the point is behind the camera (`w ≤ 0`), so off-screen vertices can't win.

```rust
/// World point → screen pixel, or None if behind the camera. `viewport` = (x, y, w, h).
pub fn project_to_screen(view_proj: &Xform, origin: &Point, p: &Point,
                         viewport: (f64, f64, f64, f64)) -> Option<(f64, f64)> {
    let m = view_proj.to_cols();                                  // column-major m[col][row]
    // world → camera-relative
    let v = [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2], 1.0];
    let row = |r: usize| m[0][r]*v[0] + m[1][r]*v[1] + m[2][r]*v[2] + m[3][r]*v[3];
    let w = row(3);
    if w <= 0.0 { return None; }                                 // behind the eye
    let (ndc_x, ndc_y) = (row(0)/w, row(1)/w);
    let (vx, vy, vw, vh) = viewport;
    // NDC → px (y flips back)
    Some((vx + (ndc_x * 0.5 + 0.5) * vw, vy + (0.5 - ndc_y * 0.5) * vh))
}
```

## Step 2 — the result type: `src/app/pick.rs`

```rust
#[derive(Debug)]            // Step 4 logs it with {:?}
pub enum SubKind {
    Vertex(usize),          // vertex key
    Edge(usize, usize),     // the two vertex keys, kernel edge order
    Face(usize),            // face key
}

pub struct SubHit {
    pub row: u32,       // carried through from PickHit, like the guid — downstream tools resolve
                        // the doc (doc_of_row) and the highlight off the row, not the guid
    pub guid: String,
    pub kind: SubKind,
}
```

## Step 3 — resolve, most-specific first: `src/app/scene.rs`

Given 46's `PickHit` (the row + guid + the world hit point), walk the hit mesh's vertices then edges in
screen space, and fall back to the face containing the hit. `R_PX` is the click slop — ~8 px feels right
and matches 48's thin-geometry radius. Widen `scene.rs`'s imports first:

```rust
use crate::app::pick::{PickHit, SubHit, SubKind};
use crate::engine::pick::project_to_screen;
```

```rust
const R_PX: f64 = 8.0;

impl Scene {
    pub fn resolve_subobject(&self, hit: &PickHit, cursor: (f64, f64), view_proj: &Xform,
                             origin: &Point, viewport: (f64, f64, f64, f64)) -> Option<SubHit> {
        let d = self.doc_of_row(hit.row);          // 46's PickHit carries the row → owning doc
        let m = match self.docs[d].session.lookup.get(&hit.guid) {
            Some(session_rust::Geometry::Mesh(m)) => m,   // same doc-resolved lookup as 46's pick_ray
            _ => return None,   // BRep sub-objects (trims/edges) are their own lesson
        };
        // ONE frame lookup, hoisted out of the per-vertex loops below (read-only, so it can
        // coexist with the `m` borrow — no clone needed here, unlike 46's mutable path).
        let frame = self.placed_frame(hit.row);
        let world = |vk: usize| -> Option<Point> {
            // local → world through the row's placed frame (kernel gap #5's transform_point)
            Some(frame.transform_point(&m.vertex_point(vk)?))
        };
        let px = |vk: usize| world(vk)
            .and_then(|p| project_to_screen(view_proj, origin, &p, viewport));
        let dist2 = |a: (f64, f64)| {
            let dx = a.0 - cursor.0;
            let dy = a.1 - cursor.1;
            dx*dx + dy*dy
        };

        // 1) nearest VERTEX within R_PX
        let mut best_v: Option<(usize, f64)> = None;
        for &vk in m.vertex.keys() {
            if let Some(s) = px(vk) { let d = dist2(s);
                if d <= R_PX*R_PX && best_v.map_or(true, |(_, bd)| d < bd) {
                    best_v = Some((vk, d)); } }
        }
        if let Some((vk, _)) = best_v {
            return Some(SubHit { row: hit.row, guid: hit.guid.clone(), kind: SubKind::Vertex(vk) }); }

        // 2) nearest EDGE within R_PX (point-to-segment in screen space)
        let mut best_e: Option<((usize, usize), f64)> = None;
        for (a, b) in m.edges() {
            if let (Some(pa), Some(pb)) = (px(a), px(b)) { let d = seg_dist2(cursor, pa, pb);
                if d <= R_PX*R_PX && best_e.map_or(true, |(_, bd)| d < bd) {
                    best_e = Some(((a, b), d)); } }
        }
        if let Some(((a, b), _)) = best_e {
            return Some(SubHit { row: hit.row, guid: hit.guid.clone(), kind: SubKind::Edge(a, b) }); }

        // 3) FACE the ray landed on (local point-in-polygon over the mesh faces)
        let inv = frame.inverse()?;                               // the hoisted placed frame again
        let local = inv.transform_point(&hit.point);              // world hit → local
        let fk = face_containing(m, &local)?;
        Some(SubHit { row: hit.row, guid: hit.guid.clone(), kind: SubKind::Face(fk) })
    }
}

/// Squared screen distance from `p` to segment `a`–`b`.
fn seg_dist2(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (abx, aby) = (b.0 - a.0, b.1 - a.1);
    let len2 = abx*abx + aby*aby;
    let t = if len2 > 0.0 {
        (((p.0 - a.0)*abx + (p.1 - a.1)*aby) / len2).clamp(0.0, 1.0)
    } else { 0.0 };
    let (cx, cy) = (a.0 + t*abx, a.1 + t*aby);
    let (dx, dy) = (p.0 - cx, p.1 - cy);
    dx*dx + dy*dy
}
```

`face_containing(m, local_point)` is a small helper: for each `m.faces()`, fan-triangulate its
`face_vertices` (via `m.vertex_point`) and test the point against each triangle in the face's plane —
the first face that contains it wins. It's the one piece 46's ray-cast couldn't hand us — the kernel's
`triangle_bvh_ray_cast` returns the hit *point*, not the hit *face* — so we recover the face here:

```rust
/// The mesh face whose polygon contains `p` (mesh-LOCAL). Fan-triangulates each face and does a
/// same-side point-in-triangle test in 3D — `p` came from the ray hit, so it lies on the face plane.
/// (The kernel already triangulates faces for rendering; expose that cache and reuse it to skip the fan.)
fn face_containing(m: &Mesh, p: &Point) -> Option<usize> {
    for fk in m.faces() {
        let vs = match m.face_vertices(fk) { Some(v) if v.len() >= 3 => v, _ => continue };
        let pts: Vec<Point> = vs.iter().filter_map(|&vk| m.vertex_point(vk)).collect();
        if pts.len() < 3 { continue; }
        for i in 1..pts.len() - 1 {                       // fan: (pts[0], pts[i], pts[i+1])
            if point_in_tri(p, &pts[0], &pts[i], &pts[i + 1]) { return Some(fk); }
        }
    }
    None
}

/// p inside triangle abc if it's on the inner side of all three edges, measured against the triangle's
/// OWN normal — so a small off-plane error from the ray hit is ignored (only the in-plane sign matters).
fn point_in_tri(p: &Point, a: &Point, b: &Point, c: &Point) -> bool {
    let sub = |u: &Point, v: &Point| [u[0]-v[0], u[1]-v[1], u[2]-v[2]];
    let cross = |u: [f64;3], v: [f64;3]| [u[1]*v[2]-u[2]*v[1], u[2]*v[0]-u[0]*v[2], u[0]*v[1]-u[1]*v[0]];
    let dot = |u: [f64;3], v: [f64;3]| u[0]*v[0] + u[1]*v[1] + u[2]*v[2];
    let n = cross(sub(b, a), sub(c, a));                  // triangle normal
    let e0 = dot(n, cross(sub(b, a), sub(p, a)));
    let e1 = dot(n, cross(sub(c, b), sub(p, b)));
    let e2 = dot(n, cross(sub(a, c), sub(p, c)));
    (e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0) || (e0 <= 0.0 && e1 <= 0.0 && e2 <= 0.0)
}
```

> **Why screen space, not world.** In world units, "within 8 px" would be a distance that shrinks as you
> zoom in and balloons as you zoom out — a vertex you can't hit up close and a whole face you snap to from
> far away. Pixels are the unit the user actually aims in, so the radius test *must* be post-projection.
> The same reason the gumball's handles are a fixed pixel size (later).

## Step 4 — wire it + verify

Call `resolve_subobject` right after 46's `pick_ray` succeeds — in `State::on_left_click`,
replace 46's `match self.scene.pick_ray(&ray) { … }` with:

```rust
    if let Some(hit) = self.scene.pick_ray(&ray) {
        if let Some(sub) = self.scene.resolve_subobject(&hit, self.cursor, &vp, &origin, viewport) {
            log::info!("sub-pick {}: {:?}", sub.guid, sub.kind);
        }
    }
```

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **Hover a corner** → `Vertex(k)`. Move a few px onto an edge → `Edge(a,b)`. Move into the face
  interior → `Face(k)`. The transitions should feel like Rhino's sub-object highlight.
- **Zoom out until the box is tiny** → clicking anywhere still resolves (usually `Vertex`/`Edge`, since
  everything is within 8 px of *something*), and zooming in re-separates them. That zoom-independence is
  the screen-space radius working; a world-space radius would break here.
- A `#[cfg(test)]` on `seg_dist2` (endpoints, midpoint, perpendicular foot) and `face_containing` (a
  point inside vs. outside a known quad) pins the geometry without a browser.

## Recap

```
Ch 46: ray-cast meshes → which mesh + world hit point.
Ch 47: SUB-OBJECT. Resolve the hit to vertex / edge / face, SCREEN-SPACE, most-specific-first.
       project_to_screen (the forward of 41: world −origin → view_proj → NDC → px, None if
       behind). Priority: (1) nearest projected VERTEX within R_PX (~8) → Vertex(key); (2) else
       nearest projected EDGE by screen point-to-segment → Edge(a,b); (3) else the FACE the ray
       hit, recovered by transforming the world hit to local and point-in-polygon over
       m.faces() — because the kernel ray-cast returns the point, not the face. All projection
       goes through the row's placed frame (hoisted once, before the loops). Returns
       SubHit{row, guid, kind} (the key lives inside kind). Pixel radius (not world) so the test is zoom-independent — the
       unit the user aims in. Vertex/edge screen-proximity is the same trick 44 uses to pick
       1D/0D geometry that a ray can't hit exactly.
```

Edited: `engine/pick.rs` (`project_to_screen` — forward projection), `app/pick.rs` (`SubKind`, `SubHit`),
`app/scene.rs` (`resolve_subobject` vertex→edge→face, `seg_dist2`, `face_containing`).

## Next

`48-pick-thin-geometry.md` — Lines, polylines, and points are 1D and 0D: a ray passes *through* them,
never *hits* them. 44 picks them by the same screen-space radius used for vertices here — ray↔segment and
ray↔point distance with a `pick_radius` floor in pixels — and settles the **solid-vs-thin priority**: a
line lying on a mesh face shouldn't steal the click from the face at equal depth.
