# Place and update object rows

Move, recolour, hide and reveal one object while its neighbouring row stays unchanged.

This sixth cumulative checkpoint is a candidate. Its source-retention tests and wasm typecheck pass; 54 native tests pass with one GPU-dependent test ignored; desktop/upright browser interactions pass.

## Prerequisites and ownership

Keep frame, painting, mathematics, dock and camera unchanged. This subject owns the object-row layout, row table, bounds helpers and growing buffers. The pipeline cache, binding layouts and diagnostics are supporting dependencies shared with later rendering subjects; their implementation remains in place.

`ObjectRow` is convenient CPU input: a double-precision placement, bounds, tint, flags and optional extreme points. `Instance` is the shader-facing contract. `#[repr(C)]` fixes field order, and the `Pod` derive rejects fields that cannot safely be uploaded as bytes. The size assertion pins its 112-byte stride. Padding has a purpose: the fourth word after spacing carries the packed edge colour.

```rust
--8<-- "session_viewer/src/engine/gpu/instance/layout.rs:object-instance-layout"
```

## Build

Add the row modules and their inspector registration, preserving all earlier implementations.

From `session_tests`, materialize a fresh checkpoint:

```sh
node --import ./scripts/typescript.mjs scripts/course.ts completion-checkpoint rows ../session_viewer/target/learn-rows
```

### Ownership and allocation

`GpuCtx` holds cloneable device and queue handles plus a shared pipeline cache. A device creates resources; a queue copies bytes and submits commands. Cloning a handle does not create a second GPU. `GrowBuf` records row length, capacity, stride and permitted usage. Growth allocates more space, copies existing bytes, and replaces the buffer handle. Any bind group referencing the old buffer must then be rebuilt. Keeping capacity separate from length allows an empty scene to retain reusable allocation.

`InstanceTable::append` replaces the empty placeholder on the first upload. A widget row must remain last, so appending scene objects first removes that row and rebuilds the retained prefix. The table retains CPU rows because editing one field must preserve every other field. `write_at` uploads only the changed row; chunked writes keep individual transfers bounded. A geometry revision lets downstream caches distinguish a real geometry or visibility change from unchanged state.

The binding layout describes which buffers shaders may read, their visibility and storage type; the bind group supplies actual buffer handles. Pipeline descriptions are cached and lazily compiled, so constructing row storage does not compile every scene shader. Timing diagnostics report through the engine callback without requiring application panels. Rendering later explains shader composition and target-specific pipeline keys using this retained implementation.

### Placement and bounds

Object matrices store rotation and scale as floats. Translation remains double precision on the CPU and is uploaded relative to a scene anchor, avoiding precision loss far from the world origin. A placement change updates the row and its translation, then recomputes its world bounds. If extreme points are available, transforming those points gives a tighter rotated box than transforming the corners of an already enlarged bounding box.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/table.rs:object-placement"
```

Reanchoring is distance-limited and throttled. Moving the camera a tiny amount should not rebuild every translation; moving it far enough must eventually do so. The table tracks clipping and closed-solid rows separately so those passes do not scan unrelated objects. Retiring marks rows hidden and dead; burying also removes their spatial contribution. A stable row number lets geometry lanes reference an object while its placement changes.

### Independent effects

Colour overrides set a flag and update the tint without changing geometry. Clearing an override clears the flag and resets the multiplier to white, allowing the geometry's own colour to show. Hidden rows remain allocated. Non-pickable rows are visible decorative geometry whose flag tells the picking pass to reject them; hiding and non-pickability are independent.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/table.rs:object-reveal"
```

Reveal clamps the line fraction to zero through one and point scale to nonnegative values. Geometry lanes interpret these values later. This inspector displays reveal as rectangle width to make the stored value observable; it does not claim to render a trimmed curve or enforce picking.

```rust
--8<-- "session_viewer/examples/course/rows.rs:course-rows"
```

## Run and observe

Use the shared-cache browser procedure in [Maintaining](../maintaining.md), then open `?tutorial=rows`. Two rectangles appear: pink row zero and blue non-pickable row one. Left/right arrows move only row zero by twenty scene units. H hides and restores it; R halves and restores its width. C toggles its colour override. The visible view reads the real table's CPU mirror while the same edits write its GPU buffers.

The inspector remains legible at 390 pixels wide by scaling the scene. It is a keyboard experiment, not touch-device acceptance. GPU lane rendering, shader reveal and pick rejection require their later checkpoints.

## Predict and try

Press R, move right twice, then press H twice. Predict which values survive hiding. The rectangle returns at its moved position with half width: visibility changes one flag, leaving placement and reveal intact. Row one remains stationary throughout.

## Question and answer

Why rebuild a bind group after buffer growth if the rows still have the same meaning?

A bind group holds a particular GPU buffer handle. Growth replaces that allocation; retaining the old bind group would keep shaders reading the old storage even though the CPU table and new buffer contain the correct rows.

## Validation

The six-subject retention suite passes three tests and the independent rows wasm checkpoint compiles. Full production native/default/tutorial wasm typechecks pass after the adapter extraction. The native suite passes 54 tests with one GPU-dependent test ignored. Browser checks pass 70 captures across all six modes, including exact row round trips. Whole-course acceptance remains pending; source equality and CPU inspection cannot substitute for them.
