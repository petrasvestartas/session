# 32b Points II — billboard clouds at scale

> **Big picture.** *Phase 4 — one scene, one draw call.* 32a gave handles a real instanced sphere —
> 144 triangles each, fine for dozens. A `PointCloud` has *millions*, and it doesn't need 3-D
> roundness: a flat circle that always faces the camera looks identical at point sizes and costs
> **1 triangle instead of 144**. This is the standard scale trick in every CAD/point-cloud viewer:
> spend geometry only where the eye can tell the difference.

<svg viewBox="0 0 380 172" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a 3-vertex equilateral triangle whose fragment shader draws its inscribed anti-aliased circle" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="190" y="18" fill="#888" text-anchor="middle">PointCloud — millions, 2-D is enough</text>
  <polygon points="118,52 86.8,106 149.2,106" fill="none" stroke="#3a3a3a"/>
  <circle cx="118" cy="88" r="18" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="118" y="128" fill="#d7dae0" text-anchor="middle">3-vert triangle</text>
  <text x="118" y="144" fill="#555" text-anchor="middle">1 tri · incircle = the dot</text>
  <text x="228" y="76" fill="#666" font-size="10">fs carves the incircle:</text>
  <text x="228" y="92" fill="#666" font-size="10">discard length(corner) &gt; 1</text>
  <text x="228" y="112" fill="#555" font-size="10">½ a quad's verts · 1 tri, not 144</text>
</svg>

## Files we touch

```
src/shaders/point.wgsl         # NEW — 3-vert triangle billboard; inscribed SDF circle
src/engine/pipelines/build.rs  # build_point_pipeline (a 3-line variation of the sphere one)
src/engine/pipelines/mod.rs    # Pipelines gains `point`
src/engine/gpu.rs              # CloudPoint row, CloudUniform, points buffer, one draw
```

## Step 1 — the cloud row: `src/engine/gpu.rs`

**1a. Add `CloudPoint` next to `GlyphPoint`** (32a). Half the size — at a million points, bytes/row is
the budget that matters:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CloudPoint {
    position: [f32; 3],   // 12 B — mesh-local
    instance_id: u32,     //  4 B — fills position's tail
    color: [f32; 4],      // 16 B
}                         // = 32 B total, two 16-byte rows, zero padding
```

> No per-point radius — the whole cloud shares **one global** dot size, and it lives in the cloud's
> **own** uniform: `CloudUniform.size` (px). *Not* `line.thickness` — the billboard shares nothing with
> the line uniform (it never needed `thickness`/`proj_y`/`ortho_h`, only a viewport), and a scanned cloud
> almost always wants a *different* (usually finer) dot than the line width. A *per-point* size would
> instead need a lone `f32` after `color` — a third 16-byte row (→ 48 B, like `GlyphPoint`); skip that
> until a cloud actually needs varying dots (at millions of points the 16 B/row saved is real). Global
> size lives in the uniform (Step 1b), not the row.

**1b. Give the cloud its own uniform — `CloudUniform`.** The billboard needs exactly two things from the
CPU: how big the dot is, and the viewport it is measured against. That is a tiny uniform of its own — no
`line`/`sphere` fields borrowed. Add it next to `LineUniform` at the bottom of `gpu.rs`:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CloudUniform {
    size: f32,   // global point-cloud dot size, px
    vp_w: f32,   // framebuffer width, px
    vp_h: f32,   // framebuffer height, px
    _pad: f32,
}                // 16 B — one vec4; its own buffer + bind group
```

`size: 4.0` is a fixed default now (a slider like the line one lands with the HUD, 47); keep `vp_w`/`vp_h`
in sync with the framebuffer on resize. Carrying **both** viewport dimensions is what keeps the dot round
on any window aspect (Step 2) — the line uniform only had `vp_h`, so borrowing it would have squashed the
circle anyway.

## Step 2 — the billboard shader: `src/shaders/point.wgsl`

