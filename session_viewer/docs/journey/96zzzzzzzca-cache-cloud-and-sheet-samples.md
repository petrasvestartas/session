# 22.5.10 · Cache cloud and sheet sample recipes

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Cache the cloud and drawing-sheet colour recipes for each physical sample count. Retain original point images, source columns, drawing geometry, placements and IDs. Keep original-owner queries independently single-sample, including the preceding rear/front and wide-fringe corrections.

## Type

Continue from [Compose cloud colour, depth and queries](96zzzzzzzc-compose-cloud-depth-with-physical-addresses.md). [Save or recover your work](recovery.md).

### 1. `src/cloud_gpu.rs`

Cache matching cloud colour recipes while retaining source columns, point images/raster, original-owner queries and current appearance.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT],
  );
  [point, resolve, pick]
}

fn record_bytes(
```

</details>

Replace it with:

```rust
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT],
    1,
  );
  ([point, resolve, pick],resolver)
}

fn record_bytes(
```

<details>
<summary>Locate this block</summary>

```rust
    "vs_point",
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window, &layouts.uniform], "vertex", "color", &[format,crate::primitive_image::FORMAT]);
  let pick = pipeline(
    &ctx.device,
    &resolver,
```

</details>

Replace it with:

```rust
    "vs_point",
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
    1,
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window, &layouts.uniform], "vertex", "color", &[format,crate::primitive_image::FORMAT],1);
  let pick = pipeline(
    &ctx.device,
    &resolver,
```

<details>
<summary>Locate this block</summary>

```rust
  });
  Layouts { uniform, window: window_layout, columns: columns_layout, image: image_layout }
}
fn pipelines(ctx: &GpuCtx, format: wgpu::TextureFormat, layouts: &Layouts) -> [wgpu::RenderPipeline; 3] {
  let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("current production cloud discs"),
    source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0).into()),
```

</details>

Replace it with:

```rust
  });
  Layouts { uniform, window: window_layout, columns: columns_layout, image: image_layout }
}
fn pipelines(ctx: &GpuCtx, format: wgpu::TextureFormat, layouts: &Layouts) -> ([wgpu::RenderPipeline; 3],wgpu::ShaderModule) {
  let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("current production cloud discs"),
    source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0).into()),
```

<details>
<summary>Locate this block</summary>

```rust
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample: Default::default(),
    multiview_mask: None,
    cache: None,
  })
```

</details>

Replace it with:

```rust
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample: wgpu::MultisampleState {count:samples,..Default::default()},
    multiview_mask: None,
    cache: None,
  })
```

<details>
<summary>Locate this block</summary>

```rust
  vertex: &str,
  fragment: &str,
  formats: &[wgpu::TextureFormat],
) -> wgpu::RenderPipeline {
  let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some(fragment),
```

</details>

Replace it with:

```rust
  vertex: &str,
  fragment: &str,
  formats: &[wgpu::TextureFormat],
  samples:u32,
) -> wgpu::RenderPipeline {
  let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some(fragment),
```

<details>
<summary>Locate this block</summary>

```rust
    let Some(t) = &self.targets else {
      return;
    };
    pass.set_pipeline(&self.resolve);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.set_bind_group(2,&self.settings_group,&[]);
```

</details>

Replace it with:

```rust
    let Some(t) = &self.targets else {
      return;
    };
    pass.set_pipeline(&self.resolve.active);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.set_bind_group(2,&self.settings_group,&[]);
```

<details>
<summary>Locate this block</summary>

```rust
      );
      self.targets = Some(Targets { color, depth, ids, size, group });
    }
    self.render_points(ctx, size);
    self.key = Some(key);
    self.preparations += 1;
```

</details>

Replace it with:

```rust
      );
      self.targets = Some(Targets { color, depth, ids, size, group });
    }
    self.set_samples(ctx,self.samples);
    self.render_points(ctx, size);
    self.key = Some(key);
    self.preparations += 1;
