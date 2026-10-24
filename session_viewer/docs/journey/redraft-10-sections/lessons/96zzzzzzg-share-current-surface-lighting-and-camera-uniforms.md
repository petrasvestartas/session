# 22.1 · Original surface colours and lighting controls

**Goal:** Expose current headlight and backface presentation through the existing surface shader, editor commands and browser settings. · **Steps:** 2 · **Source:** [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs) · [engine/gpu/pass.rs](../../../../src/engine/gpu/pass.rs) · [engine/gpu/targets.rs](../../../../src/engine/gpu/targets.rs) · [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs)

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

## <a id="checkpoint-96zzzzzzg-share-current-surface-lighting-and-camera-uniforms"></a>1. Share flat and Arctic surface frames

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

Keep original fill colors and Arctic sky shading in one surface shader, with stable ambient cameras during wide zoom.

### 1. `src/surface_view.rs`

Encode the existing anchored frame once, on the stack, with its current eye and projection kind.

Create the file and type:

```rust
pub const BYTES: u64 = 96;
pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    crate::gpu_layout::uniform(device, BYTES)
}
pub fn bytes(matrix: [f32; 16], display: [f32; 4]) -> [u8; 96] {
    let frame = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let eye = frame.eye();
    let mut values = [0.; 24];
    values[..16].copy_from_slice(&matrix);
    values[16..20].copy_from_slice(&display);
    values[20..].copy_from_slice(&[
        eye[0] as f32,
        eye[1] as f32,
        eye[2] as f32,
        frame.ortho_half_height() as f32,
    ]);
    let mut bytes = [0; 96];
    for (out, value) in bytes.chunks_exact_mut(4).zip(values) {
        out.copy_from_slice(&value.to_ne_bytes());
    }
    bytes
}
```

### 2. `src/lib.rs`

Keep one internal owner for the shared surface-frame layout.

**Locate this existing block:**

```rust
mod shader_check;

mod draw_slots;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shader_check;
mod surface_view;

mod draw_slots;
```

### 3. `src/renderer.rs`

Share the frame layout with derived surface pipelines. Allocate the exact shared frame size. Reuse the same frame encoder while preparing original picks. Choose current flat or Arctic sky mode in the shared frame; keep normal inspection and opacity.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            },
        );
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let display = [
            if normals {
                1.0f32
            } else {
                0.0
            },
            self.opacity.get(),
            0.0,
            0.0,
        ];
        bytes.extend(
            display
                .iter()
                .flat_map(|value| value.to_ne_bytes()),
        );
        self.queue
```

Replace it with:

```rust
            },
        );
        self.prepare_clouds(*transform);
        let display = [
            f32::from(normals),
            self.opacity.get(),
            if self.ambient_enabled.get() {
                2.
            } else {
                0.
            },
            0.,
        ];
        let bytes = crate::surface_view::bytes(*transform, display);
        self.queue
```

**Locate this block:**

```rust
                self.size[1],
                self.density,
            );
            let mut bytes = transform
                .iter()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>();
            bytes.resize(80, 0);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
```

Replace it with:

```rust
                self.size[1],
                self.density,
            );
            let bytes = crate::surface_view::bytes(transform, [0.; 4]);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
```

**Locate this block:**

```rust
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
```

Replace it with:

```rust
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: crate::surface_view::BYTES,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
```

**Locate this block:**

```rust
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout =
            crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

Replace it with:

```rust
        });
        let view_layout = crate::surface_view::layout(&device);
        let model_layout =
            crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
```

### 4. `src/face_pick.rs`

Keep every consumer of the shared frame on the same binding layout.

**Locate this existing block:**

```rust
            let storage = layout(device);
            let view = crate::gpu_layout::uniform(device, 80);
            let model = if shared {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            let storage = layout(device);
            let view = crate::surface_view::layout(device);
            let model = if shared {
```

### 5. `src/ambient_source_pipeline.rs`

Keep every consumer of the shared frame on the same binding layout.

