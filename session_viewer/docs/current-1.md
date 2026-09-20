# current-1 · Refresh diagnostics and resource checks
<!-- locator: off -->

Continuing from checkpoint 21, the grid still orbits, pans and zooms while diagnostics retain the first GPU failure.

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/engine/gpu/device.rs
Two edits: keep the first device error when later submissions fail; test that the original message survives. Overwriting it hides the failure that caused the rest.
<!-- file: current-1 session_viewer/src/engine/gpu/device.rs type -->

## Step 2 · src/engine/gpu/surface_outline.rs
Three edits: measure fractional silhouette coverage with selection disabled and enabled; check the accumulated coverage; require the same outline width in both states. Counting only fully black pixels misses antialiased edges.
<!-- file: current-1 session_viewer/src/engine/gpu/surface_outline.rs type -->

## Step 3 · src/engine/gpu/upload.rs
Replace the upload vector with an empty vector to release its allocation. Clearing its length alone leaves the capacity allocated.
<!-- file: current-1 session_viewer/src/engine/gpu/upload.rs type -->

## Check

<!-- checkpoint: current-1 -->

Expected: the grid and world axes remain visible, orbit still works, and the status area has no error message.

![Full viewer result for current 1](screenshots/current-empty.png)

If it fails:

- Later errors hide the allocation failure: the stored failure is overwritten.
- Memory stays allocated after replacement: the upload vector is cleared but its capacity is retained.

## What changed

<!-- tree: current-1 session_viewer/src -->

Data flow: GPU failure or scene replacement → diagnostics and released upload storage.
Every file at this point: [source at checkpoint current-1](../lessons/current-1/index.md).

## Next

[Continue with current-2](current-2.md).

## Expected viewer result

With an empty scene, the viewer shows the grid and world axes. Orbit, pan and zoom should work. Runtime diagnostics add no visible editing control. Captured in the maintained viewer with an empty manifest and no object selected.

[![Full viewer result for current 1](screenshots/current-empty.png)](screenshots/current-empty.png)
