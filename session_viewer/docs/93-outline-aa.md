# 92 Selection outline + AA polish — the look, without the tax

> **Big picture.** *Phase 11 closes.* The archive's selection outline was its most expensive habit:
> an extra full-scene mask rasterization plus an **81-tap** box search in the composite — every
> frame, selection or not, even with everything off. Same crisp result here, three structural fixes:
> the mask renders **only when dirty** (78's flag, scoped), the search becomes **separable** (two
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
  <text x="340" y="112" fill="#888" text-anchor="middle">nothing selected → all three passes skip; static scene → nothing renders at all (78)</text>
  <defs><marker id="ah69" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/shaders/outline_mask.wgsl    # NEW — selected instances only, white on black (MSAA coverage)
src/shaders/outline_sep.wgsl     # NEW — the two 1×N distance passes (one shader, direction uniform)
src/shaders/composite.wgsl       # + outline ramp over the 73 output
src/engine/gpu/mod.rs            # mask/dist targets; the passes, gated on selection + dirty
```

## Step 1 — the mask, MSAA coverage as free AA: `outline_mask.wgsl`

Render *only the selected instances* (the vs reads `FLAG_SELECTED`; unselected rows collapse to w=0 —
the 41 trick inverted) as flat white into a small offscreen target, **4× MSAA, resolved**. The
resolve is the magic: edge pixels come out fractionally gray (coverage), so the outline inherits
sub-pixel smoothness before any blur exists — the archive's technique, and the real payoff of
lesson 24's MSAA decision.

The shader is the scene pipelines' vertex stages minus all color logic — one vs entry per lane in
one module (a WGSL file may hold several; each mask pipeline picks its entry), one shared `fs`
returning white. The mesh and cylinder entries are below; the **ribbon, sphere and dot lanes get
the same treatment** — copy each lane's vs, drop the color, add the selection gate — because the
mask must cover everything selectable (Step 4). Half or full res — half is fine, the ramp hides it:

```wgsl
// outline_mask.wgsl — selected instances only, flat white on black.
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

struct CylinderSegment {
    p0: vec3<f32>,
    radius: f32,
    p1: vec3<f32>,
    instance_id: u32,
    color: vec4<f32>,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
};

const FLAG_SELECTED: u32 = 1u;   // bit 0 (58)
const COLLAPSED: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);   // w=0 → clipped, the 41 trick

// triangle.wgsl's vs minus normals/colors — only position + the row id matter here.
@vertex
fn vs_mesh(@location(0) position: vec3<f32>,
           @location(3) inst_id: u32) -> @builtin(position) vec4<f32> {
    let inst = instances[inst_id];
    if ((inst.flags & FLAG_SELECTED) == 0u) { return COLLAPSED; }
    return mvp * (inst.model * vec4<f32>(position, 1.0));
}