```

<details>
<summary>Locate this block</summary>

```rust
      total: 0,
    }
  }
  pub fn set_edl(&mut self,ctx:&GpuCtx,value:f32) {
    let value=if value.is_finite(){value.max(0.)}else{0.};if value==self.edl{return;}
    self.edl=value;ctx.queue.write_buffer(&self.settings,12,&value.to_ne_bytes());
```

</details>

Replace it with:

```rust
      total: 0,
    }
  }
  pub fn set_samples(&mut self,ctx:&GpuCtx,samples:u32) {
    assert!(matches!(samples,1|4));self.samples=samples;if self.targets.is_none(){return;}
    self.resolve.select(samples,||pipeline(&ctx.device,&self.resolver,&[&self.image_layout,&self.pick.get_bind_group_layout(1),&self.point.get_bind_group_layout(0)],"vertex","color",&[self.format,crate::primitive_image::FORMAT],samples));
  }
  pub fn set_edl(&mut self,ctx:&GpuCtx,value:f32) {
    let value=if value.is_finite(){value.max(0.)}else{0.};if value==self.edl{return;}
    self.edl=value;ctx.queue.write_buffer(&self.settings,12,&value.to_ne_bytes());
```

<details>
<summary>Locate this block</summary>

```rust
      resource_group(ctx, &layouts.uniform, "cloud frame settings", [(0, settings.as_entire_binding())]);
    let window = make_buffer("cloud pick origin", 16);
    let window_group = resource_group(ctx, &layouts.window, "cloud pick origin", [(0, window.as_entire_binding())]);
    let [point, resolve, pick] = pipelines(ctx, format, &layouts);
    Self {clipping:clipping.clone(),clip_revision:0,
      point,
      resolve,
      pick,
      settings,
      settings_group,edl:0.25,
```

</details>

Replace it with:

```rust
      resource_group(ctx, &layouts.uniform, "cloud frame settings", [(0, settings.as_entire_binding())]);
    let window = make_buffer("cloud pick origin", 16);
    let window_group = resource_group(ctx, &layouts.window, "cloud pick origin", [(0, window.as_entire_binding())]);
    let ([point, resolve, pick],resolver) = pipelines(ctx, format, &layouts);
    Self {clipping:clipping.clone(),clip_revision:0,
      point,
      resolve:Variants::new(resolve,1),resolver,format,samples:1,
      pick,
      settings,
      settings_group,edl:0.25,
```

<details>
<summary>Locate this block</summary>

```rust
pub struct Lane {
  clipping:wgpu::BindGroup,clip_revision:u64,
  point: wgpu::RenderPipeline,
  resolve: wgpu::RenderPipeline,
  pick: wgpu::RenderPipeline,
  edl:f32,
  settings: wgpu::Buffer,
```

</details>

Replace it with:

```rust
pub struct Lane {
  clipping:wgpu::BindGroup,clip_revision:u64,
  point: wgpu::RenderPipeline,
  resolve:Variants<wgpu::RenderPipeline>,
  resolver:wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32,
  pick: wgpu::RenderPipeline,
  edl:f32,
  settings: wgpu::Buffer,
```

<details>
<summary>Locate this block</summary>

```rust
use crate::{
  cloud::Row,
  cloud_lod::{LodWalk, Projection},
```

</details>

Replace it with:

```rust
use crate::sample_pipelines::Variants;
use crate::{
  cloud::Row,
  cloud_lod::{LodWalk, Projection},
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::sample_pipelines::Variants;
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
  pub anchor:[f64;3],
  pub eye: [f32; 3],
  pub ortho_h: f32,
  pub lod_px: f32,
}
impl Default for View {
  fn default() -> Self {
    Self { anchor:[0.;3],eye: [0., 0., 3.], ortho_h: 0., lod_px: 2. }
  }
}
impl View {
  pub fn camera(camera:&crate::camera::Camera)->Self {Self::anchored(camera,[0.;3])}
  pub fn anchored(camera:&crate::camera::Camera,anchor:[f64;3])->Self {
    let forward = camera.orientation.rotate_vector(session_rust::Vector::y_axis());
    Self {
      anchor,
      eye: std::array::from_fn(|i| (camera.target[i] - anchor[i] - forward[i] * camera.distance) as f32),
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
  anchor:[u64;3],
  transform: [f32; 16],
  size: [u32; 2],
  density: u32,
  eye: [u32; 3],
  ortho: u32,
  lod: u32,
  revision: u64,
  clipping:u64,
}

pub struct Lane {
  clipping:wgpu::BindGroup,clip_revision:u64,
  point: wgpu::RenderPipeline,
  resolve:Variants<wgpu::RenderPipeline>,
  resolver:wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32,
  pick: wgpu::RenderPipeline,
  edl:f32,
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
  pub fn new(ctx: &GpuCtx, format: wgpu::TextureFormat,clipping:&wgpu::BindGroup) -> Self {
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
    let ([point, resolve, pick],resolver) = pipelines(ctx, format, &layouts);
    Self {clipping:clipping.clone(),clip_revision:0,
      point,
      resolve:Variants::new(resolve,1),resolver,format,samples:1,
      pick,
      settings,
      settings_group,edl:0.25,
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
  pub fn set_samples(&mut self,ctx:&GpuCtx,samples:u32) {
    assert!(matches!(samples,1|4));self.samples=samples;if self.targets.is_none(){return;}
    self.resolve.select(samples,||pipeline(&ctx.device,&self.resolver,&[&self.image_layout,&self.pick.get_bind_group_layout(1),&self.point.get_bind_group_layout(0)],"vertex","color",&[self.format,crate::primitive_image::FORMAT],samples));
  }
  pub fn set_edl(&mut self,ctx:&GpuCtx,value:f32) {
    let value=if value.is_finite(){value.max(0.)}else{0.};if value==self.edl{return;}
    self.edl=value;ctx.queue.write_buffer(&self.settings,12,&value.to_ne_bytes());
  }
  pub fn set_clipping(&mut self,revision:u64){self.clip_revision=revision;}
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
      let model=crate::render_space::placed(&row.model,view.anchor).to_f32();
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
    let settings = [1., size[0] as f32, size[1] as f32, self.edl, 0., 0., 0., 0., size[0] as f32, size[1] as f32, 0., 0.];
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
          depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }),
          stencil_ops: None,
        }),
        ..Default::default()
      });
      pass.set_pipeline(&self.point);
      pass.set_bind_group(2,&self.clipping,&[]);
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
      anchor:view.anchor.map(f64::to_bits),
      transform,
      size,
      density: density.to_bits(),
      eye: view.eye.map(f32::to_bits),
      ortho: view.ortho_h.to_bits(),
      lod: view.lod_px.to_bits(),
      revision: self.revision,clipping:self.clip_revision,
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
    self.set_samples(ctx,self.samples);
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
    pass.set_pipeline(&self.resolve.active);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.set_bind_group(2,&self.settings_group,&[]);
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
  samples:u32,
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
      depth_compare: Some(crate::depth::CLOSER_EQUAL),
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample: wgpu::MultisampleState {count:samples,..Default::default()},
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
fn pipelines(ctx: &GpuCtx, format: wgpu::TextureFormat, layouts: &Layouts) -> ([wgpu::RenderPipeline; 3],wgpu::ShaderModule) {
  let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("current production cloud discs"),
    source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0).into()),
  });
  let resolver = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("forward-depth cloud resolve"),
    source: wgpu::ShaderSource::Wgsl(include_str!("cloud_resolve.wgsl").into()),
  });
  let point = pipeline(
    &ctx.device,
    &shader,
    &[&layouts.uniform, &layouts.columns,&crate::clipping_gpu::layout(&ctx.device)],
    "vs_point",
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
    1,
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window, &layouts.uniform], "vertex", "color", &[format,crate::primitive_image::FORMAT],1);
  let pick = pipeline(
    &ctx.device,
    &resolver,
    &[&layouts.image, &layouts.window],
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT],
    1,
  );
  ([point, resolve, pick],resolver)
}

