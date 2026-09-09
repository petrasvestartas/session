# 18 · Finite-triangle visibility and production convergence

## You are building

```mermaid
flowchart TB
    P["physical pass<br/>depth · gradient · primitive id"] --> Q{"plane test<br/>accepts?"}
    Q -- yes --> V["visible"]
    Q -- no --> W["finite test:<br/>winning + neighbour triangles"]
    W -- "nearer hit" --> H["hidden"]
    W -- "no hit" --> T["finite test:<br/>every triangle in the axis tile"]
    T -- "nearer hit" --> H
    T -- "no hit" --> V
```

Built once per camera and geometry revision, read by the tile test:

```mermaid
flowchart LR
    c1["project_triangles.wgsl<br/>96-byte records"] --> c2["triangle_tiles.wgsl<br/>count per tile"] --> c3["scan_triangle_tiles.wgsl<br/>prefix sums"] --> c4["fill<br/>(primitive, max depth)"]
```

## Starting point

- Checkpoint 17. The ink shader hides a stroke sample when the surface's depth plane, carried to the stroke axis through the stored gradient, lies in front of the axis.
- That plane is infinite. A narrow strip beside a seam has a plane that crosses the seam ray outside the strip, so the seam disappeared at teapot concavities and where solids touch.
- This lesson ends at the current production runtime.

<!-- supplied: 18 -->

![The depth plane continues beyond the finite triangle; only a finite nearer hit can hide the axis.](illustrations/finite-triangle.svg)

## Part A · Remember which triangle won

### Step 1 · Metadata carries the primitive

- The physical metadata target grows from two to four half floats: gradient in `xy`, a lossless triangle address in `zw`.
- Each 14-bit half of the address skips exponent zero, so it survives `Rgba16Float` without NaNs or denormals.

```mermaid
flowchart LR
    D["physical depth"] --> M["Rgba16Float metadata<br/>xy gradient · zw primitive"]
    P["pull_triangle index"] -- "physical_triangle" --> M
    style M fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/shaders/physical.wgsl type -->

- `pull_triangle` numbers every triangle; `vs_triangle` is the plain physical draw, `vs_face` adds the source face on top.

<!-- file: 18 session_viewer/src/shaders/triangle.wgsl type -->

- Every other physical writer widens its metadata to `vec4` with a zero address, keeping the conservative plane rule.

<!-- file: 18 session_viewer/src/shaders/background.wgsl type -->

<!-- file: 18 session_viewer/src/shaders/grid.wgsl type -->

<!-- file: 18 session_viewer/src/shaders/splat.wgsl type -->

<!-- file: 18 session_viewer/src/shaders/splat_resolve.wgsl type -->

<!-- file: 18 session_viewer/src/shaders/text_outline.wgsl type -->

## Part B · Project each triangle once

### Step 2 · The projected record

`ProjectedTriangle` is six `vec4<f32>`; the Rust mirror test asserts the same offsets and `PROJECTED_BYTES`:

| Offset | Field | Meaning |
|---|---|---|
| 0 | `edge0` | inward edge equation `xyz`; `w` = reference x |
| 16 | `edge1` | edge equation; `w` = reference y |
| 32 | `edge2` | edge equation; `w` = reference depth |
| 48 | `edge3` | fourth edge of a near-clipped quad; `w` = corner count |
| 64 | `gradient` | screen depth gradient `xy`, nearest corner depth `z` |
| 80 | `bounds` | screen min `xy`, max `zw` |

- `projected_triangle_at` returns `(depth, 1)` when the point is inside every edge, else `(0, 0)`.
- `visibility_tile_span` doubles the tile size until the grid has at most `262144` tiles; the CPU `TileLayout` uses the same rule.

```mermaid
flowchart LR
    R["ProjectedTriangle<br/>6 × vec4 · 96 B"] -- "projected_triangle_at" --> H["(depth, inside)"]
    T["visibility_tile_span"] --> G["≤ 262144 tiles"]
    style R fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/shaders/projected_triangle.wgsl type -->

### Step 3 · The projection shader

- One compute invocation per triangle reads the arena's vertex, object and index columns through the same instance and translation rows the draw uses.
- Near-plane clipping happens before the divide, so a triangle crossing the eye becomes a quad or vanishes, never a garbage projection.

