# Decide which ink is visible

Render object edges over solid surfaces while hiding strokes behind the geometry and retaining stable object identities.

## Prerequisites and ownership

This is a chapter of the existing scene-rendering subject, using its `geometry` checkpoint. Complete camera projection, object rows and the rendering overview first. It teaches the lane/pass contracts, triangle projection and tile lists, shared ink visibility, clipping caps, ambient shading and surface outlines. Application input and document edits consume these completed renderer components later.

## Build

Read the registered lane lifecycle, then follow projected triangles from counting through list construction to an ink fragment.

### Give every lane the same resource contract

```rust
--8<-- "session_viewer/src/engine/gpu/lane.rs:render-lane"
```

`lane.rs` separates resetting row counts from releasing allocations. Retargeting changes color format or sample count and may rebuild pipelines. Appending takes this lane's typed data from `Upload`; drawing receives the current shared bindings and view. `bytes` accounts for buffers and textures, so a feature cannot disappear from resource reporting merely because it draws through a trait object.

`RowLane` adds overwrite and retirement operations for edited objects. Retirement assigns old rows to a hidden sink instead of immediately compacting all later offsets. Its `Registered` descriptor supplies construction, row counting, upload merging and byte stride. Vectors, planes and matrices use this registry; the common host does not need a new branch for each append or reset.

`pass.rs` supplies stage hooks for work affecting several lanes, such as clipping or ambient shading. `render.rs` orders those hooks around solid surfaces, depth-dependent effects and ink. `frame.rs` defines the common view, line and clipping uniforms plus binding bundles. `targets.rs` owns the attachments and their formats/sample counts; `view.rs` holds rendering choices. These are different responsibilities: changing a view option does not necessarily require reallocating an attachment.

### Reuse geometry without reusing identity

`arena.rs` and `faces.rs` keep shared vertex/index storage and face drawing. `instance.rs`, `instanced.rs` and `slots.rs` connect reusable definitions to placed object rows. A slot carries an object row and a triangle-ID base. Slot zero means an ordinary draw keeps each row's own owner; instanced draws substitute their placed owner.

`slot_table.wgsl` reads that mapping, and `scene.wgsl` supplies shared object state. A triangle's ID must remain unique across both arena geometry and instances. Otherwise two identical-looking instances could resolve to the same object when picked or tested for visibility. Sharing vertices saves memory; it does not justify sharing selection identity.

### Project once, then restrict each visibility query

`project_triangles.wgsl` transforms triangles into screen-space records described by `projected_triangle.wgsl`. Each record carries clipped coverage, edge equations, bounds and depth information. `triangle_tiles.rs` manages those buffers and binds the current geometry, object transforms and slot table. If an allocation moves, it rebuilds the binding that holds the old handle.

The screen is divided into tiles. Tile width grows when necessary to bound total tile count. `triangle_tiles.wgsl` draws a four-corner rectangle over the tiles touched by a triangle's bounds, then rejects tiles outside its edge equations. This uses rasterization to find relevant tiles rather than testing every triangle for every ink pixel.

There are three logical stages: count each tile's triangle references, compute disjoint ranges in a shared reference pool, then fill those ranges. A tile record retains its count, list start, write cursor and overflow flag. References carry triangle identity and depth information.

```wgsl
--8<-- "session_viewer/src/shaders/scan_triangle_tiles.wgsl:visibility-prefix"
```

The exclusive prefix sum gives each tile the sum of earlier counts, which becomes its list offset. Workgroup memory lets 256 invocations cooperate; barriers ensure that one iteration reads a completed prior iteration. Further scan passes combine block totals and add those offsets to each tile. Counts saturate at capacity instead of wrapping into apparently valid small offsets.

### Treat capacity as asynchronous state

The initial reference pool is bounded. A GPU report copies required capacity to a mapped readback buffer, and a later frame polls it. `next_pool_words` can grow the pool toward its ceiling; projected-record capacity has its own checked growth. The renderer cannot assume that a report still in flight says there was no overflow.

Invalidation clears the leading record so shaders do not interpret old lists as current. While lists are unavailable, ink uses the defined fitted-plane path. Release shrinks allocations back to placeholders and clears cached projection facts. Capacity, valid contents and pending readback are separate states even when they refer to one buffer.

