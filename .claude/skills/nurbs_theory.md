# NURBS Theory Reference

## B-Spline Basis Functions

### Recursion Formula (Cox-de Boor)

Order 1 (degree 0) — piecewise constant:
```
N_{i,1}(t) = 1  if knot[i] <= t < knot[i+1]
             0  otherwise
```

Higher orders — recursive blend of two lower-order functions:
```
N_{i,k}(t) = ((t - knot[i]) / (knot[i+k-1] - knot[i])) * N_{i,k-1}(t)
           + ((knot[i+k] - t) / (knot[i+k] - knot[i+1])) * N_{i+1,k-1}(t)
```

Convention: `0/0 = 0` when a knot span has zero length.

### Partition of Unity

All basis functions sum to 1.0 at every parameter value within the valid domain:
```
sum(N_{i,k}(t)) = 1.0   for t in [knot[degree], knot[cv_count])
```

This ensures the curve is a proper weighted average of control points.

### Basis Function Properties

- **Non-negative:** `N_{i,k}(t) >= 0`
- **Local support:** `N_{i,k}(t) = 0` outside `[knot[i], knot[i+k])`
- Each basis function spans at most `k` knot intervals
- A point at parameter `t` is influenced by at most `k` (order) control points

## Knot Vectors

### Terminology

- **Order** = degree + 1 (OpenNURBS/Rhino convention: `order = degree + 1`)
- **Degree** = polynomial degree of each span
- **Knot count** varies by convention (see below)
- **Span** = interval between consecutive distinct knots in the domain

### OpenNURBS Knot Convention

OpenNURBS omits two "superfluous" knots (first and last) compared to textbooks:

| Convention | Knot Count Formula |
|------------|-------------------|
| **OpenNURBS / Rhino** | `knot_count = order + cv_count - 2` |
| **Textbook / OpenGL / IGES / STEP** | `knot_count = order + cv_count` |

The two omitted knots are unused in evaluation and "make it appear the first and last spans are different from interior spans." Omitting them simplifies evaluation, degree changing, knot insertion/deletion, periodic closure, and curve fitting.

Example — degree 3, 7 CVs:
- OpenNURBS: `knot_count = 4 + 7 - 2 = 9`
- Textbook:  `knot_count = 4 + 7 = 11`

### Clamped (Open) Knot Vector

Start/end knots have multiplicity = order. The curve passes through the first and last control points.

```
degree=3, 7 CVs, OpenNURBS knots:
[0, 0, 0, 1, 2, 3, 4, 4, 4]

Textbook equivalent (adds two superfluous knots):
[0, 0, 0, 0, 1, 2, 3, 4, 4, 4, 4]
```

### Uniform Knot Vector

Equal spacing between consecutive knots in interior. Clamped-uniform has repeated ends with uniform interior.

### Periodic Knot Vector

Knot spacing wraps around — used for closed curves where start = end seamlessly. Control points also wrap: the first `degree` CVs repeat at the end.

### Non-Uniform Knot Vector

Arbitrary spacing between knots. Enables local control over parameterization speed.

## Knot Multiplicity and Continuity

Multiplicity `m` of a knot reduces continuity at that parameter:

| Multiplicity | Continuity Lost | Remaining Continuity |
|-------------|-----------------|---------------------|
| 1 | None | C^(degree-1) |
| 2 | C^(degree-1) | C^(degree-2) |
| k | C^(degree-1) ... C^(degree-k) | C^(degree-k-1) |
| degree | All interior | C^0 (position only, corner) |
| degree+1 (=order) | Position | Discontinuous (curve breaks) |

For cubic (degree 3):
- Single knot: C2 continuity
- Double knot: C1 continuity (tangent continuous, curvature break)
- Triple knot: C0 continuity (position only, sharp corner)

## Continuity Classes

### Parametric Continuity (C)

- **C0:** Position continuous (curves meet)
- **C1:** First derivative continuous (tangent vector equal in magnitude and direction)
- **C2:** Second derivative continuous (curvature continuous)
- **Cn:** nth derivative continuous

### Geometric Continuity (G)

- **G0:** Position continuous (= C0)
- **G1:** Tangent direction continuous (not necessarily equal magnitude)
- **G2:** Curvature continuous (curvature matches at join)

G-continuity is weaker than C-continuity. G1 requires collinear tangent vectors; C1 requires identical tangent vectors. A B-spline of degree `d` has C^(d-1) continuity at single interior knots.

## NURBS: Non-Uniform Rational B-Splines

### Definition

A NURBS curve adds per-control-point weights `w_i` to the B-spline formulation:

```
C(t) = sum(w_i * N_{i,k}(t) * P_i) / sum(w_i * N_{i,k}(t))
```

When all weights are equal, this reduces to a standard B-spline (non-rational).

### Why Rational?

