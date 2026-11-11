---
name: session_viewer_math_scene
description: The 3D side of a session viewer stage deck - chapter header, net (named nodes or grid), points, camera, floor grid, and hand-placed points, lines, arrows, labels and pictures. Use when setting up or changing what a chapter draws on the left.
argument-hint: <chapter or scene>
---

The chapter header, before `slides:`. A grid net:

```yaml
title: A pillow
camera: {yaw: -52, pitch: 30}        # or {view: top} for a flat case
net:                                 # the structure; gives the matrices and the solve
  grid: [9, 9]                       # nodes n<row>_<col>, cables h<row>_<col>, v<row>_<col>
  fixed: corners                     # corners | boundary | [names]
  lift: {n0_0: 4, n8_8: 4}           # these nodes are fixed at this height
  q: 1                               # one number or {cable: value}
  rim: 10                            # force density of the rim cables
  load: [0, 0, -0.1]                 # every free node, or {node: [x, y, z]}
```

A small named net:

```yaml
net:
  fixed: {n1: [0, 0, 0], n2: [5, 0, 3], n3: [0, 7, 3], n4: [7, 5, 0]}
  free: {n0: [3.6, 3.4, 1.9]}        # drawn here until `solve`
  cables: {e0: [n0, n1], e1: [n0, n2], e2: [n0, n3], e3: [n0, n4]}
  q: 1
  load: {n0: [0, 0, -5]}
```

Without a net, name positions and add objects in a step:

```yaml
points:
  n: [0, 0, 0]
  A: [-4, 3, 0]
  W1: [0, -4, 0]
slides:
  - title: A load on two cables
    steps:
      - say: 'A cable runs from support $A$ to node $n$. A load hangs there.'
        add:
          - {id: A, point: A, color: fixed, label: 'A'}
          - {id: c1, line: [n, A]}
          - {id: W, arrow: [n, W1], color: load, width: 3.6, label: 'p'}
          - {id: n, point: n, color: free, label: 'n'}
```

What to remember:

- `show: net` draws all of a net; `show: [cables, fixed]`, `show: loads` and `show: forces`
  (solved cables, blue and thin to pink and thick by force) draw parts.
- `solve: true` moves the free nodes to equilibrium; `solve: {q: {e2: 4}}` or
  `solve: {load: [0, 0, 1], rim: 6}` solves a changed net. Everything attached to a node
  follows it.
- A position is a name, `[x, y, z]`, or an expression in quotes: `'n0 + [0, 0, -2]'`,
  `'mid(A, B)'`, `'lerp(A, B, 0.9)'`, `'[x(n1), y(n0), z(n0)]'`.
- An arrow runs between two positions and its head ends exactly on the second: to make arrows
  touch end to end, give them the same named point.
- Force and vector arrows use `width: 3.6`; cables keep their default and every point is the same
  dot (never give a point a `width`). Do not tune sizes per slide: lines, dots and heads already
  scale with the slide, phone included.
- A force diagram stands beside the structure, at named points of its own (`Q0`, `Q1`, ...), with
  no dots at its corners: the arrows meet tip to tail. Force parts are drawn there, with the walk
  order chosen so the parts lie outside the polygon (`axes: [y, x]` for one force, `[x, y]` for
  the next); see `equilibrium/01-equilibrium.yaml`.
- An angle mark is a `polyline` of about nine points on an arc, from the projection to the cable
  or force, in the colour of that axis, plus a `label:` + `tex:` just outside its
  middle. Keep its radius under the shorter side of the triangle; see
  `equilibrium/01-equilibrium.yaml` (2D) and `force-density/03-projections.yaml` (3D).
- Add points last in a step so they draw over the lines that meet there.
- Pictures go in the column: `image: img/name.png`, files in the deck's `img/` folder.
- The camera frames every named point, including where the net comes to rest. Add a named point
  to widen the view; use `camera: {fit: [n0, n1], yaw: -52, pitch: 22}` only for a close-up.
