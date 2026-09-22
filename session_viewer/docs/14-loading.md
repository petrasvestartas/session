# 14 · Loading scenes

The local fixture loads from a manifest and still supports selection and control points.

![Two request generations in flight: the older one is dropped, the newer one is staged in manifest order and swapped in whole while the previous scene stays on screen.](illustrations/loading.svg)

## Step 1 · src/app/manifest.rs

The manifest describes scene files and their placements.

`lessons/14/src/app/manifest.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/14/src/app/manifest.rs:step-1a"
```

`lessons/14/src/app/manifest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:step-1b"
```

`lessons/14/src/app/manifest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:step-1c"
```

`lessons/14/src/app/manifest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:step-1d"
```

Copy this part from the lesson folder to the path shown.

`lessons/14/src/app/manifest.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:step-1e"
```

## Step 2 · src/app/validate.rs

Copy this file from the lesson folder to the path shown.

`lessons/14/src/app/validate.rs` · copy the file, new file, start with these lines

```rust
--8<-- "lessons/14/src/app/validate.rs:step-2a"
```

`lessons/14/src/app/validate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:step-2b"
```

Copy this part from the lesson folder to the path shown.

`lessons/14/src/app/validate.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:step-2c"
```

## Step 3 · src/app/decode.rs

Decode converts serialized bytes into retained source documents.

`lessons/14/src/app/decode.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/14/src/app/decode.rs:step-3a"
```

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:step-3b"
```

## Step 4 · src/app/route.rs

Route helpers read viewer options from the page URL.

`lessons/14/src/app/route.rs` · edit · type this

Added at the top of `lessons/13/src/app/route.rs`

```rust
--8<-- "lessons/14/src/app/route.rs:step-4a"
```

Added after the `}` line of `lessons/13/src/app/route.rs`

```rust
--8<-- "lessons/14/src/app/route.rs:step-4b"
```

## Step 5 · src/app/live.rs

Live notifications request a conditional reload instead of replacing geometry directly.

`lessons/14/src/app/live.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/14/src/app/live.rs:step-5a"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5b"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5c"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5d"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5e"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5f"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5g"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5h"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5i"
```

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:step-5j"
```

Run `cargo check` in `lessons/14/`.

## Step 6 · src/app/loader.rs

The loader stages manifest and geometry work before publishing it.

`lessons/14/src/app/loader.rs` · edit · type this

Replaces the 116 lines from `use super::scene::{FileDoc, Scene, StreamedIn…` of `lessons/13/src/app/loader.rs`

```rust
--8<-- "lessons/14/src/app/loader.rs:step-6a"
```

Replaces `fn spawn_stream_rest` in `lessons/13/src/app/loader.rs`

```rust
--8<-- "lessons/14/src/app/loader.rs:step-6b"
```

## Step 7 · src/app/mod.rs

The application module connects source loading and interaction helpers.

`lessons/14/src/app/mod.rs` · edit · type this

Replaces `mod scene` in `lessons/13/src/app/mod.rs`

```rust
--8<-- "lessons/14/src/app/mod.rs:step-7"
```

## Step 8 · src/app/scene.rs

The scene owns source documents and maps their identities to GPU rows.

`lessons/14/src/app/scene.rs` · edit · type this

Added after the `pub docs: Vec<FileDoc>,` line in `struct Scene` of `lessons/13/src/app/scene.rs`

```rust
--8<-- "lessons/14/src/app/scene.rs:step-8a"
```

Added after the `docs: Vec::new(),` line in `fn new` of `lessons/13/src/app/scene.rs`

```rust
--8<-- "lessons/14/src/app/scene.rs:step-8b"
```

Added after the `self.docs.clear();` line in `fn clear` of `lessons/13/src/app/scene.rs`

```rust
--8<-- "lessons/14/src/app/scene.rs:step-8c"
```

## Step 9 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners.

`lessons/14/src/lib.rs` · edit · type this

Added after the `mod engine;` line of `lessons/13/src/lib.rs`

