# Session Viewer

A browser CAD viewer written in Rust, compiled to WebAssembly and rendered with WebGPU. It displays meshes, BReps, NURBS, lines, points, streamed point clouds and text while retaining original source identities.

Start with the [course](docs/README.md), which rebuilds the viewer from an empty crate, or read the [architecture reference](ARCHITECTURE.md).

## Run

The verified toolchain is Rust/Cargo 1.97.1 and Trunk 0.21.14; use the supplied Cargo lockfile.

```sh
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14 --locked
REGEN_PROTO=0 NO_COLOR=true trunk serve
```

Open <http://localhost:8770/> in a browser exposing WebGPU. The local route loads `assets/view_local.yaml` and its local PB files. A named route such as `/view_mixed` resolves the corresponding public scene manifest. The current working session also exposes the viewer through <http://localhost:8771/view_mixed>.

Chrome on Linux with Vulkan is the tested browser. Other browsers and hardware are not implied to have passed.

## Interaction

| Input | Behavior |
| --- | --- |
| Left click | Select the original object; visible geometry turns yellow. |
| Ctrl + left click | Select an original mesh, BRep or NURBS boundary edge. |
| F10 | Show and select the current parent's original vertices or control points. |
| Escape | Clear sub-selection and controls while retaining the parent. |
| T | Toggle centered white-on-black selected names; enabled by default. |
| H / S | Hide the selected object / show hidden objects. |
| O | Toggle black surface silhouettes (off by default; two extra mask passes over the solid geometry). |

Source text can face the camera or remain in a fixed world plane. BRep boundaries reuse the incident face mesh's exact samples; curved constrained boundaries are refined before triangulation. Per-face normals preserve planar faces, smooth interiors and sharp creases.

## Memory

Video memory goes to per-pixel attachments, not to geometry. Antialiasing is 4x only for solid geometry, inside the adapter's pixel budget and below two physical pixels per CSS pixel; `?msaa=4` or `?msaa=1` forces it. Picking renders a window around the cursor into an attachment of that size, and the finite-visibility pool is sized for the scene and grows on demand. `?dpr=1.5` caps the device pixel ratio the canvas is rendered at, for people who prefer memory over crispness; nothing caps it by default. If the browser loses the WebGPU device because video memory ran out, the page reloads itself once at device scale 1 without antialiasing and says so in the status line. `?inspect=1` publishes the exact texture and buffer bytes the viewer owns.

## Publish and verify

The existing scripts live in the parent Session directory. They publish verified immutable geometry before updating scene metadata:

```sh
cd ..
bash/view_put.sh out/scan.pb
bash/view_live.sh scene.yaml scan.pb
```

The course supplies the complete checkpoint patches, pinned shared prerequisites and a reconstruction driver under `docs/reconstruction/`; the lessons are in `docs/`.
