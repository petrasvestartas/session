# 02 Camera, grid, mouse, keys and fingers

- You end with a white backdrop and a grey 1 m grid (±5 m, red X, green Y, blue Z) that orbits under the right button, pans under the middle one and zooms toward the wheel; 1-7 snap to the six faces and iso, Space flips the projection, C resets, F fits. On a phone one finger orbits, two fingers pan and pinch, a double tap fits.
- The camera is a quaternion frame plus a target and a distance, so there is no pole and no gimbal lock; every matrix it hands the GPU is built about an anchor, so the lanes of later lessons can sit far from the origin without f32 cancellation.
- The engine half gains its floor: pipelines described as data and built in one place, the two per-frame uniforms every shader reads (mvp, line), a depth attachment cleared for reverse-Z, and the frame list. Lesson 3 adds a lane by adding a field, a layout and a draw; nothing here is rewritten.
- Rendering stays on demand: a frame is asked for only when an input, a message or a resize set `needs_frame`, so a still grid costs nothing.
- The crate does not compile between Step 1 and the end of Step 24; it is typed top down (math, camera, engine floor, State, bindings, shell) and checked once.

<svg viewBox="0 0 720 396" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="The shell on top: lib.rs routes events to Input, index.html hands the fingers to the canvas, Cargo.toml links the kernel. Below the line the two halves: app/ (input.rs, touch.rs, route.rs) on the left, the empty Upload contract in the middle, engine/ (pipelines, frame, targets, view, backdrop, render, mod) on the right. The root files camera.rs, math.rs and state.rs sit under both; State::render hands Gpu::present a FrameInput" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="ah" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#333"/></marker></defs>
  <g fill="none" stroke="#333">
    <rect x="14" y="12" width="224" height="40"/><rect x="250" y="12" width="224" height="40"/><rect x="486" y="12" width="220" height="40"/>
  </g>
  <g fill="#222">
    <text x="22" y="28">lib.rs — App { state, proxy, input }</text><text x="258" y="28">index.html — the canvas owns the fingers</text><text x="494" y="28">Cargo.toml</text>
  </g>
  <g fill="#666" font-size="9">
    <text x="22" y="44">Msg::Fit · KeyboardInput -&gt; Input::key · rest -&gt; Input::mouse</text>
    <text x="258" y="44">touch-action: none · user-scalable=no · no contextmenu</text>
    <text x="494" y="44">session_rust = ../session_rust · bytemuck</text>
  </g>
  <line x1="14" y1="70" x2="706" y2="70" stroke="#999"/>
  <g fill="#222">
    <text x="14" y="88">app/  the browser side: bindings, fingers, the knob query</text>
    <text x="410" y="88">engine/  wgpu only, never a kernel type</text>
  </g>
  <g fill="none" stroke="#333">
    <rect x="14" y="96" width="304" height="72"/>
    <rect x="14" y="180" width="304" height="60"/>
    <rect x="14" y="252" width="304" height="46"/>
    <rect x="410" y="96" width="296" height="72"/>
    <rect x="410" y="180" width="296" height="60"/>
    <rect x="410" y="252" width="296" height="60"/>
    <rect x="326" y="122" width="72" height="40" stroke-dasharray="3 2"/>
  </g>
  <g fill="#222">
    <text x="22" y="112">app/input.rs — Input { orbiting, panning, ctrl, .. }</text>
    <text x="22" y="196">app/touch.rs — Touches -&gt; Act { None, Moved, Fit }</text>
    <text x="22" y="268">app/route.rs — query(name)</text>
    <text x="418" y="112">pipelines/mod.rs — Target · DepthMode · PipelineDesc · build</text>
    <text x="418" y="196">gpu/frame.rs — FrameInput · FrameCx · Binds · LineUniform</text>
    <text x="418" y="268">gpu/backdrop.rs — BackdropLane: background, grid</text>
    <text x="362" y="138" text-anchor="middle">Upload</text>
  </g>
  <g fill="#666" font-size="10">
    <text x="22" y="128">key: 1-7 views · Space · C reset · F fit_all</text>
    <text x="22" y="142">mouse: RMB orbit · MMB / Ctrl+RMB pan · wheel zoom_at</text>
    <text x="22" y="156">Touch -&gt; touch.event(camera, t, viewport, dpr)</text>
    <text x="22" y="212">1 finger orbit · 2 fingers pan by midpoint, pinch by span</text>
    <text x="22" y="226">double tap Fit · travel divided by the device pixel ratio</text>
    <text x="22" y="284">app/mod.rs — pub mod input, touch, route</text>
    <text x="418" y="128">layouts.rs — Layouts { mvp, line }</text>
    <text x="418" y="142">gpu/buffers.rs — uniform_buffer · bind_group</text>
    <text x="418" y="156">gpu/view.rs — View::from_env · performance.rs — now_ms</text>
    <text x="418" y="212">gpu/targets.rs — Targets { depth, samples } · begin_pass</text>
    <text x="418" y="226">write(): eye + ortho_h solved once, 48 B line block</text>
    <text x="418" y="284">shaders/background.wgsl · shaders/grid.wgsl (50 verts)</text>
    <text x="418" y="298">gpu/render.rs — encode_frame -&gt; scene_list · gpu/mod.rs — Gpu</text>
    <text x="362" y="153" text-anchor="middle" font-size="9">lesson 3</text>
  </g>
  <line x1="14" y1="314" x2="706" y2="314" stroke="#999"/>
  <g fill="none" stroke="#333">
    <rect x="14" y="326" width="224" height="40"/><rect x="250" y="326" width="224" height="40"/><rect x="486" y="326" width="220" height="40"/>
  </g>
  <g fill="#222">
    <text x="22" y="342">camera.rs — Camera</text><text x="258" y="342">math.rs</text><text x="494" y="342">state.rs — State { gpu, camera }</text>
  </g>
  <g fill="#666" font-size="9">
    <text x="22" y="358">orientation: Quaternion · target · distance · fit</text>
    <text x="258" y="358">Mat4 · Aabb::empty · eye_from_view_proj · ortho_h</text>
    <text x="494" y="358">aspect · viewport · fit_all · render</text>
  </g>
  <line x1="596" y1="326" x2="596" y2="312" stroke="#333" marker-end="url(#ah)"/>
  <text x="14" y="386" fill="#666" font-size="10">State::render -&gt; Gpu::present(&amp;FrameInput { view_proj, clear }) -&gt; FrameUniforms::write -&gt; encode_frame -&gt; submit</text>
</svg>

## Step 1 - Link the kernel and the byte casts

- `Camera` speaks the kernel's `Quaternion`, `Vector` and `Xform`, so `session_rust` becomes a path dependency; the crate directory sits next to it, exactly as the final tree does.
- `bytemuck` turns a uniform struct into the bytes `write_buffer` wants; `Location` and `Performance` are the two browser objects the knob query and the clock read.

_Type it._
**Find** in `Cargo.toml`:

```toml
[dependencies]
```

**Add below it:**

```toml
session_rust = { path = "../session_rust" }
```

_Type it._
**Find** in `Cargo.toml`:

```toml
js-sys = "0.3"
```

**Add below it:**

```toml
bytemuck = { version = "1", features = ["derive"] }
```

_Type it._
**Find** in `Cargo.toml`:

```toml
    ] }
```

**Add above it:**

```toml
    "Location",
    "Performance",
```

## Step 2 - Write the shared math

- `Aabb` starts EMPTY (min above max) so a scene can begin with no box and grow one file at a time; `fit` refuses an empty box instead of framing infinity.
- `eye_from_view_proj` and `ortho_half_height` recover the eye and the ortho scale from the matrix alone, so the GPU side never needs the `Camera`; `Mat4` is 16 raw doubles, not a kernel `Xform` (Strings and a guid per copy), because later lessons copy placements by the thousand.

_Paste it._
**Create `src/math.rs`**

