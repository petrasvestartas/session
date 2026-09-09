# 17 · Select source faces and text; draw one clean silhouette

**Start:** checkpoint 16. **Finish:** original face selection, selectable text in both orientations, padded white-on-black names, joined strokes and consistent black solid outlines.

## Extend source identity before adding visual effects

The selection model gains a source-face variant alongside objects, edges and controls. `FaceSource { parent, face }` maps each rendered triangle to its original polygon/BRep/surface face. `FACE_TAG` distinguishes that result in the integer ID path. The mapping survives triangulation and instance transforms.

Ctrl+Shift requests eligible faces as well as edges. A nearby source edge keeps precedence; an interior click selects the source face. Highlight only triangles belonging to that face. Do not choose an arbitrary triangulation triangle and call it a CAD face.

## Authored text is an object

`app/scene_text.rs` owns stable source rows for manifest text and document titles. Camera-facing text and fixed-plane text follow the same select/hide/show lifecycle. Their different orientation affects placement, not identity.

`state/text.rs` turns visible source rows into labels and derives the centered selection name. That derived name has no source row, so it cannot steal the parent's click. `T` changes only derived selected-object names; an authored label remains selectable even when names are hidden.

White glyphs remain white when a text object is selected. The selected object's name uses 13.5 CSS-pixel text, black backing, generous horizontal padding and maximum rounded corners. Padding belongs to the shaped line box; it must include the rounded ends without clipping the first or last glyph.

## Two masks, one black result

Ordinary outlines describe a union of opaque solid coverage. Touching/overlapping solids should not receive an extra per-object silhouette seam. Source mesh/BRep edges remain a separate geometry display.

The selected mask supplies the wider selected contour. The compositor takes the maximum ordinary/selected coverage instead of blending black twice. Selected interiors suppress the ordinary contour. This avoids duplicate darkness and uneven borders.

![The selected solid's yellow strokes are below the black silhouette; standalone selected curves are above it.](illustrations/frame.svg)

The order is essential:

1. Draw ordinary ink and selected solid boundary strokes.
2. Composite the single black silhouette.
3. Draw selected standalone curves over coincident mesh ink.

The first ordering keeps yellow cone/cylinder strokes from erasing their black border. The second keeps a selected polyline visible when it coincides with a mesh boundary. Physical depth still hides a truly covered span.

The ordinary radius is 2.25 CSS pixels and selected is 3.375 CSS pixels. Physical mask resolution follows DPR. Multisample coverage and a smooth one-pixel transition avoid a hard stair-step fringe. `O` toggles the silhouette effect independently of source-edge display.

## A curve's subdivisions share one join

A rendered polyline is a chain of segments. If each segment independently rounds its cap or computes a slightly different join plane, shared vertices can become darker or develop tiny gaps.

The source producer marks explicit chain ranges. The GPU stroke record stores previous/next neighbors within that chain. At a shared vertex, both segments use the **same ordered pair** in `join_plane(before, after)`. One side includes the boundary sample and the other excludes it. Exactly one segment owns the shared sample.

Do not join unrelated edges merely because endpoints coincide. The explicit source chain distinguishes an authored curve from touching geometry. Exact closed endpoints wrap to the first/last segment. A circle is constructed as the exact rational circle primitive; adding a mismatched closing segment would create a kink instead.

## Write the files

Follow [Complete file changes for 17](../lessons/17/index.md). Read face/source-text records first, then their picking/highlight paths. Read `surface_outline.rs` and its shader beside `render.rs`; the draw order is as important as mask creation. Finally inspect `SegRows` chain ranges and the shared WGSL join.

The old `selection_outline` implementation is explicitly removed. The new surface-outline owner serves ordinary and selected coverage; do not keep both implementations active.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

In the local fixture, Ctrl+Shift-click a mesh/BRep/surface interior and verify a source face highlights. Click an edge with the same modifiers and verify edge precedence. Select a solid and orbit: the black outer border should remain continuous with uniform apparent width. Press O twice to hide and restore silhouettes.

Run the browser checks against this lesson server:

```sh
export VIEWER_URL=http://localhost:8780/
export VIEWER_INTERACTION_FIXTURE="$COURSE_WORK/interaction.pb"
cargo run --locked --target x86_64-unknown-linux-gnu --example interaction_fixture -- "$VIEWER_INTERACTION_FIXTURE"
node tests/interaction.cjs
node tests/scene-text.cjs
node tests/world-text.cjs
```

These scripts supply local manifests/fixtures and exercise real clicks at DPR 1 and 2. They check selectable/hideable text in both orientations, white glyphs and T independence. The course setup supplies `NODE_PATH` and browser settings.

After stopping Trunk, build and run the native stroke/overlap fixtures:

```sh
cargo build --locked --target x86_64-unknown-linux-gnu --example selftest --example mk_stroke_joins --example mk_selection_overlap
python3 tests/stroke-joins.py
python3 tests/selection-overlap.py
```

The fixtures compare sparse/dense representations and overlapping source geometry. They check core retention and genuinely hidden spans separately. The concave/touching finite-occluder case is still the next lesson's work; increasing stroke subdivision alone does not solve it.

Continue to [finite triangle visibility and final convergence](18-finite-visibility.md).