| Group · binding | Rust | WGSL |
|---|---|---|
| 0 · 0 | `Layouts::mvp` | `mvp` |
| 1 · 0 | `Layouts::line` | `line` (viewport size) |
| 2 · 0, 1 | `Layouts::instance` | `instances`, `translations` |
| 3 · 0–2 | arena vertices, object ids, indices | `physical_vertices`, `physical_objects`, `physical_indices` |
| 3 · 3 | `TriangleTiles::projected` | `projected` (read_write) |
| 3 · 4 | `live_count` uniform | `live_count` |

```mermaid
flowchart LR
    A["arena columns · instances"] -- "cs_main per triangle" --> C["near-plane clip"]
    C --> Q["quad or nothing"]
    Q --> P["projected[] record"]
    style P fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/shaders/project_triangles.wgsl type lines=1-60 -->

<!-- file: 18 session_viewer/src/shaders/project_triangles.wgsl type lines=61-127 -->

## Part C · A compact screen index

### Step 4 · Count and fill

- One quad per projected triangle covers its tile bounds; `covered_tile` discards tiles the polygon cannot touch.
- `fs_count` counts references per tile. `fs_fill` runs after the scan and writes `(primitive, nearest possible depth)` pairs into the tile's range; a cursor past the count sets the overflow flag instead of writing.

```mermaid
flowchart LR
    Q["quad per projected triangle"] -- "covered_tile" --> C["fs_count · tile counts"]
    C -- "after scan" --> F["fs_fill<br/>(primitive, max depth)"]
    F -- "cursor past count" --> O["overflow flag"]
    style F fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/shaders/triangle_tiles.wgsl type -->

### Step 5 · Prefix sums instead of a per-tile cap

- Tile records are `count / offset / cursor / overflow`; block records are `sum / prefix`.
- Sums saturate at the buffer capacity, so an oversubscribed pool can never wrap into a plausible offset.

```mermaid
flowchart LR
    C["tile counts"] -- "scan_tiles" --> B["block sums"]
    B -- "scan_blocks" --> P["block prefixes"]
    P -- "finish_offsets" --> O["tile offsets · saturating"]
    style O fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/shaders/scan_triangle_tiles.wgsl type -->

### Step 6 · The owner

- `TileLayout` mirrors `visibility_tile_span`; the reference pool budgets `REFERENCES_PER_TILE` per tile overall, and a dense tile borrows spare space.

```mermaid
flowchart LR
    K["ProjectionKey<br/>camera · geometry revision"] -- "changed" --> E["encode<br/>project · count · scan · fill"]
    L["TileLayout · REFERENCES_PER_TILE"] --> P["prepare storage"]
    P --> E
    style E fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=1-52 -->

- `ProjectionKey` is the cache key: camera matrix plus the object table's geometry revision. Selection is not in it.

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=53-82 -->

- `prepare` resizes storage for the triangle count and framebuffer; beyond the device's storage binding limit it releases the tables and reports so the ink shader keeps the plane rule.

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=83-165 -->

- `encode` runs project → clear headers → count → three scan dispatches → fill, then records the key.

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=166-239 -->

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=240-301 -->

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=302-327 -->

- Layouts and pipelines: the project pass sees groups 0–2 from compute, the raster pass reads `projected` in the vertex stage and writes records in the fragment stage.

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=328-417 -->

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=418-497 -->

Copy the rest of the file:

<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs copy lines=498-595 -->

<!-- check: 18 -->

## Part D · The ink query

### Step 7 · Refine the rejection, keep the cheap test

- `ink_visible_plane` is the old test. When it accepts, nothing else runs.
- When it rejects: test the winning primitive at the axis, then the four sample-matched neighbours. A finite nearer hit confirms occlusion.
- Otherwise walk the axis pixel's tile list: skip references whose nearest possible depth cannot beat the axis, skip triangles whose bounds miss the point, then run the finite test. An overflowing or incomplete list keeps the rejection.

The blank lines separate the helpers; type them so the file matches production:

```mermaid
flowchart LR
    A["ink_visible_plane"] -- "accepts" --> V["visible"]
    A -- "rejects" --> W["ink_primitive + neighbours<br/>finite test"]
    W -- "no hit" --> T["tile list of the pixel"]
    T --> V
    T -- "nearer finite hit" --> H["hidden"]
    style W fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/shaders/ink_visibility.wgsl type hunks=1-11 -->

<!-- file: 18 session_viewer/src/shaders/ink_visibility.wgsl type hunks=12,13 -->

## Part E · Rust owners and wiring

### Step 8 · Faces draws the physical pass

- The physical and object-ID triangle pipelines move into `Faces`, so the primitive numbers written by the color pass are the same numbers the projection shader uses.

```mermaid
flowchart LR
    F["Faces<br/>draw_physical · draw_object_ids"] -- "same primitive numbers" --> C["color pass"]
    F --> J["projection shader"]
    style F fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/engine/gpu/faces.rs type -->