```rust
//! Small f64/f32 math shared by the app and the engine: the column-major `Mat4`, point
//! transforms, the f32 `Aabb` with empty semantics, and the two camera facts recovered from
//! a view-projection. Nothing here touches wgpu; the only kernel type named is `Xform`.

use session_rust::Xform;

/// One object's world placement as 16 raw column-major doubles (index = col * 4 + row).
/// Not a kernel `Xform`: that struct carries Strings and a guid and allocates per copy.
pub type Mat4 = [f64; 16];

/// `a * b` in the kernel's column-major convention. Allocates nothing.
pub fn mat_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0.0f64; 16];
    for i in 0..4 {
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + i] * b[j * 4 + k];
            }
            out[j * 4 + i] = sum;
        }
    }
    out
}

/// The GPU edge: f64 world math stays CPU-side, the instance row is f32.
pub fn mat_to_f32(m: &Mat4) -> [f32; 16] {
    std::array::from_fn(|i| m[i] as f32)
}

/// A point through an affine matrix, f32 in and out (the arithmetic runs in f64).
pub fn xform_point(m: &Mat4, p: [f32; 3]) -> [f32; 3] {
    let w = xform_point_f64(m, [p[0] as f64, p[1] as f64, p[2] as f64]);
    [w[0] as f32, w[1] as f32, w[2] as f32]
}

/// A point through an affine matrix in f64.
pub fn xform_point_f64(m: &Mat4, p: [f64; 3]) -> [f64; 3] {
    [
        m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12],
        m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13],
        m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14],
    ]
}

/// The camera's vertical field of view, degrees; the pen math and the shaders' push assume it.
pub const FOVY_DEG: f64 = 60.0;

/// `a * b` for two column-major f32 matrices (the record builder folds mvp x model per cloud).
pub fn mat_mul_f32(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut m = [0.0f32; 16];
    for col in 0..4 {
        for r in 0..4 {
            let mut s = 0.0;
            for k in 0..4 {
                s += a[k * 4 + r] * b[col * 4 + k];
            }
            m[col * 4 + r] = s;
        }
    }
    m
}

/// The length of the matrix's first column: the uniform scale a placement applies.
pub fn mat_scale(m: &[f32; 16]) -> f64 {
    ((m[0] as f64).powi(2) + (m[1] as f64).powi(2) + (m[2] as f64).powi(2)).sqrt()
}

/// An axis-aligned box in f32 with an EMPTY state (min > max), so a scene can start with no
/// box and grow one file at a time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Aabb {
    /// The inverted box nothing has grown yet.
    pub fn empty() -> Self {
        Self { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3] }
    }

    /// True once at least one point went in.
    pub fn is_finite(&self) -> bool {
        self.min.iter().chain(self.max.iter()).all(|v| v.is_finite()) && self.min[0] <= self.max[0]
    }

    /// Widen by one point.
    pub fn grow(&mut self, p: [f32; 3]) {
        for (k, v) in p.iter().enumerate() {
            self.min[k] = self.min[k].min(*v);
            self.max[k] = self.max[k].max(*v);
        }
    }

    /// Widen by another box; an empty box changes nothing.
    pub fn union(&mut self, other: &Aabb) {
        if !other.is_finite() {
            return;
        }
        self.grow(other.min);
        self.grow(other.max);
    }

    /// The box of this box's eight corners through `m` (conservative for rotations).
    pub fn placed(&self, m: &Mat4) -> Aabb {
        let mut out = Aabb::empty();
        if !self.is_finite() {
            return out;
        }
        for c in 0..8u32 {
            let p = [
                if c & 1 == 0 { self.min[0] } else { self.max[0] },
                if c & 2 == 0 { self.min[1] } else { self.max[1] },
                if c & 4 == 0 { self.min[2] } else { self.max[2] },
            ];
            out.grow(xform_point(m, p));
        }
        out
    }

    /// The smallest axis length - a plate's thickness - 0 when empty.
    pub fn thinnest(&self) -> f32 {
        if !self.is_finite() {
            return 0.0;
        }
        (self.max[0] - self.min[0]).min(self.max[1] - self.min[1]).min(self.max[2] - self.min[2])
    }

    /// The diagonal length, 0 when empty.
    pub fn diagonal(&self) -> f32 {
        if !self.is_finite() {
            return 0.0;
        }
        let d = [self.max[0] - self.min[0], self.max[1] - self.min[1], self.max[2] - self.min[2]];
        (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
    }

    /// Whether `p` lies inside (closed box).
    pub fn contains(&self, p: [f64; 3]) -> bool {
        (0..3).all(|k| p[k] >= self.min[k] as f64 && p[k] <= self.max[k] as f64)
    }

}

/// The camera position recovered from the combined view-projection alone: the eye is where
/// clip x, y and w vanish at once, so three rows and one 3x3 solve give it. Orthographic has no
/// eye (rows 0, 1, 3 are dependent); the fallback is the view direction pushed far back.
pub fn eye_from_view_proj(vp: &Xform) -> [f32; 3] {
    let a = [vp[(0, 0)], vp[(0, 1)], vp[(0, 2)], vp[(0, 3)]];
    let b = [vp[(1, 0)], vp[(1, 1)], vp[(1, 2)], vp[(1, 3)]];
    let c = [vp[(3, 0)], vp[(3, 1)], vp[(3, 2)], vp[(3, 3)]];
    let rows = [[a[0], a[1], a[2]], [b[0], b[1], b[2]], [c[0], c[1], c[2]]];
    let rhs = [-a[3], -b[3], -c[3]];
    let d = det3(&rows);

    let norm: f64 = rows.iter().map(|r| (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()).product();
    if d.abs() <= 1e-9 * norm.max(1e-30) {
        let f = [vp[(2, 0)], vp[(2, 1)], vp[(2, 2)]];
        let len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt().max(1e-30);
        return [0, 1, 2].map(|k| (f[k] / len * 1.0e9) as f32);
    }

    let mut eye = [0.0f32; 3];
    for k in 0..3 {
        let mut m = rows;
        for row in 0..3 {
            m[row][k] = rhs[row];
        }
        eye[k] = (det3(&m) / d) as f32;
    }
    eye
}

/// Determinant of a 3x3 given by rows.
fn det3(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// Ortho half-height in world units, 0.0 in perspective. The w row says which projection this
/// is (all zeros = orthographic); row 1 is the y basis scaled by 1/h, so 1/|row1.xyz| is the
/// world half-height, and rotation and the anchor drop out.
pub fn ortho_half_height(vp: &Xform) -> f32 {
    let w2 = vp[(3, 0)].powi(2) + vp[(3, 1)].powi(2) + vp[(3, 2)].powi(2);
    if w2 > 1e-12 {
        return 0.0;
    }
    let r1 = vp[(1, 0)].powi(2) + vp[(1, 1)].powi(2) + vp[(1, 2)].powi(2);
    if r1 <= 1e-30 {
        return 0.0;
    }
    (1.0 / r1.sqrt()) as f32
}
```

## Step 3 - Give the camera a frame

- The orientation is a quaternion and the source of truth; `position` and `up` are derived by `update_position` after every change, so there is no Euler pole and the up vector cannot drift. `new` starts at the iso view, 3 m from the origin, in millimetres.
- The file is built over Steps 3-8; each step adds one group of methods above `update_position`.

_Type it._
**Create `src/camera.rs`**

```rust
use crate::math::{Aabb, FOVY_DEG};
use session_rust::{Point, Quaternion, Vector, Xform};

/// World unit the scene coordinates are expressed in.
#[derive(Clone, Copy, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}

impl Unit {
    /// Scale factor from this unit to meters (mm → 0.001, m → 1.0).
    pub fn to_meters(self) -> f64 {
        match self {
            Unit::Millimeters => 0.001,
            Unit::Meters => 1.0,
        }
    }
}

/// A named standard view direction (the six orthographic faces plus isometric).
#[derive(Clone, Copy)]
pub enum View {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Iso,
}

/// Orbit camera: a quaternion orientation plus a target/distance, from which the
/// eye position and up vector are derived every time anything changes.
pub struct Camera {
    pub target: [f64; 3],
    pub distance: f64,
    pub orientation: Quaternion, // source of truth - a full, singularity-free frame
    pub world_up: [f64; 3],      // +Z turnable axis (yaw rotates about this)
    pub position: [f64; 3],      // derived by update_position
    pub up: [f64; 3],            // derived by update_position
    pub perspective: bool,
    pub unit: Unit,
    /// Scene bounding-sphere radius in metres, set by `fit`. Floors the far plane so zooming
    /// into one detail can never clip the rest of the scene (0 = pure distance-scaled range).
    pub scene_extent: f64,
}

impl Camera {
    /// A camera at the isometric view (45° yaw, −30° pitch), distance 3, perspective, millimeters.
    pub fn new() -> Self {
        use std::f64::consts::{FRAC_PI_6};

        // iso start: yaw 45 deg about T, pitch -30 deg about the tileted right axis
        let yaw_q = Quaternion::from_axis_angle(Vector::z_axis(), -FRAC_PI_6);
        let rv = yaw_q.rotate_vector(Vector::x_axis());
        let pitch_q = Quaternion::from_axis_angle(rv, -FRAC_PI_6);
        let orientation = (pitch_q * yaw_q).normalized();

        let mut cam = Self {
            target: [0.0; 3],
            distance: 3.0,
            orientation,
            world_up: [0.0, 0.0, 1.0],
            position: [0.0; 3],
            up: [0.0, 0.0, 1.0],
            perspective: true,
            unit: Unit::Millimeters,
            scene_extent: 0.0,
        };

        cam.update_position();

        cam
    }

    /// Recompute `position` and `up` from `orientation`/`target`/`distance` — call after any change.
    pub fn update_position(&mut self) {
        let fwd = self.orientation.rotate_vector(Vector::y_axis()); // eye -> target
        let up = self.orientation.rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.position[i] = self.target[i] - fwd[i] * self.distance;
            self.up[i] = up[i];
        }
    }
}

/// `p . v` for a raw point and a kernel vector.
fn dot3(p: &[f64; 3], v: &Vector) -> f64 {
    p[0] * v[0] + p[1] * v[1] + p[2] * v[2]
}
```

## Step 4 - Orbit, pan and zoom

- Orbit yaws about the world Z and pitches about the camera's CURRENT right axis; pan slides the target across the view plane scaled by the distance; zoom is multiplicative, x0.9 per detent, and can approach zero but never reach it.
- `zoom_at` keeps the world point under the cursor fixed: it finds that point on the target plane from the view frame and pulls the target toward it by the same factor the distance shrinks.

_Type it._
**Find** in `src/camera.rs`:

```rust
        cam.update_position();

        cam
    }
```

**Add below it:**

```rust

    /// Orbit by mouse deltas: yaw about `world_up`, pitch about the current right axis.
    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let wu = Vector::new(self.world_up[0], self.world_up[1], self.world_up[2]);
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let yaw_q = Quaternion::from_axis_angle(wu, (-dx * 0.005) as f64);
        let pitch_q = Quaternion::from_axis_angle(right, (-dy * 0.005) as f64);

        self.orientation = (yaw_q * (pitch_q * self.orientation.duplicate())).normalized();
        self.update_position();
    }

    /// Slide the target (and eye) across the view plane by mouse deltas, scaled by distance.
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let k = self.distance * 0.0015;
        for i in 0..3 {
            self.target[i] += (-(dx as f64) * right[i] + dy as f64 * self.up[i]) * k;
        }
        self.update_position();
    }

    /// Dolly in/out by scaling `distance`.
    /// NO range clamp - zoom is multiplicative
    /// x0.9 per detent, so it approaches but never reaches zero
    /// And near/fat planes scale with distances.
    /// Only a not-zero guard remains.
    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance * (1.0 - amount as f64 * 0.1)).max(1.0e-6);
        self.update_position();
    }

    /// CAD zoom: dolly toward the mouse cursor
    /// The point under the mouse stays under the mouse.
    /// The cursor's world point on the target plane is computed from the view frame.
    /// Then the target is pulled toward it by the zoom factor.
    /// 'cursor'/'viewport' in physical px.
    pub fn zoom_at(&mut self, amount: f32, cursor: (f64, f64), viewport: (f64, f64)){
        let new_dist = (self.distance * (1.0 - amount as f64 * 0.1)).max(1.0e-6);
        let k = new_dist / self.distance; // actual factor after the guard
        let ndc_x = 2.0 * cursor.0 / viewport.0 - 1.0;
        let ndc_y = 1.0 - 2.0 * cursor.1 / viewport.1;
        // Frustum half-extents at the target plane
        // ortho h matches perspective at the target
        let half_h = self.distance * (FOVY_DEG * 0.5).to_radians().tan();
        let half_w = half_h * (viewport.0 / viewport.1);
        let right = self.orientation.rotate_vector(Vector::x_axis());
        for i in 0..3 {
            let cursor_off = right[i] * ndc_x * half_w + self.up[i] * ndc_y * half_h;
            self.target[i] += cursor_off * (1.0 - k); // keeps the curson's world point fixed
        }
        self.distance = new_dist;
        self.update_position();
    }
```

## Step 5 - Build the view-projection

- `view_proj_anchored` subtracts a caller-supplied anchor from the eye and the target before `look_at`, so the matrix the GPU multiplies never carries a large translation; this lesson passes the origin, lesson 3 the camera target, and `origin()` gives the target in WORLD units for that.
- Reverse-Z: near and far are swapped in `Xform::perspective`, and the far plane is floored at `dist + 2·scene_extent` so zooming into one detail cannot clip the rest of the scene.

_Type it._
**Find** in `src/camera.rs`:

```rust
        self.distance = new_dist;
        self.update_position();
    }
```

**Add below it:**

