# Rhino 8 tl.dll Geometry Algorithms Reference

Comprehensive API map of `tl.dll` (711 C-style exports, ~7100 C++ class methods) covering NURBS curves, surfaces, meshing, trimmed surfaces, and BReps. Source: IDA/Ghidra decompilation of Rhino 8 x64 binaries.

---

## Data Structures

### TL_NURB (40 bytes)
```c
struct TL_NURB {
    int dim;        // 1, 2, or 3
    int is_rat;     // 0 or 1
    int order;      // >= 2 (degree = order - 1)
    int cv_count;   // >= order
    double* cv;     // stride = dim + is_rat
    double* knot;   // count = order + cv_count - 2
    int cache;
};
```

### tagTL_NURBSRF (surface version)
Same layout as TL_NURB but with dual order/cv_count/knot for U and V directions.

### tagTL_BEZSRF (Bezier surface)
Subset of NURBSRF with implicit uniform knots.

### ON_PointGrid
Point grid for interpolation — stores dim, point counts in u/v, stride, and pointer to contiguous double array.

### TL_2dMeshInput
Meshing parameter struct with boundary polylines, iso-constraints, point scale, and tolerance fields. Has `IsValid()`, `MakeValid()`, `Read()`/`Write()` for serialization.

---

## A. NURBS Curves (~120 functions)

### Creation / Interpolation

| Function | Description |
|----------|-------------|
| `TL_CubicNurbInterpolate` | Cubic B-spline through points — tridiagonal solver for CV computation |
| `TL_CubicNurbThroughPoints` | Higher-level wrapper: computes chord-length params, calls `TL_CubicNurbInterpolate` |
| `TL_CubicNurbInterpolatePeriodic` | Periodic cubic interpolation (closed curves) |
| `TL_CubicNurbInterpolateHermite` | Hermite interpolation with specified tangents |
| `TL_CubicNurbInterpolatePoints` | Cubic interpolation through point array |
| `TL_NurbInterpolate` | General order interpolation through points |
| `TL_NurbGrevilleInterpolate` | Interpolation at Greville abscissae (knot averages) |
| `TL_NurbThroughPoints` | General NURB through points |
| `TL_CreateNurb` | Allocate TL_NURB struct |
| `TL_CopyNurb` | Deep copy a NURB |
| `TL_CopyNurbSpan` | Copy single span |
| `TL_CopySubNurb` | Copy sub-curve |
| `TL_NurbLine` | Create degree-1 NURB line |
| `TL_NurbArc` | Create rational NURB arc |
| `TL_QuadraticNurbArc` | Quadratic rational arc |
| `TL_QuadraticNurbEllipse` | Quadratic rational ellipse |
| `TL_QuinticNurbEllipse` | Quintic polynomial ellipse approximation |
| `TL_CubicNurbFitEllipse` | Cubic fit to ellipse |
| `TL_NurbFitToHelix` | Fit NURB to helix |
| `TL_NurbFitToSweptHelix` | Fit to swept helix |
| `TL_NurbFitToToridalHelix` | Fit to toroidal helix |
| `TL_NurbSphere` | NURB sphere |
| `TL_NurbTorus` | NURB torus |
| `TL_NurbFrustum` | NURB frustum (cone/cylinder) |
| `TL_ConeNurb` | NURB cone |

### Evaluation

| Function | Description |
|----------|-------------|
| `TL_EvNurb` | Evaluate point on NURB at parameter t |
| `TL_EvNurb1Der` | Point + first derivative |
| `TL_EvNurb2Der` | Point + first + second derivatives |
| `TL_EvNurbPoint` | Point only (no derivatives) |
| `TL_EvNurbPointList` | Evaluate at multiple parameters |
| `TL_EvNurbSpan` | Evaluate within single span |
| `TL_EvNurbClosestPoint` | Find closest point on NURB to test point |
| `TL_EvNurbFarthestPoint` | Find farthest point |
| `TL_EvNurbFrenetFrame` | Frenet frame (tangent, normal, binormal) |
| `TL_EvNurbUnitTangent` | Unit tangent vector |
| `TL_EvNurbStartPoint` | Start point |
| `TL_EvNurbEndPoint` | End point |
| `TL_EvCurvature` | Curvature at parameter |
| `TL_EvNurbBasis` | B-spline basis functions |
| `TL_EvNurbBasisDer` | B-spline basis function derivatives |
| `TL_EvdeBoor` | de Boor evaluation algorithm |
| `TL_EvdeBoor2` | de Boor with second level |

### Arc Length

| Function | Description |
|----------|-------------|
| `TL_LengthNurb` | Total arc length of NURB |
| `TL_LengthNurbPolygon` | Control polygon length |
| `TL_ArcLengthPointOnNurb` | Parameter at given arc length |
| `TL_ArcLengthPointsOnNurb` | Multiple parameters at arc lengths |
| `TL_ArcLengthPointsOnNurbFast` | Fast approximation |
| `TL_ArcLengthPointsOnNurbSlow` | High accuracy version |
| `TL_ArcLengthPointsOnNurbTolerance` | With tolerance control |

### Modification

| Function | Description |
|----------|-------------|
| `TL_TrimNurb` | Trim to sub-domain |
| `TL_SplitNurb` | Split at parameter |
| `TL_ExtendNurb` | Extend beyond domain |
| `TL_ReverseNurb` | Reverse direction |
| `TL_MergeNurbs` | Join two NURBs end-to-end |
| `TL_IncreaseNurbDegree` | Degree elevation |
| `TL_ChangeNurbDim` | Change spatial dimension |
| `TL_CollapseNurb` | Collapse to point |
| `TL_FairNurb` | Fair (smooth) a NURB |
| `TL_TranslateNurb` | Translate control points |
| `TL_XformNurb` | Apply 4x4 transform |
| `TL_ModifyNurbEndDirections` | Adjust end tangent directions |
| `TL_ModifyNurbEndPoints` | Move endpoints |
| `TL_SetClosedNurbStart` | Change seam point of closed NURB |
| `TL_MoveControlVertices` | Move CVs |
| `TL_SetNurbDomain` | Reparametrize domain |
| `TL_ImproveNurbDomain` | Improve domain for numerical stability |
| `TL_ClampKnot` | Clamp knot vector |
| `TL_ClampNurbEnd` | Clamp one end |

### Knot Operations

| Function | Description |
|----------|-------------|
| `TL_AddNurbKnot` | Insert knot |
| `TL_RemoveNurbKnots` | Remove knots within tolerance |
| `TL_RemoveExcessKnots` | Remove unnecessary knots |
| `TL_RemoveExcessNurbKnots` | Variant |
| `TL_MergeKnots` | Merge knot vectors |
| `TL_MakeKnotsPeriodic` | Make knot vector periodic |
| `TL_MakeKnotsUniform` | Make uniform spacing |
| `TL_MakeEndKnotMultiple` | Ensure end knot multiplicity |
| `TL_SuperfluousKnot` | Detect removable knots |
| `TL_CompareKnots` | Compare knot vectors |
| `TL_ValidateKnotVector` | Validate knot vector |
| `TL_GetKnotDomain` | Get parameter domain from knots |
| `TL_GetKnotSpan` | Get span index for parameter |
| `TL_GetKnotSpanCount` | Number of spans |
| `TL_GetKnotSpanIndices` | All span boundary indices |
| `TL_GetKnotMultiplicity` | Multiplicity at knot value |
| `TL_GetKnotTolerance` | Tolerance for knot comparisons |
| `TL_GetKnotList` | Extract unique knot values |
| `TL_CountMultipleKnots` | Count knots with multiplicity > 1 |
| `TL_GrevilleAbcissa` | Greville point (knot average) for CV index |
| `TL_GrevilleKnots` | All Greville points |
| `TL_AddNurbGrevilleAbcissa` | Insert Greville knot |

