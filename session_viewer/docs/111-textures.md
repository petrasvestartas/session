# 111 Textures — paint an image onto geometry (optional appendix)

The CAD default look — shaded + edges + arctic GI — never uses textures, so this lesson sits outside
the main path. But applying an **image** to a surface is the one rendering idea the 84 lessons skip,
and it's three small pieces: **upload** a texture, **bind** it (texture + sampler) to the mesh
pipeline, **sample** it in the fragment shader. We add all three to the existing mesh pass — every
object in the scene picks up the texture, no new pipeline.

**Where it plugs into what we built:** the mesh already binds three groups — `@group(0)` mvp,
`@group(1)` time, `@group(2)` the instances storage buffer (lesson 29). `@location(3)` is the
per-vertex instance id. So the material lands at the **next free** group and location:

```
group 0  mvp          group 1  time         group 2  instances      group 3  ← material (NEW)
loc 0 pos   loc 1 normal   loc 2 color   loc 3 inst_id            (uv → loc 4, optional variant below)
```

> The Phase-3 roadmap note predates instancing and says "group-2 / `@location(3)`" — both were taken
> by lesson 29. Material = **`@group(3)`**, an optional per-vertex UV = **`@location(4)`**.
> (77's label atlas also uses group 3 — but on the TEXT pipeline; the material's group 3 is on the
> TRIANGLE pipeline. Same slot, different pipelines, no contention.)

## Why UVs are optional here

A texture needs a 2D coordinate per pixel. The honest way is a `uv` baked per vertex — but the kernel
meshes (`create_box`, `create_dodecahedron`, the BRep tessellations) don't carry UVs, so there'd be
nothing to sample. Instead this lesson uses **triplanar projection**: sample the texture three times —
once on each world axis plane (yz, zx, xy) — and blend by the face normal. It needs no UV attribute
and works on *every* mesh in the scene immediately. The `@location(4)` per-vertex-UV route is shown as
a variant at the end for meshes that do carry UVs.

<svg viewBox="0 0 700 200" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="pipeline: a checker texture plus a sampler feed a triplanar projection that blends three axis-plane samples by the face normal, multiplied by the lit term to shade the box" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(20,30)">
    <rect x="0" y="0" width="90" height="90" fill="#d2b48c" stroke="#3a3a3a"/>
    <rect x="0"  y="0"  width="30" height="30" fill="#5a463799"/>
    <rect x="60" y="0"  width="30" height="30" fill="#5a463799"/>
    <rect x="30" y="30" width="30" height="30" fill="#5a463799"/>
    <rect x="0"  y="60" width="30" height="30" fill="#5a463799"/>
    <rect x="60" y="60" width="30" height="30" fill="#5a463799"/>
    <text x="45" y="108" fill="#888" text-anchor="middle">albedo texture</text>
    <text x="45" y="120" fill="#888" text-anchor="middle">+ sampler (Repeat)</text>
  </g>
  <path d="M 118,75 L 168,75" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah85)"/>
  <g transform="translate(180,26)" font-size="10">
    <text x="0" y="6" fill="#6fb3ff">triplanar — sample 3×, blend by |n|</text>
    <text x="0" y="26" fill="#c9d4e0">cx = sample(world_pos.yz · s)</text>
    <text x="0" y="42" fill="#c9d4e0">cy = sample(world_pos.zx · s)</text>
    <text x="0" y="58" fill="#c9d4e0">cz = sample(world_pos.xy · s)</text>
    <text x="0" y="78" fill="#e0b040">albedo = cx·bw.x + cy·bw.y + cz·bw.z</text>
  </g>
  <path d="M 470,55 L 520,55" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah85)"/>
  <g transform="translate(530,20)">
    <path d="M 0,50 L 60,74 L 130,50 L 70,28 Z" fill="#c9ad84" stroke="#3a3a3a"/>
    <path d="M 0,50 L 0,92 L 60,116 L 60,74 Z" fill="#7d6444" stroke="#3a3a3a"/>
    <path d="M 60,74 L 60,116 L 130,92 L 130,50 Z" fill="#a68a63" stroke="#3a3a3a"/>
    <text x="60" y="140" fill="#888" text-anchor="middle" font-size="10">albedo × lit</text>
  </g>
  <defs>
    <marker id="ah85" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker>
  </defs>
