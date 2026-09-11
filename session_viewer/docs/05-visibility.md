# 05 · Depth and visible ink

## You are building

```mermaid
flowchart TB
    faces["opaque faces<br/>triangle.wgsl"] -- "fs_main → PhysicalColor" --> phys["physical pass<br/>depth + gradient targets"]
    back["background + grid<br/>backdrop.rs"] -- "depth Always / ReadOnly" --> phys
    phys -- "textureLoad depth, gradient" --> ink["ink pass<br/>ink_visibility.wgsl"]
    ink -- "carry depth to the stroke axis" --> test{"ink_visible"}
    test -- "true" --> cov["stroke coverage color"]
    test -- "false" --> discard
```

![Reversed depth, and why a thick stroke must transfer the surface depth to its axis before comparing.](illustrations/ink-visibility.svg)

## Starting point

- Checkpoint 04d (all drawing modules present): faces, strokes, markers and clouds draw into one color target with a single-sample depth buffer, and every stroke fragment compares its own depth at its own pixel.
- Rear edges shine through solids at grazing angles: a thick stroke covers samples beside its axis, and those samples belong to a surface whose depth changes sharply within one pixel.
- Depth is reversed: near is larger, far approaches zero.

<!-- step-status: start -->

**Does it compile yet?** Yes, after every step of this lesson — `cargo check` was run at the end of each one to make sure. A step that writes a file Rust has not been told about yet compiles without checking any of it, so keep going to the checkpoint: that build is the real test.

<!-- step-status: end -->

## Step 1 · The physical contract shared by every shader

![Where this step sits in the viewer: Shaders, with 8 of 11 zones built so far.](illustrations/locator-53c0d29f7b.svg){ .locator data-strip="illustrations/strip-ef21ae124d.svg" }

Two constants and two output structs, appended to every shader module. `physical_gradient` is the rasterizer's own depth slope of the winning primitive, scaled so `Rg16Float` keeps it.

| Contract | Where it lives |
|---|---|
| `Depth32Float` attachment, cleared to `0.0` (reverse-Z far) | `targets.rs::begin_faces` |
| Solids write with `CompareFunction::Greater` (`DepthMode::Opaque`) | `pipelines/mod.rs` |
| Grid tests without writing (`DepthMode::ReadOnly`), background `DepthMode::Always` | `backdrop.rs` below |
| `@location(1) gradient: vec2<f32>` beside every physical color/ID | `physical.wgsl` below |

```mermaid
flowchart LR
    A["fs_main depth"] -- "physical_gradient" --> B["PhysicalColor<br/>color + gradient"]
    C["fs_id"] --> E["PhysicalId<br/>id + gradient"]
    style B fill:#fa9ebc,stroke:#fa9ebc,color:#111
    style E fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/physical.wgsl type -->

## Step 2 · Backdrop shaders

![Where this step sits in the viewer: Shaders, with 8 of 11 zones built so far.](illustrations/locator-53c0d29f7b.svg){ .locator data-strip="illustrations/strip-ef21ae124d.svg" }

- The background is one oversized triangle at `w = 1.0`, depth `Always`, so it never occludes.
- The grid builds fifty vertices from `vertex_index` alone; it subtracts `line.anchor` because instance rows are rebased on the camera anchor.
- Both return `PhysicalColor` with a zero gradient: neither is a surface ink can be carried across.

```mermaid
flowchart LR
    V["vertex_index"] -- "CORNERS" --> B["background.wgsl<br/>depth Always"]
    V -- "FLOOR lines" --> G["grid.wgsl<br/>line.anchor"]
    B & G -- "zero gradient" --> P["PhysicalColor"]
    style B fill:#fa9ebc,stroke:#fa9ebc,color:#111
    style G fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/background.wgsl type -->

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/grid.wgsl type -->

- Fifty vertices from the vertex index alone, no buffer. It subtracts `line.anchor` because the instance rows are rebased on the camera anchor and the grid has to agree with them.

## Step 3 · The backdrop lane

![Where this step sits in the viewer: Lanes, with 8 of 11 zones built so far.](illustrations/locator-be21b3fc34.svg){ .locator data-strip="illustrations/strip-445a1edf20.svg" }