### Compatibility

| Function | Description |
|----------|-------------|
| `TL_MakeNurbsCompatible` | Full compatibility: degree + knots + rational |
| `TL_MakeNurbsDegreeCompatible` | Match degrees only |
| `TL_MakeNurbsKnotCompatible` | Match knot vectors by insertion |
| `TL_MakeNurbsRatCompatible` | Make all rational or all non-rational |
| `TL_MakeNurbsDomainCompatible` | Match parameter domains |
| `TL_MakeNurbsKinkCompatible` | Insert knots at kinks |
| `TL_MakeNurbsKinkCompatibleEx` | Extended version |
| `TL_MakeNurbPeriodic` | Make NURB periodic |
| `TL_MakeNurbNonRational` | Convert to non-rational |
| `TL_MakeNurbRational` | Convert to rational |
| `TL_MakeNurbC1` | Ensure C1 continuity |
| `TL_MakeNurbC2` | Ensure C2 continuity |
| `TL_MakeNurbCVsPeriodic` | Make CVs periodic |

### Fitting

| Function | Description |
|----------|-------------|
| `TL_FitNurbToPoints` | Least-squares fit to points |
| `TL_FitNurbToPointsAndParams` | Fit with specified parameters |
| `TL_FitNurbToPointsParams` | Fit with params |
| `TL_FitNurbToPointsParamsKnots` | Fit with params and knots |
| `TL_FitNurbToNurb` | Refit NURB to target structure |
| `TL_NurbFitToNurb` | Same, alternate name |
| `TL_NurbFitToNurbs` | Fit to multiple NURBs |
| `TL_NurbFitToFunction` | Fit to function |
| `TL_ReallyFitNurbToPoints` | High-quality fit |
| `TL_CreateNurbLSqFit` | Initialize least-squares fitter |
| `TL_AddPointNurbLSqFit` | Add point to fitter |
| `TL_AddPointListNurbLSqFit` | Add point list |
| `TL_PegCVNurbLSqFit` | Pin a CV |
| `TL_PegCVsNurbLSqFit` | Pin multiple CVs |
| `TL_SolveNurbLSqFit` | Solve the fit |
| `TL_UniformCubicFit` | Uniform cubic fit |
| `TL_CubicBezierFit` | Cubic Bezier fit |
| `TL_3DPolylineFitToNurb` | Fit polyline to NURB |
| `TL_FitNurbInSections` | Sectional fit |
| `TL_2dNurbFitToNurbOnSrf` | 2D fit on surface |

### Bezier

| Function | Description |
|----------|-------------|
| `TL_EvBezier` | Evaluate Bezier |
| `TL_EvBezier1Der` | Bezier + 1st derivative |
| `TL_EvBezier2Der` | Bezier + 2nd derivative |
| `TL_EvBezierPoint` | Bezier point only |
| `TL_EvBezierClosestPoint` | Closest point on Bezier |
| `TL_EvBezierFarthestPoint` | Farthest point |
| `TL_EvBezierUnitTangent` | Unit tangent |
| `TL_EvBezierRPFrame` | Rotation-minimizing frame |
| `TL_EvBezierStartPoint` | Start point |
| `TL_EvBezierEndPoint` | End point |
| `TL_EvBezierLocalExtremum` | Local extremum |
| `TL_EvdeCasteljau` | de Casteljau evaluation |
| `TL_EvCubicBezier1Der` | Cubic Bezier 1st derivative |
| `TL_EvCubicBezierClosestPoint` | Cubic Bezier closest point |
| `TL_Ev1dBezier` | 1D Bezier evaluation |
| `TL_EvBernsteinBasis` | Bernstein basis functions |
| `TL_ConvertNurbToBezier` | Single span to Bezier |
| `TL_ConvertNurbToBeziers` | All spans to Beziers |
| `TL_ConvertBezierToNurb` | Bezier to NURB |
| `TL_SplitBezier` | Split Bezier at parameter |
| `TL_TrimBezier` | Trim Bezier to sub-domain |
| `TL_ExtendBezier` | Extend Bezier |
| `TL_IncreaseBezierDegree` | Degree elevation |
| `TL_ChangeBezierOrder` | Change order |
| `TL_ChangeBezierDim` | Change dimension |
| `TL_MultiplyBeziers` | Multiply two Beziers |
| `TL_CreateBezier` | Allocate Bezier |
| `TL_CopyBezier` | Deep copy |
| `TL_DestroyBezier` | Free |
| `TL_MakeBezierRational` | Make rational |
| `TL_MakeBezierNonRational` | Make non-rational |
| `TL_SetBezierKnots` | Set knots |
| `TL_SetBezierEndCondition` | Set end conditions |
| `TL_GetBezierBoundingBox` | Bounding box |
| `TL_GetBezierRoots` | Root finding |
| `TL_GetMonotoneBezierRoot` | Monotone root |
| `TL_IsBezierLinear` | Linearity test |
| `TL_IsBezierCircular` | Circularity test |
| `TL_IsBezierClosed` | Closure test |
| `TL_IsBezierPlanar` | Planarity test |
| `TL_IsBezierMonotone` | Monotonicity test |
| `TL_ReparametrizeRationalBezier` | Reparametrize |

### Query / Test

| Function | Description |
|----------|-------------|
| `TL_GetNurbBoundingBox` | Bounding box from CVs |
| `TL_GetNurbTightBoundingBox` | Tight bounding box (evaluates) |
| `TL_GetNurbDomain` | Parameter domain |
| `TL_GetNurbClosestPoint` | Closest point |
| `TL_GetNurbNurbClosestPoint` | Closest point between two NURBs |
| `TL_GetNurbFlags` | NURB flags |
| `TL_GetNurbKink` | Find kink parameter |
| `TL_GetNurbKinks` | All kinks |
| `TL_GetNurbSpan` | Span domain |
| `TL_GetNurbWeightFlag` | Weight flag |
| `TL_IsNurbLinear` | Linearity test |
| `TL_IsNurbCircular` | Circularity test |
| `TL_IsNurbConic` | Conic test |
| `TL_IsNurbEllipse` | Ellipse test |
| `TL_IsNurbClosed` | Closure test |
| `TL_IsNurbPlanar` | Planarity test |
| `TL_IsNurbMonotone` | Monotonicity test |
| `TL_IsNurbUniform` | Uniformity test |
| `TL_AreKnotsPeriodic` | Periodic knot test |
| `TL_AreNurbCVsPeriodic` | Periodic CV test |
| `TL_ValidateNurb` | Full validation |
| `TL_CompareNurbs` | Compare two NURBs |
| `TL_NurbCV` | Access CV |
| `TL_NurbPoint` | Evaluate point |

