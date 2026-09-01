# 117 HiZ occlusion — what's behind things costs nothing

> **Big picture.** *Phase 15.* Frustum culling (55) can't help when everything IS in
> view — an assembly's interior, the city behind the first facade. The GPU already
> knows what's hidden: LAST frame's depth buffer. Reduce it to a mip pyramid of MAX
> depths (reverse-Z: max = farthest... careful — we keep NEAREST, so MIN of reverse-Z
> bits, which is max distance), test each object/batch/meshlet box against the coarsest
> mip that covers it, and skip what can't show. This is 90's "lever 2", now implemented;
> the two-phase variant is the correctness fix for popping.

## Design

- Pyramid: after the main pass, a compute chain builds `hiz[mip]` from `hiz[mip-1]`
  (2×2 reduction keeping the FARTHEST value in reverse-Z terms). WebGPU has no min/max
  samplers — sample 4 texels by hand; the whole chain is ~10 dispatches at 1080p.
- Test (in the cull shaders 102/104 already run, one extra block): project the box to a
  screen rect + nearest depth; pick the mip where the rect covers ≤2×2 texels; if the
  box's NEAREST depth is farther than the stored FARTHEST depth in those texels,
  nothing of it can show — cull.
- Single-phase first: test against LAST frame's pyramid. Costs one frame of popping
  when the camera swings fast — visible, honest, shippable.
- Two-phase (the fix, second step): pass 1 draws what was visible last frame, build the
  pyramid from THAT, pass 2 tests the previously-culled set and draws the newly
  visible — no popping, one extra cull dispatch, the standard recipe.
- Interaction with render-on-demand (80): a static frame builds no new pyramid — the
  cull result is cached with the frame, so idle stays free.

## Steps (sketch)

1. `hiz.wgsl` reduction + the pyramid texture (mip chain, `R32Float`, STORAGE per mip).
2. The box-vs-pyramid test as a WGSL fn shared by the batch/meshlet/object cull passes.
3. Two-phase re-plumbing of the main pass (early/late lists) once single-phase numbers
   justify it.

## Verify

- Stand in front of the densest object; put the scan BEHIND it: solid-pass time drops
  toward the front object's cost alone; HUD shows occlusion-culled counts.
- Swing the camera 180° fast in single-phase: one frame of popping, then correct —
  then the same test in two-phase: none.
- An empty scene or first frame: pyramid uninitialized → the test must PASS everything
  (cleared-to-far pyramid, not garbage).

## Recap

```
Ch 106: HiZ. Last frame's depth, mip-reduced keeping the farthest value; boxes test the
        coarsest covering mip; hidden means skipped in the same cull passes 102/104
        already run. Single-phase (one-frame popping) first, two-phase (early/late)
        when it matters. 90's lever 2, cashed in.
```