```rust
--8<-- "lessons/14/src/lib.rs:step-9a"
```

Added after the `File(FileDoc),` line in `enum Msg` of `lessons/13/src/lib.rs`

```rust
--8<-- "lessons/14/src/lib.rs:step-9b"
```

Added after the `Msg::File(doc) => state.append(doc),` line in `fn user_event` of `lessons/13/src/lib.rs`

```rust
--8<-- "lessons/14/src/lib.rs:step-9c"
```

## Step 10 · src/state.rs

State coordinates input, selection and frame requests.

`lessons/14/src/state.rs` · edit · type this

Added after the `);` line in `fn append` of `lessons/13/src/state.rs`

```rust
--8<-- "lessons/14/src/state.rs:step-10a"
```

Added after the `let mut labels = self.scene_labels.clone();` line in `fn update_label` of `lessons/13/src/state.rs`

```rust
--8<-- "lessons/14/src/state.rs:step-10b"
```

Added after the `self.status(&format!("Text: {error}"));` line in `fn update_label` of `lessons/13/src/state.rs`

```rust
--8<-- "lessons/14/src/state.rs:step-10c"
```

## Step 11 · Trunk.toml

Copy this file from the lesson folder to the path shown.

`lessons/14/Trunk.toml` · 17 lines · copy the file, replace the whole file

```toml
--8<-- "lessons/14/Trunk.toml"
```

The maintained viewer adds one more table here, a `[[hooks]]` pre-build step that runs `docs/build_site.sh` so the course site is served next to the viewer.

## Step 12 · docs/build_site.sh

Copy this file from the lesson folder to the path shown.

`lessons/14/docs/build_site.sh` · 23 lines · copy the file, new file

```bash
--8<-- "lessons/14/docs/build_site.sh"
```

## Step 13 · index.html

Add the `copy-dir` link only now: Trunk refuses to build when its source directory is missing.

`lessons/14/index.html` · edit · type this

Added after the `<link data-trunk rel="copy-file" href="assets…` line of `lessons/13/index.html`

```html
--8<-- "lessons/14/index.html:step-13"
```

Copy each file from the lesson folder to the path shown.

Copy from `lessons/14/` (tooling this checkpoint needs but the course does not teach):

- `lessons/14/tests/lifecycle.cjs`
- `lessons/14/tests/loading.cjs`

## Check

Run `trunk serve` in `lessons/14/` and open <http://127.0.0.1:8770/>.

Expected: The local fixture loads from a manifest and still supports selection and control points; status: **the status clears when loading finishes**.

![Checkpoint 14: the same interaction fixture, now fetched through the manifest and protobuf path.](screenshots/14.png)

If it fails:

- Nothing loads: the status names a failed fetch, parse or decode stage.
- A replaced scene reappears: an old loader generation is allowed to publish.

## What changed

```text
lessons/14/src/app/
├── walk/
│   ├── bounds.rs
│   ├── brep.rs
│   ├── brep_edges.rs
│   ├── brep_orient.rs
│   ├── cloud.rs
│   ├── curves.rs
│   ├── encode.rs
│   ├── frames.rs
│   ├── mesh.rs
│   ├── mesh_ink.rs
│   ├── mesh_topology.rs
│   ├── mod.rs
│   └── points.rs
├── cloud_query.rs
├── decode.rs  +
├── feedback.rs
├── fetch.rs
├── input.rs
├── inspection.rs
├── knobs.rs
├── live.rs  +
├── loader.rs  ~
├── manifest.rs  +
├── mod.rs  ~
├── route.rs  ~
├── scene.rs  ~
├── selection.rs
├── stream.rs
├── touch.rs
└── validate.rs  +
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/14/`.

## Next

[15 · Publication and streamed reads](15-publication.md): bounded metadata windows for streamed clouds, and the publication helpers.

## Expected viewer result

Checkpoint 14: the same interaction fixture, now fetched through the manifest and protobuf path.

[![Full viewer result for 14 loading](screenshots/14.png)](screenshots/14.png)
