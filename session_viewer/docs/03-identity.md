# 03 · Keep source identity while batching drawing

**Start:** checkpoint 02. **Finish:** two styled instances share the rendering machinery and retain distinct identities.

## An object is more than its triangles

A source object has an identity, placement, style and bounds. Its display can contain many triangles or segments. A GPU row is an efficient address within the current scene revision; it is not a persistent CAD identifier.

![Source, display preparation and GPU resources have separate owners.](illustrations/ownership.svg)

Read the instance record as the contract between source objects and drawing. Positions can share a geometry buffer while an object row supplies the model transform, color and flags. That allows selection and hiding to update a small row without regenerating a mesh.

`struct` groups named fields. `impl` defines operations on that type. `Vec<T>` owns a growable sequence of `T`; a slice such as `&[T]` borrows a sequence without taking ownership. Passing grouped records makes call sites easier to understand than passing many unrelated scalars.

## Track both directions

Preparation maps source identity to a row and appends display data. Picking will later return that row, and Scene must map it back to the original document/object. Multiple placed instances of one source document still need distinct instance addresses.

Do not search for an object by approximately matching triangle positions. Coincident objects can have different identities; one object can have many tessellation vertices. Preserve explicit source maps as data is produced.

## Rust memory is not automatically WGSL memory

`#[repr(C)]` makes Rust field layout predictable, but WGSL has its own alignment rules. A three-component vector can occupy sixteen-byte aligned space in a shader record. Read field sizes, padding, array stride and binding size together. The final layout tests validate offsets as well as total record size.

Flags are bit fields: selecting an object sets a bit while preserving its other flags. Later visibility caching must distinguish a hidden-state change from a color/selection change, because only the former changes physical occluders.

## Write the files

Follow [Complete file changes for 03](../lessons/03/index.md). Add the instance contract and source-to-row mapping, then wire the uploads and drawing. Read the complete shader bindings alongside their Rust layouts.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

At <http://localhost:8780/?data=off&inspect=1>, expect two independently styled objects. Orbit and zoom: their relative placement remains fixed while the camera moves. The checkpoint inspection reports two objects.

A wrong stride commonly makes the first object correct and later objects corrupt. A wrong source map can look visually correct until selection returns another object. Both are representation bugs, so preserve the explicit mapping now.

**Before continuing:** explain why a row can change after scene replacement while a source GUID remains stable. Continue to [drawing modules](04-modules.md).