fn record_bytes(
  draw: &Draw,
  transform: [f32; 16],
  density: f32,
  view: View,
  ranges: &[crate::cloud_lod::Range],
) -> Result<(Vec<u8>, u32), &'static str> {
  let row = &draw.row;
  let local=crate::render_space::placed(&row.model,view.anchor);
  let model=local.to_f32();
  let scale = row.model.uniform_scale();
  let mut total = 0u32;
  let m=(&session_rust::Xform::from_matrix(transform.map(f64::from))*&local).to_f32();
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

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="cloud_composite_reference.rs"]
pub(crate) mod composite_reference;
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-20.rs"
```

</details>

### 2. `src/renderer.rs`

Late-created clouds adopt the current count before preparing source records.

<details>
<summary>Locate the existing block</summary>

```rust
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));
        lane.set_samples(&self.text_context(),self.targets.samples);
        lane.set_scene(&self.text_context(),scene,selected)
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-21.rs"
```

</details>

### 3. `src/sheet_gpu.rs`

Retain original drawing, raster/font resources and independent owner queries while caching matching colour recipes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            .collect::<Vec<_>>();
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty(){return;}
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
```

</details>

Replace it with:

```rust
            .collect::<Vec<_>>();
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32) {
        assert!(matches!(samples,1|4));self.samples=samples;if self.draws.is_empty(){return;}
        self.pipeline.select(samples,||crate::sheet_pipeline::colour(device,self.format,samples));
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty(){return;}

        pass.set_pipeline(&self.pipeline.active);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
```

<details>
<summary>Locate this block</summary>

```rust
        }
        drop(old);
        self.cache.retain(|_, owner| owner.strong_count() > 0);
        Ok(())
    }
    pub fn camera(&self, camera: &crate::camera::Camera) {
```

</details>

Replace it with:

```rust
        }
        drop(old);
        self.cache.retain(|_, owner| owner.strong_count() > 0);
        self.set_samples(device,self.samples);
        Ok(())
    }
    pub fn camera(&self, camera: &crate::camera::Camera) {
```

<details>
<summary>Locate this block</summary>

```rust
                            });
                        let group = crate::gpu_layout::group(
                            device,
                            &self.pipeline.get_bind_group_layout(1),
                            &uniform,
                        );
                        Draw {
```

</details>

Replace it with:

```rust
                            });
                        let group = crate::gpu_layout::group(
                            device,
                            &self.pipeline.active.get_bind_group_layout(1),
                            &uniform,
                        );
                        Draw {
```

<details>
<summary>Locate this block</summary>

```rust
        let group = crate::gpu_layout::group(device, &pipeline.get_bind_group_layout(0), &view);
        Self {
            projection: Default::default(),
            pipeline,
            pick,
            view,
            group,
```

</details>

Replace it with:

```rust
        let group = crate::gpu_layout::group(device, &pipeline.get_bind_group_layout(0), &view);
        Self {
            projection: Default::default(),
            pipeline:crate::sample_pipelines::Variants::new(pipeline,1),format,samples:1,
            pick,
            view,
            group,
```

<details>
<summary>Locate this block</summary>

```rust
}
pub struct Lane {
    projection: std::cell::Cell<Option<f32>>,
    pipeline: wgpu::RenderPipeline,
    pick: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    group: wgpu::BindGroup,
```

</details>

Replace it with:

