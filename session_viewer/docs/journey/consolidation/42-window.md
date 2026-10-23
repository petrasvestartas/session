# 6.1 · A bounded integer picking pass

**Goal:** Connect real canvas input to bounded integer GPU picking for meshes and ink. · **Steps:** 12 · **Source:** [engine/gpu/pick.rs](../../../src/engine/gpu/pick.rs) · [app/selection.rs](../../../src/app/selection.rs) · [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs) · [app/walk/mesh_topology.rs](../../../src/app/walk/mesh_topology.rs) (`src/engine/gpu/pick.rs,src/app/selecting.rs,src/shaders/scene.wgsl`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Cursor Pick Crop Window (r_px Radius)</text>
  <rect x="68" y="46" width="236" height="114" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="104" y1="138" x2="268" y2="66" stroke="#2563eb" stroke-width="3"/>
  <circle cx="186" cy="102" r="5" fill="#dc2626"/>
  <rect x="158" y="74" width="56" height="56" fill="none" stroke="#111111" stroke-width="2" stroke-dasharray="3 2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="66" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">15×15 Crop</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Rg32Uint Attachment: vec2&lt;u32&gt;(owner, part)</text>
  <rect x="466" y="46" width="188" height="112" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="528" y1="46" x2="528" y2="158" stroke="#111111" stroke-width="1"/>
  <line x1="592" y1="46" x2="592" y2="158" stroke="#111111" stroke-width="1"/>
  <line x1="466" y1="83" x2="654" y2="83" stroke="#111111" stroke-width="1"/>
  <line x1="466" y1="121" x2="654" y2="121" stroke="#111111" stroke-width="1"/>
  <rect x="528" y="83" width="64" height="38" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[7, 2]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="497" y="68" text-anchor="middle" font-family="monospace" font-size="9" fill="#64748b">[0,0]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="623" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">[7,1]</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Priority: Point &gt; Edge &gt; Face Hit</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="866,138 934,62 1002,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="866" y1="138" x2="934" y2="62" stroke="#f59e0b" stroke-width="4"/>
  <circle cx="934" cy="62" r="7" fill="#fef08a" stroke="#dc2626" stroke-width="2.5"/>
</svg>
</div>

## <a id="checkpoint-42-window"></a>1. Bound the cursor image and decode a hit

**What this code does:** Bound picking work around the cursor independently of the canvas size.

### 1. `src/pick_window.rs`

Bound the cursor window and decode exact integer owners.

Create the file and type:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub copy: [u32; 4],
    pub draw: [u32; 4],
    pub cursor: [u32; 2],
    pub radius: u32,
    pub full: [u32; 2],
}

impl Window {
    pub fn new(screen: [f32; 2], full: [u32; 2], density: f32) -> Option<Self> {
        if full.contains(&0)
            || !density.is_finite()
            || density <= 0.0
            || screen
                .iter()
                .any(|v| !v.is_finite() || v.abs() > 1.0)
        {
            return None;
        }
        let at = [
            (f64::from(screen[0]) + 1.0) * f64::from(full[0]) * 0.5,
            (1.0 - f64::from(screen[1])) * f64::from(full[1]) * 0.5,
        ];
        let at: [u32; 2] =
            std::array::from_fn(|i| (at[i].floor() as u32).min(full[i] - 1));
        let radius = (6.0 * f64::from(density)).ceil().min(128.0) as u32;
        let [w, h] = full.map(|n| n.min(2 * radius + 1));
        let x = at[0].saturating_sub(radius).min(full[0] - w);
        let y = at[1].saturating_sub(radius).min(full[1] - h);
        let left = x.saturating_sub(3);
        let top = y.saturating_sub(3);
        let right = (u64::from(x) + u64::from(w) + 3).min(u64::from(full[0])) as u32;
        let bottom = (u64::from(y) + u64::from(h) + 3).min(u64::from(full[1])) as u32;
        Some(Self {
            copy: [x, y, w, h],
            draw: [left, top, right - left, bottom - top],
            cursor: [at[0] - x, at[1] - y],
            radius,
            full,
        })
    }

    // Cropping changes clip x/y only; homogeneous depth and the perspective divide retain the main view's meaning.
    pub fn crop(self) -> [f32; 16] {
        let [x, y, w, h] = self.draw.map(f64::from);
        let [fw, fh] = self.full.map(f64::from);
        [
            fw / w,
            0.0,
            0.0,
            0.0,
            0.0,
            fh / h,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            (fw - 2.0 * x - w) / w,
            (2.0 * y + h - fh) / h,
            0.0,
            1.0,
        ]
        .map(|n| n as f32)
    }

    pub fn row_bytes(self) -> u32 {
        (self.copy[2] * 8).div_ceil(256) * 256
    }

    pub fn bytes(self) -> u64 {
        u64::from(self.row_bytes()) * u64::from(self.copy[3])
    }

    pub fn closest(self, bytes: &[u8]) -> Result<Option<[u32; 2]>, &'static str> {
        if (bytes.len() as u64) < self.bytes() {
            return Err("Incomplete pick image");
        }
        let mut best = None;
        for y in 0..self.copy[3] {
            for x in 0..self.copy[2] {
                let offset = (y * self.row_bytes() + x * 8) as usize;
                let row =
                    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
                let sub = u32::from_le_bytes(
                    bytes[offset + 4..offset + 8]
                        .try_into()
                        .unwrap(),
                );
                let dx = i64::from(x) - i64::from(self.cursor[0]);
                let dy = i64::from(y) - i64::from(self.cursor[1]);
                let distance = (dx * dx + dy * dy) as u64;
                if row == 0 || distance > u64::from(self.radius).pow(2) {
                    continue;
                }
                let key = (sub == 0, distance, row, sub);
                if best.is_none_or(|old| key < old) {
                    best = Some(key);
                }
            }
        }
        Ok(best.map(|(_, _, row, sub)| [row, sub]))
    }
}
```

### 2. `src/lib.rs`

Register the new picking component.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod timer;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod timer;

pub mod pick_window;
```

### 3. `src/pick_window_tests.rs`

Copy the focused checks.

Create the file and type:

```rust
use crate::pick_window::Window;

#[test]
fn finite_pick_windows_match_density_and_remain_inside_the_canvas() {
    for density in [0.125, 1.0, 2.0, 1000.0] {
        for full in [
            [1, 1],
            [3, 2],
            [960, 640],
            [u32::MAX, u32::MAX],
        ] {
            for screen in [[-1.0, -1.0], [1.0, 1.0], [0.0, 0.0]] {
                let w = Window::new(screen, full, density).unwrap();
                assert!(w.radius <= 128);
                assert!(w.copy[2] <= 257 && w.copy[3] <= 257);
                assert!(w.draw[2] <= 263 && w.draw[3] <= 263);
                for i in 0..2 {
                    assert!(w.cursor[i] < w.copy[i + 2]);
                    assert!(
                        u64::from(w.draw[i]) + u64::from(w.draw[i + 2])
                            <= u64::from(full[i])
                    );
                    assert!(w.copy[i] >= w.draw[i]);
                }
                assert_eq!(w.row_bytes() % 256, 0);
                assert!(w.crop().iter().all(|v| v.is_finite()));
            }
        }
    }
    assert_eq!(
        Window::new([0.0; 2], [960, 640], 1.0)
            .unwrap()
            .radius,
        6
    );
    assert_eq!(
        Window::new([0.0; 2], [960, 640], 2.0)
            .unwrap()
            .radius,
        12
    );
    for screen in [
        [f32::NAN, 0.0],
        [0.0, f32::INFINITY],
        [1.01, 0.0],
    ] {
        assert!(Window::new(screen, [960, 640], 1.0).is_none());
    }
    for density in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(Window::new([0.0; 2], [960, 640], density).is_none());
    }
    assert!(Window::new([0.0; 2], [0, 640], 1.0).is_none());
}

#[test]
fn cropped_pixel_centres_preserve_homogeneous_depth_in_both_projections() {
    for screen in [
        [-1.0, -1.0],
        [0.0, 0.0],
        [0.9, -0.8],
        [1.0, 1.0],
    ] {
        let w = Window::new(screen, [960, 640], 2.0).unwrap();
        let m = w.crop();
        for weight in [1.0, 0.25, 7.0] {
            for [x, y] in [
                [w.draw[0], w.draw[1]],
                [
                    w.copy[0] + w.cursor[0],
                    w.copy[1] + w.cursor[1],
                ],
            ] {
                let p = [
                    (2.0 * (f64::from(x) + 0.5) / 960.0 - 1.0) * weight,
                    (1.0 - 2.0 * (f64::from(y) + 0.5) / 640.0) * weight,
                    0.4 * weight,
                    weight,
                ];
                let actual: [f64; 4] = std::array::from_fn(|r| {
                    (0..4)
                        .map(|c| f64::from(m[c * 4 + r]) * p[c])
                        .sum()
                });
                let expected = [
                    2.0 * (f64::from(x - w.draw[0]) + 0.5) / f64::from(w.draw[2]) - 1.0,
                    1.0 - 2.0 * (f64::from(y - w.draw[1]) + 0.5) / f64::from(w.draw[3]),
                ];
                for i in 0..2 {
                    assert!((actual[i] / actual[3] - expected[i]).abs() < 0.00001);
                }
                assert_eq!([actual[2], actual[3]], [p[2], p[3]]);
            }
        }
    }
}

#[test]
fn pick_decoding_ignores_padding_and_prefers_visible_ink_within_the_tolerance() {
    let w = Window::new([0.0; 2], [960, 640], 1.0).unwrap();
    let mut bytes = vec![0; w.bytes() as usize];
    let at = |x: u32, y: u32| (y * w.row_bytes() + x * 8) as usize;
    let mut put = |x: u32, y: u32, row: u32, sub: u32| {
        let offset = at(x, y);
        bytes[offset..offset + 4].copy_from_slice(&row.to_le_bytes());
        bytes[offset + 4..offset + 8].copy_from_slice(&sub.to_le_bytes());
    };
    put(6, 6, 16_777_217, 0);
    put(8, 6, 16_777_218, 1);
    put(0, 0, 1, 1);
    bytes[(w.copy[2] * 8) as usize..w.row_bytes() as usize].fill(255);
    assert_eq!(
        w.closest(&bytes).unwrap(),
        Some([16_777_218, 1])
    );
    bytes[at(8, 6)..at(8, 6) + 8].fill(0);
    assert_eq!(
        w.closest(&bytes).unwrap(),
        Some([16_777_217, 0])
    );
    bytes[at(6, 6)..at(6, 6) + 8].fill(0);
    assert_eq!(w.closest(&bytes).unwrap(), None);
    assert!(w.closest(&bytes[..bytes.len() - 1]).is_err());
}
```

### 4. `src/lib.rs`

Register the copied checks.

**Locate this existing block:**

```rust
mod timer;

pub mod pick_window;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod timer;

pub mod pick_window;

#[cfg(test)]
mod pick_window_tests;
```

## <a id="checkpoint-42a-id"></a>2. Keep exact object IDs in shared GPU settings

**What this code does:** Share explicit uniform layouts between visible drawing and integer picking.

### 1. `src/gpu_layout.rs`

Share explicit uniform bindings between draw passes.

Create the file and type:

```rust
// Explicit layouts let visible and ID pipelines share the same bind groups without relying on independently inferred compatibility.
pub fn uniform(device: &wgpu::Device, bytes: u64) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("shared uniform"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: std::num::NonZeroU64::new(bytes),
            },
            count: None,
        }],
    })
}

pub fn group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("shared uniform"),
        layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    })
}
```

### 2. `src/lib.rs`

Register the new picking component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod pick_window_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod pick_window_tests;

pub mod gpu_layout;
```

### 3. `src/gpu_mesh.rs`

Preserve exact IDs and the current visible drawing policy.

**Locate this existing block:**

```rust
    }
    let flag: f32 = if selected {
        1.0
    } else {
        0.0
    };
    bytes[64..68].copy_from_slice(&flag.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(selected).to_ne_bytes());
    let id = if object.metadata.is_locked {
        0
    } else {
        object.id.value()
    };
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model)
```

### 4. `src/triangle.wgsl`

Preserve exact IDs and the current visible drawing policy.

**Locate this existing block:**

```wgsl
    model: mat4x4<f32>,
    selection: vec4<f32>,
    normal_matrix: mat4x4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x > 0.5);
    output.world = world.xyz;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.world = world.xyz;
