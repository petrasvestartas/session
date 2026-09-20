# current-10 · Split curves and faces while keeping the shell joined
<!-- locator: off -->

Splitting a curve keeps its cutter, and splitting a face keeps both regions in the joined shell.

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after step 14; steps 1–13 fail and build again at step 14.

<!-- step-status: end -->

## Step 1 · src/app/command.rs
Add Split, its input checks and contextual syntax. Confirm cutter selection with Enter only while a split is pending.
<!-- file: current-10 session_viewer/src/app/command.rs type -->

## Step 2 · src/app/input.rs
Route Enter and Escape to the pending split before ordinary scene shortcuts. Ctrl component picking must still identify the intended face.
<!-- file: current-10 session_viewer/src/app/input.rs type -->

## Step 3 · src/app/inspection.rs
Expose the new selection and resource state to the browser inspection data. Report retained resources as well as visible objects so hidden allocations are counted.
<!-- file: current-10 session_viewer/src/app/inspection.rs type -->

## Step 4 · src/app/mod.rs
Declare the new application modules so their files join the crate. A source file is not compiled until a module declaration names it.
<!-- file: current-10 session_viewer/src/app/mod.rs type -->

## Step 5 · src/app/modeling.rs
Connect curve creation and replacement to the split workflow. Preserve source placements when new curve pieces are inserted.
<!-- file: current-10 session_viewer/src/app/modeling.rs type -->

## Step 6 · src/app/scene.rs
Expose the owning document identity to split operations. A GPU row alone is not a persistent source key.
<!-- file: current-10 session_viewer/src/app/scene.rs type -->

## Step 7 · src/app/session_io.rs
Preserve split results and visible curve pens when reopening a session. Restore display widths after decoding the retained source.
<!-- file: current-10 session_viewer/src/app/session_io.rs type -->

## Step 8 · src/app/splitting.rs
Convert cutters, split source curves or trimmed faces, and replace the result in one transaction. Reject off-surface cutters and ambiguous trim overlaps before changing the source.
<!-- file: current-10 session_viewer/src/app/splitting.rs type -->

## Step 9 · src/app/ui.rs
Expose Split in the command and tool interface. Leave the target selected while subsequent picks collect cutters.
<!-- file: current-10 session_viewer/src/app/ui.rs type -->

## Step 10 · src/shaders/ribbon.wgsl
Keep split curves visible with the same stroke filtering as the original curve. Joined segments must use consistent width and endpoint coverage.
<!-- file: current-10 session_viewer/src/shaders/ribbon.wgsl type -->

## Step 11 · src/state.rs
Store pending split state and route object picks to cutter collection. Cancel the operation if a scene replacement invalidates its source identities.
<!-- file: current-10 session_viewer/src/state.rs type -->

## Step 12 · src/state/edit.rs
Dispatch Split and rebuild source selection after history changes. Undo must restore the target and its original component addresses.
<!-- file: current-10 session_viewer/src/state/edit.rs type -->

## Step 13 · src/state/panel.rs
Refresh the hierarchy when a split changes object membership. Do not retain row ranges from before the split.
<!-- file: current-10 session_viewer/src/state/panel.rs type -->

## Step 14 · src/state/splitting.rs
Retain the target, selected face and cutter rows until confirmation. Escape clears this temporary state without committing a document edit.
<!-- file: current-10 session_viewer/src/state/splitting.rs type -->

## Check

<!-- checkpoint: current-10 -->

Expected: confirming the cutters creates curve pieces or joined face regions, and the status reports the split result.

![Full viewer result for current 10](screenshots/extensions-split-face.png)

If it fails:

- The wrong face splits: a multi-face shell has no explicit selected face.
- The cutter is rejected: it does not lie on the selected surface within tolerance.

## What changed

<!-- tree: current-10 session_viewer/src -->

Data flow: target and cutters → trimmed-region split → source transaction → refreshed selection.
Every file at this point: [source at checkpoint current-10](../lessons/current-10/index.md).

## Next

[Continue with current-11](current-11.md).

## Expected viewer result

The shell has been divided into two face regions by an on-surface line. The cutter remains and the selected half is highlighted. The shell stays joined, and the editable session stores the updated source geometry. Use Undo to restore the original or Save to keep this result. See the [phone layout](screenshots/extensions-split-phone.png).

[![Full viewer result for current 10](screenshots/extensions-split-face.png)](screenshots/extensions-split-face.png)
