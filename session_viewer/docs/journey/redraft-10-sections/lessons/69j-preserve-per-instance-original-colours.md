# 13.2 · Independent presentation and reusable CAD preparation

**Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** · **Lesson 13.2** (Redraft `7.6`) · **15 checkpoints (`69j-preserve-per-instance-original-colours` → `69af-exercise-independent-original-cad-reuse`)**

> **Learning Goal:** Preserve per instance original colours.

## Visual WebGPU & Architecture Schematic

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1px solid #cbd5e1;border-radius:10px;background:#f8fafc;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#475569"/>
    </marker>
    <marker id="arrow-blue" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <marker id="arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#059669"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Section 7 WebGPU Streaming & Instancing: HTTP Range Requests, Shared Source Meshes & 2D Drawing Sheets</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How massive multi-gigabyte scenes stream out-of-core via HTTP 206 Partial Content and draw 1,000+ placements from 1 GPU buffer.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. HTTP 206 Range Streaming (Ch 12)</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• Fetch compact Manifest + World Bounds</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• Prioritize visible frustum chunks</text>
    <text x="28" y="144" font-size="10.5" fill="#1e3a8a">• Range: bytes=start-end (bounded memory)</text>
    <text x="28" y="162" font-size="10.5" fill="#1e3a8a">• Evict cold off-screen chunks under</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">  memory budget without losing metadata</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="295" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. GPU Hardware Instancing (Ch 13)</text>
    <text x="307" y="108" font-size="10.5" fill="#7c2d12">• 1 Shared Definition Buffer (e.g. Bolt)</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#c2410c">• N Instance Rows (@builtin(instance_index)):</text>
    <text x="307" y="142" font-size="10" fill="#7c2d12">  Per-instance mat4x4 transform, tint,</text>
    <text x="307" y="157" font-size="10" fill="#7c2d12">  and distinct instance_owner_id</text>
    <text x="307" y="176" font-size="10.5" fill="#7c2d12">• draw_indexed(0..idx_cnt, 0, 0..N_inst)</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="592" y="68" width="228" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. 2D Drawing Sheets &amp; Blocks</text>
    <text x="604" y="108" font-size="10.5" fill="#064e3b">• Paper-space Sheet frames &amp; title</text>
    <text x="604" y="124" font-size="10.5" fill="#064e3b">  blocks (`SheetInit / SheetRows`)</text>
    <text x="604" y="144" font-size="10.5" font-weight="600" fill="#047857">• Calibrated mm/inch pen widths</text>
    <text x="604" y="162" font-size="10.5" fill="#064e3b">• Switch smoothly between 3D Model</text>
    <text x="604" y="178" font-size="10.5" fill="#064e3b">  view and 2D Sheet layouts</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
viewer colour override → original instance colour → shared definition colour.
original source validation → derived values → scoped recording.
two original BReps → one walk and replay → independent commands → unchanged saved sources.
```

- Preserve per instance original colours.
- Encode knots, local points, pcurve values and curve descriptors.
- Exercise independent original CAD reuse.
- **Why it works (Does recolouring one placement alter the definition?):** No. Resolve its colour in the draw settings. Preserve the original definition record and every other placement.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **HTTP / Fetch** | `Range: bytes=start-end` -> `206 Partial Content` | Fetches only the exact byte slice of an octree node or sheet column needed for the current camera view. |
| **Protobuf Wire** | Varint tag + length prefix scanner | Skips over multi-megabyte coordinate arrays by reading their byte range offsets without allocating their contents. |
| **Rust (Residency)** | `budget.reserve(bytes)?` -> LRU eviction | Guarantees resident + in-flight pages never exceed the memory cap; coarse root pages stay pinned so the scene is never blank. |
| **GPU Instancing** | Shared definition buffer + per-instance record | Draws N placed copies of a complex CAD component using 1 shared vertex/index allocation and N compact instance records. |

**Production source (`session_viewer/src`):** [app/scene_instances.rs](../../../../src/app/scene_instances.rs), [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs), [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs), [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs), [app/hierarchy.rs](../../../../src/app/hierarchy.rs), [app/sheet_query.rs](../../../../src/app/sheet_query.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-69j-preserve-per-instance-original-colours"></a>

<details>
<summary>Implementation 69j-preserve-per-instance-original-colours: Preserve per instance original colours</summary>

Preserve per instance original colours.

Data flow: viewer colour override → original instance colour → shared definition colour.

### 1. `src/gpu_mesh.rs`

Resolve instance colours without mutating original shared geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        Self { geometry, model_group, id: object.id, model, previous: bytes }
    }

    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())!=0}

    pub fn settings_bytes(&self) -> u64 { self.model.size() }
```

</details>

Replace it with:

```rust
        Self { geometry, model_group, id: object.id, model, previous: bytes }
    }

    pub(crate) fn packed(&self)->[u8;160] {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())!=0}

    pub fn settings_bytes(&self) -> u64 { self.model.size() }
```

<details>
<summary>Locate this block</summary>

```rust
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour.into_iter().map(|v|v as f32/255.).chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes
}
```

</details>

Replace it with:

```rust
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes
}
```

<details>
<summary>Locate this block</summary>

```rust
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[u8;3]>}

fn settings(object: &Object, style:Style) -> [u8; 160] {
    let mut bytes = [0; 160];
```

</details>

Replace it with:

```rust
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style) -> [u8; 160] {
    let mut bytes = [0; 160];
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct GpuMesh {
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: [u8; 160],
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style) -> [u8; 160] {
    let mut bytes = [0; 160];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes
}

impl GpuMesh {
    pub fn upload(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, object: &Object, selected: bool) -> Self {
        let geometry = Rc::new(GpuGeometry::upload(device, Rc::clone(&object.mesh)));
        Self::with_geometry(device, layout, object, Style {selected,pickable:!object.metadata.is_locked,colour:None}, geometry)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        let bytes = settings(object, style);
        let model = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("object placement"),
            contents: &bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let model_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("object placement"),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { geometry, model_group, id: object.id, model, previous: bytes }
    }

    pub(crate) fn packed(&self)->[u8;160] {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())!=0}

    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,160)] {
            if next[start..end] != self.previous[start..end] {
                queue.write_buffer(&self.model, start as u64, &next[start..end]);
                writes[0] += 1;
                writes[1] += end - start;
            }
        }
        self.previous = next;
        writes
    }

    pub fn bind_model(&self, pass: &mut wgpu::RenderPass<'_>) { pass.set_bind_group(1, &self.model_group, &[]); }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(1, &self.model_group, &[]);
        self.geometry.draw(pass);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69j-preserve-per-instance-original-colours-01.rs"
```

```text
--8<-- "journey/code/69j-preserve-per-instance-original-colours-02.rs"
```

```text
--8<-- "journey/code/69j-preserve-per-instance-original-colours-03.rs"
```

</details>

### 2. `src/cloud_gpu.rs`

Resolve instance colours without mutating original shared geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    model[0], model[1], model[2], model[12], model[4], model[5], model[6], model[13], model[8], model[9], model[10],
    model[14],
  ];
  let rgb = draw.color.map_or([1., 1., 1.], |c| c.map(|v| v as f32 / 255.));
  let px = row.point_px;
  let flags = u32::from(draw.selected) | if draw.color.is_some() { 256 } else { 0 };
  let instance = if draw.pickable { row.id.value() - 1 } else { u32::MAX };
```

</details>

Replace it with:

```rust
    model[0], model[1], model[2], model[12], model[4], model[5], model[6], model[13], model[8], model[9], model[10],
    model[14],
  ];
  let rgb = draw.color.map_or([1., 1., 1.], |c| [c[0],c[1],c[2]]);
  let px = row.point_px;
  let flags = u32::from(draw.selected) | if draw.color.is_some() { 256 } else { 0 };
  let instance = if draw.pickable { row.id.value() - 1 } else { u32::MAX };
```

<details>
<summary>Locate this block</summary>

```rust
        let records=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("bounded cloud draw records"),size:16,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
        let group=resource_group(ctx,&self.columns_layout,"cloud source and records",[(0,records.as_entire_binding()),(1,columns.positions.as_entire_binding()),(2,columns.colors.as_entire_binding()),(3,columns.normals.as_entire_binding())]);(records,group)
      };
      let id=row.id;draws.push(Draw {row,columns,records,group,visible:scene.visible(id),pickable:scene.pick_id(id.value()).is_some(),selected:selected.contains(&id),color:scene.view.faces.get(&id).copied(),total:0});
    }
    self.draws = draws;
    self.revision = self.revision.wrapping_add(1);
```

</details>

Replace it with:

```rust
        let records=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("bounded cloud draw records"),size:16,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
        let group=resource_group(ctx,&self.columns_layout,"cloud source and records",[(0,records.as_entire_binding()),(1,columns.positions.as_entire_binding()),(2,columns.colors.as_entire_binding()),(3,columns.normals.as_entire_binding())]);(records,group)
      };
      let id=row.id;draws.push(Draw {row,columns,records,group,visible:scene.visible(id),pickable:scene.pick_id(id.value()).is_some(),selected:selected.contains(&id),color:scene.face_rgba(id),total:0});
    }
    self.draws = draws;
    self.revision = self.revision.wrapping_add(1);
```

<details>
<summary>Locate this block</summary>

```rust
  visible:bool,
  pickable:bool,
  selected:bool,
  color:Option<[u8;3]>,
  total:u32,
}
struct Image {
```

</details>

Replace it with:

```rust
  visible:bool,
  pickable:bool,
  selected:bool,
  color:Option<[f32;4]>,
  total:u32,
}
struct Image {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{
  cloud::Row,
  cloud_lod::{LodWalk, Projection},
  text_gpu_support::{GpuCtx, resource_group},
};
use std::{collections::HashMap, rc::Rc};
use wgpu::util::DeviceExt;
const MAX_RECORDS: usize = 4096;

#[derive(Clone, Copy)]
pub struct View {
  pub eye: [f32; 3],
  pub ortho_h: f32,
  pub lod_px: f32,
}
impl Default for View {
  fn default() -> Self {
    Self { eye: [0., 0., 3.], ortho_h: 0., lod_px: 2. }
  }
}
impl View {
  pub fn camera(camera: &crate::camera::Camera) -> Self {
    let forward = camera.orientation.rotate_vector(session_rust::Vector::y_axis());
    Self {
      eye: std::array::from_fn(|i| (camera.target[i] - forward[i] * camera.distance) as f32),
      ortho_h: if camera.projection == crate::camera::Projection::Orthographic {
        (camera.distance * std::f64::consts::FRAC_PI_6.tan()) as f32
      } else {
        0.
      },
      lod_px: 2.,
    }
  }
}
struct Columns {
  _page:Option<Rc<crate::page_cache::Held<crate::range_page::Page>>>,
  positions: wgpu::Buffer,
  colors: wgpu::Buffer,
  normals: wgpu::Buffer,
  has_normals: bool,
}
impl Columns {
  fn new(ctx: &GpuCtx, row: &Row) -> Result<Self, &'static str> {
    let source = &row.prepared.source;
    let count = source.len();
    let limit = ctx.device.limits().max_storage_buffer_binding_size as usize;
    if count.checked_mul(12).is_none_or(|n| n > limit) {
      return Err("Cloud positions exceed this device's storage binding limit");
    }
    let positions = source.coords().iter().flat_map(|v| (*v as f32).to_ne_bytes()).collect::<Vec<_>>();
    let colors = (0..count)
      .map(|i| {
        if source.colors().is_empty() {
          0xff00_0000
        } else {
          let c = &source.colors()[i * 4..i * 4 + 4];
          (c[0] as u32) | (c[1] as u32) << 8 | (c[2] as u32) << 16 | (c[3] as u32) << 24
        }
      })
      .flat_map(u32::to_ne_bytes)
      .collect::<Vec<_>>();
    let has_normals = !source.normals().is_empty();
    let normals = source
      .normals()
      .chunks_exact(3)
      .map(|n| crate::cloud_normals::oct16(&[n[0], n[1], n[2]]).unwrap_or(0))
      .flat_map(u32::to_ne_bytes)
      .collect::<Vec<_>>();
    Ok(Self {
      _page:None,
      positions: buffer(&ctx.device, "original cloud positions", &positions),
      colors: buffer(&ctx.device, "original cloud colors", &colors),
      normals: buffer(&ctx.device, "original cloud normals", &normals),
      has_normals,
    })
  }
  fn page(ctx:&GpuCtx,held:Rc<crate::page_cache::Held<crate::range_page::Page>>)->Result<Self,&'static str> {
    let page=&held.value;let count=page.rows.end-page.rows.start;
    if u64::from(count)*12>u64::from(ctx.device.limits().max_storage_buffer_binding_size) {return Err("Cloud page exceeds the device storage binding limit");}
    let positions=page.display.iter().flat_map(|value|value.to_ne_bytes()).collect::<Vec<_>>();let colors=(0..count as usize).flat_map(|i|page.colors.get(i).copied().unwrap_or(0xff00_0000).to_ne_bytes()).collect::<Vec<_>>();let normals=page.normals.iter().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();
    Ok(Self {positions:buffer(&ctx.device,"streamed source positions",&positions),colors:buffer(&ctx.device,"streamed source colours",&colors),normals:buffer(&ctx.device,"streamed source normals",&normals),has_normals:!page.normals.is_empty(),_page:Some(held)})
  }
  fn bytes(&self) -> u64 {
    self.positions.size() + self.colors.size() + self.normals.size()
  }
}
fn buffer(device: &wgpu::Device, label: &str, bytes: &[u8]) -> wgpu::Buffer {
  device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some(label),
    contents: if bytes.is_empty() { &[0; 4] } else { bytes },
    usage: wgpu::BufferUsages::STORAGE,
  })
}
enum Keep {Resident(Rc<session_rust::PointCloud>),Stream(Rc<crate::range_cloud::Prepared>)}
impl Keep {
  fn key(&self,first:u32)->(usize,u32) {match self {Self::Resident(source)=>(Rc::as_ptr(source) as usize,first),Self::Stream(source)=>(Rc::as_ptr(source) as usize,first)}}
}
struct RowView {
  keep:Keep,
  id:crate::scene::ObjectId,
  model:session_rust::Xform,
  nodes:Rc<Vec<crate::cloud::LodNode>>,
  descriptor:crate::cloud::Cloud,
  controls_visible:bool,
  point_px:f32,
  rows:std::ops::Range<u32>,
  address:u32,
}
impl RowView {
  fn resident(row:&Row)->Self {Self {keep:Keep::Resident(row.prepared.source.clone()),id:row.id,model:row.model.clone(),nodes:row.prepared.nodes.clone(),descriptor:row.prepared.descriptor(),controls_visible:row.controls_visible,point_px:if row.point_px>0. {row.point_px} else {row.prepared.source.point_size as f32},rows:0..row.prepared.source.len() as u32,address:0}}
  fn streamed(row:&crate::range_cloud::Row,rows:std::ops::Range<u32>,address:u32)->Self {Self {keep:Keep::Stream(row.prepared.clone()),id:row.id,model:row.model.clone(),nodes:row.prepared.nodes.clone(),descriptor:crate::cloud::Cloud {spacing:row.prepared.source.lod.spacing[0] as f32,node_first:0,node_count:row.prepared.nodes.len() as u32,resident:row.prepared.source.fields.count},controls_visible:row.controls_visible,point_px:if row.point_px>0. {row.point_px} else {row.prepared.source.properties.point_px},rows,address}}
  fn key(&self)->(usize,u32,u32) {let (source,first)=self.keep.key(self.rows.start);(source,first,self.rows.end)}
}
struct Draw {
  row:RowView,
  columns:Rc<Columns>,
  records:wgpu::Buffer,
  group:wgpu::BindGroup,
  visible:bool,
  pickable:bool,
  selected:bool,
  color:Option<[f32;4]>,
  total:u32,
}
struct Image {
  texture: wgpu::Texture,
  view: wgpu::TextureView,
  bytes: u64,
}
impl Image {
  fn new(device: &wgpu::Device, size: [u32; 2], format: wgpu::TextureFormat, bytes: u64) -> Self {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: Some("cached cloud image"),
      size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    Self { texture, view, bytes: bytes * size[0] as u64 * size[1] as u64 }
  }
}
impl Drop for Image {
  fn drop(&mut self) {
    self.texture.destroy();
  }
}
struct Targets {
  color: Image,
  depth: Image,
  ids: Image,
  size: [u32; 2],
  group: wgpu::BindGroup,
}
#[derive(PartialEq)]
struct Key {
  transform: [f32; 16],
  size: [u32; 2],
  density: u32,
  eye: [u32; 3],
  ortho: u32,
  lod: u32,
  revision: u64,
}

