# Complete viewer implementation reference

This is the preserved 43-section reconstruction of an earlier viewer. The [current cumulative course](journey.md) now covers all subjects 00–97 in 980 authored checkpoints and ends with the maintained runtime. Use that route for current features; keep this historical reference available while browser acceptance remains pending.

The final reference checkpoint records the viewer before the September 29 browser-recovery changes. Current fixes are covered by the cumulative course and its [final handoff](journey/final-handoff.md). The historical listings reconstruct their stored snapshots, but their final file list omits newer production files, so historical source parity remains pending. Each reference section has earlier cumulative build evidence; individual listings are not runnable checkpoints. This reference uses `workspace/handwritten`; the current course uses `workspace/journey`. Do not mix their files.

This material remains useful for inspecting the older implementation. Follow the current cumulative course for the complete route from the opening GPU frame to the maintained viewer.

[Architecture and time map](map.md) · [Reference workflow](how-to-learn.md) · [Feature preservation contract](journey/destination.md)

## Chapters

Work in order. Each tutorial contains the teaching and code in order, followed by compilation and behavior checks. Lettered chapter names preserve existing links; gaps in numbering are intentional.

- [00 · Load Rust in the browser](00-environment.md)
- [01 · First WebGPU frame](01-first-frame.md)
- [02 · Camera](02-camera.md)
- [03 · Object rows and identity](03-identity.md)
- [04a · Meshes on the GPU](04a-meshes.md)
- [04b · Strokes and arrows](04b-strokes.md)
- [04c · Markers](04c-markers.md)
- [04d · Point clouds](04d-clouds.md)
- [05 · Depth and visible ink](05-visibility.md) — review and experiment
- [06 · CAD face rules](06-cad-contract.md)
- [07 · Shared boundaries](07-boundaries.md) — review and experiment
- [08 · Trims, holes and periodic seams](08-trimming.md) — review and experiment
- [09 · Normals and shading](09-normals.md) — review and experiment
- [10 · Text shaping](10-text-layout.md)
- [11 · Text rendering](11-text-rendering.md)
- [12 · The viewer shell and picking](12-picking.md)
- [13 · Source controls and cloud picks](13-controls.md)
- [14 · Loading scenes](14-loading.md)
- [15 · Publication and streamed reads](15-publication.md)
- [16 · Resource accounting and release](16-accounting.md)
- [17 · Source faces, text objects and one silhouette](17-source-presentation.md)
- [18 · Finite-triangle visibility](18-finite-visibility.md)
- [18a · Instancing](18a-instancing.md)
- [18b · Clipping planes and section caps](18b-clipping.md)
- [19 · Sheets: batched drawings with lazy metadata](19-sheets.md)
- [20 · The document and its rows](20-history.md)
- [21 · Direct editing](21-editing.md)
- [22 · The egui layer](22-runtime-helpers.md)
- [23 · The command line: one verb per file](23-geometry-commands.md)
- [23a · Tools that ask for points](23a-tools.md)
- [23b · Shapes](23b-shapes.md)
- [23c · Surfacing](23c-surfacing.md)
- [23d · Annotate and measure](23d-annotate-measure.md)
- [24 · Snapping](24-placed-controls.md)
- [25 · Draw a solid, readable gumball](25-gumball.md)
- [26 · The nested session panel](26-nested-panel.md)
- [30 · The layers panel and the layer tree](30-layer-tree.md)
- [31 · Split curves and faces](31-splitting.md)
- [32 · Ambient occlusion](32-colors-lighting.md)
- [33 · GTAO, Arctic and Outline](33-contact-shadows.md)
- [35 · Element features](35-attributes.md)
- [36 · Translucent faces and Opacity](36-translucent-faces.md)
- [37 · Self-test: the finished viewer](37-command-dock.md)
