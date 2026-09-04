# Task: a wireframe that is never cut by its own faces and never shows through other geometry

Repository `session`, crate `session_viewer` (Rust, wgpu 29 + winit 0.30, wasm32 in the browser
with a native harness). Work on `main`, currently at `91d898b7`.

## How to see it

`trunk serve`, open `http://localhost:8770/?scene=view_mixed`, hover the floor model (the timber
deck on the right of the scene) and **zoom out with the wheel**. Lines that belong to elements
BEHIND other elements appear over them, and the further out you go the more of them there are.
Press `5` for the orthographic top view and the same thing happens. At the fit view (`F`) the
model is nearly clean, which is why this is easy to miss until you pull back.

The floor model alone, headless, at 2.6 times the fit distance:

```bash
VIEWER_W=1800 VIEWER_H=1400 VIEWER_ORBIT=0,209 VIEWER_ZOOM=-10 \
cargo run --release --example selftest --target x86_64-unknown-linux-gnu -- out.ppm <floor.pb>
```

## The defect

In `?scene=view_mixed` the floor model (201 timber meshes and their 290 outline polylines) shows
lines that belong to elements BEHIND other elements. It is worst looking straight down and when
zoomed out; at the fit view it is nearly clean. The same defect exists in the orthographic top
view (key `5`).

At that zoomed-out camera the offsets are far larger than the material: the worst samples are
covered by a 200 mm beam whose face is only 4.0 mm in front of the outline, while the face push
is 50.1 mm and the outline's lift 6.8 mm - a margin of -52.9 mm. Of the 290 outlines, 129 are
fully covered there and 141 partly.

Measured, not guessed. `examples/census_plates.rs` ray-casts an outline sample every 50 mm
against every plate in front of it, from the exact camera of a render:

| camera | covered samples | samples that surface |
|---|---|---|
| fit (iso) | 19,440 | 4 |
| straight down, perspective | 16,846 | 452 |
| 2.6x the fit distance | 16,804 | 508 |
| 4.3x the fit distance | 16,729 | 518 |
| straight down, orthographic | 18,497 | 453 |

## Why it happens

Two mechanisms move ink and faces apart in depth, both in world units:

- `src/shaders/triangle.wgsl`: a face recedes along its view ray by
  `min(PUSH_FRAC * eye_depth, PUSH_MAX_THICK * thickness)` = `min(0.4 %, 25 %)`.
- `src/shaders/ribbon.wgsl`: ink lifts toward the eye by `LIFT_RADII_WIRE` = 3 pen half-widths
  (mesh wires) or `LIFT_RADII_FREE` = 1 (free polylines), capped at `LIFT_MAX_THICK` = 25 % of
  the object's thickness. `sphere.wgsl` and `glyph.wgsl` do the same for vertex markers and dots.
- `thickness` is measured by the walk across the mesh's own dominant face normals
  (`src/app/walk/bounds.rs::mesh_thickness`), so a plate baked rotated into world coordinates
  measures its plate thickness, not its axis-aligned box. `src/engine/gpu/objects.rs::thickness`
  scales it by the placement and floors it at 0.1 % of the diagonal.

Both mechanisms are needed close up: at arm's length one pixel is several millimetres, so a wire
drawn exactly on its face z-fights and is eaten by it. Both are fatal far away: the elements in
this model interpenetrate at their joints, so another element's outline can sit 1 to 3 mm behind
a face, while at 20 m one pixel is 15 mm and the push saturates at a quarter of the plate
thickness (15 mm on a 60 mm plate). Every surfacing sample above is a place where the material
in front is thinner than push + lift.

## What was already tried, and why each failed

1. **Push capped by the object's AABB diagonal.** Useless for long thin plates: 1 % of a 3.4 m
   diagonal is 34 mm against a 27 mm plate.
2. **Push capped by a quarter of the orientation-free thickness** (the state you are looking at).
   Removes the fit-view bleed; leaves the numbers in the table.
