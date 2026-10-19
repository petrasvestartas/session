use session_rust::{AABB, Point, Quaternion, Vector, Xform};

pub const FOVY_DEG:f64=60.0;

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
        let wu = Vector::new(self.world_up[0], self.world_up[1], self.world_up[2]);
        let right = self.orientation.rotate_vector(Vector::x_axis());
        // horizontal pixels turn about up, vertical pixels tilt about right
        let yaw_q = Quaternion::from_axis_angle(wu, (-dx * 0.005) as f64);
        let pitch_q = Quaternion::from_axis_angle(right, (-dy * 0.005) as f64);

        self.orientation = (yaw_q * (pitch_q * self.orientation.duplicate())).normalized();
        self.update_position();
    }

    /// Slide the camera sideways by mouse pixels.
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let right = self.orientation.rotate_vector(Vector::x_axis());
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
    pub fn ray(&self, cursor: (f64, f64), viewport: (f64, f64)) -> Option<(Point, Vector)> {
        // no ray for an empty viewport or a lost cursor
        if viewport.0 <= 0.0 || viewport.1 <= 0.0 || !cursor.0.is_finite() || !cursor.1.is_finite()
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
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let forward = self.orientation.rotate_vector(Vector::y_axis());
        // the cursor's point on the target plane
        let mut on_plane = [0.0; 3];

        for i in 0..3 {
            on_plane[i] = target[i] + right[i] * ndc_x * half_w + self.up[i] * ndc_y * half_h;
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
            Vector::new(dir[0] / length, dir[1] / length, dir[2] / length),
        ))
    }

    /// Zoom so the point under the cursor stays under the cursor.
    pub fn zoom_at(&mut self, amount: f32, cursor: (f64, f64), viewport: (f64, f64)) {
        // nothing for an empty viewport or a lost cursor
        if viewport.0 <= 0.0 || viewport.1 <= 0.0 || !cursor.0.is_finite() || !cursor.1.is_finite()
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
        let right = self.orientation.rotate_vector(Vector::x_axis());

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
        let right = self.orientation.rotate_vector(Vector::x_axis());
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
            let c = [corner[0] - t[0], corner[1] - t[1], corner[2] - t[2]];
            let dx = (c[0] * right[0] + c[1] * right[1] + c[2] * right[2])
                .clamp((-1.0 - shift_x) * half_w, (1.0 - shift_x) * half_w);
            let dy = (c[0] * up[0] + c[1] * up[1] + c[2] * up[2])
                .clamp((-1.0 - shift_y) * half_h, (1.0 - shift_y) * half_h);
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
        Point::new(self.target[0] / s, self.target[1] / s, self.target[2] / s)
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
            Xform::perspective(FOVY_DEG.to_radians(), aspect, far, dist * NEAR_FRACTION)
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

        let fwd = self.orientation.rotate_vector(Vector::y_axis());
        let up = self.orientation.rotate_vector(Vector::z_axis());
        let right = self.orientation.rotate_vector(Vector::x_axis());

        // the distance every corner needs to be in view
        let mut distance: f64 = 0.0;
        let mut extent: f64 = 0.0;

        for p in self.offsets(bounds) {
            let (x, y, z) = (dot3(&p, &right), dot3(&p, &up), dot3(&p, &fwd));
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
        let fwd = self.orientation.rotate_vector(Vector::y_axis()); // eye to target
        let up = self.orientation.rotate_vector(Vector::z_axis());

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
    (distance * 0.9_f64.powf(f64::from(amount).clamp(-10.0, 10.0))).clamp(1.0e-6, 1.0e15)
}