**Locate this existing block:**

```rust
        let create = |shared| {
            let view = crate::gpu_layout::uniform(device, 80);
            let model = if shared {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let create = |shared| {
            let view = crate::surface_view::layout(device);
            let model = if shared {
```

### 6. `src/cap_pipeline.rs`

Keep every consumer of the shared frame on the same binding layout.

**Locate this existing block:**

```rust
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let view = crate::gpu_layout::uniform(device, 80);
        let model =
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let view = crate::surface_view::layout(device);
        let model =
```

### 7. `src/cap_pick_pipeline.rs`

Keep every consumer of the shared frame on the same binding layout.

**Locate this existing block:**

```rust
        let clip = crate::clipping_gpu::layout(device);
        let view = crate::gpu_layout::uniform(device, 80);
        let model =
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let clip = crate::clipping_gpu::layout(device);
        let view = crate::surface_view::layout(device);
        let model =
```

### 8. `src/outline_gpu.rs`

Keep every consumer of the shared frame on the same binding layout.

**Locate this existing block:**

```rust
    });
    let view = crate::gpu_layout::uniform(device, 80);
    let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    });
    let view = crate::surface_view::layout(device);
    let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
```

### 9. `src/pick_gpu.rs`

Grow only the shared pick view; its crop frame keeps its own size. Share exact frame bytes with the color renderer.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        let image = self.image.as_mut().unwrap();
        image.window = window;
        let mut bytes: Vec<u8> = request
            .transform
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.resize(80, 0);
        queue.write_buffer(&self.view, 0, &bytes);
```

Replace it with:

```rust
        }
        let image = self.image.as_mut().unwrap();
        image.window = window;
        let bytes = crate::surface_view::bytes(request.transform, [0.; 4]);
        queue.write_buffer(&self.view, 0, &bytes);
```

**Locate this block:**

```rust
        };
        let view = buffer(80);
        let crop = buffer(80);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
```

Replace it with:

```rust
        };
        let view = buffer(crate::surface_view::BYTES);
        let crop = buffer(80);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
```

### 10. `src/triangle.wgsl`

Read the same eye/projection block in both surface-record variants. Use current flat source colors and eye-facing Arctic sky shading in the existing shared shader.

**Locate this existing block:**

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index: u32) -> ObjectSettings { return object_settings; }

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
    @location(5) @interpolate(flat) primitive:u32,
}

const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let contact=abs(colour.a-1./255.)<0.5/255.;
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, SELECTION_COLOR, (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;
    output.clip_flags=select(object.selection.z,(object.selection.z&~(8192u|32768u))|CONTACT_FLAG,contact);
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let contact=(input.clip_flags&CONTACT_FLAG)!=0u;
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    if(contact){return vec4<f32>(input.colour,1.);}
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
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, eye:vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index: u32) -> ObjectSettings { return object_settings; }

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
    @location(5) @interpolate(flat) primitive:u32,
}

const CONTACT_FLAG:u32=65536u;
fn geometry(position: vec3<f32>, colour: vec4<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let contact=abs(colour.a-1./255.)<0.5/255.;
    let displayed=select(colour.rgb,object.face_colour.xyz,object.face_colour.w>0.5&&!contact);
    output.colour = select(displayed, SELECTION_COLOR, (object.selection.x & 1u) != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;
    output.clip_flags=select(object.selection.z,(object.selection.z&~(8192u|32768u))|CONTACT_FLAG,contact);
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

fn shade(input: VertexOutput, front: bool) -> vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let contact=(input.clip_flags&CONTACT_FLAG)!=0u;
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    if(contact){return vec4<f32>(input.colour,1.);}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let m=view.transform;
    let v=select(normalize(view.eye.xyz-input.world),normalize(vec3<f32>(m[0].z,m[1].z,m[2].z)),view.eye.w>0.);
    let n=select(normal,-normal,dot(normal,v)<0.);
    let brightness=select(1.,mix(0.72,1.,0.5+0.5*n.z),view.display.z>1.5);
    return vec4<f32>(input.colour*brightness,select(1.,view.display.y,closed));
}
```