3. **A slope-scaled hardware depth bias on the face pipeline** (`DepthBiasState`, no vertex-shader
   push). A face seen at a grazing angle then recedes by many millimetres, so the plate probe in
   `docs/_gate.sh` went red at 269 pixels.
4. **Faces fixed; ink drawn INSIDE its face plane** (commit `deb11910`, reverted in `4dda30a5`).
   The ribbon was folded along its centre line and each side corner took the depth of its face
   plane at that pixel; free outlines lying on a plate face inherited that face's normal and the
   plate's thickness through a new `src/app/walk/hosts.rs`. This removed the penetration
   completely (0 surfacing samples from six cameras) but broke the look: at a crease the two
   halves of a ribbon separate and the round caps overshoot, so a zoomed-in box shows torn
   corners and doubled edges. The user rejected it. Read that commit before repeating it.

## How other viewers solve this, and what we got wrong

This is the classic *coincident topology* problem: an edge that lies exactly on the surface it
decorates. Every CAD viewer hits it and there are three standard answers.

1. **A depth-buffer-space offset on the FACES, not a world-space one.** The face is biased by
   `constant x (one depth-buffer unit at that pixel) + slope_scale x (the face's depth slope per
   pixel)`, and the result is CLAMPED. Both terms are measured in depth, so the bias is
   automatically a few micrometres near the camera and only as large as the depth precision
   demands far away - it never has a size in millimetres that can cross a joint. The lines then
   need no offset of their own at all. Public implementations to read:
   - VTK: `vtkMapper::SetResolveCoincidentTopologyToPolygonOffset`, and the relative
     polygon/line/point offset parameters. VTK's documentation of this problem is the clearest
     short statement of it in any open codebase.
   - Open CASCADE: `Graphic3d_AspectFillArea3d::SetPolygonOffsets` (`Aspect_POM_Fill`), used by
     the V3d viewer for "shaded with edges" - a real CAD kernel doing exactly this task.
   - Coin3D / FreeCAD: the `SoPolygonOffset` node, applied to the shape, not the wireframe.
   - OpenSceneGraph: `osg::PolygonOffset`. three.js: `Material.polygonOffset` with
     `polygonOffsetFactor` / `polygonOffsetUnits`, as used by its wireframe-over-shaded examples.
   - The semantics themselves: OpenGL 4.6 spec section 14.6.5 "Depth Offset", and the Vulkan
     `depthBiasConstantFactor` / `depthBiasSlopeFactor` / `depthBiasClamp` documentation.
2. **Edges derived from the depth and normal buffers in screen space**, with no line geometry at
   all. An edge drawn where a depth or normal discontinuity is cannot be behind its own surface.
   Blender's overlay engine does this, as do most engine outline passes. It costs the ability to
   style or pick an individual edge.
3. **Analytic hidden-line removal**, which is what a kernel does for a drawing: compute
   visibility in exact geometry and emit visible and hidden segments. Open CASCADE's `HLRBRep`
   is the readable open implementation. It is view-dependent CPU work, right for print output
   and too slow to re-run per frame while orbiting.

What this repository does instead: it offsets in the VERTEX SHADER, in world millimetres, by a
fraction of eye depth and of the object's thickness. That has no relation to depth precision, so
there is no scale at which it is both large enough to win a tie and small enough to stay inside a
3 mm joint. The hardware path was tried once (attempt 3 above) and misused: `slope_scale` was set
to 2.0 with `clamp` left at 0.0, so a face seen edge-on received an unbounded bias. `wgpu 29`
exposes all three fields as `DepthBiasState { constant: i32, slope_scale: f32, clamp: f32 }`
(`wgpu-types-29.0.3/src/render.rs:654`), and this viewer uses a reverse-Z `Depth32Float` buffer
with a `GreaterEqual` test, so the bias must be NEGATIVE to push a face away and the clamp must
be set. That experiment - faces on a small clamped hardware bias, ink with no lift at all,
`DepthMode::ReadOnlyEqual` kept on the ribbon - has never been run here, and it is the one every
product in the list above is running.

