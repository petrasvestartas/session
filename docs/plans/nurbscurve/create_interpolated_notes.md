# Notes: create_interpolated Implementation

## Algorithm from TL.DLL Decompilation

### 1. Entry Point: TL_CubicNurbThroughPoints

**Step 1: Validation**
- point_count > 1
- closed_type < 4
- If closed, start_tan and end_tan must be NULL

**Step 2: CV count calculation**
- Open: cv_count = point_count + 2
- Closed (type 1/2): cv_count = point_count + 3

**Step 3: Parameterization**
```cpp
// Uniform (knot_style == 0)
params[i] = i;

// Chord-length (knot_style == 1)
params[0] = 0;
for (i = 1; i < point_count; i++) {
    dist = distance(points[i], points[i-1]);
    params[i] = params[i-1] + dist;
}

// Centripetal (knot_style == 2)
params[0] = 0;
for (i = 1; i < point_count; i++) {
    dist = sqrt(distance(points[i], points[i-1]));
    params[i] = params[i-1] + dist;
}
```

**Step 4: Tolerance check**
- Fail if min_dist <= max_dist * 1.49e-08

**Step 5: Call TL_CubicNurbInterpolate**

### 2. Core Solver: TL_CubicNurbInterpolate

**Knot vector structure (clamped cubic):**
```
knots = [t0, t0, t0, t0, t1, t2, ..., tn, tn, tn, tn]
         └─quadruple─┘              └─quadruple─┘
```
For cubic (order=4): need quadruple knots at ends.

**Greville points:**
```
g[i] = (knot[i+1] + knot[i+2] + knot[i+3]) / 3
```
These are the parameter values where we want the curve to pass through input points.

**Matrix setup (tridiagonal):**
For each interior point i (1 to cv_count-3):
1. Evaluate basis functions at Greville point g[i]
2. This gives coefficients for the tridiagonal system

**End conditions (for open curves with auto tangent):**
- Type 3 (Bessel): Scale tangent from point spacing
  - start_tan = (points[1] - points[0]) * scale
  - scale = (params[1] - params[0]) / (knot[4] - knot[3])

### 3. Tridiagonal Solver: TL_SolveTriDiagonal

Thomas algorithm:
```cpp
// Forward elimination
for (i = 1; i < n; i++) {
    scale = 1.0 / diag[i-1];
    upper[i-1] *= scale;
    factor = lower[i] * scale;
    diag[i] -= factor * upper[i-1];
    rhs[i] -= factor * rhs[i-1];
}

// Solve last equation
solution[n-1] = rhs[n-1] / diag[n-1];

// Back substitution
for (i = n-2; i >= 0; i--) {
    solution[i] = (rhs[i] - upper[i] * solution[i+1]) / diag[i];
}
```

### 4. Basis Function Evaluation

For cubic B-spline at parameter t in span [knot[i], knot[i+1]):
```cpp
// de Boor-Cox recursion or direct formula
N[i,0] = 1 if knot[i] <= t < knot[i+1], else 0
N[i,p] = (t - knot[i]) / (knot[i+p] - knot[i]) * N[i,p-1]
       + (knot[i+p+1] - t) / (knot[i+p+1] - knot[i+1]) * N[i+1,p-1]
```

## Implementation Plan

### File: nurbscurve.cpp

```cpp
// 1. Tridiagonal solver
static bool solve_tridiagonal(
    int dim, int n,
    std::vector<double>& lower,
    std::vector<double>& diag,
    std::vector<double>& upper,
    std::vector<double>& rhs,
    std::vector<double>& solution
);

// 2. Compute parameters from points
static std::vector<double> compute_parameters(
    const std::vector<Point>& points,
    int knot_style  // 0=uniform, 1=chord, 2=centripetal
);

// 3. Build knot vector from parameters
static std::vector<double> build_knot_vector(
    const std::vector<double>& params,
    int cv_count,
    int order
);

// 4. Main entry point
NurbsCurve NurbsCurve::create_interpolated(
    const std::vector<Point>& points,
    int degree,
    bool closed,
    int knot_style
);
```

## Key Constants
- Machine epsilon check: 1.49e-08
- Default knot_style: 1 (chord-length)
- Default degree: 3 (cubic)
