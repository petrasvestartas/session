# 06 · CAD face contract

## You are building

```mermaid
flowchart TB
    S["BRep / NurbsSurface<br/>(f64 source)"] -- "face_meshes_q / from_u_v_q" --> M["kernel Mesh per face<br/>positions · u,v · normals"]
    M -- "to_render" --> R["RenderMesh (f32)"]
    R -- "push_face" --> A["ArenaRows<br/>verts · vids · idx"]
    M -- "mesh_topology" --> T["MeshTopo<br/>edges · edge_faces · normals"]
    T -- "edges_and_dots" --> I["SegRows pipes<br/>GlyphRows spheres"]
    A & I -- "Upload" --> G["GPU"]
```

![One face, three representations: BRep source in f64, kernel mesh with u,v, normals and boundary tags, viewer rows in f32 that keep the face and edge identities.](illustrations/cad-contract.svg)

## Starting point

- Checkpoint 05: hand-built quads, strokes and markers in `fixture.rs`; physical depth and visible ink work.
- Nothing yet reads a Session `Mesh`, `BRep` or `NurbsSurface`. This lesson adds the whole `app/walk` producer layer and its first CAD consumer.
- A shading crease is a kernel decision; the viewer only carries it. One kernel edit comes first.

<!-- supplied: 06 -->

<!-- step-status: start -->

**Does it compile yet?** Yes, after every step of this lesson — `cargo check` was run at the end of each one to make sure. A step that writes a file Rust has not been told about yet compiles without checking any of it, so keep going to the checkpoint: that build is the real test.

<!-- step-status: end -->

## Step 1 · Kernel: one-sided normals at a C0 knot

![Where this step sits in the viewer: Kernel, with 8 of 11 zones built so far.](illustrations/locator-ac4fbc5f7d.svg)

- A knot repeated `degree` times folds the surface; averaging normals across that fold makes a sharp edge look rounded.
- The grid mesher splits a shading vertex on the crease side: same position and `u`/`v`, different normal, so the split never invents a CAD vertex.
- Face keys are sorted before accumulation: float sums are order-dependent, and map order must not reach the mesh bytes.

```mermaid
flowchart TB
    K["repeated knot"] -- "sorted face keys" --> N["accumulated normals"]
    N -- "split_crease_normals" --> V["two shading vertices<br/>same u,v"]
    style V fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_rust/src/remesh_nurbssurface_grid.rs type hunks=1-4 -->

`split_crease_normals` is `pub(crate)`: the trimmed mesher in `nurbssurface_trimmed.rs` shares it.

<!-- file: 06 session_rust/src/remesh_nurbssurface_grid.rs type hunks=5-5 -->

<!-- check: 06 -->

## Step 2 · Row encodings

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- Every producer packs the same four things: pen width → world radius, colour → RGBA8, unit normal → 16-bit octahedral code, two normals → one `facing` word.
- `FACING_UNKNOWN` is all ones and means "no adjacency, always draw"; `pack_facing` steps around that value.

```mermaid
flowchart LR
    W["pen width"] -- "encode_width" --> R["world radius"]
    C["colour"] -- "pack_rgba" --> A["RGBA8"]
    N["unit normal"] -- "oct16" --> O["16-bit code"]
    P["two normals"] -- "pack_facing" --> F["facing word"]
    style R fill:#f0bcdb,stroke:#ce4095,color:#111
    style F fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/walk/encode.rs type -->

## Step 3 · What a producer reports

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- `WalkCx`: where an object's rows land (vertex base, object row). `Row`: what the producer measured (local box, spacing, flags, thickness).
- The `mod.rs` also declares the modules you type in the following steps; nothing compiles them until `app/mod.rs` names `walk` in step 11.

