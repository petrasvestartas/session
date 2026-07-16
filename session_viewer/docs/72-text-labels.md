# 72 Text labels — names in the scene

> **Big picture.** *Phase 12 closes.* The tree names objects in a panel; big assemblies want names
> **in the scene** — floating at the object, facing the camera, readable at any angle. The technique
> is 32b's billboard trick grown up: a **glyph atlas** (every character rasterized once into one
> texture) and one quad per character, all labels in **one draw call**. This is also the course's
> first textured draw — the atlas is the only texture the CAD look ever needs.

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
src/engine/text.rs             # NEW — font atlas build + TextVertex quad assembly (archive text.rs recipe)
src/shaders/text.wgsl          # NEW — billboard vs (32b's trick) + atlas-sampling fs
src/engine/pipelines/build.rs  # build_text_pipeline (first pipeline with a texture bind group)
src/engine/gpu/mod.rs          # label buffer + one draw, after the gumball, before egui
```

## Step 1 — the atlas: `src/engine/text.rs`

Rasterize a monospace set (ASCII 32–126 is plenty for names) into one `R8Unorm` texture at load —
the archive bakes a small bitmap font directly (`create_font_atlas(&device, &queue)`); a fixed
cell grid makes UVs trivial:

```rust
pub const CELL_W: u32 = 8;  pub const CELL_H: u32 = 14;  pub const COLS: u32 = 16;

/// One texture, glyphs on a fixed grid: glyph g at ((g−32)%COLS, (g−32)/COLS).
/// UV rect = cell × cell size / texture size. Upload once; sampled forever.
pub fn create_font_atlas(device: &wgpu::Device, queue: &wgpu::Queue) -> (wgpu::TextureView, wgpu::Sampler) {
    /* fill a CPU byte grid from an embedded 8×14 bitmap font table, write_texture, linear sampler */
}
```

(Embedding a public-domain 8×14 bitmap font table keeps this dependency-free and wasm-friendly — the
archive's choice. A TTF rasterizer is a later luxury; label text is small and monospace reads fine.)

## Step 2 — quads: `src/engine/text.rs`

One vertex format, four corners per character, **anchor + pixel offset** split exactly like the
gumball's screen-constant math — the anchor projects, the offset is applied in NDC-per-pixel after:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub anchor: [f32; 3],    // 12 B — world position of the LABEL (same for all its verts)
    pub px_off: [f32; 2],    //  8 B — this corner's offset from the anchor, in PIXELS
    pub uv: [f32; 2],        //  8 B — into the atlas
    pub color: [f32; 4],     // 16 B
}                            // 44 B — a plain vertex buffer (not a storage table; text is rebuilt rarely)

/// Label "name" at world `p`: chars → 6 verts each (two triangles), advancing CELL_W px per column.
pub fn label_verts(text: &str, p: [f32; 3], color: [f32; 4], out: &mut Vec<TextVertex>) { /* … */ }
```

Labels come from the document: object `name`s (the tree's names, 70) at each object's box-top center
(`world_aabb` again), rebuilt only when the scene or names change — never per frame.

## Step 3 — the shader: `src/shaders/text.wgsl`

```wgsl
@vertex
fn vs_main(in: TextVertex) -> VsOut {
    let clip = mvp * vec4<f32>(in.anchor, 1.0);
    // pixel offset applied AFTER projection (32b's billboard move): ×clip.w cancels the divide,
    // 2/viewport maps px → NDC. Labels face the camera and hold their size at every zoom.
    let ndc_off = in.px_off * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * clip.w;
    var o: VsOut;
    o.pos = vec4<f32>(clip.xy + ndc_off, clip.zw);
    o.uv = in.uv;  o.color = in.color;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let a = textureSample(atlas, samp, in.uv).r;    // R8: coverage
    if (a < 0.05) { discard; }
    return vec4<f32>(in.color.rgb, in.color.a * a);
}
```

(This wants a `vp_w` on the line uniform — the exact upgrade 32b deferred; take it now, the struct
grows one padded slot. The pipeline: alpha blend on, depth **test on / write off** — labels hide
behind geometry but never punch holes in it. First texture bind group of the course: one
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
  rewrites when names/objects change). Behind geometry, labels occlude correctly; they never z-fight
  (depth write off).
- Rename an object (edit the Session name via a future CLI verb or reconcile) → the label follows on
  the next rebuild — labels are a projection of document state like everything else in Phase 12.

## Recap

```
Ch 71: tree ↔ viewport — one selection, two views.
Ch 72: LABELS. A glyph atlas (ASCII on a fixed CELL grid, R8Unorm, baked once — archive text.rs) +
       one quad per character: TextVertex { anchor (world, shared), px_off (per corner, PIXELS), uv,
       color }. The vs projects the anchor then adds px_off·2/viewport·clip.w in NDC — 32b's
       billboard move → camera-facing, zoom-constant. fs samples coverage, alpha-discards. Depth test
       on / write off; alpha blend; the course's first (and only) texture bind group. Labels rebuilt
       on document change, never per frame; every label in ONE draw. Phase 12 complete: the document
       is visible as a tree, in sync with the viewport, and named in the scene.
```

Edited: `engine/text.rs` (NEW — atlas + `label_verts`), `shaders/text.wgsl` (NEW),
`engine/pipelines/build.rs` (`build_text_pipeline`, texture layout, `vp_w` on the line uniform),
`engine/gpu/mod.rs` (label buffer + draw).

## Next

`73-control-points.md` — Phase 13: sub-object editing. F10 turns control points on; the gumball grabs
a CV; the surface reshapes — with the two kernel gotchas (`set_cv_4d`, `invalidate_triangle_bvh`) and
a **partial** GPU upload that rewrites only the changed vertex range.
