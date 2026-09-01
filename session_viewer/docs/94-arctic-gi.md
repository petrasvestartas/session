# 94 Arctic + cheap GI — a better default look for the same money

> **Big picture.** *Phase 11.* 72 bought occlusion data at ~12 reads/px; this lesson spends the same
> data three more ways — **zero additional texture fetches** — to fake global illumination well
> enough that flat-shaded CAD scenes read as photographed: directional sky occlusion from the bent
> normal, bounce light from a polynomial, and contact micro-shadows on the key light. The "arctic"
> preset (bright hemisphere white) becomes a toggle; the *default* look improves too.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the ao and bent normal from 72 feed three effects in the composite: sky visibility, multibounce, and key light micro shadowing" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="40" width="150" height="40" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
  <text x="85" y="57" fill="#d7dae0" text-anchor="middle">79's outputs</text>
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
src/state.rs / ui            # B toggle + settings checkbox (60)
```

Everything happens in the composite pass 72 created — the AO texture (with the bent normal encoded
beside it) is already bound; the scene color already flows through. This lesson is shader math.

## Step 1 — sky visibility: the bent normal earns its keep

Plain AO darkens uniformly. The **bent normal** — the average *unoccluded* direction 72 wrote for
free — says *which way* the opening faces. Weight the ambient hemisphere by how much of the **sky**
the bent normal sees, and creases darken toward the ground while upward-facing surfaces stay bright —
the directional cue that reads as real GI:

72 stored the bent normal as `bent*0.5+0.5` in the AO target's `gba` channels (79's `RGBA16Float`: `r`=ao,
`gba`=bent), so decoding is one line — add it beside `multi_bounce` at the top of `composite.wgsl`:

```wgsl
// decode 79's bent normal (stored bent*0.5+0.5 in gba). GUARD the normalize: on background
// pixels gba holds whatever the AO pass cleared to — (0,0,0) decodes to the zero vector and
// normalize(0) is NaN, which then poisons sky_vis, the multiply, and the whole pixel.
fn decode_bent(v: vec3<f32>) -> vec3<f32> {
    let b = v * 2.0 - 1.0;
    let l = length(b);
    if (l < 1e-4) { return vec3<f32>(0.0, 0.0, 1.0); }   // background: call it "sky-facing"
    return b / l;
}
```

Then, in the composite's fragment body where 72 did the flat `scene_rgb * ao`, replace that with:

```wgsl
    let bent = decode_bent(ao_tex_sample.gba);                 // ao_tex_sample = 79's upsampled texel (r=ao)
    let sky = clamp(dot(bent, vec3<f32>(0.0, 0.0, 1.0)) * 0.5 + 0.5, 0.0, 1.0);
    // arctic hemisphere floor 0.72 (archive value)
    let sky_vis = mix(0.72, 1.0, sky * ao);
    var ambient = scene_rgb * sky_vis;                          // scene_rgb = 79's lit color; there is no separate ambient term
```

## Step 2 — multi-bounce: light doesn't die in one hit

Real light bounces: a white floor's crease isn't black, it's *warm white*. Jimenez et al.'s
approximation reconstructs that from AO and albedo alone — one polynomial, no rays:

```wgsl
// GTAO multi-bounce (Jimenez 2016): brighter albedo returns more of the occluded energy.
// The second argument is ALBEDO — the unlit surface base color — NOT the lit scene color:
// scene_rgb already has the key light baked in (79's pipeline), and feeding it here
// double-counts the shading: dark-in-shadow pixels would "bounce" less light exactly where
// bounce matters most.
fn multi_bounce(ao: f32, albedo: vec3<f32>) -> vec3<f32> {
    let a = 2.0404 * albedo - 0.3324;
    let b = -4.7951 * albedo + 0.6417;
    let c = 2.7552 * albedo + 0.6903;
    return max(vec3<f32>(ao), ((ao * a + b) * ao + c) * ao);
}
    // usage: ambient *= multi_bounce(ao, albedo);   // instead of the flat `* ao`
