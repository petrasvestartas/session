# 45 Pipelines are data — one `build`, fourteen descriptions

> First of six refactor lessons (45-50). Start from the end of lesson 44 (`docs/45_cloud_octree/`
> is that tree). Every lesson in the block keeps the frame pixel-identical: `./docs/_gate.sh`
> must print `gate OK` at the end of each one.
>
> How the edits in this block are written: a **Create** on a file that already exists replaces
> its whole content. A **Remove** names a first and a last line and deletes them and everything
> between (inclusive); written with `up to`, it stops just before the second line. When the two
> lines hold backticks they are quoted as two code blocks under the verb instead. Anchors are
> whole lines, leading spaces included.

<svg viewBox="0 0 700 290" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="eleven copy-paste pipeline builders become one PipelineDesc literal fed through one build function that returns a render pipeline" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <defs><marker id="pg" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#7ed37e"/></marker></defs>
  <text x="105" y="22" fill="#888" font-size="11" text-anchor="middle">before — 11 copy-paste builders</text>
  <g fill="none" stroke="#3a3a3a">
    <rect x="36" y="74" width="150" height="22"/><rect x="32" y="68" width="150" height="22"/><rect x="28" y="62" width="150" height="22"/>
    <rect x="24" y="56" width="150" height="22"/><rect x="20" y="50" width="150" height="22"/><rect x="16" y="44" width="150" height="22"/>
  </g>
  <text x="24" y="59" fill="#d7dae0" font-size="10">build_triangle_pipeline()</text>
  <text x="24" y="116" fill="#666" font-size="10">… build_splat_resolve_pipeline()</text>
  <text x="24" y="130" fill="#666" font-size="10">×11, each repeating the descriptor</text>
  <text x="212" y="118" fill="#7ed37e" font-size="18">▶</text>
  <text x="350" y="22" fill="#888" font-size="11" text-anchor="middle">after — a pipeline is data</text>
  <rect x="250" y="34" width="210" height="150" fill="none" stroke="#7ed37e"/>
  <g fill="#d7dae0" font-size="10">
    <text x="258" y="50">PipelineDesc {</text>
    <text x="258" y="65">  label, shader: &amp;ShaderModule,</text>
    <text x="258" y="80">  vs, fs: &amp;str,</text>
    <text x="258" y="95">  groups: &amp;[&amp;BindGroupLayout],</text>
    <text x="258" y="110">  vertex_buffers: &amp;[..],</text>
    <text x="258" y="125">  topology, blend: Option&lt;..&gt;,</text>
    <text x="258" y="140">  write_color: bool,</text>
    <text x="258" y="155">  depth: DepthMode,</text>
    <text x="258" y="170">}</text>
  </g>
  <rect x="250" y="196" width="210" height="22" fill="none" stroke="#7ed37e"/>
  <text x="258" y="211" fill="#d7dae0" font-size="10">Target { format, samples }</text>
  <g stroke="#7ed37e" marker-end="url(#pg)">
    <line x1="460" y1="109" x2="480" y2="109"/><line x1="460" y1="207" x2="500" y2="132"/><line x1="582" y1="109" x2="600" y2="109"/>
  </g>
  <rect x="482" y="88" width="100" height="42" fill="none" stroke="#7ed37e" stroke-width="1.3"/>
  <text x="532" y="106" fill="#d7dae0" font-size="10" text-anchor="middle">build(device,</text>
  <text x="532" y="121" fill="#d7dae0" font-size="10" text-anchor="middle">target, &amp;desc)</text>
  <rect x="602" y="88" width="90" height="42" fill="none" stroke="#6fb3ff"/>
  <text x="647" y="106" fill="#d7dae0" font-size="10" text-anchor="middle">wgpu::</text>
  <text x="647" y="121" fill="#d7dae0" font-size="10" text-anchor="middle">RenderPipeline</text>
  <text x="355" y="232" fill="#888" font-size="9" text-anchor="middle">15 render descs (one for the point pass)</text>
  <g fill="#888" font-size="10">
    <text x="16" y="258">DepthMode { Opaque, ReadOnly, ReadOnlyEqual, Always } — reverse-Z: nearer is GREATER</text>
    <text x="16" y="274">Pipelines::new(device, target, &amp;Layouts) — groups: 0 mvp · 1 line/cloud · 2 instances · 3 family rows</text>
  </g>
</svg>

## Goal

`pipelines/build.rs` shrinks from eleven copy-paste builders (845 lines) to one `build` fed by a
`PipelineDesc` literal per pipeline, and the nine bind-group layouts leave `Gpu::build` for a
`Layouts` struct. Same pixels, `Gpu` loses 14 fields.

## Why

Every builder repeated a 60-line descriptor to change two settings, so nobody could see WHICH two.
A pipeline is data: label, shader, entry points, groups, vertex buffers, topology, blend, colour
mask, depth mode. Written as one struct, the fourteen pipelines fit on one screen and a new one is
a literal, not a function. The same move takes the maths (`Mat4`, the eye solve, `Aabb`) out of
the GPU file into `math.rs`, deletes the `time` uniform (declared, never read) and the dead
`edges` pipeline, and drops the point pass's unused instance binding.

## Files

| file | change | lines after |
|---|---|---|
| `src/math.rs` | created | 135 |
| `src/engine/pipelines/layouts.rs` | created | 103 |
| `src/engine/pipelines/build.rs` | rewritten | 183 (was 845) |
| `src/engine/pipelines/mod.rs` | rewritten | 146 (was 80) |
| `src/shaders/splat.wgsl`, `src/shaders/triangle.wgsl` | one binding each | 276 · 135 |
| `src/shaders/edges.wgsl` | deleted | — |
| `src/engine/gpu/mod.rs` | edited | 2125 (was 2447) |
| `src/app/scene.rs`, `src/lib.rs`, `src/selftest.rs`, `examples/check_determinism.rs` | edited | — |

The build only compiles again at the end of the lesson: create the four files first, then edit.
Steps 3 and 4 use `Create` on a file that already exists: empty it, then paste the listing.

## Step 1 — `src/math.rs`

The maths the app and the engine share, moved out of `gpu/mod.rs` and `scene.rs` unchanged:
`Mat4`, `mat_mul`, `mat_to_f32`, `xform_point`, `grow_bounds`, the eye solve and the ortho
half-height as free functions, plus a small `Aabb` that replaces every loose `min/max` pair.

**Create `src/math.rs`**

