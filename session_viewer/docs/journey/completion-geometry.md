# Render kernel geometry

Draw a mesh, a boundary representation, an arrowed curve and a numerical matrix using the production GPU renderer.

This seventh cumulative subject has reviewed teaching and recorded desktop/upright Windows Intel WebGPU evidence for its four-object fixture. Its independent wasm build, 185 default native tests and separate 13 GPU tests pass. Complete renderer runtime acceptance remains pending, including the instanced-color comparison described below.

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

### Borrow tables without losing object identity

`Walk` borrows the upload tables mutably for one conversion. Its `solid` method splits those borrows into triangle storage and edge/dot storage, so converters can append to both without cloning the scene. `WalkCx` carries the object row, existing vertex base, display quality and feature-visibility switches. The returned `Row` describes local bounds, spacing and flags; placement remains in the shared object table. Attribute and interaction geometry uses the parent row, so showing a feature does not create a separately pickable object.

`encode.rs` defines the CPU side of the primitive packing contract. An explicit positive pen width becomes a negative half-width in pixels; the ordinary width of one and invalid widths become zero, meaning the default pen. RGBA channels are clamped, rounded to bytes and packed with red in the low byte. Face directions use octahedral projection: divide by the sum of absolute components, fold the lower hemisphere and quantize two signed coordinates. Two 16-bit normals share a word; a lone normal is repeated. The reserved all-ones word means unknown facing, so a real packed pair must avoid it.

`walk_point` appends one signed-distance-field dot with the parent row and unknown face orientation. A default pen becomes a three-pixel radius; an explicit width uses the same negative pixel-radius encoding as lines. Its bounding box contains the point centre, not a camera-dependent pixel halo. This keeps fitting in scene coordinates while the glyph shader supplies the visible screen-sized footprint.

`Baselines` records the object, pipe, ribbon and vector table lengths before appending a file. `file_extent` unions only newly appended live rows after applying their placement; dead definition rows must not enlarge the fitted scene. Sheet detection requires a common placement and a local z band thinner than 0.001 scene units. `mark_sheet` flags those new rows and replaces only zero/default segment and arrow radii with a positive 0.5 scene-unit radius. Explicit negative pixel widths survive. Recording each table boundary prevents a sheet from changing pens belonging to an earlier file.

`interactions.rs` classifies contact, joint, cut and drill features as interactions; other features belong to the element itself. `hosted` traverses graph edges in creation order, associates their interactions with unclaimed feature indices, and falls back to individual entries for interaction features whose originating interaction is absent. This keeps the display grouping stable without duplicating one feature under several owners. The geometry walker uses the same classification for its two visibility switches; the later document UI consumes the returned labels and indices.

### Convert mesh topology into faces and ink

`mesh_topology.rs` turns kernel vertex keys into compact GPU slots. A dense lookup is used when the largest key is less than four times the vertex count; sparse keys use a hash map instead of a huge mostly empty vector. Sorted face keys make traversal deterministic. Newell normals sum every polygon edge, with degenerate faces represented by no normal. The topology records unique edges, their adjacent faces and whether the faces walk the shared edge in opposite directions. Hole rings are oriented against the outer border for this comparison, preserving the outer-edge ordering used by colors, widths and picking IDs.

`MeshOpts` separates three conversion policies: ordinary objects permit sheet fills and open surfaces, sampled surfaces use smooth-edge rules without sheet lanes, and element meshes are never flagged open. A single zero width identifies a printed fill. Vertex spacing is estimated from the bounds diagonal divided by the square root of the vertex count. At 200,000 triangles the raw-mesh path omits edges and dots to bound display cost; the resulting shaded surface does not promise a full wireframe. Native profiling uses `Lap`; its browser counterpart has no clock.

`mesh_ink.rs` derives visible ink from the topology rather than drawing every triangle edge. Zero widths hide edges; nearly coplanar adjacent faces suppress their shared diagonal unless all-edges mode is enabled. For a smooth surface, borders and creases sharper than 25 degrees remain features, while other seams carry the silhouette marker for view-dependent drawing. The two face normals are corrected for inconsistent winding before packing. Smooth sampled edges receive no kernel edge-picking ID. At 10,000 edges the wireframe uses black to keep dense ink consistent; this changes display color, not kernel geometry.

