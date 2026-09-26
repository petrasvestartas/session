# Viewer course plan

Written 2026-09-26 from `docs/plans/docs-handoff.md`. The course teaches a reader who has never seen WebGPU to type
`session_viewer/src` from an empty folder, one hour at a time. This file is the working plan: the part list is
fixed, the lesson list of a part is fixed when that part is cut, and the numbers below are re-measured as parts land.

## Requirements (from petras)

- Typing every code block from the first page to the last gives exactly `session_viewer/src`. Nothing typed is
  ever replaced: a lesson adds files, or appends to files it owns, or inserts one `register:` line into a list.
- One lesson works on its own: it builds, runs and shows something new. At most 250 typed lines (Rust, WGSL,
  toml, html; tests, examples, assets and `Cargo.lock` are downloads).
- Step by step to the final viewer: window, clear colour, triangle, camera, first mesh, ... No sideways tutorial.
- The voice: problem and idea first, concrete before general, one idea per sentence, a term defined when it
  appears, step headings name the idea, a checkpoint "run it now, you should see ...", a 2-4 sentence recap.
  Pages 00-03 of the old chain are the voice model.

## Measured state (2026-09-26, `python3 docs/check_budget.py`)

- Old chain: 43 lessons, 71,372 typed viewer lines, 32 lessons over 250. Lesson 01 alone types 4,572 lines and
  shows nothing until lesson 12 opens the window.
- `check_write_once.py`, `check_lesson37.py` and `check_complete.py viewer` (5,728 unshown lines) run here;
  `cut.py --check` needs the raw-cut baseline in the local `~/.cache` and is skipped in the cloud.
- The chain is one master crate `docs/lessons/37` (src plus teaching comments and `--8<--` section markers),
  cut by `docs/cut.py` from `FILES.txt` (file, first lesson), `REGISTER.txt` (file, tag, lesson) and
  `SERIES.txt` (id, parent, page).

## Ids

- Three digits, in typing order: `000` is the empty project, the last is the finished viewer. `cut.py`,
  `check_complete.py` and the page names accept `\d{2,3}[a-z]?`; a letter (`012a`) inserts a lesson later without
  renumbering. Pages are `docs/NNN-slug.md`.
- The old chain converts from the front: while a part is being cut, `SERIES.txt` is the new ids followed by the
  old ids still to convert (`000 ... 011 04a 04b ... 37`). `cut.py` orders lessons by their row in `SERIES.txt`,
  so the mixed chain stays one chain and every crate still cuts. The master keeps its id `37` until the tail is
  converted; then it becomes `docs/lessons/final` and `check_lesson37.py` becomes `check_master.py`.

## Rules of the cut (what the tools enforce)

- A line of the master belongs to a lesson: the section `// --8<-- [start:NNN-slug]` around it, or the
  `register:tag` it carries (`REGISTER.txt`), or the file's first lesson (`FILES.txt`). Crate N keeps every line
  of lesson <= N.
- Inside a file, lesson order is line order: a section may not be earlier than the lines around it, and a
  later lesson may only append (write-once). The one exception is a single line carrying `register:`, inserted
  anywhere; an attribute line directly above it joins it.
- Every crate compiles on wasm and natively, so a lesson's cut must leave no dangling call: a later lesson's
  lines are a whole item (fn, impl block, struct, shader function) or one `register:` line in a list.

## Restructuring `src` (step 3, behaviour-neutral)

The old lesson 01 is big because the frame loop reaches every lane: `Gpu::build` makes every lane, `encode_frame`
runs every pass, `State::render` polls picks, clouds and panels. Four moves, all without a visual change, gated by
`cargo test` (native, `RUST_TEST_THREADS=4`), `cargo xtest`, `cargo check --target wasm32-unknown-unknown` and the
`viewer-check` run:

1. **Order by lesson.** Inside a file, move whole items (fns, impl blocks, shader functions) so the earliest
   lesson's items come first. Rust and WGSL accept any item order. This alone makes most sections legal.
2. **Hoist later blocks.** A multi-line block a later lesson adds in the middle of a function moves into a helper
   method appended to the file (or a later file), called by one `register:` line. `State::render` keeps its
   `features::BEFORE_PICKS` hooks, the passes keep `pass::PASSES`, the lanes `lane::REGISTRY`, the verbs their
   `REGISTRY`: new features join through these lists, not through edits to old lines.
3. **Split by first lesson.** A file whose items belong to lessons far apart splits along that seam
   (`scene_sync.rs` 3,265 lines, `clip.rs` 2,380, `ssao.rs` 1,798, `layers.rs` 1,397, `objects.rs` 977, `slots.rs`
   736). The module keeps its name; the new file is a `mod` with `#[path]` or a plain submodule, joined by one line.
4. **The page splits too.** `index.html` keeps the canvas, the base style and the rust link in lesson 000. The
   early-fetch script, the error box, the status line, the phone input and the docs corner move to files under
   `src/page/` that Trunk inlines (`<link data-trunk rel="inline">`), each linked by one `register:` line in the
   lesson that needs it. The built page keeps the same content.

