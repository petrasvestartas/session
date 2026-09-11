# New Visual Class - Rust Template

Extends basic class with visual properties.

## Struct Additions (src/name.rs)

```rust
use crate::{Color, Xform};

#[derive(Clone, Serialize, Deserialize)]
pub struct Name {
    // ... basic fields ...
    pub width: f64,
    pub color: Color,
    pub xform: Xform,
}

impl Name {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_name".to_string(),
            _x: x,
            _y: y,
            _z: z,
            width: 1.0,
            color: Color::red(),
            xform: Xform::identity(),
        }
    }

    pub fn duplicate(&self) -> Self {
        let mut copy = self.clone();
        copy.guid = Uuid::new_v4().to_string();
        copy
    }

    pub fn transform(&mut self) {
        let m = &self.xform;
        let nx = m[0]*self._x + m[1]*self._y + m[2]*self._z + m[3];
        let ny = m[4]*self._x + m[5]*self._y + m[6]*self._z + m[7];
        let nz = m[8]*self._x + m[9]*self._y + m[10]*self._z + m[11];

        self._x = nx;
        self._y = ny;
        self._z = nz;
        self.xform = Xform::identity();
    }

    pub fn transformed(&self) -> Self {
        let mut result = self.duplicate();
        result.transform();
        result
    }
}
```

## JSON Additions

```rust
pub fn jsondump(&self) -> serde_json::Value {
    serde_json::json!({
        "color": self.color.jsondump(),
        "guid": self.guid,
        "name": self.name,
        "type": "Name",
        "width": self.width,
        "x": self._x,
        "xform": self.xform.jsondump(),
        "y": self._y,
        "z": self._z
    })
}
```

## Test Additions

```rust
pub fn run_name_transformation() -> TestResult {
    MINI_TEST!("transformation", {
        use crate::{Name, Xform};

        let mut obj = Name::new(1.0, 2.0, 3.0);
        obj.xform = Xform::translation(10.0, 0.0, 0.0);

        let copy = obj.transformed();
        MINI_CHECK!(copy[0] == 11.0);
        MINI_CHECK!(obj[0] == 1.0);  // Original unchanged

        obj.transform();
        MINI_CHECK!(obj[0] == 11.0);
    })
}
```
