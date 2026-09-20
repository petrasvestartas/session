# Build the Session Viewer

You type a CAD viewer in Rust, WebAssembly and WebGPU from an empty folder, one file at a time. Every lesson ends with something you can see in the browser.

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

| Lesson | You type | See |
|---|---|---|
| [00 · Empty project to WASM](00-environment.md) | Crate, wasm32 default, Trunk, a status message | a line of text |
| [01 · First WebGPU frame](01-first-frame.md) | Adapter, device, surface, pipeline | one triangle |
| [02 · Camera](02-camera.md) | Orbit, pan, cursor zoom, reversed depth | |
| [03 · Object rows and identity](03-identity.md) | `Instance` rows, storage bind group | |
| [04a · Meshes on the GPU](04a-meshes.md) | Arena buffers, object table, `triangle.wgsl` | |
| [04b · Strokes](04b-strokes.md) | Segment lane, screen-space ribbon shader | |
| [04c · Markers](04c-markers.md) | Vertex markers, free dots | |
| [04d · Point clouds](04d-clouds.md) | LOD nodes, splats | |
| [05 · Depth and visible ink](05-visibility.md) | Reversed depth, ink visibility | camera + meshes |
| [06 · CAD face contract](06-cad-contract.md) | Face meshes, UVs, normals | |
| [07 · Shared boundaries](07-boundaries.md) | One chain per edge, constrained meshing | |
| [08 · Trims and seams](08-trimming.md) | Holes, seams, poles | |
| [09 · Normals and shading](09-normals.md) | Analytic normals, creases | CAD visualization |
| [10 · Text shaping](10-text-layout.md) | Fonts, shaping, clusters | |
| [11 · Text rendering](11-text-rendering.md) | Placement, coverage, plates | |
| [12 · Production shell and picking](12-picking.md) | winit `App`, `State`, ID pass, selection | |
| [13 · Source controls](13-controls.md) | F10 controls, cloud queries | picking + interaction |
| [14 · Loading scenes](14-loading.md) | Manifests, validation | |
| [15 · Publication and streamed reads](15-publication.md) | Revisions, metadata window | |
| [16 · Resource accounting](16-accounting.md) | Source cache, capacity numbers | |
| [17 · Faces, text objects, silhouettes](17-source-presentation.md) | Source faces, selectable text, outlines | |
| [18 · Finite-triangle visibility](18-finite-visibility.md) | Projected triangles, tile lists | |
| [19 · Sheets](19-sheets.md) | Batched drawings, ranged slices | |
| [20 · The document](20-history.md) | Transactions, undo and redo | full viewer |
| [21 · Editing](21-editing.md) | Gumball, snapping, construction plane, command line, layers | |

Do them in order. [How to use this course](how-to-learn.md) explains the code-block labels; [Reading failures](debugging.md) explains the errors.
