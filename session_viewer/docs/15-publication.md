# 15 · Publication and streamed reads
<!-- locator: off -->

The local scene stays visible while streamed cloud metadata loads in one bounded request.

![The file is small fields between huge arrays; the window fetches the small fields once and skips the arrays by length.](illustrations/metadata-window.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/app/stream.rs

Streaming reads bounded chunks and keeps stable source addresses. Display prefixes do not limit source queries.
<!-- file: 15 session_viewer/src/app/stream.rs type hunks=1-2 -->
<!-- file: 15 session_viewer/src/app/stream.rs type hunks=3-3 -->
<!-- file: 15 session_viewer/src/app/stream.rs type hunks=4-6 -->
Download this part from its link to the path shown.
<!-- file: 15 session_viewer/src/app/stream.rs copy hunks=7-7 -->
Download each file to the path shown.
<!-- supplied: 15 -->
## Check

<!-- checkpoint: 15 -->

Expected: The local scene stays visible while streamed cloud metadata loads in one bounded request; status: **the status clears when loading finishes**.

![Checkpoint 15: the local scene is unchanged; the difference is in the network panel of a streamed cloud, where the header reads collapse into one window request.](screenshots/15.png)

If it fails:

- A source query misses points: display-prefix indices replace original source IDs.
- Range loading downloads the whole file: the server does not honour the byte range.

## What changed

<!-- tree: 15 session_viewer/src/app -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 15](../lessons/15/index.md).

## Next

[16 · Resource accounting](16-accounting.md): what the viewer can and cannot measure about its own memory.

## Expected viewer result

Checkpoint 15: the local scene is unchanged; the difference is in the network panel of a streamed cloud, where the header reads collapse into one window request.

[![Full viewer result for 15 publication](screenshots/15.png)](screenshots/15.png)