## What a fix must satisfy

- **Zero surfacing samples** from the six cameras below, at 1x, 4x and 16x the fit distance.
- **The close-up must not change.** `assets/pb/view_local_boxes.pb` at `VIEWER_ZOOM=5` renders
  262,685 non-background pixels today: full-width red mesh edges reaching every corner,
  continuous, with black vertex markers on top and no doubling or tearing.
- `docs/_gate.sh` prints `gate OK` (it renders three probe plates, one rotated 30 degrees, whose
  inset bottom outlines must never be visible from above).
- `cargo xtest` passes; `cargo check --target wasm32-unknown-unknown` and
  `cargo clippy --release --all-targets --target x86_64-unknown-linux-gnu` are warning-free.
- Frame time must not regress by more than about 20 % on `view_mixed`, `view_lines` and
  `view_meshes` (`examples/bench_frame.rs`, numbers in `docs/_PERF.md`).

Approaches not yet tried, in the order I would try them:

1. The in-face model of attempt 4 WITHOUT its two artifacts: one plane per segment (the nearer
   of its two faces) instead of a folded ribbon, and caps that do not extend past the line ends.
2. A separate hidden-line pass against the object id buffer that already exists for picking
   (`src/engine/gpu/pick.rs`, `Target::ID`): a wire is hidden by any OTHER object but never by
   its own faces. Costs one more pass; needs no depth offsets at all.
3. Per-object depth ranges, or rendering wires with the depth of their own object's faces only.

## How to verify

```bash
cd session_viewer
export CARGO_TARGET_DIR=$HOME/.cache/tmain REGEN_PROTO=0

# the close-up reference and any scene, headless (prints the pixel count and the camera)
cargo run --release --example selftest --target x86_64-unknown-linux-gnu -- out.ppm assets/pb/view_local_boxes.pb
# knobs: VIEWER_W/H, VIEWER_ZOOM=<int>, VIEWER_ORBIT="dx,dy", VIEWER_VIEW=top|front|iso (forces
# orthographic), VIEWER_ORTHO=1, VIEWER_NO_EDGES=1, BENCH_NO_MARKERS=1, VIEWER_MSAA=1|4

docs/_gate.sh                     # the probe plates; prints "gate OK" or the first failure
cargo xtest                       # the shader/Rust mirror tests
```

The oracle. `examples/census_plates.rs` prints, per outline and per camera distance, the covering
plate, the push, the lift, the separation and the margin, and a FAIL count. The harness logs its
camera as `camera: eye (x, y, z) mm`; divide by 1000 and pass it in:

```bash
F=<floor model .pb>                     # bucket key pb/view_mixed_floor_model.pb
CEN=$CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release/examples/census_plates

VIEWER_W=1800 VIEWER_H=1400 CENSUS_EYE=-4.0261,-6.9735,4.649   $CEN $F   # fit
VIEWER_W=1800 VIEWER_H=1400 CENSUS_EYE=-0.0102,-0.0177,9.2979  $CEN $F   # straight down
VIEWER_W=1800 VIEWER_H=1400 CENSUS_EYE=-1.3614,-2.358,8.8903   $CEN $F   # tilted
VIEWER_W=1800 VIEWER_H=1400 CENSUS_EYE=-6.3117,-0.2981,6.821   $CEN $F   # side
VIEWER_W=1800 VIEWER_H=1400 CENSUS_EYE=-0.0265,-0.0459,24.1165 $CEN $F   # 2.6x
VIEWER_W=1800 VIEWER_H=1400 CENSUS_EYE=-0.0427,-0.0739,38.8398 $CEN $F   # 4.3x
# orthographic: parallel rays, CENSUS_ORTHO_H is the view half-height in mm
VIEWER_W=1800 VIEWER_H=1400 CENSUS_EYE=0,0,100 CENSUS_FWD=0,0,-1 CENSUS_ORTHO_H=5368 $CEN $F
```