`knobs.rs` reads each diagnostic environment flag once through `OnceLock`. Presence enables the flag, even when its text is "0". Profiling, edge suppression, dot suppression, coplanar edges, smooth seams and coarse BRep edges are separate switches. Set them before starting a native process; changing the environment after first access cannot alter the cached value. `VIEWER_DROP_SESSIONS` remains a recognized flag but no longer changes behavior, so it is not a memory-release control.

### Tessellate a BRep without replacing its model

`walk_brep` first tries a recorded congruent shape; otherwise it asks the kernel for one display mesh per face. Finite-quality refinement uses bounded meshing. If that budget fails during a candidate walk, the candidate is rejected; an ordinary walk can fall back to the default display quality. Face signs correct both vertex normals and triangle winding before upload. The converter keeps UV samples for later edits, appends face-source identities and then draws boundary and silhouette ink. Smooth, open, closed and single-face row flags describe how later rendering and clipping should treat the result.

`face_signs` preserves authored normals for an open shell. For a solid, shared edge chains build a face-adjacency graph. A breadth-first traversal assigns signs so neighbouring faces walk their common boundary in opposite directions. The signed volume of each connected group then decides whether the entire group must flip outward. Sorted triangle accumulation and smaller-key tie breaking make this decision reproducible; this display correction does not rewrite the BRep topology.

`brep_edges.rs` first locates a face-mesh chain from constrained-edge labels or an isoparametric boundary. It checks that the chain reaches the actual edge endpoints before allowing that face to supply visibility information. Display chords are then evaluated on the exact 3D curve and mapped back to the corresponding coarse chain segment. Closed-curve parameters are unwrapped across the seam. Each chord carries its rise above the supporting facet, with a safety margin, so curved ink can remain visible on its own faceted surface without using an arbitrary global depth bias.

`SharedWalks` limits shape reuse to the lifetime of a document walk. A thread-local map holds recordings keyed by the congruent shape and quality tolerance; nested scopes join the same map, and the outermost guard frees it on drop. The first shape records the ranges it appended. A matching copy replays those ranges in its own frame and object row rather than tessellating again. An unsupported recording is remembered as unavailable. This bounds cache lifetime and avoids confusing shared display geometry with shared object identity.

`display.rs` admits refinement before allocating its working set. Checked size estimates reject unsupported or excessive sources; `None` means retain the existing display, not delete the object. `Quality::for_error` quantizes local tolerance in half-octave steps and keeps a zoom-out dead band, reducing repeated rebuilds. Curve sampling splits at knot spans, then subdivides exact subcurves with a finite stack, a depth limit of 16 and a segment budget capped at 4,096. Reaching a limit marks the result limited rather than claiming the requested error was met. `candidate` uses a drop guard to restore thread-local active/failure state, so a failed nested conversion cannot silently become an accepted replacement.

`display_features.rs` budgets element annotations alongside refined geometry. It rejects more than 256 features or outlines, more than 4,096 total points, non-finite coordinates and excessive contact-triangulation work. Checked arithmetic counts source strings and coordinates as well as transient copies. `snapshot` creates fresh polylines containing the drawing and identity fields that this display path consumes; it does not retain cached planes, spare capacity or dash arrays. The original feature remains the editing source. This selective copy makes the reservation meaningful instead of cloning an unbounded hidden cache.

### Strokes, dashes and reveal

```rust
--8<-- "session_viewer/src/engine/gpu/segments.rs:stroke-row-layout"
```

`ribbon.wgsl` builds each six-vertex stroke quad in screen space, with widths projected independently at its endpoints. Neighbour cut planes join a chain without double coverage; marker trims and arrowhead cut planes remove covered shaft ends. Dash coverage includes the stored cumulative offset, while reveal moves each segment endpoint toward its own start. Silhouette-only segments inspect the sag sign bit and adjacent face orientation. Fragment coverage is evaluated before the shared visibility query; the color and mask passes use the same centre-line clipping and surface test.