```

### 5. `src/chain.rs`

Preserve exact IDs and the current visible drawing policy.

**Locate this existing block:**

```rust
impl Source {
    pub fn visible(&self) -> bool {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Source {
    pub fn locked(&self) -> bool {
        match self {
            Self::Line(r) => r.is_locked,
            Self::Polyline(r) => r.is_locked,
            Self::Curve(r, _) => r.is_locked,
        }
    }

    pub fn visible(&self) -> bool {
```

### 6. `src/scene.rs`

Resolve only current visible, unlocked object rows.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
}

impl Scene {
    pub fn demo() -> Self {
```

Replace it with:

```rust
}

impl Scene {
    pub fn pick_id(&self, value: u32) -> Option<ObjectId> {
        self.objects
            .iter()
            .filter(|r| r.visible() && !r.metadata.is_locked)
            .map(|r| r.id)
            .chain(
                self.lines
                    .iter()
                    .filter(|r| r.visible() && !r.prepared.source.is_locked)
                    .map(|r| r.id),
            )
            .chain(
                self.paths
                    .iter()
                    .filter(|r| r.visible() && !r.prepared.source.locked())
                    .map(|r| r.id),
            )
            .chain(
                self.points
                    .iter()
                    .filter(|r| r.visible() && !r.prepared.source.is_locked)
                    .map(|r| r.id),
            )
            .find(|id| id.value() == value)
    }

    pub fn demo() -> Self {
```

**Locate this block:**

```rust
pub struct ObjectId(u32);

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
```

Replace it with:

```rust
pub struct ObjectId(u32);

impl ObjectId {
    pub fn value(self) -> u32 {
        self.0
    }
}

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
```

### 7. `src/renderer.rs`

Use the same explicit mesh settings layout for both drawing passes.

**Locate this existing block:**

```rust
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: None,
            vertex: wgpu::VertexState {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 144);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"),
            bind_group_layouts: &[Some(&view_layout), Some(&model_layout)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
```

## <a id="checkpoint-42b-ink"></a>3. Share point and stroke coverage with picking

**What this code does:** Use the same marker and stroke geometry in visible and ID shaders.

### 1. `src/stroke.wgsl`

Share the existing geometry and coverage calculations.

**Locate this existing block:**

```wgsl
}

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (a.z < 0.0 && b.z < 0.0) { return hidden(colour); }
    if (a.z < 0.0) { a = mix(a, b, -a.z / (b.z - a.z)); }
    if (b.z < 0.0) { b = mix(b, a, -b.z / (a.z - b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width);
}

@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {
    let coverage = clamp(input.half_width + 0.5 - abs(input.side), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (a.z < 0.0 && b.z < 0.0) { return hidden(colour); }
    if (a.z < 0.0) { a = mix(a, b, -a.z / (b.z - a.z)); }
    if (b.z < 0.0) { b = mix(b, a, -b.z / (a.z - b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width);
}

fn colour(input: Output) -> vec4<f32> {
    let coverage = clamp(input.half_width + 0.5 - abs(input.side), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

### 2. `src/marker.wgsl`

Share the existing geometry and coverage calculations.

**Locate this existing block:**

```wgsl
}
@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = diameter * view.density * 0.5;
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius);
}
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {
    let coverage = clamp(input.radius + 0.5 - length(input.offset), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = diameter * view.density * 0.5;
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius);
}
fn colour(input: Output) -> vec4<f32> {
    let coverage = clamp(input.radius + 0.5 - length(input.offset), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

### 3. `src/stroke_pick.wgsl`

Output an integer owner only where ink is visible.

Create the file and type:

```wgsl
@group(1) @binding(0) var<uniform> crop: mat4x4<f32>;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, index);
    var output: PickOutput;
    output.position = crop * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width);
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
}
```

### 4. `src/marker_pick.wgsl`

Output an integer owner only where ink is visible.

Create the file and type:

```wgsl
@group(1) @binding(0) var<uniform> crop: mat4x4<f32>;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(center, diameter, colour, index);
    var output: PickOutput;
    output.position = crop * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius);
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
}
```

## <a id="checkpoint-42c-path"></a>4. Pick connected paths and arrowheads with their drawn shape

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

**What this code does:** Keep joined strokes and shaped arrowheads consistent in both passes.

### 1. `src/chain.wgsl`

Share connected geometry and shaped coverage.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    }
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

Replace it with:

```wgsl
    }
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, previous, next, heads, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

**Locate this block:**

```wgsl
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, screen(position), screen(a), direction, half_width, 0u, 0.0);
}
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {
    let local = input.pixel - input.origin;
    let distance = dot(local, perpendicular(input.direction));
    var coverage = clamp(input.half_width + 0.5 - abs(distance), 0.0, 1.0);
```

Replace it with:

```wgsl
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, screen(position), screen(a), direction, half_width, 0u, 0.0);
}
fn colour(input: Output) -> vec4<f32> {
    let local = input.pixel - input.origin;
    let distance = dot(local, perpendicular(input.direction));
    var coverage = clamp(input.half_width + 0.5 - abs(distance), 0.0, 1.0);
```

**Locate this block:**

```wgsl
}
@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32) -> Output {
    var hidden: Output; hidden.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
```

Replace it with:

```wgsl
}
fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, previous: vec3<f32>, next: vec3<f32>, heads: u32, index: u32) -> Output {
    var hidden: Output; hidden.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
```

### 2. `src/chain_pick.wgsl`

Return owners from the visible connected ink.

Create the file and type:

```wgsl
@group(1) @binding(0) var<uniform> crop: mat4x4<f32>;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
    @location(5) @interpolate(flat) part: u32,
    @location(6) @interpolate(flat) head_length: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, previous, next, heads, index);
    var output: PickOutput;
    output.position = crop * source.position;
    output.colour = source.colour;
    output.pixel = source.pixel;
    output.origin = source.origin;
    output.direction = source.direction;
    output.half_width = source.half_width;
    output.part = source.part;
    output.head_length = source.head_length;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.pixel, input.origin, input.direction, input.half_width, input.part, input.head_length);
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
}
```

## <a id="checkpoint-42d-owners"></a>5. Attach owners to visible ink instances

**What this code does:** Keep picking identities separate from display coordinates and GPU geometry.

### 1. `src/stroke_gpu.rs`

Validate and cache original owners separately from display data.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        });
        Self {
            pipeline,
            uniform,
```

Replace it with:

```rust
        });
        Self {
            pipeline,
            ids: None,
            id_data: Vec::new(),
            uniform,
```

**Locate this block:**

```rust
        );
    }

    pub fn usage(&self) -> [u64; 2] {
        [
            self.data.len() as u64 / self.stride,
```

Replace it with:

```rust
        );
    }

    pub fn set_ids(
        &mut self,
        device: &wgpu::Device,
        ids: impl IntoIterator<Item = [u32; 2]>,
    ) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        let data: Vec<u8> = ids
            .into_iter()
            .flatten()
            .flat_map(u32::to_ne_bytes)
            .collect();
        if data.len() as u64 / 8 != self.data.len() as u64 / self.stride {
            return Err("Pick IDs must match display instances");
        }
        if data == self.id_data {
            return Ok(());
        }
        self.ids = if data.is_empty() {
            None
        } else {
            Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ink IDs"),
                    contents: &data,
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            )
        };
        self.id_data = data;
        Ok(())
    }

    pub fn pick_bytes(&self) -> u64 {
        self.ids
            .as_ref()
            .map_or(0, wgpu::Buffer::size)
    }

    pub fn usage(&self) -> [u64; 2] {
        [
            self.data.len() as u64 / self.stride,
```

