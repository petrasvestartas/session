# C++ Class Definitions - Source File Locations

All C++ classes are defined in header files (`.h`) in `session_cpp/src/` directory.

## Complete List of Classes and Their Files

| # | Class/Namespace | Header File | Doxygen Status |
|---|-----------------|-------------|----------------|
| 1 | **Arrow** | `src/arrow.h` | ✅ Documented |
| 2 | **BoundingBox** | `src/boundingbox.h` | ✅ Documented (updated) |
| 3 | **BVH** | `src/bvh.h` | ✅ Documented (updated) |
| 4 | **BVHNode** | `src/bvh.h` | ✅ Documented (updated) |
| 5 | **Color** | `src/color.h` | ✅ Documented |
| 6 | **Cylinder** | `src/cylinder.h` | ✅ Documented |
| 7 | **Edge** | `src/edge.h` | ✅ Documented |
| 8 | **Graph** | `src/graph.h` | ✅ Documented |
| 9 | **Line** | `src/line.h` | ✅ Documented |
| 10 | **Mesh** | `src/mesh.h` | ✅ Documented |
| 11 | **Objects** | `src/objects.h` | ✅ Documented |
| 12 | **Plane** | `src/plane.h` | ✅ Documented |
| 13 | **Point** | `src/point.h` | ✅ Documented |
| 14 | **PointCloud** | `src/pointcloud.h` | ✅ Documented |
| 15 | **Polyline** | `src/polyline.h` | ✅ Documented |
| 16 | **Quaternion** | `src/quaternion.h` | ✅ Documented |
| 17 | **Session** | `src/session.h` | ✅ Documented |
| 18 | **Tolerance** | `src/tolerance.h` | ✅ Documented |
| 19 | **Tree** | `src/tree.h` | ✅ Documented |
| 20 | **TreeNode** | `src/treenode.h` | ✅ Documented |
| 21 | **Vector** | `src/vector.h` | ✅ Documented |
| 22 | **Vertex** | `src/vertex.h` | ✅ Documented |
| 23 | **VertexData** | `src/mesh.h` | ✅ Documented |
| 24 | **Xform** | `src/xform.h` | ✅ Documented |
| 25 | **encoders** (namespace) | `src/encoders.h` | ✅ Documented |

## Doxygen Documentation Generation

### Current Generated Documentation Includes:

All 25 classes/namespaces are now properly documented in the generated HTML:

**Classes visible in `annotated.html`:**
- Arrow, BoundingBox, BVH, BVHNode, Color, Cylinder, Edge, Graph, Line, Mesh, Objects, Plane, Point, PointCloud, Polyline, Quaternion, Session, Tolerance, Tree, TreeNode, Vector, Vertex, VertexData, Xform

**Namespace:**
- `session_cpp::encoders` - Contains utility functions for JSON encoding/decoding

### Building Documentation

```bash
cd session_cpp/docs
doxygen Doxyfile
```

Output location: `session_cpp/docs_output/html/index.html`

### Doxygen Configuration

**Input directory:** `../src` (all `.h` and `.cpp` files)
**Recursive:** Yes
**Extract all:** Yes
**Extract private:** Yes
**Extract static:** Yes

### Recent Updates

**Added Doxygen comments to:**
1. `BoundingBox` - Added `@brief` description
2. `BVH` - Added `@brief` description  
3. `BVHNode` - Added `@brief` description

These were missing Doxygen comments and are now properly documented.

## Verification

To verify all classes are documented:

```bash
# List all generated class documentation files
ls session_cpp/docs_output/html/class*.html | wc -l
# Should show 25+ files (classes + their member pages)

# Check if specific classes are present
grep -l "BoundingBox\|BVH" session_cpp/docs_output/html/annotated.html
```

## Note on Encoders

The `encoders` namespace contains utility functions (not a class):
- `json_dump()` - Write JSON to file
- `json_load()` - Read JSON from file
- `json_dumps()` - Serialize to JSON string
- `json_loads()` - Deserialize from JSON string

It's documented in the source file view: `encoders_8h_source.html`

## Summary

✅ **All 23 main classes + 2 helper classes (BVHNode, VertexData) + 1 namespace = 26 total entities**  
✅ **All properly documented with Doxygen comments**  
✅ **All generated in HTML documentation**  
✅ **Documentation includes descriptions, member lists, and inheritance diagrams**

If you're seeing outdated documentation, rebuild with:
```bash
cd session_cpp/docs
rm -rf ../docs_output
doxygen Doxyfile
```