`SegRows` separates surface-edge pipes, editable ribbons and streamed sheet rows, with parallel tables for source IDs, chain ranges, dash lengths, cumulative offsets and trims. Upload expands a 40-byte segment into a 72-byte `StrokeSegment` carrying previous/next neighbours and visibility slack. A special neighbour code marks an end covered by an arrowhead; the sign bit of the sag field marks silhouette-only ink, including negative zero. Sheet chunks must continue the resident source interval in order. Keeping source intervals separate from GPU row offsets lets picking recover the original entity after chunked uploads.

`walk_sheet_slice` converts streamed drawing segments into a dedicated sheet table, keeping them separate from editable ribbons. Six position floats make one segment. Missing color, width and entity-ID entries receive black, a hairline and the missing-ID sentinel respectively. Positive sheet widths become positive half-widths in scene units. The draw record preserves both the source slice offset and the first uploaded row, so source identity survives partial residency. Its bounds include both endpoints of every complete segment.

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

### Generate planes and arrows from compact rows

A clipping plane uses `walk/plane.rs`, a separate producer from the ordinary orientation grid. It emits a rectangle and an arrow toward the cut side, sets the clipping-plane row flag and includes a mirrored point in its bounds to keep the manipulation centre at the plane origin. The first cut enables solid verification for subsequently walked meshes. Closedness checks balance directed edge uses; signed volume distinguishes inward winding. Hole rings and oversized vertex keys delegate closedness to the kernel. Only a verified solid receives the closed flag used by capping.

`walk_plane` stores a frame rather than building a separate mesh for every grid line. Its row contains the origin, x/y axes, pen radius and object identity; the current plane size determines initial bounds, including the z-arrow tip. `walk_obb` instead emits the twelve fixed corner pairs as ribbons and unions their corner positions. Both return thin rows: their outlines participate in fitting without pretending to enclose a shaded solid.

`PlaneLane` registers construction, row counting and merging in the same lane registry as other primitives. A plane occupies 48 bytes, checked on the Rust side, and one instanced draw generates 22 grid lines plus three arrows. The global plane size is stored as atomic f64 bits and copied to frame uniforms, so changing it changes the next drawing without rebuilding the frame rows. Buffer growth requires rebinding group three. Color and object-ID pipelines share the geometry, keeping a picked plane aligned with its visible drawing.

`plane.wgsl` derives each grid endpoint from the vertex index and the stored frame. Eleven positions along each axis produce the ten-by-ten grid. The z direction is the cross product of x and y. Grid segments call the shared arrow function without heads and with ordinary depth behavior; the three colored axis arrows request end heads and the overlay variant. This explains why a solid can hide the grey grid while the orientation arrows remain readable.

`VectorRow` is a checked 64-byte GPU record containing endpoints, pen, object identity, color, head settings and endpoint trims. The head bits independently select start, end and head-only drawing. `VectorLane` owns the row buffer and matching color/ID pipelines, while its registry entry provides creation, row counting, merge and stride. The draw generates eighteen vertices per row; it does not upload a separately tessellated arrowhead mesh.

`vector.wgsl` is the thin entry adapter for the shared arrow implementation. The instance index selects a stored vector row and the vertex index selects one of eighteen generated corners. It passes every endpoint, pen, head and trim field individually to `arrow_corner`, with the ordinary depth variant. Keep this field-by-field call when adding another arrow producer; the driver constraint described above applies at the shared function boundary, even when the caller already has a convenient packed row.

### Kernel Matrix display

A kernel `Matrix` becomes a component carrying dimensions, values and display settings. The converter rejects empty dimensions or mismatched value counts. It computes the nonzero value range, packs a header and cells, and records a bound for camera fitting. The shader uses that packed data to draw cells, grid lines and numeric values; zero has its own colour rather than moving the nonzero range.

```rust
--8<-- "session_viewer/src/app/walk/matrix.rs:kernel-matrix-upload"
```

`MatrixLane` stores variable-sized matrices in a word buffer: a sixteen-word header followed by three words per value. A separate list records block starts, which become the instance indices for six-vertex quad draws. The value stores an f32 for color plus an integer whole part and signed thousandths for its label. Keeping those representations separate prevents large values from losing their displayed decimals solely because f32 color storage has limited precision.

`matrix.wgsl` expands the quad in cell coordinates, with cell zero centred on the object origin and later rows extending downward. The fragment stage finds the cell and reads its packed value. Numeric labels are generated from seven-segment digits, a minus sign and decimal point; trailing decimal zeros are dropped. They are not glyph-atlas text, so this small numeric display does not load a font. The header supplies zero/minimum/maximum colors and the nonzero range, allowing zero cells to remain visually distinct.