Create the file. There is **no template mesh**: three corner positions come straight from
`@builtin(vertex_index)` (the lesson-25 buffer-less trick) — **one equilateral triangle whose inscribed
circle is the dot** (half a quad's verts; at cloud point sizes the triangle's few extra discarded
corners cost nothing, while vertex/primitive setup — the real bottleneck at millions of points — halves).
The fragment shader carves that incircle with a signed-distance test, so the edge stays crisp and
anti-aliased at any size:

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> cloud: CloudUniform;
struct Instance { model: mat4x4<f32>, color: vec4<f32>, flags: u32, };
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
struct CloudPoint { position: vec3<f32>, instance_id: u32, color: vec4<f32>, };
@group(3) @binding(0) var<storage, read> points: array<CloudPoint>;
struct CloudUniform { size: f32, vp_w: f32, vp_h: f32, _pad: f32, };

// One logical point = 3 verts (1 triangle); corner is vertex_index % 3.
// Equilateral triangle whose INCIRCLE (radius 1 in corner-space) is the visible dot —
// corners sit at distance 2 from centre (circumradius = 2× the inradius), √3 ≈ 1.7320508.
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>( 0.0,        2.0),
    vec2<f32>(-1.7320508, -1.0),
    vec2<f32>( 1.7320508, -1.0),
);

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>,   // triangle-local; the incircle (radius 1) is the dot
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let p      = points[vid / 3u];               // 3 verts per point → vid/3 = the point
    let model  = instances[p.instance_id].model;
    let world  = (model * vec4<f32>(p.position, 1.0)).xyz;
    let clip   = mvp * vec4<f32>(world, 1.0);
    let corner = CORNERS[vid % 3u];
    let px     = cloud.size;   // the cloud's own global dot size — its own uniform, not the line's
    // px→NDC per axis via (vp_w,vp_h) keeps dots round; clip.w cancels the perspective divide.
    let off    = corner * px * 2.0 / vec2<f32>(cloud.vp_w, cloud.vp_h) * clip.w;
    var o: VsOut;
    o.pos    = vec4<f32>(clip.xy + off, clip.zw);
    o.color  = p.color;
    o.corner = corner;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let d = length(in.corner);            // SDF circle: soft, anti-aliased edge
    let a = clamp((1.0 - d) * 8.0, 0.0, 1.0);
    if (a < 0.01) { discard; }
    return vec4<f32>(in.color.rgb, in.color.a * a);
}
```

<svg viewBox="0 0 380 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="one draw of 3 times N vertices; point equals vid divided by 3, corner equals vid modulo 3" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="190" y="16" fill="#888" text-anchor="middle">draw(0..3·N, 0..1) — one vertex stream, NOT instances</text>
  <rect x="22"  y="44" width="82" height="24" fill="none" stroke="#6fb3ff" stroke-width="0.9"/>
  <rect x="106" y="44" width="82" height="24" fill="none" stroke="#555" stroke-width="0.9"/>
  <rect x="190" y="44" width="82" height="24" fill="none" stroke="#6fb3ff" stroke-width="0.9"/>
  <rect x="274" y="44" width="82" height="24" fill="none" stroke="#555" stroke-width="0.9"/>
  <text x="63"  y="38" fill="#6fb3ff" text-anchor="middle" font-size="9">vid/3=0 → points[0]</text>
  <text x="147" y="38" fill="#888"    text-anchor="middle" font-size="9">vid/3=1 → points[1]</text>
  <text x="231" y="38" fill="#6fb3ff" text-anchor="middle" font-size="9">vid/3=2 → points[2]</text>
  <text x="315" y="38" fill="#888"    text-anchor="middle" font-size="9">vid/3=3 → points[3]</text>
  <text x="18" y="60" fill="#666" text-anchor="end" font-size="10">vid</text>
  <text x="34"  y="60" fill="#d7dae0" text-anchor="middle">0</text>
  <text x="62"  y="60" fill="#d7dae0" text-anchor="middle">1</text>
  <text x="90"  y="60" fill="#d7dae0" text-anchor="middle">2</text>
  <text x="118" y="60" fill="#d7dae0" text-anchor="middle">3</text>
  <text x="146" y="60" fill="#d7dae0" text-anchor="middle">4</text>
  <text x="174" y="60" fill="#d7dae0" text-anchor="middle">5</text>
  <text x="206" y="60" fill="#d7dae0" text-anchor="middle">6</text>
  <text x="234" y="60" fill="#d7dae0" text-anchor="middle">7</text>
  <text x="262" y="60" fill="#d7dae0" text-anchor="middle">8</text>
  <text x="290" y="60" fill="#d7dae0" text-anchor="middle">9</text>
  <text x="318" y="60" fill="#d7dae0" text-anchor="middle">10</text>
  <text x="346" y="60" fill="#d7dae0" text-anchor="middle">11</text>
  <text x="18" y="88" fill="#666" text-anchor="end" font-size="10">%3</text>
  <text x="34"  y="88" fill="#5bbf87" text-anchor="middle">0</text>
  <text x="62"  y="88" fill="#5bbf87" text-anchor="middle">1</text>
  <text x="90"  y="88" fill="#5bbf87" text-anchor="middle">2</text>
  <text x="118" y="88" fill="#5bbf87" text-anchor="middle">0</text>
  <text x="146" y="88" fill="#5bbf87" text-anchor="middle">1</text>
  <text x="174" y="88" fill="#5bbf87" text-anchor="middle">2</text>
  <text x="206" y="88" fill="#5bbf87" text-anchor="middle">0</text>
  <text x="234" y="88" fill="#5bbf87" text-anchor="middle">1</text>
  <text x="262" y="88" fill="#5bbf87" text-anchor="middle">2</text>
  <text x="290" y="88" fill="#5bbf87" text-anchor="middle">0</text>
  <text x="318" y="88" fill="#5bbf87" text-anchor="middle">1</text>
  <text x="346" y="88" fill="#5bbf87" text-anchor="middle">2</text>
  <text x="190" y="118" fill="#666" text-anchor="middle" font-size="10">let p = points[vid / 3u];   let corner = CORNERS[vid % 3u];</text>
  <text x="190" y="138" fill="#e06c6c" text-anchor="middle" font-size="10">points[instance_index] would collapse all N onto point 0</text>
</svg>

> Dividing by `vec2(vp_w, vp_h)` — both carried by `CloudUniform` — grows the dot by the same pixel
> count on each axis, so it stays a **circle** on any window aspect, not the oval a single `vp_h` gives.
> (The line/sphere uniform only has `vp_h`; another reason the cloud carries its own.)

## Step 3 — the point pipeline: `src/engine/pipelines/build.rs` + `mod.rs`

**3a. Add `build_point_pipeline` after `build_sphere_pipeline`.** Copy the whole `build_sphere_pipeline`
body and rename as you paste: `fn build_sphere_pipeline` → `fn build_point_pipeline`, the shader label
`"sphere.shader"` → `"point.shader"`, `include_str!("../../shaders/sphere.wgsl")` →
`include_str!("../../shaders/point.wgsl")`, and the pipeline/layout labels `"sphere"`/`"sphere.layout"`
→ `"point"`/`"point.layout"`. Then apply three real changes — no vertex buffer (corners come from
`vertex_index`), **alpha blending on** (the SDF edge is translucent), and depth **write off**
(billboards are transparent overlays):

```rust
        // in vertex state:
        // no template — vertex_index builds the triangle
        buffers: &[],
        // in the fragment target:
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        // in depth_stencil:
        depth_write_enabled: Some(false),