<!-- file: 18 session_viewer/src/engine/gpu/arena.rs type -->

### Step 9 · Bindings 6 and 7

- The ink instance group gains the projected table and the tile buffer; the mvp, line and instance layouts become visible to compute.

```mermaid
flowchart LR
    I["ink_instance layout"] -- "binding 6" --> P["projected table"]
    I -- "binding 7" --> T["tile buffer"]
    G["geometry_revision"] --> K["cache key"]
    style I fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/engine/pipelines/layouts.rs type -->

<!-- file: 18 session_viewer/src/engine/pipelines/mod.rs type -->

- `geometry_revision` counts placement, rebase and hidden-state changes; selection flags do not bump it.

<!-- file: 18 session_viewer/src/engine/gpu/objects.rs type -->

- Metadata textures and the pick copy widen to four channels.

<!-- file: 18 session_viewer/src/engine/gpu/targets.rs type -->

<!-- file: 18 session_viewer/src/engine/gpu/pick.rs type -->

<!-- file: 18 session_viewer/src/engine/gpu/instance.rs type -->

### Step 10 · The tile pass runs before ink

- `triangle_tile_pass` prepares storage, rebinds the ink group when a buffer was replaced, then encodes; both the color frame and an ID-only frame call it.

```mermaid
flowchart LR
    T["triangle_tile_pass"] -- "prepare · rebind · encode" --> I["ink passes"]
    C["color frame"] --> T
    D["ID-only frame"] --> T
    style T fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/engine/gpu/render.rs type -->

<!-- file: 18 session_viewer/src/engine/gpu/mod.rs type -->

<!-- check: 18 -->

## Part F · State split and presentation defaults

### Step 11 · Streamed queries move out of `state.rs`

- The methods are unchanged; `State` still owns the query. The file only groups the page/answer/resolve workflow.

```mermaid
flowchart LR
    S["state.rs"] -- "unchanged methods" --> Q["state/cloud_query.rs<br/>page · answer · resolve"]
    Q -- "owned by" --> S
    style Q fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/state/cloud_query.rs type lines=1-38 -->

<!-- file: 18 session_viewer/src/state/cloud_query.rs type lines=39-101 -->

<!-- file: 18 session_viewer/src/state/cloud_query.rs type lines=102-159 -->

<!-- file: 18 session_viewer/src/state/cloud_query.rs type lines=160-212 -->

<!-- file: 18 session_viewer/src/state.rs type -->

### Step 12 · Selected text is black on yellow

- `ink_color` derives black ink from the selection flag without touching the authored color; both text renderers read it.
- Plates and planes fill the whole rounded backing yellow instead of drawing a border; every backing reserves a full cap at each end.

```mermaid
flowchart LR
    L["TextLabel selected"] -- "ink_color()" --> B["black ink"]
    L --> Y["yellow rounded backing"]
    B --> G["glyph pass"]
    Y --> P["text_plate · text_plane"]
    style B fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/engine/text.rs type -->

<!-- file: 18 session_viewer/src/engine/gpu/text.rs type -->

<!-- file: 18 session_viewer/src/engine/gpu/text_plate.rs type -->

<!-- file: 18 session_viewer/src/shaders/text_plate.wgsl type -->

<!-- file: 18 session_viewer/src/engine/gpu/text_plane.rs type -->

<!-- file: 18 session_viewer/src/shaders/text_plane.wgsl type -->

<!-- file: 18 session_viewer/src/state/text.rs type -->

<!-- file: 18 session_viewer/src/app/inspection.rs type -->

### Step 13 · The default pen

- Silhouettes start **off**: the two coverage masks and the compositor are a full-screen pass per frame, which is slow on integrated GPUs. `O` turns them on; `?outlines=1` starts with them on.

