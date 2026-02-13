"""IDAPython script to decompile RHC_RhinoNetworkSurface from rhcommon_c.dll"""
import ida_auto
import ida_funcs
import ida_hexrays
import ida_name
import idautils
import idc
import os

OUTPUT_DIR = r"C:\pc\3_code\code_rust\session\ida_output"
os.makedirs(OUTPUT_DIR, exist_ok=True)

# Wait for auto-analysis to complete
ida_auto.auto_wait()

targets = [
    "RHC_RhinoNetworkSurface",
    "RHC_RhinoNetworkSurface2",
]

# Also search for any TL_ functions related to network/loft/gordon
all_funcs = {}
for ea in idautils.Functions():
    name = ida_funcs.get_func_name(ea)
    if name:
        all_funcs[name] = ea

# Find target functions and related ones
to_decompile = {}
for name, ea in all_funcs.items():
    for t in targets:
        if t in name:
            to_decompile[name] = ea
    # Also grab anything with Network, Loft, Gordon, NurbSrf in the name
    for kw in ["Network", "network", "Gordon", "gordon", "LoftNurb", "NurbSrf",
               "TL_Loft", "TL_Nurb", "TL_Network", "TL_Gordon",
               "TL_Blend", "TL_Fit", "CoonsPatch", "Coons"]:
        if kw in name:
            to_decompile[name] = ea

# Write index
with open(os.path.join(OUTPUT_DIR, "index.txt"), "w") as f:
    f.write(f"Total functions in DLL: {len(all_funcs)}\n")
    f.write(f"Functions to decompile: {len(to_decompile)}\n\n")
    for name, ea in sorted(to_decompile.items()):
        f.write(f"  0x{ea:X}  {name}\n")

# Decompile each function
for name, ea in sorted(to_decompile.items()):
    outfile = os.path.join(OUTPUT_DIR, f"{name}.c")
    try:
        cfunc = ida_hexrays.decompile(ea)
        if cfunc:
            code = str(cfunc)
            with open(outfile, "w") as f:
                f.write(f"// Function: {name}\n")
                f.write(f"// Address: 0x{ea:X}\n\n")
                f.write(code)
            print(f"OK: {name} -> {outfile}")
        else:
            print(f"FAIL: {name} - decompile returned None")
    except Exception as e:
        print(f"FAIL: {name} - {e}")

# Also dump all export names for reference
with open(os.path.join(OUTPUT_DIR, "all_exports.txt"), "w") as f:
    for name in sorted(all_funcs.keys()):
        f.write(f"{name}\n")

print(f"\nDone. Output in {OUTPUT_DIR}")
idc.qexit(0)
