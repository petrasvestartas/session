# 60 Gumball IV — rotate and scale

> **Big picture.** *Phase 9.* 59 built the whole drag skeleton — deferred press, live matrix path,
> Command commit. Rotate and scale add **only new delta math** into that skeleton: an arc drag
> becomes an angle (ray–plane intersection + `atan2`), a sphere drag becomes a factor (distance
> ratios). Scale is where naive math bites hardest — the archive shipped two specific damping fixes
> after real fights with runaway scaling, and both are baked in below.

<svg viewBox="0 0 680 140" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="rotation: intersect the ray with the arc plane and take the atan2 angle difference; scale: ratio of distances from the origin with damping" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(120,72)">
    <ellipse cx="0" cy="0" rx="62" ry="40" fill="none" stroke="#4f7dd0" stroke-width="1.5"/>
    <line x1="0" y1="0" x2="55" y2="-18" stroke="#888"/><line x1="0" y1="0" x2="10" y2="-39" stroke="#d7dae0"/>
    <path d="M 38,-13 A 40,26 0 0 1 8,-27" fill="none" stroke="#6fb3ff" stroke-width="1.6" marker-end="url(#ah55)"/>
    <text x="66" y="-30" fill="#888">θ = atan2(v·B, v·A)</text>
    <text x="0" y="62" fill="#666" text-anchor="middle">ray ∩ arc plane → angle vs press angle</text>
  </g>
  <g transform="translate(430,72)">
    <circle cx="0" cy="0" r="4" fill="#dcdcdc"/>
    <line x1="0" y1="0" x2="120" y2="0" stroke="#888" stroke-dasharray="3 3"/>
    <circle cx="52" cy="0" r="5" fill="#e05555"/><circle cx="96" cy="0" r="5" fill="none" stroke="#e05555"/>
    <text x="52" y="-14" fill="#888" text-anchor="middle">press</text><text x="96" y="-14" fill="#666" text-anchor="middle">now</text>
    <text x="60" y="34" fill="#666" text-anchor="middle">factor = (d_now / d_press) ^ damping</text>
    <text x="60" y="52" fill="#888" text-anchor="middle" font-size="10">sign preserved · √ per-axis · ⁴√ uniform</text>
  </g>
  <defs><marker id="ah55" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/engine/gumball.rs   # angle_on_arc_plane + scale_ratio — the two new deltas
