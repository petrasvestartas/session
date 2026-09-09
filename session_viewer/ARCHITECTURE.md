# Session Viewer architecture

Reference for the finished viewer: one browser application, Rust compiled to WebAssembly, winit for input, wgpu over browser WebGPU. The [course](docs/README.md) builds it from an empty crate; this page describes the result.

## Module graph

```mermaid
graph TD
    lib["lib.rs · App<br/>winit events, Msg handlers"] --> state["state.rs · State<br/>camera · selection · frame demand"]
    state --> camera["camera.rs"]
    state --> scene["app/scene.rs · Scene<br/>Rc&lt;Session&gt; documents, placements, row maps"]
    state --> gpu["engine/gpu/mod.rs · Gpu<br/>device, layouts, targets, lanes"]
    state --> text_state["state/text.rs · labels"]
    state --> cq["state/cloud_query.rs · source pages"]
    lib --> loader["app/loader.rs · fetch, validate, stage"]
    loader --> manifest["app/manifest.rs · validate.rs · decode.rs · stream.rs"]
    scene --> walk["app/walk/* · producers<br/>mesh · brep · curves · cloud · points · text"]
    walk --> upload["engine/gpu/upload.rs · Upload<br/>typed rows, no wgpu types"]
    upload --> gpu
    gpu --> lanes["lanes: arena · segments · glyphs · cloud+splat · text · surface_outline · triangle_tiles"]
    gpu --> pick["gpu/pick.rs · Picker"]
    gpu --> render["gpu/render.rs · frame list"]
    lanes --> shaders["shaders/*.wgsl"]
    input["app/input.rs · touch.rs"] --> state
```

The one upward flow is a pick answer: `Picker` returns a row and sub-ID, `Scene` maps them to a source identity, `State` selects.

Higher layers drive lower ones, never the reverse: a shader knows an object row, `Scene` knows which source object that row is, and input asks `State` to select rather than touching a buffer.

## Owners

| Owner | Responsibility |
|---|---|
| `lib.rs::App` | Window and event loop, `Msg` handlers, the one place a redraw is requested |
| `state.rs::State` | Camera and selection transitions, `needs_frame` / `dirty`, pick requests |
| `state/text.rs` | Selected-object names, source text presentation |
| `state/cloud_query.rs` | Streamed-cloud source queries across pages, original-ID resolution |
| `app/scene.rs::Scene` | Retained `Rc<Session>` documents, placements, row → source maps |
| `app/scene_text.rs` | Stable rows for authored text and document titles |
| `app/selection.rs` | `SelectionMode`, `ControlId`, `Controls` |
| `app/walk/` | Source geometry → typed `Upload` rows with source identity and bounds |
| `engine/gpu/mod.rs::Gpu` | One device and queue, layouts, frame uniforms, targets, every lane |
| `gpu/arena.rs` | Mesh vertex, index and object columns; `faces.rs` source-face rows; `triangle_tiles.rs` finite-visibility cache |
| `gpu/segments.rs` | Joined boundary pipes and standalone ribbons with source-edge IDs |
| `gpu/glyphs.rs`, `cloud.rs`, `splat.rs` | Markers and control dots; cloud LOD nodes; point splat prelude and resolve |
| `gpu/surface_outline.rs` | Ordinary and selected coverage masks rasterized in one pass, block maxima of each, one black compositor that skips every pixel no covered texel can reach; the masks are reused while camera, geometry and selection stand still |
| `gpu/text.rs`, `text_plate.rs`, `text_plane.rs`, `text_outline.rs` | Shaped glyph runs, plates, fixed-plane text, imported outlines |
| `gpu/pick.rs::Picker` | ID targets, bounded readback windows, generations and cancellation |
| `app/loader.rs`, `fetch.rs`, `live.rs`, `stream.rs` | Fetching, validation, staged replacement, bounded ranged reads |

![Source documents become display data; input and picking share the same scene identity.](docs/illustrations/ownership.svg)

## Three representations of an object

| Representation | Example | Lifetime and precision |
|---|---|---|
| Source | One BRep face, its trim uses, original edge IDs | Retained `Session` data, f64 |
| Display | Triangles, ordered boundary node chains, markers | Rebuilt on geometry change; source maps kept |
| GPU | Packed vertices, `Instance` rows, draw ranges | Object-relative f32 plus rebased translations |

Selecting a source face (Ctrl+Shift) selects the face, not a tessellation triangle. Every segment of one BRep edge carries the original edge ID. F10 shows original vertices or control points, not display subdivisions.

## A frame

`Gpu::encode_frame` in `gpu/render.rs`:

