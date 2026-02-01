# Web App Recipes

Composable geometry recipes using session API. Each recipe shows all 3 languages with correct, tested API calls.

## Circle + Subdivide into N Points

**C++:**
```cpp
#include "primitives.h"
NurbsCurve circle = Primitives::circle(0.0, 0.0, 0.0, 1.0);
auto [points, params] = circle.divide_by_count(10, true); // 10 points along circle
```

**Python:**
```python
from session_py import Primitives
circle = Primitives.circle(0.0, 0.0, 0.0, 1.0)
points, params = circle.divide_by_count(10, True)
```

**Rust:**
```rust
use crate::primitives::Primitives;
let circle = Primitives::circle(0.0, 0.0, 0.0, 1.0);
let (points, params) = circle.divide_by_count(10, true);
```

## Ellipse + Subdivide by Arc Length

**C++:**
```cpp
#include "primitives.h"
NurbsCurve ellipse = Primitives::ellipse(0.0, 0.0, 0.0, 2.0, 1.0);
auto [points, params] = ellipse.divide_by_length(0.5);
```

**Python:**
```python
from session_py import Primitives
ellipse = Primitives.ellipse(0.0, 0.0, 0.0, 2.0, 1.0)
points, params = ellipse.divide_by_length(0.5)
```

**Rust:**
```rust
use crate::primitives::Primitives;
let ellipse = Primitives::ellipse(0.0, 0.0, 0.0, 2.0, 1.0);
let (points, params) = ellipse.divide_by_length(0.5);
```

## Arc Through 3 Points

**C++:**
```cpp
#include "primitives.h"
NurbsCurve arc = Primitives::arc(Point(0.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(2.0, 0.0, 0.0));
```

**Python:**
```python
from session_py import Primitives
from session_py import Point
arc = Primitives.arc(Point(0.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(2.0, 0.0, 0.0))
```

**Rust:**
```rust
use crate::primitives::Primitives;
use crate::point::Point;
let arc = Primitives::arc(&Point::new(0.0, 0.0, 0.0), &Point::new(1.0, 1.0, 0.0), &Point::new(2.0, 0.0, 0.0));
```

## Open Curve from Points + Adaptive Polyline

**C++:**
```cpp
#include "nurbscurve.h"
std::vector<Point> pts = {Point(0,0,0), Point(1,2,0), Point(2,0,0), Point(3,2,0), Point(4,0,0)};
NurbsCurve curve = NurbsCurve::create(false, 2, pts);
auto [polyline_pts, polyline_params] = curve.to_polyline_adaptive(0.1, 0.0, 0.0);
```

**Python:**
```python
from session_py import NurbsCurve
from session_py import Point
pts = [Point(0,0,0), Point(1,2,0), Point(2,0,0), Point(3,2,0), Point(4,0,0)]
curve = NurbsCurve.create(False, 2, pts)
polyline_pts, polyline_params = curve.to_polyline_adaptive(0.1, 0.0, 0.0)
```

**Rust:**
```rust
use crate::nurbscurve::NurbsCurve;
use crate::point::Point;
let pts = vec![Point::new(0.0,0.0,0.0), Point::new(1.0,2.0,0.0), Point::new(2.0,0.0,0.0), Point::new(3.0,2.0,0.0), Point::new(4.0,0.0,0.0)];
let curve = NurbsCurve::create(false, 2, &pts, 3, 1.0);
let (polyline_pts, polyline_params) = curve.to_polyline_adaptive(0.1, 0.0, 0.0);
```

## Curve Evaluation at Parameter

**C++:**
```cpp
#include "nurbscurve.h"
NurbsCurve curve = NurbsCurve::create(false, 2, points);
curve.set_domain(0.0, 1.0);
Point pt = curve.point_at(0.5);
Vector tangent = curve.tangent_at(0.5);
auto [frame_plane, kappa] = curve.frame_at(0.5);
```

