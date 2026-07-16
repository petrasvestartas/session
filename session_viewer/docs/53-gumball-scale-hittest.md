# 53 Gumball II — constant size + pickable handles

> **Big picture.** *Phase 9.* A gizmo you can't reliably see or grab is decoration. Two properties
> make the gumball a tool: it stays **~140 px on screen at every zoom** (like every CAD gizmo —
> imagine grabbing a 3-pixel arrow), and its handles **hit-test before the scene** (clicking an arrow
> must never select the object behind it). Both are small, both have a real archive bug attached, and
> both were fixed there so we inherit the fix.

<svg viewBox="0 0 680 160" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="orbiting around a target offset from the gumball keeps the euclidean distance constant while the view space depth changes; scaling by view z keeps the widget a constant screen size" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <circle cx="330" cy="80" r="60" fill="none" stroke="#3a3a3a" stroke-dasharray="4 3"/>
  <circle cx="330" cy="80" r="3" fill="#888"/><text x="330" y="70" fill="#888" text-anchor="middle" font-size="10">orbit target</text>
  <circle cx="378" cy="112" r="5" fill="#e0b040"/><text x="392" y="128" fill="#e0b040" font-size="10">gumball (offset!)</text>
  <circle cx="270" cy="80" r="5" fill="none" stroke="#d7dae0"/><text x="240" y="66" fill="#d7dae0" font-size="10">camera A</text>
  <circle cx="330" cy="20" r="5" fill="none" stroke="#d7dae0"/><text x="352" y="16" fill="#d7dae0" font-size="10">camera B</text>
  <line x1="275" y1="82" x2="373" y2="110" stroke="#e05555" stroke-width="1.1"/>
  <line x1="332" y1="25" x2="377" y2="107" stroke="#e05555" stroke-width="1.1"/>
  <text x="120" y="120" fill="#e05555" font-size="10">Euclidean distance: SAME from A and B…</text>
  <text x="120" y="136" fill="#666" font-size="10">…but the widget's SCREEN DEPTH differs → it "breathes" while orbiting</text>
  <text x="540" y="60" fill="#6fb3ff" font-size="10">fix: depth = (view · origin).z</text>
  <text x="540" y="76" fill="#888" font-size="10">— distance along the LOOK</text>
  <text x="540" y="90" fill="#888" font-size="10">direction, what projection</text>
  <text x="540" y="104" fill="#888" font-size="10">actually divides by</text>
</svg>

## Files we touch

```
src/engine/gumball.rs   # SCREEN_PX + hit_test(ray, geom) → nearest HandleKind
# per-frame scale from VIEW-SPACE Z; gumball hit BEFORE pick; hover highlight
src/state.rs
```

## Step 1 — the scale factor, from view-space Z: `src/state.rs`

The naive formula — scale by *Euclidean distance* from camera to gumball — has a subtle bug the
archive hit: during an orbit around a target the gumball is offset from, the Euclidean distance
stays constant while the gumball's *screen depth* changes, so the widget visibly breathes. The
correct depth is the **view-space Z** — the third row of the view matrix applied to the origin:

```rust
pub const SCREEN_PX: f32 = 140.0;   // add to gumball.rs — desired on-screen size

    /// Gumball scale so ARC_RADIUS spans ~SCREEN_PX pixels. Depth = VIEW-SPACE Z, not Euclidean
    /// distance (the archive's orbit-breathing bug). Mirrors 44's world_per_pixel per projection.
    fn gumball_scale(&self, origin: [f32; 3]) -> f32 {
        let vm = self.camera.view_matrix().to_cols();              // column-major m[col][row]
        let vz = vm[0][2] as f32 * origin[0] + vm[1][2] as f32 * origin[1]
               + vm[2][2] as f32 * origin[2] + vm[3][2] as f32;    // (V · o).z
        let depth = (-vz).max(0.001);                              // camera looks down −Z
        let vp_h = self.gpu.config.height as f32;
        let world_per_px = if self.camera.is_ortho() {
            (self.camera.ortho_h() as f32) / vp_h
        } else {
            // same formula as 44 / cylinder.wgsl
            depth / (self.camera.proj_y() as f32 * vp_h)
        };
        SCREEN_PX * world_per_px / crate::engine::gumball::ARC_RADIUS
    }
```

`refresh_gumball` (52) now passes `self.gumball_scale(o)` instead of `1.0` — and must also run when
the **camera** moves, not just the selection (the scale depends on depth). Cheapest correct hook:
rebuild in `render()` whenever `self.gb.is_some()` and the camera changed this frame. ~400 rows,
one small buffer write — nothing at 60 fps.