### 11. `src/clipping_uniform.rs`

Share the existing scale-stable inverse and reject invalid inputs.

**Locate this existing block:**

```rust
}
fn inverse(m: &Xform) -> Option<Xform> {
    let scales: [f64; 4] = std::array::from_fn(|row| {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub(crate) fn inverse(m: &Xform) -> Option<Xform> {
    if m.m.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let scales: [f64; 4] = std::array::from_fn(|row| {
```

### 12. `src/ambient_camera.rs`

Keep valid wide orthographic cameras in the real ambient pass.

**Locate this existing block:**

```rust
    let mvp = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let inverse = mvp.inverse()?.m.map(|v| v as f32);
    if inverse.iter().any(|v| !v.is_finite()) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mvp = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let inverse = crate::clipping_uniform::inverse(&mvp)?
        .m
        .map(|v| v as f32);
    if inverse.iter().any(|v| !v.is_finite()) {
```

### 13. `src/pyramid_gpu.rs`

Reuse the same current camera inversion in the optional depth-pyramid path.

**Locate this existing block:**

```rust
        }
        let Some(inverse) =
            session_rust::Xform::from_matrix(matrix.map(f64::from)).inverse()
        else {
            self.error = Some("Ambient depth camera is singular");
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
        let Some(inverse) = crate::clipping_uniform::inverse(
            &session_rust::Xform::from_matrix(matrix.map(f64::from)),
        ) else {
            self.error = Some("Ambient depth camera is singular");
```

### 14. `src/surface_lighting_tests.rs`

Supply real mode pixels, uniform/fallback restoration, current camera bytes and wide ambient/pyramid controls.

Create the file and type:

