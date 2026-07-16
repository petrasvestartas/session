# 77 Capstone — the full loop, on real work

> **Big picture.** *The course, closed.* Seventy-six lessons built a WebGPU CAD viewer from a blank
> window: one draw call per geometry class, an f64 kernel behind an f32 screen, a live file, a
> command line, undoable everything, curves and solids, an engineered-fast arctic look. The capstone
> adds **no features**. It runs the whole system on two real models, end to end, and treats every
> stumble as the last lesson: *fixing whatever breaks is the exercise.* When both scenes pass the
> loop, the viewer isn't a course project anymore — it's a tool.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the acceptance loop: load, fit, pick, tree reveal, gumball, numeric entry, draw with snap, save, undo history — run on both the floor model and the pdf drawing" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2">
    <rect x="12" y="20" width="88" height="26"/><rect x="118" y="20" width="88" height="26"/><rect x="224" y="20" width="88" height="26"/><rect x="330" y="20" width="88" height="26"/>
    <rect x="12" y="66" width="88" height="26"/><rect x="118" y="66" width="88" height="26"/><rect x="224" y="66" width="88" height="26"/><rect x="330" y="66" width="88" height="26"/>
  </g>
  <g fill="#d7dae0" font-size="10" text-anchor="middle">
    <text x="56" y="37">load .pb (34)</text><text x="162" y="37">fit (15)</text><text x="268" y="37">pick (42–44)</text><text x="374" y="37">tree reveal (71)</text>
    <text x="56" y="83">gumball (52–56)</text><text x="162" y="83">draw + snap (57–59)</text><text x="268" y="83">save (39)</text><text x="374" y="83">undo walk (51)</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.1"><line x1="100" y1="33" x2="116" y2="33" marker-end="url(#ah77)"/><line x1="206" y1="33" x2="222" y2="33" marker-end="url(#ah77)"/><line x1="312" y1="33" x2="328" y2="33" marker-end="url(#ah77)"/><path d="M 374,46 V56 H 56 V64" fill="none" marker-end="url(#ah77)"/><line x1="100" y1="79" x2="116" y2="79" marker-end="url(#ah77)"/><line x1="206" y1="79" x2="222" y2="79" marker-end="url(#ah77)"/><line x1="312" y1="79" x2="328" y2="79" marker-end="url(#ah77)"/></g>
  <g transform="translate(460,20)">
    <text x="0" y="14" fill="#888">× two scenes:</text>
    <text x="0" y="36" fill="#d7dae0" font-size="10">floor_model.pb — mesh-heavy (491 obj)</text>
    <text x="0" y="54" fill="#d7dae0" font-size="10">30700_querschnitt_gg.pb — 42,232 curves</text>
    <text x="0" y="82" fill="#666" font-size="10">every phase's verify line must pass</text>
    <text x="0" y="96" fill="#666" font-size="10">on BOTH, in ONE session</text>
  </g>
  <defs><marker id="ah77" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## The script — scene A: the floor model

`DEMO_SESSION_URL = "session_data/floor_model.pb"`. Run in order, no skipping — the order is chosen
so each step's output is the next step's input:

1. **Load + fit.** 491 objects log (34a); `F` frames the floor (34b). HUD: 60 fps, single-digit
   draws, `drawn/s = 0` once you let go (66).
2. **Pick the far corner beam.** Click it → guid logs, outline rings it (69), gumball at its volume
   center (52), tree unfolds to its row (71). Shift+click a second beam — both ring, centroid moves.
3. **Gumball.** Drag X: live, axis-locked (54). Click the arrow, type `250` ⏎ — exact (56). Rotate
   the arc 30° — pinned to the plane (55).
4. **Draw with snap.** `cplane` onto a beam's face (75). `rect` two corners with `[End]` snaps (59).
   `box`, height `120`. `curve` through four clicks (60). Every one lands *exactly* — `probe`
   endpoints to prove it (49).
5. **Save.** Wait the debounce (39) → one download. Reload the saved file (34a URL swap or 40's
   watch): your box, rect, and curve are there, placed to the digit.
6. **The undo walk.** `undo` repeatedly, all the way down: curve → box → rect → rotate → 250-move →
   drag — each reverts *exactly*, in reverse birth order, mixed kinds (51's promise). `redo` back to
   the top. The scene must land byte-identical — hash the session (38b's fingerprints) before and
   after as the referee.

## The script — scene B: the PDF drawing

`30700_querschnitt_gg.pb` — 42,232 curves, the geometry that punishes different code paths:

1. Load, `F`, orbit — interactive (31's one draw + 37's cull). `drawn/total` moves as you zoom.
2. **Pick one line in dense hatching** — the intended one wins (44's tolerance + priority).
   **Marquee a few thousand** — instant, exact (45).
3. **Gumball-drag the marquee'd region** — live at full rate (54's matrix-only; the stress gate).
   Numeric-move it back `−(what you dragged)`; verify with `probe` it's home.
4. Hide the selection (46), marquee again — hidden lines neither pick nor select. Show all.
5. Draw a `polyline` annotation over it with endpoint snaps; save; undo the polyline.

## When something breaks — that's the lesson

The capstone's real syllabus is the fix loop. Typical finds and where they point:

| symptom | first suspect |
|---|---|
| picks the wrong thing / nothing at density | 44's tolerance, 42's BVH candidates, 36's world boxes |
| a moved object saves in its old place | 54's commit — `apply_delta` missing an arm, hashes stale |
| undo restores at the wrong row / loses color | 51/38b — `commit` renumbering, snapshot too shallow |
| tree and viewport disagree | a mutation bypassing the Scene verbs (45/46's single authority) |
| fps sags on scene B only | a per-object CPU loop that scales with N — profile before pulling 76's levers |
| something draws but won't pick / hide | a map missed a source — `all_objects()` audit (64) |

Fix, re-run the *whole* script for that scene (regressions hide behind the step you just fixed), and
move on. Both scenes green in one session = done.

## Recap — the course

```
Phase 0–3  (01–28): window → triangle → camera → kernel meshes → CAD look basics → perf counter.
Phase 4    (29–35): ONE DRAW per class — instancing, arena, cylinder lines, point sprites; camera-
                    relative f64→f32; real files; Scene owns the document, Gpu owns the device.
Phase 5    (36–37): BVH + frustum cull — flags, not draw calls.
Phase 6    (38–40): the file is ALIVE — per-object arena, diff by content hash, save gates, watch.
Phase 7    (41–46): pick everything — ray, meshes, sub-objects, thin geometry, selection, visibility.
Phase 8    (47–51): the interface — egui, THE command bus, options, history, trait-Command undo.
Phase 9    (52–59): transform & draw — gumball (defer/live/commit), tools as commands, ghosts, snap.
Phase 10   (60–64): curves & surfaces — sample, tessellate-once, real edges, trims; all_objects().
Phase 11   (65–69): the arctic look, engineered — analytic ground, render-on-demand, GTAO+GI, outline.
Phase 12   (70–72): the document visible — virtualized tree, two-way sync, labels.
Phase 13   (73–76): editing the insides — CVs (4-d!), Greville refit, work plane, perf headroom.
Capstone   (77):    the loop, twice, green.
```

The through-lines, if you take three things from seventy-seven lessons: **route everything through
one path** (one draw per class, one point resolver, one selection set, one Command pipe — every
"zero tool edits" moment came from a choke point built early); **the kernel is f64 truth, the GPU is
an f32 view** (camera-relative, matrices-not-geometry, tessellate-once); and **quality is a
contract, performance is architecture** — the fast paths came from skipping work, never from
degrading it.

Where next: the roadmap's optional appendix (materials/textures), 76's levers when a scene demands
them, and the parked kernel work (booleans, STEP) that this viewer is now a worthy front-end for.
