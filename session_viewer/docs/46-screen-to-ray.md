# 46 Screen → ray — unproject the mouse into a world ray

> **Big picture.** *Phase 7 — picking & selection (46–50).* Everything interactive from here on —
> select, gumball, draw tools, snapping — starts with one question: *what is under the cursor?*
> WebGPU can't answer it GPU-side (no synchronous readback), so we build the CAD-standard CPU answer
> in stages: turn the cursor into a ray (this lesson), cast it (47–49), select with it (50).

Picking starts here. The cursor is a 2D pixel; everything selectable is 3D. Every pick in Phase 7 —
mesh (47), vertex/edge/face (48), line/point (49), marquee (50) — begins by turning that pixel into a
**world-space ray**: the line through the scene that the cursor points down. This lesson builds that
one function; the next four cast the ray it returns.

The math is "run the camera backwards." `view_proj` maps world → clip → NDC; picking needs the inverse,
NDC → world. Unproject two points down the cursor at different depths, and the line through them is the
ray. Two details make it robust: the viewer is **camera-relative** (33), so the inverse lands in
camera-relative space and must be shifted back to world; and the far point uses **`ndc_z = 0.5`**, not
the far plane — a real precision fix, not a shortcut.

<svg viewBox="0 0 680 170" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a cursor pixel becomes NDC, is unprojected through the inverse view-projection at two depths into camera-relative points, shifted by the origin into world space, and the line through them is the ray" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="34" width="96" height="30" fill="none" stroke="#6fb3ff"/><text x="58" y="53" fill="#d7dae0" text-anchor="middle">cursor px</text>
  <rect x="132" y="34" width="96" height="30" fill="none" stroke="#6fb3ff"/><text x="180" y="49" fill="#d7dae0" text-anchor="middle">NDC</text><text x="180" y="60" fill="#666" text-anchor="middle" font-size="9">x,y ∈ [-1,1]</text>
  <rect x="254" y="34" width="140" height="30" fill="none" stroke="#6fb3ff"/><text x="324" y="49" fill="#d7dae0" text-anchor="middle">inverse(view_proj)</text><text x="324" y="60" fill="#666" text-anchor="middle" font-size="9">z=1.0 near · z=0.5 far</text>
  <rect x="420" y="34" width="120" height="30" fill="none" stroke="#6fb3ff"/><text x="480" y="49" fill="#d7dae0" text-anchor="middle">+ origin</text><text x="480" y="60" fill="#666" text-anchor="middle" font-size="9">cam-rel → world</text>
  <rect x="566" y="34" width="104" height="30" fill="none" stroke="#6fb3ff"/><text x="618" y="53" fill="#d7dae0" text-anchor="middle">world ray</text>
  <g stroke="#6fb3ff" stroke-width="1.4">
    <line x1="106" y1="49" x2="130" y2="49" marker-end="url(#ah41)"/>
    <line x1="228" y1="49" x2="252" y2="49" marker-end="url(#ah41)"/>
    <line x1="394" y1="49" x2="418" y2="49" marker-end="url(#ah41)"/>
    <line x1="540" y1="49" x2="564" y2="49" marker-end="url(#ah41)"/>
  </g>
  <text x="340" y="104" fill="#888" text-anchor="middle">ray.origin = world near point · ray.dir = normalize(world far − world near)</text>
  <text x="340" y="126" fill="#666" text-anchor="middle">two unprojected depths define the line; the far one avoids the ill-conditioned far plane</text>
  <defs><marker id="ah41" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
# NEW — Ray { origin, dir }; screen_to_world_ray(view_proj, origin, cursor, viewport)
src/engine/pick.rs
src/engine/mod.rs    # pub mod pick;
# cursor field + on_left_click: build a ray, intersect z=0, log the hit (the verify)
src/state.rs
src/lib.rs           # CursorMoved stashes state.cursor; a Left MouseInput arm calls it
```

`engine/pick.rs` names only `Point`/`Vector`/`Xform` — geometry primitives, never `Session`/`Mesh` — so
it stays on the engine side of 35's litmus. The *dispatch* ("which object did the ray hit?") is app-layer
and arrives in 47.

## Step 1 — the ray type + unproject: `src/engine/pick.rs`

```rust
use session_rust::{Point, Vector, Xform};

