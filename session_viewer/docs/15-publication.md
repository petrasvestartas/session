# 15 · Publish immutable geometry and reuse it

**Start:** checkpoint 14. **Finish:** geometry revisions are verified before manifests refer to them, and streamed metadata reads avoid needless serial requests.

## Publish references only after their target is valid

```text
local geometry → content hash → upload immutable revision → verify bytes
                                                            ↓
                                           stable alias when needed
                                                            ↓
                                                publish scene manifest
                                                            ↓
                                                browser revalidation
```

A mutable manifest can change placement or style while continuing to reference the same immutable geometry hash. The browser can then reuse the decoded document. Publishing the manifest first creates a window where it refers to missing or incomplete geometry.

The existing `view_put.sh` and `view_live.sh` workflows remain the user-facing commands. Their local helpers perform upload, verification and alias/manifest updates. Write credentials stay on the local side; they do not belong in WASM assets or a browser manifest.

## Failure must preserve the prior valid scene

A failed geometry upload or verification prevents the manifest update. Repeated unchanged geometry verifies and reuses its revision instead of sending another full payload. Placement/style metadata remains intact rather than being rebuilt from a lossy minimal manifest.

The course includes local mock publication tests for ordering, failed upload, failed verification and unchanged-content reuse. Run those without an R2 account. Real publication is an external action and is not required to learn or verify this checkpoint.

## Reduce requests without guessing the file layout

Streamed protobuf metadata contains length-delimited fields. The loader can skip large payload ranges after reading their headers. Small metadata fields near the end can share a bounded cached range instead of requesting each header and body serially.

`MetadataWindow` retains a bounded tail/read window and validates the exposed source revision. Larger arrays keep their explicit per-array/total bounds. The optimization must not accidentally request an entire point/ID payload just to locate a later field.

The earlier matched-density benchmark reduced source requests from 118 to 51 and improved observed loading time. Those historical loading measurements are in [Measurements](measurements.md); they do not establish faster GPU navigation.

## Write the files

Follow [Complete file changes for 15](../lessons/15/index.md). Read metadata range parsing and revision validation, then the local publication helpers. Paths starting `bash/` are siblings of `session_viewer` beneath `$COURSE_WORK`, as shown in the file list.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Verify the local viewer still loads, then stop Trunk and run the supplied mock publication test:

```sh
python3 tests/publication.py
```

 No remote write is needed. A failed verification must leave the old manifest untouched; an unchanged revision must avoid another geometry PUT.

Run the streamed-control fixture from lesson 13 after metadata changes: all source pages and original IDs must remain correct. A lower request count is not a success if source selection becomes incomplete.

**Before continuing:** explain how a placement-only update can change the scene with zero new geometry bytes. Continue to [resource accounting](16-verification.md).
