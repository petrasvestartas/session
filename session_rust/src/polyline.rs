use crate::{Color, Plane, Point, Tolerance, Vector, Xform};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use uuid::Uuid;

/// A polyline defined by a collection of points with an associated plane.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Polyline")]
pub struct Polyline {
    pub guid: String,
    pub name: String,
    pub points: Vec<Point>,
    pub plane: Plane,
    pub width: f64,
    pub linecolor: Color,
    #[serde(default = "Xform::identity")]
    pub xform: Xform,
}

impl Default for Polyline {
    fn default() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_polyline".to_string(),
            points: Vec::new(),
            plane: Plane::default(),
            width: 1.0,
            linecolor: Color::white(),
            xform: Xform::identity(),
        }
    }
}

impl Polyline {
    /// Creates a new `Polyline` with default guid and name.
    ///
    /// # Arguments
    ///
    /// * `points` - The collection of points.
    pub fn new(points: Vec<Point>) -> Self {
        // Delegate plane computation to Plane::from_points
        let plane = if points.len() >= 3 {
            Plane::from_points(points.clone())
        } else {
            Plane::default()
        };

        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_polyline".to_string(),
            points,
            plane,
            width: 1.0,
            linecolor: Color::white(),
            xform: Xform::identity(),
        }
    }

    /// Returns the number of points in the polyline.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns true if the polyline has no points.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns the number of segments in the polyline.
    /// A polyline with n points has n-1 segments.
    pub fn segment_count(&self) -> usize {
        if self.points.len() > 1 {
            self.points.len() - 1
        } else {
            0
        }
    }

    /// Calculates the total length of the polyline.
    pub fn length(&self) -> f64 {
        let mut total_length = 0.0;
        for i in 0..self.segment_count() {
            let mut segment_vector = self.points[i + 1].clone() - self.points[i].clone();
            total_length += segment_vector.magnitude();
        }
        total_length
    }

    /// Returns a reference to the point at the given index.
    pub fn get_point(&self, index: usize) -> Option<&Point> {
        self.points.get(index)
    }

    /// Returns a mutable reference to the point at the given index.
    pub fn get_point_mut(&mut self, index: usize) -> Option<&mut Point> {
        self.points.get_mut(index)
    }

    /// Adds a point to the end of the polyline.
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
        // Recompute plane if we have at least 3 points
        if self.points.len() == 3 {
            self.plane = Plane::from_points(self.points.clone());
        }
    }

    /// Inserts a point at the specified index.
    pub fn insert_point(&mut self, index: usize, point: Point) {
        self.points.insert(index, point);
        // Recompute plane if we have at least 3 points
        if self.points.len() == 3 {
            self.plane = Plane::from_points(self.points.clone());
        }
    }

    /// Removes and returns the point at the specified index.
    pub fn remove_point(&mut self, index: usize) -> Option<Point> {
        if index < self.points.len() {
            let point = self.points.remove(index);
            // Recompute plane if we still have at least 3 points
            if self.points.len() == 3 {
                self.plane = Plane::from_points(self.points.clone());
            }
            Some(point)
        } else {
            None
        }
    }

    /// Reverses the order of points in the polyline.
    pub fn reverse(&mut self) {
        self.points.reverse();
        self.plane.reverse();
    }

    /// Returns a new polyline with reversed point order.
    pub fn reversed(&self) -> Self {
        let mut reversed = self.clone();
        reversed.reverse();
        reversed
    }

    pub fn transform(&mut self) {
        let xform = self.xform.clone();
        for pt in &mut self.points {
            xform.transform_point(pt);
        }
        self.xform = Xform::identity();
    }

    pub fn transformed(&self) -> Self {
        let mut result = self.clone();
        result.transform();
        result
    }

    /// Serializes the Polyline to a JSON string.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        self.serialize(&mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserializes a Polyline from a JSON string.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Polyline to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes a Polyline from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Geometric Utilities
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Shift polyline points by specified number of positions
    pub fn shift(&mut self, times: i32) {
        if self.points.is_empty() {
            return;
        }
        let len = self.points.len();
        let shift_amount = ((times % len as i32) + len as i32) % len as i32;
        self.points.rotate_left(shift_amount as usize);
    }

    /// Calculate squared length of polyline (faster, no sqrt)
    pub fn length_squared(&self) -> f64 {
        let mut length = 0.0f64;
        for i in 0..self.segment_count() {
            let segment = self.points[i + 1].clone() - self.points[i].clone();
            length += segment.length_squared();
        }
        length
    }

    /// Get point at parameter t along a line segment (t=0 is start, t=1 is end)
    pub fn point_at_parameter(start: &Point, end: &Point, t: f64) -> Point {
        let s = 1.0 - t;
        let t_f32 = t;
        let s_f32 = s;
        Point::new(
            if start.x() == end.x() {
                start.x()
            } else {
                s_f32 * start.x() + t_f32 * end.x()
            },
            if start.y() == end.y() {
                start.y()
            } else {
                s_f32 * start.y() + t_f32 * end.y()
            },
            if start.z() == end.z() {
                start.z()
            } else {
                s_f32 * start.z() + t_f32 * end.z()
            },
        )
    }

    /// Find closest point on line segment to given point, returns parameter t
    pub fn closest_point_to_line(point: &Point, line_start: &Point, line_end: &Point) -> f64 {
        let d = line_end.clone() - line_start.clone();
        let dod = d.length_squared();

        if dod > 0.0 {
            if (point.clone() - line_start.clone()).length_squared()
                <= (point.clone() - line_end.clone()).length_squared()
            {
                (point.clone() - line_start.clone()).dot(&d) / dod
            } else {
                1.0 + (point.clone() - line_end.clone()).dot(&d) / dod
            }
        } else {
            0.0
        }
    }

    /// Check if two line segments overlap and return the overlapping segment
    pub fn line_line_overlap(
        line0_start: &Point,
        line0_end: &Point,
        line1_start: &Point,
        line1_end: &Point,
    ) -> Option<(Point, Point)> {
        let mut t = [0.0, 1.0, 0.0, 0.0];
        t[2] = Self::closest_point_to_line(line1_start, line0_start, line0_end);
        t[3] = Self::closest_point_to_line(line1_end, line0_start, line0_end);

        let do_overlap = !((t[2] < 0.0 && t[3] < 0.0) || (t[2] > 1.0 && t[3] > 1.0));
        t.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let overlap_valid = (t[2] - t[1]).abs() > Tolerance::ZERO_TOLERANCE;

        if do_overlap && overlap_valid {
            Some((
                Self::point_at_parameter(line0_start, line0_end, t[1]),
                Self::point_at_parameter(line0_start, line0_end, t[2]),
            ))
        } else {
            None
        }
    }

    /// Calculate average of two line segments
    pub fn line_line_average(
        line0_start: &Point,
        line0_end: &Point,
        line1_start: &Point,
        line1_end: &Point,
    ) -> (Point, Point) {
        let output_start = Point::new(
            (line0_start.x() + line1_start.x()) * 0.5,
            (line0_start.y() + line1_start.y()) * 0.5,
            (line0_start.z() + line1_start.z()) * 0.5,
        );
        let output_end = Point::new(
            (line0_end.x() + line1_end.x()) * 0.5,
            (line0_end.y() + line1_end.y()) * 0.5,
            (line0_end.z() + line1_end.z()) * 0.5,
        );
        (output_start, output_end)
    }

    /// Calculate overlap average of two line segments
    pub fn line_line_overlap_average(
        line0_start: &Point,
        line0_end: &Point,
        line1_start: &Point,
        line1_end: &Point,
    ) -> (Point, Point) {
        let line_a = Self::line_line_overlap(line0_start, line0_end, line1_start, line1_end);
        let line_b = Self::line_line_overlap(line1_start, line1_end, line0_start, line0_end);

        if let (Some((line_a_start, line_a_end)), Some((line_b_start, line_b_end))) =
            (line_a, line_b)
        {
            let mid_line0_start = Point::new(
                (line_a_start.x() + line_b_start.x()) * 0.5,
                (line_a_start.y() + line_b_start.y()) * 0.5,
                (line_a_start.z() + line_b_start.z()) * 0.5,
            );
            let mid_line0_end = Point::new(
                (line_a_end.x() + line_b_end.x()) * 0.5,
                (line_a_end.y() + line_b_end.y()) * 0.5,
                (line_a_end.z() + line_b_end.z()) * 0.5,
            );
            let mid_line1_start = Point::new(
                (line_a_start.x() + line_b_end.x()) * 0.5,
                (line_a_start.y() + line_b_end.y()) * 0.5,
                (line_a_start.z() + line_b_end.z()) * 0.5,
            );
            let mid_line1_end = Point::new(
                (line_a_end.x() + line_b_start.x()) * 0.5,
                (line_a_end.y() + line_b_start.y()) * 0.5,
                (line_a_end.z() + line_b_start.z()) * 0.5,
            );

            let mid0_vec = mid_line0_end.clone() - mid_line0_start.clone();
            let mid1_vec = mid_line1_end.clone() - mid_line1_start.clone();

            if mid0_vec.length_squared() > mid1_vec.length_squared() {
                (mid_line0_start, mid_line0_end)
            } else {
                (mid_line1_start, mid_line1_end)
            }
        } else {
            Self::line_line_average(line0_start, line0_end, line1_start, line1_end)
        }
    }

    /// Create line from projected points onto a base line
    pub fn line_from_projected_points(
        line_start: &Point,
        line_end: &Point,
        points: &[Point],
    ) -> Option<(Point, Point)> {
        if points.is_empty() {
            return None;
        }

        let mut t_values: Vec<f64> = points
            .iter()
            .map(|p| Self::closest_point_to_line(p, line_start, line_end))
            .collect();

        t_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let output_start = Self::point_at_parameter(line_start, line_end, t_values[0]);
        let output_end =
            Self::point_at_parameter(line_start, line_end, t_values[t_values.len() - 1]);

        if (t_values[0] - t_values[t_values.len() - 1]).abs() > Tolerance::ZERO_TOLERANCE {
            Some((output_start, output_end))
        } else {
            None
        }
    }

    /// Find closest distance and point from a point to this polyline
    pub fn closest_distance_and_point(&self, point: &Point) -> (f64, usize, Point) {
        let mut edge_id = 0;
        let mut closest_distance = f64::MAX;
        let mut best_t = 0.0;

        for i in 0..self.segment_count() {
            let t = Self::closest_point_to_line(point, &self.points[i], &self.points[i + 1]);
            let point_on_segment =
                Self::point_at_parameter(&self.points[i], &self.points[i + 1], t);
            let distance = point.distance(&point_on_segment);

            if distance < closest_distance {
                closest_distance = distance;
                edge_id = i;
                best_t = t;
            }

            if closest_distance < Tolerance::ZERO_TOLERANCE {
                break;
            }
        }

        let closest_point =
            Self::point_at_parameter(&self.points[edge_id], &self.points[edge_id + 1], best_t);
        (closest_distance, edge_id, closest_point)
    }

    /// Check if polyline is closed (first and last points are the same)
    pub fn is_closed(&self) -> bool {
        if self.points.len() < 2 {
            return false;
        }
        self.points
            .first()
            .unwrap()
            .distance(self.points.last().unwrap())
            < Tolerance::ZERO_TOLERANCE
    }

    /// Calculate center point of polyline
    pub fn center(&self) -> Point {
        if self.points.is_empty() {
            return Point::new(0.0, 0.0, 0.0);
        }

        let n = if self.is_closed() && self.points.len() > 1 {
            self.points.len() - 1
        } else {
            self.points.len()
        };

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;

        for i in 0..n {
            sum_x += self.points[i].x();
            sum_y += self.points[i].y();
            sum_z += self.points[i].z();
        }

        Point::new(sum_x / n as f64, sum_y / n as f64, sum_z / n as f64)
    }

    /// Calculate center as vector
    pub fn center_vec(&self) -> Vector {
        let center = self.center();
        Vector::new(center.x(), center.y(), center.z())
    }

    /// Get average plane from polyline points
    pub fn get_average_plane(&self) -> (Point, Vector, Vector, Vector) {
        let origin = self.center();

        let x_axis = if self.points.len() >= 2 {
            let mut x = self.points[1].clone() - self.points[0].clone();
            x.normalize_self();
            x
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };

        let z_axis = self.average_normal();
        let mut y_axis = z_axis.cross(&x_axis);
        y_axis.normalize_self();

        (origin, x_axis, y_axis, z_axis)
    }

    /// Get fast plane calculation from polyline
    pub fn get_fast_plane(&self) -> (Point, Plane) {
        let origin = if !self.points.is_empty() {
            self.points[0].clone()
        } else {
            Point::new(0.0, 0.0, 0.0)
        };

        let average_normal = self.average_normal();
        let plane = Plane::from_point_normal(origin.clone(), average_normal);
        (origin, plane)
    }

    /// Calculate middle line between two line segments
    pub fn get_middle_line(
        line0_start: &Point,
        line0_end: &Point,
        line1_start: &Point,
        line1_end: &Point,
    ) -> (Point, Point) {
        let p0 = Point::new(
            (line0_start.x() + line1_start.x()) * 0.5,
            (line0_start.y() + line1_start.y()) * 0.5,
            (line0_start.z() + line1_start.z()) * 0.5,
        );
        let p1 = Point::new(
            (line0_end.x() + line1_end.x()) * 0.5,
            (line0_end.y() + line1_end.y()) * 0.5,
            (line0_end.z() + line1_end.z()) * 0.5,
        );
        (p0, p1)
    }

    /// Extend line segment by specified distances at both ends
    pub fn extend_line(
        line_start: &mut Point,
        line_end: &mut Point,
        distance0: f64,
        distance1: f64,
    ) {
        let mut v = line_end.clone() - line_start.clone();
        v.normalize_self();

        *line_start = line_start.clone() - (v.clone() * distance0);
        *line_end = line_end.clone() + (v * distance1);
    }

    /// Scale line segment inward by specified distance
    pub fn scale_line(line_start: &mut Point, line_end: &mut Point, distance: f64) {
        let v = line_end.clone() - line_start.clone();
        *line_start = line_start.clone() + (v.clone() * distance);
        *line_end = line_end.clone() - (v * distance);
    }

    /// Extend polyline segment
    pub fn extend_segment(
        &mut self,
        segment_id: usize,
        dist0: f64,
        dist1: f64,
        proportion0: f64,
        proportion1: f64,
    ) {
        if segment_id >= self.segment_count() {
            return;
        }

        let mut p0 = self.points[segment_id].clone();
        let mut p1 = self.points[segment_id + 1].clone();
        let v = p1.clone() - p0.clone();

        if proportion0 != 0.0 || proportion1 != 0.0 {
            p0 -= v.clone() * proportion0;
            p1 += v * proportion1;
        } else {
            let v_norm = v.normalize();
            p0 -= v_norm.clone() * dist0;
            p1 += v_norm * dist1;
        }

        self.points[segment_id] = p0;
        self.points[segment_id + 1] = p1;

        if self.is_closed() {
            let len = self.points.len();
            if segment_id == 0 {
                let first = self.points[0].clone();
                self.points[len - 1] = first;
            } else if segment_id + 1 == len - 1 {
                let last = self.points[len - 1].clone();
                self.points[0] = last;
            }
        }
    }

    /// Extend segment equally on both ends (static utility)
    pub fn extend_segment_equally_static(
        segment_start: &mut Point,
        segment_end: &mut Point,
        dist: f64,
        proportion: f64,
    ) {
        if dist == 0.0 && proportion == 0.0 {
            return;
        }

        let v = segment_end.clone() - segment_start.clone();

        if proportion != 0.0 {
            *segment_start = segment_start.clone() - (v.clone() * proportion);
            *segment_end = segment_end.clone() + (v * proportion);
        } else {
            let mut v_norm = v;
            v_norm.normalize_self();
            *segment_start = segment_start.clone() - (v_norm.clone() * dist);
            *segment_end = segment_end.clone() + (v_norm * dist);
        }
    }

    /// Extend polyline segment equally
    pub fn extend_segment_equally(&mut self, segment_id: usize, dist: f64, proportion: f64) {
        if segment_id >= self.segment_count() {
            return;
        }

        // Extract points to avoid borrowing issues
        let mut start = self.points[segment_id].clone();
        let mut end = self.points[segment_id + 1].clone();
        Self::extend_segment_equally_static(&mut start, &mut end, dist, proportion);
        self.points[segment_id] = start;
        self.points[segment_id + 1] = end;

        if self.points.len() > 2 && self.is_closed() {
            let len = self.points.len();
            if segment_id == 0 {
                self.points[len - 1] = self.points[0].clone();
            } else if segment_id + 1 == len - 1 {
                self.points[0] = self.points[len - 1].clone();
            }
        }
    }

    /// Move polyline by direction vector
    pub fn move_by(&mut self, direction: &Vector) {
        for point in &mut self.points {
            *point += direction.clone();
        }
    }

    /// Check if polyline is clockwise oriented
    pub fn is_clockwise(&self, _plane: &Plane) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut sum = 0.0;
        let n = if self.is_closed() {
            self.points.len() - 1
        } else {
            self.points.len()
        };

        for i in 0..n {
            let current = &self.points[i];
            let next = &self.points[(i + 1) % n];
            sum += (next.x() - current.x()) * (next.y() + current.y());
        }

        sum > 0.0
    }

    /// Flip polyline direction (reverse point order)
    pub fn flip(&mut self) {
        self.points.reverse();
    }

    /// Get convex/concave corners of polyline
    pub fn get_convex_corners(&self) -> Vec<bool> {
        if self.points.len() < 3 {
            return Vec::new();
        }

        let closed = self.is_closed();
        let normal = self.average_normal();
        let n = if closed {
            self.points.len() - 1
        } else {
            self.points.len()
        };
        let mut convex_corners = Vec::with_capacity(n);

        for current in 0..n {
            let prev = if current == 0 { n - 1 } else { current - 1 };
            let next = if current == n - 1 { 0 } else { current + 1 };

            let mut dir0 = self.points[current].clone() - self.points[prev].clone();
            dir0.normalize_self();

            let mut dir1 = self.points[next].clone() - self.points[current].clone();
            dir1.normalize_self();

            let mut cross = dir0.cross(&dir1);
            cross.normalize_self();

            let dot = cross.dot(&normal);
            let is_convex = dot >= 0.0;
            convex_corners.push(is_convex);
        }

        convex_corners
    }

    /// Interpolate between two polylines
    pub fn tween_two_polylines(
        polyline0: &Polyline,
        polyline1: &Polyline,
        weight: f64,
    ) -> Polyline {
        if polyline0.points.len() != polyline1.points.len() {
            return polyline0.clone();
        }

        let mut result = Polyline::default();
        result.points.reserve(polyline0.points.len());

        for i in 0..polyline0.points.len() {
            let diff = polyline1.points[i].clone() - polyline0.points[i].clone();
            let interpolated = polyline0.points[i].clone() + (diff * weight);
            result.points.push(interpolated);
        }

        result
    }

    /// Calculate average normal from polyline points
    fn average_normal(&self) -> Vector {
        let len = self.points.len();
        if len < 3 {
            return Vector::new(0.0, 0.0, 1.0);
        }

        let closed = self.is_closed();
        let n = if closed && len > 1 { len - 1 } else { len };

        let mut average_normal = Vector::new(0.0, 0.0, 0.0);

        for i in 0..n {
            let prev = if i == 0 { n - 1 } else { i - 1 };
            let next = (i + 1) % n;

            let v1 = self.points[prev].clone() - self.points[i].clone();
            let v2 = self.points[i].clone() - self.points[next].clone();
            let cross = v1.cross(&v2);
            average_normal += &cross;
        }

        average_normal.normalize_self();
        average_normal
    }
}

