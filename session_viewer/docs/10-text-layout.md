# 10 · Text shaping
<!-- locator: off -->

The text specimen page compares five shaped text sizes against browser text.

![Shape once, place per frame, raster per device scale, then a plate pass and a glyph pass.](illustrations/text-pipeline.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

Download each file to the path shown.
<!-- supplied: 10 -->
## Step 1 · assets/text/OFL.txt

Download this file from its link to the path shown.
<!-- file: 10 session_viewer/assets/text/OFL.txt copy -->
## Step 2 · assets/text/README.md

Download this file from its link to the path shown.
<!-- file: 10 session_viewer/assets/text/README.md copy -->
## Step 3 · src/engine/performance.rs

Performance counters separate frame timing from resource capacity. A retained allocation is not the same as its live payload.
<!-- file: 10 session_viewer/src/engine/performance.rs type -->
## Step 4 · src/engine/text.rs

Text layout retains shaped glyph positions for rendering. Use the same font bytes and size when comparing with browser text.
<!-- file: 10 session_viewer/src/engine/text.rs type lines=1-45 -->
<!-- file: 10 session_viewer/src/engine/text.rs type lines=46-73 -->
<!-- file: 10 session_viewer/src/engine/text.rs type lines=74-137 -->
<!-- file: 10 session_viewer/src/engine/text.rs type lines=138-204 -->
<!-- file: 10 session_viewer/src/engine/text.rs type lines=205-236 -->
<!-- file: 10 session_viewer/src/engine/text.rs type lines=237-301 -->
<!-- file: 10 session_viewer/src/engine/text.rs type lines=302-339 -->
Download this part from its link to the path shown.
<!-- file: 10 session_viewer/src/engine/text.rs copy lines=340-455 -->
## Step 5 · src/engine/mod.rs

The engine module exposes the rendering implementation. A missing module declaration leaves its file outside the build.
<!-- file: 10 session_viewer/src/engine/mod.rs type -->
<!-- check: 10 -->
## Step 6 · src/text_layout.rs

Download this file from its link to the path shown.
<!-- file: 10 session_viewer/src/text_layout.rs copy -->
## Step 7 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 10 session_viewer/src/lib.rs type -->
## Step 8 · assets/text-layout.html

Download this file from its link to the path shown.
<!-- file: 10 session_viewer/assets/text-layout.html copy -->
## Step 9 · index.html

Download this file from its link to the path shown.
<!-- file: 10 session_viewer/index.html copy -->
## Check

<!-- checkpoint: 10 -->

Expected: The text specimen page compares five shaped text sizes against browser text; status: **PASS**.

![Checkpoint 10: the reference page shapes one string at five sizes; the browser row behind each specimen has the same width, and the report lists every glyph with its cluster, advance and baseline.](screenshots/10-text-layout.png)

If it fails:

- The specimen widths differ: font bytes, size or kerning settings differ.
- Glyph order is wrong: character order replaces the shaped glyph sequence.

## What changed

<!-- tree: 10 session_viewer/src/engine -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 10](../lessons/10/index.md).

## Next

[11 · Text rendering](11-text-rendering.md): placement, raster scale, coverage atlas and the black plates.

## Expected viewer result

Checkpoint 10: the canvas itself is unchanged.

[![Full viewer result for 10 text layout](screenshots/10.png)](screenshots/10.png)
