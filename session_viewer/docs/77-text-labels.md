# 77 Text labels — names in the scene

> **Big picture.** *Phase 12 closes.* The tree names objects in a panel; big assemblies want names
> **in the scene** — floating at the object, facing the camera, readable at any angle. The technique
> is 32b's billboard trick grown up: a **glyph atlas** (every character rasterized once into one
> texture) and one quad per character, all labels in **one draw call**. This is also the course's
> first textured draw — the atlas is its first texture bind group (90 binds another, on the triangle
> pipeline: different pipeline, same group slot — no contention).

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a font atlas texture holds every glyph once; each label character becomes a billboarded quad sampling its glyph rectangle; all labels draw in one call" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="16" y="20" width="130" height="80" fill="none" stroke="#3a3a3a"/>
  <g fill="#888" font-size="12"><text x="24" y="40">A B C D E</text><text x="24" y="60">a b c d e</text><text x="24" y="80">0 1 2 3 .</text></g>
  <text x="81" y="114" fill="#888" text-anchor="middle">atlas — rasterized ONCE</text>
  <line x1="156" y1="60" x2="216" y2="60" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah72)"/>
  <g transform="translate(230,26)">
    <rect x="0" y="18" width="16" height="20" fill="none" stroke="#6fb3ff"/><rect x="17" y="18" width="16" height="20" fill="none" stroke="#6fb3ff"/><rect x="34" y="18" width="16" height="20" fill="none" stroke="#6fb3ff"/><rect x="51" y="18" width="16" height="20" fill="none" stroke="#6fb3ff"/>
    <text x="34" y="12" fill="#d7dae0" text-anchor="middle">"wall"</text>
    <text x="34" y="58" fill="#666" text-anchor="middle" font-size="10">1 quad / char, uv → its atlas rect</text>
  </g>
  <g transform="translate(420,20)">
    <text x="0" y="16" fill="#888">anchor: 3-D point → clip (mvp)</text>
    <text x="0" y="34" fill="#888">offset: SCREEN px, after projection</text>
    <text x="0" y="52" fill="#888">→ faces the camera, constant size</text>
    <text x="0" y="80" fill="#666" font-size="10">every label in the scene: ONE draw</text>
  </g>
  <defs><marker id="ah72" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
# NEW — font atlas build + TextVertex quad assembly (archive text.rs recipe)
src/engine/text.rs
src/shaders/text.wgsl          # NEW — billboard vs (32b's trick) + atlas-sampling fs
src/engine/pipelines/build.rs  # build_text_pipeline(…, samples) — first pipeline with a texture bind group
src/engine/gpu/mod.rs          # label buffer + TextUniform + one draw, after the gumball, before egui
```

## Step 1 — the atlas: `src/engine/text.rs`

Rasterize a monospace set (ASCII 32–126 is plenty for names) into one `R8Unorm` texture at load —
the archive bakes a small bitmap font directly (`create_font_atlas(&device, &queue)`); a fixed
cell grid makes UVs trivial:

```rust
pub const CELL_W: u32 = 8;  pub const CELL_H: u32 = 14;  pub const COLS: u32 = 16;
const ROWS: u32 = 6;                        // ceil(95 / 16) — ASCII 32..=126

