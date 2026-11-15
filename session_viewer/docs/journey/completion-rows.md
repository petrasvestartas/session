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

### Shader build inputs

Cargo runs the retained production `build.rs` before compiling this checkpoint. It expands each shader's include directives once per shader, strips line comments and redundant whitespace, and writes generated WGSL beneath Cargo's own `OUT_DIR`. The `shader!` macro reads those generated files. Include `projected_triangle.wgsl` alongside the five pipeline helpers: ink visibility includes its layout. Supplying another checkout's generated shaders would conceal a missing dependency, so validation must also build with no inherited `OUT_DIR`.

### Ownership and allocation

`GpuCtx` holds cloneable device and queue handles plus a shared pipeline cache. A device creates resources; a queue copies bytes and submits commands. Cloning a handle does not create a second GPU. `GrowBuf` records row length, capacity, stride and permitted usage. Growth allocates more space, copies existing bytes, and replaces the buffer handle. Any bind group referencing the old buffer must then be rebuilt. Keeping capacity separate from length allows an empty scene to retain reusable allocation.

`buffers.rs` separates three operations. `append` grows by roughly one half when necessary and returns whether the handle changed. `reserve` can allocate room for a known batch plus its expected remainder. `fill` sends repeated rows in chunks bounded by `WRITE_CHUNK`, instead of allocating a second CPU array as large as the entire scene. The `Pod` bound permits byte uploads of the row type; its byte size must still match the buffer stride.

The device has both a maximum allocation size and, for storage buffers, a maximum binding size. `fits` checks the stricter applicable limit before admitting more rows. The growth helpers deliberately retain a requested size larger than that limit: they do not silently truncate a scene. The caller must reject that request. Predict what a request for 1,200 rows does on a 1,000-row limit: the helper returns 1,200, so omitting the admission check produces an invalid allocation rather than a partial successful upload.

`reset` keeps the allocation and forgets the logical rows. `release` destroys the old allocation and replaces it with one placeholder row. `packed` instead records GPU copies of selected ranges into one contiguous replacement; its caller must submit those commands and update dependent bindings. These operations distinguish an empty document, memory release and compaction. They must not be used interchangeably merely because all three can reduce the visible row count.

`Template` uploads a small immutable vertex/index mesh once and binds it for repeated instances. `uniform_buffer` initializes a single shader-readable value, while `resource_group` associates concrete resources with explicit binding numbers. Rust's const-generic array length keeps that entry list on the stack; the shader layout still determines which binding numbers and resource types are valid.

`InstanceTable::append` replaces the empty placeholder on the first upload. A widget row must remain last, so appending scene objects first removes that row and rebuilds the retained prefix. The table retains CPU rows because editing one field must preserve every other field. `write_at` uploads only the changed row; chunked writes keep individual transfers bounded. A geometry revision lets downstream caches distinguish a real geometry or visibility change from unchanged state.

The binding layout describes which buffers shaders may read, their visibility and storage type; the bind group supplies actual buffer handles. Pipeline descriptions are cached and lazily compiled, so constructing row storage does not compile every scene shader. Timing diagnostics report through the engine callback without requiring application panels. Rendering later explains shader composition and target-specific pipeline keys using this retained implementation.

### Describe bindings before supplying resources

`pipelines/bindings.rs` builds individual layout entries. A buffer entry declares its shader stages and uniform or storage access, with no dynamic offset and no declared minimum size. A texture entry declares a two-dimensional view, sample type and whether it is multisampled. These declarations do not allocate resources. A depth texture and an unsigned integer identity texture can have the same dimensions while requiring different binding types.

`pipelines/layouts.rs` combines those entries into the renderer's shared contracts. The line group supplies pen/view settings and clipping planes. The object group supplies row data and relative translations. The ink variant additionally declares single-sample and multisampled depth/identity textures, projected triangles and tile lists. A shader's group and binding numbers must match this contract; reordering a Rust array without preserving its explicit binding numbers changes which resource a shader reads.

### Compile a shared description on first use

`pipelines/core.rs::Lazy<T>` owns an `Rc<LazyCell<T, ...>>`. `Rc` lets several owners share one value on the current thread. `LazyCell` calls its stored `FnOnce` constructor only when dereferenced; `FnOnce` permits consuming the captured pipeline description. Cloning the wrapper shares that cell instead of recompiling. Equality and hashing compare the shared allocation's identity, so two references to one lazy shader are the same key even before compilation.

`Cache` keeps shader, layout and pipeline maps behind `RefCell`. This permits mutation through a shared cache reference, with borrowing checked at runtime. A map borrow must end before another operation needs an incompatible borrow. `Cell` holds small copied counters and the clipping switch without exposing a reference to their contents. These types solve ownership within one thread; they do not make the cache safe to share freely across threads.