### Compare ink with the right surface

`triangle.wgsl` shares transformed vertices across face color, source-face picking, selection masks and clipping counts. Mirrored object transforms reverse the interpretation of raster front-facing state. A supplied normal is normalized; missing normals fall back to screen derivatives. Closed translucent solids drop back faces and use the global opacity, while open or printed surfaces follow their own rules. Color writes also carry primitive identity; clipped color uses a per-sample kept-side mask. Keeping these outputs together makes visible surfaces, selected faces and ink occlusion agree on the same placed geometry.

`face_coverage.wgsl` derives surface coverage from nonzero primitive IDs at each sample. One entry returns the fraction of covered samples; another writes full coverage only to the corresponding sample mask. Empty pixels discard. These are different outputs for different downstream passes: an averaged coverage value cannot replace an exact per-sample mask at a multisampled edge.

`triangle.wgsl` writes surface color, depth and primitive identity. `ink_visibility.wgsl` consumes the single-sample or multisampled versions of those attachments and the tile lists. It accounts for local depth slope and the geometric sag between a curved edge and its faceted surface; a fixed global depth offset would either hide legitimate edges or expose strokes through unrelated geometry.

`physical.wgsl` converts physical sizes and screen-space widths, while `normals.wgsl` supplies shared normal calculations. Ribbon, vector, plane, glyph and sphere shaders can reuse the same visibility contract. Their object row still controls visibility, selection, placement and reveal. A drawable lane and a surface used for occlusion are related but not interchangeable.

### A section cap needs depth and identity too

`clip/pipelines.rs` generates the cap shader prefix for the target sample count. Single-sample and multisample count/primitive textures require different WGSL declarations and load expressions; the same cap implementation follows that prefix. The primitive loader joins the two 16-bit ID halves. Cap picking uses a single-sample target with separate count and owner passes, then writes IDs for the section. Matching these declarations to the bindings is part of capping correctness, even when a one-sample screenshot happens to look right.

`clip.rs` represents a kept half-space as a normal and offset. It tests whether object bounds cross a clipping plane, including slack for GPU precision, and builds per-plane resources. `clip.wgsl` supplies shared clipping calculations. Solid crossings behind a plane determine where a section exists; an open surface must not be treated as a closed solid merely because it intersects the plane.

`cap.wgsl` draws the section's hatch/color, writes the plane's depth and a cap primitive marker, and supplies the covered sample mask. Other clipping planes still constrain that mask. Caps therefore participate in visibility and selection rather than being a flat color pasted over the final frame. Plane origin and hatch direction keep hatch coordinates tied to the authored section.

### Shade and outline the resulting surface coverage

`ssao.rs` owns depth-dependent ambient resources and sample-count-specific pipelines. Browser pipeline warming is scheduled after geometry can be presented; `ambient_warm.rs` handles that deferred ownership. The ambient depth/geometry/composite shaders and `ssao.wgsl` separate preparation, occlusion and composition instead of embedding all work in every face shader.

`surface_outline.rs` renders coverage masks, resolves multisampled coverage when needed, and builds coarse and dilated masks to skip empty regions. `face_coverage.wgsl` produces coverage; `surface_outline.wgsl` composites the border using a bounded offset table. Selection outlines and ordinary borders can then use the appropriate masks without confusing object IDs with coverage values.

### Reconstruct ambient shading from visible geometry

`Ssao` owns the reduced-resolution depth, radius, normal, filtered-occlusion and history images. Its cached key combines frame uniforms with geometry revision: an unchanged view and scene can reuse the result. Camera changes can reuse reprojected history only while the geometry revision agrees; changed geometry invalidates that assumption. Bindings are rebuilt when depth, primitive IDs, geometry buffers or the slot table change. Image dimensions and device scale also determine whether existing resources fit. The pass runs after faces, because it needs their actual depth and primitive identity.

`ssao/pipelines.rs` constructs the pipeline family for preparing depth, reducing it, finding occupied regions, estimating occlusion, filtering, storing history, upsampling and compositing. Layouts distinguish depth, integer identity/radius, sampled floating-point images and storage buffers. The target sample count controls multisampled scene bindings, while reduced working images remain single-sampled. Treat this as one resource contract: selecting a shader entry point with the wrong texture sample type is a validation failure, not a visual-quality setting.

