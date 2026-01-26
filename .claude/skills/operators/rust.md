# Operators - Rust

## Index Operator

```rust
use std::ops::{Index, IndexMut};

impl Index<usize> for ClassName {
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

impl IndexMut<usize> for ClassName {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self._x,
            1 => &mut self._y,
            2 => &mut self._z,
            _ => panic!("Index out of bounds"),
        }
    }
}
```

## Equality Operators

```rust
impl PartialEq for ClassName {
    fn eq(&self, other: &Self) -> bool {
        self._x == other._x && self._y == other._y && self._z == other._z
        // Note: guid is NOT compared
    }
}
```

## Arithmetic Operators

```rust
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

// In-place
impl AddAssign<&Vector> for ClassName {
    fn add_assign(&mut self, v: &Vector) {
        self._x += v[0];
        self._y += v[1];
        self._z += v[2];
    }
}

impl MulAssign<f64> for ClassName {
    fn mul_assign(&mut self, scalar: f64) {
        self._x *= scalar;
        self._y *= scalar;
        self._z *= scalar;
    }
}

// Copy operators
impl Add<&Vector> for &ClassName {
    type Output = ClassName;
    fn add(self, v: &Vector) -> ClassName {
        let mut result = self.duplicate();
        result += v;
        result
    }
}

impl Mul<f64> for &ClassName {
    type Output = ClassName;
    fn mul(self, scalar: f64) -> ClassName {
        let mut result = self.duplicate();
        result *= scalar;
        result
    }
}
```
