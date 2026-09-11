# 04c · Markers

## You are building

```mermaid
flowchart TB
    R["fixture: GlyphPoint rows"] --> G["GlyphRows<br/>spheres · dots"]
    G -- "GlyphLane::append" --> G3["group 3<br/>glyphs: array&lt;GlyphPoint&gt;"]
    G3 --> SP["sphere.wgsl<br/>quad Template × instance_index<br/>culled by incident faces"]
    G3 --> DT["glyph.wgsl<br/>3 verts per dot, incircle is the disc"]
    SP --> FS["coverage · ink_disc_visible"]
    DT --> FS
```

Vertex input of the marker pipeline (`pipelines::template_layout`):

| Slot | Rust | WGSL |
|---|---|---|
| 0 | `Template.vbo`, 12-byte positions, `step_mode: Vertex` | `@location(0) tmpl: vec3<f32>` |
| — | `draw_indexed(.., 0..spheres.len())` | `@builtin(instance_index) gi` selects the glyph row |

The dot pipeline binds no vertex buffer: `@builtin(vertex_index) / 3` is the row.

![A sphere is four template corners pushed out by the pixel radius plus the feather; a free dot is one equilateral triangle whose incircle is the disc.](illustrations/markers.svg)

## Starting point

- Checkpoint 04b: meshes and strokes. Both ink lanes share the visibility rule appended by `ink_module`.
- Markers are the vertex-sized ink: mesh vertex markers (solid lane) and free points (flat lane), one 48-byte row for both.

## Step 1 · The glyph row

- `center` is a `vec3` in WGSL, so the row is 48 bytes with `radius` in the padding slot.
- `facing` plus `facing_ext` hold up to six incident face normals as oct16 pairs; a marker hides when every incident face turns away.

```mermaid
flowchart LR
    W["walk · vertex or point"] --> R["GlyphPoint<br/>center · radius · facing"]
    R -- "48 B · storage" --> T["glyph table"]
    style R fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=1-56 -->

## Step 2 · The lane

- One table per kind, one bind group each, two shader modules, five pipelines.

```mermaid
flowchart LR
    GR["GlyphRows<br/>spheres · dots"] -- "append" --> GL["GlyphLane"]
    GL -- "draw_spheres · template" --> P["ink pass"]
    GL -- "draw_dots · 3 verts" --> P
    style GL fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=57-100 -->

<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=101-153 -->

- Markers draw the template `spheres.len()` times; dots draw `DOT_VERTS * dots.len()` vertices with no template.

<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=154-238 -->

- `source_dot` is the pipeline for streamed source queries; it is declared with the others so the lane never grows a second pipeline set.

<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=239-294 -->

<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs copy lines=295-321 -->

## Step 3 · Vertex markers

- Same bindings as the ribbon shader; the row is `GlyphPoint`. The `LineUniform` mirror lists the whole 80-byte block, `origin` and `frame` included; a sphere sizes and culls against `vp_w`/`vp_h`, the attachment it is drawn into.

```mermaid
flowchart TB
    G["glyphs · @group(3)"] -- "faces_front" --> K["keep or hide"]
    T["template corner"] -- "vs_main · screen_radius" --> Q["quad around disc"]
    Q -- "fs_main" --> D["antialiased disc"]
    style Q fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04c session_viewer/src/shaders/sphere.wgsl type lines=1-16 -->

- `screen_radius` and `to_px` turn a world or pen radius into pixels; `faces_front` decodes the packed normals.

<!-- file: 04c session_viewer/src/shaders/sphere.wgsl type lines=17-68 -->

- The template corner is offset in clip space by the pixel radius plus the feather, so the quad always contains the antialiased disc.

<!-- file: 04c session_viewer/src/shaders/sphere.wgsl type lines=69-124 -->

- The facing cull is skipped when the eye is inside the object and when `line.opacity` is zero: in x-ray a vertex on the far side of a cube is exactly what you want to see.

<!-- file: 04c session_viewer/src/shaders/sphere.wgsl type lines=125-153 -->

## Step 4 · Free dots

- One equilateral triangle per dot; its incircle is the visible disc, so three vertices cover it without a template.

```mermaid
flowchart TB
    G["glyphs · @group(3)"] -- "vs_main · 3 verts" --> T["equilateral triangle"]
    T -- "fs_main · incircle" --> D["dot disc"]
    G -- "vs_source · fs_source_id" --> S["source id pass"]
    style T fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04c session_viewer/src/shaders/glyph.wgsl type lines=1-19 -->