```rust
}
pub struct Lane {
    projection: std::cell::Cell<Option<f32>>,
    pipeline:crate::sample_pipelines::Variants<wgpu::RenderPipeline>,
    format:wgpu::TextureFormat,samples:u32,
    pick: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    group: wgpu::BindGroup,
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{
    scene::{ObjectId, Scene},
    sheet_source::Prepared,
};
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};
use wgpu::util::DeviceExt;
struct Geometry {
    _page: crate::sheet_stream::DisplayPage,
    source: Rc<Prepared>,
    vertices: wgpu::Buffer,
    count: u32,
}
struct Draw {
    id: ObjectId,
    geometry: Rc<Geometry>,
    uniform: wgpu::Buffer,
    group: wgpu::BindGroup,
    bytes: Vec<u8>,
}
pub struct Lane {
    projection: std::cell::Cell<Option<f32>>,
    pipeline:crate::sample_pipelines::Variants<wgpu::RenderPipeline>,
    format:wgpu::TextureFormat,samples:u32,
    pick: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    group: wgpu::BindGroup,
    draws: Vec<Draw>,
    cache: HashMap<(usize, u32), Weak<Geometry>>,
    pub uploads: u64,
    pub uploaded_bytes: u64,
}
impl Lane {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let (pipeline, pick) = crate::sheet_pipeline::pipelines(device, format);
        let view = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sheet view"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = crate::gpu_layout::group(device, &pipeline.get_bind_group_layout(0), &view);
        Self {
            projection: Default::default(),
            pipeline:crate::sample_pipelines::Variants::new(pipeline,1),format,samples:1,
            pick,
            view,
            group,
            draws: Vec::new(),
            cache: HashMap::new(),
            uploads: 0,
            uploaded_bytes: 0,
        }
    }
    fn geometry(
        &mut self,
        device: &wgpu::Device,
        source: &Rc<Prepared>,
        page: crate::sheet_stream::DisplayPage,
        rows: std::ops::Range<u32>,
    ) -> Result<Rc<Geometry>, &'static str> {
        let key = (page.key(), rows.start);
        Ok(
            if let Some(owner) = self.cache.get(&key).and_then(Weak::upgrade) {
                owner
            } else {
                let count = rows.end - rows.start;
                if count as u64 * 32 > device.limits().max_buffer_size {
                    return Err("Sheet display exceeds the device buffer limit");
                }
                let mut bytes = Vec::with_capacity(count as usize * 32);
                for (i, p) in page
                    .display
                    .chunks_exact(6)
                    .enumerate()
                    .skip(rows.start as usize)
                    .take(count as usize)
                {
                    for v in &p[..3] {
                        bytes.extend(v.to_le_bytes());
                    }
                    bytes.extend(page.radius(i).to_le_bytes());
                    for v in &p[3..] {
                        bytes.extend(v.to_le_bytes());
                    }
                    bytes.extend(page.colors[i].to_le_bytes());
                }
                let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("shared original sheet segments"),
                    contents: &bytes,
                    usage: wgpu::BufferUsages::VERTEX,
                });
                self.uploads += 1;
                self.uploaded_bytes += bytes.len() as u64;
                let owner = Rc::new(Geometry {
                    _page: page.clone(),
                    source: source.clone(),
                    vertices,
                    count,
                });
                self.cache.insert(key, Rc::downgrade(&owner));
                owner
            },
        )
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        selected: &[ObjectId],
        bases: &HashMap<(ObjectId, usize), u32>,
    ) -> Result<(), &'static str> {
        self.set_anchored(device,queue,scene,selected,bases,[0.;3])
    }
    pub fn set_anchored(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,scene:&Scene,selected:&[ObjectId],bases:&HashMap<(ObjectId,usize),u32>,anchor:[f64;3])->Result<(),&'static str>{
        let mut old = std::mem::take(&mut self.draws);
        for row in scene
            .sheets()
            .iter()
            .filter(|row| scene.visible(row.id) && row.prepared.original.segment_count > 0)
        {
            for page in row.prepared.pages() {
                for rows in chunks(
                    page.rows.end - page.rows.start,
                    device.limits().max_buffer_size,
                )? {
                    let key = page.key();
                    let geometry =
                        self.geometry(device, &row.prepared, page.clone(), rows.clone())?;
                    let base = bases
                        .get(&(row.id, key))
                        .ok_or("Missing sheet original entity addresses")?
                        .checked_add(rows.start)
                        .ok_or("Drawing pick address overflow")?;
                    let bytes = settings(scene, row, base, selected.contains(&row.id),anchor);
                    let draw = if let Some(i) = old
                        .iter()
                        .position(|d| d.id == row.id && Rc::ptr_eq(&d.geometry, &geometry))
                    {
                        let mut draw = old.swap_remove(i);
                        if draw.bytes != bytes {
                            queue.write_buffer(&draw.uniform, 0, &bytes);
                            draw.bytes = bytes;
                        }
                        draw
                    } else {
                        let uniform =
                            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("independent sheet placement"),
                                contents: &bytes,
                                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                            });
                        let group = crate::gpu_layout::group(
                            device,
                            &self.pipeline.active.get_bind_group_layout(1),
                            &uniform,
                        );
                        Draw {
                            id: row.id,
                            geometry,
                            uniform,
                            group,
                            bytes,
                        }
                    };
                    self.draws.push(draw);
                }
            }
        }
        drop(old);
        self.cache.retain(|_, owner| owner.strong_count() > 0);
        self.set_samples(device,self.samples);
        Ok(())
    }
    pub fn camera(&self, camera: &crate::camera::Camera) {
        let lens = 1.0 / std::f64::consts::FRAC_PI_6.tan();
        let lens = match camera.projection {
            crate::camera::Projection::Perspective => lens,
            crate::camera::Projection::Orthographic => lens / camera.distance,
        };
        self.projection.set(Some(lens as f32));
    }
    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        let projection_y = self.projection.get().unwrap_or_else(|| {
            (transform[1] * transform[1]
                + transform[5] * transform[5]
                + transform[9] * transform[9])
                .sqrt()
        });
        let bytes = transform
            .iter()
            .copied()
            .chain([size[0] as f32, size[1] as f32, density, projection_y])
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>();
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32) {
        assert!(matches!(samples,1|4));self.samples=samples;if self.draws.is_empty(){return;}
        self.pipeline.select(samples,||crate::sheet_pipeline::colour(device,self.format,samples));
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty(){return;}

        pass.set_pipeline(&self.pipeline.active);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
            pass.set_vertex_buffer(0, draw.geometry.vertices.slice(..));
            pass.draw(0..6, 0..draw.geometry.count);
        }
    }
    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup,clipping:&wgpu::BindGroup) {
        if self.draws.is_empty(){return;}
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(3,clipping,&[]);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(2, crop, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
            pass.set_vertex_buffer(0, draw.geometry.vertices.slice(..));
            pass.draw(0..6, 0..draw.geometry.count);
        }
    }
    pub fn usage(&self) -> [u64; 5] {
        let mut unique = std::collections::HashSet::new();
        let bytes = self
            .draws
            .iter()
            .filter(|d| unique.insert(Rc::as_ptr(&d.geometry)))
            .map(|d| d.geometry.vertices.size())
            .sum::<u64>();
        [
            self.draws.len() as u64,
            unique.len() as u64,
            bytes + self.draws.len() as u64 * 96 + 80,
            self.uploads,
            self.uploaded_bytes,
        ]
    }
    pub fn sources(&self) -> impl Iterator<Item = &Rc<Prepared>> {
        self.draws.iter().map(|draw| &draw.geometry.source)
    }
}

pub fn chunks(count: u32, bytes: u64) -> Result<Vec<std::ops::Range<u32>>, &'static str> {
    let size = u32::try_from((bytes / 32).min(u64::from(u32::MAX)))
        .map_err(|_| "Drawing buffer range overflow")?;
    if size == 0 {
        return Err("Device cannot hold one drawing segment");
    }
    let mut out = Vec::new();
    let mut from = 0;
    while from < count {
        let to = from + size.min(count - from);
        out.push(from..to);
        from = to;
    }
    Ok(out)
}
#[cfg(test)]
mod tests {
    #[test]
    fn drawing_buffer_chunks_keep_exact_original_intervals_within_device_limits() {
        assert_eq!(super::chunks(10, 128).unwrap(), vec![0..4, 4..8, 8..10]);
        assert!(super::chunks(1, 31).is_err());
        assert!(super::chunks(0, 32).unwrap().is_empty());
    }
}

fn settings(scene: &Scene, row: &crate::sheet_source::Row, base: u32, selected: bool,anchor:[f64;3]) -> Vec<u8> {
    let mut bytes = crate::render_space::placed(&row.model,anchor)
        .m
        .iter()
        .flat_map(|v| (*v as f32).to_le_bytes())
        .collect::<Vec<_>>();
    bytes.extend(
        scene
            .face_rgba(row.id)
            .unwrap_or([0.; 4])
            .into_iter()
            .flat_map(f32::to_le_bytes),
    );
    bytes.extend(
        [
            scene.pick_id(row.id.value()).map_or(0, ObjectId::value),
            base,
            u32::from(selected),
            0,
        ]
        .into_iter()
        .flat_map(u32::to_le_bytes),
    );
    bytes
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-22.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-23.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-24.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-25.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-26.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-27.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-28.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-29.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-30.rs"
```

