# 06 · CAD face contract

## You are building

![Diagram: BRep / NurbsSurface\ (f64 source) · kernel Mesh per face\ positions · u,v · normals · RenderMesh (f32) · ArenaRows\ verts · vids · idx · MeshTopo\ edges · edge_faces · normals · SegRows pipes\ GlyphRows spheres…](illustrations/06-01.svg)

![One face, three representations: BRep source in f64, kernel mesh with u,v, normals and boundary tags, viewer rows in f32 that keep the face and edge identities.](illustrations/cad-contract.svg)

## Starting point

- Checkpoint 05: hand-built quads, strokes and markers in `fixture.rs`; physical depth and visible ink work.
- Nothing yet reads a Session `Mesh`, `BRep` or `NurbsSurface`. This lesson adds the whole `app/walk` producer layer and its first CAD consumer.
- A shading crease is a kernel decision; the viewer only carries it. One kernel edit comes first.

<!-- supplied: 06 -->

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · Kernel: one-sided normals at a C0 knot

![Where this step sits in the viewer: Kernel, with 8 of 11 zones built so far.](illustrations/locator-9c306e3b0e.svg){ .locator data-strip="illustrations/strip-a84a26918d.svg" }

- A knot repeated `degree` times folds the surface; averaging normals across that fold makes a sharp edge look rounded.
- The grid mesher splits a shading vertex on the crease side: same position and `u`/`v`, different normal, so the split never invents a CAD vertex.
- Face keys are sorted before accumulation: float sums are order-dependent, and map order must not reach the mesh bytes.

![Diagram: repeated knot · accumulated normals · two shading vertices\ same u,v](illustrations/06-02.svg)

<span class="zone-mark" data-strip="illustrations/strip-a84a26918d.svg" data-zone="Kernel"></span>

<!-- file: 06 session_rust/src/remesh_nurbssurface_grid.rs type hunks=1-4 -->

`split_crease_normals` is `pub(crate)`: the trimmed mesher in `nurbssurface_trimmed.rs` shares it.

<span class="zone-mark" data-strip="illustrations/strip-a84a26918d.svg" data-zone="Kernel"></span>

<!-- file: 06 session_rust/src/remesh_nurbssurface_grid.rs type hunks=5-5 -->

<!-- check: 06 -->

## Step 2 · Row encodings

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-8bf646ae4a.svg){ .locator data-strip="illustrations/strip-bb17a255a3.svg" }

- Every producer packs the same four things: pen width → world radius, colour → RGBA8, unit normal → 16-bit octahedral code, two normals → one `facing` word.
- `FACING_UNKNOWN` is all ones and means "no adjacency, always draw"; `pack_facing` steps around that value.

![Diagram: pen width · world radius · colour · RGBA8 · unit normal · 16-bit code…](illustrations/06-03.svg)

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/encode.rs type -->

## Step 3 · What a producer reports

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-8bf646ae4a.svg){ .locator data-strip="illustrations/strip-bb17a255a3.svg" }

- `WalkCx`: where an object's rows land (vertex base, object row). `Row`: what the producer measured (local box, spacing, flags, and whether it drew faces).
- The `mod.rs` also declares the modules you type in the following steps; nothing compiles them until `app/mod.rs` names `walk` in step 11.

![Diagram: WalkCx\ vertex base · row · producer · Row\ bounds · spacing · flags](illustrations/06-04.svg)

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mod.rs type -->

## Step 4 · Per-file sweeps

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-8bf646ae4a.svg){ .locator data-strip="illustrations/strip-bb17a255a3.svg" }

- A sheet (planar file) is detected after the walk from the object rows, so producers stay ignorant of documents.
- The sweeps read only this file's rows, against the `Baselines` captured before the walk.

![Diagram: Upload rows · file_extent · mark_sheet · object rows, one per producer](illustrations/06-05.svg)

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/bounds.rs type -->

## Step 5 · Fused mesh topology

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-8bf646ae4a.svg){ .locator data-strip="illustrations/strip-bb17a255a3.svg" }