- One owner for two pipelines; no buffers, no upload, `retarget` when the sample count changes.
- `draw_grid` binds `mvp` and the `line` block, matching `@group(0)`/`@group(1)` in `grid.wgsl`.

```mermaid
flowchart LR
    S["SHADERS"] -- "build_background" --> L["BackdropLane"]
    S -- "build_grid" --> L
    L -- "draw_background" --> F["faces pass"]
    L -- "draw_grid · Binds" --> F
    style L fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 05 session_viewer/src/engine/gpu/backdrop.rs type -->

<!-- check: 05 -->

## Step 4 · The ink visibility test

![Where this step sits in the viewer: Shaders, with 8 of 11 zones built so far.](illustrations/locator-53c0d29f7b.svg){ .locator data-strip="illustrations/strip-ef21ae124d.svg" }

A stroke is drawn as a ribbon of fragments around its mathematical axis. The physical depth at a fragment beside the axis belongs to whatever surface is there, not to the axis:

```text
      fragment ●─────── stroke footprint ───────● fragment
                 \                              /
                  \      axis (depth d)        /
   surface ────────●─────────────────────────●──────── surface
                   z0            depth varies across the footprint
```

Comparing `z0` with `d` directly hides ink on its own face. Instead the physical gradient carries the surface depth from the fragment to the axis point, and only that predicted depth is compared with the axis.

Replace the whole shader in five pieces.

### 4a · Bindings, tolerances and the axis record

- `scene_gradient_*` are the new attachments from step 1; `SCENE_MSAA` picks the multisampled view.
- Tolerances are expressed in float precision and rasterizer snapping, not in world units.

```mermaid
flowchart LR
    G["scene_gradient_*<br/>@group(2) @binding(4/5)"] --> K["ink_visibility.wgsl"]
    T["DEPTH_REL_TOL · SLOPE_PX · KINK"] --> K
    K --> X["InkAxis record"]
    style X fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=1-43 -->

### 4b · Reading depth and fitting a neighbouring pair

- Outside the viewport counts as cleared, so a stroke overhangs the canvas edge.
- `ink_pair_planar` accepts two adjacent texels as one surface only when their slopes agree within `KINK`; a step to another surface is many times the slope.

```mermaid
flowchart TB
    P["pixel + sample"] -- "textureLoad" --> D["ink_depth"]
    D --> T["ink_tolerance"]
    D -- "two texels" --> N["ink_pair_planar"]
    N -- "slopes within KINK" --> C["ink_carry_visible"]
    style N fill:#fa9ebc,stroke:#fa9ebc,color:#111
    style C fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=44-101 -->

### 4c · Carrying a stroke fragment's surface to the axis

- `ink_axis_visible` fits a plane from the fragment's texel and one neighbour away from the stroke, then evaluates it at the axis.
- `ink_carry_visible` is one-sided: a farther texel can never hide, a nearer texel hides unless its surface passes through the axis.

```mermaid
flowchart TB
    F["fragment texel"] -- "ink_step" --> N["neighbour texel"]
    F & N -- "fit plane" --> A["ink_axis_visible"]
    A -- "predicted depth" --> C["ink_carry_visible"]
    style A fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=102-137 -->

### 4d · Discs: markers stand or fall with their centre

A marker is a camera-facing disc; its rim must not be uncovered by a grazing surface that crosses the disc's depth within a few pixels.

```mermaid
flowchart LR
    C["disc centre + depth"] --> F["ink_disc_fragment_visible"]
    C -- "toward_eye" --> V["ink_disc_visible"]
    F --> V
    style F fill:#fa9ebc,stroke:#fa9ebc,color:#111
    style V fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=138-187 -->

### 4e · Corner fits and the fast path

- `ink_disc_source_hidden` tries each quadrant so a face boundary cannot discard a valid fit.
- `ink_visible` is the entry point strokes call: when the primitive's own gradient is valid, one `textureLoad` and a dot product decide; the neighbouring-pair fit is the fallback for gradients outside the attachment's range.
- A neighbouring triangle is treated as an infinite plane: the fit extends its slope past its edges.

```mermaid
flowchart LR
    Q["four quadrants"] --> H["ink_disc_source_hidden"]
    G["own gradient valid"] -- "one textureLoad" --> V["ink_visible"]
    G -- "else" --> P["ink_axis_visible fallback"]
    P --> V
    style V fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=188-236 -->

