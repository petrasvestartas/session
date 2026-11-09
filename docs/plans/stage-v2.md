# Stage v2: mathematical slides in the session viewer

Redo of the first stage attempt (kept on the local branch `backup/stage-v1`). Written 2026-11-10.

## What was asked

1. Left: a 3D orthographic view. Right: the mathematics. Matrices are rounded squares.
2. From slide to slide the 3D view stays; only notation changes, or the view is transformed
   (for example a vector is taken apart into its x, y, z cosine projections).
3. Smooth, minimal, step by step: derivation and visual explanation together.
4. The example exists to extend the viewer **as the viewer it is**, plus transformations, morphs
   and well-set equations. Objects added to the viewer align cleanly and never overlap.
5. Study manim first (`C:\brg\manim`).
6. Kernel functions of general use (morphs, transforms, points, lines, polylines) may go into
   session_rust / session_py / session_cpp. Everything else lives in session_viewer.
7. Content, split into chapters so no animation runs long:
   - level 1: plug in fixed coordinates, connectivity, force densities; get the free coordinates;
   - level 2: first principles of the five nodes;
   - level 3: the specific matrices;
   - a **separate deck**: the Jacobian and the matrix equation, following the note screenshots
     (`C:\brg\FD Explanations`) and the study PDF, down to the final résumé.
8. Decks are session_viewer example files, YAML only "as with static scenes", pushed to Cloudflare.
9. Push the whole stack with `push_mono.sh`, resolving conflicts.

## What v1 got wrong

| Requirement | v1 | v2 |
|---|---|---|
| viewer as it is | 3D drawn by an egui overlay with a private camera | kernel geometry in the real scene, real camera and orbit |
| kernel functions | none | `lerp`/`morph` family in all three kernels, same tests |
| separate deck | one deck, seven chapters | `force-density` (levels 1-3) and `force-density-derivations` |
| as static scenes | new dialect | the scene manifest, extended: `name`, `items`, `texts`, `view`, `grid` keep their meaning |
| manim | skimmed | timing, Write, matching and layout follow the studied formulas (below) |

What v1 got right and v2 keeps as an idea, rewritten: TeX laid out from the OpenType MATH table of
New Computer Modern; glyphs from a swash atlas through the egui painter; a native test that plays
every step of every deck and fails on overlaps.

## Findings that fix the design

**Viewer** (study of the scene pipeline):
- `gpu.set_placement(row, &Xform)`: one 128-byte write, no re-walk. Moves an object per frame.
- `scene.preview_geometry(row, geom, gpu)`: re-walks one object. Re-shapes a line per frame.
- `gpu.set_reveal(row, trim, scale)`: ribbons end at `trim`, arrows shorten, dots scale.
- `gpu.set_hidden`, `set_object_color`, `set_gradient`, `set_head_scale`.
- `FLAG_SPATIAL` + `set_spatial_view({view_proj, split_x})`: own projection and a clip at the
  split for ribbons, arrows and dots. Faces are not clipped (not needed for line diagrams).
- Camera fields are public: `target`, `distance`, `orientation` (quaternion), `perspective`.
- No per-row alpha. A fade is a colour lerp to the background plus hide at the end.
- Kernel `Text` is not displayed by `add_file`; glyphon labels cannot draw math-font glyph ids.
  So labels and all mathematics are drawn by the stage painter at positions projected with the
  real camera matrix.
- `BEFORE_PICKS` hook list is where a per-frame tick belongs.

**Kernel**: has `Point::lerp`, `Quaternion::slerp`, `Polyline::tween_two_polylines` (equal counts).
Missing and added by this work, C++ first, then Python and Rust, one mini test each:
- `Vector::lerp(a, b, t)`, `Color::lerp(a, b, t)`
- `Line::lerp(a, b, t)`
- `Polyline::resampled(n)` (n points at equal arc length) and `Polyline::morph(a, b, t)`
  (unequal counts are resampled to the larger one first)
- `Xform::interpolate(a, b, t)` (translation and scale linear, rotation by quaternion slerp)

**Manim** (formulas the stage copies):
- `smooth(t) = t^3 (10 s^2 + 5 s t + t^2)`, `s = 1 - t`; `there_and_back`; `double_smooth`.
- lagged parts: `sub = clip(alpha * ((N-1) lag + 1) - i lag, 0, 1)`.
- Write: 1 s under 15 glyphs else 2 s, lag `min(4/(n+1), 0.2)`, each glyph outline-then-fill;
  here each glyph is wiped in left to right, then settles to full ink.
- Matching transform: mapped parts, then repeated longest common run, leftovers fade out toward
  the target centre and in from the source centre, scaled from 0.
- Indicate: scale 1.2, accent colour, there-and-back. Frame: a rounded rectangle drawn round.
- Layout buffers as fractions of the slide height: 0.0125, 0.031, 0.0625, 0.125.
- Camera moves interpolate the orientation quaternion, target and size.