pub struct Lane {
  point: wgpu::RenderPipeline,
  resolve: wgpu::RenderPipeline,
  pick: wgpu::RenderPipeline,
  settings: wgpu::Buffer,
  settings_group: wgpu::BindGroup,
  columns_layout: wgpu::BindGroupLayout,
  image_layout: wgpu::BindGroupLayout,
  window: wgpu::Buffer,
  window_group: wgpu::BindGroup,
  draws: Vec<Draw>,
  targets: Option<Targets>,
  key: Option<Key>,
  revision: u64,
  pub uploads: u64,
  pub preparations: u64,
  pub skips: u64,
  pub total: u64,
}
impl Lane {
  pub fn new(ctx: &GpuCtx, format: wgpu::TextureFormat) -> Self {
    let layouts = layouts(ctx);
    let make_buffer = |label, size| {
      ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
      })
    };
    let settings = make_buffer("cloud frame settings", 48);
    let settings_group =
      resource_group(ctx, &layouts.uniform, "cloud frame settings", [(0, settings.as_entire_binding())]);
    let window = make_buffer("cloud pick origin", 16);
    let window_group = resource_group(ctx, &layouts.window, "cloud pick origin", [(0, window.as_entire_binding())]);
    let [point, resolve, pick] = pipelines(ctx, format, &layouts);
    Self {
      point,
      resolve,
      pick,
      settings,
      settings_group,
      columns_layout: layouts.columns,
      image_layout: layouts.image,
      window,
      window_group,
      draws: Vec::new(),
      targets: None,
      key: None,
      revision: 0,
      uploads: 0,
      preparations: 0,
      skips: 0,
      total: 0,
    }
  }
  pub fn set_scene(
    &mut self,
    ctx: &GpuCtx,
    scene: &crate::scene::Scene,
    selected: &[crate::scene::ObjectId],
  ) -> Result<(), &'static str> {
    let mut cache:HashMap<(usize,u32,u32),Rc<Columns>>=self.draws.iter().map(|draw|(draw.row.key(),draw.columns.clone())).collect();
    let mut candidates=Vec::new();
    for row in scene.clouds() {let view=RowView::resident(row);let key=view.key();let columns=if let Some(columns)=cache.get(&key) {columns.clone()} else {let columns=Rc::new(Columns::new(ctx,row)?);self.uploads+=1;cache.insert(key,columns.clone());columns};candidates.push((view,columns));}
    for row in scene.streams() {let mut address=0u32;for page in row.prepared.pages() {
      let view=RowView::streamed(row,page.value.rows.clone(),address);let key=view.key();let columns=if let Some(columns)=cache.get(&key) {columns.clone()} else {let columns=Rc::new(Columns::page(ctx,page.clone())?);self.uploads+=1;cache.insert(key,columns.clone());columns};address=address.checked_add(page.value.rows.end-page.value.rows.start).filter(|value|*value<=crate::sub_id::LIMIT).ok_or("Streamed pick addresses exceed their source table")?;candidates.push((view,columns));
    }}
    let mut old=std::mem::take(&mut self.draws);let mut draws=Vec::with_capacity(candidates.len());
    for (row,columns) in candidates {
      let (records,group)=if let Some(i)=old.iter().position(|draw|draw.row.id==row.id&&draw.row.rows==row.rows&&Rc::ptr_eq(&draw.columns,&columns)) {let draw=old.swap_remove(i);(draw.records,draw.group)} else {
        let records=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("bounded cloud draw records"),size:16,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
        let group=resource_group(ctx,&self.columns_layout,"cloud source and records",[(0,records.as_entire_binding()),(1,columns.positions.as_entire_binding()),(2,columns.colors.as_entire_binding()),(3,columns.normals.as_entire_binding())]);(records,group)
      };
      let id=row.id;draws.push(Draw {row,columns,records,group,visible:scene.visible(id),pickable:scene.pick_id(id.value()).is_some(),selected:selected.contains(&id),color:scene.face_rgba(id),total:0});
    }
    self.draws = draws;
    self.revision = self.revision.wrapping_add(1);
    self.key = None;
    Ok(())
  }
  fn records(
    &mut self,
    ctx: &GpuCtx,
    transform: [f32; 16],
    size: [u32; 2],
    density: f32,
    view: View,
  ) -> Result<(), &'static str> {
    let mut total = 0u64;
    let mut records_used = 0usize;
    let mut walk = LodWalk::default();
    for draw in &mut self.draws {
      draw.total = 0;
      if !draw.visible {
        continue;
      }
      let row = &draw.row;
      let model = row.model.to_f32();
      let projection = Projection {
        eye: view.eye,
        ortho_h: view.ortho_h,
        height_px: size[1],
        lod_px: if row.controls_visible {0.} else {view.lod_px},
        nodes: &row.nodes,
      };
      walk.select(&projection, &row.descriptor, &model);
      walk.ranges.retain_mut(|range| {let start=range.first.max(row.rows.start);let end=(range.first+range.count).min(row.rows.end);range.first=start;range.count=end.saturating_sub(start);range.count>0});
      records_used += walk.ranges.len();
      if records_used > MAX_RECORDS {
        return Err("Cloud view exceeds the 4096-record budget");
      }
      let (bytes, count) = record_bytes(draw, transform, density, view, &walk.ranges)?;
      draw.total = count;
      if bytes.len() as u64 > draw.records.size() {
        let capacity = walk.ranges.len().next_power_of_two().min(MAX_RECORDS);
        draw.records = ctx.device.create_buffer(&wgpu::BufferDescriptor {
          label: Some("bounded cloud draw records"),
          size: 16 + 160 * capacity as u64,
          usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
          mapped_at_creation: false,
        });
        draw.group = resource_group(
          ctx,
          &self.columns_layout,
          "cloud source and records",
          [
            (0, draw.records.as_entire_binding()),
            (1, draw.columns.positions.as_entire_binding()),
            (2, draw.columns.colors.as_entire_binding()),
            (3, draw.columns.normals.as_entire_binding()),
          ],
        );
      }
      ctx.queue.write_buffer(&draw.records, 0, &bytes);
      total += draw.total as u64;
    }
    self.total = total;
    Ok(())
  }
  fn render_points(&self, ctx: &GpuCtx, size: [u32; 2]) {
    let settings = [1., size[0] as f32, size[1] as f32, 0., 0., 0., 0., 0., size[0] as f32, size[1] as f32, 0., 0.];
    ctx.queue.write_buffer(&self.settings, 0, &settings.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());
    let t = self.targets.as_ref().unwrap();
    let mut encoder = ctx.device.create_command_encoder(&Default::default());
    {
      let color = |view| {
        Some(wgpu::RenderPassColorAttachment {
          view,
          depth_slice: None,
          resolve_target: None,
          ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
        })
      };
      let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("cached original cloud discs"),
        color_attachments: &[color(&t.color.view), color(&t.ids.view)],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
          view: &t.depth.view,
          depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.), store: wgpu::StoreOp::Store }),
          stencil_ops: None,
        }),
        ..Default::default()
      });
      pass.set_pipeline(&self.point);
      pass.set_bind_group(0, &self.settings_group, &[]);
      for draw in &self.draws {
        if draw.total > 0 {
          pass.set_bind_group(1, &draw.group, &[]);
          pass.draw(0..6 * draw.total, 0..1);
        }
      }
    }
    ctx.queue.submit([encoder.finish()]);
  }
  pub fn prepare(
    &mut self,
    ctx: &GpuCtx,
    transform: [f32; 16],
    size: [u32; 2],
    density: f32,
    view: View,
  ) -> Result<(), &'static str> {
    let key = Key {
      transform,
      size,
      density: density.to_bits(),
      eye: view.eye.map(f32::to_bits),
      ortho: view.ortho_h.to_bits(),
      lod: view.lod_px.to_bits(),
      revision: self.revision,
    };
    if self.key.as_ref() == Some(&key) {
      self.skips += 1;
      return Ok(());
    }
    self.records(ctx, transform, size, density, view)?;
    if self.targets.as_ref().is_none_or(|t| t.size != size) {
      let color = Image::new(&ctx.device, size, wgpu::TextureFormat::Rgba8Unorm, 4);
      let depth = Image::new(&ctx.device, size, crate::depth::FORMAT, 4);
      let ids = Image::new(&ctx.device, size, wgpu::TextureFormat::Rg32Uint, 8);
      let group = resource_group(
        ctx,
        &self.image_layout,
        "cached cloud pixels",
        [
          (0, wgpu::BindingResource::TextureView(&depth.view)),
          (1, wgpu::BindingResource::TextureView(&color.view)),
          (2, wgpu::BindingResource::TextureView(&ids.view)),
        ],
      );
      self.targets = Some(Targets { color, depth, ids, size, group });
    }
    self.render_points(ctx, size);
    self.key = Some(key);
    self.preparations += 1;
    Ok(())
  }
  pub fn failed(&mut self) {
    self.targets = None;
    self.key = None;
    self.total = 0;
  }
  pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
    let Some(t) = &self.targets else {
      return;
    };
    pass.set_pipeline(&self.resolve);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.draw(0..3, 0..1);
  }
  pub fn pick(
    &self,
    ctx: &GpuCtx,
    pass: &mut wgpu::RenderPass<'_>,
    window: crate::pick_window::Window,
    mode: crate::pick_sources::Mode,
  ) {
    let Some(t) = &self.targets else {
      return;
    };
    let words = [window.draw[0], window.draw[1], mode as u32, 0];
    ctx.queue.write_buffer(&self.window, 0, &words.into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
    pass.set_pipeline(&self.pick);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.draw(0..3, 0..1);
  }
  pub fn usage(&self) -> [u64; 5] {
    let mut seen = std::collections::HashSet::new();
    let columns =
      self.draws.iter().filter(|d| seen.insert(Rc::as_ptr(&d.columns))).map(|d| d.columns.bytes()).sum::<u64>();
    let targets = self.targets.as_ref().map_or(0, |t| t.color.bytes + t.depth.bytes + t.ids.bytes);
    [
      self.total,
      columns
        + self.draws.iter().map(|d| d.records.size()).sum::<u64>()
        + targets
        + self.settings.size()
        + self.window.size(),
      self.uploads,
      self.preparations,
      self.skips,
    ]
  }
}
fn pipeline(
  device: &wgpu::Device,
  shader: &wgpu::ShaderModule,
  groups: &[&wgpu::BindGroupLayout],
  vertex: &str,
  fragment: &str,
  formats: &[wgpu::TextureFormat],
) -> wgpu::RenderPipeline {
  let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some(fragment),
    bind_group_layouts: &groups.iter().map(|g| Some(*g)).collect::<Vec<_>>(),
    immediate_size: 0,
  });
  device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    label: Some(fragment),
    layout: Some(&layout),
    vertex: wgpu::VertexState {
      module: shader,
      entry_point: Some(vertex),
      buffers: &[],
      compilation_options: Default::default(),
    },
    fragment: Some(wgpu::FragmentState {
      module: shader,
      entry_point: Some(fragment),
      targets: &formats.iter().copied().map(|format| Some(format.into())).collect::<Vec<_>>(),
      compilation_options: Default::default(),
    }),
    primitive: Default::default(),
    depth_stencil: Some(wgpu::DepthStencilState {
      format: crate::depth::FORMAT,
      depth_write_enabled: Some(true),
      depth_compare: Some(wgpu::CompareFunction::LessEqual),
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample: Default::default(),
    multiview_mask: None,
    cache: None,
  })
}