/// A world-space ray. `dir` is normalized. (Clone only — kernel Point/Vector aren't Copy.)
#[derive(Clone)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vector,
}

/// Unproject a cursor pixel into a world ray. `view_proj` is the camera-relative matrix (33) and
/// `origin` its rebase point, so the unprojected points come out camera-relative and get shifted to
/// world by `+ origin`. `viewport` = (x, y, w, h) in physical pixels.
pub fn screen_to_world_ray(view_proj: &Xform, origin: &Point, cursor: (f64, f64),
                           viewport: (f64, f64, f64, f64)) -> Option<Ray> {
    let (vx, vy, w, h) = viewport;
    let ndc_x = ((cursor.0 - vx) / w) * 2.0 - 1.0;
    let ndc_y = 1.0 - ((cursor.1 - vy) / h) * 2.0;   // pixel-y is top-down; NDC-y is bottom-up

    // full 4×4 kernel inverse — see the history below
    let inv = view_proj.inverse()?.to_cols();
    // camera-relative → world (Point + Vector)
    let shift = Vector::new(origin[0], origin[1], origin[2]);
    // Reverse-Z (26): the NEAR plane is ndc_z = 1.0, the far plane ndc_z = 0.0. Unproject near
    // at 1.0 and a well-conditioned mid-depth at 0.5 (NOT the far plane — see Step 2).
    let near = unproject(&inv, ndc_x, ndc_y, 1.0)? + shift;
    let far  = unproject(&inv, ndc_x, ndc_y, 0.5)? + shift;

    let dir = (far - near).normalized();             // Point − Point → Vector, then unit length
    Some(Ray { origin: near, dir })
}

/// NDC (with a z) → a point, via the inverse view-projection and the perspective divide.
fn unproject(m: &[[f64; 4]; 4], x: f64, y: f64, z: f64) -> Option<Point> {
    let v = [x, y, z, 1.0];
    let row = |r: usize| m[0][r]*v[0] + m[1][r]*v[1] + m[2][r]*v[2] + m[3][r]*v[3];   // (M · v)[r]
    let w = row(3);                                  // homogeneous w
    if w.abs() < 1e-12 { return None; }              // w≈0 → point at infinity, unusable
    Some(Point::new(row(0)/w, row(1)/w, row(2)/w))   // perspective divide
}
```

> **A bug this lesson found in the kernel.** `Xform::inverse()` used to be **affine-only** — it
> inverted the 3×3 block + translation, silently assuming a `[0,0,0,1]` bottom row. A *perspective*
> `view_proj` has a projective bottom row, so its "inverse" was simply wrong — rays landed nowhere
> near the cursor (the archive hit the identical bug and carried a private `mat4_inverse` forever).
> Writing this lesson exposed it, and the kernel now ships a **full cofactor 4×4 inverse** in all
> three languages, with a `P·P⁻¹ = I` perspective check in the minitest — so the plain
> `view_proj.inverse()` above just works. If you're on a kernel predating that fix: verify with a
> perspective matrix before trusting any unprojection. See `_KERNEL_GAPS.md` for the audit trail.
>
> `Point + Vector`, `Point − Point → Vector`, and `Vector::normalized()` are all verified kernel ops.

> **A ray is infinite; the downstream casts are not.** 47 and 49 turn this ray into a *segment* by
> unprojecting a far point at `origin + dir · 1.0e7` — anything farther than 10⁷ world units down
> the ray is unpickable. That's 10 km in millimetre units, so ordinary CAD never notices; if your
> scene ever works in microns (10⁷ µm = 10 m!) or kilometres, raise the cap or derive it from the
> scene bounds. It lives in the cast functions, not here — flagged here so the constant doesn't
> surprise you later.

## Step 2 — why `ndc_z = 0.5`, not the far plane

The obvious choice for the ray's second point is the far plane. It's a **trap**, and a real archive bug
(`project_picking_bug_fix`). A perspective projection crams almost all depth precision near the camera;
at the far plane the inverse matrix's `w`-row denominator collapses toward zero (`far/near` is often
10⁴–10⁶), so `c[3]` is tiny and `c/c[3]` explodes — the unprojected far point jitters by metres as the
cursor moves a pixel, and the ray direction is garbage.

`ndc_z = 0.5` sits in the well-conditioned middle of the depth range: `c[3]` stays comfortably away from
zero, the divide is stable, and the point is still far enough from `near` to define the direction
cleanly. The ray it yields is identical in direction (any two distinct points on the line give the same
ray) but numerically solid. **Never unproject at the exact far plane for a pick ray.**

> **Ortho.** An orthographic projection has no perspective divide (`w` is constant), so the far plane is
> fine there — but a symmetric ortho frustum (near = +N, far = −N, per `camera.rs`'s `orthographic(…, r, -r)`) puts the *camera plane* at `ndc_z`
> mid-range, so unproject near at `0.5` and far at `1.0`. If you carry a `ProjMode` (16), branch the two
> `ndc_z` values on it; the rest of the function is identical.

## Step 3 — wire it up + the self-check: `src/state.rs` + `src/lib.rs`

Track the cursor, and on a left-click build the ray and intersect it with the ground plane `z = 0` —
if the ray is right, the hit lands exactly under the cursor from every camera angle.

**3a. The cursor lands on `State`** (the pick code lives there; `App` in lib.rs only forwards) —
add to `struct State` and initialize in `State::new`'s `Ok(Self { … })`:

```rust
// in `struct State`:
    pub cursor: (f64, f64),      // latest cursor position, physical pixels