`CENSUS_RECOLOR=<out.pb>` writes a copy of the file whose outline segments are magenta where a
plate covers them, so a render of that copy shows every geometrically hidden line that is
nevertheless drawn.

The census models the shipped rule in its own constants at the top of the file (`PUSH_FRAC`,
`PUSH_MAX_THICK`, `LIFT_RADII_FREE`, `LIFT_MAX_THICK`, `PEN_PX`). **Change them whenever you
change the shaders, or the tool measures a rule that no longer exists.**

`examples/dump_geometry.rs <file.pb>` prints every object of a file with its box or its points.

## Where the two line styles live

The same 40 B row (`CylinderSegment`: two ends, a radius, an instance id, a colour, a `facing`
word holding its two adjacent face normals as oct16) is drawn two ways, and the depth problem
above is present in both:

| what | file |
|---|---|
| the lane: two row tables (`pipes` = mesh/BRep edges, `ribbons` = free linework), the pipelines, the draws, the style switch | `src/engine/gpu/segments.rs` (`draw_pipes`, `draw_ribbons`, `build_pipelines`) |
| FLAT (the default): one camera-facing quad per segment, six vertices pulled by index, a capsule SDF in the fragment, the hairline rule, the density taper, the facing cull, the lift | `src/shaders/ribbon.wgsl` |
| TUBES: a real 3D cylinder per segment on a 6-sided unit-cylinder template, lit like a surface; the radius is the screen pen unless the row carries a world radius | `src/shaders/cylinder.wgsl` (template built by `unit_cylinder` in `segments.rs`) |
| the style itself: `LineStyle::{Tubes, Flat}`, `toggle_line_style` (key `L`), `VIEWER_LINE_STYLE=tubes` / `?style=tubes` | `src/engine/gpu/view.rs` |
| where the style reaches a draw, and the frame order | `src/engine/gpu/render.rs` (`scene_list`, `id_pass`) |
| the pen the shaders read: `thickness`, `proj_y`, `ortho_h`, viewport, eye, `feather` | `src/engine/gpu/frame.rs` (`LineUniform`, 48 B) |
| blend and depth modes of each pipeline (`ReadOnlyEqual` on the ribbons is load-bearing) | `src/engine/pipelines/mod.rs` |
| who produces the rows | `src/app/walk/mesh_ink.rs` (mesh edges + their `facing`), `curves.rs` (line, polyline, NURBS), `frames.rs` (plane, box), `encode.rs` (`pack_facing`, `encode_width`, `pack_rgba`) |
| vertex markers and free points, same lift problem | `src/engine/gpu/glyphs.rs`, `src/shaders/sphere.wgsl`, `src/shaders/glyph.wgsl` |

Neither style is currently reliable, and a fix has to say which one it is fixing:

- **Flat** ties with the face it decorates, which is what the push and the lift exist to break,
  and that is the defect above. It antialiases itself (`?aa=`, 1.5 px) and is the cheaper of the
  two.
- **Tubes** are real geometry, so they never tie - but a tube is a solid: it is shaded, it grows
  and shrinks with the pen, its ends are flat caps that overlap at corners, and it is smoothed
  only by MSAA, which is off for scenes with no solid geometry. `L` switches between them at
  runtime, so both can be compared on the same camera.

## Ground rules of this repository

- `session_viewer/ARCHITECTURE.md` is the map: `app/` knows `session_rust` and never wgpu,
  `engine/gpu/` knows wgpu and never the kernel, one file per lane, and the frame list in
  `engine/gpu/render.rs` is the draw order. Keep that shape.
- At most four parameters per function; grouped inputs become a named struct. No closures unless
  they are the fastest way. A docstring on every function. Minimal comments, and they say WHY.
- Numbers in comments, commit messages or docs are measured or absent.
- Never add Claude, Codex or any AI as a git author or co-author.
- After every push, watch the runs it triggers to completion (`gh run list`, `gh run watch`):
  `viewer-check`, `viewer-pages` and `Session mini tests` must all be green.
