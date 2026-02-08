# SKILLS_NURBS_MESHING.md

## What Was Built

End-to-end pipeline: C++ NURBS surface → triangulated mesh → JSON session → Python deserialization → Rhino 3D visualization.

### Features Implemented
1. **Trimmed planar surface creation** (`NurbsSurface::create_planar`) from boundary curves
2. **Inner loops (holes)** in NurbsSurface — data model across C++, Python, Rust, protobuf
3. **NurbsTriangulation** — constrained Delaunay with Ruppert refinement, hole support, planar optimization
4. **Cached mesh** — `mutable Mesh m_mesh` with lazy computation via `mesh()` method
5. **Session JSON pipeline** — surfaces carry embedded mesh + trim curves in JSON
6. **Rhino Python bridge** — `session_rhino` module converts session_py objects to Rhino geometry
7. **`boundary_curves_3d()`** — evaluates UV trim loops on surface to produce 3D curves

---

## Pipeline Architecture

```
C++ main.cpp
  │  NurbsSurface::create_planar(boundary_curves)
  │  srf->add_inner_loop(hole_uv_curve)
  │  srf->mesh()  ← triggers NurbsTriangulation, caches in m_mesh
  │  session.add(session.add_nurbssurface(srf))
  │  session.jsondump()  ← NurbsSurface includes mesh + outer_loop + inner_loops
  ▼
JSON file (session_data/nurbs_meshing.json)
  │  Each nurbssurface contains:
  │    - NURBS data (CVs, knots, orders)
  │    - outer_loop (degree-1 UV curve)
  │    - inner_loops[] (hole UV curves)
  │    - mesh (triangulated, embedded)
  ▼
Python (session_py)
  │  Session.__jsonload__(data)
  │  srf.mesh()        → returns cached Mesh from JSON
  │  srf.boundary_curves_3d()  → evaluates UV loops on surface → 3D NurbsCurves
  ▼
Rhino Python (session_rhino)
  │  Session.load(filepath)  → deserializes JSON via session_py
  │  scene.add(srf)          → rhino_nurbssurface.to_rhino()
  │  scene.add(mesh)         → rhino_mesh.to_rhino()
  │  scene.add(curve)        → rhino_nurbscurve.to_rhino()
  │  scene.draw()            → adds to Rhino document
  ▼
Rhino viewport
```

---

## Key Concepts

### Trim Loops in UV Space
- **Outer loop** (`m_outer_loop`): boundary of the surface in (u,v) parameter space. Degree 1 for polygons, sampled for higher-degree curves.
- **Inner loops** (`m_inner_loops[]`): holes. Same UV representation.
- UV curves are NOT 3D — they need `point_at(u,v)` evaluation on the surface to become 3D geometry.
- `boundary_curves_3d()` does this: reads each loop's CVs, evaluates on surface, returns 3D NurbsCurves.

### create_planar() Algorithm
1. Collect all CVs from input boundary curves
2. Fit plane via PCA (`Plane::from_points_pca`)
3. Project points onto plane axes → compute bounding box
4. Add 5% padding → create bilinear patch (2×2 CVs, degree 1)
5. Project boundary curve to UV space:
   - **Degree ≤ 1**: use CVs directly (they ARE the points)
   - **Degree > 1**: sample 10 pts/span to capture curvature
6. Store as degree-1 outer_loop in UV space

### add_hole() — 3D to UV Projection
```cpp
Point orig = srf.point_at(0, 0);   // UV origin in 3D
Point pu   = srf.point_at(1, 0);   // end of U axis
Point pv   = srf.point_at(0, 1);   // end of V axis
// Build orthonormal frame, project 3D hole points → (u,v)
```
Works for any plane orientation because the bilinear surface maps [0,1]² → 3D linearly.

### NurbsTriangulation (C++ only)
- Constrained Delaunay triangulation in UV space
- Outer loop vertices → boundary polygon
- Inner loop vertices → hole polygons
- Triangle removal: outside boundary OR inside any hole
- Ruppert refinement: circumcenter insertion, respects holes
- Planar optimization: skip 8×8 grid for planar surfaces
- Final: map UV vertices to 3D via `surface.point_at(u,v)`