- One pass over the faces gives the ink lanes everything: unique edges with pen colours, the two faces at each edge, face normals, closedness.
- Edges hang off their low vertex on an intrusive chain; a mesh with sparse vertex keys still indexes in O(1) through `SlotMap`.

![Diagram: Mesh faces · mesh_topology · MeshTopo\ edges · edge_faces · normals](illustrations/06-06.svg)

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=1-48 -->

- Newell normals, not the first three corners: a reflex second corner would invert the normal and turn a flat region into a crease.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=49-95 -->

- Faces are slotted by arrival, not by direction; `opposed` records a winding disagreement instead of declaring the solid open.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=96-173 -->

## Step 6 · Ink: pipes for edges, spheres for vertices

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-8bf646ae4a.svg){ .locator data-strip="illustrations/strip-bb17a255a3.svg" }

- The ink pass reads the topology and the f32 positions by slot, and writes only `SegRows.pipes` and `GlyphRows.spheres`.
- When a pair's winding disagrees, the second normal is negated: the facing test needs two outward normals.

![Diagram: MeshTopo · SegRows.pipes · Incidence CSR · GlyphRows.spheres](illustrations/06-07.svg)

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=1-67 -->

- The rest is the crease test: the cosine between two face normals decides whether a shared edge is a border, a crease, or an interior diagonal nobody should see.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=68-110 -->

- A smooth tessellation inks only borders and creases; a coplanar diagonal is dropped unless `VIEWER_ALL_EDGES` asks for it.
- `pipe_ids` gets the source edge index for an authored mesh and `u32::MAX` for a tessellation seam: selection must never return an invented edge.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=111-146 -->

- Incidence is CSR over the edges: each vertex knows its widest visible edge and every incident edge, so a marker can carry up to six face normals.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=147-192 -->

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=193-250 -->

- One entry point does both lanes in order — pipes, then markers unless `VIEWER_NO_DOTS` — so a caller cannot produce edges without the vertices that belong to them.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=251-268 -->

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs copy lines=269-373 -->

## Step 7 · One mesh into the tables

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-8bf646ae4a.svg){ .locator data-strip="illustrations/strip-bb17a255a3.svg" }

- Gates first: above `MESH_RAW_MIN` triangles a mesh is faces only; a print fill (single width 0) takes the sheet index runs.
- `MeshOpts::SURFACE` marks a tessellation: `FLAG_SMOOTH` tells the marker lane its vertices are samples, and its seams are sampling rather than geometry. `OBJECT` and `ELEMENT` are the authored-mesh presets, which differ in whether an open mesh may be flagged open.

![Diagram: Mesh · walk_mesh · ArenaRows · Ink · Row](illustrations/06-08.svg)

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=1-56 -->

- Named presets keep those three decisions out of the producer bodies.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=57-78 -->

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh.rs copy lines=79-141 -->

- Faces go into the arena with `vids = cx.row`; the ink pass runs only on decorated meshes with a topology.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=142-236 -->

## Step 8 · Curves into the ribbon lane

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-8bf646ae4a.svg){ .locator data-strip="illustrations/strip-bb17a255a3.svg" }

- Lines and polylines become one flat ribbon per span with `FACING_UNKNOWN`: free linework has no faces to cull against.

![Diagram: Line · Polyline · SegRows ribbons · NurbsCurve · walk_nurbscurve](illustrations/06-09.svg)

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=1-61 -->

- A NURBS curve is sampled by turning angle of its control polygon, so a full circle gets the same chord count at any radius.
- `render_position` is the f64 → f32 boundary for every sampled point; a producer already holding f32 endpoints casts them where it builds them.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=62-125 -->

- A NURBS curve reaches the GPU as a polyline, sampled by its own turning rather than a fixed count, and then takes the polyline path. One sampling rule, used everywhere a curve is drawn.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=126-147 -->

