# Common Fields - Rust

## Implementation

```rust
use uuid::Uuid;

#[derive(Clone)]
pub struct ClassName {
    pub guid: String,
    pub name: String,

    // Visual classes only:
    pub width: f64,
    pub color: Color,
    pub xform: Xform,
}

impl ClassName {
    pub fn new() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_classname".to_string(),
            width: 1.0,
            color: Color::red(),
            xform: Xform::identity(),
        }
    }
}

impl Default for ClassName {
    fn default() -> Self {
        Self::new()
    }
}
```

## GUID Generation

```rust
use uuid::Uuid;

// In new():
guid: Uuid::new_v4().to_string(),

// In duplicate() - generate NEW guid:
pub fn duplicate(&self) -> Self {
    let mut copy = self.clone();
    copy.guid = Uuid::new_v4().to_string();  // NEW guid
    copy
}
```

## Private Coordinates Pattern

```rust
pub struct Point {
    #[serde(rename = "x")]
    _x: f64,
    #[serde(rename = "y")]
    _y: f64,
    #[serde(rename = "z")]
    _z: f64,
}

impl Index<usize> for Point {
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
```