- Only rational B-splines can represent conic sections (circles, ellipses, parabolas) exactly
- Plain B-splines and Bezier curves can only approximate circles
- A circle uses degree-2 NURBS with weights of 1 and cos(angle/2) alternating

### Homogeneous Coordinates

Rational curves are often stored in homogeneous form: `(w*x, w*y, w*z, w)`. This allows evaluation using the standard non-rational algorithm on 4D points, then dividing by `w`.

OpenNURBS stores rational CVs as `(w*x, w*y, w*z, w)` with `cv_stride = dim + 1 = 4`.

## Parameterization Styles

### Uniform

Knot spacing = constant (typically 1.0).
```
knots = [0, 1, 2, 3, 4, ...]
```

### Chord-Length

Knot spacing proportional to Euclidean distance between consecutive interpolation points:
```
d_i = |P_{i+1} - P_i|
knot[i+1] = knot[i] + d_i
```

### Centripetal (Square Root of Chord)

Knot spacing proportional to square root of chord length:
```
knot[i+1] = knot[i] + sqrt(|P_{i+1} - P_i|)
```

Reduces cusps and loops in interpolation compared to chord-length.

### Arc-Length

Knot spacing proportional to actual arc length of the curve. Used by Rhino's `CreateControlPointCurve` (see below).

## Rhino-Specific Behavior

### CurveKnotStyle Enum

Used with `CreateInterpolatedCurve` (not `CreateControlPointCurve`):

| Value | Name | Description |
|-------|------|-------------|
| 0 | Uniform | Knot spacing = 1.0 |
| 1 | Chord | Chord-length spacing (degree 3 only) |
| 2 | ChordSquareRoot | Centripetal spacing (degree 3 only) |
| 3 | UniformPeriodic | Periodic + uniform |
| 4 | ChordPeriodic | Periodic + chord-length |
| 5 | ChordSquareRootPeriodic | Periodic + centripetal |

### CreateControlPointCurve — Arc-Length Domain

`Curve.CreateControlPointCurve(points, degree)` creates a NURBS from control points with:

1. **Uniform knot spacing** internally (delta = arc_length / n_spans)
2. **Domain = [0, arc_length]** where arc_length is computed from the resulting curve

This is a Rhino-proprietary behavior. The open-source `rhino3dm` library uses `knot_delta = 1.0` and domain `[0, n_spans]`.

**Verified example** — 11 CVs, degree 2:
```
Our create():    domain = [0, 9],      knot_delta = 1.0
Rhino:           domain = [0, 11.301], knot_delta = 11.301/9 = 1.2557

Rhino knot vector (OpenNURBS convention, 11 knots):
[0, 0, 1.2557, 2.5113, 3.7670, 5.0227, 6.2783, 7.5340, 8.7897, 10.0454, 11.3010, 11.3010]

All interior spacings identical = 1.25567 ≈ 11.301026 / 9
Curve arc length = 11.301026 ≈ Rhino domain end 11.301028
```

### CreateInterpolatedCurve

Uses the `CurveKnotStyle` enum to determine knot placement. Interpolation points become edit points (Greville abscissae), not control points.

## Greville Abscissae

The Greville point (edit point) for control vertex `i` is the average of `degree` consecutive knots:

```
greville[i] = (knot[i] + knot[i+1] + ... + knot[i+degree-1]) / degree
```

For OpenNURBS convention (knot index starts at 0, knot_count = order + cv_count - 2):
```
greville[i] = (1/degree) * sum(knot[i+j] for j in 0..degree)
```

Edit points in Rhino are the Greville abscissae — they lie on the curve and editing them moves associated control points.

## de Boor Evaluation Algorithm

Evaluates a B-spline at parameter `t` without explicitly computing basis functions. Analogous to de Casteljau for Bezier curves.

1. Find knot span `[knot[i], knot[i+1])` containing `t`
2. Extract `order` relevant control points
3. Perform `degree` rounds of linear interpolation (triangular scheme)
4. Final point is the curve value at `t`

Complexity: O(degree^2) per evaluation. Can simultaneously produce derivatives.

## Key Relationships

```
order = degree + 1
knot_count = order + cv_count - 2          (OpenNURBS)
knot_count = order + cv_count              (textbook)
span_count = cv_count - degree             (non-periodic, clamped)
domain = [knot[degree-1], knot[cv_count-1]]  (OpenNURBS indexing)
```

## Sources

- [Bartosz Ciechanowski — Curves and Surfaces](https://ciechanow.ski/curves-and-surfaces/)
- [Rhino Developer — Superfluous Knots](https://developer.rhino3d.com/guides/opennurbs/superfluous-knots/)
- [Rhino Developer — CurveKnotStyle Enum](https://developer.rhino3d.com/api/rhinocommon/rhino.geometry.curveknotstyle)
- [Rhino Developer — Essential Mathematics: Parametric Curves](https://developer.rhino3d.com/guides/general/essential-mathematics/parametric-curves-surfaces/)
