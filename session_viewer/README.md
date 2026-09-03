# session_viewer

A browser-only, WebGPU-only CAD viewer for `session` geometry, written in Rust and
compiled to WASM with [Trunk](https://trunkrs.dev). Renders meshes, NURBS surfaces and
BReps from the geometry kernel with camera-relative f64, reverse-Z depth and CPU
ray + BVH picking.

## Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

A recent WebGPU browser (Chrome, Edge, Firefox, or Safari 18+).

### WebGPU on Ubuntu

Linux browsers don't expose hardware WebGPU out of the box.

**Chrome** needs Vulkan features, and on Wayland desktops must run under XWayland
(Vulkan is incompatible with Chrome's Wayland backend — the window won't open):

```bash
google-chrome --ozone-platform=x11 --enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE
```

To make it permanent, copy `/usr/share/applications/google-chrome.desktop` to
`~/.local/share/applications/` and add the same switches to every `Exec=` line.
Leave `chrome://flags` at defaults. If Chrome refuses to start at all, delete stale
`~/.config/google-chrome/Singleton*` symlinks.

**Firefox** needs one pref: `about:config` → `dom.webgpu.enabled` → `true`, then
restart Firefox.

## Run the viewer

```bash
cd session_viewer
trunk serve        # → http://localhost:8770
```

Open http://localhost:8770 in a WebGPU browser. Edits under `src/`, `Cargo.toml` and
`index.html` hot-reload.

## Read the docs (build-log lessons)

The step-by-step lessons that build this viewer up (`01-run` → `44-cloud-octree`, then the
refactor block `46` → `51`) live in
`docs/` as a self-contained static site — no build step, no `session_tests` dependency.

```bash
cd session_viewer/docs
python serve.py    # → http://localhost:8771
```

Open http://localhost:8771. The left sidebar lists every lesson; click to read. Each
`NN_title/` folder is a full crate snapshot of the viewer at the end of that lesson —
copy one out and `trunk serve` it to see exactly that chapter's result. See
[docs/README.md](docs/README.md) for how the site works and how to add a lesson.

## Architecture

[ARCHITECTURE.md](ARCHITECTURE.md) is the source of truth for module layout, the
GPU/f64 boundary policy and the render pipeline.