```mermaid
flowchart TD
    A["triangle_tile_pass<br/>project triangles, bin into screen tiles<br/>(only when camera or geometry changed)"] --> B["point_pass<br/>cloud splat prelude"]
    B --> C["begin_faces: backdrop, grid, opaque faces, cloud resolve<br/>writes Depth32Float + Rgba16Float gradient/primitive metadata"]
    C --> D["selection mask · solid mask<br/>R8Unorm coverage against physical depth"]
    D --> E["begin_ink: face highlight, print geometry, unselected strokes,<br/>selected solid strokes, black silhouette, selected standalone curves,<br/>markers, text"]
    E --> F{"pick pending?"}
    F -- yes --> G["id_pass: same lists, Rg32Uint IDs, scissored window, copy out"]
    F -- no --> H["submit · present"]
    G --> H
```

Order matters twice in the ink pass: selected solid strokes go below the silhouette so their yellow fringe cannot narrow its black border, and selected standalone curves go above it so a coincident mesh edge cannot erase them. Both obey physical occlusion.

![Physical surfaces, readable source ink, one black silhouette, then foreground annotations.](docs/illustrations/frame.svg)

## Memory

Per-pixel attachments dominate: at 4x MSAA the colour, depth and `Rgba16Float` metadata targets cost 64 bytes per physical pixel, at 1x 12. `Targets::samples_for` chooses 4x only for solid geometry, inside the adapter's pixel budget (9 Mpx discrete, 2.5 Mpx integrated, 4.2 Mpx unknown), and below two physical pixels per CSS pixel, where the pixel density already halves the stair-steps; `?msaa=4` and `?msaa=1` force it. `?dpr=1.5` caps the device pixel ratio the canvas is rendered at, for people who prefer memory over crispness; nothing caps it by default. When the browser reports a lost device (out of video memory), the page reloads itself once with `dpr=1&msaa=1`, the status line says so, and a second loss shows the error panel. `?inspect=1` publishes `gpu_texture_estimate_bytes` and `gpu_buffer_capacity_bytes`, which match the GPU process's real allocation to within the browser's own swapchain.

## Depth and visible ink

- Depth is reversed: near is larger, the clear value is zero, opaque faces compare `Greater`.
- A stroke covers samples beside its axis. The ink shader transfers the winning primitive's depth to the axis through the stored gradient before comparing, so a line on a surface is not hidden by the surface beside it.
- A neighbouring triangle's plane can cross the axis outside the triangle. The finite fallback in `triangle_tiles.rs` then tests the actual triangles that intersect the axis's screen tile: projected records (six `vec4<f32>` each), per-tile counts, a prefix scan, and filled `(primitive, max depth)` lists. Overflowing or incomplete lists keep the conservative rejection. The list pool is sized for the scene, two references per tile plus eight per triangle, and the scan writes the words it needed into the first record; the CPU reads that back a frame later and grows the pool before the next projection, so a small scene never pays the 64 MB ceiling of a 262 144-tile grid.