## Step 5 · Shaders emit the gradient

![Where this step sits in the viewer: Shaders, with 8 of 11 zones built so far.](illustrations/locator-53c0d29f7b.svg){ .locator data-strip="illustrations/strip-ef21ae124d.svg" }

Every fragment that writes physical depth also returns its gradient. Face shaders return the real slope; splats, sheets and ID passes return zero because they are not surfaces ink can be carried across.

```mermaid
flowchart LR
    T["triangle.wgsl fs_main"] -- "real slope" --> P["PhysicalColor"]
    S["splat · splat_resolve"] -- "zero gradient" --> P
    O["text_outline.wgsl"] -- "fs_physical_id" --> I["PhysicalId"]
    style P fill:#fa9ebc,stroke:#fa9ebc,color:#111
    style I fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/triangle.wgsl type -->

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/splat.wgsl type -->

- The point shader gains the physical metadata output, so a splat writes a gradient like every other surface - a zero gradient, because a point is not a surface ink can be carried across.

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/splat_resolve.wgsl type -->

- The resolve is where a private pass rejoins the shared one: it reads the lane's own depth and colour, lights each point from its neighbours, and writes `frag_depth` so the scene's depth test does the rest.

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 05 session_viewer/src/shaders/text_outline.wgsl type -->

- Imported lettering returns the physical output like every fragment in this pass, but its pipelines are depth read-only: the glyphs are drawn against the depth their page already wrote, never lit and never re-spaced.

## Step 6 · Targets: the gradient attachment and a sample budget

![Where this step sits in the viewer: GPU core, with 8 of 11 zones built so far.](illustrations/locator-7e63ed245a.svg){ .locator data-strip="illustrations/strip-203427a3dc.svg" }

- `Rg16Float` gradient texture beside depth; single/multisampled views are swapped exactly like the depth views so bind groups stay valid at both sample counts.
- `begin_faces` clears the gradient to transparent alongside the reverse-Z depth clear.
- `msaa_budget`/`samples_for` decide the sample count from the adapter type and pixel count; multisampling smooths hard face edges only, ribbons and discs antialias themselves.

```mermaid
flowchart TB
    A["adapter type + pixels"] -- "msaa_budget" --> S["samples_for"]
    S --> T["Targets<br/>depth + Rg16Float gradient"]
    T -- "begin_faces clears" --> F["faces pass"]
    style T fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 05 session_viewer/src/engine/gpu/targets.rs type -->

## Step 7 · Pipelines: one flag adds the second color target

![Where this step sits in the viewer: GPU core, with 8 of 11 zones built so far.](illustrations/locator-7e63ed245a.svg){ .locator data-strip="illustrations/strip-203427a3dc.svg" }

- `PipelineDesc::physical()` appends the `Rg16Float` target; `ReadOnlyEqual` pipelines keep the gradient their face already wrote by masking their writes.
- `module` appends `physical.wgsl` after `normals.wgsl`, so every shader sees `PhysicalColor`.

