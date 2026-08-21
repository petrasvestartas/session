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
        let half_h = self.distance * f64::to_radians(30.0).tan();
        let half_w = half_h * (viewport.0 / viewport.1);
        let right = self.orientation.rotate_vector(Vector::x_axis());
        for i in 0..3 {
            let cursor_off = right[i] * ndc_x * half_w + self.up[i] * ndc_y * half_h;
            self.target[i] += cursor_off * (1.0 - k); // keeps the curson's world point fixed
        }
        self.distance = new_dist;
        self.update_position();
    }


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
    pub fn toggle_projection_framed(&mut self, min: [f32; 3], max: [f32; 3], aspect: f64) {
        self.perspective = !self.perspective;
        if !self.perspective {
            return;
        }
        let s = self.unit.to_meters();
        let t = self.origin(); // target, world units
        let right = self.orientation.rotate_vector(Vector::x_axis());
        let up = Vector::new(self.up[0], self.up[1], self.up[2]);
        let fwd = Vector::new(
            (self.target[0] - self.position[0]) / self.distance,
            (self.target[1] - self.position[1]) / self.distance,
            (self.target[2] - self.position[2]) / self.distance,
        );
        let half_h = self.distance * f64::to_radians(30.0).tan() / s; // world units
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
        self.fit(
            [lo[0] as f32, lo[1] as f32, lo[2] as f32],
            [hi[0] as f32, hi[1] as f32, hi[2] as f32],
            aspect,
        );
    }

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
            Xform::perspective(f64::to_radians(60.0), aspect, far, dist * 0.01)
        } else {
            let h = dist * f64::to_radians(30.0).tan();
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

    /// Frame an AABB: center the target on it and set distance so its bounding sphere fills the FOV (+10%).
    pub fn fit(&mut self, min: [f32; 3], max: [f32; 3], aspect: f64) {
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
        let half_fov_y = f64::to_radians(60.0) * 0.5;
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
            let dot = |v: session_rust::Vector| p[0] * v[0] + p[1] * v[1] + p[2] * v[2];
            let (x, y, z) = (dot(right.clone()), dot(up.clone()), dot(fwd.clone()));
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

    /// Set the world unit (mm/m) applied by `view_proj`'s scale.
    pub fn set_unit(&mut self, unit: Unit) {
        self.unit = unit;
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
