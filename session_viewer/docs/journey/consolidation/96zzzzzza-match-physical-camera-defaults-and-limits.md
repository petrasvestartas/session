# 21.5 · Physical camera, world grid and framed navigation

**Goal:** Use current wheel units and preserve the visible region when changing from orthographic to perspective. · **Steps:** 6 · **Source:** [camera.rs](../../../src/camera.rs) · [state.rs](../../../src/state.rs) · [engine/gpu/objects.rs](../../../src/engine/gpu/objects.rs) · [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs) · [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs) · [shaders/grid.wgsl](../../../src/shaders/grid.wgsl) · [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl) · [app/cplane.rs](../../../src/app/cplane.rs) (`src/camera.rs,src/app/units.rs,src/engine/gpu/anchor.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 21.5 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera_reference_test…</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera_reference_fixt…</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">6 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/camera.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/units.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Match physical camera def…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. f64 Camera Anchor Subtract</text>
  <rect x="596" y="52" width="208" height="38" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">p_rel = (p_f64 - anchor_f64)</text>
  <rect x="596" y="100" width="208" height="38" rx="5" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Zero f32 Precision Jitter</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Physical camera, world grid…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Unit-Aware Physical Camera</text>
  <rect x="882" y="52" width="188" height="38" rx="5" fill="#dcfce7" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="75" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Units: mm / m / inch</text>
  <rect x="882" y="100" width="188" height="38" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="123" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Physical Viewport Limits</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Match wheel input and fra…</text>
</svg>
</div>

## <a id="checkpoint-96zzzzzza-match-physical-camera-defaults-and-limits"></a>1. Match physical camera defaults and limits

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/camera.rs, src/camera_r…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">File unit</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">physical default and limi…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing camera and named…</text>
</svg>
</div>

**What this code does:** Keep camera defaults and numeric limits in physical units.

### 1. `src/camera.rs`

Reuse the existing isometric recipe and convert the three-metre default once. Bound physical camera travel independently of fitted document size. Retain the current physical minimum distance when fitting tiny originals. Keep the orthographic minimum in metres.

**Locate this existing block:**

```rust
    fn default() -> Self {
        Self {
            unit: Unit::Millimeters,
            target: [0.0; 3],
            distance: 3.0,
            radius: 1.0,
            orientation: Quaternion::from_axis_angle(
                Vector::x_axis(),
                -std::f64::consts::FRAC_PI_2,
            ),
            aspect: 640.0 / 480.0,
            projection: Projection::Perspective,
            grid: None,
            grid_plane: None,
            frame: [0., 0., 1., 1.],
        }
    }
}

impl Camera {
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        if bounds
            .min
            .iter()
            .chain(&bounds.max)
            .any(|v| !v.is_finite())
            || (0..3).any(|i| bounds.min[i] > bounds.max[i])
        {
            return;
        }
        let centre = bounds.centre();
        let radius = bounds.radius();
        if !radius.is_finite() || centre.iter().any(|v| !v.is_finite()) {
            return;
        }
        if radius == 0.0 {
            self.target = centre;
            return;
        }
        let tx = self.aspect * HALF_FOV.tan() * (self.frame[2] - self.frame[0]);
        let ty = HALF_FOV.tan() * (self.frame[3] - self.frame[1]);
        if !tx.is_finite() || tx <= 0.0 || !ty.is_finite() || ty <= 0.0 {
            return;
        }
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let mut distance: f64 = 0.0;
        for corner in 0..8 {
            let p: [f64; 3] = std::array::from_fn(|i| {
                let coordinate = if corner & (1 << i) == 0 {
                    bounds.min[i]
                } else {
                    bounds.max[i]
                };
                coordinate - centre[i]
            });
            let dot = |axis: &Vector| (0..3).map(|i| p[i] * axis[i]).sum::<f64>();
            let z = dot(&forward);
            distance = distance.max(dot(&right).abs() / tx + z);
            distance = distance.max(dot(&up).abs() / ty + z);
        }
        self.target = centre;
        self.radius = radius;
        self.distance = (distance * 1.05).max(1.0e-6);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += dx as f64 * right[i] + dy as f64 * up[i];
        }
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.distance = (self.distance / factor as f64)
                .clamp(self.radius * 0.2, self.radius * 50.0);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.orbit(radians as f64, 0.0);
    }

    pub fn orbit(&mut self, yaw: f64, pitch: f64) {
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let turn = Quaternion::from_axis_angle(Vector::z_axis(), yaw);
        let tilt = Quaternion::from_axis_angle(right, pitch);
        self.orientation = (turn * (tilt * self.orientation.duplicate())).normalized();
    }

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
            View::IsoBack => {
                // the isometric from the opposite corner: turned half round
                let yaw_q = Quaternion::from_axis_angle(z, PI - FRAC_PI_6);
                let rv = yaw_q.rotate_vector(x);
                (Quaternion::from_axis_angle(rv, -FRAC_PI_6) * yaw_q).normalized()
            }
        };

        self.projection = Projection::Orthographic;
    }

    pub fn isometric(&mut self) {
        let turn = Quaternion::from_axis_angle(
            Vector::z_axis(),
            -std::f64::consts::FRAC_PI_6,
        );
        let right = turn.rotate_vector(Vector::x_axis());
        let tilt = Quaternion::from_axis_angle(right, -std::f64::consts::FRAC_PI_6);
        self.orientation = (tilt * turn).normalized();
    }

    pub fn view_projection(&self) -> Xform {
        self.view_projection_anchored([0.; 3])
    }

    pub fn view_projection_anchored(&self, anchor: [f64; 3]) -> Xform {
        let scale = self.unit.to_meters();
        let local: [f64; 3] =
            std::array::from_fn(|i| (self.target[i] - anchor[i]) * scale);
        let distance = self.distance * scale;
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let target = Point::new(local[0], local[1], local[2]);
        let eye = Point::new(
            local[0] - forward[0] * distance,
            local[1] - forward[1] * distance,
            local[2] - forward[2] * distance,
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = distance * NEAR_FRACTION;
        let reach = self
            .grid
            .and_then(|grid| {
                grid.reach_on(
                    self.grid_plane.unwrap_or_else(|| {
                        crate::cplane::CPlane::view(
                            &self
                                .orientation
                                .rotate_vector(Vector::y_axis()),
                        )
                    }),
                    self.target,
                )
                .ok()
            })
            .map_or(self.radius, |reach| self.radius.max(reach));
        let reach = reach * scale;
        let far = (distance * 10.0).max(distance + 2.0 * reach);
        let projection = match self.projection {
            Projection::Perspective => {
                Xform::perspective(2.0 * HALF_FOV, self.aspect, far, near)
            }
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6 * scale);
                Xform::orthographic(-w, w, -h, h, depth_range, -depth_range)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    fn default() -> Self {
        let unit = Unit::Millimeters;
        let mut camera = Self {
            unit,
            target: [0.; 3],
            distance: 3. / unit.to_meters(),
            radius: 0.,
            orientation: Quaternion::identity(),
            aspect: 640. / 480.,
            projection: Projection::Perspective,
            grid: None,
            grid_plane: None,
            frame: [0., 0., 1., 1.],
        };
        camera.isometric();
        camera
    }
}

impl Camera {
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        if bounds
            .min
            .iter()
            .chain(&bounds.max)
            .any(|v| !v.is_finite())
            || (0..3).any(|i| bounds.min[i] > bounds.max[i])
        {
            return;
        }
        let centre = bounds.centre();
        let radius = bounds.radius();
        if !radius.is_finite() || centre.iter().any(|v| !v.is_finite()) {
            return;
        }
        if radius == 0.0 {
            self.target = centre;
            return;
        }
        let tx = self.aspect * HALF_FOV.tan() * (self.frame[2] - self.frame[0]);
        let ty = HALF_FOV.tan() * (self.frame[3] - self.frame[1]);
        if !tx.is_finite() || tx <= 0.0 || !ty.is_finite() || ty <= 0.0 {
            return;
        }
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let mut distance: f64 = 0.0;
        for corner in 0..8 {
            let p: [f64; 3] = std::array::from_fn(|i| {
                let coordinate = if corner & (1 << i) == 0 {
                    bounds.min[i]
                } else {
                    bounds.max[i]
                };
                coordinate - centre[i]
            });
            let dot = |axis: &Vector| (0..3).map(|i| p[i] * axis[i]).sum::<f64>();
            let z = dot(&forward);
            distance = distance.max(dot(&right).abs() / tx + z);
            distance = distance.max(dot(&up).abs() / ty + z);
        }
        self.target = centre;
        self.radius = radius;
        self.distance = (distance * 1.05).max(1.0e-6 / self.unit.to_meters());
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += dx as f64 * right[i] + dy as f64 * up[i];
        }
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            let scale = self.unit.to_meters();
            self.distance =
                (self.distance / factor as f64).clamp(1.0e-6 / scale, 1.0e15 / scale);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.orbit(radians as f64, 0.0);
    }

    pub fn orbit(&mut self, yaw: f64, pitch: f64) {
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let turn = Quaternion::from_axis_angle(Vector::z_axis(), yaw);
        let tilt = Quaternion::from_axis_angle(right, pitch);
        self.orientation = (turn * (tilt * self.orientation.duplicate())).normalized();
    }

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
            View::IsoBack => {
                // the isometric from the opposite corner: turned half round
                let yaw_q = Quaternion::from_axis_angle(z, PI - FRAC_PI_6);
                let rv = yaw_q.rotate_vector(x);
                (Quaternion::from_axis_angle(rv, -FRAC_PI_6) * yaw_q).normalized()
            }
        };

        self.projection = Projection::Orthographic;
    }

    pub fn isometric(&mut self) {
        let turn = Quaternion::from_axis_angle(
            Vector::z_axis(),
            -std::f64::consts::FRAC_PI_6,
        );
        let right = turn.rotate_vector(Vector::x_axis());
        let tilt = Quaternion::from_axis_angle(right, -std::f64::consts::FRAC_PI_6);
        self.orientation = (tilt * turn).normalized();
    }

    pub fn view_projection(&self) -> Xform {
        self.view_projection_anchored([0.; 3])
    }

    pub fn view_projection_anchored(&self, anchor: [f64; 3]) -> Xform {
        let scale = self.unit.to_meters();
        let local: [f64; 3] =
            std::array::from_fn(|i| (self.target[i] - anchor[i]) * scale);
        let distance = self.distance * scale;
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let target = Point::new(local[0], local[1], local[2]);
        let eye = Point::new(
            local[0] - forward[0] * distance,
            local[1] - forward[1] * distance,
            local[2] - forward[2] * distance,
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = distance * NEAR_FRACTION;
        let reach = self
            .grid
            .and_then(|grid| {
                grid.reach_on(
                    self.grid_plane.unwrap_or_else(|| {
                        crate::cplane::CPlane::view(
                            &self
                                .orientation
                                .rotate_vector(Vector::y_axis()),
                        )
                    }),
                    self.target,
                )
                .ok()
            })
            .map_or(self.radius, |reach| self.radius.max(reach));
        let reach = reach * scale;
        let far = (distance * 10.0).max(distance + 2.0 * reach);
        let projection = match self.projection {
            Projection::Perspective => {
                Xform::perspective(2.0 * HALF_FOV, self.aspect, far, near)
            }
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6);
                Xform::orthographic(-w, w, -h, h, depth_range, -depth_range)
```

### 2. `src/camera_reference_tests.rs`

Supply the exact maintained-camera source and its zoom helper as a native-only independent reference; it is not part of learner browser implementation.

Create the file and type:

```rust
use session_rust::{AABB, Point, Quaternion, Vector, Xform};

pub const FOVY_DEG: f64 = 60.0;

/// The unit the scene file is written in.
#[derive(Clone, Copy, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}

impl Unit {
    /// Factor from this unit to meters.
    pub fn to_meters(self) -> f64 {
        match self {
            Unit::Millimeters => 0.001,
            Unit::Meters => 1.0,
        }
    }
}

/// Near plane distance as a fraction of the target distance.
pub const NEAR_FRACTION: f64 = 1.0e-4;

/// Half width of the ground grid in scene units; matches HALF in grid.wgsl.
const GRID_HALF: f64 = 5000.0;

/// A named standard view.
#[derive(Clone, Copy, Debug)]
pub enum View {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Iso,
    IsoBack,
}

/// Orbit camera: an orientation, a target and a distance.
pub struct Camera {
    pub target: [f64; 3],        // the point looked at, meters
    pub distance: f64,           // eye to target, meters
    pub orientation: Quaternion, // where the camera faces
    pub world_up: [f64; 3],      // the axis yaw turns around
    pub position: [f64; 3],      // the eye, computed from the above
    pub up: [f64; 3],            // the up direction, computed from the above
    pub perspective: bool,       // false = orthographic
    pub unit: Unit,              // the scene file unit
    pub scene_extent: f64,       // scene radius in meters, keeps the far plane wide
    pub frame: [f64; 4], // the canvas part the panels leave free: left, top, right, bottom in fractions
    pub grid: Option<[f64; 3]>, // a construction plane's grid centre, scene units; None for the ground grid
}

/// The part of the camera the user chose: where it looks from and at.
#[derive(Debug, PartialEq)]
pub struct CameraPose {
    target: [f64; 3],   // the point looked at
    position: [f64; 3], // the eye
    up: [f64; 3],       // the up direction
    perspective: bool,  // false = orthographic
}

impl Camera {
    /// The current pose.
    pub fn pose(&self) -> CameraPose {
        CameraPose {
            target: self.target,
            position: self.position,
            up: self.up,
            perspective: self.perspective,
        }
    }

    /// A camera at the isometric view.
    pub fn new() -> Self {
        use std::f64::consts::FRAC_PI_6;

        // turn 30° about Z, then tilt 30° down
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
            frame: [0.0, 0.0, 1.0, 1.0],
            grid: None,
        };

        cam.update_position();

        cam
    }

    /// Turn the camera around the target by mouse pixels.
    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let wu = Vector::new(
            self.world_up[0],
            self.world_up[1],
            self.world_up[2],
        );
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        // horizontal pixels turn about up, vertical pixels tilt about right
        let yaw_q = Quaternion::from_axis_angle(wu, (-dx * 0.005) as f64);
        let pitch_q = Quaternion::from_axis_angle(right, (-dy * 0.005) as f64);

        self.orientation =
            (yaw_q * (pitch_q * self.orientation.duplicate())).normalized();
        self.update_position();
    }

    /// Slide the camera sideways by mouse pixels.
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        // farther away, a pixel moves more
        let k = self.distance * 0.0015;

        for i in 0..3 {
            self.target[i] += (-(dx as f64) * right[i] + dy as f64 * self.up[i]) * k;
        }

        self.update_position();
    }

    /// Move the eye toward or away from the target.
    pub fn zoom(&mut self, amount: f32) {
        self.distance = zoom_distance(self.distance, amount);
        self.update_position();
    }

    /// The world ray under a cursor pixel: origin and unit direction, scene units.
    pub fn ray(
        &self,
        cursor: (f64, f64),
        viewport: (f64, f64),
    ) -> Option<(Point, Vector)> {
        // no ray for an empty viewport or a lost cursor
        if viewport.0 <= 0.0
            || viewport.1 <= 0.0
            || !cursor.0.is_finite()
            || !cursor.1.is_finite()
        {
            return None;
        }

        // pixel to -1..1 on both axes, y up
        let [shift_x, shift_y] = self.shift();
        let ndc_x = 2.0 * cursor.0 / viewport.0 - 1.0 - shift_x;
        let ndc_y = 1.0 - 2.0 * cursor.1 / viewport.1 - shift_y;
        // work in scene units, not meters
        let s = self.unit.to_meters();
        let target = self.origin();
        let distance = self.distance_world();
        // half the view size at the target plane
        let half_h = distance * (FOVY_DEG * 0.5).to_radians().tan();
        let half_w = half_h * (viewport.0 / viewport.1);
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        // the cursor's point on the target plane
        let mut on_plane = [0.0; 3];

        for i in 0..3 {
            on_plane[i] =
                target[i] + right[i] * ndc_x * half_w + self.up[i] * ndc_y * half_h;
        }

        if !self.perspective {
            // orthographic: parallel rays, start behind the whole scene
            let back = distance + 2.0 * (self.scene_extent / s).max(distance);
            let origin = Point::new(
                on_plane[0] - forward[0] * back,
                on_plane[1] - forward[1] * back,
                on_plane[2] - forward[2] * back,
            );
            return Some((origin, forward));
        }

        // perspective: every ray starts at the eye
        let eye = [
            self.position[0] / s,
            self.position[1] / s,
            self.position[2] / s,
        ];
        let mut dir = [0.0; 3];

        for i in 0..3 {
            dir[i] = on_plane[i] - eye[i];
        }

        let length = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();

        if !length.is_finite() || length <= 0.0 {
            return None;
        }

        Some((
            Point::new(eye[0], eye[1], eye[2]),
            Vector::new(
                dir[0] / length,
                dir[1] / length,
                dir[2] / length,
            ),
        ))
    }

    /// Zoom so the point under the cursor stays under the cursor.
    pub fn zoom_at(&mut self, amount: f32, cursor: (f64, f64), viewport: (f64, f64)) {
        // nothing for an empty viewport or a lost cursor
        if viewport.0 <= 0.0
            || viewport.1 <= 0.0
            || !cursor.0.is_finite()
            || !cursor.1.is_finite()
        {
            return;
        }

        let new_dist = zoom_distance(self.distance, amount);
        let k = new_dist / self.distance; // how much closer
        // pixel to -1..1 on both axes, y up
        let [shift_x, shift_y] = self.shift();
        let ndc_x = 2.0 * cursor.0 / viewport.0 - 1.0 - shift_x;
        let ndc_y = 1.0 - 2.0 * cursor.1 / viewport.1 - shift_y;
        // half the view size at the target plane
        let half_h = self.distance * (FOVY_DEG * 0.5).to_radians().tan();
        let half_w = half_h * (viewport.0 / viewport.1);
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());

        for i in 0..3 {
            let cursor_off = right[i] * ndc_x * half_w + self.up[i] * ndc_y * half_h;
            self.target[i] += cursor_off * (1.0 - k); // pull the target toward the cursor
        }

        self.distance = new_dist;
        self.update_position();
    }

    /// Switch between perspective and orthographic.
    pub fn toggle_projection(&mut self) {
        self.perspective = !self.perspective;
    }

    /// Switch projection and keep what was on screen in view.
    pub fn toggle_projection_framed(&mut self, bounds: &AABB, aspect: f64) {
        self.perspective = !self.perspective;

        // only perspective can lose content; refit to what ortho showed
        if !self.perspective || !bounds.is_valid() {
            return;
        }

        let s = self.unit.to_meters();
        let t = self.origin(); // target, scene units
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = Vector::new(self.up[0], self.up[1], self.up[2]);
        let fwd = Vector::new(
            (self.target[0] - self.position[0]) / self.distance,
            (self.target[1] - self.position[1]) / self.distance,
            (self.target[2] - self.position[2]) / self.distance,
        );
        let half_h = self.distance * (FOVY_DEG * 0.5).to_radians().tan() / s; // half view height
        let half_w = half_h * aspect;
        let [shift_x, shift_y] = self.shift();
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];

        // clip every box corner to the visible rectangle
        for corner in bounds.corners() {
            let c = [
                corner[0] - t[0],
                corner[1] - t[1],
                corner[2] - t[2],
            ];
            let dx = (c[0] * right[0] + c[1] * right[1] + c[2] * right[2]).clamp(
                (-1.0 - shift_x) * half_w,
                (1.0 - shift_x) * half_w,
            );
            let dy = (c[0] * up[0] + c[1] * up[1] + c[2] * up[2]).clamp(
                (-1.0 - shift_y) * half_h,
                (1.0 - shift_y) * half_h,
            );
            let dz = c[0] * fwd[0] + c[1] * fwd[1] + c[2] * fwd[2];

            for i in 0..3 {
                let p = t[i] + dx * right[i] + dy * up[i] + dz * fwd[i];
                lo[i] = lo[i].min(p);
                hi[i] = hi[i].max(p);
            }
        }

        let clipped = AABB::from_points(
            &[
                Point::new(lo[0], lo[1], lo[2]),
                Point::new(hi[0], hi[1], hi[2]),
            ],
            0.0,
        );
        self.fit(&clipped, aspect);
    }

    /// The target in scene units.
    pub fn origin(&self) -> Point {
        let s = self.unit.to_meters();
        Point::new(
            self.target[0] / s,
            self.target[1] / s,
            self.target[2] / s,
        )
    }

    /// Eye to target distance in scene units.
    pub fn distance_world(&self) -> f64 {
        self.distance / self.unit.to_meters()
    }

    /// How far round the target the depth range must reach, in meters: the scene, and the ground grid's farthest corner, so neither is clipped.
    fn depth_reach(&self) -> f64 {
        let s = self.unit.to_meters();
        let half = GRID_HALF * s;
        let x = self.target[0].abs() + half;
        let y = self.target[1].abs() + half;
        let grid = match self.grid {
            None => (x * x + y * y + self.target[2] * self.target[2]).sqrt(),
            // a grid on a construction plane, turned any way about its centre
            Some(c) => {
                let d = [0, 1, 2].map(|k| self.target[k] - c[k] * s);
                (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() + half * 2.0_f64.sqrt()
            }
        };
        let scene = if self.scene_extent > 0.0 {
            self.scene_extent
        } else {
            self.distance
        };

        scene.max(grid)
    }

    /// The view-projection matrix, relative to the target.
    pub fn view_proj(&self, aspect: f64) -> Xform {
        self.view_proj_anchored(aspect, &self.origin())
    }

    /// The view-projection matrix, relative to `anchor` (scene units).
    pub fn view_proj_anchored(&self, aspect: f64, anchor: &Point) -> Xform {
        let dist = self.distance;
        let a = self.unit.to_meters();
        let anchor = Point::new(anchor[0] * a, anchor[1] * a, anchor[2] * a); // to meters
        // far plane reaches the whole scene and the ground grid
        let reach = self.depth_reach();
        let far = (dist * 10.0).max(dist + 2.0 * reach);
        let projection = if self.perspective {
            // far and near swapped: depth 1 is near (reverse-Z)
            Xform::perspective(
                FOVY_DEG.to_radians(),
                aspect,
                far,
                dist * NEAR_FRACTION,
            )
        } else {
            let h = dist * (FOVY_DEG * 0.5).to_radians().tan(); // half view height
            // depth range covers the scene and the ground grid, not more
            let r = (dist + 2.0 * reach).max(1.0e-6);
            Xform::orthographic(-aspect * h, aspect * h, -h, h, r, -r)
        };

        // eye and target relative to the anchor
        let eye = Point::new(
            self.position[0] - anchor[0],
            self.position[1] - anchor[1],
            self.position[2] - anchor[2],
        );
        let target = Point::new(
            self.target[0] - anchor[0],
            self.target[1] - anchor[1],
            self.target[2] - anchor[2],
        );
        let up = Vector::new(self.up[0], self.up[1], self.up[2]);
        let view = Xform::look_at_right_handed(&eye, &target, &up);

        // scene units to meters
        let s = self.unit.to_meters();
        let scale = Xform::scale_xyz(s, s, s);

        // the target lands in the middle of the free part, not of the canvas
        let [shift_x, shift_y] = self.shift();
        let lens = Xform::translation(shift_x, shift_y, 0.0);

        lens * projection * view * scale
    }

    /// Where the middle of the free part sits on the canvas, -1..1 on both axes, y up.
    pub fn shift(&self) -> [f64; 2] {
        [
            self.frame[0] + self.frame[2] - 1.0,
            1.0 - self.frame[1] - self.frame[3],
        ]
    }

    /// Turn to a named view, orthographic.
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
            View::IsoBack => {
                // the isometric from the opposite corner: turned half round
                let yaw_q = Quaternion::from_axis_angle(z, PI - FRAC_PI_6);
                let rv = yaw_q.rotate_vector(x);
                (Quaternion::from_axis_angle(rv, -FRAC_PI_6) * yaw_q).normalized()
            }
        };

        self.perspective = false;
        self.update_position();
    }

    /// Reset to a fresh default camera.
    pub fn reset(&mut self) {
        let (frame, grid) = (self.frame, self.grid);
        *self = Camera::new();
        self.frame = frame;
        self.grid = grid;
    }

    /// Frame a box: look at its center, back off until it fits the free part of the canvas.
    pub fn fit(&mut self, bounds: &AABB, aspect: f64) {
        self.fit_in(bounds, aspect, self.frame);
    }

    /// Frame a box in the part of the canvas the panels leave free, `frame` its left, top, right and bottom as fractions of the canvas: the box fits that part and sits in its middle.
    pub fn fit_in(&mut self, bounds: &AABB, aspect: f64, frame: [f64; 4]) {
        self.frame = frame;

        if !bounds.is_valid() {
            return;
        }

        let s = self.unit.to_meters();

        // look at the box center
        self.target = [bounds.cx * s, bounds.cy * s, bounds.cz * s];

        // half the view angle, sideways and up
        let half_fov_y = FOVY_DEG.to_radians() * 0.5;
        let half_fov_x = (aspect * half_fov_y.tan()).atan();
        let (full_x, full_y) = (half_fov_x.tan(), half_fov_y.tan());
        let (tx, ty) = (
            full_x * (frame[2] - frame[0]),
            full_y * (frame[3] - frame[1]),
        );

        let fwd = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());

        // the distance every corner needs to be in view
        let mut distance: f64 = 0.0;
        let mut extent: f64 = 0.0;

        for p in self.offsets(bounds) {
            let (x, y, z) = (
                dot3(&p, &right),
                dot3(&p, &up),
                dot3(&p, &fwd),
            );
            extent = extent.max((x * x + y * y + z * z).sqrt());
            distance = distance.max(x.abs() / tx + z);
            distance = distance.max(y.abs() / ty + z);
        }

        if extent <= 0.0 {
            return;
        }

        self.distance = (distance * 1.05).max(1.0e-6); // 5% margin
        self.scene_extent = extent; // farthest corner from the target
        self.update_position();
    }

    /// The eight box corners in meters, relative to the target.
    fn offsets(&self, bounds: &AABB) -> [[f64; 3]; 8] {
        let s = self.unit.to_meters();
        let mut out = [[0.0; 3]; 8];

        for (offset, c) in out.iter_mut().zip(bounds.corners()) {
            *offset = [
                c[0] * s - self.target[0],
                c[1] * s - self.target[1],
                c[2] * s - self.target[2],
            ];
        }

        out
    }

    /// Widen the far plane for a box that arrived later, view untouched.
    pub fn grow_extent(&mut self, bounds: &AABB) {
        if !bounds.is_valid() {
            return;
        }

        let mut extent: f64 = 0.0;

        for p in self.offsets(bounds) {
            extent = extent.max((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt());
        }

        if extent.is_finite() && extent > self.scene_extent {
            self.scene_extent = extent;
        }
    }

    /// Set the scene file unit.
    pub fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
    }

    /// Recompute the eye and up from orientation, target and distance.
    pub fn update_position(&mut self) {
        let fwd = self
            .orientation
            .rotate_vector(Vector::y_axis()); // eye to target
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());

        for i in 0..3 {
            self.position[i] = self.target[i] - fwd[i] * self.distance;
            self.up[i] = up[i];
        }
    }
}

/// Dot product of an array and a vector.
fn dot3(p: &[f64; 3], v: &Vector) -> f64 {
    p[0] * v[0] + p[1] * v[1] + p[2] * v[2]
}

fn zoom_distance(distance: f64, amount: f32) -> f64 {
    if !amount.is_finite() {
        return distance;
    }

    // at most ten steps per event, never zero
    (distance * 0.9_f64.powf(f64::from(amount).clamp(-10.0, 10.0)))
        .clamp(1.0e-6, 1.0e15)
}
```

### 3. `src/camera_reference_fixture.json`

Supply the dated source and reference hashes for independent camera validation.

Create the file and type:

```json
{
  "installed": false,
  "scope": "Optional native reference fixture only; no learner/browser runtime dependency.",
  "cameraSourceSha256": "2eb0d51723129bdea1cddc3f35760217923959afe2a50dfe1fc6e129c8a3d58f",
  "frameSourceSha256": "2a492fe36f15db4201a687a1d06e120945d38410a075375fece6ec1d3d4323fb",
  "extraction": "Camera source before its first test module plus the later exact zoom_distance helper. Only the private FOV import is replaced by its actual source constant.",
  "fovDegrees": 60.0,
  "fixtureSha256": "26aeb8005c5bf0f0b2e45ba48b02d58e2358130524f43c6f9e08a21c1d4e6ff5"
}
```

### 4. `src/camera_defaults_tests.rs`

Supply independent default/rotation, reference-integrity, zoom-bound and tiny-fit checks in both units.

Create the file and type:

```rust
use crate::{
    camera::{Camera, Unit},
    camera_reference_tests as current,
};
use session_rust::{AABB, Point, Vector};
use sha2::{Digest, Sha256};
#[test]
fn current_default_pose_uses_metres_and_the_shared_isometric_owner() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("camera_reference_fixture.json")).unwrap();
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(include_bytes!("camera_reference_tests.rs"))
        ),
        manifest["fixtureSha256"].as_str().unwrap()
    );
    let actual = Camera::default();
    let expected = current::Camera::new();
    let scale = actual.unit.to_meters();
    assert_eq!(actual.distance * scale, expected.distance);
    assert_eq!(actual.radius * scale, expected.scene_extent);
    assert_eq!(
        actual.target.map(|v| v * scale),
        expected.target
    );
    assert_eq!(
        actual.unit.to_meters(),
        expected.unit.to_meters()
    );
    for [x, y, z] in [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]] {
        let a = actual
            .orientation
            .rotate_vector(Vector::new(x, y, z));
        let b = expected
            .orientation
            .rotate_vector(Vector::new(x, y, z));
        for i in 0..3 {
            assert!((a[i] - b[i]).abs() < 1e-12);
        }
    }
}
#[test]
fn physical_zoom_bounds_and_tiny_fit_match_the_maintained_camera() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        let mut actual = Camera::default();
        actual.unit = unit;
        let scale = unit.to_meters();
        let mut expected = current::Camera::new();
        expected.unit = match unit {
            Unit::Millimeters => current::Unit::Millimeters,
            Unit::Meters => current::Unit::Meters,
        };
        actual.distance = expected.distance / scale;
        actual.zoom(1e-30);
        for _ in 0..1000 {
            expected.zoom(-10.);
        }
        assert!(
            (actual.distance * scale - expected.distance).abs()
                <= expected.distance * 1e-14
        );
        actual.zoom(1e30);
        for _ in 0..1000 {
            expected.zoom(10.);
        }
        assert!(
            (actual.distance * scale - expected.distance).abs()
                <= expected.distance * 1e-14
        );
        let bounds = crate::bounds::Bounds {
            min: [-1e-12; 3],
            max: [1e-12; 3],
        };
        actual.fit(&bounds);
        expected.fit(
            &AABB::from_points(
                &[
                    Point::new(-1e-12, -1e-12, -1e-12),
                    Point::new(1e-12, 1e-12, 1e-12),
                ],
                0.,
            ),
            actual.aspect,
        );
        assert!((actual.distance * scale - expected.distance).abs() < 1e-18);
    }
}

