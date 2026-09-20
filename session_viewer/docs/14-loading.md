# 14 · Loading scenes
<!-- locator: off -->

The local fixture loads from a manifest and still supports selection and control points.

![Two request generations in flight: the older one is dropped, the newer one is staged in manifest order and swapped in whole while the previous scene stays on screen.](illustrations/loading.svg)

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 1–5 and 7; step 6 fails and builds again at step 7.

<!-- step-status: end -->

## Step 1 · src/app/manifest.rs

The manifest describes scene files and their placements. Report malformed entries before starting geometry downloads.
<!-- file: 14 session_viewer/src/app/manifest.rs type lines=1-42 -->
<!-- file: 14 session_viewer/src/app/manifest.rs type lines=43-125 -->
<!-- file: 14 session_viewer/src/app/manifest.rs type lines=126-145 -->
<!-- file: 14 session_viewer/src/app/manifest.rs type lines=146-224 -->
Download this part from its link to the path shown.
<!-- file: 14 session_viewer/src/app/manifest.rs copy lines=225-359 -->
## Step 2 · src/app/validate.rs

Download this file from its link to the path shown.
<!-- file: 14 session_viewer/src/app/validate.rs copy lines=1-168 -->
<!-- file: 14 session_viewer/src/app/validate.rs type lines=169-270 -->
Download this part from its link to the path shown.
<!-- file: 14 session_viewer/src/app/validate.rs copy lines=271-550 -->
## Step 3 · src/app/decode.rs

Decode converts serialized bytes into retained source documents. Fail invalid input before allocating render rows.
<!-- file: 14 session_viewer/src/app/decode.rs type lines=1-58 -->
<!-- file: 14 session_viewer/src/app/decode.rs type lines=59-157 -->
## Step 4 · src/app/route.rs

Route helpers read viewer options from the page URL. Missing options must retain usable defaults.
<!-- file: 14 session_viewer/src/app/route.rs type -->
## Step 5 · src/app/live.rs

Live notifications request a conditional reload instead of replacing geometry directly. Detach the event callback when its owner is dropped.
<!-- file: 14 session_viewer/src/app/live.rs type lines=1-59 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=60-81 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=82-106 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=107-168 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=169-184 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=185-253 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=254-343 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=344-366 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=367-429 -->
<!-- file: 14 session_viewer/src/app/live.rs type lines=430-461 -->
<!-- check: 14 -->
## Step 6 · src/app/loader.rs

The loader stages manifest and geometry work before publishing it. A cancelled generation must not post into the new scene.
<!-- file: 14 session_viewer/src/app/loader.rs type -->
## Step 7 · src/app/mod.rs

The application module connects source loading and interaction helpers. Declare each file before importing it elsewhere.
<!-- file: 14 session_viewer/src/app/mod.rs type -->
## Step 8 · src/app/scene.rs

The scene owns source documents and maps their identities to GPU rows. Rebuild that mapping whenever rows are replaced.
<!-- file: 14 session_viewer/src/app/scene.rs type -->
## Step 9 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 14 session_viewer/src/lib.rs type -->
## Step 10 · src/state.rs

State coordinates input, selection and frame requests. Cancel stale asynchronous results when the scene or camera changes.
<!-- file: 14 session_viewer/src/state.rs type -->
## Step 11 · Trunk.toml

Download this file from its link to the path shown.
<!-- file: 14 session_viewer/Trunk.toml copy -->
## Step 12 · docs/build_site.sh

Download this file from its link to the path shown.
<!-- file: 14 session_viewer/docs/build_site.sh copy -->
## Step 13 · index.html

The `copy-dir` link arrives here rather than in lesson 12 because Trunk refuses to build at all when a `copy-dir` source is missing, and nothing fills that directory until the hook above exists.
<!-- file: 14 session_viewer/index.html type -->
Download each file to the path shown.
<!-- supplied: 14 -->
## Check

<!-- checkpoint: 14 -->

Expected: The local fixture loads from a manifest and still supports selection and control points; status: **the status clears when loading finishes**.

![Checkpoint 14: the same interaction fixture, now fetched through the manifest and protobuf path.](screenshots/14.png)

If it fails:

- Nothing loads: the status names a failed fetch, parse or decode stage.
- A replaced scene reappears: an old loader generation is allowed to publish.

## What changed

<!-- tree: 14 session_viewer/src/app -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 14](../lessons/14/index.md).

## Next

[15 · Publication and streamed reads](15-publication.md): bounded metadata windows for streamed clouds, and the publication helpers.

## Expected viewer result

Checkpoint 14: the same interaction fixture, now fetched through the manifest and protobuf path.

[![Full viewer result for 14 loading](screenshots/14.png)](screenshots/14.png)
