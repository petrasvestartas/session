# 22.5.9 · Compose cloud colour, depth and queries

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 30–59 minutes.** [Estimate](typing-load.md).

Make the visible cloud and its original-owner queries agree with actual depth. A nearer cloud hides rear source ink, while a genuinely nearer line remains visible and pickable. Preserve full original point records and cached point images.

## Type

Continue from [Read section crossings at each sample](96zzzzzzzb-read-section-crossings-at-each-sample.md). [Save or recover your work](recovery.md).

### 1. `src/cloud_gpu.rs`

Keep cloud colour, physical depth/addresses and original-owner queries coherent without changing original sources or adding GPU images.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
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
```

</details>

Replace it with:

```rust
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window, &layouts.uniform], "vertex", "color", &[format,crate::primitive_image::FORMAT]);
  let pick = pipeline(
    &ctx.device,
    &resolver,
    &[&layouts.image, &layouts.window],
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT],
  );
  [point, resolve, pick]
}
```

<details>
<summary>Locate this block</summary>

```rust
    pass.set_pipeline(&self.resolve);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.draw(0..3, 0..1);
  }
  pub fn pick(
```

</details>

Replace it with:

```rust
    pass.set_pipeline(&self.resolve);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.set_bind_group(2,&self.settings_group,&[]);
    pass.draw(0..3, 0..1);
  }
  pub fn pick(
```

<details>
<summary>Locate this block</summary>

```rust
    Ok(())
  }
  fn render_points(&self, ctx: &GpuCtx, size: [u32; 2]) {
    let settings = [1., size[0] as f32, size[1] as f32, 0., 0., 0., 0., 0., size[0] as f32, size[1] as f32, 0., 0.];
    ctx.queue.write_buffer(&self.settings, 0, &settings.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());
    let t = self.targets.as_ref().unwrap();
    let mut encoder = ctx.device.create_command_encoder(&Default::default());
```

</details>

Replace it with:

```rust
    Ok(())
  }
  fn render_points(&self, ctx: &GpuCtx, size: [u32; 2]) {
    let settings = [1., size[0] as f32, size[1] as f32, self.edl, 0., 0., 0., 0., size[0] as f32, size[1] as f32, 0., 0.];
    ctx.queue.write_buffer(&self.settings, 0, &settings.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());
    let t = self.targets.as_ref().unwrap();
    let mut encoder = ctx.device.create_command_encoder(&Default::default());
```

<details>
<summary>Locate this block</summary>

```rust
      total: 0,
    }
  }
  pub fn set_clipping(&mut self,revision:u64){self.clip_revision=revision;}
  pub fn set_scene(
    &mut self,
```

</details>

Replace it with:

```rust
      total: 0,
    }
  }
  pub fn set_edl(&mut self,ctx:&GpuCtx,value:f32) {
    let value=if value.is_finite(){value.max(0.)}else{0.};if value==self.edl{return;}
    self.edl=value;ctx.queue.write_buffer(&self.settings,12,&value.to_ne_bytes());
  }
  pub fn set_clipping(&mut self,revision:u64){self.clip_revision=revision;}
  pub fn set_scene(
    &mut self,
```

<details>
<summary>Locate this block</summary>

```rust
      resolve,
      pick,
      settings,
      settings_group,
      columns_layout: layouts.columns,
      image_layout: layouts.image,
      window,
```

</details>

Replace it with:

```rust
      resolve,
      pick,
      settings,
      settings_group,edl:0.25,
      columns_layout: layouts.columns,
      image_layout: layouts.image,
      window,
```

<details>
<summary>Locate this block</summary>

```rust
  point: wgpu::RenderPipeline,
  resolve: wgpu::RenderPipeline,
  pick: wgpu::RenderPipeline,
  settings: wgpu::Buffer,
  settings_group: wgpu::BindGroup,
  columns_layout: wgpu::BindGroupLayout,
```

</details>

Replace it with:

```rust
  point: wgpu::RenderPipeline,
  resolve: wgpu::RenderPipeline,
  pick: wgpu::RenderPipeline,
  edl:f32,
  settings: wgpu::Buffer,
  settings_group: wgpu::BindGroup,
  columns_layout: wgpu::BindGroupLayout,
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
  resolve: wgpu::RenderPipeline,
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
    let [point, resolve, pick] = pipelines(ctx, format, &layouts);
    Self {clipping:clipping.clone(),clip_revision:0,
      point,
      resolve,
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
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window, &layouts.uniform], "vertex", "color", &[format,crate::primitive_image::FORMAT]);
  let pick = pipeline(
    &ctx.device,
    &resolver,
    &[&layouts.image, &layouts.window],
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT],
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
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-09.rs"
```

</details>

### 2. `src/cloud_resolve.wgsl`

Keep cloud colour, physical depth/addresses and original-owner queries coherent without changing original sources or adding GPU images.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(0) @binding(0) var sdepth:texture_depth_2d;
@group(0) @binding(1) var scolor:texture_2d<f32>;
@group(0) @binding(2) var sids:texture_2d<u32>;
struct Window {origin:vec2<u32>,mode:u32,padding:u32}
@group(1) @binding(0) var<uniform> window:Window;
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32> {
    let xy=vec2<f32>(f32((i<<1u)&2u),f32(i&2u));return vec4<f32>(xy*2.0-1.0,0.0,1.0);
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

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var sdepth:texture_depth_2d;
@group(0) @binding(1) var scolor:texture_2d<f32>;
@group(0) @binding(2) var sids:texture_2d<u32>;
struct Window {origin:vec2<u32>,mode:u32,padding:u32}
@group(1) @binding(0) var<uniform> window:Window;
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32> {
    let xy=vec2<f32>(f32((i<<1u)&2u),f32(i&2u));return vec4<f32>(xy*2.0-1.0,0.0,1.0);
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-16.rs"
```