// Explicit three-scene-unit Top pose for pre-existing unit-sized fixtures.
pub(crate) fn top_fixture() -> Camera {
    let mut camera = Camera::default();
    camera.distance = 3.;
    camera.radius = 1.;
    camera.orientation = session_rust::Quaternion::from_axis_angle(
        Vector::x_axis(),
        -std::f64::consts::FRAC_PI_2,
    );
    camera
}
pub(crate) fn editor_fixture() -> crate::editor::Editor {
    let mut editor = crate::editor::Editor::default();
    editor.camera = top_fixture();
    editor
}
```

### 5. `src/lib.rs`

Register optional native camera comparisons; browser builds omit both modules.

**Locate this existing block:**

```rust
#[cfg(test)]
mod camera_reverse_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod camera_reverse_tests;

#[cfg(test)]
mod camera_defaults_tests;
#[cfg(test)]
mod camera_reference_tests;
```

### 6. `src/camera_space_tests.rs`

Declare the existing Top pose for exact rectangle crossing/window sets; retain distant source, pixel and ownership assertions.

**Locate this existing block:**

```rust
    let point_source = scene.points()[0].prepared.source.clone();
    let mut camera = Camera::default();
    camera.projection = Projection::Orthographic;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let point_source = scene.points()[0].prepared.source.clone();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = Projection::Orthographic;