```mermaid
flowchart LR
    D["PipelineDesc"] -- ".physical()" --> P["second target<br/>Rg16Float"]
    M["module()"] -- "append physical.wgsl" --> S["shader source"]
    L["scene_gradient entry"] --> B["ink bind group layout"]
    style P fill:#fa9ebc,stroke:#fa9ebc,color:#111
    style L fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 05 session_viewer/src/engine/pipelines/mod.rs type -->

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 05 session_viewer/src/engine/pipelines/layouts.rs type -->

- Group 2 grows: the ink variant now carries the depth and gradient views. This is the binding change that makes the visibility test possible at all.

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 05 session_viewer/src/engine/gpu/instance.rs type -->

- The only change is to the mirror test: `physical.wgsl` joins the shader sources it parses, so the new physical output is checked against the Rust side like everything else.

## Step 8 · Lanes read and write the gradient

![Where this step sits in the viewer: GPU core, Lanes, with 8 of 11 zones built so far.](illustrations/locator-7cc87e5e20.svg){ .locator data-strip="illustrations/strip-5fb45cfc9e.svg" }

- The ink bind group gains bindings 4 and 5: `@group(2) @binding(4/5)` in step 4a.
- The arena, splats and outline text build their pipelines with `.physical()`; the arena also gains a selection-mask pipeline.

```mermaid
flowchart TB
    G["gradient views"] -- "bindings 4 and 5" --> O["objects.rs ink group"]
    O --> A["arena · draw_selection_mask"]
    O --> S["splat"]
    O --> X["text_outline retarget"]
    style O fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 05 session_viewer/src/engine/gpu/objects.rs type -->

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 05 session_viewer/src/engine/gpu/arena.rs type -->

- Two changes: the face pipelines gain `.physical()`, which adds the gradient target, and a selection-mask pipeline appears for the coverage the outline pass will read.

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 05 session_viewer/src/engine/gpu/splat.rs type -->

- `.physical()` on the ID pipeline and the resolve: the cloud now writes the same metadata as every other surface, which is what lets ink judge itself against a point cloud.

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 05 session_viewer/src/engine/gpu/text_outline.rs type -->

- The outline lane joins the physical pass with `.physical()`, the one flag that adds the gradient target to a pipeline.

## Step 9 · Wire the lane and the sample count

![Where this step sits in the viewer: GPU core, with 8 of 11 zones built so far.](illustrations/locator-7e63ed245a.svg){ .locator data-strip="illustrations/strip-203427a3dc.svg" }

- `retarget` rebuilds targets, ink bind groups and every lane's pipelines when the sample count flips, and only then.
- The backdrop draws first inside `begin_faces`, before any geometry.

```mermaid
flowchart LR
    R["resize"] -- "samples_for" --> T["retarget"]
    T --> P["targets · ink groups · lanes"]
    B["BackdropLane"] -- "first in begin_faces" --> F["frame"]
    style T fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 05 session_viewer/src/engine/gpu/mod.rs type -->

## Step 10 · The fixture and the page

![Where this step sits in the viewer: Page, Shell, with 8 of 11 zones built so far.](illustrations/locator-c88f51e1ff.svg){ .locator data-strip="illustrations/strip-75e0f98df5.svg" }

The grey box and the sloping floor are the shapes the visibility test is judged on.

```mermaid
flowchart LR
    X["fixture.rs<br/>grey_box · floor"] -- "scene()" --> U["Upload"]
    Q["?fixture · ?distance"] -- "parse_distance" --> L["lib.rs"]
    U --> L
    style X fill:#fa9ebc,stroke:#fa9ebc,color:#111