```rust
//! Small f64/f32 math shared by the app and the engine: the column-major `Mat4`, point
//! transforms, the f32 `Aabb`, and the two camera facts recovered from a view-projection.
//! Nothing here touches wgpu or a kernel type beyond `Xform`.

use session_rust::Xform;

/// One object's world placement as the 16 raw column-major doubles the GPU row needs.
/// NOT a kernel `Xform`: that one heap-allocates twice per construction (name + guid), which
/// measured as 300 ms of a 90k-line sheet's walk for numbers nothing downstream names.
pub type Mat4 = [f64; 16];

/// `a * b` in the kernel's convention: column-major, index = col * 4 + row.
/// Matches `impl Mul for &Xform` element for element - and allocates nothing.
pub fn mat_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0.0f64; 16];
    for i in 0..4 {
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + i] * b[j * 4 + k];
            }
            out[j * 4 + i] = sum;
        }
    }
    out
}

/// The GPU edge: f64 world math stays CPU-side, the instance row is f32.
pub fn mat_to_f32(m: &Mat4) -> [f32; 16] {
    std::array::from_fn(|i| m[i] as f32)
}

/// A local point through a column-major placement, f64 inside, f32 at the edges.
pub fn xform_point(m: &Mat4, p: [f32; 3]) -> [f32; 3] {
    let x = p[0] as f64;
    let y = p[1] as f64;
    let z = p[2] as f64;
    [
        (m[0] * x + m[4] * y + m[8] * z + m[12]) as f32,
        (m[1] * x + m[5] * y + m[9] * z + m[13]) as f32,
        (m[2] * x + m[6] * y + m[10] * z + m[14]) as f32,
    ]
}

/// Widen a min/max pair to hold `p`.
pub fn grow_bounds(min: &mut [f32; 3], max: &mut [f32; 3], p: [f32; 3]) {
    for k in 0..3 {
        min[k] = min[k].min(p[k]);
        max[k] = max[k].max(p[k]);
    }
}

/// An axis-aligned box in f32 world units. `empty()` is inverted, so the first `grow` sets it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Aabb {
    /// The inverted box: every `grow` or `union` replaces it.
    pub fn empty() -> Self {
        Self { min: [f32::INFINITY; 3], max: [f32::NEG_INFINITY; 3] }
    }

    /// Widen the box to hold `p`.
    pub fn grow(&mut self, p: [f32; 3]) {
        grow_bounds(&mut self.min, &mut self.max, p);
    }

    /// Widen the box to hold `other`; an empty `other` changes nothing.
    pub fn union(&mut self, other: &Aabb) {
        for k in 0..3 {
            self.min[k] = self.min[k].min(other.min[k]);
            self.max[k] = self.max[k].max(other.max[k]);
        }
    }

    /// False for the empty box: nothing has been grown into it.
    pub fn is_finite(&self) -> bool {
        self.min.iter().chain(&self.max).all(|v| v.is_finite())
    }
}

/// The camera position, recovered from the view-projection alone: the eye is where clip x, y
/// and w all vanish, so rows 0, 1, 3 give a 3x3 solve. Orthographic has no eye (those rows are
/// dependent), so the fallback is the view direction pushed 1e9 back - an eye at infinity.
pub fn eye_from_view_proj(vp: &Xform) -> [f32; 3] {
    let r = |i: usize| [vp[(i, 0)], vp[(i, 1)], vp[(i, 2)], vp[(i, 3)]];
    let (a, b, c) = (r(0), r(1), r(3));

    // Cramer on [a b c] . p = -[a3 b3 c3]
    let det3 = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let rows = [[a[0], a[1], a[2]], [b[0], b[1], b[2]], [c[0], c[1], c[2]]];
    let rhs = [-a[3], -b[3], -c[3]];
    let d = det3(rows);

    // Scale-free singularity test: compare against the product of the row magnitudes, so it
    // fires on genuine dependence rather than on a scene whose units make everything small.
    let norm: f64 = rows.iter().map(|r| (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()).product();
    if d.abs() <= 1e-9 * norm.max(1e-30) {
        // Orthographic: row 3 carries no direction, so take the view axis from row 2 (depth)
        // and stand a long way back along it.
        let f = [vp[(2, 0)], vp[(2, 1)], vp[(2, 2)]];
        let len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt().max(1e-30);
        return [0, 1, 2].map(|k| (f[k] / len * 1.0e9) as f32);
    }

    [0, 1, 2].map(|k| {
        let mut m = rows;
        for row in 0..3 {
            m[row][k] = rhs[row];
        }
        (det3(m) / d) as f32
    })
}

/// Ortho half-height in world units (mm), 0.0 in perspective. The w row tells the projection
/// apart (ortho: all zeros); row 1 is the y basis scaled by s/h, so 1/|row1.xyz| is the
/// half-height. Left at 0.0 in ortho, every pen pins to a zoom-independent world size.
pub fn ortho_half_height(vp: &Xform) -> f32 {
    let w2 = vp[(3, 0)].powi(2) + vp[(3, 1)].powi(2) + vp[(3, 2)].powi(2);
    if w2 > 1e-12 {
        return 0.0;
    }
    let r1 = vp[(1, 0)].powi(2) + vp[(1, 1)].powi(2) + vp[(1, 2)].powi(2);
    if r1 <= 1e-30 {
        return 0.0;
    }
    (1.0 / r1.sqrt()) as f32
}
```

## Step 2 — `src/engine/pipelines/layouts.rs`

The bind-group layouts are the SHAPE of a bind group. They were nine blocks inside `Gpu::build`;
now `Layouts::new(device)` builds eight (the `time` layout goes) through two shared helpers and
three splat-specific ones.

**Create `src/engine/pipelines/layouts.rs`**

```rust
//! `Layouts` — every bind-group layout the viewer binds, built once per device.
//! A layout is the SHAPE of a bind group (binding index, stages, buffer kind); the buffers
//! themselves live in `gpu/`. Pipelines and bind groups both reference these, never their own.

/// One buffer binding, visible to `stages`.
fn buffer_entry(binding: u32, stages: wgpu::ShaderStages, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: stages,
        ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
        count: None,
    }
}

/// One uniform buffer at binding 0.
fn uniform_layout(device: &wgpu::Device, label: &str, stages: wgpu::ShaderStages) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[buffer_entry(0, stages, wgpu::BufferBindingType::Uniform)],
    })
}

/// One read-only storage buffer at binding 0, vertex-visible: the row table every ink lane reads.
fn storage_layout(device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[buffer_entry(0, wgpu::ShaderStages::VERTEX, wgpu::BufferBindingType::Storage { read_only: true })],
    })
}

/// A vertex-visible read-only storage binding: the point pass pulls everything by vertex index.
fn splat_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    buffer_entry(binding, wgpu::ShaderStages::VERTEX, wgpu::BufferBindingType::Storage { read_only: true })
}

/// Splat group 0: the frame (mvp, cloud uniform) and the record table.
fn splat_group0_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.group0.layout"),
        entries: &[
            buffer_entry(0, wgpu::ShaderStages::VERTEX, wgpu::BufferBindingType::Uniform),
            buffer_entry(1, wgpu::ShaderStages::VERTEX, wgpu::BufferBindingType::Uniform),
            splat_entry(2),
        ],
    })
}

/// Splat group 1: a lane's points (pos, col, nrm). The depth and colour the lane draws into
/// are render attachments of the point pass, not bindings.
fn splat_group1_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.group1.layout"),
        entries: &[
            splat_entry(0),
            splat_entry(1),
            splat_entry(2),
        ],
    })
}

/// The resolve pass reads the point pass's two targets from its fragment stage: the depth
/// texture (nearest point per pixel, 0 = empty) and its colour.
fn splat_resolve_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("splat.resolve.layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    })
}

/// The eight bind-group layouts. Group scheme for every draw: 0 = mvp, 1 = line/cloud uniform,
/// 2 = instances, 3 = the family's row table.
pub struct Layouts {
    pub mvp: wgpu::BindGroupLayout,
    pub line: wgpu::BindGroupLayout,
    pub instance: wgpu::BindGroupLayout,
    pub segment: wgpu::BindGroupLayout,
    pub glyph: wgpu::BindGroupLayout,
    pub splat_group0: wgpu::BindGroupLayout,
    pub splat_group1: wgpu::BindGroupLayout,
    pub splat_resolve: wgpu::BindGroupLayout,
}

impl Layouts {
    /// Build every layout once; they outlive any pipeline or bind group made from them.
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            mvp: uniform_layout(device, "mvp.layout", wgpu::ShaderStages::VERTEX),
            // FRAGMENT too: the splat resolve reads the cloud uniform (bound with this layout)
            // from its fragment stage.
            line: uniform_layout(device, "line.layout", wgpu::ShaderStages::VERTEX_FRAGMENT),
            instance: storage_layout(device, "instance.layout"),
            segment: storage_layout(device, "segments.layout"),
            glyph: storage_layout(device, "glyphs.layout"),
            splat_group0: splat_group0_layout(device),
            splat_group1: splat_group1_layout(device),
            splat_resolve: splat_resolve_layout(device),
        }
    }
}
```