```rust

    /// Build the combined `projection · view · unit-scale` matrix for the given aspect ratio.
    /// The camera target in WORLD units (mm), not the internal metres — this is the anchor the
    /// instance table rebases about, and that table holds world coordinates. Mixing the two
    /// units silently disables camera-relative rendering, which is the whole defence against
    /// f32 cancellation when zooming in far from the origin.
    pub fn origin(&self) -> Point{
        let s = self.unit.to_meters();
        Point::new(self.target[0] / s, self.target[1] / s, self.target[2] / s)
    }

    /// Distance from eye to target in WORLD units (mm) — how big the view is, for callers that
    /// must scale a tolerance to the current zoom.
    pub fn distance_world(&self) -> f64 {
        self.distance / self.unit.to_meters()
    }

    pub fn view_proj(&self, aspect: f64) -> Xform {
        self.view_proj_anchored(aspect, &self.origin())
    }

    /// Camera-relative to a caller-supplied ANCHOR instead of the target.
    /// Instances rebased about the same anchor stay valid while the target drifts (pan/zoom)
    /// panning theb costs 1x uniform instead of an instance-table rebuild.
    /// `anchor` is in WORLD units (mm), matching `origin()` and the rebased instance table.
    pub fn view_proj_anchored(&self, aspect: f64, anchor: &Point) -> Xform {
        let dist = self.distance;
        let a = self.unit.to_meters();
        let anchor = Point::new(anchor[0] * a, anchor[1] * a, anchor[2] * a); // world -> metres
        // Far must reach the whole scene, not just 10x the focus distance: zoomed close to one
        // detail, `dist * 10` shrinks below the scene's own extent and everything past it clips -
        // the "scene vanishes when I zoom in" bug. `dist + 2*scene_extent` reaches any scene point
        // from any eye position via the target (triangle inequality); the `max` keeps the plain
        // distance-scaled range whenever it is already wider. Reverse-Z absorbs the larger ratio.
        let far = (dist * 10.0).max(dist + 2.0 * self.scene_extent);
        let projection = if self.perspective {
            //                                          far ↓   near ↓   — swapped (reverse-Z)
            Xform::perspective(FOVY_DEG.to_radians(), aspect, far, dist * 0.01)
        } else {
            let h = dist * (FOVY_DEG * 0.5).to_radians().tan();
            let r = (dist * 100.0).max(dist + 2.0 * self.scene_extent); // same floor as perspective
            Xform::orthographic(-aspect * h, aspect * h, -h, h, r, -r)
        };

        let eye    = Point::new(self.position[0] - anchor[0], self.position[1] - anchor[1], self.position[2] - anchor[2]);
        let target = Point::new(self.target[0]   - anchor[0], self.target[1]   - anchor[1], self.target[2]   - anchor[2]);
        let up     = Vector::new(self.up[0], self.up[1], self.up[2]);
        let view   = Xform::look_at_right_handed(&eye, &target, &up);

        // units
        let s = self.unit.to_meters();
        let scale = Xform::scale_xyz(s, s, s);

        projection * view * scale
    }
```

## Step 6 - Toggle the projection

- Space flips perspective and orthographic; the framed variant refits the perspective camera to the rectangle the ortho view was showing, because perspective divides by depth and content near the eye but off-axis would otherwise present sky.

_Paste it._
**Find** in `src/camera.rs`:

```rust
        self.distance = new_dist;
        self.update_position();
    }
```

**Add below it:**

```rust


    /// Flip between perspective and orthographic projection.
    pub fn toggle_projection(&mut self) {
        self.perspective = !self.perspective;
    }

    /// The Space toggle, but it can never LOSE what the user was looking at. Ortho puts content
    /// on screen no matter how far off the view AXIS it sits and how much NEARER than the target
    /// plane it is (parallel projection); perspective divides by depth, so content near the eye
    /// but off-axis - the thing the user zoomed against, with the target still out on empty grid -
    /// falls outside the 60-degree cone and the swap presents sky. No camera state is "equivalent"
    /// across the swap, so the honest move is to KEEP THE CONTENT, not the numbers: clip the
    /// scene's bounds to the rectangle the ortho view was actually showing (the view-plane rect,
    /// half-height distance*tan30 at the target) and refit the perspective camera to that -
    /// orientation untouched, target recentred on the formerly visible geometry. The other
    /// direction (perspective -> ortho) only ever widens what is visible and keeps the plain flip.
    pub fn toggle_projection_framed(&mut self, bounds: &Aabb, aspect: f64) {
        self.perspective = !self.perspective;
        if !self.perspective || !bounds.is_finite() {
            return;
        }
        let (min, max) = (bounds.min, bounds.max);
        let s = self.unit.to_meters();
        let t = self.origin(); // target, world units
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let up = Vector::new(self.up[0], self.up[1], self.up[2]);
        let fwd = Vector::new(
            (self.target[0] - self.position[0]) / self.distance,
            (self.target[1] - self.position[1]) / self.distance,
            (self.target[2] - self.position[2]) / self.distance,
        );
        let half_h = self.distance * (FOVY_DEG * 0.5).to_radians().tan() / s; // world units
        let half_w = half_h * aspect;
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for k in 0..8u32 {
            let c = [
                (if k & 1 == 0 { min[0] } else { max[0] }) as f64 - t[0],
                (if k & 2 == 0 { min[1] } else { max[1] }) as f64 - t[1],
                (if k & 4 == 0 { min[2] } else { max[2] }) as f64 - t[2],
            ];
            let dx = (c[0] * right[0] + c[1] * right[1] + c[2] * right[2]).clamp(-half_w, half_w);
            let dy = (c[0] * up[0] + c[1] * up[1] + c[2] * up[2]).clamp(-half_h, half_h);
            let dz = c[0] * fwd[0] + c[1] * fwd[1] + c[2] * fwd[2];
            for i in 0..3 {
                let p = t[i] + dx * right[i] + dy * up[i] + dz * fwd[i];
                lo[i] = lo[i].min(p);
                hi[i] = hi[i].max(p);
            }
        }
        let clipped = Aabb { min: [lo[0] as f32, lo[1] as f32, lo[2] as f32], max: [hi[0] as f32, hi[1] as f32, hi[2] as f32] };
        self.fit(&clipped, aspect);
    }
```

## Step 7 - Snap to the named views

- The seven views are seven quaternions about Z or X, and every snap switches to orthographic; reset is `Camera::new()` again.

_Type it._
**Find** in `src/camera.rs`:

```rust
        projection * view * scale
    }
```

**Add below it:**

```rust

    /// Snap the orientation to a named standard view (Front/Top/Iso/…); switches to orthographic.
    pub fn set_view(&mut self, view: View) {
        use std::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI};
        let z = Vector::z_axis();
        let x = Vector::x_axis();
        self.orientation = match view {
            View::Front => Quaternion::from_axis_angle(z, 0.0),
            View::Back => Quaternion::from_axis_angle(z, PI),
            View::Right => Quaternion::from_axis_angle(z, FRAC_PI_2),
            View::Left => Quaternion::from_axis_angle(z, -FRAC_PI_2),
            View::Top => Quaternion::from_axis_angle(x, -FRAC_PI_2),
            View::Bottom => Quaternion::from_axis_angle(x, FRAC_PI_2),
            View::Iso => {
                let yaw_q = Quaternion::from_axis_angle(z, -FRAC_PI_6);
                let rv = yaw_q.rotate_vector(x);
                (Quaternion::from_axis_angle(rv, -FRAC_PI_6) * yaw_q).normalized()
            }
        };

        self.perspective = false;
        self.update_position();
    }

    /// Reset to a fresh default camera.
    pub fn reset(&mut self) {
        *self = Camera::new();
    }
```

## Step 8 - Fit the box

- `fit` frames a box along the camera's OWN axes, not its bounding sphere: every corner has to be inside the frustum, and a corner further from the eye needs proportionally more distance, hence `x / tan + z`.
- `grow_extent` only widens the far-plane floor when a scene streams in after the last fit; `set_unit` chooses the scale `view_proj` applies.

_Paste it._
**Find** in `src/camera.rs`:

```rust
        *self = Camera::new();
    }
```

**Add below it:**

```rust

    /// Frame an AABB: center the target on it and set distance so its bounding sphere fills the FOV (+10%).
    pub fn fit(&mut self, bounds: &Aabb, aspect: f64) {
        if !bounds.is_finite() {
            return;
        }
        let (min, max) = (bounds.min, bounds.max);
        // unit scale
        let s = self.unit.to_meters();

        // target + box center
        self.target = [
            (min[0] as f64 + max[0] as f64) * 0.5 * s,
            (min[1] as f64 + max[1] as f64) * 0.5 * s,
            (min[2] as f64 + max[2] as f64) * 0.5 * s,
        ];

        // Fit along the CAMERA'S OWN AXES, not the box's bounding sphere.
        //
        // The sphere form (radius = half the diagonal, distance = radius / sin(half_fov)) is
        // orientation-free, which sounds like a virtue and is actually why it sits so far back
        // on anything elongated: half the diagonal of a 216 x 58 x 73 m layout is 118 m, while
        // the view only has to cover 108 m across and 29 m up. Measured on the mixed scene it
        // chose 260 m where 122 m frames everything - 2.1x too far. `sin` costs a little more
        // again: it fits a SPHERE tangentially, where a box wants `tan` on its projected extent.
        let half_fov_y = FOVY_DEG.to_radians() * 0.5;
        let half_fov_x = (aspect * half_fov_y.tan()).atan();
        let (tx, ty) = (half_fov_x.tan(), half_fov_y.tan());

        let fwd = self.orientation.rotate_vector(Vector::y_axis());
        let up = self.orientation.rotate_vector(Vector::z_axis());
        let right = self.orientation.rotate_vector(Vector::x_axis());

        // Every corner has to be inside the frustum. A corner `z` further from the eye needs
        // proportionally more distance, hence the `+ z` rather than a max of the two separately.
        let mut distance: f64 = 0.0;
        let mut extent: f64 = 0.0;
        for c in 0..8u32 {
            let p = [
                (if c & 1 == 0 { min[0] } else { max[0] }) as f64 * s - self.target[0],
                (if c & 2 == 0 { min[1] } else { max[1] }) as f64 * s - self.target[1],
                (if c & 4 == 0 { min[2] } else { max[2] }) as f64 * s - self.target[2],
            ];
            let (x, y, z) = (dot3(&p, &right), dot3(&p, &up), dot3(&p, &fwd));
            extent = extent.max((x * x + y * y + z * z).sqrt());
            distance = distance.max(x.abs() / tx + z);
            distance = distance.max(y.abs() / ty + z);
        }
        if extent <= 0.0 {
            return;
        }

        // 5% of breathing room - the old 10% was compensating for a distance that was already
        // twice what it needed to be.
        self.distance = (distance * 1.05).max(1.0e-6); // no upper clamp - it culled big scenes
        self.scene_extent = extent; // far-plane floor, see view_proj_anchored
        self.update_position();
    }

    /// Grow the far-plane floor to cover a scene that streamed in after the last fit.
    /// without touching the view. Same definition as fit's: the farthest scene corner from the target in meters.
    pub fn grow_extent(&mut self, bounds: &Aabb) {
        if !bounds.is_finite() {
            return;
        }
        let (min, max) = (bounds.min, bounds.max);
        let s = self.unit.to_meters();
        let mut extent: f64 = 0.0;
        for c in 0..8u32{
            let p = [
                (if c & 1 == 0 {min[0]} else {max[0]}) as f64 * s - self.target[0],
                (if c & 2 == 0 {min[1]} else {max[1]}) as f64 * s - self.target[1],
                (if c & 4 == 0 {min[2]} else {max[2]}) as f64 * s - self.target[2],
            ];
            extent = extent.max((p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt());
        }
        if extent.is_finite() && extent > self.scene_extent {
            self.scene_extent = extent;
        }
    }

    /// Set the world unit (mm/m) applied by `view_proj`'s scale.
    pub fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
    }
```

## Step 9 - Describe pipelines as data

