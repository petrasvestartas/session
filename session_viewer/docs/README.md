# Build the Session Viewer

A code-first course. Start from an empty Rust crate, type the parts worth understanding, copy the boilerplate, compile after every step, and finish with the current production viewer, byte for byte.

## What you build

App side, from the window to the upload rows:

![Diagram: lib.rs · App (winit) · state.rs · State · app/loader.rs\ manifest · protobuf · camera.rs · app/input.rs\ touch.rs · app/scene.rs\ source documents\ + identity…](illustrations/README-01.svg)

GPU side, from `Gpu` to its passes:

![Diagram: engine/gpu · Gpu · Faces · Text · Tiles · segments\ ribbon.wgsl\ ink_visibility.wgsl · glyphs · cloud · splat…](illustrations/README-02.svg)

Read [How to use this course](how-to-learn.md) first: it is short, and it says how a lesson is built. Nothing in this course is hidden — every lesson ends in **Questions and answers**, where each question is followed by the reasoning that gets you there and then the answer.

Then read [The map](map.md): one picture of the whole viewer, which every single step of the course reopens with your current position lit. It is the answer to "where does this file sit in the bigger picture", and it is the same picture every time.

Then read [Words before code](words.md) once: every term the lessons use before they have room to explain it, with the file where it first appears. Come back to it whenever a sentence stops making sense. When something breaks, [Reading failures](debugging.md) covers the errors this course actually produces.

## How a lesson reads

- **You are building**: the mechanism this lesson adds, as one diagram.
- **Step k**: one idea, then the code. Every code block names its file and one of four actions: **NEW FILE**, **CURRENT → REPLACE WITH**, **CURRENT → ADD BELOW**, **DELETE**.
- **TYPE THIS** marks code worth writing by hand. **COPY** marks boilerplate, pages, lockfiles, fixtures. **READ ONLY** marks a complete file shown for orientation.
- **Supplied files** are tooling (native examples, fixtures, parity ports) installed by one command; the course does not teach them.
- **Check**: `cargo check` where it is known to pass, then the checkpoint build and what you should see.

Every block is cut from the verified patch of that checkpoint. A replay audit types each lesson literally and requires the result to hash to the checkpoint, and every `cargo check` marker was measured on the typed state.

## Course

| Lesson | You add | Checkpoint |
|---|---|---|
| [00 · Empty project to WASM](00-environment.md) | Crate, wasm32 default, Trunk, a status message | |
| [01 · First WebGPU frame](01-first-frame.md) | Adapter, device, surface, pipeline, one triangle | **1 · WebGPU works** |
| [02 · Camera](02-camera.md) | Production camera and math: orbit, pan, cursor zoom, reversed depth | |
| [03 · Object rows and identity](03-identity.md) | `Instance` rows, storage bind group, `instance_index` | |
| [04a · Meshes on the GPU](04a-meshes.md) | Arena buffers, object table, vertex pulling, `triangle.wgsl` | |
| [04b · Strokes](04b-strokes.md) | Segment lane and the screen-space ribbon shader | |
| [04c · Markers](04c-markers.md) | Vertex markers on a quad template, free dots as one triangle | |
| [04d · Point clouds](04d-clouds.md) | LOD nodes, splat prelude and resolve | |
| [05 · Depth and visible ink](05-visibility.md) | Reversed depth, physical metadata, the ink visibility test | **2 · Camera + meshes work** |
| [06 · CAD face contract](06-cad-contract.md) | Face meshes, UVs, normals, boundary provenance | |
| [07 · Shared boundaries](07-boundaries.md) | One canonical chain per edge, constrained meshing, edge IDs | |
| [08 · Trims and seams](08-trimming.md) | Holes, periodic seams, poles, natural boundaries | |
| [09 · Normals and shading](09-normals.md) | Analytic normals, creases, cofactor normal transform | **3 · CAD visualization works** |
| [10 · Text shaping](10-text-layout.md) | Fonts, shaping, advances, clusters | |
| [11 · Text rendering](11-text-rendering.md) | Placement, raster size, coverage, plates | |
| [12 · Production shell and picking](12-picking.md) | winit `App`, `State`, ID pass, async readback, yellow selection | |
| [13 · Source controls](13-controls.md) | F10 controls, streamed-cloud source queries | **4 · Picking + interaction work** |
| [14 · Loading scenes](14-loading.md) | Manifests, validation, staged replacement | |
| [15 · Publication and streamed reads](15-publication.md) | Immutable revisions, bounded metadata window | |
| [16 · Resource accounting](16-accounting.md) | Weak source cache, owned-capacity numbers | |
| [17 · Faces, text objects, silhouettes](17-source-presentation.md) | Source faces, selectable text, one black outline, joined strokes | |
| [18 · Finite-triangle visibility](18-finite-visibility.md) | Projected triangles, tile lists, cache | |
| [19 · Sheets](19-sheets.md) | Batched drawings, ranged slices, lazy entity metadata | |
| [20 · The document](20-history.md) | Kernel history: transactions, tombstones, undo and redo, purge on save | **5 · Full viewer** |

