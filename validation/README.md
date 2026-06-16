# Validation harness — our kernel vs OCCT and Rhino

Test-time-only comparison of our geometry kernel against real CAD kernels.
Nothing here is a runtime dependency of the kernel (the no-3rd-party-libs rule
applies to kernel source, not to an offline oracle).

## Oracles

- **OCCT** (built from source). `occt_oracle/` is a CMake superbuild that
  `ExternalProject_Add`s OCCT **V8_0_0_rc2** as static libs (recipe from
  github.com/petrasvestartas/compas_occt) and links a small C++ `oracle.exe`.
  - Build: `cmake -S occt_oracle -B occt_oracle/build -G "Visual Studio 17 2022" -A x64`
    then `cmake --build occt_oracle/build --config Release` (first build is long —
    it compiles OCCT). Needs `-DCMAKE_POLICY_VERSION_MINIMUM=3.5` (set in the
    CMakeLists) for CMake 4.x.
  - `oracle.exe <in> <out>` runs `ssi` (bounded-face `BRepAlgoAPI_Section`,
    matching our finite surface extents) and `interpolate` (`GeomAPI_Interpolate`)
    from primitive specs; see the grammar at the top of `oracle.cpp`.
- **Rhino 8** (full RhinoCommon, headless, needs a license). Driven via the
  `session_rhino/.venv` + `rhinoinside` bridge. `rhinoinside.load(r"C:\Program
  Files\Rhino 8\System")` then `Rhino.Geometry.*`. Used for
  `Curve.CreateInterpolatedCurve` and `Intersection.SurfaceSurface`.
  (pythonocc is NOT usable here — conda-forge only, no PyPI wheel.)

## Scripts

- `interp_cases.py` — battery of interpolated-curve cases (point set + style).
- `rhino_battery.py` — dump Rhino `CreateInterpolatedCurve` for the battery to
  `_rhino_battery.json` (run with the rhino venv).
- `compare_battery.py` — our `create_interpolated` vs the Rhino battery.
  **Result: bit-identical (~1e-14) for all open + periodic cases.**
- `compare_interp.py` — single-case ours vs OCCT `GeomAPI_Interpolate`.
- `run_ssi.py` — our `surface_surface` vs OCCT `oracle.exe ssi` on a battery,
  compared as 3D point sets (point-to-segment Hausdorff, sampling-robust).

## Findings (2026-06)

- **Curve-from-points matches Rhino bit-for-bit** (1e-14) across chord/uniform/
  centripetal, 3–7 points, 2D/3D, open and periodic. Locked by golden CV
  assertions in `nurbscurve_test` (all 3 languages). 2-point case fixed to
  Rhino's degree-1 line.
- **Anything × plane** intersections match OCCT to ~0.01–0.02, limited by OCCT's
  coarse section sampling (chord sagitta), not our error — curves are ~exact.
- **Surface × surface (marching)** SSI: trace seam-closure bug fixed (loops
  close); curves become the correct loci (~1e-2 vs OCCT). Reaching 1e-6 needs
  analytic quadric×quadric intersection (in progress), not marching+fit.

## Regenerating golden data

If `create_interpolated` parity values ever need refreshing:
`session_rhino/.venv/Scripts/python.exe validation/rhino_battery.py`, then read
`_rhino_battery.json`.