This is distinct from the earlier TeX matrix layout. TeX arranges mathematical glyphs in a slide. A kernel Matrix is a placed scene object with a GPU row and numerical colour scale. The checkpoint displays the cells; the application text path supplies component titles later.

### Device, uploads and frames

`Gpu::from_setup` builds the renderer from an already opened device and queue. The retained canvas owns acquisition and presentation. Cloned wgpu handles refer to the same device; constructing the renderer does not open a second canvas or adapter. The production asynchronous constructor opens its device and delegates to the same setup function.

`set_scene` uploads typed lane tables and the object rows. A lane owns its buffers and pipelines, reports its memory usage and handles reset, release and target changes. Reset can retain capacity for another scene; release frees resources. A bind group must be rebuilt if its underlying buffer allocation changes.

Each frame uses physical target dimensions and a separate logical size. The ratio determines pixel-scaled widths. The camera matrix is expressed relative to the renderer's current anchor, matching the relative translations in object storage. Rebasing only one side would move the scene incorrectly.

The frame encoder receives the acquired colour view, records the passes and submits one command buffer. Pipeline and attachment formats must agree, including sample count. Surface acquisition and presentation remain outside this example's renderer call, in the retained canvas host. Application redraw pacing, asynchronous readbacks and recovery are separate acceptance work; this small scene does not establish their correctness.

`pipelines/geometry.rs` delegates the arena vertex layout to `GpuVertex`, keeping pipeline inputs synchronized with the packed storage format. Its ignored native test renders an empty frame, uploads a solid and alternates four- and one-sample targets. Compilation counts must grow on first use but stay unchanged when returning to an already compiled target; it also asks the cache to compile unused variants. This is an explicit adapter-dependent test of lazy pipeline ownership, not a result supplied by a documentation build.

### Keep allocation, identity and frame state separate

`Gpu` is the resource owner: device/queue, frame uniforms, targets, object table, fixed lanes, registered lanes, passes, picker and performance state live together. `lane_list!` produces shared or mutable trait-object views over each fixed lane once; registered lanes and passes extend the same lifecycle separately. This central list keeps resource accounting and reset/release operations aligned. A headless renderer has no surface, while a browser renderer also carries pacing state. Application code receives this complete renderer rather than opening a second device for each feature.

`Upload` owns the CPU rows waiting for transfer. Merging an upload moves each typed table, shifts mesh vertex indices by the destination vertex base and rebases valid face-source IDs; the all-ones missing-ID sentinel stays unchanged. Registered lanes merge through `REGISTRY`, so a new primitive family does not require changes throughout the upload loop. `drop_uploaded` replaces vectors with empty vectors and clears lane rows after transfer, freeing their capacity instead of retaining a second full CPU copy.

`arena.rs` packs the kernel render vertex into a 20-byte GPU vertex: three position floats, one octahedral normal word and one RGBA word. The kernel and GPU representations therefore have different strides. A reserved normal code represents absent normals, and quantization avoids that code. The arena keeps object-row IDs separately from vertices and maintains distinct index streams for ordinary faces, printed fills and text. Packing saves transfer and device memory while preserving the source representation used by editing.

`Faces` keeps the mapping from drawn triangles to source faces alongside the selected source-face uniform. Selecting a face changes that small uniform and its revision rather than recoloring every vertex. Physical, opaque, clipped, object-ID, face-pick, highlight and mask pipelines serve different passes over the same geometry. Their shared bind group includes geometry/source buffers and selection; its layout must agree with the pulled face shader, while instance slots supply the placed owner.

`Slots` lets many placed objects draw one definition without duplicating its geometry. Each eight-byte instance entry carries an object row and triangle-ID base. Slot zero contains `OWN_ROW`, meaning ordinary draws keep the row already stored on each primitive. Definition draws use ranges of later slots; their vertex-buffer layout advances once per instance. Updating slots changes placement identity without rewriting vertices. Clearing can either keep capacity or release it, while restoring the mandatory slot-zero entry.