```

### 7. `src/drawing_tests.rs`

Keep the plane-click fixture at its original unit-sized Top pose so existing distant source points cannot become accidental near-cursor snaps.

**Locate this existing block:**

```rust
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn editor() -> Editor {
    let mut e = crate::camera_defaults_tests::editor_fixture();
    e.apply(Action::Close).unwrap();
```

### 8. `src/editor.rs`

Retain exact Reset/aspect and navigation-independent Undo assertions, with default distances now expressed in metres.

**Locate this existing block:**

```rust
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(
            editor.camera.distance * editor.camera.unit.to_meters(),
            3.0
        );
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(
            editor.camera.distance * editor.camera.unit.to_meters(),
            1.5
        );
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
```

### 9. `src/navigation_complete_tests.rs`

Retain the same cursor-zoom pose and exact target-plane invariance in both projections and panel frames.

**Locate this existing block:**

```rust
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        camera.projection = projection;
        camera.frame([0., 0., 0.7, 0.8]);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Projection::Orthographic,
    ] {
        let mut camera = crate::camera_defaults_tests::top_fixture();
        camera.projection = projection;
        camera.frame([0., 0., 0.7, 0.8]);
```

### 10. `src/navigation_tests.rs`

Declare the original three-unit wheel fixture so its absolute accumulation tolerance and history assertions remain unchanged.

**Locate this existing block:**

```rust
fn distance(delta: f64, mode: u32) -> f64 {
    let mut editor = Editor::default();
    assert_eq!(
        editor
            .apply(wheel(delta, mode, 480.0).unwrap())
            .unwrap(),
        Change::View
    );
    editor.camera.distance
}

#[test]
fn wheel_units_agree_and_small_movements_accumulate() {
    assert!((distance(48.0, 0) - distance(3.0, 1)).abs() < 1.0e-6);
    assert!((distance(48.0, 0) - distance(0.1, 2)).abs() < 1.0e-6);
    let mut editor = Editor::default();
    for _ in 0..10 {
        editor
            .apply(wheel(-12.0, 0, 480.0).unwrap())
            .unwrap();
    }
    assert!((editor.camera.distance - distance(-120.0, 0)).abs() < 1.0e-5);
    editor
        .apply(wheel(120.0, 0, 480.0).unwrap())
        .unwrap();
    assert!((editor.camera.distance - 3.0).abs() < 1.0e-5);
}

#[test]
fn wheel_rejects_invalid_input_and_bounds_large_jumps() {
    for delta in [0.0, f64::NAN, f64::INFINITY] {
        assert!(wheel(delta, 0, 480.0).is_none());
    }
    for height in [0.0, -1.0, f64::NAN] {
        assert!(wheel(10.0, 0, height).is_none());
    }
    assert!(wheel(10.0, 99, 480.0).is_none());
    assert_eq!(distance(f64::MAX, 2), distance(1_000.0, 0));
    assert_eq!(distance(-f64::MAX, 2), distance(-1_000.0, 0));
}

#[test]
fn navigation_preserves_document_history() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn distance(delta: f64, mode: u32) -> f64 {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    assert_eq!(
        editor
            .apply(wheel(delta, mode, 480.0).unwrap())
            .unwrap(),
        Change::View
    );
    editor.camera.distance
}

#[test]
fn wheel_units_agree_and_small_movements_accumulate() {
    assert!((distance(48.0, 0) - distance(3.0, 1)).abs() < 1.0e-6);
    assert!((distance(48.0, 0) - distance(0.1, 2)).abs() < 1.0e-6);
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    for _ in 0..10 {
        editor
            .apply(wheel(-12.0, 0, 480.0).unwrap())
            .unwrap();
    }
    assert!((editor.camera.distance - distance(-120.0, 0)).abs() < 1.0e-5);
    editor
        .apply(wheel(120.0, 0, 480.0).unwrap())
        .unwrap();
    assert!((editor.camera.distance - 3.0).abs() < 1.0e-5);
}

#[test]
fn wheel_rejects_invalid_input_and_bounds_large_jumps() {
    for delta in [0.0, f64::NAN, f64::INFINITY] {
        assert!(wheel(delta, 0, 480.0).is_none());
    }
    for height in [0.0, -1.0, f64::NAN] {
        assert!(wheel(10.0, 0, height).is_none());
    }
    assert!(wheel(10.0, 99, 480.0).is_none());
    assert_eq!(distance(f64::MAX, 2), distance(1_000.0, 0));
    assert_eq!(distance(-f64::MAX, 2), distance(-1_000.0, 0));
}

#[test]
fn navigation_preserves_document_history() {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor.apply(Action::AddBox).unwrap();
```

### 11. `src/picking.rs`

Declare the original Top ray before asserting turquoise-before-pink original surface ordering and bounded-ray rejection.

**Locate this existing block:**

```rust
        let mut scene = Scene::demo();
        let camera = Camera::default();
        let ray = camera.ray([0.0, 0.0]).unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut scene = Scene::demo();
        let camera = crate::camera_defaults_tests::top_fixture();
        let ray = camera.ray([0.0, 0.0]).unwrap();
```

### 12. `src/primitive_tests.rs`

Declare the axis-aligned Top view before testing the height-axis refusal and orbit recovery.

**Locate this existing block:**

```rust
    assert_eq!(draft.next().unwrap().kind, Kind::Height);
    let mut camera = crate::camera::Camera::default();
    assert!(draft.on_axis(&camera, [0., 0.]).is_none());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(draft.next().unwrap().kind, Kind::Height);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    assert!(draft.on_axis(&camera, [0., 0.]).is_none());
```

### 13. `src/projection_tests.rs`

Keep the close orthographic fixture at its declared three-unit Top pose and preserve behind-eye picking, history and Reset projection assertions.

**Locate this existing block:**

```rust
fn close_orthographic_picking_sees_geometry_behind_the_eye() {
    let mut editor = Editor::default();
    let before = editor.camera.uniform();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn close_orthographic_picking_sees_geometry_behind_the_eye() {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    let before = editor.camera.uniform();
```

### 14. `src/global_outline_tests.rs`

Declare the existing unit-sized camera/editor poses for GPU fixture scenes; retain exact pixels, primitive/source IDs, released-drag completion, caches, uploads and release.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        .geometry()
        .unwrap()
        .to_proto();
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
```

Replace it with:

```rust
        .geometry()
        .unwrap()
        .to_proto();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
```

**Locate this block:**

```rust
#[test]
fn actual_rebased_pick_finishes_a_released_drag_without_reuploading_sources() {
    use crate::editor::{Action, Change, Editor};
    let mut editor = Editor::default();
    editor.scene.clear();
    let owner = editor
        .scene
```

Replace it with:

```rust
#[test]
fn actual_rebased_pick_finishes_a_released_drag_without_reuploading_sources() {
    use crate::editor::{Action, Change, Editor};
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor.scene.clear();
    let owner = editor
        .scene
```

**Locate this block:**

```rust
        crate::camera::Projection::Orthographic,
    ] {
        for density in [1u32, 2] {
            let mut camera = crate::camera::Camera::default();
            camera.isometric();
            camera.projection = projection;
            camera.aspect = 4. / 3.;
```

Replace it with:

```rust
        crate::camera::Projection::Orthographic,
    ] {
        for density in [1u32, 2] {
            let mut camera = crate::camera_defaults_tests::top_fixture();
            camera.isometric();
            camera.projection = projection;
            camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
        },
    ];
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 3.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
        },
    ];
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 3.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
        &gpu.meshes,
        Some(&gpu.instances),
    );
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
        &gpu.meshes,
        Some(&gpu.instances),
    );
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
    };
    gpu.set_grid_on(settings, plane([0., 0., -0.75]))
        .unwrap();
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
    };
    gpu.set_grid_on(settings, plane([0., 0., -0.75]))
        .unwrap();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release() {
    let scene = Scene::empty();
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release() {
    let scene = Scene::empty();
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
            .unwrap()
            .moved
    );
    let mut far_camera = crate::camera::Camera::default();
    far_camera.distance = camera.distance;
    far_camera.aspect = camera.aspect;
    far_camera.projection = camera.projection;
```

Replace it with:

```rust
            .unwrap()
            .moved
    );
    let mut far_camera = crate::camera_defaults_tests::top_fixture();
    far_camera.distance = camera.distance;
    far_camera.aspect = camera.aspect;
    far_camera.projection = camera.projection;
```

**Locate this block:**

```rust
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
    camera.projection = crate::camera::Projection::Orthographic;
```

Replace it with:

```rust
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
    camera.projection = crate::camera::Projection::Orthographic;
```

### 15. `src/pick_gpu_tests.rs`

Declare the existing unit-sized camera/editor poses for GPU fixture scenes; retain exact pixels, primitive/source IDs, released-drag completion, caches, uploads and release.

**Locate this existing block:**

```rust
fn actual_control_hits_resolve_the_original_curve_control_instead_of_a_sample() {
    let mut editor = crate::editor::Editor::default();
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    editor
        .apply(crate::editor::Action::AddCurve)
        .unwrap();
    editor
        .apply(crate::editor::Action::Controls(true))
        .unwrap();
    let id = editor.scene.paths()[0].id;
    let mut renderer = renderer(&editor.scene);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control);
    renderer
        .request_pick([0., 0.6], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&editor.scene, hit),
        Some(crate::pick_sources::Target {
            owner: id,
            part: crate::pick_sources::Part::Control(1)
        })
    );
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge);
    renderer
        .request_pick([0., 0.1], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&editor.scene, hit),
        Some(crate::pick_sources::Target {
            owner: id,
            part: crate::pick_sources::Part::Curve
        })
    );
    editor.scene.show_controls(false);
    renderer.set_scene(&editor.scene, None);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control);
    renderer
        .request_pick([0., 0.6], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}

fn colour_pixels(renderer: &mut Renderer) -> Vec<u8> {
    colour_pixels_at(renderer, identity())
}
fn colour_pixels_at(renderer: &mut Renderer, transform: [f32; 16]) -> Vec<u8> {
    colour_pixels_display(renderer, transform, false)
}
fn colour_pixels_display(
    renderer: &mut Renderer,
    transform: [f32; 16],
    normals: bool,
) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("selection pixels"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    renderer.draw_mode(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &transform,
        normals,
    );
    let bytes = renderer
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection readback"),
            size: 320 * 240 * 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut encoder = renderer
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &bytes,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280),
                rows_per_image: Some(240),
            },
        },
        texture.size(),
    );
    renderer.queue.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::channel();
    bytes
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = send.send(result);
        });
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    receive
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap()
        .unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec();
    bytes.unmap();
    result
}

#[test]
fn actual_part_highlights_are_local_leave_black_edges_and_release_derived_data() {
    use crate::pick_sources::{Part, Target};
    let mut source = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.8, -0.6, 0.5]),
        (20, [0., -0.6, 0.5]),
        (30, [0., 0.6, 0.5]),
        (40, [-0.8, 0.6, 0.5]),
        (50, [0.8, -0.6, 0.5]),
        (60, [0.8, 0.6, 0.5]),
    ] {
        source.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    source.add_face(vec![10, 20, 30, 40], Some(37));
    source.add_face(vec![20, 50, 60, 30], Some(91));
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert(PreparedMesh::new(source).unwrap())
        .unwrap();
    let weak = Rc::downgrade(&scene.objects()[0].mesh);
    let mut renderer = renderer(&scene);
    let stats = renderer.stats();
    let before = colour_pixels(&mut renderer);
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner,
                part: Part::Face(37),
            }),
        )
        .unwrap();
    let face = colour_pixels(&mut renderer);
    assert!(renderer.highlight_bytes() > 0);
    let mut changed = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(face.chunks_exact(4))
        .enumerate()
    {
        if a != b {
            let x = index % 320;
            let y = index / 320;
            assert!((31..=160).contains(&x) && (47..=192).contains(&y));
            changed += 1;
        }
    }
    assert!(
        changed > 10000,
        "The original face, rather than the whole object, is visibly selected"
    );
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner,
                part: Part::MeshEdge(10, 20),
            }),
        )
        .unwrap();
    let edge = colour_pixels(&mut renderer);
    let mut changed = 0;
    let mut black = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(edge.chunks_exact(4))
        .enumerate()
    {
        let x = index % 320;
        let y = index / 320;
        if a != b {
            assert!((30..=161).contains(&x) && (190..=193).contains(&y));
            changed += 1;
        }
        if (35..150).contains(&x)
            && (191..=192).contains(&y)
            && b[0] < 160
            && b[1] < 140
            && b[2] < 80
        {
            black += 1;
        }
    }
    assert!(
        changed > 80 && black > 80,
        "A yellow fringe surrounds the original black edge: changed={changed}, dark={black}"
    );
    assert_eq!(
        renderer.stats(),
        stats,
        "Highlights reuse primary geometry"
    );
    renderer.set_highlight(&scene, None).unwrap();
    assert_eq!(renderer.highlight_bytes(), 0);
    assert_eq!(colour_pixels(&mut renderer), before);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_control_highlight_surrounds_only_the_original_marker_and_keeps_source_precision()
 {
    let mut editor = crate::editor::Editor::default();
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    editor
        .apply(crate::editor::Action::AddCurve)
        .unwrap();
    editor
        .apply(crate::editor::Action::Controls(true))
        .unwrap();
    let owner = editor.scene.paths()[0].id;
    let original = editor.scene.paths()[0].points();
    let mut renderer = renderer(&editor.scene);
    let before = colour_pixels(&mut renderer);
    renderer
        .set_highlight(
            &editor.scene,
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::Control(1),
            }),
        )
        .unwrap();
    let after = colour_pixels(&mut renderer);
    let mut changed = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .enumerate()
    {
        if a != b {
            let x = index % 320;
            let y = index / 320;
            assert!((151..=168).contains(&x) && (39..=56).contains(&y));
            changed += 1;
        }
    }
    assert!(
        changed > 20,
        "The halo belongs to the original control rather than a curve sample"
    );
    assert_eq!(editor.scene.paths()[0].points(), original);
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    renderer.set_scene(&editor.scene, None);
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_multiple_mesh_selection_reuses_geometry_and_pick_ownership() {
    let mut scene = Scene::demo();
    scene.clear();
    let panel = |x: f64| {
        PreparedMesh::triangle(
            [
                [x - 0.25, -0.4, 0.5],
                [x + 0.25, -0.4, 0.5],
                [x, 0.4, 0.5],
            ],
            [0.5, 0.5, 0.5],
        )
        .unwrap()
    };
    let a = scene.insert(panel(-0.4)).unwrap();
    let b = scene.insert(panel(0.4)).unwrap();
    let mut renderer = renderer(&scene);
    let baseline = colour_pixels(&mut renderer);
    let geometry = renderer.stats();
    renderer
        .select_objects(&scene, &[a, b])
        .unwrap();
    let selected = colour_pixels(&mut renderer);
    let pixel = |bytes: &[u8], x: usize| {
        bytes[(120 * 320 + x) * 4..(120 * 320 + x) * 4 + 3].to_vec()
    };
    for x in [96, 224] {
        let p = pixel(&selected, x);
        assert!(p[0] > 0 && p[0] == p[1] && p[2] == 0);
    }
    renderer
        .select_objects(&scene, &[b])
        .unwrap();
    let one = colour_pixels(&mut renderer);
    assert_eq!(pixel(&one, 96), pixel(&baseline, 96));
    assert_eq!(pixel(&one, 224), pixel(&selected, 224));
    assert_eq!(
        &renderer.stats()[..2],
        &geometry[..2],
        "Only selected settings change, not primary geometry"
    );
    for (x, id) in [(-0.4, a), (0.4, b)] {
        renderer
            .request_pick([x, 0.], identity())
            .unwrap();
        assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    }
}

#[test]
fn actual_tool_preview_is_temporary_bounded_and_absent_from_object_picking() {
    let mut scene = Scene::demo();
    scene.clear();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let stats = renderer.stats();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit([-0.5, 0., 0.2]).unwrap();
    draft.hover(Some([0.5, 0., 0.2])).unwrap();
    renderer.set_preview(Some(&draft)).unwrap();
    assert_eq!(renderer.preview_bytes(), 76);
    let after = colour_pixels(&mut renderer);
    let changed = before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        changed > 100,
        "The actual unfinished rubber band and cursor render"
    );
    let p = &after[(120 * 320 + 160) * 4..(120 * 320 + 160) * 4 + 3];
    assert_eq!(
        p,
        &[255, 255, 0],
        "The preview uses current selection yellow: {p:?}"
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "Drafts have no saved-object pick owner"
    );
    assert_eq!(renderer.stats(), stats);
    assert!(scene.lines().is_empty());
    renderer.set_preview(None).unwrap();
    assert_eq!(renderer.preview_bytes(), 0);
    assert_eq!(colour_pixels(&mut renderer), before);
    renderer.set_preview(Some(&draft)).unwrap();
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.preview_bytes(), 0);
}

#[test]
fn actual_layer_colours_change_display_settings_without_editing_or_reuploading_original_geometry()
 {
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert(triangle(0.5)).unwrap();
    let source = Rc::clone(scene.objects()[0].geometry().unwrap());
    let original = source.to_proto();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let allocations = renderer.stats();
    Rc::make_mut(&mut scene.view)
        .faces
        .insert(id, [0, 0, 255]);
    renderer.set_scene(&scene, None);
    let blue = colour_pixels(&mut renderer);
    assert_ne!(blue, before);
    let centre = (120 * 320 + 160) * 4;
    assert!(
        blue[centre + 2] > blue[centre].saturating_add(100)
            && blue[centre + 2] > blue[centre + 1].saturating_add(100)
    );
    assert_eq!(
        &renderer.stats()[..2],
        &allocations[..2],
        "Layer colour reuses primary geometry and its settings allocation"
    );
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    Rc::make_mut(&mut scene.view)
        .edges
        .insert(id, [0, 255, 0]);
    renderer.set_scene(&scene, None);
    let green_edges = colour_pixels(&mut renderer);
    let count = green_edges
        .chunks_exact(4)
        .filter(|p| p[1] > p[0].saturating_add(100) && p[1] > p[2].saturating_add(100))
        .count();
    assert!(
        count > 100,
        "Actual source boundary colour is independent: {count}"
    );
    Rc::make_mut(&mut scene.view)
        .faces
        .remove(&id);
    renderer.set_scene(&scene, None);
    let restored_faces = colour_pixels(&mut renderer);
    assert_eq!(
        &restored_faces[centre..centre + 4],
        &before[centre..centre + 4]
    );
    assert_ne!(
        restored_faces, before,
        "Restoring Faces leaves the independent edge override"
    );
    Rc::make_mut(&mut scene.view)
        .edges
        .remove(&id);
    renderer.set_scene(&scene, None);
    assert_eq!(colour_pixels(&mut renderer), before);
    assert!(Rc::ptr_eq(
        &source,
        scene.objects()[0].geometry().unwrap()
    ));
}

#[test]
fn actual_delayed_gpu_answer_finishes_a_released_drag_and_reuses_its_geometry() {
    use crate::editor::{Action, Change, Editor};
    let mut e = Editor::default();
    e.scene.clear();
    let id = e.scene.insert(triangle(0.2)).unwrap();
    let original = e.scene.placement(id).unwrap().m;
    let source = e.scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    let mut renderer = renderer(&e.scene);
    let transform = e.camera.uniform();
    let gpu = renderer.stats();
    let size = [320., 240.];
    e.apply(Action::CanvasStart {
        from: [0., 0.],
        to: [0.2, 0.],
        size,
        modified: false,
        pointer: 1,
    })
    .unwrap();
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    e.apply(Action::CanvasRelease {
        screen: [0.4, 0.],
        size,
        add: false,
        remove: false,
    })
    .unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, original);
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let hit = renderer
        .take_pick(transform)
        .unwrap()
        .unwrap()
        .and_then(|pixel| renderer.resolve_pick(&e.scene, pixel));
    assert_eq!(hit.unwrap().owner, id);
    assert_eq!(
        e.apply(Action::CanvasHit(
            hit.map(|target| target.owner)
        ))
        .unwrap(),
        Change::Scene
    );
    assert_ne!(e.scene.placement(id).unwrap().m, original);
    assert!(Rc::ptr_eq(
        &source,
        e.scene.objects()[0].geometry().unwrap()
    ));
    renderer.set_scene(&e.scene, e.selected);
    assert_eq!(&renderer.stats()[..2], &gpu[..2]);
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, original);
}

#[test]
fn actual_surface_pick_keeps_one_cad_face_and_four_domain_edge_ids() {
    use crate::pick_sources::{Mode, Part};
    let mut scene = Scene::demo();
    scene.clear();
    let source = Rc::new(crate::surface::example());
    let _ = source.guid();
    let before = source.to_proto();
    let id = scene.insert_surface(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    let camera = crate::camera::Camera::default();
    let transform = camera.uniform();
    let gpu = renderer.stats();
    let finish_at = |renderer: &mut Renderer| {
        renderer
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        renderer
            .take_pick(transform)
            .unwrap()
            .unwrap()
            .and_then(|hit| renderer.resolve_pick(&scene, hit))
    };
    renderer.set_pick_mode(Mode::Face);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    assert_eq!(
        finish_at(&mut renderer).unwrap().part,
        Part::Face(0)
    );
    renderer.set_pick_mode(Mode::Edge);
    for (point, edge) in [
        ([-1., 0., 0.], 0),
        ([1., 0., 0.], 1),
        ([0., -1., 0.], 2),
        ([0., 1., 0.], 3),
    ] {
        let point =
            camera
                .view_projection()
                .transform_point(&session_rust::Point::new(
                    point[0], point[1], point[2],
                ));
        renderer
            .request_pick([point[0] as f32, point[1] as f32], transform)
            .unwrap();
        let target = finish_at(&mut renderer).unwrap();
        assert_eq!(target.owner, id);
        assert_eq!(target.part, Part::SurfaceEdge(edge));
    }
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(source.to_proto(), before);
}

#[test]
fn actual_cad_hole_has_no_face_pick_and_integer_faces_retain_original_identity() {
    use crate::pick_sources::{Mode, Part};
    let mut scene = Scene::demo();
    scene.clear();
    let mut original = session_rust::BRep::create_block_with_hole(1.6, 1.2, 0.24, 0.26);
    original.surfacecolor = session_rust::Color::new(0.35, 0.65, 0.85, 1.);
    let source = Rc::new(original);
    let _ = source.guid();
    let before = source.to_proto();
    let id = scene.insert_cad(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    let camera = crate::camera::Camera::default();
    let transform = camera.uniform();
    let gpu = renderer.stats();
    let finish_at = |renderer: &mut Renderer| {
        renderer
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        renderer
            .take_pick(transform)
            .unwrap()
            .unwrap()
            .and_then(|hit| renderer.resolve_pick(&scene, hit))
    };
    renderer.set_pick_mode(Mode::Face);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    assert!(
        finish_at(&mut renderer).is_none(),
        "The actual shaded hole stays empty"
    );
    let p = camera
        .view_projection()
        .transform_point(&session_rust::Point::new(0.5, 0., 0.12));
    renderer
        .request_pick([p[0] as f32, p[1] as f32], transform)
        .unwrap();
    let target = finish_at(&mut renderer).unwrap();
    assert_eq!(target.owner, id);
    assert_eq!(
        target.part,
        Part::Face(6),
        "The visible top cap retains its original CAD identity"
    );
    renderer.set_pick_mode(Mode::Edge);
    let mut seen = 0;
    for border in &scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .edges
    {
        let points = &border.points;
        if !points
            .iter()
            .all(|p| (p[2] - 0.12).abs() < 1e-10)
        {
            continue;
        }
        let middle = points
            .windows(2)
            .nth((points.len() - 1) / 2)
            .unwrap();
        let p = std::array::from_fn::<_, 3, _>(|i| (middle[0][i] + middle[1][i]) * 0.5);
        let p = camera
            .view_projection()
            .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
        renderer
            .request_pick([p[0] as f32, p[1] as f32], transform)
            .unwrap();
        assert_eq!(
            finish_at(&mut renderer)
                .unwrap_or_else(|| panic!(
                    "Missing original CAD edge {} at {:?}",
                    border.edge, p
                ))
                .part,
            Part::BRepEdge(border.edge)
        );
        seen += 1;
    }
    assert_eq!(
        seen, 5,
        "All four outer edges and the original hole rim resolve"
    );
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(source.to_proto(), before);
}

#[test]
fn actual_screen_text_overlay_does_not_create_scene_pick_owners_or_reupload_geometry() {
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert(triangle(0.2)).unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let gpu = renderer.stats();
    renderer.text_demo().unwrap();
    let with_text = colour_pixels(&mut renderer);
    assert_ne!(with_text, before);
    assert_eq!(renderer.text_error(), None);
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(scene.objects().len(), 1);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    renderer
        .request_pick([-0.85, 0.8], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    assert_eq!(renderer.text_usage()[0], 1);
    renderer.set_text(Vec::new()).unwrap();
    assert_eq!(renderer.text_usage(), [0; 6]);
    assert_eq!(colour_pixels(&mut renderer), before);
}

#[test]
fn actual_owned_text_plane_respects_depth_integer_picks_selection_cache_and_close() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = scene.insert(triangle(0.2)).unwrap();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let gpu = renderer.stats();
    let mut source = example(TextPlacement::WorldPlane {
        world: [-0.1, 0.05, 0.6],
        right: [1., 0., 0.],
        up: [0., 1., 0.],
        world_height: 0.08,
    });
    source.label.text = "Hi".into();
    let source = Rc::new(source);
    let original = serde_json::to_string(&*source).unwrap();
    let weak = Rc::downgrade(&source);
    let id = scene
        .insert_text(Prepared::new(source.clone()).unwrap())
        .unwrap();
    drop(source);
    renderer.set_scene(&scene, None);
    assert_eq!(
        colour_pixels(&mut renderer),
        before,
        "Actual fixed text behind the opaque solid is hidden"
    );
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        mesh.value()
    );
    scene
        .place_text(
            id,
            session_rust::Xform::translation(0., 0., -0.45),
        )
        .unwrap();
    renderer.set_scene(&scene, None);
    let front = colour_pixels(&mut renderer);
    assert_ne!(front, before);
    assert_eq!(renderer.text_error(), None);
    assert!(
        front
            .chunks_exact(4)
            .filter(|p| p[0] < 10 && p[1] < 10 && p[2] < 10)
            .count()
            > 20
    );
    let raster = renderer.text_scene_usage()[2];
    assert_eq!(
        raster, 1,
        "Placement reuses already rasterized original words"
    );
    assert_eq!(renderer.text_usage()[0], 1);
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap(),
        [id.value(), 0]
    );
    renderer
        .select_objects(&scene, &[id])
        .unwrap();
    let selected = colour_pixels(&mut renderer);
    assert!(
        selected
            .chunks_exact(4)
            .filter(|p| p[0] > 200 && p[1] > 200 && p[2] < 40)
            .count()
            > 20
    );
    assert_eq!(renderer.text_scene_usage()[2], raster);
    assert_eq!(renderer.text_usage()[0], 1);
    assert_eq!(&renderer.stats()[..2], &gpu[..2]);
    assert_eq!(
        serde_json::to_string(&*scene.texts()[0].prepared.source).unwrap(),
        original
    );
    Rc::make_mut(&mut scene.view).lock(&[id], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.06, -0.008], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "The locked plate remains an occluder without a selectable id"
    );
    Rc::make_mut(&mut scene.view).hide(&[id], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        mesh.value()
    );
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(renderer.text_scene_usage(), [0; 4]);
    assert_eq!(renderer.text_usage(), [0; 6]);
}

#[test]
fn actual_anchor_nameplate_and_billboard_follow_the_current_depth_and_plate_rules() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    for (placement, overlay) in [
        (
            TextPlacement::Anchor {
                world: [0., 0., 0.6],
                offset: [0.; 2],
            },
            false,
        ),
        (
            TextPlacement::Nameplate {
                world: [0., 0., 0.6],
                padding: [17., 4.],
                rounded: true,
            },
            true,
        ),
        (
            TextPlacement::WorldBillboard {
                world: [0., 0., 0.6],
                world_height: 0.08,
            },
            false,
        ),
    ] {
        let mut scene = Scene::demo();
        scene.clear();
        let mesh = scene.insert(triangle(0.2)).unwrap();
        let mut renderer = renderer(&scene);
        let before = colour_pixels(&mut renderer);
        let mut source = example(placement);
        source.label.text = "Hi".into();
        let id = scene
            .insert_text(Prepared::new(Rc::new(source)).unwrap())
            .unwrap();
        renderer.set_scene(&scene, None);
        let after = colour_pixels(&mut renderer);
        assert_eq!(
            after != before,
            overlay,
            "Only the nameplate overlays its own solid"
        );
        renderer
            .request_pick([0., 0.], identity())
            .unwrap();
        assert_eq!(
            finish(&mut renderer).unwrap()[0],
            if overlay {
                id.value()
            } else {
                mesh.value()
            }
        );
        assert_eq!(renderer.text_error(), None);
        let fonts = vec![
            crate::text::FONT_BYTES.to_vec(),
            crate::text::FALLBACK_BYTES.to_vec(),
            crate::text::SYMBOL_BYTES.to_vec(),
        ];
        assert!(
            renderer
                .replace_text_fonts(vec![vec![0; 8]])
                .is_err()
        );
        assert_eq!(colour_pixels(&mut renderer), after);
        renderer.replace_text_fonts(fonts).unwrap();
        assert_eq!(colour_pixels(&mut renderer), after);
        assert_eq!(renderer.text_usage()[5], 1);
    }
}

#[test]
fn actual_world_text_clip_uses_canvas_coordinates_in_the_small_integer_pick_window() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    let mut scene = Scene::demo();
    scene.clear();
    let mut source = example(TextPlacement::WorldPlane {
        world: [-0.2, 0.1, 0.3],
        right: [1., 0., 0.],
        up: [0., 1., 0.],
        world_height: 0.2,
    });
    source.label.text = "Hi".into();
    source.label.clip = Some([140., 100., 160., 140.]);
    let id = scene
        .insert_text(Prepared::new(Rc::new(source)).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    assert_eq!(renderer.text_error(), None);
    let mut clean = Scene::demo();
    clean.clear();
    let mut no_text = super::pick_gpu_tests::renderer(&clean);
    let baseline = colour_pixels(&mut no_text);
    let mut changed = 0;
    for (i, (p, b)) in before
        .chunks_exact(4)
        .zip(baseline.chunks_exact(4))
        .enumerate()
    {
        let (x, y) = (i % 320, i / 320);
        if x < 140 || x >= 160 || y < 100 || y >= 140 {
            assert_eq!(
                p, b,
                "Clipped text does not affect pixels outside the CSS clip"
            );
        } else {
            changed += usize::from(p != b);
        }
    }
    assert!(changed > 50);
    renderer
        .request_pick([-0.0625, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    renderer
        .request_pick([0.125, 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_selected_silhouette_has_one_union_no_internal_diagonal_and_keeps_source_and_picks()
 {
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = |x: f64| {
        let mut source = session_rust::Mesh::create_box(0.6, 0.8, 0.1);
        source.set_objectcolor(session_rust::Color::new(0.5, 0.5, 0.5, 1.));
        let mut prepared = PreparedMesh::new(source).unwrap();
        prepared.model = session_rust::Xform::translation(x, 0., 0.5);
        prepared
    };
    let a = scene.insert(mesh(-0.3)).unwrap();
    let b = scene.insert(mesh(0.3)).unwrap();
    let sources = scene
        .objects()
        .iter()
        .map(|row| row.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let mut renderer = renderer(&scene);
    let geometry = renderer.stats();
    renderer
        .select_objects(&scene, &[a, b])
        .unwrap();
    let first = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    assert_eq!(alpha.len(), 320 * 240);
    assert!(alpha.iter().filter(|v| **v > 0).count() > 300);
    for y in 90..150 {
        for x in 90..230 {
            assert_eq!(
                alpha[y * 320 + x],
                0,
                "The combined coverage has no outline along the shared object join or tessellation diagonals"
            );
        }
    }
    let stats = renderer.outline_usage();
    assert_eq!(colour_pixels(&mut renderer), first);
    assert_eq!(renderer.outline_alpha(), alpha);
    assert_eq!(renderer.outline_usage()[1], stats[1]);
    assert!(renderer.outline_usage()[2] > stats[2]);
    assert_eq!(&renderer.stats()[..2], &geometry[..2]);
    assert_eq!(
        scene
            .objects()
            .iter()
            .map(|row| row.geometry().unwrap().to_proto())
            .collect::<Vec<_>>(),
        sources
    );
    renderer
        .request_pick([-0.3, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], a.value());
    renderer
        .request_pick([0.3, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], b.value());
    renderer.select_objects(&scene, &[]).unwrap();
    colour_pixels(&mut renderer);
    assert!(renderer.outline_alpha().is_empty());
    assert_eq!(renderer.outline_usage()[0], 0);
}

#[test]
fn actual_surface_outline_culls_occluded_selection_and_selected_edge_ink_cannot_narrow_its_alpha()
 {
    use crate::pick_sources::{Part, Target};
    let mut scene = Scene::demo();
    scene.clear();
    let back = scene.insert(triangle(0.6)).unwrap();
    let front = scene.insert(triangle(0.2)).unwrap();
    let mut renderer = renderer(&scene);
    renderer
        .select_objects(&scene, &[back])
        .unwrap();
    colour_pixels(&mut renderer);
    assert!(
        renderer
            .outline_alpha()
            .iter()
            .all(|v| *v == 0),
        "A selected surface wholly behind the current opaque depth adds no silhouette"
    );
    renderer
        .select_objects(&scene, &[front])
        .unwrap();
    let plain = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    assert!(alpha.iter().any(|v| *v > 128));
    let edge = scene
        .objects()
        .iter()
        .find(|row| row.id == front)
        .unwrap()
        .mesh
        .parts
        .edges[0];
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner: front,
                part: Part::MeshEdge(edge.0, edge.1),
            }),
        )
        .unwrap();
    let inked = colour_pixels(&mut renderer);
    assert_eq!(
        renderer.outline_alpha(),
        alpha,
        "Source edge highlighting cannot change the actual outline width"
    );
    let mut opaque = 0;
    for ((a, b), coverage) in plain
        .chunks_exact(4)
        .zip(inked.chunks_exact(4))
        .zip(&alpha)
    {
        if *coverage >= 250 && a[..3].iter().all(|c| *c < 8) {
            opaque += 1;
            assert!(
                b[..3].iter().all(|c| *c < 12),
                "Yellow source edge ink cannot paint over black outline pixels"
            );
        }
    }
    assert!(opaque > 100);
    scene.clear();
    renderer.set_scene(&scene, None);
    colour_pixels(&mut renderer);
    assert_eq!(renderer.outline_usage()[0], 0);
}

#[test]
fn actual_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths = Vec::new();
    for curved in [false, true] {
        let mut scene = Scene::demo();
        scene.clear();
        let id = if curved {
            scene
                .insert_cad(Rc::new(session_rust::BRep::create_cylinder(
                    0.4, 0.8,
                )))
                .unwrap()
        } else {
            let mut prepared =
                PreparedMesh::new(session_rust::Mesh::create_box(0.8, 0.8, 0.2))
                    .unwrap();
            prepared.model = session_rust::Xform::translation(0., 0., 0.5);
            scene.insert(prepared).unwrap()
        };
        let mut renderer = renderer(&scene);
        renderer
            .select_objects(&scene, &[id])
            .unwrap();
        let geometry = renderer.stats();
        let mut css = Vec::new();
        for density in [1., 2.] {
            renderer.set_density(density);
            colour_pixels(&mut renderer);
            let alpha = renderer.outline_alpha();
            assert!(alpha.iter().any(|v| *v == 255));
            let row = &alpha[120 * 320..121 * 320];
            let width = row[160..]
                .iter()
                .map(|v| *v as f64 / 255.)
                .sum::<f64>()
                / density as f64;
            assert!(
                (0.7..1.7).contains(&width),
                "Actual outer silhouette CSS coverage remains bounded: {width}"
            );
            css.push(width);
            assert_eq!(&renderer.stats()[..2], &geometry[..2]);
        }
        assert!(
            (css[0] - css[1]).abs() < 0.5,
            "DPI keeps the outline in CSS units: {css:?}"
        );
        widths.push(css);
    }
    for i in 0..2 {
        assert!(
            (widths[0][i] - widths[1][i]).abs() < 0.25,
            "A smooth cylinder and flat box use one coverage width: {widths:?}"
        );
    }
}

#[test]
fn actual_selected_bore_keeps_its_open_center_and_black_inner_contour() {
    let source = Rc::new(session_rust::BRep::create_block_with_hole(
        1.2, 1.2, 0.4, 0.25,
    ));
    let _ = source.guid();
    let original = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert_cad(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    renderer
        .select_objects(&scene, &[id])
        .unwrap();
    let pixels = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    for y in 112..128 {
        for x in 152..168 {
            assert_eq!(
                alpha[y * 320 + x],
                0,
                "The bore center stays open"
            );
        }
    }
    let mut inner = 0;
    for y in 80..160 {
        for x in 110..210 {
            let i = y * 320 + x;
            if alpha[i] >= 250 {
                inner += 1;
                assert!(
                    pixels[i * 4..i * 4 + 3]
                        .iter()
                        .all(|c| *c < 12),
                    "The actual inner silhouette is black"
                );
            }
        }
    }
    assert!(
        inner > 100,
        "The real bore boundary has an inner outline: {inner}"
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    renderer
        .request_pick([0.4, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    assert_eq!(
        source.to_proto(),
        original,
        "Coverage cannot change original topology"
    );
}

#[test]
fn actual_cloud_pixels_and_whole_ids_keep_cached_source_uploads_density_and_locked_occlusion()
 {
    let mut source = session_rust::PointCloud::from_coords(
        vec![0., 0., 0.2],
        vec![255, 0, 0, 255],
        vec![],
    );
    source.point_size = 16.;
    let source = Rc::new(source);
    let _ = source.guid();
    let before = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = scene.insert(triangle(0.6)).unwrap();
    let cloud = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    for density in [1., 2.] {
        renderer.set_density(density);
        let first = colour_pixels(&mut renderer);
        let center = &first[(120 * 320 + 160) * 4..(120 * 320 + 160) * 4 + 4];
        assert!(
            center[0] > 240 && center[1] < 10 && center[2] < 10,
            "Original cloud color covers the solid behind it"
        );
        let usage = renderer.cloud_usage();
        assert_eq!(usage[0], 1);
        assert_eq!(usage[2], 1);
        assert!(usage[1] > 320 * 240 * 16);
        assert_eq!(colour_pixels(&mut renderer), first);
        assert_eq!(renderer.cloud_usage()[3], usage[3]);
        assert!(renderer.cloud_usage()[4] > usage[4]);
        renderer
            .request_pick([0., 0.], identity())
            .unwrap();
        assert_eq!(
            finish(&mut renderer),
            Some([cloud.value(), 0])
        );
    }
    renderer.set_density(1.);
    Rc::make_mut(&mut scene.view).lock(&[cloud], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer),
        None,
        "The locked cloud writes zero while occluding the mesh"
    );
    Rc::make_mut(&mut scene.view).hide(&[cloud], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer),
        Some([mesh.value(), 0])
    );
    assert_eq!(source.to_proto(), before);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}
#[test]
fn actual_cloud_copy_and_normal_shading_preserve_one_source_and_release_all_derived_owners()
 {
    let source = Rc::new(session_rust::PointCloud::from_coords(
        vec![-0.3, 0., 0.2, 0.3, 0., 0.2],
        vec![0, 255, 0, 255, 0, 255, 0, 255],
        vec![0., 0., 1., 0., 0., 1.],
    ));
    let weak = Rc::downgrade(&source);
    let mut scene = Scene::demo();
    scene.clear();
    let cloud = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let pixels = colour_pixels(&mut renderer);
    assert!(
        pixels
            .chunks_exact(4)
            .any(|p| p[1] > 160 && p[0] < 10 && p[2] < 10)
    );
    assert_eq!(renderer.cloud_usage()[2], 1);
    let copy = scene.copy_row(cloud, false).unwrap();
    scene.translate(copy, [0., 0.3, 0.]).unwrap();
    renderer.set_scene(&scene, None);
    colour_pixels(&mut renderer);
    assert_eq!(
        renderer.cloud_usage()[2],
        1,
        "Copies share one original upload"
    );
    assert_eq!(renderer.cloud_usage()[0], 4);
    renderer
        .request_pick([-0.3, 0.3], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        copy.value()
    );
    drop(source);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
    assert!(weak.upgrade().is_none());
}

fn camera_pixels(renderer: &mut Renderer, camera: &crate::camera::Camera) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("selection pixels"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    renderer.cloud_camera(camera);
    renderer.draw_mode(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &camera.uniform(),
        false,
    );
    let bytes = renderer
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection readback"),
            size: 320 * 240 * 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut encoder = renderer
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &bytes,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280),
                rows_per_image: Some(240),
            },
        },
        texture.size(),
    );
    renderer.queue.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::channel();
    bytes
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = send.send(result);
        });
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    receive
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap()
        .unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec();
    bytes.unmap();
    result
}

#[test]
fn actual_large_cloud_changes_detail_with_the_view_and_reuses_its_original_upload() {
    let source = Rc::new(crate::cloud::large_example());
    assert_eq!(source.len(), 2_000_064);
    let mut scene = Scene::demo();
    scene.clear();
    scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.aspect = 320. / 240.;
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 100. / std::f64::consts::FRAC_PI_6.tan();
    let coarse = camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    let uploads = renderer.cloud_usage()[2];
    let prepared = renderer.cloud_usage()[3];
    assert_eq!(camera_pixels(&mut renderer, &camera), coarse);
    assert_eq!(renderer.cloud_usage()[3], prepared);
    camera.distance = 0.7 / std::f64::consts::FRAC_PI_6.tan();
    let fine = camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 2_000_064);
    assert_ne!(fine, coarse);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    assert_eq!(source.point_id(64), 64);
    assert_eq!(source.point_id(source.len() - 1), 2_000_063);
    camera.distance = 100. / std::f64::consts::FRAC_PI_6.tan();
    assert_eq!(camera_pixels(&mut renderer, &camera), coarse);
    assert_eq!(renderer.cloud_usage()[0], 64);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}

#[test]
fn actual_cloud_control_pick_preserves_full_u32_source_ids_and_local_highlight() {
    use crate::pick_sources::{Mode, Part};
    let mut proto = session_rust::PointCloud::from_coords(
        vec![-0.4, 0., 0.2, 0.4, 0., 0.2],
        vec![],
        vec![],
    )
    .to_proto();
    proto.point_size = 16.;
    proto.point_ids = vec![u32::MAX, 16_777_217];
    let source = Rc::new(session_rust::PointCloud::from_proto(proto));
    let _ = source.guid();
    let original = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    scene.cloud_controls(Some(owner));
    let mut renderer = renderer(&scene);
    renderer.set_pick_mode(Mode::Control);
    let uploaded = renderer.cloud_usage()[2];
    let geometry = renderer.stats();
    for density in [1., 2.] {
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320. * density) as u32,
            height: (240. * density) as u32,
        });
        for (x, id) in [(-0.4, u32::MAX), (0.4, 16_777_217)] {
            renderer
                .request_pick([x, 0.], identity())
                .unwrap();
            let pixel = finish(&mut renderer).unwrap();
            let target = renderer.resolve_pick(&scene, pixel).unwrap();
            assert_eq!(target.part, Part::CloudPoint(id));
            assert_eq!(target.owner, owner);
            renderer
                .set_highlight(&scene, Some(target))
                .unwrap();
            assert!(renderer.highlight_bytes() > 0);
            assert_eq!(renderer.stats(), geometry);
        }
    }
    assert_eq!(renderer.cloud_usage()[2], uploaded);
    assert_eq!(source.to_proto(), original);
    Rc::make_mut(&mut scene.view).lock(&[owner], true);
    renderer.set_density(1.);
    renderer.resize(Viewport {
        width: 320,
        height: 240,
    });
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.4, 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    scene.clear();
    renderer.set_scene(&scene, None);
    renderer.set_highlight(&scene, None).unwrap();
    assert_eq!(renderer.cloud_usage(), [0; 5]);
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_resident_cloud_controls_query_original_points_outside_the_coarse_display_subset()
 {
    use crate::pick_sources::{Mode, Part};
    let mut proto = crate::cloud::large_example().to_proto();
    for xyz in proto.coords[..64 * 3].chunks_exact_mut(3) {
        xyz[2] = 0.;
    }
    let source = Rc::new(session_rust::PointCloud::from_proto(proto));
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.aspect = 320. / 240.;
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 100. / std::f64::consts::FRAC_PI_6.tan();
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    let uploads = renderer.cloud_usage()[2];
    scene.cloud_controls(Some(owner));
    renderer.set_scene(&scene, None);
    renderer.set_pick_mode(Mode::Control);
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 2_000_064);
    let transform = camera.view_projection().to_f32();
    let p = camera
        .view_projection()
        .transform_point(&session_rust::Point::new(0.25, 0.25, 0.2));
    renderer
        .request_pick([p[0] as f32, p[1] as f32], transform)
        .unwrap();
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let pixel = renderer
        .take_pick(transform)
        .unwrap()
        .unwrap()
        .unwrap();
    let target = renderer.resolve_pick(&scene, pixel).unwrap();
    let Part::CloudPoint(id) = target.part else {
        panic!("Original cloud point identity");
    };
    assert!(id >= 64);
    assert!(target.valid(&scene));
    assert_eq!(renderer.cloud_usage()[2], uploads);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    renderer.cancel_pick();
    assert!(renderer.take_pick(transform).is_none());
    scene.cloud_controls(None);
    renderer.set_scene(&scene, None);
    renderer.set_pick_mode(Mode::Object);
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}

#[test]
fn actual_streamed_pages_pick_original_ids_share_uploads_and_release_after_eviction() {
    use crate::pick_sources::{Mode, Part};
    let prepared = crate::range_example::prepared();
    let weak = Rc::downgrade(&prepared.source);
    let mut scene = Scene::empty();
    let owner = scene
        .insert_stream(
            prepared.clone(),
            session_rust::Xform::identity(),
            12.,
        )
        .unwrap();
    scene.cloud_controls(Some(owner));
    let mut renderer = renderer(&scene);
    renderer.set_pick_mode(Mode::Control);
    for (cursor, id) in [
        ([-0.5, 0.5], u32::MAX),
        ([0.5, 0.5], 16_777_217),
        ([-0.5, -0.5], 10),
    ] {
        renderer
            .request_pick(cursor, identity())
            .unwrap();
        let hit = finish(&mut renderer).expect("Actual streamed point coverage");
        let target = renderer.resolve_pick(&scene, hit).unwrap();
        assert_eq!(target.owner, owner);
        assert_eq!(target.part, Part::CloudPoint(id));
    }
    let pixels = colour_pixels(&mut renderer);
    assert!(
        pixels
            .chunks_exact(4)
            .any(|p| p[2] > 160 && p[0] < 10)
    );
    assert!(
        pixels
            .chunks_exact(4)
            .any(|p| p[0] > 160 && p[2] < 10)
    );
    let uploads = renderer.cloud_usage()[2];
    let copy = scene.copy_row(owner, false).unwrap();
    scene.translate(copy, [0.2, 0., 0.]).unwrap();
    renderer.set_scene(&scene, None);
    colour_pixels(&mut renderer);
    assert_eq!(
        renderer.cloud_usage()[2],
        uploads,
        "Placed owners share original page uploads"
    );
    Rc::make_mut(&mut scene.view).lock(&[owner, copy], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.5, 0.5], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).lock(&[owner, copy], false);
    let held = Rc::downgrade(&prepared.page(2).unwrap());
    prepared.pages.borrow_mut().clear();
    assert!(
        held.upgrade().is_some(),
        "GPU page owner retains precise pick identities"
    );
    renderer.set_scene(&scene, None);
    assert!(held.upgrade().is_none());
    renderer
        .request_pick([-0.5, 0.5], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "Evicted detailed points are absent"
    );
    renderer
        .request_pick([-0.5, -0.5], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer
            .resolve_pick(&scene, hit)
            .unwrap()
            .part,
        Part::CloudPoint(10)
    );
    scene.clear();
    renderer.set_scene(&scene, None);
    drop(prepared);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
    assert!(weak.upgrade().is_none());
}

#[test]
fn actual_definition_instances_batch_one_geometry_keep_integer_picks_and_release_records()
 {
    let mut original = crate::instance_specimen::message();
    original.objects.as_mut().unwrap().instances[0].xform = None;
    original.objects.as_mut().unwrap().instances[1].flags = 0;
    original.xforms = vec![
        session_rust::proto::XformEntry {
            guid: "first".into(),
            xform: Some(session_rust::Xform::translation(-0.5, 0., 0.3).to_proto()),
        },
        session_rust::proto::XformEntry {
            guid: "second".into(),
            xform: Some(session_rust::Xform::translation(0.5, 0., 0.3).to_proto()),
        },
    ];
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    crate::scene_document::import(
        &mut scene,
        &prost::Message::encode_to_vec(&original),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    let a = scene.objects()[0].id;
    let b = scene.objects()[1].id;
    let weak = Rc::downgrade(scene.objects()[0].geometry().unwrap());
    let mut renderer = renderer(&scene);
    assert_eq!(
        &renderer.instance_usage()[..3],
        &[1, 2, 16760]
    );
    let source_uploads = renderer.stats()[0];
    let pixels = colour_pixels(&mut renderer);
    assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
    for (cursor, id) in [([-0.5, -0.1], a), ([0.5, -0.1], b)] {
        renderer
            .request_pick(cursor, identity())
            .unwrap();
        assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    }
    let mut coloured = original.clone();
    let first = &mut coloured.objects.as_mut().unwrap().instances[0];
    first.flags |= session_rust::InstanceRef::FLAG_COLOR;
    first.color = Some(session_rust::proto::Color {
        r: 0.9,
        g: 0.08,
        b: 0.05,
        a: 0.5,
        ..Default::default()
    });
    let mut colour_scene = Scene::empty();
    crate::scene_document::import(
        &mut colour_scene,
        &prost::Message::encode_to_vec(&coloured),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    let mut colour_renderer = super::pick_gpu_tests::renderer(&colour_scene);
    let colours = colour_pixels(&mut colour_renderer);
    assert!(colours.chunks_exact(4).filter(|p|p[0]>p[1].saturating_add(50)&&p[0]>p[2].saturating_add(50)).count()>500);
    assert!(
        colours
            .chunks_exact(4)
            .filter(|p| p[2] > p[0].saturating_add(20))
            .count()
            > 500
    );
    let writes = renderer.instance_usage()[3];
    colour_pixels(&mut renderer);
    assert_eq!(renderer.instance_usage()[3], writes);
    scene.translate(a, [0., 0.2, 0.]).unwrap();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.stats()[0], source_uploads);
    assert!(renderer.instance_usage()[3] > writes);
    renderer
        .select_objects(&scene, &[a])
        .unwrap();
    let selected = colour_pixels(&mut renderer);
    assert!(
        selected
            .chunks_exact(4)
            .any(|p| p[0] > 140 && p[1] > 90 && p[2] < 40)
    );
    Rc::make_mut(&mut scene.view).hide(&[b], true);
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.instance_usage()[1], 1);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(&renderer.instance_usage()[..3], &[0, 0, 0]);
    assert!(weak.upgrade().is_none());
}

#[test]
fn actual_congruent_cad_replay_matches_independent_faces_ink_and_owner_picks() {
    let first = Rc::new(
        session_rust::BRep::create_block_with_hole(0.7, 0.6, 0.2, 0.12).transformed(
            &session_rust::Xform::translation(-0.35, -0.3, 0.3),
        ),
    );
    let mut cache = crate::cad_shape_cache::Cache::default();
    cache.prepare(first.clone()).unwrap();
    let motion = &session_rust::Xform::translation(0.1, -0.1, 0.)
        * &session_rust::Xform::rotation_z(90., true);
    let copy = Rc::new(first.transformed(&motion));
    let mut shared = Scene::empty();
    let id = shared
        .insert_prepared_cad(cache.prepare(copy.clone()).unwrap())
        .unwrap();
    assert_eq!(cache.replays, 1);
    let mut alone = Scene::empty();
    alone.insert_cad(copy.clone()).unwrap();
    let mut shared_renderer = renderer(&shared);
    let mut independent_renderer = renderer(&alone);
    let a = colour_pixels(&mut shared_renderer);
    let b = colour_pixels(&mut independent_renderer);
    let changed = a
        .chunks_exact(4)
        .zip(b.chunks_exact(4))
        .filter(|(a, b)| {
            a[..3]
                .iter()
                .zip(&b[..3])
                .any(|(a, b)| a.abs_diff(*b) > 3)
        })
        .count();
    assert!(
        changed < 100,
        "Congruent faces and source ink differed at {changed} pixels"
    );
    assert!(
        a.chunks_exact(4)
            .filter(|p| p[0] < 180 && p[1] < 180 && p[2] < 180)
            .count()
            > 500
    );
    let mut hits = 0;
    for point in [[-0.1, -0.05], [0.25, -0.05], [0.1, 0.15]] {
        shared_renderer
            .request_pick(point, identity())
            .unwrap();
        independent_renderer
            .request_pick(point, identity())
            .unwrap();
        let actual = finish(&mut shared_renderer);
        let independent = finish(&mut independent_renderer);
        assert_eq!(actual, independent);
        if let Some(actual) = actual {
            assert_eq!(actual[0], id.value());
            hits += 1;
        }
    }
    assert!(
        hits > 0,
        "Actual original CAD ownership must be picked"
    );
    let weak = Rc::downgrade(&copy);
    drop(copy);
    shared.clear();
    alone.clear();
    shared_renderer.set_scene(&shared, None);
    independent_renderer.set_scene(&alone, None);
    drop(cache);
    assert!(weak.upgrade().is_none());
}

#[test]
fn actual_nested_rotated_and_mirrored_definition_placements_keep_independent_integer_owners()
 {
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    crate::scene_document::import(
        &mut scene,
        &crate::nested_example::bytes(),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 16760]);
    for row in scene.objects() {
        let point = row
            .model
            .transform_point(&session_rust::Point::new(
                (0.123456789012345 + 0.8) / 3.,
                0.8 / 3.,
                0.,
            ));
        gpu.request_pick(
            [point[0] as f32, point[1] as f32],
            identity(),
        )
        .unwrap();
        assert_eq!(finish(&mut gpu).unwrap()[0], row.id.value());
    }
    let id = scene.objects()[1].id;
    gpu.select_objects(&scene, &[id]).unwrap();
    let pixels = colour_pixels(&mut gpu);
    assert!(
        pixels
            .chunks_exact(4)
            .filter(|p| p[0] > 140 && p[1] > 90 && p[2] < 40)
            .count()
            > 500
    );
    let bytes = crate::scene_archive::save(&scene).unwrap();
    let mut restored = Scene::empty();
    crate::scene_document::open(&mut restored, &bytes).unwrap();
    for a in scene.objects() {
        let guid = scene.instances[&a.id].instance.guid();
        let b = restored
            .objects()
            .iter()
            .find(|row| restored.instances[&row.id].instance.guid() == guid)
            .unwrap();
        for (a, b) in a.model.m.iter().zip(&b.model.m) {
            assert!(
                (a - b).abs() < 1e-12,
                "Canonical placement for {guid}: {a} {b}"
            );
        }
    }
}

#[test]
fn actual_sheet_batches_share_source_uploads_keep_world_pens_full_entity_ids_and_locked_occlusion()
 {
    use crate::pick_sources::{Mode, Part, Target};
    let mut source = crate::sheet_example::source();
    source.entity_count = u32::MAX;
    source.source_ids = vec![u32::MAX - 1; 5];
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    let id = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(Rc::new(source)).unwrap(),
        ))
        .unwrap();
    let copy = scene.copy_row(id, false).unwrap();
    scene
        .place_sheet(
            copy,
            session_rust::Xform::translation(0., 0.1, 0.),
        )
        .unwrap();
    let weak = Rc::downgrade(&scene.sheets()[0].prepared);
    let mut renderer = renderer(&scene);
    assert_eq!(renderer.sheet_usage()[..2], [2, 1]);
    let uploads = renderer.sheet_usage()[3];
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.sheet_usage()[3], uploads);
    renderer.set_pick_mode(Mode::Edge);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&scene, hit),
        Some(Target {
            owner: id,
            part: Part::SheetEntity(u32::MAX - 1)
        })
    );
    renderer
        .request_pick([0., -0.4], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&scene, hit),
        Some(Target {
            owner: copy,
            part: Part::SheetEntity(u32::MAX - 1)
        })
    );
    assert!(hit[1] < u32::MAX - 1);
    let pixels = colour_pixels(&mut renderer);
    let black = (0..240)
        .filter(|y| {
            let p = &pixels[(*y * 320 + 160) * 4..][..4];
            p[0] < 30 && p[1] < 30 && p[2] < 30
        })
        .count();
    assert!(
        black >= 8 && black < 20,
        "Physical .025mm pens draw separate three-pixel boundaries: {black}"
    );
    scene
        .place_sheet(
            copy,
            session_rust::Xform::translation(0., 0.2, 0.),
        )
        .unwrap();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.sheet_usage()[3], uploads);
    let front = scene.insert(triangle(0.1)).unwrap();
    renderer.set_scene(&scene, None);
    renderer.set_pick_mode(Mode::Object);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        front.value()
    );
    Rc::make_mut(&mut scene.view).lock(&[front], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).hide(&[front], true);
    Rc::make_mut(&mut scene.view).lock(&[id], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.sheet_usage()[..2], [0, 0]);
    assert!(weak.upgrade().is_none());
}

#[test]
fn actual_evicted_sheet_pages_remain_charged_until_renderer_refresh_cancels_old_picks()
{
    let budget = crate::page_cache::Budget::new(256);
    let source = Rc::new(crate::sheet_range::Source {
        guid: crate::sheet_example::source().guid,
        name: "paged drawing".into(),
        url: "plan.pb".into(),
        fields: crate::stream::SheetFields {
            count: 2,
            entities: u32::MAX,
            ..Default::default()
        },
    });
    let stream = crate::sheet_stream::Stream::new(source, budget.clone());
    let raw = [-0.5f64, 0., 0.3, 0.5, 0., 0.3]
        .into_iter()
        .flat_map(f64::to_le_bytes)
        .collect::<Vec<_>>();
    let page = crate::sheet_page::Page::decode(
        0..1,
        2,
        u32::MAX,
        &raw,
        None,
        None,
        Some(&(u32::MAX - 1).to_le_bytes()),
    )
    .unwrap();
    let held = stream
        .adopt(page, budget.reserve(192).unwrap())
        .unwrap();
    let weak = Rc::downgrade(&held);
    let mut scene = Scene::empty();
    let id = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::streamed(stream.clone()),
        ))
        .unwrap();
    let mut renderer = renderer(&scene);
    drop(held);
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge);
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(hit[0], id.value());
    assert!(renderer.resolve_pick(&scene, hit).is_some());
    assert_eq!(budget.used(), 84);
    stream.pages.borrow_mut().clear();
    assert_eq!(budget.used(), 84);
    assert!(budget.reserve(192).is_err());
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    renderer.set_scene(&scene, None);
    assert!(renderer.take_pick(identity()).is_none());
    assert_eq!(budget.used(), 0);
    assert!(weak.upgrade().is_none());
    assert!(renderer.resolve_pick(&scene, hit).is_none());
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_sheet_pen_projection_uses_camera_lens_independently_of_panel_frame_shift() {
    let source = session_rust::proto::Sheet {
        guid: crate::sheet_example::source().guid,
        coords: vec![-0.6, 0., 0.3, 0.6, 0., 0.3],
        widths: vec![0.35],
        segment_count: 1,
        ..Default::default()
    };
    let mut scene = Scene::empty();
    scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(Rc::new(source)).unwrap(),
        ))
        .unwrap();
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    let pen_rows = |pixels: Vec<u8>| {
        pixels
            .chunks_exact(320 * 4)
            .filter(|row| row[160 * 4..][..3].iter().all(|v| *v < 30))
            .count()
    };
    renderer.sheet_camera(&camera);
    let first = pen_rows(colour_pixels_at(
        &mut renderer,
        camera.uniform(),
    ));
    assert!(first > 20);
    camera.frame = [0., 0., 1., 0.2];
    renderer.sheet_camera(&camera);
    let shifted = pen_rows(colour_pixels_at(
        &mut renderer,
        camera.uniform(),
    ));
    assert_eq!(
        shifted, first,
        "A panel shifts the frame without changing the original world pen"
    );
}

#[test]
fn actual_mesh_control_pixel_keeps_sparse_vertex_key_at_both_densities() {
    let mut mesh = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.5, -0.5, 0.25]),
        (20, [0.5, -0.5, 0.25]),
        (30, [0., 0.5, 0.25]),
    ] {
        mesh.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    mesh.add_face(vec![10, 20, 30], Some(37));
    let mut scene = Scene::empty();
    let owner = scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    scene.show_controls(true);
    let source = crate::source_edit::original(&scene, owner).unwrap();
    let model = session_rust::Xform::translation(0.125, 0.25, 0.);
    scene.place(owner, model).unwrap();
    for density in [1., 2.] {
        let mut renderer = renderer(&scene);
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320. * density) as u32,
            height: (240. * density) as u32,
        });
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);
        renderer
            .request_pick([-0.375, -0.25], identity())
            .unwrap();
        let pixel = finish(&mut renderer).expect("Actual placed control pixel");
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::SourceControl(
                    crate::source_controls::ControlId::Vertex(10)
                )
            })
        );
        scene.show_controls(false);
        renderer.set_scene(&scene, None);
        renderer
            .request_pick([-0.375, -0.25], identity())
            .unwrap();
        assert!(finish(&mut renderer).is_none());
        scene.show_controls(true);
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}

#[test]
fn actual_controls_are_blue_seven_css_pixels_and_selected_yellow_at_both_densities() {
    let mut mesh = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.5, -0.5, 0.25]),
        (20, [0.1, 0.1, 0.25]),
        (30, [0.8, 0.1, 0.25]),
        (40, [0.1, 0.8, 0.25]),
    ] {
        mesh.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    mesh.add_face(vec![20, 30, 40], Some(37));
    let mut scene = Scene::empty();
    let owner = scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    scene.show_controls(true);
    assert!(
        scene
            .control_markers()
            .iter()
            .all(|m| m.display.diameter == 7.
                && m.display.colour == [0.15, 0.35, 0.9, 1.])
    );
    let source = crate::source_edit::original(&scene, owner).unwrap();
    let target = crate::pick_sources::Target {
        owner,
        part: crate::pick_sources::Part::SourceControl(
            crate::source_controls::ControlId::Vertex(10),
        ),
    };
    for density in [1., 2.] {
        let mut renderer = renderer(&scene);
        renderer.set_density(density);
        let pixels = colour_pixels(&mut renderer);
        let center = (180 * 320 + 80) * 4;
        for (actual, expected) in pixels[center..center + 4]
            .iter()
            .zip([38_i16, 89, 230, 255])
        {
            assert!(
                (*actual as i16 - expected).abs() <= 2,
                "Actual blue control pixel {:?}",
                &pixels[center..center + 4]
            );
        }
        let coverage: f32 = (60..100)
            .map(|x| (255. - pixels[(180 * 320 + x) * 4] as f32) / 217.)
            .sum();
        assert!(
            (coverage / density - 7.).abs() < 0.6,
            "Seven CSS pixel row coverage at density {density}: {coverage}"
        );
        renderer
            .set_highlight(&scene, Some(target))
            .unwrap();
        let selected = colour_pixels(&mut renderer);
        assert_eq!(
            &selected[center..center + 4],
            &[255, 255, 0, 255]
        );
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);
        renderer
            .request_pick([-0.5, -0.5], identity())
            .unwrap();
        let pixel = finish(&mut renderer).unwrap();
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(target)
        );
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}

#[test]
fn actual_control_net_pixel_addresses_parent_without_a_fake_edge_or_control() {
    let mut editor = crate::editor::Editor::default();
    editor
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn actual_control_hits_resolve_the_original_curve_control_instead_of_a_sample() {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    editor
        .apply(crate::editor::Action::AddCurve)
        .unwrap();
    editor
        .apply(crate::editor::Action::Controls(true))
        .unwrap();
    let id = editor.scene.paths()[0].id;
    let mut renderer = renderer(&editor.scene);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control);
    renderer
        .request_pick([0., 0.6], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&editor.scene, hit),
        Some(crate::pick_sources::Target {
            owner: id,
            part: crate::pick_sources::Part::Control(1)
        })
    );
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge);
    renderer
        .request_pick([0., 0.1], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&editor.scene, hit),
        Some(crate::pick_sources::Target {
            owner: id,
            part: crate::pick_sources::Part::Curve
        })
    );
    editor.scene.show_controls(false);
    renderer.set_scene(&editor.scene, None);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control);
    renderer
        .request_pick([0., 0.6], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}

fn colour_pixels(renderer: &mut Renderer) -> Vec<u8> {
    colour_pixels_at(renderer, identity())
}
fn colour_pixels_at(renderer: &mut Renderer, transform: [f32; 16]) -> Vec<u8> {
    colour_pixels_display(renderer, transform, false)
}
fn colour_pixels_display(
    renderer: &mut Renderer,
    transform: [f32; 16],
    normals: bool,
) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("selection pixels"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    renderer.draw_mode(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &transform,
        normals,
    );
    let bytes = renderer
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection readback"),
            size: 320 * 240 * 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut encoder = renderer
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &bytes,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280),
                rows_per_image: Some(240),
            },
        },
        texture.size(),
    );
    renderer.queue.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::channel();
    bytes
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = send.send(result);
        });
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    receive
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap()
        .unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec();
    bytes.unmap();
    result
}

#[test]
fn actual_part_highlights_are_local_leave_black_edges_and_release_derived_data() {
    use crate::pick_sources::{Part, Target};
    let mut source = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.8, -0.6, 0.5]),
        (20, [0., -0.6, 0.5]),
        (30, [0., 0.6, 0.5]),
        (40, [-0.8, 0.6, 0.5]),
        (50, [0.8, -0.6, 0.5]),
        (60, [0.8, 0.6, 0.5]),
    ] {
        source.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    source.add_face(vec![10, 20, 30, 40], Some(37));
    source.add_face(vec![20, 50, 60, 30], Some(91));
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert(PreparedMesh::new(source).unwrap())
        .unwrap();
    let weak = Rc::downgrade(&scene.objects()[0].mesh);
    let mut renderer = renderer(&scene);
    let stats = renderer.stats();
    let before = colour_pixels(&mut renderer);
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner,
                part: Part::Face(37),
            }),
        )
        .unwrap();
    let face = colour_pixels(&mut renderer);
    assert!(renderer.highlight_bytes() > 0);
    let mut changed = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(face.chunks_exact(4))
        .enumerate()
    {
        if a != b {
            let x = index % 320;
            let y = index / 320;
            assert!((31..=160).contains(&x) && (47..=192).contains(&y));
            changed += 1;
        }
    }
    assert!(
        changed > 10000,
        "The original face, rather than the whole object, is visibly selected"
    );
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner,
                part: Part::MeshEdge(10, 20),
            }),
        )
        .unwrap();
    let edge = colour_pixels(&mut renderer);
    let mut changed = 0;
    let mut black = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(edge.chunks_exact(4))
        .enumerate()
    {
        let x = index % 320;
        let y = index / 320;
        if a != b {
            assert!((30..=161).contains(&x) && (190..=193).contains(&y));
            changed += 1;
        }
        if (35..150).contains(&x)
            && (191..=192).contains(&y)
            && b[0] < 160
            && b[1] < 140
            && b[2] < 80
        {
            black += 1;
        }
    }
    assert!(
        changed > 80 && black > 80,
        "A yellow fringe surrounds the original black edge: changed={changed}, dark={black}"
    );
    assert_eq!(
        renderer.stats(),
        stats,
        "Highlights reuse primary geometry"
    );
    renderer.set_highlight(&scene, None).unwrap();
    assert_eq!(renderer.highlight_bytes(), 0);
    assert_eq!(colour_pixels(&mut renderer), before);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_control_highlight_surrounds_only_the_original_marker_and_keeps_source_precision()
 {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    editor
        .apply(crate::editor::Action::AddCurve)
        .unwrap();
    editor
        .apply(crate::editor::Action::Controls(true))
        .unwrap();
    let owner = editor.scene.paths()[0].id;
    let original = editor.scene.paths()[0].points();
    let mut renderer = renderer(&editor.scene);
    let before = colour_pixels(&mut renderer);
    renderer
        .set_highlight(
            &editor.scene,
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::Control(1),
            }),
        )
        .unwrap();
    let after = colour_pixels(&mut renderer);
    let mut changed = 0;
    for (index, (a, b)) in before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .enumerate()
    {
        if a != b {
            let x = index % 320;
            let y = index / 320;
            assert!((151..=168).contains(&x) && (39..=56).contains(&y));
            changed += 1;
        }
    }
    assert!(
        changed > 20,
        "The halo belongs to the original control rather than a curve sample"
    );
    assert_eq!(editor.scene.paths()[0].points(), original);
    editor
        .apply(crate::editor::Action::Close)
        .unwrap();
    renderer.set_scene(&editor.scene, None);
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_multiple_mesh_selection_reuses_geometry_and_pick_ownership() {
    let mut scene = Scene::demo();
    scene.clear();
    let panel = |x: f64| {
        PreparedMesh::triangle(
            [
                [x - 0.25, -0.4, 0.5],
                [x + 0.25, -0.4, 0.5],
                [x, 0.4, 0.5],
            ],
            [0.5, 0.5, 0.5],
        )
        .unwrap()
    };
    let a = scene.insert(panel(-0.4)).unwrap();
    let b = scene.insert(panel(0.4)).unwrap();
    let mut renderer = renderer(&scene);
    let baseline = colour_pixels(&mut renderer);
    let geometry = renderer.stats();
    renderer
        .select_objects(&scene, &[a, b])
        .unwrap();
    let selected = colour_pixels(&mut renderer);
    let pixel = |bytes: &[u8], x: usize| {
        bytes[(120 * 320 + x) * 4..(120 * 320 + x) * 4 + 3].to_vec()
    };
    for x in [96, 224] {
        let p = pixel(&selected, x);
        assert!(p[0] > 0 && p[0] == p[1] && p[2] == 0);
    }
    renderer
        .select_objects(&scene, &[b])
        .unwrap();
    let one = colour_pixels(&mut renderer);
    assert_eq!(pixel(&one, 96), pixel(&baseline, 96));
    assert_eq!(pixel(&one, 224), pixel(&selected, 224));
    assert_eq!(
        &renderer.stats()[..2],
        &geometry[..2],
        "Only selected settings change, not primary geometry"
    );
    for (x, id) in [(-0.4, a), (0.4, b)] {
        renderer
            .request_pick([x, 0.], identity())
            .unwrap();
        assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    }
}

#[test]
fn actual_tool_preview_is_temporary_bounded_and_absent_from_object_picking() {
    let mut scene = Scene::demo();
    scene.clear();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let stats = renderer.stats();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit([-0.5, 0., 0.2]).unwrap();
    draft.hover(Some([0.5, 0., 0.2])).unwrap();
    renderer.set_preview(Some(&draft)).unwrap();
    assert_eq!(renderer.preview_bytes(), 76);
    let after = colour_pixels(&mut renderer);
    let changed = before
        .chunks_exact(4)
        .zip(after.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        changed > 100,
        "The actual unfinished rubber band and cursor render"
    );
    let p = &after[(120 * 320 + 160) * 4..(120 * 320 + 160) * 4 + 3];
    assert_eq!(
        p,
        &[255, 255, 0],
        "The preview uses current selection yellow: {p:?}"
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "Drafts have no saved-object pick owner"
    );
    assert_eq!(renderer.stats(), stats);
    assert!(scene.lines().is_empty());
    renderer.set_preview(None).unwrap();
    assert_eq!(renderer.preview_bytes(), 0);
    assert_eq!(colour_pixels(&mut renderer), before);
    renderer.set_preview(Some(&draft)).unwrap();
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.preview_bytes(), 0);
}

#[test]
fn actual_layer_colours_change_display_settings_without_editing_or_reuploading_original_geometry()
 {
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert(triangle(0.5)).unwrap();
    let source = Rc::clone(scene.objects()[0].geometry().unwrap());
    let original = source.to_proto();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let allocations = renderer.stats();
    Rc::make_mut(&mut scene.view)
        .faces
        .insert(id, [0, 0, 255]);
    renderer.set_scene(&scene, None);
    let blue = colour_pixels(&mut renderer);
    assert_ne!(blue, before);
    let centre = (120 * 320 + 160) * 4;
    assert!(
        blue[centre + 2] > blue[centre].saturating_add(100)
            && blue[centre + 2] > blue[centre + 1].saturating_add(100)
    );
    assert_eq!(
        &renderer.stats()[..2],
        &allocations[..2],
        "Layer colour reuses primary geometry and its settings allocation"
    );
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    Rc::make_mut(&mut scene.view)
        .edges
        .insert(id, [0, 255, 0]);
    renderer.set_scene(&scene, None);
    let green_edges = colour_pixels(&mut renderer);
    let count = green_edges
        .chunks_exact(4)
        .filter(|p| p[1] > p[0].saturating_add(100) && p[1] > p[2].saturating_add(100))
        .count();
    assert!(
        count > 100,
        "Actual source boundary colour is independent: {count}"
    );
    Rc::make_mut(&mut scene.view)
        .faces
        .remove(&id);
    renderer.set_scene(&scene, None);
    let restored_faces = colour_pixels(&mut renderer);
    assert_eq!(
        &restored_faces[centre..centre + 4],
        &before[centre..centre + 4]
    );
    assert_ne!(
        restored_faces, before,
        "Restoring Faces leaves the independent edge override"
    );
    Rc::make_mut(&mut scene.view)
        .edges
        .remove(&id);
    renderer.set_scene(&scene, None);
    assert_eq!(colour_pixels(&mut renderer), before);
    assert!(Rc::ptr_eq(
        &source,
        scene.objects()[0].geometry().unwrap()
    ));
}

#[test]
fn actual_delayed_gpu_answer_finishes_a_released_drag_and_reuses_its_geometry() {
    use crate::editor::{Action, Change, Editor};
    let mut e = crate::camera_defaults_tests::editor_fixture();
    e.scene.clear();
    let id = e.scene.insert(triangle(0.2)).unwrap();
    let original = e.scene.placement(id).unwrap().m;
    let source = e.scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    let mut renderer = renderer(&e.scene);
    let transform = e.camera.uniform();
    let gpu = renderer.stats();
    let size = [320., 240.];
    e.apply(Action::CanvasStart {
        from: [0., 0.],
        to: [0.2, 0.],
        size,
        modified: false,
        pointer: 1,
    })
    .unwrap();
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    e.apply(Action::CanvasRelease {
        screen: [0.4, 0.],
        size,
        add: false,
        remove: false,
    })
    .unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, original);
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let hit = renderer
        .take_pick(transform)
        .unwrap()
        .unwrap()
        .and_then(|pixel| renderer.resolve_pick(&e.scene, pixel));
    assert_eq!(hit.unwrap().owner, id);
    assert_eq!(
        e.apply(Action::CanvasHit(
            hit.map(|target| target.owner)
        ))
        .unwrap(),
        Change::Scene
    );
    assert_ne!(e.scene.placement(id).unwrap().m, original);
    assert!(Rc::ptr_eq(
        &source,
        e.scene.objects()[0].geometry().unwrap()
    ));
    renderer.set_scene(&e.scene, e.selected);
    assert_eq!(&renderer.stats()[..2], &gpu[..2]);
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, original);
}

#[test]
fn actual_surface_pick_keeps_one_cad_face_and_four_domain_edge_ids() {
    use crate::pick_sources::{Mode, Part};
    let mut scene = Scene::demo();
    scene.clear();
    let source = Rc::new(crate::surface::example());
    let _ = source.guid();
    let before = source.to_proto();
    let id = scene.insert_surface(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    let camera = crate::camera_defaults_tests::top_fixture();
    let transform = camera.uniform();
    let gpu = renderer.stats();
    let finish_at = |renderer: &mut Renderer| {
        renderer
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        renderer
            .take_pick(transform)
            .unwrap()
            .unwrap()
            .and_then(|hit| renderer.resolve_pick(&scene, hit))
    };
    renderer.set_pick_mode(Mode::Face);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    assert_eq!(
        finish_at(&mut renderer).unwrap().part,
        Part::Face(0)
    );
    renderer.set_pick_mode(Mode::Edge);
    for (point, edge) in [
        ([-1., 0., 0.], 0),
        ([1., 0., 0.], 1),
        ([0., -1., 0.], 2),
        ([0., 1., 0.], 3),
    ] {
        let point =
            camera
                .view_projection()
                .transform_point(&session_rust::Point::new(
                    point[0], point[1], point[2],
                ));
        renderer
            .request_pick([point[0] as f32, point[1] as f32], transform)
            .unwrap();
        let target = finish_at(&mut renderer).unwrap();
        assert_eq!(target.owner, id);
        assert_eq!(target.part, Part::SurfaceEdge(edge));
    }
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(source.to_proto(), before);
}

#[test]
fn actual_cad_hole_has_no_face_pick_and_integer_faces_retain_original_identity() {
    use crate::pick_sources::{Mode, Part};
    let mut scene = Scene::demo();
    scene.clear();
    let mut original = session_rust::BRep::create_block_with_hole(1.6, 1.2, 0.24, 0.26);
    original.surfacecolor = session_rust::Color::new(0.35, 0.65, 0.85, 1.);
    let source = Rc::new(original);
    let _ = source.guid();
    let before = source.to_proto();
    let id = scene.insert_cad(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    let camera = crate::camera_defaults_tests::top_fixture();
    let transform = camera.uniform();
    let gpu = renderer.stats();
    let finish_at = |renderer: &mut Renderer| {
        renderer
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        renderer
            .take_pick(transform)
            .unwrap()
            .unwrap()
            .and_then(|hit| renderer.resolve_pick(&scene, hit))
    };
    renderer.set_pick_mode(Mode::Face);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    assert!(
        finish_at(&mut renderer).is_none(),
        "The actual shaded hole stays empty"
    );
    let p = camera
        .view_projection()
        .transform_point(&session_rust::Point::new(0.5, 0., 0.12));
    renderer
        .request_pick([p[0] as f32, p[1] as f32], transform)
        .unwrap();
    let target = finish_at(&mut renderer).unwrap();
    assert_eq!(target.owner, id);
    assert_eq!(
        target.part,
        Part::Face(6),
        "The visible top cap retains its original CAD identity"
    );
    renderer.set_pick_mode(Mode::Edge);
    let mut seen = 0;
    for border in &scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .edges
    {
        let points = &border.points;
        if !points
            .iter()
            .all(|p| (p[2] - 0.12).abs() < 1e-10)
        {
            continue;
        }
        let middle = points
            .windows(2)
            .nth((points.len() - 1) / 2)
            .unwrap();
        let p = std::array::from_fn::<_, 3, _>(|i| (middle[0][i] + middle[1][i]) * 0.5);
        let p = camera
            .view_projection()
            .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
        renderer
            .request_pick([p[0] as f32, p[1] as f32], transform)
            .unwrap();
        assert_eq!(
            finish_at(&mut renderer)
                .unwrap_or_else(|| panic!(
                    "Missing original CAD edge {} at {:?}",
                    border.edge, p
                ))
                .part,
            Part::BRepEdge(border.edge)
        );
        seen += 1;
    }
    assert_eq!(
        seen, 5,
        "All four outer edges and the original hole rim resolve"
    );
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(source.to_proto(), before);
}

#[test]
fn actual_screen_text_overlay_does_not_create_scene_pick_owners_or_reupload_geometry() {
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert(triangle(0.2)).unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let gpu = renderer.stats();
    renderer.text_demo().unwrap();
    let with_text = colour_pixels(&mut renderer);
    assert_ne!(with_text, before);
    assert_eq!(renderer.text_error(), None);
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(scene.objects().len(), 1);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    renderer
        .request_pick([-0.85, 0.8], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    assert_eq!(renderer.text_usage()[0], 1);
    renderer.set_text(Vec::new()).unwrap();
    assert_eq!(renderer.text_usage(), [0; 6]);
    assert_eq!(colour_pixels(&mut renderer), before);
}

#[test]
fn actual_owned_text_plane_respects_depth_integer_picks_selection_cache_and_close() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = scene.insert(triangle(0.2)).unwrap();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let gpu = renderer.stats();
    let mut source = example(TextPlacement::WorldPlane {
        world: [-0.1, 0.05, 0.6],
        right: [1., 0., 0.],
        up: [0., 1., 0.],
        world_height: 0.08,
    });
    source.label.text = "Hi".into();
    let source = Rc::new(source);
    let original = serde_json::to_string(&*source).unwrap();
    let weak = Rc::downgrade(&source);
    let id = scene
        .insert_text(Prepared::new(source.clone()).unwrap())
        .unwrap();
    drop(source);
    renderer.set_scene(&scene, None);
    assert_eq!(
        colour_pixels(&mut renderer),
        before,
        "Actual fixed text behind the opaque solid is hidden"
    );
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        mesh.value()
    );
    scene
        .place_text(
            id,
            session_rust::Xform::translation(0., 0., -0.45),
        )
        .unwrap();
    renderer.set_scene(&scene, None);
    let front = colour_pixels(&mut renderer);
    assert_ne!(front, before);
    assert_eq!(renderer.text_error(), None);
    assert!(
        front
            .chunks_exact(4)
            .filter(|p| p[0] < 10 && p[1] < 10 && p[2] < 10)
            .count()
            > 20
    );
    let raster = renderer.text_scene_usage()[2];
    assert_eq!(
        raster, 1,
        "Placement reuses already rasterized original words"
    );
    assert_eq!(renderer.text_usage()[0], 1);
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap(),
        [id.value(), 0]
    );
    renderer
        .select_objects(&scene, &[id])
        .unwrap();
    let selected = colour_pixels(&mut renderer);
    assert!(
        selected
            .chunks_exact(4)
            .filter(|p| p[0] > 200 && p[1] > 200 && p[2] < 40)
            .count()
            > 20
    );
    assert_eq!(renderer.text_scene_usage()[2], raster);
    assert_eq!(renderer.text_usage()[0], 1);
    assert_eq!(&renderer.stats()[..2], &gpu[..2]);
    assert_eq!(
        serde_json::to_string(&*scene.texts()[0].prepared.source).unwrap(),
        original
    );
    Rc::make_mut(&mut scene.view).lock(&[id], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.06, -0.008], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "The locked plate remains an occluder without a selectable id"
    );
    Rc::make_mut(&mut scene.view).hide(&[id], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        mesh.value()
    );
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(renderer.text_scene_usage(), [0; 4]);
    assert_eq!(renderer.text_usage(), [0; 6]);
}

#[test]
fn actual_anchor_nameplate_and_billboard_follow_the_current_depth_and_plate_rules() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    for (placement, overlay) in [
        (
            TextPlacement::Anchor {
                world: [0., 0., 0.6],
                offset: [0.; 2],
            },
            false,
        ),
        (
            TextPlacement::Nameplate {
                world: [0., 0., 0.6],
                padding: [17., 4.],
                rounded: true,
            },
            true,
        ),
        (
            TextPlacement::WorldBillboard {
                world: [0., 0., 0.6],
                world_height: 0.08,
            },
            false,
        ),
    ] {
        let mut scene = Scene::demo();
        scene.clear();
        let mesh = scene.insert(triangle(0.2)).unwrap();
        let mut renderer = renderer(&scene);
        let before = colour_pixels(&mut renderer);
        let mut source = example(placement);
        source.label.text = "Hi".into();
        let id = scene
            .insert_text(Prepared::new(Rc::new(source)).unwrap())
            .unwrap();
        renderer.set_scene(&scene, None);
        let after = colour_pixels(&mut renderer);
        assert_eq!(
            after != before,
            overlay,
            "Only the nameplate overlays its own solid"
        );
        renderer
            .request_pick([0., 0.], identity())
            .unwrap();
        assert_eq!(
            finish(&mut renderer).unwrap()[0],
            if overlay {
                id.value()
            } else {
                mesh.value()
            }
        );
        assert_eq!(renderer.text_error(), None);
        let fonts = vec![
            crate::text::FONT_BYTES.to_vec(),
            crate::text::FALLBACK_BYTES.to_vec(),
            crate::text::SYMBOL_BYTES.to_vec(),
        ];
        assert!(
            renderer
                .replace_text_fonts(vec![vec![0; 8]])
                .is_err()
        );
        assert_eq!(colour_pixels(&mut renderer), after);
        renderer.replace_text_fonts(fonts).unwrap();
        assert_eq!(colour_pixels(&mut renderer), after);
        assert_eq!(renderer.text_usage()[5], 1);
    }
}

#[test]
fn actual_world_text_clip_uses_canvas_coordinates_in_the_small_integer_pick_window() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    let mut scene = Scene::demo();
    scene.clear();
    let mut source = example(TextPlacement::WorldPlane {
        world: [-0.2, 0.1, 0.3],
        right: [1., 0., 0.],
        up: [0., 1., 0.],
        world_height: 0.2,
    });
    source.label.text = "Hi".into();
    source.label.clip = Some([140., 100., 160., 140.]);
    let id = scene
        .insert_text(Prepared::new(Rc::new(source)).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    assert_eq!(renderer.text_error(), None);
    let mut clean = Scene::demo();
    clean.clear();
    let mut no_text = super::pick_gpu_tests::renderer(&clean);
    let baseline = colour_pixels(&mut no_text);
    let mut changed = 0;
    for (i, (p, b)) in before
        .chunks_exact(4)
        .zip(baseline.chunks_exact(4))
        .enumerate()
    {
        let (x, y) = (i % 320, i / 320);
        if x < 140 || x >= 160 || y < 100 || y >= 140 {
            assert_eq!(
                p, b,
                "Clipped text does not affect pixels outside the CSS clip"
            );
        } else {
            changed += usize::from(p != b);
        }
    }
    assert!(changed > 50);
    renderer
        .request_pick([-0.0625, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    renderer
        .request_pick([0.125, 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_selected_silhouette_has_one_union_no_internal_diagonal_and_keeps_source_and_picks()
 {
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = |x: f64| {
        let mut source = session_rust::Mesh::create_box(0.6, 0.8, 0.1);
        source.set_objectcolor(session_rust::Color::new(0.5, 0.5, 0.5, 1.));
        let mut prepared = PreparedMesh::new(source).unwrap();
        prepared.model = session_rust::Xform::translation(x, 0., 0.5);
        prepared
    };
    let a = scene.insert(mesh(-0.3)).unwrap();
    let b = scene.insert(mesh(0.3)).unwrap();
    let sources = scene
        .objects()
        .iter()
        .map(|row| row.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let mut renderer = renderer(&scene);
    let geometry = renderer.stats();
    renderer
        .select_objects(&scene, &[a, b])
        .unwrap();
    let first = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    assert_eq!(alpha.len(), 320 * 240);
    assert!(alpha.iter().filter(|v| **v > 0).count() > 300);
    for y in 90..150 {
        for x in 90..230 {
            assert_eq!(
                alpha[y * 320 + x],
                0,
                "The combined coverage has no outline along the shared object join or tessellation diagonals"
            );
        }
    }
    let stats = renderer.outline_usage();
    assert_eq!(colour_pixels(&mut renderer), first);
    assert_eq!(renderer.outline_alpha(), alpha);
    assert_eq!(renderer.outline_usage()[1], stats[1]);
    assert!(renderer.outline_usage()[2] > stats[2]);
    assert_eq!(&renderer.stats()[..2], &geometry[..2]);
    assert_eq!(
        scene
            .objects()
            .iter()
            .map(|row| row.geometry().unwrap().to_proto())
            .collect::<Vec<_>>(),
        sources
    );
    renderer
        .request_pick([-0.3, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], a.value());
    renderer
        .request_pick([0.3, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], b.value());
    renderer.select_objects(&scene, &[]).unwrap();
    colour_pixels(&mut renderer);
    assert!(renderer.outline_alpha().is_empty());
    assert_eq!(renderer.outline_usage()[0], 0);
}

#[test]
fn actual_surface_outline_culls_occluded_selection_and_selected_edge_ink_cannot_narrow_its_alpha()
 {
    use crate::pick_sources::{Part, Target};
    let mut scene = Scene::demo();
    scene.clear();
    let back = scene.insert(triangle(0.6)).unwrap();
    let front = scene.insert(triangle(0.2)).unwrap();
    let mut renderer = renderer(&scene);
    renderer
        .select_objects(&scene, &[back])
        .unwrap();
    colour_pixels(&mut renderer);
    assert!(
        renderer
            .outline_alpha()
            .iter()
            .all(|v| *v == 0),
        "A selected surface wholly behind the current opaque depth adds no silhouette"
    );
    renderer
        .select_objects(&scene, &[front])
        .unwrap();
    let plain = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    assert!(alpha.iter().any(|v| *v > 128));
    let edge = scene
        .objects()
        .iter()
        .find(|row| row.id == front)
        .unwrap()
        .mesh
        .parts
        .edges[0];
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner: front,
                part: Part::MeshEdge(edge.0, edge.1),
            }),
        )
        .unwrap();
    let inked = colour_pixels(&mut renderer);
    assert_eq!(
        renderer.outline_alpha(),
        alpha,
        "Source edge highlighting cannot change the actual outline width"
    );
    let mut opaque = 0;
    for ((a, b), coverage) in plain
        .chunks_exact(4)
        .zip(inked.chunks_exact(4))
        .zip(&alpha)
    {
        if *coverage >= 250 && a[..3].iter().all(|c| *c < 8) {
            opaque += 1;
            assert!(
                b[..3].iter().all(|c| *c < 12),
                "Yellow source edge ink cannot paint over black outline pixels"
            );
        }
    }
    assert!(opaque > 100);
    scene.clear();
    renderer.set_scene(&scene, None);
    colour_pixels(&mut renderer);
    assert_eq!(renderer.outline_usage()[0], 0);
}

#[test]
fn actual_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths = Vec::new();
    for curved in [false, true] {
        let mut scene = Scene::demo();
        scene.clear();
        let id = if curved {
            scene
                .insert_cad(Rc::new(session_rust::BRep::create_cylinder(
                    0.4, 0.8,
                )))
                .unwrap()
        } else {
            let mut prepared =
                PreparedMesh::new(session_rust::Mesh::create_box(0.8, 0.8, 0.2))
                    .unwrap();
            prepared.model = session_rust::Xform::translation(0., 0., 0.5);
            scene.insert(prepared).unwrap()
        };
        let mut renderer = renderer(&scene);
        renderer
            .select_objects(&scene, &[id])
            .unwrap();
        let geometry = renderer.stats();
        let mut css = Vec::new();
        for density in [1., 2.] {
            renderer.set_density(density);
            colour_pixels(&mut renderer);
            let alpha = renderer.outline_alpha();
            assert!(alpha.iter().any(|v| *v == 255));
            let row = &alpha[120 * 320..121 * 320];
            let width = row[160..]
                .iter()
                .map(|v| *v as f64 / 255.)
                .sum::<f64>()
                / density as f64;
            assert!(
                (0.7..1.7).contains(&width),
                "Actual outer silhouette CSS coverage remains bounded: {width}"
            );
            css.push(width);
            assert_eq!(&renderer.stats()[..2], &geometry[..2]);
        }
        assert!(
            (css[0] - css[1]).abs() < 0.5,
            "DPI keeps the outline in CSS units: {css:?}"
        );
        widths.push(css);
    }
    for i in 0..2 {
        assert!(
            (widths[0][i] - widths[1][i]).abs() < 0.25,
            "A smooth cylinder and flat box use one coverage width: {widths:?}"
        );
    }
}

#[test]
fn actual_selected_bore_keeps_its_open_center_and_black_inner_contour() {
    let source = Rc::new(session_rust::BRep::create_block_with_hole(
        1.2, 1.2, 0.4, 0.25,
    ));
    let _ = source.guid();
    let original = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert_cad(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    renderer
        .select_objects(&scene, &[id])
        .unwrap();
    let pixels = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    for y in 112..128 {
        for x in 152..168 {
            assert_eq!(
                alpha[y * 320 + x],
                0,
                "The bore center stays open"
            );
        }
    }
    let mut inner = 0;
    for y in 80..160 {
        for x in 110..210 {
            let i = y * 320 + x;
            if alpha[i] >= 250 {
                inner += 1;
                assert!(
                    pixels[i * 4..i * 4 + 3]
                        .iter()
                        .all(|c| *c < 12),
                    "The actual inner silhouette is black"
                );
            }
        }
    }
    assert!(
        inner > 100,
        "The real bore boundary has an inner outline: {inner}"
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    renderer
        .request_pick([0.4, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    assert_eq!(
        source.to_proto(),
        original,
        "Coverage cannot change original topology"
    );
}

#[test]
fn actual_cloud_pixels_and_whole_ids_keep_cached_source_uploads_density_and_locked_occlusion()
 {
    let mut source = session_rust::PointCloud::from_coords(
        vec![0., 0., 0.2],
        vec![255, 0, 0, 255],
        vec![],
    );
    source.point_size = 16.;
    let source = Rc::new(source);
    let _ = source.guid();
    let before = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = scene.insert(triangle(0.6)).unwrap();
    let cloud = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    for density in [1., 2.] {
        renderer.set_density(density);
        let first = colour_pixels(&mut renderer);
        let center = &first[(120 * 320 + 160) * 4..(120 * 320 + 160) * 4 + 4];
        assert!(
            center[0] > 240 && center[1] < 10 && center[2] < 10,
            "Original cloud color covers the solid behind it"
        );
        let usage = renderer.cloud_usage();
        assert_eq!(usage[0], 1);
        assert_eq!(usage[2], 1);
        assert!(usage[1] > 320 * 240 * 16);
        assert_eq!(colour_pixels(&mut renderer), first);
        assert_eq!(renderer.cloud_usage()[3], usage[3]);
        assert!(renderer.cloud_usage()[4] > usage[4]);
        renderer
            .request_pick([0., 0.], identity())
            .unwrap();
        assert_eq!(
            finish(&mut renderer),
            Some([cloud.value(), 0])
        );
    }
    renderer.set_density(1.);
    Rc::make_mut(&mut scene.view).lock(&[cloud], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer),
        None,
        "The locked cloud writes zero while occluding the mesh"
    );
    Rc::make_mut(&mut scene.view).hide(&[cloud], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer),
        Some([mesh.value(), 0])
    );
    assert_eq!(source.to_proto(), before);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}
#[test]
fn actual_cloud_copy_and_normal_shading_preserve_one_source_and_release_all_derived_owners()
 {
    let source = Rc::new(session_rust::PointCloud::from_coords(
        vec![-0.3, 0., 0.2, 0.3, 0., 0.2],
        vec![0, 255, 0, 255, 0, 255, 0, 255],
        vec![0., 0., 1., 0., 0., 1.],
    ));
    let weak = Rc::downgrade(&source);
    let mut scene = Scene::demo();
    scene.clear();
    let cloud = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let pixels = colour_pixels(&mut renderer);
    assert!(
        pixels
            .chunks_exact(4)
            .any(|p| p[1] > 160 && p[0] < 10 && p[2] < 10)
    );
    assert_eq!(renderer.cloud_usage()[2], 1);
    let copy = scene.copy_row(cloud, false).unwrap();
    scene.translate(copy, [0., 0.3, 0.]).unwrap();
    renderer.set_scene(&scene, None);
    colour_pixels(&mut renderer);
    assert_eq!(
        renderer.cloud_usage()[2],
        1,
        "Copies share one original upload"
    );
    assert_eq!(renderer.cloud_usage()[0], 4);
    renderer
        .request_pick([-0.3, 0.3], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        copy.value()
    );
    drop(source);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
    assert!(weak.upgrade().is_none());
}

fn camera_pixels(renderer: &mut Renderer, camera: &crate::camera::Camera) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("selection pixels"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    renderer.cloud_camera(camera);
    renderer.draw_mode(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &camera.uniform(),
        false,
    );
    let bytes = renderer
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection readback"),
            size: 320 * 240 * 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut encoder = renderer
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &bytes,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280),
                rows_per_image: Some(240),
            },
        },
        texture.size(),
    );
    renderer.queue.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::channel();
    bytes
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = send.send(result);
        });
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    receive
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap()
        .unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec();
    bytes.unmap();
    result
}

#[test]
fn actual_large_cloud_changes_detail_with_the_view_and_reuses_its_original_upload() {
    let source = Rc::new(crate::cloud::large_example());
    assert_eq!(source.len(), 2_000_064);
    let mut scene = Scene::demo();
    scene.clear();
    scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.aspect = 320. / 240.;
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 100. / std::f64::consts::FRAC_PI_6.tan();
    let coarse = camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    let uploads = renderer.cloud_usage()[2];
    let prepared = renderer.cloud_usage()[3];
    assert_eq!(camera_pixels(&mut renderer, &camera), coarse);
    assert_eq!(renderer.cloud_usage()[3], prepared);
    camera.distance = 0.7 / std::f64::consts::FRAC_PI_6.tan();
    let fine = camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 2_000_064);
    assert_ne!(fine, coarse);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    assert_eq!(source.point_id(64), 64);
    assert_eq!(source.point_id(source.len() - 1), 2_000_063);
    camera.distance = 100. / std::f64::consts::FRAC_PI_6.tan();
    assert_eq!(camera_pixels(&mut renderer, &camera), coarse);
    assert_eq!(renderer.cloud_usage()[0], 64);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}

#[test]
fn actual_cloud_control_pick_preserves_full_u32_source_ids_and_local_highlight() {
    use crate::pick_sources::{Mode, Part};
    let mut proto = session_rust::PointCloud::from_coords(
        vec![-0.4, 0., 0.2, 0.4, 0., 0.2],
        vec![],
        vec![],
    )
    .to_proto();
    proto.point_size = 16.;
    proto.point_ids = vec![u32::MAX, 16_777_217];
    let source = Rc::new(session_rust::PointCloud::from_proto(proto));
    let _ = source.guid();
    let original = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    scene.cloud_controls(Some(owner));
    let mut renderer = renderer(&scene);
    renderer.set_pick_mode(Mode::Control);
    let uploaded = renderer.cloud_usage()[2];
    let geometry = renderer.stats();
    for density in [1., 2.] {
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320. * density) as u32,
            height: (240. * density) as u32,
        });
        for (x, id) in [(-0.4, u32::MAX), (0.4, 16_777_217)] {
            renderer
                .request_pick([x, 0.], identity())
                .unwrap();
            let pixel = finish(&mut renderer).unwrap();
            let target = renderer.resolve_pick(&scene, pixel).unwrap();
            assert_eq!(target.part, Part::CloudPoint(id));
            assert_eq!(target.owner, owner);
            renderer
                .set_highlight(&scene, Some(target))
                .unwrap();
            assert!(renderer.highlight_bytes() > 0);
            assert_eq!(renderer.stats(), geometry);
        }
    }
    assert_eq!(renderer.cloud_usage()[2], uploaded);
    assert_eq!(source.to_proto(), original);
    Rc::make_mut(&mut scene.view).lock(&[owner], true);
    renderer.set_density(1.);
    renderer.resize(Viewport {
        width: 320,
        height: 240,
    });
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.4, 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    scene.clear();
    renderer.set_scene(&scene, None);
    renderer.set_highlight(&scene, None).unwrap();
    assert_eq!(renderer.cloud_usage(), [0; 5]);
    assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_resident_cloud_controls_query_original_points_outside_the_coarse_display_subset()
 {
    use crate::pick_sources::{Mode, Part};
    let mut proto = crate::cloud::large_example().to_proto();
    for xyz in proto.coords[..64 * 3].chunks_exact_mut(3) {
        xyz[2] = 0.;
    }
    let source = Rc::new(session_rust::PointCloud::from_proto(proto));
    let mut scene = Scene::demo();
    scene.clear();
    let owner = scene
        .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.aspect = 320. / 240.;
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 100. / std::f64::consts::FRAC_PI_6.tan();
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    let uploads = renderer.cloud_usage()[2];
    scene.cloud_controls(Some(owner));
    renderer.set_scene(&scene, None);
    renderer.set_pick_mode(Mode::Control);
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 2_000_064);
    let transform = camera.view_projection().to_f32();
    let p = camera
        .view_projection()
        .transform_point(&session_rust::Point::new(0.25, 0.25, 0.2));
    renderer
        .request_pick([p[0] as f32, p[1] as f32], transform)
        .unwrap();
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let pixel = renderer
        .take_pick(transform)
        .unwrap()
        .unwrap()
        .unwrap();
    let target = renderer.resolve_pick(&scene, pixel).unwrap();
    let Part::CloudPoint(id) = target.part else {
        panic!("Original cloud point identity");
    };
    assert!(id >= 64);
    assert!(target.valid(&scene));
    assert_eq!(renderer.cloud_usage()[2], uploads);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    renderer.cancel_pick();
    assert!(renderer.take_pick(transform).is_none());
    scene.cloud_controls(None);
    renderer.set_scene(&scene, None);
    renderer.set_pick_mode(Mode::Object);
    camera_pixels(&mut renderer, &camera);
    assert_eq!(renderer.cloud_usage()[0], 64);
    assert_eq!(renderer.cloud_usage()[2], uploads);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
}

#[test]
fn actual_streamed_pages_pick_original_ids_share_uploads_and_release_after_eviction() {
    use crate::pick_sources::{Mode, Part};
    let prepared = crate::range_example::prepared();
    let weak = Rc::downgrade(&prepared.source);
    let mut scene = Scene::empty();
    let owner = scene
        .insert_stream(
            prepared.clone(),
            session_rust::Xform::identity(),
            12.,
        )
        .unwrap();
    scene.cloud_controls(Some(owner));
    let mut renderer = renderer(&scene);
    renderer.set_pick_mode(Mode::Control);
    for (cursor, id) in [
        ([-0.5, 0.5], u32::MAX),
        ([0.5, 0.5], 16_777_217),
        ([-0.5, -0.5], 10),
    ] {
        renderer
            .request_pick(cursor, identity())
            .unwrap();
        let hit = finish(&mut renderer).expect("Actual streamed point coverage");
        let target = renderer.resolve_pick(&scene, hit).unwrap();
        assert_eq!(target.owner, owner);
        assert_eq!(target.part, Part::CloudPoint(id));
    }
    let pixels = colour_pixels(&mut renderer);
    assert!(
        pixels
            .chunks_exact(4)
            .any(|p| p[2] > 160 && p[0] < 10)
    );
    assert!(
        pixels
            .chunks_exact(4)
            .any(|p| p[0] > 160 && p[2] < 10)
    );
    let uploads = renderer.cloud_usage()[2];
    let copy = scene.copy_row(owner, false).unwrap();
    scene.translate(copy, [0.2, 0., 0.]).unwrap();
    renderer.set_scene(&scene, None);
    colour_pixels(&mut renderer);
    assert_eq!(
        renderer.cloud_usage()[2],
        uploads,
        "Placed owners share original page uploads"
    );
    Rc::make_mut(&mut scene.view).lock(&[owner, copy], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.5, 0.5], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).lock(&[owner, copy], false);
    let held = Rc::downgrade(&prepared.page(2).unwrap());
    prepared.pages.borrow_mut().clear();
    assert!(
        held.upgrade().is_some(),
        "GPU page owner retains precise pick identities"
    );
    renderer.set_scene(&scene, None);
    assert!(held.upgrade().is_none());
    renderer
        .request_pick([-0.5, 0.5], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "Evicted detailed points are absent"
    );
    renderer
        .request_pick([-0.5, -0.5], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer
            .resolve_pick(&scene, hit)
            .unwrap()
            .part,
        Part::CloudPoint(10)
    );
    scene.clear();
    renderer.set_scene(&scene, None);
    drop(prepared);
    assert_eq!(renderer.cloud_usage(), [0; 5]);
    assert!(weak.upgrade().is_none());
}

#[test]
fn actual_definition_instances_batch_one_geometry_keep_integer_picks_and_release_records()
 {
    let mut original = crate::instance_specimen::message();
    original.objects.as_mut().unwrap().instances[0].xform = None;
    original.objects.as_mut().unwrap().instances[1].flags = 0;
    original.xforms = vec![
        session_rust::proto::XformEntry {
            guid: "first".into(),
            xform: Some(session_rust::Xform::translation(-0.5, 0., 0.3).to_proto()),
        },
        session_rust::proto::XformEntry {
            guid: "second".into(),
            xform: Some(session_rust::Xform::translation(0.5, 0., 0.3).to_proto()),
        },
    ];
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    crate::scene_document::import(
        &mut scene,
        &prost::Message::encode_to_vec(&original),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    let a = scene.objects()[0].id;
    let b = scene.objects()[1].id;
    let weak = Rc::downgrade(scene.objects()[0].geometry().unwrap());
    let mut renderer = renderer(&scene);
    assert_eq!(
        &renderer.instance_usage()[..3],
        &[1, 2, 16760]
    );
    let source_uploads = renderer.stats()[0];
    let pixels = colour_pixels(&mut renderer);
    assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
    for (cursor, id) in [([-0.5, -0.1], a), ([0.5, -0.1], b)] {
        renderer
            .request_pick(cursor, identity())
            .unwrap();
        assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    }
    let mut coloured = original.clone();
    let first = &mut coloured.objects.as_mut().unwrap().instances[0];
    first.flags |= session_rust::InstanceRef::FLAG_COLOR;
    first.color = Some(session_rust::proto::Color {
        r: 0.9,
        g: 0.08,
        b: 0.05,
        a: 0.5,
        ..Default::default()
    });
    let mut colour_scene = Scene::empty();
    crate::scene_document::import(
        &mut colour_scene,
        &prost::Message::encode_to_vec(&coloured),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    let mut colour_renderer = super::pick_gpu_tests::renderer(&colour_scene);
    let colours = colour_pixels(&mut colour_renderer);
    assert!(colours.chunks_exact(4).filter(|p|p[0]>p[1].saturating_add(50)&&p[0]>p[2].saturating_add(50)).count()>500);
    assert!(
        colours
            .chunks_exact(4)
            .filter(|p| p[2] > p[0].saturating_add(20))
            .count()
            > 500
    );
    let writes = renderer.instance_usage()[3];
    colour_pixels(&mut renderer);
    assert_eq!(renderer.instance_usage()[3], writes);
    scene.translate(a, [0., 0.2, 0.]).unwrap();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.stats()[0], source_uploads);
    assert!(renderer.instance_usage()[3] > writes);
    renderer
        .select_objects(&scene, &[a])
        .unwrap();
    let selected = colour_pixels(&mut renderer);
    assert!(
        selected
            .chunks_exact(4)
            .any(|p| p[0] > 140 && p[1] > 90 && p[2] < 40)
    );
    Rc::make_mut(&mut scene.view).hide(&[b], true);
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.instance_usage()[1], 1);
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(&renderer.instance_usage()[..3], &[0, 0, 0]);
    assert!(weak.upgrade().is_none());
}

#[test]
fn actual_congruent_cad_replay_matches_independent_faces_ink_and_owner_picks() {
    let first = Rc::new(
        session_rust::BRep::create_block_with_hole(0.7, 0.6, 0.2, 0.12).transformed(
            &session_rust::Xform::translation(-0.35, -0.3, 0.3),
        ),
    );
    let mut cache = crate::cad_shape_cache::Cache::default();
    cache.prepare(first.clone()).unwrap();
    let motion = &session_rust::Xform::translation(0.1, -0.1, 0.)
        * &session_rust::Xform::rotation_z(90., true);
    let copy = Rc::new(first.transformed(&motion));
    let mut shared = Scene::empty();
    let id = shared
        .insert_prepared_cad(cache.prepare(copy.clone()).unwrap())
        .unwrap();
    assert_eq!(cache.replays, 1);
    let mut alone = Scene::empty();
    alone.insert_cad(copy.clone()).unwrap();
    let mut shared_renderer = renderer(&shared);
    let mut independent_renderer = renderer(&alone);
    let a = colour_pixels(&mut shared_renderer);
    let b = colour_pixels(&mut independent_renderer);
    let changed = a
        .chunks_exact(4)
        .zip(b.chunks_exact(4))
        .filter(|(a, b)| {
            a[..3]
                .iter()
                .zip(&b[..3])
                .any(|(a, b)| a.abs_diff(*b) > 3)
        })
        .count();
    assert!(
        changed < 100,
        "Congruent faces and source ink differed at {changed} pixels"
    );
    assert!(
        a.chunks_exact(4)
            .filter(|p| p[0] < 180 && p[1] < 180 && p[2] < 180)
            .count()
            > 500
    );
    let mut hits = 0;
    for point in [[-0.1, -0.05], [0.25, -0.05], [0.1, 0.15]] {
        shared_renderer
            .request_pick(point, identity())
            .unwrap();
        independent_renderer
            .request_pick(point, identity())
            .unwrap();
        let actual = finish(&mut shared_renderer);
        let independent = finish(&mut independent_renderer);
        assert_eq!(actual, independent);
        if let Some(actual) = actual {
            assert_eq!(actual[0], id.value());
            hits += 1;
        }
    }
    assert!(
        hits > 0,
        "Actual original CAD ownership must be picked"
    );
    let weak = Rc::downgrade(&copy);
    drop(copy);
    shared.clear();
    alone.clear();
    shared_renderer.set_scene(&shared, None);
    independent_renderer.set_scene(&alone, None);
    drop(cache);
    assert!(weak.upgrade().is_none());
}

#[test]
fn actual_nested_rotated_and_mirrored_definition_placements_keep_independent_integer_owners()
 {
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    crate::scene_document::import(
        &mut scene,
        &crate::nested_example::bytes(),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    let mut gpu = renderer(&scene);
    assert_eq!(&gpu.instance_usage()[..3], &[1, 2, 16760]);
    for row in scene.objects() {
        let point = row
            .model
            .transform_point(&session_rust::Point::new(
                (0.123456789012345 + 0.8) / 3.,
                0.8 / 3.,
                0.,
            ));
        gpu.request_pick(
            [point[0] as f32, point[1] as f32],
            identity(),
        )
        .unwrap();
        assert_eq!(finish(&mut gpu).unwrap()[0], row.id.value());
    }
    let id = scene.objects()[1].id;
    gpu.select_objects(&scene, &[id]).unwrap();
    let pixels = colour_pixels(&mut gpu);
    assert!(
        pixels
            .chunks_exact(4)
            .filter(|p| p[0] > 140 && p[1] > 90 && p[2] < 40)
            .count()
            > 500
    );
    let bytes = crate::scene_archive::save(&scene).unwrap();
    let mut restored = Scene::empty();
    crate::scene_document::open(&mut restored, &bytes).unwrap();
    for a in scene.objects() {
        let guid = scene.instances[&a.id].instance.guid();
        let b = restored
            .objects()
            .iter()
            .find(|row| restored.instances[&row.id].instance.guid() == guid)
            .unwrap();
        for (a, b) in a.model.m.iter().zip(&b.model.m) {
            assert!(
                (a - b).abs() < 1e-12,
                "Canonical placement for {guid}: {a} {b}"
            );
        }
    }
}

#[test]
fn actual_sheet_batches_share_source_uploads_keep_world_pens_full_entity_ids_and_locked_occlusion()
 {
    use crate::pick_sources::{Mode, Part, Target};
    let mut source = crate::sheet_example::source();
    source.entity_count = u32::MAX;
    source.source_ids = vec![u32::MAX - 1; 5];
    let mut scene = Scene::empty();
    scene.next_id = 16_777_217;
    let id = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(Rc::new(source)).unwrap(),
        ))
        .unwrap();
    let copy = scene.copy_row(id, false).unwrap();
    scene
        .place_sheet(
            copy,
            session_rust::Xform::translation(0., 0.1, 0.),
        )
        .unwrap();
    let weak = Rc::downgrade(&scene.sheets()[0].prepared);
    let mut renderer = renderer(&scene);
    assert_eq!(renderer.sheet_usage()[..2], [2, 1]);
    let uploads = renderer.sheet_usage()[3];
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.sheet_usage()[3], uploads);
    renderer.set_pick_mode(Mode::Edge);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&scene, hit),
        Some(Target {
            owner: id,
            part: Part::SheetEntity(u32::MAX - 1)
        })
    );
    renderer
        .request_pick([0., -0.4], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(
        renderer.resolve_pick(&scene, hit),
        Some(Target {
            owner: copy,
            part: Part::SheetEntity(u32::MAX - 1)
        })
    );
    assert!(hit[1] < u32::MAX - 1);
    let pixels = colour_pixels(&mut renderer);
    let black = (0..240)
        .filter(|y| {
            let p = &pixels[(*y * 320 + 160) * 4..][..4];
            p[0] < 30 && p[1] < 30 && p[2] < 30
        })
        .count();
    assert!(
        black >= 8 && black < 20,
        "Physical .025mm pens draw separate three-pixel boundaries: {black}"
    );
    scene
        .place_sheet(
            copy,
            session_rust::Xform::translation(0., 0.2, 0.),
        )
        .unwrap();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.sheet_usage()[3], uploads);
    let front = scene.insert(triangle(0.1)).unwrap();
    renderer.set_scene(&scene, None);
    renderer.set_pick_mode(Mode::Object);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        front.value()
    );
    Rc::make_mut(&mut scene.view).lock(&[front], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).hide(&[front], true);
    Rc::make_mut(&mut scene.view).lock(&[id], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0., -0.5], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    scene.clear();
    renderer.set_scene(&scene, None);
    assert_eq!(renderer.sheet_usage()[..2], [0, 0]);
    assert!(weak.upgrade().is_none());
}

#[test]
fn actual_evicted_sheet_pages_remain_charged_until_renderer_refresh_cancels_old_picks()
{
    let budget = crate::page_cache::Budget::new(256);
    let source = Rc::new(crate::sheet_range::Source {
        guid: crate::sheet_example::source().guid,
        name: "paged drawing".into(),
        url: "plan.pb".into(),
        fields: crate::stream::SheetFields {
            count: 2,
            entities: u32::MAX,
            ..Default::default()
        },
    });
    let stream = crate::sheet_stream::Stream::new(source, budget.clone());
    let raw = [-0.5f64, 0., 0.3, 0.5, 0., 0.3]
        .into_iter()
        .flat_map(f64::to_le_bytes)
        .collect::<Vec<_>>();
    let page = crate::sheet_page::Page::decode(
        0..1,
        2,
        u32::MAX,
        &raw,
        None,
        None,
        Some(&(u32::MAX - 1).to_le_bytes()),
    )
    .unwrap();
    let held = stream
        .adopt(page, budget.reserve(192).unwrap())
        .unwrap();
    let weak = Rc::downgrade(&held);
    let mut scene = Scene::empty();
    let id = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::streamed(stream.clone()),
        ))
        .unwrap();
    let mut renderer = renderer(&scene);
    drop(held);
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge);
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    let hit = finish(&mut renderer).unwrap();
    assert_eq!(hit[0], id.value());
    assert!(renderer.resolve_pick(&scene, hit).is_some());
    assert_eq!(budget.used(), 84);
    stream.pages.borrow_mut().clear();
    assert_eq!(budget.used(), 84);
    assert!(budget.reserve(192).is_err());
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    renderer.set_scene(&scene, None);
    assert!(renderer.take_pick(identity()).is_none());
    assert_eq!(budget.used(), 0);
    assert!(weak.upgrade().is_none());
    assert!(renderer.resolve_pick(&scene, hit).is_none());
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_sheet_pen_projection_uses_camera_lens_independently_of_panel_frame_shift() {
    let source = session_rust::proto::Sheet {
        guid: crate::sheet_example::source().guid,
        coords: vec![-0.6, 0., 0.3, 0.6, 0., 0.3],
        widths: vec![0.35],
        segment_count: 1,
        ..Default::default()
    };
    let mut scene = Scene::empty();
    scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(Rc::new(source)).unwrap(),
        ))
        .unwrap();
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    let pen_rows = |pixels: Vec<u8>| {
        pixels
            .chunks_exact(320 * 4)
            .filter(|row| row[160 * 4..][..3].iter().all(|v| *v < 30))
            .count()
    };
    renderer.sheet_camera(&camera);
    let first = pen_rows(colour_pixels_at(
        &mut renderer,
        camera.uniform(),
    ));
    assert!(first > 20);
    camera.frame = [0., 0., 1., 0.2];
    renderer.sheet_camera(&camera);
    let shifted = pen_rows(colour_pixels_at(
        &mut renderer,
        camera.uniform(),
    ));
    assert_eq!(
        shifted, first,
        "A panel shifts the frame without changing the original world pen"
    );
}

#[test]
fn actual_mesh_control_pixel_keeps_sparse_vertex_key_at_both_densities() {
    let mut mesh = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.5, -0.5, 0.25]),
        (20, [0.5, -0.5, 0.25]),
        (30, [0., 0.5, 0.25]),
    ] {
        mesh.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    mesh.add_face(vec![10, 20, 30], Some(37));
    let mut scene = Scene::empty();
    let owner = scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    scene.show_controls(true);
    let source = crate::source_edit::original(&scene, owner).unwrap();
    let model = session_rust::Xform::translation(0.125, 0.25, 0.);
    scene.place(owner, model).unwrap();
    for density in [1., 2.] {
        let mut renderer = renderer(&scene);
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320. * density) as u32,
            height: (240. * density) as u32,
        });
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);
        renderer
            .request_pick([-0.375, -0.25], identity())
            .unwrap();
        let pixel = finish(&mut renderer).expect("Actual placed control pixel");
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(crate::pick_sources::Target {
                owner,
                part: crate::pick_sources::Part::SourceControl(
                    crate::source_controls::ControlId::Vertex(10)
                )
            })
        );
        scene.show_controls(false);
        renderer.set_scene(&scene, None);
        renderer
            .request_pick([-0.375, -0.25], identity())
            .unwrap();
        assert!(finish(&mut renderer).is_none());
        scene.show_controls(true);
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}

#[test]
fn actual_controls_are_blue_seven_css_pixels_and_selected_yellow_at_both_densities() {
    let mut mesh = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.5, -0.5, 0.25]),
        (20, [0.1, 0.1, 0.25]),
        (30, [0.8, 0.1, 0.25]),
        (40, [0.1, 0.8, 0.25]),
    ] {
        mesh.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
    }
    mesh.add_face(vec![20, 30, 40], Some(37));
    let mut scene = Scene::empty();
    let owner = scene
        .insert(PreparedMesh::new(mesh).unwrap())
        .unwrap();
    scene.show_controls(true);
    assert!(
        scene
            .control_markers()
            .iter()
            .all(|m| m.display.diameter == 7.
                && m.display.colour == [0.15, 0.35, 0.9, 1.])
    );
    let source = crate::source_edit::original(&scene, owner).unwrap();
    let target = crate::pick_sources::Target {
        owner,
        part: crate::pick_sources::Part::SourceControl(
            crate::source_controls::ControlId::Vertex(10),
        ),
    };
    for density in [1., 2.] {
        let mut renderer = renderer(&scene);
        renderer.set_density(density);
        let pixels = colour_pixels(&mut renderer);
        let center = (180 * 320 + 80) * 4;
        for (actual, expected) in pixels[center..center + 4]
            .iter()
            .zip([38_i16, 89, 230, 255])
        {
            assert!(
                (*actual as i16 - expected).abs() <= 2,
                "Actual blue control pixel {:?}",
                &pixels[center..center + 4]
            );
        }
        let coverage: f32 = (60..100)
            .map(|x| (255. - pixels[(180 * 320 + x) * 4] as f32) / 217.)
            .sum();
        assert!(
            (coverage / density - 7.).abs() < 0.6,
            "Seven CSS pixel row coverage at density {density}: {coverage}"
        );
        renderer
            .set_highlight(&scene, Some(target))
            .unwrap();
        let selected = colour_pixels(&mut renderer);
        assert_eq!(
            &selected[center..center + 4],
            &[255, 255, 0, 255]
        );
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);
        renderer
            .request_pick([-0.5, -0.5], identity())
            .unwrap();
        let pixel = finish(&mut renderer).unwrap();
        assert_eq!(
            renderer.resolve_pick(&scene, pixel),
            Some(target)
        );
    }
    assert!(crate::source_edit::same(
        &source,
        &crate::source_edit::original(&scene, owner).unwrap()
    ));
}

#[test]
fn actual_control_net_pixel_addresses_parent_without_a_fake_edge_or_control() {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor
```

### 16. `src/snap_tests.rs`

Keep the original Top pose for exact grid-rounding and unrounded typed-coordinate assertions.

**Locate this existing block:**

```rust
fn snapping_settings_preserve_a_draft_and_typed_coordinates_bypass_grid_rounding() {
    use crate::editor::{Action, Editor};
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn snapping_settings_preserve_a_draft_and_typed_coordinates_bypass_grid_rounding() {
    use crate::editor::{Action, Editor};
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor.apply(Action::Close).unwrap();
    editor
```

### 17. `src/widget_tests.rs`

Keep the original three-unit isometric widget fixture, exact hover pixels, physical tile reuse and no-pick-owner checks.

**Locate this existing block:**

```rust
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera::Camera::default();
    camera.isometric();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut renderer = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.isometric();
```

## <a id="checkpoint-96zzzzzzb-omit-unused-background-pick-pipelines"></a>2. Omit unused background pick pipelines

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_gpu.rs, src/grid…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Lane role</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">optional base integer sha…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged original picking</text>
</svg>
</div>

**What this code does:** Create an integer pipeline only for lanes that can own a pick.

### 1. `src/stroke_gpu.rs`

Give background lanes no base integer shader or pipeline while keeping the original lane builder and pick path.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
```

Replace it with:

```rust
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
```

**Locate this block:**

```rust
            multiview_mask: None,
            cache: None,
        });
        let pick_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ink IDs"),
                layout: Some(&pick_layout),
```

Replace it with:

```rust
            multiview_mask: None,
            cache: None,
        });
        let pick_pipeline = pick_shader.as_ref().map(|pick_shader| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ink IDs"),
                layout: Some(&pick_layout),
```

**Locate this block:**

```rust
            32 => include_str!("marker_pick.wgsl"),
            _ => unreachable!(),
        };
        let pick_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(
                format!(
                    "{source}\n{}\n{pick_source}",
                    include_str!("pick_crop.wgsl")
                )
                .into(),
            ),
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
```

Replace it with:

```rust
            32 => include_str!("marker_pick.wgsl"),
            _ => unreachable!(),
        };
        let pick_shader = pickable.then(|| {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("ink IDs"),
                source: wgpu::ShaderSource::Wgsl(
                    format!(
                        "{source}\n{}\n{pick_source}",
                        include_str!("pick_crop.wgsl")
                    )
                    .into(),
                ),
            })
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
```

**Locate this block:**

```rust
        vertices_per_instance: u32,
        attributes: &[wgpu::VertexAttribute],
        depth: wgpu::DepthStencilState,
    ) -> Self {
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
```

Replace it with:

```rust
        vertices_per_instance: u32,
        attributes: &[wgpu::VertexAttribute],
        depth: wgpu::DepthStencilState,
        pickable: bool,
    ) -> Self {
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
```

**Locate this block:**

```rust
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
            crate::depth::grid(),
        )
    }
