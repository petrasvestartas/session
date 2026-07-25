# 59 Snapping — drawing becomes precise

> **Big picture.** *Phase 9 closes.* Freehand clicks are never exact — and CAD is nothing if not
> exact. Snapping pulls the cursor's point to the nearest *meaningful* location: a line's endpoint, a
> mesh vertex, a grid crossing. The architecture note is the important part: snap lives in **one
> place** — the Get-loop's point acquisition — so `line`, `polyline`, `rect`, `box`, and every future
> tool became snap-aware the moment this lesson lands, with zero tool changes. That's the reward for
> routing all point input through one function since 48.

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
    <text x="0" y="98" fill="#666" font-size="10">radius in PIXELS (43's rule) — zoom-independent</text>
  </g>
</svg>

## Files we touch

```
# NEW — SnapKind + snap(scene, raw_point, cursor, view…) → (Point, Option<SnapKind>)
src/app/snap.rs
src/state.rs       # cursor_world_point() consults snap; marker glyph; `snap` CLI toggle
src/app/commands.rs # `snap` on|off verb registration (Step 2)
```

## Step 1 — kinds and candidates: `src/app/snap.rs` (NEW)

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
const GRID_STEP: f64 = 1000.0;   // must match the lesson-20 grid spacing

/// Snap `raw` (the pick-or-z=0 point under the cursor). Candidates are compared in SCREEN pixels —
/// project each candidate (43's project_to_screen) and measure against the cursor. Returns the
/// snapped point + what it snapped to (None = free point, use raw).
pub fn snap(scene: &Scene, raw: &Point, cursor: (f64, f64), view_proj: &Xform, origin: &Point,
            viewport: (f64, f64, f64, f64)) -> (Point, Option<SnapKind>) {
    let mut best: Option<(Point, SnapKind, f64)> = None;
    let mut consider = |p: Point, k: SnapKind| {
        if let Some(s) = crate::engine::pick::project_to_screen(view_proj, origin, &p, viewport) {
            let d2 = (s.0 - cursor.0).powi(2) + (s.1 - cursor.1).powi(2);
            if d2 <= SNAP_PX * SNAP_PX {
                let better = best.as_ref().map_or(true, |(_, bk, bd)|
                    k.priority() < bk.priority() || (k.priority() == bk.priority() && d2 < *bd));
                if better { best = Some((p, k, d2)); }
            }
        }
    };

    // geometry candidates — only from objects NEAR the cursor
    // (36's BVH keeps this O(few), not O(N))
    for guid in scene.objects_in(&query_box_around(raw, scene)) {
        match &scene.session.lookup[guid] {
            Geometry::Line(l) => {
                consider(l.start(), SnapKind::Endpoint);
                consider(l.end(), SnapKind::Endpoint);
            }
            Geometry::Polyline(pl) => for p in pl.get_points() { consider(p, SnapKind::Endpoint); },
            Geometry::Point(p) => consider(p.clone(), SnapKind::Vertex),
            Geometry::Mesh(m) => {
                // boundary verts — the handles (32a)
                for vk in m.naked_vertices(true) {
                    if let Some(mut p) = m.vertex_point(vk) {
                        p.xform = m.xform.clone();
                        consider(p.transformed(), SnapKind::Vertex);
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

(`query_box_around` builds a small world `OBB` around `raw` — a few hundred units, or 44's
`world_per_pixel × SNAP_PX` scaled to depth — and reuses 36's `objects_in`. The mesh arm transforms
local vertices by `mesh.xform` — same world-frame discipline as 36/43; skipping it snaps to where the
mesh *isn't*.)

## Step 2 — one call site: `src/state.rs`

`cursor_world_point()` — the function 48's clicks and 58's `on_move` both already use — grows two
lines, and every tool inherits snapping:

```rust
    fn cursor_world_point(&mut self) -> Option<Point> {
        let raw = /* pick_ray hit point, else ray ∩ z=0 — unchanged (48) */;
        if !self.snap_enabled { self.snap_marker = None; return Some(raw); }
        let vp = self.camera.view_proj(self.aspect());              // same trio as 41/42
        let origin = self.camera.origin();
        let viewport = (0.0, 0.0, self.gpu.config.width as f64, self.gpu.config.height as f64);
        let (p, kind) = crate::app::snap::snap(&self.scene, &raw, self.cursor,
                                               &vp, &origin, viewport);
        self.snap_marker = kind.map(|k| (p.clone(), k));           // the live marker (Step 3)
        Some(p)
    }
```

Both fields are new: add `snap_enabled: bool` and `snap_marker: Option<(Point, SnapKind)>` to
`struct State`, and initialize them in `State::new` (`snap_enabled: true`, `snap_marker: None`) —
otherwise the `self.snap_enabled` / `self.snap_marker` above don't exist. `state.rs` also needs
`use crate::app::snap::SnapKind;` for the field's type.

Plus a `snap` verb in the registry — `"snap"` toggles `snap_enabled` and logs the state (`VERBS` too).
This is the CLI-option pattern from 48; a per-kind toggle UI can wait for the settings panel.

## Step 3 — the marker: `src/state.rs`

Users trust snap only when they can *see* it. Reuse the preview machinery (58): when `snap_marker` is
set during an active command, append one white glyph at the snap point to the preview upload
(`GlyphPoint { center, radius: SPHERE-ish px, color: white }` on the preview row) — and the CLI
prompt can suffix the kind: `line: pick TO point  [End]`. Both are two lines where the ghost is
already built; no new pipeline, no new pass.

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
- Zoom far out → snapping still engages within the same ~10 *pixels* (zoom-independent, 43's rule).
- `snap` ⏎ → toggled off, the marker disappears, clicks are raw again. `rect`/`box`/`polyline` all
  snap too — they were never edited; the Get-loop was.

## Recap

```
Ch 58: ghosts — tools show a future; finish makes it real.
Ch 59: PRECISION. app/snap.rs: SnapKind { Endpoint/Vertex(0) > Grid(6) } — RANK first, pixel
       distance second, radius ~10 px screen-space (zoom-independent). Candidates: line/polyline
       endpoints, Point objects, mesh boundary verts (transformed by mesh.xform — world frame or
       you snap to nowhere), gathered via 36's BVH around the raw point; grid = nearest crossing
       when on z=0.
       ONE call site — cursor_world_point(), which 48's clicks and 58's on_move already share — so
       every existing and future tool became snap-aware with zero tool edits. Live feedback: white
       marker glyph on the preview row + `[End]` suffix in the prompt. `snap` verb toggles. Phase 9
       complete: select, transform, create — precisely.
```

Edited: `app/snap.rs` (NEW — `SnapKind`, `snap`), `state.rs` (`cursor_world_point` consults snap,
marker, `snap_enabled`), `app/commands.rs` (`snap` verb).

## Next

`60-nurbscurve.md` — Phase 10: curved geometry. The kernel's `NurbsCurve` gets drawn (sampled to a
polyline through the 31 tube path) and drawable (`curve` tool: control-point clicks, Enter) — smooth
at every zoom, undoable as one Command.