src/state.rs            # begin_drag/on_move match on the handle group; commit path unchanged
```

## Step 1 — the rotation delta: `src/engine/gumball.rs`

An arc for axis `n` lives in the plane through the gumball origin with normal `n`. Intersect the pick
ray with that plane; the hit's angle around the axis (measured against the plane's two basis vectors
`A`, `B`) minus the press angle is the rotation:

```rust
/// Angle (radians) of the ray's intersection with the plane (origin o, normal n), measured with
/// atan2 against in-plane basis (a_dir, b_dir). None when the ray grazes the plane edge-on.
pub fn angle_on_arc_plane(ray: &crate::engine::pick::Ray, o: &Point, n: &Vector,
                          a_dir: &Vector, b_dir: &Vector) -> Option<f64> {
    let denom = ray.dir.dot(n);
    if denom.abs() < 1e-9 { return None; }                    // looking along the arc plane
    let t = (o.clone() - ray.origin.clone()).dot(n) / denom;  // ray ∩ plane
    if t < 0.0 { return None; }
    let hit = ray.origin.clone() + &ray.dir * t;
    let v = hit - o.clone();
    Some(v.dot(b_dir).atan2(v.dot(a_dir)))                    // angle in the (A, B) frame
}
```

For `RotateX`: `n = X`, `a_dir = Y`, `b_dir = Z` — the same `(i+1)%3 / (i+2)%3` pairing 57's arc
builder used, so the measured angle and the drawn arc agree. Put the pairing in a helper the drag
code and `begin_drag` both call — add it below `angle_on_arc_plane`, still in `gumball.rs`:

```rust
/// The arc's (normal, in-plane A, in-plane B) for a handle's axis — the same
/// (i+1)%3 / (i+2)%3 pairing build()'s arc loop draws, so measured angle matches the arc.
pub fn arc_basis(k: HandleKind) -> (Vector, Vector, Vector) {
    use HandleKind::*;
    match k {
        RotateX | ScaleX | TranslateX => (Vector::x_axis(), Vector::y_axis(), Vector::z_axis()),
        RotateY | ScaleY | TranslateY => (Vector::y_axis(), Vector::z_axis(), Vector::x_axis()),
        _                             => (Vector::z_axis(), Vector::x_axis(), Vector::y_axis()),
    }
}
```

The live delta is a rotation **about the axis line through the gumball origin** — the kernel has
that in one call: `Xform::rotation_around_line(&line, ang, /* degrees: */ false)`. Mind the third
positional argument: it is a bare `bool` meaning **"the value is in degrees"** — `false` here
because the drag math works in radians, and 61's typed entry passes `true`. A bool at a call site
is a trap (nothing at the call says what it means, and the two lessons pass *opposite* values) —
annotate it every single time, as above. Step 3's
match arm builds it. Everything else — `set_live_models`, release, `TransformObjects`, undo — is
59's code untouched. That's the skeleton paying off. (`state.rs` gains `Line` in its
`session_rust` use for the axis-line constructor; `gumball.rs` needs nothing new — 59 already
imported `Point`/`Vector`.)

## Step 2 — the scale delta, with the two archive fixes: `src/engine/gumball.rs`

Scale compares *how far from the origin* the cursor is now vs. at press. Two real bugs live here:

**Fix 1 — preserve the sign.** The axis-scale spheres sit on the **negative** axis (57). The naive
`d.max(1e-4)` clamp turns a negative press-distance into `1e-4` — and the very first move computes
`ratio = d_now / 1e-4` ≈ *tens of thousands*. The object vanishes to a point or explodes off screen.
Clamp *away from zero, keeping the sign*:

```rust
fn clamp_signed(d: f64) -> f64 {
    if d < 0.0 { d.min(-1e-4) } else { d.max(1e-4) }
}
```

**Fix 2 — damp the response.** A raw ratio feels violent — half the screen of mouse travel is 10×.
The archive settled on square-root response for axis scale and **fourth root** for uniform scale
(whose press-distance can be tiny — you clicked the *center* sphere, so `d_press ≈ 0` makes raw
ratios enormous):

```rust
/// Axis scale (ScaleX/Y/Z): signed ratio of axis projections, √-damped.
pub fn axis_scale_ratio(t_now: f64, t_press: f64) -> f64 {
    let r = clamp_signed(t_now) / clamp_signed(t_press);
    r.signum() * r.abs().powf(0.5)
}

/// Uniform scale (center sphere): ratio of in-plane distances from the origin, ⁴√-damped.
pub fn uniform_scale_ratio(d_now: f64, d_press: f64) -> f64 {
    (d_now.max(1e-4) / d_press.max(1e-4)).powf(0.25)
}
```

Axis scale reuses 59's `closest_param_on_axis` for `t`; uniform scale measures the ray's hit
distance on the **view-facing plane** through the origin — Step 1's plane intersection with
`n = camera forward`, packaged as one more `gumball.rs` helper (below `uniform_scale_ratio`):

```rust
/// Distance from the gumball origin to the ray ∩ plane(o, n) hit — uniform scale's input.
pub fn view_plane_distance(ray: &Ray, o: &Point, n: &Vector) -> Option<f64> {
    let denom = ray.dir.dot(n);
    if denom.abs() < 1e-9 { return None; }
    let t = (o.clone() - ray.origin.clone()).dot(n) / denom;
    if t < 0.0 { return None; }
    let hit = ray.origin.clone() + &ray.dir * t;
    Some((hit - o.clone()).magnitude())
}
```

The live delta is scale-about-the-origin via `Xform::scale_non_uniform(&ctx.origin, sx, sy, sz)` — a
verified kernel constructor, scaling about a point in one call, no translate-sandwich needed. The
dragged axis gets the (floored) ratio, the others `1.0`; uniform gets `(f, f, f)` — Step 3's arms
spell it out.

## Step 3 — dispatch by handle group: `src/state.rs`

`begin_drag` and the mouse-move arm switch on the handle's group; each group stashes its own press
reference (`t0` for translate/axis-scale, `a0` for rotate, `d0` for uniform scale) in `DragCtx`, and
every group ends at the same two lines — 59's batched `set_live_models`, `TransformObjects` on
release.

**3a. The stash.** Add `a0: f64,` and `d0: f64,` to 59's `DragCtx` struct. In `begin_drag` (59),
find the `let t0 = …unwrap_or(0.0);` line → insert after it (the `use HandleKind::*;` above it
already covers these names). Like `t0`, these are PRESS references — 59's Step-1 aside is the why:
every delta is measured from the press value, never from the previous frame, so mouse jitter and
f64 round-off never compound, and the release commits exactly one delta.

```rust
        let (mut a0, mut d0) = (0.0, 0.0);
        match handle {
            RotateX | RotateY | RotateZ => {
                let (n, a_dir, b_dir) = crate::engine::gumball::arc_basis(handle);
                a0 = crate::engine::gumball::angle_on_arc_plane(&ray, &origin, &n, &a_dir, &b_dir)
                    .unwrap_or(0.0);
            }
            ScaleUniform => {
                let fwd = self.camera.orientation.rotate_vector(Vector::y_axis());
                d0 = crate::engine::gumball::view_plane_distance(&ray, &origin, &fwd)
                    .unwrap_or(0.0);
            }
            _ => {}
        }