```

Replace it with:

```rust
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
            crate::depth::grid(),
            false,
        )
    }
```

**Locate this block:**

```rust
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],
            depth,
        )
    }
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Replace it with:

```rust
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],
            depth,
            true,
        )
    }
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

**Locate this block:**

```rust
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32],
            depth,
        )
    }
    pub fn overlay(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Replace it with:

```rust
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32],
            depth,
            true,
        )
    }
    pub fn overlay(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

**Locate this block:**

```rust
            vertices_per_instance,
            attributes,
            crate::depth::ink(),
        )
    }
```

Replace it with:

```rust
            vertices_per_instance,
            attributes,
            crate::depth::ink(),
            true,
        )
    }
```

**Locate this block:**

```rust
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids)) = (&self.vertices, &self.ids) else {
            return;
        };
        pass.set_pipeline(&self.pick_pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
```

Replace it with:

```rust
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (
            &self.vertices,
            &self.ids,
            &self.pick_pipeline,
        ) else {
            return;
        };
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
```

**Locate this block:**

```rust
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: wgpu::RenderPipeline,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    uniform: wgpu::Buffer,
```

Replace it with:

```rust
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    uniform: wgpu::Buffer,
```

### 2. `src/grid_pick_pipeline_tests.rs`

Supply actual resource, grid pixels, no-owner, original-pick/source/upload and release checks.

Create the file and type:

```rust
#[test]
fn grid_omits_its_base_integer_pipeline_without_stealing_original_picks() {
    let scene = crate::scene::Scene::empty();
    let mut gpu = crate::cap_gpu_tests::renderer(&scene);
    let grid = super::Lane::grid(&gpu.device, wgpu::TextureFormat::Rgba8Unorm);
    let original = super::Lane::new(&gpu.device, wgpu::TextureFormat::Rgba8Unorm);
    assert!(grid.pick_pipeline.is_none());
    assert!(original.pick_pipeline.is_some());
    gpu.set_grid_on(
        Some(crate::grid::Settings {
            spacing: 0.25,
            half_cells: 5,
        }),
        crate::cplane::CPlane::Xy,
    )
    .unwrap();
    let matrix = crate::cap_gpu_tests::matrix();
    let image = crate::cap_gpu_tests::pixels_at(&gpu, matrix);
    assert!(
        image
            .chunks_exact(4)
            .filter(|p| p[..3] != [255; 3])
            .count()
            > 100
    );
    gpu.request_pick([0., 0.], matrix).unwrap();
    crate::cap_gpu_tests::wait(&gpu);
    assert!(
        gpu.take_pick(matrix)
            .unwrap()
            .unwrap()
            .is_none()
    );
    let mut scene = scene;
    let owner = scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [-0.8, -0.8, -0.5],
                    [0.8, -0.8, -0.5],
                    [0., 0.8, -0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    gpu.set_scene(&scene, None);
    let uploads = gpu.stats()[0];
    gpu.request_pick([0., 0.], matrix).unwrap();
    crate::cap_gpu_tests::wait(&gpu);
    let pick = gpu
        .take_pick(matrix)
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(pick[0], owner.value());
    assert_eq!(gpu.stats()[0], uploads);
    assert!(std::rc::Rc::ptr_eq(
        &source,
        scene.objects()[0].geometry().unwrap()
    ));
    gpu.set_scene(&crate::scene::Scene::empty(), None);
    gpu.set_grid_on(None, crate::cplane::CPlane::Xy)
        .unwrap();
    assert_eq!(gpu.usage(), [0; 4]);
}
```

### 3. `src/stroke_gpu.rs`

Register optional native inspection within the lane module; browser compilation excludes it.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
#[path = "grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;
```

## <a id="checkpoint-96zzzzzzc-draw-the-construction-grid-as-physical-lines"></a>3. Draw the construction grid as physical lines

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/grid.rs, src/grid.wgsl</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Existing builder body</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">precise grid endpoints</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original surfaces</text>
</svg>
</div>

**What this code does:** Reuse the grid owner with a physical background pipeline.

### 1. `src/grid.rs`

Use the maintained 1000-unit cell recipe. The hardware axes share one physical width.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                start: [0.0; 3],
                end,
                colour,
                width: 2.0,
            });
