# Rhino TL.DLL Algorithm Analysis

Detailed analysis of decompiled algorithms from Rhino 8's TL.DLL.

## 1. NURBS Curve Interpolation

### TL_CubicNurbThroughPoints (Entry Point)

**Source:** `D:\BuildAgent\work\dujour\src4\tl\NURB_FIT.cpp`

**Signature:**
```c
int TL_CubicNurbThroughPoints(
    uint dim,           // Dimension (2 or 3)
    int point_count,    // Number of input points (must be > 1)
    double* points,     // Input point array
    uint closed_type,   // 0=open, 1=closed, 2=closed periodic
    double* start_tan,  // Start tangent (NULL for auto)
    double* end_tan,    // End tangent (NULL for auto)
    int knot_style,     // 0=uniform, 1=chord, 2=centripetal
    uint* output        // Output NURB handle (10 uint32 values)
)
```

**Algorithm:**
1. **Validation:**
   - point_count > 1
   - closed_type < 4
   - If closed, start_tan and end_tan must be NULL

2. **Control point count:**
   - Open curve: `cv_count = point_count + 2`
   - Closed (type 1 or 2): `cv_count = point_count + 3`

3. **Parameterization (knot_style):**
   - **0 (Uniform):** Parameters are simply `[0, 1, 2, ..., n]`
   - **1 (Chord-length):** Sum of distances between consecutive points
   - **2 (Centripetal):** Sum of `sqrt(distance)` between points

4. **Parameter vector construction:**
   - First compute cumulative chord/sqrt-chord lengths
   - For closed curves, wrap parameters around
   - Min/max distance tracking for validation

5. **Tolerance check:**
   - Fails if `min_dist <= max_dist * 1.49e-08` (machine epsilon)

6. **Calls `TL_CubicNurbInterpolatePoints`** with computed parameters

### TL_CubicNurbInterpolate (Core Solver)

**Source:** `D:\BuildAgent\work\dujour\src4\tl\NURB_FIT.cpp`

**Signature:**
```c
int TL_CubicNurbInterpolate(
    int dim,            // Dimension
    int cv_count,       // Control vertex count (>= 4)
    double* knot,       // Knot vector
    int start_cond,     // Start end condition (0-3)
    int end_cond,       // End end condition (0-3)
    double* cv          // Control vertices (input/output)
)
```

**End Condition Types:**
- **0 (Free):** First two CVs define tangent, natural second derivative
- **1 (First Derivative):** Tangent vector specified in cv[dim..2*dim-1]
- **2 (Second Derivative):** Curvature specified
- **3 (Natural/Bessel):** Scale tangent from point spacing

**Algorithm:**

1. **Knot vector validation:**
   ```
   knot[0] == knot[1] == knot[2] (triple start)
   knot[cv_count-2] != knot[cv_count-1]
   knot[cv_count] == knot[cv_count+1] (triple end)
   ```

2. **Special case for 4 CVs:**
   - If start_cond == 2: Bezier interpolation with 2/3, 1/3 blend
   - If end_cond == 2: Similar for end

3. **Start condition processing:**
   - **Type 3 (Bessel):** Scale tangent by `distance / (knot[3] - knot[2])`
   - **Type 1:** Use `TL_EvNurbBasis` and `TL_EvNurbBasisDer` for derivative
   - **Type 2:** Second derivative condition

4. **Interior point matrix setup:**
   - Loop through interior points (1 to cv_count-3)
   - Evaluate basis functions at each Greville point
   - Build tridiagonal matrix

5. **End condition processing:**
   - Similar to start condition

6. **Solve tridiagonal system:**
   - Calls `TL_SolveTriDiagonal`

### TL_SolveTriDiagonal (Matrix Solver)

**Source:** `D:\BuildAgent\work\dujour\src4\tl\MATH.cpp`

**Signature:**
```c
int TL_SolveTriDiagonal(
    int dim,            // Dimension of each variable
    int n,              // Number of equations
    double* lower,      // Lower diagonal
    double* diag,       // Main diagonal
    double* upper,      // Upper diagonal (becomes scratch space)
    double* rhs,        // Right-hand side (input)
    double* solution    // Solution (output)
)
```

