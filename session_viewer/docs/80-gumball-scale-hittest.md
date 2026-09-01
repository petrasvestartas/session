# 80 Gumball II — constant size + pickable handles

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
correct depth is the **view-space Z**: how far the gumball origin sits *along the look direction* —
`dot(origin − eye, forward)`, with `forward` read straight off the camera quaternion the way
`update_position` does (17). In `src/engine/gumball.rs`, add below the `SHAFT_R` const:

```rust
pub const SCREEN_PX: f32 = 140.0;   // desired on-screen size of the widget
```

In `src/state.rs`, add to `impl State` (next to `refresh_gumball`, 65); the `proj_y`/`ortho_h`/
`vp_h` lines are 57's pick-site trio verbatim, and the camera fields (`unit`, `orientation`,
`position`, `perspective`, `distance`) are all in meters/camera space, so the mm gumball origin is
scaled by `unit` first. The depth is its own function — Step 3's hit tolerance needs the same
number:

```rust
    /// View-space Z of the gumball anchor — how far it sits along the LOOK direction. The scale
    /// AND the hit tolerance both want THIS depth, not camera.distance: that's the eye→TARGET
    /// depth, and a widget offset from the target is off by exactly that offset.
    fn gumball_depth(&self, origin: [f64; 3]) -> f64 {
        let unit = self.camera.unit.to_meters();                            // mm → m
        let fwd = self.camera.orientation.rotate_vector(Vector::y_axis());  // eye → target (17)
        ((origin[0] * unit - self.camera.position[0]) * fwd[0]
       + (origin[1] * unit - self.camera.position[1]) * fwd[1]
       + (origin[2] * unit - self.camera.position[2]) * fwd[2]).max(0.001)
    }

    /// Gumball scale so ARC_RADIUS spans ~SCREEN_PX pixels. Depth = VIEW-SPACE Z, not Euclidean
    /// distance (the archive's orbit-breathing bug). Mirrors 57's world_per_pixel per projection.
    fn gumball_scale(&self, origin: [f64; 3]) -> f32 {
        let unit = self.camera.unit.to_meters();                            // mm → m
        let depth = self.gumball_depth(origin);
        let proj_y  = 1.0 / (30.0_f64).to_radians().tan() * unit;           // 57's trio, verbatim
        let ortho_h = if self.camera.perspective { 0.0 }
                      else { 2.0 * self.camera.distance * (30.0_f64).to_radians().tan() * unit };
        let vp_h    = self.gpu.config.height as f64;
        let world_per_px = self.camera.world_per_pixel(depth, proj_y, ortho_h, vp_h);   // mm / px
        (crate::engine::gumball::SCREEN_PX as f64 * world_per_px) as f32
            / crate::engine::gumball::ARC_RADIUS
    }
```

(`use session_rust::Vector;` if `state.rs` doesn't import it yet; the origin is 65's f64
`selection_centroid` — no casts needed here.) In `refresh_gumball` (65), find the
`let g = crate::engine::gumball::build([0.0, 0.0, 0.0], 1.0, self.gpu.gb_row);` line → compute the
scale first, stash it for the dirty gate below, and build with it:

```rust
                let s = self.gumball_scale(o);
                self.gb_scale = s;                      // render()'s dirty gate compares to this
                let g = crate::engine::gumball::build([0.0, 0.0, 0.0], s, self.gpu.gb_row);
```

(add `gb_scale: f32` to `struct State`, init `0.0` in `State::new`).

The scale must also refresh when the **camera** moves, not just the selection (it depends on
depth) — but *not* by rebuilding every drawn frame: that's ~400 rows rebuilt plus two allocs plus
two uploads at 60 fps for a widget that only changes when the view does. Gate it on an actual
change — in `render()` (`src/state.rs`), right after the `let view_proj = …` line, insert:

```rust
        // camera moved? then the screen-constant scale is stale — rebuild ON CHANGE, not per
        // frame (selection changes already refresh at their own sites, 65; under 78's
        // render-on-demand this whole hook only runs when a frame exists at all)
        if self.gb.is_some() {
            if let Some(o) = self.scene.selection_centroid() {
                let s = self.gumball_scale(o);
                if (s - self.gb_scale).abs() > 1e-6 * s { self.refresh_gumball(); }
            }
        }
```