- `PipelineDesc` names what differs between the viewer's pipelines (shader, entries, bind groups, vertex layouts, topology, colour, depth) and `build` is the only place wgpu is asked for one; a lane makes one base per shader and derives its variants with `with` and `depth`.
- `DepthMode` is reverse-Z throughout: nearer is `Greater`. `Target` (format + sample count) is what a lane rebuilds against when MSAA flips in a later lesson; `ColorWrite` and the vertex layouts grow then.

_Paste it._
**Create `src/engine/pipelines/mod.rs`**

```rust
//! Pipelines are data. `PipelineDesc` names what differs between the viewer's render
//! pipelines and `build` is the only place wgpu is asked for one. Every lane owns its own
//! descs and rebuilds them through `retarget` when the MSAA sample count flips.

pub mod layouts;

pub use layouts::Layouts;

/// Where a pipeline draws: the colour format and the sample count of the pass.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub format: wgpu::TextureFormat,
    pub samples: u32,
}

/// How a pipeline treats depth. Every compare is reverse-Z: nearer is GREATER.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DepthMode {
    /// Write, strict `Greater`: solids and depth-only prepasses.
    Opaque,
    /// Test only, strict `Greater`: the grid.
    ReadOnly,
    /// No test, no write: the background.
    Always,
}

impl DepthMode {
    /// The (write, compare) pair wgpu wants.
    fn state(self) -> (bool, wgpu::CompareFunction) {
        match self {
            DepthMode::Opaque => (true, wgpu::CompareFunction::Greater),
            DepthMode::ReadOnly => (false, wgpu::CompareFunction::Greater),
            DepthMode::Always => (false, wgpu::CompareFunction::Always),
        }
    }
}

/// What a pipeline writes to the colour target.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorWrite {
    /// Overwrite: solids, ids, the backdrop.
    Opaque,
}

impl ColorWrite {
    /// The (blend, write mask) pair wgpu wants.
    fn state(self) -> (Option<wgpu::BlendState>, wgpu::ColorWrites) {
        match self {
            ColorWrite::Opaque => (None, wgpu::ColorWrites::ALL),
        }
    }
}

/// Everything `build` needs for one render pipeline. A lane makes one base per shader and
/// derives its variants with `with` and `depth`.
#[derive(Clone)]
pub struct PipelineDesc<'a> {
    pub label: &'a str,
    pub shader: &'a wgpu::ShaderModule,
    pub vs: &'a str,
    pub fs: &'a str,
    pub groups: &'a [&'a wgpu::BindGroupLayout],
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub topology: wgpu::PrimitiveTopology,
    pub color: ColorWrite,
    pub depth: DepthMode,
}

impl<'a> PipelineDesc<'a> {
    /// A base over `shader` with `vs_main`, opaque colour and opaque depth; the variants
    /// change the label, the fragment entry, the colour mode and the depth mode.
    pub fn new(shader: &'a wgpu::ShaderModule, groups: &'a [&'a wgpu::BindGroupLayout], vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>], topology: wgpu::PrimitiveTopology) -> Self {
        Self { label: "", shader, vs: "vs_main", fs: "fs_main", groups, vertex_buffers, topology, color: ColorWrite::Opaque, depth: DepthMode::Opaque }
    }

    /// The variant `label`, drawn with fragment entry `fs`.
    pub fn with(&self, label: &'a str, fs: &'a str) -> Self {
        let mut d = self.clone();
        d.label = label;
        d.fs = fs;
        d
    }

    /// The same desc with another depth mode.
    pub fn depth(mut self, depth: DepthMode) -> Self {
        self.depth = depth;
        self
    }
}

/// Compile one WGSL source into a module; the caller keeps it and shares it across pipelines.
pub fn module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some(label), source: wgpu::ShaderSource::Wgsl(source.into()) })
}

/// The pipeline layout for `groups`, in slot order.
fn pipeline_layout(device: &wgpu::Device, label: &str, groups: &[&wgpu::BindGroupLayout]) -> wgpu::PipelineLayout {
    let mut slots: Vec<Option<&wgpu::BindGroupLayout>> = Vec::with_capacity(groups.len());
    for g in groups {
        slots.push(Some(*g));
    }
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some(label), bind_group_layouts: &slots, immediate_size: 0 })
}

/// One render pipeline from its description. Everything not in the desc is the same for all
/// of them: one colour target, `Depth32Float`, no cull, no depth bias, fill mode.
pub fn build(device: &wgpu::Device, target: Target, desc: &PipelineDesc) -> wgpu::RenderPipeline {
    let layout = pipeline_layout(device, desc.label, desc.groups);
    let (depth_write, depth_compare) = desc.depth.state();
    let (blend, write_mask) = desc.color.state();

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(desc.label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: desc.shader,
            entry_point: Some(desc.vs),
            buffers: desc.vertex_buffers,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: desc.shader,
            entry_point: Some(desc.fs),
            targets: &[Some(wgpu::ColorTargetState { format: target.format, blend, write_mask })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: desc.topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(depth_write),
            depth_compare: Some(depth_compare),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: target.samples, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}
```

## Step 10 - Shape the two bind groups

- Group 0 is the mvp (vertex only), group 1 the line/pen block (vertex and fragment); every draw in every lesson binds these two first, so they are made once per device and outlive every pipeline and bind group.

_Type it._
**Create `src/engine/pipelines/layouts.rs`**

```rust
//! `Layouts` — every bind-group layout the viewer binds, built once per device. A layout is
//! the SHAPE of a bind group; the buffers live in `gpu/`. Group scheme for every draw:
//! 0 = mvp, 1 = line/pen uniform.

/// One buffer binding, visible to `stages`.
fn buffer_entry(binding: u32, stages: wgpu::ShaderStages, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: stages,
        ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
        count: None,
    }
}

/// One uniform buffer at binding 0.
fn uniform_layout(device: &wgpu::Device, label: &str, stages: wgpu::ShaderStages) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[buffer_entry(0, stages, wgpu::BufferBindingType::Uniform)],
    })
}

/// The bind-group layouts every lane shares.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,
    pub line: wgpu::BindGroupLayout,
}

impl Layouts {
    /// Build every layout once; they outlive any pipeline or bind group made from them.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            mvp: uniform_layout(device, "mvp.layout", wgpu::ShaderStages::VERTEX),
            line: uniform_layout(device, "line.layout", wgpu::ShaderStages::VERTEX_FRAGMENT),
        }
    }
}
```

## Step 11 - Make the buffer helpers

- `uniform_buffer` makes one COPY_DST uniform from any `Pod` and `bind_group` binds buffers in binding order; the file is rewritten around them and `GpuCtx` is unchanged.

_Type it._
**Create `src/engine/gpu/buffers.rs`**

```rust
//! The GPU floor every lane stands on: `GpuCtx` (device + queue) and the two buffer helpers.
//! No lane, no shader and no per-frame state lives here.

use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// The device/queue pair every resource is made with and every write goes through.
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// A uniform buffer holding one `T`, writable every frame.
pub fn uniform_buffer<T: Pod>(device: &wgpu::Device, label: &str, value: &T) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::bytes_of(value),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

/// A bind group over `buffers` in binding order, one entry each.
pub fn bind_group(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, label: &str, buffers: &[&wgpu::Buffer]) -> wgpu::BindGroup {
    let mut entries: Vec<wgpu::BindGroupEntry> = Vec::with_capacity(buffers.len());
    for (i, b) in buffers.iter().enumerate() {
        entries.push(wgpu::BindGroupEntry { binding: i as u32, resource: b.as_entire_binding() });
    }
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor { label: Some(label), layout, entries: &entries })
}
```

## Step 12 - Write the per-frame uniforms

- `FrameInput` is everything a frame needs from the caller: the camera matrix and the clear colour. `FrameUniforms::write` solves the eye and the ortho half-height ONCE per frame and writes the 48 B `LineUniform`; the `const` assert pins the size the shaders mirror.
- `Binds` borrows the two bind groups for one pass, so a lane draw takes one argument; `FrameCx` carries the knobs, the anchor and the framebuffer size.

_Paste it._
**Create `src/engine/gpu/frame.rs`**

```rust
//! The per-frame uniforms every shader reads: the camera matrix (group 0) and the line/pen block
//! (group 1), written once per frame from a `FrameInput`. The eye and the
//! ortho half-height are solved here ONCE and read by the inside test.

use crate::engine::pipelines::Layouts;
use crate::math::{eye_from_view_proj, ortho_half_height, FOVY_DEG};
use session_rust::Xform;
use super::buffers::{bind_group, uniform_buffer, GpuCtx};
use super::view::View;

/// What one frame needs from the caller: the camera and the clear colour.
pub struct FrameInput {
    pub view_proj: Xform,
    pub clear: wgpu::Color,
}

/// What `FrameUniforms::write` needs besides the camera: the knobs, the anchor the instance
/// rows are rebased about, and the framebuffer size in pixels.
pub struct FrameCx<'a> {
    pub view: &'a View,
    pub anchor: [f32; 3],
    pub size: (u32, u32),
}

/// The two bind groups every lane draw needs, borrowed for one pass.
pub struct Binds<'a> {
    pub mvp: &'a wgpu::BindGroup,
    pub line: &'a wgpu::BindGroup,
}

/// The line/pen block (group 1), 48 B - three vec4s; the mirror test checks the shaders' copy.
/// `eye` and `anchor` are in the anchored frame the instance rows use.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineUniform {
    pub thickness: f32, // on-screen pen width, px
    pub proj_y: f32,    // cot(fovy/2) x unit scale
    pub ortho_h: f32,   // ortho half-height x unit scale, 0 = perspective
    pub vp_h: f32,      // framebuffer height, px
    pub vp_w: f32,      // framebuffer width, px
    pub eye: [f32; 3],  // camera position, anchored world units
    pub anchor: [f32; 3],
    pub feather: f32, // antialiasing ramp of the ink lanes, px
}

const _: () = assert!(std::mem::size_of::<LineUniform>() == 48);

/// The two uniform buffers with their bind groups, plus this frame's solved camera facts.
pub struct FrameUniforms {
    mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    pub mvp_group: wgpu::BindGroup,
    pub line_group: wgpu::BindGroup,
    /// This frame's camera matrix as f32.
    pub mvp_f32: [f32; 16],
    /// Ortho half-height this frame (0 = perspective).
    pub ortho_h: f32,
    /// Eye in anchored world units, for the inside test.
    pub eye: [f32; 3],
}

impl FrameUniforms {
    /// The two buffers and bind groups with no camera yet.
    pub fn new(ctx: &GpuCtx, l: &Layouts, size: (u32, u32)) -> Self {
        let mvp_buffer = uniform_buffer(&ctx.device, "mvp.buffer", &Xform::identity().to_f32());
        let line = LineUniform {
            thickness: 2.0,
            proj_y: 1.0,
            ortho_h: 0.0,
            vp_h: size.1 as f32,
            vp_w: size.0 as f32,
            eye: [0.0; 3],
            anchor: [0.0; 3],
            feather: 1.5,
        };
        let line_buffer = uniform_buffer(&ctx.device, "line.buffer", &line);

        let mvp_group = bind_group(ctx, &l.mvp, "mvp.bind_group", &[&mvp_buffer]);
        let line_group = bind_group(ctx, &l.line, "line.bind_group", &[&line_buffer]);

        Self { mvp_buffer, line_buffer, mvp_group, line_group, mvp_f32: [0.0; 16], ortho_h: 0.0, eye: [0.0; 3] }
    }

    /// Per-frame uniforms: camera and the line/pen block. The eye and the
    /// ortho half-height are solved once here and kept for the rest of the frame.
    pub fn write(&mut self, ctx: &GpuCtx, input: &FrameInput, cx: &FrameCx) {
        self.mvp_f32 = input.view_proj.to_f32();
        self.ortho_h = ortho_half_height(&input.view_proj);
        self.eye = eye_from_view_proj(&input.view_proj);
        ctx.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&self.mvp_f32));

        let line = LineUniform {
            thickness: 2.0,
            feather: cx.view.feather_px,
            proj_y: 1.0 / (FOVY_DEG as f32 * 0.5).to_radians().tan() * 0.001,
            ortho_h: self.ortho_h,
            vp_h: cx.size.1 as f32,
            vp_w: cx.size.0 as f32,
            eye: self.eye,
            anchor: cx.anchor,
        };
        ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
    }
}
```

