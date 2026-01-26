# Rhino Interpolated Curve - Clean API Reference

Complete mapping from C# API to native implementation.

## API Layers

```
Rhino C# API (RhinoCommon.dll)
    ↓
P/Invoke Bridge (rhcommon_c.dll)
    ↓
Native Implementation (TL.DLL)
```

## 1. C# API (Clean, Documented)

**File:** `Rhino.Geometry.Curve`

```csharp
public static Curve CreateInterpolatedCurve(
    IEnumerable<Point3d> points,  // Points to interpolate (>= 2)
    int degree,                    // Curve degree (1, 3, 5...)
    CurveKnotStyle knots,          // Parameterization style
    Vector3d startTangent,         // Start tangent (Unset = auto)
    Vector3d endTangent            // End tangent (Unset = auto)
)
```

### CurveKnotStyle Enum

| Value | Int | Description |
|-------|-----|-------------|
| Uniform | 0 | Parameter spacing = 1.0 |
| Chord | 1 | Chord-length parameterization |
| ChordSquareRoot | 2 | Centripetal (sqrt chord) |
| UniformPeriodic | 3 | Periodic + uniform |
| ChordPeriodic | 4 | Periodic + chord |
| ChordSquareRootPeriodic | 5 | Periodic + centripetal |

## 2. P/Invoke Declaration

**File:** `UnsafeNativeMethods.cs`

```csharp
[DllImport("rhcommon_c.dll")]
internal static extern IntPtr RHC_RhinoInterpCurve(
    int degree,
    int count,
    Point3d[] arrayPts,
    Vector3d startTan,
    Vector3d endTan,
    int knotStyle
);
```

## 3. Native Bridge (rhcommon_c.dll)

Export: `RHC_RhinoInterpCurve`

This function:
1. Validates inputs
2. Converts Point3d[] to double*
3. Calls `TL_CubicNurbThroughPoints` (for degree=3)
4. Returns ON_NurbsCurve pointer

## 4. Core Algorithm (TL.DLL)

### TL_CubicNurbThroughPoints

```c
int TL_CubicNurbThroughPoints(
    uint dim,           // 3 for 3D points
    int point_count,    // Number of points
    double* points,     // Point array [x0,y0,z0,x1,y1,z1,...]
    uint closed_type,   // 0=open, 1=closed, 2=periodic
    double* start_tan,  // Start tangent or NULL
    double* end_tan,    // End tangent or NULL
    int knot_style,     // 0=uniform, 1=chord, 2=sqrt-chord
    uint* output        // Output: NURB handle (40 bytes)
);
```

### Algorithm Steps

```
1. PARAMETERIZATION
   ├── uniform: t[i] = i
   ├── chord: t[i] = t[i-1] + distance(p[i-1], p[i])
   └── centripetal: t[i] = t[i-1] + sqrt(distance(p[i-1], p[i]))

2. KNOT VECTOR
   ├── First 3 knots = t[0] (clamped start)
   ├── Interior knots = t[1..n-2]
   └── Last 3 knots = t[n-1] (clamped end)

3. END CONDITIONS
   ├── If tangent given: Use as derivative constraint
   ├── If NULL: Auto-compute using Bessel tangent
   └── Bessel: scale = distance / (knot_span)

4. MATRIX SETUP
   ├── Evaluate basis functions at each parameter
   ├── Build tridiagonal matrix [lower, diag, upper]
   └── RHS = input points

5. SOLVE
   └── Thomas algorithm (TL_SolveTriDiagonal)

6. OUTPUT
   └── Control points that interpolate input points
```

## 5. Key Sub-Functions

### TL_SolveTriDiagonal
```c
// Thomas algorithm for tridiagonal systems
int TL_SolveTriDiagonal(
    int dim,        // Dimension per variable
    int n,          // Number of equations
    double* lower,  // Lower diagonal
    double* diag,   // Main diagonal
    double* upper,  // Upper diagonal (modified)
    double* rhs,    // Right-hand side
    double* x       // Solution output
);
```

