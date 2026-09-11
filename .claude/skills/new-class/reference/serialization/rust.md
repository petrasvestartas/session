# Serialization - Rust

## Import Convention

Each import on a separate line. Never combine multiple symbols using `{...}`.

**Wrong:**
```rust
use crate::{ConvexHull, Point};
use crate::{BVH, OBB, Point, Vector};
```

**Correct:**
```rust
use crate::ConvexHull;
use crate::Point;
```

## JSON (serde_json)

Rust uses serde `Serialize`/`Deserialize` traits. The struct derives these traits,
and custom `Serialize`/`Deserialize` impls control field order and naming.

```rust
use serde::{Serialize, Deserialize};

impl ClassName {
    // Core: string serialization
    pub fn json_dumps(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    // Core: string deserialization
    pub fn json_loads(json_string: &str) -> Self {
        serde_json::from_str(json_string).unwrap_or_else(|_| Self::default())
    }

    // File wrappers
    pub fn json_dump(&self, filename: &str) {
        use std::fs::File;
        use std::io::Write;
        if let Ok(json) = serde_json::to_string_pretty(self) {
            if let Ok(mut file) = File::create(filename) {
                let _ = file.write_all(json.as_bytes());
            }
        }
    }

    pub fn json_load(filename: &str) -> Self {
        use std::fs::File;
        use std::io::Read;
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return Self::default();
        }
        serde_json::from_str(&contents).unwrap_or_else(|_| Self::default())
    }
}
```

### Custom Serialize/Deserialize

For complex types (NurbsCurve, NurbsSurface) that need custom field ordering:

```rust
impl Serialize for ClassName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        use serde::ser::SerializeMap;
        // Fields in alphabetical order
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("guid", &self.guid)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("x", &self._x)?;
        map.end()
    }
}
```

## Protobuf (prost)

```rust
use prost::Message;

impl ClassName {
    // Core: binary serialization
    pub fn pb_dumps(&self) -> Vec<u8> {
        let msg = crate::proto::ClassName {
            guid: self.guid.clone(),
            name: self.name.clone(),
            x: self._x,
            y: self._y,
            z: self._z,
        };
        msg.encode_to_vec()
    }

    // Core: binary deserialization
    pub fn pb_loads(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let msg = crate::proto::ClassName::decode(data)?;
        let mut obj = Self::default();
        obj.guid = msg.guid;
        obj.name = msg.name;
        obj._x = msg.x;
        obj._y = msg.y;
        obj._z = msg.z;
        Ok(obj)
    }

    // File wrappers
    pub fn pb_dump(&self, filename: &str) {
        let data = self.pb_dumps();
        std::fs::write(filename, data).expect("Failed to write protobuf file");
    }

    pub fn pb_load(filename: &str) -> Self {
        let data = std::fs::read(filename).expect("Failed to read protobuf file");
        Self::pb_loads(&data).expect("Failed to parse protobuf")
    }
}
```

## Nested Objects

```rust
// In Serialize impl:
map.serialize_entry("linecolor", &self.linecolor)?;
map.serialize_entry("xform", &self.xform)?;

// In pb_dumps():
let proto = crate::proto::ClassName {
    linecolor: Some(crate::proto::Color {
        r: self.linecolor[0] as i32,
        // ...
    }),
    xform: Some(crate::proto::Xform {
        matrix: self.xform.m.to_vec(),
        // ...
    }),
    // ...
};

// In pb_loads():
if let Some(color) = msg.linecolor {
    obj.linecolor = Color::new(color.r as u8, color.g as u8, color.b as u8, color.a as u8);
}
if let Some(xform) = msg.xform {
    for (i, val) in xform.matrix.iter().enumerate() {
        if i < 16 { obj.xform.m[i] = *val; }
    }
}
```
