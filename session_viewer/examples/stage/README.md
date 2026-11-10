# Stage decks: mathematical slides written in YAML

A deck is a folder of YAML files. The viewer draws the 3D side with kernel `Point`, `Line` and
`Polyline` geometry in its own scene, typesets the TeX itself, and animates every step.

```text
https://petrasvestartas.github.io/session/?stage=force-density                  1 · the equations of five nodes (6.2.2), no matrices
https://petrasvestartas.github.io/session/?stage=force-density-matrices         2 · matrices, the Jacobian, any net, its forces (6.4.2, 6.4.3)
https://petrasvestartas.github.io/session/?stage=force-density-materialization  3 · elongations and unstretched lengths (6.5), résumé
&chapter=7&step=3                                                           open at chapter 7, after its third step
```

Keys: **→ / Space** next step, **←** back, **↓ / ↑** next and previous slide, **1–9** chapter,
**R** reset view. Drag the 3D side to orbit, wheel to zoom.

On a phone: swipe left for the next step and right for the one before, tap to advance, hold and
drag the 3D side to orbit, double tap it to reset the view. Held upright, the 3D side fills the
top of the screen and the column sits below it.

## The shape of a deck

```text
force-density/
  index.yaml             name and the list of chapter files
  01-method.yaml         one chapter = one 3D scene + its slides
  02-equilibrium.yaml
  img/                   pictures the chapters show
```

A chapter is a list of slides, a slide is a list of steps, and a step is one sentence plus what
appears with it. The 3D scene stays while the slides of a chapter change the right-hand column.

```yaml
title: A pillow
net:                              # the structure, once per chapter
  grid: [10, 10]
  fixed: boundary
  q: 1
  load: [0, 0, 0.35]
slides:
  - title: Fixed boundary, loaded nodes
    steps:
      - say: 'All 36 boundary nodes are fixed.'
        show: [cables, fixed]
      - say: 'Every free node carries the same load.'
        show: loads
      - say: 'The unknowns: 64 nodes.'
        eq: '\mathbf{D}_N\,\mathbf{x}_N = \mathbf{p} - \mathbf{D}_F\,\mathbf{x}_F'
      - say: 'One solve gives the shape.'
        solve: true
```

Write TeX and sentences in **single quotes**, so YAML leaves backslashes alone. Inside `say` and
`text`, `$...$` is math and `**...**` is bold.

## Add a slide

Open the chapter file, go to the place in `slides:` where the slide belongs, and add:

```yaml
  - title: Double the load
    steps:
      - say: 'Double the load. Every height doubles.'
        solve: {load: [0, 0, 0.7]}
      - say: 'In numbers:'
        eq: 'z_{max} = 2 \cdot 2.1 = 4.2'
```

That is a complete slide. Then look at it (see *Look at a deck* below).

## Change a slide

Find the slide by its `title:` and edit the step:

- change the sentence: edit `say:`;
- change a formula: edit the TeX after `eq:`;
- add a step: add another `- say: ...` block where it belongs;
- remove a step: delete its block.

Steps play in the order written, so nothing else has to be renumbered.

## What a step can do

Each key is one thing that happens with the sentence. Keys play top to bottom.

| key | what happens |
|---|---|
| `say: '...'` | the sentence under the column |
| `eq: 'a = b'` | writes an equation; equations of a slide line up on their `=` |
| `becomes: 'a = c'` | rewrites the last equation in place (equal symbols travel, the rest fades) |
| `then: 'c = d'` | writes the next line, grown out of the last equation |
| `text: '...'` | a line of prose |
| `code: \|` + indented lines | program text in a monospace face, `#` comments dimmed |
| `image: img/portrait.png` | a PNG from the deck folder; `image: {src: img/a.png, height: 8}` sets its height in em. Publish pictures with the same `aws s3 sync`, using `--include '*.png' --content-type image/png` and no `--delete` |
| `matrix: DN` | a matrix of the net by name (table below), or `{rows: [[1, -1], [0, 1]]}` typed by hand |
| `row: ['\mathbf{u} =', {id: C, matrix: C}, ...]` | formulas and matrices on one line |
| `show: [cables, fixed]` | draws parts of the net: `fixed`, `free`, `cables`, `loads` or `net` (all of it) |
| `solve: true` | moves the free nodes to equilibrium; `solve: {load: [0, 0, 1], q: 2, rim: 6}` solves a changed net |
| `project: {of: F1, parts: 'F_{1,%}'}` | the dashed box an arrow spans, the walk along its edges in x, y, z colours, and a label on each edge (`%` becomes the axis); `axes: [x, z]` for a flat case |
| `walk: [F1, F2, F3]` | puts each arrow at the tip of the one before (the arrows run between named points); `walk: back` returns them |
| `node: {of: n4_4, row: DN}` | lights a node of the net, its cables and its row in a matrix; `clear: true` puts the colours back |
| `eliminate: {matrix: M, row: 2, with: 1}` | adds the multiple of row 1 to row 2 that clears the entry under the pivot; rows count from 1 and the new entries are computed |
| `multiply: {left: A, right: B, into: C, entry: [0, 1]}` | tints a row of `A` and a column of `B` and reveals that entry of `C` (added with `empty: true`) |
| `fill: xF` | shows the fixed coordinates empty, lets each node's label grow its coordinates and fly into its row |
| `reveal: C` | shows `C`, `CN` or `CF` empty and fills it row by row, each cable lit in turn |
| `highlight: [P0, 'M[2,:]']` | pulses an object, a matrix row, column or entry |
| `add: {...}` | any object (next section) |
| `do: [...]` | several of the above in a list, when one key is needed twice |

A matrix cell is a rounded fill: pink for positive, blue for negative, dark for zero. A matrix
with more than 14 rows or columns is drawn as its pattern: one small square per non-zero entry.