/// 95 glyphs × 14 rows × 1 byte (8 px/row, MSB left) — a monospace face pre-rasterized to
/// 1-bit and hex-packed. Dependency-free, wasm-friendly (the archive's choice; a TTF
/// rasterizer is a later luxury — label text is small and monospace reads fine).
const FONT_HEX: &str = concat!(
    "0000000000000000000000000000000000001818181818080000181800000000343434340000",
    "00000000000000001a12127f3424ff2c684800000008083c6a68683c0a0b4a3e0000000070d8",
    "d8730c304e09090f000000003c2020307059cdc7673f00000000181818180000000000000000",
    "00040808181010101018080800000010101808080c0c0808181000000000086a3c3c6a080000",
    "0000000000000000080808ff0808080000000000000000000000000018180000000000000000",
    "00003c0000000000000000000000000000001818000000000206040c08181810302000000000",
    "3c266263435b6362263c000000001828080808080808083f000000003c460206060c1830607e",
    "000000003c4602061c060202463c000000000e0e163626467f060606000000007e60607c4602",
    "0202463c000000001c3260407c666363663c000000007e0206040c0c08181030000000003c66",
    "62663c664343663c000000003c664243673f0202063c00000000000000181800000018180000",
    "000000000018180000001818000000000000030e78e0780e030000000000000000ff0000ff00",
    "000000000000000040781e031e784000000000003c2602060c1818001818000000001e6341cf",
    "9b91919bcf4000000000181c343426267e4343c1000000007c6662667c626363637e00000000",
    "1e32606060606060321e000000007c46424343434342467c000000007f6060607e606060607f",
    "000000007f6060607e6060606060000000001e32604040474363331e00000000434343437f43",
    "43434343000000007e18181818181818187e000000003e060606060606064c78000000004346",
    "4c5878684c464243000000006060606060606060607f00000000e3e7e7d7dbdbc3c3c3c30000",
    "0000636373535b4b4f474747000000003c66624343434362663c000000007e636363637e6060",
    "6060000000003c66624343434362663c000000007c464242467c46424341000000003c624060",
    "381e0203463c00000000ff181818181818181818000000006363636363636362663c00000000",
    "c34362622626341c1c1800000000c1c1c1d95b5f77766666000000004362361c181c342662c3",
    "00000000c36226341c1818181818000000007f03060c0c181030607f0000001c181818181818",
    "181818180000000040602030101818080c04000000380808080808080808080800000000183c",
    "2643000000000000000000000000000000000000000000003010080000000000000000000000",
    "000000003c66023e6242663a0000006060607c6663636363667c0000000000001e3260606060",
    "321e0000000202023e6642424242663e0000000000003c62437f4060623e0000000e18187e18",
    "1818181818180000000000003e6662424262663e0000006060607c6662626262626200000008",
    "0800380808080808087f0000000808003808080808080808000000606060626468786c666263",
    "000000701010101010101018180e0000000000007e5b4b4b4b4b4b4b0000000000007c666262",
    "626262620000000000003c6662434362663c0000000000007c6662636363667c000000000000",
    "3e6662424262663e0000000000003f383030303030300000000000003c2260380e02663c0000",
    "000010107e1010101010180e000000000000626262626262663a00000000000043626626343c",
    "1c1800000000000081c1d95b5f76762600000000000062263c181c3466430000000000004362",
    "2226341c1c180000000000007e0604081830607e0000000e0808081818701818080800000008",
    "080808080808080808080000007018181818080e08181818000000000000000000790e000000",
);

fn font_bits() -> Vec<u8> {
    (0..FONT_HEX.len() / 2)
        .map(|i| u8::from_str_radix(&FONT_HEX[i * 2..i * 2 + 2], 16).unwrap())
        .collect()
}

