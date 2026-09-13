# Editing lesson evidence

Start with [the implementation lessons](../extend-implementation.md). Each starts from frozen checkpoint 21 and includes exact code edits. Their patches are alternatives against that starting point; do not blindly stack them. For their combined implementation, follow [nine sequential checkpoints](../extend-integrated-tutorial.md). Every checkpoint compiles and the final visible edits reproduce all 118 maintained runtime/build files.

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

## Tutorial result images

Every implementation page ends with a linked full-viewer screenshot. `docs/tutorial-results.json` selects the image and describes the visible result; captions identify maintained-viewer references when a frozen checkpoint has older controls. Change that manifest, then run `python3 docs/extensions.py --write` and `python3 docs/tutorial_results.py --write`. `python3 docs/check_site.py` checks both the source endings and the built pages.

`19-sheets-overview.png` is a 1200×800 browser capture of `?scene=view_sheets&inspect=1` after **5** and **F**, with two sheets and 178,647 ribbon segments loaded. `current-empty.png` uses the same viewport with the local manifest replaced by `name: Empty scene` and `items: []`. Both use the maintained viewer and the Chrome/GPU configuration described above.

`21-editing-overview.png` uses the frozen checkpoint 21 runtime built from `target/docs/course-cache/snapshots/21`. Load the nested fixture, press **5**, **F**, select the placed polyline, then **7**, **F**, zoom out three wheel steps, **T** to hide nameplates, **L** and **:**. Type `move 10 0 0` without submitting. The 1200×800 capture includes the original DOM layers and command controls.

## Docked workspace, touch and session files

[Checkpoint 8](../current-8.md) adds the bottom command area, right Layers panel,
left toolbar, source-subobject gumball editing and Save/Open. Its code block explains
how to extend the `TOOLBAR` table. The [desktop capture](../screenshots/extensions-workspace-desktop.png)
and [phone capture](../screenshots/extensions-workspace-phone.png) show the complete workspace.

`tests/docked-workspace.cjs` checks desktop DPR 1/2, a 390×844 phone viewport,
touch commit/cancel, second-finger cancellation and an actual file download/reopen.
`tests/source-editing.cjs` checks mesh, NURBS and BRep source faces/edges plus touch
editing of a source mesh vertex. These use Chrome touch emulation, not physical phones.
The `.session` format stores retained source documents, placements, hidden identities
and annotations. Live undo remains available after saving; opening starts a new history.
Partially streamed scenes and BRep edits needing trim reconstruction are not supported.

[Checkpoint 9](../current-9.md) removes duplicate layer summaries, adds child visibility/selection locks/colors, and patches cached BRep samples during dragging. The `tests/live-shell-editing.cjs` browser regression checks that scene rows are not rebuilt during a drag or on release; `tests/layer-workspace.cjs` checks layer state through Save/Open.
