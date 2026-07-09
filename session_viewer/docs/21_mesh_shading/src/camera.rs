use session_rust::{Point, Quaternion, Vector, Xform};

#[derive(Clone, Copy, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}

impl Unit {
    pub fn to_meters(self) -> f64 {
        match self {
            Unit::Millimeters => 0.001,
            Unit::Meters => 1.0,
        }
    }
}

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

pub struct Camera {
    pub target: [f64; 3],
    pub distance: f64,
    pub orientation: Quaternion, // source of truth - a full, singularity-free frame
    pub world_up: [f64; 3],      // +Z turnable axis (yaw rotates about this)
    pub position: [f64; 3],      // derived by update_position
    pub up: [f64; 3],            // derived by update_position
    pub perspective: bool,
    pub unit: Unit,
}

impl Camera {
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
        };

        cam.update_position();

        cam
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let wu = Vector::new(self.world_up[0], self.world_up[1], self.world_up[2]);
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let yaw_q = Quaternion::from_axis_angle(wu, (-dx * 0.005) as f64);
        let pitch_q = Quaternion::from_axis_angle(right, (-dy * 0.005) as f64);

        self.orientation = (yaw_q * (pitch_q * self.orientation.duplicate())).normalized();
        self.update_position();
    }

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

    pub fn toggle_projection(&mut self) {
        self.perspective = !self.perspective;
    }

    pub fn view_proj(&self, aspect: f64) -> Xform {
        let dist = self.distance;
        let projection = if self.perspective {
            Xform::perspective(f64::to_radians(60.0), aspect, dist * 0.001, dist * 100.0)
        } else {
            let h = dist * f64::to_radians(30.0).tan();
            let r = dist * 100.0;
            Xform::orthographic(-aspect * h, aspect * h, -h, h, -r, r)
        };

        let eye = Point::new(self.position[0], self.position[1], self.position[2]);
        let target = Point::new(self.target[0], self.target[1], self.target[2]);
        let up = Vector::new(self.up[0], self.up[1], self.up[2]);
        let view = Xform::look_at_right_handed(&eye, &target, &up);

        // units
        let s = self.unit.to_meters();
        let scale = Xform::scale_xyz(s, s, s);

        projection * view * scale
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
        };

        self.perspective = false;
        self.update_position();
    }

    pub fn reset(&mut self) {
        *self = Camera::new();
    }

    pub fn fit(&mut self, min: [f32; 3], max: [f32; 3], aspect: f64) {
        // unit scale
        let s = self.unit.to_meters();

        // target + box center
        self.target = [
            (min[0] as f64 + max[0] as f64) * 0.5 * s,
            (min[1] as f64 + max[1] as f64) * 0.5 * s,
            (min[2] as f64 + max[2] as f64) * 0.5 * s,
        ];

        // bounding-box-sphere radius = half  the diagonal
        let dx = (max[0] as f64 - min[0] as f64) * 0.5 * s;
        let dy = (max[1] as f64 - min[1] as f64) * 0.5 * s;
        let dz = (max[2] as f64 - min[2] as f64) * 0.5 * s;
        let radius = (dx * dx + dy * dy + dz * dz).sqrt();
        if radius <= 0.0 {
            return;
        }

        // limiting half-FOV 60 deg vertical, narrower horizontal on a tall convas
        let half_fov_y = f64::to_radians(60.0) * 0.5;
        let half_fov_x = (aspect * half_fov_y.tan()).atan();
        let half_fov = half_fov_y.min(half_fov_x);

        // distance so the shere fills that half-angle, plus 10% room
        self.distance = (radius / half_fov.sin() * 1.1).clamp(0.2, 100.0);
        self.update_position();
    }

    pub fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
    }

    pub fn update_position(&mut self) {
        let fwd = self.orientation.rotate_vector(Vector::y_axis()); // eye -> target
        let up = self.orientation.rotate_vector(Vector::z_axis());
        for i in 0..3 {
            self.position[i] = self.target[i] - fwd[i] * self.distance;
            self.up[i] = up[i];
        }
    }
}