**Algorithm:**
Classic Thomas algorithm (tridiagonal matrix algorithm):

1. **Forward elimination:**
   ```
   for i = 1 to n-1:
       scale = 1.0 / diag[i-1]
       if (diag[i-1] == 0): return -2  // Singular
       upper[i-1] = scale * upper[i-1]
       factor = lower[i] * upper[i-1]
       diag[i] = diag[i] - factor * diag[i-1]
       solution[i] = (rhs[i] - lower[i] * solution[i-1]) / diag[i]
   ```

2. **Back substitution:**
   ```
   for i = n-2 down to 0:
       solution[i] = solution[i] - upper[i] * solution[i+1]
   ```

3. **Multi-dimensional handling:**
   - For dim > 1, processes each coordinate independently
   - Interleaved storage pattern

## 2. Greville Abscissa Calculation

### TL_GrevilleAbcissa

**Source:** `D:\BuildAgent\work\dujour\src4\tl\MATH.cpp`

**Signature:**
```c
int TL_GrevilleAbcissa(
    int order,          // Curve order (degree + 1)
    int cv_count,       // CV count
    double* knot,       // Knot vector
    int cv0,            // Start CV index
    int cv1,            // End CV index
    double* g           // Output Greville points
)
```

**Algorithm:**
- For order == 2: Simply copy knot values (linear case)
- For order > 2: Call `ON_GrevilleAbcissa` (OpenNURBS function)

**Greville formula:**
```
g[i] = (knot[i+1] + knot[i+2] + ... + knot[i+degree]) / degree
```

## 3. Curve Blending

### TL_BlendNurbs

**Source:** `D:\BuildAgent\work\dujour\src4\tl\NURB.cpp`

**Signature:**
```c
int TL_BlendNurbs(
    uint dim,           // Dimension (max 3)
    int is_rational,    // 0 or 1
    uint point_count,   // Number of points to blend
    double* xform1,     // 4x4 transformation matrix for curve 1
    double blend1,      // Blend factor for curve 1
    void* points1,      // Points from curve 1
    double* xform2,     // 4x4 transformation matrix for curve 2
    double blend2,      // Blend factor for curve 2
    void* points2,      // Points from curve 2
    double* output      // Blended output points
)
```

**Algorithm:**
Linear interpolation between transformed points:

**Non-rational case (is_rational == 0):**
```c
for each point i:
    p1 = transform(xform1, points1[i])
    p2 = transform(xform2, points2[i])
    output[i] = blend1 * p1 + blend2 * p2
```

**Rational case (is_rational == 1):**
```c
for each point i:
    w1 = points1[i].weight
    w2 = points2[i].weight
    // Weight-adjusted blending
    output[i].xyz = blend1 * p1.xyz + blend2 * p2.xyz
    output[i].w = blend1 * w1 + blend2 * w2
```

## 4. Curve Offset

### TL_OffsetNurb

**Source:** `D:\BuildAgent\work\dujour\src4\tl\offset.cpp`

**Algorithm Overview:**
1. **Validate input:** dim must be 2 or 3, NURB must be valid
2. **Transform to XY plane** using rotation matrix
3. **Zero out Z-component** if 3D (project to plane)
4. **Call `TL_Offset2dNurb`** for actual offset
5. **Transform back** using inverse matrix

**Key insight:** 3D curve offset is done by:
1. Computing plane of curve
2. Projecting to 2D
3. Offsetting in 2D (much simpler)
4. Transforming back to 3D

## 5. Boolean Operations

### TL_BrepUnion

**Workflow:**
1. **Cast to TL_Brep:** Check if input is already TL_Brep or promote from ON_Brep
2. **Create TL_BrepBoolean:** Initialize with `copy = true`
3. **Merge:** `TL_BrepBoolean::Merge(brep1, brep2, tolerance)`
4. **Union:** `TL_BrepBoolean::Union()` returns new TL_Brep
5. **Cleanup:** Destroy temporary TL_BrepBoolean

### TL_BrepBoolean Workflow