## Step 13 - Open the pass over a depth target

- `Targets` owns the depth attachment at the surface size and opens the ONE render pass; depth clears to 0 because reverse-Z puts far at 0. The `msaa` slot stays `None` until a later lesson asks for 4 samples.
- `TextureSpec` and `texture_view` are the only way a texture is made from here on; nothing in this file knows what is drawn.

_Paste it._
**Create `src/engine/gpu/targets.rs`**

```rust
//! `Targets` - the depth and MSAA colour attachments a frame renders into, sized to the
//! surface at the sample count the scene chose (`samples_for`), and the one render pass that
//! clears them. Nothing here knows what is drawn; it only opens the pass.

use super::buffers::GpuCtx;

/// The attachments of the frame's render pass and the sample count they were made at.
/// `msaa` exists only at 4x.
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: Option<wgpu::TextureView>,
    pub samples: u32,
}

impl Targets {
    /// Both attachments for `size` and `format` at `samples` (1 or 4).
    pub fn new(ctx: &GpuCtx, size: (u32, u32), format: wgpu::TextureFormat, samples: u32) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT;
        let depth = texture_view(ctx, "depth", &TextureSpec { size, format: wgpu::TextureFormat::Depth32Float, samples, usage });
        let msaa = if samples > 1 {
            Some(texture_view(ctx, "msaa_color", &TextureSpec { size, format, samples, usage }))
        } else {
            None
        };

        Self { depth, msaa, samples }
    }

    /// Open the frame's render pass: colour cleared to `clear`, depth cleared to 0 (reverse-Z
    /// far). At 1x the pass draws straight into `view`; at 4x it resolves into it.
    pub fn begin_pass<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView, clear: wgpu::Color) -> wgpu::RenderPass<'a> {
        let (target, resolve) = match &self.msaa {
            Some(msaa) => (msaa, Some(view)),
            None => (view, None),
        };
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: resolve,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }
}

/// What a 2D texture is made of: pixels, format, sample count, and what it is used for.
pub struct TextureSpec {
    pub size: (u32, u32),
    pub format: wgpu::TextureFormat,
    pub samples: u32,
    pub usage: wgpu::TextureUsages,
}

/// A 2D texture to `spec`.
pub fn texture(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> wgpu::Texture {
    ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d { width: spec.size.0.max(1), height: spec.size.1.max(1), depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: spec.samples,
        dimension: wgpu::TextureDimension::D2,
        format: spec.format,
        usage: spec.usage,
        view_formats: &[],
    })
}

/// A 2D texture's default view, the texture itself dropped (wgpu keeps it alive).
pub fn texture_view(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> wgpu::TextureView {
    texture(ctx, label, spec).create_view(&wgpu::TextureViewDescriptor::default())
}
```

## Step 14 - Draw the background and the grid in WGSL

- The background is one triangle three times the screen at the far plane (z = w = 1), so there is no vertex buffer and no seam between two triangles.
- The grid builds its 50 vertices from the vertex index: 22 lines per direction at 1 m over ±5 m, then the three axes; it subtracts `line.anchor` before the mvp, matching the anchored matrix of Step 5. Its `LineUniform` mirrors Step 12 field for field.

_Type it._
**Create `src/shaders/background.wgsl`**

```wgsl
// The background: one fullscreen triangle at the far plane, flat white.

struct VsOut {
    @builtin(position) pos: vec4<f32>,
}

const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(3.0, -1.0),
    vec2<f32>(-1.0, 3.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var o: VsOut;
    o.pos = vec4<f32>(CORNERS[vid], 1.0, 1.0);
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
```

_Type it._
**Create `src/shaders/grid.wgsl`**

```wgsl
// The ground grid and axes: 50 vertices built from the vertex index, no buffer. Authored in
// world millimetres about the origin, minus the camera anchor the instance rows are rebased on.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,
    feather: f32,
};

const STEP: f32 = 1000.0;   // mm per cell
const HALF: f32 = 5000.0;   // +-5 m floor
const N: u32 = 5u;          // cells per side of the centre
const PER_DIR: u32 = 22u;   // (2N + 1) lines x 2 endpoints
const FLOOR: u32 = 44u;     // both directions; axes follow

const GREY: vec3<f32> = vec3<f32>(0.55, 0.55, 0.55);
const RED: vec3<f32> = vec3<f32>(0.85, 0.30, 0.30);
const GREEN: vec3<f32> = vec3<f32>(0.30, 0.70, 0.30);
const BLUE: vec3<f32> = vec3<f32>(0.30, 0.45, 0.85);

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let far = (vid % 2u) == 1u;
    var wp: vec3<f32>;
    var c: vec3<f32>;
    if (vid < FLOOR) {
        let dir = vid / PER_DIR;
        let k = (vid % PER_DIR) / 2u;
        let t = (f32(k) - f32(N)) * STEP;
        let end = select(-HALF, HALF, far);
        wp = select(vec3<f32>(end, t, 0.0), vec3<f32>(t, end, 0.0), dir == 1u);
        c = GREY;
    } else {
        let axis = (vid - FLOOR) / 2u;
        if (axis == 0u) {
            wp = vec3<f32>(select(0.0, HALF, far), 0.0, 0.0);
            c = RED;
        } else if (axis == 1u) {
            wp = vec3<f32>(0.0, select(0.0, HALF, far), 0.0);
            c = GREEN;
        } else {
            wp = vec3<f32>(0.0, 0.0, select(0.0, 1000.0, far));
            c = BLUE;
        }
    }
    var o: VsOut;
    o.pos = mvp * vec4<f32>(wp - line.anchor, 1.0);
    o.color = c;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
```

## Step 15 - Assemble the backdrop lane

- A lane is a struct of pipelines with `new`, `retarget` and one `draw_*` per pipeline that returns its draw count; the background draws with depth `Always`, the grid with `ReadOnly`, so every later object paints over both.

_Type it._
**Create `src/engine/gpu/backdrop.rs`**

```rust
//! The backdrop lane: the fullscreen background triangle and the vertexless 50-vertex grid.
//! No table, no upload; two pipelines and two draws that open every frame.

use crate::engine::pipelines::{build, module, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::GpuCtx;
use super::frame::Binds;
use wgpu::PrimitiveTopology::{LineList, TriangleList};

/// Vertices the grid shader builds from the vertex index: 44 floor + 6 axis.
const GRID_VERTS: u32 = 50;

/// The two backdrop pipelines and their shader modules.
pub struct BackdropLane {
    background_shader: wgpu::ShaderModule,
    grid_shader: wgpu::ShaderModule,
    background: wgpu::RenderPipeline,
    grid: wgpu::RenderPipeline,
}

impl BackdropLane {
    /// Compile both shaders once and build the pipelines for `target`.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let background_shader = module(&ctx.device, "background.shader", include_str!("../../shaders/background.wgsl"));
        let grid_shader = module(&ctx.device, "grid.shader", include_str!("../../shaders/grid.wgsl"));
        let background = build_background(ctx, &background_shader, target);
        let grid = build_grid(ctx, l, &grid_shader, target);

        Self { background_shader, grid_shader, background, grid }
    }

    /// Rebuild both pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.background = build_background(ctx, &self.background_shader, target);
        self.grid = build_grid(ctx, l, &self.grid_shader, target);
    }

    /// The background: one fullscreen triangle, nothing bound. Always 1 draw.
    pub fn draw_background(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        pass.set_pipeline(&self.background);
        pass.draw(0..3, 0..1);
        1
    }

    /// The grid draws before the geometry with depth writes off, so every object paints over
    /// it. The line block carries the anchor it subtracts. Always 1 draw.
    pub fn draw_grid(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        pass.set_pipeline(&self.grid);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.draw(0..GRID_VERTS, 0..1);
        1
    }
}

/// The background pipeline: always drawn, never writes depth.
fn build_background(ctx: &GpuCtx, shader: &wgpu::ShaderModule, target: Target) -> wgpu::RenderPipeline {
    let base = PipelineDesc::new(shader, &[], &[], TriangleList);
    build(&ctx.device, target, &base.with("background", "fs_main").depth(DepthMode::Always))
}

/// The grid pipeline: depth-tested lines, no depth write.
fn build_grid(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> wgpu::RenderPipeline {
    let groups = [&l.mvp, &l.line];
    let base = PipelineDesc::new(shader, &groups, &[], LineList);
    build(&ctx.device, target, &base.with("grid", "fs_main").depth(DepthMode::ReadOnly))
}
```

## Step 16 - List the frame

- `encode_frame` opens the pass and runs `scene_list`, the ordered draws; the order is the contract (depth writers first, blended ink after). It knows no surface, so a headless render in a later lesson calls the same function.

_Type it._
**Create `src/engine/gpu/render.rs`**

```rust
//! The frame list. `encode_frame` runs ONE scene pass whose `scene_list`
//! is the ordered lane draws. The order is the contract: everything that writes depth first,
//! the blended ink after.

use super::frame::Binds;
use super::Gpu;

impl Gpu {
    /// Encode the whole frame into `view`. Returns the draw count.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, clear: wgpu::Color) -> u32 {
        let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group };
        let mut pass = self.targets.begin_pass(encoder, view, clear);
        self.scene_list(&mut pass, &b)
    }

    /// The scene list, in order:
    /// 1 background · 2 grid.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = 0u32;

        draws += self.backdrop.draw_background(pass);
        draws += self.backdrop.draw_grid(pass, b);
        draws
    }
}
```

## Step 17 - Read the knobs and the clock

- `View` reads every runtime knob once at startup, `?aa=` on the web and `VIEWER_AA` natively, through the one query parser in `route.rs`; a knob read per frame would cost a DOM call per frame.
- `now_ms` is `performance.now()` in the browser and the system clock natively, so a timing printed by either build means the same thing.

_Type it._
**Create `src/engine/gpu/view.rs`**