// cylinder.wgsl's vs verbatim, minus the color output, plus the selection gate.
@vertex
fn vs_cyl(@location(0) tmpl: vec3<f32>,
          @builtin(instance_index) si: u32) -> @builtin(position) vec4<f32> {
    let seg = segments[si];
    let inst = instances[seg.instance_id];
    if ((inst.flags & FLAG_SELECTED) == 0u) { return COLLAPSED; }
    let w0 = (inst.model * vec4<f32>(seg.p0, 1.0)).xyz;
    let w1 = (inst.model * vec4<f32>(seg.p1, 1.0)).xyz;
    let axis = w1 - w0;
    let len = length(axis);
    let dir = select(vec3<f32>(0.0, 0.0, 1.0), axis / len, len > 1e-9);
    let ref0 = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 0.0, 0.0), abs(dir.z) > 0.9);
    let right = normalize(cross(ref0, dir));
    let up = cross(dir, right);
    let center = w0 + dir * (len * tmpl.z);
    let clip_c = mvp * vec4<f32>(center, 1.0);
    var r = line.thickness * clip_c.w / (line.proj_y * line.vp_h);
    if (line.ortho_h > 0.0) { r = line.thickness * line.ortho_h / line.vp_h; }
    if (seg.radius > 0.0) { r = seg.radius; }
    let world = center + (right * tmpl.x + up * tmpl.y) * r;
    return mvp * vec4<f32>(world, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0);
}
```

The mask pipelines are the scene pipelines' shapes with this module's entries, a single `R8Unorm`
color target (no depth attachment — the ring shows through occluders, which is what you want for
"where is my selection"), and `multisample.count = 4`. Pin that 4 explicitly: it is the *mask
target's* sample count, **independent of the scene's dynamic samples** (35's `msaa_for` flips 1↔4
and `set_scene` rebuilds the scene pipelines on the flip — the mask set is keyed to its own target
and never needs that rebuild).

## Step 2 — separable distance: `outline_sep.wgsl`

"Is any mask pixel within R of me?" — the archive answered with an R×R box (81 taps at R=4). The
separable identity: horizontal pass stores the nearest-mask distance along the row; vertical pass
combines column-neighbors' row-distances into the true 2-D distance. Two passes × (2R+1) taps = 18:

Both passes are the **same** formula — `out = min over k of sqrt(sample(p + k·dir)² + k²)` — once pass 1
seeds the mask as a distance (on-mask → 0, off → ∞): then `sqrt(0² + k²) = |k|` (pass 1) and
`sqrt(dist1² + k²)` (pass 2) fall out of one line. So it's genuinely one shader with a `dir` + `seed`
uniform, ping-ponging two R16Float targets:

```wgsl
struct Sep { dir: vec2<f32>, radius: f32, seed: f32 };  // dir=(1,0)|(0,1); seed=1 on pass 1 (mask in)
@group(0) @binding(0) var<uniform> sep: Sep;
@group(0) @binding(1) var src: texture_2d<f32>;         // pass1: mask coverage; pass2: dist field

// INF must FIT the target: R16Float tops out at 65504 — write 1e9 and you store inf, and
// inf * 0 in a later combine is NaN. 60000 is "infinitely far" at any sane outline radius.
const INF: f32 = 60000.0;

// pass1: coverage>0.5 → on-mask distance 0, else INF. pass2: src already IS a distance — pass through.
fn seed(c: vec2<i32>) -> f32 {
    let dim = vec2<i32>(textureDimensions(src));
    let d = textureLoad(src, clamp(c, vec2<i32>(0), dim - vec2<i32>(1)), 0).r;
    return select(d, select(INF, 0.0, d > 0.5), sep.seed > 0.5);
}

@fragment
fn fs_main(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
    let p = vec2<i32>(i32(frag.x), i32(frag.y));
    let stp = vec2<i32>(i32(sep.dir.x), i32(sep.dir.y));
    let r = i32(sep.radius);
    var best = INF;
    for (var k = -r; k <= r; k = k + 1) {
        let d = seed(p + stp * k);
        best = min(best, sqrt(d*d + f32(k*k)));         // separable euclidean combine
    }
    return vec4<f32>(best, 0.0, 0.0, 1.0);              // R16Float distance
}
```

Add the same 6-line fullscreen-triangle `vs_main` the GTAO/composite shaders carry (70's trick) to
the top of this file too — a pipeline's vertex stage lives in its own module here:

```wgsl
@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4<f32> {
    let xy = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u)) * 2.0 - 1.0;
    return vec4<f32>(xy, 0.0, 1.0);
}
```

Pass 1 binds the mask with `seed=1, dir=(1,0)` → dist1; pass 2 binds dist1 with `seed=0, dir=(0,1)`
→ dist2 (the ping-pong). Because a `write_buffer` lands before the whole submit, the two passes
need **two** small `Sep` uniform buffers + bind groups (one per pass), not one rewritten in between.
The coverage grays from Step 1 ride along: mask > 0.5 is inside, and the fractional value feeds the
ramp for the anti-aliased inner edge.

## Step 3 — the ramp in composite: `composite.wgsl`

Declare Step 2's output as a new binding in composite's post-process group (the next free slot after
79's AO texture; reuse the fullscreen `samp` sampler the pass already binds), plus the ring width:

```wgsl
// find the composite binding block (79's AO texture) → add after it:
@group(0) @binding(4) var outline_dist: texture_2d<f32>;   // Step 2's R16Float distance field

