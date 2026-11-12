# Session Viewer

The active tutorials live in `session_viewer/docs/journey/`, the course source used by the GitHub Pages documentation build.

A browser CAD viewer in Rust, compiled to WebAssembly and rendered with WebGPU. It shows meshes, BReps, NURBS, lines, points, point clouds and text, and keeps the identity of each original object.

- [Course](docs/README.md): type the viewer step by step.
- [Architecture](ARCHITECTURE.md): reference.

## Run

Toolchain: Rust/Cargo 1.97.1 and Trunk 0.21.14. Run from this folder.

```sh
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14
cargo generate-lockfile
cargo check --lib
trunk serve
```

Open <http://localhost:8770/> in a browser with WebGPU. Tested: Chrome on Linux with Vulkan. If WebGPU is missing, run `./docs/open-chrome.sh` after starting the server. Diagnostics: [docs/debugging.md](docs/debugging.md).

## Controls

| Input | Action |
| --- | --- |
| Left click | Select an object |
| Shift + left click | Add to the selection |
| Ctrl + left click | Select a mesh, BRep or NURBS boundary edge |
| F10 | Show the vertices or control points of the selected parent |
| Escape | Clear the sub-selection |
| H / S | Hide the selection / show hidden objects |
| T | Toggle names |
| D | Toggle the headlight |
| G | Toggle soft shadows (Arctic) |
| O | Toggle black silhouettes |
| P | Toggle x-ray |
| B | Toggle red back faces |

Commands go in the command area below the viewport. Type a command, press Enter, then pick an option. Examples: `Point`, `Line`, `Polyline`, `Curve`, or a full command such as `Line 0,0,0 100,0,0`. `Snap On|Off`, `Lay On|Off`, `Arctic On|Off` and `Element Interactions|Attributes On|Off` switch settings.

Arctic details: [docs/ssao.md](docs/ssao.md). Command walkthrough: [docs/command-line-walkthrough.md](docs/command-line-walkthrough.md).

## Memory

Antialiasing, pixel ratio and inspection are URL options: `?msaa=4` or `?msaa=1`, `?dpr=1.5` (default cap 2, `?dpr=3` asks for more), `?inspect=1` to publish the texture and buffer bytes.

## Publish

### Scene geometry

Run from the Session directory:

```sh
cd ..
bash/view_put.sh out/scan.pb
bash/view_live.sh scene.yaml scan.pb
```

### Stage decks (slide YAML)

A slide change needs no git push. The live site reads the decks from R2.

1. Edit the YAML in `examples/stage/<deck>/`.
2. Preview locally:

   ```sh
   bash ../../wood/tools/buildslot.sh stage env CARGO_BUILD_JOBS=8 trunk serve
   ```

   Open <http://127.0.0.1:8080/?stage=decks/equilibrium/index.yaml&data=off>. The page reloads on save.

3. Push to R2 from this folder:

   ```sh
   cd session_viewer
   aws s3 sync examples/stage s3://session-viewer-data/stage/ --delete \
     --exclude '*' --include '*.yaml' --content-type application/yaml --cache-control no-cache --profile r2 \
     --endpoint-url https://0520459c6817bd96c1e25fcb49461c4e.r2.cloudflarestorage.com
   ```

4. Check <https://petrasvestartas.github.io/session/?stage=equilibrium> and reload.

`--delete` removes files in the bucket that are not in `examples/stage`, so sync from a clean checkout.

Viewer code changes (anything else in `session_viewer/`) go through git: push to `main` and the `viewer-pages` workflow deploys them.

Decks: [equilibrium](examples/stage/equilibrium/), [force-density](examples/stage/force-density/), [force-density-matrices](examples/stage/force-density-matrices/), [force-density-derivation](examples/stage/force-density-derivation/), [force-density-materialization](examples/stage/force-density-materialization/). The deck format is in [examples/stage/README.md](examples/stage/README.md). `cargo test --lib stage::` plays every step of all five.