`placed_triangle` reverses a one-based visible triangle ID into an arena triangle and an optional instance-row override. The slot texture is 1,024 texels wide; its header gives the instance count and ordinary arena range. Ordinary IDs keep `SLOT_OWN_ROW`. For an instance ID, binary search finds the last slot whose ID range starts before it, then a range check rejects gaps. This makes a picked or shaded instance refer to its own placement while reusing the definition triangle. Zero and out-of-range IDs return `NO_TRIANGLE`.

`instanced.rs` keeps instance triangle-ID ranges synchronized with the arena. A partial change writes the affected slot ranges to the slot texture and both face/stroke slot buffers; a texture-capacity change falls back to writing the full table. The same draw ranges are then installed in both lanes. Every update invalidates projected triangle lists and announces changed geometry to the object visibility cache. The registered `Instanced` pass follows arena growth before rendering, so new ordinary triangles cannot collide with old instance IDs.

`patch.rs` describes editable allocation in units of each lane, not one interchangeable row count. `Counts` stores vertex, face-index, printed-index, text-index, source-face, stroke, marker and registered-lane counts; `Span` pairs start offsets with those counts. Capacity fitting compares every lane. Byte accounting includes the associated CPU mirrors and source-ID storage, so its per-lane stride is not necessarily the packed GPU vertex stride. These contracts let the application decide whether a replacement fits an existing allocation without teaching the renderer about editing commands.

`Upload::pad` reserves hidden spare rows for a growing editable preview. It adds roughly half the requested counts, rounds index allocations to whole triangles and directs spare primitives to a sink row. Missing face and edge IDs use their explicit sentinels. The returned count difference records the actual allocation, including rounding. Spare allocation is not visible geometry: later edits can fill it without moving every following object, while the sink keeps unused rows from drawing.

`objects/lane.rs` adapts the object table to the common `Lane` lifecycle. Reset delegates to logical row reset, release delegates to allocation release with device/layout access, and byte reporting includes its buffers with zero texture bytes. This small adapter is how the renderer can manage object rows alongside drawing lanes without duplicating their lifetime rules in the host.

`hull_of` gathers extreme points from the positions written by a conversion: arena vertices, both endpoints of pipes/ribbons, and marker/dot centres, plus explicitly supplied points. It reduces each source before combining extremes. Registered lanes, resident cloud positions or sheet rows make this path return no hull because their points are not represented by those readers. A final span check requires the candidate to cover the declared bounds. Rejecting an incomplete hull preserves conservative fitting instead of silently underestimating an object.

`FrameInput` contains the camera matrix, clear color and time supplied by the caller; `FrameCx` adds target size, CSS-to-framebuffer scale, anchor and view settings. `Binds::set` installs the common camera, pen and object groups at indices zero, one and two. `LineUniform` has an explicitly checked 80-byte layout because shaders read its fields at fixed offsets. Physical viewport dimensions and logical pixel scale must agree: otherwise a nominal screen-sized pen changes thickness with device scale or a subviewport uses the wrong origin.

`View` stores rendering choices separately from GPU allocation. Startup reads native environment variables or browser query values, clamps pen softness and opacity, and distinguishes line, mesh-edge, point and outline controls. Enabling Arctic shading also enables outlines. A coarse primary pointer selects solid default faces; the desktop default is 0.95 opacity. Browser device scale is capped at two unless an explicit valid cap is supplied, with reduced mode capped at one. A narrow viewport alone does not prove that the browser reports a touch-first device.

`Targets` owns depth, optional multisampled color and per-sample triangle-ID attachments for one size and sample count. Triangle IDs occupy two integer 16-bit channels. Shaders bind both single-sample and multisample views, so the unused variant receives a one-by-one placeholder with the correct sample type. Keeping those placeholders owned with the targets prevents dangling resource assumptions. A size or sample-count change requires matching targets and pipelines; changing only a texture cannot change the pipeline attachment contract.

`PassTimer` is optional instrumentation, available only when the device supports both timestamp queries and timestamps inside encoders. It writes at most 32 marks, resolves them to a GPU buffer, copies to a readable buffer and converts adjacent tick differences using the queue timestamp period. Collection waits for GPU completion and unmaps the readback before reuse. These measured GPU spans are different from CPU command-encoding time; the blocking collection belongs to an explicit measurement run, not an assumed free operation in ordinary animation.

