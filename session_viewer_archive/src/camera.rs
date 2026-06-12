use session_rust::quaternion::Quaternion;
use session_rust::xform::Xform;
use session_rust::{Point, Vector};
use session_rust::tolerance::Tolerance;
use winit::event::{MouseButton, MouseScrollDelta};
use winit::keyboard::{KeyCode, ModifiersState};

const MIN_ZOOM:       f32 = 0.01;
const MAX_ZOOM:       f32 = 10000.0;
const MM_TO_UNIT:     f32 = 0.001; // session geometry is in mm; viewer unit = 1 m

// ============================================================

#[derive(Clone, Copy, PartialEq)]
pub enum ProjMode { Perspective, Ortho }

#[derive(Clone, Copy)]
pub enum NamedView { Top, Bottom, Left, Right }

// ============================================================
// CAMERA
// Z-up turntable orbit (Blender/Maya standard).
// Orientation stored as quaternion to avoid gimbal lock.
// Reference right vector tracked per-frame for stable pole handling.
// All session geometry is in millimetres; MM_TO_UNIT (0.001) is baked
// into view_proj so 1 viewer unit = 1 m.
// ============================================================
pub struct Camera {
    pub position:   [f32; 3],
    pub target:     [f32; 3],
    pub up:         [f32; 3],
    pub distance:   f32,
    pub orientation: Quaternion,
    pub world_up:   [f32; 3],
    pub last_right: [f32; 3],
    pub aspect:     f32,
    pub proj_mode:  ProjMode,
    pub ortho_scale: f32,  // half-height of ortho frustum in viewer units
    /// Off-center projection shift in NDC: centers the perspective inside the
    /// VISIBLE viewport (window minus egui panels) instead of the full window.
    /// Baked into view_proj/proj_matrix/proj_and_inverse, so rendering, SSAO,
    /// picking (generic inverse) and frustum culling all stay consistent.
    pub ndc_offset: [f32; 2],
    initial_target:      [f32; 3],
    initial_orientation: Quaternion,
    initial_distance:    f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        let world_up = [0.0_f32, 0.0, 1.0];
        let target   = [0.0_f32, 0.0, 0.0];
        let distance = 3.0_f32;

        let yaw_q = Quaternion::from_axis_angle(
            Vector::new(0.0, 0.0, 1.0),
            Tolerance::PI / 4.0,
        );
        let rv = yaw_q.rotate_vector(Vector::new(1.0, 0.0, 0.0));
        let pitch_q = Quaternion::from_axis_angle(
            Vector::new(rv[0], rv[1], rv[2]),
            -(Tolerance::PI / 6.0),
        );
        let orientation = (pitch_q * yaw_q).normalized();

        let offset = orientation.rotate_vector(Vector::new(0.0, -distance as f64, 0.0));
        let fwd = Vector::new(-offset[0], -offset[1], -offset[2]).normalized();
        let wu  = Vector::new(world_up[0] as f64, world_up[1] as f64, world_up[2] as f64);
        let r0  = fwd.cross(&wu).normalized();
        let last_right = [r0[0] as f32, r0[1] as f32, r0[2] as f32];

        let initial_orientation = orientation.duplicate();