```rust
use crate::{
    camera::{Camera, Projection, Unit},
    cap_gpu_tests::{at, pixels_at, renderer},
    scene::Scene,
};
use session_rust::{Point, Vector, Xform};

fn original() -> session_rust::Mesh {
    let mut mesh = session_rust::Mesh::from_vertices_and_faces(
        vec![
            Point::new(0., -800., -800.),
            Point::new(0., 800., -800.),
            Point::new(0., 800., 800.),
            Point::new(0., -800., 800.),
        ],
        vec![vec![0, 1, 2, 3]],
    );
    mesh.set_objectcolor(session_rust::Color::new(0.2, 0.4, 0.6, 1.));
    mesh
}
fn camera() -> Camera {
    let mut c = Camera::default();
    c.projection = Projection::Orthographic;
    c.aspect = 4. / 3.;
    c
}

#[test]
fn surface_frame_bytes_preserve_camera_and_display_domains() {
    assert_eq!(crate::surface_view::BYTES, 96);
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            let mut c = camera();
            c.unit = unit;
            c.distance = 3. / unit.to_meters();
            c.projection = projection;
            c.target = [1e9, -1e9, 1e9];
            let matrix = c.view_projection_anchored(c.target).to_f32();
            let bytes = crate::surface_view::bytes(matrix, [1., 0.7, 2., 1.]);
            let values = bytes
                .chunks_exact(4)
                .map(|b| f32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            assert_eq!(&values[..16], &matrix);
            assert_eq!(&values[16..20], &[1., 0.7, 2., 1.]);
            assert!(values.iter().all(|v| v.is_finite()));
            if projection == Projection::Perspective {
                assert_eq!(values[23], 0.);
                let forward = c.orientation.rotate_vector(Vector::y_axis());
                for i in 0..3 {
                    let expected = -forward[i] * c.distance;
                    assert!(
                        (values[20 + i] as f64 - expected).abs() < c.distance * 2e-6
                    );
                }
            } else {
                assert!(values[23] > 0.);
            }
        }
    }
}

#[test]
fn scaled_camera_inverse_keeps_wide_views_and_invalid_guards() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            let mut c = camera();
            c.unit = unit;
            c.distance = 24. / unit.to_meters();
            c.projection = projection;
            let matrix = c.view_projection().to_f32();
            let transform = Xform::from_matrix(matrix.map(f64::from));
            let inverse = crate::clipping_uniform::inverse(&transform).unwrap();
            let identity = &transform * &inverse;
            for i in 0..16 {
                assert!(
                    (identity.m[i]
                        - if i % 5 == 0 {
                            1.
                        } else {
                            0.
                        })
                    .abs()
                        < 1e-7
                );
            }
            assert!(
                crate::ambient_camera::uniform(matrix, [320, 240], [128, 96], [0., 1.])
                    .is_some()
            );
            if unit == Unit::Millimeters && projection == Projection::Orthographic {
                assert!(
                    transform.inverse().is_none(),
                    "The real unscaled-kernel regression control must fail"
                );
            }
        }
    }
    for value in [0., f64::NAN, f64::INFINITY] {
        assert!(
            crate::clipping_uniform::inverse(&Xform::from_matrix([value; 16]))
                .is_none()
        );
    }
}

#[test]
fn actual_unlit_and_sky_preserve_original_records() {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(crate::prepared::PreparedMesh::new(original()).unwrap())
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let matrix = camera().uniform();
    let mut r = renderer(&scene);
    let uploads = r.stats()[0];
    let flat = pixels_at(&r, matrix);
    assert_eq!(
        at(&flat, 160, 120),
        &[51, 102, 153, 255],
        "Unlit fill retains exact original color"
    );
    // Isolate the actual sky fragment mode from the separate ambient compositor.
    r.ambient_enabled.set(true);
    assert!(r.ambient.borrow().is_none());
    let sky = pixels_at(&r, matrix);
    assert_eq!(
        at(&sky, 160, 120),
        &[44, 88, 132, 255],
        "Current vertical-face sky factor is0.86"
    );
    r.ambient_enabled.set(false);
    for uniform in [true, false] {
        if uniform {
            r.use_uniform_surface_records();
        } else {
            r.set_scene(&scene, None);
        }
        assert_eq!(pixels_at(&r, matrix), flat);
        r.ambient_enabled.set(true);
        assert_eq!(pixels_at(&r, matrix), sky);
        r.ambient_enabled.set(false);
    }
    assert_eq!(pixels_at(&r, matrix), flat);
    assert_eq!(r.stats()[0], uploads);
    assert_eq!(scene.objects()[0].id, owner);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
}

#[test]
fn wide_ambient_and_depth_pyramid_keep_real_passes_without_geometry_uploads() {
    let mut scene = Scene::empty();
    let source = session_rust::Mesh::create_box(6000., 4000., 1000.);
    scene
        .insert(crate::prepared::PreparedMesh::new(source).unwrap())
        .unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut c = camera();
    c.set_view(crate::camera::View::Top);
    c.distance = 24000.;
    let r = renderer(&scene);
    r.grow_camera(&mut c);
    r.set_ambient(true);
    r.set_depth_pyramid(true);
    let uploads = r.stats()[0];
    let first = pixels_at(&r, c.uniform());
    assert_eq!(r.ambient_error(), None);
    assert_eq!(r.depth_pyramid_error(), None);
    assert!(r.ambient_usage()[4] > 0);
    assert_eq!(r.depth_pyramid_usage()[3], 6);
    let ambient = r.ambient_usage();
    let pyramid = r.depth_pyramid_usage();
    assert_eq!(pixels_at(&r, c.uniform()), first);
    assert_eq!(r.ambient_usage()[3], ambient[3]);
    assert_eq!(r.depth_pyramid_usage(), pyramid);
    c.zoom(2.);
    let _ = pixels_at(&r, c.uniform());
    assert_eq!(r.ambient_error(), None);
    assert_eq!(r.depth_pyramid_error(), None);
    assert_eq!(r.stats()[0], uploads);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
}
```

### 15. `src/renderer.rs`

Register optional GPU validation outside the required browser graph.

