# Editing lesson evidence

Start with [the implementation lessons](../extend-implementation.md). Each starts from frozen checkpoint 21 and includes exact code edits. Their patches are alternatives against that starting point; do not blindly stack them. For their combined implementation, follow [seven sequential checkpoints](../extend-integrated-tutorial.md). Every checkpoint compiles and the final visible edits reproduce all 114 maintained runtime/build files.

## Five browser rounds

[rounds.json](rounds.json) records successful runs at 1200×800, 960×720, perspective projection, 2× DPI and a live resize to 900×700. Every round checks:

- Nested-group and graph-edge selection; hide and show both endpoints.
- Point/line/polyline creation, line trim/extend, explosion and undo.
- Gumball movement, axis scaling, rotation and uniform scaling; constant size after zoom.
- Control dragging, undo and cancellation.
- Five select/deselect cycles: fixed mesh capacity; temporary tile allocation returns to zero.

The native tests also cover NURBS operations and placed-control coordinate conversion. Browser screenshots show the small [nested fixture](nested.pb); they do not establish performance on large production models.

## Reproduce

Use the existing Playwright installation and a running viewer:

```sh
export VIEWER_URL=http://localhost:8770/
node tests/editing-extensions.cjs
node docs/extensions/capture.cjs
```

Both scripts launch headed Chrome and close their own browser. `CHROME_BIN` selects its executable; `VIEWER_CHROME_ARGS` supplies a JSON array of launch arguments. Match the GPU configuration of your working desktop launcher. [screenshots.json](screenshots.json) records the configuration used here, including the NVIDIA environment variables. A default automation profile on this hybrid-GPU machine had no adapter or failed texture import; matching the desktop configuration produced real frames without console errors.

## Visual changes

The archive served as a reference for cylindrical shafts and conical arrowheads; the new widget uses unlit, flat colors. The maintained viewer now builds its own fixed triangle mesh: 96 CSS-pixel arms, 4.4-pixel shaft diameter, 14-pixel cone length, 11-pixel cone base diameter and 3.6-pixel rotation tubes. Its hit tests retain the current handle layout.

One mesh and uniform occupy 371,616 GPU bytes. A selected widget renders into a cropped tile at twice the screen resolution, with four MSAA samples. Color, depth and resolve attachments occupy `tile_width × tile_height × 36` bytes, capped at 1024×1024 (36 MiB). Deselect and scene clear destroy the tile; resizing replaces it. Source geometry and undo history are separate costs.

The interface uses egui 0.34.3, egui-winit and egui-wgpu, matching the archive framework. Its customization uses black text and black/white text selection, with white window, header and control backgrounds. Command input is capped at 2048 characters and history at eight entries. Requested texture frees are processed between frames; remaining font textures are freed on drop. egui renderer-private buffer capacities are excluded from the viewer GPU-byte inspection.

The small [gumball picture](../screenshots/extensions-gumball.png) is a direct browser capture of a 280×280 region; [the full frame](../screenshots/extensions-gumball-overview.png) provides context. All pictures are actual viewer output.
