# 19 · Sheets: batched drawings with lazy metadata

A sheet is one drawing published as a single batch of flat segment arrays, streamed by range like a cloud. Each segment carries only an entity id; the guid, name and kind wait in a `.meta` side table next to the file and are read when one entity is picked. The format is [session_proto/sheet.proto](kernel/sheet_proto.md).

![As objects, every line pays for a GUID string, a name, a colour and four copies of itself; as one batch a line is a few numbers and a small source id, with guid, name and kind in a side table read only when something is selected.](illustrations/sheet-cost.svg)

## Step 1 · src/app/walk/sheet.rs

The columns of one streamed slice, and where the slice goes in the sheet.

`lessons/19/src/app/walk/sheet.rs` · type this, new file

```rust
--8<-- "lessons/19/src/app/walk/sheet.rs:sheet-rows"
```

## Step 2 · src/app/walk/sheet.rs

Append a slice to the sheet rows of the segment lane, one draw per slice, and return its box.

`lessons/19/src/app/walk/sheet.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/walk/sheet.rs:sheet-walk"
```

## Step 3 · src/app/walk/sheet.rs

A test that short columns get defaults and the box covers every segment end.

`lessons/19/src/app/walk/sheet.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/19/src/app/walk/sheet.rs:sheet-walk-tests"
```

## Step 4 · src/app/sheet_query.rs

The side table layout: its head, one fixed-size record per entity, and the JSON entity behind it.

`lessons/19/src/app/sheet_query.rs` · type this, new file

```rust
--8<-- "lessons/19/src/app/sheet_query.rs:side-table"
```

## Step 5 · src/app/sheet_query.rs

One entity lookup with its cancel flag, and the answer it posts back.

`lessons/19/src/app/sheet_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/sheet_query.rs:entity-query"
```

## Step 6 · src/app/sheet_query.rs

Read one entity in the background: the head once, then its record, then its blob.

`lessons/19/src/app/sheet_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/sheet_query.rs:read-entity"
```

## Step 7 · src/app/sheet_query.rs

A test on a hand-built side table, and one that a dropped query cancels its read.

`lessons/19/src/app/sheet_query.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/19/src/app/sheet_query.rs:side-table-tests"
```

## Step 8 · src/app/stream.rs

Find where a sheet's four arrays sit in the file, from the probe the loader already made.

`lessons/19/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/stream.rs:sheet-fields"
```

## Step 9 · src/app/stream.rs

Read segments `[from, to)` as four range requests at once; the brace closes the browser-only module.

`lessons/19/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/stream.rs:sheet-slice"
```

## Step 10 · src/app/loader.rs

A sheet in the scene list posts its first segments now, or stages them while a scene is being replaced.

`lessons/19/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/loader.rs:sheet-start"
```

## Step 11 · src/app/loader.rs

Read the first 25 000 segments and name the side table beside the sheet file.

`lessons/19/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/loader.rs:sheet-prefix"
```

## Step 12 · src/app/loader.rs

Keep reading 500 000-segment slices in the background until the sheet ends or the page budget runs out.

`lessons/19/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/loader.rs:sheet-rest"
```

## Step 13 · src/app/scene.rs

The first slice as it arrives, and the sheet's slot in the scene.

`lessons/19/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/scene.rs:sheet-types"
```

## Step 14 · src/app/scene.rs

Add a sheet as one row with an empty shell document, grow it slice by slice, and map a pick to its entity.

`lessons/19/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/scene.rs:sheet-scene"
```

## Step 15 · src/state.rs

Add a sheet and its later slices to the scene, and let the camera grow to cover them.

`lessons/19/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/state.rs:sheet-state"
```

## Step 16 · src/state/sheet_query.rs

Open `impl State` in its own file: select a picked entity, light it up and start its lookup.

`lessons/19/src/state/sheet_query.rs` · type this, new file

```rust
--8<-- "lessons/19/src/state/sheet_query.rs:sheet-pick"
```

## Step 17 · src/state/sheet_query.rs

Show the entity's name and kind when its answer arrives; the brace closes the impl.

`lessons/19/src/state/sheet_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/state/sheet_query.rs:sheet-answer"
```

## Step 18 · src/lib.rs

The message for a later slice, and the handler that adds a sheet and starts reading the rest.

`lessons/19/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/lib.rs:sheet-stream"
```

## Step 19 · src/app/inspection.rs

The picked sheet entity, for the inspection snapshot.

`lessons/19/src/app/inspection.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/19/src/app/inspection.rs:sheet-entity"
```

## Step 20 · examples/mk_sheet.rs

Copy `examples/mk_sheet.rs` from `lessons/19/`: the publisher that turns a session of lines, polylines and curves into one sheet and its `.meta` file. Run it with `cargo run --example mk_sheet --target x86_64-unknown-linux-gnu -- in.pb out.pb`.

## Step 21 · registration lines

Copy the lines tagged `register:sheets` and `register:sheet_query` from these files of `lessons/19/`:

- `src/app/walk/mod.rs` and `src/app/mod.rs`: the `sheet` and `sheet_query` modules.
- `src/state.rs`: the `sheet_query` module, dropping the lookup in flight on clear and on every new selection, and a sheet pick sent to `apply_sheet_pick`.
- `src/state/features.rs`: the query in flight and the query counter.
- `src/lib.rs`: the three sheet messages and their handlers.
- `src/app/loader.rs`: the staged sheet, its post, and `start_sheet` in the item loop.
- `src/app/scene.rs`: the sheet list, its start value and clear, the sheet in the row namers, and the entity of a pick.
- `src/app/inspection.rs`: the picked entity in the snapshot.

Run `cargo check` in `lessons/19/`.

## Check

`cargo check` compiles, and `cargo xtest --lib sheet` passes the slice and side-table tests. With `trunk serve` in `lessons/19/`, `?scene=scenes/view_sheets.yaml` streams two drawings as two objects; press **5** then **F** to fit them, and clicking a line shows **Selected entity N, fetching…** and then its name and kind.

![Checkpoint 19: both sheets fitted in top view.](screenshots/19-sheets-overview.png)