struct Layouts {
  uniform: wgpu::BindGroupLayout,
  window: wgpu::BindGroupLayout,
  columns: wgpu::BindGroupLayout,
  image: wgpu::BindGroupLayout,
}
fn layouts(ctx: &GpuCtx) -> Layouts {
  let uniform = crate::gpu_layout::uniform(&ctx.device, 48);
  let window_layout = crate::gpu_layout::uniform(&ctx.device, 16);
  let storage = |binding| wgpu::BindGroupLayoutEntry {
    binding,
    visibility: wgpu::ShaderStages::VERTEX,
    ty: wgpu::BindingType::Buffer {
      ty: wgpu::BufferBindingType::Storage { read_only: true },
      has_dynamic_offset: false,
      min_binding_size: None,
    },
    count: None,
  };
  let columns_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some("cloud source columns and bounded records"),
    entries: &[storage(0), storage(1), storage(2), storage(3)],
  });
  let texture = |binding, sample_type| {
    crate::text_gpu_support::texture_entry(binding, wgpu::ShaderStages::FRAGMENT, sample_type, false)
  };
  let image_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some("cached cloud images"),
    entries: &[
      texture(0, wgpu::TextureSampleType::Depth),
      texture(1, wgpu::TextureSampleType::Float { filterable: false }),
      texture(2, wgpu::TextureSampleType::Uint),
    ],
  });
  Layouts { uniform, window: window_layout, columns: columns_layout, image: image_layout }
}
fn pipelines(ctx: &GpuCtx, format: wgpu::TextureFormat, layouts: &Layouts) -> [wgpu::RenderPipeline; 3] {
  let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("current production cloud discs"),
    source: wgpu::ShaderSource::Wgsl(include_str!("cloud_points.wgsl").into()),
  });
  let resolver = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("forward-depth cloud resolve"),
    source: wgpu::ShaderSource::Wgsl(include_str!("cloud_resolve.wgsl").into()),
  });
  let point = pipeline(
    &ctx.device,
    &shader,
    &[&layouts.uniform, &layouts.columns],
    "vs_point",
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window], "vertex", "color", &[format]);
  let pick = pipeline(
    &ctx.device,
    &resolver,
    &[&layouts.image, &layouts.window],
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint],
  );
  [point, resolve, pick]
}

fn record_bytes(
  draw: &Draw,
  transform: [f32; 16],
  density: f32,
  view: View,
  ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
  let row = &draw.row;
  let model = row.model.to_f32();
  let scale = row.model.uniform_scale();
  let mut total = 0u32;
  let m = (&session_rust::Xform::from_matrix(transform.map(f64::from)) * &row.model).to_f32();
  let rot = [
    model[0], model[1], model[2], model[12], model[4], model[5], model[6], model[13], model[8], model[9], model[10],
    model[14],
  ];
  let rgb = draw.color.map_or([1., 1., 1.], |c| [c[0],c[1],c[2]]);
  let px = row.point_px;
  let flags = u32::from(draw.selected) | if draw.color.is_some() { 256 } else { 0 };
  let instance = if draw.pickable { row.id.value() - 1 } else { u32::MAX };
  let mut bytes = vec![0; 16];
  for range in ranges {
    let cumulative = total;
    total = total
      .checked_add(range.count)
      .filter(|n| *n <= u32::MAX / 6)
      .ok_or("Cloud vertex count exceeds the draw limit")?;
    let words = m
      .map(f32::to_bits)
      .into_iter()
      .chain([rgb[0].to_bits(), rgb[1].to_bits(), rgb[2].to_bits(), (px * density * 0.5).max(0.5).to_bits()])
      .chain([range.first-row.rows.start, range.count, cumulative, radius_factor(range, px, scale, view.ortho_h).to_bits()])
      .chain(rot.map(f32::to_bits))
      .chain([if draw.columns.has_normals { range.first-row.rows.start } else { u32::MAX }, instance, flags, row.address]);
    bytes.extend(words.flat_map(u32::to_ne_bytes));
  }
  bytes[..4].copy_from_slice(&(ranges.len() as u32).to_ne_bytes());
  bytes[4..8].copy_from_slice(&total.to_ne_bytes());
  Ok((bytes, total))
}

pub fn radius_factor(range: &crate::cloud_lod::Range, px: f32, scale: f64, ortho_h: f32) -> f32 {
  crate::cloud_lod::radius_factor(range, px, scale, ortho_h) * if ortho_h > 0. { 1. } else { 1000. }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69j-preserve-per-instance-original-colours-04.rs"
```

```text
--8<-- "journey/code/69j-preserve-per-instance-original-colours-05.rs"
```

```text
--8<-- "journey/code/69j-preserve-per-instance-original-colours-06.rs"
```

</details>

### 3. `src/renderer.rs`

Apply per-instance original colour to mesh, point, path and line lanes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.view.faces.get(&id).copied()}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}
```

</details>

Replace it with:

```rust
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}
```

<details>
<summary>Locate this block</summary>

```rust
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker();if let Some(colour)=scene.view.faces.get(&row.id) {marker.colour=rgba(*colour);}marker}).chain(scene.control_markers().into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments();if let Some(colour)=scene.view.faces.get(&row.id) {for segment in &mut segments {segment.stroke.colour=rgba(*colour);}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke();if let Some(colour)=scene.view.faces.get(&row.id) {stroke.colour=rgba(*colour);}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
```

</details>

Replace it with:

```rust
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker();if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers().into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments();if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke();if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
    cloud_error:std::cell::RefCell<Option<String>>,
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode: crate::pick_sources::Mode,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, include_str!("cloud_points.wgsl").into(), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 160);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout)], immediate_size: 0 });
        let mesh_pipeline = |depth| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface());
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.picker.cancel(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.picker.cancel(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker();if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers().into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments();if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke();if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request { window, transform, density: self.density, mode: self.pick_mode })?;
        self.pump_pick(transform); Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density) else { self.picker.cancel(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) { self.picker.submit(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref()); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density) { Ok(view) => view, Err(error) => {
            self.picker.cancel(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => { self.pump_pick(transform); None },
            Some(Err(error)) => Some(Err(error)), None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.picker.cancel(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) { self.picker.cancel(); }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] { crate::memory::gpu(&self.meshes) }

    pub fn uploaded_bytes(&self) -> u64 {
        self.geometry.uploaded_bytes + self.settings_allocations as u64 * 160 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self, settings: Option<crate::grid::Settings>) -> Result<(), &'static str> {
        if settings == self.grid_settings { return Ok(()); }
        let strokes = settings.map(crate::grid::Settings::strokes).transpose()?.unwrap_or_default();
        self.grid.set(&self.device, strokes)?; self.grid_settings = settings; Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let matrix=camera.view_projection();
        let layout=origin.and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:[0.;3],framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:[[0.;4];4]}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(merged)?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(merged)?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene) {
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {row.prepared.extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);}}}
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::camera(camera));}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        let display = [if normals { 1.0f32 } else { 0.0 }, 0.0, 0.0, 0.0];
        bytes.extend(display.iter().flat_map(|value| value.to_ne_bytes()));
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r, g, b, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            if let Some(clouds)=self.clouds.borrow().as_ref() {clouds.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        if self.meshes.iter().any(GpuMesh::selected) {
            let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density);
        } else {self.outline.borrow_mut().take();}
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.queue.submit([encoder.finish()]);
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}

#[cfg(all(test,not(target_arch="wasm32")))]
impl Renderer {
 pub fn outline_alpha(&self)->Vec<u8> {
  let owner=self.outline.borrow();let Some(outline)=owner.as_ref() else {return Vec::new();};let Some(texture)=outline.alpha_texture() else {return Vec::new();};let [width,height]=self.size;let pitch=width.div_ceil(256)*256;
  let buffer=self.device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual outline alpha readback"),size:pitch as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=self.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});self.queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());self.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=buffer.slice(..).get_mapped_range();let pixels=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize].iter().copied()).collect();drop(bytes);buffer.unmap();pixels
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69j-preserve-per-instance-original-colours-07.rs"
```

```text
--8<-- "journey/code/69j-preserve-per-instance-original-colours-08.rs"
```

</details>

### 4. `src/gpu_geometry.rs`

Draw shared indexed geometry with a bounded instance count.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn allocated_bytes(&self) -> u64 { self.vertices.size() + self.normals.size() + self.indices.size() }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_vertex_buffer(1, self.normals.slice(..));
        pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69k-create-a-shared-definition-face-pipeline-01.rs"
```

</details>

<a id="checkpoint-69k-create-a-shared-definition-face-pipeline"></a>

<details>
<summary>Implementation 69k-create-a-shared-definition-face-pipeline: Create a shared definition face pipeline</summary>

Create a shared definition face pipeline.

Data flow: shared vertices → instance index → bounded storage record → placed colour face.

### 1. `src/instance_gpu.rs`

Create a storage-backed shared-definition colour pipeline.

Create the file and type:

```rust
--8<-- "journey/code/69k-create-a-shared-definition-face-pipeline-02.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod instance_scene_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/69k-create-a-shared-definition-face-pipeline-03.rs" skip=39
```

</details>

<a id="checkpoint-69l-reuse-bounded-independent-draw-records"></a>

<details>
<summary>Implementation 69l-reuse-bounded-independent-draw-records: Reuse bounded independent draw records</summary>

Reuse bounded independent draw records.

Data flow: shared GPU source → bounded record chunks → changed-only upload → indexed instance draw.

### 1. `src/instance_gpu.rs`

Reuse bounded shared-definition draw batches and changed-only record uploads.

<details>
<summary>Locate the existing block</summary>

```rust
        Self {pipeline,layout,batches:BTreeMap::new(),writes:0,written_bytes:0}
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69l-reuse-bounded-independent-draw-records-01.rs"
```

</details>

<a id="checkpoint-69m-draw-batched-faces-with-independent-integer-owners"></a>

<details>
<summary>Implementation 69m-draw-batched-faces-with-independent-integer-owners: Draw batched faces with independent integer owners</summary>

Draw batched faces with independent integer owners.

Data flow: visible instance IDs → shared face batches → independent selection → actual integer picks.

### 1. `src/renderer.rs`

Draw original definition faces in shared batches while retaining independent source picking.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            if let Some(clouds)=self.clouds.borrow().as_ref() {clouds.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
```

</details>

Replace it with:

```rust
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id)) {mesh.draw(&mut pass);}
            self.instances.draw(&mut pass);
            if let Some(clouds)=self.clouds.borrow().as_ref() {clouds.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
```

<details>
<summary>Locate this block</summary>

```rust

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }
```

</details>

Replace it with:

```rust

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }
```

<details>
<summary>Locate this block</summary>

```rust
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
```

</details>

Replace it with:

```rust
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)))?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn usage(&self) -> [u64; 4] { crate::memory::gpu(&self.meshes) }

    pub fn uploaded_bytes(&self) -> u64 {
        self.geometry.uploaded_bytes + self.settings_allocations as u64 * 160 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }
```

</details>

Replace it with:

```rust
    pub fn usage(&self) -> [u64; 4] { crate::memory::gpu(&self.meshes) }

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * 160 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }
```

<details>
<summary>Locate this block</summary>

```rust
            self.meshes.push(row);
        }
        drop(old);
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
```

</details>

Replace it with:

```rust
            self.meshes.push(row);
        }
        drop(old);
        self.instance_rows=scene.instances.keys().copied().collect();
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
```

<details>
<summary>Locate this block</summary>

```rust
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
```

</details>

Replace it with:

```rust
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
```

<details>
<summary>Locate this block</summary>

```rust
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface());
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

</details>

Replace it with:

```rust
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface());
        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
```

<details>
<summary>Locate this block</summary>

```rust
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
```

</details>

Replace it with:

```rust
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
    cloud_error:std::cell::RefCell<Option<String>>,
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode: crate::pick_sources::Mode,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, include_str!("cloud_points.wgsl").into(), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 160);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout)], immediate_size: 0 });
        let mesh_pipeline = |depth| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface());
        let instances=crate::instance_gpu::Lane::new(&device,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.picker.cancel(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.picker.cancel(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker();if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers().into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments();if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke();if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.instance_rows=scene.instances.keys().copied().collect();
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request { window, transform, density: self.density, mode: self.pick_mode })?;
        self.pump_pick(transform); Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density) else { self.picker.cancel(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) { self.picker.submit(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref()); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density) { Ok(view) => view, Err(error) => {
            self.picker.cancel(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => { self.pump_pick(transform); None },
            Some(Err(error)) => Some(Err(error)), None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.picker.cancel(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) { self.picker.cancel(); }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] { crate::memory::gpu(&self.meshes) }

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * 160 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self, settings: Option<crate::grid::Settings>) -> Result<(), &'static str> {
        if settings == self.grid_settings { return Ok(()); }
        let strokes = settings.map(crate::grid::Settings::strokes).transpose()?.unwrap_or_default();
        self.grid.set(&self.device, strokes)?; self.grid_settings = settings; Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)))?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let matrix=camera.view_projection();
        let layout=origin.and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:[0.;3],framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:[[0.;4];4]}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(merged)?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(merged)?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene) {
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {row.prepared.extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);}}}
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::camera(camera));}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        let display = [if normals { 1.0f32 } else { 0.0 }, 0.0, 0.0, 0.0];
        bytes.extend(display.iter().flat_map(|value| value.to_ne_bytes()));
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r, g, b, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id)) {mesh.draw(&mut pass);}
            self.instances.draw(&mut pass);
            if let Some(clouds)=self.clouds.borrow().as_ref() {clouds.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        if self.meshes.iter().any(GpuMesh::selected) {
            let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density);
        } else {self.outline.borrow_mut().take();}
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.queue.submit([encoder.finish()]);
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}