// in `State::new`'s Ok(Self { … }), next to the other field inits:
    cursor: (0.0, 0.0),
```

Mouse events arrive in **lib.rs** (`App::window_event`). In the existing
`WindowEvent::CursorMoved` arm — right beside the `self.last_cursor = …` line it already ends
with — stash the position on `State` too:

```rust
                state.cursor = (position.x, position.y);   // physical pixels, for picking
```

and add a left-button arm next to the existing `MouseButton::Right` one:

```rust
            WindowEvent::MouseInput { state: btn, button: MouseButton::Left, .. } => {
                if btn == ElementState::Pressed { state.on_left_click(); }
            }
```

**3b. The click handler itself** — add to `impl State` (in `state.rs`), together with the small
`aspect()` helper every pick lesson from here on reuses:

```rust
    /// Viewport aspect ratio — picking and projection share it from here on.
    pub fn aspect(&self) -> f64 {
        self.gpu.config.width as f64 / self.gpu.config.height as f64
    }

    /// Left-click entry point (wired from lib.rs). Today: the ground-plane self-check;
    /// 47 replaces the z=0 block with the real pick.
    pub fn on_left_click(&mut self) {
        let vp = self.camera.view_proj(self.aspect());
        let origin = self.camera.origin();
        let viewport = (0.0, 0.0, self.gpu.config.width as f64, self.gpu.config.height as f64);
        if let Some(ray) =
            crate::engine::pick::screen_to_world_ray(&vp, &origin, self.cursor, viewport) {
            // intersect with z = 0: origin.z + t·dir.z = 0
            if ray.dir[2].abs() > 1e-9 {
                let t = -ray.origin[2] / ray.dir[2];
                if t > 0.0 {
                    let hit = &ray.origin + &ray.dir * t;   // borrow: Point/Vector aren't Copy
                    log::info!("grid hit: ({:.1}, {:.1}, {:.1})", hit[0], hit[1], hit[2]);
                    // (optional) drop a Point marker at `hit` through 32's glyph path to see it
                }
            }
        }
    }
