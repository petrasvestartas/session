//@category Analysis
//@description Extract TL_* functions from Rhino TL.DLL
import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.*;
import ghidra.program.model.listing.*;
import java.io.*;
import java.util.*;

public class ExtractTLFunctions extends GhidraScript {
    @Override
    protected void run() throws Exception {
        DecompInterface decomp = new DecompInterface();
        decomp.openProgram(currentProgram);

        DecompileOptions options = new DecompileOptions();
        decomp.setOptions(options);

        FunctionManager fm = currentProgram.getFunctionManager();
        String outDir = "C:/tmp/rhino_decompiled/tl_functions/";
        new File(outDir).mkdirs();

        // Target function prefixes
        String[] highPriority = {
            "TL_CubicNurbThroughPoints",
            "TL_CubicNurbInterpolate",
            "TL_NurbThroughPoints",
            "TL_NurbInterpolate",
            "TL_GrevilleAbcissa",
            "TL_BlendNurbs",
            "TL_MergeNurbs",
            "TL_OffsetNurb",
            "TL_LoftNurbSrf",
            "TL_NurbSrfInterpolate",
            "TL_SolveTriDiagonal"
        };

        String[] patterns = {
            "TL_Nurb", "TL_Brep", "TL_Blend", "TL_Offset",
            "TL_Loft", "TL_Intersect", "TL_Greville", "TL_Cubic",
            "TL_Bezier", "TL_Solve", "TL_Fit", "TL_Mesh"
        };

        Set<String> highPrioritySet = new HashSet<>(Arrays.asList(highPriority));
        PrintWriter index = new PrintWriter(new FileWriter(outDir + "index.txt"));
        int count = 0;

        for (Function func : fm.getFunctions(true)) {
            String name = func.getName();

            // Check if matches any pattern
            boolean matches = false;
            for (String p : patterns) {
                if (name.contains(p)) {
                    matches = true;
                    break;
                }
            }

            if (!matches) continue;

            // Determine priority
            boolean isHighPriority = false;
            for (String hp : highPriority) {
                if (name.contains(hp)) {
                    isHighPriority = true;
                    break;
                }
            }

            String filename = name.replaceAll("[^a-zA-Z0-9_]", "_") + ".c";
            PrintWriter out = new PrintWriter(new FileWriter(outDir + filename));

            out.println("/*");
            out.println(" * Function: " + name);
            out.println(" * Address: " + func.getEntryPoint());
            out.println(" * Signature: " + func.getSignature());
            out.println(" * Priority: " + (isHighPriority ? "HIGH" : "normal"));
            out.println(" */");
            out.println();

            DecompileResults results = decomp.decompileFunction(func, 300, monitor);
            if (results.decompileCompleted()) {
                ClangTokenGroup ctg = results.getCCodeMarkup();
                if (ctg != null) {
                    out.println(results.getDecompiledFunction().getC());
                }
                index.println((isHighPriority ? "[HIGH] " : "") + name + " -> " + filename);
                count++;
            } else {
                out.println("// Decompilation failed: " + results.getErrorMessage());
            }
            out.close();

            if (monitor.isCancelled()) break;
        }

        index.println();
        index.println("Total functions extracted: " + count);
        index.close();
        decomp.dispose();

        println("Extracted " + count + " functions to " + outDir);
    }
}
