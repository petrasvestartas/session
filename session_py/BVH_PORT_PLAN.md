# BVH C++ to Python Port Plan

## Overview
Port C++ LBVH implementation (~766 lines) to Python with exact same functionality.

## Key C++ Features to Port

### 1. LBVH Construction (Karras 2012)
- **Current Python**: Simple recursive midpoint split
- **C++ Target**: Linear BVH with Morton code sorting
  - Radix sort (3 passes, 10 bits each)
  - CLZ (count leading zeros) for common prefix
  - determine_range() - find object range for each internal node  
  - find_split() - binary search for split position
  - O(n) construction vs O(n log n)

### 2. Ray Casting (NEW)
- Priority queue traversal ordered by AABB entry distance
- Returns candidate leaf IDs sorted by distance
- Prunes boxes behind ray origin
- Support for find_all vs find_first modes

### 3. Optimized Collision Detection
- **Current Python**: Per-object traversal
- **C++ Target**: Dual self-traversal of BVH tree
  - Single traversal finds all pairs
  - Canonicalized pair ordering avoids duplicates
  - Returns (pairs, colliding_indices, check_count)

### 4. Internal AABB Type
- Lightweight `BvhAABB` named tuple (cx, cy, cz, hx, hy, hz)
- Avoids BoundingBox overhead during traversal

## Implementation Steps

### Step 1: Core Infrastructure
- [x] Add BvhAABB named tuple
- [ ] Add _clz32() function
- [ ] Add _radix_sort() function  
- [ ] Add _ray_aabb_intersect() function

### Step 2: LBVH Build
- [ ] Replace build() with LBVH algorithm
- [ ] Implement common_prefix()
- [ ] Implement determine_range()
- [ ] Implement find_split()
- [ ] Post-order AABB computation

### Step 3: Ray Casting
- [ ] Implement ray_cast() method
- [ ] Heap-based traversal
- [ ] Distance-ordered results

### Step 4: Collision Detection
- [ ] Replace check_all_collisions() with dual traversal
- [ ] Stack-based pair processing
- [ ] Canonicalized ordering

### Step 5: Tests
- [ ] Port all C++ test cases
- [ ] Verify Morton code correctness
- [ ] Verify LBVH topology
- [ ] Verify ray casting results
- [ ] Verify collision detection matches C++

## Test Cases to Port

From `bvh_test.cpp` (321 lines):
1. expand_bits
2. Morton code at origin/corners
3. Morton code spatial locality  
4. BVH node creation/leaf
5. BVH build empty/single/multiple
6. AABB intersection
7. check_all_collisions
8. merge_aabb
9. Performance test (100 boxes)
10. Fixed 100 boxes collision test (deterministic)
11. Ray casting tests (NEW)

## Files to Modify

1. `bvh.py` - Complete rewrite (~600 lines)
2. `bvh_test.py` - Port all C++ tests (~400 lines)

## Estimated Scope

- **Lines of code**: ~1000 lines (bvh.py + tests)
- **Complexity**: High (LBVH algorithm is intricate)
- **Testing**: Must match C++ results exactly

## Next Steps

1. Implement helper functions (_clz32, _radix_sort, _ray_aabb_intersect)
2. Rewrite build() method with LBVH
3. Add ray_cast() method
4. Update check_all_collisions() with dual traversal
5. Port and verify all tests