## Your own blocks

A chapter can name a group of step keys and reuse it. `$name` is a hole the step fills:

```yaml
blocks:
  given:
    - text: '**Given.** $what'
    - matrix: {id: $id, rows: $rows}
    - highlight: [$id]
slides:
  - title: The load
    steps:
      - say: 'The load on the free node.'
        given: {what: the load, id: p, rows: [[0, 0, -5]]}
```

## The net

```yaml
net:                                   # named nodes and cables
  fixed: {P1: [0, 0, 0], P2: [5, 0, 3]}
  free: {P0: [3.6, 3.4, 1.9]}          # where it is drawn before `solve`
  cables: {a: [P0, P1], b: [P0, P2]}
  q: 1                                 # one number, or {a: 2, b: 1}
  load: {P0: [0, 0, -5]}               # per node, or [x, y, z] for every free node
```

```yaml
net:                                   # a grid: nodes n<row>_<col>, cables h<row>_<col> and v<row>_<col>
  grid: [10, 10]
  fixed: corners                       # corners, boundary, or a list of node names
  lift: {n0_0: 4, n9_9: 4}             # these nodes are fixed at this height
  q: 1
  rim: 6                               # force density of the rim cables
  load: [0, 0, 0.35]
```

Matrices the net gives by name:

| name | matrix |
|---|---|
| `C`, `CN`, `CF` | branch-node matrix; its free and fixed columns |
| `q`, `Q` | force densities as a column and as a diagonal |
| `DN`, `DF` | `C_N^T Q C_N` and `C_N^T Q C_F` |
| `xF`, `p`, `b` | fixed coordinates, loads, and the right-hand side `p - D_F x_F` (columns x, y, z) |
| `xN` | the solved free coordinates |
| `l`, `L`, `f` | solved lengths (column, diagonal) and forces `L q` |
| `U` | `D_N` after elimination: the triangle a solver works with |

Node and cable names are also object names: `color: {target: a, color: accent}`,
`highlight: [P0]`, `'P0 + [0, 0, -2]'`.

## Other objects (`add:`)

| 3D, drawn on the left | |
|---|---|
| `point: P0` | a dot; `label: 'P_0'` attaches a label placed clear of lines |
| `line: [P0, P1]`, `arrow: [P0, P1]` | `width`, `dash: true`, `label` at the midpoint |
| `polyline: [A, B, C]`, `polygon: [...]` | open or closed |
| `cuboid: [P0, P1]` | the dashed box between two corners |
| `label: P1` + `tex: '(0,\,0,\,0)'` | a formula at a 3D position |
| `label: P1` + `image: img/a.png` | a picture pinned to a 3D position, `height` in em; it faces the screen and keeps its size |

A position is a name from `points:` or the net, `[x, y, z]`, or an expression:
`'P0 + [0, 0, -2]'`, `'mid(P0, P1)'`, `'lerp(P1, P0, 0.9)'`. Geometry follows a point while
`move: {point: P0, to: [3, 3, 0.25]}` animates it.

Every object takes `id` (to refer to it later), `color` (`ink dim faint free fixed load x y z
accent pos neg cell` or `#rrggbb`) and `size`.

## Other actions

| action | effect |
|---|---|
| `remove: [id, ...]` | fade out |
| `morph: {target: id, tex: '...'}` | `becomes` for an object named by id |
| `derive: {from: id, id: new, tex: '...'}` | `then` for an object named by id; with 3D geometry the new shape moves out of the old one |
| `fly: {from: P1.label, to: 'M[0,:]'}` | copies glyphs into empty matrix cells (`matrix: {..., empty: true}`) |
| `set: {target: 'M[0,1]', tex: '4'}` | change one matrix entry |
| `show` / `hide: ['M[2,:]']` | reveal or hide entries of a matrix |
| `color: {target: 'M[0,:]', color: accent}` | recolour until `color: reset` |
| `dim` / `undim: [id, ...]` | fade back and forth |
| `move`, `camera: {yaw: -50, pitch: 30}`, `wait: 1` | animate a point, the camera, or pause |

`with: true` beside an action starts it together with the one before; `dur: 2` sets its seconds.
Mistakes (an unknown name, an unknown TeX command) are listed in red at the top of the slide.

## Look at a deck

```bash
cd session/session_viewer
cargo test --target x86_64-pc-windows-msvc --lib stage::     # plays every step: no errors, no overlaps
bash ../../wood/tools/buildslot.sh stage env CARGO_BUILD_JOBS=8 trunk serve
# then open http://127.0.0.1:8080/?stage=decks/force-density/index.yaml&data=off
# trunk copies examples/stage into the site as decks/ and reloads the page when a YAML file is saved
```

Publish (the live site reads the decks from R2, no rebuild needed for a YAML change):

```bash
aws s3 sync examples/stage s3://session-viewer-data/stage/ --delete \
  --exclude '*' --include '*.yaml' --content-type application/yaml --cache-control no-cache --profile r2 \
  --endpoint-url https://0520459c6817bd96c1e25fcb49461c4e.r2.cloudflarestorage.com
```

The runtime is `src/app/stage/`: `deck.rs` (YAML, step keys, blocks), `net.rs` (the net, its
matrices and the solve), `tex.rs` and `font.rs` (TeX and code layout), `run.rs` (the stage, its
navigation and the frame), `object.rs`, `layout.rs`, `matrix.rs` (objects and the column),
`geometry.rs`, `spatial.rs`, `view.rs` (the 3D side in the viewer's scene), `actions.rs`,
`morph.rs`, `tween.rs` (what a step does and how it moves), `check.rs` (overlaps) and `paint.rs`,
`draw.rs` (drawing the column and the labels).