```

**3b. In `mod.rs`**, add `pub point: wgpu::RenderPipeline,` after `sphere` and build it with the same
layouts (the `points` storage buffer reuses the glyph bind-group layout shape — one read-only storage
entry at binding 0).

## Step 4 — build the buffer + draw: `src/engine/gpu.rs`

**4a. Add the point-cloud fields to `struct Gpu`** — right after the glyph trio (`glyph_count`). Same
three fields as the glyph set, plus the cloud's own uniform:

```rust
    // 32b — the point cloud: the glyph set's trio + its own size/viewport uniform
    pub point_buffer: wgpu::Buffer,
    pub point_bind_group: wgpu::BindGroup,
    pub point_count: u32,
    pub cloud_buffer: wgpu::Buffer,        // CloudUniform — global size + viewport
    pub cloud_bind_group: wgpu::BindGroup,
```

**4b. Load the cloud from a real file** — in `new()`, right after the mesh arena `for` loop and *before*
`instance_buffer` is built (the cloud pushes one more `Instance`). It is one object, so it gets one
instance row of its own:

```rust
        // A real point cloud from a file. The browser has no filesystem, so embed the bytes at compile
        // time and parse the string — read_xyz(path) is native-only; read_xyz_from_str works everywhere.
        // bunny_rgb.xyz is "x y z r g b" per line, so read_xyz_from_str fills PointCloud colors too.
        let bunny = session_rust::read_xyz_from_str(
            include_str!("../../../session_data/bunny_rgb.xyz"));  // 397 pts + per-point color

        let cloud_ri = instances.len() as u32;                     // the cloud's own instance row
        instances.push(Instance {                                  // scale the tiny bunny up + lift it
            model: (Xform::translation(0.0, 0.0, 900.0)
                  * Xform::scale_xyz(4000.0, 4000.0, 4000.0)).to_f32(),
            color: [0.0; 4],
            flags: 0,
            _pad: [0; 3],
        });

        let has_color = bunny.color_count() == bunny.point_count();  // did the file carry colors?
        let mut points: Vec<CloudPoint> = Vec::new();
        for (i, p) in bunny.get_points().iter().enumerate() {
            let color = if has_color { bunny.get_color(i).to_f32() } else { [0.4, 0.7, 1.0, 1.0] };
            points.push(CloudPoint {
                position: p.to_f32(),
                instance_id: cloud_ri,        // every point shares the cloud's one model matrix
                color,                         // ← from the FILE, not hardcoded
            });
        }
