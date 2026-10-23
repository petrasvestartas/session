# 5.1 · Original lines and a screen-space stroke lane

**Goal:** Insert an editable line as one undoable command and prove its width in actual browser pixels. · **Steps:** 7 · **Source:** [app/walk/curves.rs](../../../../src/app/walk/curves.rs) · [engine/gpu/segments.rs](../../../../src/engine/gpu/segments.rs) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl) · [app/walk/points.rs](../../../../src/app/walk/points.rs) · [shaders/glyph.wgsl](../../../../src/shaders/glyph.wgsl) · [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl) · [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl) · [shaders/grid.wgsl](../../../../src/shaders/grid.wgsl) (`src/shaders/ribbon.wgsl,src/shaders/glyph.wgsl,src/engine/gpu/segments.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D Segment (p0 -&gt; p1) + Near-Plane Clip</text>
  <line x1="108" y1="48" x2="108" y2="158" stroke="#dc2626" stroke-width="2" stroke-dasharray="4 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="108" y="42" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#dc2626">w = near</text>
  <line x1="56" y1="134" x2="108" y2="112" stroke="#94a3b8" stroke-width="2" stroke-dasharray="3 3"/>
  <line x1="108" y1="112" x2="296" y2="62" stroke="#111111" stroke-width="2.5"/>
  <circle cx="56" cy="134" r="4" fill="#94a3b8"/>
  <circle cx="108" cy="112" r="5" fill="#dc2626"/>
  <circle cx="296" cy="62" r="5" fill="#2563eb"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="136" y="128" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">p0_clip</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="288" y="80" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">p1</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@vertex: 6-Vertex Screen Quad (±n * r)</text>
  <polygon points="444,114 648,54 664,96 460,154" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <line x1="444" y1="114" x2="664" y2="96" stroke="#2563eb" stroke-width="1.25" stroke-dasharray="4 3"/>
  <line x1="452" y1="134" x2="656" y2="75" stroke="#111111" stroke-width="2" stroke-dasharray="3 3"/>
  <circle cx="452" cy="134" r="4.5" fill="#dc2626"/>
  <circle cx="656" cy="75" r="4.5" fill="#dc2626"/>
  <line x1="554" y1="104" x2="542" y2="68" stroke="#16a34a" stroke-width="2.25" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="524" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">+n</text>
  <line x1="554" y1="104" x2="566" y2="140" stroke="#16a34a" stroke-width="2.25" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="582" y="144" font-family="monospace" font-size="10" font-weight="700" fill="#16a34a">-n</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@fragment: SDF Anti-Aliased Stroke &amp; Caps</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <path d="M 842 134 L 916 72 L 1022 96" fill="none" stroke="#2563eb" stroke-width="10" stroke-linecap="round" stroke-linejoin="round"/>
  <polygon points="1034,99 1010,82 1004,108" fill="#2563eb"/>
  <circle cx="842" cy="134" r="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <circle cx="916" cy="72" r="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
</svg>
</div>

## <a id="checkpoint-35-stroke"></a>1. Prepare a stroke without changing its source

**What this code does:** Prepare finite line endpoints, colour and screen width while retaining the original kernel line.

### 1. `src/stroke.rs`

Keep exact kernel ownership separate from a checked packed display record.

Create the file and type:

```rust
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stroke {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub colour: [f32; 4],
    pub width: f32,
}

#[derive(Clone)]
pub struct PreparedLine {
    pub source: Rc<session_rust::Line>,
    pub display: Stroke,
}

impl PreparedLine {
    pub fn new(source: Rc<session_rust::Line>) -> Result<Self, &'static str> {
        let _ = source.guid();
        let start = std::array::from_fn(|i| source.start()[i] as f32);
        let end = std::array::from_fn(|i| source.end()[i] as f32);
        let colour = source.linecolor.to_f32();
        let width = if source.width.is_finite() && source.width > 0.0 {
            source.width as f32
        } else {
            1.0
        };
        let width = if width == 0.0 {
            1.0
        } else {
            width
        };
        let display = Stroke {
            start,
            end,
            colour,
            width,
        };
        if display
            .values()
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err("Line exceeds the display range");
        }
        Ok(Self { source, display })
    }
}