</svg>

## Files we touch

```
src/engine/gpu/mod.rs             # generate + upload the texture; build the material bind group; bind it
src/engine/pipelines/mod.rs   # thread material_layout into Pipelines::new
src/engine/pipelines/build.rs # add material_layout to the triangle pipeline's layout
src/shaders/triangle.wgsl     # declare texture + sampler at group(3); triplanar sample
```

## Step 1 — generate + upload a texture: `src/engine/gpu/mod.rs`

We *generate* a 256×256 checkerboard in code — no image file, no asset loader, so it works
identically on native and wasm. **Find** the `instance_bind_group` block (ends with
`entries: &[wgpu::BindGroupEntry {binding: 0, resource: instance_buffer.as_entire_binding()}],`) and
**insert after it**:

```rust
// --- Material: a generated checker texture + sampler (group 3) ---
// Swap the loop for `image::load_from_memory(include_bytes!("wood.png"))` to use a real image.
const TEX_SIZE: u32 = 256;
let mut texels = vec![0u8; (TEX_SIZE * TEX_SIZE * 4) as usize];
for y in 0..TEX_SIZE {
    for x in 0..TEX_SIZE {
        let on = ((x / 32) + (y / 32)) % 2 == 0;
        let c = if on { [210u8, 180, 140, 255] } else { [90u8, 70, 55, 255] };
        let i = ((y * TEX_SIZE + x) * 4) as usize;
        texels[i..i + 4].copy_from_slice(&c);
    }
}

let material_tex = device.create_texture(&wgpu::TextureDescriptor {
    label: Some("albedo"),
    size: wgpu::Extent3d { width: TEX_SIZE, height: TEX_SIZE, depth_or_array_layers: 1 },
    mip_level_count: 9,    // 256→1: the FULL chain — 1 mip + linear min filter shimmers the
                           // moment a textured face minifies (zoomed out, grazing angles)
    sample_count: 1,                                  // sampled, NOT an MSAA render target
    dimension: wgpu::TextureDimension::D2,
    format: wgpu::TextureFormat::Rgba8UnormSrgb,      // sRGB → sampler linearizes, lighting stays correct
    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    view_formats: &[],
});

// wgpu has no generate_mipmap — box-filter the chain on the CPU (trivial for a generated
// texture; a real image pipeline would downsample in a compute pass instead).
// Rows under 64 px are < 256 B/row → pad the staging rows (COPY_BYTES_PER_ROW_ALIGNMENT —
// 77's atlas needed the same dance).
fn upload_mips(queue: &wgpu::Queue, tex: &wgpu::Texture, base: &[u8], size: u32) {
    let (mut w, mut level, mut mip) = (size, base.to_vec(), 0u32);
    loop {
        let unpadded = (w * 4) as usize;
        let bpr = unpadded.next_multiple_of(256);
        let mut staged = vec![0u8; bpr * w as usize];
        for r in 0..w as usize {
            staged[r * bpr..r * bpr + unpadded]
                .copy_from_slice(&level[r * unpadded..(r + 1) * unpadded]);
        }
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {                  // wgpu 29 name (was ImageCopyTexture)
                texture: tex, mip_level: mip,
                origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &staged,
            wgpu::TexelCopyBufferLayout {                 // wgpu 29 name (was ImageDataLayout)
                offset: 0, bytes_per_row: Some(bpr as u32), rows_per_image: Some(w) },
            wgpu::Extent3d { width: w, height: w, depth_or_array_layers: 1 });
        if w == 1 { break; }
        let nw = w / 2;
        let mut next = vec![0u8; (nw * nw * 4) as usize];
        for y in 0..nw {
            for x in 0..nw {
                for c in 0..4 {
                    let mut acc = 0u32;
                    for dy in 0..2 { for dx in 0..2 {
                        acc += level[(((2 * y + dy) * w + 2 * x + dx) * 4 + c) as usize] as u32;
                    }}
                    next[((y * nw + x) * 4 + c) as usize] = (acc / 4) as u8;
                }
            }
        }
        level = next; w = nw; mip += 1;
    }
}
upload_mips(&queue, &material_tex, &texels, TEX_SIZE);

let material_view = material_tex.create_view(&wgpu::TextureViewDescriptor::default());
let material_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    label: Some("albedo.sampler"),
    address_mode_u: wgpu::AddressMode::Repeat,
    address_mode_v: wgpu::AddressMode::Repeat,
    address_mode_w: wgpu::AddressMode::Repeat,
    mag_filter: wgpu::FilterMode::Linear,
    min_filter: wgpu::FilterMode::Linear,
    mipmap_filter: wgpu::MipmapFilterMode::Linear,    // wgpu 29: MipmapFilterMode, NOT FilterMode;
                                                      // Linear so mip transitions don't band
    ..Default::default()
});

let material_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some("material.layout"),
    entries: &[
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        },
    ],
});

let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    label: Some("material.bind_group"),
    layout: &material_layout,
    entries: &[
        wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&material_view) },
        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&material_sampler) },
    ],
});
```