```mermaid
flowchart LR
    C["WalkCx<br/>vertex base · row"] --> P["producer"]
    P --> R["Row<br/>bounds · spacing · flags"]
    style C fill:#f0bcdb,stroke:#ce4095,color:#111
    style R fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/walk/mod.rs type -->

## Step 4 · Per-file sweeps and thickness

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- A sheet (planar file) is detected after the walk from the object rows, so producers stay ignorant of documents.
- Thickness is measured along the mesh's own dominant face normals, not the axis-aligned box: a rotated plate measures its plate thickness.

```mermaid
flowchart LR
    U["Upload rows"] -- "Baselines::capture" --> B["file_extent"]
    B -- "is_planar" --> S["mark_sheet"]
    T["tris + normals"] -- "mesh_thickness" --> K["thickness"]
    style S fill:#f0bcdb,stroke:#ce4095,color:#111
    style K fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/walk/bounds.rs type -->

## Step 5 · Fused mesh topology

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- One pass over the faces gives the ink lanes everything: unique edges with pen colours, the two faces at each edge, face normals, closedness.
- Edges hang off their low vertex on an intrusive chain; a mesh with sparse vertex keys still indexes in O(1) through `SlotMap`.

```mermaid
flowchart LR
    M["Mesh faces"] -- "SlotMap" --> T["mesh_topology"]
    T --> E["MeshTopo<br/>edges · edge_faces · normals"]
    style E fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=1-48 -->

- Newell normals, not the first three corners: a reflex second corner would invert the normal and turn a flat region into a crease.

<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=49-95 -->

- Faces are slotted by arrival, not by direction; `opposed` records a winding disagreement instead of declaring the solid open.

<!-- file: 06 session_viewer/src/app/walk/mesh_topology.rs type lines=96-173 -->

## Step 6 · Ink: pipes for edges, spheres for vertices

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- The ink pass reads the topology and the f32 positions by slot, and writes only `SegRows.pipes` and `GlyphRows.spheres`.
- When a pair's winding disagrees, the second normal is negated: the facing test needs two outward normals.

```mermaid
flowchart LR
    T["MeshTopo"] -- "push_pipes" --> P["SegRows.pipes"]
    T -- "incidence" --> I["Incidence CSR"]
    I -- "push_markers" --> S["GlyphRows.spheres"]
    style P fill:#f0bcdb,stroke:#ce4095,color:#111
    style S fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=1-67 -->

- The rest is the crease test: the cosine between two face normals decides whether a shared edge is a border, a crease, or an interior diagonal nobody should see.

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=68-110 -->

- A smooth tessellation inks only borders and creases; a coplanar diagonal is dropped unless `VIEWER_ALL_EDGES` asks for it.
- `pipe_ids` gets the source edge index for an authored mesh and `u32::MAX` for a tessellation seam: selection must never return an invented edge.

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=111-146 -->

- Incidence is CSR over the edges: each vertex knows its widest visible edge and every incident edge, so a marker can carry up to six face normals.

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=147-192 -->

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=193-250 -->

- One entry point does both lanes in order — pipes, then markers unless `VIEWER_NO_DOTS` — so a caller cannot produce edges without the vertices that belong to them.

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs type lines=251-268 -->

<!-- file: 06 session_viewer/src/app/walk/mesh_ink.rs copy lines=269-373 -->

## Step 7 · One mesh into the tables

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- Gates first: above `MESH_RAW_MIN` triangles a mesh is faces only; a print fill (single width 0) takes the sheet index runs.
- `MeshOpts::SURFACE` marks a tessellation: `FLAG_SMOOTH` tells the marker lane its vertices are samples, and its seams are sampling rather than geometry. `OBJECT` and `ELEMENT` are the authored-mesh presets, which differ in whether an open mesh may be flagged open.

