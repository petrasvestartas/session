# Rhino DLL Decompilation Skill

Reverse engineering workflow for analyzing Rhino 8's compiled C++ components.

## Prerequisites

```bash
# Install Ghidra
winget install ghidra  # or download from https://github.com/NationalSecurityAgency/ghidra/releases

# Install Java JDK 21+
winget install Microsoft.OpenJDK.21

# Set JAVA_HOME in Ghidra config
sed -i 's|^JAVA_HOME_OVERRIDE=$|JAVA_HOME_OVERRIDE=C:/Program Files/Microsoft/jdk-21.0.9.10-hotspot|' \
    /c/tools/ghidra_12.0_PUBLIC/support/launch.properties
```

## Target DLLs

| DLL | Size | Contents |
|-----|------|----------|
| `RhinoCore.dll` | 38MB | High-level Rhino commands, calls TL.DLL |
| `TL.DLL` | 7MB | Core math algorithms (TL_* functions) |
| `rhcommon_c.dll` | 3MB | Native C wrapper for RhinoCommon |
| `opennurbs.dll` | - | Open source NURBS library |

Location: `C:\Program Files\Rhino 8\System\`

## Ghidra Headless Analysis

### 1. Create Analysis Project

```bash
mkdir -p /tmp/ghidra_project
JAVA_HOME="/c/Program Files/Microsoft/jdk-21.0.9.10-hotspot" \
  /c/tools/ghidra_12.0_PUBLIC/support/analyzeHeadless.bat \
  /tmp/ghidra_project ProjectName \
  -import "/c/Program Files/Rhino 8/System/tl.dll"
```

### 2. Extraction Script

Save to `ghidra_scripts/ExtractFunctions.java`:

```java
//@category Analysis
import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.*;
import ghidra.program.model.listing.*;
import java.io.*;

public class ExtractFunctions extends GhidraScript {
    @Override
    protected void run() throws Exception {
        DecompInterface decomp = new DecompInterface();
        decomp.openProgram(currentProgram);
        FunctionManager fm = currentProgram.getFunctionManager();

        PrintWriter out = new PrintWriter(new FileWriter("C:/tmp/output.c"));
        String[] targets = {"TL_Nurb", "TL_Cubic", "Interp", "Greville"};

        for (Function func : fm.getFunctions(true)) {
            String name = func.getName();
            for (String t : targets) {
                if (name.contains(t)) {
                    out.println("// Function: " + name);
                    out.println("// Address: " + func.getEntryPoint());
                    DecompileResults results = decomp.decompileFunction(func, 120, monitor);
                    if (results.decompileCompleted()) {
                        out.println(results.getDecompiledFunction().getC());
                    }
                    break;
                }
            }
        }
        out.close();
        decomp.dispose();
    }
}
```

### 3. Run Extraction

```bash
JAVA_HOME="/c/Program Files/Microsoft/jdk-21.0.9.10-hotspot" \
  /c/tools/ghidra_12.0_PUBLIC/support/analyzeHeadless.bat \
  /tmp/ghidra_project ProjectName \
  -process tl.dll \
  -postScript ExtractFunctions.java \
  -noanalysis
```

## Key Rhino Functions

### NURBS Curve Interpolation

| Function | Location | Purpose |
|----------|----------|---------|
| `RHC_RhinoInterpCurve` | rhcommon_c.dll | P/Invoke entry point |
| `RhinoInterpCurve` | RhinoCore.dll | Dispatcher |
| `TL_CubicNurbThroughPoints` | TL.DLL | Cubic interpolation |
| `TL_NurbThroughPoints` | TL.DLL | General degree |
| `TL_CubicNurbInterpolate` | TL.DLL | Core tridiagonal solver |
| `TL_GrevilleAbcissa` | TL.DLL | Greville points |

### Algorithm Parameters

```c
TL_CubicNurbThroughPoints(
    uint dim,           // 3 for 3D
    int point_count,
    double* points,
    uint closed_type,   // 0=open, 1-2=closed
    double* start_tan,  // NULL for auto
    double* end_tan,
    int knot_style,     // 1=chord, 2=centripetal, 0=uniform
    uint* output)
```

### End Conditions

- `0` = Free boundary
- `1` = First derivative
- `2` = Second derivative (curvature)
- `3` = Natural (auto from point spacing)

## .NET Decompilation

For RhinoCommon.dll (.NET assembly):

```bash
# Install ILSpy
dotnet tool install -g ilspycmd

# Decompile
ilspycmd "/c/Program Files/Rhino 8/System/RhinoCommon.dll" \
  -p -o /tmp/rhino_decompiled
```

## Output Files

After analysis:
- `/tmp/rhinocore_interp.c` - Decompiled TL.DLL functions
- `/tmp/rhinocore_calls.c` - Call graph from RhinoCore
- `/tmp/rhino_decompiled/` - Decompiled .NET source

## Source Paths Revealed

From decompiled error messages:
- `D:\BuildAgent\work\dujour\src4\tl\NURB_FIT.cpp`
- `D:\BuildAgent\work\dujour\src4\tl\MATH.cpp`