Not allowed: changing what a frame draws, renaming public knobs, moving assets, editing tests to fit the cut.

## Parts

Typed lines are the files whose first lesson falls in the part (`register:` lines of shared files such as
`lib.rs`, `state.rs`, `scene.rs` are counted where they land). Lessons are typed lines / ~230, so every lesson
has room under 250. The order is dependency order: a lesson only uses what earlier lessons typed, and the reader
sees geometry as soon as a mesh can reach the GPU.

| Part | Ids | What the reader can do at the end | Typed | Lessons |
|---|---|---|---|---|
| 1 First pixels | 000-029 | An empty page becomes a grey canvas, a white background, a grid, a camera that orbits, pans and zooms with the mouse and fingers, a depth buffer, a resize that keeps the picture sharp, and frame timing. | 6,872 | ~30 |
| 2 First mesh | 030-059 | One `.pb` file downloads, decodes and walks into GPU rows; object rows and the arena draw a shaded mesh; the camera fits it. | 6,686 | ~30 |
| 3 Strokes, markers, clouds | 060-081 | Lines as pipes and ribbons, vectors with arrowheads, markers and dots, point clouds with LOD and splats; curves, points, frames and planes walk into them. | 4,940 | ~22 |
| 4 CAD faces and ink | 082-096 | BReps walk into faces with trims, holes and seams; shared edges draw once; slots and hulls give every triangle its object; visible-ink tests hide lines behind faces. | 3,437 | ~15 |
| 5 Text | 097-110 | Labels are shaped from the bundled fonts, drawn on planes and plates, and checked on the text-quality page. | 3,079 | ~14 |
| 6 Picking and selection | 111-120 | A click reads the object id under the cursor; objects, edges, faces and control points select; keys run actions; the inspection API and session files. | 2,200 | ~10 |
| 7 Scenes, streams, sheets | 121-137 | Manifests list many files; live sources poll; clouds and drawing sheets stream in slices and answer point queries; released documents are accounted and hydrated back. | 3,831 | ~17 |
| 8 Finite visibility, instancing, clipping | 138-169 | Triangle tiles settle visibility exactly; surface outlines and face coverage; instanced copies; clipping planes with section caps. | 7,156 | ~32 |
| 9 Editing | 170-210 | Objects, control points and gumball drags edit the scene and sync back to the documents; undo and redo; snapping; construction planes; layers; previews. | 9,346 | ~41 |
| 10 Panels | 211-215 | egui panels over the frame: theme, pointer routing, overlay, number box, the phone keyboard. | 985 | ~5 |
| 11 Commands | 216-233 | The command line, tools, the verb registry and the plain verbs: point, line, curve, polyline, fit, hide, show, undo, redo, save, open, delete, explode, clipping plane, selection modes. | 4,121 | ~18 |
| 12 Tools and shapes | 234-284 | Gather, cut and shape tools; move, copy, rotate, scale, orient; lasso; trim and extend; thirteen primitive shapes; extrude, loft and the NURBS curves and surfaces; measure, area, length, volume, text, arrowheads, project to plane. | 11,561 | ~51 |
| 13 Gumball, layers, hierarchy | 285-301 | The gumball widget; the layer tree and graph panels; groups and edges; splitting. | 3,714 | ~17 |
| 14 Lighting and looks | 302-313 | Ambient occlusion, contact shadows, GPU timing, the Arctic and outline looks, attributes and opacity. | 2,628 | ~12 |
| 15 Self-test | 314-317 | The native harness renders scenes to files and checks lifecycle pixels. | 816 | ~4 |

Total 71,372 typed lines, about 311 lessons. Ids of a part are fixed when it is cut; a part that needs more
lessons than its range pushes the later ranges, which are re-stated here.

## Part 1 in detail (first pixels)

Every lesson ends in the browser with `trunk serve`. Lessons 000-008 are cut and pushed (2026-09-26); their typed
lines are measured by `check_budget.py`.