```

> `read_xyz_from_str` is the kernel's `io` layer. An XYZ line may carry per-point color (`x y z r g b`,
> either 0–255 or 0–1 — auto-detected) which lands in `PointCloud`'s colors, and the draw reads
> `get_color(i)`, so a scanned cloud shows its **real** colors instead of a hardcoded tint. It will grow
> `read_ply`/`read_obj` too — each format lands in the geometry it describes (cloud, mesh, curves).

**4c. Build the point buffer + the cloud uniform** — beside the glyph and line-uniform blocks. Points are
a storage table (reuse `glyph_layout`); size+viewport is a uniform (reuse `line_layout` — same shape):

```rust
        // points storage — same shape as the glyph set, so it reuses glyph_layout
        let point_count = points.len() as u32;                     // real length; empty ⇒ 0 verts drawn
        // storage_buffer (never create_buffer_init) is 32a's empty-cloud guard
        let point_buffer = storage_buffer(&device, "points.buffer", &points);
        let point_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("points.bind_group"),
            layout: &glyph_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: point_buffer.as_entire_binding() }],
        });

        // cloud uniform — the cloud's OWN global size + viewport (reuses line_layout: uniform@0)
        let cloud_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cloud.buffer"),
            contents: bytemuck::bytes_of(&CloudUniform {
                size: 4.0,
                vp_w: config.width as f32,
                vp_h: config.height as f32,
                _pad: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let cloud_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cloud.bind_group"),
            layout: &line_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: cloud_buffer.as_entire_binding() }],
        });
```

**What every line does — copy, slot. copy, slot.** Each resource is two moves: make a **buffer** (copy
your data to the GPU), then a **bind group** (tag that buffer with a slot). This block does it twice:

- `storage_buffer(…, &points)` → **copy** the points array to the GPU. (`create_buffer_init(…)` does the
  same for the small `CloudUniform` struct — both just copy CPU bytes into GPU memory.)
- `create_bind_group { layout, entries: [{ binding: 0, resource: the_buffer }] }` → **the plug**: `layout`
  is the shape (reuse a matching one); `entries` says *which* buffer goes at binding 0.

A **storage** buffer is a big array the shader indexes (`points[i]`); a **uniform** buffer is a few
globals every run reads (`cloud.size`). `usage` and `as_entire_binding()` are detail you can ignore until
it matters.

**4d. Add the five fields to `Ok(Self { … })`** — after `glyph_count`:

```rust
            point_buffer,
            point_bind_group,
            point_count,
            cloud_buffer,
            cloud_bind_group,
```

**4e. Draw after the spheres** in `clear()` — no vertex/index buffer, three verts per point:

```rust
            pass.set_pipeline(&self.pipelines.point);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.cloud_bind_group, &[]);   // cloud size + viewport, NOT the line uniform
            pass.set_bind_group(2, &self.instance_bind_group, &[]);
            pass.set_bind_group(3, &self.point_bind_group, &[]);
            pass.draw(0..3 * self.point_count, 0..1);              // 3 verts per point, no template
            draws += 1;