</details>

### 4. `src/sheet_pipeline.rs`

Retain original drawing, raster/font resources and independent owner queries while caching matching colour recipes. Add only physical-sample source qualification around the previously typed colour recipe.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        "pick_fragment",
        wgpu::TextureFormat::Rg32Uint,
        None,
    );
    (pipeline, pick)
}
```

</details>

Replace it with:

```rust
        "pick_fragment",
        wgpu::TextureFormat::Rg32Uint,
        None,
        1,
    );
    (pipeline, pick)
}

pub fn colour(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
// original view/model
// original clipping layout
// original expanded source
 let expanded=if samples==4 {expanded.replace("@interpolate(linear)","@interpolate(linear,sample)").replace("@location(8) original_clip:","@location(8) @interpolate(perspective,sample) original_clip:")}else{expanded};
// original source reference
// original shader
// original colour layout
// original render
 pipeline
}
```

<details>
<summary>Locate this block</summary>

```rust
        "fragment",
        format,
        Some(wgpu::BlendState::ALPHA_BLENDING),
    );
    let pick = make(
        device,
```

</details>

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

<details>
<summary>Locate this block</summary>

```rust
    fragment: &str,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("sheet batch"),layout:Some(layout),vertex:wgpu::VertexState {module:shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:32,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x3,3=>Uint32]}]},fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::ink()),multisample:Default::default(),multiview_mask:None,cache:None})
}

pub fn pipelines(
```

</details>

Replace it with:

```rust
    fragment: &str,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
    samples:u32,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("sheet batch"),layout:Some(layout),vertex:wgpu::VertexState {module:shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:32,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x3,3=>Uint32]}]},fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::ink()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}

pub fn pipelines(
```

<details>
<summary>Complete updated block reference</summary>

```rust
pub fn make(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vertex: &str,
    fragment: &str,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
    samples:u32,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("sheet batch"),layout:Some(layout),vertex:wgpu::VertexState {module:shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:32,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x3,3=>Uint32]}]},fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::ink()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}

pub fn pipelines(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {
    let view_layout = crate::gpu_layout::uniform(device, 80);
    let model_layout = crate::gpu_layout::uniform(device, 96);
    let crop_layout = crate::gpu_layout::uniform(device, 80);
    let expanded=crate::finite_project_source::expand(include_str!("sheet.wgsl"));
    let source=expanded.as_str();
    let clip_layout=crate::clipping_gpu::layout(device);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sheet pens"),
        source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(source,2,0).into()),
    });
    let pick_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sheet original entities"),
        source: wgpu::ShaderSource::Wgsl(
            crate::clipping_gpu::source(&format!(
                "{source}\n{}\n{}",
                include_str!("pick_crop.wgsl"),
                include_str!("sheet_pick.wgsl")
            ),3,0)
            .into(),
        ),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sheet pens"),
        bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&clip_layout)],
        immediate_size: 0,
    });
    let pick_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sheet entity picks"),
        bind_group_layouts: &[Some(&view_layout), Some(&model_layout), Some(&crop_layout),Some(&clip_layout)],
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
        1,
    );
    let pick = make(
        device,
        &pick_layout,
        &pick_shader,
        "pick_vertex",
        "pick_fragment",
        wgpu::TextureFormat::Rg32Uint,
        None,
        1,
    );
    (pipeline, pick)
}

pub fn colour(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
// original view/model
// original clipping layout
// original expanded source
 let expanded=if samples==4 {expanded.replace("@interpolate(linear)","@interpolate(linear,sample)").replace("@location(8) original_clip:","@location(8) @interpolate(perspective,sample) original_clip:")}else{expanded};
// original source reference
// original shader
// original colour layout
// original render
 pipeline
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-31.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-34.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-35.rs"
```

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-36.rs"
```

</details>

### 5. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet view/model in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original view/model
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-37.rs"
```

</details>

### 6. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet clipping layout in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original clipping layout
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-38.rs"
```

</details>

### 7. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet expanded source in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original expanded source
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-39.rs"
```

</details>

### 8. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet source reference in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original source reference
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-40.rs"
```

</details>

### 9. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet shader in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original shader
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-41.rs"
```

</details>

### 10. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet colour layout in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original colour layout
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-42.rs"
```

</details>

### 11. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet render in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original render
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-43.rs"
```

</details>

### 12. `src/sheet_pipeline.rs`

Match the alternate sheet colour descriptor to the actual pass.

<details>
<summary>Locate the existing block</summary>

```rust
        Some(wgpu::BlendState::ALPHA_BLENDING),
    );
 pipeline
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        Some(wgpu::BlendState::ALPHA_BLENDING),
        samples,
    );
 pipeline
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-44.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/cloud_address_tests.rs`

Extend every existing physical address/rear-front source/cropped-query assertion to both counts and require real exact sample returns with full cached cloud usage retained.

<details>
<summary>Locate the existing block</summary>

```text
use super::*;
use std::rc::Rc;

