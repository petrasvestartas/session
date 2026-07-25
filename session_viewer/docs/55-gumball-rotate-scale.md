# 55 Gumball IV — rotate and scale

> **Big picture.** *Phase 9.* 54 built the whole drag skeleton — deferred press, live matrix path,
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

For `RotateX`: `n = X`, `a_dir = Y`, `b_dir = Z` — the same `(i+1)%3 / (i+2)%3` pairing 52's arc
builder used, so the measured angle and the drawn arc agree. The live delta is a rotation **about the
axis line through the gumball origin** — the kernel has that in one call:

```rust
    // body of the RotateX | RotateY | RotateZ arm (Step 3). n / a_dir / b_dir follow the
    // (i+1)%3 / (i+2)%3 pairing 52's arc builder used, so the measured angle matches the arc:
    let (n, a_dir, b_dir) = match ctx.handle {
        RotateX => (Vector::x_axis(), Vector::y_axis(), Vector::z_axis()),
        RotateY => (Vector::y_axis(), Vector::z_axis(), Vector::x_axis()),
        _       => (Vector::z_axis(), Vector::x_axis(), Vector::y_axis()),
    };
    let now = angle_on_arc_plane(ray, &ctx.origin, &n, &a_dir, &b_dir)?;   // press handler stashed ctx.a0
    let ang = now - ctx.a0;                                    // radians since press
    let axis_line = Line::from_points(&ctx.origin, &(ctx.origin.clone() + n.clone()));
    let delta = Xform::rotation_around_line(&axis_line, ang, false);   // false = radians
```

(Everything else — `set_live_model`, release, `TransformObjects`, undo — is 54's code untouched.
That's the skeleton paying off.)

## Step 2 — the scale delta, with the two archive fixes: `src/engine/gumball.rs`

Scale compares *how far from the origin* the cursor is now vs. at press. Two real bugs live here:

**Fix 1 — preserve the sign.** The axis-scale spheres sit on the **negative** axis (52). The naive
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

Axis scale reuses 54's `closest_param_on_axis` for `t`; uniform scale measures the ray's hit distance
on the **view-facing plane** through the origin (Step 1's intersection with `n = camera forward`).
The live delta is scale-about-the-origin:

```rust
    let f = /* ratio from above */.max(0.01);                  // floor: no zero/mirror via drag
    let delta = Xform::scale_non_uniform(&ctx.origin, sx, sy, sz);   // kernel: scale about a point
    // axis scale: the dragged axis gets f, the others 1.0; uniform: (f, f, f)
```

(`Xform::scale_non_uniform(origin, sx, sy, sz)` is a verified kernel constructor — scaling about a
point in one call, no translate-sandwich needed.)

## Step 3 — dispatch by handle group: `src/state.rs`

`begin_drag` and the mouse-move arm switch on the handle's group; each group stashes its own press
reference (`t0` for translate/axis-scale, `a0` for rotate, `d0` for uniform scale) in `DragCtx` — so
**add `a0: f64` and `d0: f64` to 54's `DragCtx` struct** and set them in `begin_drag` (else E0609 at
`ctx.a0`/`ctx.d0` in the shown deltas), and
every group ends at the same two lines — `set_live_model` per object, `TransformObjects` on release:

```rust
    use HandleKind::*;
    let delta = match ctx.handle {
        TranslateX | TranslateY | TranslateZ => /* 54 */,
        RotateX | RotateY | RotateZ          => /* Step 1: angle → rotation about origin+axis */,
        ScaleX | ScaleY | ScaleZ             =>
            /* Step 2: axis ratio → scale_non_uniform, one axis */,
        ScaleUniform                          =>
            /* Step 2: plane ratio → scale_non_uniform, all axes */,
    };
```

The commit path does not change **at all** — `apply_delta` (54) composes any affine delta into
`mesh.xform` and bakes thin geometry the same way, and the Command's absolute snapshots don't care
what kind of transform happened. `label()` can read the group for nicer log lines
(`rotate 3 object(s)`).

## Step 4 — verify

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
Ch 54: translate — the drag skeleton (defer, live matrix, Command commit).
Ch 55: ROTATE + SCALE = two delta functions on that skeleton. Rotate: ray ∩ arc plane (denom guard
       when edge-on) → atan2(v·B, v·A) in the (i+1,i+2) basis 52 drew the arc in → rotation
       about the gumball origin. Scale: distance ratios with the TWO archive fixes —
       clamp_signed (spheres sit on the NEGATIVE axis; an unsigned clamp makes the first
       frame's ratio ~10⁴) and damped response (√ axis, ⁴√ uniform — the center sphere's press
       distance is near zero) + a 0.01 floor so a drag can never zero or mirror.
       scale_non_uniform(origin,…) scales about the anchor in one kernel call. Commit path
       byte-identical to 54 — absolute snapshots don't care what moved.
```

Edited: `engine/gumball.rs` (`angle_on_arc_plane`, `clamp_signed`, `axis_scale_ratio`,
`uniform_scale_ratio`), `state.rs` (per-group `begin_drag` stash + delta dispatch).

## Next

`56-gumball-numeric.md` — the click that 54's threshold deliberately left free: click a handle
*without* dragging and a tiny popup opens at the cursor — type `500` ⏎ and the selection moves
exactly 500 mm along that axis. Reuses the Get-loop input, with three archive gotchas (the lmb gate,
deferred drag interplay, and an Escape guard) already accounted for.
