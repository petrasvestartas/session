# 14 · Loading scenes

The loader fetches a scene manifest and each geometry file, checks and decodes them, and posts the documents to the event loop: from here the canvas shows a scene. A reload stages the new scene whole while the old one stays on screen.

![Two request generations in flight: the older one is dropped, the newer one is staged in manifest order and swapped in whole while the previous scene stays on screen.](illustrations/loading.svg)

## Step 1 · src/app/manifest.rs

The manifest types: one entry per geometry file with its placement, and the texts placed in the world.

`lessons/14/src/app/manifest.rs` · type this, new file

```rust
--8<-- "lessons/14/src/app/manifest.rs:manifest-types"
```

## Step 2 · src/app/manifest.rs

An item's placement, and the decoded size of a file stored gzip-compressed.

`lessons/14/src/app/manifest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:manifest-item"
```

## Step 3 · src/app/manifest.rs

Parse YAML, JSON or TOML, refuse broken values item by item, then place or name item `i`.

`lessons/14/src/app/manifest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:manifest-parse"
```

## Step 4 · src/app/manifest.rs

Check one world text: some text, a positive height, and two unit axes at right angles.

`lessons/14/src/app/manifest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:manifest-text"
```

## Step 5 · src/app/manifest.rs

Small helpers: default axes, finite checks, the TOML switch, content-addressed names and the auto grid.

`lessons/14/src/app/manifest.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:manifest-helpers"
```

## Step 6 · src/app/manifest.rs

Tests: the three formats read alike, an encoded item needs its size, and bad records are refused.

`lessons/14/src/app/manifest.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/14/src/app/manifest.rs:manifest-tests"
```

## Step 7 · src/app/validate.rs

The scene caps: two million objects and 64 tree levels, each with the message that refuses a file.

`lessons/14/src/app/validate.rs` · type this, new file

```rust
--8<-- "lessons/14/src/app/validate.rs:validate-limits"
```

## Step 8 · src/app/validate.rs

Check the NURBS records of session JSON before the kernel parses it.

`lessons/14/src/app/validate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:validate-json"
```

## Step 9 · src/app/validate.rs

Walk a whole protobuf session, a loaded one, or a definitions list, checking every record against the caps.

`lessons/14/src/app/validate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:validate-session"
```

## Step 10 · src/app/validate.rs

Points, lines, polylines, elements and placements: finite numbers, whole triples and affine matrices.

`lessons/14/src/app/validate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:validate-records"
```

## Step 11 · src/app/validate.rs

NURBS curves and surfaces: order, control count, knots in order, and enough control storage.

`lessons/14/src/app/validate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:validate-nurbs"
```

## Step 12 · src/app/validate.rs

Meshes name only existing vertices, cloud and octree arrays agree in length, and BRep vertices are finite.

`lessons/14/src/app/validate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:validate-mesh"
```

## Step 13 · src/app/validate.rs

Tests: huge counts, missing indices and mismatched JSON controls are refused.

`lessons/14/src/app/validate.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:validate-tests"
```

## Step 14 · src/app/validate.rs

Read one protobuf varint, refusing one longer than ten bytes.

`lessons/14/src/app/validate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/validate.rs:validate-varint"
```

## Step 15 · src/app/range_gate.rs

A waiting read's turn in the queue, with the waker that resumes it.

`lessons/14/src/app/range_gate.rs` · type this, new file

```rust
--8<-- "lessons/14/src/app/range_gate.rs:gate-turn"
```

## Step 16 · src/app/range_gate.rs

The gate: three slots, a first-come queue, and `close` to turn waiting reads away when their scene goes.

`lessons/14/src/app/range_gate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/range_gate.rs:gate"
```

## Step 17 · src/app/range_gate.rs

Entering is a hand-written future; dropping it leaves the queue or passes its slot on.

`lessons/14/src/app/range_gate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/range_gate.rs:gate-enter"
```

## Step 18 · src/app/range_gate.rs

The permit a read holds, which frees its slot when dropped.

`lessons/14/src/app/range_gate.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/range_gate.rs:gate-permit"
```

## Step 19 · src/app/range_gate.rs

Tests: a few reads run, the rest queue in order, and dropped or closed reads free their place.

`lessons/14/src/app/range_gate.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/14/src/app/range_gate.rs:gate-tests"
```

## Step 20 · src/app/fetch.rs

Fetch limits, and how a failed read is described and whether it is worth one more try.

`lessons/14/src/app/fetch.rs` · type this, new file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-errors"
```

## Step 21 · src/app/fetch.rs

What a GET returns, and its options: skip the cache, revalidate, a conditional ETag and a byte range.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-reply"
```

## Step 22 · src/app/fetch.rs

GET a URL, leaving a large body in a JavaScript buffer outside wasm memory.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-get"
```

## Step 23 · src/app/fetch.rs

Send the request with an abort controller, or adopt the one `index.html` started while the wasm downloaded.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-start"
```

## Step 24 · src/app/fetch.rs

Read the body as it arrives into a growing buffer, so only a stall trips the deadline.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-body"
```

## Step 25 · src/app/fetch.rs

Unpack a gzip body that arrived still packed, with the browser's `DecompressionStream`.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-gunzip"
```

## Step 26 · src/app/fetch.rs

Whole-file GETs, a HEAD request for the size, and a range read that checks its length and ETag.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-helpers"
```

## Step 27 · src/app/fetch.rs

Sleep, yield to the browser, and a `Task` that starts work now and is awaited later.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-timers"
```

## Step 28 · src/app/fetch.rs