### TL_EvNurbBasis
```c
// Evaluate NURBS basis functions at parameter t
void TL_EvNurbBasis(
    int order,      // Curve order (degree + 1)
    double* knot,   // Knot vector
    double t,       // Parameter value
    double* basis   // Output: order basis values
);
```

### TL_GrevilleAbcissa
```c
// Compute Greville points (average of degree knots)
// g[i] = (knot[i+1] + ... + knot[i+degree]) / degree
int TL_GrevilleAbcissa(
    int order,
    int cv_count,
    double* knot,
    int cv0, int cv1,
    double* g
);
```

## 6. Data Structures

### Input Point Array
```
points[0] = x0, points[1] = y0, points[2] = z0,
points[3] = x1, points[4] = y1, points[5] = z1,
...
```

### Output NURB Handle (40 bytes)
```c
struct {
    uint32_t dim;        // 3
    uint32_t is_rat;     // 0 (non-rational)
    uint32_t order;      // 4 (cubic)
    uint32_t cv_count;   // point_count + 2
    double*  cv;         // Control vertices
    double*  knot;       // Knot vector
    uint32_t flags;      // Internal flags
    uint32_t reserved;
};
```

## 7. Implementation in session_cpp

```cpp
NurbsCurve NurbsCurve::create_interpolated(
    const std::vector<Point>& points,
    int degree,
    CurveKnotStyle knot_style,
    const Vector* start_tangent,
    const Vector* end_tangent
) {
    // 1. Compute parameters based on knot_style
    std::vector<double> params = compute_parameters(points, knot_style);

    // 2. Build knot vector (clamped)
    std::vector<double> knots = build_clamped_knots(params, degree);

    // 3. Compute end tangents if not provided
    Vector start_tan = start_tangent ? *start_tangent
                                     : bessel_tangent_start(points, params);
    Vector end_tan = end_tangent ? *end_tangent
                                 : bessel_tangent_end(points, params);

    // 4. Build interpolation matrix (tridiagonal)
    auto [lower, diag, upper, rhs] = build_interp_matrix(
        points, params, knots, degree, start_tan, end_tan
    );

    // 5. Solve tridiagonal system
    std::vector<Point> cvs = solve_tridiagonal(lower, diag, upper, rhs);

    // 6. Construct NURBS curve
    return NurbsCurve(degree, cvs, knots);
}
```

## 8. Test Vectors

### Simple 4-Point Case
```
Input:  [(0,0,0), (1,1,0), (2,0,0), (3,1,0)]
Degree: 3
Knots:  Chord

Expected Output:
  Knots: [0,0,0,0, 1.414, 2.828, 4.242, 4.242,4.242,4.242]
  CVs: [(0,0,0), (0.33,0.47,0), (1,1.2,0), (2,0.2,0), (2.67,0.53,0), (3,1,0)]
```

## 9. Knot Style Mapping

| Rhino CurveKnotStyle | TL.DLL knot_style | closed_type |
|----------------------|-------------------|-------------|
| Uniform (0) | 0 | 0 |
| Chord (1) | 1 | 0 |
| ChordSquareRoot (2) | 2 | 0 |
| UniformPeriodic (3) | 0 | 1 or 2 |
| ChordPeriodic (4) | 1 | 1 or 2 |
| ChordSquareRootPeriodic (5) | 2 | 1 or 2 |

## 10. Source Files

| Layer | File | Key Functions |
|-------|------|---------------|
| C# | `Curve.cs` | `CreateInterpolatedCurve` |
| C# | `CurveKnotStyle.cs` | Enum definition |
| P/Invoke | `UnsafeNativeMethods.cs` | `RHC_RhinoInterpCurve` |
| Native | `NURB_FIT.cpp` | `TL_CubicNurbThroughPoints` |
| Native | `MATH.cpp` | `TL_SolveTriDiagonal` |
