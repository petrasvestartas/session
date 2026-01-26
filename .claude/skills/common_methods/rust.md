# Common Methods - Rust

## Implementation

```rust
impl ClassName {
    pub fn str(&self) -> String {
        format!("ClassName(x={}, y={})", self._x, self._y)
    }

    pub fn repr(&self) -> String {
        format!(
            "ClassName(\n  name={},\n  x={},\n  y={}\n)",
            self.name, self._x, self._y
        )
    }

    pub fn is_valid(&self) -> bool {
        !self._x.is_nan() && !self._y.is_nan()
    }

    pub fn duplicate(&self) -> Self {
        let mut copy = self.clone();
        copy.guid = Uuid::new_v4().to_string();  // NEW guid
        copy
    }
}
```

## Display Trait

```rust
use std::fmt;

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}
```

## Clone Requirement

```rust
#[derive(Clone)]
pub struct ClassName {
    // fields...
}
```
