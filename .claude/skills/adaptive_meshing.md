# Adaptive NURBS Surface Meshing — Algorithm & Reference

## Problem Statement
Uniform per-span grid meshing (trimesh_grid.cpp) over-tessellates flat regions. Need curvature-adaptive tessellation: few triangles on flat areas, many on curved.

## Algorithm: Adaptive Quadtree in UV Parameter Space

### Source Repositories (study these for implementation)

| Repository | Lang | URL | Key Files |
|---|---|---|---|
| **verb** | Haxe/JS | https://github.com/pboyer/verb | `src/verb/eval/Tess.hx` — AdaptiveRefinementNode |
| **curvo** | Rust | https://github.com/mattatz/curvo | `src/tessellation/adaptive_tessellation_node.rs`, `adaptive_tessellation_processor.rs`, `surface_tessellation.rs` |
| **truck** | Rust | https://github.com/ricosjp/truck | `truck-meshalgo` crate |
| **Gmsh** | C++ | https://gitlab.onelab.info/gmsh/gmsh | `src/mesh/meshGFace.cpp` — parametric surface meshing |
| **geogram** | C++ | https://github.com/BrunoLevy/geogram | CVT remeshing (post-process only) |
| **CGAL** | C++ | https://github.com/CGAL/cgal | `isotropic_remeshing()` with `Adaptive_sizing_field` |
| **LNLib** | C++ | https://github.com/BIMCoderLiang/LNLib | NURBS Book algorithms, evaluation reference |

### Core Algorithm (from verb + curvo)

```
Phase 1: Build Quadtree
  Input: NurbsSurface, options (norm_tol, min_depth, max_depth)

  1. Create root node covering full UV domain [u0,u1] × [v0,v1]
  2. Evaluate surface point + normal at 4 corners + center (5 evals)
  3. Recursive subdivision:

     should_divide(node, depth):
       if depth < min_depth: return BOTH
       if depth >= max_depth: return NONE

       // Normal deviation test (squared norm of difference)
       split_v = |N[0]-N[1]|² > norm_tol || |N[2]-N[3]|² > norm_tol
       split_u = |N[1]-N[2]|² > norm_tol || |N[3]-N[0]|² > norm_tol

       // Center normal test
       for each corner normal Ni:
         if |Ni - N_center|² > norm_tol: split in appropriate direction

       return direction (U, V, BOTH, or NONE)

     divide(node, direction):
       if direction == U or BOTH:
         split at u_mid → 2 children (left, right)
       if direction == V or BOTH:
         split at v_mid → 2 children (bottom, top)
       Evaluate new corner/mid points from surface
       Set neighbor pointers between siblings
       Propagate parent's neighbor pointers to children

Phase 2: Extract Triangles from Leaves
  For each leaf node:
    1. Collect corners: 4 base corners
    2. For each edge, check neighbor's subdivision level:
       - If neighbor is more refined, get its edge points
       - This gives 4, 5, or 6+ vertices per leaf
    3. Triangulate:
       - 4 vertices → 2 triangles (diagonal split)
       - 5 vertices → 3 triangles (T-junction: fan from opposite corner)
       - 6+ vertices → fan from computed center point
    4. Deduplicate vertices via UV hash map
```

### Parameters

```
norm_tol:   2.5e-2 (verb default) — squared norm of normal difference
            Equivalent to ~18° angle between normals
min_depth:  0 (verb) or 1 (recommended: ensures at least 1 subdivision)
max_depth:  10 (verb default, cap = 1024×1024 cells)
```

### Data Structures

```cpp
struct QuadNode {
    double u0, v0, u1, v1;       // UV bounds
    Point corners[4];             // 3D positions: BL, BR, TR, TL
    Vector normals[4];            // surface normals at corners
    Point center;                 // 3D at UV center
    Vector center_normal;         // normal at center
    int children[4];              // -1 = leaf, else index into node pool
    int neighbors[4];             // adjacent nodes: bottom, right, top, left
    int depth;
};
```

### Handling Closed Surfaces + Poles

**Closed U**: UV wrapping — when node edge is at u_max, its neighbor at u_min.
  - Root nodes: create initial row spanning [u_min,u_max], set wrap-around neighbors
  - verb approach: `divideRationalSurfaceAdaptive()` creates initial grid of nodes covering spans, then links neighbors including wrap-around

