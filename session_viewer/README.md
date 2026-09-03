# session_viewer

A browser-only, WebGPU-only CAD viewer for `session` geometry, written in Rust and compiled to
WASM with [Trunk](https://trunkrs.dev). Draws meshes, BReps, NURBS, linework, points and point
clouds from the geometry kernel with camera-relative f64, reverse-Z depth, an octree-streamed
point lane and GPU id-buffer picking.

## Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

A recent WebGPU browser (Chrome, Edge, Firefox, or Safari 18+).

### WebGPU on Ubuntu

Linux browsers don't expose hardware WebGPU out of the box.

**Chrome** needs Vulkan features, and on Wayland desktops must run under XWayland
(Vulkan is incompatible with Chrome's Wayland backend, the window won't open):

```bash
google-chrome --ozone-platform=x11 --enable-features=Vulkan,DefaultANGLEVulkan,VulkanFromANGLE
```

To make it permanent, copy `/usr/share/applications/google-chrome.desktop` to
`~/.local/share/applications/` and add the same switches to every `Exec=` line.
Leave `chrome://flags` at defaults. If Chrome refuses to start at all, delete stale
`~/.config/google-chrome/Singleton*` symlinks.

**Firefox** needs one pref: `about:config` -> `dom.webgpu.enabled` -> `true`, then restart.

## Run the viewer

```bash
cd session_viewer
trunk serve        # -> http://localhost:8770
```

Open http://localhost:8770 in a WebGPU browser. That shows the LOCAL scene and nothing else:
`assets/view_local.toml` and the `assets/pb/view_local_*.pb` it names, served by trunk, so the
page works with the network off. Edit the manifest and reload; edits under `src/`, `Cargo.toml`
and `index.html` hot-reload.

Every other scene lives in the Cloudflare R2 bucket `session-viewer-data` and is opened by name,
from the bucket, never from `assets/`:

```
http://localhost:8770/?scene=view_lines          # scenes/view_lines.toml in the bucket
http://localhost:8770/view_mixed                 # the same, path form
```

The deployed page at https://petrasvestartas.github.io/session/ takes neither of those: with no
query it watches `view_live.toml` in the bucket and re-reads it every poll, so publishing
geometry redraws it without a build or a deploy:

```bash
bash/view_put.sh out/scan.pb              # pb/view_scan.pb + scenes/view_scan.toml, prints the ?scene=
bash/view_live.sh scene.toml scan.pb      # replace the live scene; open pages swap in ~1 s
```

## Native harness

The same tree renders headless through Vulkan for numbers and pixels (see `ARCHITECTURE.md`
section 11):

```bash
CARGO_TARGET_DIR=~/.cache/tmain REGEN_PROTO=0 \
cargo run --release --example selftest --target x86_64-unknown-linux-gnu -- out.ppm assets/view_local.toml
cargo xtest      # the mirror and parser tests
```

## Read the docs

`docs/` is the lesson series that builds this viewer from an empty crate, one compilable step at
a time; `docs_archive/` holds the earlier series it replaced. Serve either as a static site:

```bash
cd session_viewer/docs
python serve.py    # -> http://localhost:8771
```

## Architecture

[ARCHITECTURE.md](ARCHITECTURE.md) is the source of truth: the two halves and the line between
them, the frame order, picking, the thickness rule, point streaming, the live source, and the
checklist for adding or deleting a lane.