impl Stroke {
    pub fn values(&self) -> [f32; 11] {
        let [a, b, c] = self.start;
        let [d, e, f] = self.end;
        let [r, g, blue, alpha] = self.colour;
        [
            a, b, c, d, e, f, r, g, blue, alpha, self.width,
        ]
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.values()
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect()
    }
}
```

### 2. `src/lib.rs`

Register stroke preparation and its checks.

**Locate this existing block:**

```rust
mod startup_tests;
pub mod suspension;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod startup_tests;
pub mod stroke;
#[cfg(test)]
mod stroke_tests;
pub mod suspension;
```

### 3. `src/stroke_tests.rs`

Copy source ownership, packed layout and invalid-display checks.

Create the file and type:

```rust
use crate::stroke::PreparedLine;
use session_rust::{Color, Line};
use std::rc::Rc;

#[test]
fn display_packing_preserves_the_original_source_owner_and_precision() {
    let mut line = Line::new(0.123456789012345, 0.0, 0.0, 1.0, 2.0, 3.0);
    line.linecolor = Color::new(0.2, 0.4, 0.6, 1.0);
    line.width = 3.0;
    let source = Rc::new(line);
    let weak = Rc::downgrade(&source);
    let first = PreparedLine::new(Rc::clone(&source)).unwrap();
    let second = PreparedLine::new(Rc::clone(&source)).unwrap();
    assert!(Rc::ptr_eq(&first.source, &second.source));
    assert_eq!(first.source.start()[0], 0.123456789012345);
    assert_ne!(
        first.display.start[0] as f64,
        first.source.start()[0]
    );
    assert_eq!(first.display.colour, [0.2, 0.4, 0.6, 1.0]);
    assert_eq!(first.display.width, 3.0);
    assert_eq!(first.display.bytes().len(), 44);
    assert_eq!(
        f32::from_ne_bytes(
            first.display.bytes()[40..44]
                .try_into()
                .unwrap()
        ),
        3.0
    );
    let guid = first.source.guid().to_owned();
    drop(first);
    drop(source);
    assert_eq!(second.source.guid(), guid);
    assert!(weak.upgrade().is_some());
    drop(second);
    assert!(weak.upgrade().is_none());
}

#[test]
fn default_widths_and_invalid_display_coordinates_follow_their_separate_rules() {
    for width in [0.0, -2.0, f64::NAN, f64::INFINITY, 1e-100] {
        let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        line.width = width;
        assert_eq!(
            PreparedLine::new(Rc::new(line))
                .unwrap()
                .display
                .width,
            1.0
        );
    }
    for value in [f64::NAN, f64::INFINITY, 1e100] {
        assert!(
            PreparedLine::new(Rc::new(Line::new(
                value, 0.0, 0.0, 1.0, 0.0, 0.0
            )))
            .is_err()
        );
    }
    let mut wide = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    wide.width = 1e100;
    assert!(PreparedLine::new(Rc::new(wide)).is_err());
}
```

## <a id="checkpoint-35a-document"></a>2. Retain line owners in document history

**What this code does:** Give prepared lines stable document IDs, history ownership and world-space bounds.

### 1. `src/scene.rs`

Retain exact line owners and apply placement before display conversion. Start the demo with no line rows. Include original placed line endpoints in Fit’s world bounds. Use the same checked ID sequence while keeping line placement separate from kernel geometry. Release line rows on Close and replacement.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
```

Replace it with:

```rust
    }

    pub fn clear(&mut self) {
        self.lines = Vec::new();
        self.objects = Vec::new();
```

**Locate this block:**

```rust
        Some(bounds)
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
```

Replace it with:

```rust
        Some(bounds)
    }

    pub fn lines(&self) -> &[LineObject] {
        &self.lines
    }

    pub fn insert_line(
        &mut self,
        prepared: crate::stroke::PreparedLine,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }

    pub fn place_line(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid line placement");
        }
        let row = self
            .lines
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Line not found")?;
        row.model = model;
        Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
```

**Locate this block:**

```rust
                }
            }
        }
        bounds
    }
```

Replace it with:

```rust
                }
            }
        }
        for line in &self.lines {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        bounds
    }
```

**Locate this block:**

```rust
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            objects: Vec::new(),
            next_id: 1,
            extra: None,
        };
```

Replace it with:

```rust
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            objects: Vec::new(),
            lines: Vec::new(),
            next_id: 1,
            extra: None,
        };
```

**Locate this block:**

```rust
#[derive(Clone)]
pub struct Scene {
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
```

Replace it with:

```rust
#[derive(Clone)]
pub struct LineObject {
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
}

impl LineObject {
    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [
            self.model.transform_point(&line.start()),
            self.model.transform_point(&line.end()),
        ]
    }

    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke {
            start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32),
            ..self.prepared.display
        }
    }
}

#[derive(Clone)]
pub struct Scene {
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
```

### 2. `src/document.rs`