### Offset / Blend

| Function | Description |
|----------|-------------|
| `TL_OffsetNurb` | 3D offset |
| `TL_Offset2dNurb` | 2D offset |
| `TL_Offset2dBezier` | 2D Bezier offset |
| `TL_OffsetBezier` | 3D Bezier offset |
| `TL_Offset2dPolyline` | 2D polyline offset |
| `TL_BevelOffset` | Bevel corner treatment |
| `TL_ConvertOffsetSegmentsToNurb` | Convert offset segments to NURB |
| `TL_BlendNurbs` | Blend between two NURBs |
| `TL_BlendCrvToSrf` | Blend curve to surface |
| `TL_BlendNurbToSrf` | Blend NURB to surface |
| `TL_GetBlendingWeights` | Compute blend weights |
| `TL_GetChordalG2BlendQuinticCV12` | G2 quintic blend CVs |

### Intersection

| Function | Description |
|----------|-------------|
| `TL_IntersectNurbNurb` | Curve-curve intersection |
| `TL_IntersectNurbPlane` | Curve-plane intersection |

### Orientation

| Function | Description |
|----------|-------------|
| `TL_NurbOrientation` | 2D curve orientation (CW/CCW) |
| `TL_NurbListOrientation` | List orientation |
| `TL_NurbArrayOrientation` | Array orientation |
| `TL_3dNurbOrientation` | 3D orientation |
| `TL_3dNurbListOrientation` | 3D list orientation |
| `TL_3dNurbArrayOrientation` | 3D array orientation |

### Editing (Interactive)

| Function | Description |
|----------|-------------|
| `TL_EditNurbGrevillePointsBegin` | Begin Greville edit |
| `TL_EditNurbPoints` / `Begin` / `End` | CV point editing |
| `TL_EditNurbPolygon` / `Begin` / `End` | Control polygon editing |
| `TL_EditNurbHandleBar` / `Begin` / `End` | Handle bar editing |

---

## B. NURBS Surfaces (~100+ functions)

### Creation

| Function | Description |
|----------|-------------|
| `TL_LoftNurbSrf` | Loft through section curves |
| `TL_LoftNurb` | Loft helper |
| `TL_LoftCubicBezier` | Cubic Bezier loft |
| `TL_LoftQuadraticBezier` | Quadratic Bezier loft |
| `TL_RevolveNurb` | Surface of revolution |
| `TL_ExtrudeNurb` | Extrude curve along vector |
| `TL_RuleNurbSrf` | Ruled surface between two curves |
| `TL_CoonsPatchNurbSrf` | Coons patch from boundary curves |
| `TL_PlaneNurbSrf` | Planar NURB surface |
| `TL_Sweep` | 1-rail and 2-rail sweep |
| `TL_SwingNurb` | Swing surface |
| `TL_CreateNurbSrf` | Allocate surface struct |
| `TL_CopyNurbSrf` | Deep copy |
| `TL_CopyNurbSrfSpan` | Copy single span |
| `TL_NurbSrfInterpolate` | Surface interpolation through point grid |
| `TL_NurbSrfPoint` | Access surface CV |
| `TL_ConvertBezSrfToNurbSrf` | Bezier surface to NURB surface |
| `TL_TensorProduct` | Tensor product surface from curves |
| `TL_2CornerFill` | 2-corner fill surface |

#### TL_NurbsSurface Class Methods

| Method | Description |
|--------|-------------|
| `CreateCubicSurfaceThroughPointGrid` | **Key**: Bicubic interpolation through point grid with centripetal parameterization |
| `CreateSurfaceThroughPointGrid` | General order interpolation |
| `CreateSurfaceOfRevolution` | Revolution surface |
| `ConvertFromCurve` | Create surface from curve |
| `ConvertToCurve` | Extract isocurve |
| `ConvertToRationalCurve` | Extract rational isocurve |
| `IncreaseDegree` | Degree elevation in one direction |
| `MakePeriodic` | Make periodic in one direction |
| `MorphTo` / `MorphFrom` | Convert to/from flat TL_NURBSRF |
| `RemoveKnots` | Remove knots |
| `RemoveMicroKnotSpans` | Remove tiny spans |
| `RemoveSpan` | Remove a span |
| `Pullback` | Pull 3D curve onto surface (get 2D parameter curve) |
| `Pushup` | Push 2D parameter curve to 3D |
| `GetLocalClosestPoint` | Local closest point search |
| `Offset` | Offset surface |

### Evaluation

| Function | Description |
|----------|-------------|
| `TL_EvNurbSrf` | Evaluate surface at (u,v) |
| `TL_EvNurbSrf1Der` | Point + first partial derivatives |
| `TL_EvNurbSrf2Der` | Point + first + second derivatives |
| `TL_EvNurbSrfPoint` | Point only |
| `TL_EvNurbSrfNormal` | Surface normal |
| `TL_EvNurbSrfCornerPoint` | Corner point |
| `TL_EvNurbSrfIsoParam` | Isocurve extraction |
| `TL_EvNurbSrfPointGrid` | Evaluate grid of points |
| `TL_EvNurbSrf9x9Grid` | 9x9 evaluation grid |
| `TL_EvNurbSrfSpan` | Evaluate within span |
| `TL_EvNurbSrfSingular1Der` | 1st derivative at singular point |
| `TL_EvNurbSrfSingular2Der` | 2nd derivative at singular point |
| `TL_EvPrincipalCurvatures` | Principal curvatures and directions |
| `TL_EvNormal` | Surface normal from derivatives |
| `TL_EvNormalCurvature` | Normal curvature |
| `TL_EvJacobian` | Jacobian matrix |

### Modification

| Function | Description |
|----------|-------------|
| `TL_SplitNurbSrf` | Split surface at parameter |
| `TL_ExtendNurbSrf` | Extend surface |
| `TL_OffsetNurbSrf` | Offset surface |
| `TL_OffsetNurbSrfBox` | Box-style offset |
| `TL_OffsetSurface` | General offset |
| `TL_TransposeNurbSrf` | Swap U/V directions |
| `TL_ReverseNurbSrf` | Reverse direction |
| `TL_IncreaseNurbSrfDegree` | Degree elevation |
| `TL_MakeNurbSrfNonRational` | Convert to non-rational |
| `TL_MakeNurbSrfRational` | Convert to rational |
| `TL_MakeNurbSrfPeriodic` | Make periodic |
| `TL_CollapseNurbSrfSide` | Collapse one side to point |
| `TL_AddNurbSrfKnot` | Insert knot |
| `TL_RemoveNurbSrfKnots` | Remove knots |
| `TL_XformNurbSrf` | Transform surface |
| `TL_TranslateNurbSrf` | Translate |

### Surface Compatibility

| Function | Description |
|----------|-------------|
| `TL_MakeNurbSrfsCompatible` | Full compatibility for surface array |
| `TL_MakeNurbSrfsDomainCompatible` | Match domains |
| `TL_MakeNurbSrfsKnotCompatible` | Match knots by insertion |
| `TL_MakeNurbSrfsRatCompatible` | Match rational/non-rational |
| `TL_MakeArcLengthCompatibleRuledSrf` | Arc-length compatible ruled surface |

