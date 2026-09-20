# 19 · Sheets: batched drawings with lazy metadata
<!-- locator: off -->

Two vector sheets stream into top view and clicking a segment resolves its source entity.

![As objects, every line pays for a GUID string, a name, a colour and four copies of itself; as one batch a line is a few numbers and a small source id, with guid, name and kind in a side table read only when something is selected.](illustrations/sheet-cost.svg)

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 3 and 9; steps 4–8 fail and build again at step 9.

<!-- step-status: end -->

## Step 1 · session_proto/sheet.proto

Read this source file from its link; the checkpoint already contains it.
<!-- listing: 19 session_proto/sheet.proto -->
Download each file to the path shown.
<!-- supplied: 19 -->
## Step 2 · src/app/stream.rs

Streaming reads bounded chunks and keeps stable source addresses. Display prefixes do not limit source queries.
<!-- file: 19 session_viewer/src/app/stream.rs type -->
## Step 3 · src/app/loader.rs

The loader stages manifest and geometry work before publishing it. A cancelled generation must not post into the new scene.
<!-- file: 19 session_viewer/src/app/loader.rs type -->
## Step 4 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 19 session_viewer/src/lib.rs type -->
## Step 5 · src/app/walk/sheet.rs

Sheet slices append compact segments without creating one object per line. Pad short attribute arrays so segment IDs stay aligned.
<!-- file: 19 session_viewer/src/app/walk/sheet.rs type lines=1-64 -->
Download this part from its link to the path shown.
<!-- file: 19 session_viewer/src/app/walk/sheet.rs copy lines=65-109 -->
## Step 6 · src/app/walk/mod.rs

The geometry walk dispatches source types into their render buffers. Keep object-row indices consistent across every output.
<!-- file: 19 session_viewer/src/app/walk/mod.rs type -->
## Step 7 · src/engine/gpu/segments.rs

The segment buffers store strokes and the object rows they belong to. Preserve source IDs when expanding or joining segments.
<!-- file: 19 session_viewer/src/engine/gpu/segments.rs type -->
## Step 8 · src/app/scene.rs

The scene owns source documents and maps their identities to GPU rows. Rebuild that mapping whenever rows are replaced.
<!-- file: 19 session_viewer/src/app/scene.rs type -->
## Step 9 · src/app/sheet_query.rs

Sheet queries resolve a picked segment to its source entity. Discard a reply after the active scene changes.
<!-- file: 19 session_viewer/src/app/sheet_query.rs type lines=1-58 -->
<!-- file: 19 session_viewer/src/app/sheet_query.rs type lines=59-101 -->
<!-- file: 19 session_viewer/src/app/sheet_query.rs type lines=102-129 -->
Download this part from its link to the path shown.
<!-- file: 19 session_viewer/src/app/sheet_query.rs copy lines=130-257 -->
## Step 10 · src/app/mod.rs

The application module connects source loading and interaction helpers. Declare each file before importing it elsewhere.
<!-- file: 19 session_viewer/src/app/mod.rs type -->
## Step 11 · src/state.rs

State coordinates input, selection and frame requests. Cancel stale asynchronous results when the scene or camera changes.
<!-- file: 19 session_viewer/src/state.rs type -->
## Step 12 · src/state/sheet_query.rs

State associates sheet replies with the current selection. A late response must not replace a newer selection.
<!-- file: 19 session_viewer/src/state/sheet_query.rs type -->
## Step 13 · src/app/inspection.rs

Inspection reports retained resources and source information. Count shared documents once instead of once per placement.
<!-- file: 19 session_viewer/src/app/inspection.rs type -->
<!-- check: 19 -->
## Check

<!-- checkpoint: 19 -->

Expected: Two vector sheets stream into top view and clicking a segment resolves its source entity; status: **Selected entity … fetching…**.

[![Full viewer result for 19 sheets](screenshots/19-sheets-overview.png)](screenshots/19-sheets-overview.png)

If it fails:

- No sheets arrive: the manifest or data server does not provide the sheet stream.
- A picked entity has the wrong name: a display segment index replaces its source entity ID.

## What changed

<!-- tree: 19 session_viewer/src -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 19](../lessons/19/index.md).

## Next

[20 · The document](20-history.md): undo, redo and save in the kernel.

## Expected viewer result

Open `?scene=view_sheets&inspect=1`, press **5** for top view and **F** to fit both drawings. The sheet linework should remain readable across the full viewer. This reference was captured in the maintained viewer; entity lookup and streamed reads are checked above.

[![Full viewer result for 19 sheets](screenshots/19-sheets-overview.png)](screenshots/19-sheets-overview.png)
