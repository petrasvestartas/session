---
name: session_viewer_math
description: Make or change a mathematical presentation (stage deck) for the session viewer - a 3D picture on the left, equations and matrices on the right, written only in YAML. Use for "/session_viewer_math <request>", "add a slide", "make a presentation about ...", "explain this derivation in the viewer".
argument-hint: <what to present or change>
---

A presentation is a folder of YAML in `session_viewer/examples/stage/<name>/`: `index.yaml`
(name + chapter files) and one file per chapter. A chapter is one 3D scene and a list of slides;
a slide is a list of steps; a step is one sentence (`say`) plus what appears with it. The full
format is `session_viewer/examples/stage/README.md`: read it before writing. The five
force-density decks beside it are the worked examples; copy from them.

## Do this, in order

1. **Outline first.** One presentation = one idea a student can hold. More than about 8 chapters
   or 25 steps in a chapter: split it. Start from what is drawn, then what is computed. Every
   presentation ends on a result and a pointer to the next one.
2. **Pick the scene** of each chapter: a `net:` when the subject is a cable net (it gives every
   matrix and the solved shape), named `points:` otherwise. See `/session_viewer_math_scene`.
3. **Write the steps** with the short keys and blocks, not raw actions. See
   `/session_viewer_math_steps`.
4. **Check and look.** Run the test, open the chapter, step through it. See
   `/session_viewer_math_publish`. A chapter is not done until you have looked at it.
5. **Publish** only when asked: upload the YAML (and pictures) to R2. The viewer itself needs a
   push only when Rust changed.

## Rules that keep decks good

- One sentence per step, about 15 words, plain words, present tense. TeX and sentences in
  **single quotes**.
- The 3D picture stays; only notation changes or objects move. Do not rebuild the scene per slide.
- Numbers on a slide come from the net or are checked by hand; never invent them.
- Same names everywhere: nodes `n0…`, cables `e0…`, forces `f_k`, force densities `q_k`.
  The load is `p`. Say "cable", not "bar" or "branch".
- Projections, the same in every deck: `\Delta x_k = x_k - x_0`, length `l_k`, angles `\alpha_k`,
  `\beta_k`, `\gamma_k` between cable k and its x, y, z part, each drawn as a small arc at the node
  between the force and that part (one cable at a time). The cosine is the side over the length,
  `\cos\alpha_k = |\Delta x_k| / l_k`, shown before it is used; the part is
  `f_{k,x} = f_k \Delta x_k / l_k`, so the sign comes from the direction.
- Colours: `free` pink, `fixed` blue, `load` green, `accent` violet, axes `x` pink, `y` green,
  `z` blue. No orange, no other hues.
- A flat subject: `camera: {view: top}` with all points at z = 0, and call the axes x and y.
- No duplicated slides: if two slides show the same picture with the same sentence, cut one.
- Mistakes (unknown name, unknown TeX command, overlapping blocks) fail the test with the
  chapter and step named: fix the YAML, do not loosen the test.

## Adding to the engine

Only when a pattern repeats in three places and no key or block covers it. A block is a function
in `session_viewer/src/app/stage/deck.rs` that expands to existing actions, plus one test in
`tests.rs` and one row in the README. Drawing stays in the viewer's own wgpu pipelines (points,
lines and arrows go into the scene through `spatial.rs`; arrowheads are `arrow.wgsl`): never
paint 3D geometry over the scene.