**Locate this block:**

```rust
    pipeline: wgpu::RenderPipeline,
    uniform: wgpu::Buffer,
    group: wgpu::BindGroup,
    vertices: Option<wgpu::Buffer>,
```

Replace it with:

```rust
    pipeline: wgpu::RenderPipeline,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    uniform: wgpu::Buffer,
    group: wgpu::BindGroup,
    vertices: Option<wgpu::Buffer>,
```

### 2. `src/renderer.rs`

Match every visible instance to its source owner in the same order.

**Locate this existing block:**

```rust
            .expect("Prepared line placement fits the display range");
        let layout = self.pipeline.get_bind_group_layout(1);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            .expect("Prepared line placement fits the display range");
        let controls = scene.control_markers();
        self.markers
            .set_ids(
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|r| r.visible())
                    .map(|r| {
                        [
                            if r.prepared.source.is_locked {
                                0
                            } else {
                                r.id.value()
                            },
                            1,
                        ]
                    })
                    .chain(controls.iter().map(|r| {
                        [
                            scene
                                .pick_id(r.parent.value())
                                .map_or(0, ObjectId::value),
                            1,
                        ]
                    })),
            )
            .expect("Marker IDs match markers");
        self.paths
            .set_ids(
                &self.device,
                scene
                    .paths()
                    .iter()
                    .filter(|r| r.visible())
                    .flat_map(|r| {
                        vec![
                            [
                                if r.prepared.source.locked() {
                                    0
                                } else {
                                    r.id.value()
                                },
                                1
                            ];
                            r.segments().len()
                        ]
                    }),
            )
            .expect("Path IDs match segments");
        self.strokes
            .set_ids(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .filter(|r| r.visible())
                    .map(|r| {
                        [
                            if r.prepared.source.is_locked {
                                0
                            } else {
                                r.id.value()
                            },
                            1,
                        ]
                    }),
            )
            .expect("Line IDs match lines");
        let layout = self.pipeline.get_bind_group_layout(1);
```

### 3. `src/scene.rs`

Remove a selected original row of any current geometry kind. Retain selection for every original geometry kind.

**Locate this existing block:**

```rust
    pub fn remove(&mut self, id: ObjectId) -> bool {
        let Some(index) = self
            .objects
            .iter()
            .position(|object| object.id == id)
        else {
            return false;
        };
        self.objects.remove(index);
        if self.extra == Some(id) {
            self.extra = None;
        }
        true
    }

    pub fn contains(&self, id: ObjectId) -> bool {
        self.objects
            .iter()
            .any(|object| object.id == id)
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn remove(&mut self, id: ObjectId) -> bool {
        let existed = self.contains(id);
        self.objects.retain(|row| row.id != id);
        self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id);
        self.points.retain(|row| row.id != id);
        if self.extra == Some(id) {
            self.extra = None;
        }
        existed
    }

    pub fn contains(&self, id: ObjectId) -> bool {
        self.objects
            .iter()
            .map(|r| r.id)
            .chain(self.lines.iter().map(|r| r.id))
            .chain(self.paths.iter().map(|r| r.id))
            .chain(self.points.iter().map(|r| r.id))
            .any(|row| row == id)
    }
```

### 4. `src/editor.rs`

Delete ink rows while preserving the existing cold-mesh source guard.

**Locate this existing block:**

```rust
    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        let row = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if row.geometry().is_none() {
            return Err("Reload editable sources before Delete");
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .is_some_and(|row| row.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
```

### 5. `src/edit_intent.rs`

Let original ink deletion proceed without requesting mesh rehydration.

**Locate this existing block:**

```rust
            Self::Move { id, .. } | Self::Delete { id } => {
                let row = editor
                    .scene
                    .objects()
                    .iter()
                    .find(|row| row.id == *id)
                    .ok_or("Object not found")?;
                Ok(ReloadKey::of(row).into_iter().collect())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Self::Move { id, .. } | Self::Delete { id } => {
                let Some(row) = editor
                    .scene
                    .objects()
                    .iter()
                    .find(|row| row.id == *id)
                else {
                    return if editor.scene.contains(*id) {
                        Ok(Vec::new())
                    } else {
                        Err("Object not found")
                    };
                };
                Ok(ReloadKey::of(row).into_iter().collect())
```

## <a id="checkpoint-42e-pass"></a>6. Draw integer ink with the shared geometry buffers

**What this code does:** Add integer pipelines without another geometry upload.

### 1. `src/stroke_gpu.rs`

Reuse visible geometry in the integer ink pass.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        });
        Self {
            pipeline,
            ids: None,
```

Replace it with:

```rust
        });
        Self {
            pipeline,
            pick_pipeline,
            ids: None,
```

**Locate this block:**

```rust
            multiview_mask: None,
            cache: None,
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("stroke view"),
            size: 80,
```

Replace it with:

```rust
            multiview_mask: None,
            cache: None,
        });
        let pick_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ink IDs"),
                layout: Some(&pick_layout),
                vertex: wgpu::VertexState {
                    module: &pick_shader,
                    entry_point: Some("pick_vertex"),
                    compilation_options: Default::default(),
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: stride,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes,
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: 8,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &wgpu::vertex_attr_array![7 => Uint32x2],
                        },
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &pick_shader,
                    entry_point: Some("pick_fragment"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())],
                }),
                primitive: Default::default(),
                depth_stencil: Some(crate::depth::ink()),
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("stroke view"),
            size: 80,
```

**Locate this block:**

```rust
        attributes: &[wgpu::VertexAttribute],
        depth: wgpu::DepthStencilState,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