**Python:**
```python
from session_py import NurbsCurve
curve = NurbsCurve.create(False, 2, points)
curve.set_domain(0.0, 1.0)
pt = curve.point_at(0.5)
tangent = curve.tangent_at(0.5)
frame_plane, kappa = curve.frame_at(0.5)
```

**Rust:**
```rust
use crate::nurbscurve::NurbsCurve;
let mut curve = NurbsCurve::create(false, 2, &points, 3, 1.0);
curve.set_domain(0.0, 1.0);
let pt = curve.point_at(0.5);
let tangent = curve.tangent_at(0.5);
let (frame_plane, kappa) = curve.frame_at(0.5);
```

## Curve Frames Along Length

**C++:**
```cpp
#include "nurbscurve.h"
#include "primitives.h"
NurbsCurve circle = Primitives::circle(0.0, 0.0, 0.0, 5.0);
auto [pts, params] = circle.divide_by_count(20, true);
std::vector<Plane> frames;
for (auto t : params) {
    auto [plane, kappa] = circle.frame_at(t);
    frames.push_back(plane);
}
```

**Python:**
```python
from session_py import Primitives
circle = Primitives.circle(0.0, 0.0, 0.0, 5.0)
pts, params = circle.divide_by_count(20, True)
frames = []
for t in params:
    plane, kappa = circle.frame_at(t)
    frames.append(plane)
```

**Rust:**
```rust
use crate::primitives::Primitives;
let circle = Primitives::circle(0.0, 0.0, 0.0, 5.0);
let (pts, params) = circle.divide_by_count(20, true);
let frames: Vec<_> = params.iter().map(|&t| {
    let (plane, _kappa) = circle.frame_at(t);
    plane
}).collect();
```

## Ellipse + Perpendicular Frames

**C++:**
```cpp
#include "nurbscurve.h"
#include "primitives.h"
NurbsCurve ellipse = Primitives::ellipse(0.0, 0.0, 0.0, 2.0, 1.0);
auto [pts, params] = ellipse.divide_by_count(4, true);
std::vector<Plane> frames;
for (auto t : params) {
    auto [plane, kappa] = ellipse.frame_at(t);
    frames.push_back(plane);
}
```

**Python:**
```python
from session_py import Primitives
ellipse = Primitives.ellipse(0.0, 0.0, 0.0, 2.0, 1.0)
pts, params = ellipse.divide_by_count(4, True)
frames = []
for t in params:
    plane, kappa = ellipse.frame_at(t)
    frames.append(plane)
```

**Rust:**
```rust
use crate::primitives::Primitives;
let ellipse = Primitives::ellipse(0.0, 0.0, 0.0, 2.0, 1.0);
let (pts, params) = ellipse.divide_by_count(4, true);
let frames: Vec<_> = params.iter().map(|&t| {
    let (plane, _kappa) = ellipse.frame_at(t);
    plane
}).collect();
```

## Cylinder Surface + Evaluate Point

**C++:**
```cpp
#include "primitives.h"
NurbsSurface cyl = Primitives::cylinder_surface(0.0, 0.0, 0.0, 1.0, 5.0);
Point p = cyl.point_at(0.0, 0.5); // (u=0, v=0.5) -> (1, 0, 2.5)
```

**Python:**
```python
from session_py import Primitives
cyl = Primitives.cylinder_surface(0.0, 0.0, 0.0, 1.0, 5.0)
p = cyl.point_at(0.0, 0.5)
```

**Rust:**
```rust
use crate::primitives::Primitives;
let cyl = Primitives::cylinder_surface(0.0, 0.0, 0.0, 1.0, 5.0);
let p = cyl.point_at(0.0, 0.5);
```

## Mesh from Vertices and Faces