pub(super) fn retarget(_r:&mut Renderer,count:u32){assert_eq!(count,1);}
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
            .replace("texture_2d<u32>", "texture_multisampled_2d<u32>")
            .replace("texture_depth_2d", "texture_depth_multisampled_2d");
    }
    code = code.replace("SAMPLE", if count == 4 { "i32(gid.x)" } else { "0" });
    let module = r.device.create_shader_module(wgpu::ShaderModuleDescriptor {
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
    let output = r.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let read = r.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let group = r.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&r.targets.primitive.view),
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
    let mut encoder = r.device.create_command_encoder(&Default::default());
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
        .map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    crate::cap_gpu_tests::wait(r);
    rx.recv().unwrap().unwrap();
    let values = read
        .slice(..)
        .get_mapped_range()
        .chunks_exact(16)
        .take(count as usize)
        .map(|b| {
            std::array::from_fn(|i| u32::from_ne_bytes(b[i * 4..i * 4 + 4].try_into().unwrap()))
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
                    &[[0., 0., -0.3], [1., 0., -1.3], [0., 1., -0.3]],
                    2.,
                )
                .unwrap();
                crate::plane_original::insert(&mut scene, Rc::new(plane)).unwrap();
            }
            let mut r = crate::cap_gpu_tests::renderer(&scene);
            r.set_clipping(clipped, true);
            retarget(&mut r,count);
            let matrix = crate::cap_gpu_tests::matrix();
            let before_pixels = crate::cap_gpu_tests::pixels_at(&r, matrix);
            let before = physical(&r);
            for value in &before {
                let address = value[0] | (value[1] << 16);
                assert!(address > 0);
                assert_eq!(address >= crate::primitive_image::CAP_START, clipped);
            }
            let coords = if clipped {
                vec![0.28, 0.2, -0.585]
            } else {
                vec![0.3, 0.2, -0.1]
            };
            let mut source =
                session_rust::PointCloud::from_coords(coords, vec![0, 0, 255, 255], vec![]);
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
                serde_json::to_vec_pretty(&serde_json::json!({"before": before, "after": after}))
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
                r.take_pick(matrix).unwrap().unwrap().unwrap()[0],
                owner.value()
            );
            assert_eq!(source.to_proto(), original);
        }
    }
}

