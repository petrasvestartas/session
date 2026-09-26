# Build the Session Viewer

You type a CAD viewer in Rust, WebAssembly and WebGPU from an empty folder, one file at a time. Most lessons end with something you can see in the browser.

## Install on a fresh computer

Linux, in a terminal:

```sh
# 1. compiler tools, curl, git
sudo apt install build-essential curl git

# 2. Rust, pinned to the version the course was verified with
curl https://sh.rustup.rs -sSf | sh
rustup toolchain install 1.97.1
rustup default 1.97.1
rustup target add wasm32-unknown-unknown

# 3. Trunk: builds the page and serves it
cargo install trunk --version 0.21.14 --locked

# 4. the kernel the viewer draws
git clone --recurse-submodules https://github.com/petrasvestartas/session.git
```

5. Chrome, for WebGPU.

Check:

```sh
rustc --version      # rustc 1.97.1
trunk --version      # trunk 0.21.14
ls session           # session_rust  session_cpp  session_py  session_proto ...
```

## Where you type

Your crate goes inside `session`, next to `session_rust`. Name the folder `session_view`.

```text
session/
├── session_rust/
├── session_cpp/  session_py/  session_proto/
└── session_view/        ← lesson 00 creates this; every command runs inside it
```

## Run

```sh
cargo check --lib
trunk serve --port 8780
```

Open <http://localhost:8780/>. Stop with Ctrl+C.

## Lessons

The course is one chain, from 00 to 37. Every lesson adds new files or appends to files it already owns, so nothing you type is ever replaced later; the crate in `docs/lessons/37` is the finished viewer. Letter ids (04a-04d, 18a, 18b, 23a-23d) are lessons inserted where their subject belongs. The numbers 27, 28, 29 and 34 are retired: their code now sits in the lessons that own its subject.

- [00 · Empty project to a WASM message](00-environment.md)
- [01 · First WebGPU frame](01-first-frame.md)
- [02 · Camera](02-camera.md)
- [03 · Object rows and identity](03-identity.md)
- [04a · Meshes on the GPU](04a-meshes.md)
- [04b · Strokes and arrows](04b-strokes.md)
- [04c · Markers](04c-markers.md)
- [04d · Point clouds](04d-clouds.md)
- [05 · Depth and visible ink](05-visibility.md)
- [06 · CAD face rules](06-cad-contract.md)
- [07 · Shared boundaries](07-boundaries.md)
- [08 · Trims, holes and periodic seams](08-trimming.md)
- [09 · Normals and shading](09-normals.md)
- [10 · Text shaping](10-text-layout.md)
- [11 · Text rendering](11-text-rendering.md)
- [12 · The viewer shell and picking](12-picking.md)
- [13 · Source controls and cloud picks](13-controls.md)
- [14 · Loading scenes](14-loading.md)
- [15 · Publication and streamed reads](15-publication.md)
- [16 · Resource accounting and release](16-accounting.md)
- [17 · Source faces, text objects and one silhouette](17-source-presentation.md)
- [18 · Finite-triangle visibility](18-finite-visibility.md)
- [18a · Instancing](18a-instancing.md)
- [18b · Clipping planes and section caps](18b-clipping.md)
- [19 · Sheets: batched drawings with lazy metadata](19-sheets.md)
- [20 · The document and its rows](20-history.md)
- [21 · Direct editing](21-editing.md)
- [22 · The egui layer](22-runtime-helpers.md)
- [23 · The command line: one verb per file](23-geometry-commands.md)
- [23a · Tools that ask for points](23a-tools.md)
- [23b · Shapes](23b-shapes.md)
- [23c · Surfacing](23c-surfacing.md)
- [23d · Annotate and measure](23d-annotate-measure.md)
- [24 · Snapping](24-placed-controls.md)
- [25 · Draw a solid, readable gumball](25-gumball.md)
- [26 · The nested session panel](26-nested-panel.md)
- [30 · The layers panel and the layer tree](30-layer-tree.md)
- [31 · Split curves and faces](31-splitting.md)
- [32 · Ambient occlusion](32-colors-lighting.md)
- [33 · GTAO, Arctic and Outline](33-contact-shadows.md)
- [35 · Attributes On|Off](35-attributes.md)
- [36 · Translucent faces and Opacity](36-translucent-faces.md)
- [37 · Self-test: the finished viewer](37-command-dock.md)

Do them in order. [How to use this course](how-to-learn.md) explains the line above each code block; [Reading failures](debugging.md) explains the errors.
