# 37 · Self-test: the finished viewer

The last lesson adds a native harness: it loads scenes, draws them without a window and writes the image, so the viewer can be checked from a terminal. With it, `lessons/37` is the viewer itself: the same files as `session_viewer/src`, plus the teaching comments.

## Step 1 · src/lib.rs

The harness module exists only in native builds, because the browser has no files to read or write.

`lessons/37/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/37/src/lib.rs:selftest-mod"
```

## Step 2 · src/selftest.rs

New file: the files to load, read from manifests or single `.pb` paths given on the command line.

`lessons/37/src/selftest.rs` · type this, new file

```rust
--8<-- "lessons/37/src/selftest.rs:selftest-files"
```

## Step 3 · src/selftest.rs

Fit the camera to the scene, adjust it from `VIEWER_*` environment variables, and log where it looks from.

`lessons/37/src/selftest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/37/src/selftest.rs:selftest-camera"
```

## Step 4 · src/selftest.rs

Decode and walk each file, timing both, then upload once; build the per-frame input from the camera.

`lessons/37/src/selftest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/37/src/selftest.rs:selftest-load"
```

## Step 5 · src/selftest.rs

Benchmark helpers: drag frame times per quality tier, forty test labels, and up to six clipping planes.

`lessons/37/src/selftest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/37/src/selftest.rs:selftest-bench"
```

## Step 6 · src/selftest.rs

Write a PPM image, then `render_scene`: load, frame, optionally time many frames, draw one and count its ink.

`lessons/37/src/selftest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/37/src/selftest.rs:selftest-render"
```

## Step 7 · src/selftest.rs

Write the id frame with a guid map beside it, and report what one pixel picks.

`lessons/37/src/selftest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/37/src/selftest.rs:selftest-ids"
```

## Step 8 · src/selftest.rs

Declare the lifecycle checks, and check that every BRep edge keeps its id after upload.

`lessons/37/src/selftest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/37/src/selftest.rs:selftest-cad"
```

## Step 9 · src/selftest/lifecycle.rs

New file: MSAA changes, resize, a second walk, an edit and its undo, and a reload must all draw identical pixels and ids.

`lessons/37/src/selftest/lifecycle.rs` · copy the file

```rust
--8<-- "lessons/37/src/selftest/lifecycle.rs:lifecycle"
```

Copy these files from `lessons/37/`:

- `examples/selftest.rs`: renders the given scenes to a PPM file and prints the ink count.
- `examples/check_cad_fixture.rs`: runs the BRep edge check on the `cad_fixture` files.
- `examples/check_hidden_line_lifecycle.rs`: runs the lifecycle checks.
- `tests/color-channels.cjs`: a browser check of face and edge colours, reset, save and open.
- `tests/final-review.md`: the last review's record of what was checked.

Run `cargo check` in `lessons/37/`.

## Check

`cargo run --target x86_64-unknown-linux-gnu --example selftest -- frame.ppm assets/view_local.yaml` in `lessons/37/` writes `frame.ppm` and prints how many pixels hold ink.

`lessons/37` now equals the viewer, and three commands prove it from `session_viewer/`:

- `python3 docs/check_lesson37.py` reads zero differences between `src` and `docs/lessons/37` once comments are stripped.
- `cargo xtest` passes the native tests, the same in `session_viewer/` and in `lessons/37/`.
- `node tests/lifecycle.cjs`, with the viewer served by `trunk serve`, runs the browser self-test: focus, pointer cancel, a hidden canvas and a DPR change.

## Next

[Capstone](capstone.md): what to build on top of the finished viewer.
