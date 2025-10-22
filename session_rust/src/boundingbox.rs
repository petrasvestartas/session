use crate::{Plane, Point, Vector, Xform};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "BoundingBox")]
pub struct BoundingBox {
    pub center: Point,
    pub x_axis: Vector,
    pub y_axis: Vector,
    pub z_axis: Vector,
    pub half_size: Vector,
    pub guid: String,
    pub name: String,
    #[serde(default = "Xform::identity")]
    pub xform: Xform,
}

impl BoundingBox {
    pub fn new(
        center: Point,
        x_axis: Vector,
        y_axis: Vector,
        z_axis: Vector,
        half_size: Vector,
    ) -> Self {
        BoundingBox {
            center,
            x_axis,
            y_axis,
            z_axis,
            half_size,
            guid: Uuid::new_v4().to_string(),
            name: "my_boundingbox".to_string(),
            xform: Xform::identity(),
        }
    }

    pub fn from_plane(plane: &Plane, dx: f64, dy: f64, dz: f64) -> Self {
        BoundingBox {
            center: plane.origin(),
            x_axis: plane.x_axis(),
            y_axis: plane.y_axis(),
            z_axis: plane.z_axis(),
            half_size: Vector::new(dx * 0.5, dy * 0.5, dz * 0.5),
            guid: Uuid::new_v4().to_string(),
            name: String::new(),
            xform: Xform::identity(),
        }
    }

    pub fn from_point(point: Point, inflate: f64) -> Self {
        BoundingBox {
            center: point,
            x_axis: Vector::new(1.0, 0.0, 0.0),
            y_axis: Vector::new(0.0, 1.0, 0.0),
            z_axis: Vector::new(0.0, 0.0, 1.0),
            half_size: Vector::new(inflate, inflate, inflate),
            guid: Uuid::new_v4().to_string(),
            xform: Xform::identity(),
            name: String::new(),
        }
    }

