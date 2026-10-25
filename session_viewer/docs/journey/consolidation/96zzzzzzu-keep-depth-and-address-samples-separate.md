# 22.5 · Coherent multisampling and frame-resource ownership

**Steps:** 19 · **Source:** [engine/gpu/render.rs](../../../src/engine/gpu/render.rs) · [engine/gpu/pass.rs](../../../src/engine/gpu/pass.rs) · [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs) · [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">4× MSAA Subpixel Samples (s0..s3) Inside Pixel Square</text>
  <rect x="108" y="50" width="108" height="104" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="116" y1="146" x2="208" y2="56" stroke="#2563eb" stroke-width="2.5"/>
  <polygon points="116,146 208,56 216,56 216,154 116,154" fill="#dbeafe" fill-opacity="0.65"/>
  <circle cx="142" cy="76" r="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="188" cy="68" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <circle cx="136" cy="124" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <circle cx="182" cy="116" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="252" y="106" font-family="monospace" font-size="10" font-weight="700" fill="#111111">cov = 3/4</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Key + Fill + Rim Rig + Ground Plane Major/Minor Grid</text>
  <polygon points="432,140 492,98 688,98 628,140" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="462" y1="119" x2="658" y2="119" stroke="#dc2626" stroke-width="1.75"/>
  <line x1="560" y1="98" x2="500" y2="140" stroke="#16a34a" stroke-width="1.75"/>
  <polygon points="520,122 560,62 600,122" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <ellipse cx="560" cy="126" rx="34" ry="8" fill="#64748b" fill-opacity="0.45"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Anti-Aliased CAD Viewport + Ground Grid &amp; Contact Shadow</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="826,144 876,104 1042,104 992,144" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1.2"/>
  <ellipse cx="934" cy="130" rx="36" ry="8" fill="#94a3b8" fill-opacity="0.5"/>
  <polygon points="894,126 934,58 974,126" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
</svg>
</div>

## <a id="checkpoint-96zzzzzzu-keep-depth-and-address-samples-separate"></a>1. Keep depth and address samples separate

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Offscreen Rg32Uint Texture (owner_id, sub_id)</text>
  <rect x="74" y="48" width="224" height="106" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="146" y="76" width="80" height="50" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="105" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">5×5 px Window</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">COPY_SRC Attachment</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">copy_texture_to_buffer (256B Row Pitch) -&gt; Staging</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Staging Buffer (MAP_READ | COPY_DST)</text>
  <rect x="446" y="90" width="228" height="24" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="106" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">bytes_per_row = 256 (WebGPU alignment)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">slice.map_async(MapMode::Read, ...)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Callback Reads Mapped [u32; 2] Winner</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="58" width="216" height="44" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">get_mapped_range() -&gt; (id, sub)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="94" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">staging.unmap()</text>
  <rect x="846" y="114" width="176" height="32" rx="4" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Selected Owner Updated</text>
</svg>
</div>

### 1. `src/frame_targets.rs`

Create the file and type:

```rust
pub(crate) struct Targets {
    pub depth: wgpu::TextureView,
    pub primitive: crate::primitive_image::Image,
    color: Option<wgpu::TextureView>,
    pub samples: u32,
}

impl Targets {
    pub fn new(
        device: &wgpu::Device,
        size: [u32; 2],
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert!(matches!(samples, 1 | 4) && !size.contains(&0));
        let image = |label, format| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: size[0],
                        height: size[1],
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        Self {
            depth: image("scene depth", crate::depth::FORMAT),
            primitive: crate::primitive_image::Image::with_samples(
                device, size, samples,
            ),
            color: (samples == 4).then(|| image("multisampled scene colour", format)),
            samples,
        }
    }

    pub fn color<'a>(&'a self, single: &'a wgpu::TextureView) -> &'a wgpu::TextureView {
        self.color.as_ref().unwrap_or(single)
    }

    pub fn resolve(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        single: &wgpu::TextureView,
    ) {
        let Some(color) = &self.color else {
            return;
        };
        let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("resolve scene colour"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color,
                resolve_target: Some(single),
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Discard,
                },
            })],
            ..Default::default()
        });
    }
}
```

### 2. `src/primitive_image.rs`

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn new(device: &wgpu::Device, size: [u32; 2]) -> Self {
        Self::with_samples(device, size, 1)
    }
    pub fn with_samples(device: &wgpu::Device, size: [u32; 2], samples: u32) -> Self {
        assert!(matches!(samples, 1 | 4));
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("physical triangle addresses"),
            size: wgpu::Extent3d {
                width: size[0].max(1),
                height: size[1].max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | if samples == 1 {
                    wgpu::TextureUsages::COPY_SRC
                } else {
                    wgpu::TextureUsages::empty()
                },
            view_formats: &[],
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
mod fit_tests;
pub mod geometry_cache;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod fit_tests;
mod frame_targets;
pub mod geometry_cache;
```

### 4. `src/frame_target_tests.rs`

Create the file and type:

```rust
use crate::frame_targets::Targets;

fn read(device: &wgpu::Device, buffer: &wgpu::Buffer) -> Vec<u8> {
    let (tx, rx) = std::sync::mpsc::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let data = buffer.slice(..).get_mapped_range().to_vec();
    buffer.unmap();
    data
}

#[test]
fn actual_samples_keep_depth_and_integer_addresses_and_resolve_only_colour() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) =
        pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e| {
        panic!("actual frame targets: {e}")
    }));
    for samples in [1, 4] {
        let mut targets = Targets::new(
            &device,
            [8, 8],
            wgpu::TextureFormat::Rgba8Unorm,
            samples,
        );
        assert_eq!(
            targets.primitive.texture.sample_count(),
            samples
        );
        assert_eq!(
            targets.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        assert_eq!(
            targets
                .primitive
                .texture
                .usage()
                .contains(wgpu::TextureUsages::COPY_SRC),
            samples == 1
        );
        let output = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("resolved float colour"),
            size: wgpu::Extent3d {
                width: 8,
                height: 8,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let single = output.create_view(&Default::default());
        assert_eq!(
            targets.color(&single) == &single,
            samples == 1
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual distinct samples"),
            source: wgpu::ShaderSource::Wgsl(r#"
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4f {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3));return vec4f(p[i],0.5,1);
}
struct Hit { @location(0) color:vec4f, @location(1) primitive:vec2u, @builtin(frag_depth) depth:f32 }
@fragment fn fragment(@builtin(sample_index) sample:u32)->Hit {
 return Hit(vec4f(f32(sample&1u),f32((sample>>1u)&1u),0,1),vec2u(sample+1u,50000u+sample),f32(sample+1u)*0.125);
}"#.into()),
        });
        let pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("per-sample colour, integer and depth"),
            layout: None,
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
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(crate::primitive_image::target(true)),
                ],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: crate::depth::FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        });
        let storage = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let mut source = format!(
            r#"
@group(0) @binding(0) var depth:texture_depth_2d;
@group(0) @binding(1) var physical:texture_2d<u32>;
@group(0) @binding(2) var<storage,read_write> values:array<vec4u>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) id:vec3u) {{
 if(id.x>={samples}u){{return;}}
 let primitive=textureLoad(physical,vec2i(2),SAMPLE).xy;
 values[id.x]=vec4u(bitcast<u32>(textureLoad(depth,vec2i(2),SAMPLE)),primitive,id.x);
}}
"#
        );
        if samples == 4 {
            source = source
                .replace(
                    "texture_depth_2d",
                    "texture_depth_multisampled_2d",
                )
                .replace(
                    "texture_2d<u32>",
                    "texture_multisampled_2d<u32>",
                );
        }
        source = source.replace(
            "SAMPLE",
            if samples == 4 {
                "i32(id.x)"
            } else {
                "0"
            },
        );
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual sample readback"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let compute =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &module,
                entry_point: Some("read_samples"),
                compilation_options: Default::default(),
                cache: None,
            });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &compute.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&targets.depth),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &targets.primitive.view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: storage.as_entire_binding(),
                },
            ],
        });
        let result = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let pixels = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 256 * 8,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("actual matching sample attachments"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: targets.color(&single),
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &targets.primitive.view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &targets.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(0.),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                ..Default::default()
            });
            pass.set_pipeline(&pipe);
            pass.draw(0..3, 0..1);
        }
        targets.resolve(&mut encoder, &single);
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&compute);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
        encoder.copy_texture_to_buffer(
            output.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &pixels,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(8),
                },
            },
            output.size(),
        );
        queue.submit([encoder.finish()]);
        let actual = read(&device, &result);
        for sample in 0..samples as usize {
            let words = actual[sample * 16..sample * 16 + 16]
                .chunks_exact(4)
                .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            assert_eq!(
                f32::from_bits(words[0]),
                (sample + 1) as f32 * 0.125
            );
            assert_eq!(
                &words[1..],
                &[
                    sample as u32 + 1,
                    50000 + sample as u32,
                    sample as u32
                ]
            );
        }
        let actual = read(&device, &pixels);
        let expected = if samples == 4 {
            [128u8, 128, 0, 255]
        } else {
            [0, 0, 0, 255]
        };
        assert!(
            actual[2 * 256 + 2 * 4..2 * 256 + 2 * 4 + 4]
                .iter()
                .zip(expected)
                .all(|(&a, b)| a.abs_diff(b) <= 1)
        );
        let depth = targets.depth.clone();
        let primitive = targets.primitive.texture.clone();
        let other = if samples == 1 {
            4
        } else {
            1
        };
        targets = Targets::new(
            &device,
            [17, 13],
            wgpu::TextureFormat::Rgba8Unorm,
            other,
        );
        assert_eq!(
            targets.primitive.texture.sample_count(),
            other
        );
        assert_eq!(
            [
                targets.primitive.texture.width(),
                targets.primitive.texture.height()
            ],
            [17, 13]
        );
        assert_ne!(targets.depth, depth);
        assert_ne!(targets.primitive.texture, primitive);
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&compute);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
        queue.submit([encoder.finish()]);
        let retained = read(&device, &result);
        for sample in 0..samples as usize {
            let at = sample * 16;
            assert_eq!(
                f32::from_bits(u32::from_ne_bytes(
                    retained[at..at + 4].try_into().unwrap()
                )),
                (sample + 1) as f32 * 0.125
            );
            assert_eq!(
                u32::from_ne_bytes(retained[at + 4..at + 8].try_into().unwrap()),
                sample as u32 + 1
            );
            assert_eq!(
                u32::from_ne_bytes(retained[at + 8..at + 12].try_into().unwrap()),
                50000 + sample as u32
            );
        }
        targets = Targets::new(
            &device,
            [8, 8],
            wgpu::TextureFormat::Rgba8Unorm,
            samples,
        );
        assert_eq!(
            targets.primitive.texture.sample_count(),
            samples
        );
    }
}
```

### 5. `src/lib.rs`

**Locate this existing block:**

```rust
mod fit_tests;
mod frame_targets;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod fit_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod frame_target_tests;
mod frame_targets;
```

## <a id="checkpoint-96zzzzzzv-reuse-coherent-frame-targets"></a>2. Reuse coherent frame targets

### 1. `src/renderer.rs`

Use your editor’s **whole-word Replace All** in this file. Find this field reference:

```rust
self.depth
```

Replace every match with:

```rust
self.targets.depth
```

### 2. `src/renderer.rs`

Use your editor’s **whole-word Replace All** in this file. Find this field reference:

```rust
self.primitive
```

Replace every match with:

```rust
self.targets.primitive
```

### 3. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                }
            }
        }
        self.widget.draw(&mut encoder, view);
        if let Some(pyramid) = self.pyramid.borrow_mut().as_mut() {
```

Replace it with:

```rust
                }
            }
        }
        self.targets.resolve(&mut encoder, output);
        self.widget.draw(&mut encoder, output);
        if let Some(pyramid) = self.pyramid.borrow_mut().as_mut() {
```

**Locate this block:**

```rust
        transform: &[f32; 16],
        normals: bool,
    ) {
        let (clip_group, clip_revision) = {
            let mut clip = self.clipping.borrow_mut();
            clip.frame_anchored(
```

Replace it with:

```rust
        transform: &[f32; 16],
        normals: bool,
    ) {
        let output = view;
        let view = self.targets.color(output);
        let (clip_group, clip_revision) = {
            let mut clip = self.clipping.borrow_mut();
            clip.frame_anchored(
```

**Locate this block:**

```rust
        self.space.anchor()
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] {
            self.cancel_pick();
        }
        self.size = [size.width, size.height];
        self.targets.depth = Self::depth(&self.device, size.width, size.height);
        self.targets.primitive = crate::primitive_image::Image::new(&self.device, self.size);
    }

    pub fn set_density(&mut self, density: f32) {
```

Replace it with:

```rust
        self.space.anchor()
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size == [size.width, size.height] {
            return;
        }
        self.cancel_pick();
        self.size = [size.width, size.height];
        self.targets = crate::frame_targets::Targets::new(
            &self.device,
            self.size,
            self.format,
            self.targets.samples,
        );
    }

    pub fn set_density(&mut self, density: f32) {
```

**Locate this block:**

```rust
            settings_bytes: 0,
            uniform,
            view_group,
            depth,
            primitive,
            strokes,
            paths,
            markers,
```

Replace it with:

```rust
            settings_bytes: 0,
            uniform,
            view_group,
            targets,
            strokes,
            paths,
            markers,
```

**Locate this block:**

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let primitive = crate::primitive_image::Image::new(&device, [640, 480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

Replace it with:

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        let targets =
            crate::frame_targets::Targets::new(&device, [640, 480], format, 1);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

**Locate this block:**

```rust
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    primitive: crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
```

Replace it with:

```rust
    view_group: wgpu::BindGroup,
    targets: crate::frame_targets::Targets,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
mod fit_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
mod frame_target_tests;
mod frame_targets;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod fit_tests;
mod frame_targets;
```

### 5. `src/renderer.rs`

**Locate this existing block:**

```rust
#[path = "round_caps_tests.rs"]
mod round_caps_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "round_caps_tests.rs"]
mod round_caps_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "frame_target_tests.rs"]
mod frame_target_tests;
```

### 6. `src/global_outline_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "Rebased original face and ink pixels must match exactly"
    );
    assert_eq!(
        primitive_pixels(&gpu, &gpu.primitive.texture),
        addresses
```

Replace it with:

```rust
        "Rebased original face and ink pixels must match exactly"
    );
    assert_eq!(
        primitive_pixels(&gpu, &gpu.targets.primitive.texture),
        addresses
```

**Locate this block:**

```rust
    old[12] = -2000.;
    gpu.set_highlight(&scene, target).unwrap();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, old);
    let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
    let geometry = gpu
        .highlight
        .geometry_for_test()
```

Replace it with:

```rust
    old[12] = -2000.;
    gpu.set_highlight(&scene, target).unwrap();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, old);
    let addresses = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
    let geometry = gpu
        .highlight
        .geometry_for_test()
```

**Locate this block:**

```rust
                .count()
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
    }
```

Replace it with:

```rust
                .count()
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture),
            addresses
        );
    }
```

**Locate this block:**

```rust
            "A valid original highlight must change actual pixels: {target:?}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        pixels.push(selected);
```

Replace it with:

```rust
            "A valid original highlight must change actual pixels: {target:?}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture),
            addresses
        );
        pixels.push(selected);
```

**Locate this block:**

```rust
    let transform = camera.uniform();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    let uploads = gpu.stats()[0];
    let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
    let mut pixels = Vec::new();
    for target in targets {
        assert!(target.valid(&scene));
```

Replace it with:

```rust
    let transform = camera.uniform();
    let baseline = crate::cap_gpu_tests::pixels_at(&gpu, transform);
    let uploads = gpu.stats()[0];
    let addresses = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
    let mut pixels = Vec::new();
    for target in targets {
        assert!(target.valid(&scene));
```

**Locate this block:**

```rust
    for opacity in [0., 0.5, 0.95] {
        gpu.set_opacity(opacity);
        gpu.set_scene(&scene, None);
        let stored = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
        gpu.instances.ordinary.clear();
        assert_eq!(
            pixels(&gpu),
            stored,
            "Exact transparent placements at {opacity}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        gpu.set_scene(&scene, None);
```

Replace it with:

```rust
    for opacity in [0., 0.5, 0.95] {
        gpu.set_opacity(opacity);
        gpu.set_scene(&scene, None);
        let stored = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
        gpu.instances.ordinary.clear();
        assert_eq!(
            pixels(&gpu),
            stored,
            "Exact transparent placements at {opacity}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture),
            addresses
        );
        gpu.set_scene(&scene, None);
```

**Locate this block:**

```rust
    for opacity in [0., 0.5, 0.95, 1.] {
        gpu.set_opacity(opacity);
        gpu.set_scene(&scene, None);
        let stored = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
        gpu.instances.ordinary.clear();
        assert_eq!(
            pixels(&gpu),
            stored,
            "Exact ordinary blend at {opacity}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        gpu.set_scene(&scene, None);
```

Replace it with:

```rust
    for opacity in [0., 0.5, 0.95, 1.] {
        gpu.set_opacity(opacity);
        gpu.set_scene(&scene, None);
        let stored = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
        gpu.instances.ordinary.clear();
        assert_eq!(
            pixels(&gpu),
            stored,
            "Exact ordinary blend at {opacity}"
        );
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture),
            addresses
        );
        gpu.set_scene(&scene, None);
```

**Locate this block:**

```rust
        wanted
    );
    let stored = pixels(&gpu);
    let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
    gpu.instances.ordinary.clear();
    assert_eq!(pixels(&gpu), stored);
    assert_eq!(
        primitive_pixels(&gpu, &gpu.primitive.texture),
        addresses
    );
    gpu.set_scene(&scene, None);
```

Replace it with:

```rust
        wanted
    );
    let stored = pixels(&gpu);
    let addresses = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
    gpu.instances.ordinary.clear();
    assert_eq!(pixels(&gpu), stored);
    assert_eq!(
        primitive_pixels(&gpu, &gpu.targets.primitive.texture),
        addresses
    );
    gpu.set_scene(&scene, None);
```

**Locate this block:**

```rust
        gpu.meshes[0].packed()
    );
    let stored = pixels(&gpu);
    let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
    gpu.instances.ordinary.clear();
    assert_eq!(pixels(&gpu), stored);
    assert_eq!(
        primitive_pixels(&gpu, &gpu.primitive.texture),
        addresses
    );
    gpu.set_scene(&scene, None);
```

Replace it with:

```rust
        gpu.meshes[0].packed()
    );
    let stored = pixels(&gpu);
    let addresses = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
    gpu.instances.ordinary.clear();
    assert_eq!(pixels(&gpu), stored);
    assert_eq!(
        primitive_pixels(&gpu, &gpu.targets.primitive.texture),
        addresses
    );
    gpu.set_scene(&scene, None);
```

**Locate this block:**

```rust
        .unwrap();
        assert_ne!(pixels(&gpu), plain);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            addresses
        );
        gpu.set_highlight(&scene, None).unwrap();
```

Replace it with:

```rust
        .unwrap();
        assert_ne!(pixels(&gpu), plain);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture),
            addresses
        );
        gpu.set_highlight(&scene, None).unwrap();
```

**Locate this block:**

```rust
    for opacity in [1., 0.5] {
        gpu.set_opacity(opacity);
        let plain = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.primitive.texture);
        let key = *scene.objects()[0]
            .geometry()
            .unwrap()
```

Replace it with:

```rust
    for opacity in [1., 0.5] {
        gpu.set_opacity(opacity);
        let plain = pixels(&gpu);
        let addresses = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
        let key = *scene.objects()[0]
            .geometry()
            .unwrap()
```

**Locate this block:**

```rust
        gpu.select_objects(&scene, &[]).unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            plain
        );
        let before = gpu.meshes[0].packed();
```

Replace it with:

```rust
        gpu.select_objects(&scene, &[]).unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture),
            plain
        );
        let before = gpu.meshes[0].packed();
```

**Locate this block:**

```rust
            .set_primitive_base(&gpu.queue, base)
            .unwrap();
        pixels(&gpu);
        let plain = primitive_pixels(&gpu, &gpu.primitive.texture);
        assert_eq!(plain[120 * 320 + 160], base + 1);
        gpu.select_objects(&scene, &[owner]).unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            plain
        );
        assert_eq!(
```

Replace it with:

```rust
            .set_primitive_base(&gpu.queue, base)
            .unwrap();
        pixels(&gpu);
        let plain = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
        assert_eq!(plain[120 * 320 + 160], base + 1);
        gpu.select_objects(&scene, &[owner]).unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture),
            plain
        );
        assert_eq!(
```

**Locate this block:**

```rust
            height: 240 * density,
        });
        assert!(
            primitive_pixels(&gpu, &gpu.primitive.texture)
                .iter()
                .all(|word| *word == 0)
        );
```

Replace it with:

```rust
            height: 240 * density,
        });
        assert!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture)
                .iter()
                .all(|word| *word == 0)
        );
```

**Locate this block:**

```rust
    gpu.set_scene(&Scene::empty(), None);
    cap_primitive_frame(&gpu);
    assert!(
        primitive_pixels(&gpu, &gpu.primitive.texture)
            .iter()
            .all(|word| *word == 0)
    );
```

Replace it with:

```rust
    gpu.set_scene(&Scene::empty(), None);
    cap_primitive_frame(&gpu);
    assert!(
        primitive_pixels(&gpu, &gpu.targets.primitive.texture)
            .iter()
            .all(|word| *word == 0)
    );
```

**Locate this block:**

```rust
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
```

Replace it with:

```rust
            height: 240 * density,
        });
        cap_primitive_frame(&gpu);
        let words = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
        let at = (96 * density * 320 * density + 208 * density) as usize;
        assert_eq!(words[at], 0xfffffff0);
        assert_eq!(words[0], 0);
        gpu.select_objects(&scene, &[owner]).unwrap();
        cap_primitive_frame(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture)[at],
            0xfffffff1
        );
        gpu.select_objects(&scene, &[]).unwrap();
```

**Locate this block:**

```rust
    scene.translate(b, [0.5, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    assert!(
        primitive_pixels(&gpu, &gpu.primitive.texture)
            .iter()
            .all(|word| *word == 0)
    );
```

Replace it with:

```rust
    scene.translate(b, [0.5, 0., 0.]).unwrap();
    let mut gpu = renderer(&scene);
    assert!(
        primitive_pixels(&gpu, &gpu.targets.primitive.texture)
            .iter()
            .all(|word| *word == 0)
    );
```

**Locate this block:**

```rust
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert!(
        primitive_pixels(&gpu, &gpu.primitive.texture)
            .iter()
            .all(|id| *id == 0)
    );
```

Replace it with:

```rust
    gpu.set_scene(&Scene::empty(), None);
    pixels(&gpu);
    assert!(
        primitive_pixels(&gpu, &gpu.targets.primitive.texture)
            .iter()
            .all(|id| *id == 0)
    );
```

**Locate this block:**

```rust
    });
    assert_eq!(
        [
            gpu.primitive.texture.width(),
            gpu.primitive.texture.height()
        ],
        [640, 480]
    );
```

Replace it with:

```rust
    });
    assert_eq!(
        [
            gpu.targets.primitive.texture.width(),
            gpu.targets.primitive.texture.height()
        ],
        [640, 480]
    );
```

**Locate this block:**

```rust
    .unwrap();
    gpu.set_scene(&instances, None);
    pixels(&gpu);
    let placed = primitive_pixels(&gpu, &gpu.primitive.texture);
    let ids = gpu
        .meshes
        .iter()
```

Replace it with:

```rust
    .unwrap();
    gpu.set_scene(&instances, None);
    pixels(&gpu);
    let placed = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
    let ids = gpu
        .meshes
        .iter()
```

**Locate this block:**

```rust
        .unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.primitive.texture),
            before,
            "A derived selected-face mesh must retain original triangle addresses"
        );
```

Replace it with:

```rust
        .unwrap();
        pixels(&gpu);
        assert_eq!(
            primitive_pixels(&gpu, &gpu.targets.primitive.texture),
            before,
            "A derived selected-face mesh must retain original triangle addresses"
        );
```

**Locate this block:**

```rust
        pixels(&gpu);
        let before = primitive_pixels(&gpu, &gpu.primitive.texture);
        assert_eq!(before[0], 0);
        assert_eq!(before[120 * 320 + 80], 2);
        assert_eq!(before[120 * 320 + 240], 3);
        assert_eq!(
            gpu.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        let key = *scene.objects()[0]
```

Replace it with:

```rust
        pixels(&gpu);
        let before = primitive_pixels(&gpu, &gpu.targets.primitive.texture);
        assert_eq!(before[0], 0);
        assert_eq!(before[120 * 320 + 80], 2);
        assert_eq!(before[120 * 320 + 240], 3);
        assert_eq!(
            gpu.targets.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        let key = *scene.objects()[0]
```

### 7. `src/contact_feature_tests.rs`

**Locate this existing block:**

```rust
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&r.depth),
                },
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&r.targets.depth),
                },
```

### 8. `src/ambient_input_tests.rs`

**Locate this existing block:**

```rust
        &normal.create_view(&Default::default()),
        &g.depth,
        &g.view_group,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        &normal.create_view(&Default::default()),
        &g.targets.depth,
        &g.view_group,
```

### 9. `src/ambient_contact_tests.rs`

**Locate this existing block:**

```rust
        &normals.create_view(&Default::default()),
        &g.depth,
        &g.view_group,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        &normals.create_view(&Default::default()),
        &g.targets.depth,
        &g.view_group,
```

### 10. `src/frame_target_tests.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[test]
fn identical_resize_retains_targets_geometry_sources_and_exact_single_sample_pixels() {
    use crate::{
        editor::{Action, Editor},
        viewport::Viewport,
    };
    let mut editor = Editor::default();
    editor
        .apply(Action::Replace(crate::specimen::bytes()))
        .unwrap();
    let source = editor.scene.objects()[0]
        .geometry()
        .unwrap()
        .clone();
    let weak = std::rc::Rc::downgrade(&source);
    let original = crate::archive::Bundle::decode(
        &crate::scene_archive::save(&editor.scene).unwrap(),
    )
    .unwrap();
    let documents = |b: &crate::archive::Bundle| {
        use prost::Message;
        b.documents
            .iter()
            .map(|d| session_rust::proto::Session::decode(d.as_slice()).unwrap())
            .collect::<Vec<_>>()
    };
    let before = documents(&original);
    let metadata = serde_json::to_value(&original.metadata).unwrap();
    let mut renderer = crate::cap_gpu_tests::renderer(&editor.scene);
    let geometry = renderer
        .geometry_owners()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(geometry.len(), editor.scene.objects().len());
    assert!(!geometry.is_empty());
    let mut camera = crate::camera::Camera::default();
    camera.aspect = 4. / 3.;
    camera.fit(&editor.scene.bounds().unwrap());
    let transform = renderer.camera_matrix(&camera);
    let baseline = crate::cap_gpu_tests::pixels_at(&renderer, transform);
    assert!(
        baseline
            .chunks_exact(4)
            .filter(|p| p[..3] != [255, 255, 255])
            .count()
            > 1000
    );
    export("original-frame-before", &baseline);
    let primitive = renderer.targets.primitive.texture.clone();
    let depth = renderer.targets.depth.clone();
    let stats = renderer.stats();
    let uploads = renderer.uploaded_bytes();
    for _ in 0..4 {
        renderer.resize(Viewport {
            width: 320,
            height: 240,
        });
    }
    assert_eq!(renderer.targets.depth, depth);
    assert_eq!(renderer.targets.primitive.texture, primitive);
    assert_eq!(renderer.targets.samples, 1);
    assert!(crate::cap_gpu_tests::pixels_at(&renderer, transform) == baseline);
    renderer.resize(Viewport {
        width: 640,
        height: 480,
    });
    assert_ne!(renderer.targets.depth, depth);
    assert_ne!(renderer.targets.primitive.texture, primitive);
    renderer.resize(Viewport {
        width: 320,
        height: 240,
    });
    assert!(crate::cap_gpu_tests::pixels_at(&renderer, transform) == baseline);
    export(
        "original-frame-after",
        &crate::cap_gpu_tests::pixels_at(&renderer, transform),
    );
    assert_eq!(
        renderer.geometry_owners().count(),
        geometry.len()
    );
    assert_eq!(renderer.stats(), stats);
    assert_eq!(renderer.uploaded_bytes(), uploads);
    assert!(
        renderer
            .geometry_owners()
            .zip(&geometry)
            .all(|(a, b)| std::rc::Rc::ptr_eq(a, b))
    );
    let after = crate::archive::Bundle::decode(
        &crate::scene_archive::save(&editor.scene).unwrap(),
    )
    .unwrap();
    assert!(documents(&after) == before);
    assert_eq!(
        serde_json::to_value(after.metadata).unwrap(),
        metadata
    );
    drop(source);
    drop(geometry);
    drop(renderer);
    drop(editor);
    assert!(weak.upgrade().is_none());
}

fn export(name: &str, image: &[u8]) {
    if let Some(root) = std::env::var_os("VIEWER_FRAME_TARGET_PROOF_DIR") {
        use std::io::Write;
        std::fs::create_dir_all(&root).unwrap();
        let mut file = std::fs::File::create(
            std::path::Path::new(&root).join(format!("{name}.ppm")),
        )
        .unwrap();
        write!(file, "P6\n320 240\n255\n").unwrap();
        for p in image.chunks_exact(4) {
            file.write_all(&p[..3]).unwrap();
        }
    }
}
```

## <a id="checkpoint-96zzzzzzw-match-resolve-base-and-view-formats"></a>3. Keep resolved colours in the canvas format

### 1. `src/frame_targets.rs`

**Locate this existing block:**

```rust
    ) -> Self {
        assert!(matches!(samples, 1 | 4) && !size.contains(&0));
        let image = |label, format| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: size[0],
                        height: size[1],
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        Self {
            depth: image("scene depth", crate::depth::FORMAT),
            primitive: crate::primitive_image::Image::with_samples(
                device, size, samples,
            ),
            color: (samples == 4).then(|| image("multisampled scene colour", format)),
            samples,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Self {
        Self::with_formats(device, size, format, format, samples)
    }

    pub fn with_formats(
        device: &wgpu::Device,
        size: [u32; 2],
        base: wgpu::TextureFormat,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert_eq!(
            base.remove_srgb_suffix(),
            format.remove_srgb_suffix()
        );
        assert!(matches!(samples, 1 | 4) && !size.contains(&0));
        let image = |label, base, format| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: size[0],
                        height: size[1],
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format: base,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[format],
                })
                .create_view(&wgpu::TextureViewDescriptor {
                    format: Some(format),
                    ..Default::default()
                })
        };
        Self {
            depth: image(
                "scene depth",
                crate::depth::FORMAT,
                crate::depth::FORMAT,
            ),
            primitive: crate::primitive_image::Image::with_samples(
                device, size, samples,
            ),
            color: (samples == 4)
                .then(|| image("multisampled scene colour", base, format)),
            samples,
```

### 2. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        self.cancel_pick();
        self.size = [size.width, size.height];
        self.targets = crate::frame_targets::Targets::new(
            &self.device,
            self.size,
            self.format,
```

Replace it with:

```rust
        }
        self.cancel_pick();
        self.size = [size.width, size.height];
        self.targets = crate::frame_targets::Targets::with_formats(
            &self.device,
            self.size,
            self.output_format,
            self.format,
```

**Locate this block:**

```rust
            text: std::cell::RefCell::new(None),
            text_error: std::cell::RefCell::new(None),
            format,
            device,
            queue,
            pipeline,
```

Replace it with:

```rust
            text: std::cell::RefCell::new(None),
            text_error: std::cell::RefCell::new(None),
            format,
            output_format,
            device,
            queue,
            pipeline,
```

**Locate this block:**

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        let targets =
            crate::frame_targets::Targets::new(&device, [640, 480], format, 1);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

Replace it with:

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        let targets = crate::frame_targets::Targets::with_formats(
            &device,
            [640, 480],
            output_format,
            format,
            1,
        );
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

**Locate this block:**

```rust
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(
            &device,
            crate::clipping_gpu::source(include_str!("cloud_points.wgsl"), 2, 0),
```

Replace it with:

```rust
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        Self::with_formats(device, queue, format, format, scene)
    }

    pub fn with_formats(
        device: wgpu::Device,
        queue: wgpu::Queue,
        output_format: wgpu::TextureFormat,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(
            &device,
            crate::clipping_gpu::source(include_str!("cloud_points.wgsl"), 2, 0),
```

**Locate this block:**

```rust
    format: wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances: crate::instance_gpu::Lane,
```

Replace it with:

```rust
    format: wgpu::TextureFormat,
    output_format: wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances: crate::instance_gpu::Lane,
```

### 3. `src/browser.rs`

**Locate this existing block:**

```rust
    let text_fonts = std::rc::Rc::new(crate::text_fonts::Inbox::default());
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let text_fonts = std::rc::Rc::new(crate::text_fonts::Inbox::default());
    let mut renderer = Renderer::with_formats(
        device,
        queue,
        config.format,
        config.format.add_srgb_suffix(),
```

### 4. `src/frame_target_tests.rs`

**Locate this existing block:**

```rust
    }));
    for samples in [1, 4] {
        let mut targets = Targets::new(
            &device,
            [8, 8],
            wgpu::TextureFormat::Rgba8Unorm,
            samples,
        );
        assert_eq!(
            targets.primitive.texture.sample_count(),
            samples
        );
        assert_eq!(
            targets.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        assert_eq!(
            targets
                .primitive
                .texture
                .usage()
                .contains(wgpu::TextureUsages::COPY_SRC),
            samples == 1
        );
        let output = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("resolved float colour"),
            size: wgpu::Extent3d {
                width: 8,
                height: 8,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let single = output.create_view(&Default::default());
        assert_eq!(
            targets.color(&single) == &single,
            samples == 1
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual distinct samples"),
            source: wgpu::ShaderSource::Wgsl(r#"
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4f {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3));return vec4f(p[i],0.5,1);
}
struct Hit { @location(0) color:vec4f, @location(1) primitive:vec2u, @builtin(frag_depth) depth:f32 }
@fragment fn fragment(@builtin(sample_index) sample:u32)->Hit {
 return Hit(vec4f(f32(sample&1u),f32((sample>>1u)&1u),0,1),vec2u(sample+1u,50000u+sample),f32(sample+1u)*0.125);
}"#.into()),
        });
        let pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("per-sample colour, integer and depth"),
            layout: None,
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
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(crate::primitive_image::target(true)),
                ],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: crate::depth::FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        });
        let storage = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let mut source = format!(
            r#"
@group(0) @binding(0) var depth:texture_depth_2d;
@group(0) @binding(1) var physical:texture_2d<u32>;
@group(0) @binding(2) var<storage,read_write> values:array<vec4u>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) id:vec3u) {{
 if(id.x>={samples}u){{return;}}
 let primitive=textureLoad(physical,vec2i(2),SAMPLE).xy;
 values[id.x]=vec4u(bitcast<u32>(textureLoad(depth,vec2i(2),SAMPLE)),primitive,id.x);
}}
"#
        );
        if samples == 4 {
            source = source
                .replace(
                    "texture_depth_2d",
                    "texture_depth_multisampled_2d",
                )
                .replace(
                    "texture_2d<u32>",
                    "texture_multisampled_2d<u32>",
                );
        }
        source = source.replace(
            "SAMPLE",
            if samples == 4 {
                "i32(id.x)"
            } else {
                "0"
            },
        );
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual sample readback"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let compute =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &module,
                entry_point: Some("read_samples"),
                compilation_options: Default::default(),
                cache: None,
            });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &compute.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&targets.depth),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &targets.primitive.view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: storage.as_entire_binding(),
                },
            ],
        });
        let result = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let pixels = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 256 * 8,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("actual matching sample attachments"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: targets.color(&single),
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &targets.primitive.view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &targets.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(0.),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                ..Default::default()
            });
            pass.set_pipeline(&pipe);
            pass.draw(0..3, 0..1);
        }
        targets.resolve(&mut encoder, &single);
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&compute);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
        encoder.copy_texture_to_buffer(
            output.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &pixels,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(8),
                },
            },
            output.size(),
        );
        queue.submit([encoder.finish()]);
        let actual = read(&device, &result);
        for sample in 0..samples as usize {
            let words = actual[sample * 16..sample * 16 + 16]
                .chunks_exact(4)
                .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            assert_eq!(
                f32::from_bits(words[0]),
                (sample + 1) as f32 * 0.125
            );
            assert_eq!(
                &words[1..],
                &[
                    sample as u32 + 1,
                    50000 + sample as u32,
                    sample as u32
                ]
            );
        }
        let actual = read(&device, &pixels);
        let expected = if samples == 4 {
            [128u8, 128, 0, 255]
        } else {
            [0, 0, 0, 255]
        };
        assert!(
            actual[2 * 256 + 2 * 4..2 * 256 + 2 * 4 + 4]
                .iter()
                .zip(expected)
                .all(|(&a, b)| a.abs_diff(b) <= 1)
        );
        let depth = targets.depth.clone();
        let primitive = targets.primitive.texture.clone();
        let other = if samples == 1 {
            4
        } else {
            1
        };
        targets = Targets::new(
            &device,
            [17, 13],
            wgpu::TextureFormat::Rgba8Unorm,
            other,
        );
        assert_eq!(
            targets.primitive.texture.sample_count(),
            other
        );
        assert_eq!(
            [
                targets.primitive.texture.width(),
                targets.primitive.texture.height()
            ],
            [17, 13]
        );
        assert_ne!(targets.depth, depth);
        assert_ne!(targets.primitive.texture, primitive);
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&compute);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
        queue.submit([encoder.finish()]);
        let retained = read(&device, &result);
        for sample in 0..samples as usize {
            let at = sample * 16;
            assert_eq!(
                f32::from_bits(u32::from_ne_bytes(
                    retained[at..at + 4].try_into().unwrap()
                )),
                (sample + 1) as f32 * 0.125
            );
            assert_eq!(
                u32::from_ne_bytes(retained[at + 4..at + 8].try_into().unwrap()),
                sample as u32 + 1
            );
            assert_eq!(
                u32::from_ne_bytes(retained[at + 8..at + 12].try_into().unwrap()),
                50000 + sample as u32
            );
        }
        targets = Targets::new(
            &device,
            [8, 8],
            wgpu::TextureFormat::Rgba8Unorm,
            samples,
        );
        assert_eq!(
            targets.primitive.texture.sample_count(),
            samples
        );
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }));
    for (base, format) in [
        (
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Rgba8Unorm,
        ),
        (
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        ),
        (
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        ),
    ] {
        for samples in [1, 4] {
            let mut targets =
                Targets::with_formats(&device, [8, 8], base, format, samples);
            assert_eq!(
                targets.primitive.texture.sample_count(),
                samples
            );
            assert_eq!(
                targets.primitive.texture.format(),
                crate::primitive_image::FORMAT
            );
            assert_eq!(
                targets
                    .primitive
                    .texture
                    .usage()
                    .contains(wgpu::TextureUsages::COPY_SRC),
                samples == 1
            );
            let output = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("resolved float colour"),
                size: wgpu::Extent3d {
                    width: 8,
                    height: 8,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: base,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[format],
            });
            let single = output.create_view(&wgpu::TextureViewDescriptor {
                format: Some(format),
                ..Default::default()
            });
            assert_eq!(
                targets.color(&single) == &single,
                samples == 1
            );
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual distinct samples"),
            source: wgpu::ShaderSource::Wgsl(r#"
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4f {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3));return vec4f(p[i],0.5,1);
}
struct Hit { @location(0) color:vec4f, @location(1) primitive:vec2u, @builtin(frag_depth) depth:f32 }
@fragment fn fragment(@builtin(sample_index) sample:u32)->Hit {
 return Hit(vec4f(f32(sample&1u),f32((sample>>1u)&1u),0,1),vec2u(sample+1u,50000u+sample),f32(sample+1u)*0.125);
}"#.into()),
        });
            let pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("per-sample colour, integer and depth"),
                layout: None,
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
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(crate::primitive_image::target(true)),
                    ],
                }),
                primitive: Default::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: crate::depth::FORMAT,
                    depth_write_enabled: Some(true),
                    depth_compare: Some(wgpu::CompareFunction::Always),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: samples,
                    ..Default::default()
                },
                multiview_mask: None,
                cache: None,
            });
            let storage = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: 64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });
            let mut source = format!(
                r#"
@group(0) @binding(0) var depth:texture_depth_2d;
@group(0) @binding(1) var physical:texture_2d<u32>;
@group(0) @binding(2) var<storage,read_write> values:array<vec4u>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) id:vec3u) {{
 if(id.x>={samples}u){{return;}}
 let primitive=textureLoad(physical,vec2i(2),SAMPLE).xy;
 values[id.x]=vec4u(bitcast<u32>(textureLoad(depth,vec2i(2),SAMPLE)),primitive,id.x);
}}
"#
            );
            if samples == 4 {
                source = source
                    .replace(
                        "texture_depth_2d",
                        "texture_depth_multisampled_2d",
                    )
                    .replace(
                        "texture_2d<u32>",
                        "texture_multisampled_2d<u32>",
                    );
            }
            source = source.replace(
                "SAMPLE",
                if samples == 4 {
                    "i32(id.x)"
                } else {
                    "0"
                },
            );
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("actual sample readback"),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });
            let compute =
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &module,
                    entry_point: Some("read_samples"),
                    compilation_options: Default::default(),
                    cache: None,
                });
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &compute.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&targets.depth),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &targets.primitive.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: storage.as_entire_binding(),
                    },
                ],
            });
            let result = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: 64,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let pixels = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: 256 * 8,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let mut encoder = device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("actual matching sample attachments"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: targets.color(&single),
                            resolve_target: None,
                            depth_slice: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                store: wgpu::StoreOp::Store,
                            },
                        }),
                        Some(wgpu::RenderPassColorAttachment {
                            view: &targets.primitive.view,
                            resolve_target: None,
                            depth_slice: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                                store: wgpu::StoreOp::Store,
                            },
                        }),
                    ],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: &targets.depth,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(0.),
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        },
                    ),
                    ..Default::default()
                });
                pass.set_pipeline(&pipe);
                pass.draw(0..3, 0..1);
            }
            targets.resolve(&mut encoder, &single);
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&compute);
                pass.set_bind_group(0, &group, &[]);
                pass.dispatch_workgroups(1, 1, 1);
            }
            encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
            encoder.copy_texture_to_buffer(
                output.as_image_copy(),
                wgpu::TexelCopyBufferInfo {
                    buffer: &pixels,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(256),
                        rows_per_image: Some(8),
                    },
                },
                output.size(),
            );
            queue.submit([encoder.finish()]);
            let actual = read(&device, &result);
            for sample in 0..samples as usize {
                let words = actual[sample * 16..sample * 16 + 16]
                    .chunks_exact(4)
                    .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                    .collect::<Vec<_>>();
                assert_eq!(
                    f32::from_bits(words[0]),
                    (sample + 1) as f32 * 0.125
                );
                assert_eq!(
                    &words[1..],
                    &[
                        sample as u32 + 1,
                        50000 + sample as u32,
                        sample as u32
                    ]
                );
            }
            let actual = read(&device, &pixels);
            let expected = if samples == 4 {
                let midpoint = if format.is_srgb() {
                    188
                } else {
                    128
                };
                [midpoint, midpoint, 0, 255]
            } else {
                [0, 0, 0, 255]
            };
            assert!(
                actual[2 * 256 + 2 * 4..2 * 256 + 2 * 4 + 4]
                    .iter()
                    .zip(expected)
                    .all(|(&a, b)| a.abs_diff(b) <= 1)
            );
            let depth = targets.depth.clone();
            let primitive = targets.primitive.texture.clone();
            let other = if samples == 1 {
                4
            } else {
                1
            };
            targets = Targets::with_formats(&device, [17, 13], base, format, other);
            assert_eq!(
                targets.primitive.texture.sample_count(),
                other
            );
            assert_eq!(
                [
                    targets.primitive.texture.width(),
                    targets.primitive.texture.height()
                ],
                [17, 13]
            );
            assert_ne!(targets.depth, depth);
            assert_ne!(targets.primitive.texture, primitive);
            let mut encoder = device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&compute);
                pass.set_bind_group(0, &group, &[]);
                pass.dispatch_workgroups(1, 1, 1);
            }
            encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
            queue.submit([encoder.finish()]);
            let retained = read(&device, &result);
            for sample in 0..samples as usize {
                let at = sample * 16;
                assert_eq!(
                    f32::from_bits(u32::from_ne_bytes(
                        retained[at..at + 4].try_into().unwrap()
                    )),
                    (sample + 1) as f32 * 0.125
                );
                assert_eq!(
                    u32::from_ne_bytes(retained[at + 4..at + 8].try_into().unwrap()),
                    sample as u32 + 1
                );
                assert_eq!(
                    u32::from_ne_bytes(retained[at + 8..at + 12].try_into().unwrap()),
                    50000 + sample as u32
                );
            }
            targets = Targets::with_formats(&device, [8, 8], base, format, samples);
            assert_eq!(
                targets.primitive.texture.sample_count(),
                samples
            );
        }
    }
```

## <a id="checkpoint-96zzzzzzx-reuse-count-specific-surface-pipelines"></a>4. Reuse surface pipelines for each sample count

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

### 1. `src/sample_pipelines.rs`

Create the file and type:

```rust
pub struct Variants<T> {
    pub active: T,
    pub alternate: Option<T>,
    pub samples: u32,
}
impl<T> Variants<T> {
    pub fn new(active: T, samples: u32) -> Self {
        assert!(matches!(samples, 1 | 4));
        Self {
            active,
            alternate: None,
            samples,
        }
    }
    pub fn select(&mut self, samples: u32, create: impl FnOnce() -> T) -> bool {
        assert!(matches!(samples, 1 | 4));
        if self.samples == samples {
            return false;
        }
        let alternate = self.alternate.get_or_insert_with(create);
        std::mem::swap(&mut self.active, alternate);
        self.samples = samples;
        true
    }
}
```

### 2. `src/surface_pipelines.rs`

Create the file and type:

```rust
pub fn pipelines(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    view_layout: &wgpu::BindGroupLayout,
    model_layout: &wgpu::BindGroupLayout,
    samples: u32,
) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("triangle"),
        source: wgpu::ShaderSource::Wgsl(
            sampled(
                crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0),
                samples,
            )
            .into(),
        ),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("mesh settings"),
        bind_group_layouts: &[
            Some(view_layout),
            Some(model_layout),
            Some(&crate::clipping_gpu::layout(device)),
            Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device)),
        ],
        immediate_size: 0,
    });
    let mesh_pipeline = |depth, primitive_write| {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
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
            multisample: wgpu::MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        })
    };
    (
        mesh_pipeline(crate::depth::surface(), true),
        mesh_pipeline(crate::depth::ink(), false),
    )
}

pub fn sampled(source: String, samples: u32) -> String {
    if samples == 1 {
        return source;
    }
    source
        .replace(
            "@location(0) colour:",
            "@location(0) @interpolate(perspective,sample) colour:",
        )
        .replace(
            "@location(1) world:",
            "@location(1) @interpolate(perspective,sample) world:",
        )
        .replace(
            "@location(2) normal:",
            "@location(2) @interpolate(perspective,sample) normal:",
        )
}
```

### 3. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            if self.opacity.get() >= 1. {
                self.instances.draw(&mut pass);
            }
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
```

Replace it with:

```rust
            if self.opacity.get() >= 1. {
                self.instances.draw(&mut pass);
            }
            pass.set_pipeline(&self.surfaces.active.1);
            pass.set_bind_group(0, &self.view_group, &[]);
```

**Locate this block:**

```rust
            });
            self.grid.draw(&mut pass, &self.view_group);
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
                if !self.instances.draw_ordinary(&mut pass, mesh) {
                    pass.set_pipeline(&self.pipeline);
                    self.instances.bind_own(&mut pass);
                    mesh.draw_pulled(&mut pass);
                }
```

Replace it with:

```rust
            });
            self.grid.draw(&mut pass, &self.view_group);
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.surfaces.active.0);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
                if !self.instances.draw_ordinary(&mut pass, mesh) {
                    pass.set_pipeline(&self.surfaces.active.0);
                    self.instances.bind_own(&mut pass);
                    mesh.draw_pulled(&mut pass);
                }
```

**Locate this block:**

```rust
        let anchor = self.anchor();
        self.highlight.set_at(
            &self.device,
            &self.pipeline.get_bind_group_layout(1),
            scene,
            target,
            anchor,
```

Replace it with:

```rust
        let anchor = self.anchor();
        self.highlight.set_at(
            &self.device,
            &self
                .surfaces
                .active
                .0
                .get_bind_group_layout(1),
            scene,
            target,
            anchor,
```

**Locate this block:**

```rust
        self.highlight
            .set(
                &self.device,
                &self.pipeline.get_bind_group_layout(1),
                scene,
                None,
            )
            .expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
```

Replace it with:

```rust
        self.highlight
            .set(
                &self.device,
                &self
                    .surfaces
                    .active
                    .0
                    .get_bind_group_layout(1),
                scene,
                None,
            )
            .expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self
            .surfaces
            .active
            .0
            .get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene
            .objects()
```

**Locate this block:**

```rust
            output_format,
            device,
            queue,
            pipeline,
            highlight_pipeline,
            highlight,
            preview,
            widget,
```

Replace it with:

```rust
            output_format,
            device,
            queue,
            surfaces,
            highlight,
            preview,
            widget,
```

**Locate this block:**

```rust
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
```

Replace it with:

```rust
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &surfaces.active.0.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
```

**Locate this block:**

```rust
            crate::finite_project_source::scan(),
            "Finite scan",
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(include_str!("triangle.wgsl"), 2, 0).into(),
            ),
        });
        let view_layout = crate::surface_view::layout(&device);
        let model_layout =
            crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"),
            bind_group_layouts: &[
                Some(&view_layout),
                Some(&model_layout),
                Some(&crate::clipping_gpu::layout(&device)),
                Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device)),
            ],
            immediate_size: 0,
        });
        let mesh_pipeline = |depth, primitive_write| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("triangle"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[crate::draw_slots::layout()],
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
        let instances =
            crate::instance_gpu::Lane::new(&device, &queue, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(), false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
```

Replace it with:

```rust
            crate::finite_project_source::scan(),
            "Finite scan",
        );
        let view_layout = crate::surface_view::layout(&device);
        let model_layout =
            crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let surfaces = crate::sample_pipelines::Variants::new(
            crate::surface_pipelines::pipelines(
                &device,
                format,
                &view_layout,
                &model_layout,
                1,
            ),
            1,
        );
        let tiles = std::cell::RefCell::new(crate::tile_gpu::Tiles::new(
            &device,
            &view_layout,
        ));
        let instances =
            crate::instance_gpu::Lane::new(&device, &queue, format, &view_layout);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
```

**Locate this block:**

```rust
    instance_rows: std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
```

Replace it with:

```rust
    instance_rows: std::collections::HashSet<ObjectId>,
    surfaces:
        crate::sample_pipelines::Variants<(wgpu::RenderPipeline, wgpu::RenderPipeline)>,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
```

### 4. `src/instance_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.slots.bind(pass, 0, 0);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
```

Replace it with:

```rust
        self.slots.bind(pass, 0, 0);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipelines.active.0);
        self.records.bind(pass);
```

**Locate this block:**

```rust
        pass: &mut wgpu::RenderPass<'_>,
        mesh: &GpuMesh,
    ) -> bool {
        self.draw_with(pass, mesh, &self.ordinary_pipeline)
    }
    pub fn draw_with(
        &self,
```

Replace it with:

```rust
        pass: &mut wgpu::RenderPass<'_>,
        mesh: &GpuMesh,
    ) -> bool {
        self.draw_with(pass, mesh, &self.pipelines.active.1)
    }
    pub fn draw_with(
        &self,
```

**Locate this block:**

```rust
            written_bytes: 8,
        }
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
```

Replace it with:

```rust
            written_bytes: 8,
        }
    }
    pub fn set_samples(
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        samples: u32,
    ) {
        self.pipelines.select(samples, || {
            pipelines(
                device,
                format,
                &crate::surface_view::layout(device),
                &self.layout,
                samples,
            )
        });
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
```

**Locate this block:**

```rust
        Self {
            lookup: None,
            space: None,
            pipeline,
            ordinary_pipeline,
            layout,
            batches: BTreeMap::new(),
            records,
```

Replace it with:

```rust
        Self {
            lookup: None,
            space: None,
            pipelines,
            layout,
            batches: BTreeMap::new(),
            records,
```

**Locate this block:**

```rust
        view: &wgpu::BindGroupLayout,
    ) -> Self {
        let layout = crate::object_rows::layout(device);
        let source = source();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shared definition placements"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("shared instance settings"),
                bind_group_layouts: &[
                    Some(view),
                    Some(&layout),
                    Some(&crate::clipping_gpu::layout(device)),
                    Some(&GpuGeometry::storage_layout(device)),
                ],
                immediate_size: 0,
            });
        let create = |blend| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("shared original definition drawing"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[crate::draw_slots::layout()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format,
                            blend,
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(crate::primitive_image::target(true)),
                    ],
                }),
                primitive: Default::default(),
                depth_stencil: Some(crate::depth::surface()),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = create(None);
        let ordinary_pipeline = create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records = crate::object_rows::Rows::new(
            &crate::text_gpu_support::GpuCtx::new(device, queue),
            &layout,
```

Replace it with:

```rust
        view: &wgpu::BindGroupLayout,
    ) -> Self {
        let layout = crate::object_rows::layout(device);
        let pipelines = crate::sample_pipelines::Variants::new(
            pipelines(device, format, view, &layout, 1),
            1,
        );
        let records = crate::object_rows::Rows::new(
            &crate::text_gpu_support::GpuCtx::new(device, queue),
            &layout,
```

**Locate this block:**

```rust
        0,
    )
}
impl Lane {
    pub fn new(
        device: &wgpu::Device,
```

Replace it with:

```rust
        0,
    )
}
fn pipelines(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    view: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
    samples: u32,
) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {
    let source = crate::surface_pipelines::sampled(source(), samples);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("shared definition placements"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("shared instance settings"),
            bind_group_layouts: &[
                Some(view),
                Some(layout),
                Some(&crate::clipping_gpu::layout(device)),
                Some(&GpuGeometry::storage_layout(device)),
            ],
            immediate_size: 0,
        });
    let create = |blend| {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shared original definition drawing"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format,
                        blend,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(crate::primitive_image::target(true)),
                ],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()),
            multisample: wgpu::MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        })
    };
    (
        create(None),
        create(Some(wgpu::BlendState::ALPHA_BLENDING)),
    )
}
impl Lane {
    pub fn new(
        device: &wgpu::Device,
```

**Locate this block:**

```rust
    pub(crate) space: Option<crate::slot_space::SlotSpace>,
    pipeline: wgpu::RenderPipeline,
    ordinary_pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    batches: BTreeMap<usize, Batch>,
    records: crate::object_rows::Rows,
```

Replace it with:

```rust
    pub(crate) space: Option<crate::slot_space::SlotSpace>,
    pipelines:
        crate::sample_pipelines::Variants<(wgpu::RenderPipeline, wgpu::RenderPipeline)>,
    layout: wgpu::BindGroupLayout,
    batches: BTreeMap<usize, Batch>,
    records: crate::object_rows::Rows,
```

### 5. `src/global_outline_tests.rs`

**Locate this existing block:**

```rust
            .unwrap();
        let layout = r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .unwrap();
        let layout = r.surfaces.active.0.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
```

### 6. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod camera_navigation_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod camera_navigation_tests;

mod sample_pipelines;
mod surface_pipelines;
```

### 7. `src/global_outline_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    far.translate(owner, anchor).unwrap();
    let layout = gpu.pipeline.get_bind_group_layout(1);
    gpu.meshes[0] = crate::gpu_mesh::GpuMesh::upload_at(
```

Replace it with:

```rust
    let anchor = [1e9, -1e9, 1e9];
    let mut far = scene.clone();
    far.translate(owner, anchor).unwrap();
    let layout = gpu
        .surfaces
        .active
        .0
        .get_bind_group_layout(1);
    gpu.meshes[0] = crate::gpu_mesh::GpuMesh::upload_at(
```

**Locate this block:**

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    let layout = gpu.pipeline.get_bind_group_layout(1);
    gpu.meshes[0] = crate::gpu_mesh::GpuMesh::upload(
        &gpu.device,
        &layout,
```

Replace it with:

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    let layout = gpu
        .surfaces
        .active
        .0
        .get_bind_group_layout(1);
    gpu.meshes[0] = crate::gpu_mesh::GpuMesh::upload(
        &gpu.device,
        &layout,
```

**Locate this block:**

```rust
        &gpu.device,
        &gpu.pipeline.get_bind_group_layout(0),
    );
    let output = |size| {
        gpu.device
```

Replace it with:

```rust
        &gpu.device,
        &gpu.surfaces
            .active
            .0
            .get_bind_group_layout(0),
    );
    let output = |size| {
        gpu.device
```

### 8. `src/sample_surface_tests.rs`

Create the file and type:

```rust
use super::*;
use std::rc::Rc;
fn surface_frame(
    r: &Renderer,
    target: &crate::frame_targets::Targets,
    shared: bool,
) -> Vec<u8> {
    let output = r
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("actual multisampled original surfaces"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: r.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    let view = output.create_view(&Default::default());
    let matrix = crate::cap_gpu_tests::reversed(
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    );
    r.queue.write_buffer(
        &r.uniform,
        0,
        &crate::surface_view::bytes(matrix, [0., 1., 0., 0.], [320, 240], 1.),
    );
    r.clipping
        .borrow_mut()
        .frame_anchored(&r.queue, matrix, r.anchor(), 240, 1.);
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("original and shared surface count witness"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: target.color(&view),
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &target.primitive.view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_bind_group(0, &r.view_group, &[]);
        pass.set_bind_group(2, &r.clipping.borrow().group, &[]);
        for mesh in r
            .meshes
            .iter()
            .filter(|mesh| !shared || !r.instance_rows.contains(&mesh.id))
        {
            pass.set_pipeline(&r.surfaces.active.0);
            r.instances.bind_own(&mut pass);
            mesh.draw_pulled(&mut pass);
        }
        if shared {
            r.instances.draw(&mut pass);
        }
    }
    target.resolve(&mut encoder, &view);
    let buffer = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 320 * 240 * 4,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    encoder.copy_texture_to_buffer(
        output.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280),
                rows_per_image: Some(240),
            },
        },
        output.size(),
    );
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    crate::cap_gpu_tests::wait(r);
    rx.recv().unwrap().unwrap();
    let pixels = buffer.slice(..).get_mapped_range().to_vec();
    buffer.unmap();
    pixels
}
fn actual_physical_samples(
    r: &Renderer,
    target: &crate::frame_targets::Targets,
) -> usize {
    let module=r.device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("read every physical original sample"),source:wgpu::ShaderSource::Wgsl(r#"
@group(0) @binding(0) var primitive:texture_multisampled_2d<u32>;
@group(0) @binding(1) var depth:texture_depth_multisampled_2d;
@group(0) @binding(2) var<storage,read_write> output:array<vec4u>;
@compute @workgroup_size(64) fn read_samples(@builtin(global_invocation_id) id:vec3u) {
 if(id.x>=320u*240u*4u){return;}let p=id.x/4u;let at=vec2i(i32(p%320u),i32(p/320u));let sample=id.x%4u;
 output[id.x]=vec4u(textureLoad(primitive,at,i32(sample)).xy,bitcast<u32>(textureLoad(depth,at,i32(sample))),sample);
}"#.into())});
    let pipeline = r
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("read_samples"),
            compilation_options: Default::default(),
            cache: None,
        });
    let size = 320 * 240 * 4 * 16;
    let output = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
    let read = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let group = r
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &target.primitive.view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&target.depth),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: output.as_entire_binding(),
                },
            ],
        });
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups((320 * 240 * 4u32).div_ceil(64), 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &read, 0, size);
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    crate::cap_gpu_tests::wait(r);
    rx.recv().unwrap().unwrap();
    let bytes = read.slice(..).get_mapped_range();
    let mut mixed = 0;
    for pixel in bytes.chunks_exact(64) {
        let mut addresses = [0; 4];
        for (i, sample) in pixel.chunks_exact(16).enumerate() {
            let words = std::array::from_fn::<_, 4, _>(|j| {
                u32::from_ne_bytes(sample[j * 4..j * 4 + 4].try_into().unwrap())
            });
            assert_eq!(words[3], i as u32);
            let address = words[0] | (words[1] << 16);
            let depth = f32::from_bits(words[2]);
            assert_eq!(
                depth,
                if address == 0 {
                    0.
                } else {
                    1.
                }
            );
            addresses[i] = address;
        }
        if addresses.iter().any(|a| *a == 0) && addresses.iter().any(|a| *a != 0) {
            mixed += 1;
        }
    }
    drop(bytes);
    read.unmap();
    assert!(
        mixed > 100,
        "Actual original silhouettes have mixed covered and uncovered samples"
    );
    mixed
}
#[test]
fn original_and_shared_meshes_switch_samples_without_source_or_geometry_rebuilds() {
    let mut scene = Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &crate::instance_specimen::display_bytes(),
        "",
        &session_rust::Xform::identity(),
        0.,
    )
    .unwrap();
    scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [-0.3, -0.85, 0.],
                    [0.3, -0.85, 0.],
                    [0., -0.3, 0.],
                ],
                [0.1, 0.8, 0.3],
            )
            .unwrap(),
        )
        .unwrap();
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    let geometry = r
        .meshes
        .iter()
        .map(|m| Rc::clone(&m.geometry))
        .collect::<Vec<_>>();
    let sources = scene
        .objects()
        .iter()
        .map(|o| o.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let stats = r.stats();
    let usage = r.instance_usage();
    let one = crate::frame_targets::Targets::new(&r.device, [320, 240], r.format, 1);
    let four = crate::frame_targets::Targets::new(&r.device, [320, 240], r.format, 4);
    let first = surface_frame(&r, &one, true);
    assert!(
        first
            .chunks_exact(4)
            .filter(|p| p[..3] != [255; 3])
            .count()
            > 5000
    );
    let surface1 = r.surfaces.active.0.clone();
    r.surfaces.select(4, || {
        crate::surface_pipelines::pipelines(
            &r.device,
            r.format,
            &crate::surface_view::layout(&r.device),
            &crate::gpu_layout::uniform(&r.device, crate::object_record::BYTES as u64),
            4,
        )
    });
    r.instances
        .set_samples(&r.device, r.format, 4);
    let surface4 = r.surfaces.active.0.clone();
    assert_ne!(surface1, surface4);
    let shared = surface_frame(&r, &four, true);
    assert!(
        surface_frame(&r, &four, false) == shared,
        "Original uniforms and shared records have identical4x colour"
    );
    let mixed = actual_physical_samples(&r, &four);
    assert!(
        first != shared,
        "Actual colour changes on silhouette coverage"
    );
    for count in [4, 1, 4] {
        r.surfaces.select(count, || {
            panic!("A retained surface variant must not compile again")
        });
        r.instances
            .set_samples(&r.device, r.format, count);
        let pixels = surface_frame(
            &r,
            if count == 1 {
                &one
            } else {
                &four
            },
            true,
        );
        assert!(
            pixels
                == if count == 1 {
                    first.clone()
                } else {
                    shared.clone()
                }
        );
        assert_eq!(
            r.surfaces.active.0,
            if count == 1 {
                surface1.clone()
            } else {
                surface4.clone()
            }
        );
    }
    let matrix = crate::cap_gpu_tests::reversed(
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    );
    for (i, x) in [-0.6, 0.6].into_iter().enumerate() {
        r.request_pick([x, 0.], matrix).unwrap();
        crate::cap_gpu_tests::wait(&r);
        let pick = r
            .take_pick(matrix)
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(
            pick[0],
            scene.objects()[i].id.value(),
            "Fresh owner queries remain single-sample after surface4x switching"
        );
    }
    assert_eq!(r.stats(), stats);
    assert_eq!(r.instance_usage(), usage);
    for ((gpu, before), source) in r.meshes.iter().zip(&geometry).zip(&sources) {
        assert!(Rc::ptr_eq(&gpu.geometry, before));
        let row = scene
            .objects()
            .iter()
            .find(|o| o.id == gpu.id)
            .unwrap();
        assert_eq!(&row.geometry().unwrap().to_proto(), source);
    }
    let root = std::path::Path::new("surface-pixels");
    crate::sample_test_proof::write(root.join("one.rgba"), first).unwrap();
    crate::sample_test_proof::write(root.join("four.rgba"), shared).unwrap();
    crate::sample_test_proof::write(
        root.join("physical.json"),
        format!("{{\"actualMixedCoveragePixels\":{mixed}}}\n"),
    )
    .unwrap();
}
```

### 9. `src/sample_test_proof.rs`

Create the file and type:

```rust
use std::{
    io,
    path::{Path, PathBuf},
};

pub fn write(path: impl AsRef<Path>, bytes: impl AsRef<[u8]>) -> io::Result<()> {
    let Some(root) = std::env::var_os("VIEWER_SAMPLE_PROOF_DIR") else {
        return Ok(());
    };
    let path = PathBuf::from(root).join(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, bytes)
}
```

### 10. `src/renderer.rs`

**Locate this existing block:**

```rust
#[path = "frame_target_tests.rs"]
mod frame_target_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "frame_target_tests.rs"]
mod frame_target_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "sample_surface_tests.rs"]
mod sample_tests;
```

### 11. `src/lib.rs`

**Locate this existing block:**

```rust
mod sample_pipelines;
mod surface_pipelines;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod sample_pipelines;
mod surface_pipelines;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod sample_test_proof;
```

## <a id="checkpoint-96zzzzzzy-generate-ink-for-physical-samples"></a>5. Generate ink shaders for physical samples

### 1. `src/ink_plane.wgsl`

**Locate this existing block:**

```wgsl
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis)->bool{
 let fringe=ink_primitive(pixel,0u);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),0u);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis,sample:u32)->bool{
 let fringe=ink_primitive(pixel,sample);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),sample);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
```

### 2. `src/finite_visibility.wgsl`

**Locate this existing block:**

```wgsl
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
 let primitive=ink_primitive(pixel,0u);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool,sag:vec2<f32>)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags,sag);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32,sag:vec2<f32>)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 var axis=finite_ink_axis(pixel,ends,depths);axis.sag=sag;let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,0u))){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,0u);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis)){return false;}
 if(plane_visible){return true;}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32,sample:u32)->bool{
 let primitive=ink_primitive(pixel,sample);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32,sample:u32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,sample:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative,sample)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative,sample);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative,sample);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative,sample);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool,sag:vec2<f32>,sample:u32)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags,sag,sample);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool,sample:u32)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance,sample)||!ink_disc_visible(pixel,centre,depth,sample);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32,sag:vec2<f32>,sample:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags,sample);}
 var axis=finite_ink_axis(pixel,ends,depths);axis.sag=sag;let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance,sample)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,sample);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis,sample)){return false;}
 if(plane_visible){return true;}
```

### 3. `src/finite_stroke.wgsl`

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.),sample);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 4. `src/finite_chain.wgsl`

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.),sample);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 5. `src/finite_marker.wgsl`

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.position.xy,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input.colour,input.radius,input.position.xy-input.finite_axis_value.xy);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.position.xy,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false,sample);if(alpha<=0.){discard;}let c=colour(input.colour,input.radius,input.position.xy-input.finite_axis_value.xy);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 6. `src/finite_stroke_pick.wgsl`

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.))<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.),0u)<=0.){discard;}
    return input.id;
```

### 7. `src/finite_chain_pick.wgsl`

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(0.))<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(0.),0u)<=0.){discard;}
    return input.id;
```

### 8. `src/finite_marker_pick.wgsl`

**Locate this existing block:**

```wgsl
    if (colour(input.colour,input.radius,pixel-input.finite_axis_value.xy).a <= 0.0) { discard; }
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(input.colour,input.radius,pixel-input.finite_axis_value.xy).a <= 0.0) { discard; }
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true,0u)<=0.){discard;}
    return input.id;
```

### 9. `src/finite_sources.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        0,
    )
}
```

Replace it with:

```rust
        0,
    )
}

fn multisampled(mut source: String, stride: u64) -> String {
    source = source
        .replace(
            "var finite_depth:texture_depth_2d;",
            "var finite_depth:texture_depth_multisampled_2d;",
        )
        .replace(
            "var finite_primitive:texture_2d<u32>;",
            "var finite_primitive:texture_multisampled_2d<u32>;",
        )
        .replace(
            "textureLoad(finite_depth,at,0)",
            "textureLoad(finite_depth,at,i32(sample))",
        )
        .replace(
            "textureLoad(finite_primitive,at,0)",
            "textureLoad(finite_primitive,at,i32(sample))",
        )
        .replace(
            "@interpolate(linear) finite_pixel:",
            "@interpolate(linear,sample) finite_pixel:",
        )
        .replace(
            "@interpolate(linear) pixel:",
            "@interpolate(linear,sample) pixel:",
        );
    if stride == 32 {
        source=source.replace("@location(10) @interpolate(flat) finite_depths:vec2<f32>,",
            "@location(10) @interpolate(flat) finite_depths:vec2<f32>,\n    @location(11) @interpolate(linear,sample) finite_pixel:vec2<f32>,")
            .replace("colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0))", "colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0))")
            .replace("Output(position, colour, radius,flags,axis,depths)","Output(position, colour, radius,flags,axis,depths,finite_screen(position))")
            .replace("input.position.xy","input.finite_pixel");
    }
    source
}
```

**Locate this block:**

```rust
            "#include \"ink_plane.wgsl\"",
            include_str!("ink_plane.wgsl"),
        );
    let base = if grid {
        base.replace(
            "const FINITE_GRID:bool=false;",
```

Replace it with:

```rust
            "#include \"ink_plane.wgsl\"",
            include_str!("ink_plane.wgsl"),
        );
    let base = if samples == 4 {
        multisampled(base, stride)
    } else {
        base
    };
    let base = if grid {
        base.replace(
            "const FINITE_GRID:bool=false;",
```

**Locate this block:**

```rust
pub fn source_metadata(stride: u64, pick: bool, grid: bool, varying: bool) -> String {
    let (base, ids) = match stride {
        44 => (
            include_str!("finite_stroke.wgsl"),
```

Replace it with:

```rust
pub fn source_metadata(stride: u64, pick: bool, grid: bool, varying: bool) -> String {
    source_with_samples(stride, pick, grid, varying, 1)
}
pub fn source_with_samples(
    stride: u64,
    pick: bool,
    grid: bool,
    varying: bool,
    samples: u32,
) -> String {
    assert!(matches!(samples, 1 | 4) && (!pick || samples == 1));
    let (base, ids) = match stride {
        44 => (
            include_str!("finite_stroke.wgsl"),
```

### 10. `src/tile_shader_tests.rs`

**Locate this existing block:**

```rust
    ];
    for stride in [44, 72, 32] {
        sources.push((
            "finite color",
            crate::finite_sources::source(stride, false),
        ));
        sources.push((
            "finite pick",
            crate::finite_sources::source(stride, true),
        ));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ];
    for (stride, grid, varying) in [
        (44, false, false),
        (72, false, false),
        (32, false, false),
        (44, true, false),
        (44, false, true),
        (32, false, true),
    ] {
        for samples in [1, 4] {
            sources.push((
                "finite color",
                crate::finite_sources::source_with_samples(
                    stride, false, grid, varying, samples,
                ),
            ));
        }
        sources.push((
            "finite pick",
            crate::finite_sources::source_metadata(stride, true, grid, varying),
        ));
```

### 11. `src/finite_ink.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            grid,
            true,
            varying,
            &view,
```

Replace it with:

```rust
            grid,
            true,
            varying,
            1,
            &view,
```

**Locate this block:**

```rust
            grid,
            false,
            varying,
            &view,
            &crop,
            &layout,
```

Replace it with:

```rust
            grid,
            false,
            varying,
            1,
            &view,
            &crop,
            &layout,
```

**Locate this block:**

```rust
        }),
        primitive: Default::default(),
        depth_stencil: Some(depth),
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
```

Replace it with:

```rust
        }),
        primitive: Default::default(),
        depth_stencil: Some(depth),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
```

**Locate this block:**

```rust
    varying: bool,
    view: &wgpu::BindGroupLayout,
    crop: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
    attributes: &[wgpu::VertexAttribute],
) -> wgpu::RenderPipeline {
    let source = crate::finite_sources::source_metadata(stride, picking, grid, varying);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("finite original ink"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
```

Replace it with:

```rust
    varying: bool,
    samples: u32,
    view: &wgpu::BindGroupLayout,
    crop: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
    attributes: &[wgpu::VertexAttribute],
) -> wgpu::RenderPipeline {
    let source = crate::finite_sources::source_with_samples(
        stride, picking, grid, varying, samples,
    );
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("finite original ink"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
```

## <a id="checkpoint-96zzzzzzz-cache-ink-pipelines-with-independent-queries"></a>6. Cache matching ink pipelines and bindings

### 1. `src/stroke_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            include_str!("grid.wgsl"),
            44,
            2,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
            crate::depth::grid(),
```

Replace it with:

```rust
            include_str!("grid.wgsl"),
            44,
            2,
            attributes(44),
            crate::depth::grid(),
```

**Locate this block:**

```rust
            &crate::stroke::shader(),
            44,
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],
            depth,
            true,
        )
```

Replace it with:

```rust
            &crate::stroke::shader(),
            44,
            6,
            attributes(44),
            depth,
            true,
        )
```

**Locate this block:**

```rust
            &crate::finite_project_source::expand(include_str!("guide.wgsl")),
            48,
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32],
            depth,
            true,
        )
```

Replace it with:

```rust
            &crate::finite_project_source::expand(include_str!("guide.wgsl")),
            48,
            6,
            attributes(48),
            depth,
            true,
        )
```

**Locate this block:**

```rust
            &crate::marker::shader(),
            32,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Float32x4],
        )
    }
```

Replace it with:

```rust
            &crate::marker::shader(),
            32,
            6,
            attributes(32),
        )
    }
```

**Locate this block:**

```rust
            &crate::chain::shader(),
            72,
            18,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32,
                4 => Float32x3, 5 => Float32x3, 6 => Uint32],
        )
    }
```

Replace it with:

```rust
            &crate::chain::shader(),
            72,
            18,
            attributes(72),
        )
    }
```

**Locate this block:**

```rust
            &crate::stroke::shader(),
            44,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
        )
    }
```

Replace it with:

```rust
            &crate::stroke::shader(),
            44,
            6,
            attributes(44),
        )
    }
```

**Locate this block:**

```rust
pub struct Lane {
    flags: Option<wgpu::Buffer>,
    flag_data: Vec<u32>,
```

Replace it with:

```rust
pub(crate) fn attributes(stride: u64) -> &'static [wgpu::VertexAttribute] {
    const STROKE: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32];
    const CHAIN: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32];
    const MARKER: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4];
    const GUIDE: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32];
    match stride {
        44 => &STROKE,
        72 => &CHAIN,
        32 => &MARKER,
        48 => &GUIDE,
        _ => unreachable!(),
    }
}

pub struct Lane {
    flags: Option<wgpu::Buffer>,
    flag_data: Vec<u32>,
```

### 2. `src/finite_ink.rs`

**Locate this existing block:**

```rust
pub struct Ink {
    color: wgpu::RenderPipeline,
    pick: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    bindings: RefCell<
        Option<(
            wgpu::Buffer,
            wgpu::Buffer,
            wgpu::TextureView,
            wgpu::TextureView,
            wgpu::BindGroup,
        )>,
    >,
}
#[derive(Clone)]
pub struct Tables {
    pub projected: wgpu::Buffer,
    pub tiles: wgpu::Buffer,
    pub primitive: wgpu::TextureView,
}
impl Ink {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        stride: u64,
        grid: bool,
        varying: bool,
    ) -> Self {
        let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
        let view = crate::surface_view::layout(device);
        let crop = crate::gpu_layout::uniform(device, 80);
        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("finite ink coverage"),
                entries: &[
                    crate::tile_pipelines::entry(
                        0,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::BufferBindingType::Storage { read_only: true },
                    ),
                    crate::tile_pipelines::entry(
                        1,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::BufferBindingType::Storage { read_only: true },
                    ),
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Depth,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    crate::text_gpu_support::texture_entry(
                        4,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::TextureSampleType::Uint,
                        false,
                    ),
                ],
            });
        let color = pipeline(
            device,
            format,
            stride,
            grid,
            false,
            varying,
            1,
            &view,
            &crop,
            &layout,
            &attributes,
        );
        let pick = pipeline(
            device,
            format,
            stride,
            grid,
            true,
            varying,
            1,
            &view,
            &crop,
            &layout,
            &attributes,
        );
        Self {
            color,
            pick,
            layout,
            bindings: RefCell::new(None),
        }
    }
    fn group(
        &self,
        device: &wgpu::Device,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("finite original ink coverage"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: tables.projected.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: tables.tiles.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(depth),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&tables.primitive),
                },
            ],
        })
    }
    pub fn set(
        &self,
        device: &wgpu::Device,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) {
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
            return;
        }
        let group = self.group(device, tables, depth);
        *bindings = Some((
            tables.projected.clone(),
            tables.tiles.clone(),
            depth.clone(),
            tables.primitive.clone(),
            group,
        ));
    }
    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        view: &wgpu::BindGroup,
        vertices: &wgpu::Buffer,
        flags: &wgpu::Buffer,
        count: u32,
        verts: u32,
    ) -> bool {
        let bindings = self.bindings.borrow();
        let Some((_, _, _, _, group)) = bindings.as_ref() else {
            return false;
        };
        pass.set_pipeline(&self.color);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, flags.slice(..));
        pass.draw(0..verts, 0..count);
        true
    }
    pub fn pick(
        &self,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass<'_>,
        view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup,
        vertices: &wgpu::Buffer,
        ids: &wgpu::Buffer,
        flags: &wgpu::Buffer,
        count: u32,
        verts: u32,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) {
        let group = self.group(device, tables, depth);
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_bind_group(2, &group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, ids.slice(..));
        pass.set_vertex_buffer(2, flags.slice(..));
        pass.draw(0..verts, 0..count);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Ink {
    color: crate::sample_pipelines::Variants<(
        wgpu::RenderPipeline,
        wgpu::BindGroupLayout,
    )>,
    pick: wgpu::RenderPipeline,
    query: wgpu::BindGroupLayout,
    spec: (wgpu::TextureFormat, u64, bool, bool),
    bindings: RefCell<
        Option<(
            wgpu::Buffer,
            wgpu::Buffer,
            wgpu::TextureView,
            wgpu::TextureView,
            wgpu::BindGroup,
        )>,
    >,
}
#[derive(Clone)]
pub struct Tables {
    pub projected: wgpu::Buffer,
    pub tiles: wgpu::Buffer,
    pub primitive: wgpu::TextureView,
}
use crate::stroke_gpu::attributes;
fn layout(device: &wgpu::Device, samples: u32) -> wgpu::BindGroupLayout {
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("finite ink coverage"),
        entries: &[
            crate::tile_pipelines::entry(
                0,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
            crate::tile_pipelines::entry(
                1,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            ),
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: samples == 4,
                },
                count: None,
            },
            crate::text_gpu_support::texture_entry(
                4,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::TextureSampleType::Uint,
                samples == 4,
            ),
        ],
    });
    layout
}
impl Ink {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        stride: u64,
        grid: bool,
        varying: bool,
    ) -> Self {
        Self::with_samples(device, format, stride, grid, varying, 1)
    }
    pub fn with_samples(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        stride: u64,
        grid: bool,
        varying: bool,
        samples: u32,
    ) -> Self {
        assert!(matches!(samples, 1 | 4));
        let attrs = attributes(stride);
        let view = crate::surface_view::layout(device);
        let crop = crate::gpu_layout::uniform(device, 80);
        let query = layout(device, 1);
        let layout = if samples == 1 {
            query.clone()
        } else {
            layout(device, samples)
        };
        let color = pipeline(
            device, format, stride, grid, false, varying, samples, &view, &crop,
            &layout, attrs,
        );
        let pick = pipeline(
            device, format, stride, grid, true, varying, 1, &view, &crop, &query, attrs,
        );
        Self {
            color: crate::sample_pipelines::Variants::new((color, layout), samples),
            pick,
            query,
            spec: (format, stride, grid, varying),
            bindings: RefCell::new(None),
        }
    }
    pub fn set_samples(&mut self, device: &wgpu::Device, samples: u32) {
        let (format, stride, grid, varying) = self.spec;
        if self.color.select(samples, || {
            let layout = if samples == 1 {
                self.query.clone()
            } else {
                layout(device, samples)
            };
            let view = crate::surface_view::layout(device);
            let crop = crate::gpu_layout::uniform(device, 80);
            (
                pipeline(
                    device,
                    format,
                    stride,
                    grid,
                    false,
                    varying,
                    samples,
                    &view,
                    &crop,
                    &layout,
                    attributes(stride),
                ),
                layout,
            )
        }) {
            self.bindings.get_mut().take();
        }
    }
    pub fn clear_bindings(&mut self) {
        self.bindings.get_mut().take();
    }
    fn group(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("finite original ink coverage"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: tables.projected.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: tables.tiles.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(depth),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&tables.primitive),
                },
            ],
        })
    }
    pub fn set(
        &self,
        device: &wgpu::Device,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) {
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
            return;
        }
        let group = self.group(device, &self.color.active.1, tables, depth);
        *bindings = Some((
            tables.projected.clone(),
            tables.tiles.clone(),
            depth.clone(),
            tables.primitive.clone(),
            group,
        ));
    }
    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        view: &wgpu::BindGroup,
        vertices: &wgpu::Buffer,
        flags: &wgpu::Buffer,
        count: u32,
        verts: u32,
    ) -> bool {
        let bindings = self.bindings.borrow();
        let Some((_, _, _, _, group)) = bindings.as_ref() else {
            return false;
        };
        pass.set_pipeline(&self.color.active.0);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, flags.slice(..));
        pass.draw(0..verts, 0..count);
        true
    }
    pub fn pick(
        &self,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass<'_>,
        view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup,
        vertices: &wgpu::Buffer,
        ids: &wgpu::Buffer,
        flags: &wgpu::Buffer,
        count: u32,
        verts: u32,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) {
        let group = self.group(device, &self.query, tables, depth);
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_bind_group(2, &group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, ids.slice(..));
        pass.set_vertex_buffer(2, flags.slice(..));
        pass.draw(0..verts, 0..count);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "finite_sample_tests.rs"]
mod sample_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "ink_pixels_tests.rs"]
mod pixels_tests;
```

### 3. `src/finite_sample_tests.rs`

Create the file and type:

```rust
use super::*;
#[test]
fn actual_colour_variants_reuse_pipelines_and_keep_fresh_queries_single_sample() {
    let renderer = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let device = &renderer.device;
    device.on_uncaptured_error(std::sync::Arc::new(|e| {
        panic!("sample-aware ink: {e}")
    }));
    let buffer = |size| {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("actual ink binding witness"),
            size,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        })
    };
    let projected = buffer(96);
    let tiles = buffer(32);
    let one = crate::frame_targets::Targets::new(
        device,
        [16, 16],
        wgpu::TextureFormat::Rgba8Unorm,
        1,
    );
    let four = crate::frame_targets::Targets::new(
        device,
        [16, 16],
        wgpu::TextureFormat::Rgba8Unorm,
        4,
    );
    let table = |target: &crate::frame_targets::Targets| Tables {
        projected: projected.clone(),
        tiles: tiles.clone(),
        primitive: target.primitive.view.clone(),
    };
    for (stride, grid, varying) in [
        (44, false, false),
        (72, false, false),
        (32, false, false),
        (44, true, false),
        (44, false, true),
        (32, false, true),
    ] {
        let mut ink = Ink::new(
            device,
            wgpu::TextureFormat::Rgba8Unorm,
            stride,
            grid,
            varying,
        );
        let first = ink.color.active.0.clone();
        let query = ink.pick.clone();
        assert!(ink.color.alternate.is_none());
        ink.set(device, &table(&one), &one.depth);
        assert!(ink.bindings.borrow().is_some());
        ink.set_samples(device, 4);
        assert!(ink.bindings.borrow().is_none());
        let second = ink.color.active.0.clone();
        assert_ne!(second, first);
        assert_eq!(ink.pick, query);
        ink.set(device, &table(&four), &four.depth);
        assert!(ink.bindings.borrow().is_some());
        let _query = ink.group(device, &ink.query, &table(&one), &one.depth);
        ink.set_samples(device, 4);
        assert_eq!(ink.color.active.0, second);
        assert!(ink.bindings.borrow().is_some());
        ink.set_samples(device, 1);
        assert_eq!(ink.color.active.0, first);
        assert_eq!(ink.pick, query);
        assert!(ink.bindings.borrow().is_none());
        ink.set_samples(device, 4);
        assert_eq!(ink.color.active.0, second);
        assert_eq!(ink.pick, query);
        let direct = Ink::with_samples(
            device,
            wgpu::TextureFormat::Rgba8Unorm,
            stride,
            grid,
            varying,
            4,
        );
        assert!(direct.color.alternate.is_none());
        direct.set(device, &table(&four), &four.depth);
        let _query = direct.group(
            device,
            &direct.query,
            &table(&one),
            &one.depth,
        );
    }
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
}
```

### 4. `src/ink_pixels_tests.rs`

Create the file and type:

```rust
use super::*;
use wgpu::util::DeviceExt;
const SIZE: [u32; 2] = [128, 96];
fn read(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image: &wgpu::Texture,
    bytes: u32,
) -> Vec<u8> {
    let pitch = SIZE[0] * bytes;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("actual ink pixels"),
        size: u64::from(pitch * SIZE[1]),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        image.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(SIZE[1]),
            },
        },
        image.size(),
    );
    queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let pixels = buffer.slice(..).get_mapped_range().to_vec();
    buffer.unmap();
    pixels
}
fn output(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("resolved actual ink"),
        size: wgpu::Extent3d {
            width: SIZE[0],
            height: SIZE[1],
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}
fn geometry_bytes(stride: u64) -> Vec<u8> {
    let mut values = if stride == 32 {
        vec![0., 0., 0.5, 36., 1., 0., 0., 1.]
    } else {
        vec![
            -0.7, 0., 0.5, 0.7, 0., 0.5, 1., 0., 0., 1., 12.,
        ]
    };
    if stride == 72 {
        values.extend([
            -0.7,
            0.,
            0.5,
            0.7,
            0.,
            0.5,
            f32::from_bits(3),
        ]);
    }
    values
        .into_iter()
        .flat_map(f32::to_ne_bytes)
        .collect()
}
fn geometry(device: &wgpu::Device, stride: u64) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("retained original ink vertex"),
        contents: &geometry_bytes(stride),
        usage: wgpu::BufferUsages::VERTEX,
    })
}
fn occlusion(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    target: &crate::frame_targets::Targets,
    visible: u32,
) {
    let source = format!(
        r#"
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4f {{
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3));return vec4f(p[i],0.5,1);
}}
struct Hit {{@location(0) primitive:vec2u,@builtin(frag_depth) depth:f32}}
@fragment fn fragment(@builtin(sample_index) sample:u32)->Hit {{
 return Hit(vec2u(0),select(0.9,0.0,sample<{visible}u));
}}"#
    );
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("different real depth in each sample"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("actual ink occluder"),
        layout: None,
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
            targets: &[Some(crate::primitive_image::target(true))],
        }),
        primitive: Default::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: crate::depth::FORMAT,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Always),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: target.samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("fresh physical sample depths"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target.primitive.view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.draw(0..3, 0..1);
    }
    queue.submit([encoder.finish()]);
}
fn draw(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    ink: &Ink,
    target: &crate::frame_targets::Targets,
    tables: &Tables,
    view: &wgpu::BindGroup,
    clip: &wgpu::BindGroup,
    vertices: &wgpu::Buffer,
    flags: &wgpu::Buffer,
    stride: u64,
) -> Vec<u8> {
    let image = output(device, wgpu::TextureFormat::Rgba8Unorm);
    let single = image.create_view(&Default::default());
    ink.set(device, tables, &target.depth);
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("real finite colour samples"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target.color(&single),
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_bind_group(2, clip, &[]);
        assert!(ink.draw(
            &mut pass,
            view,
            vertices,
            flags,
            1,
            if stride == 72 {
                18
            } else {
                6
            }
        ));
    }
    target.resolve(&mut encoder, &single);
    queue.submit([encoder.finish()]);
    read(device, queue, &image, 4)
}
fn query(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    ink: &Ink,
    target: &crate::frame_targets::Targets,
    tables: &Tables,
    view: &wgpu::BindGroup,
    clip: &wgpu::BindGroup,
    vertices: &wgpu::Buffer,
    flags: &wgpu::Buffer,
    stride: u64,
) -> [u32; 2] {
    let image = output(device, wgpu::TextureFormat::Rg32Uint);
    let single = image.create_view(&Default::default());
    let ids = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: &[17u32, 1]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>(),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let mut bytes = vec![0u8; 80];
    for i in 0..4 {
        bytes[i * 20..i * 20 + 4].copy_from_slice(&1f32.to_ne_bytes());
    }
    let crop = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: &bytes,
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let crop = crate::gpu_layout::group(
        device,
        &crate::gpu_layout::uniform(device, 80),
        &crop,
    );
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("fresh independent single sample owner query"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &single,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_bind_group(3, clip, &[]);
        ink.pick(
            device,
            &mut pass,
            view,
            &crop,
            vertices,
            &ids,
            flags,
            1,
            if stride == 72 {
                18
            } else {
                6
            },
            tables,
            &target.depth,
        );
    }
    queue.submit([encoder.finish()]);
    let pixels = read(device, queue, &image, 8);
    let at = (48 * 128 + 64) * 8;
    [
        u32::from_ne_bytes(pixels[at..at + 4].try_into().unwrap()),
        u32::from_ne_bytes(pixels[at + 4..at + 8].try_into().unwrap()),
    ]
}
#[test]
fn actual_four_sample_ink_reads_each_depth_and_keeps_queries_independent() {
    let renderer = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let device = &renderer.device;
    let queue = &renderer.queue;
    let buffer = |bytes: &[u8], usage| {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage,
        })
    };
    let projected = buffer(&[0; 96], wgpu::BufferUsages::STORAGE);
    let tiles = buffer(&[0; 32], wgpu::BufferUsages::STORAGE);
    let matrix = session_rust::Xform::identity()
        .m
        .map(|v| v as f32);
    let uniform = buffer(
        &crate::surface_view::bytes(matrix, [0., 1., 0., 0.], SIZE, 1.),
        wgpu::BufferUsages::UNIFORM,
    );
    let view = crate::gpu_layout::group(
        device,
        &crate::surface_view::layout(device),
        &uniform,
    );
    let clipping = crate::clipping_gpu::Clipping::new(device);
    let one = crate::frame_targets::Targets::new(
        device,
        SIZE,
        wgpu::TextureFormat::Rgba8Unorm,
        1,
    );
    let four = crate::frame_targets::Targets::new(
        device,
        SIZE,
        wgpu::TextureFormat::Rgba8Unorm,
        4,
    );
    let table = |target: &crate::frame_targets::Targets| Tables {
        projected: projected.clone(),
        tiles: tiles.clone(),
        primitive: target.primitive.view.clone(),
    };
    let path = std::path::Path::new("ink-pixels");
    for (case, (stride, grid, varying)) in [
        (44, false, false),
        (72, false, false),
        (32, false, false),
        (44, true, false),
        (44, false, true),
        (32, false, true),
    ]
    .into_iter()
    .enumerate()
    {
        let vertices = geometry(device, stride);
        let flags = buffer(&[0; 8], wgpu::BufferUsages::VERTEX);
        let mut ink = Ink::with_samples(
            device,
            wgpu::TextureFormat::Rgba8Unorm,
            stride,
            grid,
            varying,
            4,
        );
        let mut mixed = Vec::new();
        for visible in 0..=4 {
            occlusion(device, queue, &four, visible);
            let pixels = draw(
                device,
                queue,
                &ink,
                &four,
                &table(&four),
                &view,
                &clipping.group,
                &vertices,
                &flags,
                stride,
            );
            let centre = &pixels[(48 * 128 + 64) * 4..(48 * 128 + 65) * 4];
            let expected = (255. * (1. - visible as f32 / 4.)).round() as i32;
            assert_eq!(
                centre[0], 255,
                "family{case} visible{visible}"
            );
            assert!(
                (i32::from(centre[1]) - expected).abs() <= 1,
                "family{case} visible{visible}: actual{centre:?}, expected green{expected}"
            );
            assert_eq!(centre[1], centre[2]);
            if visible == 2 {
                crate::sample_test_proof::write(
                    path.join(format!("family-{case}.rgba")),
                    &pixels,
                )
                .unwrap();
                mixed = pixels;
            }
        }
        occlusion(device, queue, &one, 1);
        assert_eq!(
            query(
                device,
                queue,
                &ink,
                &one,
                &table(&one),
                &view,
                &clipping.group,
                &vertices,
                &flags,
                stride
            ),
            [17, 1],
            "query uses fresh1x depth while4x colour depth is unrelated"
        );
        occlusion(device, queue, &one, 0);
        assert_eq!(
            query(
                device,
                queue,
                &ink,
                &one,
                &table(&one),
                &view,
                &clipping.group,
                &vertices,
                &flags,
                stride
            ),
            [0, 0],
            "fresh1x occlusion controls query ownership"
        );
        occlusion(device, queue, &four, 2);
        ink.set_samples(device, 1);
        occlusion(device, queue, &one, 1);
        let single = draw(
            device,
            queue,
            &ink,
            &one,
            &table(&one),
            &view,
            &clipping.group,
            &vertices,
            &flags,
            stride,
        );
        assert_eq!(
            &single[(48 * 128 + 64) * 4..(48 * 128 + 65) * 4],
            &[255, 0, 0, 255]
        );
        ink.set_samples(device, 4);
        let returned = draw(
            device,
            queue,
            &ink,
            &four,
            &table(&four),
            &view,
            &clipping.group,
            &vertices,
            &flags,
            stride,
        );
        assert!(
            returned == mixed,
            "family{case}:4→1→4 returns every actual colour pixel exactly"
        );
    }
}
```

## <a id="checkpoint-96zzzzzzza-retarget-existing-overlay-lanes"></a>7. Retarget existing overlay lanes

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

### 1. `src/stroke_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            finite: None,
            strict_depth,
            pipeline,
            pick_pipeline,
```

Replace it with:

```rust
            finite: None,
            strict_depth,
            pipeline,
            recipe: (format, depth, physical),
            samples: 1,
            pick_pipeline,
```

**Locate this block:**

```rust
                ),
            })
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth = depth.depth_compare == Some(crate::depth::CLOSER);
        let mut targets = vec![Some(wgpu::ColorTargetState {
            format,
            blend: if physical {
                None
            } else {
                Some(wgpu::BlendState::ALPHA_BLENDING)
            },
            write_mask: wgpu::ColorWrites::ALL,
        })];
        if physical {
            targets.push(Some(crate::primitive_image::target(true)));
        }
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: stride,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes,
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            primitive: wgpu::PrimitiveState {
                topology: if physical {
                    wgpu::PrimitiveTopology::LineList
                } else {
                    wgpu::PrimitiveTopology::TriangleList
                },
                ..Default::default()
            },
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pick_pipeline = pick_shader.as_ref().map(|pick_shader| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ink IDs"),
```

Replace it with:

```rust
                ),
            })
        });
        let strict_depth = depth.depth_compare == Some(crate::depth::CLOSER);
        let pipeline = crate::sample_pipelines::Variants::new(
            pipeline(
                device,
                format,
                source,
                stride,
                attributes,
                depth.clone(),
                physical,
                1,
            ),
            1,
        );
        let pick_pipeline = pick_shader.as_ref().map(|pick_shader| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ink IDs"),
```

**Locate this block:**

```rust
    ) -> Self {
        let view_layout = crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"),
            bind_group_layouts: &[Some(&view_layout)],
            immediate_size: 0,
        });
        let pick_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("ink pick settings"),
```

Replace it with:

```rust
    ) -> Self {
        let view_layout = crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pick_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("ink pick settings"),
```

**Locate this block:**

```rust
        }) {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(
```

Replace it with:

```rust
        }) {
            return;
        }
        pass.set_pipeline(&self.pipeline.active);
        pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(
```

**Locate this block:**

```rust
        ));
        self.reset_flags(device)
            .expect("Original ink clipping flags");
    }
    pub fn finite_frame(
        &self,
        device: &wgpu::Device,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) {
        if let Some(ink) = &self.finite {
            ink.set(device, tables, depth)
        }
```

Replace it with:

```rust
        ));
        self.reset_flags(device)
            .expect("Original ink clipping flags");
        self.set_samples(device, self.samples);
    }
    pub fn set_samples(&mut self, device: &wgpu::Device, samples: u32) {
        assert!(matches!(samples, 1 | 4));
        self.samples = samples;
        if self.data.is_empty() {
            if let Some(ink) = &mut self.finite {
                ink.clear_bindings();
            }
            return;
        }
        if let Some(ink) = &mut self.finite {
            ink.set_samples(device, samples);
            return;
        }
        let (format, depth, physical) = &self.recipe;
        self.pipeline.select(samples, || {
            let source = if *physical {
                include_str!("grid.wgsl").into()
            } else {
                match self.stride {
                    44 => crate::stroke::shader(),
                    72 => crate::chain::shader(),
                    32 => crate::marker::shader(),
                    48 => {
                        crate::finite_project_source::expand(include_str!("guide.wgsl"))
                    }
                    _ => unreachable!(),
                }
            };
            pipeline(
                device,
                *format,
                &source,
                self.stride,
                attributes(self.stride),
                depth.clone(),
                *physical,
                samples,
            )
        });
    }
    pub fn finite_frame(
        &self,
        device: &wgpu::Device,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) {
        if self.data.is_empty() {
            return;
        }
        if let Some(ink) = &self.finite {
            ink.set(device, tables, depth)
        }
```

**Locate this block:**

```rust
        if self.finite.is_some() {
            self.reset_flags(device)?;
        }
        Ok(())
    }
```

Replace it with:

```rust
        if self.finite.is_some() {
            self.reset_flags(device)?;
        }
        self.set_samples(device, self.samples);
        Ok(())
    }
```

**Locate this block:**

```rust
    varying: bool,
    finite: Option<crate::finite_ink::Ink>,
    strict_depth: bool,
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
```

Replace it with:

```rust
    varying: bool,
    finite: Option<crate::finite_ink::Ink>,
    strict_depth: bool,
    pipeline: crate::sample_pipelines::Variants<wgpu::RenderPipeline>,
    recipe: (
        wgpu::TextureFormat,
        wgpu::DepthStencilState,
        bool,
    ),
    samples: u32,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
```

**Locate this block:**

```rust
pub(crate) fn attributes(stride: u64) -> &'static [wgpu::VertexAttribute] {
    const STROKE: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32];
```

Replace it with:

```rust
pub(crate) fn sampled(source: &str, stride: u64) -> String {
    let source = source.replace(
        "@interpolate(linear)",
        "@interpolate(linear,sample)",
    );
    if stride != 32 {
        return source;
    }
    source.replace("@location(2) @interpolate(flat) radius: f32,","@location(2) @interpolate(flat) radius: f32,\n    @location(3) @interpolate(linear,sample) pixel:vec2f,")
 .replace("colour, vec2<f32>(0.0), 0.0)","colour, vec2<f32>(0.0), 0.0,vec2f(0.))")
 .replace("Output(position, colour, centre, radius)","Output(position, colour, centre, radius,(position.xy/position.w*vec2f(0.5,-0.5)+0.5)*view.size)")
 .replace("input.position.xy-input.centre","input.pixel-input.centre")
}
fn pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    source: &str,
    stride: u64,
    attributes: &[wgpu::VertexAttribute],
    depth: wgpu::DepthStencilState,
    physical: bool,
    samples: u32,
) -> wgpu::RenderPipeline {
    let sampled = if samples == 4 {
        Some(sampled(source, stride))
    } else {
        None
    };
    let source = sampled.as_deref().unwrap_or(source);
    let view_layout = crate::surface_view::layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("ink settings"),
        bind_group_layouts: &[Some(&view_layout)],
        immediate_size: 0,
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("stroke"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let mut targets = vec![Some(wgpu::ColorTargetState {
        format,
        blend: if physical {
            None
        } else {
            Some(wgpu::BlendState::ALPHA_BLENDING)
        },
        write_mask: wgpu::ColorWrites::ALL,
    })];
    if physical {
        targets.push(Some(crate::primitive_image::target(true)));
    }
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("stroke"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vertex"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: stride,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes,
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fragment"),
            compilation_options: Default::default(),
            targets: &targets,
        }),
        primitive: wgpu::PrimitiveState {
            topology: if physical {
                wgpu::PrimitiveTopology::LineList
            } else {
                wgpu::PrimitiveTopology::TriangleList
            },
            ..Default::default()
        },
        depth_stencil: Some(depth),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    });
    pipeline
}
pub(crate) fn attributes(stride: u64) -> &'static [wgpu::VertexAttribute] {
    const STROKE: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32];
```

### 2. `src/highlight.rs`

**Locate this existing block:**

```rust
    }
    pub fn rebase_face(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn set_samples(&mut self, gpu: &wgpu::Device, samples: u32) {
        for lane in [
            &mut self.strokes,
            &mut self.paths,
            &mut self.markers,
        ] {
            lane.set_samples(gpu, samples);
        }
    }
    pub fn rebase_face(
```

### 3. `src/tool_preview.rs`

**Locate this existing block:**

```rust
impl Preview {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Preview {
    pub fn set_samples(&mut self, gpu: &wgpu::Device, samples: u32) {
        self.strokes.set_samples(gpu, samples);
        self.markers.set_samples(gpu, samples);
    }
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

### 4. `src/measure_gpu.rs`

**Locate this existing block:**

```rust
impl Measurement {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Measurement {
    pub fn set_samples(&mut self, gpu: &wgpu::Device, samples: u32) {
        self.strokes.set_samples(gpu, samples);
    }
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

### 5. `src/guide_gpu.rs`

**Locate this existing block:**

```rust
impl Guide {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Guide {
    pub fn set_samples(&mut self, gpu: &wgpu::Device, samples: u32) {
        self.segments.set_samples(gpu, samples);
        self.marks.set_samples(gpu, samples);
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

### 6. `src/stroke_gpu.rs`

**Locate this existing block:**

```rust
#[path = "grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "stroke_sample_tests.rs"]
mod sample_tests;
```

### 7. `src/stroke_sample_tests.rs`

Create the file and type:

```rust
use super::*;
fn frame(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    lane: &Lane,
    target: &crate::frame_targets::Targets,
    view: &wgpu::BindGroup,
) -> Vec<u8> {
    let output = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("actual overlay samples"),
        size: wgpu::Extent3d {
            width: 128,
            height: 96,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let single = output.create_view(&Default::default());
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut attachments = vec![Some(wgpu::RenderPassColorAttachment {
            view: target.color(&single),
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                store: wgpu::StoreOp::Store,
            },
        })];
        if lane.recipe.2 {
            attachments.push(Some(wgpu::RenderPassColorAttachment {
                view: &target.primitive.view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            }));
        }
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("real overlay draw"),
            color_attachments: &attachments,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        lane.draw(&mut pass, view);
    }
    target.resolve(&mut encoder, &single);
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 128 * 96 * 4,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        output.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(512),
                rows_per_image: Some(96),
            },
        },
        output.size(),
    );
    queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let pixels = buffer.slice(..).get_mapped_range().to_vec();
    buffer.unmap();
    pixels
}
#[test]
fn actual_overlay_colours_retarget_lazily_without_reuploading_vertices_or_queries() {
    use wgpu::util::DeviceExt;
    let r = crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());
    let device = &r.device;
    let queue = &r.queue;
    let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: &crate::surface_view::bytes(
            session_rust::Xform::identity()
                .m
                .map(|v| v as f32),
            [0., 1., 0., 0.],
            [128, 96],
            1.,
        ),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let view = crate::gpu_layout::group(
        device,
        &crate::surface_view::layout(device),
        &uniform,
    );
    let one = crate::frame_targets::Targets::new(
        device,
        [128, 96],
        wgpu::TextureFormat::Rgba8Unorm,
        1,
    );
    let four = crate::frame_targets::Targets::new(
        device,
        [128, 96],
        wgpu::TextureFormat::Rgba8Unorm,
        4,
    );
    let storage = |size| {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        })
    };
    let tables = crate::finite_ink::Tables {
        projected: storage(96),
        tiles: storage(32),
        primitive: four.primitive.view.clone(),
    };
    let mut empty = Lane::new(device, wgpu::TextureFormat::Rgba8Unorm);
    empty.enable_finite(
        device,
        wgpu::TextureFormat::Rgba8Unorm,
        false,
    );
    empty.set_samples(device, 4);
    empty.finite_frame(device, &tables, &four.depth);
    assert!(empty.data.is_empty());
    assert!(empty.vertices.is_none());
    assert_eq!(empty.samples, 4);
    empty
        .set(
            device,
            [crate::stroke::Stroke {
                start: [-0.7, 0., 0.5],
                end: [0.7, 0., 0.5],
                width: 8.,
                colour: [1., 0., 0., 1.],
            }],
        )
        .unwrap();
    empty.finite_frame(device, &tables, &four.depth);
    empty
        .set(device, std::iter::empty())
        .unwrap();
    empty.set_samples(device, 4);
    empty.finite_frame(device, &tables, &one.depth);
    assert!(empty.data.is_empty());
    let root = std::path::Path::new("overlay-pixels");
    for case in 0..6 {
        let mut lane = match case {
            0 => Lane::new(device, wgpu::TextureFormat::Rgba8Unorm),
            1 => Lane::joined(device, wgpu::TextureFormat::Rgba8Unorm),
            2 => Lane::markers(device, wgpu::TextureFormat::Rgba8Unorm),
            3 => Lane::guides(device, wgpu::TextureFormat::Rgba8Unorm),
            4 => Lane::overlay(device, wgpu::TextureFormat::Rgba8Unorm),
            _ => Lane::grid(device, wgpu::TextureFormat::Rgba8Unorm),
        };
        let original = lane.pipeline.active.clone();
        let pick = lane.pick_pipeline.clone();
        lane.set_samples(device, 4);
        assert_eq!(
            lane.pipeline.samples, 1,
            "An empty lane must not compile4x"
        );
        assert!(lane.pipeline.alternate.is_none());
        let stroke = crate::stroke::Stroke {
            start: [-0.7, 0., 0.5],
            end: [0.7, 0., 0.5],
            width: 8.,
            colour: [1., 0., 0., 1.],
        };
        match case {
            1 => lane
                .set_chains(
                    device,
                    [crate::chain::Segment {
                        stroke,
                        previous: stroke.start,
                        next: stroke.end,
                        heads: 3,
                    }],
                )
                .unwrap(),
            2 => lane
                .set_points(
                    device,
                    [crate::marker::Marker {
                        center: [0., 0., 0.5],
                        diameter: 28.,
                        colour: [1., 0., 0., 1.],
                    }],
                )
                .unwrap(),
            3 => lane
                .set_guides(
                    device,
                    &[crate::guide_gpu::Segment {
                        stroke,
                        dashed: true,
                    }],
                )
                .unwrap(),
            _ => lane.set(device, [stroke]).unwrap(),
        }
        assert_eq!(lane.pipeline.samples, 4);
        let alternate = lane.pipeline.active.clone();
        assert_ne!(alternate, original);
        assert_eq!(lane.pick_pipeline, pick);
        lane.set_ids(device, [[17, 1]]).unwrap();
        let vertices = lane.vertices.clone();
        let ids = lane.ids.clone();
        let data = lane.data.clone();
        let uploads = [lane.uploads, lane.uploaded_bytes];
        let first = frame(device, queue, &lane, &four, &view);
        assert!(
            first
                .chunks_exact(4)
                .filter(|p| p[1] < 250)
                .count()
                > 30,
            "Actual nonempty overlay family{case}"
        );
        crate::sample_test_proof::write(
            root.join(format!("family-{case}.rgba")),
            &first,
        )
        .unwrap();
        lane.set_samples(device, 1);
        assert_eq!(lane.pipeline.active, original);
        let single = frame(device, queue, &lane, &one, &view);
        assert!(single.chunks_exact(4).any(|p| p[1] < 250));
        lane.set_samples(device, 4);
        assert_eq!(lane.pipeline.active, alternate);
        assert!(frame(device, queue, &lane, &four, &view) == first);
        assert_eq!(lane.vertices, vertices);
        assert_eq!(lane.ids, ids);
        assert_eq!(lane.data, data);
        assert_eq!([lane.uploads, lane.uploaded_bytes], uploads);
        assert_eq!(lane.pick_pipeline, pick);
    }
}
```

## <a id="checkpoint-96zzzzzzzb-read-section-crossings-at-each-sample"></a>8. Read section crossings at each sample

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

### 1. `src/cap_pipeline.rs`

**Locate this existing block:**

```rust
    count: bool,
) -> wgpu::RenderPipeline {
    let attributes = wgpu::vertex_attr_array![0=>Float32x3];
    let buffers = if count {
        vec![wgpu::VertexBufferLayout {
            array_stride: 20,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes,
        }]
    } else {
        Vec::new()
    };
    pipeline_buffers(
        device, source, groups, vertex, fragment, targets, &buffers, count,
    )
}
pub(crate) fn pipeline_buffers(
    device: &wgpu::Device,
    source: String,
    groups: &[&wgpu::BindGroupLayout],
    vertex: &str,
    fragment: &str,
    targets: &[wgpu::ColorTargetState],
    buffers: &[wgpu::VertexBufferLayout<'_>],
    count: bool,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("original closed-solid sections"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("original closed-solid sections"),
        bind_group_layouts: &groups
            .iter()
            .map(|g| Some(*g))
            .collect::<Vec<_>>(),
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("original closed-solid sections"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(vertex),
            compilation_options: Default::default(),
            buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(fragment),
            compilation_options: Default::default(),
            targets: &targets
                .iter()
                .cloned()
                .map(Some)
                .collect::<Vec<_>>(),
        }),
        primitive: Default::default(),
        depth_stencil: if count {
            None
        } else {
            Some(crate::depth::surface())
        },
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
pub struct Pipelines {
    pub count: wgpu::RenderPipeline,
    pub record_count: wgpu::RenderPipeline,
    pub geometry: wgpu::BindGroupLayout,
    pub color: wgpu::RenderPipeline,
    pub counts: wgpu::BindGroupLayout,
    pub frame: wgpu::BindGroupLayout,
}
impl Pipelines {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let view = crate::surface_view::layout(device);
        let model =
            crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
        let clip = crate::clipping_gpu::layout(device);
        let frame = crate::gpu_layout::uniform(device, 16);
        let counts =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("section crossing texture"),
                entries: &[crate::text_gpu_support::texture_entry(
                    0,
                    wgpu::ShaderStages::FRAGMENT,
                    wgpu::TextureSampleType::Float {
                        filterable: false,
                    },
                    false,
                )],
            });
        let add = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        };
        let target = wgpu::ColorTargetState {
            format: wgpu::TextureFormat::R16Float,
            blend: Some(wgpu::BlendState {
                color: add,
                alpha: add,
            }),
            write_mask: wgpu::ColorWrites::RED,
        };
        let count = pipeline(
            device,
            crate::clipping_gpu::source(include_str!("cap_count.wgsl"), 2, 0),
            &[&view, &model, &clip],
            "vs_count",
            "fs_count",
            &[target.clone()],
            true,
        );
        let geometry = crate::cap_geometry::layout(device);
        let source = crate::object_rows::source(include_str!("cap_count.wgsl"));
        let record_count = pipeline_buffers(
            device,
            crate::clipping_gpu::source(&source, 2, 0),
            &[
                &view,
                &crate::object_rows::layout(device),
                &clip,
                &geometry,
            ],
            "vs_record_count",
            "fs_count",
            &[target],
            &[crate::draw_slots::layout()],
            true,
        );
        let color = pipeline(
            device,
            crate::clipping_gpu::source(include_str!("cap_color.wgsl"), 1, 0),
            &[&frame, &clip, &counts],
            "vs_cap",
            "fs_cap",
            &[
                wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                },
                crate::primitive_image::target(true),
            ],
            false,
        );
        Self {
            count,
            record_count,
            geometry,
            color,
            counts,
            frame,
        }
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    count: bool,
    samples: u32,
) -> wgpu::RenderPipeline {
    let attributes = wgpu::vertex_attr_array![0=>Float32x3];
    let buffers = if count {
        vec![wgpu::VertexBufferLayout {
            array_stride: 20,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attributes,
        }]
    } else {
        Vec::new()
    };
    pipeline_buffers(
        device, source, groups, vertex, fragment, targets, &buffers, count, samples,
    )
}
pub(crate) fn pipeline_buffers(
    device: &wgpu::Device,
    source: String,
    groups: &[&wgpu::BindGroupLayout],
    vertex: &str,
    fragment: &str,
    targets: &[wgpu::ColorTargetState],
    buffers: &[wgpu::VertexBufferLayout<'_>],
    count: bool,
    samples: u32,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("original closed-solid sections"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("original closed-solid sections"),
        bind_group_layouts: &groups
            .iter()
            .map(|g| Some(*g))
            .collect::<Vec<_>>(),
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("original closed-solid sections"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(vertex),
            compilation_options: Default::default(),
            buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(fragment),
            compilation_options: Default::default(),
            targets: &targets
                .iter()
                .cloned()
                .map(Some)
                .collect::<Vec<_>>(),
        }),
        primitive: Default::default(),
        depth_stencil: if count {
            None
        } else {
            Some(crate::depth::surface())
        },
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}
pub struct Pipelines {
    pub count: wgpu::RenderPipeline,
    pub record_count: wgpu::RenderPipeline,
    pub geometry: wgpu::BindGroupLayout,
    pub color: wgpu::RenderPipeline,
    pub counts: wgpu::BindGroupLayout,
    pub frame: wgpu::BindGroupLayout,
}
impl Pipelines {
    pub fn with_samples(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert!(matches!(samples, 1 | 4));
        let view = crate::surface_view::layout(device);
        let model =
            crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
        let clip = crate::clipping_gpu::layout(device);
        let frame = crate::gpu_layout::uniform(device, 16);
        let counts =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("section crossing texture"),
                entries: &[crate::text_gpu_support::texture_entry(
                    0,
                    wgpu::ShaderStages::FRAGMENT,
                    wgpu::TextureSampleType::Float {
                        filterable: false,
                    },
                    samples == 4,
                )],
            });
        let add = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        };
        let target = wgpu::ColorTargetState {
            format: wgpu::TextureFormat::R16Float,
            blend: Some(wgpu::BlendState {
                color: add,
                alpha: add,
            }),
            write_mask: wgpu::ColorWrites::RED,
        };
        let count = pipeline(
            device,
            count_source(include_str!("cap_count.wgsl"), samples),
            &[&view, &model, &clip],
            "vs_count",
            "fs_count",
            &[target.clone()],
            true,
            samples,
        );
        let geometry = crate::cap_geometry::layout(device);
        let source = crate::object_rows::source(include_str!("cap_count.wgsl"));
        let record_count = pipeline_buffers(
            device,
            count_source(&source, samples),
            &[
                &view,
                &crate::object_rows::layout(device),
                &clip,
                &geometry,
            ],
            "vs_record_count",
            "fs_count",
            &[target],
            &[crate::draw_slots::layout()],
            true,
            samples,
        );
        let color = pipeline(
            device,
            crate::clipping_gpu::source(&colour_source(samples), 1, 0),
            &[&frame, &clip, &counts],
            "vs_cap",
            "fs_cap",
            &[
                wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                },
                crate::primitive_image::target(true),
            ],
            false,
            samples,
        );
        Self {
            count,
            record_count,
            geometry,
            color,
            counts,
            frame,
        }
    }
}

fn count_source(source: &str, samples: u32) -> String {
    let source = crate::clipping_gpu::source(source, 2, 0);
    if samples == 1 {
        source
    } else {
        source.replace(
            "@location(0) world_pos:",
            "@location(0) @interpolate(perspective,sample) world_pos:",
        )
    }
}
fn colour_source(samples: u32) -> String {
    let source = include_str!("cap_color.wgsl");
    if samples == 1 {
        return source.into();
    }
    source.replace("var counts:texture_2d<f32>;","var counts:texture_multisampled_2d<f32>;")
 .replace("const SAMPLES:u32=1u;","const SAMPLES:u32=4u;")
 .replace("textureLoad(counts,at,0)","textureLoad(counts,at,i32(sample))")
 .replace("@location(0) @interpolate(flat) plane: u32,","@location(0) @interpolate(flat) plane: u32,\n    @location(1) @interpolate(linear,sample) pixel:vec2f,")
 .replace("out.plane = plane;","out.plane = plane;out.pixel=(out.pos.xy/out.pos.w*vec2f(0.5,-0.5)+0.5)*cap_frame.size;")
 .replace("in.pos.xy","in.pixel")
 .replace("let samples = clip_samples(t, gradient);","let samples = select(0u,0xffffffffu,t>=0.);")
}
```

### 2. `src/cap_color.wgsl`

**Locate this existing block:**

```wgsl
// Where the view ray meets a plane inside a closed solid: outlined hatch or solid grey.
fn fs_cap(in: CapVertex) -> CapOut {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + cap_frame.origin, cap_frame.size, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let h = vec3<f32>(ndc, 1.0);
    // scene units across the hatch lines, and their change per pixel
    let u = dot(clipping.hatch[i].xyz, h) / dot(clipping.hatch_w[i].xyz, h);
    let slope = vec2<f32>(dpdx(u), dpdy(u));
    var kept = 0xffffffffu;
    var edge = 0.0;

    // the other planes cut the cap sample by sample
    for (var j = 0u; j < clipping.count; j++) {
        let t = dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0));
        let gradient = vec2<f32>(dpdx(t), dpdy(t));
        let samples = clip_samples(t, gradient);
        kept &= select(samples, 0xffffffffu, j == i);
        if (j != i && dot(gradient, gradient) > 1e-30) {
            let distance = t / max(length(gradient), 1e-20);
            edge = max(edge, 1.0 - smoothstep(clipping.outline - 0.5, clipping.outline + 0.5, distance));
        }
    }

    let at = vec2<i32>(in.pos.xy);
    var inside = 0u;
    var selected = false;

    // inside a solid where more crossings leave than enter behind the plane
    for (var k = 0u; k < SAMPLES; k++) {
        let count = count_at(at, k);
        inside |= select(0u, 1u << k, count > 0.5);
        selected = selected || count > SELECTED_CROSSING - 0.5;
    }

    let mask = inside & kept;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
// Where the view ray meets a plane inside a closed solid: outlined hatch or solid grey.
fn fs_cap(in:CapVertex,@builtin(sample_index) sample:u32)->CapOut {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + cap_frame.origin, cap_frame.size, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let h = vec3<f32>(ndc, 1.0);
    // scene units across the hatch lines, and their change per pixel
    let u = dot(clipping.hatch[i].xyz, h) / dot(clipping.hatch_w[i].xyz, h);
    let slope = vec2<f32>(dpdx(u), dpdy(u));
    var kept = 0xffffffffu;
    var edge = 0.0;

    // the other planes cut the cap sample by sample
    for (var j = 0u; j < clipping.count; j++) {
        let t = dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0));
        let gradient = vec2<f32>(dpdx(t), dpdy(t));
        let samples = clip_samples(t, gradient);
        kept &= select(samples, 0xffffffffu, j == i);
        if (j != i && dot(gradient, gradient) > 1e-30) {
            let distance = t / max(length(gradient), 1e-20);
            edge = max(edge, 1.0 - smoothstep(clipping.outline - 0.5, clipping.outline + 0.5, distance));
        }
    }

    let at = vec2<i32>(in.pos.xy);
    let count=count_at(at,sample);
    let inside=select(0u,1u<<sample,count>0.5);
    let selected=count>SELECTED_CROSSING-0.5;

    let mask = inside & kept;
```

### 3. `src/cap_pick_pipeline.rs`

**Locate this existing block:**

```rust
            true,
        );
        let nearest = crate::cap_pipeline::pipeline(
            device,
            source,
            &[&view, &model, &clip, &records],
            "vs_pick_count",
            "fs_pick_owner",
            &[target.clone()],
            true,
        );
        let shared = crate::object_rows::source(include_str!("cap_count.wgsl"));
        let source = crate::clipping_gpu::source(&shared, 2, 0);
        let layouts = [
            &view,
            &crate::object_rows::layout(device),
            &clip,
            &record_geometry,
        ];
        let record_count = crate::cap_pipeline::pipeline_buffers(
            device,
            source.clone(),
            &layouts,
            "vs_record_pick_count",
            "fs_pick_count",
            &[target.clone()],
            &[crate::draw_slots::layout()],
            true,
        );
        let record_nearest = crate::cap_pipeline::pipeline_buffers(
            device,
            source,
            &layouts,
            "vs_record_pick_count",
            "fs_pick_owner",
            &[target],
            &[crate::draw_slots::layout()],
            true,
        );
        let ids = crate::cap_pipeline::pipeline(
            device,
            crate::clipping_gpu::source(include_str!("cap_pick.wgsl"), 1, 0),
            &[&frame, &clip, &owners],
            "vs_cap",
            "fs_cap_id",
            &[
                wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rg32Uint,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                },
                crate::primitive_image::target(true),
            ],
            false,
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            true,
            1,
        );
        let nearest = crate::cap_pipeline::pipeline(
            device,
            source,
            &[&view, &model, &clip, &records],
            "vs_pick_count",
            "fs_pick_owner",
            &[target.clone()],
            true,
            1,
        );
        let shared = crate::object_rows::source(include_str!("cap_count.wgsl"));
        let source = crate::clipping_gpu::source(&shared, 2, 0);
        let layouts = [
            &view,
            &crate::object_rows::layout(device),
            &clip,
            &record_geometry,
        ];
        let record_count = crate::cap_pipeline::pipeline_buffers(
            device,
            source.clone(),
            &layouts,
            "vs_record_pick_count",
            "fs_pick_count",
            &[target.clone()],
            &[crate::draw_slots::layout()],
            true,
            1,
        );
        let record_nearest = crate::cap_pipeline::pipeline_buffers(
            device,
            source,
            &layouts,
            "vs_record_pick_count",
            "fs_pick_owner",
            &[target],
            &[crate::draw_slots::layout()],
            true,
            1,
        );
        let ids = crate::cap_pipeline::pipeline(
            device,
            crate::clipping_gpu::source(include_str!("cap_pick.wgsl"), 1, 0),
            &[&frame, &clip, &owners],
            "vs_cap",
            "fs_cap_id",
            &[
                wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rg32Uint,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                },
                crate::primitive_image::target(true),
            ],
            false,
            1,
        );
```

### 4. `src/caps_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let (Some(pipes), Some(counts)) = (&self.pipes, &mut self.counts) else {
            return;
        };
        for plane in 0..planes {
```

Replace it with:

```rust
        let (Some(pipes), Some(counts)) = (&self.pipes, &mut self.counts) else {
            return;
        };
        let pipes = &pipes.active;
        for plane in 0..planes {
```

**Locate this block:**

```rust
            self.counts = Some(Counts {
                image,
                group,
                geometry: Default::default(),
            });
        }
        self.counts
```

Replace it with:

```rust
            self.counts = Some(Counts {
                image,
                group,
                geometry,
            });
        }
        self.counts
```

**Locate this block:**

```rust
            return;
        }
        if self.pipes.is_none() {
            let pipes = crate::cap_pipeline::Pipelines::new(device, format);
            self.frame_group = Some(crate::gpu_layout::group(
                device,
                &pipes.frame,
                &self.frame,
            ));
            self.pipes = Some(pipes);
        }
        let pipes = self.pipes.as_ref().unwrap();
        if self
            .counts
            .as_ref()
            .is_none_or(|c| c.image.size != size)
        {
            let image = Image::new(
                device,
                size,
                wgpu::TextureFormat::R16Float,
                2,
            );
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("original section crossings"),
```

Replace it with:

```rust
            return;
        }
        if self.pipes.is_none() {
            let pipes = crate::cap_pipeline::Pipelines::with_samples(
                device,
                format,
                self.samples,
            );
            self.frame_group = Some(crate::gpu_layout::group(
                device,
                &pipes.frame,
                &self.frame,
            ));
            self.pipes = Some(crate::sample_pipelines::Variants::new(
                pipes,
                self.samples,
            ));
        }
        self.pipes
            .as_mut()
            .unwrap()
            .select(self.samples, || {
                crate::cap_pipeline::Pipelines::with_samples(
                    device,
                    format,
                    self.samples,
                )
            });
        let pipes = &self.pipes.as_ref().unwrap().active;
        if self.counts.as_ref().is_none_or(|c| {
            c.image.size != size || c.image.texture.sample_count() != self.samples
        }) {
            let geometry = self
                .counts
                .take()
                .map_or_else(Default::default, |c| c.geometry);
            let image = Image::with_samples(
                device,
                size,
                wgpu::TextureFormat::R16Float,
                2,
                self.samples,
            );
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("original section crossings"),
```

**Locate this block:**

```rust
            passes: 0,
        }
    }
    pub fn set_scene(
        &mut self,
        queue: &wgpu::Queue,
```

Replace it with:

```rust
            passes: 0,
        }
    }
    pub fn set_samples(&mut self, samples: u32) {
        assert!(matches!(samples, 1 | 4));
        self.samples = samples;
    }
    pub fn set_scene(
        &mut self,
        queue: &wgpu::Queue,
```

**Locate this block:**

```rust
            targets: Default::default(),
            solids: Default::default(),
            pipes: None,
            counts: None,
            frame,
            frame_group: None,
```

Replace it with:

```rust
            targets: Default::default(),
            solids: Default::default(),
            pipes: None,
            samples: 1,
            counts: None,
            frame,
            frame_group: None,
```

**Locate this block:**

```rust
    pub solids: crate::solid_cache::Solids,
    pipes: Option<crate::cap_pipeline::Pipelines>,
    counts: Option<Counts>,
    frame: wgpu::Buffer,
    frame_group: Option<wgpu::BindGroup>,
```

Replace it with:

```rust
    pub solids: crate::solid_cache::Solids,
    pipes: Option<crate::sample_pipelines::Variants<crate::cap_pipeline::Pipelines>>,
    samples: u32,
    counts: Option<Counts>,
    frame: wgpu::Buffer,
    frame_group: Option<wgpu::BindGroup>,
```

### 5. `src/cap_image.rs`

**Locate this existing block:**

```rust
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("original section data"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        Self {
            texture,
            view,
            size,
            bytes: bytes * u64::from(size[0]) * u64::from(size[1]),
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Self {
        Self::with_samples(device, size, format, bytes, 1)
    }
    pub fn with_samples(
        device: &wgpu::Device,
        size: [u32; 2],
        format: wgpu::TextureFormat,
        bytes: u64,
        samples: u32,
    ) -> Self {
        assert!(matches!(samples, 1 | 4));
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("original section data"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        Self {
            texture,
            view,
            size,
            bytes: bytes * u64::from(size[0]) * u64::from(size[1]) * u64::from(samples),
        }
```

### 6. `src/sample_surface_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )
    .unwrap();
}
```

Replace it with:

```rust
    )
    .unwrap();
}

#[test]
fn actual_section_caps_retarget_crossings_and_colour_without_changing_originals() {
    let (scene, owner) = crate::cap_gpu_tests::scene(false);
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    let geometry = r
        .meshes
        .iter()
        .map(|m| Rc::clone(&m.geometry))
        .collect::<Vec<_>>();
    let stats = r.stats();
    let sources = scene
        .objects()
        .iter()
        .map(|o| o.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let one = crate::frame_targets::Targets::new(&r.device, [320, 240], r.format, 1);
    let four = crate::frame_targets::Targets::new(&r.device, [320, 240], r.format, 4);
    let first = surface_frame(&r, &one, true, true);
    let surface1 = r.surfaces.active.0.clone();
    let root = std::path::Path::new("cap-pixels");
    let mut four_pixels = Vec::new();
    for count in [4, 1, 4] {
        r.surfaces.select(count, || {
            crate::surface_pipelines::pipelines(
                &r.device,
                r.format,
                &crate::surface_view::layout(&r.device),
                &crate::gpu_layout::uniform(
                    &r.device,
                    crate::object_record::BYTES as u64,
                ),
                count,
            )
        });
        r.instances
            .set_samples(&r.device, r.format, count);
        r.caps.borrow_mut().set_samples(count);
        let pixels = surface_frame(
            &r,
            if count == 1 {
                &one
            } else {
                &four
            },
            true,
            true,
        );
        let centre = crate::cap_gpu_tests::at(&pixels, 208, 96);
        assert!(
            centre[..3]
                .iter()
                .all(|v| (150..=155).contains(v)),
            "Actual closed cap remains grey at{count}x: {centre:?}"
        );
        if count == 1 {
            assert!(pixels == first);
            assert_eq!(r.surfaces.active.0, surface1);
        } else if four_pixels.is_empty() {
            four_pixels = pixels;
        } else {
            assert!(pixels == four_pixels);
        }
    }
    crate::sample_test_proof::write(root.join("solid-four.rgba"), &four_pixels)
        .unwrap();
    r.set_clipping(true, false);
    let hatch = surface_frame(&r, &four, true, true);
    assert!(hatch != four_pixels);
    assert!(
        hatch
            .chunks_exact(4)
            .filter(|p| p[..3].iter().all(|v| *v < 20))
            .count()
            > 400
    );
    crate::sample_test_proof::write(root.join("hatch-four.rgba"), hatch).unwrap();
    assert_eq!(r.stats(), stats);
    for ((gpu, before), source) in r.meshes.iter().zip(&geometry).zip(&sources) {
        assert!(Rc::ptr_eq(&gpu.geometry, before));
        assert_eq!(
            &scene
                .objects()
                .iter()
                .find(|o| o.id == gpu.id)
                .unwrap()
                .geometry()
                .unwrap()
                .to_proto(),
            source
        );
    }
    r.select_objects(&scene, &[owner]).unwrap();
    let selected = surface_frame(&r, &four, true, true);
    let centre = crate::cap_gpu_tests::at(&selected, 208, 96);
    assert!(centre[0] > 240 && centre[1] > 240 && centre[2] < 20);
    crate::sample_test_proof::write(root.join("selected-four.rgba"), selected).unwrap();
    let matrix = crate::cap_gpu_tests::matrix();
    r.request_pick([0.3, 0.2], matrix).unwrap();
    crate::cap_gpu_tests::wait(&r);
    assert_eq!(
        r.take_pick(matrix)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value(),
        "Single-sample closed-cap owner query is independent of4x crossing resources"
    );
}
```

**Locate this block:**

```rust
                &four
            },
            true,
        );
        assert!(
            pixels
```

Replace it with:

```rust
                &four
            },
            true,
            false,
        );
        assert!(
            pixels
```

**Locate this block:**

```rust
        .set_samples(&r.device, r.format, 4);
    let surface4 = r.surfaces.active.0.clone();
    assert_ne!(surface1, surface4);
    let shared = surface_frame(&r, &four, true);
    assert!(
        surface_frame(&r, &four, false) == shared,
        "Original uniforms and shared records have identical4x colour"
    );
    let mixed = actual_physical_samples(&r, &four);
```

Replace it with:

```rust
        .set_samples(&r.device, r.format, 4);
    let surface4 = r.surfaces.active.0.clone();
    assert_ne!(surface1, surface4);
    let shared = surface_frame(&r, &four, true, false);
    assert!(
        surface_frame(&r, &four, false, false) == shared,
        "Original uniforms and shared records have identical4x colour"
    );
    let mixed = actual_physical_samples(&r, &four);
```

**Locate this block:**

```rust
    let usage = r.instance_usage();
    let one = crate::frame_targets::Targets::new(&r.device, [320, 240], r.format, 1);
    let four = crate::frame_targets::Targets::new(&r.device, [320, 240], r.format, 4);
    let first = surface_frame(&r, &one, true);
    assert!(
        first
            .chunks_exact(4)
```

Replace it with:

```rust
    let usage = r.instance_usage();
    let one = crate::frame_targets::Targets::new(&r.device, [320, 240], r.format, 1);
    let four = crate::frame_targets::Targets::new(&r.device, [320, 240], r.format, 4);
    let first = surface_frame(&r, &one, true, false);
    assert!(
        first
            .chunks_exact(4)
```

**Locate this block:**

```rust
            r.instances.draw(&mut pass);
        }
    }
    target.resolve(&mut encoder, &view);
    let buffer = r
        .device
```

Replace it with:

```rust
            r.instances.draw(&mut pass);
        }
    }
    if sections {
        let clip = r.clipping.borrow();
        r.caps.borrow_mut().draw(
            &r.device,
            &mut encoder,
            target.color(&view),
            &target.primitive.view,
            &target.depth,
            &r.view_group,
            &clip.group,
            &r.meshes,
            clip.rows.len() as u32,
            Some(&r.instances),
        );
    }
    target.resolve(&mut encoder, &view);
    let buffer = r
        .device
```

**Locate this block:**

```rust
    r.clipping
        .borrow_mut()
        .frame_anchored(&r.queue, matrix, r.anchor(), 240, 1.);
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
```

Replace it with:

```rust
    r.clipping
        .borrow_mut()
        .frame_anchored(&r.queue, matrix, r.anchor(), 240, 1.);
    if sections {
        r.caps.borrow_mut().prepare(
            &r.device,
            &r.queue,
            r.format,
            [320, 240],
            true,
        );
    }
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
```

**Locate this block:**

```rust
            view_formats: &[],
        });
    let view = output.create_view(&Default::default());
    let matrix = crate::cap_gpu_tests::reversed(
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    );
    r.queue.write_buffer(
        &r.uniform,
        0,
```

Replace it with:

```rust
            view_formats: &[],
        });
    let view = output.create_view(&Default::default());
    let matrix = if sections {
        crate::cap_gpu_tests::matrix()
    } else {
        crate::cap_gpu_tests::reversed(
            session_rust::Xform::identity()
                .m
                .map(|v| v as f32),
        )
    };
    r.queue.write_buffer(
        &r.uniform,
        0,
```

**Locate this block:**

```rust
    shared: bool,
) -> Vec<u8> {
    let output = r
        .device
```

Replace it with:

```rust
    shared: bool,
    sections: bool,
) -> Vec<u8> {
    let output = r
        .device
```

## <a id="checkpoint-96zzzzzzzc-compose-cloud-depth-with-physical-addresses"></a>9. Compose cloud colour, depth and queries

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

### 1. `src/cloud_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &[&layouts.image, &layouts.window],
        "vertex",
        "pick",
        &[wgpu::TextureFormat::Rg32Uint],
    );
```

Replace it with:

```rust
        &[&layouts.image, &layouts.window],
        "vertex",
        "pick",
        &[
            wgpu::TextureFormat::Rg32Uint,
            crate::primitive_image::FORMAT,
        ],
    );
```

**Locate this block:**

```rust
    let resolve = pipeline(
        &ctx.device,
        &resolver,
        &[&layouts.image, &layouts.window],
        "vertex",
        "color",
        &[format],
    );
    let pick = pipeline(
        &ctx.device,
```

Replace it with:

```rust
    let resolve = pipeline(
        &ctx.device,
        &resolver,
        &[
            &layouts.image,
            &layouts.window,
            &layouts.uniform,
        ],
        "vertex",
        "color",
        &[format, crate::primitive_image::FORMAT],
    );
    let pick = pipeline(
        &ctx.device,
```

**Locate this block:**

```rust
        pass.set_pipeline(&self.resolve);
        pass.set_bind_group(0, &t.group, &[]);
        pass.set_bind_group(1, &self.window_group, &[]);
        pass.draw(0..3, 0..1);
    }
    pub fn pick(
```

Replace it with:

```rust
        pass.set_pipeline(&self.resolve);
        pass.set_bind_group(0, &t.group, &[]);
        pass.set_bind_group(1, &self.window_group, &[]);
        pass.set_bind_group(2, &self.settings_group, &[]);
        pass.draw(0..3, 0..1);
    }
    pub fn pick(
```

**Locate this block:**

```rust
            1.,
            size[0] as f32,
            size[1] as f32,
            0.,
            0.,
            0.,
            0.,
            0.,
            size[0] as f32,
            size[1] as f32,
            0.,
```

Replace it with:

```rust
            1.,
            size[0] as f32,
            size[1] as f32,
            self.edl,
            0.,
            0.,
            0.,
            0.,
            size[0] as f32,
            size[1] as f32,
            0.,
```

**Locate this block:**

```rust
            total: 0,
        }
    }
    pub fn set_clipping(&mut self, revision: u64) {
        self.clip_revision = revision;
    }
```

Replace it with:

```rust
            total: 0,
        }
    }
    pub fn set_edl(&mut self, ctx: &GpuCtx, value: f32) {
        let value = if value.is_finite() {
            value.max(0.)
        } else {
            0.
        };
        if value == self.edl {
            return;
        }
        self.edl = value;
        ctx.queue
            .write_buffer(&self.settings, 12, &value.to_ne_bytes());
    }
    pub fn set_clipping(&mut self, revision: u64) {
        self.clip_revision = revision;
    }
```

**Locate this block:**

```rust
            pick,
            settings,
            settings_group,
            columns_layout: layouts.columns,
            image_layout: layouts.image,
            window,
```

Replace it with:

```rust
            pick,
            settings,
            settings_group,
            edl: 0.25,
            columns_layout: layouts.columns,
            image_layout: layouts.image,
            window,
```

**Locate this block:**

```rust
    pick: wgpu::RenderPipeline,
    settings: wgpu::Buffer,
    settings_group: wgpu::BindGroup,
    columns_layout: wgpu::BindGroupLayout,
```

Replace it with:

```rust
    pick: wgpu::RenderPipeline,
    edl: f32,
    settings: wgpu::Buffer,
    settings_group: wgpu::BindGroup,
    columns_layout: wgpu::BindGroupLayout,
```

### 2. `src/cloud_resolve.wgsl`

**Locate this existing block:**

```wgsl
}
struct Pixel {@location(0) color:vec4<f32>,@builtin(frag_depth) depth:f32}
@fragment fn color(@builtin(position) p:vec4<f32>)->Pixel {
    let q=vec2<i32>(p.xy);let d=textureLoad(sdepth,q,0);if (d<=0.0) {discard;}
    return Pixel(textureLoad(scolor,q,0),d);
}
struct Pick {@location(0) owner:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment fn pick(@builtin(position) p:vec4<f32>)->Pick {
    let q=vec2<i32>(p.xy)+vec2<i32>(window.origin);let d=textureLoad(sdepth,q,0);if (d<=0.0) {discard;}
    let ids=textureLoad(sids,q,0).xy;
    var hit=vec2<u32>(0u);
    if (window.mode==0u) {hit=vec2<u32>(ids.x,0u);}
    if (window.mode==3u && ids.x!=0u && ids.y!=0u) {hit=vec2<u32>(ids.x,(0x40000000u|(ids.y-1u))+1u);}
    return Pick(hit,d);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}
struct Pixel {@location(0) color:vec4<f32>,@location(1) primitive:vec2u,@builtin(frag_depth) depth:f32}
fn log_depth(d:f32)->f32{return -log2(max(d,1.0e-7));}
@group(2) @binding(0) var<uniform> cloud:vec4f;
@fragment fn color(@builtin(position) p:vec4<f32>)->Pixel {
    let q=vec2<i32>(p.xy);let d=textureLoad(sdepth,q,0);if (d<=0.0) {discard;}
    var rgb=textureLoad(scolor,q,0).rgb;
    if(cloud.w>0.){
        let extent=vec2i(textureDimensions(sdepth));
        let me=log_depth(d);
        var sum=0.;
        let taps=array(vec2i(-1,0),vec2i(1,0),vec2i(0,-1),vec2i(0,1));
        for(var k=0;k<4;k++){
            let tap=q+taps[k];
            if(any(tap<vec2i(0))||any(tap>=extent)){continue;}
            let nd=textureLoad(sdepth,tap,0);
            if(nd==0.){continue;}
            sum+=max(0.,me-log_depth(nd));
        }
        rgb*=max(exp(-sum*75.*cloud.w),0.25);
    }

    return Pixel(vec4f(rgb,1.),vec2u(0u),d);
}
struct Pick {@location(0) owner:vec2<u32>,@location(1) primitive:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment fn pick(@builtin(position) p:vec4<f32>)->Pick {
    let q=vec2<i32>(p.xy)+vec2<i32>(window.origin);let d=textureLoad(sdepth,q,0);if (d<=0.0) {discard;}
    let ids=textureLoad(sids,q,0).xy;
    var hit=vec2<u32>(0u);
    if (window.mode==0u) {hit=vec2<u32>(ids.x,0u);}
    if (window.mode==3u && ids.x!=0u && ids.y!=0u) {hit=vec2<u32>(ids.x,(0x40000000u|(ids.y-1u))+1u);}
    return Pick(hit,vec2<u32>(0u),d);
}
```

### 3. `src/renderer.rs`

**Locate this existing block:**

```rust
        );
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cloud depth composite"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &self.targets.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                ..Default::default()
            });
            if let Some(clouds) = self.clouds.borrow().as_ref() {
                clouds.draw(&mut pass);
            }
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        );
        if let Some(clouds) = self.clouds.borrow().as_ref() {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cloud depth composite"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &self.targets.primitive.view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &self.targets.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    },
                ),
                ..Default::default()
            });
            clouds.draw(&mut pass);
        }
```

### 4. `src/sheet_gpu.rs`

**Locate this existing block:**

```rust
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
            pass.set_vertex_buffer(0, draw.geometry.vertices.slice(..));
            pass.draw(0..6, 0..draw.geometry.count);
        }
    }
    pub fn pick(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        crop: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
    ) {
        pass.set_pipeline(&self.pick);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
            pass.set_vertex_buffer(0, draw.geometry.vertices.slice(..));
            pass.draw(0..6, 0..draw.geometry.count);
        }
    }
    pub fn pick(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        crop: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
    ) {
        if self.draws.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pick);
```

### 5. `src/finite_visibility.wgsl`

**Locate this existing block:**

```wgsl
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(ink_primitive(source_at,sample)==0u&&ink_depth(source_at,sample)>axis.depth+tolerance){return false;}
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
```

### 6. `src/pick_encode.rs`

**Locate this existing block:**

```rust
    }
    let tables = tables.map(|tables| crate::finite_ink::Tables {
        projected: tables.projected.clone(),
        tiles: tables.tiles.clone(),
        primitive: image.primitive.view.clone(),
    });
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("finite integer ink"),
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
        pass.set_bind_group(3, clipping, &[]);
        for lane in ink {
            lane.pick_finite(
                device,
                &mut pass,
                view_group,
                crop_group,
                tables.as_ref(),
                &image.depth,
            );
        }
    }
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("other original integer owners"),
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
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        if let Some(clouds) = clouds {
            clouds.pick(ctx, &mut pass, window, mode);
        }
        if let Some(sheets) = sheets {
            sheets.pick(&mut pass, crop_group, clipping);
        }
        if let Some(text) = text {
            text.draw_ids(&mut pass, crop_group);
        }
    }
    // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &image.texture,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: window.copy[0] - window.draw[0],
                y: window.copy[1] - window.draw[1],
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &image.readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(window.row_bytes()),
                rows_per_image: Some(window.copy[3]),
            },
        },
        wgpu::Extent3d {
            width: window.copy[2],
            height: window.copy[3],
            depth_or_array_layers: 1,
        },
    );
    encoder.finish()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    if let Some(clouds) = clouds {
        let mut pass = overlay(
            &mut encoder,
            image,
            "original cloud query depth",
            true,
        );
        clouds.pick(ctx, &mut pass, window, mode);
    }
    let tables = tables.map(|tables| crate::finite_ink::Tables {
        projected: tables.projected.clone(),
        tiles: tables.tiles.clone(),
        primitive: image.primitive.view.clone(),
    });
    {
        let mut pass = overlay(
            &mut encoder,
            image,
            "finite integer ink",
            false,
        );
        pass.set_bind_group(3, clipping, &[]);
        for lane in ink {
            lane.pick_finite(
                device,
                &mut pass,
                view_group,
                crop_group,
                tables.as_ref(),
                &image.depth,
            );
        }
    }
    {
        let mut pass = overlay(
            &mut encoder,
            image,
            "other original integer owners",
            false,
        );
        if let Some(sheets) = sheets {
            sheets.pick(&mut pass, crop_group, clipping);
        }
        if let Some(text) = text {
            text.draw_ids(&mut pass, crop_group);
        }
    }
    // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &image.texture,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: window.copy[0] - window.draw[0],
                y: window.copy[1] - window.draw[1],
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &image.readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(window.row_bytes()),
                rows_per_image: Some(window.copy[3]),
            },
        },
        wgpu::Extent3d {
            width: window.copy[2],
            height: window.copy[3],
            depth_or_array_layers: 1,
        },
    );
    encoder.finish()
}

fn overlay<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    image: &'a Image,
    label: &'a str,
    physical: bool,
) -> wgpu::RenderPass<'a> {
    let colours = [
        Some(wgpu::RenderPassColorAttachment {
            view: &image.colour,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
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
    ];
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(label),
        color_attachments: &colours[..if physical {
            2
        } else {
            1
        }],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &image.depth,
            depth_ops: physical.then_some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        ..Default::default()
    })
}
```

### 7. `src/cloud_address_tests.rs`

Create the file and type:

```rust
use super::*;
use std::rc::Rc;

pub(super) fn retarget(_r: &mut Renderer, count: u32) {
    assert_eq!(count, 1);
}
fn physical(r: &Renderer) -> Vec<[u32; 4]> {
    let count = r.targets.samples;
    let mut code = format!(
        r#"
@group(0) @binding(0) var ids: texture_2d<u32>;
@group(0) @binding(1) var depth: texture_depth_2d;
@group(0) @binding(2) var<storage,read_write> output: array<vec4u>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) gid: vec3u) {{
 if gid.x >= {count}u {{return;}}
 let at = vec2i(208,96);
 output[gid.x] = vec4u(textureLoad(ids,at,SAMPLE).xy,bitcast<u32>(textureLoad(depth,at,SAMPLE)),gid.x);
}}
"#
    );
    if count == 4 {
        code = code
            .replace(
                "texture_2d<u32>",
                "texture_multisampled_2d<u32>",
            )
            .replace(
                "texture_depth_2d",
                "texture_depth_multisampled_2d",
            );
    }
    code = code.replace(
        "SAMPLE",
        if count == 4 {
            "i32(gid.x)"
        } else {
            "0"
        },
    );
    let module = r
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual cloud physical address witness"),
            source: wgpu::ShaderSource::Wgsl(code.into()),
        });
    let pipeline = r
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("read_samples"),
            compilation_options: Default::default(),
            cache: None,
        });
    let output = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
    let read = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let group = r
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &r.targets.primitive.view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&r.targets.depth),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: output.as_entire_binding(),
                },
            ],
        });
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &read, 0, 64);
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    crate::cap_gpu_tests::wait(r);
    rx.recv().unwrap().unwrap();
    let values = read
        .slice(..)
        .get_mapped_range()
        .chunks_exact(16)
        .take(count as usize)
        .map(|b| {
            std::array::from_fn(|i| {
                u32::from_ne_bytes(b[i * 4..i * 4 + 4].try_into().unwrap())
            })
        })
        .collect();
    read.unmap();
    values
}
#[test]
fn opaque_cloud_depth_clears_occluded_physical_triangle_and_cap_addresses() {
    for clipped in [false, true] {
        for count in [1] {
            let (mut scene, _) = crate::cap_gpu_tests::scene(false);
            if clipped {
                let plane = crate::clipping_geometry::plane_from(
                    crate::clipping_geometry::Mode::Points,
                    &[
                        [0., 0., -0.3],
                        [1., 0., -1.3],
                        [0., 1., -0.3],
                    ],
                    2.,
                )
                .unwrap();
                crate::plane_original::insert(&mut scene, Rc::new(plane)).unwrap();
            }
            let mut r = crate::cap_gpu_tests::renderer(&scene);
            r.set_clipping(clipped, true);
            retarget(&mut r, count);
            let matrix = crate::cap_gpu_tests::matrix();
            let before_pixels = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let before = physical(&r);
            for value in &before {
                let address = value[0] | (value[1] << 16);
                assert!(address > 0);
                assert_eq!(
                    address >= crate::primitive_image::CAP_START,
                    clipped
                );
            }
            let coords = if clipped {
                vec![0.28, 0.2, -0.585]
            } else {
                vec![0.3, 0.2, -0.1]
            };
            let mut source = session_rust::PointCloud::from_coords(
                coords,
                vec![0, 0, 255, 255],
                vec![],
            );
            source.point_size = 28.;
            let source = Rc::new(source);
            let _ = source.guid();
            let original = source.to_proto();
            let owner = scene
                .insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap())
                .unwrap();
            r.set_scene(&scene, None);
            r.set_clipping(clipped, true);
            let after_pixels = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let after = physical(&r);
            crate::sample_test_proof::write(
                format!("cloud-address-{clipped}-{count}.json"),
                serde_json::to_vec_pretty(
                    &serde_json::json!({"before": before, "after": after}),
                )
                .unwrap(),
            )
            .unwrap();
            crate::sample_test_proof::write(
                format!("cloud-address-{clipped}-{count}.rgba"),
                &after_pixels,
            )
            .unwrap();
            assert!(before_pixels != after_pixels);
            let rgba = crate::cap_gpu_tests::at(&after_pixels, 208, 96);
            assert!(
                rgba[2] > 250 && rgba[0] < 5 && rgba[1] < 5,
                "The actual cloud lies in front of the original face/cap"
            );
            for (prior, current) in before.iter().zip(&after) {
                assert!(
                    f32::from_bits(current[2]) > f32::from_bits(prior[2]),
                    "Cloud writes its nearer actual depth"
                );
                assert_eq!(
                    [current[0], current[1]],
                    [0, 0],
                    "Cloud depth must not retain an occluded triangle or cap address"
                );
            }
            r.request_pick([0.3, 0.2], matrix).unwrap();
            crate::cap_gpu_tests::wait(&r);
            assert_eq!(
                r.take_pick(matrix)
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                owner.value()
            );
            assert_eq!(source.to_proto(), original);
        }
    }
}

#[test]
fn actual_nearer_cloud_hides_original_ink_without_hiding_front_sources() {
    let mut rows = Vec::new();
    for density in [1., 2.] {
        for count in [1] {
            let mut scene = Scene::empty();
            let face = scene
                .insert(
                    crate::prepared::PreparedMesh::triangle(
                        [
                            [-2., -2., 0.6],
                            [2., -2., 0.6],
                            [0., 2., 0.6],
                        ],
                        [0.1, 0.6, 0.1],
                    )
                    .unwrap(),
                )
                .unwrap();
            let mut line = session_rust::Line::new(-0.4, 0.2, 0.6, 0.6, 0.2, 0.6);
            line.width = 6.;
            line.linecolor = session_rust::Color::new(1., 0., 0., 1.);
            let line = Rc::new(line);
            let _ = line.guid();
            let line_before = line.to_proto();
            let ink = scene
                .insert_line(crate::stroke::PreparedLine::new(line.clone()).unwrap())
                .unwrap();
            let mut r = crate::cap_gpu_tests::renderer(&scene);
            r.set_density(density);
            retarget(&mut r, count);
            let matrix = crate::cap_gpu_tests::reversed(
                session_rust::Xform::identity()
                    .m
                    .map(|v| v as f32),
            );
            let before = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let z_before = physical(&r);
            let pixel = crate::cap_gpu_tests::at(&before, 208, 96);
            assert!(
                pixel[0] > 240 && pixel[1] < 10 && pixel[2] < 10,
                "Original front red line exists before cloud: {pixel:?}"
            );
            let coords = (-3..=3)
                .flat_map(|y| {
                    (-3..=3).flat_map(move |x| {
                        [
                            0.3 + x as f64 * 0.03,
                            0.2 + y as f64 * 0.03,
                            0.2,
                        ]
                    })
                })
                .collect();
            let mut points = session_rust::PointCloud::from_coords(
                coords,
                [0, 0, 255, 255].repeat(49),
                vec![],
            );
            points.point_size = 96.;
            let points = Rc::new(points);
            let _ = points.guid();
            let cloud_before = points.to_proto();
            let cloud = scene
                .insert_cloud(crate::cloud::Prepared::new(points.clone()).unwrap())
                .unwrap();
            r.set_scene(&scene, None);
            let actual = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let z_after = physical(&r);
            for (prior, current) in z_before.iter().zip(&z_after) {
                assert!(f32::from_bits(current[2]) > f32::from_bits(prior[2]));
                assert_eq!([current[0], current[1]], [0, 0]);
            }
            let pixel = crate::cap_gpu_tests::at(&actual, 208, 96);
            assert!(
                pixel[2] > 240 && pixel[0] < 10 && pixel[1] < 10,
                "Nearer opaque blue cloud hides the original red line: {pixel:?}"
            );
            r.request_pick([0.3, 0.2], matrix).unwrap();
            crate::cap_gpu_tests::wait(&r);
            let picked = r.take_pick(matrix).unwrap().unwrap();
            assert_eq!(picked.unwrap()[0], cloud.value());
            let geometry = r.meshes[0].geometry.clone();
            let cloud_uploads = r.cloud_usage()[2];
            scene.translate(ink, [0., 0., -0.5]).unwrap();
            r.set_scene(&scene, None);
            let front = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let visible = crate::cap_gpu_tests::at(&front, 208, 96);
            assert!(
                visible[0] > 240 && visible[1] < 10 && visible[2] < 10,
                "A genuinely nearer original line remains visible: {visible:?}"
            );
            r.request_pick([0.3, 0.2], matrix).unwrap();
            crate::cap_gpu_tests::wait(&r);
            assert_eq!(
                r.take_pick(matrix)
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                ink.value()
            );
            scene.translate(ink, [0., 0., 0.5]).unwrap();
            r.set_scene(&scene, None);
            assert!(
                crate::cap_gpu_tests::pixels_at(&r, matrix) == actual,
                "Restoring original placement restores exact hidden-ink pixels"
            );
            assert_eq!(
                r.cloud_usage()[2],
                cloud_uploads,
                "Original cloud source upload is retained through line placement changes"
            );
            let cached = r.cloud_usage()[..4].to_vec();
            assert!(
                crate::cap_gpu_tests::pixels_at(&r, matrix) == actual,
                "Fresh owner queries retain exact repeated-frame pixels"
            );
            assert!(Rc::ptr_eq(&r.meshes[0].geometry, &geometry));
            assert_eq!(r.cloud_usage()[..4], cached);
            crate::sample_test_proof::write(
                format!("cloud-ink-front-{density}-{count}.rgba"),
                &front,
            )
            .unwrap();
            assert_eq!(line.to_proto(), line_before);
            assert_eq!(points.to_proto(), cloud_before);
            crate::sample_test_proof::write(
                format!("cloud-ink-fixed-{density}-{count}.rgba"),
                &actual,
            )
            .unwrap();
            rows.push(serde_json::json!({"density":density,"samples":count,"physicalBefore":z_before,"physicalAfter":z_after,"actualRgba":pixel,"freshOwner":picked,"cloud":cloud.value(),"ink":ink.value(),"face":face.value()}));
        }
    }
    for density in [1., 2.] {
        for count in [1] {
            let mut scene = Scene::empty();
            scene
                .insert(
                    crate::prepared::PreparedMesh::triangle(
                        [
                            [-2., -2., 0.6],
                            [2., -2., 0.6],
                            [0., 2., 0.6],
                        ],
                        [0.1, 0.6, 0.1],
                    )
                    .unwrap(),
                )
                .unwrap();
            let mut source = session_rust::Line::new(-0.4, 0., 0.2, 0.6, 0., 0.2);
            source.width = 60.;
            source.linecolor = session_rust::Color::new(1., 0., 0., 1.);
            let owner = scene
                .insert_line(crate::stroke::PreparedLine::new(Rc::new(source)).unwrap())
                .unwrap();
            let mut r = crate::cap_gpu_tests::renderer(&scene);
            r.set_density(density);
            retarget(&mut r, count);
            let matrix = crate::cap_gpu_tests::reversed(
                session_rust::Xform::identity()
                    .m
                    .map(|v| v as f32),
            );
            let actual = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let visible = crate::cap_gpu_tests::at(&actual, 208, 96);
            assert!(
                visible[0] > 240 && visible[1] < 10 && visible[2] < 10,
                "Actual wide source fringe is visible outside its query-axis halo: {visible:?}"
            );
            r.request_pick([0.3, 0.2], matrix).unwrap();
            crate::cap_gpu_tests::wait(&r);
            let picked = r.take_pick(matrix).unwrap().unwrap();
            crate::sample_test_proof::write(
                format!("cloud-ink-wide-fringe-{density}-{count}.rgba"),
                &actual,
            )
            .unwrap();
            crate::sample_test_proof::write(format!("cloud-ink-wide-fringe-{density}-{count}.json"),serde_json::to_vec_pretty(&serde_json::json!({"density":density,"samples":count,"pixel":visible,"freshOwner":picked,"expected":owner.value()})).unwrap()).unwrap();
            assert_eq!(
                picked.unwrap()[0],
                owner.value(),
                "A visible wide source fringe retains its original owner even when its axis lies outside the query halo"
            );
        }
    }

    crate::sample_test_proof::write(
        "cloud-ink-fixed.json",
        serde_json::to_vec_pretty(&rows).unwrap(),
    )
    .unwrap();
}
```

### 8. `src/cloud_composite_reference.rs`

Create the file and type:

```rust
use super::*;
pub(crate) fn pixels(
    lane: &Lane,
    ctx: &GpuCtx,
    samples: u32,
    reference: bool,
    edl: f32,
) -> Vec<u8> {
    ctx.queue
        .write_buffer(&lane.settings, 12, &edl.to_ne_bytes());
    let size = [320, 240];
    let format = wgpu::TextureFormat::Rgba8Unorm;
    let target = crate::frame_targets::Targets::new(&ctx.device, size, format, samples);
    let output = ctx
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("independent cloud composite"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    let view = output.create_view(&Default::default());
    let reference_pipeline = reference.then(|| {
        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("exact maintained cloud resolve witness"),
                source: wgpu::ShaderSource::Wgsl(PRIMARY.into()),
            });
        let uniform = crate::gpu_layout::uniform(&ctx.device, 48);
        let layout =
            ctx.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[Some(&uniform), Some(&lane.image_layout)],
                    immediate_size: 0,
                });
        ctx.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("exact maintained cloud composite"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rg16Uint,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
                }),
                primitive: Default::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    depth_compare: Some(crate::depth::CLOSER_EQUAL),
                    ..crate::depth::surface()
                }),
                multisample: wgpu::MultisampleState {
                    count: samples,
                    ..Default::default()
                },
                multiview_mask: None,
                cache: None,
            })
    });
    let mut encoder = ctx
        .device
        .create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("actual cached point images under two independent composites"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: target.color(&view),
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &target.primitive.view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        if let Some(pipeline) = &reference_pipeline {
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &lane.settings_group, &[]);
            pass.set_bind_group(1, &lane.targets.as_ref().unwrap().group, &[]);
            pass.draw(0..3, 0..1);
        } else {
            lane.draw(&mut pass);
        }
    }
    target.resolve(&mut encoder, &view);
    let read = ctx
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 320 * 240 * 4,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    encoder.copy_texture_to_buffer(
        output.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &read,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280),
                rows_per_image: Some(240),
            },
        },
        output.size(),
    );
    ctx.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    ctx.device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(10)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let data = read.slice(..).get_mapped_range().to_vec();
    read.unmap();
    data
}

const PRIMARY: &str = r##"// Point cloud settings, 48 bytes; matches CloudUniform in Rust.
struct CloudUniform {
    size: f32, // point size scale; applied on the CPU
    vp_w: f32, // target width, px
    vp_h: f32, // target height, px
    edl: f32, // eye-dome lighting strength; 0 = off
    _pad0: f32, // padding
    _pad1: f32, // padding
    origin: vec2<f32>, // top-left of this target in the canvas, px
    frame: vec2<f32>, // canvas size, px
};

@group(0) @binding(0) var<uniform> cloud: CloudUniform; // cloud settings
@group(1) @binding(0) var sdepth: texture_depth_2d; // nearest point depth per pixel
@group(1) @binding(1) var scolor: texture_2d<f32>; // its color

// One vertex of the fullscreen triangle.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
};

@vertex
// Place the three corners of a screen-covering triangle.
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var o: VsOut;
    let x = f32(i32(vid & 1u) * 4 - 1);
    let y = f32(i32(vid >> 1u) * 4 - 1);
    o.pos = vec4<f32>(x, y, 0.0, 1.0);
    return o;
}

// Output: color, triangle id, and the depth written to the scene.
struct FsOut {
    @location(0) color: vec4<f32>, // rgba
    @location(1) primitive: vec2<u32>, // triangle id; none for points
    @builtin(frag_depth) depth: f32, // point depth into the scene
};

// Depth on a log scale that grows with distance.
fn log_depth(d: f32) -> f32 {
    return -log2(max(d, 1.0e-7));
}

// Copy the point texture into the scene, darkening edges when EDL is on.
fn shade(in: VsOut) -> FsOut {
    let pix = vec2<i32>(in.pos.xy);
    let d = textureLoad(sdepth, pix, 0);

    // no point here
    if (d == 0.0) {
        discard;
    }

    var o: FsOut;
    var rgb = textureLoad(scolor, pix, 0).rgb;

    // eye-dome lighting: darken where neighbours are nearer
    if (cloud.edl > 0.0) {
        let w = i32(cloud.vp_w);
        let h = i32(cloud.vp_h);
        let me = log_depth(d);
        var sum = 0.0;
        var taps = array<vec2<i32>, 4>(vec2<i32>(-1, 0), vec2<i32>(1, 0), vec2<i32>(0, -1), vec2<i32>(0, 1));

        for (var k = 0; k < 4; k++) {
            let q = pix + taps[k];

            if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) {
                continue;
            }

            let nd = textureLoad(sdepth, q, 0);

            if (nd == 0.0) {
                continue;
            }

            sum += max(0.0, me - log_depth(nd));
        }

        // darken by the depth steps, never below a quarter
        let shade = max(exp(-sum * 75.0 * cloud.edl), 0.25);
        rgb *= shade;
    }

    o.color = vec4<f32>(rgb, 1.0);
    o.depth = d;
    o.primitive = vec2<u32>(0u);
    return o;
}

@fragment
// Resolve one pixel.
fn fs_main(in: VsOut) -> FsOut {
    return shade(in);
}
"##;
```

### 9. `src/cloud_composite_tests.rs`

Create the file and type:

```rust
use super::*;
use std::rc::Rc;
#[test]
fn actual_cloud_composite_matches_current_opaque_and_eye_dome_pixels() {
    let mut records = Vec::new();
    let mut alpha_differences = 0;
    let mut shaded_differences = 0;
    for edges in [false, true] {
        for alpha in [0u8, 255] {
            let mut coords = Vec::new();
            let mut colors = Vec::new();
            for y in 0..30 {
                for x in 0..40 {
                    coords.extend([
                        if edges {
                            -1.05 + x as f64 * 0.055
                        } else {
                            -0.8 + x as f64 * 0.04
                        },
                        if edges {
                            -1.02 + y as f64 * 0.07
                        } else {
                            -0.6 + y as f64 * 0.04
                        },
                        if (x / 4 + y / 4) % 2 == 0 {
                            0.15
                        } else {
                            0.35
                        },
                    ]);
                    colors.extend([80, 140, 220, i32::from(alpha)]);
                }
            }
            let mut cloud =
                session_rust::PointCloud::from_coords(coords, colors, vec![]);
            cloud.point_size = 10.;
            let cloud = Rc::new(cloud);
            let _ = cloud.guid();
            let original = cloud.to_proto();
            let mut scene = Scene::empty();
            let owner = scene
                .insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap())
                .unwrap();
            let mut r = crate::cap_gpu_tests::renderer(&scene);
            let ctx = r.text_context();
            let matrix = crate::cap_gpu_tests::reversed(
                session_rust::Xform::identity()
                    .m
                    .map(|v| v as f32),
            );
            for samples in [1] {
                super::cloud_address_tests::retarget(&mut r, samples);
                let default_pixels = crate::cap_gpu_tests::pixels_at(&r, matrix);
                let usage = r.cloud_usage();
                for edl in [0., 0.25] {
                    r.clouds
                        .borrow_mut()
                        .as_mut()
                        .unwrap()
                        .set_edl(&ctx, edl);
                    let full = crate::cap_gpu_tests::pixels_at(&r, matrix);
                    let lane = r.clouds.borrow();
                    let lane = lane.as_ref().unwrap();
                    let actual = crate::cloud_gpu::composite_reference::pixels(
                        lane, &ctx, samples, false, edl,
                    );
                    let reference = crate::cloud_gpu::composite_reference::pixels(
                        lane, &ctx, samples, true, edl,
                    );
                    assert_eq!(
                        full, reference,
                        "Whole rendered cloud equals independent maintained compositor at both counts and EDL modes"
                    );
                    assert_eq!(
                        actual, reference,
                        "Every RGBA channel matches exact maintained WGSL"
                    );
                    let rgba = actual
                        .chunks_exact(4)
                        .zip(reference.chunks_exact(4))
                        .filter(|(a, b)| a != b)
                        .count();
                    let rgb = actual
                        .chunks_exact(4)
                        .zip(reference.chunks_exact(4))
                        .filter(|(a, b)| a[..3] != b[..3])
                        .count();
                    let a = actual
                        .chunks_exact(4)
                        .zip(reference.chunks_exact(4))
                        .filter(|(a, b)| a[3] != b[3])
                        .count();
                    if alpha == 0 && edl == 0. {
                        assert_eq!(a, 0);
                        alpha_differences += a;
                    }
                    if alpha == 255 && edl > 0. {
                        assert_eq!(rgb, 0);
                        shaded_differences += rgb;
                    }
                    if alpha == 255 && edl == 0. {
                        assert_eq!(rgba, 0);
                    }
                    if edl == 0.25 {
                        let default_diff = default_pixels
                            .chunks_exact(4)
                            .zip(reference.chunks_exact(4))
                            .filter(|(a, b)| a != b)
                            .count();
                        assert_eq!(
                            default_diff, 0,
                            "Default whole viewer uses maintained eye-dome shading and opaque cloud output"
                        );
                    }
                    if !edges
                        && ((alpha == 0 && edl == 0. && samples == 1)
                            || (alpha == 255 && edl > 0. && samples == 1))
                    {
                        for (name, bytes) in [
                            ("actual", &actual),
                            ("reference", &reference),
                        ] {
                            crate::sample_test_proof::write(
                                format!("cloud-parity-{alpha}-{samples}-{name}.rgba"),
                                bytes,
                            )
                            .unwrap();
                        }
                    }
                    records.push(serde_json::json!({"viewportEdges":edges,"sourceAlpha":alpha,"samples":samples,"edl":edl,"rgbaDifferences":rgba,"rgbDifferences":rgb,"alphaDifferences":a}));
                }
                assert_eq!(&r.cloud_usage()[..4], &usage[..4]);
                r.request_pick([0., 0.], matrix).unwrap();
                crate::cap_gpu_tests::wait(&r);
                assert_eq!(
                    r.take_pick(matrix)
                        .unwrap()
                        .unwrap()
                        .unwrap()[0],
                    owner.value()
                );
                assert_eq!(cloud.to_proto(), original);
            }
        }
    }
    assert_eq!(
        (alpha_differences, shaded_differences),
        (0, 0)
    );
    crate::sample_test_proof::write(
        "cloud-parity-observed.json",
        serde_json::to_vec_pretty(&records).unwrap(),
    )
    .unwrap();
}
```

### 10. `src/renderer.rs`

**Locate this existing block:**

```rust
#[path = "sample_surface_tests.rs"]
mod sample_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "sample_surface_tests.rs"]
mod sample_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "cloud_address_tests.rs"]
mod cloud_address_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "cloud_composite_tests.rs"]
mod cloud_composite_tests;
```

### 11. `src/cloud_gpu.rs`

**Locate this existing block:**

```rust
        }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "cloud_composite_reference.rs"]
pub(crate) mod composite_reference;
```

## <a id="checkpoint-96zzzzzzzca-cache-cloud-and-sheet-samples"></a>10. Cache cloud and sheet sample recipes

### 1. `src/cloud_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            wgpu::TextureFormat::Rg32Uint,
            crate::primitive_image::FORMAT,
        ],
    );
    [point, resolve, pick]
}
```

Replace it with:

```rust
            wgpu::TextureFormat::Rg32Uint,
            crate::primitive_image::FORMAT,
        ],
        1,
    );
    ([point, resolve, pick], resolver)
}
```

**Locate this block:**

```rust
        "vertex",
        "color",
        &[format, crate::primitive_image::FORMAT],
    );
    let pick = pipeline(
        &ctx.device,
```

Replace it with:

```rust
        "vertex",
        "color",
        &[format, crate::primitive_image::FORMAT],
        1,
    );
    let pick = pipeline(
        &ctx.device,
```

**Locate this block:**

```rust
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Rg32Uint,
        ],
    );
    let resolve = pipeline(
        &ctx.device,
```

Replace it with:

```rust
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Rg32Uint,
        ],
        1,
    );
    let resolve = pipeline(
        &ctx.device,
```

**Locate this block:**

```rust
    ctx: &GpuCtx,
    format: wgpu::TextureFormat,
    layouts: &Layouts,
) -> [wgpu::RenderPipeline; 3] {
    let shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
```

Replace it with:

```rust
    ctx: &GpuCtx,
    format: wgpu::TextureFormat,
    layouts: &Layouts,
) -> ([wgpu::RenderPipeline; 3], wgpu::ShaderModule) {
    let shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
```

**Locate this block:**

```rust
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
```

Replace it with:

```rust
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
```

**Locate this block:**

```rust
    vertex: &str,
    fragment: &str,
    formats: &[wgpu::TextureFormat],
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(fragment),
```

Replace it with:

```rust
    vertex: &str,
    fragment: &str,
    formats: &[wgpu::TextureFormat],
    samples: u32,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(fragment),
```

**Locate this block:**

```rust
        let Some(t) = &self.targets else {
            return;
        };
        pass.set_pipeline(&self.resolve);
        pass.set_bind_group(0, &t.group, &[]);
        pass.set_bind_group(1, &self.window_group, &[]);
        pass.set_bind_group(2, &self.settings_group, &[]);
```

Replace it with:

```rust
        let Some(t) = &self.targets else {
            return;
        };
        pass.set_pipeline(&self.resolve.active);
        pass.set_bind_group(0, &t.group, &[]);
        pass.set_bind_group(1, &self.window_group, &[]);
        pass.set_bind_group(2, &self.settings_group, &[]);
```

**Locate this block:**

```rust
                group,
            });
        }
        self.render_points(ctx, size);
        self.key = Some(key);
        self.preparations += 1;
```

Replace it with:

```rust
                group,
            });
        }
        self.set_samples(ctx, self.samples);
        self.render_points(ctx, size);
        self.key = Some(key);
        self.preparations += 1;
```

**Locate this block:**

```rust
            total: 0,
        }
    }
    pub fn set_edl(&mut self, ctx: &GpuCtx, value: f32) {
        let value = if value.is_finite() {
            value.max(0.)
```

Replace it with:

```rust
            total: 0,
        }
    }
    pub fn set_samples(&mut self, ctx: &GpuCtx, samples: u32) {
        assert!(matches!(samples, 1 | 4));
        self.samples = samples;
        if self.targets.is_none() {
            return;
        }
        self.resolve.select(samples, || {
            pipeline(
                &ctx.device,
                &self.resolver,
                &[
                    &self.image_layout,
                    &self.pick.get_bind_group_layout(1),
                    &self.point.get_bind_group_layout(0),
                ],
                "vertex",
                "color",
                &[self.format, crate::primitive_image::FORMAT],
                samples,
            )
        });
    }
    pub fn set_edl(&mut self, ctx: &GpuCtx, value: f32) {
        let value = if value.is_finite() {
            value.max(0.)
```

**Locate this block:**

```rust
            "cloud pick origin",
            [(0, window.as_entire_binding())],
        );
        let [point, resolve, pick] = pipelines(ctx, format, &layouts);
        Self {
            clipping: clipping.clone(),
            clip_revision: 0,
            point,
            resolve,
            pick,
            settings,
            settings_group,
```

Replace it with:

```rust
            "cloud pick origin",
            [(0, window.as_entire_binding())],
        );
        let ([point, resolve, pick], resolver) = pipelines(ctx, format, &layouts);
        Self {
            clipping: clipping.clone(),
            clip_revision: 0,
            point,
            resolve: Variants::new(resolve, 1),
            resolver,
            format,
            samples: 1,
            pick,
            settings,
            settings_group,
```

**Locate this block:**

```rust
    clipping: wgpu::BindGroup,
    clip_revision: u64,
    point: wgpu::RenderPipeline,
    resolve: wgpu::RenderPipeline,
    pick: wgpu::RenderPipeline,
    edl: f32,
    settings: wgpu::Buffer,
```

Replace it with:

```rust
    clipping: wgpu::BindGroup,
    clip_revision: u64,
    point: wgpu::RenderPipeline,
    resolve: Variants<wgpu::RenderPipeline>,
    resolver: wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    samples: u32,
    pick: wgpu::RenderPipeline,
    edl: f32,
    settings: wgpu::Buffer,
```

**Locate this block:**

```rust
use crate::{
    cloud::Row,
    cloud_lod::{LodWalk, Projection},
```

Replace it with:

```rust
use crate::sample_pipelines::Variants;
use crate::{
    cloud::Row,
    cloud_lod::{LodWalk, Projection},
```

### 2. `src/renderer.rs`

**Locate this existing block:**

```rust
        });
        lane.set_scene(&self.text_context(), scene, selected)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        });
        lane.set_samples(&self.text_context(), self.targets.samples);
        lane.set_scene(&self.text_context(), scene, selected)
```

### 3. `src/sheet_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .collect::<Vec<_>>();
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
```

Replace it with:

```rust
            .collect::<Vec<_>>();
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn set_samples(&mut self, device: &wgpu::Device, samples: u32) {
        assert!(matches!(samples, 1 | 4));
        self.samples = samples;
        if self.draws.is_empty() {
            return;
        }
        self.pipeline.select(samples, || {
            crate::sheet_pipeline::colour(device, self.format, samples)
        });
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty() {
            return;
        }

        pass.set_pipeline(&self.pipeline.active);
        pass.set_bind_group(0, &self.group, &[]);
```

**Locate this block:**

```rust
        drop(old);
        self.cache
            .retain(|_, owner| owner.strong_count() > 0);
        Ok(())
    }
    pub fn camera(&self, camera: &crate::camera::Camera) {
```

Replace it with:

```rust
        drop(old);
        self.cache
            .retain(|_, owner| owner.strong_count() > 0);
        self.set_samples(device, self.samples);
        Ok(())
    }
    pub fn camera(&self, camera: &crate::camera::Camera) {
```

**Locate this block:**

```rust
                        );
                        let group = crate::gpu_layout::group(
                            device,
                            &self.pipeline.get_bind_group_layout(1),
                            &uniform,
                        );
                        Draw {
```

Replace it with:

```rust
                        );
                        let group = crate::gpu_layout::group(
                            device,
                            &self.pipeline.active.get_bind_group_layout(1),
                            &uniform,
                        );
                        Draw {
```

**Locate this block:**

```rust
        );
        Self {
            projection: Default::default(),
            pipeline,
            pick,
            view,
            group,
```

Replace it with:

```rust
        );
        Self {
            projection: Default::default(),
            pipeline: crate::sample_pipelines::Variants::new(pipeline, 1),
            format,
            samples: 1,
            pick,
            view,
            group,
```

**Locate this block:**

```rust
    projection: std::cell::Cell<Option<f32>>,
    pipeline: wgpu::RenderPipeline,
    pick: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    group: wgpu::BindGroup,
```

Replace it with:

```rust
    projection: std::cell::Cell<Option<f32>>,
    pipeline: crate::sample_pipelines::Variants<wgpu::RenderPipeline>,
    format: wgpu::TextureFormat,
    samples: u32,
    pick: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    group: wgpu::BindGroup,
```

### 4. `src/sheet_pipeline.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "pick_fragment",
        wgpu::TextureFormat::Rg32Uint,
        None,
    );
    (pipeline, pick)
}
```

Replace it with:

```rust
        "pick_fragment",
        wgpu::TextureFormat::Rg32Uint,
        None,
        1,
    );
    (pipeline, pick)
}

pub fn colour(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    samples: u32,
) -> wgpu::RenderPipeline {
    let view_layout = crate::gpu_layout::uniform(device, 80);
    let model_layout = crate::gpu_layout::uniform(device, 96);
    let clip_layout = crate::clipping_gpu::layout(device);
    let expanded = crate::finite_project_source::expand(include_str!("sheet.wgsl"));
    let expanded = if samples == 4 {
        expanded
            .replace(
                "@interpolate(linear)",
                "@interpolate(linear,sample)",
            )
            .replace(
                "@location(8) original_clip:",
                "@location(8) @interpolate(perspective,sample) original_clip:",
            )
    } else {
        expanded
    };
    let source = expanded.as_str();
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sheet pens"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(source, 2, 0).into(),
        ),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sheet pens"),
        bind_group_layouts: &[
            Some(&view_layout),
            Some(&model_layout),
            Some(&clip_layout),
        ],
        immediate_size: 0,
    });
    let pipeline = make(
        device,
        &layout,
        &shader,
        "vertex",
        "fragment",
        format,
        Some(wgpu::BlendState::ALPHA_BLENDING),
        samples,
    );
    pipeline
}
```

**Locate this block:**

```rust
        "fragment",
        format,
        Some(wgpu::BlendState::ALPHA_BLENDING),
    );
    let pick = make(
        device,
```

Replace it with:

```rust
        "fragment",
        format,
        Some(wgpu::BlendState::ALPHA_BLENDING),
        1,
    );
    let pick = make(
        device,
```

**Locate this block:**

```rust
    blend: Option<wgpu::BlendState>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("sheet batch"),layout:Some(layout),vertex:wgpu::VertexState {module:shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:32,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x3,3=>Uint32]}]},fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::ink()),multisample:Default::default(),multiview_mask:None,cache:None})
}

pub fn pipelines(
```

Replace it with:

```rust
    blend: Option<wgpu::BlendState>,
    samples: u32,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("sheet batch"),layout:Some(layout),vertex:wgpu::VertexState {module:shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:32,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x3,3=>Uint32]}]},fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::ink()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}

pub fn pipelines(
```

### 5. `src/cloud_address_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
    }
    for density in [1., 2.] {
        for count in [1] {
            let mut scene = Scene::empty();
```

Replace it with:

```rust
        }
    }
    for density in [1., 2.] {
        for count in [1, 4] {
            let mut scene = Scene::empty();
```

**Locate this block:**

```rust
                crate::cap_gpu_tests::pixels_at(&r, matrix) == actual,
                "Fresh owner queries retain exact repeated-frame pixels"
            );
            assert!(Rc::ptr_eq(&r.meshes[0].geometry, &geometry));
            assert_eq!(r.cloud_usage()[..4], cached);
            crate::sample_test_proof::write(
```

Replace it with:

```rust
                crate::cap_gpu_tests::pixels_at(&r, matrix) == actual,
                "Fresh owner queries retain exact repeated-frame pixels"
            );
            for next in [5 - count, count] {
                retarget(&mut r, next);
                let current = crate::cap_gpu_tests::pixels_at(&r, matrix);
                if next == count {
                    assert!(
                        current == actual,
                        "Exact sample-count return pixels"
                    );
                }
                let covered = crate::cap_gpu_tests::at(&current, 208, 96);
                assert!(covered[2] > 240 && covered[0] < 10 && covered[1] < 10);
                r.request_pick([0.3, 0.2], matrix).unwrap();
                crate::cap_gpu_tests::wait(&r);
                assert_eq!(
                    r.take_pick(matrix)
                        .unwrap()
                        .unwrap()
                        .unwrap()[0],
                    cloud.value()
                );
            }
            assert!(Rc::ptr_eq(&r.meshes[0].geometry, &geometry));
            assert_eq!(r.cloud_usage()[..4], cached);
            crate::sample_test_proof::write(
```

**Locate this block:**

```rust
fn actual_nearer_cloud_hides_original_ink_without_hiding_front_sources() {
    let mut rows = Vec::new();
    for density in [1., 2.] {
        for count in [1] {
            let mut scene = Scene::empty();
            let face = scene
                .insert(
```

Replace it with:

```rust
fn actual_nearer_cloud_hides_original_ink_without_hiding_front_sources() {
    let mut rows = Vec::new();
    for density in [1., 2.] {
        for count in [1, 4] {
            let mut scene = Scene::empty();
            let face = scene
                .insert(
```

**Locate this block:**

```rust
#[test]
fn opaque_cloud_depth_clears_occluded_physical_triangle_and_cap_addresses() {
    for clipped in [false, true] {
        for count in [1] {
            let (mut scene, _) = crate::cap_gpu_tests::scene(false);
            if clipped {
                let plane = crate::clipping_geometry::plane_from(
```

Replace it with:

```rust
#[test]
fn opaque_cloud_depth_clears_occluded_physical_triangle_and_cap_addresses() {
    for clipped in [false, true] {
        for count in [1, 4] {
            let (mut scene, _) = crate::cap_gpu_tests::scene(false);
            if clipped {
                let plane = crate::clipping_geometry::plane_from(
```

**Locate this block:**

```rust
use std::rc::Rc;

pub(super) fn retarget(_r: &mut Renderer, count: u32) {
    assert_eq!(count, 1);
}
fn physical(r: &Renderer) -> Vec<[u32; 4]> {
    let count = r.targets.samples;
```

Replace it with:

```rust
use std::rc::Rc;

pub(super) fn retarget(r: &mut Renderer, count: u32) {
    assert!(matches!(count, 1 | 4));
    if r.targets.samples == count {
        return;
    }
    r.surfaces.select(count, || {
        crate::surface_pipelines::pipelines(
            &r.device,
            r.format,
            &crate::surface_view::layout(&r.device),
            &crate::gpu_layout::uniform(&r.device, crate::object_record::BYTES as u64),
            count,
        )
    });
    r.instances
        .set_samples(&r.device, r.format, count);
    for lane in [
        &mut r.mesh_ink,
        &mut r.strokes,
        &mut r.paths,
        &mut r.markers,
        &mut r.grid,
    ] {
        lane.set_samples(&r.device, count);
    }
    r.highlight.set_samples(&r.device, count);
    r.preview.set_samples(&r.device, count);
    r.measurement.set_samples(&r.device, count);
    r.guides.set_samples(&r.device, count);
    r.caps.borrow_mut().set_samples(count);
    if let Some(cloud) = r.clouds.borrow_mut().as_mut() {
        cloud.set_samples(&r.text_context(), count);
    }
    r.targets = crate::frame_targets::Targets::with_formats(
        &r.device,
        r.size,
        r.output_format,
        r.format,
        count,
    );
}
fn physical(r: &Renderer) -> Vec<[u32; 4]> {
    let count = r.targets.samples;
```

### 6. `src/cloud_composite_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    }
                    if !edges
                        && ((alpha == 0 && edl == 0. && samples == 1)
                            || (alpha == 255 && edl > 0. && samples == 1))
                    {
```

Replace it with:

```rust
                    }
                    if !edges
                        && ((alpha == 0 && edl == 0. && samples == 1)
                            || (alpha == 255
                                && edl > 0.
                                && (samples == 1 || samples == 4)))
                    {
```

**Locate this block:**

```rust
            );
            for samples in [1] {
                super::cloud_address_tests::retarget(&mut r, samples);
                let default_pixels = crate::cap_gpu_tests::pixels_at(&r, matrix);
                let usage = r.cloud_usage();
```

Replace it with:

```rust
            );
            for samples in [1, 4] {
                super::cloud_address_tests::retarget(&mut r, samples);
                let default_pixels = crate::cap_gpu_tests::pixels_at(&r, matrix);
                let usage = r.cloud_usage();
```

### 7. `src/sample_surface_tests.rs`

**Locate this existing block:**

```rust
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
}

fn retarget_basic_fixture(r: &mut Renderer, count: u32) {
    super::cloud_address_tests::retarget(r, count);
    r.sheets.set_samples(&r.device, count);
}
#[test]
fn cached_clouds_and_original_sheets_draw_in_the_actual_four_sample_main_frame() {
    let mut scene = Scene::empty();
    let mesh = scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [-0.7, -0.7, 0.6],
                    [0.7, -0.7, 0.6],
                    [0., 0.7, 0.6],
                ],
                [0.1, 0.3, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let mut original = session_rust::PointCloud::from_coords(
        vec![0., 0., 0.2],
        vec![255, 0, 0, 255],
        vec![],
    );
    original.point_size = 16.;
    let original = Rc::new(original);
    let cloud = scene
        .insert_cloud(crate::cloud::Prepared::new(original.clone()).unwrap())
        .unwrap();
    let cloud_before = original.to_proto();
    let sheet = Rc::new(crate::sheet_example::source());
    let drawing = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(sheet.clone()).unwrap(),
        ))
        .unwrap();
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    let transform = crate::cap_gpu_tests::reversed(
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    );
    let first = crate::cap_gpu_tests::pixels_at(&r, transform);
    let uploads = (
        r.stats()[0],
        r.sheet_usage()[3],
        r.cloud_usage()[2],
    );
    let preparations = r.cloud_usage()[3];
    let geometry = r
        .meshes
        .iter()
        .map(|m| Rc::clone(&m.geometry))
        .collect::<Vec<_>>();
    let root = std::path::Path::new("sheet-cloud-pixels");
    crate::sample_test_proof::write(root.join("one.rgba"), &first).unwrap();
    let mut four = Vec::new();
    for count in [4, 1, 4] {
        retarget_basic_fixture(&mut r, count);
        let pixels = crate::cap_gpu_tests::pixels_at(&r, transform);
        let centre = crate::cap_gpu_tests::at(&pixels, 160, 120);
        assert!(
            centre[0] > 240 && centre[1] < 10 && centre[2] < 10,
            "Original cloud correctly covers the face and sheet at{count}x"
        );
        assert!(
            pixels
                .chunks_exact(4)
                .filter(|p| p[..3].iter().all(|v| *v < 20))
                .count()
                > 400,
            "Drawing sheet remains visible"
        );
        assert_eq!(
            (
                r.stats()[0],
                r.sheet_usage()[3],
                r.cloud_usage()[2]
            ),
            uploads
        );
        assert_eq!(
            r.cloud_usage()[3],
            preparations,
            "Changing main sample count must not rebuild cached cloud pixels"
        );
        if count == 1 {
            assert!(pixels == first);
        } else if four.is_empty() {
            four = pixels;
        } else {
            assert!(pixels == four);
        }
        for (at, id) in [
            ([0., 0.], cloud),
            ([0., -0.5], drawing),
            ([0., -0.2], mesh),
        ] {
            r.request_pick(at, transform).unwrap();
            crate::cap_gpu_tests::wait(&r);
            assert_eq!(
                r.take_pick(transform)
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                id.value(),
                "Original owner after main{count}x switch"
            );
        }
    }
    for (mesh, before) in r.meshes.iter().zip(&geometry) {
        assert!(Rc::ptr_eq(&mesh.geometry, before));
    }
    assert_eq!(original.to_proto(), cloud_before);
    assert!(Rc::ptr_eq(
        &scene.sheets()[0].prepared.original,
        &sheet
    ));
    crate::sample_test_proof::write(root.join("four.rgba"), four).unwrap();
}
```

## <a id="checkpoint-96zzzzzzzd-retarget-cached-text"></a>11. Retarget cached text colour

### 1. `src/text_plate.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            return 0;
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
```

Replace it with:

```rust
            return 0;
        }

        pass.set_pipeline(&self.pipeline.active);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
```

**Locate this block:**

```rust
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 40, VERTS),
            pipeline: pipeline(ctx, target, false),
            id_pipeline: pipeline(ctx, Target::ID, true),
            physical_vertices: 0,
        }
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target, false);
    }

    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>, overlay: bool) -> u32 {
```

Replace it with:

```rust
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 40, VERTS),
            pipeline: Variants::new(pipeline(ctx, target, false), target.samples),
            format: target.format,
            id_pipeline: pipeline(ctx, Target::ID, true),
            physical_vertices: 0,
        }
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        if target.format != self.format {
            self.pipeline = Variants::new(pipeline(ctx, target, false), target.samples);
            self.format = target.format;
        } else {
            self.pipeline.select(target.samples, || {
                pipeline(ctx, target, false)
            });
        }
    }

    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>, overlay: bool) -> u32 {
```

**Locate this block:**

```rust
    {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text plate shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text_plate.wgsl").into()),
        });
        let id_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

Replace it with:

```rust
    {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text plate shader"),
            source: wgpu::ShaderSource::Wgsl(if target.samples == 4 {
                include_str!("text_plate.wgsl")
                    .replace(
                        "@location(0) local:",
                        "@location(0) @interpolate(linear,sample) local:",
                    )
                    .into()
            } else {
                include_str!("text_plate.wgsl").into()
            }),
        });
        let id_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

**Locate this block:**

```rust
}
pub(super) struct Plates {
    vertices: GrowBuf,
    pipeline: Pipeline,
    id_pipeline: Pipeline,
    physical_vertices: u32,
}
```

Replace it with:

```rust
}
pub(super) struct Plates {
    vertices: GrowBuf,
    pipeline: Variants<Pipeline>,
    format: wgpu::TextureFormat,
    id_pipeline: Pipeline,
    physical_vertices: u32,
}
```

**Locate this block:**

```rust
use crate::text_gpu_support::{GpuCtx, GrowBuf, VERTS};
use crate::text_gpu_support::{Pipeline, Target};
```

Replace it with:

```rust
use crate::sample_pipelines::Variants;
use crate::text_gpu_support::{GpuCtx, GrowBuf, VERTS};
use crate::text_gpu_support::{Pipeline, Target};
```

### 2. `src/text_plane.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            return 0;
        }

        self.draw_run(pass, &self.pipeline)
    }
```

Replace it with:

```rust
            return 0;
        }

        self.draw_run(pass, &self.pipeline.active)
    }
```

**Locate this block:**

```rust
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.pipeline = pipeline(ctx, target, &self.layout, false);
    }

    pub(super) fn reset(&mut self) {
```

Replace it with:

```rust
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        if target.format != self.target.format {
            self.pipeline = crate::sample_pipelines::Variants::new(
                pipeline(ctx, target, &self.layout, false),
                target.samples,
            );
        } else {
            self.pipeline.select(target.samples, || {
                pipeline(ctx, target, &self.layout, false)
            });
        }
        self.target = target;
    }

    pub(super) fn reset(&mut self) {
```

**Locate this block:**

```rust
            vertices: GrowBuf::new(ctx, "world text vertices", 64, VERTS),
            layout,
            sampler,
            pipeline,
            id_pipeline,
            target,
            rasterizations: 0,
```

Replace it with:

```rust
            vertices: GrowBuf::new(ctx, "world text vertices", 64, VERTS),
            layout,
            sampler,
            pipeline: crate::sample_pipelines::Variants::new(pipeline, target.samples),
            id_pipeline,
            target,
            rasterizations: 0,
```

**Locate this block:**

```rust
    sampler: wgpu::Sampler,
    pipeline: Pipeline,
    id_pipeline: Pipeline,
    target: Target,
    pub(super) rasterizations: u64,
```

Replace it with:

```rust
    sampler: wgpu::Sampler,
    pipeline: crate::sample_pipelines::Variants<Pipeline>,
    id_pipeline: Pipeline,
    target: Target,
    pub(super) rasterizations: u64,
```

### 3. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.plates.retarget(ctx, target);
        self.planes.retarget(ctx, target);
        self.overlay = renderer(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        if self.target.format == target.format && self.target.samples == target.samples
        {
            return;
        }
        let format_changed = self.target.format != target.format;
        self.target = target;
        self.plates.retarget(ctx, target);
        self.planes.retarget(ctx, target);
        if format_changed {
            self.rebuild_resources(ctx);
            return;
        }
        self.overlay = renderer(
```

### 4. `src/renderer.rs`

**Locate this existing block:**

```rust
                    format: self.format,
                    samples: 1,
                },
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    format: self.format,
                    samples: self.targets.samples,
                },
```

### 5. `src/sample_surface_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    ));
    crate::sample_test_proof::write(root.join("four.rgba"), four).unwrap();
}
```

Replace it with:

```rust
    ));
    crate::sample_test_proof::write(root.join("four.rgba"), four).unwrap();
}

#[test]
fn actual_text_sample_switches_keep_font_raster_sources_and_fresh_owner_queries() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    let mut scene = Scene::empty();
    let mesh = scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [-0.8, -0.7, 0.6],
                    [0.8, -0.7, 0.6],
                    [0., 0.8, 0.6],
                ],
                [0.1, 0.3, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    retarget_basic_fixture(&mut r, 4);
    let mut label = example(TextPlacement::WorldPlane {
        world: [-0.5, 0.25, 0.2],
        right: [1., 0., 0.],
        up: [0., 1., 0.],
        world_height: 0.18,
    });
    label.label.text = "Original".into();
    let original = Rc::new(label);
    scene
        .insert_text(Prepared::new(original.clone()).unwrap())
        .unwrap();
    let mut name = example(TextPlacement::Nameplate {
        world: [0.4, -0.35, 0.2],
        padding: [8., 3.],
        rounded: true,
    });
    name.label.text = "Owner".into();
    let owner = scene
        .insert_text(Prepared::new(Rc::new(name)).unwrap())
        .unwrap();
    r.set_scene(&scene, None);
    let source = serde_json::to_string(&*original).unwrap();
    let mut overlay = crate::text_example::label();
    overlay.text = "Sampled text".into();
    overlay.font_size = 18.;
    r.set_text(vec![overlay]).unwrap();
    let transform = crate::cap_gpu_tests::reversed(
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    );
    let first = crate::cap_gpu_tests::pixels_at(&r, transform);
    assert_eq!(r.text_error(), None);
    let stats = r.text_stats().unwrap();
    assert!(stats.world_plane_rasterizations > 0 && stats.raster_images > 0);
    let usage = r.text_scene_usage();
    let gpu = r.stats();
    let geometry = r.meshes[0].geometry.clone();
    let root = std::path::Path::new("text-pixels");
    crate::sample_test_proof::write(root.join("four.rgba"), &first).unwrap();
    let mut single = Vec::new();
    for count in [4, 1, 4] {
        retarget_basic_fixture(&mut r, count);
        let pixels = crate::cap_gpu_tests::pixels_at(&r, transform);
        assert_eq!(r.text_error(), None);
        if count == 4 {
            assert!(pixels == first);
        } else {
            single = pixels;
        }
        let current = r.text_stats().unwrap();
        assert_eq!(current.shape_count, stats.shape_count);
        assert_eq!(
            current.world_plane_rasterizations,
            stats.world_plane_rasterizations
        );
        assert_eq!(current.raster_images, stats.raster_images);
        assert_eq!(current.atlas_resets, stats.atlas_resets);
        assert_eq!(r.text_scene_usage(), usage);
        assert_eq!(r.stats(), gpu);
        assert!(Rc::ptr_eq(&r.meshes[0].geometry, &geometry));
        for (at, id) in [([0.4, -0.35], owner), ([-0.2, -0.2], mesh)] {
            r.request_pick(at, transform).unwrap();
            crate::cap_gpu_tests::wait(&r);
            assert_eq!(
                r.take_pick(transform)
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                id.value(),
                "Original1x owner query after{count}x text"
            );
        }
    }
    assert_eq!(
        serde_json::to_string(&*original).unwrap(),
        source
    );
    crate::sample_test_proof::write(root.join("one.rgba"), single).unwrap();
}
```

**Locate this block:**

```rust
    r.sheets.set_samples(&r.device, count);
}
#[test]
fn cached_clouds_and_original_sheets_draw_in_the_actual_four_sample_main_frame() {
```

Replace it with:

```rust
    r.sheets.set_samples(&r.device, count);
    if let Some(text) = r.text.borrow_mut().as_mut() {
        text.retarget(
            &r.text_context(),
            crate::text_gpu_support::Target {
                format: r.format,
                samples: count,
            },
        );
    }
}
#[test]
fn cached_clouds_and_original_sheets_draw_in_the_actual_four_sample_main_frame() {
```

## <a id="checkpoint-96zzzzzzze-cache-outline-section-recipes"></a>12. Cache section masks and the final outline blend

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

### 1. `src/outline_caps.rs`

Replace the entire file with:

```rust
use crate::sample_pipelines::Variants;
pub struct Caps {
    pipelines: Option<Variants<(wgpu::RenderPipeline, wgpu::BindGroupLayout)>>,
    input: Option<wgpu::TextureView>,
    group: Option<wgpu::BindGroup>,
}
impl Caps {
    pub fn new(_: &wgpu::Device) -> Self {
        Self {
            pipelines: None,
            input: None,
            group: None,
        }
    }
    pub fn clear(&mut self) {
        self.input = None;
        self.group = None;
    }
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        input: &wgpu::TextureView,
        radius: &wgpu::Buffer,
        samples: u32,
    ) {
        if self.pipelines.is_none() {
            self.pipelines = Some(Variants::new(
                pipelines(device, samples),
                samples,
            ));
        }
        if self
            .pipelines
            .as_mut()
            .unwrap()
            .select(samples, || pipelines(device, samples))
        {
            self.clear();
        }
        if self.input.as_ref() == Some(input) {
            return;
        }
        self.group = Some(
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("visible cap coverage"),
                layout: &self.pipelines.as_ref().unwrap().active.1,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(input),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: radius.as_entire_binding(),
                    },
                ],
            }),
        );
        self.input = Some(input.clone());
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, clipping: &wgpu::BindGroup) {
        if let Some(group) = &self.group {
            pass.set_pipeline(&self.pipelines.as_ref().unwrap().active.0);
            pass.set_bind_group(0, group, &[]);
            pass.set_bind_group(1, clipping, &[]);
            pass.draw(0..3, 0..1);
        }
    }
}

fn pipelines(
    device: &wgpu::Device,
    samples: u32,
) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout) {
    use wgpu::{ShaderStages as Stages, TextureSampleType as Sample};
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("visible cap coverage"),
        entries: &[
            crate::text_gpu_support::texture_entry(
                0,
                Stages::FRAGMENT,
                Sample::Uint,
                samples == 4,
            ),
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: Stages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(16),
                },
                count: None,
            },
        ],
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("visible cap coverage"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(&source(samples), 1, 0).into(),
        ),
    });
    let clipping = crate::clipping_gpu::layout(device);
    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("visible cap coverage"),
            bind_group_layouts: &[Some(&layout), Some(&clipping)],
            immediate_size: 0,
        });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("visible cap coverage"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::TextureFormat::R8Unorm.into())],
            compilation_options: Default::default(),
        }),
        primitive: Default::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: crate::depth::FORMAT,
            depth_write_enabled: Some(false),
            depth_compare: Some(crate::depth::CLOSER_EQUAL),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    });
    (pipeline, layout)
}

fn source(samples: u32) -> String {
    let source = include_str!("outline_caps.wgsl");
    if samples == 1 {
        return source.into();
    }
    source.replace("var primitives:texture_2d<u32>;","var primitives:texture_multisampled_2d<u32>;")
 .replace(r#"@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);
}
"#,r#"struct Vertex{@builtin(position) position:vec4f,@location(0) @interpolate(linear,sample) pixel:vec2f}
@vertex fn vs_main(@builtin(vertex_index) i:u32)->Vertex {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3))[i];return Vertex(vec4f(p,0,1),p*vec2f(0.5,-0.5)+0.5);
}
"#)
 .replace("fn fs_main(@builtin(position) p:vec4<f32>)->Coverage{","fn fs_main(input:Vertex,@builtin(sample_index) sample:u32)->Coverage{let p=vec4f(input.pixel*vec2f(textureDimensions(primitives)),0,1);")
 .replace("textureLoad(primitives,vec2<i32>(p.xy),0)","textureLoad(primitives,vec2<i32>(p.xy),i32(sample))")
}
```

### 2. `src/outline_composite.rs`

Replace the entire file with:

```rust
use crate::sample_pipelines::Variants;
pub struct Composite {
    pipeline: Variants<wgpu::RenderPipeline>,
    shader: wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    layout: wgpu::BindGroupLayout,
    inputs: Option<[wgpu::TextureView; 2]>,
    group: Option<wgpu::BindGroup>,
}
impl Composite {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::with_samples(device, format, 1)
    }
    pub fn with_samples(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("max outline alpha"),
                entries: &[0, 1].map(|n| {
                    crate::text_gpu_support::texture_entry(
                        n,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::TextureSampleType::Float {
                            filterable: false,
                        },
                        false,
                    )
                }),
            });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("max outline alpha"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("outline_composite.wgsl").into(),
            ),
        });
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("max outline alpha"),
                bind_group_layouts: &[Some(&layout)],
                immediate_size: 0,
            });
        let pipeline = Variants::new(
            pipeline(
                device,
                &pipeline_layout,
                &shader,
                format,
                samples,
            ),
            samples,
        );
        Self {
            pipeline,
            shader,
            format,
            layout,
            inputs: None,
            group: None,
        }
    }
    pub fn set_samples(&mut self, device: &wgpu::Device, samples: u32) {
        self.pipeline.select(samples, || {
            let layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("max outline alpha"),
                    bind_group_layouts: &[Some(&self.layout)],
                    immediate_size: 0,
                });
            pipeline(
                device,
                &layout,
                &self.shader,
                self.format,
                samples,
            )
        });
    }
    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        colour: &wgpu::TextureView,
        global: &wgpu::TextureView,
        selected: Option<&wgpu::TextureView>,
    ) {
        let inputs = [
            global.clone(),
            selected.unwrap_or(global).clone(),
        ];
        if self.inputs.as_ref() != Some(&inputs) {
            self.group = Some(
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("max outline alpha"),
                    layout: &self.layout,
                    entries: &[0, 1].map(|n| wgpu::BindGroupEntry {
                        binding: n,
                        resource: wgpu::BindingResource::TextureView(
                            &inputs[n as usize],
                        ),
                    }),
                }),
            );
            self.inputs = Some(inputs);
        }
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("one black outline blend"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline.active);
        pass.set_bind_group(0, self.group.as_ref().unwrap(), &[]);
        pass.draw(0..3, 0..1);
    }
}

fn pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    samples: u32,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("one black outline blend"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}
```

### 3. `src/outline_gpu.rs`

**Locate this existing block:**

```rust
            self.caps
                .prepare(&ctx.device, sections, &self.radius);
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            self.caps
                .prepare(&ctx.device, sections, &self.radius, 1);
        }
```

## <a id="checkpoint-96zzzzzzzf-resolve-float-outline-coverage"></a>13. Resolve float outline coverage

### 1. `src/outline_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        multiview_mask: None,
```

Replace it with:

```rust
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
```

**Locate this block:**

```rust
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
fn mask_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("selected visible coverage"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(include_str!("outline_mask.wgsl"), 2, 0).into(),
        ),
    });
    let view = crate::surface_view::layout(device);
```

Replace it with:

```rust
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}
fn mask_pipeline(device: &wgpu::Device, samples: u32) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("selected visible coverage"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(
                &if samples == 4 {
                    include_str!("outline_mask.wgsl").replace(
                        "@location(0) world:",
                        "@location(0) @interpolate(perspective,sample) world:",
                    )
                } else {
                    include_str!("outline_mask.wgsl").into()
                },
                2,
                0,
            )
            .into(),
        ),
    });
    let view = crate::surface_view::layout(device);
```

**Locate this block:**

```rust
    entry: &str,
    format: wgpu::TextureFormat,
    blend: bool,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(entry),
```

Replace it with:

```rust
    entry: &str,
    format: wgpu::TextureFormat,
    blend: bool,
    samples: u32,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(entry),
```

**Locate this block:**

```rust
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.composite_pipeline);
        pass.set_bind_group(0, group, &[]);
        pass.draw(0..3, 0..1);
    }
```

Replace it with:

```rust
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.pipelines.active.1);
        pass.set_bind_group(0, group, &[]);
        pass.draw(0..3, 0..1);
    }
```

**Locate this block:**

```rust
                    ),
                    ..Default::default()
                });
                pass.set_pipeline(&self.mask_pipeline);
                pass.set_bind_group(0, view_group, &[]);
                pass.set_bind_group(2, clipping, &[]);
                for mesh in meshes.iter().filter(|m| all || m.selected()) {
```

Replace it with:

```rust
                    ),
                    ..Default::default()
                });
                pass.set_pipeline(&self.pipelines.active.0);
                pass.set_bind_group(0, view_group, &[]);
                pass.set_bind_group(2, clipping, &[]);
                for mesh in meshes.iter().filter(|m| all || m.selected()) {
```

**Locate this block:**

```rust
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("selected visible coverage"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &m.coverage.view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
```

Replace it with:

```rust
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("selected visible coverage"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: m
                            .coverage4
                            .as_ref()
                            .map_or(&m.coverage.view, |image| &image.view),
                        depth_slice: None,
                        resolve_target: m
                            .coverage4
                            .as_ref()
                            .map(|_| &m.coverage.view),
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
```

**Locate this block:**

```rust
        }
        self.prepare(ctx, size, density, all);
        if let Some(sections) = sections {
            self.caps
                .prepare(&ctx.device, sections, &self.radius, 1);
        }
        let key = Key {
            transform,
```

Replace it with:

```rust
        }
        self.prepare(ctx, size, density, all);
        if let Some(sections) = sections {
            self.caps.prepare(
                &ctx.device,
                sections,
                &self.radius,
                self.pipelines.samples,
            );
        } else {
            self.caps.clear();
        }
        let key = Key {
            transform,
```

**Locate this block:**

```rust
            });
            self.key = None;
        }
        ctx.queue.write_buffer(
            &self.radius,
            0,
```

Replace it with:

```rust
            });
            self.key = None;
        }
        let masks = self.masks.as_mut().unwrap();
        if masks.coverage4.is_some() != (self.pipelines.samples == 4) {
            masks.coverage4 = (self.pipelines.samples == 4).then(|| {
                Image::with_samples(
                    &ctx.device,
                    size,
                    "multisampled original coverage",
                    4,
                )
            });
            self.key = None;
        }
        ctx.queue.write_buffer(
            &self.radius,
            0,
```

**Locate this block:**

```rust
            );
            self.masks = Some(Masks {
                coverage,
                coarse,
                dilated,
                alpha,
```

Replace it with:

```rust
            );
            self.masks = Some(Masks {
                coverage,
                coverage4: None,
                coarse,
                dilated,
                alpha,
```

**Locate this block:**

```rust
            |m| {
                [
                    m.coverage.bytes()
                        + m.coarse.bytes()
                        + m.dilated.bytes()
                        + m.alpha.bytes()
```

Replace it with:

```rust
            |m| {
                [
                    m.coverage.bytes()
                        + m.coverage4.as_ref().map_or(0, Image::bytes)
                        + m.coarse.bytes()
                        + m.dilated.bytes()
                        + m.alpha.bytes()
```

**Locate this block:**

```rust
            draws: 0,
        }
    }
    pub fn invalidate(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }
```

Replace it with:

```rust
            draws: 0,
        }
    }
    pub fn set_samples(&mut self, ctx: &GpuCtx, samples: u32) {
        if self.pipelines.select(samples, || {
            (
                mask_pipeline(&ctx.device, samples),
                fullscreen(
                    &ctx.device,
                    &self.shader,
                    &[&self.composite_layout],
                    "fs_main",
                    self.format,
                    true,
                    samples,
                ),
            )
        }) {
            self.key = None;
            self.caps.clear();
        }
    }
    pub fn invalidate(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }
```

**Locate this block:**

```rust
                "fs_alpha",
                wgpu::TextureFormat::R8Unorm,
                false,
            ),
            composite_pipeline: fullscreen(
                &ctx.device,
                &shader,
                &[&composite_layout],
                "fs_main",
                format,
                true,
            ),
            pool_layout,
            search_layout,
            composite_layout,
```

Replace it with:

```rust
                "fs_alpha",
                wgpu::TextureFormat::R8Unorm,
                false,
                1,
            ),
            pool_layout,
            search_layout,
            composite_layout,
```

**Locate this block:**

```rust
                "fs_dilate",
                wgpu::TextureFormat::R8Unorm,
                false,
            ),
            alpha_pipeline: fullscreen(
                &ctx.device,
```

Replace it with:

```rust
                "fs_dilate",
                wgpu::TextureFormat::R8Unorm,
                false,
                1,
            ),
            alpha_pipeline: fullscreen(
                &ctx.device,
```

**Locate this block:**

```rust
                "fs_pool",
                wgpu::TextureFormat::R8Unorm,
                false,
            ),
            dilate_pipeline: fullscreen(
                &ctx.device,
```

Replace it with:

```rust
                "fs_pool",
                wgpu::TextureFormat::R8Unorm,
                false,
                1,
            ),
            dilate_pipeline: fullscreen(
                &ctx.device,
```

**Locate this block:**

```rust
                label: Some("current surface outline"),
                source: wgpu::ShaderSource::Wgsl(include_str!("outline.wgsl").into()),
            });
        Self {
            mask_pipeline: mask_pipeline(&ctx.device),
            caps: crate::outline_caps::Caps::new(&ctx.device),
            pool_pipeline: fullscreen(
                &ctx.device,
                &shader,
```

Replace it with:

```rust
                label: Some("current surface outline"),
                source: wgpu::ShaderSource::Wgsl(include_str!("outline.wgsl").into()),
            });
        let pipelines = Variants::new(
            (
                mask_pipeline(&ctx.device, samples),
                fullscreen(
                    &ctx.device,
                    &shader,
                    &[&composite_layout],
                    "fs_main",
                    format,
                    true,
                    samples,
                ),
            ),
            samples,
        );
        Self {
            pipelines,
            shader: shader.clone(),
            format,
            caps: crate::outline_caps::Caps::new(),
            pool_pipeline: fullscreen(
                &ctx.device,
                &shader,
```

**Locate this block:**

```rust
}
impl Outline {
    pub fn new(ctx: &GpuCtx, format: wgpu::TextureFormat) -> Self {
        let texture = |binding| {
            texture_entry(
                binding,
```

Replace it with:

```rust
}
impl Outline {
    pub fn new(ctx: &GpuCtx, format: wgpu::TextureFormat) -> Self {
        Self::with_samples(ctx, format, 1)
    }
    pub fn with_samples(
        ctx: &GpuCtx,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        let texture = |binding| {
            texture_entry(
                binding,
```

**Locate this block:**

```rust
}

pub struct Outline {
    mask_pipeline: wgpu::RenderPipeline,
    caps: crate::outline_caps::Caps,
    pool_pipeline: wgpu::RenderPipeline,
    dilate_pipeline: wgpu::RenderPipeline,
    alpha_pipeline: wgpu::RenderPipeline,
    composite_pipeline: wgpu::RenderPipeline,
    pool_layout: wgpu::BindGroupLayout,
    search_layout: wgpu::BindGroupLayout,
    composite_layout: wgpu::BindGroupLayout,
```

Replace it with:

```rust
}

pub struct Outline {
    pipelines: Variants<(wgpu::RenderPipeline, wgpu::RenderPipeline)>,
    shader: wgpu::ShaderModule,
    format: wgpu::TextureFormat,
    caps: crate::outline_caps::Caps,
    pool_pipeline: wgpu::RenderPipeline,
    dilate_pipeline: wgpu::RenderPipeline,
    alpha_pipeline: wgpu::RenderPipeline,
    pool_layout: wgpu::BindGroupLayout,
    search_layout: wgpu::BindGroupLayout,
    composite_layout: wgpu::BindGroupLayout,
```

**Locate this block:**

```rust
}
struct Masks {
    coverage: Image,
    coarse: Image,
    dilated: Image,
    alpha: Image,
```

Replace it with:

```rust
}
struct Masks {
    coverage: Image,
    coverage4: Option<Image>,
    coarse: Image,
    dilated: Image,
    alpha: Image,
```

**Locate this block:**

```rust
        }
    }
    fn bytes(&self) -> u64 {
        self.size[0] as u64 * self.size[1] as u64
    }
}
impl Drop for Image {
```

Replace it with:

```rust
        }
    }
    fn bytes(&self) -> u64 {
        self.size[0] as u64
            * self.size[1] as u64
            * u64::from(self.texture.sample_count())
    }
}
impl Drop for Image {
```

**Locate this block:**

```rust
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
```

Replace it with:

```rust
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | if samples == 1 {
                    wgpu::TextureUsages::COPY_SRC
                } else {
                    wgpu::TextureUsages::empty()
                },
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
```

**Locate this block:**

```rust
}
impl Image {
    fn new(device: &wgpu::Device, size: [u32; 2], label: &str) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
```

Replace it with:

```rust
}
impl Image {
    fn new(device: &wgpu::Device, size: [u32; 2], label: &str) -> Self {
        Self::with_samples(device, size, label, 1)
    }
    fn with_samples(
        device: &wgpu::Device,
        size: [u32; 2],
        label: &str,
        samples: u32,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
```

**Locate this block:**

```rust
use crate::{
    gpu_mesh::GpuMesh,
    text_gpu_support::{GpuCtx, resource_group, texture_entry},
```

Replace it with:

```rust
use crate::sample_pipelines::Variants;
use crate::{
    gpu_mesh::GpuMesh,
    text_gpu_support::{GpuCtx, resource_group, texture_entry},
```

### 2. `src/outline_caps.rs`

**Locate this existing block:**

```rust
impl Caps {
    pub fn new(_: &wgpu::Device) -> Self {
        Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Caps {
    pub fn new() -> Self {
        Self {
```

### 3. `src/renderer.rs`

**Locate this existing block:**

```rust
            let global = global.get_or_insert_with(|| {
                crate::outline_gpu::Outline::new(&ctx, self.format)
            });
            global.encode_masks(
                &ctx,
                &mut encoder,
                &self.targets.depth,
                &self.view_group,
                &self.meshes,
                *transform,
                self.size,
                self.density,
                &clip_group,
                clip_revision,
                true,
                Some(&self.targets.primitive.view),
            );
            let mut selected = self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected) {
                let selected = selected.get_or_insert_with(|| {
                    crate::outline_gpu::Outline::new(&ctx, self.format)
                });
                selected.encode_masks(
                    &ctx,
                    &mut encoder,
                    &self.targets.depth,
                    &self.view_group,
                    &self.meshes,
                    *transform,
                    self.size,
                    self.density,
                    &clip_group,
                    clip_revision,
                    false,
                    Some(&self.targets.primitive.view),
                );
            } else {
                selected.take();
            }
            if let Some(alpha) = global.alpha_view() {
                self.outline_composite
                    .borrow_mut()
                    .get_or_insert_with(|| {
                        crate::outline_composite::Composite::new(
                            &self.device,
                            self.format,
                        )
                    })
                    .draw(
                        &self.device,
                        &mut encoder,
                        view,
                        alpha,
                        selected
                            .as_ref()
                            .and_then(|s| s.alpha_view()),
                    );
            }
        } else {
            self.global_outline.borrow_mut().take();
            self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected) {
                let mut outline = self.outline.borrow_mut();
                let outline = outline.get_or_insert_with(|| {
                    crate::outline_gpu::Outline::new(&self.text_context(), self.format)
                });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            let global = global.get_or_insert_with(|| {
                crate::outline_gpu::Outline::with_samples(
                    &ctx,
                    self.format,
                    self.targets.samples,
                )
            });
            global.encode_masks(
                &ctx,
                &mut encoder,
                &self.targets.depth,
                &self.view_group,
                &self.meshes,
                *transform,
                self.size,
                self.density,
                &clip_group,
                clip_revision,
                true,
                (self.clipping.borrow().enabled
                    && !self.clipping.borrow().rows.is_empty())
                .then_some(&self.targets.primitive.view),
            );
            let mut selected = self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected) {
                let selected = selected.get_or_insert_with(|| {
                    crate::outline_gpu::Outline::with_samples(
                        &ctx,
                        self.format,
                        self.targets.samples,
                    )
                });
                selected.encode_masks(
                    &ctx,
                    &mut encoder,
                    &self.targets.depth,
                    &self.view_group,
                    &self.meshes,
                    *transform,
                    self.size,
                    self.density,
                    &clip_group,
                    clip_revision,
                    false,
                    (self.clipping.borrow().enabled
                        && !self.clipping.borrow().rows.is_empty())
                    .then_some(&self.targets.primitive.view),
                );
            } else {
                selected.take();
            }
            if let Some(alpha) = global.alpha_view() {
                self.outline_composite
                    .borrow_mut()
                    .get_or_insert_with(|| {
                        crate::outline_composite::Composite::with_samples(
                            &self.device,
                            self.format,
                            self.targets.samples,
                        )
                    })
                    .draw(
                        &self.device,
                        &mut encoder,
                        view,
                        alpha,
                        selected
                            .as_ref()
                            .and_then(|s| s.alpha_view()),
                    );
            }
        } else {
            self.global_outline.borrow_mut().take();
            self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected) {
                let mut outline = self.outline.borrow_mut();
                let outline = outline.get_or_insert_with(|| {
                    crate::outline_gpu::Outline::with_samples(
                        &self.text_context(),
                        self.format,
                        self.targets.samples,
                    )
                });
```

### 4. `src/sample_surface_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    );
    crate::sample_test_proof::write(root.join("one.rgba"), single).unwrap();
}
```

Replace it with:

```rust
    );
    crate::sample_test_proof::write(root.join("one.rgba"), single).unwrap();
}

#[test]
fn actual_four_sample_outline_and_section_masks_keep_single_blend_and_original_owners()
{
    let (scene, owner) = crate::cap_gpu_tests::scene(false);
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    r.select_objects(&scene, &[owner]).unwrap();
    r.set_outlines(true);
    let first = crate::cap_gpu_tests::pixels(&r);
    let alpha = r.global_outline_alpha();
    assert!(alpha.iter().filter(|v| **v > 0).count() > 300);
    assert!(r.outline_alpha() == alpha);
    let usage = r.global_outline_usage()[0];
    let geometry = r
        .meshes
        .iter()
        .map(|m| Rc::clone(&m.geometry))
        .collect::<Vec<_>>();
    let uploads = r.stats()[0];
    let original = scene
        .objects()
        .iter()
        .map(|o| o.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let root = std::path::Path::new("outline-pixels");
    crate::sample_test_proof::write(root.join("selected-one.rgba"), &first).unwrap();
    let mut four = Vec::new();
    for count in [4, 1, 4] {
        retarget_basic_fixture(&mut r, count);
        let pixels = crate::cap_gpu_tests::pixels(&r);
        assert!(
            r.outline_alpha() == r.global_outline_alpha(),
            "Selected and global actual{count}x cap/face coverage remains identical"
        );
        let expected = usage
            + if count == 4 {
                4 * 320 * 240
            } else {
                0
            };
        assert_eq!(
            r.global_outline_usage()[0],
            expected,
            "Only the active4x coverage image adds memory;1x search images persist"
        );
        assert_eq!(r.stats()[0], uploads);
        if count == 1 {
            assert!(pixels == first);
        } else if four.is_empty() {
            four = pixels;
        } else {
            assert!(pixels == four);
        }
        let matrix = crate::cap_gpu_tests::matrix();
        r.request_pick([0.3, 0.2], matrix).unwrap();
        crate::cap_gpu_tests::wait(&r);
        assert_eq!(
            r.take_pick(matrix)
                .unwrap()
                .unwrap()
                .unwrap()[0],
            owner.value()
        );
    }
    crate::sample_test_proof::write(root.join("selected-four.rgba"), &four).unwrap();
    r.select_objects(&scene, &[]).unwrap();
    let global = crate::cap_gpu_tests::pixels(&r);
    crate::sample_test_proof::write(root.join("global-four.rgba"), &global).unwrap();
    let alpha = r.global_outline_alpha();
    let mut boundary = 0;
    for (i, coverage) in alpha.iter().enumerate() {
        if *coverage == 255 {
            boundary += 1;
            assert!(
                global[i * 4..i * 4 + 3]
                    .iter()
                    .all(|v| *v < 12)
            );
            assert!(
                four[i * 4..i * 4 + 3]
                    .iter()
                    .all(|v| *v < 12)
            );
        }
    }
    assert!(boundary > 300);
    for ((mesh, geometry), source) in r.meshes.iter().zip(&geometry).zip(&original) {
        assert!(Rc::ptr_eq(&mesh.geometry, geometry));
        assert_eq!(
            &scene
                .objects()
                .iter()
                .find(|o| o.id == mesh.id)
                .unwrap()
                .geometry()
                .unwrap()
                .to_proto(),
            source
        );
    }
    r.set_clipping(false, true);
    let uncut = crate::cap_gpu_tests::pixels(&r);
    assert!(
        r.global_outline_alpha() == alpha,
        "Actual4x horizontal section and full solid share the same outer silhouette"
    );
    crate::sample_test_proof::write(root.join("uncut-four.rgba"), uncut).unwrap();
}
```

**Locate this block:**

```rust
    }
}
#[test]
fn cached_clouds_and_original_sheets_draw_in_the_actual_four_sample_main_frame() {
```

Replace it with:

```rust
    }
    let ctx = r.text_context();
    for owner in [&r.global_outline, &r.outline] {
        if let Some(owner) = owner.borrow_mut().as_mut() {
            owner.set_samples(&ctx, count);
        }
    }
    if let Some(owner) = r.outline_composite.borrow_mut().as_mut() {
        owner.set_samples(&r.device, count);
    }
}
#[test]
fn cached_clouds_and_original_sheets_draw_in_the_actual_four_sample_main_frame() {
```

## <a id="checkpoint-96zzzzzzzg-retarget-original-ambient-properties"></a>14. Retarget ambient properties

### 1. `src/ambient_source_pipeline.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                }),
                primitive: Default::default(),
                depth_stencil: Some(crate::depth::state_for_ambient()),
                multisample: Default::default(),
                multiview_mask: None,
```

Replace it with:

```rust
                }),
                primitive: Default::default(),
                depth_stencil: Some(crate::depth::state_for_ambient()),
                multisample: wgpu::MultisampleState {
                    count: samples,
                    ..Default::default()
                },
                multiview_mask: None,
```

**Locate this block:**

```rust
            };
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("ambient original normals and radii"),
                source: wgpu::ShaderSource::Wgsl(
                    crate::clipping_gpu::source(&source, 2, 0).into(),
                ),
            });
            let attributes = wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32];
            let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
```

Replace it with:

```rust
            };
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("ambient original normals and radii"),
                source: wgpu::ShaderSource::Wgsl(if samples == 4 {
                    crate::clipping_gpu::source(&source, 2, 0)
                        .replace(
                            "@location(0) world:",
                            "@location(0) @interpolate(perspective,sample) world:",
                        )
                        .into()
                } else {
                    crate::clipping_gpu::source(&source, 2, 0).into()
                }),
            });
            let attributes = wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32];
            let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
```

**Locate this block:**

```rust
    pub fn new(device: &wgpu::Device) -> Self {
        let create = |shared| {
            let view = crate::surface_view::layout(device);
            let model = if shared {
```

Replace it with:

```rust
    pub fn new(device: &wgpu::Device) -> Self {
        Self::with_samples(device, 1)
    }
    pub fn with_samples(device: &wgpu::Device, samples: u32) -> Self {
        let create = |shared| {
            let view = crate::surface_view::layout(device);
            let model = if shared {
```

### 2. `src/ambient_source_gpu.rs`

**Locate this existing block:**

```rust
pub struct Sources {
    pipes: crate::ambient_source_pipeline::Pipeline,
}
impl Sources {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            pipes: crate::ambient_source_pipeline::Pipeline::new(device),
        }
    }
    pub fn encode(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        radius: &wgpu::TextureView,
        normals: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view: &wgpu::BindGroup,
        clip: &wgpu::BindGroup,
        meshes: &[crate::gpu_mesh::GpuMesh],
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        let attachments = [radius, normals].map(|view| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("original ambient source properties"),
            color_attachments: &attachments,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&self.pipes.draw);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(2, clip, &[]);
        for mesh in meshes {
            if records.is_some_and(|rows| rows.bind_record(&mut pass, mesh)) {
                pass.set_pipeline(&self.pipes.record_draw);
                mesh.geometry.draw_pulled(&mut pass, 1);
            } else {
                pass.set_pipeline(&self.pipes.draw);
                mesh.bind_model(&mut pass);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{ambient_source_pipeline::Pipeline, sample_pipelines::Variants};
pub struct Sources {
    pipes: Variants<Pipeline>,
}
impl Sources {
    pub fn new(device: &wgpu::Device) -> Self {
        Self::with_samples(device, 1)
    }
    pub fn with_samples(device: &wgpu::Device, samples: u32) -> Self {
        Self {
            pipes: Variants::new(
                Pipeline::with_samples(device, samples),
                samples,
            ),
        }
    }
    pub fn set_samples(&mut self, device: &wgpu::Device, samples: u32) {
        self.pipes.select(samples, || {
            Pipeline::with_samples(device, samples)
        });
    }
    pub fn encode(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        radius: &wgpu::TextureView,
        normals: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view: &wgpu::BindGroup,
        clip: &wgpu::BindGroup,
        meshes: &[crate::gpu_mesh::GpuMesh],
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        let attachments = [radius, normals].map(|view| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("original ambient source properties"),
            color_attachments: &attachments,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&self.pipes.active.draw);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(2, clip, &[]);
        for mesh in meshes {
            if records.is_some_and(|rows| rows.bind_record(&mut pass, mesh)) {
                pass.set_pipeline(&self.pipes.active.record_draw);
                mesh.geometry.draw_pulled(&mut pass, 1);
            } else {
                pass.set_pipeline(&self.pipes.active.draw);
                mesh.bind_model(&mut pass);
```

### 3. `src/ambient_image.rs`

**Locate this existing block:**

```rust
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ambient actual image"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        let bytes = u64::from(size[0])
            * u64::from(size[1])
            * u64::from(format.block_copy_size(None).unwrap());
        Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Self {
        Self::with_samples(device, size, format, 1)
    }
    pub fn with_samples(
        device: &wgpu::Device,
        size: [u32; 2],
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert!(matches!(samples, 1 | 4));
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ambient actual image"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | if samples == 1 {
                    wgpu::TextureUsages::COPY_SRC
                } else {
                    wgpu::TextureUsages::empty()
                },
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        let bytes = u64::from(size[0])
            * u64::from(size[1])
            * u64::from(format.block_copy_size(None).unwrap())
            * u64::from(samples);
        Self {
```

### 4. `src/ambient_images.rs`

**Locate this existing block:**

```rust
    pub fn new(device: &wgpu::Device, full: [u32; 2], density: f32) -> Self {
        let (w, h) =
            crate::pyramid_size::resolution((full[0], full[1]), f64::from(density));
        let size = [w.max(32), h.max(32)];
        let image = |size, format| Image::new(device, size, format);
        let source_radius = image(full, wgpu::TextureFormat::R32Float);
        let source_normals = image(full, wgpu::TextureFormat::Rg8Unorm);
        let linear = Pyramid::new(device, size, wgpu::TextureFormat::R32Float);
        let radii = Pyramid::new(device, size, wgpu::TextureFormat::R16Uint);
        let normals = image(size, wgpu::TextureFormat::Rg8Unorm);
        let occupancy = image(size, wgpu::TextureFormat::R8Unorm);
        let ground = image(size, wgpu::TextureFormat::R8Unorm);
        let ao = std::array::from_fn(|i| {
            image(
                if i == 2 {
                    full
                } else {
                    size
                },
                wgpu::TextureFormat::R8Unorm,
            )
        });
        let history = image(size, wgpu::TextureFormat::R8Unorm);
        let history_depth = image(size, wgpu::TextureFormat::R32Float);
        Self {
            full,
            size,
            source_radius,
            source_normals,
            linear,
            radii,
            normals,
            occupancy,
            ground,
            ao,
            history,
            history_depth,
        }
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn new(device: &wgpu::Device, full: [u32; 2], density: f32) -> Self {
        Self::with_samples(device, full, density, 1)
    }
    pub fn with_samples(
        device: &wgpu::Device,
        full: [u32; 2],
        density: f32,
        samples: u32,
    ) -> Self {
        let (w, h) =
            crate::pyramid_size::resolution((full[0], full[1]), f64::from(density));
        let size = [w.max(32), h.max(32)];
        let image = |size, format| Image::new(device, size, format);
        let source_radius = Image::with_samples(
            device,
            full,
            wgpu::TextureFormat::R32Float,
            samples,
        );
        let source_normals = Image::with_samples(
            device,
            full,
            wgpu::TextureFormat::Rg8Unorm,
            samples,
        );
        let linear = Pyramid::new(device, size, wgpu::TextureFormat::R32Float);
        let radii = Pyramid::new(device, size, wgpu::TextureFormat::R16Uint);
        let normals = image(size, wgpu::TextureFormat::Rg8Unorm);
        let occupancy = image(size, wgpu::TextureFormat::R8Unorm);
        let ground = image(size, wgpu::TextureFormat::R8Unorm);
        let ao = std::array::from_fn(|i| {
            image(
                if i == 2 {
                    full
                } else {
                    size
                },
                wgpu::TextureFormat::R8Unorm,
            )
        });
        let history = image(size, wgpu::TextureFormat::R8Unorm);
        let history_depth = image(size, wgpu::TextureFormat::R32Float);
        Self {
            full,
            size,
            source_radius,
            source_normals,
            linear,
            radii,
            normals,
            occupancy,
            ground,
            ao,
            history,
            history_depth,
        }
    }
    pub fn retarget_sources(&mut self, device: &wgpu::Device, samples: u32) {
        self.source_radius = Image::with_samples(
            device,
            self.full,
            wgpu::TextureFormat::R32Float,
            samples,
        );
        self.source_normals = Image::with_samples(
            device,
            self.full,
            wgpu::TextureFormat::Rg8Unorm,
            samples,
        );
    }
```

### 5. `src/ambient.wgsl`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    }
    if all(values==vec4<f32>(0.0)) { return 0.0; }
    let center = depth_sample(pixel.xy,0);
    let first = reconstruct(pixel.xy,0,center,values);
    if !MSAA { return first; }
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) {
            ao[i] = reconstruct(pixel.xy,i,z,values);
```

Replace it with:

```wgsl
    }
    if all(values==vec4<f32>(0.0)) { return 0.0; }
    let center = depth_sample(pixel.xy,0);
    let primitive=source_primitive(source_at(pixel.xy),0);
    let first = reconstruct(pixel.xy,0,center,values);
    if !MSAA { return first; }
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) || source_primitive(source_at(pixel.xy),i)!=primitive {
            ao[i] = reconstruct(pixel.xy,i,z,values);
```

**Locate this block:**

```wgsl
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
```

Replace it with:

```wgsl
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at,sample);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
```

**Locate this block:**

```wgsl
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
```

Replace it with:

```wgsl
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at,0)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
```

**Locate this block:**

```wgsl
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_primitive(at:vec2<i32>)->u32{
 let size=vec2<i32>(textureDimensions(source_primitives));
 let halves=textureLoad(source_primitives,clamp(at,vec2<i32>(0),size-1),0).xy;
 return physical_address(halves);
}
fn source_radius(pixel:vec2<f32>)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 let primitive=source_primitive(at);
 return select(radius,ambient.params.y,radius==0.0&&clip_is_cap(primitive));
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,0).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let primitive=source_primitive(at);
 if(clip_is_cap(primitive)){
  let plane=clip_cap_plane(primitive);
  if(plane<clipping.count){let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
```

Replace it with:

```wgsl
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_primitive(at:vec2<i32>,sample:i32)->u32{
 let size=vec2<i32>(textureDimensions(source_primitives));
 let halves=textureLoad(source_primitives,clamp(at,vec2<i32>(0),size-1),sample).xy;
 return physical_address(halves);
}
fn source_radius(pixel:vec2<f32>,sample:i32)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,sample).x;
 let primitive=source_primitive(at,sample);
 return select(radius,ambient.params.y,radius==0.0&&clip_is_cap(primitive));
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,sample).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,sample).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let primitive=source_primitive(at,sample);
 if(clip_is_cap(primitive)){
  let plane=clip_cap_plane(primitive);
  if(plane<clipping.count){let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
```

### 6. `src/ambient_pipeline.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
    }
}
```

Replace it with:

```rust
        }
    }
}

pub(crate) fn shader_source(samples: u32) -> String {
    let source = crate::clipping_gpu::source(include_str!("ambient.wgsl"), 0, 5);
    if samples == 1 {
        return source;
    }
    source
        .replace(
            "const MSAA: bool = false;",
            "const MSAA: bool = true;",
        )
        .replace(
            "var depth: texture_depth_2d;",
            "var depth: texture_depth_multisampled_2d;",
        )
        .replace(
            "var source_radii:texture_2d<f32>;",
            "var source_radii:texture_multisampled_2d<f32>;",
        )
        .replace(
            "var source_normals:texture_2d<f32>;",
            "var source_normals:texture_multisampled_2d<f32>;",
        )
        .replace(
            "var source_primitives:texture_2d<u32>;",
            "var source_primitives:texture_multisampled_2d<u32>;",
        )
}
```

**Locate this block:**

```rust
            });
        let shader = crate::clipping_gpu::source(include_str!("ambient.wgsl"), 0, 5);
        let groups = [
            Some(&source),
            Some(&level),
```

Replace it with:

```rust
            });
        let shader = shader_source(1);
        let groups = [
            Some(&source),
            Some(&level),
```

### 7. `src/ambient_shader_contract.rs`

**Locate this existing block:**

```rust
        include_str!("ambient_composite.wgsl").to_string(),
    ];
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        include_str!("ambient_composite.wgsl").to_string(),
        crate::ambient_pipeline::shader_source(1),
        crate::ambient_pipeline::shader_source(4),
    ];
```

## <a id="checkpoint-96zzzzzzzh-share-ambient-shader-recipes"></a>15. Share ambient shader recipes

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Rasterized Bytes (R8Unorm / Rgba8Unorm)</text>
  <rect x="64" y="50" width="244" height="102" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="80" y="64" width="42" height="42" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="101" y="91" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">A</text>
  <rect x="130" y="64" width="42" height="42" fill="#fef08a" stroke="#dc2626" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="151" y="91" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">CPU Shelf Bitmap &amp;[u8]</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.write_texture(TexelCopyTextureInfo, Extent3d)</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <line x1="448" y1="86" x2="672" y2="86" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="74" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">queue.write_texture(...)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="114" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">TEXTURE_BINDING | COPY_DST</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="134" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">origin: (x, y, 0), size: (w, h, 1)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Texture2D + Sampler in @fragment</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">textureSample(t_atlas, s, uv).r</text>
  <rect x="852" y="96" width="164" height="44" rx="4" fill="#fef08a" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="123" text-anchor="middle" font-family="monospace" font-size="13" font-weight="700" fill="#111111"> Crisp GPU Text </text>
</svg>
</div>

### 1. `src/ambient_pipeline.rs`

**Locate this existing block:**

```rust
use wgpu::{BufferBindingType as B, ShaderStages as S, TextureSampleType as T};
pub struct Pipelines {
    pub source: wgpu::BindGroupLayout,
    pub level: wgpu::BindGroupLayout,
    pub sample: wgpu::BindGroupLayout,
    pub history: wgpu::BindGroupLayout,
    pub prepare: wgpu::RenderPipeline,
    pub reduce: wgpu::RenderPipeline,
    pub occupancy: wgpu::RenderPipeline,
    pub ground: wgpu::RenderPipeline,
    pub raw: wgpu::RenderPipeline,
    pub filter: [wgpu::RenderPipeline; 2],
    pub upsample: wgpu::RenderPipeline,
    pub composite: wgpu::RenderPipeline,
}
fn pipeline(
    device: &wgpu::Device,
    source: &str,
    fragment: &str,
    groups: &[Option<&wgpu::BindGroupLayout>],
    formats: &[wgpu::TextureFormat],
    blend: bool,
) -> wgpu::RenderPipeline {
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("current ambient recipe"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("current ambient recipe"),
        bind_group_layouts: groups,
        immediate_size: 0,
    });
    let targets = formats
        .iter()
        .map(|format| {
            Some(wgpu::ColorTargetState {
                format: *format,
                blend: if blend {
                    Some(wgpu::BlendState::ALPHA_BLENDING)
                } else {
                    None
                },
                write_mask: wgpu::ColorWrites::ALL,
            })
        })
        .collect::<Vec<_>>();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("current ambient recipe"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some(fragment),
            compilation_options: Default::default(),
            targets: &targets,
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
impl Pipelines {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let texture = |binding, sample| {
            crate::text_gpu_support::texture_entry(binding, S::FRAGMENT, sample, false)
        };
        let uniform =
            |binding| crate::tile_pipelines::entry(binding, S::FRAGMENT, B::Uniform);
        let source =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient source images and precise camera"),
                entries: &[
                    texture(0, T::Depth),
                    uniform(1),
                    texture(
                        2,
                        T::Float {
                            filterable: false,
                        },
                    ),
                    texture(
                        3,
                        T::Float {
                            filterable: false,
                        },
                    ),
                    texture(4, T::Uint),
                    uniform(5),
                ],
            });
        let level = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient reduced original properties"),
            entries: &[
                texture(
                    0,
                    T::Float {
                        filterable: false,
                    },
                ),
                texture(1, T::Uint),
                texture(
                    2,
                    T::Float {
                        filterable: false,
                    },
                ),
            ],
        });
        let sample =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient filtered samples"),
                entries: &[
                    texture(0, T::Float { filterable: true }),
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: S::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
            });
        let mut entries = (0..4)
            .map(|i| {
                crate::tile_pipelines::entry(
                    i,
                    S::FRAGMENT,
                    B::Storage { read_only: false },
                )
            })
            .collect::<Vec<_>>();
        entries.push(texture(4, T::Float { filterable: true }));
        entries.push(texture(
            5,
            T::Float {
                filterable: false,
            },
        ));
        let history =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient reserved current history bindings"),
                entries: &entries,
            });
        let shader = shader_source(1);
        let groups = [
            Some(&source),
            Some(&level),
            Some(&sample),
            Some(&history),
        ];
        let prepare = pipeline(
            device,
            &shader,
            "fs_prepare",
            &[Some(&source)],
            &[
                wgpu::TextureFormat::R32Float,
                wgpu::TextureFormat::R16Uint,
                wgpu::TextureFormat::Rg8Unorm,
            ],
            false,
        );
        let make = |fragment| {
            pipeline(
                device,
                &shader,
                fragment,
                &groups,
                &[wgpu::TextureFormat::R8Unorm],
                false,
            )
        };
        let ground = make("fs_ground");
        let raw = make("fs_main");
        let filter = [make("fs_filter_x"), make("fs_filter_y")];
        let upsample = make("fs_upsample");
        let composite = pipeline(
            device,
            include_str!("ambient_composite.wgsl"),
            "fs_main",
            &[None, Some(&sample)],
            &[format],
            true,
        );
        let reduce = pipeline(
            device,
            include_str!("ambient_depth.wgsl"),
            "fs_reduce",
            &[Some(&level)],
            &[
                wgpu::TextureFormat::R32Float,
                wgpu::TextureFormat::R16Uint,
            ],
            false,
        );
        let occupancy = pipeline(
            device,
            include_str!("ambient_depth.wgsl"),
            "fs_occupancy",
            &[Some(&level)],
            &[wgpu::TextureFormat::R8Unorm],
            false,
        );
        Self {
            source,
            level,
            sample,
            history,
            prepare,
            reduce,
            occupancy,
            ground,
            raw,
            filter,
            upsample,
            composite,
        }
    }
}

pub(crate) fn shader_source(samples: u32) -> String {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{text_gpu_support::texture_entry, tile_pipelines::entry};
use wgpu::{BufferBindingType as B, ShaderStages as S, TextureSampleType as T};
pub struct Pipelines {
    pub source: wgpu::BindGroupLayout,
    pub level: wgpu::BindGroupLayout,
    pub sample: wgpu::BindGroupLayout,
    pub history: wgpu::BindGroupLayout,
    pub prepare: wgpu::RenderPipeline,
    pub reduce: wgpu::RenderPipeline,
    pub occupancy: wgpu::RenderPipeline,
    pub ground: wgpu::RenderPipeline,
    pub raw: wgpu::RenderPipeline,
    pub filter: [wgpu::RenderPipeline; 2],
    pub upsample: wgpu::RenderPipeline,
    pub composite: wgpu::RenderPipeline,
    pub edges: Option<wgpu::RenderPipeline>,
    pub composite_layout: wgpu::BindGroupLayout,
}
fn pipeline(
    device: &wgpu::Device,
    module: &wgpu::ShaderModule,
    vertex: &str,
    fragment: &str,
    groups: &[Option<&wgpu::BindGroupLayout>],
    formats: &[wgpu::TextureFormat],
    blend: bool,
    samples: u32,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("current ambient recipe"),
        bind_group_layouts: groups,
        immediate_size: 0,
    });
    let targets = formats
        .iter()
        .map(|format| {
            Some(wgpu::ColorTargetState {
                format: *format,
                blend: if blend {
                    Some(wgpu::BlendState::ALPHA_BLENDING)
                } else {
                    None
                },
                write_mask: wgpu::ColorWrites::ALL,
            })
        })
        .collect::<Vec<_>>();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("current ambient recipe"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module,
            entry_point: Some(vertex),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module,
            entry_point: Some(fragment),
            compilation_options: Default::default(),
            targets: &targets,
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}
impl Pipelines {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::with_samples(device, format, 1)
    }
    pub fn with_samples(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert!(matches!(samples, 1 | 4));
        let texture =
            |binding, sample| texture_entry(binding, S::FRAGMENT, sample, false);
        let uniform = |binding| entry(binding, S::FRAGMENT, B::Uniform);
        let source =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient source images and precise camera"),
                entries: &[
                    texture_entry(0, S::FRAGMENT, T::Depth, samples == 4),
                    uniform(1),
                    texture_entry(
                        2,
                        S::FRAGMENT,
                        T::Float {
                            filterable: false,
                        },
                        samples == 4,
                    ),
                    texture_entry(
                        3,
                        S::FRAGMENT,
                        T::Float {
                            filterable: false,
                        },
                        samples == 4,
                    ),
                    texture_entry(4, S::FRAGMENT, T::Uint, samples == 4),
                    uniform(5),
                ],
            });
        let level = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient reduced original properties"),
            entries: &[
                texture(
                    0,
                    T::Float {
                        filterable: false,
                    },
                ),
                texture(1, T::Uint),
                texture(
                    2,
                    T::Float {
                        filterable: false,
                    },
                ),
            ],
        });
        let sample =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient filtered samples"),
                entries: &[
                    texture(0, T::Float { filterable: true }),
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: S::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
            });
        let mut entries = (0..4)
            .map(|i| {
                entry(
                    i,
                    S::FRAGMENT,
                    B::Storage { read_only: false },
                )
            })
            .collect::<Vec<_>>();
        entries.push(texture(4, T::Float { filterable: true }));
        entries.push(texture(
            5,
            T::Float {
                filterable: false,
            },
        ));
        let history =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient reserved current history bindings"),
                entries: &entries,
            });
        let shader_source = shader_source(samples);
        let module = |label, source: String| {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source: wgpu::ShaderSource::Wgsl(source.into()),
            })
        };
        let shader = module("current ambient recipe", shader_source);
        let depth_shader = module(
            "ambient depth recipe",
            include_str!("ambient_depth.wgsl").into(),
        );
        let composite_shader = module(
            "ambient composition",
            include_str!("ambient_composite.wgsl").into(),
        );
        let groups = [
            Some(&source),
            Some(&level),
            Some(&sample),
            Some(&history),
        ];
        let prepare = pipeline(
            device,
            &shader,
            "vs_main",
            "fs_prepare",
            &[Some(&source)],
            &[
                wgpu::TextureFormat::R32Float,
                wgpu::TextureFormat::R16Uint,
                wgpu::TextureFormat::Rg8Unorm,
            ],
            false,
            1,
        );
        let make = |fragment| {
            pipeline(
                device,
                &shader,
                "vs_main",
                fragment,
                &groups,
                &[wgpu::TextureFormat::R8Unorm],
                false,
                1,
            )
        };
        let ground = make("fs_ground");
        let raw = make("fs_main");
        let filter = [make("fs_filter_x"), make("fs_filter_y")];
        let upsample = make("fs_upsample");
        let composite_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient sparse sample corrections"),
                entries: &[
                    entry(0, S::VERTEX_FRAGMENT, B::Uniform),
                    entry(
                        1,
                        S::FRAGMENT,
                        B::Storage { read_only: true },
                    ),
                    entry(2, S::VERTEX, B::Storage { read_only: true }),
                ],
            });
        let composite = pipeline(
            device,
            &composite_shader,
            "vs_main",
            "fs_main",
            &[Some(&composite_layout), Some(&sample)],
            &[format],
            true,
            samples,
        );
        let edges = (samples == 4).then(|| {
            pipeline(
                device,
                &composite_shader,
                "vs_edges",
                "fs_edges",
                &[Some(&composite_layout), Some(&sample)],
                &[format],
                true,
                samples,
            )
        });
        let reduce = pipeline(
            device,
            &depth_shader,
            "vs_main",
            "fs_reduce",
            &[Some(&level)],
            &[
                wgpu::TextureFormat::R32Float,
                wgpu::TextureFormat::R16Uint,
            ],
            false,
            1,
        );
        let occupancy = pipeline(
            device,
            &depth_shader,
            "vs_main",
            "fs_occupancy",
            &[Some(&level)],
            &[wgpu::TextureFormat::R8Unorm],
            false,
            1,
        );
        Self {
            source,
            level,
            sample,
            history,
            prepare,
            reduce,
            occupancy,
            ground,
            raw,
            filter,
            upsample,
            composite,
            edges,
            composite_layout,
        }
    }
}
pub(crate) fn shader_source(samples: u32) -> String {
```

### 2. `src/ambient_groups.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            layout: &pipes.history,
            entries: &entries,
        });
        Self {
            levels,
            occupancy,
            ground,
            ao,
            history,
            buffers,
```

Replace it with:

```rust
            layout: &pipes.history,
            entries: &entries,
        });
        let composite = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient sparse sample corrections"),
            layout: &pipes.composite_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: frame.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffers[0].as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffers[2].as_entire_binding(),
                },
            ],
        });
        Self {
            levels,
            occupancy,
            ground,
            ao,
            history,
            composite,
            buffers,
```

**Locate this block:**

```rust
                &sampler,
            )
        });
        let buffers = std::array::from_fn(|_| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient reserved single-sample work"),
                size: 16,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        });
```

Replace it with:

```rust
                &sampler,
            )
        });
        let buffers = std::array::from_fn(|i| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient reserved single-sample work"),
                size: 16,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | if i == 3 {
                        wgpu::BufferUsages::INDIRECT
                    } else {
                        wgpu::BufferUsages::empty()
                    },
                mapped_at_creation: false,
            })
        });
```

**Locate this block:**

```rust
        device: &wgpu::Device,
        pipes: &crate::ambient_pipeline::Pipelines,
        images: &crate::ambient_images::Images,
    ) -> Self {
        let levels = std::array::from_fn(|i| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
```

Replace it with:

```rust
        device: &wgpu::Device,
        pipes: &crate::ambient_pipeline::Pipelines,
        images: &crate::ambient_images::Images,
        frame: &wgpu::Buffer,
    ) -> Self {
        let levels = std::array::from_fn(|i| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
```

**Locate this block:**

```rust
    pub history: wgpu::BindGroup,
    pub buffers: [wgpu::Buffer; 4],
}
fn sampled(
```

Replace it with:

```rust
    pub history: wgpu::BindGroup,
    pub composite: wgpu::BindGroup,
    pub buffers: [wgpu::Buffer; 4],
}
fn sampled(
```

### 3. `src/ambient_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            encoder,
            &self.pipes.composite,
            &[color],
            &[(1, &state.groups.ao[2])],
            false,
```

Replace it with:

```rust
            encoder,
            &self.pipes.composite,
            &[color],
            &[
                (0, &state.groups.composite),
                (1, &state.groups.ao[2]),
            ],
            false,
```

**Locate this block:**

```rust
            let images = Images::new(device, full, density);
            let groups = Groups::new(device, &self.pipes, &images);
            let group = Groups::source(
                device,
                &self.pipes,
```

Replace it with:

```rust
            let images = Images::new(device, full, density);
            let groups = Groups::new(device, &self.pipes, &images, &self.frame);
            let group = Groups::source(
                device,
                &self.pipes,
```

## <a id="checkpoint-96zzzzzzzi-correct-ambient-at-physical-samples"></a>16. Correct ambient at physical samples

### 1. `src/ambient_gpu.rs`

**Locate this existing block:**

```rust
use crate::{ambient_groups::Groups, ambient_images::Images, ambient_pass::draw};
struct State {
    images: Images,
    groups: Groups,
    input: (
        wgpu::TextureView,
        wgpu::TextureView,
        wgpu::BindGroup,
    ),
}
pub struct Ambient {
    pipes: crate::ambient_pipeline::Pipelines,
    pub sources: crate::ambient_source_gpu::Sources,
    frame: wgpu::Buffer,
    state: Option<State>,
    cached: Option<([f32; 80], [u64; 2])>,
    pub preparations: u64,
    pub writes: u64,
    pub passes: u64,
    pub error: Option<&'static str>,
}
impl Ambient {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            pipes: crate::ambient_pipeline::Pipelines::new(device, format),
            sources: crate::ambient_source_gpu::Sources::new(device),
            frame: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("current ambient camera"),
                size: 320,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            state: None,
            cached: None,
            preparations: 0,
            writes: 0,
            passes: 0,
            error: None,
        }
    }
    pub fn clear(&mut self) {
        self.state = None;
        self.cached = None;
        self.error = None;
    }
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        full: [u32; 2],
        density: f32,
        depth: &wgpu::TextureView,
        sections: &wgpu::TextureView,
        clip: &wgpu::Buffer,
    ) {
        let (w, h) =
            crate::pyramid_size::resolution((full[0], full[1]), f64::from(density));
        let size = [w.max(32), h.max(32)];
        if self
            .state
            .as_ref()
            .is_none_or(|s| s.images.full != full || s.images.size != size)
        {
            let images = Images::new(device, full, density);
            let groups = Groups::new(device, &self.pipes, &images, &self.frame);
            let group = Groups::source(
                device,
                &self.pipes,
                &images,
                depth,
                sections,
                &self.frame,
                clip,
            );
            self.state = Some(State {
                images,
                groups,
                input: (depth.clone(), sections.clone(), group),
            });
            self.cached = None;
            self.preparations += 1;
        }
        let state = self.state.as_mut().unwrap();
        if state.input.0 != *depth || state.input.1 != *sections {
            state.input = (
                depth.clone(),
                sections.clone(),
                Groups::source(
                    device,
                    &self.pipes,
                    &state.images,
                    depth,
                    sections,
                    &self.frame,
                    clip,
                ),
            );
            self.cached = None;
        }
    }
    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        color: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
        clip: &wgpu::Buffer,
        sections: &wgpu::TextureView,
        meshes: &[crate::gpu_mesh::GpuMesh],
        matrix: [f32; 16],
        full: [u32; 2],
        density: f32,
        ground: [f32; 2],
        revision: [u64; 2],
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        if ground[1] <= 0.0 {
            self.clear();
            return;
        }
        if matrix.iter().any(|v| !v.is_finite())
            || full.contains(&0)
            || !density.is_finite()
            || density <= 0.
        {
            self.error = Some("Invalid ambient view");
            return;
        }
        self.prepare(device, full, density, depth, sections, clip);
        let state = self.state.as_ref().unwrap();
        let Some(uniform) =
            crate::ambient_camera::uniform(matrix, full, state.images.size, ground)
        else {
            self.error = Some("Invalid ambient camera inverse");
            return;
        };
        if self.cached != Some((uniform, revision)) {
            queue.write_buffer(
                &self.frame,
                0,
                &uniform
                    .into_iter()
                    .flat_map(f32::to_ne_bytes)
                    .collect::<Vec<_>>(),
            );
            self.writes += 1;
            let images = &state.images;
            let groups = &state.groups;
            let source = &state.input.2;
            self.sources.encode(
                encoder,
                &images.source_radius.view,
                &images.source_normals.view,
                depth,
                view,
                clipping,
                meshes,
                records,
            );
            self.passes += 1;
            draw(
                encoder,
                &self.pipes.prepare,
                &[
                    &images.linear.levels[0],
                    &images.radii.levels[0],
                    &images.normals.view,
                ],
                &[(0, source)],
                true,
            );
            for level in 1..6 {
                draw(
                    encoder,
                    &self.pipes.reduce,
                    &[
                        &images.linear.levels[level],
                        &images.radii.levels[level],
                    ],
                    &[(0, &groups.levels[level - 1])],
                    true,
                );
            }
            self.passes += 6;
            draw(
                encoder,
                &self.pipes.occupancy,
                &[&images.occupancy.view],
                &[(0, &groups.levels[0])],
                true,
            );
            draw(
                encoder,
                &self.pipes.ground,
                &[&images.ground.view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.occupancy),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.raw,
                &[&images.ao[0].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ground),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.filter[0],
                &[&images.ao[1].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[0]),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.filter[1],
                &[&images.ao[0].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[1]),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.upsample,
                &[&images.ao[2].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[0]),
                    (3, &groups.history),
                ],
                true,
            );
            self.passes += 6;
            self.cached = Some((uniform, revision));
        }
        draw(
            encoder,
            &self.pipes.composite,
            &[color],
            &[
                (0, &state.groups.composite),
                (1, &state.groups.ao[2]),
            ],
            false,
        );
        self.passes += 1;
        self.error = None;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::sample_pipelines::Variants;
use crate::{ambient_groups::Groups, ambient_images::Images, ambient_pass::draw};
struct State {
    images: Images,
    groups: Groups,
    input: (
        wgpu::TextureView,
        wgpu::TextureView,
        wgpu::BindGroup,
    ),
}
pub struct Ambient {
    pipes: Variants<crate::ambient_pipeline::Pipelines>,
    format: wgpu::TextureFormat,
    pub sources: crate::ambient_source_gpu::Sources,
    frame: wgpu::Buffer,
    state: Option<State>,
    cached: Option<([f32; 80], [u64; 2])>,
    pub preparations: u64,
    pub writes: u64,
    pub passes: u64,
    pub error: Option<&'static str>,
}
impl Ambient {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::with_samples(device, format, 1)
    }
    pub fn with_samples(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        count: u32,
    ) -> Self {
        Self {
            pipes: Variants::new(
                crate::ambient_pipeline::Pipelines::with_samples(device, format, count),
                count,
            ),
            format,
            sources: crate::ambient_source_gpu::Sources::with_samples(device, count),
            frame: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("current ambient camera"),
                size: 320,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            state: None,
            cached: None,
            preparations: 0,
            writes: 0,
            passes: 0,
            error: None,
        }
    }
    pub fn set_samples(&mut self, device: &wgpu::Device, count: u32) {
        if self.pipes.select(count, || {
            crate::ambient_pipeline::Pipelines::with_samples(device, self.format, count)
        }) {
            self.sources.set_samples(device, count);
            self.cached = None;
        }
    }
    pub fn clear(&mut self) {
        self.state = None;
        self.cached = None;
        self.error = None;
    }
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        full: [u32; 2],
        density: f32,
        depth: &wgpu::TextureView,
        sections: &wgpu::TextureView,
        clip: &wgpu::Buffer,
    ) {
        let (w, h) =
            crate::pyramid_size::resolution((full[0], full[1]), f64::from(density));
        let size = [w.max(32), h.max(32)];
        if self
            .state
            .as_ref()
            .is_none_or(|s| s.images.full != full || s.images.size != size)
        {
            let images =
                Images::with_samples(device, full, density, self.pipes.samples);
            let groups = Groups::new(
                device,
                &self.pipes.active,
                &images,
                &self.frame,
                self.pipes.samples,
            );
            let group = Groups::source(
                device,
                &self.pipes.active,
                &images,
                depth,
                sections,
                &self.frame,
                clip,
            );
            self.state = Some(State {
                images,
                groups,
                input: (depth.clone(), sections.clone(), group),
            });
            self.cached = None;
            self.preparations += 1;
        }
        let state = self.state.as_mut().unwrap();
        if state
            .images
            .source_radius
            .texture
            .sample_count()
            != self.pipes.samples
        {
            state
                .images
                .retarget_sources(device, self.pipes.samples);
            state.groups = Groups::new(
                device,
                &self.pipes.active,
                &state.images,
                &self.frame,
                self.pipes.samples,
            );
            state.input = (
                depth.clone(),
                sections.clone(),
                Groups::source(
                    device,
                    &self.pipes.active,
                    &state.images,
                    depth,
                    sections,
                    &self.frame,
                    clip,
                ),
            );
            self.cached = None;
            self.preparations += 1;
        }
        if state.input.0 != *depth || state.input.1 != *sections {
            state.input = (
                depth.clone(),
                sections.clone(),
                Groups::source(
                    device,
                    &self.pipes.active,
                    &state.images,
                    depth,
                    sections,
                    &self.frame,
                    clip,
                ),
            );
            self.cached = None;
        }
    }
    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        color: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
        clip: &wgpu::Buffer,
        sections: &wgpu::TextureView,
        meshes: &[crate::gpu_mesh::GpuMesh],
        matrix: [f32; 16],
        full: [u32; 2],
        density: f32,
        ground: [f32; 2],
        revision: [u64; 2],
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        if ground[1] <= 0.0 {
            self.clear();
            return;
        }
        if matrix.iter().any(|v| !v.is_finite())
            || full.contains(&0)
            || !density.is_finite()
            || density <= 0.
        {
            self.error = Some("Invalid ambient view");
            return;
        }
        self.prepare(device, full, density, depth, sections, clip);
        let state = self.state.as_ref().unwrap();
        let Some(uniform) =
            crate::ambient_camera::uniform(matrix, full, state.images.size, ground)
        else {
            self.error = Some("Invalid ambient camera inverse");
            return;
        };
        if self.cached != Some((uniform, revision)) {
            queue.write_buffer(
                &self.frame,
                0,
                &uniform
                    .into_iter()
                    .flat_map(f32::to_ne_bytes)
                    .collect::<Vec<_>>(),
            );
            self.writes += 1;
            if self.pipes.samples == 4 {
                encoder.clear_buffer(&state.groups.buffers[1], 0, None);
                encoder.clear_buffer(&state.groups.buffers[3], 4, Some(4));
            }
            let images = &state.images;
            let groups = &state.groups;
            let source = &state.input.2;
            self.sources.encode(
                encoder,
                &images.source_radius.view,
                &images.source_normals.view,
                depth,
                view,
                clipping,
                meshes,
                records,
            );
            self.passes += 1;
            draw(
                encoder,
                &self.pipes.active.prepare,
                &[
                    &images.linear.levels[0],
                    &images.radii.levels[0],
                    &images.normals.view,
                ],
                &[(0, source)],
                true,
            );
            for level in 1..6 {
                draw(
                    encoder,
                    &self.pipes.active.reduce,
                    &[
                        &images.linear.levels[level],
                        &images.radii.levels[level],
                    ],
                    &[(0, &groups.levels[level - 1])],
                    true,
                );
            }
            self.passes += 6;
            draw(
                encoder,
                &self.pipes.active.occupancy,
                &[&images.occupancy.view],
                &[(0, &groups.levels[0])],
                true,
            );
            draw(
                encoder,
                &self.pipes.active.ground,
                &[&images.ground.view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.occupancy),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.active.raw,
                &[&images.ao[0].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ground),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.active.filter[0],
                &[&images.ao[1].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[0]),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.active.filter[1],
                &[&images.ao[0].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[1]),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.active.upsample,
                &[&images.ao[2].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[0]),
                    (3, &groups.history),
                ],
                true,
            );
            self.passes += 6;
            self.cached = Some((uniform, revision));
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ambient sample-preserving composition"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipes.active.composite);
            pass.set_bind_group(0, &state.groups.composite, &[]);
            pass.set_bind_group(1, &state.groups.ao[2], &[]);
            pass.draw(0..3, 0..1);
            if let Some(edges) = &self.pipes.active.edges {
                pass.set_pipeline(edges);
                pass.draw_indirect(&state.groups.buffers[3], 0);
            }
        }
        self.passes += 1 + u64::from(self.pipes.samples == 4);
        self.error = None;
```

### 2. `src/ambient_groups.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    } else {
                        wgpu::BufferUsages::empty()
                    },
                mapped_at_creation: false,
            })
        });
```

Replace it with:

```rust
                    } else {
                        wgpu::BufferUsages::empty()
                    },
                mapped_at_creation: i == 3,
            });
            if i == 3 {
                buffer
                    .slice(0..4)
                    .get_mapped_range_mut()
                    .copy_from_slice(&6u32.to_ne_bytes());
                buffer.unmap();
            }
            buffer
        });
```

**Locate this block:**

```rust
                &sampler,
            )
        });
        let buffers = std::array::from_fn(|i| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient reserved single-sample work"),
                size: 16,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | if i == 3 {
```

Replace it with:

```rust
                &sampler,
            )
        });
        let pixels = u64::from(images.full[0]) * u64::from(images.full[1]);
        let tiles = u64::from(images.full[0].div_ceil(16))
            * u64::from(images.full[1].div_ceil(16));
        let buffers = std::array::from_fn(|i| {
            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient bounded sample work"),
                size: if i == 3 || samples == 1 {
                    16
                } else if i == 0 {
                    pixels * 4
                } else {
                    tiles * 4
                },
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | if i == 3 {
```

**Locate this block:**

```rust
        frame: &wgpu::Buffer,
    ) -> Self {
        let levels = std::array::from_fn(|i| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
```

Replace it with:

```rust
        frame: &wgpu::Buffer,
        samples: u32,
    ) -> Self {
        let levels = std::array::from_fn(|i| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
```

### 3. `src/renderer.rs`

**Locate this existing block:**

```rust
        if enabled {
            ambient.get_or_insert_with(|| {
                crate::ambient_gpu::Ambient::new(&self.device, self.format)
            });
        } else if let Some(owner) = ambient.as_mut() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if enabled {
            let owner = ambient.get_or_insert_with(|| {
                crate::ambient_gpu::Ambient::with_samples(
                    &self.device,
                    self.format,
                    self.targets.samples,
                )
            });
            owner.set_samples(&self.device, self.targets.samples);
        } else if let Some(owner) = ambient.as_mut() {
```

### 4. `src/ambient_sample_inspect.rs`

Create the file and type:

```rust
use super::*;
impl Ambient {
    pub(crate) fn actual_sample_inputs(
        &self,
    ) -> (
        [wgpu::TextureView; 2],
        [wgpu::Buffer; 4],
        [wgpu::Texture; 3],
    ) {
        let state = self.state.as_ref().unwrap();
        (
            [
                state.images.source_radius.view.clone(),
                state.images.source_normals.view.clone(),
            ],
            state.groups.buffers.clone(),
            [
                state.images.linear.texture.clone(),
                state.images.normals.texture.clone(),
                state.images.ao[2].texture.clone(),
            ],
        )
    }
}
```

### 5. `src/ambient_gpu.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "ambient_sample_inspect.rs"]
mod sample_inspect;
```

### 6. `src/sample_surface_tests.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    );
    crate::sample_test_proof::write(root.join("uncut-four.rgba"), uncut).unwrap();
}
```

Replace it with:

```rust
    );
    crate::sample_test_proof::write(root.join("uncut-four.rgba"), uncut).unwrap();
}

fn read_ambient_samples(
    r: &Renderer,
    metadata: &std::collections::HashMap<u32, (f32, [u8; 2])>,
) -> serde_json::Value {
    let (properties, work, _) = r
        .ambient
        .borrow()
        .as_ref()
        .unwrap()
        .actual_sample_inputs();
    let module=r.device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("actual four ambient source samples"),source:wgpu::ShaderSource::Wgsl(r#"
@group(0) @binding(0) var depths:texture_depth_multisampled_2d;
@group(0) @binding(1) var primitives:texture_multisampled_2d<u32>;
@group(0) @binding(2) var radii:texture_multisampled_2d<f32>;
@group(0) @binding(3) var normals:texture_multisampled_2d<f32>;
@group(0) @binding(4) var<storage,read_write> output:array<vec4u>;
@group(0) @binding(5) var<storage,read> corrections:array<u32>;
@group(0) @binding(6) var<storage,read> flags:array<u32>;
@group(0) @binding(7) var<storage,read> tiles:array<u32>;
@group(0) @binding(8) var<storage,read> draw:array<u32>;
@compute @workgroup_size(64) fn read_samples(@builtin(global_invocation_id) id:vec3u) {
 let i=id.x;if i>=320u*240u*4u{return;}let p=i/4u;let at=vec2i(i32(p%320u),i32(p/320u));let sample=i%4u;
 output[i*2u]=vec4u(textureLoad(primitives,at,i32(sample)).xy,bitcast<u32>(textureLoad(depths,at,i32(sample))),bitcast<u32>(textureLoad(radii,at,i32(sample)).x));
 output[i*2u+1u]=vec4u(bitcast<vec2u>(textureLoad(normals,at,i32(sample)).xy),corrections[p],flags[(p/320u/16u)*20u+(p%320u)/16u]);
 if i==0u {output[614400u]=vec4u(draw[0],draw[1],arrayLength(&flags),arrayLength(&tiles));output[614401u]=vec4u(draw[2],draw[3],0,0);}
 if i<300u {let tile=tiles[i];output[614402u+i]=vec4u(tile,flags[min(tile,299u)],0,0);}
}"#.into())});
    let pipeline = r
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("read_samples"),
            compilation_options: Default::default(),
            cache: None,
        });
    let size = (614402 + 300) * 16;
    let output = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
    let read = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let views = [
        &r.targets.depth,
        &r.targets.primitive.view,
        &properties[0],
        &properties[1],
    ];
    let mut entries = views
        .into_iter()
        .enumerate()
        .map(|(i, view)| wgpu::BindGroupEntry {
            binding: i as u32,
            resource: wgpu::BindingResource::TextureView(view),
        })
        .collect::<Vec<_>>();
    entries.push(wgpu::BindGroupEntry {
        binding: 4,
        resource: output.as_entire_binding(),
    });
    for (i, buffer) in work.iter().enumerate() {
        entries.push(wgpu::BindGroupEntry {
            binding: 5 + i as u32,
            resource: buffer.as_entire_binding(),
        });
    }
    let group = r
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &entries,
        });
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups((320 * 240 * 4u32).div_ceil(64), 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &read, 0, size);
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    crate::cap_gpu_tests::wait(r);
    rx.recv().unwrap().unwrap();
    let bytes = read.slice(..).get_mapped_range();
    let word =
        |at: usize| u32::from_ne_bytes(bytes[at * 4..at * 4 + 4].try_into().unwrap());
    let mut covered = 0;
    let mut mixed = 0;
    let mut distinct_radii = 0;
    let mut corrected = 0;
    for pixel in 0..320 * 240 {
        let mut ids = [0; 4];
        let mut radii = [0; 4];
        for sample in 0..4 {
            let at = (pixel * 4 + sample) * 8;
            let id = word(at) | (word(at + 1) << 16);
            let depth = f32::from_bits(word(at + 2));
            let radius = f32::from_bits(word(at + 3));
            let n = [
                f32::from_bits(word(at + 4)),
                f32::from_bits(word(at + 5)),
            ];
            assert!(depth.is_finite() && (0.0..=1.0).contains(&depth));
            ids[sample] = id;
            radii[sample] = radius.to_bits();
            if id == 0 {
                assert_eq!(depth, 0.);
                assert_eq!(radius, 0.);
                assert_eq!(n, [0.; 2]);
            } else {
                covered += 1;
                let (expected, normal) = metadata
                    .get(&id)
                    .unwrap_or_else(|| panic!("Invented physical address{id}"));
                assert_eq!(
                    radius, *expected,
                    "Actual original radius at physical{id},sample{sample}"
                );
                for j in 0..2 {
                    assert!(
                        (((n[j] * 255.).round() as i32) - i32::from(normal[j])).abs()
                            <= 1,
                        "Actual geometric normal at physical{id},sample{sample}: {n:?} vs{normal:?}"
                    );
                }
            }
        }
        if ids.iter().any(|id| *id != ids[0]) {
            mixed += 1;
        }
        if radii
            .iter()
            .filter(|r| **r != 0)
            .any(|a| radii.iter().any(|b| *b != 0 && b != a))
        {
            distinct_radii += 1;
        }
        if word(pixel * 32 + 6) != 0 {
            corrected += 1;
        }
    }
    let header = 614400 * 4;
    let vertices = word(header);
    let instances = word(header + 1);
    assert_eq!(vertices, 6);
    assert!(
        instances > 0 && instances <= 300 * 4 && instances % 4 == 0,
        "Real sparse correction draw: {instances}"
    );
    assert_eq!(word(header + 2), 300);
    assert_eq!(word(header + 3), 300);
    assert_eq!([word(header + 4), word(header + 5)], [0, 0]);
    let mut tiles = std::collections::HashSet::new();
    for i in 0..instances as usize / 4 {
        let at = (614402 + i) * 4;
        let tile = word(at);
        assert!(
            tile < 300 && tiles.insert(tile),
            "Unique bounded correction tiles"
        );
        assert_eq!(word(at + 1), 1);
    }
    assert!(
        covered > 10000 && mixed > 100 && distinct_radii > 0 && corrected > 0,
        "Actual4samples: covered{covered},mixed{mixed},distinct original radii{distinct_radii},corrected{corrected}"
    );
    drop(bytes);
    read.unmap();
    serde_json::json!({"coveredSamples":covered,"mixedPrimitivePixels":mixed,"mixedOriginalRadiusPixels":distinct_radii,"correctedPixels":corrected,"indirectInstances":instances,"uniqueEdgeTiles":tiles.len()})
}
fn ambient_face_metadata(
    r: &Renderer,
    scene: &Scene,
) -> std::collections::HashMap<u32, (f32, [u8; 2])> {
    let mut out = std::collections::HashMap::new();
    for mesh in &r.meshes {
        let object = scene
            .objects()
            .iter()
            .find(|o| o.id == mesh.id)
            .unwrap();
        let radius = f32::from_ne_bytes(
            mesh.packed()
                [crate::object_record::RANGE + 8..crate::object_record::RANGE + 12]
                .try_into()
                .unwrap(),
        );
        assert!(radius > 0.);
        for (i, tri) in mesh
            .geometry
            .source
            .indices()
            .chunks_exact(3)
            .enumerate()
        {
            let p = tri
                .iter()
                .map(|index| {
                    let v = mesh.geometry.source.vertices()[*index as usize];
                    object
                        .model
                        .transform_point(&session_rust::Point::new(
                            v[0] as f64,
                            v[1] as f64,
                            v[2] as f64,
                        ))
                })
                .collect::<Vec<_>>();
            let a = std::array::from_fn::<_, 3, _>(|j| p[1][j] - p[0][j]);
            let b = std::array::from_fn::<_, 3, _>(|j| p[2][j] - p[0][j]);
            let n = [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ];
            let sum = n.iter().map(|x| x.abs()).sum::<f64>();
            let e = [n[0] / sum, n[1] / sum];
            let e = if n[2] < 0. {
                [
                    (1. - e[1].abs())
                        * if e[0] >= 0. {
                            1.
                        } else {
                            -1.
                        },
                    (1. - e[0].abs())
                        * if e[1] >= 0. {
                            1.
                        } else {
                            -1.
                        },
                ]
            } else {
                e
            };
            let normal = e.map(|v| ((v * 0.5 + 0.5) * 255.).round() as u8);
            assert!(
                out.insert(
                    mesh.primitive_base() + 1 + i as u32,
                    (radius, normal)
                )
                .is_none()
            );
        }
    }
    out
}

#[test]
fn actual_four_sample_ambient_reads_original_properties_and_executes_sparse_corrections()
 {
    let mut scene = Scene::empty();
    let mut big = crate::cap_gpu_tests::source(false);
    big.set_objectcolor(session_rust::Color::new(0.2, 0.4, 0.6, 1.));
    scene
        .insert(crate::prepared::PreparedMesh::new(big).unwrap())
        .unwrap();
    let mut small = crate::cap_gpu_tests::source(false);
    small.transform(
        &(&session_rust::Xform::translation(0.4, 0.25, 0.35)
            * &session_rust::Xform::scale_xyz(0.45, 0.45, 0.45)),
    );
    small.set_objectcolor(session_rust::Color::new(0.25, 0.7, 0.2, 1.));
    scene
        .insert(crate::prepared::PreparedMesh::new(small).unwrap())
        .unwrap();
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    retarget_basic_fixture(&mut r, 4);
    r.set_ambient(true);
    let mut camera = crate::camera::Camera::default();
    camera.isometric();
    camera.aspect = 4. / 3.;
    camera.target = [0., 0., -0.3];
    camera.distance = 3.;
    let transform = camera.uniform();
    let first = crate::cap_gpu_tests::pixels_at(&r, transform);
    assert_eq!(r.ambient_error(), None);
    let metadata = ambient_face_metadata(&r, &scene);
    let samples = read_ambient_samples(&r, &metadata);
    let (_, _, coarse) = r
        .ambient
        .borrow()
        .as_ref()
        .unwrap()
        .actual_sample_inputs();
    let uploads = r.stats()[0];
    let geometry = r
        .meshes
        .iter()
        .map(|m| Rc::clone(&m.geometry))
        .collect::<Vec<_>>();
    let original = scene
        .objects()
        .iter()
        .map(|o| o.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let root = std::path::Path::new("ambient-pixels");
    crate::sample_test_proof::write(root.join("four.rgba"), &first).unwrap();
    crate::sample_test_proof::write(
        root.join("samples.json"),
        serde_json::to_vec_pretty(&samples).unwrap(),
    )
    .unwrap();
    let mut single = Vec::new();
    for count in [4, 1, 4] {
        retarget_basic_fixture(&mut r, count);
        let pixels = crate::cap_gpu_tests::pixels_at(&r, transform);
        assert_eq!(r.ambient_error(), None);
        let (_, _, retained) = r
            .ambient
            .borrow()
            .as_ref()
            .unwrap()
            .actual_sample_inputs();
        assert_eq!(
            retained, coarse,
            "Changing count retains all actual coarse ambient images"
        );
        if count == 4 {
            assert!(pixels == first);
        } else {
            single = pixels;
        }
        assert_eq!(r.stats()[0], uploads);
    }
    crate::sample_test_proof::write(root.join("one.rgba"), single).unwrap();
    let repeated = read_ambient_samples(&r, &metadata);
    assert_eq!(
        repeated, samples,
        "Actual correction counters reset per changed frame and return to the same bounded work"
    );
    r.instances.ordinary.clear();
    r.set_ambient(false);
    r.set_ambient(true);
    let uniforms = crate::cap_gpu_tests::pixels_at(&r, transform);
    assert!(
        uniforms == first,
        "Exact4x ordinary/shared ambient source properties"
    );
    r.set_ambient(false);
    let plain = crate::cap_gpu_tests::pixels_at(&r, transform);
    assert!(plain != first);
    crate::sample_test_proof::write(root.join("plain-four.rgba"), plain).unwrap();
    for ((mesh, before), source) in r.meshes.iter().zip(&geometry).zip(&original) {
        assert!(Rc::ptr_eq(&mesh.geometry, before));
        assert_eq!(
            &scene
                .objects()
                .iter()
                .find(|o| o.id == mesh.id)
                .unwrap()
                .geometry()
                .unwrap()
                .to_proto(),
            source
        );
    }
}
```

**Locate this block:**

```rust
fn retarget_basic_fixture(r: &mut Renderer, count: u32) {
    super::cloud_address_tests::retarget(r, count);
    r.sheets.set_samples(&r.device, count);
    if let Some(text) = r.text.borrow_mut().as_mut() {
```

Replace it with:

```rust
fn retarget_basic_fixture(r: &mut Renderer, count: u32) {
    if r.ambient_enabled.get() {
        if let Some(owner) = r.ambient.borrow_mut().as_mut() {
            owner.set_samples(&r.device, count);
        }
    }
    super::cloud_address_tests::retarget(r, count);
    r.sheets.set_samples(&r.device, count);
    if let Some(text) = r.text.borrow_mut().as_mut() {
```

## <a id="checkpoint-96zzzzzzzj-cache-physical-depth-pyramid-inputs"></a>17. Cache physical depth-pyramid inputs

### 1. `src/pyramid_pipeline.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        Self {
            prepare,
            reduce,
            input,
            level,
        }
    }
```

Replace it with:

```rust
        Self {
            prepare,
            reduce,
            level,
        }
    }
    pub fn set_samples(&mut self, device: &wgpu::Device, samples: u32) -> bool {
        self.prepare
            .select(samples, || input(device, samples))
    }
```

**Locate this block:**

```rust
                texture(1, wgpu::TextureSampleType::Uint),
            ],
        });
        let prepare = pipeline(
            device,
            include_str!("pyramid_prepare.wgsl"),
            "fs_prepare",
            &input,
        );
        let reduce = pipeline(
            device,
            include_str!("ambient_depth.wgsl"),
```

Replace it with:

```rust
                texture(1, wgpu::TextureSampleType::Uint),
            ],
        });
        let prepare =
            crate::sample_pipelines::Variants::new(input(device, samples), samples);
        let reduce = pipeline(
            device,
            include_str!("ambient_depth.wgsl"),
```

**Locate this block:**

```rust
                false,
            )
        };
        let input = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient original depth and camera"),
            entries: &[
                texture(0, wgpu::TextureSampleType::Depth),
                crate::tile_pipelines::entry(
                    1,
                    wgpu::ShaderStages::FRAGMENT,
                    wgpu::BufferBindingType::Uniform,
                ),
            ],
        });
        let level = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient depth and radius level"),
            entries: &[
```

Replace it with:

```rust
                false,
            )
        };
        let level = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient depth and radius level"),
            entries: &[
```

**Locate this block:**

```rust
        cache: None,
    })
}
impl Pipelines {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture = |binding, sample_type| {
            crate::text_gpu_support::texture_entry(
                binding,
```

Replace it with:

```rust
        cache: None,
    })
}
fn input(
    device: &wgpu::Device,
    samples: u32,
) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout) {
    let input = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("ambient original depth and camera"),
        entries: &[
            crate::text_gpu_support::texture_entry(
                0,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::TextureSampleType::Depth,
                samples == 4,
            ),
            crate::tile_pipelines::entry(
                1,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Uniform,
            ),
        ],
    });
    let source = include_str!("pyramid_prepare.wgsl");
    let source = if samples == 4 {
        source.replace(
            "var depth:texture_depth_2d;",
            "var depth:texture_depth_multisampled_2d;",
        )
    } else {
        source.into()
    };
    (
        pipeline(device, &source, "fs_prepare", &input),
        input,
    )
}
impl Pipelines {
    pub fn with_samples(device: &wgpu::Device, samples: u32) -> Self {
        let texture = |binding, sample_type| {
            crate::text_gpu_support::texture_entry(
                binding,
```

**Locate this block:**

```rust
pub struct Pipelines {
    pub prepare: wgpu::RenderPipeline,
    pub reduce: wgpu::RenderPipeline,
    pub input: wgpu::BindGroupLayout,
    pub level: wgpu::BindGroupLayout,
}
fn pipeline(
```

Replace it with:

```rust
pub struct Pipelines {
    pub prepare: crate::sample_pipelines::Variants<(
        wgpu::RenderPipeline,
        wgpu::BindGroupLayout,
    )>,
    pub reduce: wgpu::RenderPipeline,
    pub level: wgpu::BindGroupLayout,
}
fn pipeline(
```

### 2. `src/pyramid_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                ..Default::default()
            });
            if level == 0 {
                pass.set_pipeline(&self.pipes.prepare);
                pass.set_bind_group(0, &self.input.as_ref().unwrap().1, &[]);
```

Replace it with:

```rust
                ..Default::default()
            });
            if level == 0 {
                pass.set_pipeline(&self.pipes.prepare.active.0);
                pass.set_bind_group(0, &self.input.as_ref().unwrap().1, &[]);
```

**Locate this block:**

```rust
        {
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("ambient source depth"),
                layout: &self.pipes.input,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
```

Replace it with:

```rust
        {
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("ambient source depth"),
                layout: &self.pipes.prepare.active.1,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
```

**Locate this block:**

```rust
            error: None,
        }
    }
    fn prepare(
        &mut self,
        device: &wgpu::Device,
```

Replace it with:

```rust
            error: None,
        }
    }
    pub fn set_samples(&mut self, device: &wgpu::Device, samples: u32) {
        if self.pipes.set_samples(device, samples) {
            self.input = None;
            self.cached = None;
        }
    }
    fn prepare(
        &mut self,
        device: &wgpu::Device,
```

**Locate this block:**

```rust
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            pipes: crate::pyramid_pipeline::Pipelines::new(device),
            frame: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient earlier camera"),
                size: 112,
```

Replace it with:

```rust
    pub fn new(device: &wgpu::Device) -> Self {
        Self::with_samples(device, 1)
    }
    pub fn with_samples(device: &wgpu::Device, samples: u32) -> Self {
        Self {
            pipes: crate::pyramid_pipeline::Pipelines::with_samples(device, samples),
            frame: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient earlier camera"),
                size: 112,
```

### 3. `src/pyramid_gpu_tests.rs`

**Locate this existing block:**

```rust
use crate::cap_gpu_tests::{pixels, renderer, scene, wait};
fn level(
    r: &crate::renderer::Renderer,
    texture: &wgpu::Texture,
    mip: u32,
    bytes: u32,
) -> Vec<u8> {
    let width = texture.width() >> mip;
    let height = texture.height() >> mip;
    let pitch = (width * bytes).div_ceil(256) * 256;
    let buffer = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("actual pyramid fixture"),
            size: u64::from(pitch) * u64::from(height),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            mip_level: mip,
            ..texture.as_image_copy()
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    wait(r);
    rx.recv().unwrap().unwrap();
    let view = buffer.slice(..).get_mapped_range();
    let data = view
        .chunks_exact(pitch as usize)
        .flat_map(|r| r[..(width * bytes) as usize].iter().copied())
        .collect();
    drop(view);
    buffer.unmap();
    data
}
#[test]
fn actual_pyramid_uses_six_nearest_positive_depth_levels_and_keeps_radius_addresses() {
    let (scene, _) = scene(false);
    let r = renderer(&scene);
    r.set_depth_pyramid(true);
    pixels(&r);
    assert_eq!(r.depth_pyramid_error(), None);
    let owner = r.pyramid.borrow();
    let owner = owner.as_ref().unwrap();
    let linear = owner.linear().unwrap();
    let radii = owner.radii().unwrap();
    let mut previous: Option<(Vec<f32>, Vec<u16>, usize, usize)> = None;
    for mip in 0..6 {
        let width = (linear.texture.width() >> mip) as usize;
        let height = (linear.texture.height() >> mip) as usize;
        let depths = level(&r, &linear.texture, mip, 4)
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
            .collect::<Vec<_>>();
        let encoded = level(&r, &radii.texture, mip, 2)
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
            .collect::<Vec<_>>();
        assert!(
            depths
                .iter()
                .all(|z| z.is_finite() && *z >= 0.)
        );
        if let Some((before, radius, w, h)) = &previous {
            for y in 0..height {
                for x in 0..width {
                    let mut nearest = 1e30;
                    let mut chosen = 0;
                    for i in 0..4 {
                        let q =
                            (y * 2 + i / 2).min(h - 1) * w + (x * 2 + i % 2).min(w - 1);
                        if before[q] > 0. && before[q] < nearest {
                            nearest = before[q];
                            chosen = radius[q];
                        }
                    }
                    let expected = if nearest == 1e30 {
                        0.
                    } else {
                        nearest
                    };
                    assert_eq!(
                        depths[y * width + x],
                        expected,
                        "actual reduction mip{mip} at{x},{y}"
                    );
                    assert_eq!(encoded[y * width + x], chosen);
                }
            }
        } else {
            let index = 48 * width + 104;
            assert!(
                (depths[index] - 0.5).abs() < 1e-5,
                "original section depth: {}",
                depths[index]
            );
            assert_eq!(depths[0], 0., "empty depth stays absent");
            assert!(encoded[index] > 0);
        }
        previous = Some((depths, encoded, width, height));
    }
    assert_eq!(r.depth_pyramid_usage()[3], 6);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::cap_gpu_tests::{pixels, renderer, scene, wait};
fn retarget(r: &mut crate::renderer::Renderer, count: u32) {
    assert!(matches!(count, 1 | 4));
    if r.targets.samples == count {
        return;
    }
    r.surfaces.select(count, || {
        crate::surface_pipelines::pipelines(
            &r.device,
            r.format,
            &crate::surface_view::layout(&r.device),
            &crate::gpu_layout::uniform(&r.device, crate::object_record::BYTES as u64),
            count,
        )
    });
    r.instances
        .set_samples(&r.device, r.format, count);
    for lane in [
        &mut r.mesh_ink,
        &mut r.strokes,
        &mut r.paths,
        &mut r.markers,
        &mut r.grid,
    ] {
        lane.set_samples(&r.device, count);
    }
    r.highlight.set_samples(&r.device, count);
    r.preview.set_samples(&r.device, count);
    r.measurement.set_samples(&r.device, count);
    r.guides.set_samples(&r.device, count);
    r.caps.borrow_mut().set_samples(count);
    if let Some(cloud) = r.clouds.borrow_mut().as_mut() {
        cloud.set_samples(&r.text_context(), count);
    }
    r.targets = crate::frame_targets::Targets::with_formats(
        &r.device,
        r.size,
        r.output_format,
        r.format,
        count,
    );
}
fn level(
    r: &crate::renderer::Renderer,
    texture: &wgpu::Texture,
    mip: u32,
    bytes: u32,
) -> Vec<u8> {
    let width = texture.width() >> mip;
    let height = texture.height() >> mip;
    let pitch = (width * bytes).div_ceil(256) * 256;
    let buffer = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("actual pyramid fixture"),
            size: u64::from(pitch) * u64::from(height),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            mip_level: mip,
            ..texture.as_image_copy()
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    wait(r);
    rx.recv().unwrap().unwrap();
    let view = buffer.slice(..).get_mapped_range();
    let data = view
        .chunks_exact(pitch as usize)
        .flat_map(|r| r[..(width * bytes) as usize].iter().copied())
        .collect();
    drop(view);
    buffer.unmap();
    data
}
#[test]
fn actual_pyramid_uses_six_nearest_positive_depth_levels_and_keeps_radius_addresses() {
    for samples in [1, 4] {
        let (scene, _) = scene(false);
        let mut r = renderer(&scene);
        retarget(&mut r, samples);
        r.set_depth_pyramid(true);
        r.pyramid
            .borrow_mut()
            .as_mut()
            .unwrap()
            .set_samples(&r.device, samples);
        pixels(&r);
        assert_eq!(r.depth_pyramid_error(), None);
        let owner = r.pyramid.borrow();
        let owner = owner.as_ref().unwrap();
        let linear = owner.linear().unwrap();
        let radii = owner.radii().unwrap();
        let mut previous: Option<(Vec<f32>, Vec<u16>, usize, usize)> = None;
        for mip in 0..6 {
            let width = (linear.texture.width() >> mip) as usize;
            let height = (linear.texture.height() >> mip) as usize;
            let depths = level(&r, &linear.texture, mip, 4)
                .chunks_exact(4)
                .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            let encoded = level(&r, &radii.texture, mip, 2)
                .chunks_exact(2)
                .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            assert!(
                depths
                    .iter()
                    .all(|z| z.is_finite() && *z >= 0.)
            );
            if let Some((before, radius, w, h)) = &previous {
                for y in 0..height {
                    for x in 0..width {
                        let mut nearest = 1e30;
                        let mut chosen = 0;
                        for i in 0..4 {
                            let q = (y * 2 + i / 2).min(h - 1) * w
                                + (x * 2 + i % 2).min(w - 1);
                            if before[q] > 0. && before[q] < nearest {
                                nearest = before[q];
                                chosen = radius[q];
                            }
                        }
                        let expected = if nearest == 1e30 {
                            0.
                        } else {
                            nearest
                        };
                        assert_eq!(
                            depths[y * width + x],
                            expected,
                            "actual reduction mip{mip} at{x},{y}"
                        );
                        assert_eq!(encoded[y * width + x], chosen);
                    }
                }
            } else {
                let index = 48 * width + 104;
                assert!(
                    (depths[index] - 0.5).abs() < 1e-5,
                    "original section depth: {}",
                    depths[index]
                );
                assert_eq!(depths[0], 0., "empty depth stays absent");
                assert!(encoded[index] > 0);
            }
            previous = Some((depths, encoded, width, height));
        }
        assert_eq!(r.depth_pyramid_usage()[3], 6);
    }
}
```

## <a id="checkpoint-96zzzzzzzk-retarget-the-complete-colour-frame"></a>18. Retarget the complete colour frame

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU (Rust / WASM): Adapter -&gt; (Device, Queue)</text>
  <rect x="48" y="50" width="118" height="44" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="107" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">wgpu::Instance</text>
  <line x1="166" y1="72" x2="196" y2="72" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="200" y="46" width="124" height="52" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="262" y="68" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">wgpu::Device</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="262" y="86" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">+ wgpu::Queue</text>
  <rect x="74" y="112" width="224" height="38" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="135" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Rc&lt;RefCell&lt;BrowserState&gt;&gt;</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU -&gt; GPU Bus: surface.configure(&amp;device, &amp;config)</text>
  <rect x="424" y="50" width="272" height="98" rx="6" fill="#fef9c3" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="74" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">SurfaceConfiguration</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">format: Bgra8UnormSrgb</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="116" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#2563eb">width × height: clamp(1..max_2d)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="136" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#16a34a">present_mode: Fifo (VSync)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Swapchain Texture -&gt; Browser &lt;canvas&gt;</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="58" width="216" height="58" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="84" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">get_current_texture()</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="102" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">-&gt; wgpu::TextureView</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="142" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">frame.present() ✓</text>
</svg>
</div>

### 1. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let mut owner = self.pyramid.borrow_mut();
        if enabled {
            if owner.is_none() {
                *owner = Some(crate::pyramid_gpu::Pyramid::new(
                    &self.device,
                ));
```

Replace it with:

```rust
        let mut owner = self.pyramid.borrow_mut();
        if enabled {
            if owner.is_none() {
                *owner = Some(crate::pyramid_gpu::Pyramid::with_samples(
                    &self.device,
                    self.samples(),
                ));
```

**Locate this block:**

```rust
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density {
            self.cancel_pick();
```

Replace it with:

```rust
    }

    pub fn samples(&self) -> u32 {
        self.targets.samples
    }
    pub fn set_samples(&mut self, count: u32) {
        assert!(matches!(count, 1 | 4));
        if self.samples() == count {
            return;
        }
        let device = &self.device;
        let ctx = crate::text_gpu_support::GpuCtx::new(device, &self.queue);
        self.surfaces.select(count, || {
            crate::surface_pipelines::pipelines(
                device,
                self.format,
                &crate::surface_view::layout(device),
                &crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64),
                count,
            )
        });
        self.instances
            .set_samples(device, self.format, count);
        for lane in [
            &mut self.mesh_ink,
            &mut self.strokes,
            &mut self.paths,
            &mut self.markers,
            &mut self.grid,
        ] {
            lane.set_samples(device, count);
        }
        self.highlight.set_samples(device, count);
        self.preview.set_samples(device, count);
        self.measurement.set_samples(device, count);
        self.guides.set_samples(device, count);
        self.sheets.set_samples(device, count);
        self.caps.borrow_mut().set_samples(count);
        if let Some(lane) = self.clouds.borrow_mut().as_mut() {
            lane.set_samples(&ctx, count);
        }
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            lane.retarget(
                &ctx,
                crate::text_gpu_support::Target {
                    format: self.format,
                    samples: count,
                },
            );
        }
        for outline in [&self.global_outline, &self.outline] {
            if let Some(outline) = outline.borrow_mut().as_mut() {
                outline.set_samples(&ctx, count);
            }
        }
        if let Some(owner) = self.outline_composite.borrow_mut().as_mut() {
            owner.set_samples(device, count);
        }
        if self.ambient_enabled.get() {
            if let Some(owner) = self.ambient.borrow_mut().as_mut() {
                owner.set_samples(device, count);
            }
        }
        if let Some(owner) = self.pyramid.borrow_mut().as_mut() {
            owner.set_samples(device, count);
        }
        self.targets = crate::frame_targets::Targets::with_formats(
            device,
            self.size,
            self.output_format,
            self.format,
            count,
        );
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density {
            self.cancel_pick();
```

### 2. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
```

Replace it with:

```rust
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    renderer.set_samples(sample_forced.unwrap_or(1));
    editor.camera.aspect = size.aspect();
```

**Locate this block:**

```rust
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer
```

Replace it with:

```rust
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
    sample_forced: Option<u32>,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer
```

**Locate this block:**

```rust
    )?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"lighting":editor.lit,"backfaces":editor.backfaces,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute(
        "data-triangle-tiles",
        &serde_json::to_string(&renderer.tile_usage()).unwrap(),
    )?;
```

Replace it with:

```rust
    )?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"lighting":editor.lit,"backfaces":editor.backfaces,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute(
        "data-samples",
        &renderer.samples().to_string(),
    )?;
    canvas.set_attribute(
        "data-triangle-tiles",
        &serde_json::to_string(&renderer.tile_usage()).unwrap(),
    )?;
```

**Locate this block:**

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-origin")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
```

Replace it with:

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-samples")?;
    listeners.listen(window.as_ref(), "viewer-origin")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
```

**Locate this block:**

```rust
                &mut config,
                &mut renderer,
                &mut editor,
            ) {
                if let Err(error) = panel.update(None, &input_canvas) {
                    report(&format!(
```

Replace it with:

```rust
                &mut config,
                &mut renderer,
                &mut editor,
                sample_forced,
            ) {
                if let Err(error) = panel.update(None, &input_canvas) {
                    report(&format!(
```

**Locate this block:**

```rust
                return;
            }
            let mut highlight_changed = false;
            let action = if event.type_() == "cancel"
                && event
                    .target()
                    .and_then(|target| {
```

Replace it with:

```rust
                return;
            }
            let mut highlight_changed = false;
            let action = if event.type_() == "viewer-samples" {
                if let Some(event) = event.dyn_ref::<web_sys::CustomEvent>() {
                    let value = event.detail();
                    if value.is_null() {
                        sample_forced = None;
                    } else if let Some(count) = value
                        .as_f64()
                        .filter(|v| *v == 1. || *v == 4.)
                    {
                        sample_forced = Some(count as u32);
                    }
                }
                None
            } else if event.type_() == "cancel"
                && event
                    .target()
                    .and_then(|target| {
```

**Locate this block:**

```rust
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        crate::browser_rebase::prepare(&mut renderer, &mut editor)?;
```

Replace it with:

```rust
        &mut config,
        &mut renderer,
        &mut editor,
        sample_forced,
    ) {
        panel.update(None, &canvas)?;
        crate::browser_rebase::prepare(&mut renderer, &mut editor)?;
```

**Locate this block:**

```rust
    drop(probe);
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
```

Replace it with:

```rust
    drop(probe);
    let mut sample_forced = None;
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
```

### 3. `src/cloud_address_tests.rs`

**Locate this existing block:**

```rust
pub(super) fn retarget(r: &mut Renderer, count: u32) {
    assert!(matches!(count, 1 | 4));
    if r.targets.samples == count {
        return;
    }
    r.surfaces.select(count, || {
        crate::surface_pipelines::pipelines(
            &r.device,
            r.format,
            &crate::surface_view::layout(&r.device),
            &crate::gpu_layout::uniform(&r.device, crate::object_record::BYTES as u64),
            count,
        )
    });
    r.instances
        .set_samples(&r.device, r.format, count);
    for lane in [
        &mut r.mesh_ink,
        &mut r.strokes,
        &mut r.paths,
        &mut r.markers,
        &mut r.grid,
    ] {
        lane.set_samples(&r.device, count);
    }
    r.highlight.set_samples(&r.device, count);
    r.preview.set_samples(&r.device, count);
    r.measurement.set_samples(&r.device, count);
    r.guides.set_samples(&r.device, count);
    r.caps.borrow_mut().set_samples(count);
    if let Some(cloud) = r.clouds.borrow_mut().as_mut() {
        cloud.set_samples(&r.text_context(), count);
    }
    r.targets = crate::frame_targets::Targets::with_formats(
        &r.device,
        r.size,
        r.output_format,
        r.format,
        count,
    );
}
fn physical(r: &Renderer) -> Vec<[u32; 4]> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(super) fn retarget(r: &mut Renderer, count: u32) {
    r.set_samples(count)
}

fn physical(r: &Renderer) -> Vec<[u32; 4]> {
```

### 4. `src/sample_surface_tests.rs`

**Locate this existing block:**

```rust
fn retarget_basic_fixture(r: &mut Renderer, count: u32) {
    if r.ambient_enabled.get() {
        if let Some(owner) = r.ambient.borrow_mut().as_mut() {
            owner.set_samples(&r.device, count);
        }
    }
    super::cloud_address_tests::retarget(r, count);
    r.sheets.set_samples(&r.device, count);
    if let Some(text) = r.text.borrow_mut().as_mut() {
        text.retarget(
            &r.text_context(),
            crate::text_gpu_support::Target {
                format: r.format,
                samples: count,
            },
        );
    }
    let ctx = r.text_context();
    for owner in [&r.global_outline, &r.outline] {
        if let Some(owner) = owner.borrow_mut().as_mut() {
            owner.set_samples(&ctx, count);
        }
    }
    if let Some(owner) = r.outline_composite.borrow_mut().as_mut() {
        owner.set_samples(&r.device, count);
    }
}
#[test]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn retarget_basic_fixture(r: &mut Renderer, count: u32) {
    r.set_samples(count)
}

#[test]
```

### 5. `src/pyramid_gpu_tests.rs`

**Locate this existing block:**

```rust
fn retarget(r: &mut crate::renderer::Renderer, count: u32) {
    assert!(matches!(count, 1 | 4));
    if r.targets.samples == count {
        return;
    }
    r.surfaces.select(count, || {
        crate::surface_pipelines::pipelines(
            &r.device,
            r.format,
            &crate::surface_view::layout(&r.device),
            &crate::gpu_layout::uniform(&r.device, crate::object_record::BYTES as u64),
            count,
        )
    });
    r.instances
        .set_samples(&r.device, r.format, count);
    for lane in [
        &mut r.mesh_ink,
        &mut r.strokes,
        &mut r.paths,
        &mut r.markers,
        &mut r.grid,
    ] {
        lane.set_samples(&r.device, count);
    }
    r.highlight.set_samples(&r.device, count);
    r.preview.set_samples(&r.device, count);
    r.measurement.set_samples(&r.device, count);
    r.guides.set_samples(&r.device, count);
    r.caps.borrow_mut().set_samples(count);
    if let Some(cloud) = r.clouds.borrow_mut().as_mut() {
        cloud.set_samples(&r.text_context(), count);
    }
    r.targets = crate::frame_targets::Targets::with_formats(
        &r.device,
        r.size,
        r.output_format,
        r.format,
        count,
    );
}
fn level(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn retarget(r: &mut crate::renderer::Renderer, count: u32) {
    r.set_samples(count)
}

fn level(
```

### 6. `src/sample_surface_tests.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[test]
fn a_colour_sample_count_change_preserves_an_inflight_independent_owner_query() {
    let (scene, owner) = crate::cap_gpu_tests::scene(false);
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    let matrix = crate::cap_gpu_tests::matrix();
    assert_eq!(r.samples(), 1);
    r.request_pick([0.3, 0.2], matrix).unwrap();
    let usage = r.pick_usage();
    r.set_samples(4);
    assert_eq!(r.samples(), 4);
    assert_eq!(
        r.pick_usage(),
        usage,
        "Colour-only retarget must retain its independent1x query and ticket"
    );
    crate::cap_gpu_tests::wait(&r);
    assert_eq!(
        r.take_pick(matrix)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
    let first = crate::cap_gpu_tests::pixels(&r);
    let targets = (
        r.targets.depth.clone(),
        r.targets.primitive.view.clone(),
    );
    let stats = r.stats();
    r.set_samples(4);
    assert_eq!(
        (&r.targets.depth, &r.targets.primitive.view),
        (&targets.0, &targets.1)
    );
    assert_eq!(r.stats(), stats);
    assert!(crate::cap_gpu_tests::pixels(&r) == first);
    r.request_pick([-0.3, 0.2], matrix).unwrap();
    let usage = r.pick_usage();
    r.set_samples(1);
    assert_eq!(r.pick_usage(), usage);
    crate::cap_gpu_tests::wait(&r);
    assert_eq!(
        r.take_pick(matrix)
            .unwrap()
            .unwrap()
            .unwrap()[0],
        owner.value()
    );
}
```

### 7. `src/sample_composition_tests.rs`

Create the file and type:

```rust
use super::*;
use crate::{
    camera::{Camera, Projection, View},
    scene::Scene,
};
use session_rust::{Point, Xform};
use std::rc::Rc;

fn composition() -> (Scene, Vec<(ObjectId, [f64; 3])>) {
    let mut scene = Scene::empty();
    let cad = scene
        .insert_cad(Rc::new(crate::cad_display::example()))
        .unwrap();
    scene
        .place(cad, Xform::translation(-3., 1.5, 0.))
        .unwrap();
    let original = crate::surface::example();
    let roof = scene
        .insert_surface(Rc::new(original.clone()))
        .unwrap();
    let roof_place = Xform::translation(2.5, 1.5, 0.);
    scene
        .place(roof, roof_place.clone())
        .unwrap();
    for direction in 0..2 {
        for parameter in [0.25, 0.5, 0.75] {
            let mut curve = original
                .iso_curve(direction, parameter)
                .unwrap();
            curve.width = 1.5;
            curve.name = format!("Original roof iso {direction} at {parameter}");
            let id = scene
                .insert_path(
                    crate::chain::PreparedChain::curve(Rc::new(curve)).unwrap(),
                )
                .unwrap();
            scene
                .place_path(id, roof_place.clone())
                .unwrap();
        }
    }
    let mut cube = session_rust::Mesh::create_box(1.2, 1.2, 1.2);
    cube.name = "Original green cube".into();
    cube.set_objectcolor(session_rust::Color::new(0.15, 0.7, 0.25, 1.));
    let cube = scene
        .insert(crate::prepared::PreparedMesh::new(cube).unwrap())
        .unwrap();
    scene
        .place(cube, Xform::translation(0., 1.8, 0.))
        .unwrap();
    crate::scene_document::import(
        &mut scene,
        &crate::instance_specimen::display_bytes(),
        "Shared originals",
        &Xform::translation(0., -1.8, 0.8),
        0.,
    )
    .unwrap();
    let cloud = scene
        .insert_cloud(
            crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap(),
        )
        .unwrap();
    scene
        .place_cloud(cloud, Xform::translation(-3., -2., 0.3))
        .unwrap();
    let sheet = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(Rc::new(crate::sheet_example::source()))
                .unwrap(),
        ))
        .unwrap();
    scene
        .place_sheet(sheet, Xform::translation(2.5, -2., 0.))
        .unwrap();
    let mut line = session_rust::Line::new(-4.4, -3.5, 0.2, -1.3, -3.5, 0.2);
    line.width = 2.;
    let line = scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap())
        .unwrap();
    let point = scene
        .insert_point(
            crate::marker::PreparedPoint::new(Rc::new(Point::new(4.2, -3.4, 0.3)))
                .unwrap(),
        )
        .unwrap();
    for (text, placement) in [
        (
            "CAD bore",
            crate::text::TextPlacement::WorldBillboard {
                world: [-3., 3.5, 0.8],
                world_height: 0.25,
            },
        ),
        (
            "Original NURBS",
            crate::text::TextPlacement::WorldPlane {
                world: [1.5, 3.1, 0.4],
                right: [1., 0., 0.],
                up: [0., 1., 0.],
                world_height: 0.22,
            },
        ),
        (
            "Drawing",
            crate::text::TextPlacement::Nameplate {
                world: [2.5, -4.2, 0.5],
                padding: [5., 3.],
                rounded: true,
            },
        ),
    ] {
        let mut label = crate::scene_label::example(placement);
        label.label.text = text.into();
        label.label.color = [255; 4];
        scene
            .insert_text(crate::scene_label::Prepared::new(Rc::new(label)).unwrap())
            .unwrap();
    }
    (
        scene,
        vec![
            (cad, [-4.4, 1.5, 0.3]),
            (roof, [2.7, 1.7, 0.2]),
            (cloud, [-3., -2., 0.3]),
            (sheet, [2.5, -2.5, 0.3]),
            (line, [-2.5, -3.5, 0.2]),
            (point, [4.2, -3.4, 0.3]),
            (cube, [0., 1.8, 0.6]),
        ],
    )
}

fn pixels(r: &Renderer, matrix: [f32; 16]) -> Vec<u8> {
    let texture = r
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("actual combined original viewer frame"),
            size: wgpu::Extent3d {
                width: 640,
                height: 480,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: r.output_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[r.format],
        });
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(r.format),
        ..Default::default()
    });
    r.draw(&view, &Default::default(), &matrix);
    let read = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 2560 * 480,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &read,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(2560),
                rows_per_image: Some(480),
            },
        },
        texture.size(),
    );
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    crate::cap_gpu_tests::wait(r);
    rx.recv().unwrap().unwrap();
    let pixels = read.slice(..).get_mapped_range().to_vec();
    read.unmap();
    pixels
}

fn ndc(matrix: [f32; 16], point: [f64; 3]) -> [f32; 2] {
    let clip = std::array::from_fn::<_, 4, _>(|i| {
        matrix[12 + i]
            + (0..3)
                .map(|j| matrix[j * 4 + i] * point[j] as f32)
                .sum::<f32>()
    });
    assert!(clip[3] > 0.);
    [clip[0] / clip[3], clip[1] / clip[3]]
}

fn archive_contents(
    bytes: &[u8],
) -> (
    Vec<session_rust::proto::Session>,
    serde_json::Value,
) {
    use prost::Message;
    let bundle = crate::archive::Bundle::decode(bytes).unwrap();
    (
        bundle
            .documents
            .iter()
            .map(|bytes| {
                session_rust::proto::Session::decode(bytes.as_slice()).unwrap()
            })
            .collect(),
        serde_json::to_value(bundle.metadata).unwrap(),
    )
}

#[test]
fn combined_original_families_keep_holes_ink_text_queries_and_exact_sample_returns() {
    let (mut scene, owners) = composition();
    let archive = crate::scene_archive::save(&scene).unwrap();
    crate::sample_test_proof::write("composition.session", &archive).unwrap();
    crate::sample_test_proof::write("composition-owners.json", serde_json::to_vec_pretty(&owners.iter().map(|(id, point)| serde_json::json!({"id": id.value(), "guid": scene.row_guid(*id).unwrap(), "world": point})).collect::<Vec<_>>()).unwrap()).unwrap();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) =
        pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e| {
        panic!("actual composed frame: {e}")
    }));
    let mut r = Renderer::with_formats(
        device,
        queue,
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        &scene,
    );
    r.resize(crate::viewport::Viewport {
        width: 640,
        height: 480,
    });
    let mut camera = Camera::default();
    camera.aspect = 4. / 3.;
    camera.fit(&scene.bounds().unwrap());
    let geometry = r
        .meshes
        .iter()
        .map(|m| Rc::clone(&m.geometry))
        .collect::<Vec<_>>();
    let uploads = r.stats()[0];
    for projection in [
        Projection::Orthographic,
        Projection::Perspective,
    ] {
        camera.set_projection_framed(projection, scene.bounds().as_ref());
        let matrix = camera.uniform();
        r.cloud_camera(&camera);
        r.set_outlines(true);
        r.set_ambient(true);
        r.set_samples(4);
        let first = pixels(&r, matrix);
        assert_eq!(r.ambient_error(), None);
        assert_eq!(r.text_error(), None);
        assert!(
            first
                .chunks_exact(4)
                .filter(|p| p[0].abs_diff(p[2]) > 20)
                .count()
                > 5000,
            "Original coloured geometry is visible"
        );
        crate::sample_test_proof::write(
            format!("composition-{projection:?}-four.rgba"),
            &first,
        )
        .unwrap();
        for count in [1, 4] {
            r.set_samples(count);
            let current = pixels(&r, matrix);
            assert_eq!(r.ambient_error(), None);
            assert_eq!(r.text_error(), None);
            if count == 4 {
                assert!(
                    current == first,
                    "Composed4→1→4 restores every scene pixel"
                );
            } else {
                crate::sample_test_proof::write(
                    format!("composition-{projection:?}-one.rgba"),
                    &current,
                )
                .unwrap();
            }
            assert_eq!(r.stats()[0], uploads);
            for (mesh, before) in r.meshes.iter().zip(&geometry) {
                assert!(Rc::ptr_eq(&mesh.geometry, before));
            }
            let after_archive = crate::scene_archive::save(&scene).unwrap();
            crate::sample_test_proof::write(
                "composition-after.session",
                &after_archive,
            )
            .unwrap();
            assert!(
                archive_contents(&after_archive) == archive_contents(&archive),
                "Every source field, ordered sequence and presentation field is unchanged"
            );
        }
    }
    r.set_ambient(false);
    r.set_outlines(false);
    camera.set_projection_framed(
        Projection::Orthographic,
        scene.bounds().as_ref(),
    );
    camera.set_view(View::Top);
    camera.fit(&scene.bounds().unwrap());
    let matrix = camera.uniform();
    r.cloud_camera(&camera);
    let at = ndc(matrix, [-3., 1.5, 0.]);
    let x = ((at[0] + 1.) * 320.).floor() as usize;
    let y = ((1. - at[1]) * 240.).floor() as usize;
    for count in [1, 4] {
        r.set_samples(count);
        let frame = pixels(&r, matrix);
        for dy in -2..=2 {
            for dx in -2..=2 {
                let i =
                    ((y as isize + dy) as usize * 640 + (x as isize + dx) as usize) * 4;
                assert!(
                    frame[i..i + 3].iter().all(|v| *v > 250),
                    "Original BRep bore remains open at{count}x"
                );
            }
        }
        crate::sample_test_proof::write(
            format!("composition-top-{count}.rgba"),
            &frame,
        )
        .unwrap();
        for (owner, world) in &owners {
            r.request_pick(ndc(matrix, *world), matrix)
                .unwrap();
            crate::cap_gpu_tests::wait(&r);
            assert_eq!(
                r.take_pick(matrix)
                    .unwrap()
                    .unwrap()
                    .unwrap()[0],
                owner.value(),
                "Fresh original owner at{count}x for{world:?}"
            );
        }
    }
    let plane = crate::clipping_geometry::plane_from(
        crate::clipping_geometry::Mode::Xy,
        &[[0., 0., 0.3]],
        6.,
    )
    .unwrap();
    crate::plane_original::insert(&mut scene, Rc::new(plane)).unwrap();
    let clipped_archive =
        archive_contents(&crate::scene_archive::save(&scene).unwrap());
    r.set_scene(&scene, None);
    r.set_clipping(true, false);
    r.set_outlines(true);
    r.set_ambient(true);
    r.set_grid(Some(crate::grid::Settings {
        spacing: 1.,
        half_cells: 6,
    }))
    .unwrap();
    r.set_measurement(Some(&crate::measure_mark::Mark {
        points: vec![[-4.4, -3.5, 0.2], [-1.3, -3.5, 0.2]],
        label: "Original distance".into(),
    }))
    .unwrap();
    let guide = crate::guide_gpu::Overlay {
        segments: vec![crate::guide_gpu::Segment {
            stroke: crate::stroke::Stroke {
                start: [1., -3.5, 0.5],
                end: [3.7, -3.5, 0.5],
                width: 1.5,
                colour: [0.8, 0.15, 0.15, 1.],
            },
            dashed: true,
        }],
        ..Default::default()
    };
    r.set_guides(&guide).unwrap();
    let mut preview = crate::tool::Draft::new(crate::tool::Shape::Line);
    preview.submit([1., -3.1, 0.6]).unwrap();
    preview
        .hover(Some([3.7, -3.1, 0.6]))
        .unwrap();
    r.set_preview(Some(&preview)).unwrap();
    camera.set_view(View::Iso);
    camera.fit(&scene.bounds().unwrap());
    let matrix = camera.uniform();
    r.cloud_camera(&camera);
    r.set_samples(4);
    let clipped = pixels(&r, matrix);
    assert_eq!(r.cap_error(), None);
    assert!(
        r.cap_usage()[1] > 0,
        "Actual closed originals participate in cap composition"
    );
    crate::sample_test_proof::write("composition-clipped-four.rgba", &clipped).unwrap();
    r.set_samples(1);
    let single = pixels(&r, matrix);
    crate::sample_test_proof::write("composition-clipped-one.rgba", &single).unwrap();
    r.set_samples(4);
    assert!(
        pixels(&r, matrix) == clipped,
        "Caps, outlines, ambient, grid, preview, guides and text restore together"
    );
    assert!(
        archive_contents(&crate::scene_archive::save(&scene).unwrap())
            == clipped_archive
    );
    r.set_grid(None).unwrap();
    r.set_ambient(false);
    r.set_outlines(false);
    r.set_measurement(None).unwrap();
    r.set_guides(&crate::guide_gpu::Overlay::default())
        .unwrap();
    r.set_preview(None).unwrap();
    camera.set_view(View::Top);
    camera.fit(&scene.bounds().unwrap());
    let matrix = camera.uniform();
    r.cloud_camera(&camera);
    let at = ndc(matrix, [-3.15, 1.5, 0.3]);
    let x = ((at[0] + 1.) * 320.).floor() as usize;
    let y = ((1. - at[1]) * 240.).floor() as usize;
    for count in [1, 4] {
        r.set_samples(count);
        let frame = pixels(&r, matrix);
        for dy in -2..=2 {
            for dx in -2..=2 {
                let i =
                    ((y as isize + dy) as usize * 640 + (x as isize + dx) as usize) * 4;
                assert!(
                    frame[i..i + 3].iter().all(|v| *v > 250),
                    "Clipped original BRep bore must remain open at{count}x; pixel{:?}",
                    &frame[i..i + 4]
                );
            }
        }
        crate::sample_test_proof::write(
            format!("composition-clipped-top-{count}.rgba"),
            &frame,
        )
        .unwrap();
    }
}
```

### 8. `src/renderer.rs`

**Locate this existing block:**

```rust
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "cloud_address_tests.rs"]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "sample_composition_tests.rs"]
mod composition_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "cloud_address_tests.rs"]
```

## <a id="checkpoint-96zzzzzzzl-choose-bounded-frame-quality"></a>19. Choose bounded frame quality

### 1. `src/sample_quality.rs`

Create the file and type:

```rust
pub fn budget(gpu: wgpu::DeviceType) -> Option<u32> {
    match gpu {
        wgpu::DeviceType::DiscreteGpu => Some(9_000_000),
        wgpu::DeviceType::IntegratedGpu | wgpu::DeviceType::VirtualGpu => {
            Some(2_500_000)
        }
        wgpu::DeviceType::Other => Some(4_200_000),
        wgpu::DeviceType::Cpu => None,
    }
}

pub fn samples(
    solid: bool,
    size: [u32; 2],
    forced: Option<u32>,
    budget: Option<u32>,
    pixel_scale: f32,
    reduced: bool,
) -> u32 {
    if reduced {
        return 1;
    }
    if let Some(count) = forced {
        return if count == 4 {
            4
        } else {
            1
        };
    }
    if pixel_scale >= 2. {
        return 1;
    }
    let pixels = u64::from(size[0]) * u64::from(size[1]);
    if solid && budget.is_some_and(|max| pixels <= u64::from(max)) {
        4
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn policy_uses_device_budgets_actual_density_and_explicit_recovery_precedence() {
        use super::*;
        for (device, max) in [
            (wgpu::DeviceType::DiscreteGpu, 9_000_000),
            (wgpu::DeviceType::IntegratedGpu, 2_500_000),
            (wgpu::DeviceType::VirtualGpu, 2_500_000),
            (wgpu::DeviceType::Other, 4_200_000),
        ] {
            assert_eq!(budget(device), Some(max));
            assert_eq!(
                samples(
                    true,
                    [max, 1],
                    None,
                    budget(device),
                    1.,
                    false
                ),
                4
            );
            assert_eq!(
                samples(
                    true,
                    [max + 1, 1],
                    None,
                    budget(device),
                    1.,
                    false
                ),
                1
            );
        }
        assert_eq!(budget(wgpu::DeviceType::Cpu), None);
        assert_eq!(
            samples(true, [900, 760], None, None, 1., false),
            1
        );
        assert_eq!(
            samples(
                false,
                [900, 760],
                None,
                Some(9_000_000),
                1.,
                false
            ),
            1
        );
        assert_eq!(
            samples(
                true,
                [u32::MAX, u32::MAX],
                None,
                Some(u32::MAX),
                1.,
                false
            ),
            1
        );
        assert_eq!(
            samples(
                true,
                [900, 760],
                None,
                Some(9_000_000),
                1.999,
                false
            ),
            4
        );
        assert_eq!(
            samples(
                true,
                [900, 760],
                None,
                Some(9_000_000),
                2.,
                false
            ),
            1
        );
        assert_eq!(
            samples(
                false,
                [u32::MAX, u32::MAX],
                Some(4),
                None,
                3.,
                false
            ),
            4
        );
        assert_eq!(
            samples(
                true,
                [900, 760],
                Some(4),
                Some(9_000_000),
                1.,
                true
            ),
            1
        );
        for forced in [0, 1, 2, 3, 5, 8, u32::MAX] {
            assert_eq!(
                samples(
                    true,
                    [900, 760],
                    Some(forced),
                    Some(9_000_000),
                    1.,
                    false
                ),
                1
            );
        }
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod revision;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod revision;
pub mod sample_quality;
#[cfg(test)]
```

### 3. `src/renderer.rs`

**Locate this existing block:**

```rust
    }

    pub fn samples(&self) -> u32 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn has_solids(&self) -> bool {
        self.meshes
            .iter()
            .any(|mesh| mesh.geometry.index_count > 0)
            || self
                .text
                .borrow()
                .as_ref()
                .is_some_and(|lane| {
                    lane.document.runs.iter().any(|run| {
                        !matches!(
                            run.label.placement,
                            crate::text::TextPlacement::Screen { .. }
                        )
                    })
                })
    }
    pub fn samples(&self) -> u32 {
```

### 4. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    renderer.set_samples(sample_forced.unwrap_or(1));
    editor.camera.aspect = size.aspect();
```

Replace it with:

```rust
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    let samples = crate::sample_quality::samples(
        renderer.has_solids(),
        [size.width, size.height],
        sample_forced,
        sample_budget,
        size.width as f32 / rect.width().max(1.) as f32,
        crate::browser_recovery::reduced(),
    );
    renderer.set_samples(samples);
    editor.camera.aspect = size.aspect();
```

**Locate this block:**

```rust
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
    sample_forced: Option<u32>,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
```

Replace it with:

```rust
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
    sample_budget: Option<u32>,
    sample_forced: Option<u32>,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
```

**Locate this block:**

```rust
                &mut config,
                &mut renderer,
                &mut editor,
                sample_forced,
            ) {
                if let Err(error) = panel.update(None, &input_canvas) {
```

Replace it with:

```rust
                &mut config,
                &mut renderer,
                &mut editor,
                sample_budget,
                sample_forced,
            ) {
                if let Err(error) = panel.update(None, &input_canvas) {
```

**Locate this block:**

```rust
        &mut config,
        &mut renderer,
        &mut editor,
        sample_forced,
    ) {
        panel.update(None, &canvas)?;
```

Replace it with:

```rust
        &mut config,
        &mut renderer,
        &mut editor,
        sample_budget,
        sample_forced,
    ) {
        panel.update(None, &canvas)?;
```

**Locate this block:**

```rust
    drop(probe);
    let mut sample_forced = None;
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
```

Replace it with:

```rust
    drop(probe);
    let sample_budget = crate::sample_quality::budget(adapter.get_info().device_type);
    let mut sample_forced = window
        .location()
        .search()
        .ok()
        .and_then(|s| web_sys::UrlSearchParams::new_with_str(&s).ok())
        .and_then(|q| q.get("msaa"))
        .and_then(|v| v.parse::<u32>().ok());
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
```

### 5. `src/browser_recovery.rs`

**Locate this existing block:**

```rust
thread_local! { static POLICY: RefCell<crate::recovery::Recovery> = RefCell::new(Default::default()); }

pub fn density(value: f64) -> f64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
thread_local! { static POLICY: RefCell<crate::recovery::Recovery> = RefCell::new(Default::default()); }

pub fn reduced() -> bool {
    POLICY.with(|policy| policy.borrow().reduced())
}

pub fn density(value: f64) -> f64 {
```

### 6. `src/sample_surface_tests.rs`

**Locate this existing block:**

```rust
        owner.value()
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        owner.value()
    );
}

#[test]
fn automatic_sampling_eligibility_follows_active_faces_and_world_lettering() {
    let mut scene = Scene::empty();
    let mut r = crate::cap_gpu_tests::renderer(&scene);
    assert!(!r.has_solids());
    let sheet = scene
        .insert_sheet(Rc::new(
            crate::sheet_source::Prepared::new(Rc::new(crate::sheet_example::source()))
                .unwrap(),
        ))
        .unwrap();
    r.set_scene(&scene, None);
    assert!(
        !r.has_solids(),
        "Original drawing pens are ribbons, not the reference arena's sheet fills"
    );
    let mesh = scene
        .insert(
            crate::prepared::PreparedMesh::triangle(
                [
                    [-0.5, -0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    [0., 0.5, 0.5],
                ],
                [0.2, 0.4, 0.8],
            )
            .unwrap(),
        )
        .unwrap();
    r.set_scene(&scene, None);
    assert!(r.has_solids());
    r.set_opacity(0.);
    assert!(
        r.has_solids(),
        "Eligibility describes geometry, not the current opacity"
    );
    Rc::make_mut(&mut scene.view).hide(&[mesh], true);
    r.set_scene(&scene, None);
    assert!(!r.has_solids());
    let mut isolated = session_rust::Mesh::new();
    isolated.add_vertex(
        session_rust::Point::new(0., 0., 0.),
        Some(17),
    );
    let isolated = crate::prepared::PreparedMesh::new(isolated).unwrap();
    assert!(
        isolated.display.parts.edges.is_empty()
            && isolated.display.parts.dots.is_empty(),
        "Isolated source vertices do not produce mesh pipes or glyphs"
    );
    let mut empty_faces = Scene::empty();
    empty_faces.insert(isolated).unwrap();
    r.set_scene(&empty_faces, None);
    assert!(!r.has_solids());
    for kind in ["surface", "brep", "plane"] {
        let mut sample = Scene::empty();
        match kind {
            "surface" => {
                sample
                    .insert_surface(Rc::new(crate::surface::example()))
                    .unwrap();
            }
            "brep" => {
                sample
                    .insert_cad(Rc::new(session_rust::BRep::create_box(
                        1., 1., 1.,
                    )))
                    .unwrap();
            }
            _ => {
                crate::plane_original::insert(
                    &mut sample,
                    Rc::new(session_rust::Plane::xy_plane()),
                )
                .unwrap();
            }
        }
        let owner = sample.objects()[0].id;
        let source = Rc::clone(&sample.objects()[0].mesh);
        r.set_scene(&sample, None);
        assert_eq!(
            r.has_solids(),
            kind != "plane",
            "Validated surface and CAD boundaries accompany faces; original plane pens are ribbons"
        );
        assert!(Rc::ptr_eq(
            &source,
            &sample.objects()[0].mesh
        ));
        Rc::make_mut(&mut sample.view).hide(&[owner], true);
        r.set_scene(&sample, None);
        assert!(
            !r.has_solids(),
            "Hidden original source families do not qualify"
        );
    }
    r.set_scene(&scene, None);
    let mut text = crate::text_example::label();
    r.set_text(vec![text.clone()]).unwrap();
    assert!(
        !r.has_solids(),
        "Screen overlay lettering is not world geometry"
    );
    text.placement = crate::text::TextPlacement::WorldBillboard {
        world: [0.; 3],
        world_height: 0.1,
    };
    r.set_text(vec![text]).unwrap();
    assert!(
        r.has_solids(),
        "World lettering is a filled world surface"
    );
    assert!(scene.visible(sheet));
}
```

### 7. `src/sample_positions_tests.rs`

Create the file and type:

```rust
#[test]
fn actual_sample_coordinates_are_measured_without_a_fixed_pattern() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) =
        pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e| {
        panic!("sample-coordinate probe: {e}")
    }));
    let image = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("measured sample positions"),
        size: wgpu::Extent3d {
            width: 8,
            height: 8,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 4,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = image.create_view(&Default::default());
    let module=device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label:Some("sample interpolation versus builtin position"),source:wgpu::ShaderSource::Wgsl(r#"
struct Vertex { @builtin(position) position:vec4f, @location(0) @interpolate(linear,sample) pixel:vec2f }
@vertex fn vertex(@builtin(vertex_index) index:u32)->Vertex {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3))[index];
 return Vertex(vec4f(p,0.5,1), (p*vec2f(0.5,-0.5)+0.5)*8.);
}
@fragment fn fragment(input:Vertex)->@location(0) vec4f {return vec4f(input.position.xy,input.pixel);}
"#.into()),
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("actual sample-coordinate output"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vertex"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some("fragment"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 4,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    });
    let storage = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let result = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("every actual sample coordinate"),
        source: wgpu::ShaderSource::Wgsl(
            r#"
@group(0) @binding(0) var pixels:texture_multisampled_2d<f32>;
@group(0) @binding(1) var<storage,read_write> values:array<vec4f>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) id:vec3u) {
 values[id.x]=textureLoad(pixels,vec2i(2),i32(id.x));
}
"#
            .into(),
        ),
    });
    let compute = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &module,
        entry_point: Some("read_samples"),
        compilation_options: Default::default(),
        cache: None,
    });
    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &compute.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: storage.as_entire_binding(),
            },
        ],
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("actual sample positions"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.draw(0..3, 0..1);
    }
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&compute);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
    queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    result
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |r| {
            tx.send(r).unwrap()
        });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let bytes = result.slice(..).get_mapped_range();
    let values = bytes
        .chunks_exact(16)
        .map(|b| {
            std::array::from_fn::<_, 4, _>(|i| {
                f32::from_ne_bytes(b[i * 4..i * 4 + 4].try_into().unwrap())
            })
        })
        .collect::<Vec<_>>();
    drop(bytes);
    result.unmap();
    assert_eq!(values.len(), 4);
    assert!(
        values
            .iter()
            .flatten()
            .all(|v| v.is_finite() && *v >= 2. && *v < 3.)
    );
    for i in 0..4 {
        for j in 0..i {
            assert_ne!(
                &values[i][2..],
                &values[j][2..],
                "Actual sample-qualified coordinates must distinguish the four sample invocations"
            );
        }
    }
    let report = serde_json::json!({"status":"measured actual Native sample coordinates; no viewer4x coverage claim","values":values,"builtinMatchesSample":values.iter().all(|p|(p[0]-p[2]).abs()<0.002&&(p[1]-p[3]).abs()<0.002),"builtinIsCentre":values.iter().all(|p|p[0]==2.5&&p[1]==2.5),"adapter":format!("{:?}",adapter.get_info())});
    crate::sample_test_proof::write(
        "native-sample-coordinates.json",
        serde_json::to_string_pretty(&report).unwrap(),
    )
    .unwrap();
}
```

### 8. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod revision;
pub mod sample_quality;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod revision;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "sample_positions_tests.rs"]
mod sample_positions_tests;
pub mod sample_quality;
```

### 9. `src/browser.rs`

**Locate this existing block:**

```rust
                        sample_forced = None;
                    } else if let Some(count) = value
                        .as_f64()
                        .filter(|v| *v == 1. || *v == 4.)
                    {
                        sample_forced = Some(count as u32);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        sample_forced = None;
                    } else if let Some(count) = value.as_f64().filter(|v| {
                        *v >= 0. && *v <= u32::MAX as f64 && v.fract() == 0.
                    }) {
                        sample_forced = Some(count as u32);
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 22.5
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 22.5
npm --prefix ../session_tests run course -- save 22.5
```

Continue with [22.6 · Original display controls and remaining integration boundaries](96zzzzzzzm-retain-visible-instance-ranges.md).
