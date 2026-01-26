# Rhino Reverse Engineering Plan

## Problem
Raw Ghidra decompilation is unreadable garbage with meaningless variable names.

## Solution: Multi-Source Cross-Reference Approach

### Phase 1: Extract Symbol Names from RhinoCommon.dll (.NET)

The .NET layer has **real function names** that map to native calls.

```bash
# Decompile RhinoCommon.dll with ILSpy
ilspycmd "C:\Program Files\Rhino 8\System\RhinoCommon.dll" -p -o C:\tmp\rhinocommon_src
```

**Key files to examine:**
- `Rhino.Geometry.NurbsCurve.cs` - CreateFromInterpolatedPoints
- `Rhino.Geometry.Curve.cs` - Offset, Blend
- `Rhino.Geometry.Brep.cs` - CreateBooleanUnion, CreateBooleanDifference

### Phase 2: Map Native Calls (rhcommon_c.dll)

rhcommon_c.dll is the P/Invoke bridge with **readable export names**:
- `RHC_RhinoInterpCurve` → calls `TL_CubicNurbThroughPoints`
- `RHC_RhinoCurveOffset` → calls `TL_OffsetNurb`
- `RHC_RhinoBooleanUnion` → calls `TL_BrepUnion`

### Phase 3: Use OpenNURBS Source (Open Source!)

OpenNURBS is **open source**: https://github.com/mcneel/opennurbs

Key files with real implementations:
- `opennurbs_nurbscurve.cpp` - ON_NurbsCurve methods
- `opennurbs_bezier.cpp` - Bezier evaluation
- `opennurbs_knot.cpp` - Knot vector utilities

**Many TL_ functions just wrap ON_ functions!**

### Phase 4: Create Clean Pseudocode

Instead of raw decompilation, write clean pseudocode with:
- Meaningful variable names
- Algorithm documentation
- Reference to original Rhino API

## Immediate Actions

### Action 1: Decompile RhinoCommon.dll
```bash
dotnet tool install -g ilspycmd
ilspycmd "C:\Program Files\Rhino 8\System\RhinoCommon.dll" -p -o C:\tmp\rhinocommon_src
```

### Action 2: Get rhcommon_c.dll exports
```python
import pefile
pe = pefile.PE(r"C:\Program Files\Rhino 8\System\rhcommon_c.dll")
for exp in pe.DIRECTORY_ENTRY_EXPORT.symbols:
    if exp.name and b'Interp' in exp.name:
        print(exp.name.decode())
```

### Action 3: Clone OpenNURBS
```bash
git clone https://github.com/mcneel/opennurbs.git C:\tmp\opennurbs
```

### Action 4: Focus on ONE algorithm

Pick `create_interpolated` and trace it completely:

1. **RhinoCommon** (C#): `NurbsCurve.CreateFromInterpolatedPoints()`
2. **rhcommon_c** (C): `RHC_RhinoInterpCurve()`
3. **TL.DLL** (C++): `TL_CubicNurbThroughPoints()`
4. **OpenNURBS** (C++): `ON_NurbsCurve::GrevilleInterpolate()`

## Priority Functions to Reverse

| Priority | Function | Why |
|----------|----------|-----|
| 1 | Curve interpolation | Core algorithm, session needs this |
| 2 | Curve offset | Commonly needed |
| 3 | Surface loft | Surface creation |
| 4 | Boolean union | Complex but high value |

## Output Format

For each function, create:

```
algorithms/
  curve_interpolation/
    README.md           # Algorithm explanation
    pseudocode.md       # Clean pseudocode
    rhino_mapping.md    # Maps Rhino API → TL_ function
    implementation.cpp  # Clean C++ implementation
    test_vectors.json   # Test cases from Rhino
```

## Timeline

1. **Today**: Decompile RhinoCommon, clone OpenNURBS
2. **Next**: Map curve interpolation completely
3. **Then**: Write clean pseudocode
4. **Finally**: Implement in session_cpp