## Step 3 — `src/engine/pipelines/build.rs`

Replace the whole file. `Target` is where a pipeline draws, `DepthMode` is the four depth
behaviours the viewer uses (reverse-Z: nearer is greater), `PipelineDesc` is everything that
differs between pipelines, and `build` is the only place wgpu is asked for a render pipeline.

**Create `src/engine/pipelines/build.rs`**

```rust
//! Pipelines are data. `PipelineDesc` names the ten things that differ between the viewer's
//! render pipelines; `build` turns one into a `wgpu::RenderPipeline` and is the only place
//! wgpu is asked for one. Shader modules are made by the caller, once per source.

use std::sync::OnceLock;

/// Where a pipeline draws: the surface format and the MSAA sample count of the pass.
///
/// MSAA cannot be mixed WITHIN a frame - sample count is a property of the render PASS - so
/// the viewer picks one per SCENE (`Gpu::msaa_now`) and rebuilds every pipeline on a flip.
#[derive(Clone, Copy)]
pub struct Target {
    pub format: wgpu::TextureFormat,
    pub samples: u32,
}

/// How a pipeline treats depth. Every compare is reverse-Z: nearer is GREATER.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DepthMode {
    /// Write, strict `Greater`: solids, and the depth-only prepasses.
    Opaque,
    /// Test only, strict `Greater`: sheet fills and the grid.
    ReadOnly,
    /// Test only, `GreaterEqual`: blended ink that must tie with its prepass and with faces.
    ReadOnlyEqual,
    /// No test, no write: the background, and `VIEWER_NO_DEPTH`.
    Always,
}

impl DepthMode {
    /// The (write, compare) pair wgpu wants.
    fn state(self) -> (bool, wgpu::CompareFunction) {
        match self {
            DepthMode::Opaque => (true, wgpu::CompareFunction::Greater),
            DepthMode::ReadOnly => (false, wgpu::CompareFunction::Greater),
            DepthMode::ReadOnlyEqual => (false, wgpu::CompareFunction::GreaterEqual),
            DepthMode::Always => (false, wgpu::CompareFunction::Always),
        }
    }
}

/// Everything `build` needs to make one render pipeline. A pipeline is data, not a function.
pub struct PipelineDesc<'a> {
    pub label: &'a str,
    pub shader: &'a wgpu::ShaderModule,
    pub vs: &'a str,
    pub fs: &'a str,
    pub groups: &'a [&'a wgpu::BindGroupLayout],
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    pub topology: wgpu::PrimitiveTopology,
    pub blend: Option<wgpu::BlendState>,
    /// False = depth-only prepass: every colour channel masked, only depth lands.
    pub write_color: bool,
    pub depth: DepthMode,
}

const INSTANCE_ID_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 3,
    format: wgpu::VertexFormat::Uint32,
}];

const TEMPLATE_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Float32x3,
}];

/// Vertex-buffer layout for the per-vertex instance-row id (`@location(3)`, one `u32` per vertex).
pub fn instance_id_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 4,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &INSTANCE_ID_ATTRIBS,
    }
}

/// Vertex-buffer layout for the unit-cylinder / quad template positions (`@location(0)`, one `vec3<f32>`).
pub fn template_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 12,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &TEMPLATE_ATTRIBS,
    }
}

/// `mode`, or `Always` when `VIEWER_NO_DEPTH` is set. Read once; env vars never exist on wasm.
pub fn depth_or_always(mode: DepthMode) -> DepthMode {
    static NO_DEPTH: OnceLock<bool> = OnceLock::new();
    if *NO_DEPTH.get_or_init(|| std::env::var("VIEWER_NO_DEPTH").is_ok()) { DepthMode::Always } else { mode }
}

/// Compile one WGSL source into a module; the caller keeps it and shares it across pipelines.
pub fn module(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}

/// The pipeline layout for `groups`, in slot order.
fn pipeline_layout(device: &wgpu::Device, label: &str, groups: &[&wgpu::BindGroupLayout]) -> wgpu::PipelineLayout {
    let groups: Vec<Option<&wgpu::BindGroupLayout>> = groups.iter().map(|g| Some(*g)).collect();
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &groups,
        immediate_size: 0,
    })
}

/// One render pipeline from its description. Everything not in the desc is the same for all
/// of them: one colour target, `Depth32Float`, no cull, no hardware bias, fill mode.
pub fn build(device: &wgpu::Device, target: Target, desc: &PipelineDesc) -> wgpu::RenderPipeline {
    let layout = pipeline_layout(device, desc.label, desc.groups);
    let (depth_write, depth_compare) = desc.depth.state();
    let write_mask = if desc.write_color { wgpu::ColorWrites::ALL } else { wgpu::ColorWrites::empty() };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(desc.label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: desc.shader,
            entry_point: Some(desc.vs),
            buffers: desc.vertex_buffers,
            compilation_options: Default::default(),
        },
        // The pass HAS a colour attachment, so a depth-only pipeline still declares one and
        // masks every channel - Dawn rejects an empty target list against a colour pass.
        fragment: Some(wgpu::FragmentState {
            module: desc.shader,
            entry_point: Some(desc.fs),
            targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: desc.blend, write_mask })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: desc.topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        // No hardware bias anywhere: the units of `constant` on a float depth format are
        // implementation-defined, so faces recede in triangle.wgsl instead (FACE_PUSH).
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(depth_write),
            depth_compare: Some(depth_compare),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: target.samples,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}
```

## Step 4 — `src/engine/pipelines/mod.rs`

Replace the whole file. Nine shader modules are compiled once, then fifteen `PipelineDesc`
literals - fourteen for the frame's target and one, `splat_points`, for the point pass's own
single-sample RGBA8 target. The two `VIEWER_NO_DEPTH` branches survive as `depth_or_always`
on `sphere` and `ribbon_solid`.

**Create `src/engine/pipelines/mod.rs`**

