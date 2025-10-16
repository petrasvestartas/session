# Transform Methods - Final Implementation Guide

## Two Methods for Each Type

1. **`transform()`** - In-place transformation (modifies the object)
2. **`transformed()`** - Returns a transformed copy (original unchanged)

## Status

### ✅ COMPLETED

**Python - ALL 9 types:**
- Point, Line, Plane, BoundingBox, Polyline, PointCloud, Mesh, Cylinder, Arrow
- Both `transform()` and `transformed()` implemented
- All tests passing

**C++ - 2 types:**
- Point, Line
- Both `transform()` and `transformed()` implemented

### ⏳ TODO

**C++ - 7 remaining types:**
- Plane, BoundingBox, Polyline, PointCloud, Mesh, Cylinder, Arrow

**Rust - All 9 types:**
- Point, Line, Plane, BoundingBox, Polyline, PointCloud, Mesh, Cylinder, Arrow

---

## C++ Remaining Implementations

### Plane

**Header (plane.h):**
```cpp
void transform();
Plane transformed() const;
```

**Implementation (plane.cpp):**
```cpp
void Plane::transform() {
  xform.transform_point(_origin);
  xform.transform_vector(_x_axis);
  xform.transform_vector(_y_axis);
  xform.transform_vector(_z_axis);
  xform = Xform::identity();
}

Plane Plane::transformed() const {
  Plane result = *this;
  result.transform();
  return result;
}
```

### BoundingBox

**Header (boundingbox.h):**
```cpp
void transform();
BoundingBox transformed() const;
```

**Implementation (boundingbox.cpp):**
```cpp
void BoundingBox::transform() {
  xform.transform_point(center);
  xform.transform_vector(x_axis);
  xform.transform_vector(y_axis);
  xform.transform_vector(z_axis);
  xform = Xform::identity();
}

BoundingBox BoundingBox::transformed() const {
  BoundingBox result = *this;
  result.transform();
  return result;
}
```

### Polyline

**Header (polyline.h):**
```cpp
void transform();
Polyline transformed() const;
```

**Implementation (polyline.cpp):**
```cpp
void Polyline::transform() {
  for (auto& pt : points) {
    xform.transform_point(pt);
  }
  xform = Xform::identity();
}

Polyline Polyline::transformed() const {
  Polyline result = *this;
  result.transform();
  return result;
}
```

### PointCloud

**Header (pointcloud.h):**
```cpp
void transform();
PointCloud transformed() const;
```

**Implementation (pointcloud.cpp):**
```cpp
void PointCloud::transform() {
  for (auto& pt : points) {
    xform.transform_point(pt);
  }
  for (auto& n : normals) {
    xform.transform_vector(n);
  }
  xform = Xform::identity();
}

PointCloud PointCloud::transformed() const {
  PointCloud result = *this;
  result.transform();
  return result;
}
```

### Mesh

**Header (mesh.h):**
```cpp
void transform();
Mesh transformed() const;
```

**Implementation (mesh.cpp):**
```cpp
void Mesh::transform() {
  for (auto& v : vertices) {
    xform.transform_point(v);
  }
  xform = Xform::identity();
}

Mesh Mesh::transformed() const {
  Mesh result = *this;
  result.transform();
  return result;
}
```

### Cylinder

**Header (cylinder.h):**
```cpp
void transform();
Cylinder transformed() const;
```

**Implementation (cylinder.cpp):**
```cpp
void Cylinder::transform() {
  line.transform();
  xform = Xform::identity();
}

Cylinder Cylinder::transformed() const {
  Cylinder result = *this;
  result.transform();
  return result;
}
```

### Arrow

**Header (arrow.h):**
```cpp
void transform();
Arrow transformed() const;
```

**Implementation (arrow.cpp):**
```cpp
void Arrow::transform() {
  line.transform();
  xform = Xform::identity();
}

Arrow Arrow::transformed() const {
  Arrow result = *this;
  result.transform();
  return result;
}
```

---

## Rust Implementations

### Point

```rust
pub fn transform(&mut self) {
    self.xform.transform_point(self);
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

### Line

```rust
pub fn transform(&mut self) {
    let mut start = Point::new(self._x0, self._y0, self._z0);
    let mut end = Point::new(self._x1, self._y1, self._z1);
    
    self.xform.transform_point(&mut start);
    self.xform.transform_point(&mut end);
    
    self._x0 = start.x();
    self._y0 = start.y();
    self._z0 = start.z();
    self._x1 = end.x();
    self._y1 = end.y();
    self._z1 = end.z();
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

### Plane

```rust
pub fn transform(&mut self) {
    self.xform.transform_point(&mut self._origin);
    self.xform.transform_vector(&mut self._x_axis);
    self.xform.transform_vector(&mut self._y_axis);
    self.xform.transform_vector(&mut self._z_axis);
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

### BoundingBox

```rust
pub fn transform(&mut self) {
    self.xform.transform_point(&mut self.center);
    self.xform.transform_vector(&mut self.x_axis);
    self.xform.transform_vector(&mut self.y_axis);
    self.xform.transform_vector(&mut self.z_axis);
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

### Polyline

```rust
pub fn transform(&mut self) {
    for pt in &mut self.points {
        self.xform.transform_point(pt);
    }
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

### PointCloud

```rust
pub fn transform(&mut self) {
    for pt in &mut self.points {
        self.xform.transform_point(pt);
    }
    for n in &mut self.normals {
        self.xform.transform_vector(n);
    }
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

### Mesh

```rust
pub fn transform(&mut self) {
    for v in &mut self.vertices {
        self.xform.transform_point(v);
    }
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

### Cylinder

```rust
pub fn transform(&mut self) {
    self.line.transform();
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

### Arrow

```rust
pub fn transform(&mut self) {
    self.line.transform();
    self.xform = Xform::identity();
}

pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

---

## Usage Examples

### Python
```python
# In-place transformation
point.transform()

# Get transformed copy
new_point = point.transformed()
```

### C++
```cpp
// In-place transformation
point.transform();

// Get transformed copy
Point new_point = point.transformed();
```

### Rust
```rust
// In-place transformation
point.transform();

// Get transformed copy
let new_point = point.transformed();
```

---

## Implementation Pattern

For all types, `transformed()` follows the same pattern:

**Python:**
```python
def transformed(self):
    import copy
    result = copy.deepcopy(self)
    result.transform()
    return result
```

**C++:**
```cpp
Type Type::transformed() const {
  Type result = *this;
  result.transform();
  return result;
}
```

**Rust:**
```rust
pub fn transformed(&self) -> Self {
    let mut result = self.clone();
    result.transform();
    result
}
```

---

## Testing

After implementation, run tests:

```bash
# Python
cd session_py && python -m pytest src/session_py/session_test.py -v

# C++
cd session_cpp && ./build.sh && ./test.sh

# Rust
cd session_rust && cargo test
```
