use crate::{Color, Point, Vector, Xform};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PointCloud {
    pub guid: String,
    pub name: String,
    pub points: Vec<Point>,
    pub normals: Vec<Vector>,
    pub colors: Vec<Color>,
    pub xform: Xform,
}

impl Default for PointCloud {
    fn default() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_pointcloud".to_string(),
            points: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            xform: Xform::identity(),
        }
    }
}

impl PointCloud {
    pub fn new(points: Vec<Point>, normals: Vec<Vector>, colors: Vec<Color>) -> Self {
        Self {
            points,
            normals,
            colors,
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformation
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn transform(&mut self) {
        let xform = self.xform.clone();
        for pt in &mut self.points {
            xform.transform_point(pt);
        }
        for n in &mut self.normals {
            xform.transform_vector(n);
        }
        self.xform = Xform::identity();
    }

    pub fn transformed(&self) -> Self {
        let mut result = self.clone();
        result.transform();
        result
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        serde::Serialize::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    pub fn jsonload(json_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_str)?)
    }

    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = self.jsondump()?;
        std::fs::write(filepath, json_str)?;
        Ok(())
    }

    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_str = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json_str)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// No-copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

impl AddAssign<Vector> for PointCloud {
    fn add_assign(&mut self, other: Vector) {
        for p in &mut self.points {
            *p += other.clone();
        }
    }
}

impl SubAssign<Vector> for PointCloud {
    fn sub_assign(&mut self, other: Vector) {
        for p in &mut self.points {
            *p -= other.clone();
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// Copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

impl Add<Vector> for PointCloud {
    type Output = PointCloud;

    fn add(self, other: Vector) -> PointCloud {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl Sub<Vector> for PointCloud {
    type Output = PointCloud;

    fn sub(self, other: Vector) -> PointCloud {
        let mut result = self.clone();
        result -= other;
        result
    }
}

impl fmt::Display for PointCloud {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PointCloud(points={}, normals={}, colors={}, guid={}, name={})",
            self.points.len(),
            self.normals.len(),
            self.colors.len(),
            self.guid,
            self.name
        )
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// Custom Serialization - Flat arrays for efficiency
///////////////////////////////////////////////////////////////////////////////////////////

impl Serialize for PointCloud {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("PointCloud", 6)?;

        state.serialize_field("type", "PointCloud")?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("name", &self.name)?;

        // Flatten points to [x, y, z, x, y, z, ...]
        let points_flat: Vec<f64> = self
            .points
            .iter()
            .flat_map(|p| vec![p.x(), p.y(), p.z()])
            .collect();
        state.serialize_field("points", &points_flat)?;

        // Flatten normals to [x, y, z, x, y, z, ...]
        let normals_flat: Vec<f64> = self
            .normals
            .iter()
            .flat_map(|n| vec![n.x(), n.y(), n.z()])
            .collect();
        state.serialize_field("normals", &normals_flat)?;

        // Flatten colors to [r, g, b, r, g, b, ...] (no alpha)
        let colors_flat: Vec<u8> = self
            .colors
            .iter()
            .flat_map(|c| vec![c.r, c.g, c.b])
            .collect();
        state.serialize_field("colors", &colors_flat)?;

        state.serialize_field("xform", &self.xform)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for PointCloud {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Type,
            Guid,
            Name,
            Points,
            Normals,
            Colors,
            Xform,
        }

        struct PointCloudVisitor;

        impl<'de> Visitor<'de> for PointCloudVisitor {
            type Value = PointCloud;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PointCloud")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PointCloud, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut guid = None;
                let mut name = None;
                let mut points_flat: Option<Vec<f64>> = None;
                let mut normals_flat: Option<Vec<f64>> = None;
                let mut colors_flat: Option<Vec<u8>> = None;
                let mut xform = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            let _: String = map.next_value()?;
                        }
                        Field::Guid => {
                            guid = Some(map.next_value()?);
                        }
                        Field::Name => {
                            name = Some(map.next_value()?);
                        }
                        Field::Points => {
                            points_flat = Some(map.next_value()?);
                        }
                        Field::Normals => {
                            normals_flat = Some(map.next_value()?);
                        }
                        Field::Colors => {
                            colors_flat = Some(map.next_value()?);
                        }
                        Field::Xform => {
                            xform = Some(map.next_value()?);
                        }
                    }
                }

                let guid = guid.ok_or_else(|| de::Error::missing_field("guid"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let points_flat = points_flat.ok_or_else(|| de::Error::missing_field("points"))?;
                let normals_flat =
                    normals_flat.ok_or_else(|| de::Error::missing_field("normals"))?;
                let colors_flat = colors_flat.ok_or_else(|| de::Error::missing_field("colors"))?;
                let xform = xform.ok_or_else(|| de::Error::missing_field("xform"))?;

                // Reconstruct points from flat array
                let points: Vec<Point> = points_flat
                    .chunks(3)
                    .map(|chunk| Point::new(chunk[0], chunk[1], chunk[2]))
                    .collect();

                // Reconstruct normals from flat array
                let normals: Vec<Vector> = normals_flat
                    .chunks(3)
                    .map(|chunk| Vector::new(chunk[0], chunk[1], chunk[2]))
                    .collect();

                // Reconstruct colors from flat array (RGB only, alpha always 255)
                let colors: Vec<Color> = colors_flat
                    .chunks(3)
                    .map(|chunk| Color::new(chunk[0], chunk[1], chunk[2], 255))
                    .collect();

                Ok(PointCloud {
                    guid,
                    name,
                    points,
                    normals,
                    colors,
                    xform,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "type", "guid", "name", "points", "normals", "colors", "xform",
        ];
        deserializer.deserialize_struct("PointCloud", FIELDS, PointCloudVisitor)
    }
}

#[cfg(test)]
#[path = "pointcloud_test.rs"]
mod pointcloud_test;