```

<span class="zone-mark" data-strip="illustrations/strip-6e964d1d1f.svg" data-zone="Shell"></span>

<!-- file: 05 session_viewer/src/fixture.rs copy -->

<span class="zone-mark" data-strip="illustrations/strip-6e964d1d1f.svg" data-zone="Shell"></span>

<!-- file: 05 session_viewer/src/lib.rs type -->

- Wiring a lane into the shell costs a hunk or two: construct it where the others are built, and report it. That is the whole price of adding a lane to this facade.

<span class="zone-mark" data-strip="illustrations/strip-a7bdebbf9f.svg" data-zone="Page"></span>

<!-- file: 05 session_viewer/index.html copy -->

## Check

<!-- checkpoint: 05 -->

Expected:

- A grey box on a white background, twelve red edges, black corner markers.
- Orbit: edges on the far side of the box disappear behind its faces; front edges stay at full width up to the corners.
- `?fixture=floor`: magenta lines just under the sloping floor stay hidden; the red line on the floor stays visible.
- `?top`, `?perspective`, `?distance=N` select the view for repeatable inspection.

If every edge disappears, compare the depth clear and compare function against the table in step 1. If hidden edges show through, check that the face pipeline uses `.physical()` and that `fs_main` returns `physical_gradient(in.pos.z)`.

![Checkpoint 05: hidden lines stay hidden while visible strokes and corners stay readable, over the grid and backdrop.](screenshots/05.png)

## What changed

<!-- tree: 05 session_viewer/src -->

- Data flow: face fragment → depth + gradient attachments → stroke fragment loads both → `ink_visible` predicts the surface depth at the axis → coverage or discard.
- New lane: `BackdropLane` (background, grid).
- Sample count is chosen per frame from geometry and adapter budget.

**Production equivalent:** `src/engine/gpu/targets.rs`, `backdrop.rs`, `src/shaders/physical.wgsl`, `ink_visibility.wgsl`, `grid.wgsl`, `background.wgsl`; the page entry is `src/lib.rs`. The fixture is teaching scaffolding with no production counterpart: lesson 12 deletes `src/fixture.rs` and the manifest loader takes its place.

## Try

- Append `?nogrid=1`: the construction grid is gone; it was drawn by `backdrop.rs` with a read-only depth test.
- Orbit until a stroke passes behind the box: the covered span disappears cleanly, without a global depth offset.
- Append `?msaa=4` and compare the stroke fringe with `msaa=1`: multisampling changes coverage, never the visibility decision.

## Questions and answers

This lesson is the conceptual centre of the viewer. If only one lesson is worth being able to reconstruct, it is this one.

**Why can a stroke fragment not simply compare its own depth with the depth buffer?**

*How to work it out.* Draw the situation in cross-section. A stroke is a *ribbon* several pixels wide around a mathematical axis. A fragment on the edge of that ribbon reads the depth buffer at *its own* pixel — which is the surface under that pixel, not the surface under the axis. Now put the stroke on the face it belongs to: the axis is exactly on the surface, but the edge fragments sit over a surface that is a fraction nearer or further.

*The answer.* Half the ribbon loses a naive comparison and the line stitches. The fix is to carry the surface depth from the fragment's pixel to the axis using the depth gradient the face pass stored, and compare only the predicted depth at the axis with the axis itself.

**What exactly is stored in the gradient attachment, and who writes zero into it?**

*How to work it out.* Ask what you need to travel from one pixel to another along a surface: the rate at which the surface's depth changes per pixel — its screen-space slope. Then ask which things on screen are not surfaces you can slide along.

*The answer.* The winning primitive's own depth slope, scaled to survive `Rg16Float`. Surfaces write their real slope; the background, the grid, splats, sheets and every ID pass write zero, because extrapolating across them is meaningless. A zero gradient does not mean "flat" — it means "do not extrapolate me".

**Reverse-Z needs three things to agree, and you have now seen all three in code. Name them.**

*How to work it out.* Same reasoning as lesson 02, now with the code in front of you: the projection, the clear, the compare.

*The answer.* Near and far swapped in the projection; the depth attachment cleared to `0.0`; the compare `Greater`. The lesson's own troubleshooting note says exactly this, and being able to derive it beats remembering it — if every edge disappears, one of the three is wrong.

**Multisampling is chosen per frame from the adapter and the pixel count. Why does it never change the visibility decision?**

*How to work it out.* Separate the two things MSAA affects. It changes how many samples a triangle covers within a pixel — a coverage question. The ink test asks whether the axis is behind a surface — a depth question, computed per fragment from values that do not depend on the sample count.

*The answer.* Because visibility is decided from depth and gradient, not from coverage. MSAA smooths hard face edges; the ink test still asks the same question at the same place. `?msaa=4` against `?msaa=1` is the experiment that shows it: the fringe changes, the decision does not.

**What you should be able to do now**

Draw the frame on paper: which pass writes depth, which reads it, what attachments exist, where the backdrop sits. Correct: the face pass clears colour, depth and gradient and writes all three (the backdrop drawing first inside it); the ink pass loads colour, holds depth read-only and samples depth and gradient through group 2. Then predict a stroke on the far side of a box: its axis loses against the box's carried depth, so its fragments discard. Lessons 12, 17 and 18 extend this picture; they never replace it.

## Next

[06 · CAD face contract](06-cad-contract.md): BRep faces, surfaces and boundary records flow from the shared kernel into display data.