```rust
//! `Pipelines` — every pipeline the viewer draws with, as data: one `PipelineDesc` literal
//! each, built by `build::build` from shader modules made once per source. Rebuilt whole when
//! the MSAA sample count flips (the count belongs to the pass); the point pass's pipeline
//! ignores the flip, its target is always one sample.

pub mod build;
pub mod layouts;

pub use build::Target;
pub use layouts::Layouts;

use build::{build, depth_or_always, instance_id_layout, module, template_layout};
use build::{DepthMode, PipelineDesc};
use session_rust::RenderVertex;
use wgpu::PrimitiveTopology::{LineList, TriangleList};

/// Smooth AA feather + hairline fade on every blended lane.
const ALPHA: Option<wgpu::BlendState> = Some(wgpu::BlendState::ALPHA_BLENDING);

/// The point pass's colour target: linear RGBA8, the packed point colour as-is, one sample.
pub const SPLAT_COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// Every pipeline the viewer draws with, built once at startup and again on an MSAA flip.
pub struct Pipelines {
    pub triangle: wgpu::RenderPipeline,
    /// Same program, depth WRITE off: a sheet's fills are exactly coplanar, so they composite
    /// in draw order (a painter's document) instead of flickering over one shared depth value.
    pub triangle_sheet: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
    pub cylinder: wgpu::RenderPipeline,
    pub sphere: wgpu::RenderPipeline,
    pub sphere_depth: wgpu::RenderPipeline,
    pub ribbon: wgpu::RenderPipeline,
    pub ribbon_depth: wgpu::RenderPipeline,
    /// The flat lane's shader over the SOLID table; `GreaterEqual` is load-bearing (a mesh
    /// edge sits EXACTLY on its faces' depth, and strict `Greater` shreds it).
    pub ribbon_solid: wgpu::RenderPipeline,
    /// Depth-only prepasses for the solid ink: binary at half coverage, so the blended colour
    /// passes never write depth and their AA feather cannot depth-reject a later stroke.
    pub ribbon_solid_depth: wgpu::RenderPipeline,
    pub glyph: wgpu::RenderPipeline,
    pub glyph_depth: wgpu::RenderPipeline,
    /// Fullscreen composite of the point pass's targets into the frame.
    pub splat_resolve: wgpu::RenderPipeline,
    /// One quad per point into the clouds' own depth + colour targets; the depth test keeps
    /// the nearest point per pixel. Its target is `SPLAT_COLOR_FORMAT` at one sample, whatever
    /// the frame's.
    pub splat_points: wgpu::RenderPipeline,
}

impl Pipelines {
    /// Build every pipeline for `target` from the shared layouts. One shader module per source.
    pub fn new(device: &wgpu::Device, target: Target, l: &Layouts) -> Self {
        let triangle = module(device, "triangle.shader", include_str!("../../shaders/triangle.wgsl"));
        let grid = module(device, "grid.shader", include_str!("../../shaders/grid.wgsl"));
        let background = module(device, "background.shader", include_str!("../../shaders/background.wgsl"));
        let cylinder = module(device, "cylinder.shader", include_str!("../../shaders/cylinder.wgsl"));
        let sphere = module(device, "sphere.shader", include_str!("../../shaders/sphere.wgsl"));
        let ribbon = module(device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl"));
        let glyph = module(device, "glyph.shader", include_str!("../../shaders/glyph.wgsl"));
        let resolve = module(device, "splat.resolve.shader", include_str!("../../shaders/splat_resolve.wgsl"));
        let splat = module(device, "splat.shader", include_str!("../../shaders/splat.wgsl"));

        // Group scheme: 0 = mvp, 1 = line/cloud uniform, 2 = instances, 3 = the family's rows.
        let solid = [&l.mvp, &l.line, &l.instance];
        let seg = [&l.mvp, &l.line, &l.instance, &l.segment];
        let gly = [&l.mvp, &l.line, &l.instance, &l.glyph];
        let splat_groups = [&l.splat_group0, &l.splat_group1];

        Self {
            triangle: build(device, target, &PipelineDesc {
                label: "triangle", shader: &triangle, vs: "vs_main", fs: "fs_main",
                groups: &solid, vertex_buffers: &[RenderVertex::layout(), instance_id_layout()],
                topology: TriangleList, blend: ALPHA, write_color: true, depth: DepthMode::Opaque,
            }),
            triangle_sheet: build(device, target, &PipelineDesc {
                label: "triangle.sheet", shader: &triangle, vs: "vs_main", fs: "fs_main",
                groups: &solid, vertex_buffers: &[RenderVertex::layout(), instance_id_layout()],
                topology: TriangleList, blend: ALPHA, write_color: true, depth: DepthMode::ReadOnly,
            }),
            grid: build(device, target, &PipelineDesc {
                label: "grid", shader: &grid, vs: "vs_main", fs: "fs_main",
                groups: &[&l.mvp, &l.line], vertex_buffers: &[],
                topology: LineList, blend: None, write_color: true, depth: DepthMode::ReadOnly,
            }),
            background: build(device, target, &PipelineDesc {
                label: "background", shader: &background, vs: "vs_main", fs: "fs_main",
                groups: &[], vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: true, depth: DepthMode::Always,
            }),
            cylinder: build(device, target, &PipelineDesc {
                label: "cylinder", shader: &cylinder, vs: "vs_main", fs: "fs_main",
                groups: &seg, vertex_buffers: &[template_layout()],
                topology: TriangleList, blend: None, write_color: true, depth: DepthMode::Opaque,
            }),
            sphere: build(device, target, &PipelineDesc {
                label: "sphere", shader: &sphere, vs: "vs_main", fs: "fs_main",
                groups: &gly, vertex_buffers: &[template_layout()],
                topology: TriangleList, blend: ALPHA, write_color: true,
                depth: depth_or_always(DepthMode::ReadOnlyEqual), // VIEWER_NO_DEPTH
            }),
            sphere_depth: build(device, target, &PipelineDesc {
                label: "sphere.depth", shader: &sphere, vs: "vs_main", fs: "fs_depth",
                groups: &gly, vertex_buffers: &[template_layout()],
                topology: TriangleList, blend: None, write_color: false, depth: DepthMode::Opaque,
            }),
            ribbon: build(device, target, &PipelineDesc {
                label: "ribbon", shader: &ribbon, vs: "vs_main", fs: "fs_main",
                groups: &seg, vertex_buffers: &[],
                topology: TriangleList, blend: ALPHA, write_color: true, depth: DepthMode::ReadOnlyEqual,
            }),
            ribbon_depth: build(device, target, &PipelineDesc {
                label: "ribbon.depth", shader: &ribbon, vs: "vs_main", fs: "fs_depth",
                groups: &seg, vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: false, depth: DepthMode::Opaque,
            }),
            ribbon_solid: build(device, target, &PipelineDesc {
                label: "ribbon.solid", shader: &ribbon, vs: "vs_main", fs: "fs_main",
                groups: &seg, vertex_buffers: &[],
                topology: TriangleList, blend: ALPHA, write_color: true,
                depth: depth_or_always(DepthMode::ReadOnlyEqual), // VIEWER_NO_DEPTH
            }),
            ribbon_solid_depth: build(device, target, &PipelineDesc {
                label: "ribbon.solid.depth", shader: &ribbon, vs: "vs_main", fs: "fs_depth",
                groups: &seg, vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: false, depth: DepthMode::Opaque,
            }),
            glyph: build(device, target, &PipelineDesc {
                label: "glyph", shader: &glyph, vs: "vs_main", fs: "fs_main",
                groups: &gly, vertex_buffers: &[],
                topology: TriangleList, blend: ALPHA, write_color: true, depth: DepthMode::ReadOnlyEqual,
            }),
            glyph_depth: build(device, target, &PipelineDesc {
                label: "glyph.depth", shader: &glyph, vs: "vs_main", fs: "fs_depth",
                groups: &gly, vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: false, depth: DepthMode::Opaque,
            }),
            splat_resolve: build(device, target, &PipelineDesc {
                label: "splat.resolve", shader: &resolve, vs: "vs_main", fs: "fs_main",
                groups: &[&l.line, &l.splat_resolve], vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: true, depth: DepthMode::Opaque,
            }),
            splat_points: build(device, Target { format: SPLAT_COLOR_FORMAT, samples: 1 }, &PipelineDesc {
                label: "splat.points", shader: &splat, vs: "vs_point", fs: "fs_point",
                groups: &splat_groups, vertex_buffers: &[],
                topology: TriangleList, blend: None, write_color: true, depth: DepthMode::Opaque,
            }),
        }
    }
}
```