```

Replace it with:

```rust
                start: [0.0; 3],
                end,
                colour,
                width: 1.0,
            });
```

**Locate this block:**

```rust
        Self {
            spacing: 1.0,
            half_cells: 5,
        }
    }
```

Replace it with:

```rust
        Self {
            spacing: 1000.0,
            half_cells: 5,
        }
    }
```

### 2. `src/grid.wgsl`

Transform the existing endpoints and write physical color with primitive zero.

Create the file and type:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
}
@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output {
    return Output(view.transform * vec4<f32>(select(start, end, index == 1u), 1.0), colour);
}
struct Physical {
    @location(0) colour: vec4<f32>,
    @location(1) primitive: vec2<u32>,
}
@fragment
fn fragment(input: Output) -> Physical { return Physical(input.colour, vec2<u32>(0u)); }
```

### 3. `src/stroke_gpu.rs`

Declare the common target builder, then move the existing body into it immediately. Move the already-typed builder body unchanged; the following edits add the target choice. Keep the existing color-only entry point as a thin call to the shared owner. Extend the single builder body with physical targets and LineList topology; the grid remains explicitly unpickable.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            uploaded_bytes: 0,
        }
    }
```

Replace it with:

```rust
            uploaded_bytes: 0,
        }
    }

    fn layout_depth(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        source: &str,
        stride: u64,
        vertices_per_instance: u32,
        attributes: &[wgpu::VertexAttribute],
        depth: wgpu::DepthStencilState,
        pickable: bool,
    ) -> Self {
        Self::layout_target(
            device,
            format,
            source,
            stride,
            vertices_per_instance,
            attributes,
            depth,
            pickable,
            false,
        )
    }