And since a refresh still happens on most frames *while orbiting*, kill the two per-call
allocations in 65's `upload_gumball` — reuse scratch vectors. Add `gb_seg_scratch:
Vec<CylinderSegment>` and `gb_glyph_scratch: Vec<GlyphPoint>` to `struct Gpu` (init `Vec::new()`;
held for the process — wasm linear memory never shrinks, so a per-call `collect()` is the worst of
both worlds) and replace the function body:

```rust
    /// Strip the handle tags into the scratch buffers, write the two fixed-capacity buffers.
    /// The tagged copy stays on State (`self.gb`) — the hit-test below reads it there.
    pub fn upload_gumball(&mut self, g: &crate::engine::gumball::GumballGeom) {
        self.gb_seg_scratch.clear();
        self.gb_seg_scratch.extend(g.segments.iter().map(|(s, _)| *s));
        self.gb_glyph_scratch.clear();
        self.gb_glyph_scratch.extend(g.glyphs.iter().map(|(p, _)| *p));
        self.queue.write_buffer(&self.gb_segment_buffer, 0,
            bytemuck::cast_slice(&self.gb_seg_scratch));
        self.queue.write_buffer(&self.gb_glyph_buffer, 0,
            bytemuck::cast_slice(&self.gb_glyph_scratch));
        self.gb_segment_count = self.gb_seg_scratch.len() as u32;
        self.gb_glyph_count = self.gb_glyph_scratch.len() as u32;
    }