#[cfg(all(test,not(target_arch="wasm32")))]
impl Renderer {
 pub fn outline_alpha(&self)->Vec<u8> {
  let owner=self.outline.borrow();let Some(outline)=owner.as_ref() else {return Vec::new();};let Some(texture)=outline.alpha_texture() else {return Vec::new();};let [width,height]=self.size;let pitch=width.div_ceil(256)*256;
  let buffer=self.device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual outline alpha readback"),size:pitch as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=self.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});self.queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());self.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=buffer.slice(..).get_mapped_range();let pixels=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize].iter().copied()).collect();drop(bytes);buffer.unmap();pixels
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-01.rs"
```

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-02.rs"
```

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-03.rs"
```

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-04.rs"
```

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-05.rs"
```

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-06.rs"
```

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-07.rs"
```

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-08.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/pick_gpu_tests.rs`

Copy actual GPU batching, full integer owner IDs, source colour, reuse and release regressions.

<details>
<summary>Locate the existing block</summary>

```text
    scene.clear();renderer.set_scene(&scene,None);drop(prepared);assert_eq!(renderer.cloud_usage(),[0;5]);assert!(weak.upgrade().is_none());
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/69m-draw-batched-faces-with-independent-integer-owners-09.rs" skip=143
```

</details>

</details>

<a id="checkpoint-69n-inspect-and-exercise-original-definition-instances"></a>

<details>
<summary>Implementation 69n-inspect-and-exercise-original-definition-instances: Inspect and exercise original definition instances</summary>

Inspect and exercise original definition instances.

Data flow: example original → placed shared rows → source diagnostics → reversible commands → saved references.

### 1. `src/editor.rs`

Connect original instance examples and separate source, placement and GPU diagnostics.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
```

</details>

Replace it with:

```rust
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
```

<details>
<summary>Locate this block</summary>

```rust
    AddCloudLod,
    AddCloud,
    AddCad,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
```

</details>

Replace it with:

```rust
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddText(crate::text::TextPlacement),
    AddSurface,
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
    Preview,
}

pub struct Editor {
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            draft: None, tool_status: String::new(),
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        self.draft.as_ref()?;
        if text.trim().is_empty() || text.eq_ignore_ascii_case("Escape") { return Some(Ok(Action::CancelTool)); }
        text.contains(',').then(|| crate::tool_coords::parse(text).map(Action::ToolPoint))
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft = self.draft.as_mut().ok_or("No point tool is running")?;
        let name = draft.tool.name();
        let Some(built) = draft.submit(point)? else { self.tool_status = draft.prompt(); self.snap_index=None; return self.update_edit_preview(); };
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            if self.edit_preview.take().is_some() { self.draft=None; } self.tool_status="Move completed.".into();
            return Ok(if changed { Change::Scene } else { Change::Preview });
        }
        self.history.try_edit(&mut self.scene, |scene| match built {
            crate::tool::Built::Point(point) => scene.insert_point(point).map(|_| ()),
            crate::tool::Built::Line(line) => scene.insert_line(line).map(|_| ()),
            crate::tool::Built::Move(_) => unreachable!("Move uses its placement preview"),
        })?;
        self.draft = None; self.tool_status = format!("{name} created."); Ok(Change::Scene)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        hit.or_else(|| crate::snap_index::fallback(self.snapping, &self.camera, screen))
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None; }
        match action {
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit { return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point)); }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None; self.draft=Some(crate::tool::Draft::new(shape)); self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { let changed=self.edit_preview.take().is_some(); self.draft=None; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69n-inspect-and-exercise-original-definition-instances-01.rs"
```

```text
--8<-- "journey/code/69n-inspect-and-exercise-original-definition-instances-02.rs"
```

</details>

### 2. `src/commands.rs`

Connect original instance examples and separate source, placement and GPU diagnostics.

<details>
<summary>Locate the existing block</summary>

```rust
    ("Example Cloud",||Action::AddCloud),
    ("Example CAD",||Action::AddCad),
    ("Example Scene Text",||Action::AddText(crate::text::TextPlacement::WorldPlane {world:[-1.5,0.,1.],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.22})),
    ("Example Billboard",||Action::AddText(crate::text::TextPlacement::WorldBillboard {world:[-1.5,0.,1.],world_height:0.22})),
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69n-inspect-and-exercise-original-definition-instances-03.rs"
```

### 3. `src/browser.rs`

Connect original instance examples and separate source, placement and GPU diagnostics.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-highlight-bytes", &renderer.highlight_bytes().to_string())?;
    canvas.set_attribute("data-selection-part", &editor.selection().map_or_else(String::new, |target| target.describe()))?;
    let stream_usage=crate::memory::streams(editor.scenes());
    canvas.set_attribute("data-range-sources",&serde_json::to_string(&stream_usage).unwrap())?;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69n-inspect-and-exercise-original-definition-instances-04.rs"
```

</details>

<a id="checkpoint-69zza-move-between-original-rigid-frames"></a>

<details>
<summary>Implementation 69zza-move-between-original-rigid-frames: Move between original rigid frames</summary>

Write local coordinates and the rotation between two right-handed frames.

Data flow: original controls → rigid frame → local shape words → candidate rigid copy.

### 1. `src/cad_shape_key.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/69zza-move-between-original-rigid-frames-1.rs"
```

### 2. `src/lib.rs`

Register the original rigid-frame and shape-key owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod instance_gpu;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/69zza-move-between-original-rigid-frames-2.rs" skip=23
```

</details>

<a id="checkpoint-69zzb-find-an-original-frame-and-pack-key-words"></a>

<details>
<summary>Implementation 69zzb-find-an-original-frame-and-pack-key-words: Find an original frame and pack key words</summary>

Choose well-separated original surface control points for a right-handed frame.

Data flow: original controls → rigid frame → local shape words → candidate rigid copy.

### 1. `src/cad_shape_key.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit(a: [f64; 3]) -> [f64; 3] {
    let length = dot(a, a).sqrt();
    [a[0] / length, a[1] / length, a[2] / length]
}
```

</details>

After that block, type:

```rust
/// Every surface control point of a BRep, in storage order.
fn surface_points(b: &BRep) -> Vec<[f64; 3]> {
    let mut points = Vec::new();

    for surface in &b.m_surfaces {
        for i in 0..surface.cv_count(0) {
            for j in 0..surface.cv_count(1) {
                let p = surface.get_cv(i, j).unwrap_or_default();
                points.push([p[0], p[1], p[2]]);
            }
        }
    }

    points
}

/// The frame of a BRep and its size: the first control point, the first one clearly away from
/// it for x, the first one clearly off that line for z. None for a flat line or nothing.
fn frame_of(points: &[[f64; 3]]) -> Option<(Frame, f64)> {
    let origin = *points.first()?;
    let size = points
        .iter()
        .map(|p| dot(sub(*p, origin), sub(*p, origin)).sqrt())
        .fold(0.0, f64::max);

    if !(size.is_finite() && size > 0.0) {
        return None;
    }

    let x = points
        .iter()
        .map(|p| sub(*p, origin))
        .find(|d| dot(*d, *d).sqrt() > AXIS_SPAN * size)?;
    let x = unit(x);
    let z = points
        .iter()
        .map(|p| cross(x, sub(*p, origin)))
        .find(|c| dot(*c, *c).sqrt() > AXIS_SPAN * size)?;
    let z = unit(z);
    let y = cross(z, x);
    let frame = Frame {
        origin,
        axes: [x, y, z],
    };

    Some((frame, size))
}

/// What makes two BReps the same shape: their topology, parameters, weights, colours and widths
/// exactly, and their 3D points in their own frames up to `SAME_POINT`.
pub struct ShapeKey {
    pub words: Vec<u64>, // the shape, word by word
    pub frame: Frame,    // where this BRep lies
    pub step: f64,       // the length one point word counts
}

impl ShapeKey {
    fn push_words(&mut self, words: &[u64]) {
        self.words.extend_from_slice(words);
    }

    fn push_float(&mut self, value: f64) {
        self.words.push(value.to_bits());
    }

    /// A weight, rounded to `SAME_NUMBER`.
    fn push_number(&mut self, value: f64) {
        self.words.push((value / SAME_NUMBER).round() as i64 as u64);
    }

    fn push_color(&mut self, color: &Color) {
        for value in [color.r, color.g, color.b, color.a] {
            self.words.push(value.to_bits() as u64);
        }
    }

    fn push_refs(&mut self, refs: &[BRepRef]) {
        self.words.push(refs.len() as u64);

        for r in refs {
            self.push_words(&[r.index as i64 as u64, r.orientation as u64]);
        }
    }

    fn push_counts(&mut self, b: &BRep) {
        self.push_words(&[
            b.m_surfaces.len() as u64,
            b.m_curves_3d.len() as u64,
            b.m_curves_2d.len() as u64,
            b.m_vertices.len() as u64,
            b.m_edges.len() as u64,
            b.m_wires.len() as u64,
            b.m_faces.len() as u64,
            b.m_shells.len() as u64,
            b.m_solids.len() as u64,
        ]);
    }

}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69zzb-find-an-original-frame-and-pack-key-words-1.rs"
```

</details>

</details>

<a id="checkpoint-69zzc-encode-original-parameters-and-topology"></a>

<details>
<summary>Implementation 69zzc-encode-original-parameters-and-topology: Encode original parameters and topology</summary>

Encode knots, local points, pcurve values and curve descriptors.

Data flow: original controls → rigid frame → local shape words → candidate rigid copy.

### 1. `src/cad_shape_key.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
    fn push_color(&mut self, color: &Color) {
        for value in [color.r, color.g, color.b, color.a] {
            self.words.push(value.to_bits() as u64);
        }
    }

    fn push_refs(&mut self, refs: &[BRepRef]) {
        self.words.push(refs.len() as u64);

        for r in refs {
            self.push_words(&[r.index as i64 as u64, r.orientation as u64]);
        }
    }

    fn push_counts(&mut self, b: &BRep) {
        self.push_words(&[
            b.m_surfaces.len() as u64,
            b.m_curves_3d.len() as u64,
            b.m_curves_2d.len() as u64,
            b.m_vertices.len() as u64,
            b.m_edges.len() as u64,
            b.m_wires.len() as u64,
            b.m_faces.len() as u64,
            b.m_shells.len() as u64,
            b.m_solids.len() as u64,
        ]);
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69zzc-encode-original-parameters-and-topology-1.rs"
```

</details>

<a id="checkpoint-69a-describe-a-rigid-frame-and-original-shape-key"></a>

<details>
<summary>Implementation 69a-describe-a-rigid-frame-and-original-shape-key: Assemble an original rigid shape key</summary>

Assemble the existing encoders for every original surface, curve, vertex and topology record.

Data flow: original controls → rigid frame → local shape words → candidate rigid copy.

### 1. `src/cad_shape_key.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
    pub words: Vec<u64>, // the shape, word by word
    pub frame: Frame,    // where this BRep lies
    pub step: f64,       // the length one point word counts
}

