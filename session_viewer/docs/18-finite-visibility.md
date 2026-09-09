# 18 · Test finite triangles, then converge on production

**Start:** checkpoint 17. **Finish:** teapot concavities and touching-solid edges use finite occluders, with bounded GPU storage and correct cache invalidation. This is the current production runtime.

## Reproduce the exact failure

Two faces meet along `x = z = 0`. A narrow neighboring strip lies at `z = 0.2`, with x spanning `[-4, -1]`. In the fixture's perspective camera, the seam ray reaches that plane around `x = -0.181`: outside the actual strip.

The old visibility test transferred the strip's plane depth to the seam axis and rejected visible ink. It kept only 513 of 766 reference core pixels. The finite test keeps all 766. A wider strip that really covers the seam must still produce zero hidden black pixels.

![The depth plane continues beyond the finite triangle; only a finite nearer hit can hide the axis.](illustrations/finite-triangle.svg)

This is independent of CAD meshing quality. Shared curve/face samples repaired a geometry mismatch earlier. The remaining error came from the meaning of the screen-space occlusion test.

## Keep the inexpensive test, refine its rejection

`ink_visible` first runs the existing physical depth/gradient rule. When that rule accepts the ink, no triangle-list search is needed. When it rejects, a finite nearer hit is required before treating a triangle plane as an occluder.

The physical metadata now carries the exact winning primitive identity. Test that triangle at the stroke axis, then nearby sample-matched witnesses. Any finite nearer hit confirms real occlusion immediately.

Those nearby witnesses are not a complete candidate set. A thin triangle can intersect the axis while winning none of the sampled depth locations. Accepting ink merely because the few sampled owners miss it leaked hidden floor lines. The fallback therefore includes **all triangles intersecting the axis's screen tile**.

## Project each triangle once per changed view

`project_triangles.wgsl` reads the exact uploaded vertex/index/object columns, applies the same model/rebased translation and camera transform as drawing, and clips against the near plane before dividing by `w`.

A clipped triangle has zero, three or four corners. The shader computes inward edge equations, a reference screen position/depth, an affine depth gradient and screen bounds. `ProjectedTriangle` stores six `vec4<f32>` values: **96 bytes**. The Rust/WGSL layout assertions check this stride and all offsets.

For a test point, evaluate the inward edge equations. A point outside any edge is outside the finite polygon. For an inside point, evaluate depth from the reference and gradient. Under reversed depth, a sufficiently larger depth is nearer. The small relative tolerance handles floating-point precision; it is not a world-space extrusion of the edge.

## Build a compact screen index

The CPU `TileLayout` and WGSL `visibility_tile_span` start with four framebuffer pixels per tile. They double the span until there are at most 262,144 tiles. At high resolution the grid grows coarser instead of allocating an unbounded number of headers.

```text
projected triangles
        ↓ conservative bounding quads + edge/box overlap test
count references per tile
        ↓ parallel prefix sums
assign contiguous ranges in a shared pool
        ↓ same conservative rasterization
fill (primitive ID, maximum possible depth) pairs
        ↓
ink query checks only its tile's finite candidates
```

Counting and filling use the same coverage rule. Prefix sums convert counts into offsets without a fixed capacity for each tile. The pool budgets 32 references per viewport tile overall; a dense tile can borrow unused space elsewhere.

Each reference also stores a conservative maximum depth for that triangle in the tile. A triangle that cannot be nearer than the line is skipped before the more expensive finite test. This is a safe rejection bound, not a replacement for checking containment.

## Handle limits without leaking hidden geometry

The tile header stores count, offset, fill cursor and overflow. Before trusting a list, require no overflow and `cursor == count`. Prefix sums saturate instead of wrapping; filling cannot write beyond the counted range.

If projected storage exceeds the device binding limit, or a tile list is incomplete/oversubscribed, retain the physical depth rejection. This favors missing ink over showing geometry through a solid. The algorithm is bounded and does not promise unlimited exact visibility for arbitrary overdraw.

Non-triangle physical geometry writes zero primitive identity and keeps its conservative visibility rule. Metadata uses `Rgba16Float`: xy holds the gradient; zw encodes an exact primitive ID using core WGSL packing operations. The ID pass owns matching single-sample depth/metadata, so it does not borrow incompatible display MSAA sample positions.

## Own the cache and its invalidation

`TriangleTiles` owns projected records, tile storage, the small raster target and immutable pipelines. It is called explicitly before ink rendering. A key tracks the camera matrix and geometry/visibility revision.

| Event | Rebuild projection/binning? |
|---|---|
| Same camera and same geometry | No |
| Select or recolor an object | No |
| Hide/show, placement, rebase or geometry replacement | Yes |
| Camera or relevant framebuffer/grid change | Yes |
| Replace geometry with the same triangle count | Yes |

Changing the buffer allocation rebinds readers. Scene release removes the large storage/texture and leaves 128 bytes of placeholders. An ID-only request can prepare visibility even when no color frame is submitted.