### Cached Mesh
```cpp
mutable Mesh m_mesh;  // computed once, stored
Mesh NurbsSurface::mesh() const {
    if (m_mesh.number_of_vertices() == 0)
        m_mesh = NurbsTriangulation(*this).mesh();
    return m_mesh;
}
```
- `jsondump()` embeds mesh if computed
- `jsonload()` restores mesh from JSON
- Python `mesh()` returns stored mesh (no C++ triangulation in Python)

---

## Bugs Found & Fixed

### 1. Degree-3 interpolation for polyline trim curves (Rhino)
**Symptom**: wiggly rectangle edges in Rhino
**Cause**: `_eval_loop_3d()` used `CreateInterpolatedCurve(pts, 3)` — degree-3 overshoots at sharp corners
**Fix**: check `loop.degree()`, use `PolylineCurve` for degree ≤ 1

### 2. Oversampling degree-1 curves in create_planar()
**Symptom**: 41 CVs for a 4-point rectangle
**Cause**: always sampled 10 pts/span even for polylines
**Fix**: `if (crv.degree() <= 1)` use CVs directly, else sample

### 3. Missing "type" field in NurbsCurve JSON
**Symptom**: `decode_node()` couldn't identify NurbsCurve objects in session
**Fix**: added `j["type"] = "NurbsCurve"` to jsondump()

### 4. Mesh.__jsonload__ crash on null facedata/edgedata
**Symptom**: Python crash loading C++ mesh JSON with `"facedata": null`
**Fix**: changed `if "facedata" in data` to `if data.get("facedata")`

### 5. __jsonload__ signature mismatch
**Symptom**: `decode_node` calls `cls.__jsonload__(node, guid, name)` but NurbsSurface only accepted `data`
**Fix**: added `guid=None, name=None` params

---

## File Map

### C++
- `session_cpp/src/nurbssurface.h` — added `mutable Mesh m_mesh`, `m_inner_loops`, mesh/hole API
- `session_cpp/src/nurbssurface.cpp` — `create_planar()`, `mesh()` caching, JSON serialization for mesh/holes
- `session_cpp/src/triangulation_nurbs.cpp` — hole support, planar optimization
- `session_cpp/main.cpp` — test cases: freeform, curved, triangle, trapezoid, hexagon+holes

### Python
- `session_py/src/session_py/nurbssurface.py` — `m_mesh`, `mesh()`, `boundary_curves_3d()`, JSON mesh support
- `session_py/src/session_py/objects.py` — added nurbscurves/nurbssurfaces collections
- `session_py/src/session_py/session.py` — `add_nurbscurve()`, `add_nurbssurface()`

### Rhino
- `session_rhino/src/session_rhino/rhino_nurbssurface.py` — `to_rhino()` with trim support, `PolylineCurve` for degree-1
- `session_rhino/src/session_rhino/session.py` — `Session.load()` static method
- `session_rhino/examples/rhino_scene_nurbssurface_json.py` — 3-step usage example

### Proto
- `session_proto/nurbssurface.proto` — added outer_loop + inner_loops fields

---

## Sources & References

### OpenNURBS (McNeel, open source)
- https://github.com/mcneel/opennurbs
- NURBS evaluation: de Boor's algorithm, basis functions, knot span search
- Knot conventions: cv_count + order - 2 knots (Rhino drops first/last of full vector)
- `ON_NurbsSpanIndex` → our `find_span()`
- `ON_EvaluateNurbsBasis` → our `basis_functions()`

### The NURBS Book (Piegl & Tiller, Springer, 1997)
- B-spline basis function definition (Cox-de Boor recursion)
- Surface evaluation: tensor product of univariate basis functions
- Knot insertion algorithm
- Degree elevation

