# 17 Quaternion turntable

Our camera stores orientation as two angles, `yaw` and `pitch`, and clamps pitch to ±1.5 rad so it
can't flip at the poles. That clamp is the tell: **Euler angles have a singularity.** Look straight
down and "yaw" and "roll" collapse into the same motion (gimbal lock), which is why Top/Bottom in
lesson 14 had to sit a hair under vertical. The fix the whole industry uses is to store orientation
as a **quaternion** — a singularity-free rotation. The key insight: a quaternion already encodes a
*complete* frame (right, up, forward), so we read those axes **straight off it** — there is no pole
to special-case, no up-vector to reconstruct, no clamp. This is the last piece of the archive-grade
camera. We also adopt the kernel's **Z-up** convention (CAD standard): the turntable spins about
world `+Z`. Every public method keeps its signature, so **only `camera.rs` changes**.

## Why

The orientation quaternion *is* the camera's frame. Apply it to the basis vectors and you get the
axes directly — no cross products, no degeneracy:

```
right = orientation · [1, 0, 0]
up    = orientation · [0, 0, 1]
fwd   = orientation · [0, 1, 0]        (camera sits on −Y, looking +Y, when orientation = identity)
eye   = target − distance · fwd
```

Because a quaternion has no pole, orbiting straight over the top is just a continuous rotation — the
view sails over with no flip, no lock, and **no `last_right` reference vector** to track. The frame
stays upright *by construction*: orbit only ever injects **yaw about world-up** and **pitch about the
camera's own right**, and neither introduces roll, so `right` stays horizontal forever.

```
yaw   → rotate orientation about world_up (+Z)     keeps horizon level (turntable, not trackball)
pitch → rotate orientation about right = orientation·[1,0,0]
named → one from_axis_angle per view; up comes out correct for free (a Z-rotation fixes +Z)
```

## Files we touch

```
src/camera.rs   # the whole camera: quaternion orientation replaces yaw/pitch (one file)
```

## Step 1 — new fields, new `new()`: `src/camera.rs`

