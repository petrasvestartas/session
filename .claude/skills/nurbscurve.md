# NurbsCurve Skill

## Overview

NurbsCurve is a Non-Uniform Rational B-Spline curve implementation across C++, Python, and Rust. This document defines the ground truth API and test structure that must be consistent across all three languages.

## Class Structure

### Fields (All Languages)
```
guid: string          # Unique identifier (auto-generated)
name: string          # Default: "my_nurbscurve"
width: float          # Line width (default: 1.0)
linecolor: Color      # Default: Color.white()
xform: Xform          # Transformation matrix (default: identity)

# Internal NURBS data
m_dim: int            # Dimension (typically 3)
m_is_rat: int         # 1 if rational, 0 if not
m_order: int          # Order = degree + 1
m_cv_count: int       # Number of control vertices
m_cv_stride: int      # Stride between CVs
m_knot: vector<float> # Knot vector
m_cv: vector<float>   # Control vertex data
```

## API Methods (Required in All Languages)

### Static Factory Methods
- `create(periodic: bool, degree: int, points: vector<Point>) -> NurbsCurve`

### Constructors
- Default constructor (creates invalid curve)
- `NurbsCurve(dimension, is_rational, order, cv_count)`

### Validation
- `is_valid() -> bool`
- `is_valid_knot_vector() -> bool`

### Accessors
- `dimension() -> int`
- `degree() -> int`
- `order() -> int`
- `cv_count() -> int`
- `cv_size() -> int`
- `knot_count() -> int`
- `span_count() -> int`

### Control Vertex Access
- `cv(index) -> pointer/slice`
- `get_cv(index) -> Point`
- `get_cv_4d(index) -> (x, y, z, w)`
- `set_cv(index, Point) -> bool`
- `set_cv_4d(index, x, y, z, w) -> bool`
- `weight(index) -> float`
- `set_weight(index, weight) -> bool`

### Knot Access
- `knot(index) -> float`
- `set_knot(index, value) -> bool`
- `knot_multiplicity(index) -> int`
- `superfluous_knot(end) -> float`
- `knot_array() -> pointer/slice`
- `cv_array() -> pointer/slice`
- `get_knots() -> vector<float>`

### Domain & Parameterization
- `domain() -> (start, end)`
- `domain_start() -> float`
- `domain_end() -> float`
- `domain_middle() -> float`
- `set_domain(t0, t1) -> bool`
- `get_span_vector() -> vector<float>`

### Geometric Queries
- `is_rational() -> bool`
- `is_closed() -> bool`
- `is_periodic() -> bool`
- `is_linear(tolerance?) -> bool`
- `is_planar(plane?, tolerance?) -> bool`
- `is_arc(plane?, tolerance?) -> bool`
- `is_in_plane(plane, tolerance?) -> bool`
- `is_natural(end?) -> bool`
- `is_polyline(points?, params?) -> int`

### Conversion Methods
- `to_polyline_adaptive(points, params?, angle_tol?, min_edge?, max_edge?) -> bool`
- `divide_by_count(count, points, params?, include_endpoints?) -> bool`
- `divide_by_length(segment_length, points, params?) -> bool`

### Evaluation
- `point_at(t) -> Point`
- `point_at_start() -> Point`
- `point_at_end() -> Point`
- `tangent_at(t) -> Vector`
- `evaluate(t, derivative_count?) -> vector<Vector>`
- `frame_at(t, normalized, origin, xaxis, yaxis, zaxis) -> bool`
- `perpendicular_frame_at(t, normalized, origin, xaxis, yaxis, zaxis) -> bool`

### Modification Operations
- `reverse() -> bool`
- `make_rational() -> bool`
- `make_non_rational() -> bool`

### Geometric Operations
- `length(tolerance?) -> float`
- `intersect_plane(plane, tolerance?) -> vector<float>`