```

> **Drawn vs. picked matrix.** `render` draws with the *anchored* matrix — `view_proj_anchored(aspect,
> &anchor)`, anchor from `rebase_anchor` (34c) — while the pick above uses `view_proj(aspect)` +
> `origin()`: the same map, just rebased at the camera origin, so the `+ origin` shift lands the ray in
> world exactly. Only if you ever unproject the *anchored* matrix must you shift by that `anchor`
> instead of `origin`.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Click a spot on the grid; the logged `grid hit` should sit where you clicked. Now the real test —
**orbit to a steep, off-axis view and click the same grid intersection**: the coordinates must still
match the world point under the cursor. If they drift as you orbit, the `+ origin` shift (Step 1) is
missing or the `ndc_z` convention is wrong for your depth setup.

- **Off by a constant that grows with distance from world origin** → the camera-relative `+ origin`
  shift isn't applied; the ray is in cam-relative space.
- **Fine head-on, wildly wrong at grazing angles / far clicks** → you unprojected at the far plane
  instead of `0.5` (Step 2).
- **Y-flipped** (clicking top hits bottom) → the `ndc_y = 1.0 − …` line; pixel-y is top-down.

And a `#[cfg(test)]` pins the math without a browser — unproject a cursor pixel, project a point on
the ray back through the same matrix, assert it lands on the cursor. Add at the bottom of
`engine/pick.rs` (the forward projection is written out inline; 48 packages it as
`project_to_screen`):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unproject_project_round_trip() {
        let cam = crate::camera::Camera::new();   // 13's default: iso view, perspective, mm
        let vp = cam.view_proj(800.0 / 600.0);
        let origin = cam.origin();
        let viewport = (0.0, 0.0, 800.0, 600.0);
        let m = vp.to_cols();
        for cursor in [(40.0, 30.0), (400.0, 300.0), (799.0, 599.0)] {
            let ray = screen_to_world_ray(&vp, &origin, cursor, viewport).unwrap();
            for t in [0.0, 1.0, 100.0] {
                let p = &ray.origin + &ray.dir * t;
                // world → cam-relative → clip → NDC → px (the mirror of Step 1)
                let v = [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2], 1.0];
                let row = |r: usize| m[0][r]*v[0] + m[1][r]*v[1] + m[2][r]*v[2] + m[3][r]*v[3];
                let w = row(3);
                let px = ((row(0)/w * 0.5 + 0.5) * 800.0, (0.5 - row(1)/w * 0.5) * 600.0);
                assert!((px.0 - cursor.0).abs() < 1e-3 && (px.1 - cursor.1).abs() < 1e-3,
                    "cursor {cursor:?} round-tripped to {px:?}");
            }
        }
    }
}
```

Same wasm-override to run headless: `cargo test -p session_viewer round_trip --target
x86_64-unknown-linux-gnu`.

## Recap

```
Ch 45: watch — external file edits reconcile back in.
Ch 46: SCREEN → RAY. A cursor pixel → NDC (x,y ∈ [-1,1], y flipped for top-down pixels) → unproject
       through inverse(view_proj) at TWO depths → camera-relative points → + origin → WORLD
       (the ray). view_proj is camera-relative (33), so the unproject lands cam-relative and the
       origin shift is mandatory — miss it and the ray drifts with distance from world (0,0,0).
       The far point uses ndc_z = 0.5, NOT the far plane: at the far plane the perspective
       inverse's w-denominator collapses (far/near ~ 1e5) and the point explodes — a real archive
       bug. ray = { origin: world-near, dir: normalize(world-far − world-near) }. Reverse-Z (26)
       puts near at ndc_z=1.0; ortho flips the two z's. Lives in engine/pick.rs (names only
       Point/Vector/Xform — engine-side of 35's litmus).
```

Edited: `engine/pick.rs` (NEW — `Ray`, `screen_to_world_ray`, `unproject`, round-trip
`#[cfg(test)]`), `engine/mod.rs`
(`pub mod pick;`), `state.rs` (`cursor` field, `aspect()`, `on_left_click` — ray → z=0 self-check),
`lib.rs` (cursor stash + Left-button arm).

## Next

`47-raycast-meshes.md` — cast this ray at the meshes. Broad-phase with the 40 BVH to a short candidate
list, then for each candidate **inverse-transform the ray into the object's local frame** — the
placement is the row's stored xform, `scene.tables.objects[row].0` (manifest `place` × session world
xform, baked at `add_file`); 47 inverts exactly that — and hit its cached triangle BVH
(`Mesh::triangle_bvh_ray_cast`) — the nearest
`t` wins, and an occluded object never does. WebGPU has no sync depth readback, so this CPU ray + BVH
*is* the interactive pick path.
