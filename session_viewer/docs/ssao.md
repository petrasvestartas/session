# Soft ambient lighting and contact shadows

Type `SSAO`, press **Enter**, then click **On** or **Off** in the command row. Options appear after the command has been accepted. `SSAO On` and `SSAO Off` also run directly. `Arctic` is an alias, and **G** toggles the same effect when the viewport has keyboard focus. SSAO starts off.

On/Off changes the lighting without changing the camera. If you have already adjusted the view while a scene is loading, completion of the load preserves that view too. **Fit** remains available when you want to reframe the scene.

## What changed

- AO is calculated at display resolution up to 1920 pixels on its longest side, replacing the earlier half-resolution, 960-pixel limit. Larger viewports use depth-aware reconstruction.
- Two half-float textures preserve subtle shade differences. Horizontal and vertical depth-aware smoothing remove sampling grain while keeping separate faces apart.
- With MSAA enabled, the final blend uses each sample's depth, avoiding a hard single-sample outline around a softly shaded object.
- Contact radii follow individual object sizes. Small cones and tori retain compact shadows when displayed beside a building.
- Surface normals use the rendered triangle's depth slope. Thin plate edges no longer borrow a normal from unrelated neighboring geometry.
- A virtual ground receiver adds soft floor contacts. It follows the lowest visible solid; hidden objects, sheets and point clouds do not lower it. Raised geometry fades out of the contact band.

Authored colors remain under neutral sky/ground lighting. This effect approximates indirect light from the visible depth image; it cannot include hidden or off-screen occluders or produce directional sun shadows.

## Check it in `view_mixed`

Open `/view_mixed`, run `Layers On`, select the floor-model group, and run **Fit**. Clear the selection with **Escape**, hide the panel with `Layers Off`, then compare SSAO On/Off at the column bases. Repeat with the plates/contact group and the solids/BReps group to inspect the plate creases, cone and torus. The camera should remain identical through the toggles.

![Ground contacts under the floor model in view_mixed](screenshots/ssao-floor.png)

![Thin plate contacts in the same view_mixed scene](screenshots/ssao-plates.png)

These captures use the full public scene at 1440×1000 in Chrome/WebGPU.

## Cost and limits

The pass uses 64 local and 64 broader surface samples, or 16 directions with eight steps for ground contacts. Two separable thirteen-tap filters reuse the same two textures. Camera or geometry changes recompute occlusion; a stationary view reuses it.

For a 1920×1080 render target, AO adds **8,294,400 texture bytes** and a **144-byte uniform**. The 1920-pixel limit bounds the AO allocation at 14,745,600 texture bytes for a square target. These numbers exclude the viewer's existing color, depth, geometry and UI resources. Switching SSAO off releases its textures and uniform.

Full-resolution sampling improves contact detail but costs more during navigation than the earlier half-resolution pass. Low-end and integrated GPUs have not been benchmarked. Turn SSAO off to remove its cost; the existing `?dpr=1` option can also reduce the whole viewer's rendering resolution on a high-DPI display.

The browser checks in `tests/ambient-details.cjs` and `tests/ambient-floor.cjs` exercise the full public scene, shading pixels, command-row On/Off clicks and camera preservation during loading. `tests/ambient-lighting.cjs` checks resource release and can run with `AMBIENT_SPIN=1` to measure navigation rather than cached redraws. Native GPU checks cover perspective and orthographic views at 1× and 4× MSAA.

See the [command-line walkthrough](command-line-walkthrough.md) for completion, drawing, snapping and layer controls.