Reject incomplete saves while the loader still supports only meshes.

**Locate this existing block:**

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    let mut objects = proto::Objects::default();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    if !scene.lines().is_empty() {
        return Err("This checkpoint cannot save line objects yet");
    }
    let mut objects = proto::Objects::default();
```

### 3. `src/lib.rs`

Register mixed row ownership checks.

**Locate this existing block:**

```rust
pub mod history;
pub mod load_measure;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod history;
#[cfg(test)]
mod line_scene_tests;
pub mod load_measure;
```

### 4. `src/line_scene_tests.rs`

Copy mixed IDs, placement, history and release checks.

Create the file and type:

```rust
use crate::{document, history::History, scene::Scene, stroke::PreparedLine};
use session_rust::{Line, Xform};
use std::rc::Rc;

#[test]
fn line_rows_keep_distinct_ids_original_sources_and_placed_bounds() {
    let mut scene = Scene::demo();
    let source = Rc::new(Line::new(
        2.123456789,
        0.0,
        0.0,
        4.0,
        1.0,
        2.0,
    ));
    let id = scene
        .insert_line(PreparedLine::new(Rc::clone(&source)).unwrap())
        .unwrap();
    assert!(
        scene
            .objects()
            .iter()
            .all(|row| row.id != id)
    );
    scene
        .place_line(id, Xform::translation(10.0, 0.0, 0.0))
        .unwrap();
    assert!((scene.lines()[0].endpoints()[0][0] - 12.123456789).abs() < 1e-12);
    assert_eq!(scene.bounds().unwrap().max[0], 14.0);
    assert_eq!(source.start()[0], 2.123456789);
    assert!(document::snapshot(&scene).is_err());
}

#[test]
fn history_shares_line_sources_and_close_releases_every_owner() {
    let mut scene = Scene::demo();
    let mut history = History::default();
    let source = Rc::new(Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0));
    let weak = Rc::downgrade(&source);
    let id = scene
        .insert_line(PreparedLine::new(source).unwrap())
        .unwrap();
    history
        .try_edit(&mut scene, |scene| {
            scene.place_line(id, Xform::translation(3.0, 0.0, 0.0))
        })
        .unwrap();
    assert!(Rc::ptr_eq(
        &scene.lines()[0].prepared.source,
        &history.scenes().next().unwrap().lines()[0]
            .prepared
            .source
    ));
    assert_eq!(scene.lines()[0].endpoints()[0][0], 3.0);
    assert!(history.undo(&mut scene));
    assert_eq!(scene.lines()[0].endpoints()[0][0], 0.0);
    assert!(history.redo(&mut scene));
    assert_eq!(scene.lines()[0].endpoints()[0][0], 3.0);
    scene.clear();
    assert!(weak.upgrade().is_some());
    history.clear();
    assert!(weak.upgrade().is_none());
    assert!(scene.lines().is_empty());
    assert!(scene.objects().is_empty());
    assert!(scene.bounds().is_none());
}
```

## <a id="checkpoint-35b-extrude"></a>3. Extrude a stroke in screen pixels

**What this code does:** Expand a projected segment into two triangles with a width that survives camera zoom.

### 1. `src/stroke.wgsl`

Extrude in physical pixels and interpolate coverage in screen space.

Create the file and type:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0);
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

### 2. `src/stroke.rs`

Expose the same shader text to the lane and its GPU check.

**Locate this existing block:**

```rust
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const SHADER: &str = include_str!("stroke.wgsl");
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
```

## <a id="checkpoint-35c-lane"></a>4. Create the shared stroke pipeline

**What this code does:** Create one pipeline and view uniform for a batch of stroke instances.

### 1. `src/stroke_gpu.rs`

Create one depth-tested instanced lane with alpha coverage and its own view binding.

Create the file and type:

```rust
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
    uniform: wgpu::Buffer,
    group: wgpu::BindGroup,
    vertices: Option<wgpu::Buffer>,
    data: Vec<u8>,
    pub uploads: u64,
    pub uploaded_bytes: u64,
}

