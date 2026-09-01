# 101 Capstone — the full loop, on real work

> **Big picture.** *The main course, closed — Phase 14 (83–89) and the textures appendix (90) follow.*
> Seventy-six lessons built a WebGPU CAD viewer from a blank
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
    <text x="56" y="37">load .pb (34)</text><text x="162" y="37">fit (15)</text><text x="268" y="37">pick (47–49)</text><text x="374" y="37">tree reveal (83)</text>
    <text x="56" y="83">gumball (57–61)</text><text x="162" y="83">draw + snap (62–64)</text><text x="268" y="83">save (50)</text><text x="374" y="83">undo walk (64)</text>
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

The scene is the manifest: `DEMO_SCENE_URL = "scenes/drawings.toml"` (the old single-file
`DEMO_SESSION_URL` is gone) — switching content means editing the manifest's items, so point one at
`pb/floor_model.pb`. Run in order, no skipping — the order is chosen
so each step's output is the next step's input:

1. **Load + fit.** 491 objects log (34a); `F` frames the floor (34b). HUD: 60 fps, single-digit
   draws, `drawn/s = 0` once you let go (78).
2. **Pick the far corner beam.** Click it → guid logs, outline rings it (74), gumball at its volume
   center (65), tree unfolds to its row (83). Shift+click a second beam — both ring, centroid moves.
3. **Gumball.** Drag X: live, axis-locked (67). Click the arrow, type `250` ⏎ — exact (69). Rotate
   the arc 30° — pinned to the plane (68).
4. **Draw with snap.** `cplane` onto a beam's face (87). `rect` two corners with `[End]` snaps (72).
   `box`, height `120`. `curve` through four clicks (70). Every one lands *exactly* — `probe`
   endpoints to prove it (62).
5. **Save.** Wait the debounce (50) → one download. Policy: save the ACTIVE doc — this script edits
   one sheet; an all-docs save is the same call in a loop over `scene.docs`. Reload the saved file
   (point a manifest item at it, or 51's watch): your box, rect, and curve are there, placed to the
   digit.
6. **The undo walk.** `undo` repeatedly, all the way down: curve → box → rect → rotate → 250-move →
   drag — each reverts *exactly*, in reverse birth order, mixed kinds (64's promise). `redo` back to
   the top. The scene must land byte-identical — hash the session (49's fingerprints) before and
   after as the referee.

## The script — scene B: the PDF drawing

`30700_querschnitt_gg.pb` — the manifest's first item; 42,232 curves, the geometry that punishes
different code paths:

1. Load, `F`, orbit — interactive (31's one draw + 53's cull). `drawn/total` moves as you zoom.
2. **Pick one line in dense hatching** — the intended one wins (57's tolerance + priority).
   **Marquee a few thousand** — instant, exact (58).
3. **Gumball-drag the marquee'd region** — live at full rate (67's matrix-only; the stress gate).
   Numeric-move it back `−(what you dragged)`; verify with `probe` it's home.
4. Hide the selection (59), marquee again — hidden lines neither pick nor select. Show all.
5. Draw a `polyline` annotation over it with endpoint snaps; save; undo the polyline.

## When something breaks — that's the lesson

The capstone's real syllabus is the fix loop. Typical finds and where they point:

| symptom | first suspect |
|---|---|
| picks the wrong thing / nothing at density | 57's tolerance, 55's BVH candidates, 52's world boxes |
| a moved object saves in its old place | the move bypassed 67's `apply_world_delta` (Session xform never written), or 46/50's fingerprint missing `session.xform` |
| undo restores at the wrong row / loses placement | 56 — snapshot missing its `local` xform or doc; 46 `commit` renumbering |
| tree and viewport disagree | a mutation bypassing the Scene verbs (58/67's single authority) |
| fps sags on scene B only | a per-object CPU loop that scales with N — profile before pulling 81's levers |
| something draws but won't pick / hide | a map missed a source — `all_objects()` audit (47) |

Fix, re-run the *whole* script for that scene (regressions hide behind the step you just fixed), and
move on. Both scenes green in one session = done.

## Recap — the course

```
Phase 0–3  (01–28): window → triangle → camera → kernel meshes → CAD look basics → perf counter.
Phase 4    (29–39): ONE DRAW per class — instancing, arena, cylinder lines, point sprites and the
                    raw cloud lane; camera-relative f64→f32; real files; Scene owns the document,
                    Gpu owns the device.
Phase 5    (40–41): BVH + frustum cull — flags, not draw calls.
Phase 6    (45–45): the file is ALIVE — per-object arena; reconcile is per-DOC, diff by content
                    hash (which includes session.xform — 46's rewrite), save gates, watch.
Phase 7    (46–51): pick everything — ray, meshes, sub-objects, thin geometry, selection,
                    visibility.
Phase 8    (52–56): the interface — egui, THE command bus, options, history, trait-Command undo.
Phase 9    (57–64): transform & draw — gumball (defer/live/commit), tools as commands, ghosts, snap.
Phase 10   (65–69): curves & surfaces — sample, tessellate-once, real edges, trims; all_objects().
Phase 11   (70–74): the arctic look, engineered — analytic ground, render-on-demand, GTAO+GI,
                    outline.
Phase 12   (75–77): the document visible — virtualized tree, two-way sync, labels.
Phase 13   (78–81): editing the insides — CVs (4-d!), Greville refit, work plane, perf headroom.
Capstone   (82):    the loop, twice, green.
```

The through-lines, if you take three things from seventy-seven lessons: **route everything through
one path** (one draw per class, one point resolver, one selection set, one Command pipe — every
"zero tool edits" moment came from a choke point built early); **the kernel is f64 truth, the GPU is
an f32 view** (camera-relative, matrices-not-geometry, tessellate-once); and **quality is a
contract, performance is architecture** — the fast paths came from skipping work, never from
degrading it.

Where next: Phase 14 (83–89 — sections through web polish) and the textures appendix (90), 81's
levers when a scene demands them, and the parked kernel work (booleans, STEP) that this viewer is
now a worthy front-end for.
