# Small code, bounded work

## You are building

Use the same design for each extension:

```text
input → named State action → source or view state → changed GPU rows → frame
```

A shorter function is useful when it also removes work or makes ownership clearer. Moving the same work into a generic framework is not a performance improvement.

## Starting point

Every lesson ends at a **checkpoint**, a complete crate under `docs/lessons/<id>/`; the code blocks are included from those crates. The course runs 00 to 37, from an empty project to the maintained viewer.

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
- A lesson edit changes the crate in `docs/lessons/<id>/` and the line ranges its page references. A production-only improvement belongs in a clearly labelled supplement until its lesson crate carries it.
- Keep limitations beside commands: supported geometry, coordinate units, selection scope and memory limits.

## Check

```sh
cargo xtest -j4 --lib
cargo clippy -j4 --lib -- -D warnings
docs/serve.sh build --quiet
```

`docs/serve.sh build` fails on a lesson snippet that points at a missing file; `cargo check` inside `docs/lessons/<id>/` proves that lesson compiles. Neither runs the browser. Native tests do not prove browser interaction; browser checks need a usable WebGPU adapter.

## What changed

Review scope: course structure and runtime source, with focused review of ownership, upload release, frame preparation, editing, selection and panels. The integrated lesson replay matches all 114 maintained runtime/build files. This is not a proof of every shader or geometry algorithm.

Applied: one-pass bucket counts, borrowed hierarchy lookups, one visibility writer, corrected graph capacity checks, and direct staging-vector release. These remove identifiable loops or allocations; no frame-rate improvement is claimed without a benchmark.

Further work needs separate measurements: batch large flag updates into GPU writes and bound retained undo snapshots. The optional editing lessons are now individually replayable. Keep the existing lane and checkpoint machinery rather than introducing another framework.

## Course site weight

Most of the course is highlighted code, so a page weighs what the markup around each token weighs. Each change below was measured on the built site, in the order listed:

| Change | Where | Before → after |
|---|---|---|
| No per-line anchors or line spans | `pymdownx.highlight` in `mkdocs.yml` | all pages 23.95 → 17.06 MB of HTML |
| Whole kernel files on their own pages | `docs/kernel/*.md` | `07-boundaries` 2.01 → 0.35 MB, search index 3.04 → 2.54 MB |
| `.w` and `.n` tokens unwrapped between tags | `docs/hooks/lean_html.py` | all pages 16.94 → 12.53 MB |
| PNG screenshots sized, lazy unless first with no code above | `docs/hooks/lean_html.py` | `command-line-walkthrough` fetches 450 → 133 KB of images before scrolling; lessons 22-32 no longer fetch their end-of-page screenshot (18-91 KB) |
| Build tools and sources not published | `exclude_docs` in `mkdocs.yml` | 226 fewer files, 687 → 461 |

With the CPU slowed 4x in Chrome, `12-picking` (47,534 → 23,848 elements) reaches DOMContentLoaded in 790 ms instead of 1,862 ms, and `07-boundaries` in 332 ms instead of 1,698 ms. The rendered pages stayed pixel-identical and the copy button copies the same text.

- A whole kernel file is a page under `docs/kernel/` with `search: exclude: true`, linked from the lesson; never include it inline.
- The hook unwraps `.n` only because Material paints it in the plain code colour: if `--md-code-hl-name-color` is ever themed, stop unwrapping `.n`. A token next to plain text keeps its span, because one merged text run moves the glyphs after it by 1/64 px.
- SVGs keep no size attribute and load eagerly: a size on a scaled SVG changes how Chrome rasterises it, and an unsized lazy image would move an anchor target.
- Measured and left out: `navigation.instant` (the largest pages show their heading later, and a local build downloads every stylesheet twice) and a Roboto preload (first paint 0.1-0.2 s later on a slow connection).

## Try

On a large local scene, open the panel and repeatedly hide/show a group. Index storage should stay stable until the scene structure changes. Compare a document with many small groups against one flat group; total bucket-counting work should depend on object count, not object count multiplied by group count.

## Questions and answers

**Why keep explicit loops?** Their bounds and allocations are visible. Use an iterator when it avoids a temporary collection or expresses the same operation more clearly.

**Why not rewrite every old lesson?** Its checkpoint is reproducible. Change a lesson when teaching or measured behavior improves, then regenerate and verify that checkpoint.

**Does a memory limit prevent every crash?** No. It bounds the resource it names. Source geometry, history, GPU allocations and browser overhead need separate accounting.

## Full-floor rotation and wheel zoom

Camera movement must not rebuild source geometry. The maintained viewer coalesces input while GPU work is pending, then draws the latest camera pose. Chrome and native builds request high-performance graphics by default. Firefox retains the lower-power preference: on the local Firefox 157 full-floor check, requesting high performance repeatedly lost the driver connection while the same navigation passed at lower power. `?gpu=low`, `?gpu=high` and `?gpu=default` override the preference; default leaves adapter choice to the browser. These are preferences, not a guarantee of a particular adapter. Type `Report` to inspect the actual adapter.

Wheel events now enter the same temporary navigation policy as orbit and pinch. Each nonzero wheel event extends a 200 ms burst. A wheel pose queued behind busy GPU work remains pending until a frame draws it, even if that interval expires. Slow frames can temporarily use plane-based ink visibility and omit outlines; after the burst ends the viewer redraws with exact visibility and returns to idle. Canvas resolution and MSAA remain unchanged. A window blur or lost-pointer event cancels the wheel burst.

A cheaper navigation frame does not prove that full detail is cheap. After changing tiers, the policy waits for one second of timed movement before testing a more expensive tier. Idle time does not count. This bounds repeated expensive retries while still allowing detail to recover when the view becomes cheaper; the final resting frame remains full quality.

An October 7 local native test on Intel RPL-S graphics rendered the 529-object timber-floor fixture at 1200×800, opacity 0.95, for 30 orbit frames: full-quality median 94.4 ms (11 fps), temporary tier 1 median 21.9 ms (46 fps). The ink pass dominated full-quality cost. These measurements describe this local fixture and hardware, not the user's Lenovo. The browser regression separately verifies real wheel input, learned temporary tiers, full-quality restoration and no idle loop; it deliberately advances the timing clock to exercise the policy and is not a speed benchmark.

The actual `wood/examples/templates_vault_4_dome.cpp` output is a second fixture: 800 voussoirs and 2320 contacts. The same local Intel adapter, dimensions, opacity and 30-frame orbit sweep produced these native readback measurements:

| Scene | Earlier full-quality baseline | Current automatic navigation |
| --- | ---: | ---: |
| Timber floor, 529 objects | 94.4 ms / 11 fps | 40.9 ms / 24 fps |
| Vault dome, 800 objects | 42.5 ms / 24 fps | 21.2 ms / 47 fps |

The dome entered tier 1 at frame 4, about 156 ms after its first slow frame. The floor reached tier 1 at frame 3 and tier 2 at frame 6. An earlier dome run without the retry interval repeatedly returned to exact visibility and had a 37.2 ms median despite 12.4 ms temporary frames. These separate runs show why cheap-frame timing alone should not immediately restore the expensive navigation path. They are local observations, include GPU readback and temporarily trade ink visibility detail for responsiveness; they do not establish Lenovo or browser frame rates. The actual Chrome checks verify queued input, full-quality restoration and idle for both fixtures.

The final default Firefox 157 configuration separately passes the same full-floor wheel and orbit checks, full-quality restoration and idle. As in Chrome’s policy regression, the test advances the timing clock to exercise temporary tiers; it does not measure Firefox frame rates.