impl ShapeKey {
```

</details>

After that block, type:

```rust
    /// The key of `b`; None when it has no frame.
    pub fn of(b: &BRep) -> Option<Self> {
        let (frame, size) = frame_of(&surface_points(b))?;
        let mut key = Self {
            words: Vec::new(),
            frame,
            step: size * SAME_POINT,
        };
        key.push_counts(b);
        key.push_float(b.width);
        key.push_color(&b.surfacecolor);

        for surface in &b.m_surfaces {
            key.push_words(&[
                surface.m_dim as u64,
                surface.m_is_rat as u64,
                surface.m_order[0] as u64,
                surface.m_order[1] as u64,
                surface.m_cv_count[0] as u64,
                surface.m_cv_count[1] as u64,
            ]);
            key.push_knots(&surface.m_nurbsknot[0]);
            key.push_knots(&surface.m_nurbsknot[1]);

            for i in 0..surface.cv_count(0) {
                for j in 0..surface.cv_count(1) {
                    let p = surface.get_cv(i, j).unwrap_or_default();
                    key.push_point([p[0], p[1], p[2]]);
                    key.push_number(surface.weight(i, j));
                }
            }
        }

        for curve in &b.m_curves_3d {
            key.push_curve_shape(curve);

            for i in 0..curve.cv_count() {
                let p = curve.get_cv(i).unwrap_or_default();
                key.push_point([p[0], p[1], p[2]]);
                key.push_number(curve.weight(i));
            }
        }

        for curve in &b.m_curves_2d {
            key.push_curve_shape(curve);
            key.push_parameters(&curve.m_cv);
        }

        for vertex in &b.m_vertices {
            key.push_point([vertex.point[0], vertex.point[1], vertex.point[2]]);
        }

        key.push_topology(b);
        Some(key)
    }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69a-describe-a-rigid-frame-and-original-shape-key-1.rs"
```

</details>

</details>

<a id="checkpoint-69aa-retain-cloneable-derived-cad-values"></a>

<details>
<summary>Implementation 69aa-retain-cloneable-derived-cad-values: Retain cloneable derived CAD values</summary>

Retain cloneable derived CAD values.

Data flow: original source validation → derived values → scoped recording.

### 1. `src/mesh.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Mesh {
    vertices: Vec<[f32; 6]>,
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-01.rs"
```

### 2. `src/mesh_parts.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
#[derive(Default)]
pub struct Parts {
    pub faces: Vec<usize>,
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-02.rs"
```

### 3. `src/cad_face.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::Point;
pub struct Sample {pub vertex:u32,pub surface:usize,pub uv:[f64;2],pub sign:f64}
pub struct Data {pub points:Vec<Point>,pub faces:Vec<Vec<usize>>,pub vertices:Vec<[f32;6]>,pub normals:Vec<[f32;3]>,pub indices:Vec<u32>,pub face_ids:Vec<u32>,pub keys:Vec<std::collections::HashMap<usize,u32>>,pub samples:Vec<Sample>}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-03.rs"
```

### 4. `src/cad_display.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::Xform;

pub struct Boundary {pub edge:usize,pub points:Vec<[f64;3]>}
pub struct Prepared {pub source:Rc<session_rust::BRep>,pub edges:Vec<Boundary>,pub samples:Vec<crate::cad_face::Sample>,pub face_count:usize,pub edge_ids:Vec<usize>}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-04.rs"
```

### 5. `src/cad_boundary.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
use std::rc::Rc;
use session_rust::{BRep,Mesh};

pub const QUALITY:(f64,f64)=(5.,0.001);
pub struct Prepared {pub source:Rc<BRep>,pub faces:Vec<Mesh>,pub chains:Vec<Option<crate::cad_edges::EdgeChain>>}
impl Prepared {
    pub fn new(source:Rc<BRep>)->Result<Self,&'static str> {
        if !source.is_valid() {return Err("BRep topology contains an invalid reference");}
        if source.m_faces.len()>65_536||source.m_edges.len()>262_144 {return Err("BRep topology exceeds the display budget");}
        for surface in &source.m_surfaces {crate::surface::validate(surface)?;}
        for curve in source.m_curves_3d.iter().chain(&source.m_curves_2d) {crate::curve::validate(curve)?;}
        let _=source.guid();let faces=source.face_meshes_q(Some(QUALITY));
        if faces.len()!=source.m_faces.len()||faces.iter().any(|face|face.face.is_empty()) {return Err("BRep face could not be tessellated");}
        let chains=crate::cad_edges::edge_chains(&source,&faces);
        for (index,(edge,chain)) in source.m_edges.iter().zip(&chains).enumerate() {
            if !edge.degenerated&&!source.edge_faces(index).is_empty()&&chain.is_none() {return Err("A CAD edge has no coherent face boundary");}
        }
        Ok(Self {source,faces,chains})
    }
    pub fn points(&self,edge:usize)->Option<Vec<[f64;3]>> {
        let chain=self.chains.get(edge)?.as_ref()?;let face=self.faces.get(chain.face)?;
        chain.keys.iter().map(|key|face.vertex.get(key).map(|v|[v.x,v.y,v.z])).collect()
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::rc::Rc;
use session_rust::{BRep,Mesh};

pub const QUALITY:(f64,f64)=(5.,0.001);
pub struct Prepared {pub source:Rc<BRep>,pub faces:Vec<Mesh>,pub chains:Vec<Option<crate::cad_edges::EdgeChain>>}
impl Prepared {
    pub fn new(source:Rc<BRep>)->Result<Self,&'static str> {
        validate(&source)?;
        let _=source.guid();let faces=source.face_meshes_q(Some(QUALITY));
        if faces.len()!=source.m_faces.len()||faces.iter().any(|face|face.face.is_empty()) {return Err("BRep face could not be tessellated");}
        let chains=crate::cad_edges::edge_chains(&source,&faces);
        for (index,(edge,chain)) in source.m_edges.iter().zip(&chains).enumerate() {
            if !edge.degenerated&&!source.edge_faces(index).is_empty()&&chain.is_none() {return Err("A CAD edge has no coherent face boundary");}
        }
        Ok(Self {source,faces,chains})
    }
    pub fn points(&self,edge:usize)->Option<Vec<[f64;3]>> {
        let chain=self.chains.get(edge)?.as_ref()?;let face=self.faces.get(chain.face)?;
        chain.keys.iter().map(|key|face.vertex.get(key).map(|v|[v.x,v.y,v.z])).collect()
    }
}

pub fn validate(source:&BRep)->Result<(),&'static str> {
        if !source.is_valid() {return Err("BRep topology contains an invalid reference");}
        if source.m_faces.len()>65_536||source.m_edges.len()>262_144 {return Err("BRep topology exceeds the display budget");}
        for surface in &source.m_surfaces {crate::surface::validate(surface)?;}
        for curve in source.m_curves_3d.iter().chain(&source.m_curves_2d) {crate::curve::validate(curve)?;}
    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-05.rs"
```

```text
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-06.rs"
```

</details>

</details>

<a id="checkpoint-69ab-replay-sampled-geometry-in-the-new-rigid-frame"></a>

<details>
<summary>Implementation 69ab-replay-sampled-geometry-in-the-new-rigid-frame: Replay sampled geometry in the new rigid frame</summary>

Replay sampled geometry in the new rigid frame.

Data flow: recorded samples → rigid motion → new original owner → replayed display.

### 1. `src/cad_shape_cache.rs`

Replay recorded display values with the correct independent original owner.

Create the file and type:

```rust
--8<-- "journey/code/69ab-replay-sampled-geometry-in-the-new-rigid-frame-01.rs"
```

### 2. `src/lib.rs`

Register the CAD preparation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod cad_shape_key;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/69ab-replay-sampled-geometry-in-the-new-rigid-frame-02.rs" skip=24
```

</details>

<a id="checkpoint-69ac-guard-reusable-walks-with-source-validity-and-size"></a>

<details>
<summary>Implementation 69ac-guard-reusable-walks-with-source-validity-and-size: Guard reusable walks with source validity and size</summary>

Guard reusable walks with source validity and size.

Data flow: checked original → shape candidates → physical size check → replay or new walk.

### 1. `src/cad_shape_cache.rs`

Validate original sources and physical size before reusing a display walk.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/69ac-guard-reusable-walks-with-source-validity-and-size-01.rs" skip=8
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/cad_shape_cache.rs`

Copy congruent-source, chord/normal, shape-refusal and scoped-release regressions.

<details>
<summary>Locate the existing block</summary>

```text
    pub fn recordings(&self)->usize {self.records.values().map(Vec::len).sum()}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/69ac-guard-reusable-walks-with-source-validity-and-size-02.rs" skip=82
```

</details>

</details>

<a id="checkpoint-69ad-share-recordings-only-during-document-preparation"></a>

<details>
<summary>Implementation 69ad-share-recordings-only-during-document-preparation: Share recordings only during document preparation</summary>

Share recordings only during document preparation.

Data flow: one source document → scoped recordings → independent scene owners → dropped preparation cache.

### 1. `src/scene.rs`

Reuse scoped CAD recordings across original objects and definition templates.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.created=None;
        self.ids=Default::default();
        self.view=Default::default();
```

</details>

Replace it with:

```rust
    }

    pub fn clear(&mut self) {
        self.cad_walks=[0;2];self.instances.clear();
        self.created=None;
        self.ids=Default::default();
        self.view=Default::default();
```

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn insert_cad(&mut self,source:Rc<session_rust::BRep>)->Result<ObjectId,&'static str> {
        let (prepared,mesh)=crate::cad_display::prepare(source)?;let prepared=Rc::new(prepared);let id=self.insert(mesh)?;self.objects.iter_mut().find(|row|row.id==id).unwrap().cad=Some(prepared);Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
```

</details>

Replace it with:

```rust
    }

    pub fn insert_cad(&mut self,source:Rc<session_rust::BRep>)->Result<ObjectId,&'static str> {
        self.insert_prepared_cad(crate::cad_display::prepare(source)?)
    }

