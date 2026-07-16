# 69 Selection outline + AA polish — the look, without the tax

> **Big picture.** *Phase 11 closes.* The archive's selection outline was its most expensive habit:
> an extra full-scene mask rasterization plus an **81-tap** box search in the composite — every
> frame, selection or not, even with everything off. Same crisp result here, three structural fixes:
> the mask renders **only when dirty** (66's flag, scoped), the search becomes **separable** (two
> 1×N passes — 18 taps replaces 81), and FXAA **retires** (24's 4× MSAA already smooths geometry;
> a second AA pass just blurs it). Quality identical; the tax gone.

<svg viewBox="0 0 680 140" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="selected objects render once into a small mask when selection changes; a horizontal then vertical distance pass expands it; composite draws a crisp ramp where distance is under the width" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="30" width="160" height="40" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <text x="90" y="47" fill="#d7dae0" text-anchor="middle">mask: selected only</text>
  <text x="90" y="61" fill="#666" text-anchor="middle" font-size="9">renders ONLY on selection/camera change</text>
  <rect x="210" y="30" width="120" height="40" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <text x="270" y="47" fill="#d7dae0" text-anchor="middle">1×N horizontal</text><text x="270" y="61" fill="#666" text-anchor="middle" font-size="9">nearest-mask distance</text>
  <rect x="370" y="30" width="120" height="40" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <text x="430" y="47" fill="#d7dae0" text-anchor="middle">N×1 vertical</text><text x="430" y="61" fill="#666" text-anchor="middle" font-size="9">separable: 2·9 taps, not 81</text>
  <rect x="530" y="30" width="140" height="40" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <text x="600" y="47" fill="#d7dae0" text-anchor="middle">composite ramp</text><text x="600" y="61" fill="#666" text-anchor="middle" font-size="9">crisp 1 px falloff</text>
  <g stroke="#6fb3ff" stroke-width="1.3"><line x1="170" y1="50" x2="208" y2="50" marker-end="url(#ah69)"/><line x1="330" y1="50" x2="368" y2="50" marker-end="url(#ah69)"/><line x1="490" y1="50" x2="528" y2="50" marker-end="url(#ah69)"/></g>
  <text x="340" y="112" fill="#888" text-anchor="middle">nothing selected → all three passes skip; static scene → nothing renders at all (66)</text>
  <defs><marker id="ah69" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/shaders/outline_mask.wgsl    # NEW — selected instances only, white on black (MSAA coverage)
src/shaders/outline_sep.wgsl     # NEW — the two 1×N distance passes (one shader, direction uniform)
src/shaders/composite.wgsl       # + outline ramp over the 68 output
src/engine/gpu/mod.rs            # mask/dist targets; the passes, gated on selection + dirty
```

## Step 1 — the mask, MSAA coverage as free AA: `outline_mask.wgsl`

Render *only the selected instances* (the vs reads `FLAG_SELECTED`; unselected rows collapse to w=0 —
the 37 trick inverted) as flat white into a small offscreen target, **4× MSAA, resolved**. The
resolve is the magic: edge pixels come out fractionally gray (coverage), so the outline inherits
sub-pixel smoothness before any blur exists — the archive's technique, and the real payoff of
lesson 24's MSAA decision.

The whole shader is `cylinder/mesh vs` minus color logic, `fs` returning `vec4(1.0)`. Half or full
res — half is fine, the ramp hides it.

## Step 2 — separable distance: `outline_sep.wgsl`

"Is any mask pixel within R of me?" — the archive answered with an R×R box (81 taps at R=4). The
separable identity: horizontal pass stores the nearest-mask distance along the row; vertical pass
combines column-neighbors' row-distances into the true 2-D distance. Two passes × (2R+1) taps = 18:

```wgsl
// pass 1 (dir = (1,0)): out = min over k in -R..R of (mask(x+k,y) ? |k| : INF)
// pass 2 (dir = (0,1)): out = min over k of sqrt(dist1(x,y+k)² + k²)   — exact euclidean, separable
```

One shader, a `dir` uniform, ping-pong between two R16Float targets. (The coverage grays from Step 1
ride along: treat mask > 0.5 as inside, and carry the fractional value into the ramp for the
anti-aliased inner edge.)

## Step 3 — the ramp in composite: `composite.wgsl`

```wgsl
    let d = textureSample(outline_dist, samp, uv).r;
    let ring = 1.0 - smoothstep(WIDTH - 1.0, WIDTH, d);        // crisp 1 px falloff at radius WIDTH
    let sel_color = vec3<f32>(1.0, 0.72, 0.1);
    out_rgb = mix(out_rgb, sel_color, ring * (1.0 - inside));  // ring outside the object only
```

## Step 4 — the gating, where the tax dies: `engine/gpu/mod.rs`

```rust
    // the three outline passes run ONLY when:
    let outline_needed = selection_nonempty && (selection_changed || camera_changed);
    // …and with 66, a fully static frame doesn't even reach here.
```

Nothing selected → zero passes, zero cost — the archive ran the full chain regardless. And FXAA:
simply not built. Geometry edges are MSAA-resolved (24), the outline is coverage-antialiased (Step
1), the UI is egui's own AA — there is nothing left for FXAA to fix, only things for it to soften. If
a future screen-space effect ever aliases, add it *optional and off*.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select an object → a crisp amber ring, ~3 px, smooth on diagonals (coverage + ramp), hugging the
  silhouette — including through holes (64's trimmed surface outlines its hole rim too: the mask is
  geometry, not a bounding shape).
- Watch `drawn/s` (66) and frame time: selection sitting still = **no outline passes re-run**;
  orbiting with a selection = outline follows at full rate; nothing selected = identical numbers to
  68. The archive's version cost the full chain in all three states.
- Zoom text/lines: no FXAA smear — lines are exactly as 31 drew them.

## Recap

```
Ch 68: arctic GI — the look, cheap.
Ch 69: OUTLINE, structural fixes only. Mask = selected instances rendered white (FLAG_SELECTED
       inverted-collapse), 4× MSAA RESOLVED — fractional coverage IS the anti-aliasing (24's
       payoff).
       Distance = separable: 1×N then N×1 (exact euclidean via row-distance + k²), 18 taps for what
       the archive's box search did in 81. Ramp = smoothstep at WIDTH in composite, ring outside
       only. Gated: no selection → no passes; unchanged frame → nothing at all (66). FXAA retired —
       MSAA + coverage + egui already cover every edge class; a second AA is just blur. Identical
       look, zero standing tax. Phase 11 complete: ground, on-demand, GTAO, GI, outline — the arctic
       viewer, engineered fast.
```

Edited: `shaders/outline_mask.wgsl` + `shaders/outline_sep.wgsl` (NEW), `shaders/composite.wgsl`
(ramp), `engine/gpu/mod.rs` (targets, gated passes).

## Next

`70-scene-tree.md` — Phase 12: scene management UI. The Session's tree appears in a panel —
virtualized so a thousand rows scroll smoothly, with an eye icon driving 46's visibility per object.