| Id | Lesson | Types | You should see | Typed |
|---|---|---|---|---|
| 000 | Empty project to a wasm page | `Cargo.toml`, `.cargo/config.toml`, `Trunk.toml`, `.gitignore`, bare `index.html`, `lib.rs` entry | a grey page from CSS, an empty console | 201 |
| 001 | A window on the canvas | `lib.rs` App, Msg, events, `mouse`, `viewer_canvas`, `start`; `state.rs` skeleton; `app/mod.rs`; `feedback.rs` status and error; `loader.rs` boot | the canvas carries winit's width and height | 197 |
| 002 | Open the GPU | `engine/mod.rs`, `performance.rs` clock, `gpu/mod.rs` list and `Gpu`, `buffers.rs` GpuCtx, `view.rs` knobs, `device.rs` open; the state's GPU lines | console: `adapter: ...`, `viewer init OK - surface WxH`, `gpu init N ms` | 243 |
| 003 | The frame's textures | `targets.rs` TextureSpec, Attachment, Targets, `begin_faces`; the `targets` lines of `gpu/mod.rs` | cargo check passes, same console | 126 |
| 004 | Clear to a colour | `present.rs` FrameInput and present, `render.rs` Frame and encode_frame, `state.rs` render and gpu_failed, `lib.rs` redraw | the GPU paints the canvas 0.9 grey; a red experiment proves it | 121 |
| 005 | Resize and device pixels | `Gpu::resize`, `retarget`, `samples_wanted`, `State::resize`, `logical_size`, `page_hidden`, `desired_canvas_size`, `fit_canvas`, `resize_held`, `view.rs` device pixel ratio and `View`, `Targets::samples_for` and the 4x textures | the canvas follows the window; `?dpr=1`, `?msaa=4` | 236 |
| 006 | The pipeline cache | `pipelines/mod.rs` counters, Lazy, Cache, DepthMode, ColorWrite, PipelineKey, Target | `cargo xtest`: the lazy test passes | 149 |
| 007 | Describe and build a pipeline | `pipelines/mod.rs` PipelineDesc, wgsl, layout, pipeline_layout, build, compile | cargo check, same canvas | 198 |
| 008 | Shaders in the crate | `build.rs`, the `shader!` macro, `scene.wgsl`, `normals.wgsl`, `physical.wgsl`, the prelude helpers, `layouts.rs` mvp and line | the stripped shader in OUT_DIR | 190 |
| 009 | The frame uniform | `frame.rs` LineUniform, FrameUniforms, write; `write_frame_uniforms` | same picture, drawn through the frame bind group | |
| 010 | The object table | `instance.rs`, `buffers.rs` GrowBuf, `objects.rs` core: rows, translations, group 2 | tests pass | |
| 011 | The background triangle | `backdrop.rs`, `lane.rs` trait, `lane_list!`, the backdrop pass | a white canvas: the fullscreen triangle | |
| 012 | The camera | `camera.rs` pose, view and projection | tests print a projected point | |
| 013 | The anchor | `objects.rs` rebase, `State::rebase`, `view_proj_anchored` | tests pass | |
| 014 | The grid | `grid.wgsl`, `draw_grid`, `grid_list` | a grid on the floor plane | |
| 015 | Orbit, pan and zoom | `input.rs` mouse, `camera.rs` orbit, pan, zoom | the grid turns under the mouse | |
| 016 | Touch | `touch.rs`, pinch and two-finger pan | the same on a phone | |
| ... | frame timing and drag tiers, GPU errors and device loss, MSAA, the status line, camera fit and extent, spin, the docs corner | the rest of the Part 1 files | | |

While a part is being cut, the old lessons that follow it stay in `SERIES.txt` as the tail (today `04a` holds
everything Part 1 has not yet cut: 9,400 typed lines). `mkdocs.yml` sets `check_paths: false` so the old pages, whose
includes moved, build with warnings until they are re-cut.

## Page template (the voice)

```
# NNN · Idea in four words

One paragraph: the problem, then the idea. Concrete first ("object 7 paints the value 7").

## Step heading that names the idea

`lessons/NNN/src/file.rs` · new file | append | one line in the list

Two to four sentences: what the code does and the one term it introduces.

```rust
--8<-- "lessons/NNN/src/file.rs:NNN-slug"
```

## Checkpoint

Run `trunk serve`, open http://127.0.0.1:8770. You should see ...

## Recap

Two to four sentences.

Next: [NNN+1 · ...](NNN+1-slug.md)
```

Tests, examples and assets a lesson needs are one line with a download link, never a code block.

## Process per part

1. Plan the part's lessons here (id, title, files, checkpoint).
2. Restructure `src` for the part (rules above); run the four gates; commit `src` on its own.
3. Mirror `src` into the master (`lessons/37`, later `final`), add the section markers, update `FILES.txt`,
   `REGISTER.txt` and `SERIES.txt`; `python3 docs/cut.py`.
4. Build every new crate: `cargo check --lib` (wasm) and `cargo xtest` with one shared `CARGO_TARGET_DIR`.
5. Write the pages in the voice; `check_write_once.py`, `check_lesson37.py`, `check_complete.py viewer` and
   `check_budget.py` at 0 for the new ids (the old tail stays over budget until converted).
6. Commit with a plain message, push to main, watch `viewer-check`, `viewer-pages` and `Session mini tests` to
   green before the next part. Checks that need the local machine (GPU browser checks, `cut.py --check` against
   the private baseline) are skipped and named in the commit message.

## After the chain (lower priority)

Remove MkDocs (`mkdocs.yml`, `docs/hooks`, `build_site.sh`, `serve.sh`), the command reference page and the
ARCHITECTURE/README pages from the nav; redraw the illustrations in black and grey.