**Locate this existing block:**

```rust
#[path = "camera_extent_tests.rs"]
mod camera_extent_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "camera_extent_tests.rs"]
mod camera_extent_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "surface_lighting_tests.rs"]
mod surface_lighting_tests;
```

## <a id="checkpoint-96zzzzzzh-connect-headlight-and-backface-controls"></a>2. Connect headlight and backface controls

### 1. `src/renderer.rs`

Store only the two optional presentation settings. Retain current flat defaults. Arctic takes precedence while both settings remain available for restoration. Update presentation without rebuilding source records.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            },
        );
        self.prepare_clouds(*transform);
        let display = [
            f32::from(normals),
            self.opacity.get(),
            if self.ambient_enabled.get() {
                2.
            } else {
                0.
            },
            0.,
        ];
```

Replace it with:

```rust
            },
        );
        self.prepare_clouds(*transform);
        let options = self.surface_options.get();
        let display = [
            f32::from(normals),
            self.opacity.get(),
            if self.ambient_enabled.get() {
                2.
            } else {
                f32::from(options[0])
            },
            f32::from(options[1]),
        ];
```

**Locate this block:**

```rust
        clip.solid = solid;
        self.caps.borrow_mut().enabled = enabled;
    }
    pub fn set_ambient(&self, enabled: bool) {
        if self.ambient_enabled.replace(enabled) == enabled {
            return;
```

Replace it with:

```rust
        clip.solid = solid;
        self.caps.borrow_mut().enabled = enabled;
    }
    pub fn set_lighting(&self, on: bool, backfaces: bool) {
        self.surface_options.set([on, backfaces]);
    }
    pub fn set_ambient(&self, enabled: bool) {
        if self.ambient_enabled.replace(enabled) == enabled {
            return;
```

**Locate this block:**

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            world_bounds: Default::default(),
            space: Default::default(),
            opacity: std::cell::Cell::new(1.),
```

Replace it with:

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            surface_options: Default::default(),
            world_bounds: Default::default(),
            space: Default::default(),
            opacity: std::cell::Cell::new(1.),
```

**Locate this block:**

```rust
pub struct Renderer {
    world_bounds: std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space: crate::render_space::Origin,
    opacity: std::cell::Cell<f32>,
```

Replace it with:

```rust
pub struct Renderer {
    surface_options: std::cell::Cell<[bool; 2]>,
    world_bounds: std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space: crate::render_space::Origin,
    opacity: std::cell::Cell<f32>,
```

### 2. `src/triangle.wgsl`

Reuse the current wrapped headlight, soft highlight, sky and backface colors in one shader.

**Locate this existing block:**

```wgsl
    let n=select(normal,-normal,dot(normal,v)<0.);
    let brightness=select(1.,mix(0.72,1.,0.5+0.5*n.z),view.display.z>1.5);
    return vec4<f32>(input.colour*brightness,select(1.,view.display.y,closed));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let n=select(normal,-normal,dot(normal,v)<0.);
    let l=normalize(v+vec3<f32>(0.,0.,0.35));let h=normalize(l+v);
    let wrap=clamp((dot(n,l)+0.5)/1.5,0.,1.);
    let lit=min(0.40+0.55*wrap+pow(max(dot(n,h),0.),32.)*0.15,1.);
    let sky=mix(0.72,1.,0.5+0.5*n.z);
    let brightness=select(1.,select(lit,sky,view.display.z>1.5),view.display.z>0.5);
    let back=!source_front&&view.display.w>0.5;
    let base=select(input.colour,vec3<f32>(0.80,0.05,0.05),back);
    return vec4<f32>(base*brightness,select(1.,view.display.y,closed));
}
```

### 3. `src/editor.rs`

Keep presentation state beside Arctic. Keep the native flat defaults. Use the existing action path. Return before tool cancellation or history changes. Keep the existing action match exhaustive.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            Action::ViewOutline(_) => {
                unreachable!("Outline presentation returned before editor mutation")
            }
            Action::Arctic(_) => {
                unreachable!("Arctic presentation returned before editor mutation")
            }
```

Replace it with:

```rust
            Action::ViewOutline(_) => {
                unreachable!("Outline presentation returned before editor mutation")
            }
            Action::Arctic(_) | Action::Lighting(_) | Action::Backfaces(_) => {
                unreachable!("Presentation returned before editor mutation")
            }
```

**Locate this block:**

```rust
            );
            return Ok(Change::View);
        }
        if let Action::Arctic(on) = &action {
            self.arctic = on.unwrap_or(!self.arctic);
            if self.arctic {
```

Replace it with:

```rust
            );
            return Ok(Change::View);
        }
        let setting = match &action {
            Action::Lighting(on) => Some((&mut self.lit, *on, "Lighting")),
            Action::Backfaces(on) => Some((&mut self.backfaces, *on, "Backfaces")),
            _ => None,
        };
        if let Some((value, on, label)) = setting {
            *value = on.unwrap_or(!*value);
            self.tool_status = format!(
                "{label} {}",
                if *value {
                    "On"
                } else {
                    "Off"
                }
            );
            return Ok(Change::View);
        }
        if let Action::Arctic(on) = &action {
            self.arctic = on.unwrap_or(!self.arctic);
            if self.arctic {
```

**Locate this block:**

```rust
            element_interactions: true,
            opacity: 1.,
            arctic: false,
            outlines: false,
            clipping_enabled: true,
            clipping_solid: true,
```

Replace it with:

```rust
            element_interactions: true,
            opacity: 1.,
            arctic: false,
            lit: false,
            backfaces: false,
            outlines: false,
            clipping_enabled: true,
            clipping_solid: true,
```

**Locate this block:**

```rust
    pub element_interactions: bool,
    pub opacity: f32,
    pub arctic: bool,
    pub outlines: bool,
    pub clipping_enabled: bool,
    pub clipping_solid: bool,
```

Replace it with:

```rust
    pub element_interactions: bool,
    pub opacity: f32,
    pub arctic: bool,
    pub lit: bool,
    pub backfaces: bool,
    pub outlines: bool,
    pub clipping_enabled: bool,
    pub clipping_solid: bool,
```

**Locate this block:**

```rust
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    EnableControls,
```

Replace it with:

```rust
    NormalView(bool),
    Lighting(Option<bool>),
    Backfaces(Option<bool>),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    EnableControls,
```

### 4. `src/commands.rs`

Share registry entries with command completion. Reuse strict optional On/Off parsing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        };
        return Ok(Command::Editor(Action::ViewOutline(on)));
    }
```

Replace it with:

```rust
        };
        return Ok(Command::Editor(Action::ViewOutline(on)));
    }
    if name == "View Lighting" || name == "View Backfaces" {
        let on = crate::element_input::visibility(&words[count..])?;
        return Ok(Command::Editor(if name == "View Lighting" {
            Action::Lighting(on)
        } else {
            Action::Backfaces(on)
        }));
    }