### Computational Geometry
- Delaunay triangulation: Bowyer-Watson algorithm
- Constrained Delaunay: edge recovery after initial triangulation
- Ruppert's algorithm: circumcenter-based mesh refinement
- Point-in-polygon: ray casting / winding number

### RhinoCommon API
- https://developer.rhino3d.com/api/rhinocommon/
- `Rhino.Geometry.NurbsSurface.Create(dim, is_rat, order_u, order_v, n_u, n_v)`
- `Rhino.Geometry.Brep.CreatePlanarBreps(curves, tolerance)` — trimmed planar surfaces
- `Rhino.Geometry.PolylineCurve` — degree-1 curves (no interpolation overshoot)
- `Rhino.Geometry.Curve.CreateInterpolatedCurve` — for smooth curves only

### Rhino Knot Convention
Rhino uses `cv_count + order - 2` knots (drops first and last of the full `cv_count + order` vector).
Our internal representation uses `cv_count + order - 2` knots (matching OpenNURBS/Rhino).

---

## Performance Optimization (Session 2 — Feb 2026)

### Problem
Meshing was unacceptably slow for real-time OpenGL visualization:
| Surface | Verts | Faces | Time |
|---------|-------|-------|------|
| freeform 4×4 | 305 | 544 | 805ms |
| irregular 10×10 | 1572 | 2993 | 24,184ms |
| wavy 15×15 | 2835 | 5451 | 78,968ms |

### Root Causes
1. **No caching of NURBS evaluations** — same vertex evaluated hundreds of times across refinement iterations
2. **O(T) linear scan** in `find_worst_triangle` every iteration
3. **O(T) brute-force** in Delaunay insert (circumcircle test on ALL triangles)
4. **Point/Vector construction overhead** — every `point_at()` creates UUID, Color, Xform objects
5. **Delaunay + adaptive refinement unnecessary** for visualization (smooth shading handles quality)

### Optimizations Applied (C++)

#### Phase 1: Algorithmic (Delaunay + refinement path, still used for trimmed surfaces)
1. **Vertex position/normal cache** — `std::vector<std::array<double,3>>` for pos/normal, avoid re-evaluating NURBS
2. **Edge midpoint chord height cache** — `FlatMap64<double>` stores squared chord distances per edge
3. **Priority queue** — max-heap replaces O(T) linear scan for worst triangle
4. **Edge hash map** — `FlatMap64<std::pair<int,int>>` for O(1) neighbor linking (replaces O(N×T))
5. **BFS cavity search** — adjacency-guided BFS replaces O(T) brute-force circumcircle scan
6. **Locate walk hint** — `last_found_` set near insertion point, walk from there

#### Phase 2: NURBS evaluation (nurbssurface.cpp)
7. **Fast eval overloads** — `point_at(u,v,&x,&y,&z)` and `normal_at(u,v,&nx,&ny,&nz)` write raw doubles
8. **Stack-allocated basis functions** — `double N[100]` and `double ndu[10][10]` instead of heap `std::vector`
9. **Combined point+normal** — `point_and_normal_at()` shares find_span + basis computation
10. **Squared distance / cos(angle)** comparisons to avoid sqrt/acos in scoring

#### Phase 3: Hash map (triangulation_nurbs.h)
11. **FlatMap64<V>** — custom open-addressing hash map replacing `std::unordered_map<uint64_t,...>`
    - Fibonacci hashing: `(key * 0x9E3779B97F4A7C15) >> shift`
    - Linear probing, backward-shift deletion (no tombstones)
    - Flat `std::vector<Slot>` storage (cache-friendly)

#### Phase 4: Grid mesh for visualization (the big win)
12. **Direct UV grid mesh** for untrimmed surfaces — **eliminates Delaunay entirely**
    - Get span vectors, subdivide each span adaptively:
      - ≤2 spans: 8 subs/span (few-span surfaces need more detail)
      - 3–8 spans: 3 subs/span
      - >8 spans: 2 subs/span (smooth shading handles quality)
    - Evaluate `point_and_normal_at()` on regular grid
    - Emit indexed triangles directly (2 per quad cell)
    - No Delaunay, no scoring, no priority queue, no refinement