const WIDTH: f32 = 3.0;   // ring radius, px
```

Then, in the composite `fs_main` (right before writing `out_rgb` to the swapchain), derive `inside`
from the distance field itself — `d == 0` exactly on the mask, growing outward — so no separate mask
binding is needed:

```wgsl
    let d = textureSample(outline_dist, samp, uv).r;
    let inside = 1.0 - smoothstep(0.0, 1.0, d);               // ~1 on the mask, 0 one px out
    let ring = 1.0 - smoothstep(WIDTH - 1.0, WIDTH, d);        // crisp 1 px falloff at radius WIDTH
    let sel_color = vec3<f32>(1.0, 0.72, 0.1);
    out_rgb = mix(out_rgb, sel_color, ring * (1.0 - inside));  // ring outside the object only
```

## Step 4 — targets, passes, and the gating where the tax dies: `engine/gpu/mod.rs`

The plumbing is 79's post-process pattern, one more time. **Targets** — beside 79's half-res AO pair
in `targets.rs`: `outline_mask_msaa` (`R8Unorm`, `sample_count: 4`, `RENDER_ATTACHMENT`),
`outline_mask` (`R8Unorm`, 1×, `RENDER_ATTACHMENT | TEXTURE_BINDING` — the resolve target),
`outline_dist_a` / `outline_dist_b` (`R16Float`, 1×, `RENDER_ATTACHMENT | TEXTURE_BINDING`), all at
half resolution, rebuilt in `resize()` with the rest — and because the sep and composite **bind
groups** reference these half-res views, rebuild those bind groups in `resize()` too (a bind group
keeps holding the old, dead view otherwise). **Passes** — in `clear(color, &view_proj)`, today's
frame entry point, between 79's blur pass and the composite (the composite samples
`outline_dist_b`, so it must be final by then):

```rust
        // ---- outline (74): mask → separable distance ×2 — only when it can matter ----
        if self.outline_needed {
            let mut mp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("outline.mask"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.outline_mask_msaa, resolve_target: Some(&self.outline_mask),
                    depth_slice: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None,      // no depth — the ring shows through occluders
                occlusion_query_set: None, timestamp_writes: None, multiview_mask: None,
            });
            mp.set_pipeline(&self.pipelines.outline_mask_mesh);
            mp.set_bind_group(0, &self.mvp_bind_group, &[]);
            mp.set_bind_group(1, &self.line_bind_group, &[]);
            mp.set_bind_group(2, &self.instance_bind_group, &[]);
            mp.set_bind_group(3, &self.segment_bind_group, &[]);
            mp.set_vertex_buffer(0, self.arena_vbo.slice(..));
            mp.set_vertex_buffer(1, self.arena_vids.slice(..));
            mp.set_index_buffer(self.arena_ibo.slice(..), wgpu::IndexFormat::Uint32);
            mp.draw_indexed(0..self.arena_index_count, 0, 0..1);
            // Linework + vertex ink: mirror Gpu::clear's FOUR lane sub-draws exactly.
            // segments = pipes ‖ ribbons (cylinders 0..pipe_count, then procedural verts
            // 6*pipe_count..6*segment_count); glyphs = spheres ‖ dots (0..sphere_count, then
            // 3*sphere_count..3*glyph_count). A single 0..segment_count cylinder draw would give
            // selected flat linework a wrong-shaped ring — and selected dots none at all.
            if self.pipe_count > 0 {
                mp.set_pipeline(&self.pipelines.outline_mask_cyl);
                mp.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
                mp.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                mp.draw_indexed(0..self.cyl_index_count, 0, 0..self.pipe_count);
            }
            if self.segment_count > self.pipe_count {
                mp.set_pipeline(&self.pipelines.outline_mask_ribbon);
                mp.draw(6 * self.pipe_count..6 * self.segment_count, 0..1);
            }
            if self.sphere_count > 0 {
                mp.set_pipeline(&self.pipelines.outline_mask_sphere);
                mp.set_bind_group(3, &self.glyph_bind_group, &[]);   // glyph table, as in clear()
                mp.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                mp.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                mp.draw_indexed(0..self.sph_index_count, 0, 0..self.sphere_count);
            }
            if self.glyph_count > self.sphere_count {
                mp.set_pipeline(&self.pipelines.outline_mask_glyph);
                mp.set_bind_group(3, &self.glyph_bind_group, &[]);
                mp.draw(3 * self.sphere_count..3 * self.glyph_count, 0..1);
            }
            drop(mp);
            // two fullscreen sep passes — each its own pre-written Sep uniform + bind group
            for (target, bind) in [(&self.outline_dist_a, &self.outline_sep_h_bind_group),
                                   (&self.outline_dist_b, &self.outline_sep_v_bind_group)] {
                let mut sp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("outline.sep"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: target, resolve_target: None, depth_slice: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None, timestamp_writes: None, multiview_mask: None,
                });
                sp.set_pipeline(&self.pipelines.outline_sep);
                sp.set_bind_group(0, bind, &[]);
                sp.draw(0..3, 0..1);
            }
        }