```

Replace it with:

```rust
        attributes: &[wgpu::VertexAttribute],
        depth: wgpu::DepthStencilState,
    ) -> Self {
        let view_layout = crate::gpu_layout::uniform(device, 80);
        let crop_layout = crate::gpu_layout::uniform(device, 64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"),
            bind_group_layouts: &[Some(&view_layout)],
            immediate_size: 0,
        });
        let pick_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("ink pick settings"),
                bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)],
                immediate_size: 0,
            });
        let pick_source = match stride {
            44 => include_str!("stroke_pick.wgsl"),
            72 => include_str!("chain_pick.wgsl"),
            32 => include_str!("marker_pick.wgsl"),
            _ => unreachable!(),
        };
        let pick_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(format!("{source}\n{pick_source}").into()),
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
```

**Locate this block:**

```rust
        Ok(())
    }

    pub fn pick_bytes(&self) -> u64 {
        self.ids
            .as_ref()
```

Replace it with:

```rust
        Ok(())
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids)) = (&self.vertices, &self.ids) else {
            return;
        };
        pass.set_pipeline(&self.pick_pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(
            0..self.vertices_per_instance,
            0..(self.data.len() as u64 / self.stride) as u32,
        );
    }

    pub fn pick_bytes(&self) -> u64 {
        self.ids
            .as_ref()
```

**Locate this block:**

```rust
    pipeline: wgpu::RenderPipeline,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    uniform: wgpu::Buffer,
```

Replace it with:

```rust
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: wgpu::RenderPipeline,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    uniform: wgpu::Buffer,
```

## <a id="checkpoint-42f-mesh"></a>7. Draw depth-tested mesh owners into an integer attachment

**What this code does:** Use original mesh buffers and placements for exact visible owner IDs.

### 1. `src/triangle_pick.wgsl`

Draw exact mesh owners at the original placed depth.

Create the file and type:

```wgsl
@group(2) @binding(0) var<uniform> crop: mat4x4<f32>;
@vertex
fn pick_vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> @builtin(position) vec4<f32> {
    return crop * view.transform * object.model * vec4<f32>(position, 1.0);
}
@fragment
fn pick_fragment() -> @location(0) vec2<u32> { return vec2<u32>(object.selection.y, 0u); }
```

### 2. `src/pick_pipeline.rs`

Create the integer mesh attachment and shared bindings.

Create the file and type:

```rust
pub fn mesh(
    device: &wgpu::Device,
    view_layout: &wgpu::BindGroupLayout,
    model_layout: &wgpu::BindGroupLayout,
    crop_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("mesh IDs"),
        bind_group_layouts: &[
            Some(view_layout),
            Some(model_layout),
            Some(crop_layout),
        ],
        immediate_size: 0,
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("mesh IDs"),
        source: wgpu::ShaderSource::Wgsl(
            concat!(
                include_str!("triangle.wgsl"),
                "\n",
                include_str!("triangle_pick.wgsl")
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

### 3. `src/lib.rs`

Register the new picking component.

**Locate this existing block:**

```rust
mod pick_window_tests;

pub mod gpu_layout;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod pick_window_tests;

pub mod gpu_layout;

pub mod pick_pipeline;
```

## <a id="checkpoint-42g-image"></a>8. Own a small ID image, depth target and readback buffer

**What this code does:** Allocate bounded picking resources with checked padded rows.

### 1. `src/pick_image.rs`

Allocate only the bounded target and padded readback.

Create the file and type:

```rust
use crate::pick_window::Window;

pub(crate) struct Image {
    pub texture: wgpu::Texture,
    pub colour: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub readback: wgpu::Buffer,
    pub window: Window,
}

impl Image {
    pub fn new(device: &wgpu::Device, window: Window) -> Self {
        let size = wgpu::Extent3d {
            width: window.draw[2],
            height: window.draw[3],
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("bounded pick image"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rg32Uint,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let colour = texture.create_view(&Default::default());
        let depth = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("pick depth"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: crate::depth::FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&Default::default());
        // GPU image rows need 256-byte copy alignment; the decoder skips that padding and reads only the logical columns.
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bounded pick readback"),
            size: window.bytes(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Self {
            texture,
            colour,
            depth,
            readback,
            window,
        }
    }
}
```

### 2. `src/lib.rs`

Register the new picking component.

**Locate this existing block:**

```rust
pub mod gpu_layout;

pub mod pick_pipeline;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod gpu_layout;

pub mod pick_pipeline;

mod pick_image;
```

## <a id="checkpoint-42h-owner"></a>9. Bound asynchronous picking ownership

**What this code does:** Own one active request and one latest pending cursor.

### 1. `src/pick_gpu.rs`

Own bounded requests and release them on cancellation.

Create the file and type:

```rust
use crate::pick_window::Window;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU8, Ordering},
};

#[derive(Clone, Copy)]
pub struct Request {
    pub window: Window,
    pub transform: [f32; 16],
    pub density: f32,
}

pub struct Picker {
    pipeline: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    crop: wgpu::Buffer,
    crop_group: wgpu::BindGroup,
    image: Option<crate::pick_image::Image>,
    active: Option<Arc<AtomicU8>>,
    pending: Option<Request>,
    alive: Arc<AtomicBool>,
    pub submitted: u64,
    pub coalesced: u64,
}

impl Drop for Picker {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Release);
    }
}

impl Picker {
    pub fn new(
        device: &wgpu::Device,
        view_layout: &wgpu::BindGroupLayout,
        model_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let crop_layout = crate::gpu_layout::uniform(device, 64);
        let pipeline = crate::pick_pipeline::mesh(
            device,
            view_layout,
            model_layout,
            &crop_layout,
        );
        let buffer = |size| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("pick view"),
                size,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let view = buffer(80);
        let crop = buffer(64);
        let view_group = crate::gpu_layout::group(device, view_layout, &view);
        let crop_group = crate::gpu_layout::group(device, &crop_layout, &crop);
        Self {
            pipeline,
            view,
            view_group,
            crop,
            crop_group,
            image: None,
            active: None,
            pending: None,
            alive: Arc::new(AtomicBool::new(true)),
            submitted: 0,
            coalesced: 0,
        }
    }

    pub fn queue(&mut self, request: Request) {
        if self.pending.replace(request).is_some() {
            self.coalesced += 1;
        }
    }

    pub fn queued(&self) -> bool {
        self.pending.is_some()
    }

    pub fn cancel(&mut self) {
        self.pending = None;
        let mapped = self
            .active
            .take()
            .is_some_and(|ready| ready.load(Ordering::Acquire) <= 1);
        if let Some(image) = self.image.take() {
            if mapped {
                image.readback.unmap();
            }
        }
    }

    pub fn next(&mut self) -> Option<Request> {
        if self.active.is_some() {
            None
        } else {
            self.pending.take()
        }
    }

    pub fn usage(&self) -> [u64; 3] {
        [
            u64::from(self.active.is_some()),
            u64::from(self.pending.is_some()),
            self.image.as_ref().map_or(0, |image| {
                image.readback.size()
                    + u64::from(image.window.draw[2])
                        * u64::from(image.window.draw[3])
                        * 12
            }) + 144,
        ]
    }
}
```

### 2. `src/lib.rs`

Register the new picking component.

**Locate this existing block:**

```rust
pub mod pick_pipeline;

mod pick_image;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod pick_pipeline;

mod pick_image;

pub mod pick_gpu;
```

## <a id="checkpoint-42i-copy"></a>10. Record the ID draw and aligned image copy

**What this code does:** Submit surfaces and ink to a bounded image before copying its cursor region.

### 1. `src/pick_encode.rs`

Record depth-correct IDs and copy the bounded cursor region.

Create the file and type:

```rust
use crate::{
    gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane,
};

pub(crate) fn draw(
    device: &wgpu::Device,
    image: &Image,
    window: Window,
    pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup,
    crop_group: &wgpu::BindGroup,
    meshes: &[GpuMesh],
    ink: &[&Lane],
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
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
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, crop_group, &[]);
        for mesh in meshes {
            mesh.draw(&mut pass);
        }
        for lane in ink {
            lane.pick(&mut pass, crop_group);
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

### 2. `src/lib.rs`

Register the new picking component.

**Locate this existing block:**

```rust
mod pick_image;

pub mod pick_gpu;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod pick_image;

pub mod pick_gpu;

mod pick_encode;
```

## <a id="checkpoint-42j-read"></a>11. Map the integer hit without blocking browser input

**What this code does:** Read completed GPU bytes asynchronously and reuse bounded resources.

### 1. `src/pick_gpu.rs`

Map asynchronously and decode only completed bytes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.pending.take()
        }
    }
```

Replace it with:

```rust
            self.pending.take()
        }
    }

    pub fn submit(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        request: Request,
        meshes: &[GpuMesh],
        ink: &[&Lane],
    ) {
        assert!(
            self.active.is_none(),
            "Only one pick may be in flight"
        );
        let window = request.window;
        if self.image.as_ref().is_none_or(|image| {
            image.window.draw[2..] != window.draw[2..]
                || image.readback.size() != window.bytes()
        }) {
            self.image = Some(crate::pick_image::Image::new(device, window));
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
        let bytes: Vec<u8> = window
            .crop()
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
            lane.view(
                queue,
                &request.transform,
                window.full,
                request.density,
            );
        }
        queue.submit([crate::pick_encode::draw(
            device,
            image,
            window,
            &self.pipeline,
            &self.view_group,
            &self.crop_group,
            meshes,
            ink,
        )]);
        let ready = Arc::new(AtomicU8::new(0));
        self.active = Some(Arc::clone(&ready));
        let alive = Arc::clone(&self.alive);
        // Mapping finishes asynchronously; the callback publishes readiness, and the owner reads the bytes without blocking browser input.
        image
            .readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                ready.store(
                    if result.is_ok() {
                        1
                    } else {
                        2
                    },
                    Ordering::Release,
                );
                if alive.load(Ordering::Acquire) {
                    #[cfg(target_arch = "wasm32")]
                    crate::browser_pick::wake();
                }
            });
        self.submitted += 1;
    }

    pub fn take(&mut self) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let status = self.active.as_ref()?.load(Ordering::Acquire);
        if status == 0 {
            return None;
        }
        self.active = None;
        let image = self.image.as_ref().unwrap();
        if status != 1 {
            return Some(Err("Cannot read GPU pick image"));
        }
        let bytes = image.readback.slice(..).get_mapped_range();
        let result = image.window.closest(&bytes);
        drop(bytes);
        image.readback.unmap();
        Some(result)
    }
```

**Locate this block:**

```rust
use crate::pick_window::Window;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU8, Ordering},
```

Replace it with:

```rust
use crate::{gpu_mesh::GpuMesh, pick_window::Window, stroke_gpu::Lane};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU8, Ordering},
```

### 2. `src/browser_pick.rs`

Wake the browser owner when GPU mapping completes.

Create the file and type:

```rust
pub fn wake() {
    if let (Some(window), Ok(event)) = (
        web_sys::window(),
        web_sys::Event::new("viewer-pick"),
    ) {
        let _ = window.dispatch_event(&event);
    }
}
```

### 3. `src/lib.rs`

Register the browser-only completion wakeup.

**Locate this existing block:**

```rust
pub mod pick_gpu;

mod pick_encode;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod pick_gpu;

mod pick_encode;

#[cfg(target_arch = "wasm32")]
pub mod browser_pick;
```

## <a id="checkpoint-42k-input"></a>12. Select rows through actual GPU canvas clicks

**What this code does:** Connect real canvas input to bounded integer GPU picking for meshes and ink.

### 1. `src/renderer.rs`

Queue bounded GPU requests and reuse the visible geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        drop(old);
        self.geometry.prune();
    }
```

Replace it with:

```rust
        drop(old);
        self.geometry.prune();
    }

    pub fn request_pick(
        &mut self,
        screen: [f32; 2],
        transform: [f32; 16],
    ) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density)
            .ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request {
            window,
            transform,
            density: self.density,
        });
        self.pump_pick();
        Ok(())
    }

    fn pump_pick(&mut self) {
        if let Some(request) = self.picker.next() {
            self.picker.submit(
                &self.device,
                &self.queue,
                request,
                &self.meshes,
                &[&self.strokes, &self.paths, &self.markers],
            );
        }
    }

    pub fn take_pick(&mut self) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let result = self.picker.take();
        if result.is_some() && self.picker.queued() {
            self.pump_pick();
            return None;
        }
        result
    }

    pub fn cancel_pick(&mut self) {
        self.picker.cancel();
    }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [
            active,
            pending,
            bytes
                + self.strokes.pick_bytes()
                + self.paths.pick_bytes()
                + self.markers.pick_bytes(),
            self.picker.submitted,
            self.picker.coalesced,
        ]
    }
