use session_rust::{AABB, Point, Quaternion, Vector, Xform};

/// Vertical field of view in degrees.
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

/// A named standard view.
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
}

impl Camera {
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

    // --8<-- [start:step-5a]
    /// The world ray under a cursor pixel: origin and unit direction, scene units.
    pub fn ray(&self, cursor: (f64, f64), viewport: (f64, f64)) -> Option<(Point, Vector)> {
        // no ray for an empty viewport or a lost cursor
        if viewport.0 <= 0.0 || viewport.1 <= 0.0 || !cursor.0.is_finite() || !cursor.1.is_finite()
        {
            return None;
        }

        // pixel to -1..1 on both axes, y up
        let ndc_x = 2.0 * cursor.0 / viewport.0 - 1.0;
        let ndc_y = 1.0 - 2.0 * cursor.1 / viewport.1;
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
    // --8<-- [end:step-5a]
    pub fn zoom_at(&mut self, amount: f32, cursor: (f64, f64), viewport: (f64, f64)) {
        // nothing for an empty viewport or a lost cursor
        if viewport.0 <= 0.0 || viewport.1 <= 0.0 || !cursor.0.is_finite() || !cursor.1.is_finite()
        {
            return;
        }

        let new_dist = zoom_distance(self.distance, amount);
        let k = new_dist / self.distance; // how much closer
        // pixel to -1..1 on both axes, y up
        let ndc_x = 2.0 * cursor.0 / viewport.0 - 1.0;
        let ndc_y = 1.0 - 2.0 * cursor.1 / viewport.1;
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
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];

        // clip every box corner to the visible rectangle
        for corner in bounds.corners() {
            let c = [corner[0] - t[0], corner[1] - t[1], corner[2] - t[2]];
            let dx = (c[0] * right[0] + c[1] * right[1] + c[2] * right[2]).clamp(-half_w, half_w);
            let dy = (c[0] * up[0] + c[1] * up[1] + c[2] * up[2]).clamp(-half_h, half_h);
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

    /// The view-projection matrix, relative to the target.
    pub fn view_proj(&self, aspect: f64) -> Xform {
        self.view_proj_anchored(aspect, &self.origin())
    }

    /// The view-projection matrix, relative to `anchor` (scene units).
    pub fn view_proj_anchored(&self, aspect: f64, anchor: &Point) -> Xform {
        let dist = self.distance;
        let a = self.unit.to_meters();
        let anchor = Point::new(anchor[0] * a, anchor[1] * a, anchor[2] * a); // to meters
        // far plane reaches the whole scene
        let far = (dist * 10.0).max(dist + 2.0 * self.scene_extent);
        let projection = if self.perspective {
            // far and near swapped: depth 1 is near (reverse-Z)
            Xform::perspective(FOVY_DEG.to_radians(), aspect, far, dist * NEAR_FRACTION)
        } else {
            let h = dist * (FOVY_DEG * 0.5).to_radians().tan(); // half view height
            // depth range covers the scene, not more
            let extent = if self.scene_extent > 0.0 {
                self.scene_extent
            } else {
                dist
            };
            let r = (dist + 2.0 * extent).max(1.0e-6);
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

        projection * view * scale
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
        };

        self.perspective = false;
        self.update_position();
    }

    /// Reset to a fresh default camera.
    pub fn reset(&mut self) {
        *self = Camera::new();
    }

    /// Frame a box: look at its center, back off until it fits.
    pub fn fit(&mut self, bounds: &AABB, aspect: f64) {
        if !bounds.is_valid() {
            return;
        }

        let s = self.unit.to_meters();

        // look at the box center
        self.target = [bounds.cx * s, bounds.cy * s, bounds.cz * s];

        // half the view angle, sideways and up
        let half_fov_y = FOVY_DEG.to_radians() * 0.5;
        let half_fov_x = (aspect * half_fov_y.tan()).atan();
        let (tx, ty) = (half_fov_x.tan(), half_fov_y.tan());

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

#[cfg(test)]
mod tests {
    use super::*;

    /// The depth value of a point `depth` meters in front of the eye.
    fn ndc_depth(cam: &Camera, depth: f64) -> f64 {
        let fwd = cam.orientation.rotate_vector(Vector::y_axis());
        let (s, o) = (cam.unit.to_meters(), cam.origin());
        let p = Point::new(
            cam.position[0] + fwd[0] * depth,
            cam.position[1] + fwd[1] * depth,
            cam.position[2] + fwd[2] * depth,
        );
        cam.view_proj(1.5).transform_point(&Point::new(
            p[0] / s - o[0],
            p[1] / s - o[1],
            p[2] / s - o[2],
        ))[2]
    }

    /// The near plane cuts just ahead of the eye at any distance.
    #[test]
    fn near_plane_cuts_a_millimetre_ahead_not_a_beam() {
        let mut cam = Camera::new();
        cam.scene_extent = 118.0;

        for dist in [122.0, 10.0, 0.5] {
            cam.distance = dist;
            cam.update_position();
            assert!(
                ndc_depth(&cam, dist * 2.0 * NEAR_FRACTION) < 1.0,
                "dist {dist}: cut too early"
            );
            assert!(
                ndc_depth(&cam, dist * 0.5 * NEAR_FRACTION) > 1.0,
                "dist {dist}: near plane missing"
            );
            assert!(
                ndc_depth(&cam, dist + 2.0 * cam.scene_extent - 1.0e-6) > 0.0,
                "dist {dist}: far plane short of the scene"
            );
        }
    }

    #[test]
    fn distant_orthographic_depth_distinguishes_four_millimetres() {
        let mut cam = Camera::new();
        cam.perspective = false;
        cam.scene_extent = 2.0;

        for distance in [3.3, 13.2, 52.8] {
            cam.distance = distance;
            cam.update_position();
            let front = ndc_depth(&cam, distance) as f32;
            let rear = ndc_depth(&cam, distance + 0.004) as f32;
            assert!(
                front - rear > rear.abs() * 1.9073486e-6,
                "distance {distance}: hidden ink falls within float tolerance"
            );
        }
    }

    #[test]
    fn orthographic_depth_contains_the_scene_before_and_behind_the_eye() {
        let mut cam = Camera::new();
        cam.perspective = false;

        for extent in [0.0, 2.0, 118.0] {
            cam.scene_extent = extent;

            for distance in [1.0e-6, 0.5, 122.0] {
                cam.distance = distance;
                cam.update_position();

                for depth in [distance - extent, distance + extent] {
                    let projected = ndc_depth(&cam, depth);
                    assert!(
                        (0.0..=1.0).contains(&projected),
                        "extent {extent}, distance {distance}, depth {depth}: {projected}"
                    );
                }
            }
        }
    }
}

/// The distance after `amount` wheel steps, 0.9 per step.
fn zoom_distance(distance: f64, amount: f32) -> f64 {
    if !amount.is_finite() {
        return distance;
    }

    // at most ten steps per event, never zero
    (distance * 0.9_f64.powf(f64::from(amount).clamp(-10.0, 10.0))).clamp(1.0e-6, 1.0e15)
}

#[cfg(test)]
mod wheel_tests {
    // --8<-- [start:step-5b]
    #[cfg(test)]
    mod ray_tests {
        use super::*;

        fn viewport() -> (f64, f64) {
            (800.0, 400.0)
        }

        /// The center pixel looks along the view axis.
        #[test]
        fn the_centre_ray_is_the_view_axis() {
            let mut cam = Camera::new();
            cam.update_position();

            for perspective in [true, false] {
                cam.perspective = perspective;
                let (_, dir) = cam.ray((400.0, 200.0), viewport()).expect("a ray");
                let forward = cam.orientation.rotate_vector(Vector::y_axis());

                for i in 0..3 {
                    assert!((dir[i] - forward[i]).abs() < 1e-12, "{perspective}");
                }
            }
        }

        /// Perspective rays share the eye; orthographic rays share the direction.
        #[test]
        fn perspective_rays_share_an_origin_and_ortho_rays_share_a_direction() {
            let mut cam = Camera::new();
            cam.update_position();

            cam.perspective = true;
            let (a, da) = cam.ray((100.0, 80.0), viewport()).expect("a ray");
            let (b, db) = cam.ray((700.0, 320.0), viewport()).expect("a ray");

            for i in 0..3 {
                assert!((a[i] - b[i]).abs() < 1e-9, "one eye");
            }

            assert!(
                (0..3).any(|i| (da[i] - db[i]).abs() > 1e-6),
                "different directions"
            );

            cam.perspective = false;
            let (a, da) = cam.ray((100.0, 80.0), viewport()).expect("a ray");
            let (b, db) = cam.ray((700.0, 320.0), viewport()).expect("a ray");

            for i in 0..3 {
                assert!((da[i] - db[i]).abs() < 1e-12, "one direction");
            }

            assert!(
                (0..3).any(|i| (a[i] - b[i]).abs() > 1e-6),
                "different origins"
            );
        }

        /// A ray through a pixel meets the target plane at that pixel's point.
        #[test]
        fn a_ray_hits_the_target_plane_where_the_cursor_is() {
            let mut cam = Camera::new();
            cam.update_position();
            // scene units on both sides
            let target = cam.origin();
            let half_h = cam.distance_world() * (FOVY_DEG * 0.5).to_radians().tan();
            let half_w = half_h * (viewport().0 / viewport().1);
            let right = cam.orientation.rotate_vector(Vector::x_axis());
            let forward = cam.orientation.rotate_vector(Vector::y_axis());
            // a quarter right and a quarter up from the target
            let expected: Vec<f64> = (0..3)
                .map(|i| target[i] + right[i] * 0.5 * half_w + cam.up[i] * 0.5 * half_h)
                .collect();

            for perspective in [true, false] {
                cam.perspective = perspective;
                let (origin, dir) = cam.ray((600.0, 100.0), viewport()).expect("a ray");
                // walk the ray to the target plane
                let denom: f64 = (0..3).map(|i| dir[i] * forward[i]).sum();
                let num: f64 = (0..3).map(|i| (target[i] - origin[i]) * forward[i]).sum();
                let t = num / denom;

                for i in 0..3 {
                    let hit = origin[i] + dir[i] * t;
                    assert!((hit - expected[i]).abs() < 1e-6, "{perspective} axis {i}");
                }
            }
        }

        /// The ray is in scene units, not the camera's meters.
        #[test]
        fn the_ray_is_in_world_units_not_the_camera_s_metres() {
            let mut cam = Camera::new();
            cam.update_position();
            // the center pixel looks straight at the target
            let (origin, dir) = cam
                .ray((viewport().0 * 0.5, viewport().1 * 0.5), viewport())
                .expect("a ray");
            let target = cam.origin();
            let reach: f64 = (0..3).map(|i| (target[i] - origin[i]) * dir[i]).sum();
            assert!(
                (reach - cam.distance_world()).abs() < 1e-6,
                "the eye is distance_world from the target, in world units"
            );
            assert!(
                cam.distance_world() > cam.distance * 100.0,
                "the fixture is a millimetre scene, so the two really do differ"
            );
        }

        /// An empty viewport gives no ray.
        #[test]
        fn a_degenerate_viewport_has_no_ray() {
            let mut cam = Camera::new();
            cam.update_position();
            assert!(cam.ray((1.0, 1.0), (0.0, 400.0)).is_none());
            assert!(cam.ray((f64::NAN, 1.0), viewport()).is_none());
        }
    }
    // --8<-- [end:step-5b]
    use super::*;

    #[test]
    fn coalesced_wheel_events_remain_positive_and_preserve_the_cursor_anchor() {
        let mut camera = Camera::new();
        let before = camera.distance;
        camera.zoom_at(16.0, (800.0, 500.0), (1600.0, 1000.0));
        assert!(camera.distance > before * 0.3);
        assert!(camera.distance < before);
        let distance = camera.distance;
        camera.zoom(f32::NAN);
        assert_eq!(camera.distance, distance);
        assert!(
            (zoom_distance(zoom_distance(before, 1.0), 1.0) - zoom_distance(before, 2.0)).abs()
                < 1e-10
        );
    }
}