> **Order matters (archive bug #2):** compute the scale *after* the selection change creates the
> gumball, never before — or a freshly-selected object flashes a wrong-sized widget for one frame.
> The `refresh_gumball` call sites from 52 already satisfy this; keep it that way.

## Step 2 — hit-test: `src/engine/gumball.rs`

The build already tags every row with its `HandleKind` (52). Hit-testing is: distance from the pick
ray to each part's primitive — **point-to-ray** for spheres, **segment-to-ray** for shafts and arc
segments — nearest within tolerance wins, with a priority order for the overlapping center:

```rust
/// Distance-based handle pick. `tol` in world units (44's world_per_pixel × ~20 px — handles are
/// small, be generous). Priority on near-ties: ScaleUniform > axis scales > arrows > arcs
/// (the center sphere overlaps everything; a tie must not steal it).
pub fn hit_test(geom: &GumballGeom, ray: &crate::engine::pick::Ray,
                tol: f64) -> Option<HandleKind> {
    let mut best: Option<(HandleKind, f64)> = None;
    let mut consider = |kind: HandleKind, d: f64| {
        if d <= tol && best.map_or(true,
            |(bk, bd)| d < bd - 1e-9 || (d < bd + 1e-9 && rank(kind) < rank(bk))) {
            best = Some((kind, d));
        }
    };
    for (s, kind) in &geom.segments {
        consider(*kind, ray_segment_distance(ray, s.p0, s.p1));
    }
    for (g, kind) in &geom.glyphs {
        consider(*kind, ray_point_distance(ray, g.center));
    }
    best.map(|(k, _)| k)
}

fn rank(k: HandleKind) -> u8 {
    match k {
        HandleKind::ScaleUniform => 0,
        HandleKind::ScaleX | HandleKind::ScaleY | HandleKind::ScaleZ => 1,
        HandleKind::TranslateX | HandleKind::TranslateY | HandleKind::TranslateZ => 2,
        HandleKind::RotateX | HandleKind::RotateY | HandleKind::RotateZ => 3,
    }
}
```

The two distance helpers are the classic closest-approach formulas (f64, via kernel `Point`/`Vector`
ops — `Line::from_points(p0, p1).closest_point(&ray_point, true)` iterated, or write the 6-line
parametric version directly; either is fine, there are ~460 primitives and this runs per mouse
event, not per frame).

## Step 3 — gumball first, scene second: `src/state.rs`

The input pecking order grows one level. From top: egui (47) → Get-loop (48) → **gumball** → scene
picking (42–45):

```rust
    // in the left-press handler, AFTER the Get-loop check, BEFORE selection picking:
    if let Some(gb) = &self.gb {
        let tol = self.camera.world_per_pixel(self.camera.distance, proj_y, ortho_h, vp_h) * 20.0;
        if let Some(handle) = crate::engine::gumball::hit_test(gb, &ray, tol) {
            self.gb_pressed = Some((handle, self.cursor));   // 54 turns this into a drag
            return;                                          // the scene never sees this click
        }
    }
```

And hover — same test on mouse-move, brighten the hot handle (rebuild with the hovered part's color
lightened, or simplest: stash `hovered: Option<HandleKind>` and let `build` take it, tinting matching
rows toward white):

```rust
    // on cursor move, when not dragging:
    let hov = self.gb.as_ref().and_then(|g| crate::engine::gumball::hit_test(g, &ray, tol));
    if hov != self.gb_hovered { self.gb_hovered = hov; self.refresh_gumball(); }
```

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select something, zoom way in and way out → the gumball spans the **same ~140 px** throughout.
- **Orbit around an off-center selection** → the widget size holds steady. (Swap Step 1's `vz` for
  Euclidean distance to *see* the archive bug: it breathes as you orbit. Swap back.)
- Mouse over each of the ten handles → each brightens individually, arcs included; the center sphere
  wins where it overlaps the shaft roots (the `rank` order).
- Click an arrow with an object directly behind it → the object is **not** selected (the `return`).
  Click past the widget → selection works as before. 54 makes the press actually drag.

## Recap

```
Ch 52: the widget exists — geometry, overlay pass, centroid anchor.
Ch 53: USABLE. Scale = SCREEN_PX(140) · world_per_px(depth) / ARC_RADIUS with depth = VIEW-SPACE Z
       ((V·o).z, column-extracted) — Euclidean distance breathes during off-center orbits, the
       archive's documented bug; recompute after selection changes AND on camera motion (wrong-size
       first frame otherwise). hit_test: ray↔segment / ray↔point distance over the tagged rows,
       tol ≈ 20 px in world units, near-ties broken by rank (ScaleUniform > axis scales > arrows >
       arcs — the center overlaps everything). Input order is now: egui → Get-loop → GUMBALL →
       scene;
       a grabbed handle never leaks a click to selection. Hover = same test, tint the hot handle.
```

Edited: `engine/gumball.rs` (`SCREEN_PX`, `hit_test`, `rank`, distance helpers), `state.rs`
(`gumball_scale` from view-space Z, hit-before-pick, `gb_pressed`/`gb_hovered`).

## Next

`54-gumball-translate.md` — the first real transform: press an arrow, drag along its axis, watch the
selection follow live (matrix-only — no re-tessellation), release to commit a `TransformObjects`
Command with absolute before/after snapshots. Undo restores exactly. Stress-gated on the PDF drawing.