    pub fn from_points(points: &[Point], inflate: f64) -> Self {
        if points.is_empty() {
            return BoundingBox::default();
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for pt in points {
            min_x = min_x.min(pt.x());
            min_y = min_y.min(pt.y());
            min_z = min_z.min(pt.z());
            max_x = max_x.max(pt.x());
            max_y = max_y.max(pt.y());
            max_z = max_z.max(pt.z());
        }

        let center = Point::new(
            (min_x + max_x) * 0.5,
            (min_y + max_y) * 0.5,
            (min_z + max_z) * 0.5,
        );
        let half_size = Vector::new(
            (max_x - min_x) * 0.5 + inflate,
            (max_y - min_y) * 0.5 + inflate,
            (max_z - min_z) * 0.5 + inflate,
        );

        BoundingBox {
            center,
            x_axis: Vector::new(1.0, 0.0, 0.0),
            y_axis: Vector::new(0.0, 1.0, 0.0),
            z_axis: Vector::new(0.0, 0.0, 1.0),
            half_size,
            guid: Uuid::new_v4().to_string(),
            name: String::new(),
            xform: Xform::identity(),
        }
    }

    pub fn from_line(line: &crate::line::Line, inflate: f64) -> Self {
        let points = vec![line.start(), line.end()];
        Self::from_points(&points, inflate)
    }

    pub fn from_polyline(polyline: &crate::polyline::Polyline, inflate: f64) -> Self {
        Self::from_points(&polyline.points, inflate)
    }

    pub fn point_at(&self, x: f64, y: f64, z: f64) -> Point {
        Point::new(
            self.center.x() + x * self.x_axis.x() + y * self.y_axis.x() + z * self.z_axis.x(),
            self.center.y() + x * self.x_axis.y() + y * self.y_axis.y() + z * self.z_axis.y(),
            self.center.z() + x * self.x_axis.z() + y * self.y_axis.z() + z * self.z_axis.z(),
        )
    }

    pub fn min_point(&self) -> Point {
        Point::new(
            self.center.x() - self.half_size.x(),
            self.center.y() - self.half_size.y(),
            self.center.z() - self.half_size.z(),
        )
    }

    pub fn max_point(&self) -> Point {
        Point::new(
            self.center.x() + self.half_size.x(),
            self.center.y() + self.half_size.y(),
            self.center.z() + self.half_size.z(),
        )
    }

    pub fn corners(&self) -> [Point; 8] {
        [
            self.point_at(self.half_size.x(), self.half_size.y(), -self.half_size.z()),
            self.point_at(-self.half_size.x(), self.half_size.y(), -self.half_size.z()),
            self.point_at(
                -self.half_size.x(),
                -self.half_size.y(),
                -self.half_size.z(),
            ),
            self.point_at(self.half_size.x(), -self.half_size.y(), -self.half_size.z()),
            self.point_at(self.half_size.x(), self.half_size.y(), self.half_size.z()),
            self.point_at(-self.half_size.x(), self.half_size.y(), self.half_size.z()),
            self.point_at(-self.half_size.x(), -self.half_size.y(), self.half_size.z()),
            self.point_at(self.half_size.x(), -self.half_size.y(), self.half_size.z()),
        ]
    }

    pub fn two_rectangles(&self) -> [Point; 10] {
        [
            self.point_at(self.half_size.x(), self.half_size.y(), -self.half_size.z()),
            self.point_at(-self.half_size.x(), self.half_size.y(), -self.half_size.z()),
            self.point_at(
                -self.half_size.x(),
                -self.half_size.y(),
                -self.half_size.z(),
            ),
            self.point_at(self.half_size.x(), -self.half_size.y(), -self.half_size.z()),
            self.point_at(self.half_size.x(), self.half_size.y(), -self.half_size.z()),
            self.point_at(self.half_size.x(), self.half_size.y(), self.half_size.z()),
            self.point_at(-self.half_size.x(), self.half_size.y(), self.half_size.z()),
            self.point_at(-self.half_size.x(), -self.half_size.y(), self.half_size.z()),
            self.point_at(self.half_size.x(), -self.half_size.y(), self.half_size.z()),
            self.point_at(self.half_size.x(), self.half_size.y(), self.half_size.z()),
        ]
    }

    pub fn inflate(&mut self, amount: f64) {
        self.half_size = Vector::new(
            self.half_size.x() + amount,
            self.half_size.y() + amount,
            self.half_size.z() + amount,
        );
    }

    fn separating_plane_exists(
        relative_position: &Vector,
        axis: &Vector,
        box1: &BoundingBox,
        box2: &BoundingBox,
    ) -> bool {
        let dot_rp = relative_position.dot(axis).abs();

        let v1 = box1.x_axis.clone() * box1.half_size.x();
        let v2 = box1.y_axis.clone() * box1.half_size.y();
        let v3 = box1.z_axis.clone() * box1.half_size.z();
        let proj1 = v1.dot(axis).abs() + v2.dot(axis).abs() + v3.dot(axis).abs();

        let v4 = box2.x_axis.clone() * box2.half_size.x();
        let v5 = box2.y_axis.clone() * box2.half_size.y();
        let v6 = box2.z_axis.clone() * box2.half_size.z();
        let proj2 = v4.dot(axis).abs() + v5.dot(axis).abs() + v6.dot(axis).abs();

        dot_rp > (proj1 + proj2)
    }

    pub fn collides_with(&self, other: &BoundingBox) -> bool {
        let center_vec = Vector::new(self.center.x(), self.center.y(), self.center.z());
        let other_center_vec = Vector::new(other.center.x(), other.center.y(), other.center.z());
        let relative_position = Vector::from_start_and_end(&center_vec, &other_center_vec);

        !(Self::separating_plane_exists(&relative_position, &self.x_axis, self, other)
            || Self::separating_plane_exists(&relative_position, &self.y_axis, self, other)
            || Self::separating_plane_exists(&relative_position, &self.z_axis, self, other)
            || Self::separating_plane_exists(&relative_position, &other.x_axis, self, other)
            || Self::separating_plane_exists(&relative_position, &other.y_axis, self, other)
            || Self::separating_plane_exists(&relative_position, &other.z_axis, self, other)
            || Self::separating_plane_exists(
                &relative_position,
                &self.x_axis.cross(&other.x_axis),
                self,
                other,
            )
            || Self::separating_plane_exists(
                &relative_position,
                &self.x_axis.cross(&other.y_axis),
                self,
                other,
            )
            || Self::separating_plane_exists(
                &relative_position,
                &self.x_axis.cross(&other.z_axis),
                self,
                other,
            )
            || Self::separating_plane_exists(
                &relative_position,
                &self.y_axis.cross(&other.x_axis),
                self,
                other,
            )
            || Self::separating_plane_exists(
                &relative_position,
                &self.y_axis.cross(&other.y_axis),
                self,
                other,
            )
            || Self::separating_plane_exists(
                &relative_position,
                &self.y_axis.cross(&other.z_axis),
                self,
                other,
            )
            || Self::separating_plane_exists(
                &relative_position,
                &self.z_axis.cross(&other.x_axis),
                self,
                other,
            )
            || Self::separating_plane_exists(
                &relative_position,
                &self.z_axis.cross(&other.y_axis),
                self,
                other,
            )
            || Self::separating_plane_exists(
                &relative_position,
                &self.z_axis.cross(&other.z_axis),
                self,
                other,
            ))
    }

    pub fn transform(&mut self) {
        let xform = self.xform.clone();
        xform.transform_point(&mut self.center);
        xform.transform_vector(&mut self.x_axis);
        xform.transform_vector(&mut self.y_axis);
        xform.transform_vector(&mut self.z_axis);
        self.xform = Xform::identity();
    }

    pub fn transformed(&self) -> Self {
        let mut result = self.clone();
        result.transform();
        result
    }

    pub fn jsondump(&self) -> Result<String, std::boxed::Box<dyn std::error::Error>> {
        let data = serde_json::json!({
            "type": "BoundingBox",
            "center": serde_json::from_str::<serde_json::Value>(&self.center.jsondump()?)?,
            "x_axis": serde_json::from_str::<serde_json::Value>(&self.x_axis.jsondump()?)?,
            "y_axis": serde_json::from_str::<serde_json::Value>(&self.y_axis.jsondump()?)?,
            "z_axis": serde_json::from_str::<serde_json::Value>(&self.z_axis.jsondump()?)?,
            "half_size": serde_json::from_str::<serde_json::Value>(&self.half_size.jsondump()?)?,
            "guid": self.guid,
            "name": self.name,
        });
        Ok(serde_json::to_string(&data)?)
    }

    pub fn jsonload(json_data: &str) -> Result<Self, std::boxed::Box<dyn std::error::Error>> {
        let data: serde_json::Value = serde_json::from_str(json_data)?;
        let mut bbox = BoundingBox::new(
            Point::jsonload(&data["center"].to_string())?,
            Vector::jsonload(&data["x_axis"].to_string())?,
            Vector::jsonload(&data["y_axis"].to_string())?,
            Vector::jsonload(&data["z_axis"].to_string())?,
            Vector::jsonload(&data["half_size"].to_string())?,
        );
        bbox.guid = data["guid"].as_str().unwrap().to_string();
        bbox.name = data["name"].as_str().unwrap().to_string();
        Ok(bbox)
    }

    pub fn to_json(&self, filepath: &str) -> Result<(), std::boxed::Box<dyn std::error::Error>> {
        let json_string = self.jsondump()?;
        let value: serde_json::Value = serde_json::from_str(&json_string)?;
        let pretty = serde_json::to_string_pretty(&value)?;
        std::fs::write(filepath, pretty)?;
        Ok(())
    }

    pub fn from_json(filepath: &str) -> Result<Self, std::boxed::Box<dyn std::error::Error>> {
        let json_string = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json_string)
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            center: Point::new(0.0, 0.0, 0.0),
            x_axis: Vector::new(1.0, 0.0, 0.0),
            y_axis: Vector::new(0.0, 1.0, 0.0),
            z_axis: Vector::new(0.0, 0.0, 1.0),
            half_size: Vector::new(0.5, 0.5, 0.5),
            guid: Uuid::new_v4().to_string(),
            name: String::new(),
            xform: Xform::identity(),
        }
    }
}

#[cfg(test)]
#[path = "boundingbox_test.rs"]
mod boundingbox_test;