### Results

| Surface | Before | After | Speedup |
|---------|--------|-------|---------|
| freeform 4×4 (1 span) | 805ms | **0.3ms** | **2,680×** |
| irregular 10×10 (7 spans) | 24,184ms | **1.8ms** | **13,400×** |
| wavy 15×15 (12 spans) | 78,968ms | **2.4ms** | **32,900×** |
| wave 20×20 (17 spans) | ~18ms | **4.4ms** | **4×** |

All under 5ms — well within 16.7ms frame budget (60fps).

### Implementation (all 3 languages)
The fast grid mesh is implemented identically in C++, Rust, and Python:
```
mesh():
  if not trimmed:
    usp = get_span_vector(0), vsp = get_span_vector(1)
    subs = adaptive(span_count)  // 8, 3, or 2
    for each span: generate sub-parameters
    for each (u,v): point_at + normal_at → add_vertex + set_normal
    for each quad cell: 2 triangles → add_face
    return mesh
  else:
    // Delaunay + refinement (C++ only, trimmed surfaces)
```

### Key Files Modified
- `session_cpp/src/triangulation_nurbs.h` — FlatMap64, Delaunay2D with edge hash, pre-allocated buffers
- `session_cpp/src/triangulation_nurbs.cpp` — grid mesh path, all caching/scoring optimizations
- `session_cpp/src/nurbssurface.h` — fast eval method declarations
- `session_cpp/src/nurbssurface.cpp` — eval_basis_stack, point_and_normal_at
- `session_rust/src/nurbssurface.rs` — mesh() with grid approach
- `session_py/src/session_py/nurbssurface.py` — mesh() with grid approach

---

## Session 3 — Feb 5 2026: main.cpp 15-Surface Demo + Fixes

### What Was Done
1. **Extended main.cpp** with 4 new factory method surfaces: `create_loft`, `create_revolve`, `create_sweep1`, `create_sweep2`
2. **X-axis alignment** — all 15 surfaces spread along x-axis with 12-unit spacing via `srf->transform(Xform::translation(xo, 0, 0))`
3. **Fixed sweep1 crash** — profile had 3 points with degree 3 (needs >=4 points); changed to degree 2
4. **Added mesh() guard** — `if (m_mesh.number_of_vertices() == 0 && is_valid())` prevents crash on invalid surfaces
5. **Fixed output path** — `../../session_data/` was wrong; now uses absolute path

### 15 Surfaces in main.cpp
| # | Name | Type | Mesh |
|---|------|------|------|
| 1 | freeform | create(4x4, deg3) | 81v 128f |
| 2 | planar_curved | create_planar(deg3 boundary) | 60v 58f |
| 3 | rotated_curved | create_planar(3D boundary) | 50v 48f |
| 4 | rect_hole | create_planar + add_hole | 8v 8f |
| 5 | triangle | create_planar(3 pts) | 3v 1f |
| 6 | trapezoid | create_planar(4 pts, 3D) | 4v 2f |
| 7 | hexagon_2holes | create_planar + 2 holes | 13v 15f |
| 8 | varying_curvature | create(8x8, Gaussian bump) | 256v 450f |
| 9 | ridge_valley | create(8x8, ridge+valley) | 256v 450f |
| 10 | saddle_flat_corner | create(6x6, saddle) | 100v 162f |
| 11 | irregular_both_sides | create(10x10, complex) | 484v 882f |
| 12 | loft | create_loft(3 sections, deg2) | 81v 128f |
| 13 | revolve | create_revolve(vase, Z axis) | 117v 192f |
| 14 | sweep1 | create_sweep1(rail, deg2 profile) | 333v 576f |
| 15 | sweep2 | create_sweep2(2 rails, profile) | 333v 576f |

