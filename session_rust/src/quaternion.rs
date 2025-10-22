use crate::Vector;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Mul;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Quaternion {
    pub typ: String,
    pub guid: String,
    pub name: String,
    pub s: f64,
    pub v: Vector,
}

// Custom serialization to flatten vector as x, y, z only
impl Serialize for Quaternion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Quaternion", 6)?;
        state.serialize_field("type", &self.typ)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("s", &self.s)?;
        state.serialize_field("x", &self.v.x())?;
        state.serialize_field("y", &self.v.y())?;
        state.serialize_field("z", &self.v.z())?;
        state.end()
    }
}

// Custom deserialization
impl<'de> Deserialize<'de> for Quaternion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct QuaternionHelper {
            #[serde(rename = "type")]
            typ: String,
            guid: String,
            name: String,
            s: f64,
            x: f64,
            y: f64,
            z: f64,
        }

        let helper = QuaternionHelper::deserialize(deserializer)?;
        Ok(Quaternion {
            typ: helper.typ,
            guid: helper.guid,
            name: helper.name,
            s: helper.s,
            v: Vector::new(helper.x, helper.y, helper.z),
        })
    }
}

impl Quaternion {
    pub fn new(s: f64, v: Vector) -> Self {
        Quaternion {
            typ: "Quaternion".to_string(),
            guid: Uuid::new_v4().to_string(),
            name: "my_quaternion".to_string(),
            s,
            v,
        }
    }

    pub fn from_sv(s: f64, x: f64, y: f64, z: f64) -> Self {
        Quaternion {
            typ: "Quaternion".to_string(),
            guid: Uuid::new_v4().to_string(),
            name: "my_quaternion".to_string(),
            s,
            v: Vector::new(x, y, z),
        }
    }

    pub fn identity() -> Self {
        Quaternion {
            typ: "Quaternion".to_string(),
            guid: Uuid::new_v4().to_string(),
            name: "my_quaternion".to_string(),
            s: 1.0,
            v: Vector::new(0.0, 0.0, 0.0),
        }
    }

    pub fn from_axis_angle(axis: Vector, angle: f64) -> Self {
        let axis = axis.normalize();
        let half_angle = angle * 0.5;
        let s = half_angle.cos();
        let v = axis * half_angle.sin();
        Quaternion {
            typ: "Quaternion".to_string(),
            guid: Uuid::new_v4().to_string(),
            name: "my_quaternion".to_string(),
            s,
            v,
        }
    }

    pub fn rotate_vector(&self, v: Vector) -> Vector {
        let qv = self.v.clone();
        let uv = qv.cross(&v);
        let uuv = qv.cross(&uv);
        v + (uv * self.s + uuv) * 2.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.s * self.s
            + self.v.x() * self.v.x()
            + self.v.y() * self.v.y()
            + self.v.z() * self.v.z())
        .sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 1e-10 {
            Quaternion {
                typ: self.typ.clone(),
                guid: self.guid.clone(),
                name: self.name.clone(),
                s: self.s / mag,
                v: self.v.clone() / mag,
            }
        } else {
            Self::identity()
        }
    }

    pub fn conjugate(&self) -> Self {
        Quaternion {
            typ: self.typ.clone(),
            guid: self.guid.clone(),
            name: self.name.clone(),
            s: self.s,
            v: self.v.clone() * -1.0,
        }
    }

    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_data = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json_data)
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        let s = self.s * rhs.s - self.v.dot(&rhs.v);
        let v = rhs.v.clone() * self.s + self.v.clone() * rhs.s + self.v.cross(&rhs.v);
        Quaternion {
            typ: "Quaternion".to_string(),
            guid: Uuid::new_v4().to_string(),
            name: "my_quaternion".to_string(),
            s,
            v,
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
#[path = "quaternion_test.rs"]
mod tests;