## Step 9 · Edge records and the first BRep consumer

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-8bf646ae4a.svg){ .locator data-strip="illustrations/strip-bb17a255a3.svg" }

- Topology records only: which edge, which face, which orientation. The records carry no geometry.

![Diagram: BRep · face Mesh · ArenaRows · EdgeUse · EdgeChain\ records only](illustrations/06-10.svg)

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/brep_edges.rs type -->

- Each BRep face keeps its own vertices and the kernel's normals; nothing is welded across faces, so a planar face never inherits a neighbour's normal.
- `QUALITY` is a display decision: the viewer asks for finer sampling than the kernel default.

<span class="zone-mark" data-strip="illustrations/strip-bb17a255a3.svg" data-zone="Scene + walk"></span>

<!-- file: 06 session_viewer/src/app/walk/brep.rs type -->

## Step 10 · Launch-time knobs

![Where this step sits in the viewer: Shell, with 9 of 11 zones built so far.](illustrations/locator-78232d7410.svg){ .locator data-strip="illustrations/strip-3bd0a898de.svg" }

Presence-only environment flags, read once per process; always false in the browser.

![Diagram: environment flag · knobs.rs\ all_edges · seams · producers](illustrations/06-11.svg)

<span class="zone-mark" data-strip="illustrations/strip-3bd0a898de.svg" data-zone="Shell"></span>

<!-- file: 06 session_viewer/src/app/knobs.rs copy -->

## Step 11 · Wire the producers

![Where this step sits in the viewer: Shell, with 9 of 11 zones built so far.](illustrations/locator-78232d7410.svg){ .locator data-strip="illustrations/strip-3bd0a898de.svg" }

![Diagram: app/mod.rs · walk producers · knobs](illustrations/06-12.svg)

<span class="zone-mark" data-strip="illustrations/strip-3bd0a898de.svg" data-zone="Shell"></span>

<!-- file: 06 session_viewer/src/app/mod.rs type -->

<!-- check: 06 -->

## Step 12 · The fixture becomes a source scene

![Where this step sits in the viewer: Shell, with 9 of 11 zones built so far.](illustrations/locator-78232d7410.svg){ .locator data-strip="illustrations/strip-3bd0a898de.svg" }

- `CadFixture` retains the f64 source objects and a `SourceIdentity` per object row; the GPU only receives prepared tables.
- The same `add` path serves BRep and surface sources, so a row maps back to a GUID without searching triangles.

![Diagram: f64 source objects · SourceIdentity per row · Upload · lib.rs](illustrations/06-13.svg)

<span class="zone-mark" data-strip="illustrations/strip-3bd0a898de.svg" data-zone="Shell"></span>

<!-- file: 06 session_viewer/src/fixture.rs copy -->

<span class="zone-mark" data-strip="illustrations/strip-3bd0a898de.svg" data-zone="Shell"></span>

<!-- file: 06 session_viewer/src/lib.rs type -->


## Step 13 · Flat preview shading

![Where this step sits in the viewer: Page, Shaders, with 9 of 11 zones built so far.](illustrations/locator-9ee3d814c7.svg){ .locator data-strip="illustrations/strip-c2d15201e1.svg" }

The shader ignores vertex normals and shades from the finite face fallback, so a wrong normal contract cannot hide behind lighting.

![Diagram: vertex normal · fs_main · finite face fallback · PhysicalColor](illustrations/06-14.svg)

<span class="zone-mark" data-strip="illustrations/strip-62db6ccc73.svg" data-zone="Shaders"></span>

<!-- file: 06 session_viewer/src/shaders/triangle.wgsl type -->

<span class="zone-mark" data-strip="illustrations/strip-63a57b9919.svg" data-zone="Page"></span>

<!-- file: 06 session_viewer/index.html copy -->

## Check

<!-- checkpoint: 06 -->

Expected:

- A grey shaded planar face fills a large part of the canvas; its four natural boundaries draw as black ink separate from the fill.
- The status reads one object; the inspection JSON lists `sourceObjects` with a GUID and `sourceEdgeIds`.
- Orbit: fill and boundary move together.