```

(The five mask pipelines — mesh + the four ink lanes — the sep pipeline, and the bind groups are
built once in `Gpu::new`, the same `build_*_pipeline` + layout/bind-group moves as 79's gtao/blur
passes, nothing new in shape. The duplication with `clear()`'s lane split begs a small shared
helper — a `draw_ink_lanes(pass, &LanePipelines)` both call — so the mask can never drift from the
main pass when a lane is added. The `h` bind group binds `outline_mask` as `src` with
`Sep { dir: (1,0), radius: 4.0, seed: 1.0 }`; the `v` one binds `outline_dist_a` with
`dir: (0,1), seed: 0.0`.)

> **The WGSL duplication needs a rule too.** The mask's lane vertex stages are *copies* of
> `triangle.wgsl`/`cylinder.wgsl`/etc. minus color — and WGSL in wgpu has no `#include`, so there
> is physically one source of truth per file. When `cylinder.wgsl`'s vs changes (a new radius
> rule, a new lane field), the mask copy goes silently stale and the outline no longer matches the
> geometry it rings. The cheap discipline: a reciprocal comment at the top of each lane shader —
> `// mirror: outline_mask.wgsl vs_cyl — change both or neither` — and one selftest (88) that
> draws a selected known shape and asserts the ring hugs it. If a compose tool (naga_oil-style)
> ever enters the dependency tree, these copies are the first thing to unify.
>
> **Orbit-with-selection rasterizes the selection twice.** Every camera change re-renders the
> mask — the selected geometry's vertex work runs once for the scene and once for the mask, per
> drawn frame. At this course's scale that's nothing; at heavy-scene scale the alternatives are:
> derive the outline from the **stencil/depth** of the main pass (mark selected rows with a
> stencil ref during the scene draw — zero extra rasterization, but the ring then respects
> occlusion instead of showing through), or drive the mask from a **compacted indirect draw**
> (a tiny compute pass gathers selected rows into an indirect args buffer, so the mask pass
> never touches unselected vertices). Neither is built here; the design leaves the door open
> because the mask pass is already fully self-bound.

**The gate** — computed by `State` and stored as a `Gpu` field (`self.gpu.outline_needed = …;`)
right before the `clear(color, &view_proj)` call: that signature is today's entry point and stays
untouched — the flag rides on `Gpu` instead of a new parameter. Get the gating exactly right, or
the ring **stays on screen after deselect** (the composite always samples `outline_dist_b`; skip
the sep passes on the deselect frame and the stale distance field keeps drawing the ring forever).
The truth table:

| selection | changed (selection or camera) | mask pass | sep passes | what the composite shows |
|---|---|---|---|---|
| nonempty | yes | re-render | run | ring follows |
| nonempty | no | skip | skip | reuses `outline_dist_b` — still valid, nothing moved |
| **just emptied** | yes | re-render — every draw collapses (FLAG_SELECTED) → **black** | run **once** | distance → INF, ring erased |
| empty | no | skip | skip | `outline_dist_b` is already INF |

So the flag is:

```rust
    // deselect MUST re-run (erase the ring); a camera move with an empty selection needn't
    // (nothing to show, nothing stale); a static selected frame reuses the field.
    let outline_needed = selection_changed || (camera_changed && selection_nonempty);
    // …and with 71, a fully static frame doesn't even reach here.
```

Note the deselect case needs no special clear path: the mask pass re-runs with zero selected rows,
every instance collapses in the vertex shader, and the resolved mask comes out black — the two sep
passes then wash the distance field to INF on their own.

Nothing selected and nothing changing → zero passes, zero cost — the archive ran the full chain
regardless. And FXAA:
simply not built. Geometry edges are MSAA-resolved (24), the outline is coverage-antialiased (Step
1), the UI is egui's own AA — there is nothing left for FXAA to fix, only things for it to soften. If
a future screen-space effect ever aliases, add it *optional and off*.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select an object → a crisp amber ring, ~3 px, smooth on diagonals (coverage + ramp), hugging the
  silhouette — including through holes (47's trimmed surface outlines its hole rim too: the mask is
  geometry, not a bounding shape).
- Watch `drawn/s` (78) and frame time: selection sitting still = **no outline passes re-run**;
  orbiting with a selection = outline follows at full rate; nothing selected = identical numbers to
  73. The archive's version cost the full chain in all three states.
- Select something, then **deselect it** (Esc / click empty space) → the ring is *gone*, not
  frozen on screen — the just-emptied row of the truth table (mask re-renders black, sep washes
  the field to INF, once).
- Zoom text/lines: no FXAA smear — lines are exactly as 31 drew them.

## Recap

```
Ch 73: arctic GI — the look, cheap.
Ch 74: OUTLINE, structural fixes only. Mask = selected instances rendered white (FLAG_SELECTED
       inverted-collapse), 4× MSAA RESOLVED — fractional coverage IS the anti-aliasing (24's
       payoff); its draws mirror clear()'s four ink-lane sub-draws (cyl/ribbon/sphere/dot), its
       4× is pinned independent of the scene's dynamic samples.
       Distance = separable: 1×N then N×1 (exact euclidean via row-distance + k²), 18 taps for what
       the archive's box search did in 81; INF = 60000.0 because the field is R16Float (1e9 stores
       as inf, and inf×0 is NaN). Ramp = smoothstep at WIDTH in composite, ring outside
       only. Gated by the TRUTH TABLE: selection+change → full chain; static → nothing (78);
       JUST-EMPTIED → mask re-renders black (all draws collapse) + sep once, or the ring sticks
       after deselect. FXAA retired —
       MSAA + coverage + egui already cover every edge class; a second AA is just blur. Identical
       look, zero standing tax. Phase 11 complete: ground, on-demand, GTAO, GI, outline — the arctic
       viewer, engineered fast.
```

Edited: `shaders/outline_mask.wgsl` + `shaders/outline_sep.wgsl` (NEW), `shaders/composite.wgsl`
(ramp), `engine/gpu/mod.rs` (targets, gated passes).

## Next

`93-scene-tree.md` — Phase 12: scene management UI. The Session's tree appears in a panel —
virtualized so a thousand rows scroll smoothly, with an eye icon driving 59's visibility per object.