`instance/validation.rs` tests the CPU/shader boundary independently of a picture. It extracts WGSL field names in order and compares the shared instance and line-uniform declarations with the Rust-side contract, including the 80-byte uniform size. It also checks the shared translation binding and rejects lane shaders that redeclare common structures. These tests catch interface drift; they cannot establish visual driver correctness or replace the separate layout-offset assertions.

The ignored native test in `objects/gpu_tests.rs` opens a headless device, moves one object row and verifies that its neighbour remains unchanged. Scaling must update that row's ambient radius while translation preserves it. The widget row must remain after all object rows and be reminted when another upload arrives. The test is ignored by default because it requires a native adapter; compiling it or reporting an ignored count is not evidence that these GPU-backed operations ran.

### Pass order and visibility

`render.rs` first prepares registered passes, builds triangle visibility lists and projects cloud points. The face stage draws background, section caps, solid faces and cloud resolve while writing depth. Ambient occlusion and outline masks consume that result. The ink stage then draws borders, curves, registered lanes, markers and text. Selected mesh edges appear below the selection outline; selected curves appear above it. The widget uses its own depth so it remains usable over the scene.

This order explains the experiment: O removes solid face colour while leaving their edges, exposing the back edges. W controls the curve stroke lane independently; solid surface borders have a different setting. A depth buffer answers which surface is closest, while an object-ID target answers which object was hit. Neither can replace the other.

`Pass` extends the lane lifecycle with hooks around these stages. `PASSES` registers instancing, clipping, ambient occlusion and surface outlines. During a mutable hook, `each_pass` temporarily replaces that pass with `Nothing`; the hook can then borrow the rest of `Gpu` without two mutable references to the same object. The original pass is restored afterwards. A later pass registers once rather than inserting calls throughout the frame loop.

Triangle visibility storage has bounded capacity. The renderer starts with a small pool, reads overflow reports asynchronously and grows toward a ceiling. The current frame cannot assume that an unreceived report is complete. The convergence and capacity tests exercise that policy; a small box scene alone cannot prove it.

`BackdropLane` owns the fullscreen background and construction-plane grid. It stores the authored grid frame in double precision and uploads a frame relative to the current scene anchor. Updating the frame or anchor updates that small uniform rather than rebuilding grid vertices. The background and grid use distinct pipelines and topologies: a fullscreen triangle and generated line endpoints. Their resources follow the same lane lifetime as scene geometry.

`background.wgsl` generates one oversized triangle from three vertex indices, covering the viewport without a diagonal between two triangles. Its fragment output is white, or slightly grey when ambient shading is active, and writes zero primitive identity. The backdrop therefore contributes visible color but cannot masquerade as an object triangle in subsequent visibility queries.

`grid.wgsl` generates eleven lines in each construction-plane direction at 1,000-unit spacing, extending 5,000 units either side of the origin, plus three colored axes. The vertex index determines the line and endpoint; no grid vertex buffer is needed. Each local point is expanded through the supplied frame axes and origin before camera projection. This construction grid is separate from a selectable plane object and writes no primitive identity.

`objects/ink.rs` builds the object binding used by ink shaders: object and translation buffers, both depth sample variants, both primitive-ID variants, projected triangles and tile lists. Rebinding must follow attachment or buffer replacement because the bind group holds resource handles. Picking builds the same contract over its own cropped depth/ID targets; borrowing the main image attachments would test visibility in the wrong pixel coordinates.

`Picker` owns request, draw, copy, map and poll state. It clamps a cursor window to the framebuffer, adds a three-pixel visibility halo and uses a 256-byte-aligned readback row stride. A generation counter invalidates answers from cancelled requests. Source-point queries clear IDs on their first page and preserve them on later pages while reusing depth. Asynchronous map completion is published through an atomic flag; polling distinguishes no completed answer from a completed miss. Application input can consume that result without blocking the render loop for every pointer event.

### Draw dots and vertex markers

`GlyphLane` keeps flat dots and vertex markers in separate GPU tables using the same checked 48-byte `GlyphPoint` record. The record contains position, radius, color, object row and up to six packed neighbouring face normals. Dots use three generated vertices; markers use a reusable template. Separate color, object-ID and source-point-ID pipelines interpret those rows for their own output target. After a table grows, its bind group is rebuilt around the new buffer allocation.

