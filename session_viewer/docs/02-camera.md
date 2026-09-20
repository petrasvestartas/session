# 02 · Camera
<!-- locator: off -->

The triangle orbits, pans and zooms toward the cursor.

![Orbit turns the orientation about the target, pan slides the target across the camera's own plane, and the wheel scales the distance; the view-projection is rebuilt from those three every frame.](illustrations/camera-basis.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/camera.rs

The camera owns orbit, pan, zoom and projection in the same file used by the finished viewer. Subtract the world anchor before converting the matrix to f32.
<!-- file: 02 session_viewer/src/camera.rs type lines=1-48 -->
<!-- file: 02 session_viewer/src/camera.rs type lines=49-110 -->
<!-- file: 02 session_viewer/src/camera.rs type lines=111-146 -->
<!-- file: 02 session_viewer/src/camera.rs type lines=147-217 -->
<!-- file: 02 session_viewer/src/camera.rs type lines=218-291 -->
<!-- file: 02 session_viewer/src/camera.rs type lines=292-319 -->
<!-- file: 02 session_viewer/src/camera.rs type lines=320-377 -->
<!-- file: 02 session_viewer/src/camera.rs type lines=378-442 -->
Download this part from its link to the path shown.
<!-- file: 02 session_viewer/src/camera.rs copy lines=443-535 -->
<!-- check: 02 -->
## Step 2 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 02 session_viewer/src/lib.rs type -->
## Step 3 · index.html

Download this file from its link to the path shown.
<!-- file: 02 session_viewer/index.html copy -->
## Check

<!-- checkpoint: 02 -->

Expected: The triangle orbits, pans and zooms toward the cursor; status: **Checkpoint 02 · 1 objects**.

![Checkpoint 02: the same triangle seen from the production camera; drag to orbit, Shift-drag to pan, wheel to zoom at the cursor.](screenshots/02.png)

If it fails:

- Dragging moves twice as far on a high-DPI screen: the cursor is scaled twice.
- A distant model jitters: coordinates become f32 before the anchor is subtracted.
- The triangle disappears: reversed depth needs a zero clear and a Greater comparison.

## What changed

<!-- tree: 02 session_viewer/src -->

Data flow: gesture → camera → anchored matrix → uniform → vertex. Every file at this point: [source at checkpoint 02](../lessons/02/index.md).

## Next

[03 · Object rows and identity](03-identity.md): a storage buffer of per-object rows, and why a GPU row is not a source identity.

## Expected viewer result

Checkpoint 02: the same triangle seen from the production camera; drag to orbit, Shift-drag to pan, wheel to zoom at the cursor.

[![Full viewer result for 02 camera](screenshots/02.png)](screenshots/02.png)
