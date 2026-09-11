# Docstring Style Guide (C++)

## Canonical style

```cpp
/// Brief description, one sentence.
/// param_name: description
/// Returns description.
void method(...);
```

- Use `///` single-line comments only — no `/** */` Doxygen blocks
- Multi-line: each line starts with `///`
- Forbidden tags: `@brief`, `@param`, `@return`, `@class`

## When to document params

Document a param only when its name alone is ambiguous:

```cpp
/// sort_by_bbox: if true, picks the largest polyline by bbox diagonal as boundary.
/// precision: optional tolerance for vertex merging.
/// edge_gap: if > 0, insets bottom wall vertices toward face center.
```

Skip docs for self-evident params:

```cpp
// NO: vertices, faces, x, y, z, position, cap
```

## When to document return value

Only when not obvious from the method name:

```cpp
/// Returns the vertex key.
/// Returns the face key, or nullopt if invalid.
/// Returns one LoftPanel per matched face pair, in centroid-distance order.
```

## Member variables — trailing inline style

```cpp
std::string name = "my_mesh";  ///< Mesh name
size_t max_vertex = 0;         ///< Next vertex key
```

Use `///<` (not `///`) for trailing inline docs. Keep as-is where already present.

## Section banners — leave unchanged

```cpp
///////////////////////////////////////////////////////////////////////////////////////////
// Constructors
///////////////////////////////////////////////////////////////////////////////////////////
```

## Examples

```cpp
/// Create a mesh from a list of polygons.
/// precision: optional tolerance for vertex merging.
static Mesh from_polylines(const std::vector<std::vector<Point>>& polygons, std::optional<double> precision = std::nullopt);

/// Add a vertex to the mesh.
/// vkey: optional explicit key.
/// Returns the vertex key.
size_t add_vertex(const Point& position, std::optional<size_t> vkey = std::nullopt);

/// Loft between matched pairs of top/bottom polygons, producing one panel per pair.
/// Each panel has a top cap, optional bottom cap, matched quad walls, and triangle fill.
/// merge_precision: vertex-merge tolerance.
/// edge_gap: if > 0, insets bottom wall vertices toward face center.
/// Returns one LoftPanel per matched face pair, in centroid-distance order.
static LoftResult loft_panels(...);
```