The bind group ref-counts the texture, view, and sampler, so we only need to keep
`material_bind_group` alive — `material_tex`/`material_view`/`material_sampler` may drop, exactly like
`instance_buffer` isn't stored after `instance_bind_group` is built.

Four traps this Step already dodges, all caught by a compile or a validation error:
- `mipmap_filter` takes `MipmapFilterMode`, not `FilterMode`.
- `write_texture` uses `TexelCopyTextureInfo` / `TexelCopyBufferLayout` (renamed from the old
  `ImageCopyTexture` / `ImageDataLayout`).
- `sample_count: 1` — this is a *sampled* texture; MSAA (4) is only for render targets like `msaa_view`.
- `bytes_per_row` must be a multiple of 256 — level-0 rows are (1024 B), but the small mips aren't;
  `upload_mips` pads every level's staging rows (77's atlas hit the same wall).

## Step 2 — keep the bind group on `Gpu`: `src/engine/gpu/mod.rs`

**Find** the struct field `pub instance_bind_group: wgpu::BindGroup,` and **insert after it**:

```rust
pub material_bind_group: wgpu::BindGroup,
```

Then **find** the `Ok(Self {` return and add `material_bind_group,` next to `instance_bind_group,`.

## Step 3 — give the pipeline the material layout: `mod.rs` + `build.rs`

The triangle pipeline must declare a *fourth* bind-group layout or binding group 3 is a validation
error. Two edits.

**`src/engine/pipelines/mod.rs`** — add a parameter to `Pipelines::new` (after `instance_layout`) and
pass it on. **Find** the `instance_layout: &wgpu::BindGroupLayout,` param and **insert after it**:

```rust
    material_layout: &wgpu::BindGroupLayout,
```

**Find** the `triangle: build_triangle_pipeline(...)` line and add the arg:

```rust
    triangle: build_triangle_pipeline(device, samples, color_format, aspect_layout, time_layout, instance_layout, material_layout),
```

**`src/engine/pipelines/build.rs`** — **find** the `build_triangle_pipeline` signature and add the
parameter after `instance_layout`:

```rust
    material_layout: &wgpu::BindGroupLayout,
```

**Find** its `bind_group_layouts: &[Some(aspect_layout), Some(time_layout), Some(instance_layout)],`
and add the fourth entry:

```rust
        bind_group_layouts: &[Some(aspect_layout), Some(time_layout), Some(instance_layout), Some(material_layout)],
```

Back in **`gpu/mod.rs`**, **find** the `Pipelines::new(` call and pass `&material_layout` after
`&instance_layout` (Step 1 created it above this call):

