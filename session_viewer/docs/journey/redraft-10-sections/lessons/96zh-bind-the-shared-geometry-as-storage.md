# 20.1 · Shared surface geometry and finite visibility addresses

**Goal:** Use one physical primitive address for mesh and section ink visibility. · **Steps:** 21 · **Source:** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Contiguous GPU Arena: [Slice 0 (v0..vn)] [Slice 1] [Slice 2]</text>
  <rect x="48" y="68" width="244" height="56" fill="#f8fafc" stroke="#111111" stroke-width="2"/>
  <rect x="52" y="72" width="84" height="48" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="94" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 0</text>
  <rect x="140" y="72" width="72" height="48" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="176" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 1</text>
  <rect x="216" y="72" width="72" height="48" fill="#fef08a" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="252" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 2</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Vertex Pulling: idx = indices[base + vid]; v = arena[idx]</text>
  <rect x="424" y="54" width="272" height="40" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">var&lt;storage, read&gt; vertices: array&lt;GpuVertex&gt;</text>
  <rect x="454" y="110" width="212" height="36" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="132" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Dirty Sub-Range -> write_buffer()</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Single-Bind-Group Multi-Slice Scene Draw</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="828,138 864,72 900,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="904,138 940,72 976,138" fill="#bbf7d0" stroke="#111111" stroke-width="1.75"/>
  <polygon points="980,138 1016,72 1052,138" fill="#fef08a" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-96zh-bind-the-shared-geometry-as-storage"></a>1. Bind the shared geometry as storage

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Camera / View Uniform (16-Byte Aligned)</text>
  <rect x="52" y="48" width="268" height="106" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="66" y="58" width="240" height="38" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="81" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">view_proj: [[f32; 4]; 4] (64B)</text>
  <rect x="66" y="104" width="240" height="36" fill="#fef08a" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">eye_pos / viewport_px (16B)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.write_buffer -&gt; wgpu::BindGroup (@group(0))</text>
  <rect x="424" y="50" width="272" height="102" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="442" y="64" width="236" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="84" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">BufferUsages::UNIFORM | COPY_DST</text>
  <rect x="442" y="106" width="236" height="32" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">pass.set_bind_group(0, &amp;bg, &amp;[])</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Shader Uniform Read: var&lt;uniform&gt;</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">@group(0) @binding(0)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">var&lt;uniform&gt; u: Uniforms;</text>
  <rect x="828" y="114" width="212" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">u.view_proj * vec4(p, 1.0)</text>
</svg>
</div>

Reuse the existing geometry buffers for shader vertex reads.

### 1. `src/gpu_geometry.rs`

Keep the vertex-pulling bind group beside the shared buffers. Share explicit storage bindings and one buffer allocation helper; empty storage arrays retain a four-byte safe binding. Prepare one nonindexed invocation per index occurrence without changing existing drawing.

**Locate this existing block:**

```rust
    pub(crate) index_count: u32,
}

impl GpuGeometry {
    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        let bytes: Vec<u8> = source
            .vertices()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
        });
        let bytes: Vec<u8> = source
            .normals()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let normals = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh normals"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::STORAGE,
        });
        let index_count = source.indices().len() as u32;
        Self {
            source,
            vertices,
            normals,
            indices,
            index_count,
        }
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.size() + self.normals.size() + self.indices.size()
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub(crate) index_count: u32,
    pull_group: wgpu::BindGroup,
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 1, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: std::num::NonZeroU64::new(4),
            },
            count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"),
            entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(
            device: &wgpu::Device,
            label: &str,
            mut bytes: Vec<u8>,
            usage: wgpu::BufferUsages,
        ) -> wgpu::Buffer {
            if bytes.is_empty() {
                bytes.resize(4, 0);
            }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: &bytes,
                usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let bytes = source
            .vertices()
            .iter()
            .flatten()
            .flat_map(|v| v.to_ne_bytes())
            .collect();
        let vertices = buffer(
            device,
            "shared mesh vertices",
            bytes,
            wgpu::BufferUsages::VERTEX,
        );
        let bytes = source
            .normals()
            .iter()
            .flatten()
            .flat_map(|v| v.to_ne_bytes())
            .collect();
        let normals = buffer(
            device,
            "shared mesh normals",
            bytes,
            wgpu::BufferUsages::VERTEX,
        );
        let bytes = source
            .indices()
            .iter()
            .flat_map(|v| v.to_ne_bytes())
            .collect();
        let indices = buffer(
            device,
            "shared mesh indices",
            bytes,
            wgpu::BufferUsages::INDEX,
        );
        let buffers = [&vertices, &normals, &indices];
        let entries = [0, 1, 2].map(|binding| wgpu::BindGroupEntry {
            binding,
            resource: buffers[binding as usize].as_entire_binding(),
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"),
            layout: &Self::storage_layout(device),
            entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self {
            source,
            vertices,
            normals,
            indices,
            index_count,
            pull_group,
        }
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.size() + self.normals.size() + self.indices.size()
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_bind_group(3, &self.pull_group, &[]);
        pass.draw(0..self.index_count, 0..count);
    }
```

## <a id="checkpoint-96zi-read-source-vertices-through-one-shader-helper"></a>2. Read source vertices through one shader helper

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU RAM: #[repr(C)] Structs -&gt; bytemuck::cast_slice</text>
  <rect x="46" y="54" width="280" height="44" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="52" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="84" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v0: 24B</text>
  <rect x="122" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="154" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v1: 24B</text>
  <rect x="192" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="224" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v2: 24B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="290" y="80" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">...</text>
  <rect x="66" y="112" width="240" height="34" rx="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="133" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">&amp;[u8] Contiguous Byte Slice</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU -&gt; GPU DMA: queue.write_buffer(buf, 0, bytes)</text>
  <rect x="424" y="54" width="272" height="92" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <line x1="448" y1="84" x2="672" y2="84" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="74" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">create_buffer_init / write_buffer</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="112" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">VERTEX | INDEX | COPY_DST</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="132" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">array_stride = size_of::&lt;Vertex&gt;()</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU VRAM Buffer -&gt; @vertex Input Locations</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="56" width="224" height="40" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@location(0) pos: vec3&lt;f32&gt;</text>
  <rect x="822" y="106" width="224" height="40" rx="4" fill="#fce7f3" stroke="#db2777" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@location(1) color: vec3&lt;f32&gt;</text>
</svg>
</div>

Define the same vertex reader for ordinary and instanced drawing.

### 1. `src/triangle.wgsl`

Give ordinary and instanced vertex functions one object access boundary. Pass one object record explicitly through the existing geometry transform. Type the shared index-to-vertex storage reader; its resources remain inactive until the vertex entry calls it. Use one object lookup for ordinary and instanced vertex invocations.

**Locate this existing block:**

```wgsl
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput { return geometry(position, colour, normal); }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index: u32) -> ObjectSettings { return object_settings; }

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<f32>;
@group(3) @binding(1) var<storage, read> mesh_normals: array<f32>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence: u32) -> MeshInput {
    let index = mesh_indices[occurrence];
    let v = index * 6u;
    let n = index * 3u;
    return MeshInput(
        vec3<f32>(mesh_values[v], mesh_values[v+1u], mesh_values[v+2u]),
        vec3<f32>(mesh_values[v+3u], mesh_values[v+4u], mesh_values[v+5u]),
        vec3<f32>(mesh_normals[n], mesh_normals[n+1u], mesh_normals[n+2u]),
    );
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>, @builtin(instance_index) instance: u32) -> VertexOutput { return geometry(position, colour, normal, object_at(instance)); }
```

### 2. `src/instance_gpu.rs`

Customize only the object storage declaration and lookup; retain one geometry function and one vertex entry.

**Locate this existing block:**

```rust
            });
        let source=include_str!("triangle.wgsl").replace("var<uniform> object: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;").replace("fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>)","fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings)").replace("@location(2) normal: vec3<f32>) -> VertexOutput { return geometry(position, colour, normal); }","@location(2) normal: vec3<f32>, @builtin(instance_index) instance: u32) -> VertexOutput { return geometry(position, colour, normal, objects[instance]); }");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            });
        let source = include_str!("triangle.wgsl")
            .replace(
                "var<uniform> object_settings: ObjectSettings;",
                "var<storage, read> objects: array<ObjectSettings>;",
            )
            .replace(
                "return object_settings;",
                "return objects[index];",
            );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
```

### 3. `src/triangle_pick.wgsl`

Keep indexed object and source-face picking on the shared explicit object record without activating their storage vertex path.

**Locate this existing block:**

```wgsl
struct MeshPickOutput{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) @interpolate(flat) flags:u32}
@vertex fn pick_vertex(@location(0) position:vec3<f32>,@location(1) colour:vec3<f32>,@location(2) normal:vec3<f32>)->MeshPickOutput{let placed=geometry(position,colour,normal);return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags);}
@fragment fn pick_fragment(input:MeshPickOutput)->@location(0) vec2<u32>{if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}return vec2<u32>(select(0u,object.selection.y,crop.mode==0u),0u);}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct MeshPickOutput{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) @interpolate(flat) flags:u32}
@vertex fn pick_vertex(@location(0) position:vec3<f32>,@location(1) colour:vec3<f32>,@location(2) normal:vec3<f32>)->MeshPickOutput{let placed=geometry(position,colour,normal,object_settings);return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags);}
@fragment fn pick_fragment(input:MeshPickOutput)->@location(0) vec2<u32>{if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}return vec2<u32>(select(0u,object_settings.selection.y,crop.mode==0u),0u);}
```

### 4. `src/face_pick.wgsl`

Keep indexed object and source-face picking on the shared explicit object record without activating their storage vertex path.

**Locate this existing block:**

```wgsl
    let vertex = positions[indices[index]];
    let placed = geometry(vec3<f32>(vertex.x, vertex.y, vertex.z), vec3<f32>(vertex.r, vertex.g, vertex.b), vec3<f32>(0.0));
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object.selection.y, (0x20000000u | input.face) + 1u);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let vertex = positions[indices[index]];
    let placed = geometry(vec3<f32>(vertex.x, vertex.y, vertex.z), vec3<f32>(vertex.r, vertex.g, vertex.b), vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object_settings.selection.y, (0x20000000u | input.face) + 1u);
}
```

## <a id="checkpoint-96zj-activate-shared-vertex-pulling-for-color"></a>3. Activate shared vertex pulling for color

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU CommandEncoder Records GPU Commands</text>
  <rect x="54" y="50" width="264" height="102" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">wgpu::CommandEncoder</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#2563eb">1. begin_render_pass (3D Scene)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#16a34a">2. begin_render_pass (Overlay/UI)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">3. encoder.finish()</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Pass 1 (LoadOp::Clear) -&gt; Pass 2 (LoadOp::Load)</text>
  <rect x="416" y="52" width="128" height="96" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="480" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 1: Clear</text>
  <polygon points="452,132 480,88 508,132" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="52" width="128" height="96" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 2: Load</text>
  <polygon points="612,126 640,86 668,126" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <rect x="590" y="130" width="100" height="12" fill="#fef08a" stroke="#111111" stroke-width="1"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.submit([cmd_buf]) -&gt; GPU Execution</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="884,132 934,58 984,132" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="136" width="224" height="18" rx="3" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="149" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Composited Frame Ready</text>
</svg>
</div>

Rasterize ordinary and instanced triangles from their shared storage buffers.

### 1. `src/triangle.wgsl`

Activate the storage reader with the nonindexed occurrence number.

**Locate this existing block:**

```wgsl
@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>, @builtin(instance_index) instance: u32) -> VertexOutput { return geometry(position, colour, normal, object_at(instance)); }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let input = mesh_vertex(occurrence);
    return geometry(input.position, input.colour, input.normal, object_at(instance));
}
```

### 2. `src/gpu_mesh.rs`

Keep ordinary pulled color draws beside the unchanged indexed picking and mask draw.

**Locate this existing block:**

```rust
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.bind_model(pass);
        self.geometry.draw_pulled(pass, 1);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
```

### 3. `src/renderer.rs`

Share the explicit storage layout with ordinary color pipelines. The color vertex entry now reads storage instead of vertex attributes. Draw ordinary color triangles through the same shared buffers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
                mesh.draw(&mut pass);
            }
