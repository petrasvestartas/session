# 68 Arctic + cheap GI — a better default look for the same money

> **Big picture.** *Phase 11.* 67 bought occlusion data at ~12 reads/px; this lesson spends the same
> data three more ways — **zero additional texture fetches** — to fake global illumination well
> enough that flat-shaded CAD scenes read as photographed: directional sky occlusion from the bent
> normal, bounce light from a polynomial, and contact micro-shadows on the key light. The "arctic"
> preset (bright hemisphere white) becomes a toggle; the *default* look improves too.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the ao and bent normal from 67 feed three effects in the composite: sky visibility, multibounce, and key light micro shadowing" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="40" width="150" height="40" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
  <text x="85" y="57" fill="#d7dae0" text-anchor="middle">67's outputs</text>
  <text x="85" y="72" fill="#666" text-anchor="middle" font-size="10">ao · bent normal</text>
  <g stroke="#6fb3ff" stroke-width="1.2"><line x1="160" y1="50" x2="240" y2="28" marker-end="url(#ah68)"/><line x1="160" y1="60" x2="240" y2="60" marker-end="url(#ah68)"/><line x1="160" y1="70" x2="240" y2="92" marker-end="url(#ah68)"/></g>
  <g fill="none" stroke="#3a3a3a"><rect x="244" y="14" width="250" height="26"/><rect x="244" y="48" width="250" height="26"/><rect x="244" y="82" width="250" height="26"/></g>
  <g fill="#d7dae0" font-size="10">
    <text x="254" y="31">sky visibility: hemisphere × dot(bent, up) — crease→sky darkening</text>
    <text x="254" y="65">multi-bounce: one polynomial of (ao, albedo) — free bounce light</text>
    <text x="254" y="99">micro-shadow: key light × saturate(ao lifted) — grounded contacts</text>
  </g>
  <text x="580" y="60" fill="#888" text-anchor="middle">all in the</text>
  <text x="580" y="74" fill="#888" text-anchor="middle">composite —</text>
  <text x="580" y="88" fill="#888" text-anchor="middle">0 extra fetches</text>
  <defs><marker id="ah68" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/shaders/composite.wgsl   # the three effects + arctic branch + IGN output dither
src/state.rs / ui            # B toggle + settings checkbox (47)
```

Everything happens in the composite pass 67 created — the AO texture (with the bent normal encoded
beside it) is already bound; the scene color already flows through. This lesson is shader math.

## Step 1 — sky visibility: the bent normal earns its keep

Plain AO darkens uniformly. The **bent normal** — the average *unoccluded* direction 67 wrote for
free — says *which way* the opening faces. Weight the ambient hemisphere by how much of the **sky**
the bent normal sees, and creases darken toward the ground while upward-facing surfaces stay bright —
the directional cue that reads as real GI:

```wgsl
    let bent = decode_bent(ao_tex_sample.gba);                 // 67's encoding
    let sky = clamp(dot(bent, vec3<f32>(0.0, 0.0, 1.0)) * 0.5 + 0.5, 0.0, 1.0);
    let sky_vis = mix(0.72, 1.0, sky * ao);                    // arctic hemisphere floor 0.72 (archive value)
    var ambient = base_ambient * sky_vis;
```

## Step 2 — multi-bounce: light doesn't die in one hit

Real light bounces: a white floor's crease isn't black, it's *warm white*. Jimenez et al.'s
approximation reconstructs that from AO and albedo alone — one polynomial, no rays:

```wgsl
// GTAO multi-bounce (Jimenez 2016): brighter albedo returns more of the occluded energy.
fn multi_bounce(ao: f32, albedo: vec3<f32>) -> vec3<f32> {
    let a = 2.0404 * albedo - 0.3324;
    let b = -4.7951 * albedo + 0.6417;
    let c = 2.7552 * albedo + 0.6903;
    return max(vec3<f32>(ao), ((ao * a + b) * ao + c) * ao);
}
    // usage: ambient *= multi_bounce(ao, scene_rgb);   // instead of the flat `* ao`
```

The visible difference: corners between white walls glow softly instead of going gray — occlusion
with *color memory*.

## Step 3 — micro-shadowing the key light

AO is ambient occlusion, but a pinch of it on the **key light** fakes the contact shadows a real sun
would cast — the single cheapest "grounded" cue there is:

```wgsl
    let micro = clamp(ao * 1.3 - 0.3, 0.0, 1.0);   // lifted so only strong occlusion shadows the key
    let key = key_light_term * micro;
```

This applies in the **default** look too, not just arctic — the everyday view gets contact shadows
under every box for free.

## Step 4 — the arctic branch + dither + toggle

Arctic = the bright preset: hemisphere ambient dominant (the 0.72..1.0 band from Step 1), key light
soft, background near-white (65's ground already matches). One uniform flag branches the composite;
**B** toggles it and the 47 settings panel gets a checkbox (both just flip the flag + `poke()`, 66).

Last line before the swapchain — dither. A near-white gradient in 8 bits bands visibly; add the IGN
value (67's, static) scaled to one LSB:

```wgsl
    out_rgb += (ign(pixel_xy) - 0.5) * (1.0 / 255.0);   // breaks 8-bit banding, invisible as noise
```

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **Default look:** boxes now sit *in* the scene — contact micro-shadow under every edge that touches
  the ground (Step 3). Before/after screenshot makes it obvious.
- **B** → arctic: bright, white, studio-like; creases shade toward warm gray, not dead gray (Step 2),
  and a box's underside is darker than its crease-facing-up (Step 1 — cover the bent-normal term with
  a flat `ao` to see the difference: the directionality vanishes).
- The sky gradient (65's fade region) shows **no banding** stripes (Step 4's dither; comment it out
  on a large monitor to see them).
- Perf HUD: frame time unchanged from 67 — every effect here is arithmetic on data already fetched.
  And rotation still changes nothing about the image but the viewpoint.

## Recap

```
Ch 67: GTAO — ao + bent normal, ~12 reads/px.
Ch 68: THE LOOK, same money. Three effects, all in the composite, ZERO new fetches: (1) sky
       visibility — hemisphere weighted by dot(bent, up), the 0.72..1.0 arctic band; creases darken
       toward the sky opening = the directional cue that reads as GI. (2) Jimenez multi-bounce — one
       polynomial of (ao, albedo) returns energy by albedo; white corners glow instead of graying.
       (3) micro-shadow — key light × lifted AO = contact shadows, ON BY DEFAULT, not just arctic.
       Arctic itself = a uniform branch, B key + checkbox (poke on toggle, 66). IGN dither (±½ LSB,
       static) kills 8-bit banding on the bright gradients. Default look visibly better; arctic ≥
       the archive's, at a tenth the fetch budget.
```

Edited: `shaders/composite.wgsl` (sky-vis, multi-bounce, micro-shadow, arctic branch, dither),
`state.rs`/`ui` (B toggle + checkbox).

## Next

`69-outline-aa.md` — the last archive-look piece: the crisp selection outline, rebuilt without the
per-frame tax — mask passes render only when dirty (66), the 81-tap box search becomes two separable
1×N passes, and FXAA retires (4× MSAA already covers geometry).
