# Rhino DLL Decompilation Progress

**Last Updated:** 2026-01-10

## Summary

Successfully decompiled **441 functions** from Rhino 8's TL.DLL:
- 196 TL_* functions (curves, surfaces, fitting)
- 245 Brep/Boolean-related functions

## DLL Analysis Complete

### TL.DLL (7.2MB) - Core Math Library
Location: `C:\Program Files\Rhino 8\System\tl.dll`

**Build Path Discovered:** `D:\BuildAgent\work\dujour\src4\tl\`
- NURB_FIT.cpp - NURBS interpolation algorithms
- MATH.cpp - Math utilities
- BOOLEAN.cpp - Boolean operations
- OFFSET.cpp - Offset algorithms

**Exported Function Count:** 709 TL_* functions

## Decompiled Code Repository

**Location:** `C:\tmp\rhino_decompiled\`

```
tl_functions/         # 196 files - curve/surface functions
  TL_CubicNurbThroughPoints.c    [HIGH] Core interpolation
  TL_CubicNurbInterpolate.c      [HIGH] Tridiagonal solver
  TL_NurbInterpolate.c           [HIGH] General interpolation
  TL_BlendNurbs.c                [HIGH] Curve blending
  TL_GrevilleAbcissa.c           [HIGH] Greville points
  TL_OffsetNurb.c                Curve offset
  TL_LoftNurbSrf.c               Surface lofting
  TL_NurbSrfInterpolate.c        Surface interpolation
  ... (196 files total)

brep_functions/       # 245 files - boolean/intersection functions
  TL_BrepBoolean.c               Boolean operations class
  TL_BrepIntersector.c           Surface-surface intersection
  TL_BrepImprint.c               Curve imprint on faces
  TL_BrepJoin.c                  Brep joining
  TL_MeshBoolean.c               Mesh boolean operations
  TL_IntersectFaces.c            Face intersection
  ... (245 files total)
```

## Key Algorithm Discoveries

### TL_CubicNurbInterpolate Parameters
```c
TL_CubicNurbInterpolate(
    int dim,           // Dimension (2 or 3)
    int cv_count,      // Control vertex count (>= 4)
    double* knot,      // Knot vector
    int start_cond,    // Start end condition (0-3)
    int end_cond,      // End end condition (0-3)
    double* cv         // Control vertices (output)
)
```

**End Condition Types:**
- `0` = Free (natural, auto-compute from geometry)
- `1` = First derivative specified
- `2` = Second derivative specified
- `3` = Natural (scaled tangent from point spacing)

**Key Internal Functions:**
- `TL_EvNurbBasis` - Evaluate NURBS basis functions
- `TL_EvNurbBasisDer` - Evaluate basis function derivatives
- `TL_SolveTriDiagonal` - Core tridiagonal matrix solver

### TL_CubicNurbThroughPoints Parameters
```c
TL_CubicNurbThroughPoints(
    uint dim,              // Dimension
    int point_count,       // Number of points
    double* points,        // Input points
    uint closed_type,      // 0=open, 1-2=closed
    double* start_tan,     // NULL for auto
    double* end_tan,       // NULL for auto
    int knot_style,        // 0=uniform, 1=chord, 2=centripetal
    uint* output           // Output NURB handle
)
```

**Knot Styles:**
- `0` = Uniform spacing
- `1` = Chord-length parameterization
- `2` = Centripetal (sqrt of chord-length)

### TL_BrepIntersector
Constructor takes:
- Two TL_Brep references (the breps to intersect)
- Tolerance (double)
- Boolean flag

Creates:
- Array of TL_FaceIntersector for each face pair
- Edge data arrays for both breps
- Intersection curve storage

## Ghidra Project

**Location:** `C:\tmp\ghidra_rhino\RhinoTL`
- TL.DLL imported and analyzed
- Full function list available
- Can re-extract with custom scripts

## Extraction Scripts

**Location:** `C:\rust\session\.claude\ghidra_scripts\`
- `ExtractTLFunctions.java` - Extract curve/surface functions
- `ExtractBrepFunctions.java` - Extract boolean/intersection functions

## Next Steps

### Phase 1: Curve Implementation (PRIORITY)
- [ ] Analyze TL_CubicNurbInterpolate in detail
- [ ] Port TL_SolveTriDiagonal to session_cpp
- [ ] Match Rhino's exact parameterization
- [ ] Test against Rhino output

### Phase 2: Surface Implementation
- [ ] Analyze TL_LoftNurbSrf
- [ ] Port TL_NurbSrfInterpolate
- [ ] Implement surface offset

### Phase 3: Boolean Operations
- [ ] Analyze TL_BrepBoolean workflow
- [ ] Understand face intersection algorithm
- [ ] Port boolean union/difference/intersection

## Session Implementation Status

### Implemented
- Point, Vector, Line, Plane (basic geometry)
- NurbsCurve.create_interpolated (partial - different parameterization)
- Tolerance handling

### Needs Matching
- NURBS curve interpolation (exact Rhino match)
- End condition handling
- Knot style options

### Not Yet Implemented
- Brep class
- Boolean operations
- Surface interpolation
- Curve/surface intersection

## Tools Used

- **Ghidra 12.0** - Headless analysis and decompilation
- **Java 21** - Required for Ghidra
- **pefile (Python)** - DLL export listing

## References

- Rhino C++ API Docs: https://mcneel.github.io/rhino-cpp-api-docs/api/cpp/
- OpenNURBS source (partial): https://github.com/mcneel/opennurbs