        let mut cam = Self {
            position: [0.0; 3],
            target,
            up: [0.0, 0.0, 1.0],
            distance,
            orientation,
            world_up,
            last_right,
            aspect,
            proj_mode:  ProjMode::Perspective,
            ortho_scale: distance,
            ndc_offset: [0.0, 0.0],
            initial_target:      target,
            initial_orientation,
            initial_distance:    distance,
        };
        cam.update_position();
        cam
    }

    pub fn update_position(&mut self) {
        let offset = self.orientation.rotate_vector(
            Vector::new(0.0, -self.distance as f64, 0.0)
        );
        self.position = [
            self.target[0] + offset[0] as f32,
            self.target[1] + offset[1] as f32,
            self.target[2] + offset[2] as f32,
        ];

        let d = self.distance.max(1e-10);
        let fwd = Vector::new(-offset[0] / d as f64, -offset[1] / d as f64, -offset[2] / d as f64);
        let wu  = Vector::new(self.world_up[0] as f64, self.world_up[1] as f64, self.world_up[2] as f64);

        let alignment = fwd.dot(&wu).abs();
        let right = if alignment > 0.98 {
            Vector::new(self.last_right[0] as f64, self.last_right[1] as f64, self.last_right[2] as f64)
        } else {
            let computed = fwd.cross(&wu).normalized();
            let last = Vector::new(self.last_right[0] as f64, self.last_right[1] as f64, self.last_right[2] as f64);
            if computed.dot(&last) < 0.0 { computed * -1.0 } else { computed }
        };
        self.last_right = [right[0] as f32, right[1] as f32, right[2] as f32];

        let up = right.cross(&fwd).normalized();
        self.up = [up[0] as f32, up[1] as f32, up[2] as f32];
    }

    // Build view-projection matrix for the GPU (column-major [[f32;4];4]).
    //
    // All session geometry is in millimetres. MM_TO_UNIT (0.001) is baked into
    // the view matrix so 1 viewer unit = 1 m = 1000 mm. near/far below are in
    // viewer units after that scale.
    //
    // Depth buffer precision (24-bit, ~16M discrete values):
    //   Perspective — logarithmic distribution; precision clusters near the camera.
    //     far/near ratio must stay ≤ ~100 000 or z-fighting appears.
    //     Adaptive near = distance × 0.001 keeps the ratio bounded as zoom changes.
    //   Ortho — linear (uniform) distribution; ratio is irrelevant, large ranges are
    //     fine. Near must be NEGATIVE so geometry behind the camera is not clipped
    //     when orbiting near-horizontal (opposite-side grid lines are behind camera).
    pub fn view_proj(&self) -> [[f32; 4]; 4] {
        let eye   = Point::new(self.position[0] as f64, self.position[1] as f64, self.position[2] as f64);
        let tgt   = Point::new(self.target[0] as f64,   self.target[1] as f64,   self.target[2] as f64);
        let up    = Vector::new(self.up[0] as f64,       self.up[1] as f64,       self.up[2] as f64);
        let view  = Xform::look_at_right_handed(&eye, &tgt, &up);
        let scale = Xform::scale_xyz(MM_TO_UNIT as f64, MM_TO_UNIT as f64, MM_TO_UNIT as f64);
        let proj  = match self.proj_mode {
            ProjMode::Perspective => {
                // near = distance × 0.001 → far/near ≤ 100 000 at any zoom level.
                // far = 100 000 viewer units = 100 km worth of mm.
                let near = (self.distance * 0.001_f32).max(0.0001_f32);
                Xform::perspective(Tolerance::PI / 3.0, self.aspect as f64, near as f64, 100_000.0)
            }
            ProjMode::Ortho => {
                // Depth range adaptive to zoom/orbit, not a fixed ±100 km. Ortho depth is
                // linear, so a tight range is what gives precision: at ±100 000 the 24-bit
                // buffer quantises to ~12 mm and coplanar faces/edges z-fight (jaggy edges).
                // 8 × max(ortho_scale, distance) keeps the whole scene in view (grid ±36,
                // geometry sits near view-z ≈ −distance) while raising precision ~1000×.
                // Symmetric (negative near) still exposes far-side geometry past horizontal.
                let h = self.ortho_scale;
                let w = h * self.aspect;
                let depth = (self.ortho_scale.max(self.distance) * 8.0).max(1.0);
                Xform::orthographic(-w as f64, w as f64, -h as f64, h as f64, -depth as f64, depth as f64)
            }
        };
        let view_scaled = &view * &scale;
        let mut m = (&proj * &view_scaled).to_cols();
        self.apply_ndc_offset(&mut m);
        [
            [m[0][0] as f32, m[0][1] as f32, m[0][2] as f32, m[0][3] as f32],
            [m[1][0] as f32, m[1][1] as f32, m[1][2] as f32, m[1][3] as f32],
            [m[2][0] as f32, m[2][1] as f32, m[2][2] as f32, m[2][3] as f32],
            [m[3][0] as f32, m[3][1] as f32, m[3][2] as f32, m[3][3] as f32],
        ]
    }

    /// Post-multiply an NDC translation onto a projection-like matrix (column-major
    /// f64 cols): x' = x + dx·w, y' = y + dy·w.
    fn apply_ndc_offset(&self, m: &mut [[f64; 4]; 4]) {
        let dx = self.ndc_offset[0] as f64;
        let dy = self.ndc_offset[1] as f64;
        if dx == 0.0 && dy == 0.0 { return; }
        for c in 0..4 {
            m[c][0] += dx * m[c][3];
            m[c][1] += dy * m[c][3];
        }
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let eye   = Point::new(self.position[0] as f64, self.position[1] as f64, self.position[2] as f64);
        let tgt   = Point::new(self.target[0] as f64,   self.target[1] as f64,   self.target[2] as f64);
        let up    = Vector::new(self.up[0] as f64,       self.up[1] as f64,       self.up[2] as f64);
        let view  = Xform::look_at_right_handed(&eye, &tgt, &up);
        let scale = Xform::scale_xyz(MM_TO_UNIT as f64, MM_TO_UNIT as f64, MM_TO_UNIT as f64);
        let m = (&view * &scale).to_cols();
        [
            [m[0][0] as f32, m[0][1] as f32, m[0][2] as f32, m[0][3] as f32],
            [m[1][0] as f32, m[1][1] as f32, m[1][2] as f32, m[1][3] as f32],
            [m[2][0] as f32, m[2][1] as f32, m[2][2] as f32, m[2][3] as f32],
            [m[3][0] as f32, m[3][1] as f32, m[3][2] as f32, m[3][3] as f32],
        ]
    }

    pub fn pick_radius_mm(&self, viewport_h: f32, pixel_radius: f32) -> f32 {
        match self.proj_mode {
            ProjMode::Perspective => {
                let dist_mm   = self.distance * 1000.0;
                let tan_half  = (std::f32::consts::PI / 6.0).tan();
                dist_mm * tan_half * 2.0 / viewport_h * pixel_radius
            }
            ProjMode::Ortho => {
                self.ortho_scale * 1000.0 * 2.0 / viewport_h * pixel_radius
            }
        }
    }

    pub fn proj_matrix(&self) -> [[f32; 4]; 4] {
        let mut m = match self.proj_mode {
            ProjMode::Perspective => {
                let near = (self.distance * 0.001_f32).max(0.0001_f32);
                Xform::perspective(Tolerance::PI / 3.0, self.aspect as f64, near as f64, 100_000.0).to_cols()
            }
            ProjMode::Ortho => {
                let h = self.ortho_scale;
                let w = h * self.aspect;
                let depth = (self.ortho_scale.max(self.distance) * 8.0).max(1.0);
                Xform::orthographic(-w as f64, w as f64, -h as f64, h as f64, -depth as f64, depth as f64).to_cols()
            }
        };
        self.apply_ndc_offset(&mut m);
        [
            [m[0][0] as f32, m[0][1] as f32, m[0][2] as f32, m[0][3] as f32],
            [m[1][0] as f32, m[1][1] as f32, m[1][2] as f32, m[1][3] as f32],
            [m[2][0] as f32, m[2][1] as f32, m[2][2] as f32, m[2][3] as f32],
            [m[3][0] as f32, m[3][1] as f32, m[3][2] as f32, m[3][3] as f32],
        ]
    }

    /// Projection matrix and its analytic inverse, both column-major f32, built
    /// in f64. NOTE: `Xform::inverse()` is affine-only (3x3 + translation) — it
    /// silently returns garbage for a projective perspective matrix (the m[11]=-1
    /// w-row is ignored), so the inverses are written in closed form here.
    pub fn proj_and_inverse(&self) -> ([[f32; 4]; 4], [[f32; 4]; 4]) {
        let c = |m: [[f64; 4]; 4]| -> [[f32; 4]; 4] {
            [
                [m[0][0] as f32, m[0][1] as f32, m[0][2] as f32, m[0][3] as f32],
                [m[1][0] as f32, m[1][1] as f32, m[1][2] as f32, m[1][3] as f32],
                [m[2][0] as f32, m[2][1] as f32, m[2][2] as f32, m[2][3] as f32],
                [m[3][0] as f32, m[3][1] as f32, m[3][2] as f32, m[3][3] as f32],
            ]
        };
        let (mut proj, mut inv) = match self.proj_mode {
            ProjMode::Perspective => {
                let near = (self.distance * 0.001_f32).max(0.0001_f32) as f64;
                let far = 100_000.0_f64;
                let f = 1.0 / (Tolerance::PI / 6.0).tan();
                let p00 = f / self.aspect as f64;
                let p11 = f;
                let nf = near - far;
                let m22 = far / nf;
                let m32 = (near * far) / nf;
                // proj cols: (p00,0,0,0) (0,p11,0,0) (0,0,m22,-1) (0,0,m32,0)
                let proj = [
                    [p00, 0.0, 0.0, 0.0],
                    [0.0, p11, 0.0, 0.0],
                    [0.0, 0.0, m22, -1.0],
                    [0.0, 0.0, m32, 0.0],
                ];
                // inv·(x,y,d,1): view = (x/p00, y/p11, -1) / ((d + m22)/m32)
                let inv = [
                    [1.0 / p00, 0.0, 0.0, 0.0],
                    [0.0, 1.0 / p11, 0.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0 / m32],
                    [0.0, 0.0, -1.0, m22 / m32],
                ];
                (proj, inv)
            }
            ProjMode::Ortho => {
                let h = self.ortho_scale as f64;
                let w = h * self.aspect as f64;
                let depth = ((self.ortho_scale.max(self.distance) * 8.0).max(1.0)) as f64;
                let near = -depth;
                let far = depth;
                let nf = near - far;
                let sz = 1.0 / nf;
                let tz = near / nf;
                let proj = [
                    [1.0 / w, 0.0, 0.0, 0.0],
                    [0.0, 1.0 / h, 0.0, 0.0],
                    [0.0, 0.0, sz, 0.0],
                    [0.0, 0.0, tz, 1.0],
                ];
                let inv = [
                    [w, 0.0, 0.0, 0.0],
                    [0.0, h, 0.0, 0.0],
                    [0.0, 0.0, 1.0 / sz, 0.0],
                    [0.0, 0.0, -tz / sz, 1.0],
                ];
                (proj, inv)
            }
        };
        // Off-center shift: proj' = T(dx,dy)·proj, so inv' = inv·T(-dx,-dy):
        // col3 -= dx·col0 + dy·col1.
        self.apply_ndc_offset(&mut proj);
        let dx = self.ndc_offset[0] as f64;
        let dy = self.ndc_offset[1] as f64;
        for r in 0..4 {
            inv[3][r] -= dx * inv[0][r] + dy * inv[1][r];
        }
        (c(proj), c(inv))
    }

    // Set a canonical named view and switch to ortho.
    // Quaternion maps the camera's default offset [0,−dist,0] to the view direction.
    pub fn set_named_view(&mut self, view: NamedView) {
        let half_pi = Tolerance::PI / 2.0;
        match view {
            // Top: camera offset → +Z  (rotate [0,−1,0] → [0,0,+1] = CW π/2 around X)
            NamedView::Top => {
                self.orientation = Quaternion::from_axis_angle(
                    Vector::new(1.0, 0.0, 0.0), -half_pi,
                );
                self.last_right = [1.0, 0.0, 0.0];
            }
            // Bottom: camera offset → −Z  (rotate [0,−1,0] → [0,0,−1] = CCW π/2 around X)
            NamedView::Bottom => {
                self.orientation = Quaternion::from_axis_angle(
                    Vector::new(1.0, 0.0, 0.0), half_pi,
                );
                self.last_right = [1.0, 0.0, 0.0];
            }
            // Right: camera offset → +X  (rotate [0,−1,0] → [+1,0,0] = CCW π/2 around Z)
            NamedView::Right => {
                self.orientation = Quaternion::from_axis_angle(
                    Vector::new(0.0, 0.0, 1.0), half_pi,
                );
            }
            // Left: camera offset → −X  (rotate [0,−1,0] → [−1,0,0] = CW π/2 around Z)
            NamedView::Left => {
                self.orientation = Quaternion::from_axis_angle(
                    Vector::new(0.0, 0.0, 1.0), -half_pi,
                );
            }
        }
        self.proj_mode   = ProjMode::Ortho;
        self.ortho_scale = self.distance;
        self.update_position();
    }

    pub fn pan(&mut self, right_amount: f32, up_amount: f32) {
        let fwd = Vector::new(
            (self.target[0] - self.position[0]) as f64,
            (self.target[1] - self.position[1]) as f64,
            (self.target[2] - self.position[2]) as f64,
        ).normalized();
        let wu    = Vector::new(self.world_up[0] as f64, self.world_up[1] as f64, self.world_up[2] as f64);
        let right = fwd.cross(&wu).normalized();
        let up    = right.cross(&fwd).normalized();

        let speed = self.distance * 0.001;
        for i in 0..3 {
            let delta = right[i] * right_amount as f64 * speed as f64 + up[i] * up_amount as f64 * speed as f64;
            self.position[i] += delta as f32;
            self.target[i]   += delta as f32;
        }
    }

    pub fn reset(&mut self) {
        self.target      = self.initial_target;
        self.distance    = self.initial_distance;
        self.orientation = self.initial_orientation.duplicate();
        self.proj_mode   = ProjMode::Perspective;
        self.ortho_scale = self.initial_distance;
        self.update_position();
    }

    /// Fit the camera to a sphere defined by a centre (mm) and half-diagonal (mm).
    /// Keeps current orientation; adjusts target, distance, and ortho_scale.
    pub fn fit_to_box(&mut self, center_mm: [f32; 3], half_diag_mm: f32) {
        self.target = [
            center_mm[0] * MM_TO_UNIT,
            center_mm[1] * MM_TO_UNIT,
            center_mm[2] * MM_TO_UNIT,
        ];
        let r = (half_diag_mm * MM_TO_UNIT).max(0.001);
        self.distance    = (r as f64 / (Tolerance::PI / 6.0_f32 as f64).tan()).clamp(MIN_ZOOM as f64, MAX_ZOOM as f64) as f32;
        self.ortho_scale = (r * 1.2).max(0.001);
        self.update_position();
    }
}