    pub(crate) fn insert_prepared_cad(&mut self,(prepared,mesh):(crate::cad_display::Prepared,crate::prepared::PreparedMesh))->Result<ObjectId,&'static str> {
        let prepared=Rc::new(prepared);let id=self.insert(mesh)?;self.objects.iter_mut().find(|row|row.id==id).unwrap().cad=Some(prepared);Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
```

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn empty()->Self {
        Self {instances:Default::default(),created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
```

</details>

Replace it with:

```rust
    }

    pub fn empty()->Self {
        Self {cad_walks:[0;2],instances:Default::default(),created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
```

<details>
<summary>Locate this block</summary>

```rust

#[derive(Clone)]
pub struct Scene {
    pub(crate) instances:std::collections::HashMap<ObjectId,crate::instance_plan::Owner>,
    pub(crate) created:Option<u32>,
    ids:Rc<std::collections::HashSet<ObjectId>>,
```

</details>

Replace it with:

```rust

#[derive(Clone)]
pub struct Scene {
    pub cad_walks:[usize;2],
    pub(crate) instances:std::collections::HashMap<ObjectId,crate::instance_plan::Owner>,
    pub(crate) created:Option<u32>,
    ids:Rc<std::collections::HashSet<ObjectId>>,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{mesh::Mesh, prepared::PreparedMesh};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);

impl ObjectId { pub fn value(self) -> u32 { self.0 } }

#[derive(Clone)]
pub struct Object {
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
    pub metadata: Rc<crate::row_metadata::RowMetadata>,
    pub mesh: Rc<Mesh>,
    pub model: session_rust::Xform,
    editable: crate::edit_source::EditSource,
}

impl Object {
    pub fn visible(&self) -> bool { self.metadata.is_visible }

    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        match &self.editable { crate::edit_source::EditSource::Loaded { geometry, .. } => Some(geometry), _ => None }
    }

    pub fn source(&self) -> Option<&crate::document::Source> {
        match &self.editable { crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref(), _ => None }
    }

    pub fn origin(&self) -> Option<&Rc<crate::origin::Origin>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref().map(|source| &source.origin),
            crate::edit_source::EditSource::Released { origin, .. } => Some(origin),
        }
    }

    pub fn release_epoch(&self) -> Option<u64> {
        match &self.editable { crate::edit_source::EditSource::Released { epoch, .. } => Some(*epoch), _ => None }
    }

    pub fn can_unload(&self) -> bool {
        let Some(source) = self.source() else { return false; };
        let Some(geometry) = self.geometry() else { return false; };
        source.origin.location.is_some() && self.metadata.source_guid == source.guid
            && geometry.guid() == source.guid && self.metadata.name == geometry.name
            && self.metadata.is_visible == geometry.is_visible && self.metadata.is_locked == geometry.is_locked
            && source.document.objects.meshes.iter().any(|mesh| Rc::ptr_eq(mesh, geometry))
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
        let world = self.model.transform_point(&local);
        [world[0], world[1], world[2]]
    }
}

#[derive(Clone)]
pub struct LineObject {
    pub guid: String,
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
}

impl LineObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [self.model.transform_point(&line.start()), self.model.transform_point(&line.end())]
    }

    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke { start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32), ..self.prepared.display }
    }
}

#[derive(Clone)]
pub struct PathObject { pub controls_visible: bool, pub id: ObjectId, pub guid: String, pub prepared: crate::chain::PreparedChain, pub model: session_rust::Xform }

impl PathObject {
    pub fn visible(&self) -> bool { self.prepared.source.visible() }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = std::array::from_fn(|i| p[i] as f32);
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
    }

    pub fn bounds_points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.bounds_coordinates().into_iter().map(|p|
            self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.coordinates().into_iter().map(|p| self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }


}

pub struct ControlMarker { pub parent: ObjectId, pub control: crate::controls::Control, pub display: crate::marker::Marker }

#[derive(Clone)]
pub struct PointObject { pub id: ObjectId, pub guid: String, pub prepared: crate::marker::PreparedPoint, pub model: session_rust::Xform }

impl PointObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self) -> session_rust::Point { self.model.transform_point(&self.prepared.source) }
    pub fn marker(&self) -> crate::marker::Marker { crate::marker::Marker { center: self.point().to_f32(), ..self.prepared.display } }
}

#[derive(Clone)]
pub struct Scene {
    pub cad_walks:[usize;2],
    pub(crate) instances:std::collections::HashMap<ObjectId,crate::instance_plan::Owner>,
    pub(crate) created:Option<u32>,
    ids:Rc<std::collections::HashSet<ObjectId>>,
    pub view:Rc<crate::view_flags::Flags>,
    pub forest:crate::hierarchy::Forest,
    streamed:Vec<crate::range_cloud::Row>,
    clouds:Vec<crate::cloud::Row>,
    texts:Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn pick_id(&self,value:u32)->Option<ObjectId> {
        let id=ObjectId(value);(self.visible(id) && !self.view.locked.contains(&id) && !self.forest.readonly(id)).then_some(id)
    }

    pub fn empty()->Self {
        Self {cad_walks:[0;2],instances:Default::default(),created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle([
            [-0.7, -0.6, 0.25], [0.5, -0.6, 0.25], [-0.1, 0.6, 0.25],
        ], [0.9, 0.25, 0.45]).expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle([
            [-0.4, -0.2, 0.75], [0.8, -0.2, 0.75], [0.2, 0.8, 0.75],
        ], [0.05, 0.7, 0.7]).expect("Valid turquoise triangle");
        let mut scene = Self::empty();
        scene.insert(pink).expect("ID available");
        scene.insert(turquoise).expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn visible(&self,id:ObjectId)->bool {self.contains(id)&&!self.view.is_hidden(&id)}

    pub(crate) fn restore_view(&mut self,mut saved:Self) {saved.view=Rc::clone(&self.view);self.restore(saved);Rc::make_mut(&mut self.view).refresh(&self.forest);}

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in self.objects.iter().filter(|row| self.visible(row.id)) {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for line in self.lines.iter().filter(|line| self.visible(line.id)) {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self.paths.iter().filter(|row| self.visible(row.id)) {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds { Some(bounds) => bounds.include(point), None => bounds = Some(crate::bounds::Bounds::point(point)) }
            }
        }
        for row in self.points.iter().filter(|row| self.visible(row.id)) {
            let p = row.point(); let p = [p[0], p[1], p[2]];
            match &mut bounds { Some(bounds) => bounds.include(p), None => bounds = Some(crate::bounds::Bounds::point(p)) }
        }
        for row in self.texts.iter().filter(|row|self.visible(row.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.streamed.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        for row in self.clouds.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for id in ids { if let Some(next) = self.selected_bounds(*id) {
            if let Some(b) = &mut bounds { b.include(next.min); b.include(next.max); } else { bounds = Some(next); }
        } } bounds
    }

    pub fn placement(&self, id: ObjectId) -> Option<session_rust::Xform> {
        self.objects.iter().find(|r| r.id==id).map(|r| r.model.clone())
            .or_else(|| self.lines.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.paths.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.points.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.clouds.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.streamed.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.texts.iter().find(|r| r.id==id).map(|r| r.model.clone()))
    }

    pub fn with_descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {self.forest.descendants(ids).into_iter().filter(|id|self.contains(*id)).collect()}

    pub fn place_owner(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_following(id,model)
    }

    pub(crate) fn place_following(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id==id) { self.place_mesh(id,model) }
        else if self.lines.iter().any(|r| r.id==id) { self.place_line(id,model) }
        else if self.paths.iter().any(|r| r.id==id) { self.place_path(id,model) }
        else if self.clouds.iter().any(|r|r.id==id) {self.place_cloud(id,model)}
        else if self.streamed.iter().any(|r|r.id==id) {self.place_stream(id,model)}
        else if self.texts.iter().any(|r|r.id==id) {self.place_text(id,model)}
        else { self.place_point(id,model) }
    }

    pub fn translate(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<(), &'static str> {
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        if let Some(row) = self.objects().iter().find(|r| r.id == id) {
            if row.geometry().is_none() { return Err("Reload editable sources before Move"); }
            let model = &shift * &row.model; return self.place(id, model);
        }
        if let Some(row) = self.lines().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_line(id, model); }
        if let Some(row) = self.paths().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_path(id, model); }
        if let Some(row) = self.points().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_point(id, model); }
        if let Some(row)=self.streamed.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_stream(id,model);}
        if let Some(row)=self.clouds.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_cloud(id,model);}
        if let Some(row)=self.texts.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_text(id,model);}
        Err("Object not found")
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) { return None; }
        if let Some(row)=self.clouds.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.streamed.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn line_capacity_bytes(&self) -> usize { self.lines.capacity() * std::mem::size_of::<LineObject>() }

    pub fn cloud_controls(&mut self,parent:Option<ObjectId>) {
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
        for row in &mut self.streamed {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn streams(&self)->&[crate::range_cloud::Row] {&self.streamed}
    pub fn insert_stream(&mut self,prepared:Rc<crate::range_cloud::Prepared>,model:session_rust::Xform,point_px:f32)->Result<ObjectId,&'static str> {
        if !crate::placement::valid(&model.m)||!point_px.is_finite()||point_px<0. {return Err("Invalid streamed source placement or point size");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.streamed.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let row=crate::range_cloud::Row {id,guid:guid.clone(),prepared,model,point_px,controls_visible:false};
        if row.bounds().min.into_iter().chain(row.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}
        let name=row.prepared.source.name.clone();let properties=row.prepared.source.properties;
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,properties.visible,properties.locked);self.streamed.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_stream(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid streamed source placement");}
        let row=self.streamed.iter_mut().find(|row|row.id==id).ok_or("Streamed source is gone")?;let mut candidate=row.clone();candidate.model=model;
        if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn clouds(&self)->&[crate::cloud::Row] {&self.clouds}
    pub fn insert_cloud(&mut self,prepared:crate::cloud::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid().to_string();while self.clouds.iter().any(|r|r.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let name=prepared.source.name.clone();let visible=prepared.source.is_visible;let locked=prepared.source.is_locked;let row=crate::cloud::Row {point_px:0.,controls_visible:false,id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.clouds.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_cloud(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid cloud placement");}let row=self.clouds.iter_mut().find(|r|r.id==id).ok_or("Cloud object not found")?;let mut candidate=row.clone();candidate.model=model;if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!(v as f32).is_finite()) {return Err("Cloud placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn texts(&self)->&[crate::scene_label::Row] {&self.texts}
    pub(crate) fn texts_mut(&mut self)->&mut [crate::scene_label::Row] {&mut self.texts}
    pub fn insert_text(&mut self,prepared:crate::scene_label::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.texts.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}let label=prepared.source.name.clone();let row=crate::scene_label::Row {id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};row.label(false,true)?;if row.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Text bounds exceed the display range");}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,row.prepared.source.visible,row.prepared.source.locked);self.texts.push(row);crate::layer_edit::attach_generated(self,id,guid,label)?;Ok(id)
    }
    pub fn place_text(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid text placement");}let row=self.texts.iter_mut().find(|row|row.id==id).ok_or("Text object not found")?;let mut candidate=row.clone();candidate.model=model;candidate.label(false,true)?;if candidate.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Text bounds exceed the display range");}*row=candidate;Ok(())
    }
    pub fn text_labels(&self,selected:&[ObjectId])->Result<Vec<crate::text::TextLabel>,&'static str> {self.texts.iter().filter(|row|self.visible(row.id)).map(|row|row.label(selected.contains(&row.id),self.pick_id(row.id.value()).is_some())).collect()}
    pub fn points(&self) -> &[PointObject] { &self.points }

    pub fn point_capacity_bytes(&self) -> usize { self.points.capacity() * std::mem::size_of::<PointObject>() }

    pub fn insert_point(&mut self, prepared: crate::marker::PreparedPoint) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.points.iter().any(|row| row.guid == guid) || self.paths.iter().any(|row| row.guid == guid)
            || self.objects.iter().any(|row| row.guid == guid) || self.lines.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let label=prepared.source.name.clone();let node_guid=guid.clone();self.points.push(PointObject { id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_point(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid point placement"); }
        let row = self.points.iter_mut().find(|row| row.id == id).ok_or("Point not found")?;
        let point = model.transform_point(&row.prepared.source);
        if (0..3).any(|i| !(point[i] as f32).is_finite()) { return Err("Point placement exceeds the display range"); }
        row.model = model; Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        for row in &mut self.paths { row.controls_visible = visible && matches!(row.prepared.source, crate::chain::Source::Curve(..)); }
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self.paths.iter().filter(|row| row.controls_visible && self.visible(row.id)) {
            for control in row.prepared.source.controls() {
                let p = control.position().expect("Validated original control");
                let p = row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker { center: p.to_f32(), ..control.marker().expect("Prepared control marker") };
                markers.push(ControlMarker { parent: row.id, control, display });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] { &self.paths }

    pub fn path_capacity_bytes(&self) -> usize { self.paths.capacity() * std::mem::size_of::<PathObject>() }

    pub fn insert_path(&mut self, prepared: crate::chain::PreparedChain) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.paths.iter().any(|row| row.guid == guid) || self.objects.iter().any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid) { guid = uuid::Uuid::new_v4().to_string(); }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.visible(),prepared.source.locked());
        let label=prepared.source.name().to_owned();let node_guid=guid.clone();self.paths.push(PathObject { controls_visible: false, id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_path(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid path placement"); }
        let row = self.paths.iter_mut().find(|row| row.id == id).ok_or("Path not found")?;
        for p in row.prepared.source.coordinates().into_iter().chain(row.prepared.source.bounds_coordinates()) {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) { return Err("Path placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn lines(&self) -> &[LineObject] { &self.lines }

    pub fn insert_line(&mut self, prepared: crate::stroke::PreparedLine) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let mut guid=prepared.source.guid().to_owned();while self.lines.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}let label=prepared.source.name.clone();
        self.lines.push(LineObject { guid:guid.clone(), id, prepared, model: session_rust::Xform::identity() });
        crate::layer_edit::attach_generated(self,id,guid,label)?;
        Ok(id)
    }

    pub fn place_line(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid line placement"); }
        let row = self.lines.iter_mut().find(|row| row.id == id).ok_or("Line not found")?;
        for point in [row.prepared.source.start(), row.prepared.source.end()] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) { return Err("Line placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut [Object] { &mut self.objects }

    pub(crate) fn unload(&mut self, origins: &std::collections::HashSet<uuid::Uuid>, epoch: u64) {
        for row in &mut self.objects {
            if let Some(origin) = row.origin().filter(|origin| origins.contains(&origin.id)).cloned() {
                row.editable = crate::edit_source::EditSource::Released { origin, epoch };
            }
        }
    }

    pub(crate) fn hydrate(&mut self, prepared: &crate::rehydrate::PreparedReload) {
        for row in &mut self.objects {
            if !prepared.key.matches(row) { continue; }
            let Some(geometry) = prepared.geometry(&row.metadata.source_guid).cloned() else { continue; };
            let source = crate::document::Source { document: Rc::clone(&prepared.document),
                guid: row.metadata.source_guid.clone(), origin: Rc::clone(&prepared.key.origin) };
            row.editable = crate::edit_source::EditSource::Loaded { geometry, source: Some(source) };
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&prepared.model.m) { return Err("Placement must be finite, affine and fit the display range"); }
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self.objects.iter().any(|object| object.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.geometry.is_visible,prepared.geometry.is_locked);
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(&prepared.geometry));
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {cad:None,surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }

    pub fn insert_cad(&mut self,source:Rc<session_rust::BRep>)->Result<ObjectId,&'static str> {
        self.insert_prepared_cad(crate::cad_display::prepare(source)?)
    }

    pub(crate) fn insert_prepared_cad(&mut self,(prepared,mesh):(crate::cad_display::Prepared,crate::prepared::PreparedMesh))->Result<ObjectId,&'static str> {
        let prepared=Rc::new(prepared);let id=self.insert(mesh)?;self.objects.iter_mut().find(|row|row.id==id).unwrap().cad=Some(prepared);Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
        let prepared=Rc::new(crate::surface::Prepared::new(source)?);let mesh=prepared.mesh()?;let id=self.insert(mesh)?;
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
    }

    pub(crate) fn copy_row(&mut self,id:ObjectId,all:bool)->Result<ObjectId,&'static str> {
        if self.objects.iter().any(|row|row.id==id&&row.geometry().is_none()) {return Err("Reload editable sources before copying");}
        if !self.contains(id) {return Err("Object is gone");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let copy=ObjectId(self.next_id);let guid=uuid::Uuid::new_v4().to_string();
        if let Some(row)=self.objects.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.objects.push(row);}
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        else if let Some(row)=self.streamed.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.streamed.push(row);}
        else if let Some(row)=self.clouds.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.clouds.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        if let Some(owner)=self.instances.get(&id).cloned() {self.instances.insert(copy,owner);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
        for channel in [&mut view.faces,&mut view.edges] {if let Some(rgb)=channel.get(&id).copied() {channel.insert(copy,rgb);}}
        Ok(copy)
    }

    pub(crate) fn row_guid(&self,id:ObjectId)->Option<&str> {
        self.objects.iter().find(|row|row.id==id).map(|row|row.guid.as_str())
            .or_else(||self.paths.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.points.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.lines.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.clouds.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.streamed.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.texts.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
    }

    pub(crate) fn append_document(&mut self,source:&Scene,name:&str,place:&session_rust::Xform,point_px:f32)->Result<(),&'static str> {
        if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size");}
        let mut candidate=self.clone();let mut ids=source.ids.iter().copied().collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());
        let mut remap=std::collections::HashMap::new();let mut names=std::collections::HashMap::new();let mut used=candidate.ids.iter().filter_map(|id|candidate.row_guid(*id).map(str::to_owned)).collect::<std::collections::HashSet<_>>();
        for old in &ids {
            let id=ObjectId(candidate.reserve_document_key()?);remap.insert(*old,id);Rc::make_mut(&mut candidate.ids).insert(id);
            let mut guid=source.row_guid(*old).ok_or("Original source owner is missing")?.to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}names.insert(*old,guid);
        }
        macro_rules! rows {($field:ident)=>{for row in &source.$field {let mut row=row.clone();let old=row.id;row.id=remap[&old];row.guid=names[&old].clone();candidate.$field.push(row);}};}
        rows!(objects);rows!(lines);rows!(paths);rows!(points);rows!(clouds);rows!(streamed);rows!(texts);
        for old in &ids {
            let id=remap[old];if let Some(owner)=source.instances.get(old) {candidate.instances.insert(id,owner.clone());}candidate.place_following(id,place*&source.placement(*old).ok_or("Original placement is missing")?)?;
            let view=Rc::make_mut(&mut candidate.view);view.initial(id,source.visible(*old),source.view.locked.contains(old));
            if let Some(rgb)=source.view.faces.get(old) {view.faces.insert(id,*rgb);}if let Some(rgb)=source.view.edges.get(old) {view.edges.insert(id,*rgb);}
            if source.clouds.iter().any(|row|row.id==*old)&&point_px>0. {candidate.cloud_point_size(id,point_px)?;}
            if source.streamed.iter().any(|row|row.id==*old)&&point_px>0. {candidate.streamed.iter_mut().find(|row|row.id==id).unwrap().point_px=point_px;}
        }
        for doc in &source.forest.documents {
            let mut doc=(**doc).clone();let old_key=doc.key;doc.key=candidate.reserve_document_key()?;
            if !name.is_empty() {doc.label=name.to_owned();}
            let guids=doc.nodes.iter().map(|node|(node.source.guid.clone(),uuid::Uuid::new_v4().to_string())).collect::<std::collections::HashMap<_,_>>();
            for node in &mut doc.nodes {node.object=node.object.map(|id|remap[&id]);node.source.guid=guids[&node.source.guid].clone();node.portable_guid=node.source.guid.clone();node.source.parent_guid=guids.get(&node.source.parent_guid).cloned().unwrap_or_default();}
            if source.created==Some(old_key)&&candidate.created.is_none() {candidate.created=Some(doc.key);}
            candidate.forest.push(doc);
        }
        self.restore(candidate);Ok(())
    }

    pub fn face_rgba(&self,id:ObjectId)->Option<[f32;4]> {
        if let Some(rgb)=self.view.faces.get(&id) {return Some([rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);}
        let owner=self.instances.get(&id)?;
        (owner.instance.flags&session_rust::InstanceRef::FLAG_COLOR!=0).then(|| {let c=&owner.instance.color;[c.r as f32,c.g as f32,c.b as f32,c.a as f32]})
    }

    pub(crate) fn used_guids(&self)->std::collections::HashSet<String> {
        self.objects.iter().map(|r|r.guid.clone()).chain(self.lines.iter().map(|r|r.guid.clone())).chain(self.paths.iter().map(|r|r.guid.clone())).chain(self.points.iter().map(|r|r.guid.clone())).chain(self.clouds.iter().map(|r|r.guid.clone())).chain(self.streamed.iter().map(|r|r.guid.clone())).chain(self.texts.iter().map(|r|r.guid.clone())).collect()
    }

    // Prepare a single placed row before appending it; repeated instances share geometry without cloning the growing destination scene.
    pub(crate) fn append_instance(&mut self,template:&Scene,owner:crate::instance_plan::Owner,model:session_rust::Xform,guid:String,point_px:f32)->Result<ObjectId,&'static str> {
        let mut placed=template.clone();let original=placed.ids.iter().copied().next().ok_or("Missing definition display owner")?;
        if placed.ids.len()!=1 {return Err("One definition must create one drawable owner");}
        placed.place_following(original,model)?;if !placed.clouds.is_empty()&&point_px>0. {placed.cloud_point_size(original,point_px)?;}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);
        let visible=owner.instance.flags&session_rust::InstanceRef::FLAG_HIDDEN==0;let locked=owner.instance.flags&session_rust::InstanceRef::FLAG_LOCKED!=0;
        if let Some(row)=placed.objects.first_mut() {row.metadata=Rc::new(crate::row_metadata::RowMetadata {source_guid:owner.instance.guid().into(),name:owner.instance.name.clone(),is_visible:visible,is_locked:locked});}
        macro_rules! append {($field:ident)=>{if let Some(mut row)=placed.$field.pop() {row.id=id;row.guid=guid.clone();self.$field.push(row);}};}
        append!(objects);append!(lines);append!(paths);append!(points);append!(clouds);
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.instances.insert(id,owner);Ok(id)
    }

    pub(crate) fn reserve_document_key(&mut self)->Result<u32,&'static str> {
        let key=self.next_id;self.next_id=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;Ok(key)
    }

    pub(crate) fn cloud_point_size(&mut self,id:ObjectId,px:f32)->Result<(),&'static str> {
        if !px.is_finite() {return Err("Non-finite cloud point size");}
        self.clouds.iter_mut().find(|row|row.id==id).ok_or("Cloud object not found")?.point_px=px;Ok(())
    }

    pub fn clear(&mut self) {
        self.cad_walks=[0;2];self.instances.clear();
        self.created=None;
        self.ids=Default::default();
        self.view=Default::default();
        self.forest=Default::default();
        self.clouds=Vec::new();self.streamed=Vec::new();
        self.texts=Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
        self.objects = Vec::new();
        self.extra = None;
    }

    pub fn replace(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        self.clear();
        self.import(loaded)
    }

    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        let mut objects=Vec::new();let key=self.next_id;
        for prepared in loaded.meshes {
            let guid=prepared.geometry.guid().to_owned();let name=prepared.geometry.name.clone();
            let id=self.insert(prepared)?;objects.push((guid,id,name));
        }
        self.forest.add(key,loaded.document.name.clone(),loaded.tree,&objects);if let Some(origin)=objects.first().and_then(|(_,id,_)|self.objects.iter().find(|row|row.id==*id)).and_then(|row|row.origin()).cloned() {Rc::make_mut(self.forest.documents.last_mut().unwrap()).header=Some(origin);}let colours=self.forest.colours(key);Rc::make_mut(&mut self.view).faces.extend(colours);Ok(())
    }

    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_mesh(id,model)
    }

    fn place_mesh(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if !crate::placement::valid(&model.m)
        {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        if object.geometry().is_none() { return Err("Reload editable sources before Move"); }
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        self.instances.remove(&id);
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id); self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id); self.points.retain(|row| row.id != id);self.texts.retain(|row|row.id!=id);self.clouds.retain(|row|row.id!=id);self.streamed.retain(|row|row.id!=id);
        if self.extra == Some(id) { self.extra = None; }
        existed
    }

    pub(crate) fn all_ids(&self)->std::collections::HashSet<ObjectId> {(*self.ids).clone()}
    pub fn contains(&self,id:ObjectId)->bool {self.ids.contains(&id)}

    pub fn next(&self,current:Option<ObjectId>)->Option<ObjectId> {
        let mut ids=self.objects.iter().map(|r|r.id).chain(self.lines.iter().map(|r|r.id)).chain(self.paths.iter().map(|r|r.id)).chain(self.points.iter().map(|r|r.id)).chain(self.texts.iter().map(|r|r.id)).chain(self.clouds.iter().map(|r|r.id)).chain(self.streamed.iter().map(|r|r.id)).filter(|id|self.pick_id(id.value()).is_some()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());if ids.is_empty() {return None;}
        let start=ids.iter().position(|id|Some(*id)==current).map_or(0,|i|i+1);Some(ids[start%ids.len()])
    }

    pub fn add_box(&mut self) -> Result<ObjectId, &'static str> {
        let mut source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        source.set_objectcolor(session_rust::Color::new(0.65, 0.65, 0.65, 1.0));
        let id = self.insert(PreparedMesh::new(source)?)?;
        self.place(id, session_rust::Xform::translation(0.9, 0.0, 0.4))?;
        Ok(id)
    }

    pub fn toggle_extra(&mut self) {
        if let Some(id) = self.extra.take() {
            self.remove(id);
        } else {
            let extra = PreparedMesh::triangle([
                [-0.9, 0.3, 0.5], [-0.4, 0.3, 0.5], [-0.65, 0.9, 0.5],
            ], [0.2, 0.8, 0.3]).expect("Valid extra triangle");
            self.extra = Some(self.insert(extra).expect("ID available"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unloading_policy_protects_generated_unlocated_and_modified_sources() {
        let mut scene = Scene::demo(); assert!(!scene.objects[0].can_unload());
        let bytes = crate::specimen::bytes();
        scene.replace(crate::document::load(&bytes).unwrap()).unwrap(); assert!(!scene.objects[0].can_unload());
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:policy".into()));
        scene.replace(crate::document::load_at(&bytes, Some(url)).unwrap()).unwrap();
        assert!(scene.objects.iter().all(Object::can_unload));
        let row = &mut scene.objects[0];
        Rc::make_mut(&mut row.metadata).is_locked = true; assert!(!row.can_unload());
        Rc::make_mut(&mut row.metadata).is_locked = row.geometry().unwrap().is_locked;
        assert!(row.can_unload());
        let source = row.source().cloned(); let geometry = Rc::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0));
        row.editable = crate::edit_source::EditSource::Loaded { geometry, source }; assert!(!row.can_unload());
    }

    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
        let mut scene = Scene::demo();
        let before = scene.objects.clone();
        scene.next_id = u32::MAX - 1;
        let loaded = crate::document::load(&crate::specimen::bytes()).unwrap();
        let mut history = crate::history::History::default();
        assert!(history.try_edit(&mut scene, |scene| scene.replace(loaded)).is_err());
        assert_eq!(scene.next_id, u32::MAX);
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(object.geometry().unwrap(), old.geometry().unwrap()));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
        }
        assert!(!history.undo(&mut scene));
    }

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
        let second = scene.objects()[1].id;
        assert!(scene.remove(first));
        assert_eq!(scene.objects()[0].id, second);
        assert_eq!(scene.next(None), Some(second));
        assert!(!scene.contains(first));
        scene.toggle_extra();
        assert_ne!(scene.objects()[1].id, first);
        assert_ne!(scene.objects()[1].id, second);
    }

    #[test]
    fn toggling_the_extra_object_uses_its_id_after_other_removals() {
        let mut scene = Scene::demo();
        scene.toggle_extra();
        let extra = scene.objects()[2].id;
        scene.remove(scene.objects()[0].id);
        scene.toggle_extra();
        assert!(!scene.contains(extra));
        assert_eq!(scene.objects().len(), 1);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "pick_gpu_tests.rs"]
mod pick_gpu_tests;

impl Scene {
    pub fn deform_part(&mut self,target:crate::pick_sources::Target,delta:&session_rust::Xform) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
            if row.surface.is_some()||row.cad.is_some() {return Err("CAD surface part editing connects after source parameter mapping");}
            let mut source=(**row.geometry().ok_or("Reload editable sources before editing a part")?).clone();
            for key in crate::part_preview::keys(&source,target.part)? {
                let point=local.transform_point(&source.vertex_point(key).ok_or("Missing source vertex")?);
                source.vertex.get_mut(&key).ok_or("Missing source vertex")?.set_position(point);
            }
            source.clear_triangle_bvh();
            let prepared=crate::prepared::PreparedMesh::new(source)?;
            for vertex in prepared.display.vertices() {let point=model.transform_point(&session_rust::Point::new(vertex[0] as f64,vertex[1] as f64,vertex[2] as f64));
                if (0..3).any(|i| !(point[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}}
            row.mesh=Rc::new(prepared.display); row.editable=crate::edit_source::EditSource::Loaded {geometry:prepared.geometry,source:None}; return Ok(());
        }
        let row=self.paths.iter_mut().find(|r|r.id==target.owner).ok_or("Part owner is missing")?;
        let (crate::chain::Source::Curve(curve,_),Part::Control(index))=(&row.prepared.source,target.part) else {return Err("Select a curve control point");};
        let mut source=(**curve).clone(); let point=local.transform_point(&source.get_cv(index).ok_or("Missing curve control")?);
        let weight=source.get_cv_4d(index).ok_or("Missing curve control")?.3;
        let set=if source.m_is_rat {source.set_cv_4d(index,point[0]*weight,point[1]*weight,point[2]*weight,weight)} else {source.set_cv_point(index,&point)};
        if !set {return Err("Cannot set curve control");}
        let prepared=crate::chain::PreparedChain::curve(Rc::new(source))?;
        for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()) {
            let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}
        }
        row.prepared=prepared;Ok(())
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-01.rs"
```

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-02.rs"
```

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-03.rs"
```

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-04.rs"
```

</details>

### 2. `src/document_geometry.rs`

Reuse scoped CAD recordings across original objects and definition templates.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{document_source::Source,scene::{Scene,ObjectId}};
use session_rust::Xform;

pub(crate) fn append(scene:&mut Scene,loaded:&Source,place:&Xform,point_px:f32)->Result<Vec<(String,ObjectId,String)>,String> {
    let mut owners=Vec::new();
    macro_rules! add {($source:expr,$insert:expr)=>{{let source=$source;let guid=source.guid().to_owned();let label=source.name.clone();let model=place*&crate::hierarchy::world(loaded.tree.as_ref(),&loaded.document.xforms,&guid);let id=$insert.map_err(str::to_owned)?;scene.place_following(id,model).map_err(str::to_owned)?;owners.push((guid,id,label));id}};}
    for source in &loaded.document.objects.meshes {
        let mut prepared=crate::prepared::PreparedMesh::from_shared(source.clone()).map_err(str::to_owned)?;
        prepared.source=Some(crate::document::Source {document:loaded.document.clone(),guid:source.guid().to_owned(),origin:loaded.origin.clone()});
        add!(source,scene.insert(prepared));
    }
    for source in &loaded.document.objects.points {add!(source,scene.insert_point(crate::marker::PreparedPoint::new(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.lines {add!(source,scene.insert_line(crate::stroke::PreparedLine::new(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.polylines {add!(source,scene.insert_path(crate::chain::PreparedChain::polyline(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.nurbscurves {add!(source,scene.insert_path(crate::chain::PreparedChain::curve(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.nurbssurfaces {add!(source,scene.insert_surface(source.clone()));}
    for source in &loaded.document.objects.breps {add!(source,scene.insert_cad(source.clone()));}
    for source in &loaded.document.objects.pointclouds {let id=add!(source,scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).map_err(str::to_owned)?));scene.cloud_point_size(id,point_px).map_err(str::to_owned)?;}
    let templates=crate::instance_definition::templates(loaded.document.clone()).map_err(str::to_owned)?;let mut used=scene.used_guids();
    for instance in &loaded.document.objects.instances {
        let template=&templates[&instance.definition_guid];let mut guid=instance.guid().to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}
        // The kernel folds legacy instance placement into Session.xforms; combine that canonical placement with tree ancestry once, without another instance transform.
        let model=place*&crate::hierarchy::world(loaded.tree.as_ref(),&loaded.document.xforms,instance.guid());
        let owner=crate::instance_plan::Owner::new(loaded.document.clone(),instance.clone()).map_err(str::to_owned)?;
        let id=scene.append_instance(template,owner,model,guid,point_px).map_err(str::to_owned)?;owners.push((instance.guid().to_owned(),id,instance.name.clone()));
    }
    Ok(owners)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{document_source::Source,scene::{Scene,ObjectId}};
use session_rust::Xform;

pub(crate) fn append(scene:&mut Scene,loaded:&Source,place:&Xform,point_px:f32)->Result<Vec<(String,ObjectId,String)>,String> {
    let mut owners=Vec::new();let mut shapes=crate::cad_shape_cache::Cache::default();
    macro_rules! add {($source:expr,$insert:expr)=>{{let source=$source;let guid=source.guid().to_owned();let label=source.name.clone();let model=place*&crate::hierarchy::world(loaded.tree.as_ref(),&loaded.document.xforms,&guid);let id=$insert.map_err(str::to_owned)?;scene.place_following(id,model).map_err(str::to_owned)?;owners.push((guid,id,label));id}};}
    for source in &loaded.document.objects.meshes {
        let mut prepared=crate::prepared::PreparedMesh::from_shared(source.clone()).map_err(str::to_owned)?;
        prepared.source=Some(crate::document::Source {document:loaded.document.clone(),guid:source.guid().to_owned(),origin:loaded.origin.clone()});
        add!(source,scene.insert(prepared));
    }
    for source in &loaded.document.objects.points {add!(source,scene.insert_point(crate::marker::PreparedPoint::new(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.lines {add!(source,scene.insert_line(crate::stroke::PreparedLine::new(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.polylines {add!(source,scene.insert_path(crate::chain::PreparedChain::polyline(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.nurbscurves {add!(source,scene.insert_path(crate::chain::PreparedChain::curve(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.nurbssurfaces {add!(source,scene.insert_surface(source.clone()));}
    for source in &loaded.document.objects.breps {add!(source,scene.insert_prepared_cad(shapes.prepare(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.pointclouds {let id=add!(source,scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).map_err(str::to_owned)?));scene.cloud_point_size(id,point_px).map_err(str::to_owned)?;}
    let templates=crate::instance_definition::templates(loaded.document.clone(),&mut shapes).map_err(str::to_owned)?;let mut used=scene.used_guids();
    for instance in &loaded.document.objects.instances {
        let template=&templates[&instance.definition_guid];let mut guid=instance.guid().to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}
        // The kernel folds legacy instance placement into Session.xforms; combine that canonical placement with tree ancestry once, without another instance transform.
        let model=place*&crate::hierarchy::world(loaded.tree.as_ref(),&loaded.document.xforms,instance.guid());
        let owner=crate::instance_plan::Owner::new(loaded.document.clone(),instance.clone()).map_err(str::to_owned)?;
        let id=scene.append_instance(template,owner,model,guid,point_px).map_err(str::to_owned)?;owners.push((instance.guid().to_owned(),id,instance.name.clone()));
    }
    scene.cad_walks=[shapes.walks,shapes.replays];Ok(owners)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-05.rs"
```

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-06.rs"
```

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-07.rs"
```

</details>

### 3. `src/instance_definition.rs`

Reuse scoped CAD recordings across original objects and definition templates.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{instance_plan::Owner,scene::Scene};
use session_rust::Geometry;
use std::{collections::HashMap,rc::Rc};

pub fn prepare(owner:&Owner)->Result<Scene,&'static str> {
    let mut scene=Scene::empty();
    match &owner.definition {
        Geometry::Mesh(source)=>{scene.insert(crate::prepared::PreparedMesh::from_shared(source.clone())?)?;},
        Geometry::BRep(source)=>{scene.insert_cad(source.clone())?;},
        Geometry::NurbsSurface(source)=>{scene.insert_surface(source.clone())?;},
        Geometry::NurbsCurve(source)=>{scene.insert_path(crate::chain::PreparedChain::curve(source.clone())?)?;},
        Geometry::Polyline(source)=>{scene.insert_path(crate::chain::PreparedChain::polyline(source.clone())?)?;},
        Geometry::Line(source)=>{scene.insert_line(crate::stroke::PreparedLine::new(source.clone())?)?;},
        Geometry::Point(source)=>{scene.insert_point(crate::marker::PreparedPoint::new(source.clone())?)?;},
        Geometry::PointCloud(source)=>{scene.insert_cloud(crate::cloud::Prepared::new(source.clone())?)?;},
        _=>return Err("This definition family connects in a later subject"),
    }
    Ok(scene)
}

