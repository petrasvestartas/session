# 11 · Text rendering
<!-- locator: off -->

Screen-sized nameplates and a foreshortened scene label appear above the model.

![Five placements of one shaped line, and the same label rasterized once per device scale.](illustrations/text-placement.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

Download each file to the path shown.
<!-- supplied: 11 -->
## Step 1 · src/engine/gpu/text_plate.rs

Text plates draw a backing around shaped labels. Include glyph overhang when measuring the padding.
<!-- file: 11 session_viewer/src/engine/gpu/text_plate.rs type lines=1-76 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plate.rs type lines=77-104 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plate.rs type lines=105-152 -->
## Step 2 · src/shaders/text_plate.wgsl

Text plates draw rounded backing shapes behind labels. Keep their screen-space dimensions consistent with the glyph scale.
<!-- file: 11 session_viewer/src/shaders/text_plate.wgsl type -->
## Step 3 · src/engine/gpu/text_plane.rs

Plane text projects labels through their scene placement. Its depth must participate in ordinary scene occlusion.
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=1-33 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=34-83 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=84-150 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=151-232 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=233-279 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=280-355 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=356-441 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=442-512 -->
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs type lines=513-537 -->
## Step 4 · src/shaders/text_plane.wgsl

Plane labels project shaped glyphs onto their scene plane. Preserve projected depth so solids can hide the label.
<!-- file: 11 session_viewer/src/shaders/text_plane.wgsl type -->
## Step 5 · src/engine/gpu/text_plane.rs

Download this file from its link to the path shown.
<!-- file: 11 session_viewer/src/engine/gpu/text_plane.rs copy lines=538-637 -->
## Step 6 · src/engine/gpu/text.rs

The GPU text owner coordinates glyphs and label backgrounds. Apply the framebuffer scale once so glyphs remain sharp.
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=1-48 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=49-106 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=107-129 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=130-212 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=213-272 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=273-343 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=344-377 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=378-440 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=441-512 -->
<!-- file: 11 session_viewer/src/engine/gpu/text.rs type lines=513-563 -->
Download this part from its link to the path shown.
<!-- file: 11 session_viewer/src/engine/gpu/text.rs copy lines=564-978 -->
<!-- check: 11 -->
## Step 7 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 11 session_viewer/src/engine/gpu/mod.rs type -->
## Step 8 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 11 session_viewer/src/lib.rs type -->
## Step 9 · src/text_layout.rs

Remove this file; its replacement is now part of the rendering modules.
<!-- file: 11 session_viewer/src/text_layout.rs -->
## Step 10 · assets/text-layout.html

Remove this file; its replacement is now part of the rendering modules.
<!-- file: 11 session_viewer/assets/text-layout.html -->
## Step 11 · assets/text-quality.html

Download this file from its link to the path shown.
<!-- file: 11 session_viewer/assets/text-quality.html copy -->
## Step 12 · index.html

Download this file from its link to the path shown.
<!-- file: 11 session_viewer/index.html copy -->
## Check

<!-- checkpoint: 11 -->

Expected: Screen-sized nameplates and a foreshortened scene label appear above the model; status: **3 objects**.

![Checkpoint 11: two nameplates above the sphere, one rounded, and a fixed-plane label foreshortened on its own plane.](screenshots/11.png)

If it fails:

- Letters blur at one zoom level: the text frame scale disagrees with the framebuffer.
- The last glyph clips: plate padding excludes the glyph overhang.

## What changed

<!-- tree: 11 session_viewer/src/engine/gpu -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 11](../lessons/11/index.md).

## Next

[12 · maintained viewer shell and picking](12-picking.md): the winit application, `State`, and GPU picking with an integer ID pass.

## Expected viewer result

Checkpoint 11: two nameplates above the sphere, one rounded, and a fixed-plane label foreshortened on its own plane.

[![Full viewer result for 11 text rendering](screenshots/11.png)](screenshots/11.png)
