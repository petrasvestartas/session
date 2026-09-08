# Session Viewer

A browser CAD viewer written in Rust, compiled to WebAssembly and rendered with WebGPU. It displays meshes, BReps, NURBS, lines, points, streamed point clouds and text while retaining original source identities.

Start with the [complete reconstruction course](docs/README.md), or read the [architecture and measured results](ARCHITECTURE.md). The [coverage table](docs/coverage.md) connects requirements to source, tests and teaching chapters.

## Run

The verified toolchain is Rust/Cargo 1.97.1 and Trunk 0.21.14; use the supplied Cargo lockfile.

```sh
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14 --locked
REGEN_PROTO=0 NO_COLOR=true trunk serve
```

Open <http://localhost:8770/> in a browser exposing WebGPU. The local route loads `assets/view_local.yaml` and its local PB files. A named route such as `/view_mixed` resolves the corresponding public scene manifest. The current working session also exposes the viewer through <http://localhost:8771/view_mixed>.

Chrome 152 on the recorded Ubuntu/Intel Vulkan host has been tested. Browser and test-host configuration, including the Linux launch arguments used for these measurements, is documented in [course setup](docs/README.md). Other browsers and hardware are not implied to have passed.

## Interaction

| Input | Behavior |
| --- | --- |
| Left click | Select the original object; visible geometry turns yellow and selected surfaces gain a black silhouette. |
| Ctrl + left click | Select an original mesh, BRep or NURBS boundary edge. |
| F10 | Show and select the current parent's original vertices or control points. |
| Escape | Clear sub-selection and controls while retaining the parent. |
| T | Toggle centered white-on-black selected names; enabled by default. |
| H / S | Hide the selected object / show hidden objects. |

Source text can face the camera or remain in a fixed world plane. BRep boundaries reuse the incident face mesh's exact samples; curved constrained boundaries are refined before triangulation. Per-face normals preserve planar faces, smooth interiors and sharp creases.

## Publish and verify

The existing scripts live in the parent Session directory. They publish verified immutable geometry before updating scene metadata:

```sh
cd ..
bash/view_put.sh out/scan.pb
bash/view_live.sh scene.yaml scan.pb
```

See [maintained verification commands](tests/README.md) for browser interactions, text, CAD boundaries, hidden lines, lifecycle, publication and Rust/C++/Python parity. The native offscreen harness is a renderer test tool; the application remains a browser viewer.

The course supplies 17 complete source patches, pinned shared prerequisites and a clean reconstruction driver. The current tutorials are in `docs/`.