### Serialization
- `str() -> string`
- `repr() -> string`
- `jsondump() -> json`
- `jsonload(json) -> NurbsCurve`
- `json_dump(filename)`
- `json_load(filename) -> NurbsCurve`
- `protobuf_dump(filename)`
- `protobuf_load(filename) -> NurbsCurve`

## Test Structure (Must Match Exactly)

### Test 1: constructor
```
Points: [(0,0,0), (1,1,0), (2,0,0), (3,1,0)]
Curve: create(false, 2, points)
set_domain(0.0, 1.0) x2

Checks:
- is_valid() == true
- cv_count() == 4
- degree() == 2
- order() == 3
- name == "my_nurbscurve"
- guid not empty
- str() == "degree=2, cvs=4"
- repr() == "NurbsCurve(my_nurbscurve, dim=3, order=3, cvs=4, rational=false)"
- copy cv_count matches
- copy guid differs
```

### Test 2: attributes
```
Points: [(0,0,0), (1,1,0), (2,0,0), (3,1,0)]
Curve: create(false, 2, points)

Checks:
- is_valid == true
- is_valid_knot_vector == true
- dimension == 3
- degree == 2
- order == 3
- cv_count == 4
- cv_size == 3
- knot_count == 5
- span_count == 2
- cv(1) == [1.0, 1.0, 0.0]
- get_cv(1) == Point(1,1,0)
- get_cv_4d(1) == (1,1,0,1)
- set_cv(2, Point(2,0,0.5)) works
- set_cv_4d(2, 2,0,0.5,0.707) works
- weight(2) == 0.707
- set_weight(2, 0.5) -> weight(2) == 0.5
- knot(3) == 2
- set_knot(4, 2) -> knot(4) == 2
- knot_multiplicity: [2,2,1,2,2]
- superfluous_knot(1) == 4
- knot_array()[0] == 0.0
- get_knots() == [0,0,1,2,2]
- cv_array()[0] == 0.0
- domain == (0,2)
- domain_start/middle/end == 0/1/2
- set_domain(0,1) works
- get_span_vector() == [0,0.5,1]
- is_rational == true (after set_cv_4d)
- is_closed == false
- is_periodic == false
- is_linear == false
- is_planar == false
- is_arc == false
- is_in_plane == false
- is_natural == true
- is_polyline == false
```

### Test 3: Conversions
```
Points: [(0,0,0), (1,2,0), (2,0,0), (3,2,0), (4,0,0)]
Curve: create(false, 2, points)

to_polyline_adaptive with angle_tolerance=0.1:
- 27 points
- Specific point values (see test file)

divide_by_count(10, include_endpoints=true):
- 10 points
- Point values using Gauss-Legendre arc length

divide_by_length(0.5):
- 13 points
- Point values using Gauss-Legendre arc length
```

### Test 4: frame_at
```
Points: 11 control points (complex 3D curve)
Check frame at t=0.5, normalized=true
Returns origin, tangent, normal, binormal
```

### Test 5: perpendicular_frame_at
```
Same curve as frame_at
Check RMF at t=0.5, normalized=true
```

### Remaining Tests
- is_valid
- control vertices
- set_cv
- point_at
- point_at_start
- point_at_end
- domain
- is_closed
- length
- reverse
- make_rational
- tangent_at
- knot_count
- cv_size
- weight
- is_linear
- json_roundtrip
- protobuf_roundtrip
- degree
- is_rational
- set_weight
- knot
- set_knot
- set_domain
- span_count
- get_span_vector
- evaluate
- is_periodic
- make_non_rational
- divide_by_count
- intersect_plane

## Arc Length Algorithm (Gauss-Legendre + Newton-Raphson)

The divide_by_count and divide_by_length methods use:

