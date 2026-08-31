# 81 Snapping — drawing becomes precise

> **Big picture.** *Phase 9 closes.* Freehand clicks are never exact — and CAD is nothing if not
> exact. Snapping pulls the cursor's point to the nearest *meaningful* location: a line's endpoint, a
> mesh vertex, a grid crossing. The architecture note is the important part: snap lives in **one
> place** — the Get-loop's point acquisition — so `line`, `polyline`, `rect`, `box`, and every future
> tool became snap-aware the moment this lesson lands, with zero tool changes. That's the reward for
> routing all point input through one function since 53.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="candidates within a pixel radius of the cursor are ranked endpoint before grid before free; the winner replaces the raw point and a marker shows it" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <line x1="40" y1="90" x2="240" y2="40" stroke="#6fb3ff" stroke-width="2"/>
  <circle cx="240" cy="40" r="10" fill="none" stroke="#d7dae0" stroke-width="1.5"/>
  <rect x="234" y="34" width="12" height="12" fill="none" stroke="#e0b040" stroke-width="1.6"/>
  <circle cx="252" cy="52" r="3" fill="none" stroke="#888"/><text x="262" y="56" fill="#888">raw cursor</text>
  <text x="240" y="20" fill="#e0b040" text-anchor="middle">End</text>
  <g transform="translate(430,20)">
    <text x="0" y="14" fill="#888">priority (nearest wins within rank):</text>
    <text x="10" y="36" fill="#d7dae0">0 endpoint / vertex</text>
    <text x="10" y="54" fill="#d7dae0">6 grid intersection</text>
    <text x="10" y="72" fill="#d7dae0">— else: the raw point</text>
    <text x="0" y="98" fill="#666" font-size="10">radius in PIXELS (56's rule) — zoom-independent</text>
  </g>
</svg>

## Files we touch

```
# NEW — SnapKind + SnapContext + snap(scene, raw, cursor, ctx) → (Point, Option<SnapKind>)
src/app/snap.rs
src/state.rs       # cursor_world_point() consults snap; marker glyph; `snap` CLI toggle
src/app/commands.rs # `snap` on|off verb registration (Step 2)
```

## Step 1 — kinds and candidates: `src/app/snap.rs` (NEW)

Create the file and register it: `pub mod snap;` in `src/app/mod.rs`.

The archive's `SnapKind` carries a **priority** — when an endpoint and a grid crossing both fall
inside the radius, the endpoint must win even if the grid point is a pixel closer. Rank first,
distance second:

```rust
use session_rust::{Geometry, Point, Xform};
use crate::app::scene::Scene;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SnapKind { Endpoint, Vertex, Grid }

impl SnapKind {
    /// Lower = stronger. Geometry beats construction helpers (the archive's ordering).
    pub fn priority(self) -> u8 {
        match self { SnapKind::Endpoint | SnapKind::Vertex => 0, SnapKind::Grid => 6 }
    }
    pub fn label(self) -> &'static str {
        match self { SnapKind::Endpoint => "End", SnapKind::Vertex => "Vertex",
                     SnapKind::Grid => "Grid" }
    }
}

const SNAP_PX: f64 = 10.0;
const GRID_STEP: f64 = 1000.0;   // must match the lesson-20 grid spacing (and 70's analytic
                                 // grid, if you take that lesson's optional retirement of 20's)
const MAX_CANDIDATES_PER_OBJECT: usize = 64;   // stride-thin dense polylines (below)

/// Everything snap needs from the camera, grouped — one bag instead of a six-arg signature.
pub struct SnapContext<'a> {
    pub view_proj: &'a Xform,
    pub origin: &'a Point,
    pub viewport: (f64, f64, f64, f64),
    pub world_per_pixel: f64,    // at the raw point's depth (57's trio, computed by the caller)
}

/// Snap `raw` (the pick-or-z=0 point under the cursor). Candidates are compared in SCREEN pixels —
/// project each candidate (56's project_to_screen) and measure against the cursor. Returns the
/// snapped point + what it snapped to (None = free point, use raw).
pub fn snap(scene: &Scene, raw: &Point, cursor: (f64, f64), ctx: &SnapContext)
        -> (Point, Option<SnapKind>) {
    let mut best: Option<(Point, SnapKind, f64)> = None;
    let mut consider = |p: Point, k: SnapKind| {
        if let Some(s) = crate::engine::pick::project_to_screen(ctx.view_proj, ctx.origin, &p,
                                                                ctx.viewport) {
            let d2 = (s.0 - cursor.0).powi(2) + (s.1 - cursor.1).powi(2);
            if d2 <= SNAP_PX * SNAP_PX {
                let better = best.as_ref().map_or(true, |(_, bk, bd)|
                    k.priority() < bk.priority() || (k.priority() == bk.priority() && d2 < *bd));
                if better { best = Some((p, k, d2)); }
            }
        }
    };

    // geometry candidates — only from objects NEAR the cursor
    // (52's BVH keeps this O(few), not O(N))
    for guid in scene.objects_in(&query_box_around(raw, ctx.world_per_pixel)) {
        // BVH/box-set skew can hand back a stale guid after a reconcile — never index-panic
        let Some(&row) = scene.guid_to_row.get(guid) else { continue };
        let placed = scene.placed_frame(row);      // manifest place × session world_xform (52)
        let doc = scene.doc_of_row(row);           // Scene has no session — the doc does
        let Some(geom) = doc.session.lookup.get(guid) else { continue };
        match geom {
            Geometry::Line(l) => {
                consider(l.start().transformed(&placed), SnapKind::Endpoint);
                consider(l.end().transformed(&placed), SnapKind::Endpoint);
            }
            Geometry::Polyline(pl) => {
                // stride-thin: a 100k-point polyline is NOT 100k projections per mouse-move
                let pts = pl.get_points();
                let stride = pts.len() / MAX_CANDIDATES_PER_OBJECT + 1;
                for p in pts.into_iter().step_by(stride) {
                    consider(p.transformed(&placed), SnapKind::Endpoint);
                }
            }
            Geometry::Point(p) => consider(p.transformed(&placed), SnapKind::Vertex),
            Geometry::Mesh(m) => {
                // boundary verts — the handles (32a); stride-thin the same way if profiling bites
                for vk in m.naked_vertices(true) {
                    if let Some(p) = m.vertex_point(vk) {
                        consider(p.transformed(&placed), SnapKind::Vertex);
                    }
                }
            }
            _ => {}
        }
    }
    // grid candidate — the crossing nearest the raw point, only when raw is on the ground plane
    if raw[2].abs() < 1e-6 {
        let g = Point::new((raw[0]/GRID_STEP).round()*GRID_STEP,
                           (raw[1]/GRID_STEP).round()*GRID_STEP, 0.0);
        consider(g, SnapKind::Grid);
    }

    match best {
        Some((p, k, _)) => (p, Some(k)),
        None => (raw.clone(), None),
    }
}
```

`query_box_around` builds the broad-phase world `OBB` around `raw` and reuses 52's `objects_in` —
add it below `snap` in `snap.rs` (grow the file's imports: `use session_rust::{AABB, OBB};`):

```rust
/// Broad-phase box for snap candidates. Anything that can project within SNAP_PX of the cursor
/// sits within SNAP_PX·world_per_pixel of `raw` (×2 margin, since candidates project from any
/// direction). A FIXED world radius was the old bug: zoomed out, one pixel spans meters, and the
/// box silently missed candidates that were on-screen close — snapping "broke" exactly when the
/// scene got big. Scale the box with the pixel size and the rule holds at every zoom.
fn query_box_around(raw: &Point, world_per_pixel: f64) -> OBB {
    let r = world_per_pixel * SNAP_PX * 2.0;
    OBB::from_aabb(AABB::new(raw[0], raw[1], raw[2], r, r, r))   // center + half-extents
}
```

(Every candidate is lifted into world space by its row's **placed frame** — same world-frame
discipline as 40/47; skipping it snaps to where the object *isn't*. If you ever batch candidates
per doc instead of per row, get placements from `doc.session.world_xforms()` exactly like
`add_file` does — don't invent a second placement rule.)

> **Cost note.** `snap` runs on *every mouse-move* while a command listens: one BVH query plus one
> `project_to_screen` per candidate. That's cheap at the scene sizes so far, but notice what
> actually changes when: the candidate *set* changes only with the scene (49's reconcile
> generation), their *screen positions* only with the camera (78's dirty flag). If profiling ever
> bites, cache candidates keyed on the scene generation and projections keyed on the camera
> generation — the per-move work then shrinks to re-ranking a handful of 2-D distances.

## Step 2 — one call site: `src/state.rs`

`cursor_world_point()` — the function 61's clicks and 71's `on_move` both already use — grows a
snap tail, and every tool inherits snapping. In 71's body, the pick-or-z=0 code stays untouched;
what changes is that its two exits converge on one `raw` point that snap filters. Find 71's
`if let Some(hit) = self.scene.pick_ray(&ray, tol) { return Some(hit.point); }` and everything
below it → replace with:

```rust
        let raw = if let Some(hit) = self.scene.pick_ray(&ray, tol) {
            hit.point
        } else {
            // empty space: intersect the ground plane z = 0 (71, unchanged)
            if ray.dir[2].abs() < 1e-12 { return None; }
            let t = -ray.origin[2] / ray.dir[2];
            if t < 0.0 { return None; }
            Point::new(ray.origin[0] + t * ray.dir[0], ray.origin[1] + t * ray.dir[1], 0.0)
        };
        if !self.snap_enabled { self.snap_marker = None; return Some(raw); }
        let vp = self.camera.view_proj(self.aspect());              // same trio as 48
        let origin = self.camera.origin();
        // proj_y / ortho_h / vp_h are still in scope from the tolerance computation above —
        // the same 49 numbers, now also sizing snap's broad-phase box
        let ctx = crate::app::snap::SnapContext {
            view_proj: &vp,
            origin: &origin,
            viewport: (0.0, 0.0, self.gpu.config.width as f64, self.gpu.config.height as f64),
            world_per_pixel: self.camera.world_per_pixel(
                self.camera.distance, proj_y, ortho_h, vp_h),
        };
        let (p, kind) = crate::app::snap::snap(&self.scene, &raw, self.cursor, &ctx);
        self.snap_marker = kind.map(|k| (p.clone(), k));            // the live marker (Step 3)
        Some(p)
```

Both fields are new: add `snap_enabled: bool` and `snap_marker: Option<(Point, SnapKind)>` to
`struct State`, and initialize them in `State::new` (`snap_enabled: true`, `snap_marker: None`) —
otherwise the `self.snap_enabled` / `self.snap_marker` above don't exist. `state.rs` also needs
`use crate::app::snap::SnapKind;` for the field's type.

Plus a `snap` verb in the registry — `"snap"` toggles `snap_enabled` and logs the state (`VERBS` too).
This is the CLI-option pattern from 53; a per-kind toggle UI can wait for the settings panel.

## Step 3 — the marker: `src/engine/gpu/mod.rs` + `src/state.rs`

Users trust snap only when they can *see* it. One white glyph at the snap point — 71's preview
table holds *segments*, so the marker gets its own one-glyph slot, the same fixed-capacity pattern
a third time. In `Gpu`: fields `pub marker_buffer: wgpu::Buffer, pub marker_bind_group:
wgpu::BindGroup, pub marker_count: u32` (buffer = `storage_buffer(&device, "snap.marker",
&vec![GlyphPoint::zeroed(); 1])`, bind group on `glyph_layout`, created beside 71's preview pair;
all three into `Ok(Self { … })`), plus:

```rust
    /// The snap marker: one white glyph, or none.
    pub fn set_snap_marker(&mut self, at: Option<[f32; 3]>) {
        match at {
            Some(center) => {
                let g = GlyphPoint { center, radius: 6.0, color: [1.0, 1.0, 1.0, 1.0],
                                     instance_id: self.preview_row, _pad: [0; 3] };
                self.queue.write_buffer(&self.marker_buffer, 0, bytemuck::bytes_of(&g));
                self.marker_count = 1;
            }
            None => self.marker_count = 0,
        }
    }
```

Draw it right after 71's ghost block in `clear()` — same shape, sphere pipeline:

```rust
            if self.marker_count > 0 {
                pass.set_pipeline(&self.pipelines.sphere);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.marker_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                pass.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..1);
                draws += 1;
            }
```

In `state.rs`, the cursor-move handler (right after 71's `on_move` forwarding block) pushes the
marker every move a command is listening:

```rust
        // show (or clear) the live snap marker
        let m = if self.active.is_some() {
            self.snap_marker.as_ref().map(|(p, _)| p.to_f32())
        } else { None };
        self.gpu.set_snap_marker(m);
```

And the CLI prompt can suffix the kind — `line: pick TO point  [End]` — by appending
`self.snap_marker.as_ref().map(|(_, k)| format!("  [{}]", k.label()))` where `set_prompt` (61)
writes `self.ui.prompt`.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Draw a `line`, then start another and hover near the first line's end → the white marker jumps onto
  the endpoint, the prompt shows `[End]`; click → `probe` both endpoints: **identical to the last
  digit**, not merely close. That exactness is the entire point of the lesson.
- Hover near a grid crossing on empty ground → `[Grid]`, and the placed point is a perfect multiple
  of the grid step. Where an endpoint and a grid crossing are both in radius → the **endpoint** wins
  (priority beats distance).
- Zoom far out → snapping still engages within the same ~10 *pixels* (zoom-independent, 56's rule).
- `snap` ⏎ → toggled off, the marker disappears, clicks are raw again. `rect`/`box`/`polyline` all
  snap too — they were never edited; the Get-loop was.

## Recap

```
Ch 71: ghosts — tools show a future; finish makes it real.
Ch 72: PRECISION. app/snap.rs: SnapKind { Endpoint/Vertex(0) > Grid(6) } — RANK first, pixel
       distance second, radius ~10 px screen-space (zoom-independent). Candidates: line/polyline
       endpoints, Point objects, mesh boundary verts (lifted by the row's PLACED FRAME — world
       frame or you snap to nowhere), gathered via 52's BVH around the raw point; the broad-phase
       box scales with world_per_pixel (a FIXED world radius silently broke snapping zoomed out),
       and stale guids from BVH/box-set skew are skipped, never index-panicked. Dense polylines
       are stride-thinned to 64 candidates. Camera inputs travel grouped as SnapContext; grid =
       nearest crossing when on z=0.
       ONE call site — cursor_world_point(), which 61's clicks and 71's on_move already share — so
       every existing and future tool became snap-aware with zero tool edits. Live feedback: white
       marker glyph on the preview row + `[End]` suffix in the prompt. `snap` verb toggles. Phase 9
       complete: select, transform, create — precisely.
```

Edited: `app/snap.rs` (NEW — `SnapKind`, `SnapContext`, `snap`), `state.rs` (`cursor_world_point`
consults snap, marker, `snap_enabled`), `app/commands.rs` (`snap` verb).

## Next

`52-nurbscurve.md` — Phase 10: curved geometry. The kernel's `NurbsCurve` gets drawn (sampled to a
polyline through the 31 tube path) and drawable (`curve` tool: control-point clicks, Enter) —
sampled once, cached, undoable as one Command.