```rust
        let pipelines = Pipelines::new(
            &device,
            samples,            // MSAA count — 2nd param
            config.format,
            &mvp_layout,
            &time_layout,
            &instance_layout,
            &material_layout,   // ← NEW, after instance_layout
            &line_layout,
            &segment_layout,
            &glyph_layout);     // ← keep the existing trailing arg
```

Pipelines are **rebuilt on the MSAA flip** (`msaa_for` picks 1× for flat-only scenes, 4× for solids,
and `set_scene` rebuilds views + pipelines when it changes) — so the material layout must be
threaded to that rebuild call site too, not only the startup one.

## Step 4 — declare + sample the texture: `src/shaders/triangle.wgsl`

**Find** the top two uniform lines (`@group(0) … mvp` and `@group(1) … time`) and **insert after
them**:

```wgsl
// Material — texture + sampler. group(3): 0/1/2 are mvp/time/instances.
@group(3) @binding(0) var albedo_tex: texture_2d<f32>;
@group(3) @binding(1) var albedo_smp: sampler;

// The rebase anchor, mod the texture's 400 mm tile — CPU-computed in f64, uploaded tiny.
// group 0 binding 2: the shared mvp layout (83 put sections at binding 1).
@group(0) @binding(2) var<uniform> tex_anchor: vec4<f32>;
```

The Rust half is three small edits beside 83's section buffer: a `binding: 2` entry in
`mvp_layout`'s `entries` (visibility FRAGMENT, uniform buffer), a 16-byte `tex_anchor` buffer +
`BindGroupEntry` on the `mvp_bind_group`, and the upload — where `rebase_anchor` runs in
`State::render`:

```rust
    // The triplanar pattern repeats every 400 mm, so only (anchor mod 400) can change the
    // sampling — compute it in f64 and upload it TINY. Adding the raw anchor (millions of mm)
    // to a fragment's position in f32 would drown the 400 mm tile phase in rounding: the
    // checker would swim on every re-anchor.
    let a_mod = [0, 1, 2].map(|k| anchor[k].rem_euclid(400.0) as f32);
    self.gpu.queue.write_buffer(&self.gpu.tex_anchor_buffer, 0,
        bytemuck::bytes_of(&[a_mod[0], a_mod[1], a_mod[2], 0.0]));
```

Then **find** the last line of `fs_main`,
`return vec4<f32>(in.color * select(lit, 1.0, in.print > 0.5), 1.0);` (34h's print-fill
select — KEEP the select, texture only the lit half), and **replace it**
with the triplanar sample (`n` and `in.world_pos` are already in scope from the light model above):

```wgsl
    // Triplanar UV: sample on the 3 world axis planes, blend by |normal|. No per-vertex UVs.
    // in.world_pos is ANCHOR-RELATIVE f32 (instance models are rebased). The texture phase keys
    // on TRUE world position — but adding the raw anchor back in f32 (millions of mm against a
    // 400 mm tile) loses the phase to rounding, and the checker swims on every re-anchor. Since
    // the pattern is periodic, the CPU sent anchor mod tile instead (tex_anchor, above):
    // world_pos + (anchor mod 400mm) differs from true world by a WHOLE number of tiles.
    let wp = in.world_pos + tex_anchor.xyz;
    let scale = 1.0 / 400.0;                 // world units are mm: 400 mm per tile ≈ 2.5 tiles/metre
    let wn = abs(n);
    let bw = wn / (wn.x + wn.y + wn.z);       // blend weights, sum to 1
    let cx = textureSample(albedo_tex, albedo_smp, wp.yz * scale).rgb;
    let cy = textureSample(albedo_tex, albedo_smp, wp.zx * scale).rgb;
    let cz = textureSample(albedo_tex, albedo_smp, wp.xy * scale).rgb;
    let albedo = cx * bw.x + cy * bw.y + cz * bw.z;

    // keep 34h's print-fill select; swap for `albedo * in.color * …` to tint per object
    return vec4<f32>(albedo * select(lit, 1.0, in.print > 0.5), 1.0);
```