```

Replace it with:

```rust
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
                mesh.draw_pulled(&mut pass);
            }
```

**Locate this block:**

```rust
                Some(&crate::clipping_gpu::layout(&device)),
            ],
            immediate_size: 0,
        });
        let mesh_pipeline = |depth| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }, wgpu::VertexBufferLayout {
                    array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        })
        };
        let pipeline = mesh_pipeline(crate::depth::surface());
        let tiles = std::cell::RefCell::new(crate::tile_gpu::Tiles::new(
```

Replace it with:

```rust
                Some(&crate::clipping_gpu::layout(&device)),
                Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device)),
            ],
            immediate_size: 0,
        });
        let mesh_pipeline = |depth| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("triangle"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: Default::default(),
                depth_stencil: Some(depth),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = mesh_pipeline(crate::depth::surface());
        let tiles = std::cell::RefCell::new(crate::tile_gpu::Tiles::new(
```

### 4. `src/highlight.rs`

Selection mesh overlays use the same pulled color entry.

**Locate this existing block:**

```rust
        if let Some(mesh) = &self.mesh {
            mesh.draw(pass);
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if let Some(mesh) = &self.mesh {
            mesh.draw_pulled(pass);
        }
```

### 5. `src/instance_gpu.rs`

Bind the same geometry storage layout for instanced color drawing. Instance vertices use the same storage reader and retain their own object records. Advance occurrences per shared triangle and object records per instance.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {
            pass.set_bind_group(1, &batch.group, &[]);
            batch
                .geometry
                .draw_instances(pass, batch.count);
        }
```

Replace it with:

```rust
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {
            pass.set_bind_group(1, &batch.group, &[]);
            batch.geometry.draw_pulled(pass, batch.count);
        }
```

**Locate this block:**

```rust
                    Some(&crate::clipping_gpu::layout(device)),
                ],
                immediate_size: 0,
            });
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:24,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3]},wgpu::VertexBufferLayout {array_stride:12,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![2=>Float32x3]}]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into())]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {
            pipeline,
            layout,
```

Replace it with:

```rust
                    Some(&crate::clipping_gpu::layout(device)),
                    Some(&GpuGeometry::storage_layout(device)),
                ],
                immediate_size: 0,
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shared original definition drawing"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            layout,
```

### 6. `src/pick_gpu_tests.rs`

Reuse the supplied pixel readback for the normal diagnostic instead of duplicating GPU setup. Read either color or actual world-normal pixels through one supplied helper. Supply real normal-color pixels, permuted sparse source vertices, reflected shared placements, both pixel densities and original integer owners.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        ));
    }
}
```

Replace it with:

```rust
        ));
    }
}

#[test]
fn actual_pulled_source_normals_keep_shared_reflected_placements_and_original_picks() {
    let mut source = session_rust::Mesh::new();
    for (key, p) in [
        (10, [-0.2, -0.4, 0.3]),
        (20, [0.2, -0.4, 0.3]),
        (30, [0., 0.4, 0.3]),
    ] {
        source.add_vertex(
            session_rust::Point::new(p[0], p[1], p[2]),
            Some(key),
        );
        for (name, value) in [("nx", 4.), ("ny", 0.), ("nz", 3.)] {
            source.set_vertex_attribute(key, name, value);
        }
    }
    source.add_face(vec![30, 10, 20], Some(37));
    let mut scene = Scene::empty();
    let a = scene
        .insert(PreparedMesh::new(source).unwrap())
        .unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let b = scene.copy_row(a, false).unwrap();
    scene
        .place(
            a,
            session_rust::Xform::translation(-0.5, 0., 0.),
        )
        .unwrap();
    let mut reflected = session_rust::Xform::identity();
    reflected.m[0] = -1.;
    reflected.m[12] = 0.5;
    scene.place(b, reflected).unwrap();
    assert!(Rc::ptr_eq(
        scene.objects()[0].geometry().unwrap(),
        scene.objects()[1].geometry().unwrap()
    ));
    let mut gpu = renderer(&scene);
    let usage = gpu.stats();
    for density in [1., 2.] {
        gpu.set_density(density);
        let pixels = colour_pixels_display(&mut gpu, identity(), true);
        for (x, expected, owner, position) in [
            (80, [230u8, 128, 204], a, -0.5),
            (240, [230u8, 128, 51], b, 0.5),
        ] {
            let pixel = &pixels[(132 * 320 + x) * 4..][..3];
            for (actual, expected) in pixel.iter().zip(expected) {
                assert!(
                    actual.abs_diff(expected) <= 2,
                    "Actual placed source normal: {pixel:?}"
                );
            }
            gpu.request_pick([position, -0.1], identity())
                .unwrap();
            assert_eq!(finish(&mut gpu).unwrap()[0], owner.value());
        }
        assert_eq!(&gpu.stats()[..2], &usage[..2]);
    }
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    assert_eq!(
        scene.objects()[1]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
}
```

**Locate this block:**

```rust
        &texture.create_view(&Default::default()),
        &Default::default(),
        &transform,
        false,
    );
    let bytes = renderer
        .device
```

Replace it with:

```rust
        &texture.create_view(&Default::default()),
        &Default::default(),
        &transform,
        normals,
    );
    let bytes = renderer
        .device
```

**Locate this block:**

```rust
fn colour_pixels_at(renderer: &mut Renderer, transform: [f32; 16]) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
```

Replace it with:

```rust
fn colour_pixels_at(renderer: &mut Renderer, transform: [f32; 16]) -> Vec<u8> {
    colour_pixels_display(renderer, transform, false)
}
fn colour_pixels_display(
    renderer: &mut Renderer,
    transform: [f32; 16],
    normals: bool,
) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
```

## <a id="checkpoint-96zk-assign-frame-triangle-ranges"></a>4. Assign frame triangle ranges

Give every placed color triangle the address of its projected record.

### 1. `src/gpu_mesh.rs`

Retain the row’s projected interval across object-style updates. Pack a checked primitive base alongside the existing selection bit. Reserve the low bit for selection. New rows start at zero until the frame assigns intervals. Initialize the retained interval. Preserve the base when placement or presentation changes. Read only selection, not the packed range. Validate and update just the packed base while keeping the selection bit.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    0
                },
            self.cap_token,
        );
```

Replace it with:

```rust
                    0
                },
            self.cap_token,
            self.primitive_base,
        );
```

**Locate this block:**

```rust
        self.previous
    }
    pub fn selected(&self) -> bool {
        u32::from_ne_bytes(self.previous[64..68].try_into().unwrap()) != 0
    }

    pub fn set_xray(&mut self, queue: &wgpu::Queue, on: bool) {
```

Replace it with:

```rust
        self.previous
    }
    pub fn selected(&self) -> bool {
        u32::from_ne_bytes(self.previous[64..68].try_into().unwrap()) & 1 != 0
    }

    pub fn set_primitive_base(
        &mut self,
        queue: &wgpu::Queue,
        base: u32,
    ) -> Result<(), &'static str> {
        if base > 0x7fff_ffff {
            return Err("Primitive base exceeds its packed range");
        }
        if self.primitive_base != base {
            let word = (base << 1) | u32::from(self.selected());
            self.previous[64..68].copy_from_slice(&word.to_ne_bytes());
            queue.write_buffer(&self.model, 64, &self.previous[64..68]);
            self.primitive_base = base;
        }
        Ok(())
    }

    pub fn set_xray(&mut self, queue: &wgpu::Queue, on: bool) {
```

**Locate this block:**

```rust
            id: object.id,
            model,
            previous: bytes,
            xray: false,
            cap_flags: 0,
            cap_token: 0,
```

Replace it with:

```rust
            id: object.id,
            model,
            previous: bytes,
            primitive_base: 0,
            xray: false,
            cap_flags: 0,
            cap_token: 0,
```

**Locate this block:**

```rust
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, style, 0, 0);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

Replace it with:

```rust
        style: Style,
        geometry: Rc<GpuGeometry>,
    ) -> Self {
        let bytes = settings(object, style, 0, 0, 0);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
```

**Locate this block:**

```rust
    style: Style,
    cap_flags: u32,
    cap_token: u32,
) -> [u8; 160] {
    let mut bytes = [0; 160];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id = if style.pickable {
        object.id.value()
    } else {
```

Replace it with:

```rust
    style: Style,
    cap_flags: u32,
    cap_token: u32,
    primitive_base: u32,
) -> [u8; 160] {
    let mut bytes = [0; 160];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(
        &((primitive_base << 1) | u32::from(style.selected)).to_ne_bytes(),
    );
    let id = if style.pickable {
        object.id.value()
    } else {
```

**Locate this block:**

```rust
    previous: [u8; 160],
    xray: bool,
    cap_flags: u32,
    cap_token: u32,
```

Replace it with:

```rust
    previous: [u8; 160],
    primitive_base: u32,
    xray: bool,
    cap_flags: u32,
    cap_token: u32,
```

### 2. `src/triangle.wgsl`

A triangle address must never turn an unselected original yellow.

**Locate this existing block:**

```wgsl
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
```

### 3. `src/cap_count.wgsl`

Section counting reads the same selection bit.

**Locate this existing block:**

```wgsl
 let world=object.model*vec4<f32>(position,1.0);let mirrored=dot(object.model[0].xyz,cross(object.model[1].xyz,object.model[2].xyz))<0.0;
 out.pos=view.transform*world;out.world_pos=world.xyz;out.inst_id=object.selection.w;out.plane=plane;out.flip=select(0u,1u,mirrored!=((object.selection.z&FLAG_INWARD)!=0u));out.selected=object.selection.x;return out;
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 let world=object.model*vec4<f32>(position,1.0);let mirrored=dot(object.model[0].xyz,cross(object.model[1].xyz,object.model[2].xyz))<0.0;
 out.pos=view.transform*world;out.world_pos=world.xyz;out.inst_id=object.selection.w;out.plane=plane;out.flip=select(0u,1u,mirrored!=((object.selection.z&FLAG_INWARD)!=0u));out.selected=object.selection.x&1u;return out;
}
```

### 4. `src/renderer.rs`

Assign the same row-order prefix sums used by projection before copying instance records.

**Locate this existing block:**

```rust
        }
        self.tiles
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        let mut base = 0u32;
        for mesh in &mut self.meshes {
            mesh.set_primitive_base(&self.queue, base)
                .expect("Bounded projected triangle base");
            base = base
                .checked_add(mesh.geometry.index_count / 3)
                .filter(|n| *n <= 0x7fff_ffff)
                .expect("Bounded projected triangle range");
        }
        self.tiles
```

### 5. `src/triangle.wgsl`

Carry an integer triangle address without interpolation. The nonindexed occurrence recovers its triangle, and the placed record supplies its frame interval.

**Locate this existing block:**

```wgsl
    @location(4) @interpolate(flat) mirrored:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<f32>;
@group(3) @binding(1) var<storage, read> mesh_normals: array<f32>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence: u32) -> MeshInput {
    let index = mesh_indices[occurrence];
    let v = index * 6u;
    let n = index * 3u;
    return MeshInput(
        vec3<f32>(mesh_values[v], mesh_values[v+1u], mesh_values[v+2u]),
        vec3<f32>(mesh_values[v+3u], mesh_values[v+4u], mesh_values[v+5u]),
        vec3<f32>(mesh_normals[n], mesh_normals[n+1u], mesh_normals[n+2u]),
    );
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let input = mesh_vertex(occurrence);
    return geometry(input.position, input.colour, input.normal, object_at(instance));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(4) @interpolate(flat) mirrored:u32,
    @location(5) @interpolate(flat) primitive:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<f32>;
@group(3) @binding(1) var<storage, read> mesh_normals: array<f32>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence: u32) -> MeshInput {
    let index = mesh_indices[occurrence];
    let v = index * 6u;
    let n = index * 3u;
    return MeshInput(
        vec3<f32>(mesh_values[v], mesh_values[v+1u], mesh_values[v+2u]),
        vec3<f32>(mesh_values[v+3u], mesh_values[v+4u], mesh_values[v+5u]),
        vec3<f32>(mesh_normals[n], mesh_normals[n+1u], mesh_normals[n+2u]),
    );
}

