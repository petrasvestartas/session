//@category Analysis
//@description Extract Brep/Boolean functions from Rhino TL.DLL
import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.*;
import ghidra.program.model.listing.*;
import java.io.*;

public class ExtractBrepFunctions extends GhidraScript {
    @Override
    protected void run() throws Exception {
        DecompInterface decomp = new DecompInterface();
        decomp.openProgram(currentProgram);

        DecompileOptions options = new DecompileOptions();
        decomp.setOptions(options);

        FunctionManager fm = currentProgram.getFunctionManager();
        String outDir = "C:/tmp/rhino_decompiled/brep_functions/";
        new File(outDir).mkdirs();

        String[] patterns = {
            "TL_Brep", "Boolean", "Intersect", "Imprint", "Join",
            "SSX", "CSX", "CrvSrf", "SrfSrf"
        };

        PrintWriter index = new PrintWriter(new FileWriter(outDir + "index.txt"));
        int count = 0;

        for (Function func : fm.getFunctions(true)) {
            String name = func.getName();

            boolean matches = false;
            for (String p : patterns) {
                if (name.contains(p)) {
                    matches = true;
                    break;
                }
            }

            if (!matches) continue;

            String filename = name.replaceAll("[^a-zA-Z0-9_]", "_") + ".c";
            PrintWriter out = new PrintWriter(new FileWriter(outDir + filename));

            out.println("/*");
            out.println(" * Function: " + name);
            out.println(" * Address: " + func.getEntryPoint());
            out.println(" * Signature: " + func.getSignature());
            out.println(" */");
            out.println();

            DecompileResults results = decomp.decompileFunction(func, 300, monitor);
            if (results.decompileCompleted()) {
                out.println(results.getDecompiledFunction().getC());
                index.println(name + " -> " + filename);
                count++;
            } else {
                out.println("// Decompilation failed: " + results.getErrorMessage());
            }
            out.close();

            if (monitor.isCancelled()) break;
        }

        index.println();
        index.println("Total Brep functions extracted: " + count);
        index.close();
        decomp.dispose();

        println("Extracted " + count + " Brep functions to " + outDir);
    }
}
