# 13.4 · Original sheet entity metadata

Read a bounded original sheet side-table record without loading or rewriting the drawing.

## Ownership and data flow

A sheet entity ID addresses one original side-table record. Validate its header/count and fixed offset/length entry before decoding the bounded JSON body; source revision and sheet identity must remain attached to the request. This endpoint adds pure metadata parsing. It preserves the current drawing; asynchronous selection queries and ranged sheet rendering connect in the next chapter.

## The maintained implementation

app/sheet_query.rs defines SHM1, an 8-byte header, 16-byte records and a 64KiB maximum entity blob. EntityMeta retains original guid/name/kind/width/colour. Query owns cancellation independently of drawing storage; parsing a record does not prove the source sheet is rendered or loaded completely.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/scene_instances.rs](../../../src/app/scene_instances.rs), [engine/gpu/instanced.rs](../../../src/engine/gpu/instanced.rs), [engine/gpu/slots.rs](../../../src/engine/gpu/slots.rs), [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs), [app/hierarchy.rs](../../../src/app/hierarchy.rs), [app/sheet_query.rs](../../../src/app/sheet_query.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-70za-read-original-sheet-metadata-records"></a>

<details>
<summary>Implementation 70za-read-original-sheet-metadata-records: Read original sheet metadata records</summary>

Describe the original sheet side-table layout and entity identity fields.

Data flow: original table bytes → revision-bound entity → cancellable lookup.

### 1. `src/sheet_entity.rs`

Type the whole original metadata or lookup helpers using their shared owner.

Create the file and type:

```rust
--8<-- "journey/code/70za-read-original-sheet-metadata-records-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original metadata module.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod nested_placement_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/70za-read-original-sheet-metadata-records-2.rs" skip=41
```

</details>

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 70za-read-original-sheet-metadata-records
npm --prefix ../session_tests run course -- check 70za-read-original-sheet-metadata-records
npm --prefix ../session_tests run course -- save 70za-read-original-sheet-metadata-records
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The earlier drawing remains visible while the sheet metadata helpers are prepared.

Trace one header, one record and one JSON entity. Compare missing identity keys with malformed bytes.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.