```

**Locate this block:**

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.markers
            .set_points(
                &self.device,
```

Replace it with:

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel();
        self.markers
            .set_points(
                &self.device,
```

**Locate this block:**

```rust
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 {
            self.density = density;
        }
    }
```

Replace it with:

```rust
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] {
            self.picker.cancel();
        }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density {
            self.picker.cancel();
            self.density = density;
        }
    }
```

**Locate this block:**

```rust
            grid_settings: None,
            size: [640, 480],
            density: 1.0,
        };
        renderer.set_scene(scene, None);
        renderer
```

Replace it with:

```rust
            grid_settings: None,
            size: [640, 480],
            density: 1.0,
            picker,
        };
        renderer.set_scene(scene, None);
        renderer
```

**Locate this block:**

```rust
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let mut renderer = Self {
            device,
            queue,
```

Replace it with:

```rust
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let mut renderer = Self {
            device,
            queue,
```

**Locate this block:**

```rust
    density: f32,
}

impl Renderer {
```

Replace it with:

```rust
    density: f32,
    picker: crate::pick_gpu::Picker,
}

impl Renderer {
```

### 2. `src/editor.rs`

Resolve accepted picks without changing document history.

**Locate this existing block:**

```rust
    Pick([f32; 2]),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    ReplaceAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    Close,
    UnloadSources,
    AddBox,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
}

pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene
            .objects()
            .iter()
            .filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id))
            .collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(
        &mut self,
        values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>,
    ) -> Result<bool, &'static str> {
        if values.is_empty() {
            return Ok(false);
        }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id)
                || !self
                    .scene
                    .objects()
                    .iter()
                    .any(|row| key.matches(row))
            {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values
            .into_iter()
            .map(|(key, bytes)| crate::rehydrate::PreparedReload::new(key, &bytes))
            .collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self
                .scenes()
                .flat_map(|scene| scene.objects())
                .filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row))
            {
                return Err("Reloaded source GUID or metadata does not match");
            }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared {
                scene.hydrate(plan);
            }
        }
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self
            .scenes()
            .flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload())
            .filter_map(|row| row.origin().map(|origin| origin.id))
            .collect();
        for row in self
            .scenes()
            .flat_map(|scene| scene.objects())
        {
            if let Some(origin) = row.origin() {
                if !row.can_unload() {
                    origins.remove(&origin.id);
                }
            }
        }
        if origins.is_empty() {
            return Err("No eligible imported sources to unload");
        }
        let epoch = self
            .last_release
            .checked_add(1)
            .ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() {
            scene.unload(&origins, epoch);
        }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(
        &mut self,
        id: ObjectId,
        offset: [f64; 3],
    ) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .is_some_and(|row| row.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        if self.selected == Some(id) {
            self.selected = None;
        }
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Pick(screen) => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    ReplaceAt(
        Vec<u8>,
        std::rc::Rc<crate::reload_url::ReloadUrl>,
    ),
    Close,
    UnloadSources,
    AddBox,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
}