## Step 5 — `src/shaders/splat.wgsl`

The point pass bound the whole instance table and never read it; the record table moves to
binding 2.

**Find** in `src/shaders/splat.wgsl`:

```wgsl
@group(0) @binding(2) var<storage, read> instances_unused: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read> table: array<u32>;
```

**Replace with:**

```wgsl
@group(0) @binding(2) var<storage, read> table: array<u32>;
```

## Step 6 — `src/shaders/triangle.wgsl`

The `time` uniform was declared here and read nowhere.

**Find** in `src/shaders/triangle.wgsl`:

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> time: f32;
```

**Replace with:**

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
```

## Step 7 — `src/shaders/edges.wgsl`

Built at every start since lesson 31 and bound by no pass; Step 4 already dropped its builder.

**Delete `src/shaders/edges.wgsl`**

## Step 8 — `src/engine/gpu/mod.rs`

The layouts, the point pipeline, the `time` uniform and the maths leave; `Pipelines::new`
takes three arguments. Each edit removes a block, re-roots a name onto `layouts` or `bounds`, or
restores a doc comment that a removed block carried.

**Find** in `src/engine/gpu/mod.rs`:

```rust
use crate::engine::pipelines::Pipelines;
```

**Replace with:**

```rust
use crate::engine::pipelines::{Pipelines, Target, Layouts, SPLAT_COLOR_FORMAT};
```

The point pass's target format is the pipeline's business now. **Find** in `src/engine/gpu/mod.rs`:

```rust
/// Linear RGBA8: the point colours are packed 8-bit values, the resolve reads them back as-is.
const SPLAT_COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
```

**Delete**

**Find** in `src/engine/gpu/mod.rs`:

```rust
use session_rust::{Xform, RenderVertex, Point};
```

**Add below it:**

```rust
use crate::math::{Mat4, mat_to_f32, eye_from_view_proj, ortho_half_height, Aabb};
```

`Mat4` and its two functions now live in `math.rs`. The doc comment above them belonged to
`ArenaUpload`; the next edit puts it back where it belongs.

**Find** in `src/engine/gpu/mod.rs`:

```rust
const INK_DEPTH_PREPASS: bool = false;

/// Everything `Gpu` needs to fill its buffers, built and owened by `app::scene::Scene`,
/// the engine borrows it, uploads, and forgets.
/// Lanes stay apart (SOLID pipes/spheres vs flat segments/glyphs)
/// and are spliced solid-first at upload.
/// `objects` holds the TRUE per-object transfrom + tint + flags.
/// `Gpu` builds instance rows from it and rebases them as the camera moves.
/// No Mesh, no Session, no wgpu type on the app side of this line.
/// One object's world placement as the 16 raw column-major doubles the GPU row needs.
///
/// NOT a kernel `Xform`: that struct carries `typ`/`name` Strings and a guid `OnceLock`, so
/// `Xform::identity()` heap-allocates TWICE per call and every arena row cost two more on the
/// clone into `objects_base`. On a 90k-line sheet that was ~400k allocations - 300 ms of the
/// walk - to carry 128 bytes of numbers nothing downstream ever reads a name off.
pub type Mat4 = [f64; 16];

/// `a * b` in the kernel's convention: column-major, index = col * 4 + row.
/// Matches `impl Mul for &Xform` element for element - and allocates nothing.
pub fn mat_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0.0f64; 16];
    for i in 0..4 {
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + i] * b[j * 4 + k];
            }
            out[j * 4 + i] = sum;
        }
    }
    out
}

/// The GPU edge: f64 world math stays CPU-side, the instance row is f32.
pub fn mat_to_f32(m: &Mat4) -> [f32; 16] {
    std::array::from_fn(|i| m[i] as f32)
}
```

**Replace with:**

```rust
const INK_DEPTH_PREPASS: bool = false;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub children: [i32; 8],
}

```

**Add below it:**

```rust
/// Everything `Gpu` needs to fill its buffers, built and owened by `app::scene::Scene`,
/// the engine borrows it, uploads, and forgets.
/// Lanes stay apart (SOLID pipes/spheres vs flat segments/glyphs)
/// and are spliced solid-first at upload.
/// `objects` holds the TRUE per-object transfrom + tint + flags.
/// `Gpu` builds instance rows from it and rebases them as the camera moves.
/// No Mesh, no Session, no wgpu type on the app side of this line.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub min: [f32; 3],
    pub max: [f32; 3],
```

**Replace with:**

```rust
    pub bounds: Aabb,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
impl ArenaUpload {
```

**Add below it:**

```rust
    /// Every lane empty and the box inverted, ready for the first walk.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
```

**Replace with:**

```rust
            bounds: Aabb::empty(),
```

The three `time` fields go.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub line_bind_group: wgpu::BindGroup,
    pub time: f32,  // shared: animation
    pub time_buffer: wgpu::Buffer,
    pub time_bind_group: wgpu::BindGroup,
```

**Replace with:**

```rust
    pub line_bind_group: wgpu::BindGroup,
```

Nine layout fields become one.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    // Layouts surfvive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    mvp_layout: wgpu::BindGroupLayout,
    time_layout: wgpu::BindGroupLayout,
    instance_layout: wgpu::BindGroupLayout,
    line_layout: wgpu::BindGroupLayout,
    segment_layout: wgpu::BindGroupLayout,
    glyph_layout: wgpu::BindGroupLayout,
```

**Replace with:**

```rust
    /// Layouts survive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    pub layouts: Layouts,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    splat_recs: wgpu::Buffer,
    splat_group0_layout: wgpu::BindGroupLayout,
    splat_group1_layout: wgpu::BindGroupLayout,
    splat_resolve_layout: wgpu::BindGroupLayout,
```

**Replace with:**

```rust
    splat_recs: wgpu::Buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    splat_resolve_group: wgpu::BindGroup,
    splat_point_pipeline: wgpu::RenderPipeline, // one quad per point into the two targets above
```

**Replace with:**

```rust
    splat_resolve_group: wgpu::BindGroup,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub scene_min: [f32; 3],
    pub scene_max: [f32; 3],
```

**Replace with:**

```rust
    pub bounds: Aabb,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        Self::build(None, width.max(1), height.max(1)).await
    }

```

**Add below it:**

```rust
    /// The shared constructor: negotiate the device, make every layout, buffer, bind group and
    /// pipeline, and start with an empty scene.
```