impl AddAssign<&Vector> for Polyline {
    /// Translates all points in the polyline by a vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The translation vector.
    fn add_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            *p += other.clone();
        }
        // Update plane origin
        self.plane = Plane::new(
            self.plane.origin() + other.clone(),
            self.plane.x_axis(),
            self.plane.y_axis(),
        );
    }
}

impl Add<&Vector> for Polyline {
    type Output = Polyline;

    /// Translates the polyline by a vector and returns a new polyline.
    ///
    /// # Arguments
    ///
    /// * `other` - The translation vector.
    fn add(self, other: &Vector) -> Polyline {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl SubAssign<&Vector> for Polyline {
    /// Translates all points in the polyline by the negative of a vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector to subtract.
    fn sub_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            *p -= other.clone();
        }
        // Update plane origin
        self.plane = Plane::new(
            self.plane.origin() - other.clone(),
            self.plane.x_axis(),
            self.plane.y_axis(),
        );
    }
}

impl Sub<&Vector> for Polyline {
    type Output = Polyline;

    /// Translates the polyline by the negative of a vector and returns a new polyline.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector to subtract.
    fn sub(self, other: &Vector) -> Polyline {
        let mut result = self.clone();
        result -= other;
        result
    }
}

impl fmt::Display for Polyline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Polyline(guid={}, name={}, points={})",
            self.guid,
            self.name,
            self.points.len()
        )
    }
}

#[cfg(test)]
#[path = "polyline_test.rs"]
mod polyline_test;