`ambient_depth.wgsl` builds coarser depth levels by selecting the nearest positive linear depth from each two-by-two block and carrying that sample's radius with it. Empty blocks retain zero depth. Its occupancy pass searches a five-by-five neighbourhood for any geometry, expanding the active region enough for nearby sampling. Reducing depth and radius together prevents a coarse sample from combining one surface's location with another object's shading scale.

`ambient_geometry.wgsl` resolves the physical primitive ID through the instance slot table before reading triangle vertices and their placed owner. It derives a normal from transformed triangle edges, avoiding dependence on a possibly smoothed vertex normal. Pixels without a triangle reconstruct a normal from depth neighbours, choosing nearby samples to avoid crossing a discontinuity. The normal is then oriented toward the camera. This geometry lookup lets identical definition triangles shade differently under distinct instance transforms.

`ssao.wgsl` estimates screen-space occlusion by sampling horizons in several directions around the reconstructed surface point. Sample distance grows quadratically and selects coarser depth levels farther away. Object radius, bias and a finite screen radius bound the effect. Two directional filters use depth/normal agreement to avoid blurring across unrelated surfaces. Reprojected history is accepted only when its signed depth agrees, then clamped to the current neighbourhood before blending. This is an approximation using visible depth; it cannot infer arbitrary hidden geometry.

`ambient_composite.wgsl` overlays black with the computed occlusion as alpha. Its multisample edge path draws only listed sixteen-pixel tiles and unpacks one correction byte per sample. The output sample mask selects the corrected sample instead of darkening every sample in a partly covered pixel. This separates smooth interior shading from edge coverage and preserves the attachment's multisample identity.

`ambient_warm.rs` moves browser ambient-pipeline compilation outside the presented frame. A thread-local job is keyed by device and color format, with separate ready slots for one and four samples. It schedules work only while idle, avoids duplicate pending callbacks and compiles the remaining variant when the callback has time. `requestIdleCallback` falls back to a 50-millisecond timeout. Taking a ready pipeline checks the device/format again; an unavailable pipeline remains pending work rather than blocking scene startup.

## Run and observe

Materialize and run `geometry` using the command in the rendering overview. Orbit the two boxes and curved arrow with the arrow keys. Toggle O: solid face color disappears and back edges become visible. Toggle it again: surfaces regain their occlusion role. Toggle W independently; curve strokes and surface borders belong to different lanes.

This small fixture demonstrates the relation between faces and ink. It does not exercise every clipping, ambient or overflow case described here. Those features use the same completed source and need their dedicated native/browser witnesses before acceptance.

## Predict and try

Predict whether making the face color invisible should leave all back edges hidden in this checkpoint. Toggle O while looking through a box. Its face participation changes, so back edges become visible. Compare that with W, which affects the curve lane without removing solid-face depth. Identical-looking missing color can arise from different pipeline choices.

## Question and answer

Why build a prefix sum after counting triangle references?

Each tile needs a variable-length list in one bounded buffer. The prefix sum assigns non-overlapping ranges from the counts, so parallel fills can write their own lists without reserving a fixed worst-case array for every tile.

## Validation

The geometry checkpoint has independent wasm, native and browser evidence in the rendering overview. Its native suite includes capacity and renderer contracts; the fixture checks its face/stroke controls at both sizes. The separate [13-test GPU run](release/completion-geometry-gpu-windows-20261116.json) verifies overflow growth followed by idle redraw cessation, stale-list invalidation during dragging, projection-cache invalidation and pipeline reuse. It does not identify the selected adapter or exercise the clipping and ambient suites. This chapter adds explanations and source-section markers only. Full clipping, dense visibility overflow, multisample and device-specific acceptance must retain their own evidence; a small two-box image does not prove them.

The separate [application GPU run](release/completion-application-gpu-windows-20261116.json) passes 36 assertions, including clipping, ambient and outline checks, but fails instanced-versus-baked color parity. The failure is reproduced with the original assertion unchanged. This checkpoint's mesh/BRep/curve/Matrix fixture does not exercise that parity and cannot establish complete renderer acceptance.
