# Build and understand the Session Viewer

Build the Session Viewer from scratch with Rust and wgpu. Work through one cumulative project until its code, production layout and behavior match the maintained viewer.

Begin with [chapter 1: four consolidated lessons](journey/consolidation/01-canvas.md). The [production source and coverage map](journey/production-course-map.md) distinguishes reused code, staged integration, divergent algorithms and missing coverage. All original checkpoint links and recovery IDs remain available.

**Start with [the course route](journey.md).** The course is being consolidated by subject: fewer complete lessons, lean code, and no late collection of features that belong earlier. The [lesson checklist](journey/roadmap.md) tracks acceptance separately. The [archived tutorials](archived/README.md) preserve the earlier reference course; they do not represent today's finished viewer.

Each subject has one visible goal, a prediction experiment, a question and a runnable checkpoint. Teach each concept once, in its final home. Small instructions stay inside a lesson instead of becoming separate checkpoints. [Save and recovery commands](journey/recovery.md) protect your own work. Vue hosts these documents; Rust and wgpu implement the viewer.

The first new [mathematics checkpoint](journey/03a-mathematics.md) belongs immediately after the early egui painting. It covers font metrics, TeX, glyph pixels and equals alignment; cumulative course integration is still in progress.

## Prepare your computer

These commands describe the Linux setup used for the course. Install Rust, compiler tools, Git, Node.js 20.6 or newer with npm, and Chrome with WebGPU support. A native GPU driver is needed for rendered-image exercises.

```sh
sudo apt install build-essential curl git
curl https://sh.rustup.rs -sSf | sh
```

Open a new terminal so Rust is on your path, then:

```sh
rustup toolchain install 1.97.1
rustup default 1.97.1
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
git clone --recurse-submodules https://github.com/petrasvestartas/session.git
cd session/session_viewer
rustc --version
trunk --version
```

Build the Vue documentation once so your handwritten viewer can open its documentation link. From `session_viewer`:

```sh
cd ../session_tests
npm ci
DOCS_STRICT_LINKS=1 DOCS_BASE=/docs/ npm exec vite build -- --outDir ../session_viewer/dist/docs --emptyOutDir
cd ../session_viewer
```

Keep Cargo.lock: it records the dependency versions used by the viewer. Allow extra time for installation and the first dependency build.

## Understand the production build

Read the maintained [Cargo manifest](../Cargo.toml), [shader build script](../build.rs), [Trunk configuration](../Trunk.toml) and [browser page source](../index.html). Cargo builds Rust dependencies; Trunk connects the wasm module, browser entry page and copied assets. The `cdylib` library produces the browser module, while `rlib` lets native verification examples link the same crate.

The kernel dependency disables its default features and enables `viewer-display`. The UI uses matching egui, egui-wgpu and egui-winit releases. Mathematics needs `ttf-parser` for OpenType layout, `swash` for glyph rasters, `png` for pictures and `serde_yaml_ng` for decks. Glyphon supplies the scene text renderer. These are different jobs: the mathematics painter cannot substitute scene text for font glyph IDs and MATH-table positioning. Preserve the lock so the shared font stack stays consistent.

`build.rs` expands shader includes into `OUT_DIR/shaders`, includes each file once per output shader, and strips line comments and excess whitespace. The crate's `shader!` macro reads those generated files. Edit the source shaders, not the generated cache. Block comments are rejected by this minifier instead of being silently left in its output.

Trunk uses an optimized build, and its pre-build hook prepares the Vue site. The page copies stage fonts to `stage/`, the five example decks to `decks/`, ordinary text fonts to `text/`, and the compiled documentation to `docs/`. Copying the wasm alone cannot produce a working mathematics viewer. The development server serves a mutable index without caching it, so that index names the current hashed bundle.

After the first-frame subject, predict which request fails if the `stage/` asset directory is absent: wasm can initialize, but the named font fetch cannot complete the deck. Restore the directory and reload; the expected result is a painted slide, not an empty substitute. The [startup owner](journey/consolidation/34g-heartbeat.md) explains early requests and application adoption. This production-build explanation does not replace the cumulative checkpoint checks.

## Begin

Follow [lesson 1.1: Rust and the first GPU frame](journey/consolidation/01-canvas.md). Rust concepts are explained where you use them; the [foundations](foundations.md) provide additional practice when useful.

Keep the [destination contract](journey/destination.md) nearby. It records the required features, rendering quality and acceptance checks.