@vertex
fn vertex(@builtin(vertex_index) occurrence: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let input = mesh_vertex(occurrence);
    let object=object_at(instance);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
    return output;
}
```

### 6. `src/global_outline_tests.rs`

Supply actual shared placements, frame intervals, low-bit selection, later edits, original integer picks, invalid packing and visibility rebuilding.

**Locate this existing block:**

```rust
    assert!(r.global_outline_alpha().is_empty());
    assert_eq!(r.global_outline_usage(), [0; 4]);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(r.global_outline_alpha().is_empty());
    assert_eq!(r.global_outline_usage(), [0; 4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners() {
    let mut scene = Scene::empty();
    let a = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.2, -0.4, -0.4],
                    [0.2, -0.4, -0.4],
                    [0., 0.4, -0.4],
                ],
                [0.8, 0.2, 0.1],
            )
            .unwrap(),
        )
        .unwrap();
    let b = scene.copy_row(a, false).unwrap();
    scene.translate(a, [-0.5, 0., 0.]).unwrap();
    scene.translate(b, [0.5, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    let bases = |gpu: &super::Renderer| {
        gpu.meshes
            .iter()
            .map(|row| {
                u32::from_ne_bytes(row.packed()[64..68].try_into().unwrap()) >> 1
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(bases(&gpu), vec![0, 1]);
    assert!(gpu.meshes.iter().all(|row| !row.selected()));
    let baseline = pixels(&gpu);
    gpu.select_objects(&scene, &[b]).unwrap();
    assert_eq!(bases(&gpu), vec![0, 1]);
    assert_eq!(
        gpu.meshes
            .iter()
            .map(|row| row.selected())
            .collect::<Vec<_>>(),
        vec![false, true]
    );
    let selected = pixels(&gpu);
    assert_ne!(selected, baseline);
    scene.translate(b, [0., 0.1, 0.]).unwrap();
    gpu.set_scene(&scene, Some(b));
    assert_eq!(bases(&gpu), vec![0, 1]);
    assert!(gpu.meshes[1].selected());
    for (position, owner) in [([-0.5, -0.1], a), ([0.5, 0.], b)] {
        gpu.request_pick(position, matrix()).unwrap();
        wait(&gpu);
        assert_eq!(
            gpu.take_pick(matrix())
                .unwrap()
                .unwrap()
                .unwrap()[0],
            owner.value()
        );
    }
    let before = gpu.meshes[1].packed();
    assert!(
        gpu.meshes[1]
            .set_primitive_base(&gpu.queue, 0x8000_0000)
            .is_err()
    );
    assert_eq!(gpu.meshes[1].packed(), before);
    Rc::make_mut(&mut scene.view).hide(&[a], true);
    gpu.set_scene(&scene, Some(b));
    assert_eq!(bases(&gpu), vec![0]);
    assert_eq!(gpu.meshes[0].id, b);
    assert!(gpu.meshes[0].selected());
    pixels(&gpu);
}
```

## <a id="checkpoint-96zl-own-the-integer-primitive-image"></a>5. Own the integer primitive image

Allocate the frame image that will name rasterized triangles.

### 1. `src/primitive_image.rs`

Own the current integer format and share its explicit no-blend target declaration.

Create the file and type:

```rust
pub(crate) const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rg16Uint;
pub(crate) struct Image {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}
impl Image {
    pub fn new(device: &wgpu::Device, size: [u32; 2]) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("physical triangle addresses"),
            size: wgpu::Extent3d {
                width: size[0].max(1),
                height: size[1].max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        Self { texture, view }
    }
}
pub(crate) fn target(write: bool) -> wgpu::ColorTargetState {
    wgpu::ColorTargetState {
        format: FORMAT,
        blend: None,
        write_mask: if write {
            wgpu::ColorWrites::ALL
        } else {
            wgpu::ColorWrites::empty()
        },
    }
}
```

### 2. `src/lib.rs`

Connect the private target owner.

**Locate this existing block:**

```rust
mod prepared_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod prepared_tests;
mod primitive_image;
#[cfg(test)]
```

### 3. `src/renderer.rs`

Retain the primitive image with the renderer’s frame targets. Prepare a matching image before moving the device into the renderer. Move both depth and primitive targets into their owner. Keep primitive image dimensions in physical pixels.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }
```

Replace it with:

```rust
        }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive = crate::primitive_image::Image::new(&self.device, self.size);
    }
```

**Locate this block:**

```rust
            uniform,
            view_group,
            depth,
            strokes,
            paths,
            markers,
```

Replace it with:

```rust
            uniform,
            view_group,
            depth,
            primitive,
            strokes,
            paths,
            markers,
```

**Locate this block:**

```rust
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

Replace it with:

```rust
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let primitive = crate::primitive_image::Image::new(&device, [640, 480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

**Locate this block:**

```rust
    depth: wgpu::TextureView,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
```

Replace it with:

```rust
    depth: wgpu::TextureView,
    primitive: crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
```

## <a id="checkpoint-96zm-write-triangle-addresses-beside-color"></a>6. Write triangle addresses beside color

Write color and a lossless triangle address in the same depth-tested pass.

### 1. `src/triangle.wgsl`

Keep the existing lighting and normal diagnostic in one reusable fragment helper. Write displayed color and the exact triangle address from one depth-tested fragment.

**Locate this existing block:**

```wgsl
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

struct FragmentOutput{
    @location(0) color:vec4<f32>,
    @location(1) primitive:vec2<u32>,
}
@fragment
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),vec2<u32>(input.primitive&0xffffu,input.primitive>>16u));
}
```

### 2. `src/renderer.rs`

Share both pipeline formats while allowing overlays to retain underlying addresses. Declare the matching integer output without blending. Ordinary triangle fragments own primitive addresses. Selected-face color overlays retain the underlying primitive address.

**Locate this existing block:**

```rust
        });
        let mesh_pipeline = |depth| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("triangle"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: Default::default(),
                depth_stencil: Some(depth),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = mesh_pipeline(crate::depth::surface());
        let tiles = std::cell::RefCell::new(crate::tile_gpu::Tiles::new(
            &device,
            &view_layout,
        ));
        let instances = crate::instance_gpu::Lane::new(&device, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        });
        let mesh_pipeline = |depth, primitive_write| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("triangle"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(crate::primitive_image::target(
                            primitive_write,
                        )),
                    ],
                }),
                primitive: Default::default(),
                depth_stencil: Some(depth),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = mesh_pipeline(crate::depth::surface(), true);
        let tiles = std::cell::RefCell::new(crate::tile_gpu::Tiles::new(
            &device,
            &view_layout,
        ));
        let instances = crate::instance_gpu::Lane::new(&device, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(), false);
        let highlight = crate::highlight::Highlight::new(&device, format);
```

### 3. `src/instance_gpu.rs`

Instance fragments use the same lossless second attachment.

**Locate this existing block:**

```rust
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                compilation_options: Default::default(),
                targets: &[
                    Some(format.into()),
                    Some(crate::primitive_image::target(true)),
                ],
            }),
```

### 4. `src/renderer.rs`

Clear uncovered pixels to zero and retain both outputs from the mesh pass.

**Locate this existing block:**

```rust
                ),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                ),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &self.primitive.view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                ..Default::default()
```

### 5. `src/global_outline_tests.rs`

Supply actual integer attachment readback, placed copies, original source-face overlays, shared definition instances, both densities, resize and empty clearing.

**Locate this existing block:**

```rust
    pixels(&gpu);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pixels(&gpu);
}

#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses() {
    fn addresses(gpu: &super::Renderer) -> Vec<u32> {
        let texture = &gpu.primitive.texture;
        let width = texture.width();
        let height = texture.height();
        let pitch = (width * 4 + 255) / 256 * 256;
        let read = gpu
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("actual primitive addresses"),
                size: u64::from(pitch) * u64::from(height),
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        let mut encoder = gpu
            .device
            .create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &read,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(pitch),
                    rows_per_image: Some(height),
                },
            },
            texture.size(),
        );
        gpu.queue.submit([encoder.finish()]);
        let (tx, rx) = std::sync::mpsc::channel();
        read.slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap()
            });
        wait(gpu);
        rx.recv().unwrap().unwrap();
        let bytes = read.slice(..).get_mapped_range();
        let values = bytes
            .chunks_exact(pitch as usize)
            .flat_map(|row| {
                row[..width as usize * 4]
                    .chunks_exact(4)
                    .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
            })
            .collect();
        drop(bytes);
        read.unmap();
        values
    }
    let mut scene = Scene::empty();
    let a = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.2, -0.4, -0.4],
                    [0.2, -0.4, -0.4],
                    [0., 0.4, -0.4],
                ],
                [0.8, 0.2, 0.1],
            )
            .unwrap(),
        )
        .unwrap();
    let b = scene.copy_row(a, false).unwrap();
    scene.translate(a, [-0.5, 0., 0.]).unwrap();
    scene.translate(b, [0.5, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    for density in [1., 2.] {
        gpu.set_density(density);
        pixels(&gpu);
        let before = addresses(&gpu);
        assert_eq!(before[0], 0);
        assert_eq!(before[120 * 320 + 80], 1);
        assert_eq!(before[120 * 320 + 240], 2);
        assert_eq!(
            gpu.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        let key = *scene.objects()[0]
            .geometry()
            .unwrap()
            .face
            .keys()
            .next()
            .unwrap();
        gpu.set_highlight(
            &scene,
            Some(crate::pick_sources::Target {
                owner: a,
                part: crate::pick_sources::Part::Face(key),
            }),
        )
        .unwrap();
        pixels(&gpu);
        assert_eq!(
            addresses(&gpu),
            before,
            "A derived selected-face mesh must retain original triangle addresses"
        );
        gpu.set_highlight(&scene, None).unwrap();
    }
    let mut instances = Scene::empty();
    crate::scene_document::import(
        &mut instances,
        &crate::instance_specimen::display_bytes(),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    gpu.set_scene(&instances, None);
    pixels(&gpu);
    let placed = addresses(&gpu);
    assert_eq!(placed[120 * 320 + 64], 1);
    assert_eq!(placed[120 * 320 + 256], 2);
    gpu.resize(crate::viewport::Viewport {
        width: 640,
        height: 480,
    });
    assert_eq!(
        [
            gpu.primitive.texture.width(),
            gpu.primitive.texture.height()
        ],
        [640, 480]
    );
    gpu.resize(crate::viewport::Viewport {
        width: 320,
        height: 240,
    });
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert!(addresses(&gpu).iter().all(|id| *id == 0));
}
```

## <a id="checkpoint-96zn-bind-visible-triangle-addresses-for-ink"></a>7. Bind visible triangle addresses for ink

Make the exact triangle image available to the existing shared ink shader.

### 1. `src/finite_ink.rs`

Retain the integer primitive view in the cached bindings. Use the existing shared frame tables for triangle addresses. The lossless primitive format requires an unsigned texture binding. Bind the current physical primitive view without a sampler. Compare every cached frame resource. A replaced primitive image invalidates the cache. Retain the same view identity used by the bind group. Draw with the complete cached frame group.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        verts: u32,
    ) -> bool {
        let bindings = self.bindings.borrow();
        let Some((_, _, _, _, group)) = bindings.as_ref() else {
            return false;
```

Replace it with:

```rust
        verts: u32,
    ) -> bool {
        let bindings = self.bindings.borrow();
        let Some((_, _, _, _, _, group)) = bindings.as_ref() else {
            return false;
```

**Locate this block:**

```rust
            tables.tiles.clone(),
            depth.clone(),
            tables.caps.clone(),
            group,
        ));
    }
```

Replace it with:

```rust
            tables.tiles.clone(),
            depth.clone(),
            tables.caps.clone(),
            tables.primitive.clone(),
            group,
        ));
    }
```

**Locate this block:**

```rust
        let mut bindings = self.bindings.borrow_mut();
        if bindings
            .as_ref()
            .is_some_and(|(p, t, d, c, _)| {
                p == &tables.projected
                    && t == &tables.tiles
                    && d == depth
                    && c == &tables.caps
            })
        {
            return;
```

Replace it with:

```rust
        let mut bindings = self.bindings.borrow_mut();
        if bindings
            .as_ref()
            .is_some_and(|(p, t, d, c, i, _)| {
                p == &tables.projected
                    && t == &tables.tiles
                    && d == depth
                    && c == &tables.caps
                    && i == &tables.primitive
            })
        {
            return;
```

**Locate this block:**

```rust
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&tables.caps),
                },
            ],
        })
    }
