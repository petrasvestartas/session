# Session C++ to Python Port Plan

## Current Status

**Python Session** (~600 lines) has basic structure but is missing key C++ features:
- ✅ Basic structure (objects, lookup, tree, graph, bvh)  
- ✅ JSON serialization
- ✅ Some add_* methods
- ❌ **get_geometry()** - Tree transformation hierarchy (CRITICAL)
- ❌ Complete add_* methods for all geometry types
- ❌ Ray casting integration
- ❌ Collision detection

## Key Missing Feature: get_geometry()

### C++ Algorithm (lines 332-412)

```cpp
Objects Session::get_geometry() const {
  // 1. Deep copy all objects
  Objects transformed_objects = objects;
  
  // 2. Rebuild lookup from copied objects
  std::unordered_map<std::string, Geometry> transformed_lookup;
  
  // 3. Recursively transform nodes
  std::function<void(TreeNode*, const Xform&)> transform_node = 
    [&](TreeNode* node, const Xform& parent_xform) {
      // Get geometry from lookup
      Xform current_xform = parent_xform;
      
      if (geometry found) {
        // Accumulate: geom->xform = parent_xform * geom->xform
        geom->xform = parent_xform * geom->xform;
        current_xform = geom->xform;
      }
      
      // Recursively process children with accumulated xform
      for (child : node->children()) {
        transform_node(child, current_xform);
      }
    };
  
  // 4. Start from root with identity
  transform_node(tree.root(), Xform::identity());
  
  // 5. Apply transformations to coordinates
  for (auto& geom : all_geometries) {
    geom->transform();
  }
  
  return transformed_objects;
}
```

### Python Implementation Needed

```python
def get_geometry(self) -> Objects:
    """Get all geometry with transformations applied from tree hierarchy."""
    import copy
    from .xform import Xform
    
    # Deep copy
    transformed_objects = copy.deepcopy(self.objects)
    
    # Rebuild lookup
    transformed_lookup = {}
    for geom in all_geometry_lists:
        for g in geom:
            transformed_lookup[g.guid] = g
    
    # Recursive transform
    def transform_node(node, parent_xform):
        current_xform = parent_xform
        
        if node.name in transformed_lookup:
            geom = transformed_lookup[node.name]
            geom.xform = parent_xform * geom.xform
            current_xform = geom.xform
        
        for child in node.children:
            transform_node(child, current_xform)
    
    # Start from root
    if self.tree.root:
        transform_node(self.tree.root, Xform.identity())
    
    # Apply to coordinates
    for geom in all_geometries:
        geom.transform()
    
    return transformed_objects
```

## Test Requirements

### Tree Transformation Test (lines 124-253)

**Setup**:
1. Create 3 box meshes at same origin (0,0,0)
2. Setup tree hierarchy: box1 → box2 → box3
3. Apply transformations:
   - box1: rotate Z + plane-to-plane transform
   - box2: translate + rotate (relative to box1)
   - box3: translate (relative to box2)

**Expected Results**:
- box_1: 8 vertices at specific coords (e.g., 1.366, -0.366, 0)
- box_2: 8 vertices at different coords (accumulated transform)
- box_3: 8 vertices at further coords (double accumulated)
- All 6 faces preserved with correct topology

**Validation**:
- Each mesh has 8 vertices
- Each vertex matches expected coords (±1e-4)
- Each mesh has 6 faces with correct topology

## Implementation Steps

### Step 1: Add Missing Methods to Session
- [x] Basic structure exists
- [ ] `add_mesh()` method
- [ ] `add()` method for tree hierarchy
- [ ] Complete all other `add_*` methods

### Step 2: Implement get_geometry()
- [ ] Deep copy objects
- [ ] Rebuild transformed lookup
- [ ] Recursive transformation accumulation
- [ ] Apply transforms to coordinates
- [ ] Return transformed Objects

### Step 3: Port Tests
- [ ] Basic session tests (already exist)
- [ ] Tree transformation hierarchy test
- [ ] Verify exact coord matching

## Files to Modify

1. **session.py** - Add get_geometry() and missing add_* methods
2. **session_test.py** - Port tree transformation test

## Estimated Changes

- **session.py**: +100-150 lines (get_geometry() + missing methods)
- **session_test.py**: +150-200 lines (tree transformation test)

## Dependencies

All required classes exist:
- ✅ Objects
- ✅ Tree/TreeNode  
- ✅ Mesh
- ✅ Xform (with plane_to_plane, rotation_z, translation)
- ✅ Point, Vector

## Next Action

Implement `get_geometry()` method and tree transformation test.