`glyph.wgsl` projects each dot centre and expands a triangle around its disc in screen space. Negative radii stay in pixels; positive radii are projected from scene units. Very small discs retain a half-pixel footprint and fade instead of flickering away. The fragment stage computes circular coverage before expensive visibility reads. Picking requires sufficient coverage and rejects non-pickable object rows; ordinary dots encode a marker tag, while source-query dots use the supplied point row. A visible dot and its pick identity therefore share placement but need different output encodings.

`sphere.wgsl` uses the marker template and neighbouring face normals to distinguish front-facing and back-facing vertex markers. Packed normals are transformed with the object before comparison to the eye direction. Markers shrink when projected vertex spacing is less than three diameters, with a minimum taper factor of 0.15, preventing a dense mesh from becoming a solid field of dots. Its color and ID entry points share coverage geometry; displaying a faint marker through translucent faces does not mean that an occluded marker is freely pickable.

### Render resident cloud points

`walk/cloud.rs` is the module boundary between resident cloud conversion and the streaming adapter. It re-exports `walk_cloud` from the resident implementation. Loading later registers the stream module and its slice types at this boundary; it does not replace how resident geometry is converted. Keeping this file as declarations makes the finished-subject check able to distinguish an allowed registration from a hidden implementation rewrite.

`walk/cloud/resident.rs` converts kernel positions, colors and complete normal arrays into typed cloud rows. Missing normals use the explicit `NO_NORMALS` sentinel; a partial normal array must not be indexed as if it covered every point. The file's point-size override wins over the cloud's own setting. Bounds come from the converted points and participate in the same placed-row fitting as other geometry.

`gpu/cloud.rs` separates source-relative point indices from GPU allocation rows. Each uploaded `Chunk` records its source interval and GPU start. `row_of` returns no row for a point that is not resident. A cloud can therefore grow in chunks without pretending that every source point already occupies a GPU row. `CloudDraw` also records octree-node and normal offsets; they are independent arrays with independent bases.

`gpu/lod.rs` walks the resident cloud's octree using projected spacing, camera distance, object scale and viewport height. Small clouds, missing octrees and an explicitly disabled LOD threshold draw every resident point. For larger clouds, a coarse node can be sufficient at a distant view while nearby nodes need their children. Residency bounds each selected range: display LOD cannot draw a point that has not been uploaded. The streaming chapter separately handles source queries outside that resident prefix.

`gpu/splat.rs` turns selected ranges into bounded draw records. A record carries the combined camera/object matrix, point and normal offsets, radius information, tint, owner row and selected point. Its 160-byte Rust layout matches the shader's 40-word stride. Cumulative point counts let `splat.wgsl` locate a record by binary search when a single draw spans several runs. Six generated vertices form each point's quad; the shader computes its actual coverage and depth.

The point pass owns color and depth textures, allocated when needed and recreated when size or sample count changes. Its preparation key includes the camera matrix, point-size/LOD choices, uploaded count and object-change revision. A matching key reuses the point image. Merely receiving another redraw request does not justify repeating all projection work.

`splat_resolve.wgsl` copies valid point color and reverse-Z depth into the scene. Empty point pixels have zero depth and are discarded. Optional eye-dome lighting compares neighboring logarithmic depths to darken depth boundaries. The ID pass instead writes the object row and point row, so picking and visible color have distinct outputs while using the same point coverage. Original source IDs are resolved by the later source-query adapter; a temporary GPU row is not that persistent identity.

### Text resources

`engine/text.rs` owns validated labels and shaped runs. Placement distinguishes CSS-sized overlays, camera-anchored labels and world-plane text. Shaping depends on text, font and metrics; changing colour or moving a label should reuse the shaped glyphs. Replacing fonts changes the font revision and invalidates the appropriate caches.

`gpu/text.rs` owns the GPU atlas, viewport, raster cache and separate overlay/anchored renderers. Its preparation key includes document and font revisions plus camera/canvas facts. A matching key reuses preparation. The world-plane and plate modules retain their own textures and geometry because those have different lifetime and depth rules from ordinary glyph instances. These scene-label resources are separate from the early mathematics atlas, whose layout is authored in slide coordinates.

