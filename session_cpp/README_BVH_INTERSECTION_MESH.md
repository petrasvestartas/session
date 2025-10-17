# Intersection, BVH, and Mesh Ray Acceleration — Design and Implementation Notes

This document summarizes the design, changes, and references for the BVH-backed ray–mesh intersection pipeline implemented across `intersection`, `bvh`, and `mesh` modules in `session_cpp/`.

## Overview

- The ray–mesh intersection path was upgraded to use a hierarchical BVH traversal by default, with per-mesh triangle AABB caching.
- The BVH provides broad-phase culling; exact intersection is performed with triangle tests on the candidate set.
- The `Mesh` class now owns and caches the triangle-level AABBs and BVH to avoid rebuilding for repeated ray casts.
- Deterministic ordering and tolerance-aware tie-breaking ensure results match the previous naive method.

## Changes Summary

- **`session_cpp/src/intersection.h` / `session_cpp/src/intersection.cpp`**
  - Added and now use a BVH-backed path in `Intersection::ray_mesh_bvh()`.
  - `ray_mesh_bvh()` pulls triangle candidates via BVH, then performs exact `ray_triangle()` tests, returning:
    - For `find_all = false`: the nearest hit using epsilon-aware tiebreakers.
    - For `find_all = true`: all hits sorted by `(t, face_index)` with epsilon tie-breakers for determinism.
  - Clarified AABB assumptions in `ray_box()` and `ray_mesh_bvh()`.
  - Minor robustness updates:
    - `plane_plane()` only writes output if a valid intersection line exists.
    - `line_line_parameters()` gained `near_parallel_as_closest` flag to return closest points for near-parallel lines.

- **`session_cpp/src/bvh.h` / `session_cpp/src/bvh.cpp`**
  - New traversal API: `BVH::ray_cast(const Point&, const Vector&, std::vector<int>&, bool)`.
  - Implementation details:
    - Uses a stack-based traversal over a binary BVH built from triangle AABBs.
    - Node culling via a slab-based AABB ray test (`ray_aabb_intersect()`), treating boxes as world-axis AABBs.
    - Near-first visiting order (by entry `tmin`) to prioritize close nodes; still returns the full candidate set for correctness.
  - BVH build:
    - Objects are sorted by Morton code of AABB centers for spatial locality; a simple recursive split builds a binary tree.
    - Parent AABBs are computed by merging children.

- **`session_cpp/src/mesh.h` / `session_cpp/src/mesh.cpp`**
  - Added cached, per-mesh triangle BVH and AABBs:
    - Fields: `triangle_bvh_built`, `triangle_bvh`, `triangle_boxes_cache`, `triangle_data_cache`.
    - Methods:
      - `build_triangle_bvh(bool force = false) const`
      - `triangle_bvh_ray_cast(const Point&, const Vector&, std::vector<int>&, bool) const`
      - `get_triangle_by_id(int tri_id, size_t& face_idx, size_t& sub_idx, Point& v0, Point& v1, Point& v2) const`
      - `clear_triangle_bvh() const`
    - Caches invalidate on `clear()`, `add_vertex()`, `add_face()`, and `transform()`.
  - JSON persistence:
    - `Mesh::jsondump()` persists triangle AABB cache and triangle data tuples.
    - `Mesh::jsonload()` reconstructs caches and rebuilds the BVH if cache data is present.
  - Face index mapping:
    - Triangle cache stores sequential face indices to match `Mesh::to_vertices_and_faces()` (ensures parity with naive method and tests).

## API Cheatsheet

- Intersection
  - `bool session_cpp::Intersection::ray_mesh_bvh(const Point&, const Vector&, const Mesh&, std::vector<RayHit>&, bool find_all)`
- BVH
  - `bool session_cpp::BVH::ray_cast(const Point&, const Vector&, std::vector<int>& candidate_leaf_ids, bool find_all) const`
- Mesh cache
  - `void session_cpp::Mesh::build_triangle_bvh(bool force = false) const`
  - `bool session_cpp::Mesh::triangle_bvh_ray_cast(const Point&, const Vector&, std::vector<int>&, bool) const`
  - `bool session_cpp::Mesh::get_triangle_by_id(int, size_t&, size_t&, Point&, Point&, Point&) const`
  - `void session_cpp::Mesh::clear_triangle_bvh() const`

## Implementation Details

- **Per-triangle AABB creation**
  - Faces are triangulated by fan (v0, v[i], v[i+1]).
  - AABBs are created via `BoundingBox::from_points()` (world-axis-aligned boxes).

- **BVH build (broad-phase)**
  - Morton codes computed from AABB centers, objects sorted by code.
  - A binary tree is built by recursive splitting of the sorted array.
  - Node AABBs are merges of child boxes (min/max corners).

- **Ray–AABB test**
  - Slab method with precomputed reciprocal directions; returns `[tmin, tmax]` interval and miss if `tmax < tmin`.

- **Traversal**
  - Explicit stack; push children whose AABBs intersect the ray.
  - Near-first ordering by `tmin` improves early discovery while still collecting all candidates for exact tests.

- **Exact phase and ordering**
  - Candidates are tested with `ray_triangle()`.
  - For `find_all`: hits sorted with epsilon-aware `(t, face_index)` comparator for determinism.
  - For first hit: choose minimal `t` with epsilon tie-breaking by `face_index` to match naive ordering.

- **AABB contract**
  - All BVH and ray-box routines assume axis-aligned bounding boxes in world coordinates.
  - Oriented bounding boxes require conversion to AABBs or a dedicated OBB ray test.

## Testing

- Existing tests (`intersection_test.cpp`) were used to verify parity with the naive method.
- The BVH path now returns identical hit `t` values and `face_index` ordering (within a small epsilon) compared to naive.
- All tests pass (as of this change): 243 assertions in 84 test cases.

## References

- **Ray–AABB intersection (slab test)**
  - Amy Williams, Steve Barrus, R. Keith Morley, and Peter Shirley. “An Efficient and Robust Ray–Box Intersection Algorithm.” Journal of Graphics Tools 10(1), 2005.

- **Morton code bit interleaving**
  - Sean Eron Anderson. “Bit Twiddling Hacks — Interleaving bits by binary magic numbers.”
    https://graphics.stanford.edu/~seander/bithacks.html#InterleaveTableObvious

- **BVH and LBVH construction (Morton code sorting idea)**
  - Tero Karras. “Maximizing Parallelism in the Construction of BVHs, Octrees, and k-d Trees.” Proceedings of High Performance Graphics, 2012.

These references guided the slab-based ray–AABB test, the bit-interleaving approach used for Morton codes, and the sorted-by-Morton-code BVH construction concept (adapted here to a simple CPU recursive builder).

---

If you need oriented-box support, persistent BVH across mesh edits, or multi-mesh scene BVHs, we can extend this foundation with additional structures and APIs.