#[test]
fn actual_nearer_cloud_hides_original_ink_without_hiding_front_sources() {
 let mut rows=Vec::new();
 for density in [1.,2.] {for count in [1] {
  let mut scene=Scene::empty();let face=scene.insert(crate::prepared::PreparedMesh::triangle([[-2.,-2.,0.6],[2.,-2.,0.6],[0.,2.,0.6]],[0.1,0.6,0.1]).unwrap()).unwrap();
  let mut line=session_rust::Line::new(-0.4,0.2,0.6,0.6,0.2,0.6);line.width=6.;line.linecolor=session_rust::Color::new(1.,0.,0.,1.);let line=Rc::new(line);let _=line.guid();let line_before=line.to_proto();let ink=scene.insert_line(crate::stroke::PreparedLine::new(line.clone()).unwrap()).unwrap();
  let mut r=crate::cap_gpu_tests::renderer(&scene);r.set_density(density);retarget(&mut r,count);let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));let before=crate::cap_gpu_tests::pixels_at(&r,matrix);let z_before=physical(&r);let pixel=crate::cap_gpu_tests::at(&before,208,96);assert!(pixel[0]>240&&pixel[1]<10&&pixel[2]<10,"Original front red line exists before cloud: {pixel:?}");
  let coords=(-3..=3).flat_map(|y|(-3..=3).flat_map(move|x|[0.3+x as f64*0.03,0.2+y as f64*0.03,0.2])).collect();let mut points=session_rust::PointCloud::from_coords(coords,[0,0,255,255].repeat(49),vec![]);points.point_size=96.;let points=Rc::new(points);let _=points.guid();let cloud_before=points.to_proto();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(points.clone()).unwrap()).unwrap();r.set_scene(&scene,None);let actual=crate::cap_gpu_tests::pixels_at(&r,matrix);let z_after=physical(&r);for (prior,current)in z_before.iter().zip(&z_after){assert!(f32::from_bits(current[2])>f32::from_bits(prior[2]));assert_eq!([current[0],current[1]],[0,0]);}
  let pixel=crate::cap_gpu_tests::at(&actual,208,96);assert!(pixel[2]>240&&pixel[0]<10&&pixel[1]<10,"Nearer opaque blue cloud hides the original red line: {pixel:?}");r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);let picked=r.take_pick(matrix).unwrap().unwrap();assert_eq!(picked.unwrap()[0],cloud.value());
  let geometry=r.meshes[0].geometry.clone();let cloud_uploads=r.cloud_usage()[2];
  scene.translate(ink,[0.,0.,-0.5]).unwrap();r.set_scene(&scene,None);let front=crate::cap_gpu_tests::pixels_at(&r,matrix);let visible=crate::cap_gpu_tests::at(&front,208,96);assert!(visible[0]>240&&visible[1]<10&&visible[2]<10,"A genuinely nearer original line remains visible: {visible:?}");r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],ink.value());
  scene.translate(ink,[0.,0.,0.5]).unwrap();r.set_scene(&scene,None);assert!(crate::cap_gpu_tests::pixels_at(&r,matrix)==actual,"Restoring original placement restores exact hidden-ink pixels");
  assert_eq!(r.cloud_usage()[2],cloud_uploads,"Original cloud source upload is retained through line placement changes");let cached=r.cloud_usage()[..4].to_vec();
  assert!(crate::cap_gpu_tests::pixels_at(&r,matrix)==actual,"Fresh owner queries retain exact repeated-frame pixels");
  assert!(Rc::ptr_eq(&r.meshes[0].geometry,&geometry));assert_eq!(r.cloud_usage()[..4],cached);crate::sample_test_proof::write(format!("cloud-ink-front-{density}-{count}.rgba"),&front).unwrap();assert_eq!(line.to_proto(),line_before);assert_eq!(points.to_proto(),cloud_before);
  crate::sample_test_proof::write(format!("cloud-ink-fixed-{density}-{count}.rgba"),&actual).unwrap();rows.push(serde_json::json!({"density":density,"samples":count,"physicalBefore":z_before,"physicalAfter":z_after,"actualRgba":pixel,"freshOwner":picked,"cloud":cloud.value(),"ink":ink.value(),"face":face.value()}));
 }}
 for density in [1.,2.] {for count in [1] {
  let mut scene=Scene::empty();scene.insert(crate::prepared::PreparedMesh::triangle([[-2.,-2.,0.6],[2.,-2.,0.6],[0.,2.,0.6]],[0.1,0.6,0.1]).unwrap()).unwrap();
  let mut source=session_rust::Line::new(-0.4,0.,0.2,0.6,0.,0.2);source.width=60.;source.linecolor=session_rust::Color::new(1.,0.,0.,1.);let owner=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(source)).unwrap()).unwrap();
  let mut r=crate::cap_gpu_tests::renderer(&scene);r.set_density(density);retarget(&mut r,count);let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));let actual=crate::cap_gpu_tests::pixels_at(&r,matrix);let visible=crate::cap_gpu_tests::at(&actual,208,96);assert!(visible[0]>240&&visible[1]<10&&visible[2]<10,"Actual wide source fringe is visible outside its query-axis halo: {visible:?}");
  r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);let picked=r.take_pick(matrix).unwrap().unwrap();crate::sample_test_proof::write(format!("cloud-ink-wide-fringe-{density}-{count}.rgba"),&actual).unwrap();crate::sample_test_proof::write(format!("cloud-ink-wide-fringe-{density}-{count}.json"),serde_json::to_vec_pretty(&serde_json::json!({"density":density,"samples":count,"pixel":visible,"freshOwner":picked,"expected":owner.value()})).unwrap()).unwrap();
  assert_eq!(picked.unwrap()[0],owner.value(),"A visible wide source fringe retains its original owner even when its axis lies outside the query halo");
 }}

 crate::sample_test_proof::write("cloud-ink-fixed.json",serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-45.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/cloud_composite_tests.rs`

Retain existing alpha/shading/viewport-edge/default/full-frame/original-owner parity assertions and extend them to all16settings at both physical counts.

<details>
<summary>Locate the existing block</summary>

```text
use super::*;
use std::rc::Rc;
#[test]
fn actual_cloud_composite_matches_current_opaque_and_eye_dome_pixels() {
    let mut records=Vec::new();let mut alpha_differences=0;let mut shaded_differences=0;
    for edges in [false,true] {
    for alpha in [0u8,255] {
        let mut coords=Vec::new();let mut colors=Vec::new();
        for y in 0..30 {for x in 0..40 {coords.extend([if edges {-1.05+x as f64*0.055}else{-0.8+x as f64*0.04},if edges {-1.02+y as f64*0.07}else{-0.6+y as f64*0.04},if (x/4+y/4)%2==0 {0.15}else{0.35}]);colors.extend([80,140,220,i32::from(alpha)]);}}
        let mut cloud=session_rust::PointCloud::from_coords(coords,colors,vec![]);cloud.point_size=10.;let cloud=Rc::new(cloud);let _=cloud.guid();let original=cloud.to_proto();
        let mut scene=Scene::empty();let owner=scene.insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap()).unwrap();let mut r=crate::cap_gpu_tests::renderer(&scene);let ctx=r.text_context();let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));
        for samples in [1] {
            super::cloud_address_tests::retarget(&mut r,samples);let default_pixels=crate::cap_gpu_tests::pixels_at(&r,matrix);let usage=r.cloud_usage();
            for edl in [0.,0.25] {
                r.clouds.borrow_mut().as_mut().unwrap().set_edl(&ctx,edl);
                let full=crate::cap_gpu_tests::pixels_at(&r,matrix);
                let lane=r.clouds.borrow();let lane=lane.as_ref().unwrap();
                let actual=crate::cloud_gpu::composite_reference::pixels(lane,&ctx,samples,false,edl);
                let reference=crate::cloud_gpu::composite_reference::pixels(lane,&ctx,samples,true,edl);
                assert_eq!(full,reference,"Whole rendered cloud equals independent maintained compositor at both counts and EDL modes");
                assert_eq!(actual,reference,"Every RGBA channel matches exact maintained WGSL");
                let rgba=actual.chunks_exact(4).zip(reference.chunks_exact(4)).filter(|(a,b)|a!=b).count();
                let rgb=actual.chunks_exact(4).zip(reference.chunks_exact(4)).filter(|(a,b)|a[..3]!=b[..3]).count();
                let a=actual.chunks_exact(4).zip(reference.chunks_exact(4)).filter(|(a,b)|a[3]!=b[3]).count();
                if alpha==0 && edl==0. {assert_eq!(a,0);alpha_differences+=a;}
                if alpha==255 && edl>0. {assert_eq!(rgb,0);shaded_differences+=rgb;}
                if alpha==255 && edl==0. {assert_eq!(rgba,0);}
                if edl==0.25 {let default_diff=default_pixels.chunks_exact(4).zip(reference.chunks_exact(4)).filter(|(a,b)|a!=b).count();assert_eq!(default_diff,0,"Default whole viewer uses maintained eye-dome shading and opaque cloud output");}
                if !edges&&((alpha==0&&edl==0.&&samples==1)||(alpha==255&&edl>0.&&samples==1)) {for(name,bytes)in[("actual",&actual),("reference",&reference)] {crate::sample_test_proof::write(format!("cloud-parity-{alpha}-{samples}-{name}.rgba"),bytes).unwrap();}}
                records.push(serde_json::json!({"viewportEdges":edges,"sourceAlpha":alpha,"samples":samples,"edl":edl,"rgbaDifferences":rgba,"rgbDifferences":rgb,"alphaDifferences":a}));
            }
            assert_eq!(&r.cloud_usage()[..4],&usage[..4]);r.request_pick([0.,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(cloud.to_proto(),original);
        }
    }
    }
    assert_eq!((alpha_differences,shaded_differences),(0,0));crate::sample_test_proof::write("cloud-parity-observed.json",serde_json::to_vec_pretty(&records).unwrap()).unwrap();
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-46.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/sample_surface_tests.rs`

Retain every prior surface/section assertion and add actual sheet/cloud composition at both counts, complete sources/geometry/cache retention, exact returns and fresh original owners.

<details>
<summary>Locate the existing block</summary>

```text
r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value(),"Single-sample closed-cap owner query is independent of4x crossing resources");
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-47.rs" skip=160
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.10
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original cloud and drawing-sheet colour recipes support matching physical sample counts through the shared cache. Point images/raster, source columns, drawing geometry, placements and full IDs remain retained. Original-owner queries stay one-sample and preserve coherent front/rear/wide-fringe behavior. Current complete browser rendering remains one-sample pending the remaining consumers.