```

**Locate this block:**

```rust
    ("Arctic", || Action::Arctic(None)),
    ("Select By Name", || {
        Action::Query(crate::selection_query::Query::Name(
            String::new(),
```

Replace it with:

```rust
    ("Arctic", || Action::Arctic(None)),
    ("View Lighting", || Action::Lighting(None)),
    ("View Backfaces", || Action::Backfaces(None)),
    ("Select By Name", || {
        Action::Query(crate::selection_query::Query::Name(
            String::new(),
```

### 5. `src/browser_defaults.rs`

Match current presence-based startup queries.

**Locate this existing block:**

```rust
    let query = query();
    let opacity = query.as_ref().and_then(|q| q.get("opacity"));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let query = query();
    editor.lit = query.as_ref().is_some_and(|q| q.has("lit"));
    editor.backfaces = query
        .as_ref()
        .is_some_and(|q| q.has("backface"));
    let opacity = query.as_ref().and_then(|q| q.get("opacity"));
```

### 6. `src/browser.rs`

Forward presentation through the same renderer. Expose the live view state for actual browser checks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute(
```

Replace it with:

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"lighting":editor.lit,"backfaces":editor.backfaces,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute(
```

**Locate this block:**

```rust
            renderer.set_opacity(editor.opacity);
            renderer.set_ambient(editor.arctic);
            renderer.set_outlines(editor.outlines);
            if let Err(error) =
```

Replace it with:

```rust
            renderer.set_opacity(editor.opacity);
            renderer.set_lighting(editor.lit, editor.backfaces);
            renderer.set_ambient(editor.arctic);
            renderer.set_outlines(editor.outlines);
            if let Err(error) =
```

### 7. `src/surface_controls_tests.rs`

Supply command, draft/history and actual shared/fallback face-pixel controls.

Create the file and type:

```rust
use crate::{
    commands::{Command, parse},
    editor::{Action, Change, Editor},
};
fn command(e: &mut Editor, line: &str) -> Result<Change, &'static str> {
    let action = if let Some(result) = e.tool_line(line) {
        result?
    } else {
        let Command::Editor(action) = parse(line).map_err(|_| "Invalid command")?
        else {
            return Err("Editor command");
        };
        action
    };
    e.apply(action)
}
#[test]
fn commands_preserve_active_draft_originals_annotations_and_redo() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddBox).unwrap();
    let id = e.scene.objects()[0].id;
    e.select_objects([id], false);
    let source = e.scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    e.apply(Action::Translate([1., 0., 0.]))
        .unwrap();
    let moved = e.scene.placement(id).unwrap();
    e.apply(Action::Undo).unwrap();
    command(&mut e, "Length BoundingBox").unwrap();
    let mark = e.mark.clone();
    assert!(mark.is_some());
    let epoch = e.interaction_epoch;
    for line in [
        "View Lighting On",
        "View Backfaces On",
        "View Lighting Off",
        "View Backfaces Off",
    ] {
        assert!(matches!(
            command(&mut e, line),
            Ok(Change::View)
        ));
        assert_eq!(e.mark, mark);
        assert_eq!(e.interaction_epoch, epoch);
        assert_eq!(
            e.scene.objects()[0]
                .geometry()
                .unwrap()
                .to_proto(),
            source
        );
    }
    command(&mut e, "Line").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    let points = e.draft.as_ref().unwrap().points.clone();
    let epoch = e.interaction_epoch;
    for line in [
        "vIeW lIgHtInG oN",
        "View Backfaces On",
        "View Lighting",
        "View Backfaces",
    ] {
        command(&mut e, line).unwrap();
        assert_eq!(e.draft.as_ref().unwrap().points, points);
        assert_eq!(e.interaction_epoch, epoch);
    }
    assert!(!e.lit && !e.backfaces);
    command(&mut e, "Escape").unwrap();
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, moved.m);
    assert_eq!(
        e.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
}
#[test]
fn strict_view_options_and_completion_share_existing_registry() {
    let mut e = Editor::default();
    for prefix in ["View Lighting", "View Backfaces"] {
        assert!(
            crate::commands::ACTIONS
                .iter()
                .any(|(name, _)| *name == prefix)
        );
        command(&mut e, &format!("{prefix} On")).unwrap();
        let before = [e.lit, e.backfaces];
        for suffix in ["No", "On Off", "On extra"] {
            assert!(command(&mut e, &format!("{prefix} {suffix}")).is_err());
            assert_eq!([e.lit, e.backfaces], before);
        }
        command(&mut e, &format!("{prefix} Off")).unwrap();
    }
    assert!(!e.lit && !e.backfaces);
}
```

### 8. `src/lib.rs`

Register optional command controls.

**Locate this existing block:**

```rust
mod arctic_tests;

pub mod outline_caps;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod arctic_tests;
#[cfg(test)]
mod surface_controls_tests;

pub mod outline_caps;
```

### 9. `src/renderer.rs`

Register actual native GPU controls outside the browser graph.

**Locate this existing block:**

```rust
#[path = "surface_lighting_tests.rs"]
mod surface_lighting_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "surface_lighting_tests.rs"]
mod surface_lighting_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "surface_controls_gpu_tests.rs"]
mod surface_controls_tests;
```

### 10. `src/surface_controls_gpu_tests.rs`

Supply actual headlight/backface, mirrored orientation, projection and exact restoration controls.

Create the file and type:

```rust
use crate::{
    camera::{Camera, Projection},
    cap_gpu_tests::{at, pixels_at, renderer},
    scene::Scene,
};
fn original() -> session_rust::Mesh {
    let mut mesh = session_rust::Mesh::from_vertices_and_faces(
        vec![
            session_rust::Point::new(0., -800., -800.),
            session_rust::Point::new(0., 800., -800.),
            session_rust::Point::new(0., 800., 800.),
            session_rust::Point::new(0., -800., 800.),
        ],
        vec![vec![0, 1, 2, 3]],
    );
    mesh.set_objectcolor(session_rust::Color::new(0.2, 0.4, 0.6, 1.));
    mesh
}
#[test]
fn actual_headlight_backfaces_and_original_record_restoration() {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(crate::prepared::PreparedMesh::new(original()).unwrap())
        .unwrap();
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut c = Camera::default();
    c.aspect = 4. / 3.;
    let mut r = renderer(&scene);
    for mirrored in [false, true] {
        scene
            .place_owner(
                owner,
                if mirrored {
                    session_rust::Xform::scale_xyz(1., -1., 1.)
                } else {
                    session_rust::Xform::identity()
                },
            )
            .unwrap();
        r.set_scene(&scene, None);
        let uploads = r.stats()[0];
        for projection in [
            Projection::Orthographic,
            Projection::Perspective,
        ] {
            c.projection = projection;
            let matrix = c.uniform();
            r.set_lighting(false, false);
            let flat = pixels_at(&r, matrix);
            assert_eq!(at(&flat, 160, 120), &[51, 102, 153, 255]);
            r.set_lighting(true, false);
            let head = pixels_at(&r, matrix);
            assert_ne!(head, flat);
            let color = at(&head, 160, 120);
            assert!(
                color[0] > 10 && color[0] < 51 && color[1] < 102 && color[2] < 153,
                "{projection:?} headlight {color:?}"
            );
            r.set_lighting(false, true);
            let back = pixels_at(&r, matrix);
            assert_eq!(
                at(&back, 160, 120),
                &[204, 13, 13, 255],
                "Original backface survives mirrored placement"
            );
            for uniform in [true, false] {
                if uniform {
                    r.use_uniform_surface_records();
                } else {
                    r.set_scene(&scene, None);
                }
                r.set_lighting(true, false);
                assert_eq!(pixels_at(&r, matrix), head);
                r.set_lighting(false, true);
                assert_eq!(pixels_at(&r, matrix), back);
                r.set_lighting(false, false);
                assert_eq!(pixels_at(&r, matrix), flat);
            }
            assert_eq!(r.stats()[0], uploads);
            assert_eq!(
                scene.objects()[0]
                    .geometry()
                    .unwrap()
                    .to_proto(),
                source
            );
        }
    }
    let mut front = original();
    for face in front.face.values_mut() {
        face.reverse();
    }
    let mut scene = Scene::empty();
    scene
        .insert(crate::prepared::PreparedMesh::new(front).unwrap())
        .unwrap();
    r.set_scene(&scene, None);
    r.set_lighting(false, true);
    assert_eq!(
        at(&pixels_at(&r, c.uniform()), 160, 120),
        &[51, 102, 153, 255],
        "Front-facing original retains its source color"
    );
}
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 22.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 22.1
npm --prefix ../session_tests run course -- save 22.1
```

Continue with [22.2 · Contact fills and original face queries](../../consolidation/96zzzzzzi-share-one-frame-across-color-and-ink.md).