- A dot wider than the canvas is dropped before it is placed. The test reads `frame`, the canvas the scene was projected for, not `vp_w`/`vp_h`, the attachment: a large dot survives when the pass renders only a window of the canvas, so it stays pickable.

<!-- file: 04c session_viewer/src/shaders/glyph.wgsl type lines=20-89 -->

- The ramp never exceeds the ink it feathers; `vs_source` and `fs_source_id` serve source-cloud queries.

<!-- file: 04c session_viewer/src/shaders/glyph.wgsl type lines=90-138 -->

<!-- check: 04c -->

## Step 5 · Wire the lane

- A template vertex slot and the `ink_rows` layout (one storage buffer at group 3).

```mermaid
flowchart TB
    U["Upload.glyph"] -- "set_scene" --> G["Gpu.glyphs"]
    L["template_layout · ink_rows"] --> G
    G -- "after strokes" --> P["markers on top"]
    style G fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04c session_viewer/src/engine/pipelines/mod.rs type -->

<!-- file: 04c session_viewer/src/engine/pipelines/layouts.rs type -->

<!-- file: 04c session_viewer/src/engine/gpu/upload.rs type -->

- Markers draw after strokes so their full footprint stays on top of the edges they sit on.

<!-- file: 04c session_viewer/src/engine/gpu/mod.rs type -->

<!-- file: 04c session_viewer/src/fixture.rs copy -->

<!-- file: 04c session_viewer/src/lib.rs type -->

<!-- file: 04c session_viewer/index.html copy -->

## Check

<!-- checkpoint: 04c -->

Expected:

- Triangle, polyline, and one orange dot below the triangle.
- Status reads **Checkpoint 04c · 3 objects**.
- Zoom out: the dot shrinks with its world radius, then holds at the pen width.

![Checkpoint 04c: vertex markers and free dots drawn from the glyph lane.](screenshots/04c.png)

## What changed

<!-- tree: 04c session_viewer/src/engine -->

- Data flow: `GlyphRows` → `GlyphTable` → group 3 → `sphere.wgsl` (instanced template) or `glyph.wgsl` (vertex-pulled triangles).

**Production equivalent:** `src/engine/gpu/glyphs.rs`, `src/shaders/sphere.wgsl`, `src/shaders/glyph.wgsl`.

## Try

- Append `?nomarkers=1`: the vertex markers disappear, the strokes stay; markers are a separate lane with its own draw.
- Zoom out until the markers thin out: `spacing` in the object row is what lets the shader fade them once they would overlap.
- Give one `GlyphPoint` a larger radius in `fixture.rs`: only that dot grows, because size travels per point.


## Recall

??? question "A marker is a disc, but the pipeline draws a quad template. Why not draw a disc?"
    The rasterizer only fills triangles. The four template corners are pushed out in clip space by the pixel radius plus the feather, so the quad is guaranteed to contain the antialiased disc, and the fragment stage decides what is inside it. Trading a tight shape for a simple one and letting the fragment stage do the geometry is the pattern behind strokes, markers and dots alike.

??? question "A free dot is one triangle, not a quad. What makes that enough?"
    Its incircle is the disc: an equilateral triangle's inscribed circle touches all three sides, so three vertices cover the whole disc with less area than a quad and no template buffer at all — the row comes from `@builtin(vertex_index) / 3`.

??? question "The dot's too-big-to-draw test reads `frame`, while a sphere sizes itself against `vp_w`/`vp_h`. Why the difference?"
    `frame` is the canvas the scene was projected for; `vp_w`/`vp_h` is the attachment actually being drawn. They are the same except in the pick pass, which renders a small window. A dot judged against the *window* would be dropped there and become unpickable, while a sphere genuinely needs the attachment it is being sized into. Two similar-looking numbers, two different questions.

??? question "When is the facing cull skipped, and what would you see if it never were?"
    When the eye is inside the object, and when `line.opacity` is zero — that is, in x-ray. Without the skip, `P` would show back edges but no back vertices, because every marker on the far side is culled by the faces pointing away from you. It is a one-line condition and its absence is a bug report you would struggle to phrase.

**Rebuild from memory:** the marker row is 48 bytes for a centre, a radius and packed face normals. Say where the radius lives and why — then check. The answer is the same alignment rule as lessons 03 and 04b, and by now you should be able to predict it before looking.

## Next

[04d · Point clouds](04d-clouds.md): the cloud tables, the LOD walk, and the splat prelude that resolves into the face pass.
