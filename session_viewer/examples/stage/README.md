# Stage decks: mathematical slides written in YAML

A stage deck is a folder of YAML files and nothing else. The viewer reads it, typesets the TeX
with its own OpenType-MATH layout (New Computer Modern), and animates every step. Python,
protobuf files, MathJax and SVG are not involved.

```text
https://petrasvestartas.github.io/session/?stage=force-density              published deck (R2: stage/force-density/index.yaml)
http://127.0.0.1:8080/?stage=force-density/index.yaml&data=http://127.0.0.1:9000/  a deck served from a local folder
&chapter=5&step=3                                                                  open at chapter 5, after its third step
```

The slide is 1600 × 900. The left side is an orthographic 3D view that stays the same from slide
to slide; the right side is a column of formulas, matrices and text. Blocks in the column are laid
out automatically from the top. Equations marked `align: '='` share one column for their equals
signs. Blocks wider than the column shrink to fit, and blocks move smoothly when others are added
or removed, so nothing overlaps. Narration (`say`) sits under the column, and the slide title sits
above it.

Keys: **→ / Space / PageDown** next step (a running step first finishes); **←** back;
**↓ / ↑** next and previous slide; **1–9** jump to a chapter; **R** resets the view. Drag the 3D
side to orbit, use the wheel to zoom, and double-click to reset. Click the column to advance or a
numbered circle to change chapter.

## Files

```yaml
# index.yaml
title: Force density method, step by step
chapters: [01-one-solve.yaml, 02-one-node.yaml]
```

```yaml
# 01-one-solve.yaml: one chapter, with one 3D scene shared by all of its slides
title: One linear solve
grid: {center: [3.5, 3.5], size: 9, step: 1}   # optional floor grid
axes: false                                     # optional x, y, z arrows at the origin
camera: {yaw: -58, pitch: 26}                   # degrees; fits all points unless zoom/fit is given
split: 0.46                                     # width share of the 3D side
palette: {cable: '#c9ced6'}                     # extra or replaced colours
points:                                         # named positions; `move` animates them
  P0: [3, 3, 0.25]
  P1: [0, 0, 0]
slides:
  - title: What we know
    keep: false                                 # true keeps the previous slide's formulas
    steps:
      - say: 'Narration with inline $\mathbf{x}_F$ and **bold**.'
        do: [ ...actions, played one after another... ]
```

Write TeX in **single quotes**: YAML then leaves backslashes alone (`'\frac{a}{b}'`).

Default colours: `ink dim faint free fixed load x y z accent bar pos neg bg`. Any `#rrggbb`
also works.

## Objects (`add:`)

`add` takes one object or a list of objects. Every object can have an `id`, used by later actions.

| 3D, drawn on the left | |
|---|---|
| `point: P0` | a disc; `label: 'P_0'` attaches a label placed clear of lines and other labels |
| `line: [P0, P1]`, `arrow: [P0, P1]` | `width`, `dash: true`, `label` at the midpoint |
| `polyline: [A, B, C]`, `polygon: [...]` | open or closed |
| `cuboid: [P0, P1]` | the dashed box spanned by two corners |
| `label: P1` + `tex: '(0,\,0,\,0)'` | a formula at a 3D position; `dir: [0, -1]` forces a side |

A position is a point name, `[x, y, z]`, or an expression:
`'P0 + [0, 0, -2]'`, `'mid(P0, P1)'`, `'lerp(P1, P0, 0.9)'`, `'P0 + 0.5*(P1 - P0)'`,
`'[x(P1), y(P0), z(P0)]'`, and `len(A, B)`, `unit(v)`, `norm(v)`. Positions are evaluated
every frame, so geometry follows a point while `move` animates it.

| 2D, stacked on the right | |
|---|---|
| `tex: 'l_k = \sqrt{\cdots}'` | display-style formula; `align: '='` joins the equals column |
| `text: 'Prose with $math$ and **bold**.'` | wrapped to the column |
| `matrix: {rows: [[1, -1], [0, 1]]}` | rounded square cells; sign colours (`pos`/`neg`), zeros faint |
| `row: ['\mathbf{u} =', {id: C, matrix: ...}, {id: x, matrix: ...}]` | formulas and matrices on one axis |