/// One texture, glyphs on a fixed grid: glyph g at ((g−32)%COLS, (g−32)/COLS).
/// UV rect = cell × cell size / texture size. Upload once; sampled forever.
pub fn create_font_atlas(device: &wgpu::Device,
                         queue: &wgpu::Queue) -> (wgpu::TextureView, wgpu::Sampler) {
    let bits = font_bits();
    let (w, h) = (COLS * CELL_W, ROWS * CELL_H);
    let mut texels = vec![0u8; (w * h) as usize];
    for g in 0..95u32 {
        let (cx, cy) = ((g % COLS) * CELL_W, (g / COLS) * CELL_H);
        for row in 0..CELL_H {
            let byte = bits[(g * CELL_H + row) as usize];
            for col in 0..CELL_W {
                if byte & (0x80 >> col) != 0 {
                    texels[((cy + row) * w + cx + col) as usize] = 255;
                }
            }
        }
    }
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("font.atlas"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    // write_texture validates bytes_per_row against COPY_BYTES_PER_ROW_ALIGNMENT (256) — our rows
    // are w = 128 B, so stage a padded copy (the same padded-row dance 35's readback does).
    let bpr = w.next_multiple_of(256);                    // 128 → 256
    let mut staged = vec![0u8; (bpr * h) as usize];
    for r in 0..h {
        let (s, d) = ((r * w) as usize, (r * bpr) as usize);
        staged[d..d + w as usize].copy_from_slice(&texels[s..s + w as usize]);
    }
    queue.write_texture(
        wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0,
            origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        &staged,
        wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h) },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("font.sampler"),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    (view, sampler)
}
```

(Register the module: `pub mod text;` in `src/engine/mod.rs`. The hex table was generated by
rendering DejaVu Sans Mono at 8×14 and thresholding — regenerate from any monospace face the same
way if you want a different look.)

## Step 2 — quads: `src/engine/text.rs`

One vertex format, **six vertices (two triangles) per character**, **anchor + pixel offset** split
exactly like the gumball's screen-constant math — the anchor projects, the offset is applied in
NDC-per-pixel after:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub anchor: [f32; 3],    // 12 B — ANCHOR-RELATIVE position of the LABEL (same for all its verts)
    pub px_off: [f32; 2],    //  8 B — this corner's offset from the anchor, in PIXELS
    pub uv: [f32; 2],        //  8 B — into the atlas
    pub color: [f32; 4],     // 16 B
// 44 B — a plain vertex buffer (not a storage table; text is rebuilt rarely)
}

/// Label `text` at world `p`: 6 verts (two triangles) per char, advancing CELL_W px per column,
/// centered on the anchor, floating one cell above it (shader +y = up on screen).
/// `origin` = the camera's current rebase anchor (33/34c): mvp is ANCHOR-RELATIVE, so the world
/// position must have it subtracted BEFORE the f32 cast — full f64 precision, like the instances.
pub fn label_verts(text: &str, p: [f64; 3], origin: &Point, color: [f32; 4],
                   out: &mut Vec<TextVertex>) {
    let a = [(p[0] - origin[0]) as f32, (p[1] - origin[1]) as f32, (p[2] - origin[2]) as f32];
    let (cw, ch) = (CELL_W as f32, CELL_H as f32);
    let (aw, ah) = ((COLS * CELL_W) as f32, (ROWS * CELL_H) as f32);   // atlas px size
    let n = text.chars().count();
    let x0 = -(n as f32) * cw * 0.5;                                   // center on the anchor
    for (i, c) in text.chars().enumerate() {                           // CHARS: i is the column
        let g = if (' '..='~').contains(&c) { c as u32 - 32 } else { 0 };
        let (gx, gy) = (((g % COLS) * CELL_W) as f32, ((g / COLS) * CELL_H) as f32);
        let (u0, v0) = (gx / aw, gy / ah);                             // v0 = glyph TOP row
        let (u1, v1) = ((gx + cw) / aw, (gy + ch) / ah);
        let (px0, px1) = (x0 + i as f32 * cw, x0 + (i as f32 + 1.0) * cw);
        let (top, bot) = (ch * 2.0, ch);                               // px above the anchor
        let quad = [
            ([px0, top], [u0, v0]), ([px1, top], [u1, v0]), ([px1, bot], [u1, v1]),
            ([px0, top], [u0, v0]), ([px1, bot], [u1, v1]), ([px0, bot], [u0, v1]),
        ];
        for (off, uv) in quad {
            out.push(TextVertex { anchor: a, px_off: off, uv, color });
        }
    }
}
```

Why the `origin` subtract: `mvp` is **anchor-relative** — 33's camera-relative rendering
(`view_proj_anchored(&anchor)`, with `rebuild_instances(origin)` baking the same f64 subtract into
every instance model). Labels have no instance model, so a label built from raw world positions is
right only until the camera re-anchors — then every label **jumps** by the anchor delta. Subtract
the current anchor when building the verts (above) and rebuild the label buffer whenever the anchor
moves — the same trigger that fires `rebuild_instances`. (The alternative — give each label an
`instance_id` and let the already-rebased instance model place it — works too, but is a bigger
diff.)

Two honest limitations of the 8×14 ASCII atlas, both visible on the project fixtures (the German
drawings name layers *Längsschnitt*, *Bemaßung*): non-ASCII chars fall through to glyph 0 (a blank
cell — the label loses the letter, it does not panic), and the centering counts **chars**, never
bytes — `text.len()` would mis-center any name with an umlaut. When real labels need ü/ß, widen the
atlas grid to Latin-1 and index by `c as u32`; the quad assembly doesn't change.