```rust
//! `View` - the runtime knobs a frame reads. Read ONCE at startup from the query string
//! (wasm) or the environment (native). No GPU here.

/// The knobs one frame reads.
pub struct View {
    /// Width of the antialiasing ramp on every ink lane, px (`?aa=` / `VIEWER_AA`): 1 is the
    /// exact box-filter coverage, wider trades a little blur for smoother diagonals.
    pub feather_px: f32,
}

impl View {
    /// Read every knob once.
    pub fn from_env() -> Self {
        Self {
            feather_px: knob_f32("VIEWER_AA", "aa", 1.5).clamp(0.5, 4.0),
        }
    }
}

/// One knob's raw text: the `?name=` query value on wasm, the `ENV` variable natively.
pub fn knob(env: &str, query: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = env;
        crate::app::route::query(query)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = query;
        std::env::var(env).ok()
    }
}

/// A float knob; `default` when unset or unparsable.
fn knob_f32(env: &str, query: &str, default: f32) -> f32 {
    knob(env, query).and_then(|v| v.parse().ok()).filter(|v: &f32| v.is_finite()).unwrap_or(default)
}
```

_Type it._
**Create `src/app/route.rs`**

```rust
//! ONE query parser (`query`) serves every knob.

/// The `?name=` value of this page's query string, percent-decoded.
pub fn query(name: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    let raw = search.strip_prefix('?')?;
    let prefix = format!("{name}=");
    for pair in raw.split('&') {
        if let Some(v) = pair.strip_prefix(prefix.as_str()) {
            return js_sys::decode_uri_component(v).ok()?.as_string();
        }
        if pair == name {
            return Some(String::new());
        }
    }
    None
}
```

_Type it._
**Create `src/engine/performance.rs`**

```rust
//! Clocks: `now_ms` on both targets. Native builds read the system clock.

/// Milliseconds now: `performance.now()` in the browser.
#[cfg(target_arch = "wasm32")]
pub fn now_ms() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

/// Milliseconds now: the system clock natively.
#[cfg(not(target_arch = "wasm32"))]
pub fn now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64() * 1000.0
}
```

## Step 18 - Wire the floor into Gpu

- `Gpu` gains one field per floor piece and one per lane, built in order (layouts, frame uniforms, targets, lane) from the surface's format at 1 sample; `bounds` is the world box everything uploaded will grow, empty for now, and `resize` remakes the size-bound targets.

_Type it._
**Find** in `src/engine/mod.rs`:

```rust
pub mod gpu;
```

**Add below it:**

```rust
pub mod pipelines;
pub mod performance;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
//! `Gpu` - the lowest layer of the viewer: the floor (surface, device), one file each.
//! This file builds the struct; presenting is `present.rs`.

pub mod buffers;
pub mod device;
pub mod present;

use buffers::GpuCtx;
use device::DeviceSetup;
```

**Replace with:**

```rust
//! `Gpu` - the lowest layer of the viewer: the floor (surface, device, layouts, frame
//! uniforms, targets, view knobs) and the lanes, one file each. This file
//! builds the struct; the frame list is `render.rs`, presenting is `present.rs`.

pub mod backdrop;
pub mod buffers;
pub mod device;
pub mod frame;
pub mod present;
pub mod render;
pub mod targets;
pub mod view;

use crate::engine::pipelines::{Layouts, Target};
use crate::math::Aabb;

use backdrop::BackdropLane;
use buffers::GpuCtx;
use device::DeviceSetup;
use frame::FrameUniforms;
use targets::Targets;

pub use frame::FrameInput;
pub use view::View;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
/// Everything on the GPU side of the viewer: the floor.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>,
    pub ctx: GpuCtx,
    pub config: wgpu::SurfaceConfiguration,
}
```

**Replace with:**

```rust
/// Everything on the GPU side of the viewer: the floor, then one field per lane.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>,
    pub ctx: GpuCtx,
    pub config: wgpu::SurfaceConfiguration,
    pub layouts: Layouts,
    pub frame: FrameUniforms,
    pub targets: Targets,
    pub view: View,
    pub backdrop: BackdropLane,
    /// The world box of everything uploaded; the camera fits it.
    pub bounds: Aabb,
}
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Negotiate the device, start empty.
    async fn build(window: Option<std::sync::Arc<winit::window::Window>>, size: (u32, u32)) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, size).await?;
        let ctx = GpuCtx { device, queue };
```

**Replace with:**

```rust
    /// Negotiate the device, make every layout, buffer, bind group and pipeline, start empty.
    async fn build(window: Option<std::sync::Arc<winit::window::Window>>, size: (u32, u32)) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, size).await?;
        let ctx = GpuCtx { device, queue };
        let size = (config.width, config.height);
        let target = Target { format: config.format, samples: 1 };

        let layouts = Layouts::new(&ctx.device);
        let frame = FrameUniforms::new(&ctx, &layouts, size);
        let targets = Targets::new(&ctx, size, config.format, target.samples);
        let backdrop = BackdropLane::new(&ctx, &layouts, target);
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            config,
```

**Add below it:**

```rust
            layouts,
            frame,
            targets,
            view: View::from_env(),
            backdrop,
            bounds: Aabb::empty(),
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Reconfigure the surface.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        if let Some(s) = &self.surface {
            s.configure(&self.ctx.device, &self.config);
        }
    }
```

**Replace with:**

```rust
    /// Reconfigure the surface and remake every size-bound target.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        if let Some(s) = &self.surface {
            s.configure(&self.ctx.device, &self.config);
        }
        self.targets = Targets::new(&self.ctx, (self.config.width, self.config.height), self.config.format, self.targets.samples);
    }
```

## Step 19 - Present through the frame list

- `present` now writes the uniforms, encodes through `encode_frame` and returns the encode time in ms; the surface acquire and the dropped-frame `None` are unchanged. The file is rewritten.

_Type it._
**Create `src/engine/gpu/present.rs`**

```rust
//! How a frame leaves `Gpu`: presented to the swapchain (`present`), which writes the
//! uniforms, encodes through `encode_frame`, and submits.

use super::frame::{FrameCx, FrameInput};
use super::Gpu;

impl Gpu {
    /// Per-frame uniforms.
    fn write_frame_uniforms(&mut self, input: &FrameInput) {
        let cx = FrameCx { view: &self.view, anchor: [0.0; 3], size: (self.config.width, self.config.height) };
        self.frame.write(&self.ctx, input, &cx);
    }

    /// Draw one frame to the swapchain. Returns the encode time in ms, or `None` when the
    /// surface had no texture to give (it was reconfigured; the caller asks for another frame).
    pub fn present(&mut self, input: &FrameInput) -> Option<f64> {
        self.write_frame_uniforms(input);
        let surface = self.surface.as_ref()?;
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => {
                surface.configure(&self.ctx.device, &self.config);
                return None;
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });

        let t0 = crate::engine::performance::now_ms();
        self.encode_frame(&mut encoder, &view, input.clear);
        let encode_ms = crate::engine::performance::now_ms() - t0;
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        Some(encode_ms)
    }
}
```

## Step 20 - Give State a camera

- `State` builds a `Camera` next to the `Gpu` and logs how long the GPU took to come up; `aspect` and `viewport` read the SURFACE, never the window, which is 0x0 on the web.
- `fit_all` frames `gpu.bounds`, which stays empty until lesson 3 uploads something, so F is a no-op today by design.

_Type it._
**Find** in `src/state.rs`:

```rust
//! `State` - the viewer itself: the `gpu` layer and ONE bit of shell
//! state, `needs_frame`. The viewer renders on demand, and this is the demand. Higher
//! layers drive lower ones, never the other way round.

use std::sync::Arc;
use winit::window::Window;
use crate::engine::gpu::Gpu;
```

**Replace with:**

```rust
//! `State` - the viewer itself: the layers (`gpu`, `camera`) and ONE bit of shell
//! state, `needs_frame`. The viewer renders on demand, and this is the demand. Higher
//! layers drive lower ones, never the other way round.

use std::sync::Arc;
use winit::window::Window;
use session_rust::Point;
use crate::camera::Camera;
use crate::engine::gpu::{FrameInput, Gpu};
use crate::engine::performance::now_ms;
```

_Type it._
**Find** in `src/state.rs`:

```rust
    pub gpu: Gpu,
```

**Add below it:**

```rust
    pub camera: Camera,
```

_Type it._
**Find** in `src/state.rs`:

```rust
        let gpu = Gpu::new(window.clone()).await?;
        Ok(Self { window, gpu, needs_frame: true })
    }
```

**Replace with:**

```rust
        let t0 = now_ms();
        let gpu = Gpu::new(window.clone()).await?;
        log::info!("gpu init {:.0} ms", now_ms() - t0);
        Ok(Self { window, gpu, camera: Camera::new(), needs_frame: true })
    }
```

_Type it._
**Find** in `src/state.rs`:

```rust
        Ok(Self { window, gpu, camera: Camera::new(), needs_frame: true })
    }
```

**Add below it:**

```rust

    /// The surface's width over its height (never the window's, which is 0x0 on the web).
    pub fn aspect(&self) -> f64 {
        self.gpu.config.width.max(1) as f64 / self.gpu.config.height.max(1) as f64
    }

    /// The surface size in physical pixels.
    pub fn viewport(&self) -> (f64, f64) {
        (self.gpu.config.width as f64, self.gpu.config.height as f64)
    }

    /// Fit the camera around everything loaded so far.
    pub fn fit_all(&mut self) {
        let b = &self.gpu.bounds;
        log::info!("fit: bounds {:?} .. {:?} aspect {:.3}", b.min, b.max, self.aspect());
        self.camera.fit(&self.gpu.bounds, self.aspect());
        self.needs_frame = true;
    }
```

## Step 21 - Carry the camera into the frame

- `render` builds the anchored matrix about the origin and hands `present` a `FrameInput`; the dropped-frame retry is unchanged.

_Type it._
**Find** in `src/state.rs`:

```rust
    /// The shell asks again when `needs_frame` is set - by a resize.
    pub fn render(&mut self) {
        self.needs_frame = false;
        let drawn = self.gpu.present(CLEAR);
```

**Replace with:**

```rust
    /// The shell asks again when `needs_frame` is set - by an input, a message or a resize.
    pub fn render(&mut self) {
        self.needs_frame = false;
        let view_proj = self.camera.view_proj_anchored(self.aspect(), &Point::new(0.0, 0.0, 0.0));

        let drawn = self.gpu.present(&FrameInput { view_proj, clear: CLEAR });
```

## Step 22 - Bind the mouse and the keys

- Right button orbits, middle (or Ctrl + right) pans, the wheel zooms toward the last cursor position, and every handler returns whether the frame must be redrawn; a `WindowEvent::Touch` is forwarded to `Touches` with the viewport and the device pixel ratio.

_Type it._
**Create `src/app/input.rs`**