</details>

### 3. `src/renderer.rs`

Skip absent cloud lanes before opening their composite pass. Cloud colour/depth replaces occluded physical surface words with zero in the same pass. Draw the lane already borrowed by the pass guard.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.targets.primitive.view,&self.targets.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.targets.depth,&self.view_group,&clip_group,&clip.buffer,&self.targets.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
```

</details>

Replace it with:

```rust
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.targets.primitive.view,&self.targets.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        if let Some(clouds)=self.clouds.borrow().as_ref(){let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&self.targets.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});clouds.draw(&mut pass);}
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.targets.depth,&self.view_group,&clip_group,&clip.buffer,&self.targets.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    surface_options:std::cell::Cell<[bool;2]>,
    world_bounds:std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space:crate::render_space::Origin,
    opacity:std::cell::Cell<f32>,
    ambient_enabled:std::cell::Cell<bool>,
    ambient:std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
    ambient_rows:crate::ambient_rows::Rows,
    pyramid:std::cell::RefCell<Option<crate::pyramid_gpu::Pyramid>>,
    pyramid_revision:u64,
    caps:std::cell::RefCell<crate::caps_gpu::Caps>,
    clipping:std::cell::RefCell<crate::clipping_gpu::Clipping>,
    tiles:std::cell::RefCell<crate::tile_gpu::Tiles>,
    guides:crate::guide_gpu::Guide,
    measurement:crate::measure_gpu::Measurement,
    sheets:crate::sheet_gpu::Lane,
    clouds:std::cell::RefCell<Option<crate::cloud_gpu::Lane>>,
    cloud_view:std::cell::Cell<crate::cloud_gpu::View>,
    cloud_error:std::cell::RefCell<Option<String>>,
    outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    global_outline:std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    outlines_enabled:std::cell::Cell<bool>,
    outline_composite:std::cell::RefCell<Option<crate::outline_composite::Composite>>,
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    full_text_fonts:std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    output_format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    surfaces:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::RenderPipeline)>,
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
    targets:crate::frame_targets::Targets,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,grid_anchor:[f64;3],
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode:crate::pick_sources::Mode,
    pick_cursor:Option<([f32;2],[u64;3])>,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
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
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let surfaces=crate::sample_pipelines::Variants::new(crate::surface_pipelines::pipelines(&device,format,&view_layout,&model_layout,1),1);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: crate::surface_view::BYTES,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &surfaces.active.0.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let targets = crate::frame_targets::Targets::with_formats(&device,[640,480],output_format,format,1);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,output_format,device, queue, surfaces, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
        for lane in [&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format,false);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size == [size.width, size.height] { return; }
        self.cancel_pick();
        self.size = [size.width, size.height];
        self.targets=crate::frame_targets::Targets::with_formats(&self.device,self.size,self.output_format,self.format,self.targets.samples);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.cancel_pick(); self.density = density; }
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

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let anchor=self.anchor();
        self.ambient_rows.set(scene);
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.surfaces.active.0.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.surfaces.active.0.get_bind_group_layout(1);
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
                let geometry = self.geometry.get(&crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue), &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry_at(&self.device,&layout,object,
                    style(scene,object.id,selected==Some(object.id)),geometry,anchor)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set_anchored(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases,anchor).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
        *self.world_bounds.borrow_mut()=scene.bounds();
    }

    pub fn grow_camera(&self,camera:&mut crate::camera::Camera) {
        if let Some(bounds)=self.world_bounds.borrow().as_ref() {camera.grow_extent(bounds);}
    }

    pub fn camera_matrix(&self,camera:&crate::camera::Camera)->[f32;16] {camera.view_projection_anchored(self.anchor()).to_f32()}
    pub fn rebase_delay(&self,now:f64)->Option<u32> {self.space.retry_after_ms(now)}
    pub fn prepare_origin(&mut self,camera:&crate::camera::Camera,scene:&Scene,target:Option<crate::pick_sources::Target>,ids:&[ObjectId],now:f64)->Result<crate::render_space::Rebase,&'static str> {
        let previous=self.anchor();let mut frame=self.space.update(camera.target,camera.distance,now).ok_or("Invalid camera origin")?;
        frame.moved&=previous.map(f64::to_bits)!=frame.anchor.map(f64::to_bits);
        if frame.moved {
            self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
            for mesh in &mut self.meshes {let[calls,bytes]=mesh.rebase(&self.queue,frame.anchor);self.settings_writes+=calls;self.settings_bytes+=bytes;}
            self.set_source_ink(scene);
            if !self.highlight.rebase_face(&self.queue,frame.anchor,target){self.set_highlight(scene,target)?;}
            self.select_objects(scene,ids)?;
            self.set_grid_on(self.grid_settings,self.grid_plane)?;
        }
        self.cloud_camera(camera);Ok(frame)
    }

    fn set_source_ink(&mut self,scene:&Scene) {
        let anchor=self.anchor();
        self.sources = crate::pick_sources::Sources::anchored(scene,anchor).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, std::mem::take(&mut self.sources.mesh_markers).into_iter().chain(scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker})).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        let origin=self.anchor();self.picker.queue_anchored(crate::pick_gpu::Request{window,transform,density:self.density,mode:self.pick_mode},origin)?;
        self.pick_cursor=Some((screen,origin.map(f64::to_bits)));self.pump_pick(transform);Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) else { self.cancel_pick(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let bytes=crate::surface_view::bytes(transform,[0.;4],self.size,self.density);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.targets.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) { Ok(view) => view, Err(error) => {
            self.cancel_pick(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit)))=>{self.pick_cursor=None;Some(Ok(hit))},
            Some(Ok(crate::pick_gpu::Reply::Stale))=>{
                if let Some((screen,_))=self.pick_cursor.filter(|(_,origin)|*origin!=self.anchor().map(f64::to_bits)) {
                    if let Err(error)=self.request_pick(screen,transform){self.cancel_pick();return Some(Err(error));}
                } else {
                    self.pump_pick(transform);
                    if self.picker.usage()[0]==0&&!self.picker.queued(){self.pick_cursor=None;}
                }
                None
            },
            Some(Err(error))=>{self.cancel_pick();Some(Err(error))},None=>None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.cancel_pick(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) {self.pick_cursor=None;self.picker.cancel();}

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self)->[u64;4]{let mut usage=crate::memory::gpu(&self.meshes);usage[1]+=self.geometry.allocated_bytes();usage}
    pub fn arena_cpu_bytes(&self)->usize{self.geometry.cpu_bytes()}

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        let anchor=self.anchor();
        if settings==self.grid_settings&&plane==self.grid_plane&&anchor==self.grid_anchor{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on_at(plane,anchor)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;self.grid_anchor=anchor;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.surfaces.active.0.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        let anchor=self.anchor();
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects_at(&self.device,scene,ids,anchor)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{let anchor=self.anchor();if self.measurement.set_at(&self.device,mark,anchor)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")?;}Ok(())}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self,draft:Option<&crate::tool::Draft>)->Result<(),&'static str>{let anchor=self.anchor();self.preview.set_at(&self.device,draft,anchor)}
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let anchor=self.anchor();let matrix=camera.view_projection_anchored(anchor);
        let layout=origin.map(|p|std::array::from_fn(|i|p[i]-anchor[i])).and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:self.anchor(),framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
    fn adopt_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();self.text_error.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(&self,mut labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        for (i,label) in labels.iter_mut().enumerate() {label.id=u32::MAX-i as u32;label.object=None;}
        let mut merged=self.scene_text.borrow().clone();merged.extend(labels.clone());self.adopt_text(self.merge_measurement(merged))?;*self.presentation_text.borrow_mut()=labels;Ok(())
    }
    fn set_scene_text(&self,scene:&Scene,selected:&[ObjectId])->anyhow::Result<()> {
        let labels=scene.text_labels(selected).map_err(anyhow::Error::msg)?;let mut merged=labels.clone();merged.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(merged))?;*self.scene_text.borrow_mut()=labels;Ok(())
    }
    pub fn refresh_text_bounds(&self,scene:&mut Scene)->bool {
        let mut changed=false;
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {
            let extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);
            if row.prepared.extent!=extent {row.prepared.extent=extent;changed=true;}
        }}}
        if changed {*self.world_bounds.borrow_mut()=scene.bounds();}
        changed
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::anchored(camera,self.anchor()));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.cancel_pick();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
    pub fn set_lighting(&self,on:bool,backfaces:bool){self.surface_options.set([on,backfaces]);}
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));}else if let Some(owner)=ambient.as_mut(){owner.clear();}}
    pub fn ambient_usage(&self)->[u64;5]{self.ambient.borrow().as_ref().map_or([0;5],crate::ambient_gpu::Ambient::usage)}
    pub fn ambient_error(&self)->Option<&'static str>{self.ambient.borrow().as_ref().and_then(|owner|owner.error)}
    pub fn set_depth_pyramid(&self,enabled:bool){let mut owner=self.pyramid.borrow_mut();if enabled {if owner.is_none(){*owner=Some(crate::pyramid_gpu::Pyramid::new(&self.device));}}else{*owner=None;}}
    pub fn depth_pyramid_usage(&self)->[u64;4]{self.pyramid.borrow().as_ref().map_or([0;4],crate::pyramid_gpu::Pyramid::usage)}
    pub fn depth_pyramid_error(&self)->Option<&'static str>{self.pyramid.borrow().as_ref().and_then(|p|p.error)}
    pub fn cap_error(&self)->Option<&'static str>{self.caps.borrow().pick_error}
    pub fn cap_usage(&self)->[u64;5]{self.caps.borrow().usage()}
    pub fn clipping_usage(&self)->[u64;5]{let clip=self.clipping.borrow();[clip.rows.len()as u64,u64::from(clip.enabled),u64::from(clip.solid),clip.writes,clip.buffer.size()]}
    pub fn clipping_ink_bytes(&self)->[u64;2]{[&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid].into_iter().map(crate::stroke_gpu::Lane::clipping_bytes).fold(self.highlight.clipping_bytes(),|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn clipping_error(&self)->Option<&'static str>{self.clipping.borrow().error}
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;3] {self.instances.allocation_parts()}
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
        let output=view;let view=self.targets.color(output);
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let options=self.surface_options.get();
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.targets.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers]{lane.finite_frame(&self.device,&tables,&self.targets.depth);}self.highlight.finite_frame(&self.device,&tables,&self.targets.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.targets.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
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
                }),Some(wgpu::RenderPassColorAttachment{
                    view:&self.targets.primitive.view,depth_slice:None,resolve_target:None,
                    ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
                })],
                ..Default::default()
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.surfaces.active.0);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.surfaces.active.0);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.surfaces.active.1); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.targets.primitive.view,&self.targets.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        if let Some(clouds)=self.clouds.borrow().as_ref(){let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&self.targets.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});clouds.draw(&mut pass);}
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.targets.depth,&self.view_group,&clip_group,&clip.buffer,&self.targets.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass,&self.view_group);
            self.mesh_ink.draw(&mut pass,&self.view_group);
            self.strokes.draw(&mut pass,&self.view_group);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass,&self.view_group);
            self.markers.draw(&mut pass,&self.view_group);
            self.highlight.points(&mut pass,&self.view_group);
            self.preview.draw(&mut pass,&self.view_group);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.targets.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.targets.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.text_context(),&self.text_frame(*transform)) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("scene glyphs"),depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&self.targets.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.targets.resolve(&mut encoder,output);
        self.widget.draw(&mut encoder,output);
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.targets.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {source_faces:!scene.forest.derived(id),selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}

