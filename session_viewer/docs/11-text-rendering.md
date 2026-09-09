# 11 · Render readable text at the right size

**Start:** checkpoint 10. **Finish:** shaped text draws with correct coverage, white-on-black backing, placement and resource lifetime.

## Three decisions happen after shaping

```text
shaped glyphs → place the line → choose physical raster size → coverage atlas
                        ↓                                  ↓
                    black plate                    dedicated text drawing
                        └────────── composite into the frame ──────────┘
```

Placement determines where the line belongs. Raster size determines how much physical detail its glyphs need. Compositing determines how coverage blends with the black backing and scene. Do not solve a placement or blending bug by changing the original glyph advances.

`TextFrame` carries logical/physical viewport information and camera inputs. Apply CSS-to-framebuffer scale once. A label at 14 CSS pixels should remain the same apparent size on a DPR 2 display while receiving roughly twice the raster resolution.

## Placement is explicit

| Placement | What remains fixed |
|---|---|
| Screen | A screen-space position and CSS size |
| Anchor / Nameplate | A world anchor with screen-oriented presentation |
| WorldBillboard | World location and height, with orientation following the camera |
| WorldPlane | World location, right/up axes and height |

A world-plane label foreshortens as you orbit. A billboard turns to face the camera. The original label record keeps those intentions separate; the camera transform does not mutate source text.

A selected-object name is later placed at the object's bounds center. That center can be inside a solid, so this derived annotation uses its explicit overlay placement. Authored world text retains its own depth/identity behavior.

## Coverage, padding and rounded plates

The glyph shader samples actual glyph coverage and composites white letters. A black plate must extend beyond the first/last glyph, including enough horizontal padding for rounded ends. Changing selection must not recolor the glyph fill yellow.

An antialiased fringe is partial coverage, not a gray font color baked into the shape. Its blend mode must match whether color is straight or premultiplied alpha. Compare output against black and gray backgrounds to expose incorrect blending.

Imported PDF outline text has a separate path: its source already contains vector geometry. Preserve those outlines and source positions; do not guess a replacement string/font from the triangles.

## Keep caches bounded and owned

`TextLane::prepare` updates placement/raster data as needed. `reset` clears current content; `release` returns scene-sized resources. Repeated world-scale changes can generate many raster keys, so the integration evicts safely after its bounded budget without retaining stale draw instances or reshaping unchanged content.

Resource accounting reports exposed allocations. Glyphon's private GPU capacities are not measured VRAM and must not be reported as though they were directly counted.

## Write the files

Follow [Complete file changes for 11](../lessons/11/index.md). Read the placement records, frame-scale conversion, plate/plane shader inputs and TextLane prepare/draw/release sequence. The complete listings include the dedicated shader and library integration; no text implementation is deferred to a reference link.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1> and the included text-quality reference. Inspect 12/14/16/18/24 CSS-pixel text at normal viewing size. Change browser zoom and resize; letters should retain correct shapes, spacing and unclipped backing.

Orbit a fixed-plane label and a camera-facing label. Content should not reshape merely because the camera moved. Chapter 17 adds source-object identity to every authored orientation, so they can all be selected and hidden.

**Before continuing:** distinguish source text, shaped layout, raster cache and placed draw instances. Continue to [picking](12-picking.md).
