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

## Begin

Follow [lesson 1.1: Rust and the first GPU frame](journey/consolidation/01-canvas.md). Rust concepts are explained where you use them; the [foundations](foundations.md) provide additional practice when useful.

Keep the [destination contract](journey/destination.md) nearby. It records the required features, rendering quality and acceptance checks.