#[cfg(all(test,not(target_arch="wasm32")))]
impl Renderer {
 pub fn outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(false)}
 pub fn global_outline_alpha(&self)->Vec<u8>{self.read_outline_alpha(true)}
 fn read_outline_alpha(&self,global:bool)->Vec<u8> {
  let owner=if global{self.global_outline.borrow()}else{self.outline.borrow()};let Some(outline)=owner.as_ref() else {return Vec::new();};let Some(texture)=outline.alpha_texture() else {return Vec::new();};let [width,height]=self.size;let pitch=width.div_ceil(256)*256;
  let buffer=self.device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual outline alpha readback"),size:pitch as u64*height as u64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=self.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},wgpu::Extent3d {width,height,depth_or_array_layers:1});self.queue.submit([encoder.finish()]);let (tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());self.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=buffer.slice(..).get_mapped_range();let pixels=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize].iter().copied()).collect();drop(bytes);buffer.unmap();pixels
 }
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="pyramid_gpu_tests.rs"]
mod pyramid_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ambient_gpu_tests.rs"]
mod ambient_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="global_outline_tests.rs"]
mod global_outline_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="opacity_gpu_tests.rs"]
mod opacity_gpu_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="camera_extent_tests.rs"]
mod camera_extent_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_lighting_tests.rs"]
mod surface_lighting_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_controls_gpu_tests.rs"]
mod surface_controls_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="shared_view_tests.rs"]
mod shared_view_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="contact_feature_tests.rs"]
mod contact_feature_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_metadata_tests.rs"]
mod ink_metadata_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="contact_boundary_tests.rs"]
mod contact_boundary_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="source_face_policy_tests.rs"]
mod source_face_policy_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="source_pen_tests.rs"]
mod source_pen_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="round_caps_tests.rs"]
mod round_caps_tests;
#[cfg(test)]
#[path="mesh_marker_tests.rs"]
mod mesh_marker_tests;
#[cfg(test)]
#[path="mesh_density_tests.rs"]
mod mesh_density_tests;
#[cfg(test)]
#[path="ambient_input_tests.rs"]
mod ambient_input_tests;
#[cfg(test)]
#[path="ambient_order_tests.rs"]
mod ambient_order_tests;
#[cfg(test)]
#[path="ambient_contact_tests.rs"]
mod ambient_contact_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="frame_target_tests.rs"]
mod frame_target_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="sample_surface_tests.rs"]
mod sample_tests;
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-19.rs"
```

</details>

### 4. `src/sheet_gpu.rs`

Skip empty sheet lanes before binding their unused colour or query recipe.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        }
    }
    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup,clipping:&wgpu::BindGroup) {
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(3,clipping,&[]);
        pass.set_bind_group(0, &self.group, &[]);
```