```

Replace it with:

```rust
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&tables.caps),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&tables.primitive),
                },
            ],
        })
    }
```

**Locate this block:**

```rust
                        },
                        false,
                    ),
                ],
            });
        let color = pipeline(
```

Replace it with:

```rust
                        },
                        false,
                    ),
                    crate::text_gpu_support::texture_entry(
                        4,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::TextureSampleType::Uint,
                        false,
                    ),
                ],
            });
        let color = pipeline(
```

**Locate this block:**

```rust
    pub projected: wgpu::Buffer,
    pub tiles: wgpu::Buffer,
    pub caps: wgpu::TextureView,
}
impl Ink {
    pub fn new(
```

Replace it with:

```rust
    pub projected: wgpu::Buffer,
    pub tiles: wgpu::Buffer,
    pub caps: wgpu::TextureView,
    pub primitive: wgpu::TextureView,
}
impl Ink {
    pub fn new(
```

**Locate this block:**

```rust
            wgpu::TextureView,
            wgpu::BindGroup,
        )>,
    >,
```

Replace it with:

```rust
            wgpu::TextureView,
            wgpu::TextureView,
            wgpu::BindGroup,
        )>,
    >,
```

### 2. `src/renderer.rs`

Pass the current primitive view through the existing color or integer-pick tables.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            projected: tiles.projected.clone(),
            tiles: tiles.buffer.clone(),
            caps: self.caps.borrow().depth.view.clone(),
        };
```

Replace it with:

```rust
            projected: tiles.projected.clone(),
            tiles: tiles.buffer.clone(),
            caps: self.caps.borrow().depth.view.clone(),
            primitive: self.primitive.view.clone(),
        };
```

**Locate this block:**

```rust
                    caps: self.caps.borrow().depth.view.clone(),
                }
            };
            self.picker.submit_finite(
```

Replace it with:

```rust
                    caps: self.caps.borrow().depth.view.clone(),
                    primitive: self.primitive.view.clone(),
                }
            };
            self.picker.submit_finite(
```

### 3. `src/pick_encode.rs`

Keep the primitive resource when the cropped picking pass replaces its section image.

**Locate this existing block:**

```rust
        caps: image.section.clone(),
    });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        caps: image.section.clone(),
        primitive: tables.primitive.clone(),
    });
```

### 4. `src/finite_visibility.wgsl`

Declare the lossless texture before activating primitive-aware plane fitting.

**Locate this existing block:**

```wgsl
@group(1) @binding(3) var finite_sections:texture_2d<f32>;
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(3) var finite_sections:texture_2d<f32>;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
```

## <a id="checkpoint-96zo-describe-ink-depth-and-surface-slack"></a>8. Describe ink depth and surface slack

Type the current axis record and depth-carrying error budget.

### 1. `src/ink_plane.wgsl`

Type the current axis and tolerance math once, with the declared forward-depth reader adapter.

Create the file and type:

```wgsl
const DEPTH_REL_TOL:f32=1.9073486e-6;
const SLOPE_PX:f32=0.00390625;
const KINK:f32=0.03125;
const SLOPE_INVALID:f32=0.99951171875;
struct InkAxis{at:vec2<f32>,depth:f32,along:vec2<f32>,slope:f32,sag:vec2<f32>}
fn ink_depth(pixel:vec2<f32>,sample:u32)->f32{
    let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));
    if(any(at<vec2<i32>(0))||any(at>=size)){return 0.;}
    return 1.-textureLoad(finite_depth,at,0);
}
fn ink_sag_slack(sag: vec2<f32>, gradient: vec2<f32>) -> f32 {
    return sag.x + sag.y * (abs(gradient.x) + abs(gradient.y));
}
fn ink_tolerance(depth: f32, slope: f32, lever: f32) -> f32 {
    return abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (1.0 + lever);
}
fn ink_carry_visible(predicted: f32, z: f32, depth: f32, tolerance: f32) -> bool {
    if (z > depth + abs(depth) * DEPTH_REL_TOL) {
        return abs(predicted - depth) <= tolerance;
    }
    return predicted <= depth + tolerance;
}
```

### 2. `src/finite_visibility.wgsl`

Keep surface fitting as a shared shader helper file.

**Locate this existing block:**

```wgsl
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
```

### 3. `src/finite_sources.rs`

Expand the helper before compiling either color or integer ink shaders.

**Locate this existing block:**

```rust
    };
    let base = crate::finite_project_source::expand(base).replace(
        "#include \"finite_visibility.wgsl\"",
        include_str!("finite_visibility.wgsl"),
    );
    let base = if grid {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    };
    let base = crate::finite_project_source::expand(base)
        .replace(
            "#include \"finite_visibility.wgsl\"",
            include_str!("finite_visibility.wgsl"),
        )
        .replace(
            "#include \"ink_plane.wgsl\"",
            include_str!("ink_plane.wgsl"),
        );
    let base = if grid {
```

## <a id="checkpoint-96zp-fit-a-neighboring-surface-along-the-stroke"></a>9. Fit a neighboring surface along the stroke

Type the current fallback that carries a neighboring plane to the stroke axis.

### 1. `src/ink_plane.wgsl`

Type the current neighbor validation, perpendicular step and axis-plane carry without duplicating the tolerance helpers.

**Locate this existing block:**

```wgsl
    return predicted <= depth + tolerance;
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return predicted <= depth + tolerance;
}
fn ink_pair_planar(pixel: vec2<f32>, dir: vec2<f32>, z: f32, sample: u32) -> bool {
    let z_side = ink_depth(pixel + dir, sample);
    if (z_side == 0.0) {
        return false;
    }
    let z_far = ink_depth(pixel + dir * 2.0, sample);
    if (z_far == 0.0) {
        return true;
    }
    let g = z_side - z;
    let g_far = z_far - z_side;
    return abs(g_far - g) <= abs(z) * DEPTH_REL_TOL + KINK * (abs(g) + abs(g_far));
}
fn ink_step(pixel: vec2<f32>, axis: InkAxis) -> vec2<f32> {
    let perp = vec2<f32>(-axis.along.y, axis.along.x);
    var step = vec2<f32>(sign(perp.x), 0.0);
    if (abs(perp.y) > abs(perp.x)) {
        step = vec2<f32>(0.0, sign(perp.y));
    }
    return select(step, -step, dot(pixel - axis.at, step) < 0.0);
}
fn ink_axis_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let step = ink_step(pixel, axis);
    var side = step;
    if (!ink_pair_planar(pixel, side, z, sample)) {
        side = -step;
        if (!ink_pair_planar(pixel, side, z, sample)) {
            return z <= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
        }
    }
    let z_side = ink_depth(pixel + side, sample);
    let e = axis.at - pixel;
    let det = axis.along.x * side.y - axis.along.y * side.x;
    let a = (e.x * side.y - e.y * side.x) / det;
    let b = (axis.along.x * e.y - axis.along.y * e.x) / det;
    let g = z_side - z;
    let predicted = z + a * axis.slope + b * g;
    let tolerance = ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b)) + ink_sag_slack(axis.sag, vec2<f32>(g, axis.slope));
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
```

## <a id="checkpoint-96zq-own-a-cropped-primitive-image-for-picks"></a>10. Own a cropped primitive image for picks

Retain a bounded triangle image beside integer owners and depth.

### 1. `src/pick_image.rs`

Own matching bounded primitive and depth targets. Reuse the primitive image owner at the actual cropped dimensions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                view_formats: &[],
            })
            .create_view(&Default::default());
        Self {
            texture,
            colour,
            depth,
            section,
```

Replace it with:

```rust
                view_formats: &[],
            })
            .create_view(&Default::default());
        let primitive =
            crate::primitive_image::Image::new(device, [size.width, size.height]);
        Self {
            texture,
            colour,
            depth,
            primitive,
            section,
```

**Locate this block:**

```rust
    pub depth: wgpu::TextureView,
    pub section: wgpu::TextureView,
    pub readback: wgpu::Buffer,
    pub window: Window,
```

Replace it with:

```rust
    pub depth: wgpu::TextureView,
    pub primitive: crate::primitive_image::Image,
    pub section: wgpu::TextureView,
    pub readback: wgpu::Buffer,
    pub window: Window,
```

### 2. `src/pick_gpu.rs`

Count owner, depth, section and primitive bytes honestly.

**Locate this existing block:**

```rust
                        * u64::from(image.window.draw[3])
                        * 20
            }) + 160
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        * u64::from(image.window.draw[3])
                        * 24
            }) + 160
```

### 3. `src/pick_gpu_tests.rs`

Include the actual four primitive bytes per cropped pixel while retaining the same bounded readback allocation assertion.

**Locate this existing block:**

```rust
    assert!(
        renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 20 + 192 + 3 * 52,
        "Readback does not grow with the full drawing"
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(
        renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 24 + 192 + 3 * 52,
        "Readback does not grow with the full drawing"
```

## <a id="checkpoint-96zr-share-crop-and-clipping-bindings-for-pulled-picks"></a>11. Share crop and clipping bindings for pulled picks

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Half-Space Clip: d_i = dot(P_i, n_clip) + w_clip</text>
  <polygon points="58,138 118,58 258,58 198,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="164" y1="44" x2="164" y2="156" stroke="#dc2626" stroke-width="2.5" stroke-dasharray="5 3"/>
  <line x1="164" y1="98" x2="218" y2="98" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="236" y="102" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">d &lt; 0</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Triangle-Plane Edge Segments -&gt; Welded Closed Cap Polygon</text>
  <polygon points="448,70 668,70 638,138 418,138" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <line x1="458" y1="70" x2="428" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="508" y1="70" x2="478" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="558" y1="70" x2="528" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="608" y1="70" x2="578" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <circle cx="448" cy="70" r="4" fill="#111111"/>
  <circle cx="668" cy="70" r="4" fill="#111111"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Live Section Cut with Capped Solid Interior</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,138 882,66 948,66 948,138" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <polygon points="948,66 994,82 994,146 948,138" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
</svg>
</div>

Prepare a combined uniform group without copying either uniform.

### 1. `src/pick_pipeline.rs`

Prepare an explicit shared layout and bind the existing two uniform buffers.

**Locate this existing block:**

```rust
pub fn mesh(
    device: &wgpu::Device,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn crop_clipping_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let entries = [80u64, 448];
    let entries = entries
        .into_iter()
        .enumerate()
        .map(
            |(binding, size)| wgpu::BindGroupLayoutEntry {
                binding: binding as u32,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: std::num::NonZeroU64::new(size),
                },
                count: None,
            },
        )
        .collect::<Vec<_>>();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("pick crop and clipping"),
        entries: &entries,
    })
}
pub fn crop_clipping_group(
    device: &wgpu::Device,
    crop: &wgpu::Buffer,
    clipping: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("pick crop and clipping"),
        layout: &crop_clipping_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: crop.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: clipping.as_entire_binding(),
            },
        ],
    })
}

