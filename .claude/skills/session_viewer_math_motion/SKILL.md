---
name: session_viewer_math_motion
description: Which animation to use for what in a session viewer stage deck, and which movements to cut - derive, add, move, fly, morph, solve, camera, project. Use when a step moves something, when reviewing animations, or when a slide feels busy.
argument-hint: <chapter or step>
---

A thing moves only when the movement is the idea. Otherwise it appears (`add` draws a line from
its start to its end; text is written in) or it is simply there.

| the idea | write | what the viewer does |
|---|---|---|
| the same vector, somewhere else (force polygon) | `derive: {from: f2, id: t2, arrow: [Q1, Q2]}` with the same direction and length | the arrow slides, unrotated, to its new place |
| a projection onto an axis | `project: {of: f1, parts: 'f_{1,%}', axes: [x, y]}` | the first part drops out of the arrow onto its axis; the others are drawn on in order |
| walking along axes by hand | first `derive` from the line, then `add` for each further part, in order | never `derive` every part from the diagonal: they sweep across it and show nothing |
| a value becomes an entry | `fly: {from: n1.label, to: 'xF[0,:]'}` or `fly: {from: 'eq{3}', to: 'M[0,0]'}` | the glyphs travel into the empty cells |
| a node finds its place | `solve: true`, `solve: {q: ...}`, `move: {point: X, to: ...}` | the node and everything attached to it move |
| a number that settles (a limit) | several `move`s in one step beside an `eq` with `{=...}` | the value on the slide updates while the point moves |
| the same formula, rewritten | `becomes:` (in place), `then:` (next line grown from the last) | equal symbols travel, the rest fades |
| look closer | `camera: {fit: [n0, n1], yaw: -50, pitch: 22, with: true}` and back | one smooth move; put it beside the first action of the step |

Cut these:

- **A move that undoes the one before** in the next step (labels that grow and shrink at once).
  Let the first state stand for its slide and change it when the next subject starts.
- **Remove and add again** of the same object within a slide. Decide when it is on screen.
- **Two things arriving together that are read one after the other.** Give them their own
  actions in a `do:` list, in reading order; use `with: true` only for things that are one event
  (four labels, a camera with its dimming).
- **Helpers left behind**: construction lines and points of an earlier step. Remove them in the
  step that no longer needs them, or do not draw them.
- **A step whose only change is a highlight** of what the sentence already names, unless the
  picture is crowded.

Check a movement by looking at it half way: capture with `MID=300` (the capture script takes a
frame that many milliseconds into each step). If the half-way frame shows something that is not
true of the subject, such as an arrow lying where no quantity is, the animation is wrong.
