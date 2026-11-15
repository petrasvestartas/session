# Render kernel geometry

Draw a mesh, a boundary representation, an arrowed curve and a numerical matrix using the production GPU renderer.

This seventh cumulative checkpoint is a candidate. Its independent wasm build and retention checks pass; browser interactions pass at desktop and upright sizes; 185 native tests pass with 13 ignored; complete teaching acceptance is pending.

## Prerequisites and ownership

Keep the first frame, painting, mathematics, dock, camera and object-row implementations. This subject connects kernel geometry to GPU lanes and the frame encoder. It owns resident geometry conversion, stroke construction, matrix display and renderer resources. Application decoding, selection commands, streaming and stage integration follow later.

The renderer depends on several drawing passes at construction time. Introducing these as separate checkpoints would repeatedly change the same constructor and upload wiring. This checkpoint therefore contains the complete renderer; the sections below explain its parts within one executable subject. The existing `Lane` hooks and `REGISTRY` are the extension mechanism. Application-dependent tests are registered when their application prerequisites exist. Resident cloud conversion is separate from the later streaming adapter, so loading can add its registration without rewriting conversion.

Continue with [visibility, clipping and resource ownership](completion-visibility.md), a chapter of this same runnable checkpoint.

## Build

Add the geometry registration and renderer modules, retaining all earlier subject implementations.

### From kernel objects to rows

A kernel object describes geometry in double precision. GPU lanes need compact, typed arrays with one object-row identifier per primitive. `WalkCx` supplies the row, quality and attribute policy; `Walk` borrows the output tables. Converters append vertices, indices or analytic primitives and return bounds and flags. The row table supplies placement and visibility independently of that geometry.

A mesh contributes triangles and edge information. A BRep first evaluates its surfaces and trims through the kernel display interface; the resulting display is tessellated data, not a change to the original model. A NURBS curve contributes evaluated samples. The quality settings control approximation: increasing detail changes display density, while editing still operates on the original kernel object.

The example gives each object a distinct row and translation. Bounds are transformed by the same placement before union into the scene box. Fitting unplaced bounds would centre the camera on the wrong scene. The matrix is a component rather than a geometry variant, so it enters through its own converter and registered lane.

```rust
--8<-- "session_viewer/examples/course/geometry.rs:course-geometry"
```

### Strokes, dashes and reveal

A polyline is a sequence of neighbouring point pairs. Repeated points produce no segment. Each surviving segment stores its endpoints, width, colour and object row. The accumulated length is carried to the next segment: restarting the dash offset at each vertex would visibly restart the pattern. A chain range preserves which segments belong to one joined stroke.

```rust
--8<-- "session_viewer/src/app/walk/curves.rs:curve-dash-chain"
```

The dash parser accepts positive finite lengths and uses the first on/off pair. A single positive length supplies equal on and off lengths; an empty valid pattern is solid. These are the current display semantics, not a promise to implement arbitrary alternating patterns.

Reveal is stored on the object row, not baked into a replacement curve. The current ribbon shader mixes each sampled segment's endpoint toward its start. At one half, a sampled NURBS curve therefore shows repeated gaps along its full path; it is not a prefix measured along the whole curve. Updating reveal preserves the original samples, and restoring one restores the continuous stroke. Dash phase uses cumulative length separately.

### Endpoint clearance and arrowheads

An arrowhead is oriented by a nonzero terminal segment. A head-only vector row borrows the curve's object identity and pen, while the curve's end is trimmed to avoid showing the shaft through the head. Endpoint clearance accounts for visible marker size; it is independent of merely hiding an object.

```rust
--8<-- "session_viewer/src/app/walk/curves.rs:curve-heads"
```

```rust
--8<-- "session_viewer/src/app/walk/curves.rs:endpoint-clearance"
```

### The arrow function boundary

The shared `arrow_corner` shader receives individual fields. An observed Android driver failure changed `end` and `heads` when a packed row structure crossed that function boundary. Keeping the arguments separate is a compatibility requirement; passing a convenient `VectorRow` value would reintroduce the failing interface.

| Argument | Meaning at the shared function boundary |
| --- | --- |
| `start`, `end` | Local endpoints; the object row supplies placement. |
| `radius` | Encoded shaft width consumed by the width calculation. |
| `instance_id` | Object row for placement, tint, visibility and reveal. |
| `ink` | Packed source colour. |
| `head_pens` | Requested head size in pen widths. |
| `heads` | Bits selecting start/end heads and head-only drawing. |
| `start_trim`, `end_trim` | Endpoint clearance applied along the shaft. |
| `vid` | Corner of the generated shaft or head triangles. |
| `over` | Whether this draw is the overlay variant. |