```

**Locate this block:**

```rust
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
```

Replace it with:

```rust
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            primitive: wgpu::PrimitiveState {
                topology: if physical {
                    wgpu::PrimitiveTopology::LineList
                } else {
                    wgpu::PrimitiveTopology::TriangleList
                },
                ..Default::default()
            },
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
```

**Locate this block:**

```rust
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth = depth.depth_compare == Some(crate::depth::CLOSER);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: Some(&layout),
```

Replace it with:

```rust
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth = depth.depth_compare == Some(crate::depth::CLOSER);
        let mut targets = vec![Some(wgpu::ColorTargetState {
            format,
            blend: if physical {
                None
            } else {
                Some(wgpu::BlendState::ALPHA_BLENDING)
            },
            write_mask: wgpu::ColorWrites::ALL,
        })];
        if physical {
            targets.push(Some(crate::primitive_image::target(true)));
        }
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: Some(&layout),
```

**Locate this block:**

```rust
        attributes: &[wgpu::VertexAttribute],
        depth: wgpu::DepthStencilState,
        pickable: bool,
    ) -> Self {
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
```

Replace it with:

```rust
        attributes: &[wgpu::VertexAttribute],
        depth: wgpu::DepthStencilState,
        pickable: bool,
        physical: bool,
    ) -> Self {
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
```

**Locate this block:**

```rust
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_depth(
            device,
            format,
            &crate::stroke::shader(),
            44,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
            crate::depth::grid(),
            false,
        )
    }

    fn layout_depth(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        source: &str,
```

Replace it with:

```rust
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_target(
            device,
            format,
            include_str!("grid.wgsl"),
            44,
            2,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
            crate::depth::grid(),
            false,
            true,
        )
    }

    fn layout_target(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        source: &str,
```

### 4. `src/renderer.rs`

Draw the grid in the physical background before surfaces and remove its finite setup/draw.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            pass.set_bind_group(2, &clip_group, &[]);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);
```

Replace it with:

```rust
            pass.set_bind_group(2, &clip_group, &[]);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.strokes.draw(&mut pass);
```

**Locate this block:**

```rust
                ],
                ..Default::default()
            });
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
```

Replace it with:

```rust
                ],
                ..Default::default()
            });
            self.grid.draw(&mut pass);
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
```

**Locate this block:**

```rust
            &self.strokes,
            &self.paths,
            &self.markers,
            &self.grid,
        ] {
            lane.finite_frame(&self.device, &tables, &self.depth);
        }
```

Replace it with:

```rust
            &self.strokes,
            &self.paths,
            &self.markers,
        ] {
            lane.finite_frame(&self.device, &tables, &self.depth);
        }
```

**Locate this block:**

```rust
            &mut renderer.markers,
            &mut renderer.grid,
        ] {
            lane.enable_finite(&renderer.device, format);
        }
```

Replace it with:

```rust
            &mut renderer.markers,
        ] {
            lane.enable_finite(&renderer.device, format);
        }
```

### 5. `src/widget_tests.rs`

Reuse the native pixel reader with a declared sRGB target; old callers remain unchanged.

**Locate this existing block:**

```rust
) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("selection pixels"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> Vec<u8> {
    pixels_format(
        renderer,
        transform,
        size,
        wgpu::TextureFormat::Rgba8Unorm,
    )
}
pub(crate) fn pixels_format(
    renderer: &mut Renderer,
    transform: &[f32; 16],
    size: [u32; 2],
    format: wgpu::TextureFormat,
) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("selection pixels"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
```

### 6. `src/grid_source_tests.rs`

Retain count, colors, bounds and invalid-input checks with current default spacing.

**Locate this existing block:**

```rust
    assert_eq!(strokes[22].colour, AXIS_COLOURS[0]);
    assert_eq!(strokes[22].end, [5.0, 0.0, 0.0]);
    assert_eq!(strokes[23].colour, AXIS_COLOURS[1]);
    assert_eq!(strokes[24].colour, AXIS_COLOURS[2]);
    assert_eq!(strokes[24].end, [0.0, 0.0, 1.0]);
    assert_eq!(
        Settings {
            half_cells: 500,
            ..grid
        }
        .strokes()
        .unwrap()
        .len(),
        2005
    );
    for bad in [
        Settings {
            spacing: 0.0,
            ..grid
        },
        Settings {
            spacing: f64::NAN,
            ..grid
        },
        Settings {
            spacing: f64::MAX,
            ..grid
        },
        Settings {
            half_cells: 501,
            ..grid
        },
        Settings {
            half_cells: 0,
            ..grid
        },
    ] {
        assert!(bad.strokes().is_err());
    }
    assert!(grid.reach([f64::NAN, 0.0, 0.0]).is_err());
    assert_eq!(grid.reach([0.0; 3]).unwrap(), 50.0f64.sqrt());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(strokes[22].colour, AXIS_COLOURS[0]);
    assert_eq!(strokes[22].end, [5000.0, 0.0, 0.0]);
    assert_eq!(strokes[23].colour, AXIS_COLOURS[1]);
    assert_eq!(strokes[24].colour, AXIS_COLOURS[2]);
    assert_eq!(strokes[24].end, [0.0, 0.0, 1000.0]);
    assert_eq!(
        Settings {
            half_cells: 500,
            ..grid
        }
        .strokes()
        .unwrap()
        .len(),
        2005
    );
    for bad in [
        Settings {
            spacing: 0.0,
            ..grid
        },
        Settings {
            spacing: f64::NAN,
            ..grid
        },
        Settings {
            spacing: f64::MAX,
            ..grid
        },
        Settings {
            half_cells: 501,
            ..grid
        },
        Settings {
            half_cells: 0,
            ..grid
        },
    ] {
        assert!(bad.strokes().is_err());
    }
    assert!(grid.reach([f64::NAN, 0.0, 0.0]).is_err());
    assert_eq!(
        grid.reach([0.0; 3]).unwrap(),
        50_000_000.0f64.sqrt()
    );
}
```

### 7. `src/grid_reference_fixture.json`

Supply dated independent native-grid pixels and declared physical poses.

Create the file and type:

```json
{
  "scope": "Independent maintained native empty-scene hardware-grid raster at two physical viewport sizes, single sample. Native output is not an actual browser DPR measurement.",
  "createdAt": "2026-10-15T04:41:11.533Z",
  "inputHashes": {
    "src/camera.rs": "2eb0d51723129bdea1cddc3f35760217923959afe2a50dfe1fc6e129c8a3d58f",
    "src/shaders/grid.wgsl": "67daedfc89ed5b5553d2061429a4d725d1bc12b7f8e503d04bf1d709befc21fb",
    "src/engine/gpu/backdrop.rs": "afb51eaef8ce003e22ab9b4ce0e0af2c1ee786c98b98ceb22992f2a3edf4d6bc",
    "src/selftest.rs": "dec37fbc978002c1184181627769ac0c6daeec91da64b13373ef1d33bdaa08fc",
    "examples/selftest.rs": "af55062d4534e26d77925657d8daae8437772fb49a2a5e422a2e43e684a23b76",
    "Cargo.toml": "f3c4a4e33279d03a069b041b21d5b25be8c5f2c1846adbf406c693fc40234002",
    "Cargo.lock": "9b313409fa85751cba3224e7902022ee6a9420f7fdeaf69acf13c0ec62a6d9de",
    "src/shaders/background.wgsl": "ea62408626759db7cf4a3756d83dbbd5aead2621aeee028f4c54de4310ef2aa6",
    "src/engine/gpu/frame.rs": "2a492fe36f15db4201a687a1d06e120945d38410a075375fece6ec1d3d4323fb",
    "src/engine/gpu/render.rs": "48150fc2c3afa9a453af5fa0d0aa35768ed42ee57454a3c46ef0016a518dfabc"
  },
  "pose": {
    "targetSceneUnits": [
      0,
      0,
      0
    ],
    "eyeSceneUnits": [
      0,
      0,
      12000
    ],
    "distanceSceneUnits": 12000,
    "unit": "millimetres",
    "projection": "orthographic",
    "halfHeightSceneUnits": 6928.203230276,
    "backgroundReadback": [
      255,
      255,
      255
    ],
    "note": "The maintained background pass overwrites selftest CLEAR 0.9 with white when ambient shading is off. Match the actual white readback, not the earlier clear value."
  },
  "oracleManifestSha256": "13e832ae0d5a0153ebb5f85e72e276b23347ace053b6d799030d44b2ae8d6a72",
  "frames": [
    {
      "asset": "docs/journey/checks/data/current-grid-physical-256-2026-10-15.ppm",
      "physicalSize": [
        256,
        256
      ],
      "sha256": "a1dea16ee3639ebebc9225a529366cda93d167266b93a95d29dd6b8beca84507"
    },
    {
      "asset": "docs/journey/checks/data/current-grid-physical-512-2026-10-15.ppm",
      "physicalSize": [
        512,
        512
      ],
      "sha256": "d871ce0ed9583adc0100f2134e2ce86f7754ebf5883dc06721364edf652dda19"
    }
  ]
}
```

### 8. `src/grid_hardware_tests.rs`

Supply exact native-grid raster, opaque/glass background order, no-owner, source reuse and release checks.

Create the file and type:

```rust
use crate::{
    camera::{Camera, Projection, View},
    grid::Settings,
    renderer::Renderer,
    scene::Scene,
    viewport::Viewport,
};
use sha2::{Digest, Sha256};
fn renderer(scene: &Scene, format: wgpu::TextureFormat) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) =
        pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|error| {
        panic!("Actual hardware grid: {error}")
    }));
    Renderer::new(device, queue, format, scene)
}
#[test]
fn actual_hardware_grid_matches_independent_current_native_frames() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("grid_reference_fixture.json")).unwrap();
    for density in [1u32, 2] {
        let size = 256 * density;
        let scene = Scene::empty();
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let mut gpu = renderer(&scene, format);
        gpu.resize(Viewport {
            width: size,
            height: size,
        });
        gpu.set_density(density as f32);
        let mut camera = Camera::default();
        camera.set_view(View::Top);
        camera.projection = Projection::Orthographic;
        camera.distance = 12000.;
        camera.aspect = 1.;
        camera.grid = Some(Settings::default());
        gpu.set_grid_on(camera.grid, crate::cplane::CPlane::Xy)
            .unwrap();
        let usage = gpu.grid_usage();
        assert_eq!(&usage[..2], &[25, 1100]);
        assert_eq!(gpu.clipping_ink_bytes(), [0, 0]);
        let actual = crate::widget_tests::pixels_format(
            &mut gpu,
            &camera.uniform(),
            [size, size],
            format,
        );
        let frame = &manifest["frames"][(density - 1) as usize];
        let bytes = std::fs::read(frame["asset"].as_str().unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            frame["sha256"].as_str().unwrap()
        );
        let header = format!("P6\n{size} {size}\n255\n");
        let expected = bytes
            .strip_prefix(header.as_bytes())
            .unwrap();
        let rgb = actual
            .chunks_exact(4)
            .flat_map(|p| p[..3].iter().copied())
            .collect::<Vec<_>>();
        assert!(
            rgb == expected,
            "Actual hardware grid differs from the independent current renderer at physical size {size}"
        );
        gpu.set_grid_on(camera.grid, crate::cplane::CPlane::Xy)
            .unwrap();
        assert_eq!(gpu.grid_usage(), usage);
        gpu.request_pick([0., 0.], camera.uniform())
            .unwrap();
        crate::cap_gpu_tests::wait(&gpu);
        assert!(
            gpu.take_pick(camera.uniform())
                .unwrap()
                .unwrap()
                .is_none()
        );
        gpu.set_grid_on(None, crate::cplane::CPlane::Xy)
            .unwrap();
        assert_eq!(&gpu.grid_usage()[..2], &[0, 0]);
        assert_eq!(gpu.usage(), [0; 4]);
    }
}
#[test]
fn grid_stays_behind_original_solid_color_and_never_owns_a_pick() {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(
            crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false))
                .unwrap(),
        )
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    let mut gpu = crate::cap_gpu_tests::renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.set_view(View::Top);
    camera.aspect = 4. / 3.;
    let matrix = camera.uniform();
    let uploads = gpu.stats()[0];
    let settings = Settings {
        spacing: 0.25,
        half_cells: 5,
    };
    let pixels =
        |gpu: &mut Renderer| crate::widget_tests::pixels(gpu, &matrix, [320, 240]);
    let solid = pixels(&mut gpu);
    gpu.set_grid_on(Some(settings), crate::cplane::CPlane::Xy)
        .unwrap();
    let opaque = pixels(&mut gpu);
    let center = (100 * 320 + 159) * 4;
    assert_eq!(
        &opaque[center..center + 4],
        &solid[center..center + 4]
    );
    assert!(
        opaque != solid,
        "The grid must draw outside the opaque solid"
    );
    gpu.set_opacity(0.5);
    let glass_grid = pixels(&mut gpu);
    gpu.set_grid_on(None, crate::cplane::CPlane::Xy)
        .unwrap();
    let glass_plain = pixels(&mut gpu);
    assert_ne!(
        &glass_grid[center..center + 4],
        &glass_plain[center..center + 4],
        "The hardware grid is behind the transparent original"
    );
    gpu.set_grid_on(Some(settings), crate::cplane::CPlane::Xy)
        .unwrap();
    gpu.request_pick([0., 0.], matrix).unwrap();
    crate::cap_gpu_tests::wait(&gpu);
    let pick = gpu
        .take_pick(matrix)
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(pick[0], owner.value());
    assert_eq!(gpu.stats()[0], uploads);
    assert!(std::rc::Rc::ptr_eq(
        &source,
        scene.objects()[0].geometry().unwrap()
    ));
    gpu.set_scene(&Scene::empty(), None);
    gpu.request_pick([0., 0.], matrix).unwrap();
    crate::cap_gpu_tests::wait(&gpu);
    assert!(
        gpu.take_pick(matrix)
            .unwrap()
            .unwrap()
            .is_none()
    );
    gpu.set_grid_on(None, crate::cplane::CPlane::Xy)
        .unwrap();
    assert_eq!(gpu.usage(), [0; 4]);
}
```

### 9. `src/lib.rs`

Register optional native grid validation outside the browser graph.

**Locate this existing block:**

```rust
#[cfg(test)]
mod camera_reference_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod camera_reference_tests;

#[cfg(test)]
mod grid_hardware_tests;
```

## <a id="checkpoint-96zzzzzzd-build-viewing-rays-without-matrix-inversion"></a>4. Build viewing rays without matrix inversion

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/camera.rs, src/camera_a…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Screen coordinate</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">camera axes</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">precise world ray</text>
</svg>
</div>

**What this code does:** Keep drawing rays and rendered depth consistent in both projections, including when the construction grid is hidden.

### 1. `src/camera.rs`

Construct analytic viewing rays and share the current ground/fixed-plane range independently of grid visibility. Use one depth-limit owner for the projection and ray. The orthographic limit already comes from that shared owner.

**Locate this existing block:**

```rust
        let near = distance * NEAR_FRACTION;
        let reach = self
            .grid
            .and_then(|grid| {
                grid.reach_on(
                    self.grid_plane.unwrap_or_else(|| {
                        crate::cplane::CPlane::view(
                            &self
                                .orientation
                                .rotate_vector(Vector::y_axis()),
                        )
                    }),
                    self.target,
                )
                .ok()
            })
            .map_or(self.radius, |reach| self.radius.max(reach));
        let reach = reach * scale;
        let far = (distance * 10.0).max(distance + 2.0 * reach);
        let projection = match self.projection {
            Projection::Perspective => {
                Xform::perspective(2.0 * HALF_FOV, self.aspect, far, near)
            }
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6);
                Xform::orthographic(-w, w, -h, h, depth_range, -depth_range)
            }
        };
        let shift = self.shift();
        let mut matrix = Xform::translation(shift[0], shift[1], 0.)
            * projection
            * view
            * Xform::scale_xyz(scale, scale, scale);
        if self.projection == Projection::Perspective {
            for value in &mut matrix.m {
                *value /= scale;
            }
        }
        matrix
    }

    pub fn shift(&self) -> [f64; 2] {
        [
            self.frame[0] + self.frame[2] - 1.,
            1. - self.frame[1] - self.frame[3],
        ]
    }

    pub fn frame(&mut self, frame: [f64; 4]) -> bool {
        if frame
            .iter()
            .any(|v| !v.is_finite() || *v < 0. || *v > 1.)
            || frame[0] >= frame[2]
            || frame[1] >= frame[3]
        {
            return false;
        }
        if self.frame == frame {
            return false;
        }
        self.frame = frame;
        true
    }

    pub fn pan_screen(&mut self, delta: [f64; 2], size: [f64; 2]) {
        if delta.iter().any(|v| !v.is_finite())
            || size
                .iter()
                .any(|v| !v.is_finite() || *v <= 0.)
        {
            return;
        }
        let scale = self.distance * 0.0015;
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += scale * (-delta[0] * right[i] + delta[1] * up[i]);
        }
    }

    pub fn zoom_at(&mut self, factor: f32, screen: [f32; 2]) {
        if screen
            .iter()
            .any(|v| !v.is_finite() || v.abs() > 1.)
        {
            return;
        }
        let old = self.distance;
        self.zoom(factor);
        let k = self.distance / old;
        let shift = self.shift();
        let h = old * HALF_FOV.tan();
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += (1. - k)
                * h
                * (right[i] * (screen[0] as f64 - shift[0]) * self.aspect
                    + up[i] * (screen[1] as f64 - shift[1]));
        }
    }

    pub fn uniform(&self) -> [f32; 16] {
        self.view_projection().to_f32()
    }

    pub fn ray(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen
            .iter()
            .any(|value| !value.is_finite() || value.abs() > 1.0)
        {
            return None;
        }
        self.ray_unbounded(screen)
    }

    pub fn ray_unbounded(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen.iter().any(|v| !v.is_finite()) {
            return None;
        }
        let inverse = self
            .view_projection_anchored(self.target)
            .inverse()?;
        let origin = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            crate::depth::NEAR,
        ));
        let end = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            crate::depth::FAR,
        ));
        let delta = &end - &origin;
        let max_distance = delta.magnitude();
        if !max_distance.is_finite() || max_distance <= 0.0 {
            return None;
        }
        let origin = Point::new(
            origin[0] + self.target[0],
            origin[1] + self.target[1],
            origin[2] + self.target[2],
        );
        Some(Ray {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let near = distance * NEAR_FRACTION;
        let [far, depth_range] = self.depth_limits();
        let projection = match self.projection {
            Projection::Perspective => {
                Xform::perspective(2.0 * HALF_FOV, self.aspect, far, near)
            }
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                Xform::orthographic(-w, w, -h, h, depth_range, -depth_range)
            }
        };
        let shift = self.shift();
        let mut matrix = Xform::translation(shift[0], shift[1], 0.)
            * projection
            * view
            * Xform::scale_xyz(scale, scale, scale);
        if self.projection == Projection::Perspective {
            for value in &mut matrix.m {
                *value /= scale;
            }
        }
        matrix
    }

    pub fn shift(&self) -> [f64; 2] {
        [
            self.frame[0] + self.frame[2] - 1.,
            1. - self.frame[1] - self.frame[3],
        ]
    }

    pub fn frame(&mut self, frame: [f64; 4]) -> bool {
        if frame
            .iter()
            .any(|v| !v.is_finite() || *v < 0. || *v > 1.)
            || frame[0] >= frame[2]
            || frame[1] >= frame[3]
        {
            return false;
        }
        if self.frame == frame {
            return false;
        }
        self.frame = frame;
        true
    }

    pub fn pan_screen(&mut self, delta: [f64; 2], size: [f64; 2]) {
        if delta.iter().any(|v| !v.is_finite())
            || size
                .iter()
                .any(|v| !v.is_finite() || *v <= 0.)
        {
            return;
        }
        let scale = self.distance * 0.0015;
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += scale * (-delta[0] * right[i] + delta[1] * up[i]);
        }
    }

    pub fn zoom_at(&mut self, factor: f32, screen: [f32; 2]) {
        if screen
            .iter()
            .any(|v| !v.is_finite() || v.abs() > 1.)
        {
            return;
        }
        let old = self.distance;
        self.zoom(factor);
        let k = self.distance / old;
        let shift = self.shift();
        let h = old * HALF_FOV.tan();
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.target[i] += (1. - k)
                * h
                * (right[i] * (screen[0] as f64 - shift[0]) * self.aspect
                    + up[i] * (screen[1] as f64 - shift[1]));
        }
    }

    pub fn uniform(&self) -> [f32; 16] {
        self.view_projection().to_f32()
    }

    pub fn ray(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen
            .iter()
            .any(|value| !value.is_finite() || value.abs() > 1.0)
        {
            return None;
        }
        self.ray_unbounded(screen)
    }

    fn depth_limits(&self) -> [f64; 2] {
        let scale = self.unit.to_meters();
        let distance = self.distance * scale;
        let half = crate::grid::Settings::default()
            .half()
            .unwrap()
            * scale;
        let target = self.target.map(|v| v * scale);
        let grid = match self.grid_plane {
            Some(plane) => {
                let center = plane.origin();
                let d =
                    std::array::from_fn::<_, 3, _>(|i| target[i] - center[i] * scale);
                (d.iter().map(|v| v * v).sum::<f64>()).sqrt() + half * 2f64.sqrt()
            }
            None => {
                let x = target[0].abs() + half;
                let y = target[1].abs() + half;
                (x * x + y * y + target[2] * target[2]).sqrt()
            }
        };
        let scene = if self.radius > 0. {
            self.radius * scale
        } else {
            distance
        };
        let reach = scene.max(grid);
        [
            (distance * 10.).max(distance + 2. * reach),
            (distance + 2. * reach).max(1e-6),
        ]
    }

    pub fn ray_unbounded(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen.iter().any(|v| !v.is_finite())
            || !self.distance.is_finite()
            || self.distance <= 0.
        {
            return None;
        }
        let shift = self.shift();
        let half = self.distance * HALF_FOV.tan();
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let plane = &right * ((screen[0] as f64 - shift[0]) * half * self.aspect)
            + &up * ((screen[1] as f64 - shift[1]) * half);
        let [far, range] = self
            .depth_limits()
            .map(|v| v / self.unit.to_meters());
        let (local, delta) = match self.projection {
            Projection::Perspective => {
                let direction = &plane + &(&forward * self.distance);
                (
                    &forward * (-self.distance) + &direction * NEAR_FRACTION,
                    direction * (far / self.distance - NEAR_FRACTION),
                )
            }
            Projection::Orthographic => (
                plane - &forward * (self.distance + range),
                &forward * (2. * range),
            ),
        };
        let max_distance = delta.magnitude();
        if !max_distance.is_finite() || max_distance <= 0. {
            return None;
        }
        let origin = Point::new(
            self.target[0] + local[0],
            self.target[1] + local[1],
            self.target[2] + local[2],
        );
        if (0..3).any(|i| !origin[i].is_finite()) {
            return None;
        }
        Some(Ray {
```

### 2. `src/camera_analytic_ray_tests.rs`

Supply independent current ray-line comparisons, exact projected endpoints, both units and precise distant-origin guards.

Create the file and type:

```rust
use crate::camera::{Camera, Projection, Unit};
use crate::camera_reference_tests as current;
#[test]
fn ordinary_large_orthographic_rays_match_current_lines_in_both_units() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            let mut camera = Camera::default();
            camera.unit = unit;
            let scale = unit.to_meters();
            camera.distance = 20. / scale;
            camera.target = [2. / scale, -3. / scale, 4. / scale];
            camera.projection = projection;
            camera.aspect = 1.6;
            camera.frame = [0.1, 0.05, 0.8, 0.9];
            let mut reference = current::Camera::new();
            reference.unit = match unit {
                Unit::Millimeters => current::Unit::Millimeters,
                Unit::Meters => current::Unit::Meters,
            };
            reference.distance = 20.;
            reference.target = [2., -3., 4.];
            reference.perspective = projection == Projection::Perspective;
            reference.frame = camera.frame;
            reference.update_position();
            for screen in [[0., 0.], [0.7, -0.4], [-1.3, 1.1]] {
                let ray = camera.ray_unbounded(screen).unwrap();
                let pixel = (
                    (screen[0] as f64 + 1.) * 400.,
                    (1. - screen[1] as f64) * 250.,
                );
                let (origin, direction) = reference.ray(pixel, (800., 500.)).unwrap();
                assert!(
                    (&ray.origin - &origin)
                        .cross(&direction)
                        .magnitude()
                        * scale
                        < 1e-10
                );
                assert!((&ray.direction - &direction).magnitude() < 1e-12);
                let matrix = camera.view_projection();
                let near = matrix.transform_point(&ray.origin);
                let end = &ray.origin + &(&ray.direction * ray.max_distance);
                let far = matrix.transform_point(&end);
                for axis in 0..2 {
                    assert!((near[axis] - screen[axis] as f64).abs() < 1e-9);
                    assert!((far[axis] - screen[axis] as f64).abs() < 1e-9);
                }
                assert!((near[2] - 1.).abs() < 1e-9);
                assert!(far[2].abs() < 1e-9);
            }
        }
    }
}
#[test]
fn analytic_rays_keep_precise_direction_frame_and_finite_endpoint_guards() {
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        let mut local = Camera::default();
        local.projection = projection;
        local.distance = 20000.;
        local.frame = [0.2, 0.1, 0.9, 0.8];
        local.grid_plane = Some(crate::cplane::CPlane::Frame {
            origin: [0.; 3],
            x: [1., 0., 0.],
            y: [0., 1., 0.],
        });
        let mut far = Camera::default();
        far.projection = local.projection;
        far.distance = local.distance;
        far.frame = local.frame;
        far.target = [1e12, -1e12, 1e12];
        far.grid_plane = Some(crate::cplane::CPlane::Frame {
            origin: far.target,
            x: [1., 0., 0.],
            y: [0., 1., 0.],
        });
        for screen in [[0., 0.], [0.2, -0.1], [1., -1.]] {
            let a = local.ray(screen).unwrap();
            let b = far.ray(screen).unwrap();
            for i in 0..3 {
                assert!((b.origin[i] - far.target[i] - a.origin[i]).abs() < 0.00025);
                assert_eq!(a.direction[i], b.direction[i]);
            }
            assert_eq!(a.max_distance, b.max_distance);
            assert!((a.direction.magnitude() - 1.).abs() < 1e-12);
        }
    }
    let mut camera = Camera::default();
    for invalid in [[f32::NAN, 0.], [0., f32::INFINITY]] {
        assert!(camera.ray_unbounded(invalid).is_none());
    }
    assert!(camera.ray([1.1, 0.]).is_none());
    assert!(camera.ray_unbounded([1.1, 0.]).is_some());
    for distance in [0., -1., f64::NAN, f64::INFINITY] {
        camera.distance = distance;
        assert!(camera.ray([0., 0.]).is_none());
    }
}
```

### 3. `src/lib.rs`

Register the optional native ray and depth comparisons outside the browser graph.

**Locate this existing block:**

```rust
#[cfg(test)]
mod grid_hardware_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod grid_hardware_tests;

#[cfg(test)]
mod camera_analytic_ray_tests;

#[cfg(test)]
mod camera_depth_reach_tests;
```

### 4. `src/camera_depth_reach_tests.rs`

Supply independent maintained-camera matrices and controlled fixed-plane translations in both units.

Create the file and type:

```rust
use crate::camera::{Camera, Projection, Unit};
use crate::camera_reference_tests as current;
use crate::cplane::CPlane;
use session_rust::Point;
fn fixed(center: [f64; 3]) -> CPlane {
    CPlane::Frame {
        origin: center,
        x: [0., 1., 0.],
        y: [0., 0., 1.],
    }
}
#[test]
fn ground_and_fixed_plane_depth_match_current_in_both_units_even_when_hidden() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            for physical_target in [[0.; 3], [10., -2., 3.], [1e6, -2e6, 3e6]] {
                for extent in [0., 0.2, 20000.] {
                    for center in [None, Some([7., -11., 13.])] {
                        let scale = unit.to_meters();
                        let mut camera = Camera::default();
                        camera.unit = unit;
                        camera.target = physical_target.map(|v| v / scale);
                        camera.distance = 3. / scale;
                        camera.radius = extent / scale;
                        camera.projection = projection;
                        camera.aspect = 1.6;
                        camera.frame = [0.1, 0.05, 0.8, 0.9];
                        camera.grid_plane = center.map(|c| fixed(c.map(|v| v / scale)));
                        let mut reference = current::Camera::new();
                        reference.unit = match unit {
                            Unit::Millimeters => current::Unit::Millimeters,
                            Unit::Meters => current::Unit::Meters,
                        };
                        reference.target = physical_target;
                        reference.distance = 3.;
                        reference.scene_extent = extent;
                        reference.grid = center.map(|c| c.map(|v| v / scale));
                        reference.perspective = projection == Projection::Perspective;
                        reference.frame = camera.frame;
                        reference.update_position();
                        let anchor = camera.target;
                        let expected = reference.view_proj_anchored(
                            camera.aspect,
                            &Point::new(anchor[0], anchor[1], anchor[2]),
                        );
                        let actual = camera.view_projection_anchored(anchor);
                        for p in [[0.; 3], [0.2, -0.1, 0.05], [-0.3, 0.2, -0.1]] {
                            let point =
                                Point::new(p[0] / scale, p[1] / scale, p[2] / scale);
                            let a = actual.transform_point(&point);
                            let b = expected.transform_point(&point);
                            for i in 0..3 {
                                assert!(
                                    (a[i] - b[i]).abs() < 1e-8,
                                    "Current camera mismatch in {unit:?} {projection:?}: {a:?} {b:?}"
                                );
                            }
                        }
                        for settings in [
                            None,
                            Some(crate::grid::Settings::default()),
                            Some(crate::grid::Settings {
                                spacing: 0.25,
                                half_cells: 2,
                            }),
                        ] {
                            camera.grid = settings;
                            assert_eq!(
                                camera.view_projection_anchored(anchor).m,
                                actual.m,
                                "Showing or hiding the grid cannot alter camera depth"
                            );
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn translating_the_fixed_plane_with_the_scene_preserves_camera_and_ray_ranges() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            let mut camera = Camera::default();
            camera.unit = unit;
            camera.projection = projection;
            camera.distance = 3. / unit.to_meters();
            camera.target = [0.125, -0.25, 0.5];
            camera.grid_plane = Some(fixed([0.; 3]));
            let matrix = camera
                .view_projection_anchored(camera.target)
                .m;
            let ray = camera.ray([0.2, -0.1]).unwrap();
            let shift = [1e9, -1e9, 1e9];
            camera.target = std::array::from_fn(|i| camera.target[i] + shift[i]);
            camera.grid_plane = Some(fixed(shift));
            let far = camera.ray([0.2, -0.1]).unwrap();
            let after = camera.view_projection_anchored(camera.target);
            for i in 0..16 {
                assert!(
                    (after.m[i] - matrix[i]).abs() <= matrix[i].abs() * 1e-7 + 1e-9
                );
            }
            for i in 0..3 {
                assert!((far.origin[i] - shift[i] - ray.origin[i]).abs() < 1e-5);
                assert!((far.direction[i] - ray.direction[i]).abs() < 1e-12);
            }
            assert!((far.max_distance - ray.max_distance).abs() < 1e-5);
            let mut reference = current::Camera::new();
            reference.unit = match unit {
                Unit::Millimeters => current::Unit::Millimeters,
                Unit::Meters => current::Unit::Meters,
            };
            reference.distance = 3.;
            reference.target = camera.target.map(|v| v * unit.to_meters());
            reference.grid = Some(shift);
            reference.perspective = projection == Projection::Perspective;
            reference.update_position();
            let a = after.transform_point(&Point::new(0., 0., 0.));
            let b = reference
                .view_proj_anchored(camera.aspect, &reference.origin())
                .transform_point(&Point::new(0., 0., 0.));
            for i in 0..3 {
                assert!((a[i] - b[i]).abs() < 1e-8);
            }
        }
    }
}
```

### 5. `src/camera_reverse_tests.rs`

Declare a translated fixed plane and a scene extent dominating its grid while retaining all reversed endpoints and ray bounds.

**Locate this existing block:**

```rust
                camera.distance = 4000.;
                camera.radius = 1000.;
                let direction = camera
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                camera.distance = 4000.;
                camera.radius = 20000.;
                camera.grid_plane = Some(crate::cplane::CPlane::Frame {
                    origin: target,
                    x: [1., 0., 0.],
                    y: [0., 1., 0.],
                });
                let direction = camera
```

### 6. `src/camera_space_tests.rs`

Declare the same local/far fixed plane and a dominant physical unit-equivalence extent; retain all screen, depth, ray, original, selection and source assertions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        );
    }
}
```

Replace it with:

```rust
        );
    }
}