pub fn mesh(
    device: &wgpu::Device,
```

## <a id="checkpoint-96zs-write-first-pick-primitives-from-shared-geometry"></a>12. Write first-pick primitives from shared geometry

Rasterize bounded integer owners and primitive addresses together.

### 1. `src/pick_pipeline.rs`

Reserve group three for the same shared geometry storage. The mesh pick shader now reads clipping from the combined uniform group. The mesh pick vertex entry reads storage attributes. Write original owners and triangle addresses to two integer outputs.

**Locate this existing block:**

```rust
            Some(model_layout),
            Some(crop_layout),
            Some(&crate::clipping_gpu::layout(device)),
        ],
        immediate_size: 0,
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("mesh IDs"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(
                concat!(
                    include_str!("triangle.wgsl"),
                    "\n",
                    include_str!("pick_crop.wgsl"),
                    "\n",
                    include_str!("triangle_pick.wgsl")
                ),
                3,
                0,
            )
            .into(),
        ),
    });
    // An integer attachment stores exact owner IDs; opaque depth rejects hidden faces before any pixel is read back.
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mesh IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: 24, step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3] },
                    wgpu::VertexBufferLayout { array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![2 => Float32x3] }] },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()), multisample: Default::default(), multiview_mask: None, cache: None,
        })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Some(model_layout),
            Some(&crop_clipping_layout(device)),
            Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device)),
        ],
        immediate_size: 0,
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("mesh IDs"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(
                concat!(
                    include_str!("triangle.wgsl"),
                    "\n",
                    include_str!("pick_crop.wgsl"),
                    "\n",
                    include_str!("triangle_pick.wgsl")
                ),
                2,
                1,
            )
            .into(),
        ),
    });
    // An integer attachment stores exact owner IDs; opaque depth rejects hidden faces before any pixel is read back.
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("mesh IDs"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("pick_vertex"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("pick_fragment"),
            compilation_options: Default::default(),
            targets: &[
                Some(wgpu::TextureFormat::Rg32Uint.into()),
                Some(crate::primitive_image::target(true)),
            ],
        }),
        primitive: Default::default(),
        depth_stencil: Some(crate::depth::surface()),
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
```

### 2. `src/triangle_pick.wgsl`

Reuse source vertex reads and preserve original owners in the first integer frame.

Replace the entire file with:

```wgsl
@group(2) @binding(0) var<uniform> crop:Crop;
struct MeshPickOutput{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) @interpolate(flat) flags:u32,@location(2) @interpolate(flat) primitive:u32}
@vertex fn pick_vertex(@builtin(vertex_index) occurrence:u32)->MeshPickOutput{
    let input=mesh_vertex(occurrence);let placed=geometry(input.position,input.colour,input.normal,object_settings);
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,(object_settings.selection.x>>1u)+occurrence/3u+1u);
}
struct PickFragment{@location(0) owner:vec2<u32>,@location(1) primitive:vec2<u32>}
@fragment fn pick_fragment(input:MeshPickOutput)->PickFragment{
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return PickFragment(vec2<u32>(select(0u,object_settings.selection.y,crop.mode==0u),0u),vec2<u32>(input.primitive&0xffffu,input.primitive>>16u));
}
```

### 3. `src/pick_gpu.rs`

Keep the existing fallback clipping buffer alongside its bind group. Pass the existing clipping buffer without a duplicate upload. Build the combined mesh group from the live request buffers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            clipping,
            caps,
            &self.crop,
        )]);
```

Replace it with:

```rust
            clipping,
            caps,
            &self.crop,
            clipping_buffer,
        )]);
```

**Locate this block:**

```rust
        sheets: Option<&crate::sheet_gpu::Lane>,
        tables: Option<&crate::finite_ink::Tables>,
        clipping: &wgpu::BindGroup,
        caps: Option<&mut crate::caps_gpu::Caps>,
    ) {
        assert!(
```

Replace it with:

```rust
        sheets: Option<&crate::sheet_gpu::Lane>,
        tables: Option<&crate::finite_ink::Tables>,
        clipping: &wgpu::BindGroup,
        clipping_buffer: &wgpu::Buffer,
        caps: Option<&mut crate::caps_gpu::Caps>,
    ) {
        assert!(
```

**Locate this block:**

```rust
    ) {
        let group = self
            .legacy_clipping
            .get_or_insert_with(|| crate::clipping_gpu::Clipping::new(device))
            .group
            .clone();
        self.submit_finite(
            device, queue, queued, meshes, ink, faces, text, clouds, sheets, None,
            &group, None,
        )
    }
    pub(crate) fn submit_finite(
```

Replace it with:

```rust
    ) {
        let clipping = self
            .legacy_clipping
            .get_or_insert_with(|| crate::clipping_gpu::Clipping::new(device));
        let group = clipping.group.clone();
        let buffer = clipping.buffer.clone();
        self.submit_finite(
            device, queue, queued, meshes, ink, faces, text, clouds, sheets, None,
            &group, &buffer, None,
        )
    }
    pub(crate) fn submit_finite(
```

### 4. `src/renderer.rs`

Use the scene’s current clipping buffer for bounded mesh picks.

**Locate this existing block:**

```rust
                &clipping.group,
                Some(&mut self.caps.borrow_mut()),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                &clipping.group,
                &clipping.buffer,
                Some(&mut self.caps.borrow_mut()),
```

### 5. `src/pick_encode.rs`

Receive both existing uniforms at the encoder boundary. Build only the combined binding, retaining independent crop groups for other owners. Clear and store the bounded primitive image in the same mesh depth test. Geometry owns group three in this mesh pass. Use the combined crop/clipping group only for mesh picking. Match nonindexed occurrences with the shared index reader. Source-face selection moves into its own integer color pass. Retain underlying mesh depth and primitive addresses while replacing the owner payload with a selected source-face ID. The ink predicate receives the bounded primitive image of this exact pick request.

**Locate this existing block:**

```rust
    crop: &wgpu::Buffer,
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear bounded section depth"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.section,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
    }
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("IDs"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_bind_group(3, clipping, &[]);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, crop_group, &[]);
        for mesh in meshes {
            mesh.draw(&mut pass);
        }
        if mode == crate::pick_sources::Mode::Face {
            faces.draw(&mut pass, meshes, view_group, crop_group);
        }
    }
    if let Some(caps) = caps {
        caps.encode_pick(
            device,
            &ctx.queue,
            &mut encoder,
            image,
            window,
            view_group,
            crop,
            clipping,
            meshes,
            mode,
        );
    }
    let tables = tables.map(|tables| crate::finite_ink::Tables {
        projected: tables.projected.clone(),
        tiles: tables.tiles.clone(),
        caps: image.section.clone(),
        primitive: tables.primitive.clone(),
    });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    crop: &wgpu::Buffer,
    clipping_buffer: &wgpu::Buffer,
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
    let mesh_crop =
        crate::pick_pipeline::crop_clipping_group(device, crop, clipping_buffer);
    {
        let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear bounded section depth"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.section,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
    }
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("IDs"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.colour,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.primitive.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, &mesh_crop, &[]);
        for mesh in meshes {
            mesh.draw_pulled(&mut pass);
        }
    }
    if mode == crate::pick_sources::Mode::Face {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("original source-face IDs"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            ..Default::default()
        });
        faces.draw(&mut pass, meshes, view_group, crop_group);
    }
    if let Some(caps) = caps {
        caps.encode_pick(
            device,
            &ctx.queue,
            &mut encoder,
            image,
            window,
            view_group,
            crop,
            clipping,
            meshes,
            mode,
        );
    }
    let tables = tables.map(|tables| crate::finite_ink::Tables {
        projected: tables.projected.clone(),
        tiles: tables.tiles.clone(),
        caps: image.section.clone(),
        primitive: image.primitive.view.clone(),
    });
```

### 6. `src/pick_gpu.rs`

Supply read-only access to the actual bounded texture for the GPU fixture.

**Locate this existing block:**

```rust
    }

    pub fn usage(&self) -> [u64; 3] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    #[cfg(all(test, not(target_arch = "wasm32")))]
    pub(crate) fn primitive_for_tests(&self) -> Option<(wgpu::Texture, Window)> {
        self.image
            .as_ref()
            .map(|image| (image.primitive.texture.clone(), image.window))
    }

    pub fn usage(&self) -> [u64; 3] {
```

### 7. `src/global_outline_tests.rs`

Reuse one primitive texture readback for color and cropped integer passes. Move the existing reader instead of duplicating its implementation. Keep existing color primitive assertions on the shared reader. Supply actual cropped primitive bytes, first owner picks at both densities, bounded dimensions and scene-visibility rebuilding before any color frame.

**Locate this existing block:**

```rust
}

#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses() {
    fn addresses(gpu: &super::Renderer) -> Vec<u32> {
        let texture = &gpu.primitive.texture;
        let width = texture.width();
        let height = texture.height();
        let pitch = (width * 4 + 255) / 256 * 256;
        let read = gpu
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("actual primitive addresses"),
                size: u64::from(pitch) * u64::from(height),
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        let mut encoder = gpu
            .device
            .create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &read,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(pitch),
                    rows_per_image: Some(height),
                },
            },
            texture.size(),
        );
        gpu.queue.submit([encoder.finish()]);
        let (tx, rx) = std::sync::mpsc::channel();
        read.slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap()
            });
        wait(gpu);
        rx.recv().unwrap().unwrap();
        let bytes = read.slice(..).get_mapped_range();
        let values = bytes
            .chunks_exact(pitch as usize)
            .flat_map(|row| {
                row[..width as usize * 4]
                    .chunks_exact(4)
                    .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
            })
            .collect();
        drop(bytes);
        read.unmap();
        values
    }
    let mut scene = Scene::empty();
    let a = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.2, -0.4, -0.4],
                    [0.2, -0.4, -0.4],
                    [0., 0.4, -0.4],
                ],
                [0.8, 0.2, 0.1],
            )
            .unwrap(),
        )
        .unwrap();
    let b = scene.copy_row(a, false).unwrap();
    scene.translate(a, [-0.5, 0., 0.]).unwrap();
    scene.translate(b, [0.5, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    for density in [1., 2.] {
        gpu.set_density(density);
        pixels(&gpu);
        let before = addresses(&gpu);
        assert_eq!(before[0], 0);
        assert_eq!(before[120 * 320 + 80], 1);
        assert_eq!(before[120 * 320 + 240], 2);
        assert_eq!(
            gpu.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        let key = *scene.objects()[0]
            .geometry()
            .unwrap()
            .face
            .keys()
            .next()
            .unwrap();
        gpu.set_highlight(
            &scene,
            Some(crate::pick_sources::Target {
                owner: a,
                part: crate::pick_sources::Part::Face(key),
            }),
        )
        .unwrap();
        pixels(&gpu);
        assert_eq!(
            addresses(&gpu),
            before,
            "A derived selected-face mesh must retain original triangle addresses"
        );
        gpu.set_highlight(&scene, None).unwrap();
    }
    let mut instances = Scene::empty();
    crate::scene_document::import(
        &mut instances,
        &crate::instance_specimen::display_bytes(),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    gpu.set_scene(&instances, None);
    pixels(&gpu);
    let placed = addresses(&gpu);
    assert_eq!(placed[120 * 320 + 64], 1);
    assert_eq!(placed[120 * 320 + 256], 2);
    gpu.resize(crate::viewport::Viewport {
        width: 640,
        height: 480,
    });
    assert_eq!(
        [
            gpu.primitive.texture.width(),
            gpu.primitive.texture.height()
        ],
        [640, 480]
    );
    gpu.resize(crate::viewport::Viewport {
        width: 320,
        height: 240,
    });
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert!(addresses(&gpu).iter().all(|id| *id == 0));
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

fn primitive_pixels(gpu: &super::Renderer, texture: &wgpu::Texture) -> Vec<u32> {
    let width = texture.width();
    let height = texture.height();
    let pitch = (width * 4 + 255) / 256 * 256;
    let read = gpu
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("actual primitive addresses"),
            size: u64::from(pitch) * u64::from(height),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let mut encoder = gpu
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &read,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(height),
            },
        },
        texture.size(),
    );
    gpu.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap()
        });
    wait(gpu);
    rx.recv().unwrap().unwrap();
    let bytes = read.slice(..).get_mapped_range();
    let values = bytes
        .chunks_exact(pitch as usize)
        .flat_map(|row| {
            row[..width as usize * 4]
                .chunks_exact(4)
                .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
        })
        .collect();
    drop(bytes);
    read.unmap();
    values
}
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses() {
    let mut scene = Scene::empty();
    let a = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.2, -0.4, -0.4],
                    [0.2, -0.4, -0.4],
                    [0., 0.4, -0.4],
                ],
                [0.8, 0.2, 0.1],
            )
            .unwrap(),
        )
        .unwrap();
    let b = scene.copy_row(a, false).unwrap();
    scene.translate(a, [-0.5, 0., 0.]).unwrap();
    scene.translate(b, [0.5, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    for density in [1., 2.] {
        gpu.set_density(density);
        pixels(&gpu);
        let before = primitive_pixels(&gpu, &gpu.primitive.texture);
        assert_eq!(before[0], 0);
        assert_eq!(before[120 * 320 + 80], 1);
        assert_eq!(before[120 * 320 + 240], 2);
        assert_eq!(
            gpu.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        let key = *scene.objects()[0]
            .geometry()
            .unwrap()
            .face
            .keys()
            .next()
            .unwrap();
        gpu.set_highlight(
            &scene,
            Some(crate::pick_sources::Target {
                owner: a,
                part: crate::pick_sources::Part::Face(key),
            }),
        )
        .unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            before,
            "A derived selected-face mesh must retain original triangle addresses"
        );
        gpu.set_highlight(&scene, None).unwrap();
    }
    let mut instances = Scene::empty();
    crate::scene_document::import(
        &mut instances,
        &crate::instance_specimen::display_bytes(),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    gpu.set_scene(&instances, None);
    pixels(&gpu);
    let placed = primitive_pixels(&gpu, &gpu.primitive.texture);
    assert_eq!(placed[120 * 320 + 64], 1);
    assert_eq!(placed[120 * 320 + 256], 2);
    gpu.resize(crate::viewport::Viewport {
        width: 640,
        height: 480,
    });
    assert_eq!(
        [
            gpu.primitive.texture.width(),
            gpu.primitive.texture.height()
        ],
        [640, 480]
    );
    gpu.resize(crate::viewport::Viewport {
        width: 320,
        height: 240,
    });
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert!(
        primitive_pixels(&gpu, &gpu.primitive.texture)
            .iter()
            .all(|id| *id == 0)
    );
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color() {
    let mut scene = Scene::empty();
    let a = scene
        .insert(
            PreparedMesh::triangle(
                [
                    [-0.2, -0.4, -0.4],
                    [0.2, -0.4, -0.4],
                    [0., 0.4, -0.4],
                ],
                [0.8, 0.2, 0.1],
            )
            .unwrap(),
        )
        .unwrap();
    let b = scene.copy_row(a, false).unwrap();
    scene.translate(a, [-0.5, 0., 0.]).unwrap();
    scene.translate(b, [0.5, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    assert!(
        primitive_pixels(&gpu, &gpu.primitive.texture)
            .iter()
            .all(|word| *word == 0)
    );
    for density in [1u32, 2] {
        gpu.set_density(density as f32);
        gpu.resize(crate::viewport::Viewport {
            width: 320 * density,
            height: 240 * density,
        });
        for (position, owner, primitive) in [([-0.5, 0.], a, 1u32), ([0.5, 0.], b, 2)] {
            gpu.request_pick(position, matrix()).unwrap();
            wait(&gpu);
            assert_eq!(
                gpu.take_pick(matrix())
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                owner.value()
            );
            let (texture, window) = gpu.picker.primitive_for_tests().unwrap();
            assert_eq!(
                [texture.width(), texture.height()],
                window.draw[2..]
            );
            assert!(
                texture.width() < window.full[0] && texture.height() < window.full[1]
            );
            let pixels = primitive_pixels(&gpu, &texture);
            let x = window.copy[0] + window.cursor[0] - window.draw[0];
            let y = window.copy[1] + window.cursor[1] - window.draw[1];
            assert_eq!(
                pixels[(y * texture.width() + x) as usize],
                primitive
            );
        }
    }
    Rc::make_mut(&mut scene.view).hide(&[a], true);
    gpu.set_scene(&scene, None);
    gpu.request_pick([0.5, 0.], matrix())
        .unwrap();
    wait(&gpu);
    assert_eq!(
        gpu.take_pick(matrix())
            .unwrap()
            .unwrap()
            .unwrap()[0],
        b.value()
    );
    let (texture, window) = gpu.picker.primitive_for_tests().unwrap();
    let pixels = primitive_pixels(&gpu, &texture);
    let x = window.copy[0] + window.cursor[0] - window.draw[0];
    let y = window.copy[1] + window.cursor[1] - window.draw[1];
    assert_eq!(
        pixels[(y * texture.width() + x) as usize],
        1,
        "Visible intervals must rebuild without a color frame"
    );
}
```