![A triangle's plane extends beyond its footprint; the line is visible outside the actual triangle.](docs/illustrations/finite-triangle.svg)

## Rust ↔ WGSL interfaces

Bind group scheme for every draw (`engine/pipelines/layouts.rs`):

| Group | Binding | Rust layout | WGSL |
|---|---|---|---|
| 0 | 0 | `Layouts::mvp`, uniform | `@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>` |
| 1 | 0 | `Layouts::line`, uniform | `@group(1) @binding(0) var<uniform> line: LineUniform` |
| 2 | 0 | `Layouts::instance`, storage | `@group(2) @binding(0) var<storage, read> instances: array<Instance>` |
| 2 | 1 | anchored translations, storage | `@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>` |
| 2 | 2–5 | `Layouts::ink_instance`: physical depth and gradient, single and multisampled | depth and float textures sampled by the ink fragment stage |
| 2 | 6–7 | projected triangles and tile lists | finite-visibility storage read by `ink_visibility.wgsl` |
| 3 | n | the lane's own rows (`ink_rows`, `segment_rows`, `points`) | e.g. `@group(3) @binding(0) var<storage, read> segments: array<StrokeSegment>` |

`Instance` (`gpu/instance.rs`) is one 96-byte row: `model: [f32;16]`, `color: [f32;4]`, `flags: u32`, `thickness`, `spacing`, `_pad`. The translation column is zero; the rebased translation is the 16-byte row at group 2 binding 1, so a re-anchor rewrites 16 bytes per object. Flag bits: selected, hidden, inside, print, open, sheet, smooth. Layout tests in `instance.rs` compare the Rust size and offsets with the WGSL declaration.

Meshes are vertex-pulled: `triangle.wgsl` reads `face_vertices`, `face_objects`, `face_indices` from storage at group 3 instead of a vertex buffer. Textures: color surface format, `Depth32Float` physical depth, `Rgba16Float` metadata (gradient in xy, packed primitive ID in zw), `R8Unorm` coverage masks, `Rg32Uint` pick IDs with their own `Depth32Float` and metadata.

## Flows

**Camera.** Pointer delta → `Input` → `Camera::orbit / pan / zoom_at` (f64, target + distance + quaternion) → `view_proj_anchored(aspect, anchor)` with reversed depth and a near plane at a fraction of the focus distance → `FrameUniforms` (`mvp`, eye, ortho half-height) → group 0.

**Geometry.** `Msg::File` → `Scene` retains the document → `app/walk/*` produce `Upload` rows (arena, segments, glyphs, cloud, object rows, bounds) with source maps → `Gpu::set_scene` appends rows to lane buffers and rebinds → `render.rs` draws ranges.

**Picking.** Pointer up without a drag → `State::request_selection` records mode, generation, camera → `id_pass` renders IDs into an attachment the size of the window around the cursor plus a three-texel halo, not the canvas: the pick pass sees the scene through the sub-frustum of that window (`PickView::clip_transform`), with the projection factors scaled so pens and markers keep their pixel size, and the visibility test addresses the canvas-wide tiles through `LineUniform::origin` → `Picker` maps a bounded copy asynchronously → row and sub-ID → `Scene::object_at / edge_at / face_at` → `SelectionMode` → `Instance::FLAG_SELECTED` uploaded → redraw. A camera, scene or mode change retires answers from an older generation. The ID, depth and metadata targets therefore cost a few kilobytes instead of 20 bytes per canvas pixel.

**Controls.** F10 → `Controls` collects original vertices or control points of the selected parent → marker rows uploaded with `ControlId` → a control pick returns the original identity, not the marker slot. Streamed clouds query every eligible source page (`state/cloud_query.rs`) independently of display LOD and apply the final visible original ID.

**Text.** String, font, size → `engine/text.rs` shapes glyph runs with the bundled Noto fonts → `TextPlacement` (Screen, Anchor, Nameplate, WorldPlane, WorldBillboard) → `TextLane::prepare` chooses the physical raster size once per DPR → coverage atlas → plate pass and glyph pass; fixed-plane text uses `text_plane.wgsl` with perspective-correct rounded plates that also serve the ID pass.

**Loading.** Route → manifest (TOML, YAML or JSON) → `validate.rs` checks counts, indices, transforms → protobuf decode → `PendingDocument` staged with a request generation → `Msg::File` in manifest order. Stale generations are dropped; a malformed document keeps the last valid scene.

## Lifecycles

| Change | Required work | Reused work |
|---|---|---|
| Camera, DPR or resize | Uniforms, placement, finite-visibility projection, targets; cancel stale picks | Documents, tessellation, shaped text, the visibility pool's grown size |
| Selection or color | Object flags, selected labels, masks, redraw | Mesh buffers, projected-triangle cache |
| Hide or show | Visibility flags, projection invalidation, text visibility | Source geometry and identity |
| Document replacement | Source maps, display data, bounds, caches; cancel stale work | Device and pipelines |
| Text content or font | Reshape changed runs, raster, bounds | Unchanged glyph resources |
| Idle | Nothing submitted | Everything |

`reset` keeps capacity for an edit rebuild; `release` returns scene-sized storage on replacement. `SourceCache` (`app/inspection/source_memory.rs`) holds `Weak<Session>` identities, so measuring retained source payload never extends a document's lifetime.

## Input

| Input | Result |
|---|---|
| Left drag / right drag / middle drag / wheel | Orbit / pan / pan / zoom toward the cursor |
| Left click | Select or toggle one source object |
| Ctrl + left click | Select an original mesh, BRep or NURBS edge |
| Ctrl + Shift + left click | Select an original face; a nearby eligible edge wins |
| F10 / Escape | Show the parent's original controls / leave the mode, then clear |
| 1–7, C, F, Space | Standard views, reset, fit, projection toggle |
| Q, W, E, O, D, B | Points, lines, mesh edges, silhouettes, lighting, back faces |
| H / S / T | Hide selection / show all / toggle selected names |

## Adding a feature

For a new geometry family: a `walk/` producer that emits existing `Upload` rows with bounds and source identity; a new lane only when storage or drawing differs; one line in `render.rs`; then exercise select, hide, replace and release. For a shader change: read its Rust mirror, bindings, color and ID entry points, sample count and release path together, and check the layout test in `instance.rs`. Never mutate a vertex buffer behind `Scene`: it is the source of truth for picking, controls and any future undo.

The CAD geometry contract (shared boundaries, trims, pcurves, provenance) is in the [CAD design record](docs/cad-design.md).