### Surface Query

| Function | Description |
|----------|-------------|
| `TL_GetNurbSrfBoundingBox` | Bounding box |
| `TL_GetNurbSrfClosestPoint` | Closest point |
| `TL_GetNurbSrfDomain` | Domain |
| `TL_GetNurbSrfFootprint` | UV footprint |
| `TL_GetNurbSrfKinks` | Surface kinks |
| `TL_GetNurbSrfPlane` | Best-fit plane |
| `TL_IsNurbSrfClosed` | Closure test |
| `TL_IsNurbSrfG1` | G1 continuity test |
| `TL_IsNurbSrfG2` | G2 continuity test |
| `TL_IsNurbSrfPlanar` | Planarity test |
| `TL_IsNurbSrfSingular` | Singularity test |
| `TL_ValidateNurbSrf` | Full validation |

---

## C. Network Surface / Gordon Surface

### TL2_NetworkSurface Class

| Method | Description |
|--------|-------------|
| `TL2_NetworkSurface(n_u, n_v)` | Constructor: allocate for n_u u-curves and n_v v-curves |
| `IsValid()` | Validate curves, params, tolerances |
| `GetGrid(srf)` | Build bicubic interpolating surface through intersection points |
| `GetLoftSrf(dir, grid, out)` | Build loft surface through section curves |
| `CreateSurface(out)` | **Main entry**: Gordon formula S = L_u + L_v - T |
| `SmoothTip(srf)` | G1 continuity at singular boundaries |
| `IsClosed(dir)` | Closure test |
| `IsPeriodic(dir)` | Periodicity test |
| `IsSingular(dir, end)` | Singularity test |
| `Flag(dir)` | Boundary flag (1=sing_start, 2=sing_end, 3=both, 4=closed, 5=periodic) |
| `CurvePoint(dir, idx, t, pt)` | Evaluate curve point |

### Gordon Surface Algorithm (from `CreateSurface`)

```
1. GetGrid(T)          — Bicubic interpolation through intersection points
                         Uses centripetal parameterization (sqrt of chord length)
                         Calls CreateCubicSurfaceThroughPointGrid

2. GetLoftSrf(0, T, L_u) — Loft through u-direction curves
                           Each curve refit to grid structure via TL_NurbFitToNurb
                           Interpolation via TL_CubicNurbInterpolate

3. GetLoftSrf(1, T, L_v) — Loft through v-direction curves (same process)

4. MakeNurbSrfsKnotCompatible(U)  — Make all 3 surfaces knot-compatible in U
5. Transpose + MakeNurbSrfsKnotCompatible(V) + Transpose — Same in V
6. MakeNurbSrfsRatCompatible      — Match rational/non-rational

7. GORDON FORMULA (CV combination):
   for each CV(i,j):
     output_CV = L_v_CV + L_u_CV - T_CV

8. Handle closed boundaries — copy first row/col CVs to last
9. SmoothTip — project CVs for G1 at singular edges
10. MakeNurbSrfPeriodic — if periodic boundaries
```

### SmoothTip Algorithm (G1 at singular points)

```
For each singular boundary edge:
  1. Evaluate unit tangents at tip for each curve
  2. Average normal plane: cross products → pick max → unitize
  3. Verify against surface normal (cos 3° threshold)
  4. Project adjacent CV row onto tangent plane:
     delta = CV_adjacent - CV_tip
     projected = delta - dot(delta, normal) * normal
     CV_adjacent = CV_tip + projected
```

### Coons Patch (TL_CoonsPatchNurbSrf)

Full algorithm from decompilation:

```
1. Validate: need A[0]+A[1] or B[0]+B[1] boundary curves
2. Copy, orient, reverse if flagged
3. Make rational compatible (match endpoint weights)
4. MakeNurbsCompatible: A[0]↔A[1], B[0]↔B[1]
5. Build ruled surfaces:
   ruled_A = RuleNurbSrf(A[0], A[1])
   ruled_B = RuleNurbSrf(B[0], B[1])  // then transpose
   bilinear = 4-corner tensor product (corner averages)
6. MakeNurbSrfsKnotCompatible (U and V)
7. COONS FORMULA:
   result_CV = ruled_A_CV + ruled_B_CV - bilinear_CV
8. Handle collapsed sides: CollapseNurbSrfSide
```

### TlEdgePatch (4-Edge Surface)

```
PrepareEdges()          → Convert ON_Curve* to NurbsCurves
ChainCurves()           → Auto-chain 4 curves into closed loop
MatchCurvesStructure()  → Degree-elevate, knot-insert for opposite pairs
PreparePatchStructure() → Set up patch surface
MakeCoonsPatch()        → Build via Coons formula
MatchEdges()            → Final edge refinement
```

### TL_GeneralCoon (General Coons Surface)

| Method | Description |
|--------|-------------|
| `ComputeCorners()` | Compute 4 corners |
| `ComputeXBDerivatives()` | Cross-boundary derivatives |
| `CompatibleEdges()` | Make edges compatible |
| `MakeNurbsCompatible()` | Full compatibility |
| `SetSingularEdges()` | Handle degenerate edges |
| `MakeSurface()` | Create the surface |
| `CheckSurface()` | Validate result |
| `MaxContinuity()` | Max achievable continuity |

---

## D. Sweep

### TL_Sweep

```c
TL_Sweep(const tagTL_SWEEP& input, ON_SimpleArray<tagTL_NURBSRF>& output)
TL_Sweep(const tagTL_SWEEP& input, ON_SimpleArray<tagTL_MITER_NURBSRF>& output)
TL_CreateSweepShape   // Initialize sweep
TL_DestroySweepShape  // Cleanup
TL_GetSweepXform      // Get transform at parameter
TL_GetSrfEdgeRailFrames // Rail frames for edge sweep
```

### Rail Frame Functions

| Function | Description |
|----------|-------------|
| `TL_Get1RailFrames` | Frames along single rail curve |
| `TL_Get1RailFramesCP` | Cross-product variant |
| `TL_Get2RailFrame` | Single frame from 2 rails |
| `TL_Get2RailFrames` | All frames from 2 rails |
| `TL_Get2RailFramesAtCurveParameters` | Frames at specific parameters |
| `TL_Get2RailFramesCP` | Cross-product variant |
| `TL_GetPlanarRailFrame` | Planar rail frame |
| `TL_GetPlanarRailFrames` | All planar rail frames |
| `TL_GetFramesAtCurveParameters` | General frame extraction |
| `TL_GetFramesAtNormalizedArcLengthParameters` | Arc-length normalized frames |

### CRailHeader (Fillet/Chamfer Rails)

| Method | Description |
|--------|-------------|
| `BuildRailCurves()` | Build compatible rail curves |
| `BuildFilletSrfs()` | Build fillet surfaces |
| `BuildChamferSrfs()` | Build chamfer surfaces |
| `GetCompatibleRails()` | Match rail structures |
| `GetIntersectionCurves()` | Rail intersections |
| `GetFilletSrfs()` / `GetChamferSrfs()` | Retrieve results |
| `GetOffsets()` | Offset rail curves |
| `AddRailData()` | Add rail input |
| `CheckRailEnds()` | Validate rail endpoints |

