# Stage blocks: reusable pieces of animation

Written 2026-11-10, from the eighteen chapters of the two force-density decks. Status: plan.
Nothing here is built yet except where marked *done*.

## Why

A chapter is short where it uses a ready-made piece (`show: net`, `matrix: DN`, `solve: true`,
`eq` / `becomes` / `then`) and long where it spells an idea out action by action. The same five
or six ideas are spelled out again and again. Each should be one key.

## What repeats in the decks

| idea | where it is written by hand today | lines each time |
|---|---|---|
| project a vector onto the axes: dashed box, walk along x, y, z, labelled edges | 02-equilibrium (three times), 04-projections, derivations 01 | 8-12 |
| fill a matrix from the picture: labels fly into empty rows | 03-question (fixed nodes) | 12 |
| reveal a matrix row by row while its cable or node lights up | 03-question (C), 06-matrices, 08-several-nodes | 4 per row |
| multiply row by column: tint a row and a column, show the sum, fill the entry | 06-matrices, 08-several-nodes (old), derivations 05 | 6 per entry |
| eliminate: add a multiple of one row to another, entry by entry | 09-solver | 6 per row |
| the same net under changed q, load or supports, with one formula per case | 05-one-dimension, 10-net, 11-pillow | 3 per case |
| point at one node and its cables together with its matrix row | 08-several-nodes, 10-net | 6-10 |
| tip-to-tail walk of forces and back | 02-equilibrium | 10 |
| a numbered outline of the deck | 01-method | 9 |

## The blocks

Each is a step key. It expands, when the chapter is read, into the actions that exist today, so
nothing new has to be animated and a block can always be replaced by its long form.

```yaml
- say: 'Take F1 alone: the force is the diagonal of its box.'
  project: {of: F1, parts: 'F_{1,%}'}        # box, walk along x, y, z, one label per edge

- say: 'One row per node.'
  fill: {matrix: xF, from: labels}           # the node labels fly into their rows

- say: 'Cable by cable.'
  reveal: {matrix: C, by: row}               # one sub-step per row, its cable lit meanwhile

- say: 'Row 1 times column 1.'
  multiply: {left: CNt, right: QCN, into: DN, entries: [[0, 0], [0, 1]]}

- say: 'Add half of row 1 to row 2.'
  eliminate: {matrix: M, row: 2, with: 1}    # the factor is computed, entries change one by one

- say: 'Three cases.'
  cases:
    - {solve: {q: {b: 3}}, eq: 'x_0 = 7.5'}
    - {solve: {load: {N: [4, 0, 0]}}, eq: 'x_0 = 7'}

- say: 'Its row has five entries.'
  node: n4_4                                  # the node, its cables and its row in DN, lit together

- say: 'Put each arrow at the tip of the one before.'
  walk: [F1, F2, F3]                          # tip to tail; `walk: back` returns them

- outline: true                               # the chapter titles of the deck, numbered
```

Rules for every block:

- one key, a short map of names that already exist in the chapter (`F1`, `DN`, `n4_4`);
- sensible defaults: colours of the axes, the accent for what is pointed at, automatic reset at
  the end of the step, labels placed clear of lines;
- the net supplies numbers: `eliminate` and `multiply` compute what they show, so a slide cannot
  disagree with the solve.

## Blocks written by the author

Some patterns belong to one deck only. A deck can define its own block as a template in
`index.yaml`, with `$name` standing for what the step passes in:

```yaml
blocks:
  given:                                   # used as `given: {title: ..., matrix: p}`
    - text: '**Given.** $title'
    - matrix: $matrix
    - highlight: [$matrix]
```

This is text substitution before the chapter is parsed: no code, and an unknown `$name` is
reported in red on the slide like any other mistake.

## Order of work

1. `project` (*done 2026-11-10*, used in 02-equilibrium), `walk` (*done*, 02-equilibrium) and `node` (*done*, 10-net).
2. `reveal`, `fill`, `cases` (pure expansions of existing actions).
3. `eliminate` (*done*, 09-solver; it computes on the matrix the slides typed) and `multiply`
   (*done*, with a test; no chapter uses it yet).
4. `blocks:` templates (*done*, per chapter, with a test). `outline` and `cases` are dropped:
   each saves a few lines once.
5. Rewrite the eighteen chapters with the blocks; the native test that plays every step is the
   guard that nothing moved.

Each block lands with one test (`deck::tests`: the block expands to the expected actions) and one
chapter converted to it.

## Loading

Measured 2026-11-10 on the published site, first visit, fast connection, desktop Chrome:

| | size on the wire | |
|---|---|---|
| viewer `.wasm` | 3.0 MB (9.5 MB unpacked) | the whole viewer: BRep, meshes, commands, selftests |
| four fonts | 2.0 MB (3.1 MB unpacked) | math, text, bold, mono |
| default scene | 0.9 MB | not needed for a deck |
| eleven chapter files | 0.05 MB | |

Ready after about 4.0 s; a reload with a warm cache takes 1.4 s. The files were fetched one after
another, so the fonts only started when the wasm had been compiled, and each chapter waited for
the one before.

- *Done:* a deck's fonts, index and chapters now start downloading with the page, in parallel with
  the wasm (`index.html`), and the default scene is no longer fetched for a deck.
- Next, in order of gain:
  1. *Done 2026-11-10: 3.1 MB to 1.1 MB, 49 of 50 compared frames pixel-identical.* Subset the fonts to the glyphs TeX and the decks can reach (Latin, Greek, math operators,
     delimiters and their size variants): about 3.1 MB to under 0.6 MB. Fetch the mono face only
     when a deck uses `code:`.
  2. Run `wasm-opt -Oz` on the release build (it is switched off: `data-wasm-opt="0"`); typical
     gain 15-25 % of the wasm.
  3. A `deck` build of the viewer without the parts a deck never calls (BRep tessellation,
     command dock, selftests), behind a cargo feature, published as a second wasm that
     `?stage=` pages load. Expected to halve the wasm; needs measuring before promising.
  4. Show the title slide's text as soon as the fonts are in, while the 3D scene starts.

Memory is not the problem: about 25 MB of wasm memory and 3 MB of script heap after walking
through the 81- and 100-node chapters. GPU memory was not measured.

## What draws what

- 3D side: the session viewer's own scene and wgpu pipeline. Points, lines and polylines are
  kernel geometry; arrowheads are two kernel lines.
- Formula column, titles, labels: laid out by `stage/tex.rs` from the fonts' OpenType tables and
  painted as glyph quads through egui, the same Rust library that draws the viewer's own panels.
- JavaScript on the page: the glue that wasm-bindgen generates from our Rust, `diagnostics.js`
  and the inline start-up script, all from this repository. No JavaScript library is loaded.
- Rust libraries compiled into the wasm: wgpu and winit (GPU and window), egui (panels and the
  column's painter), swash and ttf-parser (glyph outlines and font tables), serde_yaml_ng (decks).

If the column must not pass through egui either, the glyph quads can go to the viewer's own text
pass instead; `paint.rs` is the only file that would change. That is a separate decision.
