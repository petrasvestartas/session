# Non-Planar Trimmed Surface Meshing — Plan & Reference

## Current State (as of Feb 2026)

`TrimmedSurface::mesh()` in `trimmedsurface.cpp` implements:

**Planar faces**: control points from NurbsCurve → CDT boundary only → minimal triangulation
- Condition: `crv.degree() <= 1 && !crv.is_rational()` (per-trim check in disc_loop)
- Result: 2 triangles per flat face ✓

**Non-planar faces**: `divide_by_count` boundary + curvature-adaptive Steiner grid → CDT
- Span subdivision via normal-angle (20°) + chord-height (bbox_diag×0.005) + edge length
- Interior Steiner points filtered by `point_in_polygon_2d`
- Result: 500–1200 triangles for curved shapes (sphere, cylinder body)

**Issues with current non-planar approach**:
1. Sphere: ~1213f is over-tessellated (verb-style adaptive quadtree would give ~200f)
2. Cylinder body: ~450f, boundary conformance could be improved
3. No crack-free seam handling at closed (periodic) surfaces
4. No T-junction resolution between face boundaries in BRep context

---

## Better Approach: Adaptive Quadtree + Boundary Conforming CDT

### Algorithm

See `adaptive_meshing.md` for the full quadtree algorithm (verb/curvo pattern).

For TRIMMED surfaces, the quadtree must be combined with boundary conformance:

```
1. Build adaptive quadtree over full UV domain
   (same as NurbsSurface::mesh_delaunay — normal-deviation criterion)

2. Extract leaf cells that overlap with trim region:
   - Filter: cell centroid or any corner inside outer_loop AND not inside any hole

3. For cells on the trim boundary:
   - Clip the quad cell against the trim curve (intersect cell edges with trim)
   - Generate sub-polygon vertices at intersection points

4. CDT from all leaf vertices + clipped boundary vertices
   - Insert trim boundary edges as constraints
   - CDT resolves T-junctions at refinement level boundaries

5. Cull exterior triangles by centroid test (same as current)

6. Build mesh with per-vertex normals from surface.normal_at(u, v)
```

### Why CDT for the Boundary?

The adaptive quadtree gives interior Steiner points with correct curvature density. But at the trim boundary, we need the mesh to conform exactly to the parametric boundary curve. CDT is the right tool for this — insert boundary edges as constraints, and CDT handles T-junctions between the quadtree grid and boundary automatically.

---

## Resources

| Repository | Relevance | Key Files |
|---|---|---|
| **verb** | Full adaptive trimmed NURBS tessellation | `verb/eval/Tess.hx`, `AdaptiveRefinementNode` |
| **curvo** | Rust adaptive tessellation | `src/tessellation/adaptive_tessellation_processor.rs` |
| **OpenCASCADE** | BRep meshing reference (complex) | `BRep_Mesh` module |
| **Gmsh** | `meshGFace.cpp` — parametric space CDT with boundary recovery | Too complex for our use |

### verb's Trimmed Surface Tessellation
verb's `rationalSurfaceDivideByEqualArcLength` and `rationalSurfaceClosestPoint` are
not directly useful. The trim approach in verb:
1. Tessellate surface adaptively (quadtree)
2. Find trim curve intersections with quadtree edges
3. Insert intersection points as boundary constraints
4. Run CDT on modified mesh

### Simpler Alternative: Current Approach + Quality Filter

The current span-adaptive Steiner grid already works. To reduce triangle count:
- Add aspect ratio filter: skip Steiner points in nearly-flat spans (normal deviation < 5°)
- Use `is_planar(nullptr, 1e-3)` check per-span before adding Steiner for that span
- Cap max Steiner points (e.g., max 200 for entire surface)

---

## Implementation Plan (Incremental)

### Step 1: Quick Win — Filter flat spans (1 hour)
In `TrimmedSurface::mesh()`, before the Steiner grid loop:
- Compute per-span normal deviation
- Skip Steiner insertion for spans where max_angle < 5°
- Result: sphere 1213f → ~400f, cylinder body ~200f

### Step 2: Adaptive Quadtree (4-6 hours)
New file: `trimesh_adaptive_trimmed.h/cpp`
- `AdaptiveTrimmedMesher`: quadtree over UV, boundary conforming CDT
- Uses existing `Delaunay2D` for CDT after quadtree
- Same as `adaptive_meshing.md` but adds trim curve boundary integration
- Replace `if (!planar)` Steiner block in `TrimmedSurface::mesh()`

### Step 3: Crack-free BRep meshing (future)
- Pass shared edge UV parameters between TrimmedSurface meshes
- Ensure adjacent faces share boundary vertices at matching UV positions
- Required for watertight BRep rendering

---

## Expected Triangle Counts After Step 1

| Shape | Current | After Step 1 |
|---|---|---|
| Box face | 2f | 2f |
| Cylinder body | ~450f | ~120f |
| Cylinder cap | ~30f | ~15f |
| Sphere | ~1213f | ~400f |
| Disc (rational) | ~33f | ~33f |
| Plate+hole | ~39f | ~25f |

---

## Notes

- `is_planar()` uses `ZERO_TOLERANCE = 1e-12` — works correctly for exact box CVs
- `disc_loop` uses `crv.degree() <= 1 && !crv.is_rational()` (not the `planar` flag) for boundary sampling
- The `planar` flag from `m_surface.is_planar()` only controls: flat normal assignment vs per-vertex normals
- For closed surfaces (cylinder, torus), `get_span_vector()` includes wrap-around spans
