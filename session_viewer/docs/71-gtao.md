# 71 GTAO — ambient occlusion at constant quality

> **Big picture.** *Phase 11.* Ambient occlusion — creases and contacts darkening — is what makes the
> arctic look read as *lit* instead of flat. The archive's SSAO burned ~112 texture reads per pixel,
> full-res, every frame; this rebuild spends **~12 reads per drawn pixel** for equal-or-better
> quality: half-resolution GTAO, a fixed tap budget, and — the user rule again — **the same result
> every frame, moving or still**. No temporal accumulation, no motion-adaptive degradation, no idle
> refinement (if idle looked better, starting an orbit would read as a quality *drop*). And 66 makes
> the idle cost exactly zero.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="depth renders at full res; gtao computes ao at half res with fixed slices and steps; one depth-aware blur; composite upsamples depth-aware and multiplies the scene" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="8" y="34" width="110" height="34"/><rect x="150" y="34" width="160" height="34"/><rect x="342" y="34" width="130" height="34"/><rect x="504" y="34" width="166" height="34"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="63" y="49">depth (full res)</text><text x="63" y="62" fill="#666" font-size="9">the 3-D pass, free</text>
    <text x="230" y="49">GTAO — HALF res</text><text x="230" y="62" fill="#666" font-size="9">3 slices × 6 steps · IGN · RGBA16F</text>
    <text x="407" y="49">5-tap blur</text><text x="407" y="62" fill="#666" font-size="9">depth-aware, half res</text>
    <text x="587" y="49">composite</text><text x="587" y="62" fill="#666" font-size="9">depth-aware upsample × scene</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.3"><line x1="118" y1="51" x2="148" y2="51" marker-end="url(#ah67)"/><line x1="310" y1="51" x2="340" y2="51" marker-end="url(#ah67)"/><line x1="472" y1="51" x2="502" y2="51" marker-end="url(#ah67)"/></g>
  <text x="340" y="106" fill="#888" text-anchor="middle">AO is low-frequency → half res = ¼ the pixels; budget ≈ 12 reads/px/frame vs the archive's ~112</text>
  <text x="340" y="126" fill="#666" text-anchor="middle">STATIC noise, fixed taps — a mid-orbit frame and a rest frame are the same image</text>
  <defs><marker id="ah67" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/shaders/gtao.wgsl          # NEW — the AO pass (fullscreen, reads depth, writes RGBA16Float)
src/shaders/blur5.wgsl         # NEW — one 5-tap depth-aware blur
src/engine/gpu/targets.rs      # NEW — half-res AO textures, full-res scene color target
src/engine/pipelines/build.rs  # the two pipelines + a composite change
src/engine/gpu/mod.rs          # pass order: scene → gtao → blur → composite
```

The scene now renders to an offscreen color target instead of the swapchain, and a **composite** pass
(fullscreen) multiplies scene × AO into the swapchain — the standard post-process arrangement; 73's
outline joins the same composite.

## The five traps — ported from the archive, not re-derived

Each of these cost the archive real debugging time. They are the lesson:

**1. View-space position via ANALYTIC inverse projection.** GTAO reconstructs each pixel's view-space
position from depth. The tempting `inverse(proj)` upload hits 45's trap in shader form — and even a
correct CPU inverse wastes precision. For a standard perspective matrix the inverse is *analytic*:

```wgsl
// proj_info = (2/P[0][0], 2/P[1][1], near-plane terms) uploaded once; reverse-Z depth d:
fn view_pos(ndc: vec2<f32>, d: f32) -> vec3<f32> {
    // reverse-Z: z = -near / d (infinite far) — 3 ops
    let z = view_z_from_depth(d);
    return vec3<f32>(ndc.x * proj_info.x * -z, ndc.y * proj_info.y * -z, z);
}
```

Two multiplies per axis, exact, no matrix. (The archive's root perspective bug lived here — the
kernel's `Xform::inverse` used to be affine-only; lesson 45 found and fixed it kernel-wide. The
analytic form is still the right choice *here* regardless: fewer ops per pixel and no matrix upload.)

**2. Interleaved Gradient Noise, STATIC.** Every AO needs per-pixel randomization of sample
directions or it bands. Use IGN — `fract(52.9829189 * fract(0.06711056·x + 0.00583715·y))` — a
per-*pixel* pattern that a small spatial blur cancels almost perfectly. **Never** re-seed it per
frame: per-frame jitter shimmers during rotation, which is exactly the quality-change the user rule
forbids. Static noise + static taps ⇒ deterministic image.

**3. The tangent-plane gate.** On flat surfaces viewed at grazing angles, depth quantization makes
neighboring samples look like occluders — the floor grows stripes. Gate every horizon sample against
the surface plane: a sample only occludes if it rises meaningfully out of the tangent plane:

```wgsl
    if (dot(delta, normal) > len * 0.07 + bias) { /* counts as occlusion */ }
```

This single line is **mandatory** — remove it and the ground plane (69) stripes immediately.

**4. Radius in scene units, clamped — and mind the units.** The AO radius as a fraction of the
scene bbox diagonal (≈ 1–2%), clamped to sane bounds — a fixed radius is invisible on a big model
and engulfs a small one. Two anchors: (a) `scene_min/max` are world **millimetres**, but the
reconstructed view positions are **metres** — the camera bakes the mm→m unit scale into
`view_proj` — so the radius uniform needs the same `× 0.001`; (b) with progressive loading the
bbox grows on every appended file, and `set_scene` is what refreshes `scene_min/max` — put the
radius-uniform update **in `set_scene`**, not in some once-per-load hook. (The AO pass is
downstream of `set_scene` and sees only `scene_min/max` — it survives the Scene refactor
untouched.)

**5. RGBA16Float + MSAA depth reads.** AO in an 8-bit channel bands visibly on smooth walls — use
**16-bit float**. But the target carries *two* results, not one: the scalar `ao` **and** the
3-component **bent normal** 68 reads back as `.gba` — so it must be **`RGBA16Float`**, `R` = ao,
`GBA` = bent. A single `R16Float` channel can't hold a direction; write the pass to `R16Float` and 72's
`decode_bent(.gba)` reads garbage. And the depth buffer's sample count is **dynamic** since 35:
`msaa_for` picks 1× for flat-only scenes and 4× once solid geometry arrives, and `set_scene`
rebuilds the depth/msaa views *and* the pipelines mid-session on the flip. A sampler can't filter a
depth texture anyway — you bind it as a texture and `textureLoad(depth_tex, pixel, 0)` (sample 0 is
exact and cheap) — but a *multisampled* texture binding is **incompatible with a 1× depth texture**:
the bind group layout hard-codes `multisampled: true/false`. Either force 4× whenever AO is on
(simplest: make `msaa_for` return 4 while the AO toggle is set), or build both layouts + bind groups
and pick per frame from `self.samples`. And the offscreen scene-color target this lesson adds must
be recreated on the same flip, in `set_scene` beside the depth/msaa views — its sample count must
track the scene's.

<svg viewBox="0 0 560 148" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a single R16Float channel holds AO only and cannot carry a direction; RGBA16Float packs AO in R and the bent normal xyz in G B A" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="20" y="24" fill="#e06c6c">R16Float — WRONG</text>
  <g><rect x="20" y="34" width="60" height="30" fill="none" stroke="#e06c6c" stroke-width="1.3"/><text x="50" y="53" fill="#d7dae0" text-anchor="middle">ao</text></g>
  <text x="96" y="46" fill="#666">R</text>
  <text x="130" y="46" fill="#e06c6c">bent → nowhere</text>
  <text x="130" y="60" fill="#666" font-size="9">72's decode_bent(.gba) reads (0,0,0)</text>
  <text x="20" y="98" fill="#5bbf87">RGBA16Float — one texel, 4 lanes</text>
  <g fill="none" stroke="#5bbf87" stroke-width="1.3">
    <rect x="20" y="108" width="60" height="30"/><rect x="80" y="108" width="60" height="30"/><rect x="140" y="108" width="60" height="30"/><rect x="200" y="108" width="60" height="30"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle"><text x="50" y="127">ao</text><text x="110" y="127">bent.x</text><text x="170" y="127">bent.y</text><text x="230" y="127">bent.z</text></g>
  <g fill="#6fb3ff" text-anchor="middle" font-size="9"><text x="50" y="103">R</text><text x="110" y="103">G</text><text x="170" y="103">B</text><text x="230" y="103">A</text></g>
  <text x="284" y="120" fill="#888">frag out = vec4(ao, bent.xyz)</text>
  <text x="284" y="134" fill="#666" font-size="9">68 samples .r for ao, .gba for the direction</text>
</svg>

## The pass itself — shape, not scripture

With the traps handled, GTAO's core is compact: for each half-res pixel, reconstruct `view_pos` +
normal (from depth derivatives — `fwidth`-style cross of neighboring positions), then for **3
slices** (screen-space directions rotated by IGN) walk **6 steps** outward on each side, tracking the
maximum horizon angle that clears the tangent gate; the two horizons per slice integrate to a
visibility term (the GTAO closed form), averaged over slices. Output: `ao` — and, nearly free, the
**bent normal** (the average unoccluded direction), written alongside for 68 to consume:

```wgsl
    // per slice: dir = rotate(slice_dir, ign(pixel)); h1/h2 = max horizon over 6 steps each way
    // ao += integrate_arc(h1, h2, n_proj);   bent += arc_midpoint_dir;
    // → fragment out: vec4<f32>(ao / SLICES, encode(bent))   // R = ao, GBA = bent (RGBA16Float)
```

(3 × 6 × 2 ≈ 36 taps at quarter-pixel count ≈ **9 effective reads per full-res pixel**; the 5-tap
half-res blur adds ~1.3, composite upsample ~1.7 — the ~12 budget. The archive's mode-0 SSAO alone
was 48 full-res taps before its two 13-tap blurs.)

The blur: one 5-tap cross at half res, weights modulated by depth similarity (`exp(-|Δz|·k)`) so AO
never bleeds across silhouettes. The composite: bilinear-upsample AO but reject the 3 of 4 coarse
neighbors whose depth disagrees with the full-res pixel (depth-aware upsample — this is what keeps
half-res invisible at edges), then `scene_rgb * ao`.

## Verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Boxes on the ground grow soft **contact darkening**; interior corners and crease lines deepen; the
  stress drawing is untouched (lines don't occlude — AO reads depth, and tubes are thin).
- **The user-rule test, made mechanical:** screenshot mid-orbit, screenshot at rest, diff — identical
  AO (static IGN, fixed taps, no temporal anything). Then the counter-test: re-seed IGN with a frame
  counter and orbit — visible shimmer. Revert.
- **Grazing floor test:** camera low, looking across the ground — no stripes (trap 3's gate).
  Comment the gate out to see them once; restore it.
- Perf: on an iGPU, orbiting the stress file with AO on holds interactive rates, and idle is still
  **0 drawn/s** (70) — the entire AO cost rides only on frames that draw anyway.

## Recap

```
Ch 70: render-on-demand — idle is free.
Ch 71: GTAO, CONSTANT QUALITY. Scene → offscreen color; AO at HALF res in RGBA16Float (R=ao, GBA=bent), 3
       slices × 6 steps, horizons integrated closed-form; ONE 5-tap depth-aware blur; composite does
       depth-aware upsample × scene. Budget ≈ 12 reads/px/drawn-frame vs the archive's ~112.
       The five ported traps: (1) view-pos via ANALYTIC inv-projection — cheaper and exact
       (and the historic affine-only Xform::inverse bug, fixed in 41, lived here); (2) IGN
       noise, STATIC — per-frame jitter shimmers during rotation and violates the quality rule;
       (3) the tangent-plane gate dot(Δ,N) > len·0.07 + bias — mandatory or grazing floors
       stripe; (4) radius = %-of-bbox-diag, clamped — bbox is mm, view space is metres (×0.001),
       updated in set_scene; (5) depth via textureLoad sample 0 — sample count is DYNAMIC (1↔4),
       so force 4× with AO on (or carry both layouts) and recreate scene-color on the flip.
       Bent normal written beside AO — free from the horizon search, 72's input. No temporal,
       no adaptive, no idle refinement: one image, every frame.
```

Edited: `shaders/gtao.wgsl` + `shaders/blur5.wgsl` (NEW), `engine/gpu/targets.rs` (NEW — half-res
RGBA16F pair + scene color), `engine/pipelines/build.rs` (two pipelines + composite), `engine/gpu/mod.rs`
(pass order).

## Next

`72-arctic-gi.md` — the look, completed: sky-visibility from the bent normal (creases darken toward
the open sky — reads as real GI), Jimenez multi-bounce (bounce light from one polynomial), AO
micro-shadowing on the key light — all for zero extra texture fetches. Plus the B toggle.