`Layouts::new` replaces the first layout block; the remaining blocks go in the edits that follow.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let mvp_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("mvp.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu:: BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });
```

**Replace with:**

```rust
        // Every bind-group layout, once; pipelines and bind groups are made from these.
        let layouts = Layouts::new(&device);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            layout: &mvp_layout,
```

**Replace with:**

```rust
            layout: &layouts.mvp,
```

The time buffer, its layout and its bind group.

**Find** in `src/engine/gpu/mod.rs`:

```rust
            }],
        });

        // Time Uniform
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("time.buffer"),
            contents: bytemuck::bytes_of(&0.0f32),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let time_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("time.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None},
                count: None,
            }],
        });

        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("time.bind_group"),
            layout: &time_layout,
            entries: &[wgpu::BindGroupEntry{ binding: 0, resource: time_buffer.as_entire_binding() }],
```

**Replace with:**

```rust
            }],
```

The instance layout block, whole: the `});` named as the last line is the one that closes it.

**Remove** `src/engine/gpu/mod.rs` `        let (scene_min, scene_max) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);` **through** `        });`

**Find** in `src/engine/gpu/mod.rs`:

```rust
            layout: &instance_layout,
```

**Replace with:**

```rust
            layout: &layouts.instance,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let segment_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("segments.layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false, min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipe_bind_group = Self::mk_rows_group(&device, &segment_layout, "pipes.bind_group", &pipe_buffer);
        let segment_bind_group = Self::mk_rows_group(&device, &segment_layout, "segments.bind_group", &segment_buffer);
```

**Replace with:**

```rust
        let pipe_bind_group = Self::mk_rows_group(&device, &layouts.segment, "pipes.bind_group", &pipe_buffer);
        let segment_bind_group = Self::mk_rows_group(&device, &layouts.segment, "segments.bind_group", &segment_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let glyph_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("glyphs.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let sphere_bind_group = Self::mk_rows_group(&device, &glyph_layout, "spheres.bind_group", &sphere_buffer);
        let glyph_bind_group = Self::mk_rows_group(&device, &glyph_layout, "glyphs.bind_group", &glyph_buffer);
```

**Replace with:**

```rust
        let sphere_bind_group = Self::mk_rows_group(&device, &layouts.glyph, "spheres.bind_group", &sphere_buffer);
        let glyph_bind_group = Self::mk_rows_group(&device, &layouts.glyph, "glyphs.bind_group", &glyph_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let line_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("line.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                // FRAGMENT too: the flat lane's fragment stage reads the viewport size to
                // recover the fragment's ndc for the face-plane depth solve (ribbon.wgsl
                // `ink_depth`). Everything else still only touches it from the vertex stage.
                visibility: wgpu::ShaderStages::VERTEX.union(wgpu::ShaderStages::FRAGMENT),
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count:None
            }],
        });

        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"),
            layout: &line_layout,
```

**Replace with:**

```rust
        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"),
            layout: &layouts.line,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            layout: &line_layout,
```

**Replace with:**

```rust
            layout: &layouts.line,
```

The three splat layouts go. This Remove also takes the head of the `splat_group0` construction;
the next edit restores it with the `layouts` names.

**Remove** `src/engine/gpu/mod.rs` `        let splat_group0_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** `            &instance_buffer,`

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let splat_recs = zeroed_buffer(&device, "splat.rescales", 16 + 256 * 144, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
```

**Add below it:**

```rust
        let splat_group0 = Self::mk_splat_group0(
            &device,
            &layouts.splat_group0,
            &mvp_buffer,
            &cloud_buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            &splat_group1_layout,
```

**Replace with:**

```rust
            &layouts.splat_group1,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let splat_group0_stream = Self::mk_splat_group0(&device, &splat_group0_layout, &mvp_buffer, &cloud_buffer, &instance_buffer, &splat_stream_recs);
        let splat_group1_stream = Self::mk_splat_group1(&device, &splat_group1_layout, &stream_pos_buf, &stream_col_buf, &stream_nrm_buf);
        let splat_resolve_group = Self::mk_splat_resolve_group(
            &device,
            &splat_resolve_layout,
```

**Replace with:**

```rust
        let splat_group0_stream = Self::mk_splat_group0(&device, &layouts.splat_group0, &mvp_buffer, &cloud_buffer, &splat_stream_recs);
        let splat_group1_stream = Self::mk_splat_group1(&device, &layouts.splat_group1, &stream_pos_buf, &stream_col_buf, &stream_nrm_buf);
        let splat_resolve_group = Self::mk_splat_resolve_group(
            &device,
            &layouts.splat_resolve,
```

The point shader, its layout and its pipeline are built by `Pipelines::new` now; the edit after
these three removals adds the call. Each removal ends at the first `        });` after its
first line.

**Remove** `src/engine/gpu/mod.rs` `        let splat_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{` **through** `        });`

**Remove** `src/engine/gpu/mod.rs` `        let splat_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{` **through** `        });`

**Remove** `src/engine/gpu/mod.rs` `        let splat_point_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {` **through** `        });`

The old ten-argument call goes with them.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Pipelines
        let pipelines = Pipelines::new(
            &device,
            samples,
            config.format,
            &mvp_layout,
            &time_layout,
            &instance_layout,
            &line_layout,
            &segment_layout,
            &glyph_layout,
            &splat_resolve_layout,
        );
```

**Delete**

**Find** in `src/engine/gpu/mod.rs`:

```rust
            &layouts.splat_resolve,
            &splat_depth_view,
            &splat_color_view,
        );

```

**Add below it:**

```rust
        // Pipelines - one set per sample count (the point pass's ignores the count).
        let pipelines = Pipelines::new(&device, Target { format: config.format, samples }, &layouts);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            line_bind_group,
            time_buffer,    // shared: animation
            time_bind_group,
            time: 0.0,
```

**Replace with:**

```rust
            line_bind_group,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            mvp_layout,
            time_layout,
            instance_layout,
            line_layout,
            segment_layout,
            glyph_layout,
```

**Replace with:**

```rust
            layouts,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            splat_recs,
            splat_group0_layout,
            splat_group1_layout,
            splat_resolve_layout,
```

**Replace with:**

```rust
            splat_recs,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            splat_resolve_group,
            splat_point_pipeline,
```

**Replace with:**

```rust
            splat_resolve_group,
```

The empty box: infinite in both directions, so the first upload's `union` replaces it,
`grow_scene` still reads it as unset (`min >= max`), and `camera.fit` skips it (it returns on a
non-finite box).

**Find** in `src/engine/gpu/mod.rs`:

```rust
            scene_min,
            scene_max,
```

**Replace with:**

```rust
            bounds: Aabb::empty(),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.instance_bind_group = Self::mk_rows_group(&self.device, &self.instance_layout, "instances.bind_group", &self.instance_buffer);
```

**Replace with:**

```rust
            self.instance_bind_group = Self::mk_rows_group(&self.device, &self.layouts.instance, "instances.bind_group", &self.instance_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.pipe_bind_group = Self::mk_rows_group(&self.device, &self.segment_layout, "pipes.bind_group", &self.pipe_buffer);
```

**Replace with:**

```rust
            self.pipe_bind_group = Self::mk_rows_group(&self.device, &self.layouts.segment, "pipes.bind_group", &self.pipe_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.segment_bind_group = Self::mk_rows_group(&self.device, &self.segment_layout, "segments.bind_group", &self.segment_buffer);
```

**Replace with:**