Eighteen generated vertices cover shaft, end head and start head. Hidden rows return a position outside the view. The same function is shared by normal and overlay paths so their head shape and trimming agree. For a device-only failure, use the temporary [shader probe procedure](../debugging.md#probe-a-phone-only-shader-failure), record the actual adapter and remove the probe afterwards. Desktop browser success does not replace the recorded Android evidence.

### Kernel Matrix display

A kernel `Matrix` becomes a component carrying dimensions, values and display settings. The converter rejects empty dimensions or mismatched value counts. It computes the nonzero value range, packs a header and cells, and records a bound for camera fitting. The shader uses that packed data to draw cells, grid lines and numeric values; zero has its own colour rather than moving the nonzero range.

```rust
--8<-- "session_viewer/src/app/walk/matrix.rs:kernel-matrix-upload"
```

This is distinct from the earlier TeX matrix layout. TeX arranges mathematical glyphs in a slide. A kernel Matrix is a placed scene object with a GPU row and numerical colour scale. The checkpoint displays the cells; the application text path supplies component titles later.

### Device, uploads and frames

`Gpu::from_setup` builds the renderer from an already opened device and queue. The retained canvas owns acquisition and presentation. Cloned wgpu handles refer to the same device; constructing the renderer does not open a second canvas or adapter. The production asynchronous constructor opens its device and delegates to the same setup function.

`set_scene` uploads typed lane tables and the object rows. A lane owns its buffers and pipelines, reports its memory usage and handles reset, release and target changes. Reset can retain capacity for another scene; release frees resources. A bind group must be rebuilt if its underlying buffer allocation changes.

Each frame uses physical target dimensions and a separate logical size. The ratio determines pixel-scaled widths. The camera matrix is expressed relative to the renderer's current anchor, matching the relative translations in object storage. Rebasing only one side would move the scene incorrectly.

The frame encoder receives the acquired colour view, records the passes and submits one command buffer. Pipeline and attachment formats must agree, including sample count. Surface acquisition and presentation remain outside this example's renderer call, in the retained canvas host. Application redraw pacing, asynchronous readbacks and recovery are separate acceptance work; this small scene does not establish their correctness.

### Pass order and visibility

`render.rs` first prepares registered passes, builds triangle visibility lists and projects cloud points. The face stage draws background, section caps, solid faces and cloud resolve while writing depth. Ambient occlusion and outline masks consume that result. The ink stage then draws borders, curves, registered lanes, markers and text. Selected mesh edges appear below the selection outline; selected curves appear above it. The widget uses its own depth so it remains usable over the scene.

This order explains the experiment: O removes solid face colour while leaving their edges, exposing the back edges. W controls the curve stroke lane independently; solid surface borders have a different setting. A depth buffer answers which surface is closest, while an object-ID target answers which object was hit. Neither can replace the other.

`Pass` extends the lane lifecycle with hooks around these stages. `PASSES` registers instancing, clipping, ambient occlusion and surface outlines. During a mutable hook, `each_pass` temporarily replaces that pass with `Nothing`; the hook can then borrow the rest of `Gpu` without two mutable references to the same object. The original pass is restored afterwards. A later pass registers once rather than inserting calls throughout the frame loop.

Triangle visibility storage has bounded capacity. The renderer starts with a small pool, reads overflow reports asynchronously and grows toward a ceiling. The current frame cannot assume that an unreceived report is complete. The convergence and capacity tests exercise that policy; a small box scene alone cannot prove it.

### Render resident cloud points

`walk/cloud/resident.rs` converts kernel positions, colors and complete normal arrays into typed cloud rows. Missing normals use the explicit `NO_NORMALS` sentinel; a partial normal array must not be indexed as if it covered every point. The file's point-size override wins over the cloud's own setting. Bounds come from the converted points and participate in the same placed-row fitting as other geometry.

`gpu/cloud.rs` separates source-relative point indices from GPU allocation rows. Each uploaded `Chunk` records its source interval and GPU start. `row_of` returns no row for a point that is not resident. A cloud can therefore grow in chunks without pretending that every source point already occupies a GPU row. `CloudDraw` also records octree-node and normal offsets; they are independent arrays with independent bases.

`gpu/lod.rs` walks the resident cloud's octree using projected spacing, camera distance, object scale and viewport height. Small clouds, missing octrees and an explicitly disabled LOD threshold draw every resident point. For larger clouds, a coarse node can be sufficient at a distant view while nearby nodes need their children. Residency bounds each selected range: display LOD cannot draw a point that has not been uploaded. The streaming chapter separately handles source queries outside that resident prefix.

`gpu/splat.rs` turns selected ranges into bounded draw records. A record carries the combined camera/object matrix, point and normal offsets, radius information, tint, owner row and selected point. Its 160-byte Rust layout matches the shader's 40-word stride. Cumulative point counts let `splat.wgsl` locate a record by binary search when a single draw spans several runs. Six generated vertices form each point's quad; the shader computes its actual coverage and depth.

The point pass owns color and depth textures, allocated when needed and recreated when size or sample count changes. Its preparation key includes the camera matrix, point-size/LOD choices, uploaded count and object-change revision. A matching key reuses the point image. Merely receiving another redraw request does not justify repeating all projection work.

`splat_resolve.wgsl` copies valid point color and reverse-Z depth into the scene. Empty point pixels have zero depth and are discarded. Optional eye-dome lighting compares neighboring logarithmic depths to darken depth boundaries. The ID pass instead writes the object row and point row, so picking and visible color have distinct outputs while using the same point coverage. Original source IDs are resolved by the later source-query adapter; a temporary GPU row is not that persistent identity.

### Text resources

`engine/text.rs` owns validated labels and shaped runs. Placement distinguishes CSS-sized overlays, camera-anchored labels and world-plane text. Shaping depends on text, font and metrics; changing colour or moving a label should reuse the shaped glyphs. Replacing fonts changes the font revision and invalidates the appropriate caches.

`gpu/text.rs` owns the GPU atlas, viewport, raster cache and separate overlay/anchored renderers. Its preparation key includes document and font revisions plus camera/canvas facts. A matching key reuses preparation. The world-plane and plate modules retain their own textures and geometry because those have different lifetime and depth rules from ordinary glyph instances. These scene-label resources are separate from the early mathematics atlas, whose layout is authored in slide coordinates.

### Presentation and pacing

`present.rs` writes shared uniforms, prepares the widget and text, and acquires a surface texture. Success and suboptimal acquisition proceed; any other acquisition result reconfigures the surface and skips that attempt. A successfully acquired frame is encoded, submitted and presented. Optional passes can compile further pipelines after presentation. Pending pipeline work or a visibility report may require a later redraw even when the user has stopped moving the mouse.

`FramePacing` prevents old camera frames accumulating behind a busy queue. `ready` records that a redraw is waiting, then rechecks completion to avoid losing a wakeup between those two operations. `submitted` installs a queue-completion callback; that callback clears the pending flag and requests a redraw only if one was waiting. The flags are shared through `Arc` because completion outlives the submitting call. Their acquire/release operations make the completion and waiter state visible across callbacks.

The retained teaching canvas supplies this checkpoint's presentation loop. Production application pacing is taught here as a resource contract and is exercised through its application integration later; the fixture does not claim to verify every recovery and asynchronous-wakeup case.

## Run and observe

Materialize `geometry` with the completion-checkpoint command in [Maintaining](../maintaining.md), build it, and open `?tutorial=geometry`.

Expect two boxes, a curved double arrow and a coloured two-by-three matrix on white. Arrow keys orbit the camera. P toggles projection, F fits the bounds, R switches each curve segment between half and full reveal, W toggles stroke lanes while the solid boxes retain their surface borders and O toggles faces. Earlier query modes remain runnable in the same bundle.

## Predict and try

Press R, orbit right, then press R again. Predict whether the gaps close in the original world position. They do: reveal changes a row value, and orbit changes the camera; neither rewrites the curve samples. Press left to return to the starting camera.

## Question and answer

Why can a matrix, a triangle mesh and a curve share placement without sharing their vertex format?

Each lane stores its own primitive representation but carries the same object-row identifier. The shared row supplies transform and effects. The lane applies those values while interpreting its own data format.

## Validation

The seven-subject retention suite passes four tests. Independent wasm checking and optimized browser packaging use the checkpoint's own shader build script, without an inherited shader output directory. The cumulative native suite passes 185 tests with 13 ignored. Full production native test modules also compile after the adapter extraction; that separate check does not execute the entire production suite. Browser interactions pass 92 captures across seven modes at desktop and upright sizes. Original scene captures show sampled-segment reveal, face/stroke toggles, projection and orbit with exact return states. Complete renderer teaching remains pending. Picking, multisampling, physical-phone rendering, streaming and the final application require their own checks.