```

Where does the composite get `albedo`? The honest route: the mesh pipeline writes the unlit base
color to a **second render target** (MRT, `@location(1)`) beside the lit color, and the composite
binds both. The cheap route — acceptable only because the arctic palette is near-white — is a
constant `vec3<f32>(0.9, 0.9, 0.92)`: the polynomial is forgiving near 1.0 and visibly wrong on
saturated dark materials. Take the MRT route the day a colored model looks muddy; until then, name
the constant `ARCTIC_ALBEDO_APPROX` so the compromise is greppable.

The visible difference: corners between white walls glow softly instead of going gray — occlusion
with *color memory*.

## Step 3 — micro-shadowing the key light

AO is ambient occlusion, but a pinch of it on the **key light** fakes the contact shadows a real sun
would cast — the single cheapest "grounded" cue there is:

72 outputs a single already-lit `scene_rgb` (the key is baked in at shade time in `triangle.wgsl`), so
the composite has no isolated key term to dim — restate the micro-shadow as a grounding multiply on the
lit color:

```wgsl
    // lifted so only strong occlusion darkens the contact
    let micro = clamp(ao * 1.3 - 0.3, 0.0, 1.0);
    ambient *= micro;                                          // contact shadow on the lit scene (no G-buffer to split the key out)
```

This applies in the **default** look too, not just arctic — the everyday view gets contact shadows
under every box for free.

## Step 4 — the arctic branch + dither + toggle

Arctic = the bright preset: hemisphere ambient dominant (the 0.72..1.0 band from Step 1), key light
soft, background near-white (70's ground already matches). One uniform flag branches the composite;
**B** toggles it and the 52 settings panel gets a checkbox (both just flip the flag + `poke()`, 71).

Last line before the swapchain — dither. A near-white gradient in 8 bits bands visibly; add the IGN
value (79's, static) scaled to one LSB:

```wgsl
    out_rgb += (ign(pixel_xy) - 0.5) * (1.0 / 255.0);   // breaks 8-bit banding, invisible as noise
```

And because the pieces were built one at a time, here is the **assembled composite**, once,
complete — every multiply this lesson added, in order:

```wgsl
    // the whole composition, end to end:
    //   bent      = decode_bent(ao_tex.gba)          // guarded normalize (Step 1)
    //   sky_vis   = mix(0.72, 1.0, (dot(bent,up)*0.5+0.5) * ao)
    //   out_rgb   = scene_rgb * sky_vis              // lit color × directional sky visibility
    //             * multi_bounce(ao, albedo)         // bounce light from ALBEDO (Step 2)
    //             * clamp(ao * 1.3 - 0.3, 0.0, 1.0)  // contact micro-shadow (Step 3)
    //             + (ign(pixel) - 0.5) / 255.0       // dither (above)
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
- The sky gradient (70's fade region) shows **no banding** stripes (Step 4's dither; comment it out
  on a large monitor to see them).
- Perf HUD: frame time unchanged from 72 — every effect here is arithmetic on data already fetched.
  And rotation still changes nothing about the image but the viewpoint.

## Recap

```
Ch 72: GTAO — ao + bent normal, ~12 reads/px.
Ch 73: THE LOOK, same money. Three effects, all in the composite, ZERO new fetches: (1) sky
       visibility — hemisphere weighted by dot(bent, up), the 0.72..1.0 arctic band; creases darken
       toward the sky opening = the directional cue that reads as GI. (2) Jimenez multi-bounce — one
       polynomial of (ao, albedo) returns energy by albedo; white corners glow instead of graying.
       (3) micro-shadow — key light × lifted AO = contact shadows, ON BY DEFAULT, not just arctic.
       Arctic itself = a uniform branch, B key + checkbox (poke on toggle, 71). IGN dither (±½ LSB,
       static) kills 8-bit banding on the bright gradients. decode_bent is GUARDED — normalize(0)
       on background pixels is a NaN that poisons the whole pixel. Multi-bounce takes ALBEDO, not
       the lit scene_rgb (MRT base-color target when the constant approximation shows). Default
       look visibly better; arctic ≥ the archive's, at a tenth the fetch budget.
```

Edited: `shaders/composite.wgsl` (sky-vis, multi-bounce, micro-shadow, arctic branch, dither),
`state.rs`/`ui` (B toggle + checkbox).

## Next

`95-outline-aa.md` — the last archive-look piece: the crisp selection outline, rebuilt without the
per-frame tax — mask passes render only when dirty (78), the 81-tap box search becomes two separable
1×N passes, and FXAA retires (4× MSAA already covers geometry).