```

> **Order matters (archive bug #2):** compute the scale *after* the selection change creates the
> gumball, never before — or a freshly-selected object flashes a wrong-sized widget for one frame.
> The `refresh_gumball` call sites from 57 already satisfy this; keep it that way.

## Step 2 — hit-test: `src/engine/gumball.rs`

The build already tags every row with its `HandleKind` (65). Hit-testing is: distance from the pick
ray to each part's primitive — **point-to-ray** for spheres, **segment-to-ray** for shafts and arc
segments — nearest within tolerance wins, with a priority order for the overlapping center. One
frame detail: the geometry is gumball-LOCAL (57 — built around `[0,0,0]`, the row's model positions
it), so the world ray is translated into the widget's frame first; the world anchor is the same
`selection_centroid` the scale uses:

```rust
/// Distance-based handle pick. `origin` = the widget's world anchor (65's geometry is
/// gumball-local — the ray is translated, never rebuilt). `tol` in world units
/// (57's world_per_pixel at the GUMBALL's depth × ~20 px — handles are small, be generous).
/// Priority on near-ties: ScaleUniform > axis scales > arrows > arcs
/// (the center sphere overlaps everything; a tie must not steal it).
pub fn hit_test(geom: &GumballGeom, ray: &crate::engine::pick::Ray,
                origin: [f64; 3], tol: f64) -> Option<HandleKind> {
    let ray = crate::engine::pick::Ray {          // world ray → gumball-local ray (f64 subtract)
        origin: Point::new(ray.origin[0] - origin[0], ray.origin[1] - origin[1],
                           ray.origin[2] - origin[2]),
        dir: ray.dir.clone(),
    };
    let ray = &ray;
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

The two distance helpers are the classic closest-approach formulas, in f64 against 54's
`Ray { origin: Point, dir: Vector }`. The gumball rows store `[f32; 3]` endpoints
(`CylinderSegment.p0/p1`, `GlyphPoint.center`), so the helpers take `[f32; 3]` and cast up — which
is exactly what the `hit_test` calls above pass. Add `use crate::engine::pick::Ray;` and
`use session_rust::Point;` at the top of
`gumball.rs` (below 65's `use crate::engine::gpu::…` line). ~460 primitives, run per mouse event,
not per frame — the direct parametric version is plenty:

```rust
/// Shortest distance from point `c` to the ray (origin + t·dir, t ≥ 0).
fn ray_point_distance(ray: &Ray, c: [f32; 3]) -> f64 {
    let c = [c[0] as f64, c[1] as f64, c[2] as f64];
    let d = [ray.dir[0], ray.dir[1], ray.dir[2]];
    let w = [c[0]-ray.origin[0], c[1]-ray.origin[1], c[2]-ray.origin[2]];
    let dd = d[0]*d[0] + d[1]*d[1] + d[2]*d[2];
    let t = ((w[0]*d[0] + w[1]*d[1] + w[2]*d[2]) / dd).max(0.0);        // clamp behind the origin
    let cp = [ray.origin[0]+t*d[0], ray.origin[1]+t*d[1], ray.origin[2]+t*d[2]];
    let e = [c[0]-cp[0], c[1]-cp[1], c[2]-cp[2]];
    (e[0]*e[0] + e[1]*e[1] + e[2]*e[2]).sqrt()
}

/// Shortest distance between the ray (t ≥ 0) and segment p0→p1 (s ∈ [0,1]): solve the two infinite
/// lines' closest approach, clamp the segment param, re-solve the ray param, clamp it ≥ 0.
fn ray_segment_distance(ray: &Ray, p0: [f32; 3], p1: [f32; 3]) -> f64 {
    let dot = |u: [f64;3], v: [f64;3]| u[0]*v[0] + u[1]*v[1] + u[2]*v[2];
    let o = [ray.origin[0], ray.origin[1], ray.origin[2]];
    let d = [ray.dir[0], ray.dir[1], ray.dir[2]];
    let a = [p0[0] as f64, p0[1] as f64, p0[2] as f64];
    let e = [p1[0] as f64 - a[0], p1[1] as f64 - a[1], p1[2] as f64 - a[2]];
    let r = [o[0]-a[0], o[1]-a[1], o[2]-a[2]];
    let (aa, bb, cc) = (dot(d,d), dot(d,e), dot(e,e));
    let (dr, er) = (dot(d,r), dot(e,r));
    let den = aa*cc - bb*bb;
    let mut t = if den > 1e-12 { (bb*er - cc*dr) / den } else { 0.0 };  // ray param on the two lines
    if t < 0.0 { t = 0.0; }
    let s = ((bb*t + er) / cc.max(1e-12)).clamp(0.0, 1.0);              // seg param for this t, clamped
    t = ((bb*s - dr) / aa.max(1e-12)).max(0.0);                        // re-solve ray param, t ≥ 0
    let cr = [o[0]+t*d[0], o[1]+t*d[1], o[2]+t*d[2]];
    let cs = [a[0]+s*e[0], a[1]+s*e[1], a[2]+s*e[2]];
    let g = [cr[0]-cs[0], cr[1]-cs[1], cr[2]-cs[2]];
    (g[0]*g[0] + g[1]*g[1] + g[2]*g[2]).sqrt()
}
```

## Step 3 — gumball first, scene second: `src/state.rs`

The input pecking order grows one level. From top: egui (60) → Get-loop (61) → **gumball** → scene
picking (55–58). `ray` here is 55's `engine::pick::screen_to_world_ray(&vp, &origin, self.cursor,
viewport)?` result (built early, so it's in scope). `proj_y`/`ortho_h`/`vp_h` are 57's three pick-site
locals (the same numbers 31 packs into the line uniform), but 50 declares them *after* selection
picking — at this earlier gumball point they don't exist yet, so compute the same three just above
this block (exactly as the cursor-move handler below does). The tolerance keys off the **gumball's
own** depth (`gumball_depth`, Step 1) — `camera.distance` is the eye→*target* depth and an
off-target widget would get a systematically wrong tolerance:

```rust
    // in the left-press handler, AFTER the Get-loop check, BEFORE selection picking:
    if let (Some(gb), Some(o)) = (&self.gb, self.scene.selection_centroid()) {
        let tol =
            self.camera.world_per_pixel(self.gumball_depth(o), proj_y, ortho_h, vp_h) * 20.0;
        if let Some(handle) = crate::engine::gumball::hit_test(gb, &ray, o, tol) {
            self.gb_pressed = Some((handle, self.cursor));   // 59 turns this into a drag
            return;                                          // the scene never sees this click
        }
    }
```

One more ordering consequence: 60's marquee rect keys off `lmb_down` + drag distance, and a gumball
drag satisfies both — it would draw a rubber band over the widget. Extend the render() condition
from 52 with the press flag: `if self.lmb_down && self.gb_pressed.is_none() && drag >= 3.0`.

And the release half of the same gate: a press that landed on a handle must never reach 58's
selection branch (clicking an arrow would otherwise *also* select the object behind it). In
`on_left_release` (58), after 61's Get-loop reroute and before the click-vs-marquee branch:

```rust
        if self.gb_pressed.take().is_some() { return; }   // a handle ate this click (66)
```

(59 hangs the drag commit ABOVE this line; 61 replaces the line itself with the numeric-popup open —
the early `return` survives both.)

And hover — same test on mouse-move, brighten the hot handle (rebuild with the hovered part's color
lightened, or simplest: stash `hovered: Option<HandleKind>` and let `build` take it, tinting matching
rows toward white):

This is the cursor-move handler, **not** the press handler — 57's `proj_y`/`ortho_h`/`vp_h` locals
and `ray` are out of scope, so rebuild them first: `screen_to_world_ray(...)?` for `ray`, the
centroid for `o`, and the same `world_per_pixel(self.gumball_depth(o), …) * 20.0` for `tol`.

```rust
    // on cursor move, when not dragging (ray + tol + o rebuilt here as above):
    let hov = self.gb.as_ref()
        .and_then(|g| crate::engine::gumball::hit_test(g, &ray, o, tol));
    if hov != self.gb_hovered { self.gb_hovered = hov; self.refresh_gumball(); }
```

Both handles are new `State` fields — add `gb_pressed: Option<(HandleKind, (f64, f64))>` (the cursor is
54's `(f64, f64)`) and `gb_hovered: Option<HandleKind>` to `struct State`, and initialize **both to
`None`** in `State::new`. `state.rs` needs `use crate::engine::gumball::HandleKind;` for the field
types.

The tint itself is three edits. In `gumball.rs`, find 65's `build` signature:

```rust
pub fn build(o: [f32; 3], s: f32, row: u32) -> GumballGeom {
```

→ replace with:

```rust
pub fn build(o: [f32; 3], s: f32, row: u32, hovered: Option<HandleKind>) -> GumballGeom {
```

Then find the function's last two lines (the uniform-sphere push, then `    g`) and insert the tint
between them, so it runs over every emitted row:

```rust
    // lighten the hovered handle's rows toward white
    if let Some(h) = hovered {
        for (seg, k) in g.segments.iter_mut() {
            if *k == h { for c in &mut seg.color[..3] { *c = *c * 0.4 + 0.6; } }
        }
        for (gl, k) in g.glyphs.iter_mut() {
            if *k == h { for c in &mut gl.color[..3] { *c = *c * 0.4 + 0.6; } }
        }
    }
    g
```

Finally, in `refresh_gumball` (65, with Step 1's scale lines already in), the `build(…)` call gains
the new argument:
`crate::engine::gumball::build([0.0, 0.0, 0.0], s, self.gpu.gb_row, self.gb_hovered)`.

## Step 4 — the math, pinned by tests: `src/engine/gumball.rs`

The distance helpers and the rank tie-break are small, pure, and easy to regress — pin them:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Vector;   // gumball.rs itself doesn't import it until 59

    fn ray(o: [f64; 3], d: [f64; 3]) -> crate::engine::pick::Ray {
        crate::engine::pick::Ray { origin: Point::new(o[0], o[1], o[2]),
                                   dir: Vector::new(d[0], d[1], d[2]) }   // unit d, as 54's is
    }

    #[test]
    fn center_sphere_wins_where_it_overlaps() {
        let g = build([0.0, 0.0, 0.0], 1.0, 0, None);
        // straight at the origin — the shafts are nearby, but distance + rank must
        // hand the click to ScaleUniform
        let r = ray([-500.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert_eq!(hit_test(&g, &r, [0.0, 0.0, 0.0], 20.0), Some(HandleKind::ScaleUniform));
    }

    #[test]
    fn shaft_hit_is_its_axis() {
        let g = build([0.0, 0.0, 0.0], 1.0, 0, None);
        // a ray crossing the X shaft at mid-length, perpendicular to it
        let r = ray([75.0, -500.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(hit_test(&g, &r, [0.0, 0.0, 0.0], 20.0), Some(HandleKind::TranslateX));
    }

    #[test]
    fn a_miss_is_none_and_the_world_anchor_shifts_the_test() {
        let g = build([0.0, 0.0, 0.0], 1.0, 0, None);
        let away = ray([0.0, -500.0, 0.0], [0.0, -1.0, 0.0]);   // pointing AWAY: t clamps to 0
        assert_eq!(hit_test(&g, &away, [0.0, 0.0, 0.0], 20.0), None);
        // the SAME local ray against a widget sitting at x=1e6 (the 57 world-anchor case):
        let at_x = ray([1.0e6 + 75.0, -500.0, 0.0], [0.0, 1.0, 0.0]);
        assert_eq!(hit_test(&g, &at_x, [1.0e6, 0.0, 0.0], 20.0), Some(HandleKind::TranslateX));
    }
}
```

(The last assertion is the 57 precision fix under test: the geometry never carries the 1e6 — the
f64 subtraction in `hit_test` does — so the widget hit-tests exactly however far from the world
origin the selection sits.)

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select something, zoom way in and way out → the gumball spans the **same ~140 px** throughout.
- **Orbit around an off-center selection** → the widget size holds steady. (Swap Step 1's
  `gumball_depth` for Euclidean distance to *see* the archive bug: it breathes as you orbit.
  Swap back.)
- Mouse over each of the ten handles → each brightens individually, arcs included; the center sphere
  wins where it overlaps the shaft roots (the `rank` order).
- Click an arrow with an object directly behind it → the object is **not** selected (the `return`).
  Click past the widget → selection works as before. 59 makes the press actually drag.

## Recap

```
Ch 65: the widget exists — geometry, overlay pass, centroid anchor.
Ch 66: USABLE. Scale = SCREEN_PX(140) · world_per_px(depth) / ARC_RADIUS with depth = VIEW-SPACE Z
       (gumball_depth: dot(o·unit − eye, fwd)) — Euclidean distance breathes during off-center
       orbits, the archive's documented bug; recompute on selection changes AND on camera motion —
       but GATED (compare against the stashed gb_scale; no change → no rebuild, no allocs, no
       uploads) and allocation-free (upload_gumball reuses two scratch Vecs; wasm memory never
       shrinks). hit_test: ray↔segment / ray↔point distance over the tagged rows in the widget's
       LOCAL frame (the world ray is translated by the f64 anchor — geometry never carries world
       coordinates, so hit-testing is exact at 1e6 mm; pinned by #[cfg(test)]),
       tol ≈ 20 px in world units AT THE GUMBALL's depth (camera.distance is the target's depth —
       wrong for an off-target widget), near-ties broken by rank (ScaleUniform > axis scales >
       arrows > arcs — the center overlaps everything). Input order is now: egui → Get-loop →
       GUMBALL → scene;
       a grabbed handle never leaks a click to selection — and never draws 60's marquee rect
       (gb_pressed gates it). Hover = same test, tint the hot handle.
```

Edited: `engine/gumball.rs` (`SCREEN_PX`, `hit_test` + world-anchor param, `rank`, distance
helpers, `#[cfg(test)]`s), `engine/gpu/mod.rs` (`upload_gumball` on scratch buffers), `state.rs`
(`gumball_scale`/`gumball_depth` from view-space Z, the dirty-gated camera hook, hit-before-pick,
`gb_pressed`/`gb_hovered`/`gb_scale`).

## Next

`81-gumball-translate.md` — the first real transform: press an arrow, drag along its axis, watch the
selection follow live (matrix-only — no re-tessellation), release to commit a `TransformObjects`
Command with absolute before/after snapshots. Undo restores exactly. Stress-gated on the PDF drawing.