---

## E. Meshing

### Surface Meshing

| Function | Description |
|----------|-------------|
| `TL_ConvertMesh` | Convert `tagTL_MESH` with `ON_MeshParameters` and `ON_Surface` |
| `TL_ConvertMeshParams` | Convert `ON_MeshParameters` to `tagTL_MESH_CREATE_PARAMETERS` |
| `TL_MeshNurb` | Polyline approximation of NURB curve |
| `TL_MeshNurbEx` | Extended version |
| `TL_MeshCurve` | General curve meshing |
| `TL_MeshCurveEx` | Extended version |

### 2D Triangulation

| Function | Description |
|----------|-------------|
| `TL_Create2dMesh` | 2D constrained triangulation from `TL_2dMeshInput` |
| `TL_Triangulate2dPolygon` | Ear-clipping 2D polygon triangulation |
| `TL_Triangulate3dPolygon` | 3D polygon triangulation |
| `TL_MeshPlanarPolygonalRegion` | Mesh planar region with holes |

### TL_2dMeshInput Class

| Method | Description |
|--------|-------------|
| `IsValid()` | Validate input |
| `MakeValid()` | Fix invalid input |
| `HaveIsoConstraints()` | Check for iso constraints |
| `ClearIsoConstraints()` | Remove iso constraints |
| `HavePointScale()` | Check for point scale |
| `PolishPointScale()` | Refine point scale |
| `Read()` / `Write()` | Serialization |

### Mesh Boolean (TL_MeshBoolean)

| Method | Description |
|--------|-------------|
| `TL_MeshBoolean(mesh_a, mesh_b)` | Constructor |
| `IntersectAndSplitInputMeshes()` | Split meshes at intersection |
| `GetBooleanUnion(result)` | Union |
| `GetBooleanDifference(result)` | Difference |
| `GetBooleanIntersection(result)` | Intersection |
| `GetBooleanSplit(a, b)` | Split into two halves |
| `GetMergedMesh(result)` | Merge without boolean |
| `GetSplitMeshes(a, b)` | Get split meshes |
| `PerformBoolean(a, b, flag)` | General boolean |
| `SetTolerances(abs, rel)` | Set tolerances |

Free function wrappers: `TL_MeshBooleanUnion`, `TL_MeshBooleanDifference`, `TL_MeshBooleanIntersection`, `TL_MeshBooleanSplit`.

### Mesh Operations

| Function | Description |
|----------|-------------|
| `TL_HealMesh` | Repair mesh (degenerate faces, gaps) |
| `TL_AlignMeshVertices` | Snap nearby vertices |
| `TL_SplitMeshEdge` | Split edge at points |
| `TL_MeshSilhouette` | Extract silhouette edges |
| `TL_MeshEdges` | Extract feature edges |
| `TL_PackMeshTextures` | Pack texture coordinates |
| `TL_GetPolyLineOnMesh` | Polyline along mesh surface |

### Mesh-Plane Intersection (TL_MeshXPlane)

| Method | Description |
|--------|-------------|
| `Intersect(point, normal, ...)` | Intersect mesh with plane |
| `ProcessVertices()` | Classify vertices |
| `ProcessEdges()` | Find edge crossings |
| `ProcessFaces()` | Triangulate face intersections |
| `ProcessTriangle()` | Single triangle intersection |
| `ProcessQuad()` | Single quad intersection |

### NGon Detection (TL_NGonFinder)

| Method | Description |
|--------|-------------|
| `TL_NGonFinder(mesh)` | Constructor |
| `FindNGons(count)` | Find n-gon faces |
| `FindFacesInNgons(ids)` | Find faces within ngons |
| `ComputeNgonBoundaries(ngon)` | Compute boundary loops |
| `CreatePolylines(ngon, out)` | Extract boundary polylines |
| `CullUnnecessaryVertexes()` | Remove redundant vertices |
| `DetermineOuterBoundary(ngon)` | Find outer boundary |
| `IsEdgeWelded(edge)` | Check edge welding |

### Mesh Outline / Silhouette

| Class | Description |
|-------|-------------|
| `CMeshOutline` | Mesh outline from plane |
| `CNewMeshOutline` | Mesh outline from viewport (multiple meshes) |
| `CMeshBoundary` | Mesh boundary loops |
| `CMeshRegion` | Connected mesh region |
| `GetMeshOutline()` | Free function |

### Hidden Line Removal

| Function | Description |
|----------|-------------|
| `TL_HLR_Begin` | Begin hidden line calculation |
| `TL_HLR_Triangle` | Add triangle |
| `TL_HLR_Edge` | Add edge |
| `TL_HLR_End` | Finalize and get result |

---

## F. Trimmed Surfaces & BReps

### TL_Brep Class

**Creation:**

| Method | Description |
|--------|-------------|
| `CreateSurfaceOfRevolution()` | Brep from revolution |
| `SplitFace(face, curves, tol)` | Split face with curves (4 overloads) |
| `SplitFaceAtTangents(face)` | Split at tangent discontinuities |
| `SplitFacesAtTangents()` | Split all faces |
| `SplitKinkyFace(face, angle)` | Split at kink angle |
| `SplitSingularTrim(trim, tol)` | Split singular trims |
| `MergeFaces(f1, f2)` | Merge adjacent faces |

**Query:**

| Method | Description |
|--------|-------------|
| `GetClosestPoint(face, pt, ...)` | Closest point on face |
| `GetClosestPoint(pt, ...)` | Closest point on entire brep |
| `IsPointIn(pt, tol, strict)` | Point containment test |
| `IsPointInLoop(loop, u, v, tol)` | Point in trimming loop |
| `IsPointOnFace(face, u, v, tol)` | Point on face test |
| `SolidOrientation()` | Determine solid orientation |
| `IsMorphable()` | Can be morphed |
| `GetIsoCurves(face, dir, t, out)` | Extract isocurves |
| `GetIsoIntervals(face, ...)` | Iso intervals within trims |
| `GetMontoneTrimSegments(face)` | Monotone trim segments |

**Modification:**

| Method | Description |
|--------|-------------|
| `Morph(morph)` | Apply space morph |
| `SimplifyEdge(edge, tol)` | Simplify edge curve |
| `RemoveSlits()` | Remove slit faces |
| `TransferLoops(src, dst, loop)` | Transfer trimming loops |
| `SetEdgeTolerance(edge)` | Recompute edge tolerance |
| `SetTrimBoundingBoxes()` | Recompute trim bboxes |
| `SetTrimTolerance(trim)` | Recompute trim tolerance |
| `PullbackCurve(crv, ...)` | Pull curve onto brep faces |
| `AddBoundarySegment(...)` | Add trim boundary |

### Boolean Operations

#### TL_BrepBoolean Class

| Method | Description |
|--------|-------------|
| `Merge(brep_a, brep_b, tol)` | Merge two breps |
| `ClassifyMerge()` | Classify merge regions |
| `Union()` | Boolean union → TL_Brep* |
| `Intersection()` | Boolean intersection → TL_Brep* |
| `B0MinusB1()` | Boolean difference A-B → TL_Brep* |
| `B1MinusB0()` | Boolean difference B-A → TL_Brep* |
| `Split(out)` | Boolean split |
| `UnionInPlace()` | In-place union |
| `MarkOverlapFaces()` | Mark overlapping faces |
| `CheckMergeForBoolean()` | Validate merge for boolean |

