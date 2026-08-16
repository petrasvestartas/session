# Instancing plan — geometry once, placement is a transform

**Decision (locked):** the definition + transform representation lives in the **kernel/proto** (C++ ground
truth → Rust → Python). This is the only way a `.pb`/`.json` file can carry *vertex-sharing* instancing;
viewer-only dedup can't, because kernel geometry is baked in **world** coordinates and identical objects at
different places have different vertex data. Follows the Rhino block model: a **definition table** of local
geometry + placed **references**.

## The problem this solves

True instancing = one copy of a mesh's vertices, shared by N placements. That requires geometry stored
**local** and each placement expressed as an `Xform`. Today `Session` is a flat list of world-baked
`Geometry`, so the concept must be added.

## Kernel data model

New class **`InstanceRef`** (a block reference):

```
InstanceRef {
    guid: String                 // own identity
    definition_guid: String      // which definition it places
    xform: Xform                 // THE transformation — the only per-instance data
    color: Color                 // optional per-instance override (else definition color)
    flags: u32                   // reserved: selection / cull / visibility
}
```

`Session` gains a **definitions** table (mirrors Rhino `InstanceDefinitionTable`):

```
Session {
    objects:     [...]           // drawn directly; may include InstanceRef entries
    definitions: [Geometry]      // LOCAL-coord geometry, never drawn on its own; targeted by definition_guid
}
```

- A definition = local geometry stored **once**, authored at origin.
- An `InstanceRef` in `objects` = "place definition D by Xform X." 1000 refs → 1000 rows, one vertex copy.

## Proto + 3-language parity

- New `InstanceRef` message: `guid`, `definition_guid`, reuse existing **Xform** proto message, `color`, `flags`.
- `Session` message: add `repeated definitions`.
- **C++ is ground truth** — author there first, port identically to Rust + Python (same API, names, tests,
  line counts). `/new-class InstanceRef` checklist: ctor/`[]`/`==`/`!=`/str/repr group, `to_proto`/`from_proto`,
  `file_json_dump`/`file_json_load`, JSON fields alphabetical across all three languages.

## Viewer consumption (CPU→GPU) — everything drawn is a `(definition, instance)`

| Source object | Definition | Instance rows |
|---|---|---|
| `InstanceRef` | `definitions[definition_guid]`, flattened to the arena **once** | one row: `model = xform`, its `color`/`flags` |
| Regular object (world coords) | its own geometry (auto-wrapped, uploaded once) | one row: `model = identity` |

Load flow:
1. Flatten each `definitions[i]` into an arena range once → `def_by_guid: guid → def_id`.
2. Walk `objects`: `InstanceRef` → row under `def_id`; regular object → auto-definition + one identity row.
3. Group rows by `def_id`; upload instance storage buffer with `model = (xform.translation −= camera_origin).to_f32()`
   (f64 subtract THEN cast — lesson 33 camera-relative).
4. Draw loop: bind arena once, one `draw_indexed(def.range, def.base_vertex, def.rows)` per definition.

### GPU side
- Shader = lesson-29 path: `instances[@builtin(instance_index)]`. No per-vertex id buffer (drop `@location(3)` —
  it can't share vertices).
- Draw count = number of **distinct definitions**, not objects. 76 folds the loop into one indirect call.
- `flags` bit → selection (50) / culled (41); vs collapses culled rows. Geometry never re-uploaded for these.

## Caveats
- **Non-uniform-scale normals**: `(model * vec4(n,0)).xyz` is correct only for rigid/uniform transforms. Add a
  `normal_matrix` (inverse-transpose) to the row if instances scale non-uniformly. v1: rigid/uniform, noted.
- **Nested definitions** (a definition containing an `InstanceRef`): compose Xforms on load, flatten one level for v1.
- **Static vs dynamic rows**: definitions + most rows upload once; a small dynamic range gets `write_buffer` per frame.

## Work sequence
1. **Kernel/proto** (before viewer lesson 34): `InstanceRef` ×3 langs + proto + `Session.definitions` + minitests.
2. **Viewer 30/35**: `(definition, instance)` split, arena range-draws, instance rows.
3. **Viewer 34**: loader maps `definitions`→arena, `InstanceRef`→rows.
4. **38 reconcile**: guid diff frees arena ranges / patches rows; definition content-hash gates re-flatten.

All lessons stay **viewer-only**; the kernel gains `InstanceRef` as a normal geometry class.