The pipeline key retains entry points, bind-group layouts, vertex layouts, primitive topology, color/depth choices, scene sampling, clipping specialization and the target's format/sample count. Returning to an existing combination reuses its cached pipeline. A new target requires a different key even if the source geometry is unchanged. `compile_all` forces every requested lazy pipeline; constructing a cache or registering a lane alone does not do that work. Predict whether cloning a lazy shader increments compilation counters: it does not, because the constructor has not been forced again.

### Placement and bounds

Object matrices store rotation and scale as floats. Translation remains double precision on the CPU and is uploaded relative to a scene anchor, avoiding precision loss far from the world origin. A placement change updates the row and its translation, then recomputes its world bounds. If extreme points are available, transforming those points gives a tighter rotated box than transforming the corners of an already enlarged bounding box.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/table.rs:object-placement"
```

Reanchoring is distance-limited and throttled. Moving the camera a tiny amount should not rebuild every translation; moving it far enough must eventually do so. The table tracks clipping and closed-solid rows separately so those passes do not scan unrelated objects. Retiring marks rows hidden and dead; burying also removes their spatial contribution. A stable row number lets geometry lanes reference an object while its placement changes.

`hull/geometry.rs::Points` views packed `Pod` rows as float words with an explicit stride and point offsets. It avoids making a second full point array merely to find extremes. `extreme_points` normalizes coordinates around the box centre before signed-volume tests, so its tolerance is relative to the geometry's scale. Sets with at most eight points are retained directly; degenerate sets have dedicated reductions. If more than 8,192 extremes would be retained, it returns `None` and the caller uses the source box. That fallback bounds memory; it is not an empty geometric object.

For an affine placement, each output coordinate is a linear function of the input coordinates. Its extrema occur on convex-hull vertices, which explains why `placed_box` can transform the retained extremes instead of every original point. Rotating a loose axis-aligned source box generally produces a looser result. Predict the difference for a long diagonal line rotated onto one axis: transforming its two endpoints gives a tight line bound, while transforming its enclosing box leaves unnecessary width.

`hull.rs`, `instance.rs`, `objects.rs` and `pipelines/mod.rs` expose narrow module interfaces. The rows checkpoint re-exports hull geometry, the instance layout, the object table and pipeline layouts/cache. Rendering later registers hull upload, GPU validation, ink/lane code and vertex-layout construction through these same indices. A `pub use` makes an existing type available to a caller; it neither duplicates that type nor creates another buffer owner.

### Shared shader records and coordinate rules

`scene.wgsl` declares the shader side of the same contract: the camera matrix in group zero, line/clipping uniforms in group one, and object rows plus separate translations in group two. Its `Instance` is 112 bytes and its `LineUniform` is 80 bytes. `place` applies the row's model matrix before adding the scene-relative translation. The non-pickable flag is independent of colour and visibility; selected colour, layer colour and edge colour have separate rules. Field order and padding must agree with the Rust uploads, not merely have similarly spelled names.

`normals.wgsl` uses cofactors to apply the inverse-transpose direction transform. A normal cannot be transformed like a point under nonuniform scale. The determinant sign preserves orientation under reflection; a degenerate matrix or zero input normal yields zero instead of normalizing an invalid vector. Octahedral decoding unfolds two packed components into a unit direction; the 32-bit path reserves `0x80008000` for an absent normal. That sentinel must remain distinct from an ordinary direction.

`physical.wgsl` separates colour output from object/subobject identity and carries a triangle identity alongside both. Triangle IDs are split into low and high 16-bit halves because the selected four-sample target format cannot store the required 32-bit channel directly. Zero represents no triangle; callers use a nonzero encoded identity for geometry. This storage choice must agree between writing a fragment and interpreting a visibility sample.

`projected_triangle.wgsl` stores edge equations, a depth reference and gradient, screen bounds and a corner count in a 96-byte record. A clipped triangle may have four corners. Coverage first rejects empty records and points outside the bounds, then checks signed edge distances with a 1/256-pixel allowance for rasterizer snapping. Depth comes from the reference plus the gradient at that pixel. Tile span doubles until the canvas uses at most 262,144 tiles; the Rust tile layout must make the same choice.

`clip.wgsl` declares six world and screen clipping planes, hatch transforms and sample settings. A clipping-plane object is exempt from its own cutting test. Surface distance subtracts a scale-aware slack so a coplanar face yields consistently to its section cap; ink uses a separate dot-product rounding allowance to avoid dropping a line on the plane. `CLIPPING` can specialize unused tests away. This is the common data and predicate contract; the later visibility chapter connects it to crossing counts, cap rendering and sampled depth.

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

The retention and shader-include suite passes four tests. The independent rows wasm checkpoint compiles with its own build script, without an inherited shader output directory. Full production native/default/tutorial wasm typechecks pass after the adapter extraction. The native suite passes 54 tests with one GPU-dependent test ignored. Browser checks pass 70 captures across all six modes, including exact row round trips. Whole-course acceptance remains pending; source equality and CPU inspection cannot substitute for them.