## <a id="checkpoint-96zt-fit-surface-depth-beneath-a-point-disc"></a>13. Fit surface depth beneath a point disc

Type the current two-axis surface carry for a disc fragment.

### 1. `src/ink_plane.wgsl`

Type the current two-axis disc-plane carry while reusing the existing neighbor and tolerance helpers.

**Locate this existing block:**

```wgsl
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
fn ink_disc_fragment_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let d = pixel - centre;
    var along_x = vec2<f32>(select(1.0, -1.0, d.x < 0.0), 0.0);
    var along_y = vec2<f32>(0.0, select(1.0, -1.0, d.y < 0.0));
    if (!ink_pair_planar(pixel, along_x, z, sample)) {
        along_x = -along_x;
        if (!ink_pair_planar(pixel, along_x, z, sample)) {
            return z <= depth + abs(depth) * DEPTH_REL_TOL;
        }
    }
    if (!ink_pair_planar(pixel, along_y, z, sample)) {
        along_y = -along_y;
        if (!ink_pair_planar(pixel, along_y, z, sample)) {
            return z <= depth + abs(depth) * DEPTH_REL_TOL;
        }
    }
    let gx = (ink_depth(pixel + along_x, sample) - z) * along_x.x;
    let gy = (ink_depth(pixel + along_y, sample) - z) * along_y.y;
    let predicted = z - d.x * gx - d.y * gy;
    return ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}
```

## <a id="checkpoint-96zu-test-the-disc-center-against-a-surface"></a>14. Test the disc center against a surface

Type the current center check that prevents a hidden point leaking at a fringe.

### 1. `src/ink_plane.wgsl`

Type the current source-center occlusion check and reuse it with fragment visibility.

**Locate this existing block:**

```wgsl
    return ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}
fn ink_disc_source_hidden(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return false;
    }
    let d = pixel - centre;
    for (var x = 0u; x < 2u; x++) {
        let dx = vec2<f32>(select(1.0, -1.0, x == 1u), 0.0);
        if (!ink_pair_planar(pixel, dx, z, sample)) {
            continue;
        }
        let gx = (ink_depth(pixel + dx, sample) - z) * dx.x;
        for (var y = 0u; y < 2u; y++) {
            let dy = vec2<f32>(0.0, select(1.0, -1.0, y == 1u));
            if (!ink_pair_planar(pixel, dy, z, sample)) {
                continue;
            }
            let gy = (ink_depth(pixel + dy, sample) - z) * dy.y;
            let diagonal = ink_depth(pixel + dx + dy, sample);
            let expected = z + gx * dx.x + gy * dy.y;
            if (diagonal == 0.0 || abs(diagonal - expected) > ink_tolerance(z, abs(gx) + abs(gy), 2.0)) {
                continue;
            }
            let predicted = z - d.x * gx - d.y * gy;
            if (predicted > depth + ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y))) {
                return true;
            }
        }
    }
    return false;
}
fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    if (!ink_disc_fragment_visible(pixel, centre, depth, sample)) {
        return false;
    }
    let source_pixel = floor(centre) + fract(pixel);
    return !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}
```

## <a id="checkpoint-96zv-carry-the-rasterized-triangle-gradient-to-ink"></a>15. Carry the rasterized triangle gradient to ink

Read the lossless primitive address and use its projected surface gradient.

### 1. `src/ink_plane.wgsl`

Type the current gradient carry with the matching canvas-or-crop primitive reader.

**Locate this existing block:**

```wgsl
    return !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}
fn ink_primitive(pixel:vec2<f32>,sample:u32)->u32{
    let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_primitive));
    if(any(at<vec2<i32>(0))||any(at>=size)){return 0u;}
    let halves=textureLoad(finite_primitive,at,0).xy;
    return halves.x|(halves.y<<16u);
}
fn ink_visible_plane(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let primitive = ink_primitive(pixel, sample);
    var gradient = vec2<f32>(0.0);
    if (primitive != 0u) {
        let triangle = min(primitive, arrayLength(&finite_projected)) - 1u;
        if (finite_projected[triangle].edge3.w < 3.0) {
            return ink_axis_visible(pixel, axis, sample);
        }
        gradient = finite_projected[triangle].gradient.xy;
    }
    if (any(abs(gradient) >= vec2<f32>(SLOPE_INVALID))) {
        return ink_axis_visible(pixel, axis, sample);
    }
    let delta = axis.at - pixel;
    let predicted = z + dot(gradient, delta);
    let tolerance = ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)) + ink_sag_slack(axis.sag, gradient);
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
```

## <a id="checkpoint-96zw-connect-surface-fitted-point-visibility"></a>16. Connect surface-fitted point visibility

Use the disc fragment and center tests for original point color and integer picks.

### 1. `src/finite_visibility.wgsl`

Connect the point-specific surface and center tests after clipping and section checks.

**Locate this existing block:**

```wgsl
 let hidden=!finite_visible(pixel,axis,depths,flags);let glass=view.opacity>0.&&view.opacity<1.;return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 let hidden=!finite_visible(pixel,axis,depths,flags);let glass=view.opacity>0.&&view.opacity<1.;return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

### 2. `src/finite_marker.wgsl`

Use the same original point predicate for color and cropped integer ownership.

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 3. `src/finite_marker_pick.wgsl`

Use the same original point predicate for color and cropped integer ownership.

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags)<=0.){discard;}
    return input.id;
```

### 4. `src/finite_visibility_tests.rs`

Supply an actual point disc pixel and first-pick regression at both physical display scales.

**Locate this existing block:**

```rust
        "physical viewport change invalidates projection even with the same tile dimensions"
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "physical viewport change invalidates projection even with the same tile dimensions"
    );
}

#[test]
fn a_point_on_a_sloping_original_face_keeps_its_disc_and_first_pick() {
    for density in [1u32, 2] {
        let size = [320 * density, 240 * density];
        let (front, _, _) = scene(0.3, 2, false);
        let mut renderer = renderer(&front, size, density as f32);
        let flat = red_pixels(&renderer, size);
        let (sloped, _, point) = scene(0.5, 2, true);
        renderer.set_scene(&sloped, None);
        assert_eq!(
            picked(&mut renderer, [0., 0.]).unwrap()[0],
            point.value(),
            "surface-fitted point owns the first pick before color"
        );
        assert_eq!(
            red_pixels(&renderer, size),
            flat,
            "a coplanar point keeps the same full CSS disc on a sloping face"
        );
        let (hidden, solid, _) = scene(0.7, 2, true);
        renderer.set_scene(&hidden, None);
        assert_eq!(
            picked(&mut renderer, [0., 0.]).unwrap()[0],
            solid.value()
        );
        assert_eq!(red_pixels(&renderer, size), 0);
    }
}
```

## <a id="checkpoint-96zx-test-the-rasterized-neighbors-at-the-stroke-source"></a>17. Test rasterized neighbors at the stroke source

Prepare exact source tests using the matching primitive image.

### 1. `src/ink_plane.wgsl`

Build a weighted source axis and reuse one exact clipped-triangle predicate for fringe and source neighbors.

**Locate this existing block:**

```wgsl
    let tolerance = ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)) + ink_sag_slack(axis.sag, gradient);
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let tolerance = ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)) + ink_sag_slack(axis.sag, gradient);
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}

fn finite_ink_axis(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>)->InkAxis{
 let delta=ends.zw-ends.xy;let len=length(delta);let along=delta/max(len,1e-6);
 let distance=clamp(dot(pixel-ends.xy,along),0.,len);let at=ends.xy+along*distance;
 let depth=select(depths.x,(depths.x*(len-distance)+depths.y*distance)/max(len,1e-6),len>1e-6);
 return InkAxis(at,depth,along,(depths.y-depths.x)/max(len,1e-3),vec2<f32>(0.));
}
fn ink_exact_floor(axis:InkAxis,triangle:ProjectedTriangle)->f32{
 return axis.depth+abs(axis.depth)*DEPTH_REL_TOL+ink_sag_slack(axis.sag,triangle.gradient.xy);
}
fn finite_triangle_hides(primitive:u32,axis:InkAxis)->bool{
 if(primitive==0u||primitive>arrayLength(&finite_projected)){return false;}
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,1.-hit.x)));
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis)->bool{
 let fringe=ink_primitive(pixel,0u);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),0u);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
 }
 return false;
}
```