1. **5-point Gauss-Legendre quadrature** for accurate arc length integration:
```
GL_NODES = [-0.9061798459386640, -0.5384693101056831, 0.0, 0.5384693101056831, 0.9061798459386640]
GL_WEIGHTS = [0.2369268850561891, 0.4786286704993665, 0.5688888888888889, 0.4786286704993665, 0.2369268850561891]

arc_length_gauss(ta, tb):
    mid = (ta + tb) / 2
    half = (tb - ta) / 2
    sum = 0
    for i in 0..5:
        t = mid + half * GL_NODES[i]
        sum += GL_WEIGHTS[i] * derivative_at(t).magnitude()
    return half * sum
```

2. **Derivative calculation** (un-normalized, NOT unit tangent):
```
derivative_at(t):
    h = domain_length * 1e-8
    if t near start: p1=point_at(t0), p2=point_at(t0+h), dt=h
    elif t near end: p1=point_at(t1-h), p2=point_at(t1), dt=h
    else: p1=point_at(t-h), p2=point_at(t+h), dt=2h
    return (p2 - p1) / dt
```

3. **Newton-Raphson refinement** with bisection fallback:
```
find_t_at_s(s_target):
    # Binary search for bracket
    # Linear interpolation initial guess
    # Newton-Raphson with bracketing
    # Bisection fallback for robustness
```

## Expected Test Values (Conversions Test)

### divide_by_count(10, include_endpoints=true) on S-wave curve
```
Point 0: (0.000000000000000, 0.000000000000000, 0.000000000000000)
Point 1: (0.328571015882635, 0.598213506310667, 0.000000000000000)
Point 2: (0.740744941524856, 1.140321234797829, 0.000000000000000)
Point 3: (1.338523997492639, 1.232716041998164, 0.000000000000000)
Point 4: (1.712929663130383, 0.664818756620870, 0.000000000000000)
Point 5: (2.287070327006695, 0.664818745295462, 0.000000000000000)
Point 6: (2.661475993133979, 1.232716033043460, 0.000000000000000)
Point 7: (3.259255052521522, 1.140321240507253, 0.000000000000000)
Point 8: (3.671428981912368, 0.598213509892612, 0.000000000000000)
Point 9: (4.000000000000000, 0.000000000000000, 0.000000000000000)
```

### divide_by_length(0.5) on S-wave curve
```
Point 0: (0.000000000000000, 0.000000000000000, 0.000000000000000)
Point 1: (0.235272731384047, 0.441110443734231, 0.000000000000000)
Point 2: (0.504276692145966, 0.862299318703470, 0.000000000000000)
Point 3: (0.843085062978891, 1.227533014827472, 0.000000000000000)
Point 4: (1.302050970444518, 1.264156212040698, 0.000000000000000)
Point 5: (1.579813544869556, 0.853113314150178, 0.000000000000000)
Point 6: (1.928691287815458, 0.510169864866836, 0.000000000000000)
Point 7: (2.340857741884085, 0.732368000404634, 0.000000000000000)
Point 8: (2.597735401548903, 1.160594587288875, 0.000000000000000)
Point 9: (3.032790392631424, 1.300960469420597, 0.000000000000000)
Point 10: (3.407806728972739, 0.976991467650206, 0.000000000000000)
Point 11: (3.691337413616094, 0.565615072909225, 0.000000000000000)
Point 12: (3.934494402948975, 0.128829830906625, 0.000000000000000)
```

## Test Naming Convention

All tests must use the same names across languages:
- constructor
- attributes
- Conversions
- frame_at
- perpendicular_frame_at
- is_valid
- control vertices
- set_cv
- point_at
- point_at_start
- point_at_end
- domain
- is_closed
- length
- reverse
- make_rational
- tangent_at
- knot_count
- cv_size
- weight
- is_linear
- json_roundtrip
- protobuf_roundtrip
- degree
- is_rational
- set_weight
- knot
- set_knot
- set_domain
- span_count
- get_span_vector
- evaluate
- is_periodic
- make_non_rational
- divide_by_count
- intersect_plane
