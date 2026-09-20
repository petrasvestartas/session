# 20 · The document: undo, redo and save
<!-- locator: off -->

The sheet scene stays visible while document edits gain undo, redo and history-free saving.

![Edits group into transactions and a removal leaves a tombstone to restore from; the cursor moves back and forward through them, and a save purges the whole buffer because history never crosses pb or JSON.](illustrations/history.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · session_rust/src/history.rs

Read this source file from its link; the checkpoint already contains it.
<!-- listing: 20 session_rust/src/history.rs -->
## Step 2 · session_rust/src/session.rs

Read this source file from its link; the checkpoint already contains it.
<!-- listing: 20 session_rust/src/session.rs -->
<!-- check: 20 -->
## Check

<!-- checkpoint: 20 -->

Expected: The sheet scene stays visible while document edits gain undo, redo and history-free saving; status: **the status clears when loading finishes**.

[![Full viewer result for 20 history](screenshots/19-sheets-overview.png)](screenshots/19-sheets-overview.png)

If it fails:

- Undo fails to restore a removed object: its geometry and tree position are not both recorded.
- Saved history reappears: serialization includes the undo stack.

## What changed

<!-- tree: 20 session_rust/src -->

Data flow: edit → document transaction → undo cursor → restored source. Every file at this point: [source at checkpoint 20](../lessons/20/index.md).

## Next

[21 · Editing](21-editing.md): add the gumball, commands and layer controls.

## Expected viewer result

The viewer appearance is unchanged from checkpoint 19: the same two sheets still draw in top view. This lesson adds kernel history, with no new viewer controls; the undo, redo and save checks above verify the new behavior. The image is a maintained-viewer reference of the same sheet scene.

[![Full viewer result for 20 history](screenshots/19-sheets-overview.png)](screenshots/19-sheets-overview.png)
