---
name: session_viewer_math_steps
description: Writing the steps of a session viewer stage deck - the short step keys (eq, becomes, then, text, code, image, matrix, row, show, solve, highlight) and the blocks (project, walk, node, fill, reveal, eliminate, multiply, your own blocks). Use when writing or shortening slides.
argument-hint: <slide or idea>
---

A step is `say:` plus keys; keys play top to bottom. Use `do: [...]` only when a key is needed
twice or the order between different keys matters.

```yaml
  - title: The force density trick
    steps:
      - say: 'Each cable gets one chosen number: its force over its length.'
        eq: 'q_k = \frac{f_k}{l_k}'
      - say: 'Put it into one term of the sum.'
        then: 'q_k\,l_k\,\frac{x_k - x_0}{l_k}'
      - say: 'The length cancels.'
        becomes: 'q_k\,(x_k - x_0)'
```

| want | write |
|---|---|
| a new equation (lines up on `=` with the others) | `eq: '...'` |
| rewrite the last equation in place | `becomes: '...'` |
| the next line, grown out of the last one | `then: '...'` |
| a line of prose; a formula outside the `=` column | `text: '...'`; `tex: '...'` |
| program text | `code: \|` + indented lines |
| a picture | `image: img/a.png` or `{src: img/a.png, height: 8}` |
| a matrix of the net | `matrix: C` (`C CN CF q Q DN DF xF xN p b l L f U`) |
| a typed matrix | `matrix: {id: M, rows: [[2, -1], [-1, 2]]}` |
| formulas and matrices on one line | `row: ['\mathbf{u} =', {id: C, matrix: C}, {id: x, matrix: xN}]` |
| point at something | `highlight: [n0, 'DN[0,:]']` |
| colour until reset | `color: {target: 'C[:,0]', color: accent}` then `color: reset` |

Blocks, one line for a whole idea:

| block | does |
|---|---|
| `project: {of: F1, parts: 'f_{1,%}', axes: [x, y]}` | the dashed box of an arrow or line; its first part drops onto the axis, the others are drawn on in the order of `axes`; a label per part (ids `F1.box`, `F1.x`, `F1.x.label`) |
| `walk: [W, F2, F1]`, `walk: back` | arrows tip to tail (a force polygon), and home again |
| `node: {of: n4_4, row: DN}`, `clear: true` | a node, its cables and its matrix row lit together |
| `fill: xF` | fixed coordinates fly from the node labels into their matrix |
| `reveal: C` | a branch-node matrix filled row by row, each cable lit in turn |
| `eliminate: {matrix: M, row: 2, with: 1}` | one elimination step, entries computed |
| `multiply: {left: A, right: B, into: C, entry: [0, 1]}` | row times column, the entry revealed |

Your own block, when a chapter repeats a group of keys (`$name` is a hole):

```yaml
blocks:
  given:
    - text: '**Given.** $what'
    - matrix: {id: $id, rows: $rows}
slides:
  - title: The load
    steps:
      - say: 'The load on the free node.'
        given: {what: the load, id: p, rows: [[0, 0, -5]]}
```

Habits:

- Give an `id` only to what a later step refers to.
- `say` is shown at the top of the column and is the only running text; the slide `title` is a
  small grey line, so do not lean on it.
- `derive` takes no `label`: add the label as its own object, `{id: t2.l, label: 'mid(Q1, Q2)', tex: 'f_2'}`.
- Inside `say` and `text`, `$...$` is math and `**...**` is bold.
- Show the calculation, not only its result: `\cos\alpha = \frac{4}{5} = 0.8` before using 0.8.
- A matrix with more than 14 rows or columns draws as a pattern of small squares: talk about
  its shape, not its entries.
- Long forms (`add`, `derive`, `morph`, `fly`, `move`, `set`, `camera`) cover what the keys do
  not; the README lists them. Reach for them last, and read `/session_viewer_math_motion` first.