Then [the capstone](capstone.md): a section plane, with requirements, constraints, the reasoning for each decision worked through, and the full design written out — but no line-by-line instructions.

Dependencies: 01 → 02 → 03 → 04a–d → 05 are strictly sequential. 06–09 change the shared kernel and only need 05. 10–11 need 04c. 12 builds the production shell and needs everything before it. 13–16 extend `State` and loading. 17–18 refine presentation and visibility on top of 12. 19 streams drawing sheets on top of 15 and 17. 20 changes the shared kernel only and needs 19 for the inventory.

## Prepare one workspace

Run from the maintained `session_viewer` checkout. Choose a **new, empty** path for your work.

```sh
export COURSE_REPO="$PWD"
export COURSE_WORK="$HOME/viewer-course"
export RUSTUP_TOOLCHAIN=1.97.1
export REGEN_PROTO=0
export NO_COLOR=true
rustup toolchain install 1.97.1
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14 --locked
python3 "$COURSE_REPO/docs/reconstruction/replay.py" --output "$COURSE_WORK" --initialize-only
```

- `COURSE_REPO` holds the course, patches and fixtures. `COURSE_WORK` is where you type.
- The initializer extracts the pinned kernel and its siblings; it writes no viewer code.
- Python 3.13 or newer; Git is needed by the replay tools.

## Build, run, stop

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>. Stop with **Ctrl+C** before the next lesson. Port 8780 keeps the course apart from a production viewer on 8770.

A passing `cargo check` proves Rust. A Trunk build proves the WASM bundle. Pixels in the browser prove the GPU work; an empty canvas is not a passing checkpoint.

## Exact reconstruction, optional

- `replay.py --output "$COURSE_WORK" --through NN --copy-supplied` installs the supplied files for a lesson.
- `replay.py --output "$COURSE_WORK" --through NN --adopt` verifies your typed sources against the checkpoint hashes.
- `replay.py --output "$HOME/viewer-course-auto" --through NN` writes a lesson for you in a separate workspace.
- Lesson 20 ends with `converge.py`, which compares the final workspace with the frozen production inventory.

## The result

- Served viewer: <https://petrasvestartas.github.io/session/> (this course at [/docs/](https://petrasvestartas.github.io/session/docs/)).
- Source: <https://github.com/petrasvestartas/session/tree/main/session_viewer>.
- Locally, `trunk serve` serves both: the viewer at <http://localhost:8770/> with a black corner at the top right that opens the course.

## After the course

- [Architecture reference](../ARCHITECTURE.md): module graph, frame, Rust ↔ WGSL interfaces, flows, lifecycles.
- [CAD design record](cad-design.md): the shared-boundary and normal contracts, with the OCCT comparison.

## Build this site

```sh
docs/serve.sh build
python3 docs/check_site.py
python3 docs/course_pages.py --audit
docs/serve.sh
```

Two more tools keep the lessons honest about building: `python3 docs/reconstruction/step_checks.py` runs `cargo check` at the end of every step of every lesson, and `python3 docs/reconstruction/step_status.py` writes the result into each lesson as the "Does it compile yet?" line (`--check` fails when one is stale).

The build expands lesson directives from the verified patches, so lesson code is never duplicated in Git. `check_site.py` checks links, downloads and lexers; `course_pages.py --audit` checks that every checkpoint change is taught or supplied exactly once and that typing each lesson reproduces its checkpoint.

Flowcharts are D2: edit `docs/diagrams/<lesson>-<n>.d2` and run `python3 docs/diagrams.py`, which fetches its own pinned renderer the first time and writes the committed SVG. Every step's "where you are" map comes from `python3 docs/locator.py` (`--check` fails when a lesson is stale), and `python3 docs/check_svg.py docs/illustrations/*.svg` measures every label in a real Chrome when playwright is unavailable.

Illustrations are generated: `python3 docs/illustrations/draw.py` sizes every box from its text, and `node docs/check_illustrations.cjs --write` measures every label in Chrome, fails on any overflow or collision, and pins each label's measured width so other fonts cannot overflow either. The palette is the BRG Equilibrium drawing palette (navy, pink, green, yellow, pale bands), also applied to the Mermaid diagrams: a reader carries those colours from a diagram to the running viewer, so they are the drawing's own and no theme replaces them.

Everything around them - ground, plates, rules, type, radius, density, the accent - comes from a Claude Design system, and the link runs both ways. Pulling: `docs/stylesheets/theme.json` is the project exported from claude.ai/design, `python3 docs/stylesheets/theme.py` renders it into `theme.css`, `docs/stylesheets/fonts/fetch.sh` refetches the faces it names against their recorded sha256, and `check_site.py` fails the build if `theme.css`, the committed faces and `theme.json` ever disagree. `course.css` is the only consumer, so switching systems is one export rather than a search through the stylesheet. Pushing: `python3 docs/design_cards.py` builds the drawing palette, the figure plate and all the illustrations into `target/docs/cards/`, ready to upload back to the same project so the graphics can be reviewed as one set on the canvas.
