# Small code, bounded work

## You are building

Use the same design for each extension:

```text
input → named State action → source or view state → changed GPU rows → frame
```

A shorter function is useful when it also removes work or makes ownership clearer. Moving the same work into a generic framework is not a performance improvement.

## Starting point

The 25 numbered lessons reconstruct a **frozen checkpoint**. Their code blocks come from verified patches. The [seven current-viewer checkpoints](extend-integrated-tutorial.md) continue checkpoint 21 to the maintained source; the [independent editing lessons](extend-implementation.md) teach optional features separately. Replay correctness and agreement with today's source are checked separately.

## Step 1 · Name when the work runs

| When | Existing pattern | Keep out of this path |
|---|---|---|
| Load or geometry edit | `Scene` walks source once and uploads rows | Camera-dependent document copies |
| Scene structure changes | `Hierarchy::refresh` checks `row_revision` | Rebuilding a tree on expand or hide |
| Selection or visibility changes | One named action changes flags | A second hide set or undo cursor |
| Pointer moves | Preview the affected object/control | Kernel history transactions per event |
| Frame | Uniform writes, cached preparation, draw calls | Source tessellation or hierarchy construction |
| Scene release | Drop source handles and upload staging | Strong references held by observer caches |

**READ ONLY — `src/app/layers.rs`, `rows`.** One walk counts all document and type buckets. For N objects and D documents, work is O(N + D), with D counters and six fixed type counters. Building each document's member list merely to count it costs O(D × N) and allocates temporary row vectors.

**READ ONLY — `src/app/hierarchy.rs`, `row_of`.** The outer map chooses the document; the inner map accepts a borrowed `&str`. Looking up a GUID does not need a fresh `Rc<str>`. Keep document identity in the key: different placements can share GUIDs.

## Step 2 · Keep one mutation path

**READ ONLY — `src/state/edit.rs`, `toggle_layer`, and `src/state/panel.rs`, `set_rows_hidden`.** Both document/type controls and hierarchy controls use the same visibility writer. It updates `Scene.hidden`, writes only changed GPU flags, and clears selection only when hiding selected rows.

The target rows are sorted by `layers::of_layer` or `Hierarchy::targets`. That permits binary-search membership checks; a nested linear search can turn a large group action into quadratic work. If another caller supplies rows, preserve this contract.

Do not add an index merely to avoid a short scan. Add one when it avoids repeated expensive work, and name the event that invalidates it.

## Step 3 · Decide whether capacity stays

**READ ONLY — `src/engine/gpu/upload.rs`, `drop_rows`.**

```rust
*v = Vec::new();
```

Assignment drops the old vector and its elements, then leaves an empty vector with no allocation. Upload staging is finished, so retaining capacity has no purpose here.

Use `clear()` for bounded scratch space that will be filled again. It keeps capacity. Drop or replace a vector when the owning scene or operation ends. Do not call `shrink_to_fit` every frame: that turns reuse into allocator work.

For a graph budget, check vertices against the remaining capacity **before** subtracting them to check edges. Saturating subtraction alone does not reject too many vertices when the edge count is zero. If indexing cannot finish a document, omit its entire index and report the limit; a partial group must never pretend to cover all descendants.

## Step 4 · Change the teaching source once

- Explain the principle once, then link later lessons to it.
- Use the existing **You are building → Starting point → Steps → Check → What changed → Try → Questions and answers** structure.
- A new lesson needs the complete wiring, its lifecycle cleanup and an observable check. “Copy the relevant methods” is an orientation guide, not a replayable lesson.
- Frozen lesson edits must update their reconstruction patches and pass `course_pages.py --audit`. A production-only improvement belongs in a clearly labelled supplement until its lesson is regenerated.
- Keep limitations beside commands: supported geometry, coordinate units, selection scope and memory limits.

## Check

```sh
python3 docs/course_pages.py --audit
python3 docs/check_citations.py
cargo xtest -j4 --lib
cargo clippy -j4 --lib -- -D warnings
docs/serve.sh build --quiet
python3 docs/check_site.py
```

The replay audit checks every numbered lesson's coverage and recorded source hashes. It does not run the browser. Native tests do not prove browser interaction; browser checks need a usable WebGPU adapter.

## What changed

Review scope: course structure and runtime source, with focused review of ownership, upload release, frame preparation, editing, selection and panels. The integrated lesson replay matches all 114 maintained runtime/build files. This is not a proof of every shader or geometry algorithm.

Applied: one-pass bucket counts, borrowed hierarchy lookups, one visibility writer, corrected graph capacity checks, and direct staging-vector release. These remove identifiable loops or allocations; no frame-rate improvement is claimed without a benchmark.

Further work needs separate measurements: batch large flag updates into GPU writes and bound retained undo snapshots. The optional editing lessons are now individually replayable. Keep the existing lane and checkpoint machinery rather than introducing another framework.

## Try

On a large local scene, open the panel and repeatedly hide/show a group. Index storage should stay stable until the scene structure changes. Compare a document with many small groups against one flat group; total bucket-counting work should depend on object count, not object count multiplied by group count.

## Questions and answers

**Why keep explicit loops?** Their bounds and allocations are visible. Use an iterator when it avoids a temporary collection or expresses the same operation more clearly.

**Why not rewrite every old lesson?** Its checkpoint is reproducible. Change a lesson when teaching or measured behavior improves, then regenerate and verify that checkpoint.

**Does a memory limit prevent every crash?** No. It bounds the resource it names. Source geometry, history, GPU allocations and browser overhead need separate accounting.