</details>

Replace it with:

```rust
        }
    }
    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup,clipping:&wgpu::BindGroup) {
        if self.draws.is_empty(){return;}
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(3,clipping,&[]);
        pass.set_bind_group(0, &self.group, &[]);
```

<details>
<summary>Locate this block</summary>

```rust
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
```

</details>

Replace it with:

```rust
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty(){return;}
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
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
    pipeline: wgpu::RenderPipeline,
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
            pipeline,
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
                            &self.pipeline.get_bind_group_layout(1),
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
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty(){return;}
        pass.set_pipeline(&self.pipeline);
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
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-21.rs"
```

</details>

### 5. `src/finite_visibility.wgsl`

Keep cloud colour, physical depth/addresses and original-owner queries coherent without changing original sources or adding GPU images.

<details>
<summary>Locate the existing block</summary>

```wgsl
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(ink_primitive(source_at,sample)==0u&&ink_depth(source_at,sample)>axis.depth+tolerance){return false;}
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-22.rs"
```

</details>

### 6. `src/pick_encode.rs`

Reuse the existing readonly integer overlay query pass. Cloud depth is established before readonly original ink queries. Write actual cloud query depth and zero physical words before the finite query reads them. Reuse the existing query pass and attachments through a stack-only helper.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite integer ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("other original integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});
            if let Some(clouds)=clouds {clouds.pick(ctx,&mut pass,window,mode);}
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
        if let Some(clouds)=clouds{let mut pass=overlay(&mut encoder,image,"original cloud query depth",true);clouds.pick(ctx,&mut pass,window,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=overlay(&mut encoder,image,"finite integer ink",false);
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=overlay(&mut encoder,image,"other original integer owners",false);
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}

fn overlay<'a>(encoder:&'a mut wgpu::CommandEncoder,image:&'a Image,label:&'a str,physical:bool)->wgpu::RenderPass<'a>{
 let colours=[
  // original owner attachment,
  // original primitive attachment,
 ];
 // original owner pass
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-23.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-24.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-26.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-27.rs"
```

</details>

### 7. `src/pick_encode.rs`

Reuse the exact previously typed owner attachment in the shared query pass.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original owner attachment
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-28.rs"
```

</details>

### 8. `src/pick_encode.rs`

Reuse the exact previously typed primitive attachment in the shared query pass.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original primitive attachment
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-29.rs"
```

</details>

### 9. `src/pick_encode.rs`

Reuse the exact previously typed owner pass in the shared query pass.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original owner pass
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-30.rs"
```

</details>

### 10. `src/pick_encode.rs`

Keep cloud colour, physical depth/addresses and original-owner queries coherent without changing original sources or adding GPU images.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
        if let Some(clouds)=clouds{let mut pass=overlay(&mut encoder,image,"original cloud query depth",true);clouds.pick(ctx,&mut pass,window,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=overlay(&mut encoder,image,"finite integer ink",false);
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=overlay(&mut encoder,image,"other original integer owners",false);
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}

fn overlay<'a>(encoder:&'a mut wgpu::CommandEncoder,image:&'a Image,label:&'a str,physical:bool)->wgpu::RenderPass<'a>{
 let colours=[
  Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),
  Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}),
 ];
 encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("other original integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()})
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
        if let Some(clouds)=clouds{let mut pass=overlay(&mut encoder,image,"original cloud query depth",true);clouds.pick(ctx,&mut pass,window,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=overlay(&mut encoder,image,"finite integer ink",false);
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=overlay(&mut encoder,image,"other original integer owners",false);
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}

fn overlay<'a>(encoder:&'a mut wgpu::CommandEncoder,image:&'a Image,label:&'a str,physical:bool)->wgpu::RenderPass<'a>{
 let colours=[
  Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),
  Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),
 ];
 encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some(label),color_attachments:&colours[..if physical{2}else{1}],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:physical.then_some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()})
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-31.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-34.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-35.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-36.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-37.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-38.rs"
```

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-39.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/cloud_address_tests.rs`