The final State split moves unchanged streamed-query coordination into `state/cloud_query.rs`. It gives that asynchronous workflow one readable location while preserving State's ownership. The source text companion remains `state/text.rs`.

## Finish the presentation defaults

Set the default `thickness_px` in `engine/gpu/view.rs` to `1.0`. This controls screen-sized mesh/BRep/NURBS edges, lines and polylines. Explicit authored world-space widths keep their dimensions. The black silhouette radii are separate and stay unchanged.

Selected source text now uses a fully yellow backing and black letters. `TextLabel::ink_color()` derives black ink from the source selection flag without modifying the authored color or reshaping the text. Glyphon and the fixed-plane renderer both read it. `text_plate.wgsl` colors the whole rounded plate yellow; `text_plane.wgsl` blends yellow backing with black ink using the existing glyph coverage texture. Neither path adds a yellow border. Picking, depth and clipping keep the same geometry.

Deselecting restores the original text colors. Every scene text backing now has maximum rounded corners, matching the selected-object name. Horizontal padding reserves a complete cap outside each end of the shaped line; vertical padding scales with the text size. The fixed-plane shader evaluates a rounded rectangle in perspective-correct texture coordinates and uses that same shape for ID picking. Transparent corners therefore remain unpickable, and the antialiasing width follows the projected shape.

Derived selected-object names have no source owner and remain white on black. The updated native text tests check actual yellow backing/black glyph pixels and exact color restoration; the browser text tests exercise both orientations at DPR 1 and 2.

## Write the files

Follow [Complete file changes for 18](../lessons/18/index.md). Read the projected record and finite test, projection shader, count/scan/fill stages, Rust resource owner, then frame/binding integration. Copy the maintained counterexample generator and checker.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Inspect the local scene at <http://localhost:8780/?data=off&inspect=1>. Use the supplied teapot and selected-solid tests to inspect perspective/orthographic seams, concave foot boundaries and black selection silhouettes. A hidden edge should stay hidden; a visible seam should not acquire broken intervals as a neighboring face moves over its stroke fringe.

Stop Trunk and run the exact counterexample:

```sh
cargo build --locked --target x86_64-unknown-linux-gnu --example selftest --example mk_triangle_visibility
python3 tests/triangle-visibility.py
```

Expected output includes **766/766 visible core samples** and **0 covered black pixels**. This exact pixel oracle explicitly uses its original 1.5-pixel pen, independently of the viewer's new 1-pixel default. It disables ordinary silhouettes so a legitimate solid border cannot be counted as leaked hidden source ink. The separate floor and selected-overlap checks retain ordinary silhouettes.

Run the final correctness gates:

```sh
cargo fmt --package session_viewer -- --check
cargo clippy --locked --target wasm32-unknown-unknown --lib -- -D warnings
cargo clippy --locked --target x86_64-unknown-linux-gnu --all-targets -- -D warnings
cargo test --locked --target x86_64-unknown-linux-gnu -- --include-ignored --test-threads=1
```

The recorded native suite passes 89 tests including GPU tests. The image suites cover 54 hidden-line cases, 21 floor views, 40 stroke checks and 40 selected-overlap checks; the high-resolution 2800×1800 four-sample floor case also passes. [Measurements](measurements.md) explains evidence and platform limits.

## Verify that you reached the current viewer

After reproducing the complete files exactly, record and compare the final source:

```sh
python3 "$COURSE_REPO/docs/reconstruction/replay.py" --output "$COURSE_WORK" --through 18 --adopt
python3 "$COURSE_REPO/docs/reconstruction/converge.py" --workspace "$COURSE_WORK"
```

The checker requires all **91 runtime files**, rejects extra/missing implementations, and compares the frozen file hashes. The verified reconstruction has **299 identical frozen files** overall. Packaging differences are limited to the documented local input fixture and imported-document font artifacts; the renderer and application source match production.

To rebuild and browser-check every stage independently from original inputs, use a new output path:

```sh
python3 "$COURSE_REPO/docs/reconstruction/replay.py" --output "$HOME/viewer-course-clean" --through 18 --verify-clean
```

The maintained [verification record](reconstruction/verification.json) keeps historical 00–16 evidence separate from the fresh 17/18 reconstructions. This is a source/build/browser claim, not a promise that every possible model and GPU has been tested.

For future changes, use [Architecture](../ARCHITECTURE.md) to locate the owner and [Coverage](coverage.md) to find the relevant regression. Keep the source → display → GPU boundary explicit when experimenting with a different implementation.

## Repository cleanup

The obsolete selection-outline implementation and the one-off teapot investigation example are removed. Maintained depth, stroke, selection and text regressions remain, along with fonts, licenses, source fixtures and reconstruction inputs. The old `docs_archive` was removed at the user's request; `session_viewer_archive` remains the separate historical reference. Generated site/listing/build output stays under ignored `target`, and the final source inventory is checked again after cleanup.