**Poles** (singular edges, e.g., sphere top/bottom):
  - Degenerate normals (magnitude ≈ 0) at pole → `fix_normals()` propagates valid normals from adjacent corners
  - Don't subdivide further at pole — the degenerate edge collapses to a single vertex
  - In triangle extraction: detect degenerate edges (corners coincide in 3D), emit single triangle instead of quad

### Comparison: verb vs curvo

| Aspect | verb | curvo |
|---|---|---|
| Subdivision | Always binary (2 children) in chosen direction | Binary, alternating U/V direction |
| Criterion | `normSquared(n1-n2) > normTol` | Same: `(n1-n2).norm_squared() > tol` |
| T-junction | `getAllCorners()` collects neighbor edge points | Same approach via neighbor references |
| Triangulation | 4→2tri, 5→3tri, 6+→fan from center | Same: 4→2, 5→3, 6+→fan |
| Language | Haxe (compiles to JS/C++/Python) | Rust |
| Poles | Normal fix propagation | Same |

### Comparison: Quadtree vs Our Current Approaches

| Feature | trimesh_grid (current mesh()) | trimesh_delaunay (current mesh_delaunay()) | Adaptive Quadtree (proposed) |
|---|---|---|---|
| Subdivision | Uniform per span | Grid + Delaunay + Ruppert refinement | Recursive per-quad |
| Curvature adapt | Per-span only (coarse) | Per-span + angle-based refinement | Per-quad (fine-grained) |
| Flat areas | Over-tessellated | Over-tessellated | 2 triangles |
| Typical count | ~800 (sphere) | ~400 (sphere) | ~200 (sphere) |
| Speed | Fast (grid only) | Slow (CDT + refinement) | Fast (recursive eval only) |
| Seam handling | Glue vertices | Glue vertices (artifacts) | Neighbor pointers (clean) |

### Gmsh's Approach (for reference, NOT what we implement)

Gmsh uses a fundamentally different pipeline:
1. Boundary discretization → edge meshing
2. Boundary recovery in parametric space (Delaunay + edge recovery)
3. Refinement with `BGM_MeshSize()` sizing field
4. Optional quad recombination

This is overkill for our use case (no trimming curves, no embedded features). The quadtree approach is simpler and faster.

## Implementation Plan

### Files
- `session_cpp/src/trimesh_adaptive.h` — QuadNode + TrimeshAdaptive class
- `session_cpp/src/trimesh_adaptive.cpp` — Implementation (~400 lines)
- `session_cpp/src/nurbssurface.cpp` — Change `mesh_delaunay()` to use TrimeshAdaptive

### Steps
1. QuadNode struct with pool allocator (vector<QuadNode>)
2. build_quadtree(): create initial nodes per span pair, link neighbors, recurse
3. Closed surface handling: wrap-around neighbor links for closed U/V
4. Pole handling: detect degenerate normals, propagate valid normals
5. extract_mesh(): walk leaves, collect corners + T-junction points, triangulate
6. Vertex dedup via UV hash map
7. Output Mesh with positions, normals, UVs from deduplicated vertices

### Expected Triangle Counts
| Surface | Current Grid | Adaptive Quadtree | Reduction |
|---|---|---|---|
| Sphere | ~800 | ~200 | 4× |
| Flat plane | 2 | 2 | same |
| Torus | ~1200 | ~400 | 3× |
| Wave (multi-span) | ~3000 | ~800 | 4× |
| Cone (pole) | ~600 | ~150 | 4× |

## Academic References
- "Adaptive Tessellation for Trimmed NURBS Surface" (Ma & Hewitt, Eurographics 2002) — quadtree + Bézier flatness test
- "Adaptive Tessellation of NURBS Surfaces" (Espino & Bóo, ResearchGate) — fully adaptive per-quad
- "Efficient Trimmed NURBS Tessellation" (Balázs, Uni Bonn 2004) — restricted quadtree, crack-free
- "Adaptive Remeshing for Real-Time Mesh Deformation" (Eurographics 2013) — CGAL's curvature-adaptive sizing field