```rust
//! Every binding: RMB orbits, MMB (or Ctrl+RMB) pans, the wheel zooms toward the cursor;
//! 1-7 named views, Space projection, C reset, F fit.
//! Fingers go to `touch.rs`.
//! Every handler says whether the frame must be redrawn.

use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, NamedKey};
use crate::camera::View;
use crate::State;
use super::touch::{Act, Touches};

/// What the mouse is doing between events, plus the fingers.
pub struct Input {
    orbiting: bool,
    panning: bool,
    ctrl: bool,
    last_cursor: (f64, f64),
    touch: Touches,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    /// Nothing held, cursor at the origin.
    pub fn new() -> Self {
        Self { orbiting: false, panning: false, ctrl: false, last_cursor: (0.0, 0.0), touch: Touches::new() }
    }

    /// One key press (the caller filters repeats). True when the frame must be redrawn.
    pub fn key(&mut self, state: &mut State, key: Key<&str>) -> bool {
        match key {
            Key::Named(NamedKey::Space) => state.camera.toggle_projection_framed(&state.gpu.bounds, state.aspect()),
            Key::Character("1") => state.camera.set_view(View::Front),
            Key::Character("2") => state.camera.set_view(View::Back),
            Key::Character("3") => state.camera.set_view(View::Left),
            Key::Character("4") => state.camera.set_view(View::Right),
            Key::Character("5") => state.camera.set_view(View::Top),
            Key::Character("6") => state.camera.set_view(View::Bottom),
            Key::Character("7") => state.camera.set_view(View::Iso),
            Key::Character("c" | "C") => state.camera.reset(),
            Key::Character("f" | "F") => state.fit_all(),
            _ => return false,
        }
        true
    }

    /// Buttons, motion, wheel, modifiers and fingers. True when the frame must be redrawn.
    pub fn mouse(&mut self, state: &mut State, event: &WindowEvent) -> bool {
        let viewport = state.viewport();
        match event {
            WindowEvent::MouseInput { state: btn, button: MouseButton::Right, .. } => {
                self.orbiting = *btn == ElementState::Pressed;
                false
            }
            WindowEvent::MouseInput { state: btn, button: MouseButton::Middle, .. } => {
                self.panning = *btn == ElementState::Pressed;
                false
            }
            WindowEvent::CursorMoved { position, .. } => {
                let dragging = self.orbiting || self.panning;
                if dragging {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    if self.panning || self.ctrl {
                        state.camera.pan(dx, dy);
                    } else {
                        state.camera.orbit(dx, dy);
                    }
                }
                self.last_cursor = (position.x, position.y);
                dragging
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 100.0,
                };
                state.camera.zoom_at(amount, self.last_cursor, viewport);
                true
            }
            WindowEvent::ModifiersChanged(mods) => {
                self.ctrl = mods.state().control_key();
                false
            }
            WindowEvent::Touch(t) => match self.touch.event(&mut state.camera, t, viewport, device_pixel_ratio()) {
                Act::None => false,
                Act::Moved => true,
                Act::Fit => {
                    state.fit_all();
                    true
                }
            },
            _ => false,
        }
    }
}

/// Physical pixels per CSS pixel: 1 on a desktop monitor, 2-4 on a phone.
#[cfg(target_arch = "wasm32")]
fn device_pixel_ratio() -> f64 {
    web_sys::window().map(|w| w.device_pixel_ratio()).filter(|d| *d > 0.0).unwrap_or(1.0)
}

/// Native windows report logical pixels already.
#[cfg(not(target_arch = "wasm32"))]
fn device_pixel_ratio() -> f64 {
    1.0
}
```

## Step 23 - Bind the fingers

- winit routes a `"touch"` pointer to `WindowEvent::Touch` and never to the mouse arms, so the two binding sets stay independent; one finger orbits, two pan by their midpoint and pinch by their span, a double tap asks for a fit.
- Travel is divided by the device pixel ratio so a centimetre of glass means the same on every phone, and pan is scaled by the real viewport height so the model stays under the finger; the header comment says why each constant has its value.

_Paste it._
**Create `src/app/touch.rs`**

```rust
//! Touch gestures — the phone half of the camera bindings.
//!
//! winit's web backend splits pointers by `pointerType`: a pointer whose type is `"touch"` is
//! routed to `WindowEvent::Touch` and NEVER to `CursorMoved` / `MouseInput`
//! (`winit-0.30.13/src/platform_impl/web/web_sys/pointer.rs`, the `match pointer_type` arms —
//! the comment there says duplicate mouse events would be "inconsistent with other platforms").
//! So a finger cannot also reach the mouse arms in `lib.rs`, and the two sets of bindings can be
//! read, and changed, independently. (The runner registers a SECOND, unfiltered set of pointer
//! listeners on the window — `event_loop/runner.rs`, "pointermove"/"pointerdown"/"pointerup" —
//! but those raise `DeviceEvent`, which this viewer does not implement, and they return early
//! unless device events are switched on. They are not a second route into anything here.)
//!
//! | gesture | camera | the mouse binding it mirrors |
//! |---|---|---|
//! | one finger, drag | `orbit` | right-drag |
//! | two fingers, slide | `pan` | middle-drag |
//! | two fingers, spread / close | `zoom_at` their midpoint | wheel |
//! | double tap | `fit` | `F` |
//!
//! Two conversions have to happen here, or the same hand movement means different things on
//! different phones.
//!
//! FINGER TRAVEL IS IN CSS PIXELS. winit reports PHYSICAL pixels (`to_physical(scale_factor)`,
//! same file), so one centimetre of glass is three times the number on a dpr-3 phone that it is
//! on a dpr-1 laptop. Orbit is a fixed radians-per-unit, so the raw figure would spin the model
//! three times as fast for the same movement — and differently again on the next phone. Dividing
//! by the device pixel ratio makes the gesture mean one thing everywhere; on a dpr-1 screen it
//! is then exactly the mouse.
//!
//! PAN IS FINGER-EXACT. `Camera::pan` scales its argument by a hard-coded `distance * 0.0015`,
//! which equals the `2·tan(30°)` the projection really spans only when the viewport is 770 px
//! tall — anywhere else the model slides faster or slower than the hand holding it. A mouse does
//! not notice, because the cursor is not the thing being dragged. A finger IS on the thing, so
//! the error reads as the model slipping. Scaling by the real viewport height (`PAN_PER_PX`)
//! removes it, in both projections: the orthographic branch of `view_proj_anchored` uses the
//! same `distance * tan(30°)` half-height as the perspective one.

use winit::event::{Touch, TouchPhase};

use crate::camera::Camera;
use crate::engine::performance::now_ms;

/// `Camera::pan` moves the target by `arg * distance * 0.0015`, while one physical pixel is
/// worth `distance * 2·tan(30°) / viewport_height` of target motion. Their ratio is this
/// constant over the viewport height — the number of pan units one pixel of finger is worth.
const PAN_PER_PX: f64 = 2.0 * 0.577_350_269_189_625_7 / 0.001_5; // 769.8 — pan is exact at that height

/// `Camera::zoom_at` takes a WHEEL DETENT and scales distance by `1 - amount * 0.1`. A pinch
/// gives a ratio `r` instead, so invert it: `1 - amount * 0.1 = 1/r`, hence `PINCH_GAIN`.
/// Spreading the fingers (`r > 1`) shortens the distance, which is zooming in — the same sign
/// as a wheel push.
const PINCH_GAIN: f64 = 10.0;

/// Biggest span change one event may claim. A finger the browser loses and re-delivers, or a
/// third finger landing between two samples, otherwise teleports the camera.
const PINCH_MAX: f64 = 2.0;

/// A finger that never travelled this far (CSS px) and lifted within `TAP_MS` is a tap …
///
/// Both clocks are read when the event is HANDLED, not when it happened — winit's `Touch` carries
/// no timestamp — so a main thread stalled longer than the window turns a real double tap into
/// two singles. At 30-60 fps the stall is a frame and it does not matter; in a BACKGROUND tab,
/// where the browser throttles the frame loop to 1 Hz, it always will. That is a measurement
/// trap, not a bug: a viewer nobody is looking at has no gestures to miss.
const TAP_SLOP: f64 = 12.0;
const TAP_MS: f64 = 300.0;
/// … and a second tap this soon after it, and this near it, is a double tap. Both windows are
/// wider than the single-tap ones: the second tap of a real double tap is the sloppier of the two.
const DOUBLE_TAP_MS: f64 = 320.0;
const DOUBLE_TAP_SLOP: f64 = 40.0;

/// What one touch event asked for. `Fit` needs the scene bounds, which live a layer up, so it is
/// reported rather than done here — this file knows the camera and nothing else.
pub enum Act {
    None,
    Moved,
    Fit,
}

/// One finger, from its `Started` to its `Ended`. Physical pixels throughout.
struct Finger {
    id: u64,
    pos: (f64, f64),  // where it is now
    down: (f64, f64), // where it landed — a tap is a finger that never left this
    t0: f64,          // when it landed, ms
}

/// Every finger on the glass, plus what the last two-finger sample measured.
pub struct Touches {
    fingers: Vec<Finger>,
    /// Distance between the first two fingers at the previous event, and their midpoint.
    /// `span == 0.0` means NOT SEEDED: the next two-finger move records and does nothing else.
    /// Every change in finger count clears it, and that is what stops the model jumping when a
    /// second finger joins or leaves halfway through a gesture.
    span: f64,
    mid: (f64, f64),
    /// When and where the last tap lifted, for the double tap.
    tap: Option<(f64, (f64, f64))>,
}

impl Touches {
    /// No fingers down, no tap pending.
    pub fn new() -> Self {
        Self { fingers: Vec::new(), span: 0.0, mid: (0.0, 0.0), tap: None }
    }

    /// Fold one `WindowEvent::Touch` into the gesture and move the camera. `vp` is the surface
    /// size and `t.location` the finger, both in physical pixels; `dpr` is the device pixel
    /// ratio that turns physical travel back into the CSS pixels a hand feels.
    pub fn event(&mut self, cam: &mut Camera, t: &Touch, vp: (f64, f64), dpr: f64) -> Act {
        let p = (t.location.x, t.location.y);
        match t.phase {
            TouchPhase::Started => {
                self.fingers.push(Finger { id: t.id, pos: p, down: p, t0: now_ms() });
                self.span = 0.0; // the gesture just changed shape — re-seed on the next move
                Act::None
            }
            TouchPhase::Moved => self.moved(cam, t.id, p, vp, dpr),
            TouchPhase::Ended => self.lifted(t.id, p, dpr),
            // A cancel is the browser taking the gesture away — a scroll it decided to own, a
            // system edge swipe, a call arriving. Drop the finger, and never read it as a tap.
            TouchPhase::Cancelled => {
                self.drop_finger(t.id);
                self.tap = None;
                Act::None
            }
        }
    }

    /// A finger travelled. One finger orbits; two pan by their midpoint and zoom by their span.
    fn moved(&mut self, cam: &mut Camera, id: u64, p: (f64, f64), vp: (f64, f64), dpr: f64) -> Act {
        let Some(i) = self.fingers.iter().position(|f| f.id == id) else { return Act::None };
        let d = (p.0 - self.fingers[i].pos.0, p.1 - self.fingers[i].pos.1);
        self.fingers[i].pos = p;

        if self.fingers.len() == 1 {
            cam.orbit((d.0 / dpr) as f32, (d.1 / dpr) as f32);
            return Act::Moved;
        }

        // Three fingers or more still drive the two-finger gesture, off the first two down: a
        // hand resting a third finger on the glass should not stop the pan it is already doing.
        let (a, b) = (self.fingers[0].pos, self.fingers[1].pos);
        let span = (b.0 - a.0).hypot(b.1 - a.1).max(1.0);
        let mid = ((a.0 + b.0) * 0.5, (a.1 + b.1) * 0.5);
        if self.span == 0.0 {
            self.span = span; // first sample of this shape: record, do not act
            self.mid = mid;
            return Act::None;
        }

        // Pan first, then zoom about the NEW midpoint: the pan slides the model with the hand,
        // the zoom then keeps whatever is under the midpoint under it.
        let h = vp.1.max(1.0);
        cam.pan(((mid.0 - self.mid.0) * PAN_PER_PX / h) as f32, ((mid.1 - self.mid.1) * PAN_PER_PX / h) as f32);
        let r = (span / self.span).clamp(1.0 / PINCH_MAX, PINCH_MAX);
        cam.zoom_at((PINCH_GAIN * (1.0 - 1.0 / r)) as f32, mid, vp);

        self.span = span;
        self.mid = mid;
        Act::Moved
    }

    /// A finger left the glass cleanly. Only the LAST one up can be a tap — a lift that leaves
    /// other fingers down is the tail of a two-finger gesture, not a tap on anything.
    fn lifted(&mut self, id: u64, p: (f64, f64), dpr: f64) -> Act {
        let Some(f) = self.drop_finger(id) else { return Act::None };
        if !self.fingers.is_empty() {
            self.tap = None;
            return Act::None;
        }
        let now = now_ms();
        if (p.0 - f.down.0).hypot(p.1 - f.down.1) / dpr > TAP_SLOP || now - f.t0 > TAP_MS {
            self.tap = None; // a drag, or a press held long enough to mean something else
            return Act::None;
        }
        let second = self.tap.take().is_some_and(|(t0, at)| {
            now - t0 < DOUBLE_TAP_MS && (p.0 - at.0).hypot(p.1 - at.1) / dpr < DOUBLE_TAP_SLOP
        });
        if second {
            return Act::Fit; // `self.tap` is already cleared, so three taps are not two doubles
        }
        self.tap = Some((now, p));
        Act::None
    }

    /// Forget one finger and re-seed the pinch, whatever ended it.
    fn drop_finger(&mut self, id: u64) -> Option<Finger> {
        let i = self.fingers.iter().position(|f| f.id == id)?;
        self.span = 0.0;
        Some(self.fingers.remove(i))
    }
}

impl Default for Touches {
    fn default() -> Self {
        Self::new()
    }
}
```