fn fixed_plane(camera: &mut Camera, origin: [f64; 3]) {
    camera.grid_plane = Some(crate::cplane::CPlane::Frame {
        origin,
        x: [1., 0., 0.],
        y: [0., 1., 0.],
    });
}
```

**Locate this block:**

```rust
            (0., 0.),
        ];
        let mut camera = Camera::default();
        camera.projection = Projection::Orthographic;
        camera.distance = 3.;
        let baseline = lasso.found(&scene, &camera).unwrap();
        assert_eq!(baseline, vec![line, point, mesh]);
        camera.target = anchor;
        assert_eq!(lasso.found(&far, &camera).unwrap(), baseline);
        assert!(Rc::ptr_eq(
            &far.points()[0].prepared.source,
```

Replace it with:

```rust
            (0., 0.),
        ];
        let mut camera = Camera::default();
        fixed_plane(&mut camera, [0.; 3]);
        camera.projection = Projection::Orthographic;
        camera.distance = 3.;
        let baseline = lasso.found(&scene, &camera).unwrap();
        assert_eq!(baseline, vec![line, point, mesh]);
        camera.target = anchor;
        fixed_plane(&mut camera, anchor);
        assert_eq!(lasso.found(&far, &camera).unwrap(), baseline);
        assert!(Rc::ptr_eq(
            &far.points()[0].prepared.source,
```

**Locate this block:**

```rust
        far.translate(id, anchor).unwrap();
    }
    camera.target = anchor;
    let far_view = Screen::camera(&camera, [800., 600.]);
    assert_eq!(view.matrix, far_view.matrix);
    for (rectangle, expected) in [
```

Replace it with:

```rust
        far.translate(id, anchor).unwrap();
    }
    camera.target = anchor;
    fixed_plane(&mut camera, anchor);
    let far_view = Screen::camera(&camera, [800., 600.]);
    assert_eq!(view.matrix, far_view.matrix);
    for (rectangle, expected) in [
```

**Locate this block:**

```rust
        .unwrap();
    let point_source = scene.points()[0].prepared.source.clone();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = Projection::Orthographic;
    camera.distance = 3.;
    let view = Screen::camera(&camera, [800., 600.]);
```

Replace it with:

```rust
        .unwrap();
    let point_source = scene.points()[0].prepared.source.clone();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    fixed_plane(&mut camera, [0.; 3]);
    camera.projection = Projection::Orthographic;
    camera.distance = 3.;
    let view = Screen::camera(&camera, [800., 600.]);
```

**Locate this block:**

```rust
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        camera.projection = projection;
        camera.distance = 3.;
        let view = Screen::camera(&camera, [800., 600.]);
        camera.target = anchor;
        let far_view = Screen::camera(&camera, [800., 600.]);
        for placement in [
            TextPlacement::WorldPlane {
```

Replace it with:

```rust
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        fixed_plane(&mut camera, [0.; 3]);
        camera.projection = projection;
        camera.distance = 3.;
        let view = Screen::camera(&camera, [800., 600.]);
        camera.target = anchor;
        fixed_plane(&mut camera, anchor);
        let far_view = Screen::camera(&camera, [800., 600.]);
        for placement in [
            TextPlacement::WorldPlane {
```

**Locate this block:**

```rust
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        camera.projection = projection;
        camera.distance = 2.;
        let baseline = Screen::camera(&camera, [640., 480.]);
        camera.target = anchor;
        let far = Screen::camera(&camera, [640., 480.]);
        assert_eq!(baseline.matrix, far.matrix);
        assert!(baseline != far);
```

Replace it with:

```rust
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        fixed_plane(&mut camera, [0.; 3]);
        camera.projection = projection;
        camera.distance = 2.;
        let baseline = Screen::camera(&camera, [640., 480.]);
        camera.target = anchor;
        fixed_plane(&mut camera, anchor);
        let far = Screen::camera(&camera, [640., 480.]);
        assert_eq!(baseline.matrix, far.matrix);
        assert!(baseline != far);
```

**Locate this block:**

```rust
    ] {
        camera.projection = projection;
        let mut local = Camera::default();
        local.projection = projection;
        local.distance = 1.;
        local.radius = 1.;
```

Replace it with:

```rust
    ] {
        camera.projection = projection;
        let mut local = Camera::default();
        fixed_plane(&mut local, [0.; 3]);
        local.projection = projection;
        local.distance = 1.;
        local.radius = 1.;
```

**Locate this block:**

```rust
#[test]
fn anchored_camera_keeps_submillimetre_detail_at_a_distant_origin() {
    let mut camera = Camera::default();
    camera.target = [1e12, -1e12, 1e12];
    camera.distance = 1.;
    camera.radius = 1.;
    for projection in [
```

Replace it with:

```rust
#[test]
fn anchored_camera_keeps_submillimetre_detail_at_a_distant_origin() {
    let mut camera = Camera::default();
    fixed_plane(&mut camera, [0.; 3]);
    camera.target = [1e12, -1e12, 1e12];
    fixed_plane(&mut camera, [1e12, -1e12, 1e12]);
    camera.distance = 1.;
    camera.radius = 1.;
    for projection in [
```

**Locate this block:**

```rust
        mm.projection = projection;
        mm.target = [2e6, -3e6, 4e6];
        mm.distance = 5000.;
        mm.radius = 1000.;
        let mut m = Camera::default();
        m.projection = projection;
        m.unit = Unit::Meters;
        m.target = mm.target.map(|x| x * 0.001);
        m.distance = 5.;
        m.radius = 1.;
        let a = mm.view_projection_anchored(mm.target);
        let b = m.view_projection_anchored(m.target);
        let pa = a.transform_point(&Point::new(500., 250., 100.));
```

Replace it with:

```rust
        mm.projection = projection;
        mm.target = [2e6, -3e6, 4e6];
        mm.distance = 5000.;
        mm.radius = 20e6;
        let mut m = Camera::default();
        m.projection = projection;
        m.unit = Unit::Meters;
        m.target = mm.target.map(|x| x * 0.001);
        m.distance = 5.;
        m.radius = 20000.;
        let a = mm.view_projection_anchored(mm.target);
        let b = m.view_projection_anchored(m.target);
        let pa = a.transform_point(&Point::new(500., 250., 100.));
```

**Locate this block:**

```rust
            let mut camera = Camera::default();
            camera.unit = unit;
            camera.projection = projection;
            assert!(camera.view_projection().inverse().is_some());
```

Replace it with:

```rust
            let mut camera = Camera::default();
            fixed_plane(&mut camera, [0.; 3]);
            camera.unit = unit;
            camera.projection = projection;
            assert!(camera.view_projection().inverse().is_some());
```

### 7. `src/global_outline_tests.rs`

Use translated fixed planes and a declared dominant 20000-unit scene extent for controlled origin/pan comparisons; retain every existing pixel, original owner, source, upload and release assertion.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    ));
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

Replace it with:

```rust
    ));
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}

fn controlled_translation_plane(camera: &mut crate::camera::Camera, origin: [f64; 3]) {
    camera.grid_plane = Some(crate::cplane::CPlane::Frame {
        origin,
        x: [1., 0., 0.],
        y: [0., 1., 0.],
    });
    camera.radius = 20000.;
}
```

**Locate this block:**

```rust
            .is_none()
    );
    camera.target = anchor;
    assert!(
        gpu.prepare_origin(&camera, &far, None, &ids, 802.)
            .unwrap()
```

Replace it with:

```rust
            .is_none()
    );
    camera.target = anchor;
    controlled_translation_plane(&mut camera, anchor);
    assert!(
        gpu.prepare_origin(&camera, &far, None, &ids, 802.)
            .unwrap()
```

**Locate this block:**

```rust
    }
    gpu.set_scene(&far, None);
    camera.target = anchor;
    gpu.set_grid_on(
        settings,
        plane([anchor[0], anchor[1], anchor[2] - 0.75]),
```

Replace it with:

```rust
    }
    gpu.set_scene(&far, None);
    camera.target = anchor;
    controlled_translation_plane(&mut camera, anchor);
    gpu.set_grid_on(
        settings,
        plane([anchor[0], anchor[1], anchor[2] - 0.75]),
```

**Locate this block:**

```rust
        .unwrap()
        .to_proto();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 3.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
        .unwrap()
        .to_proto();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    controlled_translation_plane(&mut camera, [0.; 3]);
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 3.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
                .update(anchor, 3., 0.)
                .unwrap();
            camera.target = anchor;
            let distant = std::array::from_fn(|i| anchor[i] + pivot[i]);
            assert_eq!(
                crate::widget_layout::camera_scale(&camera, distant, 240.).unwrap(),
```

Replace it with:

```rust
                .update(anchor, 3., 0.)
                .unwrap();
            camera.target = anchor;
            controlled_translation_plane(&mut camera, anchor);
            let distant = std::array::from_fn(|i| anchor[i] + pivot[i]);
            assert_eq!(
                crate::widget_layout::camera_scale(&camera, distant, 240.).unwrap(),
```

**Locate this block:**

```rust
    ] {
        for density in [1u32, 2] {
            let mut camera = crate::camera_defaults_tests::top_fixture();
            camera.isometric();
            camera.projection = projection;
            camera.aspect = 4. / 3.;
```

Replace it with:

```rust
    ] {
        for density in [1u32, 2] {
            let mut camera = crate::camera_defaults_tests::top_fixture();
            controlled_translation_plane(&mut camera, [0.; 3]);
            camera.isometric();
            camera.projection = projection;
            camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
    gpu.set_source_ink(&far);
    gpu.set_clouds(&far, &[]).unwrap();
    camera.target = anchor;
    gpu.cloud_camera(&camera);
    let local = camera
        .view_projection_anchored(anchor)
```

Replace it with:

```rust
    gpu.set_source_ink(&far);
    gpu.set_clouds(&far, &[]).unwrap();
    camera.target = anchor;
    controlled_translation_plane(&mut camera, anchor);
    gpu.cloud_camera(&camera);
    let local = camera
        .view_projection_anchored(anchor)
```

**Locate this block:**

```rust
    ];
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 3.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
    ];
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    controlled_translation_plane(&mut camera, [0.; 3]);
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 3.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
        Some(&gpu.instances),
    );
    camera.target = anchor;
    let local = camera
        .view_projection_anchored(anchor)
        .to_f32();
```

Replace it with:

```rust
        Some(&gpu.instances),
    );
    camera.target = anchor;
    controlled_translation_plane(&mut camera, anchor);
    let local = camera
        .view_projection_anchored(anchor)
        .to_f32();
```

**Locate this block:**

```rust
        Some(&gpu.instances),
    );
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
        Some(&gpu.instances),
    );
    let mut camera = crate::camera_defaults_tests::top_fixture();
    controlled_translation_plane(&mut camera, [0.; 3]);
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
    let far_plane = plane([anchor[0], anchor[1], anchor[2] - 0.75]);
    gpu.set_grid_on(settings, far_plane).unwrap();
    camera.target = anchor;
    let local = camera
        .view_projection_anchored(anchor)
        .to_f32();
```

Replace it with:

```rust
    let far_plane = plane([anchor[0], anchor[1], anchor[2] - 0.75]);
    gpu.set_grid_on(settings, far_plane).unwrap();
    camera.target = anchor;
    controlled_translation_plane(&mut camera, anchor);
    let local = camera
        .view_projection_anchored(anchor)
        .to_f32();
```

**Locate this block:**

```rust
    gpu.set_grid_on(settings, plane([0., 0., -0.75]))
        .unwrap();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
    gpu.set_grid_on(settings, plane([0., 0., -0.75]))
        .unwrap();
    let mut camera = crate::camera_defaults_tests::top_fixture();
    controlled_translation_plane(&mut camera, [0.; 3]);
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
            .all(|p| p == &[255; 4])
    );
    camera.target = anchor;
    assert!(
        gpu.space
            .update(anchor, 2., 600.)
```

Replace it with:

```rust
            .all(|p| p == &[255; 4])
    );
    camera.target = anchor;
    controlled_translation_plane(&mut camera, anchor);
    assert!(
        gpu.space
            .update(anchor, 2., 600.)
```

**Locate this block:**

```rust
            .moved
    );
    camera.target = anchor;
    temporary_inputs(&mut gpu, anchor);
    let local = camera
        .view_projection_anchored(anchor)
```

Replace it with:

```rust
            .moved
    );
    camera.target = anchor;
    controlled_translation_plane(&mut camera, anchor);
    temporary_inputs(&mut gpu, anchor);
    let local = camera
        .view_projection_anchored(anchor)
```

**Locate this block:**

```rust
    let scene = Scene::empty();
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

Replace it with:

```rust
    let scene = Scene::empty();
    let mut gpu = renderer(&scene);
    let mut camera = crate::camera_defaults_tests::top_fixture();
    controlled_translation_plane(&mut camera, [0.; 3]);
    camera.projection = crate::camera::Projection::Orthographic;
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
```

**Locate this block:**

```rust
        uploads
    );
    far_camera.target = anchor;
    assert!(
        gpu.space
            .update(anchor, 2., 600.)
```

Replace it with:

```rust
        uploads
    );
    far_camera.target = anchor;
    controlled_translation_plane(&mut far_camera, anchor);
    assert!(
        gpu.space
            .update(anchor, 2., 600.)
```

**Locate this block:**

```rust
            .moved
    );
    let mut far_camera = crate::camera_defaults_tests::top_fixture();
    far_camera.distance = camera.distance;
    far_camera.aspect = camera.aspect;
    far_camera.projection = camera.projection;
    far_camera.target = anchor;
    gpu.cloud_camera(&far_camera);
    gpu.sheets
        .set_anchored(
```

Replace it with:

```rust
            .moved
    );
    let mut far_camera = crate::camera_defaults_tests::top_fixture();
    controlled_translation_plane(&mut far_camera, [0.; 3]);
    far_camera.distance = camera.distance;
    far_camera.aspect = camera.aspect;
    far_camera.projection = camera.projection;
    far_camera.target = anchor;
    controlled_translation_plane(&mut far_camera, anchor);
    gpu.cloud_camera(&far_camera);
    gpu.sheets
        .set_anchored(
```

**Locate this block:**

```rust
    let mut camera = crate::camera_defaults_tests::top_fixture();
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
    camera.projection = crate::camera::Projection::Orthographic;
```

Replace it with:

```rust
    let mut camera = crate::camera_defaults_tests::top_fixture();
    controlled_translation_plane(&mut camera, [0.; 3]);
    camera.distance = 2.;
    camera.aspect = 4. / 3.;
    camera.projection = crate::camera::Projection::Orthographic;
```

### 8. `src/grid_source_tests.rs`

Keep the deliberately narrow scene-only clipping positive control, and separately assert identical current hidden/shown grid matrices and every visible grid reference inside the far plane.

**Locate this existing block:**

```rust
        let original = (camera.target, camera.radius, camera.distance);
        let without = camera.view_projection();
        camera.grid = Some(Settings::default());
        let with = camera.view_projection();
        assert_eq!(
            (camera.target, camera.radius, camera.distance),
            original
        );
        let mut previously_clipped = 0;
        for point in Settings::default().points().unwrap() {
            let p = session_rust::Point::new(point[0], point[1], point[2]);
            let w = with.m[3] * point[0]
                + with.m[7] * point[1]
                + with.m[11] * point[2]
                + with.m[15];
            if w > 0.0 {
                let old = without.transform_point(&p);
                let new = with.transform_point(&p);
                if old[2] < 0.0 {
                    previously_clipped += 1;
                }
                assert!(
                    new[2] >= 0.0,
                    "An in-front grid reference cannot cross the far depth"
                );
            }
        }
        assert!(
            previously_clipped > 0,
            "This fixture must exercise grid clipping under a small-scene range"
        );
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let original = (camera.target, camera.radius, camera.distance);
        let hidden = camera.view_projection();
        let without = scene_only_depth_control(&camera);
        camera.grid = Some(Settings::default());
        let with = camera.view_projection();
        assert_eq!(
            (camera.target, camera.radius, camera.distance),
            original
        );
        assert_eq!(
            with.m, hidden.m,
            "Grid visibility cannot change the current camera range"
        );
        let mut previously_clipped = 0;
        for point in Settings::default().points().unwrap() {
            let p = session_rust::Point::new(point[0], point[1], point[2]);
            let w = with.m[3] * point[0]
                + with.m[7] * point[1]
                + with.m[11] * point[2]
                + with.m[15];
            if w > 0.0 {
                let old = without.transform_point(&p);
                let new = with.transform_point(&p);
                if old[2] < 0.0 {
                    previously_clipped += 1;
                }
                assert!(
                    new[2] >= 0.0,
                    "An in-front grid reference cannot cross the far depth"
                );
            }
        }
        assert!(
            previously_clipped > 0,
            "This fixture must exercise grid clipping under a small-scene range"
        );
    }
}

// A deliberately narrow scene-only interval is the positive clipping control.
fn scene_only_depth_control(camera: &Camera) -> session_rust::Xform {
    use session_rust::{Point, Vector, Xform};
    let forward = camera
        .orientation
        .rotate_vector(Vector::y_axis());
    let up = camera
        .orientation
        .rotate_vector(Vector::z_axis());
    let target = Point::new(
        camera.target[0],
        camera.target[1],
        camera.target[2],
    );
    let eye = &target - &(&forward * camera.distance);
    let view = Xform::look_at_right_handed(&eye, &target, &up);
    let range = camera.distance + 2. * camera.radius;
    let projection = match camera.projection {
        Projection::Perspective => Xform::perspective(
            std::f64::consts::PI / 3.,
            camera.aspect,
            (camera.distance * 10.).max(range),
            camera.distance * 1e-4,
        ),
        Projection::Orthographic => {
            let h = camera.distance * (std::f64::consts::PI / 6.).tan();
            let w = h * camera.aspect;
            Xform::orthographic(-w, w, -h, h, range, -range)
        }
    };
    projection * view
}
```

## <a id="checkpoint-96zzzzzze-grow-camera-depth-from-cached-world-bounds"></a>5. Grow camera depth from cached world bounds

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/bounds.rs, src/camera.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Displayed scene and text…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">monotonic camera extent</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">anchored frame</text>
</svg>
</div>

**What this code does:** Keep later or moved originals inside the depth range without refitting the view or walking their vertices on every camera frame.

### 1. `src/bounds.rs`

Share one numeric eight-corner traversal.

**Locate this existing block:**

```rust
    }

    pub fn centre(&self) -> [f64; 3] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn corners(&self) -> [[f64; 3]; 8] {
        std::array::from_fn(|corner| {
            std::array::from_fn(|axis| {
                if corner & (1 << axis) == 0 {
                    self.min[axis]
                } else {
                    self.max[axis]
                }
            })
        })
    }

    pub fn centre(&self) -> [f64; 3] {
```

### 2. `src/camera.rs`

Reuse the shared corners in the existing Fit owner. Grow only the depth extent, keeping the user’s pose.

**Locate this existing block:**

```rust
        let mut distance: f64 = 0.0;
        for corner in 0..8 {
            let p: [f64; 3] = std::array::from_fn(|i| {
                let coordinate = if corner & (1 << i) == 0 {
                    bounds.min[i]
                } else {
                    bounds.max[i]
                };
                coordinate - centre[i]
            });
            let dot = |axis: &Vector| (0..3).map(|i| p[i] * axis[i]).sum::<f64>();
            let z = dot(&forward);
            distance = distance.max(dot(&right).abs() / tx + z);
            distance = distance.max(dot(&up).abs() / ty + z);
        }
        self.target = centre;
        self.radius = radius;
        self.distance = (distance * 1.05).max(1.0e-6 / self.unit.to_meters());
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut distance: f64 = 0.0;
        for corner in bounds.corners() {
            let p: [f64; 3] = std::array::from_fn(|i| corner[i] - centre[i]);
            let dot = |axis: &Vector| (0..3).map(|i| p[i] * axis[i]).sum::<f64>();
            let z = dot(&forward);
            distance = distance.max(dot(&right).abs() / tx + z);
            distance = distance.max(dot(&up).abs() / ty + z);
        }
        self.target = centre;
        self.radius = radius;
        self.distance = (distance * 1.05).max(1.0e-6 / self.unit.to_meters());
    }

    pub fn grow_extent(&mut self, bounds: &crate::bounds::Bounds) {
        if bounds
            .min
            .into_iter()
            .chain(bounds.max)
            .any(|v| !v.is_finite())
            || (0..3).any(|i| bounds.min[i] > bounds.max[i])
        {
            return;
        }
        let extent = bounds
            .corners()
            .into_iter()
            .map(|p| {
                (0..3)
                    .map(|i| (p[i] - self.target[i]).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .fold(0., f64::max);
        if extent.is_finite() && extent > self.radius {
            self.radius = extent;
        }
    }
```

### 3. `src/renderer.rs`

Keep only six numeric bounds coordinates, with no Scene or source owner. Initialize the numeric cache before ordinary scene adoption. Refresh at displayed-scene adoption; camera-only growth needs no Scene. Refresh numeric bounds only when actual font metrics changed.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                                    h.max((line.line_top + line.line_height) as f64),
                                ]
                            });
                }
            }
        }
    }
```

Replace it with:

```rust
                                    h.max((line.line_top + line.line_height) as f64),
                                ]
                            });
                    if row.prepared.extent != extent {
                        row.prepared.extent = extent;
                        changed = true;
                    }
                }
            }
        }
        if changed {
            *self.world_bounds.borrow_mut() = scene.bounds();
        }
        changed
    }