```mermaid
flowchart LR
    M["Mesh"] -- "MeshOpts gates" --> W["walk_mesh"]
    W -- "faces" --> A["ArenaRows"]
    W -- "edges_and_dots" --> I["Ink"]
    W --> R["Row"]
    style W fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=1-56 -->

- `MeshOpts` names the three decisions a caller makes about a mesh: whether sheet lanes apply, whether an open mesh is allowed, and whether it is a tessellation. Named presets keep those decisions out of the producer bodies.

<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=57-78 -->

<!-- file: 06 session_viewer/src/app/walk/mesh.rs copy lines=79-141 -->

- Faces go into the arena with `vids = cx.row`; the ink pass runs only on decorated meshes with a topology.

<!-- file: 06 session_viewer/src/app/walk/mesh.rs type lines=142-236 -->

## Step 8 · Curves into the ribbon lane

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- Lines and polylines become one flat ribbon per span with `FACING_UNKNOWN`: free linework has no faces to cull against.

```mermaid
flowchart TB
    L["Line · Polyline"] -- "walk_line · walk_polyline" --> S["SegRows ribbons"]
    C["NurbsCurve"] -- "turning_degrees" --> N["walk_nurbscurve"]
    N -- "render_position" --> S
    style S fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=1-61 -->

- A NURBS curve is sampled by turning angle of its control polygon, so a full circle gets the same chord count at any radius.
- `render_position` is the single f64 → f32 boundary for every producer.

<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=62-125 -->

- A NURBS curve reaches the GPU as a polyline, sampled by its own size rather than a fixed count, and then takes the polyline path. One sampling rule, used everywhere a curve is drawn.

<!-- file: 06 session_viewer/src/app/walk/curves.rs type lines=126-147 -->

## Step 9 · Edge records and the first BRep consumer

![Where this step sits in the viewer: Scene + walk, with 9 of 11 zones built so far.](illustrations/locator-bf1f46ef56.svg)

- Topology records only: which edge, which face, which orientation. The records carry no geometry.

```mermaid
flowchart LR
    B["BRep"] -- "face_meshes_q · QUALITY" --> F["face Mesh"]
    F -- "push_face" --> A["ArenaRows"]
    B --> E["EdgeUse · EdgeChain<br/>records only"]
    style A fill:#f0bcdb,stroke:#ce4095,color:#111
    style E fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/walk/brep_edges.rs type -->

- Each BRep face keeps its own vertices and the kernel's normals; nothing is welded across faces, so a planar face never inherits a neighbour's normal.
- `QUALITY` is a display decision: the viewer asks for finer sampling than the kernel default.

<!-- file: 06 session_viewer/src/app/walk/brep.rs type -->

## Step 10 · Launch-time knobs

![Where this step sits in the viewer: Shell, with 9 of 11 zones built so far.](illustrations/locator-82bc3205bd.svg)

Presence-only environment flags, read once per process; always false in the browser.

```mermaid
flowchart LR
    E["environment flag"] -- "OnceLock" --> K["knobs.rs<br/>all_edges · seams"]
    K --> P["producers"]
    style K fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/knobs.rs copy -->

## Step 11 · Wire the producers

![Where this step sits in the viewer: Shell, with 9 of 11 zones built so far.](illustrations/locator-82bc3205bd.svg)

```mermaid
flowchart LR
    A["app/mod.rs"] -- "pub mod walk" --> W["walk producers"]
    A -- "pub mod knobs" --> K["knobs"]
    style A fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/app/mod.rs type -->

<!-- check: 06 -->

## Step 12 · The fixture becomes a source scene

![Where this step sits in the viewer: Shell, with 9 of 11 zones built so far.](illustrations/locator-82bc3205bd.svg)

- `CadFixture` retains the f64 source objects and a `SourceIdentity` per object row; the GPU only receives prepared tables.
- The same `add` path serves BRep and surface sources, so a row maps back to a GUID without searching triangles.

```mermaid
flowchart LR
    S["f64 source objects"] -- "CadFixture::add" --> I["SourceIdentity per row"]
    S -- "walk_brep · walk_surface" --> U["Upload"]
    U --> L["lib.rs"]
    style I fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/fixture.rs copy -->

<!-- file: 06 session_viewer/src/lib.rs type -->

- The teaching shell is re-typed whole because its module list and its `render` are what wire the lane you just built; the production `App` replaces it in lesson 12.