**C++:**
```cpp
#include "mesh.h"
Mesh mesh;
mesh.add_vertex(Vertex(0,0,0));
mesh.add_vertex(Vertex(1,0,0));
mesh.add_vertex(Vertex(2,0,0));
mesh.add_vertex(Vertex(0,1,0));
mesh.add_face({0, 1, 3});
mesh.add_face({1, 2, 3});
```

**Python:**
```python
from session_py import Mesh
from session_py import Vertex
mesh = Mesh()
mesh.add_vertex(Vertex(0,0,0))
mesh.add_vertex(Vertex(1,0,0))
mesh.add_vertex(Vertex(2,0,0))
mesh.add_vertex(Vertex(0,1,0))
mesh.add_face([0, 1, 3])
mesh.add_face([1, 2, 3])
```

**Rust:**
```rust
use crate::mesh::Mesh;
use crate::vertex::Vertex;
let mut mesh = Mesh::new();
mesh.add_vertex(Vertex::new(0.0, 0.0, 0.0));
mesh.add_vertex(Vertex::new(1.0, 0.0, 0.0));
mesh.add_vertex(Vertex::new(2.0, 0.0, 0.0));
mesh.add_vertex(Vertex::new(0.0, 1.0, 0.0));
mesh.add_face(&[0, 1, 3]);
mesh.add_face(&[1, 2, 3]);
```

## API Quick Reference

| What | C++ | Python | Rust |
|------|-----|--------|------|
| Circle | `Primitives::circle(cx,cy,cz,r)` | `Primitives.circle(cx,cy,cz,r)` | `Primitives::circle(cx,cy,cz,r)` |
| Ellipse | `Primitives::ellipse(cx,cy,cz,a,b)` | `Primitives.ellipse(cx,cy,cz,a,b)` | `Primitives::ellipse(cx,cy,cz,a,b)` |
| Arc | `Primitives::arc(p0,p1,p2)` | `Primitives.arc(p0,p1,p2)` | `Primitives::arc(&p0,&p1,&p2)` |
| Open curve | `NurbsCurve::create(false, deg, pts)` | `NurbsCurve.create(False, deg, pts)` | `NurbsCurve::create(false, deg, &pts, 3, 1.0)` |
| Periodic curve | `NurbsCurve::create(true, deg, pts)` | `NurbsCurve.create(True, deg, pts)` | `NurbsCurve::create(true, deg, &pts, 3, 1.0)` |
| Divide N pts | `curve.divide_by_count(n, true)` | `curve.divide_by_count(n, True)` | `curve.divide_by_count(n, true)` |
| Divide by len | `curve.divide_by_length(len)` | `curve.divide_by_length(len)` | `curve.divide_by_length(len)` |
| Adaptive poly | `curve.to_polyline_adaptive(tol,0,0)` | `curve.to_polyline_adaptive(tol,0,0)` | `curve.to_polyline_adaptive(tol,0,0)` |
| Point at t | `curve.point_at(t)` | `curve.point_at(t)` | `curve.point_at(t)` |
| Tangent at t | `curve.tangent_at(t)` | `curve.tangent_at(t)` | `curve.tangent_at(t)` |
| Frame at t | `curve.frame_at(t)` | `curve.frame_at(t)` | `curve.frame_at(t)` |
| Cylinder srf | `Primitives::cylinder_surface(cx,cy,cz,r,h)` | `Primitives.cylinder_surface(cx,cy,cz,r,h)` | `Primitives::cylinder_surface(cx,cy,cz,r,h)` |
| Cone srf | `Primitives::cone_surface(cx,cy,cz,r,h)` | `Primitives.cone_surface(cx,cy,cz,r,h)` | `Primitives::cone_surface(cx,cy,cz,r,h)` |
| Torus srf | `Primitives::torus_surface(cx,cy,cz,R,r)` | `Primitives.torus_surface(cx,cy,cz,R,r)` | `Primitives::torus_surface(cx,cy,cz,R,r)` |
| Surface pt | `surface.point_at(u, v)` | `surface.point_at(u, v)` | `surface.point_at(u, v)` |