## <a id="checkpoint-96zy-connect-fitted-surfaces-and-exact-stroke-occlusion"></a>18. Connect fitted surfaces and exact stroke occlusion

Activate surface fitting, source-neighbor tests and bounded tile fallback for original strokes.

### 1. `src/finite_visibility.wgsl`

Retain the strict construction-grid rule while original strokes use the current fitted and exact predicates. Connect current surface fitting, exact source neighbors and conservative bounded tile traversal without changing the grid predicate.

**Locate this existing block:**

```wgsl
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.0-depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,1.0-hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);let glass=view.opacity>0.&&view.opacity<1.;return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.0-depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,1.0-hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);let glass=view.opacity>0.&&view.opacity<1.;return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,1.-axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_sections));
 if(all(at>=vec2<i32>(0))&&all(at<size)&&textureLoad(finite_sections,at,0).y>=0.5){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,0u);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis)){return false;}
 if(plane_visible){return true;}
 if(any(axis.at<vec2<f32>(0.))||any(axis.at>=view.size)){return false;}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size_px=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(axis.at/span);let head=finite_tiles[1u+cell.y*size_px.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return false;}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);
  if(nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let bounds=finite_projected[primitive-1u].bounds;
  if(any(axis.at<bounds.xy-vec2<f32>(SLOPE_PX))||any(axis.at>bounds.zw+vec2<f32>(SLOPE_PX))){continue;}
  if(finite_triangle_hides(primitive,axis)){return false;}
 }
 return true;
}
```

### 2. `src/ink_plane.wgsl`

Retain the declared forward-depth rounding allowance in exact source tests until the reversed camera replaces this adapter.

**Locate this existing block:**

```wgsl
fn ink_exact_floor(axis:InkAxis,triangle:ProjectedTriangle)->f32{
 return axis.depth+abs(axis.depth)*DEPTH_REL_TOL+ink_sag_slack(axis.sag,triangle.gradient.xy);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn ink_exact_floor(axis:InkAxis,triangle:ProjectedTriangle)->f32{
 return axis.depth+abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7+ink_sag_slack(axis.sag,triangle.gradient.xy);
}
```

## <a id="checkpoint-96zz-separate-hidden-ink-color-from-pick-ownership"></a>19. Separate hidden ink color from pick ownership

Reveal hidden ink through glass without selecting an occluded original.

### 1. `src/finite_visibility.wgsl`

Pass an explicit pick mode and reuse the shared hidden-ink alpha rule. Keep one color attenuation rule while integer ownership uses physical visibility alone.

**Locate this existing block:**

```wgsl
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);let glass=view.opacity>0.&&view.opacity<1.;return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,1.-axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_sections));
 if(all(at>=vec2<i32>(0))&&all(at<size)&&textureLoad(finite_sections,at,0).y>=0.5){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,0u);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis)){return false;}
 if(plane_visible){return true;}
 if(any(axis.at<vec2<f32>(0.))||any(axis.at>=view.size)){return false;}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size_px=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(axis.at/span);let head=finite_tiles[1u+cell.y*size_px.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return false;}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);
  if(nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let bounds=finite_projected[primitive-1u].bounds;
  if(any(axis.at<bounds.xy-vec2<f32>(SLOPE_PX))||any(axis.at>bounds.zw+vec2<f32>(SLOPE_PX))){continue;}
  if(finite_triangle_hides(primitive,axis)){return false;}
 }
 return true;
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,1.-axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_sections));
 if(all(at>=vec2<i32>(0))&&all(at<size)&&textureLoad(finite_sections,at,0).y>=0.5){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,0u);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis)){return false;}
 if(plane_visible){return true;}
 if(any(axis.at<vec2<f32>(0.))||any(axis.at>=view.size)){return false;}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size_px=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(axis.at/span);let head=finite_tiles[1u+cell.y*size_px.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return false;}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);
  if(nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let bounds=finite_projected[primitive-1u].bounds;
  if(any(axis.at<bounds.xy-vec2<f32>(SLOPE_PX))||any(axis.at>bounds.zw+vec2<f32>(SLOPE_PX))){continue;}
  if(finite_triangle_hides(primitive,axis)){return false;}
 }
 return true;
}

fn finite_hidden_alpha(hidden:bool,picking:bool)->f32{
 if(picking){return select(1.,0.,hidden);}
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

### 2. `src/finite_stroke.wgsl`

Choose color attenuation or integer discard at the fragment call.

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 3. `src/finite_stroke_pick.wgsl`

Choose color attenuation or integer discard at the fragment call.

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true)<=0.){discard;}
    return input.id;
```

### 4. `src/finite_chain.wgsl`

Choose color attenuation or integer discard at the fragment call.

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 5. `src/finite_chain_pick.wgsl`

Choose color attenuation or integer discard at the fragment call.

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true)<=0.){discard;}
    return input.id;
```

### 6. `src/finite_marker.wgsl`

Choose color attenuation or integer discard at the fragment call.

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 7. `src/finite_marker_pick.wgsl`

Choose color attenuation or integer discard at the fragment call.

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags)<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
```

### 8. `src/finite_visibility_tests.rs`

Supply actual color, pre-color and post-color integer ownership checks for all three original ink families at both densities.

**Locate this existing block:**

```rust
        assert_eq!(red_pixels(&renderer, size), 0);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert_eq!(red_pixels(&renderer, size), 0);
    }
}

#[test]
fn translucent_faces_reveal_original_ink_without_giving_it_pick_ownership() {
    for density in [1u32, 2] {
        let size = [320 * density, 240 * density];
        for kind in 0..3 {
            let (hidden, solid, _) = scene(0.7, kind, false);
            let mut renderer = renderer(&hidden, size, density as f32);
            renderer.set_opacity(0.1);
            assert_eq!(
                picked(&mut renderer, [0., 0.]).unwrap()[0],
                solid.value(),
                "occluded family{kind} cannot win the first pick through glass at DPR{density}"
            );
            assert!(
                red_pixels(&renderer, size) > 30,
                "hidden family{kind} still contributes its through-glass color"
            );
            assert_eq!(
                picked(&mut renderer, [0., 0.]).unwrap()[0],
                solid.value(),
                "drawing the translucent frame does not change pick ownership"
            );
        }
    }
}
```

### 9. `src/opacity_gpu_tests.rs`

Update the supplied glass fixture to require the current nearer face owner; retain exact blending, X-ray, restoration and reuse checks.

**Locate this existing block:**

```rust
    assert_eq!(
        pick(&mut r, [0., 0.]).unwrap()[0],
        line.value()
    );
    r.set_opacity(0.);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(
        pick(&mut r, [0., 0.]).unwrap()[0],
        owner.value()
    );
    r.set_opacity(0.);
```

## <a id="checkpoint-96zza-connect-section-primitive-images"></a>20. Connect section primitives to color and first picking

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Corner Tangents t0, t1 &amp; Miter Vector m = (n0+n1)/dot</text>
  <polyline points="58,136 164,66 270,136" fill="none" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <polygon points="48,122 164,44 280,122 260,150 164,88 68,150" fill="#dbeafe" fill-opacity="0.7" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="164" y1="66" x2="164" y2="44" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="192" y="54" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">m</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Round Cap SDF (||uv|| &lt;= 1) + Arc-Length Dash Modulo</text>
  <circle cx="468" cy="102" r="28" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <line x1="468" y1="102" x2="496" y2="102" stroke="#dc2626" stroke-width="2"/>
  <rect x="526" y="88" width="46" height="28" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <rect x="592" y="88" width="24" height="28" fill="none" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="636" y="88" width="46" height="28" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Clean Mitered Corners &amp; Dashed CAD Strokes</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polyline points="834,136 904,68 974,136" fill="none" stroke="#111111" stroke-width="6" stroke-linejoin="miter" stroke-linecap="round"/>
  <line x1="964" y1="74" x2="1044" y2="136" stroke="#2563eb" stroke-width="4" stroke-dasharray="10 6" stroke-linecap="round"/>
</svg>
</div>

Share lossless cap addresses with mesh surfaces in color and first-pick images.

### 1. `src/physical.wgsl`

Own one lossless primitive encoder in the shared physical shader.

Create the file and type:

```wgsl
fn physical_primitive(primitive:u32)->vec2<u32>{
    return vec2<u32>(primitive&0xffffu,primitive>>16u);
}
```

### 2. `src/clipping_gpu.rs`

Assemble physical declarations once alongside clipping in every relevant shader.

**Locate this existing block:**

```rust
        "@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",
        include_str!("clip.wgsl")
    )
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",
        concat!(
            include_str!("physical.wgsl"),
            "\n",
            include_str!("clip.wgsl")
        )
    )
```

### 3. `src/triangle.wgsl`

Reuse the shared encoder instead of repeating the bit packing.

**Locate this existing block:**

```wgsl
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),vec2<u32>(input.primitive&0xffffu,input.primitive>>16u));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),physical_primitive(input.primitive));
}
```

### 4. `src/triangle_pick.wgsl`

Reuse the shared encoder instead of repeating the bit packing.

**Locate this existing block:**

```wgsl
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return PickFragment(vec2<u32>(select(0u,object_settings.selection.y,crop.mode==0u),0u),vec2<u32>(input.primitive&0xffffu,input.primitive>>16u));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return PickFragment(vec2<u32>(select(0u,object_settings.selection.y,crop.mode==0u),0u),physical_primitive(input.primitive));
}
```

### 5. `src/ink_plane.wgsl`

Decode a cap before any mesh lookup; reject unavailable projected addresses without clamping them to a different triangle.

**Locate this existing block:**

```wgsl
    var gradient = vec2<f32>(0.0);
    if (primitive != 0u) {
        let triangle = min(primitive, arrayLength(&finite_projected)) - 1u;
        if (finite_projected[triangle].edge3.w < 3.0) {
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    var gradient = vec2<f32>(0.0);
    if (clip_is_cap(primitive)) {
        let plane = clip_cap_plane(primitive);
        if (plane >= clipping.count) {
            return ink_axis_visible(pixel, axis, sample);
        }
        gradient = -clip_plane_slope(plane, view.size);
    } else if (primitive != 0u) {
        if (primitive > arrayLength(&finite_projected)) {
            return ink_axis_visible(pixel, axis, sample);
        }
        let triangle = primitive - 1u;
        if (finite_projected[triangle].edge3.w < 3.0) {
```

### 6. `src/cap_color.wgsl`

Add the integer output while keeping existing section consumers working. Write low and high words without float conversion.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    edge = max(edge, section_outline(in.pos.xy));
    let color = mix(select(hatched, solid, clipping.fill == 1u), vec3<f32>(0.0), edge);
    let marker = CAP_PRIMITIVE + i * 2u + select(0u, 1u, selected);
    return CapOut(vec4<f32>(color, 1.0), vec2<f32>(z,f32(i+1u)+select(0.0,8.0,selected)), z, mask);
}
```

Replace it with:

```wgsl
    edge = max(edge, section_outline(in.pos.xy));
    let color = mix(select(hatched, solid, clipping.fill == 1u), vec3<f32>(0.0), edge);
    let marker = CAP_PRIMITIVE + i * 2u + select(0u, 1u, selected);
    return CapOut(vec4<f32>(color, 1.0), vec2<f32>(z,f32(i+1u)+select(0.0,8.0,selected)), physical_primitive(marker), z, mask);
}
```

**Locate this block:**

```wgsl
    @location(1) section_depth: vec2<f32>, // visible section depth and presence
    @builtin(frag_depth) depth: f32, // depth of the plane here
    @builtin(sample_mask) mask: u32, // samples inside a solid and kept by the other planes
}
```

Replace it with:

```wgsl
    @location(1) section_depth: vec2<f32>, // visible section depth and presence
    @location(2) primitive: vec2<u32>,
    @builtin(frag_depth) depth: f32, // depth of the plane here
    @builtin(sample_mask) mask: u32, // samples inside a solid and kept by the other planes
}
```

### 7. `src/cap_pipeline.rs`

Reuse the shared unblended integer target.

**Locate this existing block:**

```rust
                },
            ],
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                },
                crate::primitive_image::target(true),
            ],