pub struct Editor {
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene
            .objects()
            .iter()
            .filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id))
            .collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(
        &mut self,
        values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>,
    ) -> Result<bool, &'static str> {
        if values.is_empty() {
            return Ok(false);
        }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id)
                || !self
                    .scene
                    .objects()
                    .iter()
                    .any(|row| key.matches(row))
            {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values
            .into_iter()
            .map(|(key, bytes)| crate::rehydrate::PreparedReload::new(key, &bytes))
            .collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self
                .scenes()
                .flat_map(|scene| scene.objects())
                .filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row))
            {
                return Err("Reloaded source GUID or metadata does not match");
            }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared {
                scene.hydrate(plan);
            }
        }
        Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self
            .scenes()
            .flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload())
            .filter_map(|row| row.origin().map(|origin| origin.id))
            .collect();
        for row in self
            .scenes()
            .flat_map(|scene| scene.objects())
        {
            if let Some(origin) = row.origin() {
                if !row.can_unload() {
                    origins.remove(&origin.id);
                }
            }
        }
        if origins.is_empty() {
            return Err("No eligible imported sources to unload");
        }
        let epoch = self
            .last_release
            .checked_add(1)
            .ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() {
            scene.unload(&origins, epoch);
        }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(
        &mut self,
        id: ObjectId,
        offset: [f64; 3],
    ) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) {
            return Err("Move needs finite coordinates");
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if offset.iter().all(|&value| value == 0.0) {
            return Ok(Change::View);
        }
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        let model = &shift * &object.model;
        self.history
            .try_edit(&mut self.scene, |scene| {
                scene.place(id, model)
            })?;
        Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        if self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .is_some_and(|row| row.geometry().is_none())
        {
            return Err("Reload editable sources before Delete");
        }
        self.history.edit(&mut self.scene, |scene| {
            scene.remove(id);
        });
        if self.selected == Some(id) {
            self.selected = None;
        }
        Ok(Change::Scene)
    }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Picked(id) => {
                self.selected = id.filter(|id| self.scene.pick_id(id.value()).is_some())
            }
            Action::Pick(screen) => {
```

### 3. `src/browser.rs`

Route actual clicks and completion events through the owning renderer.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
```

Replace it with:

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
        "data-pick-gpu",
        &serde_json::to_string(&renderer.pick_usage()).unwrap(),
    )?;
```

**Locate this block:**

```rust
    listeners.listen(window.as_ref(), "blur")?;
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
```

Replace it with:

```rust
    listeners.listen(window.as_ref(), "blur")?;
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
```

**Locate this block:**

```rust
            };
            if let Some(action) = action {
                if matches!(
                    &action,
```

Replace it with:

```rust
            };
            let action = if event.type_() == "viewer-pick" {
                match renderer.take_pick() {
                    Some(Ok(hit)) => Some(Action::Picked(
                        hit.and_then(|hit| editor.scene.pick_id(hit[0])),
                    )),
                    Some(Err(error)) => {
                        report(error);
                        None
                    }
                    None => None,
                }
            } else {
                match action {
                    Some(Action::Pick(screen)) => {
                        if let Err(error) =
                            renderer.request_pick(screen, editor.camera.uniform())
                        {
                            report(error);
                        }
                        None
                    }
                    Some(action) => {
                        renderer.cancel_pick();
                        Some(action)
                    }
                    None => None,
                }
            };
            if let Some(action) = action {
                if matches!(
                    &action,
```

### 4. `src/pick_input_tests.rs`

Copy current-row resolution and history checks.

Create the file and type:

```rust
use crate::editor::{Action, Editor};

#[test]
fn accepted_gpu_picks_resolve_current_rows_without_consuming_document_history() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddPoint).unwrap();
    let id = editor.scene.points()[0].id;
    editor
        .apply(Action::Picked(Some(id)))
        .unwrap();
    assert_eq!(editor.selected, Some(id));
    let intent = crate::edit_intent::Intent::capture(&Action::Delete, editor.selected)
        .unwrap()
        .unwrap();
    assert!(
        intent.keys(&editor).unwrap().is_empty(),
        "Original ink needs no mesh reload"
    );
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.points().is_empty());
    assert!(editor.selected.is_none());
    editor
        .apply(Action::Picked(Some(id)))
        .unwrap();
    assert!(editor.selected.is_none());
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.points()[0].id, id);
    editor
        .apply(Action::Picked(Some(id)))
        .unwrap();
    editor.apply(Action::Delete).unwrap();
    assert!(editor.scene.points().is_empty());
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.points()[0].id, id);
    editor.apply(Action::Picked(None)).unwrap();
    assert!(editor.selected.is_none());
}

