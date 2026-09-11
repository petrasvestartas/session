# Rhino DLL Decompilation Skill (Advanced)

Complex task management and memory system for reverse engineering Rhino 8 geometry algorithms.

## Target DLLs

| DLL | Size | Location | Contents |
|-----|------|----------|----------|
| `tl.dll` | 7MB | System/ | Core math algorithms (709 TL_* functions) |
| `RhinoCore.dll` | 38MB | System/ | High-level commands, CRhino* classes |
| `rhcommon_c.dll` | 3MB | System/ | P/Invoke entry points (RHC_*) |
| `opennurbs.dll` | 12MB | System/ | Open source NURBS (ON_* classes) |

## Priority Functions

### Curves (TL.DLL)
- [ ] `TL_CubicNurbThroughPoints` - Interpolated curve
- [ ] `TL_CubicNurbInterpolate` - Core tridiagonal solver
- [ ] `TL_NurbThroughPoints` - General degree interpolation
- [ ] `TL_NurbGrevilleInterpolate` - Greville interpolation
- [ ] `TL_BlendNurbs` - Curve blending
- [ ] `TL_MergeNurbs` - Curve joining
- [ ] `TL_OffsetNurb` - Curve offset
- [ ] `TL_NurbFitToNurb` - Curve refitting
- [ ] `TL_CubicBezierFit` - Bezier fitting

### Surfaces (TL.DLL)
- [ ] `TL_LoftNurbSrf` - Loft surface
- [ ] `TL_NurbSrfInterpolate` - Surface interpolation
- [ ] `TL_OffsetNurbSrf` - Surface offset
- [ ] `TL_RevolveNurb` - Revolution surface
- [ ] `TL_SwingNurb` - Swing surface
- [ ] `TL_RuleNurbSrf` - Ruled surface
- [ ] `TL_CoonsPatchNurbSrf` - Coons patch

### Boolean Operations (TL.DLL classes)
- [ ] `TL_BrepBoolean` - Union/Difference/Intersection
- [ ] `TL_BrepImprint` - Imprint curves on faces
- [ ] `TL_BrepIntersector` - Surface-surface intersection
- [ ] `TL_BrepJoin` - Join breps
- [ ] `TL_MeshBoolean` - Mesh boolean operations

### Utility (TL.DLL)
- [ ] `TL_GrevilleAbcissa` - Greville point calculation
- [ ] `TL_IntersectNurbNurb` - Curve-curve intersection
- [ ] `TL_IntersectNurbPlane` - Curve-plane intersection
- [ ] `TL_SolveTriDiagonal` - Tridiagonal solver

## Ghidra Setup

### Prerequisites
```bash
# Ghidra 12.0+
winget install ghidra

# Java JDK 21+
winget install Microsoft.OpenJDK.21

# Configure Ghidra
GHIDRA_HOME="/c/tools/ghidra_12.0_PUBLIC"
JAVA_HOME="/c/Program Files/Microsoft/jdk-21.0.9.10-hotspot"
```

### Create Project
```bash
mkdir -p /tmp/ghidra_rhino
JAVA_HOME="/c/Program Files/Microsoft/jdk-21.0.9.10-hotspot" \
  "$GHIDRA_HOME/support/analyzeHeadless.bat" \
  /tmp/ghidra_rhino RhinoProject \
  -import "/c/Program Files/Rhino 8/System/tl.dll" \
  -import "/c/Program Files/Rhino 8/System/RhinoCore.dll"
```

## Extraction Scripts