#### Free Functions

| Function | Description |
|----------|-------------|
| `TL_BooleanUnion(breps, ...)` | Union of multiple breps |
| `TL_BooleanDifference(breps, ...)` | Difference |
| `TL_BooleanIntersection(breps, ...)` | Intersection |
| `TL_BrepSplit(a, b, tol, ...)` | Split brep |
| `TL_BrepUnion(a, b, tol)` | Simple union |
| `TL_ImprintBreps(a, b, tol, ...)` | Imprint breps |
| `TL_MergeBreps(a, b, tol)` | Merge breps |
| `TL_JoinBreps(breps, tol)` | Join breps at matching edges |
| `TL_MakePlanarBreps(...)` | Create planar breps from curves |

### Intersection

#### TL_BrepIntersector Class

| Method | Description |
|--------|-------------|
| `Intersect()` | Full brep-brep intersection |
| `Intersect(faces_a, faces_b)` | Selective face intersection |
| `GatherEdgeXData()` | Gather edge intersection data |
| `GetEdgeOverlapIntervals()` | Edge overlap detection |
| `ProcessEdgeOverlaps()` | Handle edge overlaps |
| `MatchEndsAcrossFaces()` | Match intersection ends |
| `MarkDanglingIntersections(tol)` | Mark dangling intersections |
| `RemoveDanglingIntersections()` | Remove dangling |
| `ThinXDs()` | Thin intersection data |
| `TrimTripEnds()` | Trim trip endpoints |

#### TL_FaceIntersector Class

| Method | Description |
|--------|-------------|
| `Intersect(brep_a, face_a, brep_b, face_b, tol)` | Face-face intersection |
| `GetCurvePairs(flag, ...)` | Get intersection curve pairs (3D + 2D) |
| `GetSplitCurveTrips(out)` | Get splitting curves |
| `GetOverlapTrips(...)` | Get overlap regions |
| `ProcessCurveTrips(...)` | Process intersection curves |
| `AreCurveIntersections()` | Check for curve intersections |
| `MergeFSIntervals(...)` | Merge face-surface intervals |

#### Free Functions

| Function | Description |
|----------|-------------|
| `TL_IntersectBreps(a, b, tol, ...)` | Full brep intersection |
| `TL_IntersectFaces(brep, f1, f2, ...)` | Face-face intersection |
| `TL_IntersectFaceSurfaces(...)` | Face-surface intersection |
| `TL_CurveBrepIntersect(crv, brep, tol, ...)` | Curve-brep intersection |
| `TL_CurveFaceIntersect(crv, face, tol, ...)` | Curve-face intersection |
| `TL_PullCurveToFace(brep, face, crv, ...)` | Pull curve to face |

### SSX (Surface-Surface Intersection)

| Function/Class | Description |
|----------------|-------------|
| `TL_SSXEvaluator` | Surface-surface intersection evaluator |
| `TL_SSXPoint` | Intersection point (3D + 2 UV pairs) |
| `TL_SSX_EVENT` | Intersection event (point/curve/overlap) |
| `TL_IterateToSrfSrfPoint` | Newton iteration for SSX point |
| `TL_SimpleSrfSrfIntersect` | Simple SSX from seed point |

### BRep Join (TL_BrepJoin)

| Method | Description |
|--------|-------------|
| `DoJoin(tol)` | Join breps at matching edges |
| `MatchEdges(tol)` | Find matching edge pairs |
| `MatchVertices(tol)` | Find matching vertex pairs |
| `Merge()` | Merge matched breps |
| `CaptureJoinedBrep()` | Get result |
| `CheckVerticesOnEdges(tol)` | Validate vertex-edge proximity |

### BRep Region Finder (TL_BrepRegionFinder)

| Method | Description |
|--------|-------------|
| `LabelRegions()` | Label connected regions |
| `LabelConnectedComponents()` | Label components |
| `TraceBrepRegion(...)` | Trace a region boundary |
| `GetLabeledRegions(out)` | Get labeled regions as breps |
| `GetRegionOuterShells(out)` | Get outer shells |
| `OrderTrimsAroundEdge(edge)` | Order trims around edge |

### Projection / Pullback

| Function | Description |
|----------|-------------|
| `TL_Pullback` | Pull 3D curve onto surface → 2D parameter curve |
| `TL_ProjectNurbToPlane` | Project NURB onto plane |
| `TL_ProjectPointToLine` | Project point to line |
| `TL_ProjectPointToPlane` | Project point to plane |
| `TL_Classify2dNurbWRTFace` | Classify 2D curve relative to trimmed face |
| `TL_ClassifyPointWRTFace` | Classify point relative to face |

### Silhouette

| Function | Description |
|----------|-------------|
| `TL_BrepSilhouette(...)` | Brep silhouette extraction |
| `TL_SurfaceSilhouette(...)` | Surface silhouette |
| `TL_EvNurbSrfSilhouetteLine` | Silhouette line on surface |

---

## G. Edge Matching / Surface Fitting

### TL2_EdgeMatchSurface

Matches surface edges to target curves with position/tangent/curvature continuity.

| Method | Description |
|--------|-------------|
| `Create(srf)` | Initialize from surface |
| `IsValid()` | Validate setup |
| `GetInitialFitPoints(side)` | Get initial sample points |
| `MakeSurface(srf, solver)` | Build matched surface |
| `Evaluate(side, t, point)` | Evaluate target at parameter |
| `SurfaceParameter(side, t)` | Map target param to surface param |
| `TestSurface(srf)` | Test match quality |

### TL2_EdgeMatchSurfaceStrict

Stricter version with position, tangent, curvature matching per side.

| Method | Description |
|--------|-------------|
| `Create()` | Initialize |
| `MatchPositions()` | Match edge positions |
| `MatchTangents()` | Match edge tangents |
| `MatchCurvatures()` | Match edge curvatures |
| `MatchAll()` | Match everything |
| `MatchAllSmooth()` | Match with smoothing |
| `SmoothSurface()` | Apply smoothing |
| `SolveFitter()` | Solve the fitting system |
| `MeasureSurfaceError(srf)` | Measure error |

### TL2_FitSurface

General least-squares surface fitter with constraint system.

| Method | Description |
|--------|-------------|
| `AddPointEquation(...)` | Add point constraint |
| `AddDerivativeEquation(...)` | Add derivative constraint |
| `AddMinBending(...)` | Minimize bending energy |
| `AddMinCurvVar(...)` | Minimize curvature variation |
| `AddParallelism(...)` | Parallelism constraint |
| `AddSquareness(...)` | Squareness constraint |
| `AddStraightness(...)` | Straightness constraint |
| `AddPointTies(...)` | Pin points |
| `FixCV(i, j, flags)` | Fix control vertex |
| `FixRow(i)` / `FixColumn(j)` | Fix entire row/column |
| `FixSurfaceSide(iso, ...)` | Fix surface boundary |
| `FixSingularPoint(iso, pt)` | Fix singular point |
| `Solve(...)` | Solve system (SVD or normal equations) |