Supplied actual one-sample physical cloud face/cap addresses and dense rear/front-line colour/query/source/placement/cache witnesses at both physical densities. Four-sample resources are connected next.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-40.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/cloud_composite_reference.rs`

Independently render the exact maintained cloud WGSL on the same original cached point inputs; Native-only validation data.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-41.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/cloud_composite_tests.rs`

Actual one-sample independent RGBA comparison for source alpha0/255, shading0/.25, viewport edges and whole/default output, preserving original sources, owner queries and cached raster.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-42.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/renderer.rs`

Register supplied Native physical-depth/owner and independent shader witnesses.

<details>
<summary>Locate the existing block</summary>

```text
mod sample_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-43.rs" skip=17
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/cloud_gpu.rs`

Register the optional Native exact maintained compositor reference.

<details>
<summary>Locate the existing block</summary>

```text
_lod::Range, px: f32, scale: f64, ortho_h: f32) -> f32 {
  crate::cloud_lod::radius_factor(range, px, scale, ortho_h) * if ortho_h > 0. { 1. } else { 1000. }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-44.rs" skip=160
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.9
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Current cloud colour/depth/physical words and original-owner queries agree. Rear ink is hidden by actual nearer point coverage; genuinely front ink retains its colour and owner. Original alpha/source data and cached point images remain retained. Default opaque output and eye-dome shading match the independent maintained reference. The application remains single-sample until the following count-cache and consumer integration steps.

### Native cloud and query check

These actual Native GPU pictures show original cloud/face and section coverage, exact maintained opaque/eye-dome appearance, rear and genuinely front source lines, and a wide visible source fringe whose axis lies outside the small query halo. Actual independent GPU depth/address reads and fresh owner assertions accompany the colour pixels. Complete original records, geometry/uploads, exact placement restoration and repeated-frame caches remain strict. Cursor radius and edge priority are unchanged. This owned application is single-sample; count-specific cloud/sheet/text recipes and complete browser multisampling follow.

**nearer cloud over original face, one sample — actual cloud colour/depth/query check.**

![Actual Native component: nearer cloud over original face, one sample](../screenshots/journey/review/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-cloud-0.png)

**nearer cloud over original section cap, one sample — actual cloud colour/depth/query check.**

![Actual Native component: nearer cloud over original section cap, one sample](../screenshots/journey/review/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-cloud-1.png)

**original source alpha0 retained; opaque output and shading off — actual cloud colour/depth/query check.**

![Actual Native component: original source alpha0 retained; opaque output and shading off](../screenshots/journey/review/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-cloud-2.png)

**default eye-dome0.25; exact maintained compositor pixels — actual cloud colour/depth/query check.**

![Actual Native component: default eye-dome0.25; exact maintained compositor pixels](../screenshots/journey/review/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-cloud-3.png)

**rear source line hidden by real dense cloud, density2 — actual cloud colour/depth/query check.**

![Actual Native component: rear source line hidden by real dense cloud, density2](../screenshots/journey/review/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-cloud-4.png)

**genuinely front original line retains colour and owner, density2 — actual cloud colour/depth/query check.**

![Actual Native component: genuinely front original line retains colour and owner, density2](../screenshots/journey/review/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-cloud-5.png)

**visible wide source fringe retains original query owner beyond the axis halo, density2 — actual cloud colour/depth/query check.**

![Actual Native component: visible wide source fringe retains original query owner beyond the axis halo, density2](../screenshots/journey/review/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-cloud-6.png)


**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample contact browser fixture after coherent original cloud colour/depth/physical addresses and independent owner queries. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Separate labelled Native figures show face/cap coverage, maintained shading/opacity, rear/front ink and a visible wide source fringe. Complete physical-count viewer integration remains pending.

![Actual single-sample contact browser fixture after coherent original cloud colour/depth/physical addresses and independent owner queries. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Separate labelled Native figures show face/cap coverage, maintained shading/opacity, rear/front ink and a visible wide source fringe. Complete physical-count viewer integration remains pending.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The physical address image indexes original triangles and section caps. A cloud belongs to neither table, so write zero address words with its nearer colour and depth. Later ink must respect nearer physical depth even when no analytic triangle represents that occluder; a mesh-only fallback must not resurrect a hidden line.

The independent integer query has the same requirement. Establish cloud depth and zero physical words before readonly finite-ink queries, so visible front ink can return its own original owner. Reuse the existing query attachments through a stack-only pass helper; no additional image or heap array is introduced. Empty sheet lanes and absent clouds skip unused bindings.

The existing48bytecloud settings buffer supplies default eye-dome0.25 shading. Four neighboring cached depths darken real steps. Scalar updates retain the point raster, and final cloud pixels remain opaque even when original stored alpha is zero. An independent supplied Native witness renders exact maintained WGSL against the same actual inputs and checks every RGBA channel, both alpha/shading modes, viewport edges and whole/default output.

Actual Native dense-cloud witnesses inspect rear and front source colours and fresh owner queries at both densities. Their cloud covers the complete unchanged cursor neighborhood, and unrelated original boundaries lie outside it; no picker priority or radius is relaxed. Original sources, geometry/uploads, exact placement restoration and repeated-frame cache use remain strict. A wide-fringe control also requires the original line owner when its axis lies outside the small query halo; empty/out-of-window depth is not treated as an occluder. The current application is single-sample; count-specific cloud and sheet recipes follow together.

Original cloud points → coherent opaque colour/depth/zero surface address → actual ink occlusion → fresh original-owner query.



Why does correct cloud depth alone fail to establish correct ink or picking?

Analytic triangle fallbacks cannot represent a cloud, and a cloud query drawn after readonly ink can overwrite a genuinely nearer line. Keep physical depth and address words coherent, honor real non-mesh occlusion, and establish cloud query depth before readonly finite ink.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the actual physical face/cap, independent compositor and dense rear/front source witnesses. Inspect the separately labelled Native pictures. Require blue cloud pixels over a rear red line, red pixels and its original owner after moving the line in front, exact placement restoration and unchanged original point/line records. Preserve picker radius and priority by covering the complete test cursor neighborhood with real source points.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.9
npm --prefix ../session_tests run course -- save 22.5.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Nearer original cloud points write coherent opaque colour, depth and zero original triangle/cap address words. The existing48byteuniform supplies default eye-dome0.25 shading and scalar changes retain the raster. Independent Native GPU rendering with exact maintained WGSL requires zero RGBA differences across both source alpha/shading modes, viewport edges and whole/default output. Finite ink honors positive actual non-mesh depth before analytic triangle fallback; empty/out-of-window lookup is not treated as occlusion. Cloud query depth and zero physical words precede readonly finite queries, so genuinely front original ink keeps its owner. A stack-only helper reuses existing query images and descriptors. Native dense rear/front source witnesses retain chooser priority/radius, full original records, geometry/uploads, exact placement restoration and repeated-frame cache usage at both densities. A visible wide-fringe control retains its original owner with its axis outside the crop halo. Main application remains single-sample; count-specific cloud/sheet/text recipes are connected next. All 1000 Native tests, WASM/optimized builds, Native frame, canonical Chrome contact/resize checks and stored evidence pass. Ten current canonical Native/browser pictures were individually inspected. Every unique current Native component figure was also individually inspected. Complete physical-sample viewer integration, adaptive quality, remaining view controls, runtime efficiency, Cargo independence and final whole-course feature-picture review remain unfinished. Maintained Session reload camera/base-grid fixes follow the completed course.

Retain all original source, clipping, cached raster, owner-query, physical address and full reference RGBA assertions. Add actual dense rear/front line colour/query agreement, both densities, exact placement restoration and repeated-frame cache/source/geometry checks. Current source pictures demonstrate one-sample rendering; both physical counts follow.

Actual single-sample contact browser fixture after coherent original cloud colour/depth/physical addresses and independent owner queries. Both densities preserve original owners, complete source archives, shared geometry, contact shading and exact resize restoration. Separate labelled Native figures show face/cap coverage, maintained shading/opacity, rear/front ink and a visible wide source fringe. Complete physical-count viewer integration remains pending.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.9
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