```mermaid
flowchart LR
    V["View::from_env"] -- "show_outlines false" --> O["silhouettes off"]
    K["O key · ?outlines=1"] --> N["silhouettes on"]
    style V fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/src/engine/gpu/view.rs type -->

The silhouette unit block of the outline owner opts in explicitly, since the default no longer does:

<!-- file: 18 session_viewer/src/engine/gpu/surface_outline.rs copy -->

## Part G · The documentation corner

### Step 14 · One click from the viewer to the course

- A black folded corner at the top right of the page links to `docs/`; it opens the course in a new tab and never covers the canvas' input.
- Trunk copies the built site into `dist/docs`, so `trunk serve` serves the viewer and its documentation together.
- The pre-build hook rebuilds the site only when a documentation source is newer than the built page; without the course sources it writes a one-line placeholder instead of failing the build.

```mermaid
flowchart LR
    C["#viewer-docs corner"] -- "docs/" --> D["dist/docs · built site"]
    H["docs/build_site.sh hook"] -- "when stale" --> D
    T["Trunk copy-dir"] --> D
    style C fill:#1a1eb2,color:#fff
```

<!-- file: 18 session_viewer/index.html copy -->

<!-- file: 18 session_viewer/Trunk.toml copy -->

<!-- file: 18 session_viewer/docs/build_site.sh copy -->

## Check

<!-- checkpoint: 18 -->

Expected:

- With the supplied teapot fixture (`assets/pb/view_mixed_teapot.pb`) or the local scene loaded: the concave foot boundary stays continuous while orbiting; edges where two solids touch stay visible.
- A genuinely covered edge stays hidden; a visible seam does not break up as a neighbouring face moves over its stroke fringe.
- Click a manifest text: black letters on a yellow rounded backing; click away: original colors return.
- Source edges and lines draw with a one-pixel pen; `?thickness=1.5` restores the older weight.

![Checkpoint 18 with the supplied teapot: the rim and foot boundaries stay continuous from two camera positions, and the seams where the lid meets the body remain visible while a neighbouring face passes over their stroke fringe.](screenshots/18-teapot.png)

![Left: a selected manifest text is black on a yellow rounded backing. Middle: the default one-pixel pen on the polyline, magnified five times. Right: `?thickness=1.5`, the older, heavier weight, at the same magnification.](screenshots/18-text-pen.png)

Optional lint gates:

```sh
cargo fmt --package session_viewer -- --check
cargo clippy --locked --target wasm32-unknown-unknown --lib -- -D warnings
```

## The result, served

The same source you just finished is what the repository publishes:

- **Served viewer**: <https://petrasvestartas.github.io/session/> — the GitHub Pages build of `session_viewer`; its documentation corner opens this course at <https://petrasvestartas.github.io/session/docs/>.
- **Source code**: <https://github.com/petrasvestartas/session/tree/main/session_viewer> — the folder this course reconstructs, byte for byte at checkpoint 18.
- **Locally**: `trunk serve` in `session_viewer` serves the viewer at <http://localhost:8770/> and the course at <http://localhost:8770/docs/>; the black corner at the top right links the two.

## Verify you reached production

Record the checkpoint and compare every runtime file against the frozen production inventory:

```sh
python3 "$COURSE_REPO/docs/reconstruction/replay.py" --output "$COURSE_WORK" --through 18 --adopt
python3 "$COURSE_REPO/docs/reconstruction/converge.py" --workspace "$COURSE_WORK"
```

Expected:

- `converge.py` reports every runtime file identical to production; the only listed differences are the documented packaging ones (the local input manifest and imported-document font artifacts).

## What changed

<!-- tree: 18 session_viewer/src/engine -->

- Data flow: arena columns → `project_triangles.wgsl` → `projected` → `fs_count` → scan → `fs_fill` → `triangle_tiles` → `ink_visible`.
- Metadata: `Rgba16Float` with gradient in `xy` and the packed primitive in `zw`; the ID pass owns matching single-sample targets.
- Cache: rebuilt on camera, hide/show, placement, rebase or geometry replacement; reused across selection and color changes.
- `State` keeps its ownership; `state/cloud_query.rs` and `state/text.rs` are its companions.

**Production equivalent:** this checkpoint is the current production runtime.

## Try

- Orbit the teapot slowly around its foot and watch the bottom boundary: at checkpoint 17 the same view broke the line into dashes where the body's planes crossed the stroke axis.
- Set `?thickness=3` and repeat: the finite test is on the stroke axis, so a wider fringe changes the look, not the visibility decision.
- Overflow a tile on purpose by loading a dense mesh and lowering the tile size in `triangle_tiles.rs`: overflowing lists keep the conservative rejection, and hidden edges never leak through.
- Click the folded corner of the canvas: the course opens in a new tab from the same `dist/` the viewer is served from.

## Next

[Architecture reference](../ARCHITECTURE.md): the finished module graph, frame lifecycle and Rust ↔ WGSL interfaces.