### TL_RefitSurface

Adaptive surface refitting with knot insertion.

| Method | Description |
|--------|-------------|
| `GetInitialSurface()` | Create initial surface |
| `GuessSurface()` | Initial guess |
| `ClassifySpans(...)` | Classify span quality |
| `AddNewKnots()` | Insert knots at bad spans |
| `Test()` | Measure deviation |
| `Refit(controls, tol, max_iter)` | Iterative refit |
| `AdjustSeams()` | Fix seam continuity |
| `MaxDeviation()` | Get max deviation |
| `GetResult()` | Get result surface |

### TlMultiMatchSrf

Multi-sided surface matching with constraint editing.

| Method | Description |
|--------|-------------|
| `AddConstraint(side, crv, continuity)` | Add edge constraint |
| `EditConstraint(side, crv, continuity)` | Modify constraint |
| `RemoveConstraint(side)` | Remove constraint |
| `Solve()` | Solve matching |
| `GetBestResult(srf)` | Get best result |
| `GetBestTolerance()` | Best achieved tolerance |
| `SetInteriorMovement(mode)` | Interior CV movement |
| `SetInteriorStiffness(val)` | Stiffness parameter |
| `SetFreeEdgeMovement(mode)` | Free edge behavior |
| `IncreaseDegree(dir, n)` | Increase degree |
| `InsertKnot(dir, t, mult)` | Insert knot |

---

## H. Curve on Surface

### TL_CrvOnSrfEvaluator

Evaluates a curve living on a surface (paired 2D+3D curves).

| Method | Description |
|--------|-------------|
| `Create(srf, crv2d, crv3d, tol)` | Initialize |
| `Ev(t, pt3d, tan3d, uv, ...)` | Evaluate at parameter (multiple overloads) |
| `Get2dParamFrom3d(t3d, t2d)` | Map 3D param to 2D |
| `Get3dParamFrom2d(t2d, t3d)` | Map 2D param to 3D |
| `Curve2d()` | Get 2D curve |
| `Curve3d()` | Get 3D curve |
| `Surface()` | Get surface |
| `IsReversed()` | Check reversal |
| `GetError()` | Get fitting error |

---

## I. Offset (Curves and Surfaces)

### TLC_CrvOffset (Curve Offset Engine)

| Method | Description |
|--------|-------------|
| `Offset(nurb, ..., segments, count)` | Offset NURB curve |
| `Offset(curves, results, flag)` | Offset ON_Curve array |
| `Extrude(nurb, ...)` | Offset and extrude |
| `EnableLoopRemoval(flag)` | Remove self-intersecting loops |
| `EnableTrimming(mode)` | Enable offset trimming |
| `Trim(segments, orientation, flag)` | Trim offset segments |
| `WindingNumberTrim(segments, orient)` | Winding number trim |
| `SetOffsetDistance(d)` | Set offset distance |
| `FillGap(segment)` | Fill gap between segments |

### TL_WindingNumberTrimmer

| Method | Description |
|--------|-------------|
| `Trim(curves)` | Trim using winding number |
| `ConnectOffsetSegments()` | Connect segments |
| `FindOffsetRegions(...)` | Find offset regions |
| `ReorderOutput(...)` | Reorder result |
| `CleanupRegions()` | Clean up regions |

### TL_RibbonOffset (Surface Ribbon Offset)

| Method | Description |
|--------|-------------|
| `Solve()` | Compute ribbon offset |
| `CaptureResult()` | Get result curve |
| `TestDeviation(max, avg)` | Test offset deviation |
| `GetCrossSections(out)` | Get cross-section curves |
| `GetSegSurfaces(out)` | Get segment surfaces |

---

## J. Utility Functions

### Linear Algebra

| Function | Description |
|----------|-------------|
| `TL_SolveLinearSystem` | General linear system solver |
| `TL_SolveTriDiagonal` | Tridiagonal solver |
| `TL_Solve2x2` / `TL_Solve3x2` / `TL_Solve3x3` | Small system solvers |
| `TL_SolveRowReduce` | Row reduction |
| `TL_SolveSVD` / `TL_NRsvd` | SVD decomposition |
| `TL_SolvePointsSVD` | SVD point fitting |
| `TL_SolveODE` | ODE solver |
| `TL_InvertMatrix` | Matrix inversion |
| `TL_AllocateMatrix` / `TL_FreeMatrix` | Matrix allocation |
| `TL_IdentityMatrix` | Identity matrix |
| `TL_MultiplyXform` | 4x4 matrix multiply |
| `TL_InvertXform` | 4x4 matrix inversion |
| `TL_TransposeXform` | 4x4 transpose |
| `TL_CreateBandDiagonalMatrix` | Banded matrix |

### Transforms

| Function | Description |
|----------|-------------|
| `TL_GetRotationXform` | Rotation matrix |
| `TL_GetScaleXform` | Scale matrix |
| `TL_GetTranslationXform` | Translation matrix |
| `TL_GetNonuniformScaleXform` | Non-uniform scale |
| `TL_GetIdentityXform` | Identity |
| `TL_GetFrameToFrameXform` | Frame-to-frame transform |
| `TL_GetFrameToFrameRotateXform` | Frame rotation |
| `TL_GetVectorRotationXform` | Vector rotation |
| `TL_GetBoxToBoxXform` | Box-to-box mapping |
| `TL_GetOrthoProjectionXform` | Orthographic projection |
| `TL_GetSkewProjectionXform` | Skew projection |
| `TL_Get3DRotation` | 3D rotation |
| `TL_Get3DFrame` | 3D coordinate frame |

### Geometry Primitives

| Function | Description |
|----------|-------------|
| `TL_CrossProduct` | Cross product |
| `TL_DotProduct` | Dot product |
| `TL_TripleProduct` | Triple product |
| `TL_UnitVector` | Normalize vector |
| `TL_Distance` | Point distance |
| `TL_DistanceSquared` | Squared distance |
| `TL_DistanceToLine` | Point-line distance |
| `TL_DistancePlaneToBoundingBox` | Plane-bbox distance |
| `TL_DistancePointToBoundingBox` | Point-bbox distance |
| `TL_Length` / `TL_LengthSquared` | Vector length |
| `TL_NormalTo3Points` | Normal from 3 points |
| `TL_IncludedAngle` | Angle between vectors |
| `TL_BoundingBoxOfPoints` | Bounding box |
| `TL_UnionBoundingBox` | Union of bounding boxes |
| `TL_GetPerpendicular` / `TL_Get3DPerpendicular` | Perpendicular vector |

### Point/Plane Operations

| Function | Description |
|----------|-------------|
| `TL_CreatePlaneFromEquation` | Plane from ax+by+cz=d |
| `TL_CreatePlaneFromPointAndFrame` | Plane from origin + frame |
| `TL_CreatePlaneFromPointAndNormal` | Plane from origin + normal |
| `TL_EvPlanePoint` | Evaluate point on plane |
| `TL_EvPlaneNormal` | Plane normal |
| `TL_EvPlaneClosestPoint` | Closest point on plane |
| `TL_EvPlane1Der` | Plane derivatives |
| `TL_FlipPlane` | Flip plane normal |
| `TL_GetPlaneEqn` | Get plane equation |
| `TL_ValidatePlane` | Validate plane |
| `TL_GetBest3dPlaneThroughPoints` | Best-fit plane |
| `TL_Are3DPointsCoplanar` | Coplanarity test |
| `TL_Are4PointsCoplanar` | 4-point coplanarity |
| `TL_ArePointsColinear` | Collinearity test |