```

**Locate this block:**

```rust
                    .iter()
                    .find(|run| run.label.id == row.id.value())
                {
                    row.prepared.extent =
                        run.buffer
                            .layout_runs()
                            .fold([0f64; 2], |[w, h], line| {
```

Replace it with:

```rust
                    .iter()
                    .find(|run| run.label.id == row.id.value())
                {
                    let extent =
                        run.buffer
                            .layout_runs()
                            .fold([0f64; 2], |[w, h], line| {
```

**Locate this block:**

```rust
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
    pub fn refresh_text_bounds(&self, scene: &mut Scene) {
        if let Some(lane) = self.text.borrow().as_ref() {
            for row in scene.texts_mut() {
                if let Some(run) = lane
```

Replace it with:

```rust
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
    pub fn refresh_text_bounds(&self, scene: &mut Scene) -> bool {
        let mut changed = false;
        if let Some(lane) = self.text.borrow().as_ref() {
            for row in scene.texts_mut() {
                if let Some(run) = lane
```

**Locate this block:**

```rust
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
    }

    pub fn camera_matrix(&self, camera: &crate::camera::Camera) -> [f32; 16] {
        camera
            .view_projection_anchored(self.anchor())
```

Replace it with:

```rust
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
        *self.world_bounds.borrow_mut() = scene.bounds();
    }

    pub fn grow_camera(&self, camera: &mut crate::camera::Camera) {
        if let Some(bounds) = self.world_bounds.borrow().as_ref() {
            camera.grow_extent(bounds);
        }
    }

    pub fn camera_matrix(&self, camera: &crate::camera::Camera) -> [f32; 16] {
        camera
            .view_projection_anchored(self.anchor())
```

**Locate this block:**

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            space: Default::default(),
            opacity: std::cell::Cell::new(1.),
            ambient_enabled: Default::default(),
```

Replace it with:

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            world_bounds: Default::default(),
            space: Default::default(),
            opacity: std::cell::Cell::new(1.),
            ambient_enabled: Default::default(),
```

**Locate this block:**

```rust
pub struct Renderer {
    space: crate::render_space::Origin,
    opacity: std::cell::Cell<f32>,
    ambient_enabled: std::cell::Cell<bool>,
```

Replace it with:

```rust
pub struct Renderer {
    world_bounds: std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space: crate::render_space::Origin,
    opacity: std::cell::Cell<f32>,
    ambient_enabled: std::cell::Cell<bool>,
```

### 4. `src/gizmo_edit.rs`

Access the active displayed preview for derived metrics without editing original source records.

**Locate this existing block:**

```rust
    }
    pub fn scenes(&self) -> [&Scene; 2] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn scene_mut(&mut self) -> &mut Scene {
        match self {
            Self::Whole(p) => &mut p.scene,
            Self::Part(p) => &mut p.scene,
        }
    }
    pub fn scenes(&self) -> [&Scene; 2] {
```

### 5. `src/editor.rs`

Match the existing display-scene precedence when refreshing derived text metrics.

**Locate this existing block:**

```rust
    }

    pub fn gizmo_origin(&self) -> Option<[f64; 3]> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn display_scene_mut(&mut self) -> &mut Scene {
        if let Some(scene) = self
            .trim
            .as_mut()
            .and_then(|t| t.preview.as_mut())
        {
            return scene;
        }
        if let Some(p) = self
            .canvas_drag
            .as_mut()
            .and_then(|d| d.preview.as_mut())
        {
            return &mut p.scene;
        }
        if let Some(g) = self.gizmo_edit.as_mut() {
            return g.preview.scene_mut();
        }
        if let Some(p) = self.transform_preview.as_mut() {
            return p.scene_mut();
        }
        if let Some(p) = self.edit_preview.as_mut() {
            return &mut p.scene;
        }
        &mut self.scene
    }

    pub fn gizmo_origin(&self) -> Option<[f64; 3]> {
```

### 6. `src/browser_rebase.rs`

Grow from cached world bounds before origin preparation.

**Locate this existing block:**

```rust
    renderer: &mut crate::renderer::Renderer,
    editor: &crate::editor::Editor,
) -> Result<crate::render_space::Rebase, JsValue> {
    renderer
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    renderer: &mut crate::renderer::Renderer,
    editor: &mut crate::editor::Editor,
) -> Result<crate::render_space::Rebase, JsValue> {
    renderer.grow_camera(&mut editor.camera);
    renderer
```

### 7. `src/browser.rs`

Pass the existing mutable editor to cached camera preparation. Move metric refresh before the final camera preparation. Refresh the actual displayed preview’s metrics before growing its depth.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            if let Err(error) = renderer.set_measurement(editor.mark.as_ref()) {
                panel.result(error);
            }
            renderer.refresh_text_bounds(&mut editor.scene);
            text_fonts.need(
```

Replace it with:

```rust
            if let Err(error) = renderer.set_measurement(editor.mark.as_ref()) {
                panel.result(error);
            }
            text_fonts.need(
```

**Locate this block:**

```rust
                    Err(error) => panel.result(&error),
                }
            }
            let origin_frame =
                match crate::browser_rebase::prepare(&mut renderer, &editor) {
                    Ok(frame) => frame,
                    Err(error) => {
                        report(&format!("Cannot prepare view: {error:?}"));
```

Replace it with:

```rust
                    Err(error) => panel.result(&error),
                }
            }
            renderer.refresh_text_bounds(editor.display_scene_mut());
            let origin_frame =
                match crate::browser_rebase::prepare(&mut renderer, &mut editor) {
                    Ok(frame) => frame,
                    Err(error) => {
                        report(&format!("Cannot prepare view: {error:?}"));
```

**Locate this block:**

```rust
                line
            };

            if let Err(error) = crate::browser_rebase::prepare(&mut renderer, &editor) {
                report(&format!("Cannot prepare view: {error:?}"));
                return;
            }
```

Replace it with:

```rust
                line
            };

            if let Err(error) =
                crate::browser_rebase::prepare(&mut renderer, &mut editor)
            {
                report(&format!("Cannot prepare view: {error:?}"));
                return;
            }
```

**Locate this block:**

```rust
        panel.update(None, &canvas)?;
        crate::browser_rebase::prepare(&mut renderer, &editor)?;
        canvas.set_attribute("data-origin-pending", "false")?;
        inspect(&editor, &renderer)?;
        let frame_started = crate::browser_phase::now();
```

Replace it with:

```rust
        panel.update(None, &canvas)?;
        crate::browser_rebase::prepare(&mut renderer, &mut editor)?;
        canvas.set_attribute("data-origin-pending", "false")?;
        inspect(&editor, &renderer)?;
        let frame_started = crate::browser_phase::now();
```

### 8. `src/camera_extent_tests.rs`

Supply independent current-camera, original GPU pixel/pick, cache ownership and preview metric controls.

Create the file and type:

```rust
use crate::{
    bounds::Bounds,
    camera::{Camera, Projection, Unit, View},
    prepared::PreparedMesh,
    scene::Scene,
};
use std::rc::Rc;
fn pose(camera: &Camera) -> Vec<f64> {
    let mut out = camera.target.to_vec();
    out.extend([camera.distance, camera.aspect]);
    out.extend(camera.frame);
    for axis in [
        session_rust::Vector::x_axis(),
        session_rust::Vector::y_axis(),
        session_rust::Vector::z_axis(),
    ] {
        let v = camera.orientation.rotate_vector(axis);
        out.extend((0..3).map(|i| v[i]));
    }
    out
}
#[test]
fn cached_extent_matches_current_growth_in_both_units_without_reframing() {
    let bounds = Bounds {
        min: [-7., -3., -11.],
        max: [5., 13., 17.],
    };
    let corners = bounds.corners();
    assert_eq!(
        corners
            .map(|p| p.map(f64::to_bits))
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .len(),
        8
    );
    assert!(corners.contains(&bounds.min) && corners.contains(&bounds.max));
    for unit in [Unit::Millimeters, Unit::Meters] {
        for physical_target in [[0.; 3], [10., -20., 30.], [1e6, -2e6, 3e6]] {
            let scale = unit.to_meters();
            let mut camera = Camera::default();
            camera.unit = unit;
            camera.target = physical_target.map(|v| v / scale);
            camera.radius = 0.;
            camera.frame = [-0.6, -0.8, 0.9, 0.7];
            let before = pose(&camera);
            let bounds = Bounds {
                min: bounds.min.map(|v| v / scale),
                max: bounds.max.map(|v| v / scale),
            };
            let mut current = crate::camera_reference_tests::Camera::new();
            current.unit = match unit {
                Unit::Millimeters => crate::camera_reference_tests::Unit::Millimeters,
                Unit::Meters => crate::camera_reference_tests::Unit::Meters,
            };
            current.target = physical_target;
            current.scene_extent = 0.;
            let points = bounds
                .corners()
                .map(|p| session_rust::Point::new(p[0], p[1], p[2]));
            let aabb = session_rust::AABB::from_points(&points, 0.);
            current.grow_extent(&aabb);
            camera.grow_extent(&bounds);
            assert!(
                (camera.radius * scale - current.scene_extent).abs()
                    <= current.scene_extent.max(1.) * 1e-12
            );
            assert_eq!(pose(&camera), before);
            let grown = camera.radius;
            camera.grow_extent(&Bounds::point(camera.target));
            assert_eq!(camera.radius, grown);
            for bad in [
                Bounds {
                    min: [f64::NAN, 0., 0.],
                    max: [1.; 3],
                },
                Bounds {
                    min: [0.; 3],
                    max: [f64::INFINITY; 3],
                },
                Bounds {
                    min: [2.; 3],
                    max: [1.; 3],
                },
            ] {
                camera.grow_extent(&bad);
                assert_eq!(camera.radius, grown);
            }
        }
    }
}
#[test]
fn numeric_bounds_outlive_originals_without_retaining_source_owners() {
    let mut scene = Scene::empty();
    let source = Rc::new(session_rust::Mesh::create_box(2., 4., 6.));
    let weak = Rc::downgrade(&source);
    let mut prepared = PreparedMesh::from_shared(source.clone()).unwrap();
    prepared.model = session_rust::Xform::translation(100., 200., -20000.);
    scene.insert(prepared).unwrap();
    let mut renderer = crate::cap_gpu_tests::renderer(&scene);
    let uploads = renderer.stats()[0];
    let expected = scene.bounds().unwrap();
    drop(scene);
    drop(source);
    assert!(
        weak.upgrade().is_none(),
        "Numeric bounds cannot keep an original source or Scene alive"
    );
    let mut camera = Camera::default();
    camera.radius = 0.;
    let before = pose(&camera);
    let mut current = Camera::default();
    current.radius = 0.;
    current.grow_extent(&expected);
    for _ in 0..1000 {
        renderer.grow_camera(&mut camera);
    }
    assert_eq!(camera.radius, current.radius);
    assert_eq!(pose(&camera), before);
    assert_eq!(renderer.stats()[0], uploads);
    renderer.set_scene(&Scene::empty(), None);
    let mut fresh = Camera::default();
    fresh.radius = 0.;
    renderer.grow_camera(&mut fresh);
    assert_eq!(fresh.radius, 0.);
}
#[test]
fn late_originals_expand_depth_before_real_pixels_and_original_gpu_picks() {
    let mut scene = Scene::empty();
    let mut renderer = crate::cap_gpu_tests::renderer(&scene);
    let mut camera = Camera::default();
    camera.set_view(View::Top);
    camera.projection = Projection::Orthographic;
    camera.aspect = 4. / 3.;
    camera.radius = 0.;
    let point = session_rust::Point::new(0., 0., -20000.);
    assert!(
        camera
            .view_projection()
            .transform_point(&point)[2]
            < 0.,
        "Positive control lies beyond the previous range"
    );
    let owner = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-500., -500., -20000.],
                    [500., -500., -20000.],
                    [0., 500., -20000.],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    renderer.set_scene(&scene, None);
    let uploads = renderer.stats()[0];
    let before = pose(&camera);
    renderer.grow_camera(&mut camera);
    let depth = camera
        .view_projection()
        .transform_point(&point)[2];
    assert!(depth > 0. && depth < 1.);
    assert_eq!(pose(&camera), before);
    let matrix = renderer.camera_matrix(&camera);
    let pixels = crate::cap_gpu_tests::pixels_at(&renderer, matrix);
    assert_ne!(
        &crate::cap_gpu_tests::at(&pixels, 160, 120)[..3],
        &[255; 3]
    );
    renderer
        .request_pick([0., 0.], matrix)
        .unwrap();
    crate::cap_gpu_tests::wait(&renderer);
    assert_eq!(
        renderer
            .take_pick(matrix)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
    assert_eq!(renderer.stats()[0], uploads);
    let extent = camera.radius;
    Rc::make_mut(&mut scene.view).hide(&[owner], true);
    renderer.set_scene(&scene, None);
    assert!(renderer.world_bounds.borrow().is_none());
    renderer.grow_camera(&mut camera);
    assert_eq!(camera.radius, extent);
    Rc::make_mut(&mut scene.view).show_all();
    scene
        .translate(owner, [1000., 0., 0.])
        .unwrap();
    renderer.set_scene(&scene, None);
    assert_eq!(
        renderer
            .world_bounds
            .borrow()
            .as_ref()
            .unwrap()
            .min[0],
        500.
    );
    renderer.grow_camera(&mut camera);
    assert!(camera.radius >= extent);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
    assert!((uploads..=uploads + 1).contains(&renderer.stats()[0]));
}
#[test]
fn active_preview_font_metrics_refresh_numeric_bounds_only_when_changed() {
    let mut editor = crate::editor::Editor::default();
    editor.scene.clear();
    let source = Rc::new(crate::scene_label::example(
        crate::text::TextPlacement::WorldPlane {
            world: [0.; 3],
            right: [1., 0., 0.],
            up: [0., 1., 0.],
            world_height: 100.,
        },
    ));
    let mut prepared = crate::scene_label::Prepared::new(source.clone()).unwrap();
    prepared.extent = [0.; 2];
    let owner = editor.scene.insert_text(prepared).unwrap();
    let mut preview =
        crate::edit_preview::Preview::new(&editor.scene, vec![owner]).unwrap();
    preview
        .set(session_rust::Xform::translation(
            100., 0., -20000.,
        ))
        .unwrap();
    editor.edit_preview = Some(preview);
    let mut renderer = crate::cap_gpu_tests::renderer(editor.display_scene());
    let uploads = renderer.stats()[0];
    assert!(renderer.refresh_text_bounds(editor.display_scene_mut()));
    assert_eq!(
        editor.scene.texts()[0].prepared.extent,
        [0.; 2]
    );
    assert!(
        editor.display_scene().texts()[0]
            .prepared
            .extent[0]
            > 0.
    );
    assert!(Rc::ptr_eq(
        &editor.display_scene().texts()[0]
            .prepared
            .source,
        &source
    ));
    let expected = editor.display_scene().bounds().unwrap();
    assert_eq!(
        renderer
            .world_bounds
            .borrow()
            .as_ref()
            .unwrap()
            .min,
        expected.min
    );
    assert_eq!(
        renderer
            .world_bounds
            .borrow()
            .as_ref()
            .unwrap()
            .max,
        expected.max
    );
    assert!(!renderer.refresh_text_bounds(editor.display_scene_mut()));
    assert_eq!(renderer.stats()[0], uploads);
    let mut camera = Camera::default();
    camera.radius = 0.;
    let before = pose(&camera);
    renderer.grow_camera(&mut camera);
    assert!(camera.radius > 19000.);
    assert_eq!(pose(&camera), before);
    editor.edit_preview = None;
    renderer.set_scene(&editor.scene, None);
    assert!(renderer.refresh_text_bounds(editor.display_scene_mut()));
    assert!(!renderer.refresh_text_bounds(editor.display_scene_mut()));
    assert!(editor.scene.texts()[0].prepared.extent[0] > 0.);
    assert!(
        renderer
            .world_bounds
            .borrow()
            .as_ref()
            .unwrap()
            .min[2]
            > -1.
    );
}
```

### 9. `src/renderer.rs`

Register optional native validation outside the required browser graph.

**Locate this existing block:**

```rust
#[path = "opacity_gpu_tests.rs"]
mod opacity_gpu_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "opacity_gpu_tests.rs"]
mod opacity_gpu_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "camera_extent_tests.rs"]
mod camera_extent_tests;
```

## <a id="checkpoint-96zzzzzzf-match-wheel-input-and-framed-projection"></a>6. Match wheel input and framed projection

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/navigation.rs, src/edit…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Raw wheel event and actua…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing precise zoom own…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/camera.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing Fit</text>
</svg>
</div>

**What this code does:** Use current wheel units and preserve the visible region when changing from orthographic to perspective.

### 1. `src/navigation.rs`

Map raw wheel domains through the current finite amount and double-precision factor.

**Locate this existing block:**

```rust
use crate::editor::Action;

pub fn wheel(delta: f64, mode: u32, height: f64) -> Option<Action> {
    if !delta.is_finite() || delta == 0.0 || !height.is_finite() || height <= 0.0 {
        return None;
    }
    let unit = match mode {
        0 => 1.0,
        1 => 16.0,
        2 => height,
        _ => return None,
    };
    let pixels = (delta * unit).clamp(-1_000.0, 1_000.0);
    Some(Action::Zoom(
        0.9_f64.powf(pixels / 100.0) as f32
    ))
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::editor::Action;

pub fn wheel(delta: f64, mode: u32, dpr: f64) -> Option<Action> {
    let amount = match mode {
        0 if dpr.is_finite() && dpr > 0. => (-delta * dpr) as f32 / 100.,
        1 => (-delta) as f32,
        _ => return None,
    };
    if !amount.is_finite() || amount == 0. {
        return None;
    }
    Some(Action::Zoom(
        1. / 0.9_f64.powf(amount.clamp(-10., 10.) as f64),
    ))
```

### 2. `src/editor.rs`

Retain the existing zoom-at action with a precise factor. Share the same precise factor with ordinary zoom.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
```

Replace it with:

```rust
    Undo,
    Redo,
    Background,
    Zoom(f64),
    Pan(f32, f32),
```

**Locate this block:**

```rust
    ZoomAt {
        factor: f32,
        screen: [f32; 2],
    },
    Touch(Vec<Action>),
```

Replace it with:

```rust
    ZoomAt {
        factor: f64,
        screen: [f32; 2],
    },
    Touch(Vec<Action>),
```

### 3. `src/camera.rs`

Keep double precision to the distance update. The factor is already precise. Reuse the same factor for cursor anchoring.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
    }

    pub fn zoom_at(&mut self, factor: f32, screen: [f32; 2]) {
        if screen
```

Replace it with:

```rust
        }
    }

    pub fn zoom_at(&mut self, factor: f64, screen: [f32; 2]) {
        if screen
```

**Locate this block:**

```rust
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            let scale = self.unit.to_meters();
            self.distance =
                (self.distance / factor as f64).clamp(1.0e-6 / scale, 1.0e15 / scale);
        }
    }
```

Replace it with:

```rust
    }

    pub fn zoom(&mut self, factor: f64) {
        if factor.is_finite() && factor > 0.0 {
            let scale = self.unit.to_meters();
            self.distance =
                (self.distance / factor).clamp(1.0e-6 / scale, 1.0e15 / scale);
        }
    }
```

### 4. `src/touch.rs`

Let the existing pinch ratio use the precise action boundary.

**Locate this existing block:**

```rust
                actions.push(Action::ZoomAt {
                    factor: (last / first).clamp(0.1, 10.) as f32,
                    screen,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                actions.push(Action::ZoomAt {
                    factor: (last / first).clamp(0.1, 10.),
                    screen,
```

### 5. `src/browser.rs`

Use actual window density rather than canvas size or render-density caps.

**Locate this existing block:**

```rust
            let rect = canvas.get_bounding_client_rect();
            let Action::Zoom(factor) =
                crate::navigation::wheel(wheel.delta_y(), mode, rect.height())?
            else {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            let rect = canvas.get_bounding_client_rect();
            let Action::Zoom(factor) = crate::navigation::wheel(
                wheel.delta_y(),
                mode,
                web_sys::window()?.device_pixel_ratio(),
            )?
            else {
```

### 6. `src/bounds.rs`

Share the finite ordered-box guard.

**Locate this existing block:**

```rust
    }

    pub fn corners(&self) -> [[f64; 3]; 8] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn valid(&self) -> bool {
        self.min
            .into_iter()
            .chain(self.max)
            .all(f64::is_finite)
            && (0..3).all(|i| self.min[i] <= self.max[i])
    }

    pub fn corners(&self) -> [[f64; 3]; 8] {
```

### 7. `src/camera.rs`

Reuse the same guard in extent growth. Frame perspective from the clipped visible corners while preserving their forward depth.

**Locate this existing block:**

```rust
    }

    pub fn grow_extent(&mut self, bounds: &crate::bounds::Bounds) {
        if bounds
            .min
            .into_iter()
            .chain(bounds.max)
            .any(|v| !v.is_finite())
            || (0..3).any(|i| bounds.min[i] > bounds.max[i])
        {
            return;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn set_projection_framed(
        &mut self,
        mode: Projection,
        bounds: Option<&crate::bounds::Bounds>,
    ) {
        if self.projection == mode {
            return;
        }
        self.projection = mode;
        let Some(bounds) =
            bounds.filter(|b| mode == Projection::Perspective && b.valid())
        else {
            return;
        };
        let right = self
            .orientation
            .rotate_vector(Vector::x_axis());
        let up = self
            .orientation
            .rotate_vector(Vector::z_axis());
        let forward = self
            .orientation
            .rotate_vector(Vector::y_axis());
        let h = self.distance * HALF_FOV.tan();
        let w = h * self.aspect;
        let shift = self.shift();
        let mut clipped = None;
        for corner in bounds.corners() {
            let p: [f64; 3] = std::array::from_fn(|i| corner[i] - self.target[i]);
            let dot = |axis: &Vector| (0..3).map(|i| p[i] * axis[i]).sum::<f64>();
            let x = dot(&right).clamp((-1. - shift[0]) * w, (1. - shift[0]) * w);
            let y = dot(&up).clamp((-1. - shift[1]) * h, (1. - shift[1]) * h);
            let z = dot(&forward);
            let point = std::array::from_fn(|i| {
                self.target[i] + x * right[i] + y * up[i] + z * forward[i]
            });
            if let Some(bounds) = &mut clipped {
                crate::bounds::Bounds::include(bounds, point);
            } else {
                clipped = Some(crate::bounds::Bounds::point(point));
            }
        }
        self.fit(&clipped.unwrap());
    }

    pub fn grow_extent(&mut self, bounds: &crate::bounds::Bounds) {
        if !bounds.valid() {
            return;
```

### 8. `src/editor.rs`

Only an actual transition into perspective needs a world-bounds walk and framing.

**Locate this existing block:**

```rust
                    Action::NamedView(view) => self.camera.set_view(view),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    Action::NamedView(view) => self.camera.set_view(view),
                    Action::Projection(mode) => {
                        if self.camera.projection != mode {
                            let bounds = (mode
                                == crate::camera::Projection::Perspective)
                                .then(|| self.display_scene().bounds())
                                .flatten();
                            self.camera
                                .set_projection_framed(mode, bounds.as_ref());
                        }
                    }
                    Action::Fit => {
```

### 9. `src/navigation_tests.rs`

Declare current pixel/line/page units and finite conversion while retaining accumulation, roundtrip, jump and history controls.

**Locate this existing block:**

```rust
        editor
            .apply(wheel(delta, mode, 480.0).unwrap())
            .unwrap(),
        Change::View
    );
    editor.camera.distance
}

#[test]
fn wheel_units_agree_and_small_movements_accumulate() {
    assert!((distance(48.0, 0) - distance(3.0, 1)).abs() < 1.0e-6);
    assert!((distance(48.0, 0) - distance(0.1, 2)).abs() < 1.0e-6);
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    for _ in 0..10 {
        editor
            .apply(wheel(-12.0, 0, 480.0).unwrap())
            .unwrap();
    }
    assert!((editor.camera.distance - distance(-120.0, 0)).abs() < 1.0e-5);
    editor
        .apply(wheel(120.0, 0, 480.0).unwrap())
        .unwrap();
    assert!((editor.camera.distance - 3.0).abs() < 1.0e-5);
}

#[test]
fn wheel_rejects_invalid_input_and_bounds_large_jumps() {
    for delta in [0.0, f64::NAN, f64::INFINITY] {
        assert!(wheel(delta, 0, 480.0).is_none());
    }
    for height in [0.0, -1.0, f64::NAN] {
        assert!(wheel(10.0, 0, height).is_none());
    }
    assert!(wheel(10.0, 99, 480.0).is_none());
    assert_eq!(distance(f64::MAX, 2), distance(1_000.0, 0));
    assert_eq!(distance(-f64::MAX, 2), distance(-1_000.0, 0));
}

#[test]
fn navigation_preserves_document_history() {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor.apply(Action::AddBox).unwrap();
    editor
        .apply(wheel(-120.0, 0, 480.0).unwrap())
        .unwrap();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        editor
            .apply(wheel(delta, mode, 1.0).unwrap())
            .unwrap(),
        Change::View
    );
    editor.camera.distance
}

#[test]
fn wheel_units_agree_and_small_movements_accumulate() {
    assert!((distance(300.0, 0) - distance(3.0, 1)).abs() < 1.0e-6);
    assert!(wheel(0.1, 2, 1.0).is_none());
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    for _ in 0..10 {
        editor
            .apply(wheel(-12.0, 0, 1.0).unwrap())
            .unwrap();
    }
    assert!((editor.camera.distance - distance(-120.0, 0)).abs() < 1.0e-5);
    editor
        .apply(wheel(120.0, 0, 1.0).unwrap())
        .unwrap();
    assert!((editor.camera.distance - 3.0).abs() < 1.0e-5);
}

#[test]
fn wheel_rejects_invalid_input_and_bounds_large_jumps() {
    for delta in [0.0, f64::NAN, f64::INFINITY] {
        assert!(wheel(delta, 0, 1.0).is_none());
    }
    for height in [0.0, -1.0, f64::NAN] {
        assert!(wheel(10.0, 0, height).is_none());
    }
    assert!(wheel(10.0, 99, 1.0).is_none());
    assert!(wheel(f64::MAX, 0, 1.0).is_none());
    assert_eq!(distance(1e9, 0), distance(1_000.0, 0));
    assert!(wheel(-f64::MAX, 0, 1.0).is_none());
    assert_eq!(distance(-1e9, 0), distance(-1_000.0, 0));
}

#[test]
fn navigation_preserves_document_history() {
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    editor.apply(Action::AddBox).unwrap();
    editor
        .apply(wheel(-120.0, 0, 1.0).unwrap())
        .unwrap();
```

### 10. `src/projection_tests.rs`

Expect current perspective framing, then restore the declared close-pick pose while retaining all original owner and history assertions.

**Locate this existing block:**

```rust
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    let before = editor.camera.uniform();
    assert_eq!(
        editor
            .apply(Action::Projection(Projection::Orthographic))
            .unwrap(),
        Change::View
    );
    editor
        .apply(Action::Projection(Projection::Perspective))
        .unwrap();
    assert_eq!(editor.camera.uniform(), before);
    editor
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut editor = crate::camera_defaults_tests::editor_fixture();
    let mut fitted = crate::camera_defaults_tests::top_fixture();
    fitted.fit(&editor.scene.bounds().unwrap());
    assert_eq!(
        editor
            .apply(Action::Projection(Projection::Orthographic))
            .unwrap(),
        Change::View
    );
    editor
        .apply(Action::Projection(Projection::Perspective))
        .unwrap();
    assert_eq!(editor.camera.uniform(), fitted.uniform());
    editor.camera = crate::camera_defaults_tests::top_fixture();
    editor
```

### 11. `src/camera_navigation_tests.rs`

Supply independent current wheel/cursor and clipped-corner framing comparisons, units, original source and mode guards.

Create the file and type:

```rust
use crate::{
    bounds::Bounds,
    camera::{Camera, Projection, Unit},
    camera_reference_tests as current,
    editor::{Action, Editor},
};
use session_rust::{AABB, Point};
fn reference(camera: &Camera) -> current::Camera {
    let scale = camera.unit.to_meters();
    let mut out = current::Camera::new();
    out.unit = match camera.unit {
        Unit::Millimeters => current::Unit::Millimeters,
        Unit::Meters => current::Unit::Meters,
    };
    out.target = camera.target.map(|v| v * scale);
    out.distance = camera.distance * scale;
    out.scene_extent = camera.radius * scale;
    out.orientation = camera.orientation.duplicate();
    out.frame = camera.frame;
    out.perspective = camera.projection == Projection::Perspective;
    out.grid = camera.grid_plane.map(|p| {
        let origin = p.origin();
        std::array::from_fn(|i| origin[i])
    });
    out.update_position();
    out
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= a.abs().max(b.abs()).max(1.) * 1e-10,
        "{a} != {b}"
    );
}
#[test]
fn physical_wheel_modes_match_current_distance_in_both_units() {
    let cases = [
        (100., 0, 1., -1f32),
        (100., 0, 2., -2.),
        (100., 0, 3., -3.),
        (1., 1, 3., -1.),
        (-0.125, 1, 2., 0.125),
        (1e9, 0, 1., -1e7),
        (-1e9, 0, 3., 3e7),
        (0.125, 0, 2., -0.0025),
    ];
    for unit in [Unit::Millimeters, Unit::Meters] {
        for (delta, mode, dpr, amount) in cases {
            let mut actual = Camera::default();
            actual.unit = unit;
            actual.distance = 3. / unit.to_meters();
            let mut expected = reference(&actual);
            let Some(Action::Zoom(factor)) = crate::navigation::wheel(delta, mode, dpr)
            else {
                panic!("Valid current wheel domain");
            };
            actual.zoom(factor);
            expected.zoom(amount);
            close(
                actual.distance * unit.to_meters(),
                expected.distance,
            );
        }
    }
    for mode in [0, 1, 2, 99] {
        for delta in [
            f64::NAN,
            f64::INFINITY,
            -f64::INFINITY,
            f64::MAX,
            -f64::MAX,
        ] {
            assert!(crate::navigation::wheel(delta, mode, 2.).is_none());
        }
    }
    for mode in [2, 99] {
        assert!(crate::navigation::wheel(100., mode, 2.).is_none());
    }
    assert!(crate::navigation::wheel(100., 0, 0.).is_none());
}
#[test]
fn zoom_at_matches_current_cursor_anchor_and_preserves_originals() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            for dpr in [1., 2., 3.] {
                let mut editor = Editor::default();
                editor.camera.unit = unit;
                editor.camera.projection = projection;
                editor.camera.distance = 3. / unit.to_meters();
                editor.camera.aspect = 1.5;
                editor.camera.frame = [0.1, 0.05, 0.9, 0.85];
                let originals = editor
                    .scene
                    .objects()
                    .iter()
                    .map(|r| {
                        (
                            r.id,
                            r.geometry().unwrap().to_proto(),
                            r.model.m,
                        )
                    })
                    .collect::<Vec<_>>();
                let mut expected = reference(&editor.camera);
                let Some(Action::Zoom(factor)) = crate::navigation::wheel(100., 0, dpr)
                else {
                    panic!()
                };
                editor
                    .apply(Action::ZoomAt {
                        factor,
                        screen: [0.25, -0.125],
                    })
                    .unwrap();
                expected.zoom_at(-(dpr as f32), (600., 360.), (960., 640.));
                close(
                    editor.camera.distance * unit.to_meters(),
                    expected.distance,
                );
                for i in 0..3 {
                    close(
                        editor.camera.target[i] * unit.to_meters(),
                        expected.target[i],
                    );
                }
                assert_eq!(
                    editor
                        .scene
                        .objects()
                        .iter()
                        .map(|r| (
                            r.id,
                            r.geometry().unwrap().to_proto(),
                            r.model.m
                        ))
                        .collect::<Vec<_>>(),
                    originals
                );
            }
        }
    }
}
#[test]
fn framed_projection_matches_current_clipped_corners_and_mode_guards() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        for aspect in [0.4, 1.5, 3.] {
            for frame in [[0., 0., 1., 1.], [0.15, 0.1, 0.8, 0.95]] {
                for target in [[0.; 3], [10., -20., 30.]] {
                    let scale = unit.to_meters();
                    let mut actual = Camera::default();
                    actual.unit = unit;
                    actual.distance = 3. / scale;
                    actual.target = target.map(|v| v / scale);
                    actual.aspect = aspect;
                    actual.frame = frame;
                    actual.projection = Projection::Orthographic;
                    let bounds = Bounds {
                        min: std::array::from_fn(|i| {
                            (target[i] + [-20., -10., -5.][i]) / scale
                        }),
                        max: std::array::from_fn(|i| {
                            (target[i] + [30., 15., 12.][i]) / scale
                        }),
                    };
                    let points = bounds
                        .corners()
                        .map(|p| Point::new(p[0], p[1], p[2]));
                    let aabb = AABB::from_points(&points, 0.);
                    let mut expected = reference(&actual);
                    expected.toggle_projection_framed(&aabb, aspect);
                    actual
                        .set_projection_framed(Projection::Perspective, Some(&bounds));
                    expected.grow_extent(&aabb);
                    actual.grow_extent(&bounds);
                    close(actual.distance * scale, expected.distance);
                    for i in 0..3 {
                        close(actual.target[i] * scale, expected.target[i]);
                    }
                    close(actual.radius * scale, expected.scene_extent);
                    let anchor = Point::new(
                        actual.target[0],
                        actual.target[1],
                        actual.target[2],
                    );
                    let a = actual.view_projection_anchored(actual.target);
                    let b = expected.view_proj_anchored(aspect, &anchor);
                    for i in 0..16 {
                        close(a.m[i] / a.m[15], b.m[i] / b.m[15]);
                    }
                    let before = (
                        actual.target,
                        actual.distance,
                        actual.radius,
                        actual.frame,
                    );
                    actual.set_projection_framed(
                        Projection::Perspective,
                        Some(&Bounds::point([1e12; 3])),
                    );
                    assert_eq!(
                        (
                            actual.target,
                            actual.distance,
                            actual.radius,
                            actual.frame
                        ),
                        before
                    );
                    actual
                        .set_projection_framed(Projection::Orthographic, Some(&bounds));
                    assert_eq!(actual.projection, Projection::Orthographic);
                    assert_eq!(
                        (
                            actual.target,
                            actual.distance,
                            actual.radius,
                            actual.frame
                        ),
                        before
                    );
                }
            }
        }
    }
}
```

### 12. `src/lib.rs`

Register optional current-camera navigation validation outside the browser graph.

**Locate this existing block:**

```rust
#[cfg(test)]
mod camera_depth_reach_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod camera_depth_reach_tests;

#[cfg(test)]
mod camera_navigation_tests;
```

### 13. `src/fit_tests.rs`

Expect the precise factor now carried by the existing camera API; retain every original view, corner, cursor and tight tolerance assertion.

**Locate this existing block:**

```rust
            camera.zoom(1.01);
            assert!(
                (camera.distance - distance / 1.01_f32 as f64).abs()
                    < distance * 1.0e-10
            );
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            camera.zoom(1.01);
            assert!((camera.distance - distance / 1.01_f64).abs() < distance * 1.0e-10);
        }
```

### 14. `src/navigation_complete_tests.rs`

Expect the precise factor now carried by the existing camera API; retain every original view, corner, cursor and tight tolerance assertion.

**Locate this existing block:**

```rust
        );
        assert!((camera.distance - 3. / 1.4_f32 as f64).abs() < 1e-9);
        let saved = camera.view_projection().m;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        );
        assert!((camera.distance - 3. / 1.4_f64).abs() < 1e-9);
        let saved = camera.view_projection().m;
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 21.5
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 21.5
npm --prefix ../session_tests run course -- save 21.5
```

Continue with [22.1 · Original surface colours and lighting controls](96zzzzzzg-share-current-surface-lighting-and-camera-uniforms.md).