pub fn templates(document:Rc<session_rust::Session>)->Result<HashMap<String,Scene>,&'static str> {
    let mut templates=HashMap::new();
    for instance in &document.objects.instances {
        if !templates.contains_key(&instance.definition_guid) {
            let owner=Owner::new(document.clone(),instance.clone())?;templates.insert(instance.definition_guid.clone(),prepare(&owner)?);
        }
    }
    Ok(templates)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{instance_plan::Owner,scene::Scene};
use session_rust::Geometry;
use std::{collections::HashMap,rc::Rc};

pub fn prepare(owner:&Owner,shapes:&mut crate::cad_shape_cache::Cache)->Result<Scene,&'static str> {
    let mut scene=Scene::empty();
    match &owner.definition {
        Geometry::Mesh(source)=>{scene.insert(crate::prepared::PreparedMesh::from_shared(source.clone())?)?;},
        Geometry::BRep(source)=>{scene.insert_prepared_cad(shapes.prepare(source.clone())?)?;},
        Geometry::NurbsSurface(source)=>{scene.insert_surface(source.clone())?;},
        Geometry::NurbsCurve(source)=>{scene.insert_path(crate::chain::PreparedChain::curve(source.clone())?)?;},
        Geometry::Polyline(source)=>{scene.insert_path(crate::chain::PreparedChain::polyline(source.clone())?)?;},
        Geometry::Line(source)=>{scene.insert_line(crate::stroke::PreparedLine::new(source.clone())?)?;},
        Geometry::Point(source)=>{scene.insert_point(crate::marker::PreparedPoint::new(source.clone())?)?;},
        Geometry::PointCloud(source)=>{scene.insert_cloud(crate::cloud::Prepared::new(source.clone())?)?;},
        _=>return Err("This definition family connects in a later subject"),
    }
    Ok(scene)
}