```

### 8. `src/caps_gpu.rs`

Accept the renderer’s existing primitive view. Load mesh addresses and replace only pixels covered by a visible section.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                                    wgpu::LoadOp::Load
                                },
                                store: wgpu::StoreOp::Store,
```

Replace it with:

```rust
                                    wgpu::LoadOp::Load
                                },
                                store: wgpu::StoreOp::Store,
                            },
                        }),
                        Some(wgpu::RenderPassColorAttachment {
                            view: primitive,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
```

**Locate this block:**

```rust
        color: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
```

Replace it with:

```rust
        color: &wgpu::TextureView,
        primitive: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
```

### 9. `src/renderer.rs`

Pass the matching physical primitive image into the section pass.

**Locate this existing block:**

```rust
            view,
            &self.depth,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            view,
            &self.primitive.view,
            &self.depth,
```

### 10. `src/cap_pick.wgsl`

Keep the original owner and section record beside the primitive output. Use the current pick marker, independently of selection. Store lossless low and high halves in the cropped target.

**Locate this existing block:**

```wgsl
}

struct CapId{@location(0) id:vec2<u32>,@location(1) section_depth:vec2<f32>,@builtin(frag_depth) depth:f32}
@fragment
fn fs_cap_id(in: CapVertex) -> CapId {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + vec2<f32>(frame.origin), vec2<f32>(frame.full), 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let word = clip_pick_word(i, in.pos.xy, frame.size);
    let count = bitcast<i32>(atomicLoad(&pick_caps[word]));
    let owner = select(atomicLoad(&pick_caps[word + 3u]), atomicLoad(&pick_caps[word + 1u]), count == 1);
    var kept = true;

    for (var j = 0u; j < clipping.count; j++) {
        kept = kept && (j == i || dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0)) >= 0.0);
    }

    if (count < 1 || owner == 0u || owner>arrayLength(&owners) || !kept || !(z >= 0.0 && z <= 1.0)) {
        discard;
    }

    let source=owners[owner-1u];return CapId(vec2<u32>(select(0u,source.x,source.y!=0u),0u),vec2<f32>(z,f32(i+1u)),z);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

struct CapId{@location(0) id:vec2<u32>,@location(1) section_depth:vec2<f32>,@location(2) primitive:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment
fn fs_cap_id(in: CapVertex) -> CapId {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + vec2<f32>(frame.origin), vec2<f32>(frame.full), 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let word = clip_pick_word(i, in.pos.xy, frame.size);
    let count = bitcast<i32>(atomicLoad(&pick_caps[word]));
    let owner = select(atomicLoad(&pick_caps[word + 3u]), atomicLoad(&pick_caps[word + 1u]), count == 1);
    var kept = true;

    for (var j = 0u; j < clipping.count; j++) {
        kept = kept && (j == i || dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0)) >= 0.0);
    }

    if (count < 1 || owner == 0u || owner>arrayLength(&owners) || !kept || !(z >= 0.0 && z <= 1.0)) {
        discard;
    }

    let marker=CAP_PRIMITIVE+i*2u;let source=owners[owner-1u];return CapId(vec2<u32>(select(0u,source.x,source.y!=0u),0u),vec2<f32>(z,f32(i+1u)),physical_primitive(marker),z);
}
```

### 11. `src/cap_pick_pipeline.rs`

Reuse the same integer target format without blending.

**Locate this existing block:**

```rust
                },
            ],
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                },
                crate::primitive_image::target(true),
            ],
```

### 12. `src/cap_pick_gpu.rs`

Attach the existing bounded crop image, loading previous mesh addresses.

**Locate this existing block:**

```rust
                }),
            ],
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.primitive.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
```

### 13. `src/global_outline_tests.rs`

Supply actual integer color, selection, clearing and pre-color cropped ownership checks at both physical display scales.

**Locate this existing block:**

```rust
        "Visible intervals must rebuild without a color frame"
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        "Visible intervals must rebuild without a color frame"
    );
}

fn cap_primitive_frame(gpu: &super::Renderer) {
    let texture = gpu
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("cap primitive fixture"),
            size: wgpu::Extent3d {
                width: gpu.size[0],
                height: gpu.size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
    gpu.draw(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &matrix(),
    );
    wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels() {
    let (scene, owner) = crate::cap_gpu_tests::scene(false);
    let mut gpu = renderer(&scene);
    for density in [1u32, 2] {
        gpu.set_density(density as f32);
        gpu.resize(crate::viewport::Viewport {
            width: 320 * density,
            height: 240 * density,
        });
        cap_primitive_frame(&gpu);
        let words = primitive_pixels(&gpu, &gpu.primitive.texture);
        let at = (96 * density * 320 * density + 208 * density) as usize;
        assert_eq!(words[at], 0xfffffff0);
        assert_eq!(words[0], 0);
        gpu.select_objects(&scene, &[owner]).unwrap();
        cap_primitive_frame(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture)[at],
            0xfffffff1
        );
        gpu.select_objects(&scene, &[]).unwrap();
    }
    gpu.set_scene(&Scene::empty(), None);
    cap_primitive_frame(&gpu);
    assert!(
        primitive_pixels(&gpu, &gpu.primitive.texture)
            .iter()
            .all(|word| *word == 0)
    );
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner() {
    let (scene, owner) = crate::cap_gpu_tests::scene(false);
    let mut gpu = renderer(&scene);
    for density in [1u32, 2] {
        gpu.set_density(density as f32);
        gpu.resize(crate::viewport::Viewport {
            width: 320 * density,
            height: 240 * density,
        });
        assert!(
            primitive_pixels(&gpu, &gpu.primitive.texture)
                .iter()
                .all(|word| *word == 0)
        );
        gpu.request_pick([0.3, 0.2], matrix())
            .unwrap();
        wait(&gpu);
        assert_eq!(
            gpu.take_pick(matrix())
                .unwrap()
                .unwrap()
                .unwrap()[0],
            owner.value()
        );
        let (texture, window) = gpu.picker.primitive_for_tests().unwrap();
        let words = primitive_pixels(&gpu, &texture);
        let x = window.copy[0] + window.cursor[0] - window.draw[0];
        let y = window.copy[1] + window.cursor[1] - window.draw[1];
        assert_eq!(
            words[(y * texture.width() + x) as usize],
            0xfffffff0
        );
        assert!(texture.width() < window.full[0] && texture.height() < window.full[1]);
    }
}
```

## <a id="checkpoint-96zzb-read-section-visibility-from-one-image"></a>21. Read section visibility from one image

### 1. `src/finite_visibility.wgsl`

Remove the redundant section texture declaration. Decode the exact reserved plane before reconstructing its depth at the source point. Reuse the bounded crop-aware primitive reader for the section source fit.

**Locate this existing block:**

```wgsl
@group(1) @binding(2) var finite_depth:texture_depth_2d;
@group(1) @binding(3) var finite_sections:texture_2d<f32>;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
 let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_sections));if(any(at<vec2<i32>(0))||any(at>=size)){return true;}let cap=textureLoad(finite_sections,at,0).xy;if(cap.y<0.5){return true;}let plane=(u32(cap.y)-1u)&7u;let z=1.0-clip_plane_depth(plane,clip_ndc(pixel,view.size,0.0).xy);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=1.0-textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.0-depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,1.0-hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,1.-axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_sections));
 if(all(at>=vec2<i32>(0))&&all(at<size)&&textureLoad(finite_sections,at,0).y>=0.5){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,0u);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(2) var finite_depth:texture_depth_2d;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
 let primitive=ink_primitive(pixel,0u);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=1.-clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=1.0-textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.0-depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,1.0-hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,1.-axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,0u))){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,0u);
```

### 2. `src/finite_ink.rs`

Retain only physical depth and primitive texture identities in the cache. Remove the redundant table member. Remove only binding 3; stable primitive binding 4 remains valid. Stop binding the duplicate section image. Invalidate cached groups only when their actual resources change. Retain only resources used by this bind group. Match the smaller cached group.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        verts: u32,
    ) -> bool {
        let bindings = self.bindings.borrow();
        let Some((_, _, _, _, _, group)) = bindings.as_ref() else {
            return false;
```

Replace it with:

```rust
        verts: u32,
    ) -> bool {
        let bindings = self.bindings.borrow();
        let Some((_, _, _, _, group)) = bindings.as_ref() else {
            return false;
```

**Locate this block:**

```rust
            tables.projected.clone(),
            tables.tiles.clone(),
            depth.clone(),
            tables.caps.clone(),
            tables.primitive.clone(),
            group,
        ));
```

Replace it with:

```rust
            tables.projected.clone(),
            tables.tiles.clone(),
            depth.clone(),
            tables.primitive.clone(),
            group,
        ));
```

**Locate this block:**

```rust
        let mut bindings = self.bindings.borrow_mut();
        if bindings
            .as_ref()
            .is_some_and(|(p, t, d, c, i, _)| {
                p == &tables.projected
                    && t == &tables.tiles
                    && d == depth
                    && c == &tables.caps
                    && i == &tables.primitive
            })
        {
```

Replace it with:

```rust
        let mut bindings = self.bindings.borrow_mut();
        if bindings
            .as_ref()
            .is_some_and(|(p, t, d, i, _)| {
                p == &tables.projected
                    && t == &tables.tiles
                    && d == depth
                    && i == &tables.primitive
            })
        {
```

**Locate this block:**

```rust
                    resource: wgpu::BindingResource::TextureView(depth),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&tables.caps),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&tables.primitive),
                },
```

Replace it with:

```rust
                    resource: wgpu::BindingResource::TextureView(depth),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&tables.primitive),
                },
```

**Locate this block:**

```rust
                        count: None,
                    },
                    crate::text_gpu_support::texture_entry(
                        3,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::TextureSampleType::Float {
                            filterable: false,
                        },
                        false,
                    ),
                    crate::text_gpu_support::texture_entry(
                        4,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::TextureSampleType::Uint,
```

Replace it with:

```rust
                        count: None,
                    },
                    crate::text_gpu_support::texture_entry(
                        4,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::TextureSampleType::Uint,
```

**Locate this block:**

```rust
pub struct Tables {
    pub projected: wgpu::Buffer,
    pub tiles: wgpu::Buffer,
    pub caps: wgpu::TextureView,
    pub primitive: wgpu::TextureView,
}
impl Ink {
```

Replace it with:

```rust
pub struct Tables {
    pub projected: wgpu::Buffer,
    pub tiles: wgpu::Buffer,
    pub primitive: wgpu::TextureView,
}
impl Ink {
```

**Locate this block:**

```rust
            wgpu::TextureView,
            wgpu::TextureView,
            wgpu::BindGroup,
        )>,
    >,
```

Replace it with:

```rust
            wgpu::TextureView,
            wgpu::BindGroup,
        )>,
    >,
```

### 3. `src/renderer.rs`

Pass only the physical primitive image to finite ink.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let tables = crate::finite_ink::Tables {
            projected: tiles.projected.clone(),
            tiles: tiles.buffer.clone(),
            caps: self.caps.borrow().depth.view.clone(),
            primitive: self.primitive.view.clone(),
```

Replace it with:

```rust
        let tables = crate::finite_ink::Tables {
            projected: tiles.projected.clone(),
            tiles: tiles.buffer.clone(),
            primitive: self.primitive.view.clone(),
```

**Locate this block:**

```rust
                    tiles: tiles.buffer.clone(),
                    caps: self.caps.borrow().depth.view.clone(),
                    primitive: self.primitive.view.clone(),
                }
            };
```

Replace it with:

```rust
                    tiles: tiles.buffer.clone(),
                    primitive: self.primitive.view.clone(),
                }
            };
```

### 4. `src/pick_encode.rs`

Use the crop’s own matching primitive image.

**Locate this existing block:**

```rust
        tiles: tables.tiles.clone(),
        caps: image.section.clone(),
        primitive: image.primitive.view.clone(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        tiles: tables.tiles.clone(),
        primitive: image.primitive.view.clone(),
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 20.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 20.1
npm --prefix ../session_tests run course -- save 20.1
```

Continue with [20.2 · Physical formats, shared section inputs and display defaults](../../consolidation/96zzc-share-section-addresses-with-outlines-and-ambient.md).