### Native cloud and sheet check

These actual Native GPU frames show cloud occlusion of original faces/sections, original sheet/cloud composition and coherent rear/front source line pixels at the labelled physical count. Independent physical address/depth and original-owner checks accompany the images. Original records, source/geometry uploads, cached one-sample point raster and exact count-return pixels remain strict. The independent compositor also checks every RGBA channel across all16alpha/shading/edge/count settings. Complete browser rendering remains single-sample until all remaining consumers are connected. In this isolated reversed-depth fixture the sheet lies in front of the blue source face, and the red cloud lies in front of both. Its thin blue pen retains the original colour, which has deliberately low contrast against the blue face.

**cloud occludes original face; four physical samples — actual cached cloud/sheet colour check.**

![Actual Native component: cloud occludes original face; four physical samples](../screenshots/journey/review/96zzzzzzzca-cache-cloud-and-sheet-samples-cloud-sheet-0.png)

**cloud occludes original section; four physical samples — actual cached cloud/sheet colour check.**

![Actual Native component: cloud occludes original section; four physical samples](../screenshots/journey/review/96zzzzzzzca-cache-cloud-and-sheet-samples-cloud-sheet-1.png)

**original sheet pens with cached cloud; one sample — actual cached cloud/sheet colour check.**

![Actual Native component: original sheet pens with cached cloud; one sample](../screenshots/journey/review/96zzzzzzzca-cache-cloud-and-sheet-samples-cloud-sheet-2.png)

**original sheet pens with cached cloud; four samples — actual cached cloud/sheet colour check.**

![Actual Native component: original sheet pens with cached cloud; four samples](../screenshots/journey/review/96zzzzzzzca-cache-cloud-and-sheet-samples-cloud-sheet-3.png)

**rear original line hidden by cloud; density2/four samples — actual cached cloud/sheet colour check.**

![Actual Native component: rear original line hidden by cloud; density2/four samples](../screenshots/journey/review/96zzzzzzzca-cache-cloud-and-sheet-samples-cloud-sheet-4.png)

**genuinely front original line keeps colour/owner; density2/four samples — actual cached cloud/sheet colour check.**

![Actual Native component: genuinely front original line keeps colour/owner; density2/four samples](../screenshots/journey/review/96zzzzzzzca-cache-cloud-and-sheet-samples-cloud-sheet-5.png)


**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample contact browser fixture after cached original cloud and sheet colour recipes. Separate labelled Native figures show original face/cap occlusion, rear/front line ownership and sheet/cloud composition at the physical counts. Complete browser multisampling remains pending.

![Actual single-sample contact browser fixture after cached original cloud and sheet colour recipes. Separate labelled Native figures show original face/cap occlusion, rear/front line ownership and sheet/cloud composition at the physical counts. Complete browser multisampling remains pending.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The cloud owner reuses its existing point and query layouts for the alternate colour recipe. Empty owners defer unused variants until actual point images exist. Late-created clouds adopt the current frame count before preparation. Count-only switches preserve original point columns, cached one-sample raster images and their preparation counts; opaque output, eye-dome shading and query depth order keep their existing meaning.

Reuse the previously typed sheet view/model, clipping, source, shader, layout and descriptor references in a small alternate colour factory. Sample-qualified coverage and original world positions match the active pass. The sheet owner remembers the requested count even when empty and retains its source vertices, independent placements, full original entity IDs and query recipe.

Extend the supplied original cloud witnesses to both counts without dropping any one-sample, source-alpha/shading/edge, physical-address, rear/front or wide-fringe assertion. Add actual sheet/cloud composition, fresh original owners, exact count-return pixels, retained original geometry/uploads and cached point preparation. Current main browser rendering remains one-sample until the remaining consumers and production count API are connected.

Original cloud/drawing sources → retained point/pen resources → cached count-specific colour recipes → independent original owners.



Why keep original point images and owner queries single-sample while changing colour recipes?

They represent retained original point preparation and independent exact owner addresses. The colour recipe must match the physical frame attachment count; selecting it should not rebuild those sources or change the query lane.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run actual cloud face/cap, reference RGBA, rear/front/wide-fringe and sheet/cloud witnesses at both counts. Switch4→1→4 and require exact return pixels, unchanged source uploads/geometry and cached point preparation. Inspect the separate Native physical-address, cloud ink and sheet pictures; they are distinct from the canonical single-sample browser scene.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.10
npm --prefix ../session_tests run course -- save 22.5.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Cloud and sheet colour recipes use the shared two-count cache and match physical frame attachments. Immutable point/query layouts and exact existing sheet descriptors are reused. Original point columns/images/raster preparation, drawing geometry, placements, full IDs and independent one-sample query lanes remain retained. Empty cloud owners defer unused variants, late cloud data adopts the current count, and empty sheets remember it. Actual original cloud physical depth/zero words, dense rear/front and wide-fringe colour/query controls, maintained compositor RGBA defaults/edges/all16settings, actual sheet composition, source/geometry/cache retention and exact count-return pixels remain strict at both counts. Complete main rendering remains one-sample until all consumers and the production API are connected. All 1001 Native tests, WASM/optimized builds, Native frame, canonical Chrome contact/resize checks and stored evidence pass. Ten current canonical Native/browser pictures were individually inspected. Every unique current Native component figure was also individually inspected. Complete physical-sample outline/ambient/viewer integration, adaptive quality, remaining view controls, runtime efficiency, Cargo independence and final whole-course feature-picture review remain unfinished. Maintained Session reload camera/base-grid fixes follow the completed course.

Retain all preceding actual source/opaque/eye-dome/query/halo assertions and extend them to physical counts1/4. Add actual original sheet/cloud composition, full source/geometry/cache retention, exact count returns, late creation and fresh original owners.

Actual single-sample contact browser fixture after cached original cloud and sheet colour recipes. Separate labelled Native figures show original face/cap occlusion, rear/front line ownership and sheet/cloud composition at the physical counts. Complete browser multisampling remains pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.10
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
