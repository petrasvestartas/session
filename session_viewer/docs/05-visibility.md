# 05 · Separate physical surfaces from readable ink

**Start:** checkpoint 04. **Finish:** hidden lines stay hidden while visible thick lines and close-up corners remain readable. Chapter 18 will refine the finite-triangle case that this first visibility model cannot resolve.

## A thick line is not its centerline

The mathematical line has zero width. The displayed stroke covers a strip of samples around that line. At a grazing angle, those samples can lie over triangles whose depth changes sharply across a pixel.

```text
physical pass: opaque surfaces → immutable depth + visibility metadata
                                                   ↓
ink pass: segment footprint → axis position → visibility test → coverage color
```

Drawing all lines with depth disabled makes rear edges visible through solids. Adding a large global depth offset can do the same and changes with scale/projection. This viewer instead keeps physical depth and asks whether the line axis is visible at the covered sample.

## Read the depth contract as one unit

The camera uses reversed depth: near is larger, far approaches zero. Opaque depth clears to zero and compares `Greater`. A shader's calculated depth must use the same projection. The physical attachment is read during ink rendering without allowing each decorative stroke to rewrite the scene's occluders.

The physical gradient describes how the winning primitive's depth changes across screen x/y. Transferring its depth to the line axis avoids comparing the line against the wrong offset location. Keep the bounded fallback for gradients that cannot be represented reliably.

At this stage, a neighboring triangle can still be interpreted as an infinite plane. Do not assume that passing a floor test proves every concave CAD edge correct. The final course step adds the finite footprint and all-candidate tile test while preserving this physical-depth foundation.

## Coverage and joins are separate from visibility

Visibility decides whether a sample belongs to a visible stroke. Coverage decides how much that stroke covers the sample and blends its antialiased fringe. Increasing the pen width or darkening overlap does not repair a wrong occlusion decision.

Likewise, independent segment caps can overlap or leave gaps where a curve was subdivided. Chapter 17 gives both incident segments the same join plane and complementary ownership at their common endpoint.

## Write the files

Follow [Complete file changes for 05](../lessons/05/index.md). Read `physical.wgsl`, `ink_visibility.wgsl` and the stroke shader beside the target formats and pass setup. Keep the color and ID visibility contracts aligned; a visible line that cannot be picked is still a defect.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open the local checkpoint at <http://localhost:8780/?data=off&inspect=1>. Inspect crossing lines and covered geometry while orbiting. Foreground ink should stay legible; genuinely covered spans should not shine through. Use the complete fixture and checks supplied with this stage rather than substituting an unrelated simple triangle.

The later maintained depth suite separately checks 54 synthetic views, the real floor census and a close-up box's nine visible edges/seven vertices. These are complementary checks: fixing a hidden floor edge must not erase a nearby box corner.

**Before continuing:** distinguish a visibility failure, a coverage failure and an incorrect source curve. Continue to [the CAD contract](06-cad-contract.md).