## Architecture

```
YAML deck ──parse──> Deck (slides, steps, objects)
                │
                ├─ 3D objects ──> session_rust::Session (Point, Line, Polyline) ──> Scene rows
                │                   animated with set_placement / preview_geometry / set_reveal,
                │                   geometry in between computed by the kernel lerp/morph functions
                │
                └─ mathematics ─> tex layout ──> Column layout ──> stage painter (egui)
                                   labels on 3D objects use the camera's projection
```

Modules in `session_viewer/src/app/stage/`:

| file | job |
|---|---|
| `font.rs` | faces, MATH constants, glyph metrics, stretch constructions |
| `tex.rs` | lexer, parser, TeX layout to glyphs and rules; parts `{{…}}`; prose with `$…$` |
| `deck.rs` | YAML schema and validation; clear author errors with slide/step numbers |
| `expr.rs` | positions and live values: `mid(A,B)`, `A + [..]`, `len(A,B)` |
| `spatial.rs` | builds the kernel Session, owns scene rows, camera poses, per-frame geometry |
| `column.rs` | the right-hand column: stacking, equals alignment, fitting, relayout |
| `motion.rs` | clock, easing, scheduled actions, matching transforms |
| `stage.rs` | state at a step, navigation, replay for stepping back |
| `paint.rs` | glyph atlas and egui drawing of formulas, cells, frames and 3D labels |
| `tests.rs` | layout facts, parser facts, and every step of every example deck |

## YAML format (extends the scene manifest)

```yaml
name: Force density, one solve          # as in a scene
view: iso                               # as in a scene; or {yaw, pitch}
grid: true                              # as in a scene
items: []                               # as in a scene: optional .pb geometry behind the slides
palette: {cable: '#c9ced6'}
points: {P0: [3, 3, 0.25], P1: [0, 0, 0]}
slides:
  - title: What we know
    steps:
      - say: 'Four anchors are fixed.'
        do:
          - add: {id: n1, point: P1, color: fixed, label: 'P_1'}
          - add: {id: eq, tex: 'x_N = D_N^{-1}(p_x - D_F x_F)', align: '='}
          - morph: {target: eq, tex: '…'}
```

A deck folder holds `index.yaml` (`name`, `chapters`) and one file per chapter.
`?stage=<deck>` reads `stage/<deck>/index.yaml` from the data bucket.

## Decks

`examples/stage/force-density/` — the method in three levels
1. `01-one-solve.yaml` inputs plugged in, one linear solve, the node moves to equilibrium
2. `02-five-nodes.yaml` forces, direction cosines as projections, force density, linearity
3. `03-matrices.yaml` C, u = Cx, C_N | C_F, U, L, Q, D_N, D_F, the solve in matrices
4. `04-several-nodes.yaml` the 5x3 example of the study PDF: Q C_f, D_f, its structure

`examples/stage/force-density-derivations/` — the note screenshots
1. `01-setup.yaml` one bar, Pythagoras, the question, the nudge that converges
2. `02-tiny-nudge.yaml` method 1, steps A to G
3. `03-chain-rule.yaml` method 2; the meaning as a negative direction cosine; +0.706
4. `04-jacobian.yaml` (6.22)-(6.24): the row for four bars, C_N^T U L^-1
5. `05-matrix-form.yaml` (6.25)-(6.33): equilibrium, q = L^-1 f, Uq = Qu, split, D_N, D_F, solve
6. `06-materialization.yaml` (6.34)-(6.37) and the worked cable
7. `07-resume.yaml` the whole chain on one slide

Sign convention throughout: `u = Cx = x_0 - x_k`, so equilibrium reads `C_N^T U L^-1 f = p` and
(6.31) is as printed; one line on the slide says the book prints `+ p = 0` with the other sign of u.

## Order of work

1. Kernel: the five functions in C++, Python, Rust with tests; `quicktest` each class green.
2. `font.rs`, `tex.rs` with the native PNG harness; compare against the note screenshots.
3. `deck.rs`, `expr.rs`, `column.rs`, `motion.rs`, `stage.rs` with native tests (no GPU needed).
4. `spatial.rs`: Session from YAML, rows, reveal, placement, camera, split clip. Browser check.
5. `paint.rs` and the event wiring; remove the v1 module and the protobuf presentation runtime
   that the stage replaces only if nothing else uses it (the Teaching site has its own copy).
6. Decks, chapter by chapter, each checked in the browser at 1600x900 and DPR 2.
7. README for authors; upload both decks to R2; commit; `push_mono`; CI green.

## Checks

- `cargo test --lib stage::` plays every step of both decks: no author errors, no overlapping
  blocks or labels, nothing outside the slide, every animation settles.
- Browser: Playwright run per chapter, forward, back, chapter jumps, orbit; screenshots reviewed.
- Kernel: `quicktest` for vector, color, line, polyline, xform in cpp, py, rust.
