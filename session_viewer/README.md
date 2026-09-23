# Session Viewer

A browser CAD viewer written in Rust, compiled to WebAssembly and rendered with WebGPU. It displays meshes, BReps, NURBS, lines, points, streamed point clouds and text while retaining original source identities.

Start with the [course](docs/README.md), which rebuilds the viewer from an empty crate, or read the [architecture reference](ARCHITECTURE.md).

## Run

The verified toolchain is Rust/Cargo 1.97.1 and Trunk 0.21.14. Run these commands from the crate folder; Cargo writes its lockfile.

```sh
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14
cargo generate-lockfile
cargo check --lib
trunk serve
```

Open <http://localhost:8770/> in a browser exposing WebGPU. The local route loads `assets/view_local.yaml` and its local PB files. A named route such as `/view_mixed` resolves the corresponding public scene manifest. The current working session also exposes the viewer through <http://localhost:8771/view_mixed>.

Chrome on Linux with Vulkan is the tested browser. Other browsers and hardware are not implied to have passed.

## Interaction

| Input | Behavior |
| --- | --- |
| Left click | Select the original object; visible geometry turns yellow. |
| Shift + left click | Add objects to the selection; one gumball transforms the selected set. |
| Ctrl + left click | Select an original mesh, BRep or NURBS boundary edge. |
| F10 | Show and select the current parent's original vertices or control points. |
| Escape | Clear sub-selection and controls while retaining the parent. |
| T | Toggle centered white-on-black selected names; enabled by default. |
| H / S | Hide the selected object / show hidden objects. |
| D | Toggle the headlight (off by default: flat colours; `?lit=1`). |
| G | Toggle soft ambient lighting and contact shadows (off by default); uses two R16Float textures at display resolution, capped at 1920 pixels on the longest side. |
| O | Toggle black surface silhouettes (off by default; two extra mask passes over the solid geometry). |
| P | Toggle x-ray: every mesh, NURBS and BRep face disappears, only edges, points and text remain. |
| B | Toggle red back faces (off by default; `?backface=1`). |

Source text can face the camera or remain in a fixed world plane. BRep boundaries reuse the incident face mesh's exact samples; curved constrained boundaries are refined before triangulation. Per-face normals preserve planar faces, smooth interiors and sharp creases.

The white command area sits below the viewport, with full-width dividers and only its output window above the input. It accepts typing from the first page load and completes command names inline with the suggested suffix selected. Enter accepts the first matching command, then its default or arrow-selected option; Escape cancels. The completion list opens directly upward from the input and stays within the window. History and active drawing prompts wrap at narrow widths; long input scrolls inside its field. Matching commands appear first, followed by all remaining commands. Browse with Up/Down, the mouse wheel, or the scrollbar; Tab inserts a completion. Options appear only after accepting a command with Enter, Tab or Space. Type `Lay`, press Enter, then choose inline `On` or `Off`, or enter `Layers On` directly. `Arctic On` and `Arctic Off` control the same lighting as G; Rotate offers clickable axis options. The documentation triangle sits in the bottom-right corner. Layers start hidden. Each panel has a collapse control, and the layer tree highlights selected objects. Selecting or locking a group applies to its descendants.

`Element Features On` and `Element Features Off` show or hide element outlines, contacts and joints. Use Up/Down to cycle command options, then Enter to accept; Left/Right move the text caret. Coincident contact endpoints appear as dots.

`Object`, `Edge`, `Face`, and `Controls` choose selection tools. Geometry and editing commands include `Point`, `Line`, `Polyline`, `Curve`, `Move`, `Rotate`, `Scale`, `Split`, `Trim`, `Extend`, `Explode`, `Undo`, `Redo`, `Save`, and `Open`. Enter `Point`, `Line`, `Polyline`, or `Curve`, then click the canvas or type coordinates at each prompt. Two coordinates use the construction plane facing the camera; three are world coordinates. `@dx,dy,dz` is relative to the previous point. Enter finishes a polyline or curve; Escape discards a draft. Endpoints, vertices and midpoints snap within 12 screen pixels by default; `Snap Off` and `Snap On` control snapping, including during a draft. A blue marker names the active snap and a line previews the next span. Each completed draft is one undo step. Complete commands also work: `Line 0,0,0 100,0,0` and `Move 10,0,0`.

Standalone NURBS surfaces retain a fixed UV display grid and four natural domain boundaries, excluding closed directions. Control and edge previews update those samples without detecting mesh edges or rebuilding connectivity. Source line widths are screen-space weights; sheet drawing widths retain their physical pen units. The viewport is white with Arctic disabled and very light grey with Arctic enabled. Default kernel colors remain very light grey.

Arctic preserves authored colors under neutral hemisphere lighting, with soft contact occlusion and a virtual ground receiver. Contact radii follow each object's size, so a small cone or torus does not inherit a building's shadow radius. Faceted normals come directly from the existing geometry buffers. Horizon sampling, bilateral filtering, motion reprojection and depth-aware reconstruction produce a cached shading image, with sample-aware blending before MSAA resolve. Quality and resolution stay the same while orbiting, panning and zooming. The ground follows visible solids only; hidden geometry, sheets and point clouds cannot lower it. Switching On/Off preserves the camera, including while the scene finishes loading.

Occlusion is cached while camera and geometry stay unchanged. At 1920×1080, DPR 1, AO textures reserve 7.91 MB. AO buffers add 348 bytes at 1× MSAA or 8.36 MB at 4×; switching off releases them. Arctic does not require the projected-triangle table. Moving the camera recomputes the shading, so cached frame rates do not establish navigation speed on slower GPUs. This screen-space effect cannot include hidden or off-screen occluders. See [Arctic quality and controls](docs/ssao.md) and the [command-line walkthrough](docs/command-line-walkthrough.md).

## Memory

Video memory goes to per-pixel attachments, not to geometry. Antialiasing is 4x only for solid geometry, inside the adapter's pixel budget and below two physical pixels per CSS pixel; `?msaa=4` or `?msaa=1` forces it. Picking renders a window around the cursor into an attachment of that size, and the finite-visibility pool is sized for the scene and grows on demand. `?dpr=1.5` caps the device pixel ratio the canvas is rendered at, for people who prefer memory over crispness; nothing caps it by default. Every attachment the viewer replaces is destroyed on the spot: in the browser a dropped wgpu texture is otherwise held until the JavaScript garbage collector notices, and a window drag remakes the frame's attachments as it goes - at most once every 100 ms, the last picture stretched in between. If the browser loses the WebGPU device anyway, the page reloads itself once at device scale 1 without antialiasing, the status line names the browser's reason, and the address is left clean so the next reload starts at full resolution. `?inspect=1` publishes the exact texture and buffer bytes the viewer owns.

## Publish and verify

The existing scripts live in the parent Session directory. They publish verified immutable geometry before updating scene metadata:

```sh
cd ..
bash/view_put.sh out/scan.pb
bash/view_live.sh scene.yaml scan.pb
```

Every checkpoint of the course is a runnable crate under `docs/lessons/<id>/`; the lessons are in `docs/`.