### Key Gotchas
- **NurbsCurve::create degree vs points**: `create(false, degree, points)` — must have `points.size() >= degree + 1`. With 3 points, max degree is 2.
- **transform(Xform) modifies CVs in-place** — must be called BEFORE mesh() so meshing sees transformed positions
- **Trimmed surfaces (4,5,6,7)** have low vertex counts because they use Delaunay path with few boundary vertices
- **Windows exe working directory**: `start //wait //b` may change cwd; use absolute paths for file output
- **PowerShell runs Windows exes** correctly; MSYS2 bash gives exit 127 for native .exe files

### Current State
- C++ tests: **452/452 assertions pass**, 84/84 tests pass
- JSON output: `session_data/nurbs_meshing.json` (3.6MB, 15 surfaces with meshes)
- Grid mesh in all 3 languages (C++, Rust, Python) — identical algorithm
- Delaunay path retained for trimmed surfaces (C++ only)

---

## Session 4 — Feb 5 2026: Curvature-Adaptive Meshing + Sweep Fixes

### What Was Done
1. **Curvature-adaptive grid mesh** — replaced fixed per-span subdivisions (8/3/2) with midpoint chord height analysis
2. **Rewrote create_sweep1** — fixed rotation axis mapping, reduced section count
3. **Rewrote create_sweep2** — general R=T*S^T rotation, profile frame computation, reduced section count
4. **Better loft example** — 4 circles at different heights/radii (r=2.0@z=0, r=1.0@z=2, r=1.5@z=4, r=0.8@z=6)
5. **Better revolve example** — wine glass profile (7 CVs: flat base, narrow stem, wide bowl)
6. **NurbsTriangulation quality scaling** — m_max_chord_height now affects grid mesh density

### Curvature-Adaptive Algorithm (all 3 languages)
Per-span midpoint chord height analysis:
```
span_subdivisions(dir, sp, osp):
  for each span [t0, t1]:
    tm = (t0 + t1) / 2
    sample at 3 cross-positions: start, middle, end of other direction
    for each sample position s:
      p0 = point_at(t0, s), p1 = point_at(t1, s), pm = point_at(tm, s)
      linear_mid = (p0 + p1) / 2
      dev = distance(pm, linear_mid)   # chord height deviation
      slen = distance(p0, p1)           # span length
    ratio = max_dev / max_slen
    subs = 2 if ratio<0.005, 3 if <0.03, 4 if <0.08, 6 if <0.15, 8 if <0.25, else 10
```
Flat spans get 2 subdivisions, highly curved spans get up to 10. O(3*n_spans) extra evaluations.

### Sweep1 Fix
- **Section count**: `min(max(span_count*2+1, 3), 12)` instead of `max(cv_count*2, 20)`
- **Rotation fix**: profile x→tangent(fz), y→frame_x(fx), z→frame_y(fy). Was mapping arch height along tangent.

### Sweep2 Fix
- **Section count**: same reduction as sweep1
- **Profile frame**: computes (prof_side, prof_dir, prof_up) from profile start/end
- **Rotation**: general R = T * S^T where T=[tangent, x_dir, y_dir], S=[prof_side, prof_dir, prof_up]

### Results
- C++ tests: 5425/5425 assertions, 85/85 test cases
- C++ minitests: 195/195 pass
- Rust minitests: 169/169 pass
- Output: 15 surfaces, 7.6MB (down from 21MB with first threshold attempt)

### Files Modified
- `session_cpp/src/triangulation_nurbs.cpp` — span_subs lambda, quality scaling
- `session_rust/src/nurbssurface.rs` — span_subdivisions() method
- `session_py/src/session_py/nurbssurface.py` — _span_subdivisions() method
- `session_cpp/src/nurbssurface.cpp` — sweep1/sweep2 rewrite
- `session_cpp/main.cpp` — loft (circles), revolve (wine glass)

---

## Diagnostic Method

When output looks wrong:
1. **Print actual data** at each pipeline stage (CV counts, coordinates, degrees)
2. **Identify where numbers diverge** from expected values
3. **Read the source** at that stage
4. **Apply domain knowledge** (e.g., degree-1 = polyline = CVs are exact points)
5. **Fix at the root**, not downstream
