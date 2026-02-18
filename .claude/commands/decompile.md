Reverse engineering reference for Rhino 8 DLLs.

## Tools
- Ghidra: headless analysis with scripts in `.claude/ghidra_scripts/`
- ILSpy: .NET decompilation for RhinoCommon.dll
- Target DLLs: TL.DLL (math), RhinoCore.dll (commands), rhcommon_c.dll (wrapper)

## Key functions
- `TL_CubicNurbThroughPoints` — NURBS curve interpolation
- `TL_CubicNurbInterpolate` — Tridiagonal solver
- `TL_GrevilleAbcissa` — Greville point calculation

## Reference files
- `.claude/decompile_progress/ALGORITHM_ANALYSIS.md` — 441 decompiled functions
- `.claude/decompile_progress/CLEAN_API_REFERENCE.md` — C# to native mapping
- `.claude/skills/rhino-decompile-advanced.md` — Ghidra techniques
- `SKILLS_RHINO_GEOMETRY.md` — 711 C exports + ~7100 C++ methods

See `SKILLS_RHINO_DECOMPILE.md` for full guide.
