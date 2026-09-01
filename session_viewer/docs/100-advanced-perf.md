# 99 Advanced perf — headroom beyond the stress file

> **Big picture.** *Phase 13 closes.* The viewer already carries its two acceptance scenes easily —
> batching (30/31), culling (53), render-on-demand (78), and the half-res AO (79) did the heavy
> lifting. This lesson is the **map of the next three levers** for scenes 10× bigger, each sketched
> to working depth: LOD, occlusion culling, and the one WebGPU makes special — **GPU compute cull
> with indirect draws**, the payoff of lesson 27's WebGPU-only decision. Build them when a real scene
> demands them; know them now.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="three levers: lod swaps cheaper meshes by screen size, occlusion culling skips hidden objects, gpu cull writes indirect draw arguments with zero cpu involvement" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3"><rect x="14" y="24" width="200" height="60"/><rect x="240" y="24" width="200" height="60"/><rect x="466" y="24" width="200" height="60"/></g>
  <g fill="#d7dae0" text-anchor="middle"><text x="114" y="44">LOD</text><text x="340" y="44">occlusion cull</text><text x="566" y="44">GPU cull + indirect</text></g>
  <g fill="#666" text-anchor="middle" font-size="10">
    <text x="114" y="62">far object → fewer triangles</text><text x="114" y="76">screen-size picks the level</text>
    <text x="340" y="62">behind a wall → skip entirely</text><text x="340" y="76">depth-pyramid test vs 53's frustum</text>
    <text x="566" y="62">compute shader writes draw args</text><text x="566" y="76">CPU never touches per-object work</text>
  </g>
  <text x="340" y="108" fill="#888" text-anchor="middle">in order of effort — and the last one is why 27 said "WebGPU only"</text>
</svg>

## Lever 1 — LOD: pay triangles by screen size

A beam 400 px tall deserves its full tessellation; the same beam at 6 px is a colored stick. The
machinery is almost all owned already:

- **Levels:** per object, precompute 1–2 coarser versions **by re-tessellating from the NURBS/BRep
  truth** at relaxed quality (44's `mesh_q` knobs) — not by decimating the level-0 mesh: vertex-weld
  / cluster-collapse opens cracks at seams and melts the hard edges CAD shading depends on.
  Decimation is the fallback for meshes with no underlying surface (imported STL/OBJ). Then
  `arena.allocate` each level at load. Cheap: slots are just ranges (48).
- **Selection:** projected box height in px (the world box through 56's `project_to_screen`) picks
  the level; hysteresis (switch up at 120 px, down at 80) prevents flicker at the boundary.
- **The swap is an index-range choice**, not an upload: draw the object's level-k `index_range`
  instead of level-0's. With 30's single-draw arena this means bucketing draws by level — 2–3
  `draw_indexed` calls instead of 1. The draw-count HUD barely notices. Honest caveat: a *per-object*
  level choice fights the one-draw batch — past a few hundred objects the practical shape is the
  level as instance data plus lever 3's indirect machinery (the compute pass emits one compacted
  draw per occupied level). Lever 1 at scale effectively rides on lever 3.
- **Rule from the user contract:** hysteresis + generous thresholds, because a *visible* LOD pop
  during orbit is a quality change while interacting — the one thing this course never does.

## Lever 2 — occlusion culling: don't draw what a wall hides

41 culls what's *outside the view*; buildings are full of things *inside the view but behind walls*.
The modern shape (no per-object GPU queries — those stall):

- Render (or reuse) depth, build a **depth pyramid** (mips of max-depth).
- Test each object's world box against the pyramid at the mip where the box is ~2 px: if the box's
  nearest depth is farther than the stored farthest depth, nothing of it can show — set 53's
  `FLAG_CULLED`, same flip-tracking, same one-draw preservation.
- One frame of latency (test against *last* frame's depth) is the standard trade and invisible in
  practice; a newly-revealed object appears one frame late, fully drawn.
- Do this **only after** profiling shows pixels bound by hidden geometry — an open floor plan gains
  nothing; a multi-story building gains a lot.

## Lever 3 — GPU cull + indirect draw: the CPU stops counting

41/lever-2 decide per object on the CPU and re-upload flags. At 500k objects even that loop matters.
WebGPU's answer — unavailable in WebGL, which is why 27 locked WebGPU-only. The shape, in
pseudocode (a sketch of the two passes — **not code to type**):

```
compute pass (one thread per object):
    read  objects[]   (boxes, in a storage buffer — 52's data, GPU-resident)
    test  frustum planes (53's math, in WGSL) + depth pyramid (lever 2)
    write survivors' draw arguments into an INDIRECT buffer (atomicAdd a counter)

render pass:
    draw_indexed_indirect(indirect_buffer)   — the GPU draws what the GPU chose
```

- The pieces this course already has GPU-side: instance models (29), boxes are one upload away, the
  plane test is seven WGSL lines (53).
- The new primitives: `wgpu::BufferUsages::INDIRECT`, `draw_indexed_indirect`, and a compute pipeline
  (the course's first — `@compute @workgroup_size(69)`).
- What changes architecturally: `drawn/total` moves GPU-side (read back *occasionally*, async, for
  the HUD — never synchronously; that's the readback rule from 47 again).
- **The memory ledger.** GPU residency isn't free: the depth pyramid (lever 2) is ~1.3× a full-res
  R32Float frame, boxes + indirect + counter buffers are per-object small but nonzero — and on
  wasm32 *everything* shares one linear address space capped at 4 GB (less in practice, once the
  browser takes its share). 41/44 already budget the CPU side of big scenes; these GPU tables join
  the same budget. At 500k objects, count bytes before you count draws.

## Measuring — the capstone's precondition

Every lever above starts with the same sentence: *profile first*. The HUD (28/41/71) already shows
fps / frame ms / draws / drawn-vs-total / drawn-per-second; add a one-line GPU timestamp pair around
the main pass (wgpu `TimestampWrites`, feature-gated) and you can attribute frame time to passes
before touching anything. The timestamp query resolves like every other GPU readback: into a
readback buffer, `map_async`, consumed a frame later — a blocking read stalls the pipeline (and wasm
can't block at all — 55's rule again). The perf lesson that matters most is the discipline: **name the
bottleneck, then pull exactly one lever, then re-measure** — 30 (draw count), 41 (vertex work), 71
(idle), 72 (fragment work) each did precisely that, and that's why the viewer is fast. One dormant
switch already sits in the code: `INK_DEPTH_PREPASS` (`gpu/mod.rs`, currently `false`) — flipping it
doubles the flat-ink cost in exchange for correct ink ordering, so measure before and after like any
other lever.

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

This lesson's "verify" is honest bookkeeping rather than a feature demo:

- Confirm the baseline numbers on the acceptance scene — the 10-sheet manifest
  (`scenes/drawings.toml`) — and write them into `_ROADMAP.md`'s capstone entry — the capstone (82)
  checks against them. MSAA is per-scene (`msaa_for`): flat sheets run 1×, so note which mode each
  number was taken in.
- If you built lever 1: orbit in and out across an LOD boundary — **no visible pop** (hysteresis),
  triangle count on the HUD steps down, fps steps up on the far view.
- If you built lever 3: `drawn/s` and fps identical to the CPU path on the stress file (it wasn't
  CPU-bound — the win only appears at scales beyond it), and the CPU frame time drops measurably.

## Recap

```
Ch 80: work plane — Phase 13's modeling tools complete.
Ch 81: HEADROOM, mapped. (1) LOD: coarser levels RE-TESSELLATED from the NURBS/BRep truth
       (decimation cracks seams and melts hard edges), pre-allocated as extra arena ranges;
       screen-size selection with hysteresis (a pop during orbit = a quality change = forbidden);
       the swap is an index-range choice — and at scale it rides on lever 3's indirect draws. (2) Occlusion: depth pyramid, test boxes at the ~2 px mip, one-frame
       latency, feeds 53's FLAG_CULLED path — only when profiling shows hidden-geometry cost.
       (3) GPU cull + indirect: a compute pass tests frustum+pyramid per object and atomically packs
       an INDIRECT buffer; draw_indexed_indirect renders what the GPU chose; CPU per-object
       work → 0. This is what 27's WebGPU-only decision bought. Above all: name the bottleneck,
       pull ONE lever, re-measure — the discipline that built every fast lesson in this course.
```

## Next

`100-capstone.md` — everything, together: load the floor model, run the whole loop — pick, tree,
gumball, numeric entry, draw with snap, save, undo — then do it again on the PDF drawing. Fixing
whatever breaks *is* the lesson.
