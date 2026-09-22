# 15 · Publication and streamed reads

The local scene stays visible while streamed cloud metadata loads in one bounded request.

![The file is small fields between huge arrays; the window fetches the small fields once and skips the arrays by length.](illustrations/metadata-window.svg)

## Step 1 · src/app/stream.rs

Streaming reads bounded chunks and keeps stable source addresses.

`lessons/15/src/app/stream.rs` · edit · type this

Added after the `}` line of `lessons/14/src/app/stream.rs`

```rust
--8<-- "lessons/15/src/app/stream.rs:step-1a"
```

```rust
--8<-- "lessons/15/src/app/stream.rs:step-1b"
```

`lessons/15/src/app/stream.rs` · edit · type this

Added after the `const MAX_TABLE_BYTES: u64 = 128 * 1024 * 1024;` line in `mod web` of `lessons/14/src/app/stream.rs`

```rust
--8<-- "lessons/15/src/app/stream.rs:step-1c"
```

`lessons/15/src/app/stream.rs` · edit · type this

Replaces the 4 lines from `` of `lessons/14/src/app/stream.rs`

```rust
--8<-- "lessons/15/src/app/stream.rs:step-1d"
```

Replaces the 6 lines from `let skip = skip_scalar(&header, used, wire)?;` in `fn cloud_lod` of `lessons/14/src/app/stream.rs`

```rust
--8<-- "lessons/15/src/app/stream.rs:step-1e"
```

Replaces the 3 lines from `let raw = source_range(url, body, length, &fi…` in `fn cloud_lod` of `lessons/14/src/app/stream.rs`

```rust
--8<-- "lessons/15/src/app/stream.rs:step-1f"
```

Copy this part from the lesson folder to the path shown.

`lessons/15/src/app/stream.rs` · edit · copy the file

Added after the `use super::*;` line in `mod tests` of `lessons/14/src/app/stream.rs`

```rust
--8<-- "lessons/15/src/app/stream.rs:step-1g"
```

Copy each file from the lesson folder to the path shown.

Copy from `lessons/15/` (tooling this checkpoint needs but the course does not teach):

- `lessons/15/tests/publication.py`
- `lessons/15/tests/streamed-controls.cjs`

## Check

Run `trunk serve` in `lessons/15/` and open <http://127.0.0.1:8770/>.

Expected: The local scene stays visible while streamed cloud metadata loads in one bounded request; status: **the status clears when loading finishes**.

![Checkpoint 15: the local scene is unchanged; the difference is in the network panel of a streamed cloud, where the header reads collapse into one window request.](screenshots/15.png)

If it fails:

- A source query misses points: display-prefix indices replace original source IDs.
- Range loading downloads the whole file: the server does not honour the byte range.

## What changed

```text
lessons/15/src/app/
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
├── decode.rs
├── feedback.rs
├── fetch.rs
├── input.rs
├── inspection.rs
├── knobs.rs
├── live.rs
├── loader.rs
├── manifest.rs
├── mod.rs
├── route.rs
├── scene.rs
├── selection.rs
├── stream.rs  ~
├── touch.rs
└── validate.rs
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/15/`.

## Next

[16 · Resource accounting](16-accounting.md)

## Expected viewer result

The scene looks the same; the network panel shows one window request per streamed cloud.

[![Full viewer result for 15 publication](screenshots/15.png)](screenshots/15.png)
