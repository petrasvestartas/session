# Transformation - Rust

```rust
pub fn transform(&mut self) {
    let (nx, ny, nz) = self.xform.apply(self._x, self._y, self._z);
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
```

## Manual Matrix Application

```rust
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
```