Bring in `Quaternion`, then replace `yaw`/`pitch` with the orientation quaternion. **Store the state
in f64**, as plain **`[f64; 3]`** arrays — *not* `[f32;3]` (forces a cast on every line) and *not*
`Point`/`Vector`. A `Point` in this kernel is a **document object** — it carries a `guid`, a
`name: String`, a colour and width, and allocates that String on every `new()`. A camera eye is none
of those things; it's three numbers recomputed each frame. So `[f64;3]` is the right fit: `Copy`,
zero-alloc, and the default is just `[0.0; 3]` (no `Point::origin` to reach for — a point has no
intrinsic zero anyway; that's the affine-vs-vector distinction). We wrap into `Point`/`Vector` only at
the `look_at` boundary (Step 5). `world_up` is the `+Z` turntable axis; `position`/`up` are *derived*.
There is **no `last_right`** — the quaternion makes it unnecessary:

```rust
use session_rust::{Point, Quaternion, Vector, Xform};

pub struct Camera {
    pub target:      [f64; 3],     // f64 throughout — cast to f32 only at the GPU upload
    pub distance:    f64,
    pub orientation: Quaternion,   // source of truth — a full, singularity-free frame
    pub world_up:    [f64; 3],     // +Z turntable axis (yaw rotates about this)
    pub position:    [f64; 3],     // derived by update_position()
    pub up:          [f64; 3],     // derived by update_position()
    pub perspective: bool,
    pub unit:        Unit,
}

impl Camera {
    pub fn new() -> Self {
        use std::f64::consts::{FRAC_PI_4, FRAC_PI_6};
        // iso start: yaw 45° about +Z, then pitch −30° about the tilted right axis
        let yaw_q   = Quaternion::from_axis_angle(Vector::z_axis(), FRAC_PI_4);
        let rv      = yaw_q.rotate_vector(Vector::x_axis());
        let pitch_q = Quaternion::from_axis_angle(rv, -FRAC_PI_6);
        let orientation = (pitch_q * yaw_q).normalized();

        let mut cam = Self {
            target: [0.0; 3],
            distance: 3.0,
            orientation,
            world_up: [0.0, 0.0, 1.0],
            position: [0.0; 3],
            up:       [0.0, 0.0, 1.0],
            perspective: true,
            unit: Unit::Millimeters,            // DEFAULT UNIT — see lesson 16
        };
        cam.update_position();                  // fill in position + up from the orientation
        cam
    }
```

## Step 2 — derive the frame: `update_position`

The orientation already holds a complete frame, so this is now tiny — apply it to the basis vectors
and place the eye behind the target along `fwd`. `rotate_vector` returns a `Vector`; we read its
components into the `f64` arrays — no casts, no per-frame `Point` allocation:

```rust
    /// Recompute `position` and `up` from the orientation. Call after any change to
    /// orientation / distance / target. No pole handling — the quaternion has no pole.
    pub fn update_position(&mut self) {
        let fwd = self.orientation.rotate_vector(Vector::y_axis());  // eye → target
        let up  = self.orientation.rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.position[i] = self.target[i] - fwd[i] * self.distance;
            self.up[i] = up[i];
        }
    }
```

## Step 3 — orbit by quaternion: `orbit`

Yaw about `world_up`, pitch about the camera's own right (`orientation·[1,0,0]`), compose onto the
current orientation. The only casts left are the two mouse-delta scalars (`dx`/`dy` stay `f32` so the
public signature — and `lib.rs` — don't change). No clamp, no pole code:

```rust
    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let wu      = Vector::new(self.world_up[0], self.world_up[1], self.world_up[2]);
        let right   = self.orientation.rotate_vector(Vector::x_axis());
        let yaw_q   = Quaternion::from_axis_angle(wu,    (-dx * 0.005) as f64);
        let pitch_q = Quaternion::from_axis_angle(right, (-dy * 0.005) as f64);

        self.orientation = (yaw_q * (pitch_q * self.orientation.duplicate())).normalized();
        self.update_position();
    }
```

## Step 4 — pan & zoom follow the derived frame

`pan` slides `target` along the camera's right (`orientation·[1,0,0]`) and the cached `up`; `zoom`
scales `distance`. Both refresh the derived frame afterward:

```rust
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let k = self.distance * 0.0015;
        for i in 0..3 {
            self.target[i] += (-(dx as f64) * right[i] + dy as f64 * self.up[i]) * k;
        }
        self.update_position();
    }

    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance * (1.0 - amount as f64 * 0.1)).clamp(0.2, 100.0);
        self.update_position();
    }
```

## Step 5 — `view_proj` reads the derived eye/up

The projection half (lesson 16) is untouched — except `self.distance` is now `f64`, so its `as f64`
goes away. This is the **one boundary** where we wrap the `[f64;3]` arrays into `Point`/`Vector` to
call `look_at` — three constructions per frame, at the edge, not in the math. Still all f64; the f32
cast is downstream at the GPU upload:

```rust
        // … perspective / ortho projection exactly as lesson 16 (use `self.distance` directly) …

        let eye    = Point::new(self.position[0], self.position[1], self.position[2]);
        let target = Point::new(self.target[0],   self.target[1],   self.target[2]);
        let up     = Vector::new(self.up[0], self.up[1], self.up[2]);
        let view   = Xform::look_at_right_handed(&eye, &target, &up);

        let s = self.unit.to_meters();
        let scale = Xform::scale_xyz(s, s, s);
        projection * view * scale     // Xform stays f64; the f32 cast is the GPU upload, downstream
```

## Step 6 — named views are single quaternions: `set_view`

Each view is one `from_axis_angle` that rotates the default `[0,−d,0]` offset onto the right axis.
Because `up` is read straight from the quaternion, every view comes out upright with **no
special-casing** — a Z-rotation (Front/Back/Left/Right) leaves `+Z` fixed, so up stays `+Z`
automatically:

```rust
    pub fn set_view(&mut self, view: View) {
        use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_4, FRAC_PI_6};
        let z = Vector::z_axis();
        let x = Vector::x_axis();
        self.orientation = match view {
            View::Front  => Quaternion::from_axis_angle(z, 0.0),        // eye on −Y
            View::Back   => Quaternion::from_axis_angle(z, PI),         // eye on +Y
            View::Right  => Quaternion::from_axis_angle(z, FRAC_PI_2),  // eye on +X
            View::Left   => Quaternion::from_axis_angle(z, -FRAC_PI_2), // eye on −X
            View::Top    => Quaternion::from_axis_angle(x, -FRAC_PI_2), // eye on +Z (looks down)
            View::Bottom => Quaternion::from_axis_angle(x,  FRAC_PI_2), // eye on −Z
            View::Iso    => {
                let yaw_q = Quaternion::from_axis_angle(z, FRAC_PI_4);
                let rv    = yaw_q.rotate_vector(x);
                (Quaternion::from_axis_angle(rv, -FRAC_PI_6) * yaw_q).normalized()
            }
        };
        self.perspective = false;                   // named views are orthographic (lesson 14)
        self.update_position();
    }
```

## Step 7 — reset & fit refresh the frame

`reset` is still one line (`new()` rebuilds the iso orientation). `fit` is the lesson-16 logic, but
the struct is now `f64`, so **drop every `as f32`**: write the `[f64;3]` `target` and `f64` `distance`
directly, and cast the `[f32;3]` AABB to `f64` where it enters (a genuine boundary input). Add
`update_position()` at the end, since it writes `target`/`distance` directly:

```rust
    pub fn reset(&mut self) {
        *self = Camera::new();
    }

    pub fn fit(&mut self, min: [f32; 3], max: [f32; 3], aspect: f64) {
        let s = self.unit.to_meters();                 // f64

        self.target = [
            (min[0] as f64 + max[0] as f64) * 0.5 * s,
            (min[1] as f64 + max[1] as f64) * 0.5 * s,
            (min[2] as f64 + max[2] as f64) * 0.5 * s,
        ];

        let dx = (max[0] as f64 - min[0] as f64) * 0.5 * s;
        let dy = (max[1] as f64 - min[1] as f64) * 0.5 * s;
        let dz = (max[2] as f64 - min[2] as f64) * 0.5 * s;
        let radius = (dx*dx + dy*dy + dz*dz).sqrt();
        if radius <= 0.0 { return; }

        let half_fov_y = f64::to_radians(60.0) * 0.5;
        let half_fov_x = (aspect * half_fov_y.tan()).atan();
        let half_fov   = half_fov_y.min(half_fov_x);   // f64
        self.distance = (radius / half_fov.sin() * 1.1).clamp(0.2, 100.0);
        self.update_position();                        // ← refresh after writing target/distance
    }
```

## Step 8 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Orbit straight up and over the top: in lesson 14 the camera jammed under the pole; now it sails over
cleanly, the scene rotating without a flip — a quaternion simply has no pole. The world is now Z-up,
so press `5` (Top) to look straight down `+Z` and `1` (Front) to look along `+Y`; each named view
snaps to a clean, upright orthographic axis. Pan, zoom, fit (`F`) and reset (`C`) all behave as
before — their signatures never changed.

> Design note (verified native): we read right/up/fwd **straight from the quaternion**, which has no
> pole, so there's no `last_right` band or up-reconstruction. The archive instead re-levels every
> frame via `right = fwd × world_up`, which needs an ~11° `last_right` pole band — that buys
> continuous cancellation of slow roll-drift, at the cost of complexity. We rely on the periodic
> `.normalized()` instead. If drift ever shows, the band approach is the documented fallback.

## Recap

```
Ch 14–16: orientation was (yaw, pitch) with a pitch clamp — a gimbal singularity at the poles.
Ch 17: orientation is a quaternion that encodes the whole frame; update_position() reads right/up/fwd
       straight off it (no pole, no last_right). State is lightweight f64 ([f64;3] + f64) — Point is a
       heavy document object, so we wrap into it only at the look_at boundary; f32 cast once at the GPU
       upload. Orbit composes yaw·pitch quaternions; named views single rotations, upright for free.
       Z-up turntable. Only camera.rs changed.
```

Edited: `camera.rs` only (lightweight f64 `[f64;3]`/`f64` state + quaternion `orientation` + `world_up`
replace the `f32` `yaw`/`pitch` fields; `new`, `update_position`, `orbit`, `pan`, `zoom`, `view_proj`,
`set_view`, `fit`). Public method signatures unchanged, so `lib.rs`/`state.rs` are untouched.

## Next

Phase 2 — **real geometry**. `18-index-buffer.md` draws a cube from indices (DrawIndexed), then
`19` links the `session_rust` kernel and renders your first real `Mesh`. The camera is done.