A timer that aborts a fetch after 30 seconds without a byte.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-deadline"
```

## Step 29 · src/app/fetch.rs

A status-bar line for downloads of 4 MB and more.

`lessons/14/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/fetch.rs:fetch-progress"
```

## Step 30 · src/app/decode.rs

Decode limits: 25,000 objects per browser tick, a 1 MiB window, the 512 MiB cap and two refusal messages.

`lessons/14/src/app/decode.rs` · type this, new file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-limits"
```

## Step 31 · src/app/decode.rs

A file body: bytes already in wasm memory, or a fetched buffer left in JavaScript.

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-body"
```

## Step 32 · src/app/decode.rs

Read one protobuf field header at a time through the window, then its value as text or a message.

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-reader"
```

## Step 33 · src/app/decode.rs

A pacer that yields every 25,000 objects, and a macro that decodes, checks and adds one object.

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-object"
```

## Step 34 · src/app/decode.rs

Decode a file one object at a time; `s.reindex()` then rebuilds the guid indexes the hand-filled tables lack, so undo finds every object.

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-session"
```

## Step 35 · src/app/decode.rs

Count the records against the two-million cap first, and reserve room so no table doubles mid-decode.

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-count"
```

## Step 36 · src/app/decode.rs

The graph, the tree and its nodes, read with explicit stacks and the 64-level cap.

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-graph"
```

## Step 37 · src/app/decode.rs

Instances and their shared definitions, with each stored placement folded into the session's xforms.

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-instances"
```

## Step 38 · src/app/decode.rs

Session JSON, read only when the `json-sessions` feature is on.

`lessons/14/src/app/decode.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-json"
```

## Step 39 · src/app/decode.rs

Tests: a decoded file matches its source, broken files are refused, and instances keep their placements.

`lessons/14/src/app/decode.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/14/src/app/decode.rs:decode-tests"
```

## Step 40 · src/app/fonts.rs

Fetch the whole fonts once, when a name needs a character the subsets of [lesson 10](10-text-layout.md) lack.

`lessons/14/src/app/fonts.rs` · type this, new file

```rust
--8<-- "lessons/14/src/app/fonts.rs:fonts"
```

## Step 41 · src/lib.rs

Keep the arrived fonts for the page's life and hand them to the labels.

`lessons/14/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/lib.rs:use-fonts"
```

## Step 42 · src/app/live.rs

Live mode's defaults and its relay, an EventSource that raises a flag when the publisher announces an upload.

`lessons/14/src/app/live.rs` · type this, new file

```rust
--8<-- "lessons/14/src/app/live.rs:live-notify"
```

## Step 43 · src/app/live.rs

Open `impl LiveSource`: the watched scene and what was last seen of it, set from `?live=`, `?poll=` and `?notify=`.

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:live-source"
```

## Step 44 · src/app/live.rs

Read a URL with its last ETag, or compare a hash of the bytes, and adopt a valid manifest.

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:live-read"
```

## Step 45 · src/app/live.rs

One tick: when announced or due, read the manifest and its files again and return only a complete scene.

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:live-check"
```

## Step 46 · src/app/live.rs

Decode and keep each file, and build one document per manifest item; the brace closes the impl.

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:live-load"
```

## Step 47 · src/app/live.rs

The folder of a URL, and which relay messages announce a new upload.

`lessons/14/src/app/live.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/live.rs:live-relay"
```

## Step 48 · src/app/loader.rs

The loader's globals: the event-loop proxy, point and segment budgets, and generation counters that stop stale work.

`lessons/14/src/app/loader.rs` · type this, new file

```rust
--8<-- "lessons/14/src/app/loader.rs:loader-state"
```

## Step 49 · src/app/loader.rs

Post a message to the event loop, and the point and segment budgets the URL may raise.

`lessons/14/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/loader.rs:loader-budget"
```

## Step 50 · src/app/loader.rs

Boot: download the manifest while the GPU opens, load the scene, then keep polling a live source.

`lessons/14/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/loader.rs:loader-boot"
```

## Step 51 · src/app/loader.rs

Load every item: skip what exceeds the budget, fetch, unpack, decode, then post it or stage it for a whole swap.

`lessons/14/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/loader.rs:loader-route"
```

## Step 52 · src/app/loader.rs

Read the next file ahead: probe it while this one loads, fetch its body while this one decodes.

`lessons/14/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/loader.rs:loader-ahead"
```

## Step 53 · src/app/loader.rs

The per-item context, the streaming hook lessons 15 and 19 fill, and the scene budget sized to the device.

`lessons/14/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/14/src/app/loader.rs:loader-item"
```

## Step 54 · tests

Copy the browser tests `tests/lifecycle.cjs` and `tests/loading.cjs` from `lessons/14/`; they are checked, not explained.

## Step 55 · registration lines

Copy the lines tagged `register:loading` and the module tags below from these files of `lessons/14/`:

- `src/app/mod.rs`: the modules `decode`, `fetch`, `fonts`, `live`, `loader`, `manifest`, `range_gate` and `validate`, with the `#[cfg]` line above those that have one.
- `src/lib.rs`: the `Fonts` message, spawning `loader::boot` once the window exists, and handling `Fonts`.
- `src/state.rs`: asking for the whole fonts when a document is appended.

Run `cargo check` in `lessons/14/`.

## Check

`cargo check` compiles, and `cargo xtest --lib manifest`, `cargo xtest --lib validate`, `cargo xtest --lib range_gate` and `cargo xtest --lib decode` pass. Served with `trunk serve`, the canvas now shows its scene, for example `?scene=scenes/view_mixed.yaml` from the public bucket, and the status line clears when loading finishes.
