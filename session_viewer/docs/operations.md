# Shared Operation API (`viewer-core::op`)

Every automation surface — the in-app command line, `session-viewer-cli`, browser `execute_op(json)`, and the Python `session_mcp` FastMCP server — dispatches typed JSON operations through `viewer_core::op::execute(&mut dyn CommandHost, Op) -> Result<Reply, OpError>`.

Objects are always addressed by stable `(doc, guid)` pairs (`ObjectRef`), never by volatile GPU row or UI tree indices.

## Request & Response Envelope

```json
{"id": "req-1", "op": "object_create", "kind": {"type": "box", "origin": [0, 0, 0], "dx": 100, "dy": 50, "dz": 30}}
```

```json
{"id": "req-1", "ok": true, "reply": {"message": "Created box", "created": [{"doc": 0, "guid": "..."}], "changed": [], "value": null}}
```

## Operations

| `op` | Category | Summary | Example JSON |
|---|---|---|---|
| `object_create` | Create & Edit | Create a kernel geometry or scene text object on the current or specified layer. | `{"op":"object_create","kind":{"type":"box","origin":[0,0,0],"dx":100,"dy":50,"dz":30}}` |
| `object_delete` | Create & Edit | Delete the selected objects or explicit (doc, guid) targets in one undo step. | `{"op":"object_delete","targets":"selection"}` |
| `object_transform` | Create & Edit | Translate, rotate (with explicit pivot), scale (with explicit pivot), orient3, or apply a 4x4 matrix. | `{"op":"object_transform","xform":{"type":"rotate","axis":"z","degrees":45.0,"pivot":[0,0,0]}}` |
| `object_copy` | Create & Edit | Duplicate target objects transformed by `xform` in one undo step. | `{"op":"object_copy","xform":{"type":"translate","delta":[200,0,0]}}` |
| `object_explode` | Create & Edit | Explode polylines into lines, BReps/meshes into faces, or point clouds into points. | `{"op":"object_explode","targets":"selection"}` |
| `object_project` | Create & Edit | Project target objects onto XY, YZ, ZX, CPlane, 3-point, or normal plane. | `{"op":"object_project","plane":{"type":"xy","z":0.0}}` |
| `curve_interval` | Create & Edit | Trim (`0 <= t0 < t1 <= 1`) or extend (`extend: true`, `t0 <= 0, t1 >= 1`) a curve. | `{"op":"curve_interval","target":{"doc":0,"guid":"..."},"t0":0.2,"t1":0.8}` |
| `curve_arrowhead` | Create & Edit | Set curve arrowheads (`none`, `start`, `end`, `both`) on target curves. | `{"op":"curve_arrowhead","head":"end"}` |
| `selection_set` | Selection | Replace, add to, or remove from the current selection by `(doc, guid)` refs. | `{"op":"selection_set","targets":{"refs":[{"doc":0,"guid":"..."}]},"mode":"replace"}` |
| `selection_query` | Selection | Select objects matching `all`, `name`, `smaller_than`, `in_box`, or `kind`. | `{"op":"selection_query","by":{"type":"all"}}` |
| `selection_clear` | Selection | Clear the current selection. | `{"op":"selection_clear"}` |
| `layer_list` | Layers & Appearance | List all document layers and indicate which layer is current. | `{"op":"layer_list"}` |
| `layer_create` | Layers & Appearance | Create a new layer or sublayer in a document. | `{"op":"layer_create","name":"Beams"}` |
| `layer_rename` | Layers & Appearance | Rename an existing layer addressed by `(doc, name)`. | `{"op":"layer_rename","layer":{"doc":0,"name":"Beams"},"name":"Columns"}` |
| `layer_delete` | Layers & Appearance | Delete a non-root, non-current layer and its objects. | `{"op":"layer_delete","layer":{"doc":0,"name":"Columns"}}` |
| `layer_set_current` | Layers & Appearance | Set the active layer where newly created objects are placed. | `{"op":"layer_set_current","layer":{"doc":0,"name":"Beams"}}` |
| `layer_assign` | Layers & Appearance | Move target objects onto a destination layer, preserving world placement. | `{"op":"layer_assign","layer":{"doc":0,"name":"Beams"}}` |
| `layer_set` | Layers & Appearance | Batch-update visibility, lock state, or face colour for all members of a layer. | `{"op":"layer_set","layer":{"doc":0,"name":"Beams"},"visible":true,"color":[220,80,40]}` |
| `group_add` | Layers & Appearance | Group target objects of one document under a new named group node. | `{"op":"group_add","name":"Truss_01"}` |
| `graph_add_edge` | Layers & Appearance | Add an undirected edge between two objects in their document's session graph. | `{"op":"graph_add_edge","a":{"doc":0,"guid":"..."},"b":{"doc":0,"guid":"..."}}` |
| `visibility_set` | Layers & Appearance | Hide (`visible: false`) or show (`visible: true`) target objects (undoable). | `{"op":"visibility_set","visible":false}` |
| `show_all` | Layers & Appearance | Unhide every hidden object across all open documents (undoable). | `{"op":"show_all"}` |
| `lock_set` | Layers & Appearance | Lock or unlock target objects (undoable). | `{"op":"lock_set","locked":true}` |
| `unlock_all` | Layers & Appearance | Unlock every locked object across all open documents (undoable). | `{"op":"unlock_all"}` |
| `color_set` | Layers & Appearance | Set or clear face (`edge: false`) or edge (`edge: true`) RGB override (undoable). | `{"op":"color_set","edge":false,"color":[240,90,40]}` |
| `alpha_set` | Layers & Appearance | Set (`0.0..=1.0`) or clear (`null`) per-object alpha transparency override (undoable). | `{"op":"alpha_set","alpha":0.4}` |
| `scene_inspect` | Query & Measure | Return document counts, object/row counts, undo depth, bounds, camera, and view state. | `{"op":"scene_inspect"}` |
| `object_list` | Query & Measure | List scene objects with stable `(doc, guid)` refs, kinds, layers, colours, and bounds. | `{"op":"object_list"}` |
| `object_get` | Query & Measure | Inspect a single object by `(doc, guid)`. | `{"op":"object_get","target":{"doc":0,"guid":"..."}}` |
| `measure` | Query & Measure | Measure `length`, `area`, `volume`, `bounds_extent`, or point-to-point `distance`. | `{"op":"measure","what":{"type":"volume"}}` |
| `scene_open` | Document & History | Open a `.session` archive from disk, restoring documents, camera, and display settings. | `{"op":"scene_open","path":"model.session"}` |
| `scene_save` | Document & History | Save all documents, appearance overrides, camera, and view state to a `.session` archive. | `{"op":"scene_save","path":"model.session"}` |
| `scene_load_manifest` | Document & History | Clear the scene and load a `.yaml`/`.json`/`.pb` scene manifest or protobuf file. | `{"op":"scene_load_manifest","path":"assets/view_local.yaml"}` |
| `scene_import` | Document & History | Import a `.pb`, `.obj`, or `.session` file into the scene. | `{"op":"scene_import","path":"mesh.obj"}` |
| `scene_export_obj` | Document & History | Export selected or all mesh/BRep/surface/element geometry as Wavefront `.obj`. | `{"op":"scene_export_obj","path":"out.obj"}` |
| `history_undo` | Document & History | Undo the most recent geometry, layer, text, or appearance transaction. | `{"op":"history_undo"}` |
| `history_redo` | Document & History | Redo the most recently undone transaction. | `{"op":"history_redo"}` |
| `camera_get` | Camera, View & Clipping | Return camera position, target, distance, quaternion orientation, and projection. | `{"op":"camera_get"}` |
| `camera_set_view` | Camera, View & Clipping | Snap camera orientation to `top`, `bottom`, `front`, `back`, `left`, `right`, `iso`, or `isoback`. | `{"op":"camera_set_view","view":"iso"}` |
| `camera_set_pose` | Camera, View & Clipping | Set explicit camera `target`, `position` (in scene units), and optional `perspective`. | `{"op":"camera_set_pose","target":[0,0,0],"position":[500,-500,400],"perspective":true}` |
| `camera_fit` | Camera, View & Clipping | Fit camera framing to target objects, current selection, or the whole scene. | `{"op":"camera_fit"}` |
| `view_get` | Camera, View & Clipping | Return all active display flags (`arctic`, `wireframe`, `opacity`, `cull_backfaces`, etc.). | `{"op":"view_get"}` |
| `view_set` | Camera, View & Clipping | Patch one or more viewport display flags or sizes. | `{"op":"view_set","patch":{"arctic":true,"opacity":0.95}}` |
| `construction_plane_set` | Camera, View & Clipping | Set the construction plane (`xy`, `yz`, `zx`, `points`, `normal`, or `null` for view-facing). | `{"op":"construction_plane_set","plane":{"type":"xy","z":0.0}}` |
| `clipping_plane_add` | Camera, View & Clipping | Create a section clipping plane on `xy`, `yz`, `zx`, `points`, or `normal`. | `{"op":"clipping_plane_add","plane":{"type":"xy","z":120.0}}` |
| `clipping_set` | Camera, View & Clipping | Enable/disable clipping planes, switch cap fill (`solid_fill`), or flip selected planes. | `{"op":"clipping_set","enabled":true,"solid_fill":false}` |
| `render_frame` | Camera, View & Clipping | Render the current scene, camera, and view state to a `.ppm` image on native GPU hosts. | `{"op":"render_frame","width":900,"height":700,"path":"frame.ppm"}` |
| `perf_stats` | Diagnostics & Escape Hatch | Return rolling frame timing percentiles, CPU split, redraw causes, or headless counts. | `{"op":"perf_stats"}` |
| `command_run` | Diagnostics & Escape Hatch | Run any command-dock text line directly (`Box 0,0,0 10 20 30`, `View Wireframe`, etc.). | `{"op":"command_run","line":"Select All"}` |