### Polyline

| Function | Description |
|----------|-------------|
| `TL_CreatePolyline` | Allocate polyline |
| `TL_AddPolylinePoint` | Append point |
| `TL_CompressPolyline` | Remove collinear points |
| `TL_CullPolyline` | Remove points by tolerance |
| `TL_MergePolylines` | Join polylines |
| `TL_LengthPolyline` | Total length |
| `TL_PolylineParameter` | Parameter at point |
| `TL_PolylinePoint` | Point at parameter |
| `TL_PolylineTangent` | Tangent at parameter |
| `TL_IsPolylineClosed` | Closure test |
| `TL_Get2dPolygonArea` | 2D polygon area |
| `TL_XformPolyline` | Transform polyline |

### Point Lists / Grids

| Function | Description |
|----------|-------------|
| `TL_CreatePointList` / `TL_DestroyPointList` | Allocate/free |
| `TL_CreatePointGrid` / `TL_DestroyPointGrid` | Allocate/free |
| `TL_AddToPointList` | Append point |
| `TL_CollapsePointList` | Collapse to centroid |
| `TL_ReversePointList` | Reverse order |
| `TL_GetPointListBoundingBox` | Bounding box |
| `TL_GetPointListCentroid` | Centroid |
| `TL_IsPointListLinear` | Linearity test |
| `TL_IsPointListPlanar` | Planarity test |
| `TL_LinearFitToPoints` | Linear fit |
| `TL_GetQuadricFitToPoints` | Quadric surface fit |

### Line Intersection

| Function | Description |
|----------|-------------|
| `TL_Isect3dLineLine` | 3D line-line intersection |
| `TL_IsectLinePlane` | Line-plane intersection |
| `TL_3DX_LineXPlane` | Line-plane intersection variant |
| `TL_EvLineClosestPoint` | Closest point on line |
| `TL_EvChordClosestPoint` | Closest point on chord |
| `TL_GetLineEqn` | Line equation |

### Circle / Arc

| Function | Description |
|----------|-------------|
| `TL_2DCircleCenterFrom3Points` | 2D circle from 3 points |
| `TL_3DCircleCenterFrom3Points` | 3D circle from 3 points |
| `TL_2DArcMidpointFromBulge` | 2D arc midpoint from bulge factor |
| `TL_2DArcMidpointFromCenter` | 2D arc midpoint from center |
| `TL_3DArcMidpointFromBulge` | 3D arc midpoint from bulge |
| `TL_3DArcMidpointFromCenter` | 3D arc midpoint from center |
| `TL_ArcAngle` | Arc angle |
| `TL_BezierArc` | Bezier arc |
| `TL_BezierLine` | Bezier line |
| `TL_BezierParabola` | Bezier parabola |
| `TL_BezierRhoConic` | Bezier rho-conic |
| `TL_BezierShoulderConic` | Bezier shoulder conic |
| `TL_Are6PointsOnAConic` | Conic test |

### Optimization / Root Finding

| Function | Description |
|----------|-------------|
| `TL_GetFunctionRoot` | General root finder |
| `TL_BracketMinimum` | Bracket a minimum |
| `TL_EvMultiVariableMinimum` | Multi-variable minimization |
| `TL_NRdbrent` | Brent's method (Numerical Recipes) |
| `TL_QuadraticEquation` | Solve ax²+bx+c=0 |

### View / Camera

| Function | Description |
|----------|-------------|
| `TL_CreateView` / `TL_DestroyView` | View management |
| `TL_GetViewCamera` / `TL_SetViewCamera` | Camera get/set |
| `TL_GetViewXform` | View transform |
| `TL_GetCameraToWorldXform` | Camera→world |
| `TL_GetWorldToCameraXform` | World→camera |
| `TL_GetCameraToClipXform` | Camera→clip |
| `TL_ZoomExtentsView` | Zoom to fit |
| `TL_ZoomWindowView` | Zoom to window |
| `TL_PerspectiveMatch` | Perspective match |

### Conversion / Angles

| Function | Description |
|----------|-------------|
| `TL_AngleFromDegrees` | Degrees to radians |
| `TL_AngleFromGrads` | Grads to radians |
| `TL_AngleFromCHR` | CHR to radians |
| `TL_CHRFromAngle` | Radians to CHR |
| `TL_GetUnitsScale` | Unit conversion factor |
| `TL_ColorFromAutoCAD` / `TL_ColorToAutoCAD` | AutoCAD color mapping |
| `TL_ColorFromIGES` / `TL_ColorToIGES` | IGES color mapping |

---

## K. Key Differences: Our Implementation vs Rhino

| Aspect | Rhino (tl.dll) | Our `create_network` |
|--------|----------------|---------------------|
| Grid surface T(u,v) | Bicubic interpolation (`CreateCubicSurfaceThroughPointGrid`) with centripetal parameterization | Lagrange basis functions |
| Loft surfaces | Cubic B-spline interpolation through refit sections (`TL_CubicNurbInterpolate`) | Lagrange interpolation of CVs |
| Curve refitting | `TL_NurbFitToNurb` — refit each curve to grid knot structure | No refitting |
| Parameterization | Centripetal (sqrt of chord length) | Chord-length |
| Knot compatibility | `TL_MakeNurbSrfsKnotCompatible` — proper knot insertion | Manual knot merging |
| Singular tips | `SmoothTip` — project CVs onto tangent plane for G1 | Not implemented |
| Periodic handling | `TL_MakeNurbSrfPeriodic` + MorphTo/MorphFrom | Not implemented |
| Rational handling | `TL_MakeNurbSrfsRatCompatible` + endpoint weight matching | Reparametrize to non-rational |
| Coons patch | Full TL_CoonsPatchNurbSrf with collapsed side handling | Piecewise-linear fallback |
| Surface fitting | TL2_FitSurface LS fitter with bending/curvature energy | Simple LS refit |

---

## Export Counts

| Category | C Exports | C++ Methods | Total |
|----------|-----------|-------------|-------|
| NURBS Curves | ~120 | ~50 | ~170 |
| NURBS Surfaces | ~80 | ~40 | ~120 |
| Network/Coons/Gordon | 3 | ~25 | ~28 |
| Sweep | 8 | ~20 | ~28 |
| Meshing | 12 | ~150 | ~162 |
| BRep Operations | 5 | ~200 | ~205 |
| Boolean (Brep+Mesh) | 5 | ~55 | ~60 |
| Intersection | 3 | ~50 | ~53 |
| Edge Match/Fit | 0 | ~65 | ~65 |
| Offset | 12 | ~35 | ~47 |
| Bezier | ~40 | 0 | ~40 |
| Utilities | ~150 | ~20 | ~170 |
| **Total** | **~711** | **~7100** | **~7800** |