_Type it._
**Create `src/app/mod.rs`**

```rust
//! The app layer: how the viewer is brought up (the loader) and is driven (input,
//! touch). Above the engine, below the shell in lib.rs. Never names a wgpu type.

pub mod input;
pub mod touch;

#[cfg(target_arch = "wasm32")]
pub mod loader;
#[cfg(target_arch = "wasm32")]
pub mod route;
```

## Step 24 - Route events through the shell

- `App` owns an `Input`; a key press (repeats filtered) goes to `Input::key`, every event the shell does not handle itself goes to `Input::mouse`, and a `true` from either sets `needs_frame`.
- `Msg::Fit` is the first message after `Ready`, and every later message takes the same path: unwrap `Ready` first, borrow the `State`, then request a frame if the handler asked for one.

_Type it._
**Find** in `src/lib.rs`:

```rust
//! each delegating to `State`. Loading is `app/loader.rs`.

mod engine;
mod state;
pub mod app;
```

**Replace with:**

```rust
//! each delegating to `State`. Loading is `app/loader.rs`; bindings are `app/input.rs`.

mod camera;
mod engine;
mod state;
pub mod app;
pub mod math;
```

_Type it._
**Find** in `src/lib.rs`:

```rust
/// scene.
pub enum Msg {
    Ready(Box<State>),
}
```

**Replace with:**

```rust
/// scene; everything after it changes the scene in place.
pub enum Msg {
    Ready(Box<State>),
    Fit,
}
```

_Type it._
**Find** in `src/lib.rs`:

```rust
    crate::app::loader,
```

**Replace with:**

```rust
    crate::app::{input::Input, loader},
```

_Type it._
**Find** in `src/lib.rs`:

```rust
    winit::event::WindowEvent,
```

**Replace with:**

```rust
    winit::event::{ElementState, WindowEvent},
```

_Type it._
**Find** in `src/lib.rs`:

```rust
/// The winit application handler: owns `State` once async init completes.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<EventLoopProxy<Msg>>,
}
```

**Replace with:**

```rust
/// The winit application handler: owns `State` once async init completes, and the gestures.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<EventLoopProxy<Msg>>,
    input: Input,
}
```

_Type it._
**Find** in `src/lib.rs`:

```rust
        let app = App { proxy: Some(event_loop.create_proxy()), state: None };
```

**Replace with:**

```rust
        let app = App { proxy: Some(event_loop.create_proxy()), state: None, input: Input::new() };
```

_Type it._
**Find** in `src/lib.rs`:

```rust
    /// The one message, `Ready`: the loader hands over the State.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
        match msg {
            Msg::Ready(state) => self.adopt(*state),
        }
    }
```

**Replace with:**

```rust
    /// Every message after `Ready` changes the scene, so each one leaves `needs_frame` set.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
        let msg = match msg {
            Msg::Ready(state) => return self.adopt(*state),
            other => other,
        };
        let Some(state) = &mut self.state else { return };
        match msg {
            Msg::Ready(_) => {}
            Msg::Fit => state.fit_all(),
        }
        self.request_if_needed();
    }
```

_Type it._
**Find** in `src/lib.rs`:

```rust
    /// Redraw and resize here. A frame is requested only when something changed.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
```

**Replace with:**

```rust
    /// Redraw and resize here; keys and the mouse go to `Input`, which says whether anything
    /// changed. A frame is requested only when something did.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
```

_Type it._
**Find** in `src/lib.rs`:

```rust
            WindowEvent::Resized(_) => true,
            _ => false,
```

**Replace with:**

```rust
            WindowEvent::Resized(_) => true,
            WindowEvent::KeyboardInput { event, .. } => event.state == ElementState::Pressed && !event.repeat && self.input.key(state, event.logical_key.as_ref()),
            other => self.input.mouse(state, &other),
```

## Step 25 - Stop the page fighting the fingers

- `touch-action: none` on the canvas is what delivers the gestures: winit's `preventDefault` is a late veto, and by then the compositor may already have taken the drag as a scroll and sent a `pointercancel`. The viewport meta turns off the page's own pinch-zoom, `overscroll-behavior` the rubber band, and the `contextmenu` listener keeps a right-button orbit from opening a menu.

_Paste it._
**Find** in `index.html`:

```html
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
```

**Replace with:**

```html
  <!-- user-scalable=no: the page's OWN pinch-zoom would otherwise fight the model's. The viewer
       is a canvas, not a document - there is no text to enlarge, and F / double-tap already
       frame the geometry, so nothing is lost by taking the browser's gesture away. iOS Safari
       has IGNORED user-scalable since iOS 10, on purpose (it is an accessibility hazard on real
       documents), which is why the canvas also carries `touch-action: none` below - that one
       Safari does honour, and it is per-element rather than per-page.
       viewport-fit=cover fills the notch area on phones that have one. -->
  <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover"/>
```

_Paste it._
**Find** in `index.html`:

```html
    html, body { height: 100%; }
```

**Replace with:**

```html
    /* overscroll-behavior kills pull-to-refresh and the rubber band: a downward drag on the
       canvas is an orbit, and a page that bounces under it swallows half the gesture. */
    html, body { height: 100%; overscroll-behavior: none; }
```

_Paste it._
**Find** in `index.html`:

```html
    canvas { display: block; width: 100vw; height: 100vh; }
```

**Replace with:**

```html
    /* touch-action: none is what actually delivers the gestures. winit does call preventDefault,
       on `pointerdown` and on a non-passive `touchstart` (winit-0.30.13 web_sys/pointer.rs:115,
       web_sys/canvas.rs:245), but that is a LATE veto: the compositor has already begun deciding
       whether the gesture is a scroll, and where it decides yes the app gets a pointercancel and
       nothing more. touch-action tells it up front, before any of that, and it is the only one
       of the three that iOS Safari applies to page pinch-zoom.
       The rest turns off the things a long press and a double tap mean to a DOCUMENT: the iOS
       callout menu, text selection, the grey tap flash, and the legacy double-tap-to-zoom.
       100dvh after 100vh: dvh follows a mobile toolbar sliding away, vh does not, and browsers
       that never heard of dvh keep the line above. */
    canvas { display: block; width: 100vw; height: 100vh; height: 100dvh;
             touch-action: none; -webkit-touch-callout: none;
             -webkit-user-select: none; user-select: none;
             -webkit-tap-highlight-color: transparent; }
```

_Paste it._
**Find** in `index.html`:

```html
  <script>
```

**Add below it:**

```html
    // Right-drag orbits, so the browser's context menu must not open on top of
    // it. Bound to the canvas only, so a right-click anywhere else on a page
    // embedding this viewer still behaves normally.
    document.addEventListener("DOMContentLoaded", function () {
      var canvas = document.getElementById("canvas");
      if (canvas) canvas.addEventListener("contextmenu", function (e) { e.preventDefault(); });
    });

```

## Run

```bash
trunk serve
```

- Open http://localhost:8770: a white page with a grey 1 m grid and the red, green and blue axes from the iso view, and the console prints `gpu init N ms` after `viewer init OK`.
- Drag with the right button to orbit, the middle button (or Ctrl + right) to pan, roll the wheel to zoom toward the cursor; press 1-7 for the named views, Space to flip the projection, C to reset. Nothing is drawn while the mouse is still.
- On a phone (same Wi-Fi, `http://<your-ip>:8770`, `trunk serve --address 0.0.0.0`): one finger orbits, two pan and pinch, and the page never scrolls, zooms or opens a menu under a gesture.

## Why

- A quaternion frame, not yaw and pitch angles: an Euler camera has a pole where the up vector flips and orbits jump, and every fix is a clamp; a frame rotated by two small quaternions per drag has no such point and derives `up` for free.
- The matrix is built about an anchor because f32 on the GPU keeps about 7 digits: a vertex 100 m out, in millimetres, is already six digits and leaves nothing for sub-millimetre detail, while a vertex a metre from the anchor has digits to spare. Every later lane rebases its rows about the same anchor the grid subtracts.
- Reverse-Z with a floored far plane: a float depth buffer has its precision near 0, so mapping far to 0 spends it where the scene is, and the `dist + 2·scene_extent` floor means the ratio near/far can be enormous without clipping the scene behind the detail you zoomed into.
- Pipelines as data because every lane of lessons 3-7 wants three or four near-identical pipelines; one `build` and a desc that names only the differences keeps a pipeline to a line, and `retarget` rebuilds them all when the sample count changes.
- The eye and the ortho half-height are solved from the matrix once per frame and written into the line block, so no shader and no lane reads the `Camera`; the engine takes a matrix and a clear colour and nothing else.
- Fingers are a separate file from the mouse because winit already splits them and the two never share an event; the file also holds the two conversions (device pixel ratio, viewport height) that make a hand movement mean the same thing on every phone.
- `touch-action` lives in CSS and not in Rust because it is the only veto the compositor honours before it decides what a gesture is; a `preventDefault` from wasm arrives after that decision.