```

> The line uniform's `vp_h` isn't rewritten on resize in this codebase, so neither is the cloud's — dots
> hold their pixel size but skew slightly on a non-square window until the next rebuild. For exactly round
> dots after any resize, `queue.write_buffer` a fresh `CloudUniform` in `resize()` (same for the line one).

## The wiring — how `set_bind_group` reaches `@group` (read this if the numbers confuse you)

You've now written this four-line draw many times. Here is exactly what it means, once and for all.

### 1 · The CPU and the GPU are two separate machines

This is the idea everything else hangs on. **The CPU (your Rust program) and the GPU are two separate
computers with two separate memories.** Your `Vec<CloudPoint>` lives in CPU RAM. The shader runs on the
GPU. They **cannot share a variable** — the shader can't reach into your `Vec`, and your Rust can't call
the shader like a function.

Only **two kinds of things** ever cross between them:

1. **Copied bytes.** `storage_buffer(&device, "points.buffer", &points)` and `queue.write_buffer(…)`
   *copy* your bytes into GPU memory (a "buffer"). The GPU then has its own copy — editing your `Vec`
   afterwards changes nothing until you copy again.
2. **Commands.** `set_bind_group(…)` and `draw(…)` aren't run *on* the GPU by you — you *send* them to
   it. `draw(0..3*N)` means *"GPU: run the vertex shader 3·N times."* The GPU does it, massively parallel.

So the whole cloud draw is: **copy the points into a buffer → tell the GPU which buffer sits in which
slot → say "draw" → the shader (already on the GPU) reads those slots and writes pixels.**

<svg viewBox="0 0 700 300" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the CPU and GPU are separate machines with separate memory; only copied bytes and commands cross; your Vec is copied into a GPU buffer, the draw command tells the GPU to run the shader which reads the buffer and writes pixels" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="cg" markerWidth="9" markerHeight="9" refX="7" refY="3" orient="auto"><path d="M0,0 L7,3 L0,6 Z" fill="#c9a227"/></marker><marker id="cgv" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#5bbf87"/></marker></defs>
  <rect x="6" y="44" width="330" height="232" fill="#0e1116" stroke="#22323c"/>
  <rect x="364" y="44" width="330" height="232" fill="#0e130f" stroke="#24422c"/>
  <text x="171" y="36" fill="#6fb3ff" text-anchor="middle">CPU · your Rust program + RAM</text>
  <text x="529" y="36" fill="#5bbf87" text-anchor="middle">GPU · separate memory + 1000s of cores</text>
  <line x1="350" y1="44" x2="350" y2="276" stroke="#555" stroke-width="2" stroke-dasharray="4 4"/>
  <rect x="22" y="58" width="200" height="42" fill="none" stroke="#6fb3ff"/>
  <text x="122" y="76" fill="#d7dae0" text-anchor="middle" font-size="10">points: Vec&lt;CloudPoint&gt;</text>
  <text x="122" y="90" fill="#666" text-anchor="middle" font-size="9">your data, in RAM</text>
  <rect x="22" y="150" width="200" height="60" fill="none" stroke="#6fb3ff"/>
  <text x="32" y="168" fill="#d7dae0" font-size="10">clear() sends commands:</text>
  <text x="32" y="184" fill="#d7dae0" font-size="10">  set_bind_group(3, …)</text>
  <text x="32" y="199" fill="#d7dae0" font-size="10">  draw(0..3·N)</text>
  <rect x="470" y="58" width="206" height="42" fill="none" stroke="#5bbf87"/>
  <text x="573" y="76" fill="#d7dae0" text-anchor="middle" font-size="10">point_buffer</text>
  <text x="573" y="90" fill="#666" text-anchor="middle" font-size="9">a COPY of the bytes, in VRAM</text>
  <rect x="388" y="150" width="66" height="30" fill="none" stroke="#5bbf87"/>
  <text x="421" y="169" fill="#5bbf87" text-anchor="middle" font-size="10">slot 3</text>
  <rect x="470" y="146" width="206" height="42" fill="none" stroke="#5bbf87"/>
  <text x="573" y="164" fill="#d7dae0" text-anchor="middle" font-size="10">vertex shader × 3·N</text>
  <text x="573" y="178" fill="#666" text-anchor="middle" font-size="9">each: let p = points[vid/3]</text>
  <rect x="470" y="228" width="206" height="36" fill="none" stroke="#5bbf87"/>
  <text x="573" y="250" fill="#d7dae0" text-anchor="middle" font-size="10">pixels → framebuffer → screen</text>
  <text x="346" y="53" fill="#c9a227" text-anchor="middle" font-size="9">① COPY bytes</text>
  <line x1="222" y1="79" x2="470" y2="79" stroke="#c9a227" marker-end="url(#cg)"/>
  <line x1="222" y1="168" x2="386" y2="165" stroke="#c9a227" marker-end="url(#cg)"/>
  <text x="300" y="139" fill="#c9a227" text-anchor="middle" font-size="9">② attach buffer to slot</text>
  <line x1="222" y1="199" x2="468" y2="181" stroke="#c9a227" marker-end="url(#cg)"/>
  <text x="322" y="215" fill="#c9a227" text-anchor="middle" font-size="9">③ "run the shader"</text>
  <line x1="454" y1="165" x2="470" y2="165" stroke="#5bbf87" marker-end="url(#cgv)"/>
  <line x1="573" y1="188" x2="573" y2="228" stroke="#5bbf87" marker-end="url(#cgv)"/>
  <text x="585" y="212" fill="#666" font-size="9">reads slot,</text>
  <text x="585" y="223" fill="#666" font-size="9">writes pixels</text>
  <text x="350" y="292" fill="#e06c6c" text-anchor="middle" font-size="10">SEPARATE machines · NO shared variables · only COPIED BYTES + COMMANDS cross the line</text>
</svg>

Everything below — bind groups, slots, `@group` — is just the *machinery for those two crossings*: a
bind group is how you hand a buffer across; the slot number is how the command and the shader agree on
*which* buffer.

### 2 · The draw plugs each buffer into a numbered slot

`set_bind_group(N, X)` puts buffer-group `X` into **slot N**; the shader's `@group(N)` reads **slot N**.
The number is the only wire — line them up:

| slot | Rust draw — `set_bind_group(N, …)` | shader — `@group(N) @binding(0)` | the data |
|:--:|---|---|---|
| **0** | `mvp_bind_group` | `var<uniform> mvp` | camera matrix |
| **1** | `cloud_bind_group` | `var<uniform> cloud` | dot size + viewport |
| **2** | `instance_bind_group` | `var<storage> instances` | per-object model + color |
| **3** | `point_bind_group` | `var<storage> points` | the cloud's points |

That's the whole connection. (`@binding(0)` is always 0 here — each group holds just one buffer.)

**Why buffer + bind group repeats for every resource:** each is just *copy the data, then tag it with a
slot* — the same two lines, four times (mvp, cloud, instances, points). One recipe per resource.

## The key shader block, in plain words: `point.wgsl`

The **vertex shader** runs once per vertex — `3 × point_count` times. Each run turns *one point + one
corner number* into *one position on screen*:

```wgsl
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let p      = points[vid / 3u];               // WHICH point?  3 verts per point → vid/3
    let model  = instances[p.instance_id].model; // that point's object transform (its row)
    let world  = (model * vec4<f32>(p.position, 1.0)).xyz;   // local position → world
    let clip   = mvp * vec4<f32>(world, 1.0);    // world → screen (the camera)
    let corner = CORNERS[vid % 3u];              // WHICH of the 3 corners?  vid%3 = 0,1,2
    let px     = cloud.size;                     // dot size, in pixels
    let off    = corner * px * 2.0 / vec2<f32>(cloud.vp_w, cloud.vp_h) * clip.w;  // push corner out px px
    var o: VsOut;
    o.pos    = vec4<f32>(clip.xy + off, clip.zw);  // final screen pos = point + corner offset
    o.color  = p.color;
    o.corner = corner;                           // hand the corner to the fragment shader
    return o;
}
```

- `points[vid / 3u]` — the GPU numbers vertices 0,1,2,3,…; **three in a row = one triangle = one dot**, so
  `vid/3` is *which point* and `vid % 3` is *which of its 3 corners*.
- `model * position`, then `mvp * world` — put the point where the camera sees it on screen.
- `off = corner * px …` — spread the corner outward so the triangle is `px` pixels wide (holds its pixel
  size at any zoom — a screen-space billboard).

The **fragment shader** then runs once per pixel *inside* that triangle, and keeps only the round middle:

```wgsl
@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let d = length(in.corner);                 // distance from center: 0 in the middle, 1 at the rim
    let a = clamp((1.0 - d) * 8.0, 0.0, 1.0);  // fade to 0 near the rim → soft, anti-aliased edge
    if (a < 0.01) { discard; }                 // outside the circle → throw the pixel away
    return vec4<f32>(in.color.rgb, in.color.a * a);   // the colored dot
}
```

**In one line:** the vertex shader places the 3 corners of a camera-facing triangle per point; the
fragment shader carves the circle out of it.

## Organization checkpoint — the third field-cluster is the signal

Look at what `Gpu` now carries: the cylinder set (template vbo/ibo, index count, segment buffer,
bind group, count), the sphere set (same six), and the billboard set (four of the six). **The same
cluster of fields, three times** — plus a `new()` block and a `clear()` block each. That repetition,
not file length, is the signal to group:

```rust
/// One instanced drawable: a template + a storage table of rows + one draw.
/// Edges (31), handle spheres (32a), and later the gumball (57) and ghosts (63) are all this shape.
pub struct InstancedSet {
    pub template_vbo: wgpu::Buffer,
    pub template_ibo: wgpu::Buffer,
    pub index_count: u32,
    pub row_buffer: wgpu::Buffer,     // the group-3 storage table
    pub bind_group: wgpu::BindGroup,
    pub count: u32,                   // rows actually drawn (captured before any padding)
}
```

`Gpu` then holds `edges: InstancedSet`, `handles: InstancedSet`, … and each draw block collapses to
`self.edges.draw(&mut pass, &self.pipelines.cylinder)`. Doing this now is **optional** — the loose
fields work, and the course's text keeps naming them individually so either layout follows along —
but know the rule it demonstrates: *fields that always travel together are a struct; a struct with
behavior earns its own file; a **file** splits only when it gains a second kind of responsibility*
(which is exactly why `gpu.rs` becomes `gpu/mod.rs + adapters.rs` in 34b, gains `arena.rs` in 38a,
and `targets.rs` in 67 — each a new responsibility, never just new length).

## Run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

`read_xyz_from_str(include_str!(…bunny.xyz))` (Step 4b) loads the 397-point Stanford Bunny and it draws
as flat circles that hold their pixel size as you zoom. Console (F12):

```
perf: 60.0 fps | 16.67 ms | 6 draws | 5 objects
```

**6 draws** — 31's four + one sphere call (32a) + one point call (the whole cloud). Swap in a 100k-point
cloud (or a PLY once `read_ply` lands) and it stays 6: the tables grow, the call count doesn't.

## Recap

```
Ch 32a: handle points = instanced unit spheres, one draw.
Ch 32b: CLOUD points = screen-space BILLBOARDS. CloudPoint (32 B: local position, instance_id in the
        vec3 tail, color — zero padding, half a GlyphPoint; bytes/row is the budget at 1M
        points). Size is ONE global value in the cloud's OWN CloudUniform (16 B: size, vp_w, vp_h) —
        NOT line.thickness and NOT per-point (the billboard shares nothing with the line uniform).
        NO template: 3 verts from @builtin(vertex_index) make ONE equilateral triangle
        whose inscribed circle is the dot (½ a quad's verts, 1 tri not 144; overdraw negligible at
        cloud sizes); the fragment shader carves the incircle (discard length(corner) > 1),
        anti-aliased rim. Pipeline = sphere's with buffers:&[], alpha blend ON, depth write OFF.
        One draw for the whole cloud. Points done both ways: spheres where 3-D matters,
        billboards where count does.
```

Edited: `shaders/point.wgsl` (NEW), `engine/pipelines/build.rs` (`build_point_pipeline`),
`engine/pipelines/mod.rs` (`Pipelines.point`), `engine/gpu.rs` (`CloudPoint`, `CloudUniform`,
points buffer, one draw).

## Next

`33-camera-relative.md` — f32 world positions jitter far from the origin even with the f64 kernel. The
fix is **camera-relative rendering**: make the camera target the origin (f64), subtract it from every
instance row's translation before the f32 cast, and keep vertices local. A demo at x = 10 km stops
shimmering — the last precision piece before loading real scenes.