// ============================================================
// FRUSTUM (Gribb–Hartmann plane extraction for culling)
// Planes face inward; a point is inside when plane·(x,y,z,1) ≥ 0.
// Extracted for wgpu clip space (z ∈ [0, w]).
// ============================================================
pub struct Frustum {
    /// 6 planes [a,b,c,d]: left, right, bottom, top, near, far.
    pub planes: [[f32; 4]; 6],
}

impl Frustum {
    /// Build from a column-major view-projection matrix (`m[col][row]`).
    pub fn from_view_proj(m: &[[f32; 4]; 4]) -> Self {
        // row(r) = [m[0][r], m[1][r], m[2][r], m[3][r]]
        let row = |r: usize| [m[0][r], m[1][r], m[2][r], m[3][r]];
        let (r0, r1, r2, r3) = (row(0), row(1), row(2), row(3));
        let add = |a: [f32; 4], b: [f32; 4]| [a[0]+b[0], a[1]+b[1], a[2]+b[2], a[3]+b[3]];
        let sub = |a: [f32; 4], b: [f32; 4]| [a[0]-b[0], a[1]-b[1], a[2]-b[2], a[3]-b[3]];
        let mut planes = [
            add(r3, r0), // left
            sub(r3, r0), // right
            add(r3, r1), // bottom
            sub(r3, r1), // top
            r2,          // near (z ≥ 0)
            sub(r3, r2), // far
        ];
        for p in &mut planes {
            let n = (p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt();
            if n > 1e-20 { for c in p.iter_mut() { *c /= n; } }
        }
        Self { planes }
    }

    /// True if the world-space AABB is at least partially inside the frustum.
    /// Uses the positive-vertex (n-vertex) test: conservative — never culls a
    /// box that is actually visible.
    pub fn aabb_visible(&self, min: [f32; 3], max: [f32; 3]) -> bool {
        for p in &self.planes {
            let px = if p[0] >= 0.0 { max[0] } else { min[0] };
            let py = if p[1] >= 0.0 { max[1] } else { min[1] };
            let pz = if p[2] >= 0.0 { max[2] } else { min[2] };
            if p[0]*px + p[1]*py + p[2]*pz + p[3] < 0.0 {
                return false; // entirely behind this plane → outside
            }
        }
        true
    }
}

/// Transform a local-space AABB `[minx,miny,minz,maxx,maxy,maxz]` by a column-major
/// model matrix, returning the enclosing world-space AABB (min, max).
pub fn transform_aabb_cm(local: &[f32; 6], m: &[[f32; 4]; 4]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for &x in &[local[0], local[3]] {
        for &y in &[local[1], local[4]] {
            for &z in &[local[2], local[5]] {
                // column-major: world[r] = Σ_c m[c][r] * v[c]
                for r in 0..3 {
                    let w = m[0][r]*x + m[1][r]*y + m[2][r]*z + m[3][r];
                    if w < min[r] { min[r] = w; }
                    if w > max[r] { max[r] = w; }
                }
            }
        }
    }
    (min, max)
}

// ============================================================
// CAMERA CONTROLLER
// Keys:
//   Right-drag        → orbit
//   Ctrl+Right-drag   → pan
//   Scroll            → zoom (distance in perspective; ortho_scale in ortho)
//   WASD / arrows     → keyboard pan
//   C / F             → reset to initial view (perspective)
//   P                 → perspective projection
//   O                 → orthographic projection
//   T / B / L / R     → top / bottom / left / right named view (→ ortho)
// ============================================================
pub struct CameraController {
    is_right_down: bool,
    shift_held:    bool,
    select_add:    bool,
    orbit_dx:      f32,
    orbit_dy:      f32,
    is_panning:    bool,
    pan_dx:        f32,
    pan_dy:        f32,
    scroll:        f32,
    amount_left:   f32,
    amount_right:  f32,
    amount_up:     f32,
    amount_down:   f32,
    reset_pressed:    bool,
    proj_request:     Option<ProjMode>,
    view_request:     Option<NamedView>,
    orbit_speed:   f32,
    zoom_speed:    f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            is_right_down: false,
            shift_held:    false,
            select_add:    false,
            orbit_dx:      0.0,
            orbit_dy:      0.0,
            is_panning:    false,
            pan_dx:        0.0,
            pan_dy:        0.0,
            scroll:        0.0,
            amount_left:   0.0,
            amount_right:  0.0,
            amount_up:     0.0,
            amount_down:   0.0,
            reset_pressed:    false,
            proj_request:     None,
            view_request:     None,
            orbit_speed:   0.005,
            zoom_speed:    0.1,
        }
    }

    pub fn process_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Right {
            self.is_right_down = pressed;
            self.update_right_modes();
            if !pressed {
                self.orbit_dx = 0.0; self.orbit_dy = 0.0;
                self.pan_dx   = 0.0; self.pan_dy   = 0.0;
            }
        }
    }

    pub fn process_shift(&mut self, pressed: bool) {
        self.shift_held = pressed;
        self.update_right_modes();
    }

    fn update_right_modes(&mut self) {
        let panning  = self.is_right_down && self.shift_held;
        let orbiting = self.is_right_down && !self.shift_held;
        if !panning  { self.pan_dx   = 0.0; self.pan_dy   = 0.0; }
        if !orbiting { self.orbit_dx = 0.0; self.orbit_dy = 0.0; }
        self.is_panning  = panning;
        let _ = orbiting;
    }

    fn is_orbiting(&self) -> bool { self.is_right_down && !self.shift_held }

    pub fn process_mouse_move(&mut self, dx: f32, dy: f32) {
        if self.is_orbiting() { self.orbit_dx = dx; self.orbit_dy = dy; }
        if self.is_panning    { self.pan_dx   = dx; self.pan_dy   = dy; }
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = match delta {
            MouseScrollDelta::LineDelta(_, y)   => -*y,
            MouseScrollDelta::PixelDelta(p) => -(p.y as f32) * 0.01,
        };
    }

    pub fn select_add(&self) -> bool { self.select_add }

    pub fn process_modifiers(&mut self, mods: ModifiersState) {
        self.process_shift(mods.control_key());
        self.select_add = mods.shift_key();
    }

    pub fn process_key(&mut self, key: KeyCode, pressed: bool) {
        let v = if pressed { 1.0 } else { 0.0 };
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp    => self.amount_up    = v,
            KeyCode::KeyS | KeyCode::ArrowDown  => self.amount_down  = v,
            KeyCode::KeyA | KeyCode::ArrowLeft  => self.amount_left  = v,
            KeyCode::KeyD | KeyCode::ArrowRight => self.amount_right = v,
            KeyCode::KeyC => { if pressed { self.reset_pressed = true; } }
            KeyCode::KeyP => { if pressed { self.proj_request = Some(ProjMode::Perspective); } }
            KeyCode::KeyO => { if pressed { self.proj_request = Some(ProjMode::Ortho); } }
            KeyCode::KeyT => { if pressed { self.view_request = Some(NamedView::Top); } }
            KeyCode::KeyB => { if pressed { self.view_request = Some(NamedView::Bottom); } }
            KeyCode::KeyL => { if pressed { self.view_request = Some(NamedView::Left); } }
            KeyCode::KeyR => { if pressed { self.view_request = Some(NamedView::Right); } }
            _ => {}
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        // Orbit: yaw around world-Z, pitch around tracked right.
        if self.is_orbiting() && (self.orbit_dx != 0.0 || self.orbit_dy != 0.0) {
            let yaw   = (-self.orbit_dx * self.orbit_speed).clamp(-0.1, 0.1);
            let pitch = (-self.orbit_dy * self.orbit_speed).clamp(-0.1, 0.1);

            let wu    = Vector::new(camera.world_up[0] as f64,   camera.world_up[1] as f64,   camera.world_up[2] as f64);
            let right = Vector::new(camera.last_right[0] as f64, camera.last_right[1] as f64, camera.last_right[2] as f64);

            let yaw_q   = Quaternion::from_axis_angle(wu, yaw as f64);
            let pitch_q = Quaternion::from_axis_angle(right.normalized(), pitch as f64);

            let old = camera.orientation.duplicate();
            camera.orientation = (yaw_q * (pitch_q * old)).normalized();
            camera.update_position();

            self.orbit_dx = 0.0;
            self.orbit_dy = 0.0;
        }

        // Pan.
        if self.is_panning && (self.pan_dx != 0.0 || self.pan_dy != 0.0) {
            camera.pan(-self.pan_dx, self.pan_dy);
            self.pan_dx = 0.0;
            self.pan_dy = 0.0;
        }

        // Keyboard pan.
        let kr = (self.amount_left  - self.amount_right) * 30.0;
        let ku = (self.amount_down  - self.amount_up)    * 30.0;
        if kr != 0.0 || ku != 0.0 {
            camera.pan(kr, ku);
        }

        // Zoom: adjusts distance (perspective) or ortho_scale (ortho).
        if self.scroll != 0.0 {
            let factor = 1.0 + self.scroll * self.zoom_speed;
            match camera.proj_mode {
                ProjMode::Perspective => {
                    camera.distance = (camera.distance * factor).clamp(MIN_ZOOM, MAX_ZOOM);
                    camera.update_position();
                }
                ProjMode::Ortho => {
                    camera.ortho_scale = (camera.ortho_scale * factor).clamp(0.001, 100000.0);
                }
            }
            self.scroll = 0.0;
        }

        // Named view (T/B/L/R → ortho).
        if let Some(v) = self.view_request.take() {
            camera.set_named_view(v);
        }

        // Projection toggle (P/O).
        if let Some(mode) = self.proj_request.take() {
            if mode == ProjMode::Ortho && camera.proj_mode == ProjMode::Perspective {
                camera.ortho_scale = camera.distance;
            }
            camera.proj_mode = mode;
        }

        // Reset.
        if self.reset_pressed {
            camera.reset();
            self.reset_pressed = false;
        }
    }
}
