# 17 Quaternion turntable

Orientation is `yaw`/`pitch`, clamped to ±1.5 rad to dodge the poles — the tell that **Euler angles
have a singularity** (gimbal lock: yaw/roll collapse looking straight down, why Top/Bottom in lesson
14 sat off vertical). Fix: a **quaternion**, already a *complete* frame — read right/up/forward
**straight off it**, no pole, no clamp. Also switching to **Z-up** (CAD): turntable spins about world
`+Z`. Signatures stay put — **only `camera.rs` changes**.

## Why

The orientation quaternion *is* the camera's frame — apply it to the basis vectors for the axes
directly, no cross products:

```
right = orientation · [1, 0, 0]
up    = orientation · [0, 0, 1]
fwd   = orientation · [0, 1, 0]        (camera sits on −Y, looking +Y, when orientation = identity)
eye   = target − distance · fwd
```

No pole, so orbiting over the top is continuous — no flip, no lock, **no `last_right`**. Orbit only
injects **yaw about world-up** and **pitch about the camera's own right**, never roll: `right` stays
horizontal by construction.

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

Bring in `Quaternion`, replacing `yaw`/`pitch`. **Store state in f64** as plain **`[f64; 3]`** — not
`[f32;3]` (casts everywhere), not `Point` (a **document object**: `guid`, `name: String`, colour,
width, allocated per `new()`, vs. a camera eye's three numbers). `[f64;3]` is `Copy`, zero-alloc,
default `[0.0; 3]`; it wraps into `Point`/`Vector` only at `look_at` (Step 5). `world_up` is the `+Z`
axis, `position`/`up` *derived*, **no `last_right`**:

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

Already a complete frame, so this is tiny: apply it to the basis vectors, place the eye behind the
target along `fwd`. `rotate_vector` returns a `Vector`; read its components in — no casts, no
per-frame `Point` alloc:

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

Yaw about `world_up`, pitch about the camera's own right (`orientation·[1,0,0]`), composed onto the
current orientation. Only `dx`/`dy` still cast (stay `f32`; signature and `lib.rs` don't change). No
clamp, no pole code:

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

`pan` slides `target` along the camera's right (`orientation·[1,0,0]`) and cached `up`; `zoom` scales
`distance`. Both refresh the derived frame afterward:

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

Projection (lesson 16) is unchanged except `self.distance` is `f64` now. **One boundary**: `[f64;3]`
wraps into `Point`/`Vector` here for `look_at` — f32 cast stays downstream at the GPU upload:

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

Each view is one `from_axis_angle` rotating the default `[0,−d,0]` offset onto the right axis. `up`
reads straight from the quaternion, so every view comes out upright **with no special-casing**:

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

`reset` is still one line (`new()` rebuilds the iso orientation). `fit` is lesson-16 logic on the
`f64` struct now: **drop every `as f32`**, write `target`/`distance` directly, casting only the
`[f32;3]` AABB input. Add `update_position()` at the end:

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

Orbit over the top: lesson 14 jammed at the pole, now it sails through cleanly. Z-up: `5` (Top) looks
down `+Z`, `1` (Front) along `+Y`, named views snap to clean orthographic axes. Pan/zoom/fit(`F`)/
reset(`C`) unchanged.

> Verified against the archive: right/up/fwd come **straight from the quaternion**, no pole, no
> `last_right` band needed. The archive re-levels every frame via `right = fwd × world_up`, needing an
> ~11° pole band to cancel roll-drift; we rely on periodic `.normalized()` instead.

## Recap

```
Ch 14–16: orientation was (yaw, pitch) with a pitch clamp — a gimbal singularity at the poles.
Ch 17: orientation is a quaternion that encodes the whole frame; update_position() reads
       right/up/fwd straight off it (no pole, no last_right). State is lightweight f64
       ([f64;3] + f64) — Point is a heavy document object, so we wrap into it only at the
       look_at boundary; f32 cast once at the GPU upload. Orbit composes yaw·pitch quaternions;
       named views single rotations, upright for free. Z-up turntable. Only camera.rs changed.
```

Edited: `camera.rs` only (f64 `[f64;3]`/`f64` state + quaternion `orientation` + `world_up` replace
`f32` `yaw`/`pitch`; `new`, `update_position`, `orbit`, `pan`, `zoom`, `view_proj`, `set_view`, `fit`).
Signatures unchanged, so `lib.rs`/`state.rs` are untouched.

## Next

Phase 2 — **real geometry**. `18-index-buffer.md` draws a cube from indices (DrawIndexed), then `19`
links the `session_rust` kernel and renders your first real `Mesh`. The camera is done.