## Step 13 · Flat preview shading

![Where this step sits in the viewer: Page, Shaders, with 9 of 11 zones built so far.](illustrations/locator-57da0132de.svg)

The shader ignores vertex normals and shades from the finite face fallback, so a wrong normal contract cannot hide behind lighting.

```mermaid
flowchart LR
    V["vertex normal"] -. "ignored" .-> S["fs_main"]
    F["finite face fallback"] --> S
    S --> C["PhysicalColor"]
    style S fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 06 session_viewer/src/shaders/triangle.wgsl type -->

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
- Append `?distance=3` and then `?distance=12`: the pipes keep their pixel width while the faces shrink; the boundary nodes move with the mesh because they are the mesh. (`parse_distance` accepts 1 to 16 and ignores anything else, so a smaller value is not a zoom — it is a no-op.)
- Append `?thickness=3`: the boundary pipes widen on screen but stay glued to their faces, because their endpoints are face-mesh nodes, not a separately sampled curve.

## Questions and answers

From here on the questions are less about "what does this call do" and more about "why is it built this way".

**A producer reports a `Row` and is given a `WalkCx`. What is the boundary this draws, and why does it matter?**

*How to work it out.* Look at what is in each type. `WalkCx` carries positions in the output — vertex base, object row. `Row` carries measurements of the input — local box, spacing, flags. Now ask what is *absent* from both: the file, the document, the selection, the camera. That absence is the design.

*The answer.* A producer knows how to turn one geometry into rows and nothing else. That is why a sheet is detected after the walk from the object rows rather than inside a producer, and why adding a new geometry type later means writing one producer instead of editing the scene. The boundary is what makes the walk layer extensible.

**Face normals are computed with Newell's method rather than from the first three corners. What goes wrong with three corners?**

*How to work it out.* Take a flat polygon and make its second corner reflex — push it inward so the interior angle exceeds 180°. The cross product of the first two edges now points the other way, while the polygon is unchanged. Ask how often authored geometry has a reflex corner: often.

*The answer.* The face is reported as facing backwards, and downstream that becomes a crease in a surface that has none, or a back face painted red. Newell sums over every edge, so one awkward corner cannot flip the result. The general rule: for human-authored input, prefer the formula that averages over the one that samples.

**`pipe_ids` stores the source edge index for an authored mesh but `u32::MAX` for a tessellation seam. Why not just number the seams?**

*How to work it out.* Ask what happens after a pick returns that id: the viewer looks it up and tells the user what they selected. So the question becomes — is there anything in the document to name? A tessellation seam exists only because the surface was cut this finely; mesh it differently and it is gone.

*The answer.* Numbering it would make selection return an invented identity, and the user could click on something that does not exist in their model. Refusing to answer is the correct answer, and it is the same refusal as lesson 08's periodic seams.

**Face keys are sorted before their normals are accumulated. What bug does the sort prevent?**

*How to work it out.* Float addition is not associative: `(a + b) + c` and `a + (b + c)` can differ in the last bits. Then ask what decides the order here — iteration over a hash map, which is not stable.

*The answer.* Without the sort, the same mesh can produce different bytes on different runs, which breaks every hash the course verifies and makes bugs unreproducible. Determinism does not happen by itself; it is something you write down.

**What you should be able to do now**

Name the four things every producer packs and why each is packed rather than passed as-is: pen width → world radius (the GPU works in world units), colour → RGBA8 (four bytes instead of sixteen, and colour needs no more), unit normal → 16-bit octahedral code (two bytes, and a unit vector has only two degrees of freedom), two normals → one `facing` word (the marker and edge lanes need adjacency, not geometry). Then predict which a point-cloud producer skips: facing and normals — a point has no adjacency, which is why `FACING_UNKNOWN` exists.

## Next

[07 · Shared boundaries](07-boundaries.md): one canonical chain per BRep edge, constrained into every incident face, drawn from the exact mesh nodes.