```rust
            self.segment_bind_group = Self::mk_rows_group(&self.device, &self.layouts.segment, "segments.bind_group", &self.segment_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.sphere_bind_group = Self::mk_rows_group(&self.device, &self.glyph_layout, "spheres.bind_group", &self.sphere_buffer);
```

**Replace with:**

```rust
            self.sphere_bind_group = Self::mk_rows_group(&self.device, &self.layouts.glyph, "spheres.bind_group", &self.sphere_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.glyph_bind_group = Self::mk_rows_group(&self.device, &self.glyph_layout, "glyphs.bind_group", &self.glyph_buffer);
```

**Replace with:**

```rust
            self.glyph_bind_group = Self::mk_rows_group(&self.device, &self.layouts.glyph, "glyphs.bind_group", &self.glyph_buffer);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        for k in 0..3 {
            self.scene_min[k] = self.scene_min[k].min(up.min[k]);
            self.scene_max[k] = self.scene_max[k].max(up.max[k]);
        }
```

**Replace with:**

```rust
        self.bounds.union(&up.bounds);
```

And where a clear starts the box over. **Find** in `src/engine/gpu/mod.rs`:

```rust
        self.scene_min = [f32::INFINITY; 3];
        self.scene_max = [f32::NEG_INFINITY; 3];
```

**Replace with:**

```rust
        self.bounds = Aabb::empty();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.pipelines = Pipelines::new(
                &self.device,
                samples,
                self.config.format,
                &self.mvp_layout,
                &self.time_layout,
                &self.instance_layout,
                &self.line_layout,
                &self.segment_layout,
                &self.glyph_layout,
                &self.splat_resolve_layout,
            );
```

**Replace with:**

```rust
            self.pipelines = Pipelines::new(&self.device, Target { format: self.config.format, samples }, &self.layouts);
```

`splat_entry` moved to `layouts.rs`; only the bind-group builders stay, and they lose the
instance buffer that binding 2 no longer wants.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    // splat helpers - one compute-visible buffer entry, and the three bind groups,
    // rebuilt whenever any bound buffer is recreated (set_scene, resize)
    fn splat_entry(
        binding: u32,
        ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry{
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
            count: None }
    }

```

**Replace with:**

```rust
    /// Splat group 0 for one lane: the frame uniforms and that lane's record table. The three
    /// splat groups are rebuilt whenever any bound buffer is recreated (set_scene, resize).
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        cloud: &wgpu::Buffer,
        instances: &wgpu::Buffer,
```

**Replace with:**

```rust
        cloud: &wgpu::Buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                wgpu::BindGroupEntry{binding: 2, resource: instances.as_entire_binding()},
                wgpu::BindGroupEntry{binding: 3, resource: recs.as_entire_binding()},
```

**Replace with:**

```rust
                wgpu::BindGroupEntry{binding: 2, resource: recs.as_entire_binding()},
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                wgpu::BindGroupEntry{binding: 2, resource: recs.as_entire_binding()},
            ],
        })
    }

```

**Add below it:**

```rust
    /// Splat group 1 for one lane: its point buffers and the shared per-pixel depth/colour pair.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                wgpu::BindGroupEntry{binding: 2, resource: nrm.as_entire_binding()},
            ],
        })
    }

```

**Add below it:**

```rust
    /// The resolve pass's view of the point pass's two targets.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    fn rebuild_splat_groups(&mut self){
        self.splat_group0 = Self::mk_splat_group0(&self.device, &self.splat_group0_layout, &self.mvp_buffer, &self.cloud_buffer, &self.instance_buffer, &self.splat_recs);
        self.splat_group1 = Self::mk_splat_group1(&self.device, &self.splat_group1_layout, &self.point_buffer, &self.point_col_buffer, &self.point_nrm_buffer);
        self.splat_group0_stream = Self::mk_splat_group0(&self.device, &self.splat_group0_layout, &self.mvp_buffer, &self.cloud_buffer, &self.instance_buffer, &self.splat_stream_recs);
        self.splat_group1_stream = Self::mk_splat_group1(&self.device, &self.splat_group1_layout, &self.stream_pos_buf, &self.stream_col_buf, &self.stream_nrm_buf);
        self.splat_resolve_group = Self::mk_splat_resolve_group(&self.device, &self.splat_resolve_layout, &self.splat_depth_view, &self.splat_color_view);