impl Lane {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(crate::stroke::SHADER.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"), layout: None,
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: 44, step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32] }],
            },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL })],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false), depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: Default::default(), bias: Default::default() }),
            multisample: Default::default(), multiview_mask: None, cache: None,
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("stroke view"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("stroke view"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        Self {
            pipeline,
            uniform,
            group,
            vertices: None,
            data: Vec::new(),
            uploads: 0,
            uploaded_bytes: 0,
        }
    }
}
```

### 2. `src/lib.rs`

Register the shared GPU lane.

**Locate this existing block:**

```rust
pub mod stroke;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod stroke;
pub mod stroke_gpu;
#[cfg(test)]
```

## <a id="checkpoint-35ca-sync"></a>5. Upload changed stroke instances only

**What this code does:** Synchronize a stroke batch, draw it and retain vertex buffers through camera-only changes.

### 1. `src/stroke_gpu.rs`

Keep segment uploads separate from camera uniforms and release empty batches.

**Locate this existing block:**

```rust
impl Lane {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Lane {
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        strokes: impl IntoIterator<Item = crate::stroke::Stroke>,
    ) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        let mut data = Vec::new();
        for stroke in strokes {
            if stroke.width <= 0.0
                || stroke
                    .values()
                    .iter()
                    .any(|value| !value.is_finite())
            {
                return Err("Invalid stroke display data");
            }
            data.extend(stroke.bytes());
        }
        u32::try_from(data.len() / 44).map_err(|_| "Too many stroke instances")?;
        if data == self.data {
            return Ok(());
        }
        self.vertices = if data.is_empty() {
            None
        } else {
            self.uploads += 1;
            self.uploaded_bytes += data.len() as u64;
            Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("stroke instances"),
                    contents: &data,
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            )
        };
        self.data = data;
        Ok(())
    }

    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        if self.vertices.is_none() {
            return;
        }
        let bytes: Vec<u8> = transform
            .iter()
            .copied()
            .chain([size[0] as f32, size[1] as f32, density, 0.0])
            .flat_map(f32::to_ne_bytes)
            .collect();
        queue.write_buffer(&self.uniform, 0, &bytes);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vertices) = &self.vertices else {
            return;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..6, 0..(self.data.len() / 44) as u32);
    }

    pub fn usage(&self) -> [u64; 2] {
        [
            self.data.len() as u64 / 44,
            self.vertices
                .as_ref()
                .map_or(0, wgpu::Buffer::size),
        ]
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

## <a id="checkpoint-35cb-render"></a>6. Connect the stroke lane to the document renderer

**What this code does:** Draw document strokes with shared depth, correct device density and complete Close accounting.

### 1. `src/renderer.rs`

Retain one stroke lane and the physical view dimensions. Create the shared lane with the actual drawing device. Initialize native view size and default density. Keep physical viewport dimensions beside the shared depth texture. Reuse unchanged strokes and expose exact instance-buffer accounting. Include stroke payloads in measured scene upload bytes. Update camera and density separately from segment vertices. Use the same scene depth when drawing the stroke batch.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
        }
```

Replace it with:

```rust
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            self.strokes.draw(&mut pass);
        }
```

**Locate this block:**

```rust
            .collect();
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

Replace it with:

```rust
            .collect();
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

**Locate this block:**

```rust
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 80
            + self.settings_bytes as u64
    }

    pub fn stats(&self) -> [usize; 4] {
```

Replace it with:

```rust
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 80
            + self.settings_bytes as u64
            + self.strokes.uploaded_bytes
    }

    pub fn stats(&self) -> [usize; 4] {
```

**Locate this block:**

```rust
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects() {
```

Replace it with:

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

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [
            count,
            bytes,
            self.strokes.uploads,
            self.strokes.uploaded_bytes,
        ]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .map(crate::scene::LineObject::stroke),
            )
            .expect("Prepared line placement fits the display range");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects() {
```

**Locate this block:**

```rust
            uniform,
            view_group,
            depth,
        };
        renderer.set_scene(scene, None);
        renderer
```

Replace it with:

```rust
            uniform,
            view_group,
            depth,
            strokes,
            size: [640, 480],
            density: 1.0,
        };
        renderer.set_scene(scene, None);
        renderer
```

**Locate this block:**

```rust
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let mut renderer = Self {
            device,
            queue,
```

Replace it with:

```rust
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            device,
            queue,
```

**Locate this block:**

```rust
    depth: wgpu::TextureView,
}

impl Renderer {
```

Replace it with:

```rust
    depth: wgpu::TextureView,
    strokes: crate::stroke_gpu::Lane,
    size: [u32; 2],
    density: f32,
}

impl Renderer {
```

### 2. `src/browser.rs`

Use recovery’s effective density for both canvas size and stroke width. Size the canvas with the same effective density.

**Locate this existing block:**

```rust
        .max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        crate::browser_recovery::density(window.device_pixel_ratio()),
        limit,
    ) else {
        return false;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .max_texture_dimension_2d;
    let density = crate::browser_recovery::density(window.device_pixel_ratio());
    renderer.set_density(density as f32);
    let Some(size) = Viewport::from_css(rect.width(), rect.height(), density, limit)
    else {
        return false;
```

### 3. `src/memory.rs`

Count line rows, distinct kernel owners and allocated row capacity.

**Locate this existing block:**

```rust
}

pub fn gpu<'a>(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn lines<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 3] {
    let mut sources = HashSet::new();
    let mut count = [0; 3];
    for scene in scenes {
        count[0] += scene.lines().len();
        for row in scene.lines() {
            sources.insert(Rc::as_ptr(&row.prepared.source));
        }
        count[2] += scene.line_capacity_bytes();
    }
    count[1] = sources.len();
    count
}

pub fn gpu<'a>(
```

### 4. `src/scene.rs`

Expose actual line table capacity for CPU accounting. Reject unrenderable placed endpoints before changing document state.

**Locate this existing block:**

```rust
    }

    pub fn lines(&self) -> &[LineObject] {
        &self.lines
    }

    pub fn insert_line(
        &mut self,
        prepared: crate::stroke::PreparedLine,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }

    pub fn place_line(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid line placement");
        }
        let row = self
            .lines
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Line not found")?;
        row.model = model;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn line_capacity_bytes(&self) -> usize {
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn lines(&self) -> &[LineObject] {
        &self.lines
    }

    pub fn insert_line(
        &mut self,
        prepared: crate::stroke::PreparedLine,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }

    pub fn place_line(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid line placement");
        }
        let row = self
            .lines
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Line not found")?;
        for point in [
            row.prepared.source.start(),
            row.prepared.source.end(),
        ] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) {
                return Err("Line placement exceeds the display range");
            }
        }
        row.model = model;
```

### 5. `src/browser.rs`

Expose the actual new resource counters beside the existing inspector values.

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-selected-id",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-stroke-usage",
        &serde_json::to_string(&renderer.stroke_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute(
        "data-selected-id",
```

## <a id="checkpoint-35d-input"></a>7. Insert a line through the real command dock

**What this code does:** Insert an editable line as one undoable command and prove its width in actual browser pixels.

### 1. `src/editor.rs`

Add a document action for the example line. Prepare the original line, then commit insertion through existing history.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
```

Replace it with:

```rust
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0;
                line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared =
                    crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_line(prepared).map(|_| ())
                    })?;
```

**Locate this block:**

```rust
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
```

Replace it with:

```rust
    AddBox,
    AddLine,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
```

### 2. `src/browser.rs`

Expose the new action through the actual command list. Dispatch accepted input to the editor instead of directly to GPU data.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                } else {
                    let action = match line.as_str() {
                        "example box" => Action::AddBox,
                        "example triangle" => Action::ToggleExtra,
```

Replace it with:

```rust
                } else {
                    let action = match line.as_str() {
                        "example box" => Action::AddBox,
                        "example line" => Action::AddLine,
                        "example triangle" => Action::ToggleExtra,
```

**Locate this block:**

```rust
            "Example Box",
            "Example Triangle",
            "Select Next",
            "Delete",
```

Replace it with:

```rust
            "Example Box",
            "Example Line",
            "Example Triangle",
            "Select Next",
            "Delete",
```

### 3. `src/stroke_input_tests.rs`

Copy the source lifetime and undoable command checks.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    memory,
};

#[test]
fn line_command_is_one_transaction_and_close_releases_every_source() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddLine).unwrap();
    let row = &editor.scene.lines()[0];
    let id = row.id;
    let weak = std::rc::Rc::downgrade(&row.prepared.source);
    assert_eq!(row.prepared.source.start()[0], -0.6);
    assert_eq!(row.prepared.display.width, 5.0);
    editor.apply(Action::Zoom(2.0)).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.lines().is_empty());
    assert!(weak.upgrade().is_some());
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.lines()[0].id, id);
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::AddLine).unwrap();
    assert_ne!(editor.scene.lines()[0].id, id);
    assert!(weak.upgrade().is_none());
    editor.apply(Action::Close).unwrap();
    assert_eq!(memory::lines(editor.scenes()), [0; 3]);
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.lines().is_empty());
}
```

### 4. `src/lib.rs`

Register the command checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod stroke_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod stroke_input_tests;
#[cfg(test)]
mod stroke_tests;
```

## Build & Run

![Recorded visual output for Lesson 5.1 (35d-input)](../../../screenshots/journey/35d-input-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 5.1
npm --prefix ../session_tests run course -- save 5.1
```

Continue with [5.2 · Joined paths and headed ends](../../consolidation/36-chain.md).