`textureSample` needs **uniform control flow** (it takes implicit derivatives) — that's why it sits at
the function's top level, *after* the `if !front { … }` block has re-converged, not inside it.

## Step 5 — bind it in the mesh pass: `src/engine/gpu/mod.rs`

**Find**, in `clear()`, the triangle pass's `pass.set_bind_group(2, &self.instance_bind_group, &[]);`
and **insert after it**:

```rust
            pass.set_bind_group(3, &self.material_bind_group, &[]);
```

## Step 6 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8769
```

Every solid in the scene wears the checkerboard, wrapped by the triplanar projection: the box's faces
tile cleanly, and the sphere/torus show the three-axis blend seams softening across curved normals.
Orbit and the texture stays locked to the geometry (it's keyed on world position — and stays locked
through a camera re-anchor, because `tex_anchor` carries the anchor's tile phase), while the shading
still moves with the lights. **Zoom far out** — the checker fades to a smooth average, no shimmer
(the full mip chain + trilinear filtering; with `mip_level_count: 1` it would sparkle). Edges (the
cylinder pass) are untouched — they never bind the material.

## Variant — real per-vertex UVs (`@location(4)`)

For meshes that *carry* UVs, skip triplanar and sample a baked coordinate. This one touches the
**kernel** (`RenderVertex` is in `session_rust`), so it's a cross-crate change:

1. `session_rust/src/render_mesh.rs` — add `pub uv: [f32; 2]` to `RenderVertex` (stride 40 → 48); in
   `ATTRIBS` add `4 => Float32x2`; fill it from the vertex `"u"`/`"v"` attributes (like
   `nx`/`ny`/`nz`), defaulting to `[0.0, 0.0]`. The arena fill (`push_mesh`) now lives in
   `app/scene.rs` — that is where the uv reaches the vertex buffer.
2. `triangle.wgsl` — add `@location(4) uv: vec2<f32>` to `VsIn`, pass it through `VsOut`, and replace
   the triplanar block with `let albedo = textureSample(albedo_tex, albedo_smp, in.uv).rgb;`.

The instance-id buffer keeps `@location(3)`; the UV rides in the interleaved `RenderVertex` at
`@location(4)`, so no pipeline vertex-layout change beyond the wider stride `RenderVertex::layout()`
reports automatically.

## Recap

```
Textures = upload + bind + sample. Generate an RGBA8 checker, write_texture it (TexelCopyTextureInfo /
TexelCopyBufferLayout in wgpu 29; full CPU-built mip chain — 1 mip + linear min filter shimmers on
minification, and small-mip rows need 256 B padding), build a group(3) bind group of {texture,
sampler}, add that layout to the triangle pipeline, set_bind_group(3, …) in the mesh pass, and in
fs_main sample it. The triplanar projection keys on world position WITHOUT adding the raw anchor in
f32 — tex_anchor (group 0 binding 2, beside 83's sections) carries anchor mod tile, f64-computed.
UVs come from triplanar world-position projection (no attribute needed) — or, for UV-mapped meshes,
from a per-vertex uv at @location(4) (kernel RenderVertex, stride 40→48).
```

Edited: `engine/gpu/mod.rs` (generate/upload texture + mip chain + material bind group, store it,
pass its layout to `Pipelines::new`, bind group 3 in the pass, `tex_anchor` buffer at group 0
binding 2), `engine/pipelines/mod.rs` + `build.rs` (thread
`material_layout` into the triangle pipeline), `shaders/triangle.wgsl` (group-3 texture/sampler +
group-0-binding-2 `tex_anchor` + triplanar sample).

## Next

This is the last (optional) appendix — the CAD default look returns to shaded + edges + arctic GI,
which never binds a material. Fold textures in only where a rendering mode genuinely needs an image
(a preview material, a decal, an imported model's baked albedo).

## Next

[100 — sheet impostors](112-sheet-impostors.md) opens Phase 15: the `_RESEARCH_GPU_CAD.md`
findings implemented as our own source — costs scale with the screen, not the file.

