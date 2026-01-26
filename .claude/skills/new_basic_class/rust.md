# New Basic Class - Rust Template

## Implementation (src/name.rs)

```rust
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::ops::{Index, IndexMut};
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
pub struct Name {
    pub guid: String,
    pub name: String,
    #[serde(rename = "x")]
    _x: f64,
    #[serde(rename = "y")]
    _y: f64,
    #[serde(rename = "z")]
    _z: f64,
}

impl Name {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_name".to_string(),
            _x: x,
            _y: y,
            _z: z,
        }
    }

    pub fn str(&self) -> String {
        format!("Name({}, {}, {})", self._x, self._y, self._z)
    }

    pub fn repr(&self) -> String {
        format!("Name(\n  name={},\n  x={},\n  y={},\n  z={}\n)",
                self.name, self._x, self._y, self._z)
    }

    pub fn is_valid(&self) -> bool {
        !self._x.is_nan() && !self._y.is_nan() && !self._z.is_nan()
    }

    pub fn duplicate(&self) -> Self {
        let mut copy = self.clone();
        copy.guid = Uuid::new_v4().to_string();
        copy
    }

    pub fn jsondump(&self) -> serde_json::Value {
        serde_json::json!({
            "guid": self.guid,
            "name": self.name,
            "type": "Name",
            "x": self._x,
            "y": self._y,
            "z": self._z
        })
    }

    pub fn json_dump(&self, filename: &str) {
        let json = serde_json::to_string_pretty(&self.jsondump()).unwrap();
        fs::write(filename, json).unwrap();
    }

    pub fn json_load(filename: &str) -> Self {
        let data = fs::read_to_string(filename).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}

impl Default for Name {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Index<usize> for Name {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self._x,
            1 => &self._y,
            2 => &self._z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Name {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self._x,
            1 => &mut self._y,
            2 => &mut self._z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self._x == other._x && self._y == other._y && self._z == other._z
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}
```

## lib.rs Registration

```rust
pub mod name;
pub mod name_test;
pub use name::Name;
```