### ghidra_scripts/ExtractTLFunctions.java
```java
//@category Analysis
import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.*;
import ghidra.program.model.listing.*;
import java.io.*;

public class ExtractTLFunctions extends GhidraScript {
    @Override
    protected void run() throws Exception {
        DecompInterface decomp = new DecompInterface();
        decomp.openProgram(currentProgram);
        decomp.setOptions(new DecompileOptions());

        FunctionManager fm = currentProgram.getFunctionManager();
        String outDir = "C:/tmp/rhino_decompiled/";
        new File(outDir).mkdirs();

        String[] targets = {
            "TL_CubicNurb", "TL_Nurb", "TL_Brep", "TL_Blend",
            "TL_Offset", "TL_Loft", "TL_Intersect", "TL_Greville"
        };

        PrintWriter index = new PrintWriter(new FileWriter(outDir + "index.txt"));

        for (Function func : fm.getFunctions(true)) {
            String name = func.getName();
            for (String t : targets) {
                if (name.contains(t)) {
                    String filename = name.replaceAll("[^a-zA-Z0-9_]", "_") + ".c";
                    PrintWriter out = new PrintWriter(new FileWriter(outDir + filename));

                    out.println("// Function: " + name);
                    out.println("// Address: " + func.getEntryPoint());
                    out.println("// Signature: " + func.getSignature());
                    out.println();

                    DecompileResults results = decomp.decompileFunction(func, 180, monitor);
                    if (results.decompileCompleted()) {
                        out.println(results.getDecompiledFunction().getC());
                        index.println(name + " -> " + filename);
                    } else {
                        out.println("// Decompilation failed");
                    }
                    out.close();
                    break;
                }
            }
        }
        index.close();
        decomp.dispose();
    }
}
```

### Run Extraction
```bash
JAVA_HOME="/c/Program Files/Microsoft/jdk-21.0.9.10-hotspot" \
  "$GHIDRA_HOME/support/analyzeHeadless.bat" \
  /tmp/ghidra_rhino RhinoProject \
  -process tl.dll \
  -postScript ExtractTLFunctions.java \
  -scriptPath "/c/rust/session/.claude/ghidra_scripts" \
  -noanalysis
```

## Memory: Decompiled Functions

### Completed
<!-- Update this section as functions are decompiled -->

| Function | Output File | Status | Notes |
|----------|-------------|--------|-------|
| `TL_CubicNurbThroughPoints` | pending | - | Core interpolation |

### In Progress
<!-- Current focus -->

### Discovered Source Paths
From decompiled debug info:
- `D:\BuildAgent\work\dujour\src4\tl\NURB_FIT.cpp`
- `D:\BuildAgent\work\dujour\src4\tl\MATH.cpp`
- `D:\BuildAgent\work\dujour\src4\tl\BOOLEAN.cpp`

## Algorithm Notes

### TL_CubicNurbThroughPoints Parameters
```c
TL_CubicNurbThroughPoints(
    uint dim,           // 3 for 3D points
    int point_count,    // Number of input points
    double* points,     // Input points array
    uint closed_type,   // 0=open, 1-2=closed variants
    double* start_tan,  // NULL for auto-computed
    double* end_tan,    // NULL for auto-computed
    int knot_style,     // 1=chord, 2=centripetal, 0=uniform
    uint* output        // Output NURB handle
)
```

### End Condition Types
- `0` = Free boundary (auto-compute from geometry)
- `1` = First derivative (tangent specified)
- `2` = Second derivative (curvature specified)
- `3` = Natural spline (second derivative = 0)

### Knot Styles
- `0` = Uniform spacing
- `1` = Chord-length parameterization
- `2` = Centripetal (sqrt chord-length)
- `3` = Arc-length (expensive)

## .NET Decompilation (RhinoCommon)

```bash
# Install ILSpy command line
dotnet tool install -g ilspycmd

# Decompile RhinoCommon.dll
ilspycmd "/c/Program Files/Rhino 8/System/RhinoCommon.dll" \
  -p -o /tmp/rhinocommon_src

# Key files to examine
# - Rhino.Geometry.NurbsCurve
# - Rhino.Geometry.Brep
# - Rhino.Geometry.Intersect
```

## Output Locations

```
/tmp/ghidra_rhino/          # Ghidra project
/tmp/rhino_decompiled/      # Extracted C code
  tl_functions/             # TL.DLL functions
  rhinocore_functions/      # RhinoCore.dll functions
/tmp/rhinocommon_src/       # ILSpy output (.NET)
```

## Workflow

1. **Identify target function** - Use pefile to list exports
2. **Run Ghidra analysis** - Import DLL, let it analyze
3. **Extract decompiled C** - Run extraction script
4. **Clean up output** - Add type annotations, fix names
5. **Document algorithm** - Write notes in memory section
6. **Implement in session** - Port to session_cpp/session_rust

## Related Files
- `SKILLS_RHINO_DECOMPILE.md` - Basic decompilation reference
- `.claude/nurbscurve/` - NURBS curve investigation notes
