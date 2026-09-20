# 16 · Resource accounting
<!-- locator: off -->

The scene stays visible while the inspection snapshot reports retained source memory.

![Scene owns documents through Rc; the cache keeps Weak identities and a payload figure, reuses it while the pointers match, walks once when a document is replaced, and never keeps a dropped document alive.](illustrations/source-cache.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

Download each file to the path shown.
<!-- supplied: 16 -->
## Step 1 · Cargo.toml

Download this file from its link to the path shown.
<!-- file: 16 session_viewer/Cargo.toml copy -->
## Step 2 · src/app/inspection/source_memory.rs

The source cache counts retained payload once per shared identity. Weak references prevent accounting from keeping discarded documents alive.
<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs type lines=1-53 -->
<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs type lines=54-102 -->
<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs type lines=103-149 -->
Download this part from its link to the path shown.
<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs copy lines=150-423 -->
Download this part from its link to the path shown.
<!-- file: 16 session_viewer/src/app/inspection/source_memory.rs copy lines=424-556 -->
<!-- check: 16 -->
## Step 3 · src/app/inspection.rs

Inspection reports retained resources and source information. Count shared documents once instead of once per placement.
<!-- file: 16 session_viewer/src/app/inspection.rs type -->
## Check

<!-- checkpoint: 16 -->

Expected: The scene stays visible while the inspection snapshot reports retained source memory; status: **the status clears when loading finishes**.

![Checkpoint 16: the scene is unchanged; the new figures live in the inspection snapshot above.](screenshots/16.png)

If it fails:

- Payload bytes double for shared files: shared source values are counted more than once.
- The scan count grows each frame: accounting is not cached by document identity.

## What changed

<!-- tree: 16 session_viewer/src/app -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 16](../lessons/16/index.md).

## Next

[17 · Faces, text objects and silhouettes](17-source-presentation.md): source-face selection, selectable authored text and one black outline.

## Expected viewer result

Checkpoint 16: the scene is unchanged; the new figures live in the inspection snapshot above.

[![Full viewer result for 16 accounting](screenshots/16.png)](screenshots/16.png)