#[test]
fn original_lines_paths_arrows_and_control_parents_share_selection_and_delete_history()
{
    for action in [
        Action::AddLine,
        Action::AddPolyline,
        Action::AddArrow,
        Action::AddCurve,
    ] {
        let mut editor = Editor::default();
        editor.apply(Action::Close).unwrap();
        editor.apply(action).unwrap();
        let id = editor
            .scene
            .lines()
            .first()
            .map(|r| r.id)
            .or_else(|| editor.scene.paths().first().map(|r| r.id))
            .unwrap();
        editor
            .apply(Action::Picked(Some(id)))
            .unwrap();
        assert_eq!(editor.selected, Some(id));
        let intent =
            crate::edit_intent::Intent::capture(&Action::Delete, editor.selected)
                .unwrap()
                .unwrap();
        assert!(intent.keys(&editor).unwrap().is_empty());
        editor.apply(Action::Delete).unwrap();
        assert!(!editor.scene.contains(id));
        editor.apply(Action::Undo).unwrap();
        assert!(editor.scene.contains(id));
        editor.apply(Action::Redo).unwrap();
        assert!(!editor.scene.contains(id));
        editor.apply(Action::Close).unwrap();
        editor
            .apply(Action::Picked(Some(id)))
            .unwrap();
        assert!(editor.selected.is_none());
    }
}
```

### 5. `src/lib.rs`

Register the copied input checks.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
pub mod browser_pick;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
pub mod browser_pick;

#[cfg(test)]
mod pick_input_tests;
```

### 6. `src/pick_gpu_tests.rs`

Copy actual GPU depth, ink, density, bounded work and cancellation checks.

Create the file and type:

```rust
use super::*;
use crate::{renderer::Renderer, viewport::Viewport};

fn renderer(scene: &Scene) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default()))
        .expect("Actual GPU adapter");
    let (device, queue) =
        pollster::block_on(adapter.request_device(&Default::default()))
            .expect("Actual GPU device");
    let mut renderer = Renderer::new(
        device,
        queue,
        wgpu::TextureFormat::Rgba8Unorm,
        scene,
    );
    renderer.resize(Viewport {
        width: 320,
        height: 240,
    });
    renderer
}

fn finish(renderer: &mut Renderer) -> Option<[u32; 2]> {
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    renderer
        .take_pick()
        .expect("Map callback completed")
        .unwrap()
}

fn identity() -> [f32; 16] {
    session_rust::Xform::identity()
        .m
        .map(|n| n as f32)
}
fn triangle(z: f64) -> PreparedMesh {
    PreparedMesh::triangle(
        [
            [-0.8, -0.8, z],
            [0.8, -0.8, z],
            [0.0, 0.8, z],
        ],
        [0.8, 0.2, 0.1],
    )
    .unwrap()
}

#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion() {
    let mut scene = Scene::demo();
    scene.clear();
    scene.next_id = 16_777_217;
    let back = scene.insert(triangle(0.6)).unwrap();
    let mut renderer = renderer(&scene);
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer),
        Some([back.value(), 0])
    );
    let front = scene.insert(triangle(0.2)).unwrap();
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer),
        Some([front.value(), 0])
    );
    let mut locked = scene.objects[1].metadata.as_ref().clone();
    locked.is_locked = true;
    scene.objects[1].metadata = Rc::new(locked);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer), None);
    assert_eq!(scene.pick_id(front.value()), None);
    let mut hidden = scene.objects[1].metadata.as_ref().clone();
    hidden.is_visible = false;
    scene.objects[1].metadata = Rc::new(hidden);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer),
        Some([back.value(), 0])
    );
    renderer
        .request_pick([0.95, 0.95], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer), None);
}

#[test]
fn actual_ink_pick_uses_coverage_depth_density_and_bounded_coalescing() {
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = scene.insert(triangle(0.5)).unwrap();
    let mut point = session_rust::Point::new(0.0, 0.0, 0.2);
    point.width = 12.0;
    let point = scene
        .insert_point(crate::marker::PreparedPoint::new(Rc::new(point)).unwrap())
        .unwrap();
    let mut line = session_rust::Line::new(-0.4, 0.4, 0.2, 0.4, 0.4, 0.2);
    line.width = 5.0;
    let line = scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap())
        .unwrap();
    let mut path = session_rust::Polyline::new(vec![
        session_rust::Point::new(-0.4, -0.4, 0.2),
        session_rust::Point::new(0.4, -0.4, 0.2),
    ]);
    path.width = 5.0;
    let path = scene
        .insert_path(crate::chain::PreparedChain::polyline(Rc::new(path)).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let geometry = renderer.stats();
    for density in [1.0, 2.0] {
        renderer.set_density(density);
        renderer.resize(Viewport {
            width: (320.0 * density) as u32,
            height: (240.0 * density) as u32,
        });
        for (cursor, expected) in [
            ([0.0, 0.0], point),
            ([0.0, 0.4], line),
            ([0.0, -0.4], path),
            ([0.0, 0.65], mesh),
        ] {
            renderer
                .request_pick(cursor, identity())
                .unwrap();
            assert_eq!(
                finish(&mut renderer).unwrap()[0],
                expected.value()
            );
        }
    }
    assert_eq!(
        renderer.stats(),
        geometry,
        "Picking and resize reuse visible geometry"
    );
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    for _ in 0..20 {
        renderer
            .request_pick([0.95, 0.95], identity())
            .unwrap();
    }
    assert_eq!(&renderer.pick_usage()[..2], &[1, 1]);
    assert_eq!(renderer.pick_usage()[4], 19);
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    assert!(
        renderer.take_pick().is_none(),
        "A newer cursor supersedes the first result"
    );
    assert_eq!(finish(&mut renderer), None);
    assert_eq!(&renderer.pick_usage()[..2], &[0, 0]);
    assert!(
        renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 12 + 176,
        "Readback does not grow with the full drawing"
    );
}

#[test]
fn cancelled_readback_cannot_select_a_new_document_or_keep_its_source_alive() {
    let mut scene = Scene::demo();
    scene.clear();
    let old = scene.insert(triangle(0.2)).unwrap();
    let weak = Rc::downgrade(&scene.objects[0].mesh);
    let mut renderer = renderer(&scene);
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(&renderer.pick_usage()[..3], &[0, 0, 144]);
    let new = scene.insert(triangle(0.3)).unwrap();
    assert_ne!(old, new);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer), Some([new.value(), 0]));
    assert!(
        renderer.take_pick().is_none(),
        "The cancelled map has no result for the replacement"
    );
    let uploads = renderer.stats();
    renderer
        .request_pick([0.0, 0.0], identity())
        .unwrap();
    renderer.cancel_pick();
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    assert!(renderer.take_pick().is_none());
    assert_eq!(renderer.stats(), uploads);
}
```

### 7. `src/scene.rs`

Register actual GPU checks with private checked scene IDs.

**Locate this existing block:**

```rust
        assert_eq!(scene.objects().len(), 1);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert_eq!(scene.objects().len(), 1);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "pick_gpu_tests.rs"]
mod pick_gpu_tests;
```

## Build & Run

![Recorded visual output for Lesson 6.1 (42k-input)](../../screenshots/journey/42k-input.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 6.1
npm --prefix ../session_tests run course -- save 6.1
```

Continue with [6.2 · Accept a hit only for its current view](43-ticket.md).