**Key methods:**
- `Merge(brep1, brep2, tol)` - Compute intersection curves
- `Union()` - Keep faces outside both breps
- `Intersection()` - Keep faces inside both breps
- `Difference()` - Keep faces of A outside B

### DoJoin (Brep Join)

**Algorithm:**
1. **SetVertexTols:** Compute vertex tolerances
2. **MatchVertices:** Match coincident vertices
3. **CheckVerticesOnEdges:** Snap vertices to edges
4. **SplitEdges:** Split edges at matched vertices
5. **SetUserFlags:** Mark topology elements
6. **Merge:** Create merged brep
7. **MatchEdges:** Join coincident edges
8. **Cleanup:**
   - `StandardizeEdgeCurves()`
   - `Compact()`

### TL_IntersectFaces

**Signature:**
```c
bool TL_IntersectFaces(
    ON_Brep* brep1, int face1,
    ON_Brep* brep2, int face2,
    double tolerance,
    bool include_tangent,
    TL_CurvePairArray* curves_on_face1,
    TL_CurvePairArray* curves_on_face2
)
```

**Algorithm:**
1. Promote breps to TL_Brep if needed
2. Create `TL_FaceIntersector`
3. `Intersect()` - compute SSI curves
4. `GetCurvePairs()` - extract 2D/3D curve pairs

## 6. Key Data Structures

### NURB Handle (tagTL_NURB)
```c
struct tagTL_NURB {
    uint32_t dim;        // offset 0x00
    uint32_t is_rat;     // offset 0x04
    uint32_t order;      // offset 0x08
    uint32_t cv_count;   // offset 0x0C
    double*  cv;         // offset 0x10
    double*  knot;       // offset 0x18
    void*    cache;      // offset 0x20 (optional)
};
```

### Output Array from TL_CubicNurbThroughPoints
```c
uint32_t output[10]:
  [0] = dim
  [1] = is_rational (0)
  [2] = order (4 for cubic)
  [3] = cv_count
  [4,5] = cv pointer (64-bit)
  [6,7] = knot pointer (64-bit)
  [8] = flags
  [9] = reserved
```

## 7. Key Constants

- **Machine epsilon check:** `1.490116119385e-08` (~sqrt(DBL_EPSILON))
- **Default blend:** `0.3333333333333333` (1/3)
- **Default blend 2:** `0.6666666666666666` (2/3)

## 8. Source File Mapping

| File | Functions |
|------|-----------|
| NURB_FIT.cpp | TL_CubicNurbInterpolate, TL_CubicNurbThroughPoints, TL_NurbInterpolate |
| NURB.cpp | TL_BlendNurbs, TL_GrevilleKnots |
| MATH.cpp | TL_SolveTriDiagonal, TL_GrevilleAbcissa, TL_Solve* |
| OFFSET.cpp | TL_OffsetNurb, TL_Offset2dNurb |
| BOOLEAN.cpp | TL_BrepBoolean, TL_BrepUnion, TL_BrepDifference |
| TRANSFRM.cpp | TL_XformPoints, TL_GetVectorRotationXform |

## 9. Implementation Priority

### High Priority (Core NURBS)
1. TL_SolveTriDiagonal - Thomas algorithm
2. TL_CubicNurbInterpolate - Interpolation matrix
3. TL_GrevilleAbcissa - Knot averaging

### Medium Priority (Operations)
4. TL_OffsetNurb - Curve offset
5. TL_BlendNurbs - Curve blending
6. TL_NurbSrfInterpolate - Surface fitting

### Lower Priority (Booleans)
7. TL_BrepBoolean - Complex, many dependencies
8. TL_BrepIntersector - SSI algorithms

## 10. Notes for Implementation

1. **Parameterization is key:** Rhino uses raw chord-length parameters without normalization
2. **End conditions:** Bessel scaling uses distance/knot-span ratio
3. **Tridiagonal solver:** Standard Thomas algorithm, watch for singularity
4. **Offset:** Project to plane, offset in 2D, project back
5. **Booleans:** TL_Brep is enhanced ON_Brep with intersection data