`Plates::prepare` creates six vertices per label background, placing depth-tested rectangles before overlays. It preserves the original rectangle centre, half-size and corner radius before clipping the emitted quad to its screen box. That distinction prevents clipping from rounding a newly cut edge as though it were the original plate boundary. Each vertex carries optional object identity and selection; the physical vertex count separates the two depth policies within one buffer.

`text_plate.wgsl` computes signed distance to the original rounded rectangle and uses screen derivatives to soften its edge. Selection changes the black plate to yellow. The ID entry applies the pick-window transform and rejects pixels outside the rounded boundary or without an object. It writes the already encoded object-row-plus-one value, keeping an empty pixel distinct from row zero.

`text_plane.wgsl` samples rasterized glyph coverage on vertices already projected from an authored world plane. A screen clip box cuts the label; a rounded plate can supply black or selected-yellow background. The shader evaluates derivative-dependent coverage uniformly before choosing bare glyphs or a plate. Picking uses the same transformed label footprint and object identity. The pick shape is the rounded label area, not a per-glyph alpha test, so spaces within a label can still select it.

`text_outline.wgsl` renders triangulated text through the arena instead of sampling a glyph texture. The vertex stage applies object placement, tint, selection and hidden state. The fragment stage clips in canvas coordinates and outputs flat color. Its physical-ID output carries the owning object but no surface-triangle identity; text therefore remains selectable without becoming a solid face for geometric visibility reconstruction.

### Keep the manipulation widget independent of scene depth

`gizmo.rs` supplies the geometric manipulation model used by the later input adapter. A drag remembers its initial axis point, angle or reach. Translation compares closest points on an axis, rotation intersects its plane and optionally snaps to quarter turns, and scale compares reach around the pivot. Uniform scaling measures in a view-facing plane. Drag scaling is softened near the centre; a valid typed positive factor is used directly instead. The model returns an `Xform`, leaving command history and document mutation to the application.

`widget_mesh.rs` constructs the reusable manipulation mesh from three lathed arrows, three quarter-ring tubes, three scale balls and a centre hub. Vertices carry position in CSS-sized widget coordinates, packed color and one of ten handle IDs. Shared arm, ball and hub constants come from the geometric model, keeping the displayed shape consistent with hit testing. This mesh is uploaded once; hovering a handle changes a uniform rather than regenerating vertices.

`Widget` renders that mesh into a small tile with its own depth, then composites the tile over the scene. Placement and active-handle state feed a 96-byte uniform containing the matrix, screen rectangle and settings. Off-screen or absent placement disables drawing. Clearing drops tile textures and active state; a changed canvas format rebuilds the composite pipeline. Separate depth keeps scene surfaces from hiding a handle needed to manipulate the selected object.

`widget.wgsl` transforms mesh vertices into the tile and changes the active handle to orange. A second vertex entry maps a six-vertex quad into the tile's canvas rectangle. The composite fragment divides sampled RGB by alpha, guarded against zero, before returning color and alpha for blending. The widget therefore has its own small rendering/compositing contract rather than borrowing the scene-object shader and its selection rules.

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

The seven-subject retention suite passes four tests. Independent wasm checking and optimized browser packaging use the checkpoint's own shader build script, without an inherited shader output directory. The cumulative native suite passes 185 tests with 13 ignored in its default run. A separate [GPU run](release/completion-geometry-gpu-windows-20261116.json) executes those 13 original tests successfully, covering row isolation, glyph depth and cache release, outline picking, triangle-list invalidation and pipeline reuse. The log does not identify the selected adapter. Full production native test modules also compile after the adapter extraction; that separate check does not execute the entire production suite. Browser interactions pass 92 captures across seven modes at desktop and upright sizes. Original scene captures show sampled-segment reveal, face/stroke toggles, projection and orbit with exact return states. Complete renderer runtime acceptance remains pending. Picking, multisampling, physical-phone rendering, streaming and the final application require their own checks.

The separate [application GPU run](release/completion-application-gpu-windows-20261116.json) passes 36 assertions, including clipping, ambient and outline checks, but fails instanced-versus-baked color parity. The failure is reproduced with the original assertion unchanged. This checkpoint's mesh/BRep/curve/Matrix fixture does not exercise that parity and cannot establish complete renderer acceptance.