```

and add `a0, d0,` to the `DragCtx { … }` literal at the end of `begin_drag` (E0063 until you do).

**3b. The dispatch.** In 59's mouse-move handler, find the whole
`if let Some(ctx) = &self.gb_drag { … }` block (the one computing `dt`/`delta`) → replace with:

```rust
        let mut live_delta = None;
        if let (Some(ctx), Some(ray)) = (&self.gb_drag, self.cursor_ray()) {
            use crate::engine::gumball::{angle_on_arc_plane, arc_basis, axis_scale_ratio,
                                         closest_param_on_axis, uniform_scale_ratio,
                                         view_plane_distance};
            use HandleKind::*;
            let delta = match ctx.handle {
                TranslateX | TranslateY | TranslateZ =>                       // 59, unchanged math
                    closest_param_on_axis(&ray, &ctx.origin, &ctx.axis).map(|t| {
                        let dt = t - ctx.t0;
                        Xform::translation(ctx.axis[0]*dt, ctx.axis[1]*dt, ctx.axis[2]*dt)
                    }),
                RotateX | RotateY | RotateZ => {                              // Step 1
                    let (n, a_dir, b_dir) = arc_basis(ctx.handle);
                    angle_on_arc_plane(&ray, &ctx.origin, &n, &a_dir, &b_dir).map(|now| {
                        let ang = now - ctx.a0;                               // radians since press
                        let axis_line = Line::from_points(&ctx.origin,
                                                          &(ctx.origin.clone() + n.clone()));
                        Xform::rotation_around_line(&axis_line, ang,
                                                    /* degrees: */ false)
                    })
                }
                ScaleX | ScaleY | ScaleZ =>                                   // Step 2, one axis
                    closest_param_on_axis(&ray, &ctx.origin, &ctx.axis).map(|t| {
                        let f = axis_scale_ratio(t, ctx.t0).max(0.01);        // no zero/mirror
                        let (sx, sy, sz) = match ctx.handle {
                            ScaleX => (f, 1.0, 1.0),
                            ScaleY => (1.0, f, 1.0),
                            _      => (1.0, 1.0, f),
                        };
                        Xform::scale_non_uniform(&ctx.origin, sx, sy, sz)
                    }),
                ScaleUniform => {                                             // Step 2, all axes
                    let fwd = self.camera.orientation.rotate_vector(Vector::y_axis());
                    view_plane_distance(&ray, &ctx.origin, &fwd).map(|d| {
                        let f = uniform_scale_ratio(d, ctx.d0).max(0.01);
                        Xform::scale_non_uniform(&ctx.origin, f, f, f)
                    })
                }
            };
            if let Some(delta) = delta {
                // 59's batched live path: stage all rows into the scratch, ONE write_buffer
                self.gb_edits.clear();
                self.gb_edits.extend(ctx.before.iter().map(|s| (s.row, &delta * &s.placed)));
                self.gpu.set_live_models(&self.gb_edits);
                live_delta = Some(delta);
            }
        }
```

(The `if let Some(delta) = live_delta { … }` stash + `refresh_gumball_at` lines after it are 59's,
unchanged. So are 59's guards: the release's identity check skips no-op commits for every group,
since all four funnel into the same `last_delta`.)

The commit path does not change **at all** — the release still hands the final delta to 59's
`Scene::apply_world_delta(row, &delta)`: one `session.set_xform` per object, no per-variant match,
nothing bakes; and the Command's absolute `(guid, Xform)` snapshots don't care what kind of
transform happened. One thing worth saying out loud about the delta you just built: it is
**world-space**, and 59's helper conjugates it by the doc's `place` internally — for translate that
was a nicety, for rotate/scale it's load-bearing (an anchor taken in the wrong frame visibly wrecks
the object). And because `rotation_around_line` / `scale_non_uniform` anchor at a point, the whole
delta is one `Xform` — exactly what `set_xform` stores — so rotate and scale are *cheaper* than the
old bake-the-thin-geometry days, not dearer. `label()` can read the group for nicer log lines
(`rotate 3 object(s)`).

## Step 4 — pin the new math: `src/engine/gumball.rs`

The two ratio functions and the axis param are exactly the kind of pure math that regresses
silently — add to 58's `#[cfg(test)] mod tests`:

```rust
    #[test]
    fn axis_param_is_press_relative_and_guarded() {
        let o = Point::new(0.0, 0.0, 0.0);
        // a ray crossing the X axis at x=3 → t = 3
        let r = ray([3.0, -50.0, 0.0], [0.0, 1.0, 0.0]);
        assert!((closest_param_on_axis(&r, &o, &Vector::x_axis()).unwrap() - 3.0).abs() < 1e-9);
        // ray PARALLEL to the axis → None (the denom guard): the drag holds, no NaN delta
        let par = ray([0.0, 5.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(closest_param_on_axis(&par, &o, &Vector::x_axis()).is_none());
    }

    #[test]
    fn axis_scale_ratio_preserves_sign_and_damps() {
        // the scale spheres sit on the NEGATIVE axis (57): press t is negative, and the
        // sign-preserving clamp must not turn it into 1e-4 (that made first-frame ratios ~10⁴)
        assert!(axis_scale_ratio(-150.0, -75.0) > 0.0);
        // √-damped: a 4× raw ratio reads as 2×
        assert!((axis_scale_ratio(300.0, 75.0) - 2.0).abs() < 1e-9);
        // dragging THROUGH the origin keeps the sign negative — the 0.01 floor at the call
        // site (Step 3b) is what stops the mirror flip
        assert!(axis_scale_ratio(-10.0, 75.0) < 0.0);
    }
```

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **Rotate:** drag the blue arc → the selection spins about the gumball's Z, visually pinned to the
  arc plane (the cursor leads, the objects follow the angle exactly). Orbit to a shallow view of the
  same arc → the drag still tracks until the plane is edge-on, then holds (the `None` arm).
- **Axis scale:** drag a colored sphere outward → the selection stretches along that axis only,
  smoothly (√ response). Drag *through* the origin — no explosion, no mirror flip (sign-preserving
  clamp + the 0.01 floor).
- **Uniform scale:** drag the white sphere → gradual grow/shrink even though you grabbed it dead
  center (⁴√ response). This is the one that was "enormous ratio" territory before Fix 2.
- Mixed session: rotate, undo, scale, undo, redo ×2 — the history walks cleanly through mixed
  transform kinds because every commit is the same `TransformObjects` shape.

## Recap

```
Ch 59: translate — the drag skeleton (defer, live matrix, Command commit).
Ch 60: ROTATE + SCALE = two delta functions on that skeleton. Rotate: ray ∩ arc plane (denom guard
       when edge-on) → atan2(v·B, v·A) in the (i+1,i+2) basis 57 drew the arc in → rotation
       about the gumball origin (`rotation_around_line(…, /* degrees: */ false)` — a bare bool
       meaning "degrees", annotate EVERY call; 61 passes true). Scale: distance ratios with the
       TWO archive fixes —
       clamp_signed (spheres sit on the NEGATIVE axis; an unsigned clamp makes the first
       frame's ratio ~10⁴) and damped response (√ axis, ⁴√ uniform — the center sphere's press
       distance is near zero) + a 0.01 floor so a drag can never zero or mirror — all pinned by
       #[cfg(test)] (press-relative axis param, parallel-ray None, sign preservation, √ damping).
       scale_non_uniform(origin,…) scales about the anchor in one kernel call. Commit path
       byte-identical to 59 — absolute snapshots don't care what moved, and the live path is the
       same batched set_live_models.
```

Edited: `engine/gumball.rs` (`angle_on_arc_plane`, `clamp_signed`, `axis_scale_ratio`,
`uniform_scale_ratio`, `view_plane_distance`, `arc_basis`, more `#[cfg(test)]`s), `state.rs`
(per-group `begin_drag` stash + delta dispatch).

## Next

`61-gumball-numeric.md` — the click that 59's threshold deliberately left free: click a handle
*without* dragging and a tiny popup opens at the cursor — type `500` ⏎ and the selection moves
exactly 500 mm along that axis. Reuses the Get-loop input, with three archive gotchas (the lmb gate,
deferred drag interplay, and an Escape guard) already accounted for.