Two cost notes before we wire it up. **Memory:** 6 verts × 44 B per character ≈ 4× the bytes of an
instanced-quad design (one 4-vert template + a per-char instance row, 29's trick) — acceptable at
label counts (thousands of chars ≈ a few hundred KB), and the plain buffer keeps the rebuild code
dead simple; go instanced if you ever render full sheets of annotation text. **Rebuild granularity:**
the label buffer rebuilds *wholesale* on any name/anchor change — fine while labels are few; when
they get big, rewrite per-label ranges (the same partial-upload move 78 uses for mesh verts).
**Blending:** the fs `discard`s below coverage instead of using alpha-to-coverage — under 4× MSAA
that keeps glyph edges hard rather than dithered; it's the right call for text, but know that
`discard` also defeats early-z for the whole label pass.

The 44 bytes map field-for-field onto the WGSL vertex input — each Rust field is one `@location(N)`,
same order, same format. Get an offset or a `format` wrong here and the atlas samples garbage:

<svg viewBox="0 0 620 128" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="TextVertex 44-byte layout: anchor at location 0 float32x3, px_off at location 1 float32x2, uv at location 2 float32x2, color at location 3 float32x4" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="10" y="16" fill="#888">TextVertex — 44 B, one plain vertex buffer (stride 44)</text>
  <rect x="10"  y="28" width="132" height="26" fill="none" stroke="#6fb3ff"/>
  <rect x="142" y="28" width="88"  height="26" fill="none" stroke="#6fb3ff"/>
  <rect x="230" y="28" width="88"  height="26" fill="none" stroke="#6fb3ff"/>
  <rect x="318" y="28" width="176" height="26" fill="none" stroke="#6fb3ff"/>
  <g fill="#d7dae0" text-anchor="middle" font-size="11">
    <text x="76"  y="45">anchor</text>
    <text x="186" y="45">px_off</text>
    <text x="274" y="45">uv</text>
    <text x="406" y="45">color</text>
  </g>
  <g fill="#666" text-anchor="middle" font-size="10">
    <text x="76"  y="68">@location(0)</text><text x="76"  y="80">float32x3</text>
    <text x="186" y="68">@location(1)</text><text x="186" y="80">float32x2</text>
    <text x="274" y="68">@location(2)</text><text x="274" y="80">float32x2</text>
    <text x="406" y="68">@location(3)</text><text x="406" y="80">float32x4</text>
  </g>
  <g fill="#555" text-anchor="middle" font-size="10">
    <text x="76"  y="98">off 0</text>
    <text x="186" y="98">off 12</text>
    <text x="274" y="98">off 20</text>
    <text x="406" y="98">off 28</text>
  </g>
  <text x="10" y="122" fill="#888">world pt (shared) · px corner offset · atlas uv · rgba → the vs reads these by <tspan fill="#6fb3ff">@location</tspan>, not by struct name</text>
</svg>

Labels come from the document: object `name`s (the tree's names, 75) at each object's box-top
center — the row's **placed** box, 40's row-indexed `world_boxes` (manifest `place` included), not
the raw session-local box — rebuilt only when the scene, names, or the rebase anchor change — never
per frame.

## Step 3 — the shader: `src/shaders/text.wgsl`

Create the file. Group 0 is the shared `mvp` (binding 0, same slot 32b's cloud shader uses); group 1 is text's **own** `TextUniform` (just the viewport —
labels borrow nothing from the line uniform, exactly like 32b's `CloudUniform`); group 3 is the course's
first texture — the atlas + its sampler (group 2 stays the unused `instances` slot so the pipeline's
layout array is contiguous). The `TextVertex` fields arrive as `@location`s (a WGSL vertex-input type,
**not** the Rust CPU struct), matching the byte-map above:

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> view: TextUniform;

// group 3 — first texture bind group of the course: an R8Unorm atlas + a linear sampler.
@group(3) @binding(0) var atlas: texture_2d<f32>;
@group(3) @binding(1) var samp: sampler;

// Text billboards need only the viewport (px → NDC) — nothing from the line uniform, so their OWN
// uniform. Pad with two scalars (a vec3 pad rounds to 16 B and mis-sizes vs the Rust mirror).
struct TextUniform {
    vp_w: f32,          // framebuffer width, px
    vp_h: f32,          // framebuffer height, px
    _pad0: f32,
    _pad1: f32,
};                      // 16 B — one vec4

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) anchor: vec3<f32>,   // ANCHOR-RELATIVE label position (world − rebase origin)
    @location(1) px_off: vec2<f32>,   // this corner's offset from the anchor, in PIXELS
    @location(2) uv: vec2<f32>,       // into the atlas
    @location(3) color: vec4<f32>,
) -> VsOut {
    let clip = mvp * vec4<f32>(anchor, 1.0);
    // pixel offset applied AFTER projection (32b's billboard move): ×clip.w cancels the divide,
    // 2/viewport maps px → NDC. Labels face the camera and hold their size at every zoom.
    let ndc_off = px_off * 2.0 / vec2<f32>(view.vp_w, view.vp_h) * clip.w;
    var o: VsOut;
    o.pos = vec4<f32>(clip.xy + ndc_off, clip.zw);
    o.uv = uv;  o.color = color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let a = textureSample(atlas, samp, in.uv).r;    // R8: coverage
    if (a < 0.05) { discard; }
    return vec4<f32>(in.color.rgb, in.color.a * a);
}
```

(Build a `text_uniform` buffer `{ vp_w, vp_h, 0, 0 }` + a `text_bind_group` on the existing uniform
layout, and refresh `vp_w`/`vp_h` in `resize()` — the cloud's `CloudUniform` (32b) does the same; text
just drops the `size` field. The pipeline: alpha blend on, depth **test on / write off** — labels hide
behind geometry but never punch holes in it — and `cull_mode: None` (billboard quads have no
meaningful winding). Build it **inside `Pipelines::new` with the `samples` parameter** — the scene's
MSAA count is dynamic (1↔4; `set_scene` rebuilds all pipelines on the flip), and a text pipeline
pinned at the wrong count fails the render pass. First texture bind group of the course: one
`Texture` + `Sampler` layout at group 3.)

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Objects with names wear them: text floats at each box top, **facing the camera from every orbit
  angle** (billboard) at **constant size** at every zoom (the `clip.w` cancel — same behavior you
  already trust from line thickness).
- All four named views (14): labels stay upright and readable — nothing rotates into the screen.
- Perf HUD: **one** extra draw for all labels; orbiting doesn't rebuild anything (the buffer only
  rewrites when names/objects change — or on a camera re-anchor, the same trigger as
  `rebuild_instances`). Pan far from the origin and back: labels stay glued to their objects — no
  jump on re-anchor. Behind geometry, labels occlude correctly; they never z-fight
  (depth write off).
- Rename an object (edit the Session name via a future CLI verb or reconcile) → the label follows on
  the next rebuild — labels are a projection of document state like everything else in Phase 12.

## Recap

```
Ch 76: tree ↔ viewport — one selection, two views.
Ch 77: LABELS. A glyph atlas (ASCII on a fixed CELL grid, R8Unorm, baked once — archive text.rs) +
       one quad per character: TextVertex { anchor (world, shared), px_off (per corner, PIXELS), uv,
       color }. The vs projects the anchor then adds px_off·2/viewport·clip.w in NDC — 32b's
       billboard move → camera-facing, zoom-constant. fs samples coverage, alpha-discards. Depth
       test on / write off; alpha blend; the course's first texture bind group (90 adds another).
       Verts are ANCHOR-RELATIVE (mvp is anchored — subtract the rebase origin or labels jump);
       labels sit at each row's PLACED box top (40's world_boxes). Pipeline built in Pipelines::new
       with the dynamic samples. Labels rebuilt on document/anchor change, never per frame; every
       label in ONE draw. Phase 12 complete: the
       document is visible as a tree, in sync with the viewport, and named in the scene.
```

Edited: `engine/text.rs` (NEW — atlas + `label_verts`), `shaders/text.wgsl` (NEW),
`engine/pipelines/build.rs` (`build_text_pipeline`, texture layout, `TextUniform`),
`engine/gpu/mod.rs` (label buffer + `TextUniform` + draw).

## Next

`78-control-points.md` — Phase 13: sub-object editing. F10 turns control points on; the gumball grabs
a CV; the surface reshapes — with the two kernel gotchas (`set_cv_4d`, `invalidate_triangle_bvh`) and
a **partial** GPU upload that rewrites only the changed vertex range.