pub fn templates(document:Rc<session_rust::Session>,shapes:&mut crate::cad_shape_cache::Cache)->Result<HashMap<String,Scene>,&'static str> {
    let mut templates=HashMap::new();
    for instance in &document.objects.instances {
        if !templates.contains_key(&instance.definition_guid) {
            let owner=Owner::new(document.clone(),instance.clone())?;templates.insert(instance.definition_guid.clone(),prepare(&owner,shapes)?);
        }
    }
    Ok(templates)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-08.rs"
```

```text
--8<-- "journey/code/69ad-share-recordings-only-during-document-preparation-09.rs"
```

</details>

</details>

<a id="checkpoint-69ae-compare-actual-replay-pixels-and-integer-owners"></a>

<details>
<summary>Implementation 69ae-compare-actual-replay-pixels-and-integer-owners: Compare actual replay pixels and integer owners</summary>

Compare actual replay pixels and integer owners.

Data flow: replayed display → independent reference → actual pixels → original integer owner.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/pick_gpu_tests.rs`

Copy actual GPU independent face, boundary ink, integer ownership and source-release comparison.

<details>
<summary>Locate the existing block</summary>

```text
    Rc::make_mut(&mut scene.view).hide(&[b],true);renderer.set_scene(&scene,None);assert_eq!(renderer.instance_usage()[1],1);scene.clear();renderer.set_scene(&scene,None);assert_eq!(&renderer.instance_usage()[..3],&[0,0,0]);assert!(weak.upgrade().is_none());
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/69ae-compare-actual-replay-pixels-and-integer-owners-01.rs" skip=261
```

</details>

</details>

<a id="checkpoint-69af-exercise-independent-original-cad-reuse"></a>

<details>
<summary>Implementation 69af-exercise-independent-original-cad-reuse: Exercise independent original CAD reuse</summary>

Exercise independent original CAD reuse.

Data flow: two original BReps → one walk and replay → independent commands → unchanged saved sources.

<details>
<summary>Supplied example data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 1. `src/cad_shape_example.rs`

Copy congruent original CAD and document persistence/release fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/69af-exercise-independent-original-cad-reuse-01.rs"
```

</details>

### 2. `src/lib.rs`

Register the CAD preparation component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod cad_shape_cache;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/69af-exercise-independent-original-cad-reuse-02.rs" skip=26
```

### 3. `src/editor.rs`

Connect congruent original CAD examples and document-scoped preparation counters.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
```

</details>

Replace it with:

```rust
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
```

<details>
<summary>Locate this block</summary>

```rust
    AddCloud,
    AddCad,
    AddInstances,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
```

</details>

Replace it with:

```rust
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    StreamRefresh,
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Live(Box<Scene>),
    Adopt {scene:Box<Scene>,replace:bool},
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCloudLod,
    AddCloud,
    AddCad,
    AddInstances,
    AddShapeReuse,
    AddText(crate::text::TextPlacement),
    AddSurface,
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
    Preview,
}

pub struct Editor {
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            draft: None, tool_status: String::new(),
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible)||self.scene.streams().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        if !self.scene.contains(id) {return Err("Object not found");}
        let object=self.scene.objects().iter().find(|row|row.id==id);
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.is_some_and(|object|object.geometry().is_none()) {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        self.draft.as_ref()?;
        if text.trim().is_empty() || text.eq_ignore_ascii_case("Escape") { return Some(Ok(Action::CancelTool)); }
        text.contains(',').then(|| crate::tool_coords::parse(text).map(Action::ToolPoint))
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft = self.draft.as_mut().ok_or("No point tool is running")?;
        let name = draft.tool.name();
        let Some(built) = draft.submit(point)? else { self.tool_status = draft.prompt(); self.snap_index=None; return self.update_edit_preview(); };
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            if self.edit_preview.take().is_some() { self.draft=None; } self.tool_status="Move completed.".into();
            return Ok(if changed { Change::Scene } else { Change::Preview });
        }
        self.history.try_edit(&mut self.scene, |scene| match built {
            crate::tool::Built::Point(point) => scene.insert_point(point).map(|_| ()),
            crate::tool::Built::Line(line) => scene.insert_line(line).map(|_| ()),
            crate::tool::Built::Move(_) => unreachable!("Move uses its placement preview"),
        })?;
        self.draft = None; self.tool_status = format!("{name} created."); Ok(Change::Scene)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        hit.or_else(|| crate::snap_index::fallback(self.snapping, &self.camera, screen))
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None; }
        match action {
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit { return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point)); }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None; self.draft=Some(crate::tool::Draft::new(shape)); self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { let changed=self.edit_preview.take().is_some(); self.draft=None; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).chain(self.scene.texts().iter().map(|r|r.id)).chain(self.scene.clouds().iter().map(|r|r.id)).chain(self.scene.streams().iter().map(|r|r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddText(placement)=>{let source=std::rc::Rc::new(crate::scene_label::example(placement));let prepared=crate::scene_label::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;},
            Action::AddCloudLod=>{let source=std::rc::Rc::new(crate::cloud::large_example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddCloud=>{let source=std::rc::Rc::new(crate::cloud::example());let prepared=crate::cloud::Prepared::new(source)?;self.history.try_edit(&mut self.scene,|scene|scene.insert_cloud(prepared).map(|_|()))?;},
            Action::AddShapeReuse=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::cad_shape_example::bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare congruent original CAD"))?;},
            Action::AddInstances=>{self.history.try_edit(&mut self.scene,|scene|crate::scene_document::import(scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).map(|_|()).map_err(|_|"Cannot prepare original definition instances"))?;},
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Live(ready)=>{self.history.clear();self.scene.restore(*ready);},
            Action::Adopt {scene:ready,..}=>{self.history.edit(&mut self.scene,|scene|scene.restore(*ready));},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id)||self.scene.streams().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible)||self.scene.streams().iter().any(|r|Some(r.id)==parent&&r.controls_visible)) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69af-exercise-independent-original-cad-reuse-03.rs"
```

```text
--8<-- "journey/code/69af-exercise-independent-original-cad-reuse-04.rs"
```

</details>

### 4. `src/commands.rs`

Connect congruent original CAD examples and document-scoped preparation counters.

<details>
<summary>Locate the existing block</summary>

```rust
    ("Example CAD",||Action::AddCad),
    ("Example Instances",||Action::AddInstances),
    ("Example Scene Text",||Action::AddText(crate::text::TextPlacement::WorldPlane {world:[-1.5,0.,1.],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.22})),
    ("Example Billboard",||Action::AddText(crate::text::TextPlacement::WorldBillboard {world:[-1.5,0.,1.],world_height:0.22})),
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69af-exercise-independent-original-cad-reuse-05.rs"
```

### 5. `src/browser.rs`

Connect congruent original CAD examples and document-scoped preparation counters.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-text-error",&renderer.text_error().unwrap_or_default())?;
    canvas.set_attribute("data-text-atlas",&serde_json::to_string(&renderer.text_usage()).unwrap())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
    let cads=editor.scene.objects().iter().filter_map(|row|row.cad.as_ref().map(|cad|serde_json::json!({"id":row.id.value(),"source":cad.source.guid(),"faces":row.mesh.parts.faces,"edges":cad.edges.iter().map(|border|border.edge).collect::<Vec<_>>(),"vertices":cad.samples.len()}))).collect::<Vec<_>>();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69af-exercise-independent-original-cad-reuse-06.rs"
```

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 69af-exercise-independent-original-cad-reuse
npm --prefix ../session_tests run course -- check 69af-exercise-independent-original-cad-reuse
npm --prefix ../session_tests run course -- save 69af-exercise-independent-original-cad-reuse
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

Two independent original BReps share one preparation walk. The turned copy is selected yellow, the first remains blue, and both retain their original black bore and outer boundaries. Save/Open preserves both originals.

- **Try this:** Type Close, Example Shape Reuse, Fit and View Isometric; Save/Open, select one, Delete, Undo/Redo and Close.

Continue with [Feature ancestry and canonical local-to-world placement](70-evaluate-source-ancestry-once-per-document.md).
