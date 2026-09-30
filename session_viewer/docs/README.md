# Build and understand the Session Viewer

Begin with a small picture whose whole path you can explain. Then grow the same project toward the viewer we use today, preserving its functionality and carefully developed rendering.

**Start with [the course route](journey.md).** Its cumulative lessons now cover drawing, picking, undo, a perspective camera, lit solids, resizing and pointer gestures. Browser checks remain pending, and the advanced courses are still being written. The [complete implementation reference](reference-course.md) preserves the required final feature set.

You type every piece of application code. Lessons may revisit earlier files, explain why the change is needed and finish with a working checkpoint. [Save and recovery commands](journey/recovery.md) protect your own work. Vue hosts these documents; Rust and wgpu implement the viewer.

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

Follow [lesson 01: a page that Rust can reach](journey/01-canvas.md). Rust concepts are explained where you use them; the [foundations](foundations.md) provide additional practice when useful.

Keep the [destination contract](journey/destination.md) nearby. It records the features and quality the complete course must eventually demonstrate. The first triangle is a starting point, not the end of the viewer.