```

**Replace with:**

```rust
    /// Re-point the five splat bind groups at the current buffers and targets (set_scene, resize, stream growth).
    fn rebuild_splat_groups(&mut self){
        self.splat_group0 = Self::mk_splat_group0(&self.device, &self.layouts.splat_group0, &self.mvp_buffer, &self.cloud_buffer, &self.splat_recs);
        self.splat_group1 = Self::mk_splat_group1(&self.device, &self.layouts.splat_group1, &self.point_buffer, &self.point_col_buffer, &self.point_nrm_buffer);
        self.splat_group0_stream = Self::mk_splat_group0(&self.device, &self.layouts.splat_group0, &self.mvp_buffer, &self.cloud_buffer, &self.splat_stream_recs);
        self.splat_group1_stream = Self::mk_splat_group1(&self.device, &self.layouts.splat_group1, &self.stream_pos_buf, &self.stream_col_buf, &self.stream_nrm_buf);
        self.splat_resolve_group = Self::mk_splat_resolve_group(&self.device, &self.layouts.splat_resolve, &self.splat_depth_view, &self.splat_color_view);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub fn grow_scene(&mut self, min: [f32; 3], max: [f32; 3]) {
        if !min[0].is_finite() { return }
        // an empty scene starts with a zero box; the first cloud replaces it
        if self.scene_min[0] >= self.scene_max[0] {
            self.scene_min = min;
            self.scene_max = max;
            return;
        }
        for k in 0..3 {
            self.scene_min[k] = self.scene_min[k].min(min[k]);
            self.scene_max[k] = self.scene_max[k].max(max[k]);
        }
```

**Replace with:**

```rust
    pub fn grow_scene(&mut self, world: &Aabb) {
        if !world.is_finite() { return }
        // an empty scene starts with a zero box; the first cloud replaces it
        if self.bounds.min[0] >= self.bounds.max[0] {
            self.bounds = *world;
            return;
        }
        self.bounds.union(world);
```

The two camera functions are free functions in `math.rs` now. This Remove also takes the first
lines of `write_frame_uniforms`; the next edit restores them calling the free functions.

**Remove** `src/engine/gpu/mod.rs` `    /// The camera position, recovered from the combined view-projection alone.` **through** `        self.last_eye = Self::eye_from_view_proj(view_proj);`

**Find** in `src/engine/gpu/mod.rs`:

```rust
        t0.elapsed().as_secs_f64()
    }

```

**Add below it:**

```rust
    /// Per-frame uniforms: camera, the line/pen block, and the cloud block.
    fn write_frame_uniforms(&mut self, view_proj: &Xform) {
        self.mvp_f32 = view_proj.to_f32();
        self.last_ortho_h = ortho_half_height(view_proj);
        self.last_eye = eye_from_view_proj(view_proj);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            ortho_h: Self::ortho_half_height(view_proj),
```

**Replace with:**

```rust
            ortho_h: ortho_half_height(view_proj),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let eye = Self::eye_from_view_proj(view_proj); // anchored world units, like instances[]
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        // The eye outside the scene's box is outside every object in it.
        let in_scene = (0..3).all(|k| ew[k] >= self.scene_min[k] as f64 && ew[k] <= self.scene_max[k] as f64);
```

**Replace with:**

```rust
        let eye = eye_from_view_proj(view_proj); // anchored world units, like instances[]
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        // The eye outside the scene's box is outside every object in it.
        let in_scene = (0..3).all(|k| ew[k] >= self.bounds.min[k] as f64 && ew[k] <= self.bounds.max[k] as f64);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.set_pipeline(&self.splat_point_pipeline);
```

**Replace with:**

```rust
                pass.set_pipeline(&self.pipelines.splat_points);
```

`triangle.wgsl` no longer declares group 1, but the slot stays bound so every draw keeps the
0/1/2/3 scheme.

**Find** in `src/engine/gpu/mod.rs`:

```rust
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
```

**Replace with:**

```rust
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.set_bind_group(1, &self.time_bind_group, &[]);
```

**Replace with:**

```rust
                pass.set_bind_group(1, &self.line_bind_group, &[]);
```

## Step 9 — `src/app/scene.rs`

`xform_point` and `grow_bounds` now come from `math.rs`; the two loose bounds become one `Aabb`.

**Find** in `src/app/scene.rs`:

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint, Mat4, mat_mul};
```

**Replace with:**

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint};
use crate::math::{mat_mul, xform_point, grow_bounds, Aabb};
```

**Find** in `src/app/scene.rs`:

```rust
    pub fn grow_bounds(&mut self, min: [f32; 3], max: [f32; 3]) {
        for k in 0..3 {
            self.tables.min[k] = self.tables.min[k].min(min[k]);
            self.tables.max[k] = self.tables.max[k].max(max[k]);
        }
```

**Replace with:**

```rust
    pub fn grow_bounds(&mut self, world: &Aabb) {
        self.tables.bounds.union(world);
```

**Find** in `src/app/scene.rs`:

```rust
        for k in 0..3{
            t.min[k] = t.min[k].min(fmin[k]);
            t.max[k] = t.max[k].max(fmax[k]);
        }
```

**Replace with:**

```rust
        t.bounds.union(&Aabb { min: fmin, max: fmax });
```

**Remove** `src/app/scene.rs` `pub fn xform_point(m: &Mat4, p: [f32; 3]) -> [f32; 3] {` **up to** `/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes`

## Step 10 — `src/lib.rs`

`scene_min/scene_max` became `gpu.bounds`; the streamed cloud's box grows through `Aabb`.

**Find** in `src/lib.rs`:

```rust
mod camera;
```

**Add below it:**

```rust
pub mod math;
```

**Find** in `src/lib.rs`:

```rust
                    state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);
```

**Replace with:**

```rust
                    state.camera.grow_extent(state.gpu.bounds.min, state.gpu.bounds.max);
```

**Find** in `src/lib.rs`:

```rust
                    let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                    state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
```

**Replace with:**

```rust
                    let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                    state.camera.fit(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
```

**Find** in `src/lib.rs`:

```rust
                    let (mut wlo, mut whi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
```

**Replace with:**

```rust
                    let mut world = crate::math::Aabb::empty();
```

**Find** in `src/lib.rs`:

```rust
                        let w = crate::app::scene::xform_point(&slot.place.m, corner);
                        for k in 0..3 { wlo[k] = wlo[k].min(w[k]); whi[k] = whi[k].max(w[k]); }
                    }
                    state.gpu.grow_scene(wlo, whi);
                    state.scene.grow_bounds(wlo, whi);
```

**Replace with:**

```rust
                        world.grow(crate::math::xform_point(&slot.place.m, corner));
                    }
                    state.gpu.grow_scene(&world);
                    state.scene.grow_bounds(&world);
```

**Find** in `src/lib.rs`:

```rust
                let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
```

**Replace with:**

```rust
                let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                state.camera.fit(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
```

The live loader's re-frame. **Find** in `src/lib.rs`:

```rust
                    let aspect = state.gpu.config.width.max(1) as f64 / state.gpu.config.height.max(1) as f64;
                    state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
```

**Replace with:**

```rust
                    let aspect = state.gpu.config.width.max(1) as f64 / state.gpu.config.height.max(1) as f64;
                    state.camera.fit(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
```

**Find** in `src/lib.rs`:

```rust
                            state.camera.toggle_projection_framed(state.gpu.scene_min, state.gpu.scene_max, aspect);
```

**Replace with:**

```rust
                            state.camera.toggle_projection_framed(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
```

**Find** in `src/lib.rs`:

```rust
                            state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
```

**Replace with:**

```rust
                            state.camera.fit(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
```

## Step 11 — `src/selftest.rs`

Same rename, and the eye solve is a free function now.

**Find** in `src/selftest.rs`:

```rust
    camera.fit(gpu.scene_min, gpu.scene_max, w as f64 / h as f64);
```

**Replace with:**

```rust
    camera.fit(gpu.bounds.min, gpu.bounds.max, w as f64 / h as f64);
```

**Find** in `src/selftest.rs`:

```rust
        let solved = Gpu::eye_from_view_proj(&view_proj);
```

**Replace with:**

```rust
        let solved = crate::math::eye_from_view_proj(&view_proj);
```

**Find** in `src/selftest.rs`:

```rust
        camera.fit(gpu.scene_min, gpu.scene_max, aspect);
```

**Replace with:**

```rust
        camera.fit(gpu.bounds.min, gpu.bounds.max, aspect);
```

**Find** in `src/selftest.rs`:

```rust
    camera.fit(gpu.scene_min, gpu.scene_max, aspect);
```

**Replace with:**

```rust
    camera.fit(gpu.bounds.min, gpu.bounds.max, aspect);
```

## Step 12 — `examples/check_determinism.rs`

One comparison follows the `Aabb` rename.

**Find** in `examples/check_determinism.rs`:

```rust
        if a.tables.min != b.tables.min || a.tables.max != b.tables.max { fails.push("tables.bounds".into()) }
```

**Replace with:**

```rust
        if a.tables.bounds != b.tables.bounds { fails.push("tables.bounds".into()) }
```

## Check

```bash
cargo check --lib --target wasm32-unknown-unknown            # 0 warnings
cargo check --all-targets --target x86_64-unknown-linux-gnu  # 0 warnings
grep -rc 'create_render_pipeline' src | grep -v ':0'         # build.rs:1 - the only one
grep -c 'PipelineDesc {' src/engine/pipelines/mod.rs         # 14
./docs/_gate.sh                                              # gate OK
```

`Gpu` has 102 fields (was 116) — count them with
`awk '/^pub struct Gpu \{/,/^\}/' src/engine/gpu/mod.rs | grep -cE '^ +(pub )?[a-z_0-9]+:'` —
and every ink/draw/object count in `docs/_GOLDENS.tsv` is unchanged.

## Recap

- A pipeline is a `PipelineDesc` literal; `build` is the one function. Adding a wireframe pipeline
  is ten lines in `pipelines/mod.rs`, nothing in `build.rs`.
- Layouts are the shape of a bind group and live once, in `layouts.rs`; buffers stay in `gpu/`.
- Anything declared and never read (`time`, `edges`, `instances_unused`) is a cost with no pixel.

## Next

Lesson [45](46-gpu-floor.md) — the GPU floor: `GpuCtx`, `GrowBuf`, `FrameUniforms`, `Targets`,
`Upload`.