If the face is missing, follow producer → `Upload` → arena → draw range. If boundaries float off the face, the two f64 → f32 conversions differ.

![Checkpoint 06: the CAD fixture shaded flat, boundaries drawn as pipes from the face mesh nodes.](screenshots/06.png)

## What changed

<!-- tree: 06 session_viewer/src/app -->

- New producer layer: kernel mesh → `ArenaRows` faces + `SegRows`/`GlyphRows` ink, with source identity kept beside every row.
- Data flow: f64 source → kernel `Mesh` with `u`/`v`/normal attributes → f32 `RenderMesh` → arena; topology → pipes and spheres.

**Production equivalent:** Production keeps this in `src/app/walk/{mod,bounds,encode,mesh,mesh_ink,mesh_topology,curves,brep,brep_edges}.rs` and `src/app/knobs.rs`. The [CAD design record](cad-design.md) documents the producer contract.

## Try

- Append `?top=1` or `?perspective=1`: the fixture is viewed from a fixed camera, which makes a boundary that drifts off its face easy to spot.
- Append `?distance=3` and then `?distance=12`: the pipes keep their pixel width while the faces shrink; the boundary nodes move with the mesh because they are the mesh. (`parse_distance` accepts 1 to 16 and ignores anything else.)
- Append `?thickness=3`: the boundary pipes widen on screen but stay glued to their faces, because their endpoints are face-mesh nodes, not a separately sampled curve.

## Questions and answers


**A producer reports a `Row` and is given a `WalkCx`. What is the boundary this draws, and why does it matter?**

*How to work it out.* `WalkCx` gives positions in the output (vertex base, object row); `Row` reports measurements of the input (box, spacing, flags). What is *absent* from both is the design: the file, the document, the selection, the camera.

*The answer.* A producer turns one geometry into rows and knows nothing else — which is why a sheet is detected after the walk from the object rows, and why a new geometry type costs one producer rather than an edit to the scene.

**Face normals are computed with Newell's method rather than from the first three corners. What goes wrong with three corners?**

*How to work it out.* Make a flat polygon's second corner reflex — push it inward past 180°. The cross product of the first two edges now points the other way while the polygon is unchanged. Authored geometry has reflex corners often.

*The answer.* The face reads as facing backwards, and downstream that is a crease in a surface that has none, or a back face painted red. Newell sums over every edge, so one awkward corner cannot flip the result: for human-authored input, prefer the formula that averages over the one that samples.

**`pipe_ids` stores the source edge index for an authored mesh but `u32::MAX` for a tessellation seam. Why not just number the seams?**

*How to work it out.* A pick returns that id and the viewer tells the user what they selected — so, is there anything in the document to name? A tessellation seam exists only because the surface was cut this finely; mesh it differently and it is gone.

*The answer.* Numbering it would hand selection an invented identity: the user clicks something that is not in their model. Refusing is the correct answer — the same refusal as lesson 08's periodic seams.

**Face keys are sorted before their normals are accumulated. What bug does the sort prevent?**

*How to work it out.* Float addition is not associative: `(a + b) + c` and `a + (b + c)` differ in the last bits. What decides the order here is iteration over a hash map, which is not stable.

*The answer.* Without the sort the same mesh produces different bytes on different runs, which breaks every hash the course verifies and makes bugs unreproducible. Determinism is something you write down.

**What you should be able to do now**

Name the four things every producer packs and why: pen width → world radius (the GPU works in world units), colour → RGBA8 (four bytes, and colour needs no more), unit normal → 16-bit octahedral code (a unit vector has two degrees of freedom), two normals → one `facing` word (the marker and edge lanes need adjacency, not geometry). Then predict which a point-cloud producer skips: facing and normals — a point has no adjacency, which is why `FACING_UNKNOWN` exists.

## Next

[07 · Shared boundaries](07-boundaries.md): one canonical chain per BRep edge, constrained into every incident face, drawn from the exact mesh nodes.