Matrix options: `name` (TeX under it), `rowlabels`, `collabels`, `empty: true` (cells without
values, filled later by `show`, `fly` or `set`), `signs: false`, `cell` (size in em), and `colors`
(per-cell palette names). An entry may be a number or TeX (`'\dfrac{-(x_1-x_0)}{l_a}'`).

Common options: `color`, `size` (px: formulas 34, matrices 30, text 25, labels 24), `opacity`,
`gap` (px above a block), `at: [x, y]` (fixed slide position instead of the column), and
`anim: write | fade | grow | pop | none`.

A live value: `tex: 'l_k = {=len(X, P1):.3}'` is recomputed every frame.

## Parts and references

Wrap a sub-formula in double braces to make it a part: `'\sqrt{ {{(x_1 - x_0)^2}} + \cdots}'`.
Parts are matched first when one formula morphs into another, and they can be targeted. References
used by actions:

| reference | means |
|---|---|
| `eq` | the whole object |
| `eq{(x_1 - x_0)^2}` | a part, written as its TeX |
| `M[1,2]`, `M[1,:]`, `M[:,0]` | a matrix entry, row or column, also for matrices inside a `row` |
| `n0.label` | the label attached to object `n0` |

## Actions (`do:`)

Actions run one after the other. `with: true` starts an action together with the previous one,
`delay: 0.3` waits (seconds) and `dur: 1.5` sets the duration. These keys may sit beside the
action key or inside its map.

| action | effect |
|---|---|
| `add: {...}` | create, written glyph by glyph (3D lines grow, points pop) |
| `remove: [id, ...]` | fade out; the column closes the gap |
| `morph: {target: eq, tex: '...'}` | transform in place: equal glyphs and parts travel, the rest fades; any content key (`row`, `matrix`, `arrow`, ...) works |
| `derive: {from: eq, id: eq2, tex: '...'}` | a new object built from a copy of another; with 3D geometry it moves from the source shape (a projection) |
| `fly: {from: label, to: 'M[0,:]', keep: false}` | copies matching glyphs into hidden cells and reveals them |
| `set: {target: 'M[0,1]', tex: '4'}` | change one entry, morphing |
| `show` / `hide: ['M[2,:]']` | reveal or hide entries, rows or parts |
| `indicate: [ref, ...]` | pulse and tint briefly |
| `box: {target: ref, color: accent}` | a rounded frame that follows the target; its id is `box:<ref>` |
| `color: {target: ref, color: x}` | recolour; `color: reset` restores |
| `dim` / `undim: [id, ...]` | fade to the background and back |
| `move: {point: P0, to: [3, 3, 0.25]}` | animate a named point |
| `camera: {view: front}` or `{yaw, pitch, zoom, fit: [P0, P1]}` | views: `front back top side left iso` |
| `wait: 1.0`, `clear` | pause; clear the column |
| `play: [action, action]` | run actions together |

`map` on `morph`/`derive` pairs pieces explicitly; keys and values are parts or references, for
example when a matrix splits into blocks:
`map: {'[:,0]': 'CN[:,0]', '[:,1]': 'CF[:,0]'}`.

Mistakes (unknown TeX commands, missing objects) are listed in red at the top of the slide.

## Check and publish

```bash
cd session/session_viewer
cargo test --target x86_64-pc-windows-msvc --lib stage::     # every step of examples/stage/force-density: no errors, no overlaps
bash ../../wood/tools/buildslot.sh stage env CARGO_BUILD_JOBS=8 trunk build
aws s3 cp examples/stage/force-density s3://session-viewer-data/stage/force-density/ --recursive \
  --exclude '*' --include '*.yaml' --profile r2 \
  --endpoint-url https://0520459c6817bd96c1e25fcb49461c4e.r2.cloudflarestorage.com
```

The runtime lives in `src/app/stage/`: `tex.rs` (TeX parser and MATH-table layout), `font.rs`,
`deck.rs` (YAML), `eval.rs` (positions), `run.rs` (objects, layout, actions, morph matching)
and `paint.rs` (glyph atlas and egui drawing).
