# 20.7.1 · Separate primitive addresses from selection

**Chapter 20 · Shared GPU rendering · Section 20.7 · Draw slots and primitive lookup** · [Contents](contents.md)

**Typing: 9–17 minutes.** [Estimate](typing-load.md).

Selection is presentation, not part of a triangle address. Move the primitive base into the existing range record. Both color and first-pick shaders read that word, and instance slots reuse it. Keep the reserved cap interval outside ordinary geometry.

## Type

Continue from [Share the current selection yellow](96zzt-share-the-current-selection-yellow.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_image.rs`

Name the beginning of the existing reserved cap interval.

<details>
<summary>Locate the existing block</summary>

```rust
pub(crate) const FORMAT:wgpu::TextureFormat=wgpu::TextureFormat::Rg16Uint;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) const CAP_START:u32=0xfffffff0;
pub(crate) const FORMAT:wgpu::TextureFormat=wgpu::TextureFormat::Rg16Uint;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-1.rs"
```

</details>

### 2. `src/gpu_mesh.rs`

Keep selection independent of the address. Use the already aligned range record for the full primitive base. Validate the entire emitted interval and upload only its address word.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base>0x7fff_ffff{return Err("Primitive base exceeds its packed range");}
        if self.primitive_base!=base{
            let word=(base<<1)|u32::from(self.selected());
            self.previous[64..68].copy_from_slice(&word.to_ne_bytes());
            queue.write_buffer(&self.model,64,&self.previous[64..68]);self.primitive_base=base;
        }
        Ok(())
    }
```

</details>

Replace it with:

```rust
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
        if self.primitive_base!=base{
            self.previous[RANGE+4..RANGE+8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(&self.model,(RANGE+4)as u64,&self.previous[RANGE+4..RANGE+8]);self.primitive_base=base;
        }
        Ok(())
    }
```

<details>
<summary>Locate this block</summary>

```rust
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes
}
```

</details>

Replace it with:

```rust
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE+4..RANGE+8].copy_from_slice(&primitive_base.to_ne_bytes());
    bytes
}
```

<details>
<summary>Locate this block</summary>

```rust
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&((primitive_base<<1)|u32::from(style.selected)).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
```

</details>

Replace it with:

```rust
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, scene::Object};
use std::rc::Rc;
use crate::object_record::{BYTES, Record, RANGE};
use wgpu::util::DeviceExt;

pub struct GpuMesh {
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub pickable:bool,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32) -> Record {
    let mut bytes = [0; BYTES];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=if style.pickable {object.id.value()} else {0};
    bytes[68..72].copy_from_slice(&id.to_ne_bytes());
    let flags=if matches!(&object.original,Some(session_rust::Geometry::Plane(p)) if crate::clipping_geometry::is_clipping(p)){4096u32}else{0};bytes[72..76].copy_from_slice(&(flags|cap_flags).to_ne_bytes());bytes[76..80].copy_from_slice(&cap_token.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model).expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    if let Some(colour)=style.colour {for (index,value) in colour[..3].iter().copied().chain([1.]).enumerate() {bytes[144+index*4..148+index*4].copy_from_slice(&value.to_ne_bytes());}}
    bytes[RANGE..RANGE+4].copy_from_slice(&index_start.to_ne_bytes());
    bytes[RANGE+4..RANGE+8].copy_from_slice(&primitive_base.to_ne_bytes());
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
        let bytes = settings(object, style,0,0,0,geometry.index_start);
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
        Self { geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_primitive_base(&mut self,queue:&wgpu::Queue,base:u32)->Result<(),&'static str>{
        if base.checked_add(self.geometry.index_count/3).is_none_or(|end|end>=crate::primitive_image::CAP_START){return Err("Primitive range overlaps reserved caps");}
        if self.primitive_base!=base{
            self.previous[RANGE+4..RANGE+8].copy_from_slice(&base.to_ne_bytes());
            queue.write_buffer(&self.model,(RANGE+4)as u64,&self.previous[RANGE+4..RANGE+8]);self.primitive_base=base;
        }
        Ok(())
    }

    pub fn set_xray(&mut self,queue:&wgpu::Queue,on:bool){if self.xray==on{return;}self.xray=on;let flags=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap());let flags=(flags&!32768)|if on&&flags&8192!=0{32768}else{0};self.previous[72..76].copy_from_slice(&flags.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..76]);}
    pub fn closed(&self)->bool{!self.xray&&self.cap_flags&crate::solid_orientation::CLOSED!=0}
    pub fn pickable(&self)->bool{u32::from_ne_bytes(self.previous[68..72].try_into().unwrap())!=0}
    pub fn set_caps(&mut self,queue:&wgpu::Queue,flags:u32,token:u32){
        if self.cap_flags==flags&&self.cap_token==token{return;}let plane=u32::from_ne_bytes(self.previous[72..76].try_into().unwrap())&4096;self.previous[72..76].copy_from_slice(&(plane|flags|if self.xray&&flags&8192!=0{32768}else{0}).to_ne_bytes());self.previous[76..80].copy_from_slice(&token.to_ne_bytes());queue.write_buffer(&self.model,72,&self.previous[72..80]);self.cap_flags=flags;self.cap_token=token;
    }
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        let next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144), (144,RANGE),(RANGE,BYTES)] {
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

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.bind_model(pass);
        self.geometry.draw_pulled(pass, 1);
    }

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
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-2.rs"
```

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-3.rs"
```

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-4.rs"
```

</details>

### 3. `src/triangle.wgsl`

Read the same range word in color and first picking.

<details>
<summary>Locate the existing block</summary>

```wgsl
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    output.primitive=object.range.y+occurrence/3u+1u;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-5.rs"
```

</details>

### 4. `src/triangle_pick.wgsl`

Read the same range word in color and first picking.

<details>
<summary>Locate the existing block</summary>

```wgsl
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,(object_settings.selection.x>>1u)+occurrence/3u+1u);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,object_settings.range.y+occurrence/3u+1u);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-6.rs"
```

</details>

### 5. `src/instance_gpu.rs`

Keep the instance shader adaptation attached to the new ordinary expression. Build instance slots from the same full address.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::{BTreeMap,HashMap},rc::Rc};
use wgpu::util::DeviceExt;
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32,slots:crate::draw_slots::Slots}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];")
  .replace("@builtin(instance_index) instance: u32","@location(0) row:u32,@location(1) primitive_base:u32")
  .replace("let object=object_at(instance);","let object=object_at(row);")
  .replace("(object.selection.x>>1u)+occurrence/3u+1u","primitive_base+occurrence/3u+1u");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),writes:0,written_bytes:0}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:HashMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=HashMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
                let slots:Vec<crate::draw_slots::Slot>=records.iter().enumerate().map(|(row,record)|[row as u32,u32::from_ne_bytes(record[64..68].try_into().unwrap())>>1]).collect();
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    if batch.slots.set(&ctx,&slots){self.writes+=1;self.written_bytes+=(slots.len()as u64+1)*8;}
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;{let mut slot_buffer=crate::draw_slots::Slots::new(&ctx);slot_buffer.set(&ctx,&slots);self.writes+=2;self.written_bytes+=8+(slots.len()as u64+1)*8;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32,slots:slot_buffer}}
                };batches.insert(key,batch);
            }
        }
        self.batches=batches;Ok(())
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);batch.slots.bind(pass,0,1);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()+b.slots.allocated_bytes()).sum(),self.writes,self.written_bytes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::{BTreeMap,HashMap},rc::Rc};
use wgpu::util::DeviceExt;
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,records:wgpu::Buffer,group:wgpu::BindGroup,bytes:Vec<u8>,count:u32,slots:crate::draw_slots::Slots}
pub struct Lane {pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<(usize,usize),Batch>,pub writes:u64,pub written_bytes:u64}
pub(crate) fn source()->String {
 let source=include_str!("triangle.wgsl")
  .replace("var<uniform> object_settings: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;")
  .replace("return object_settings;","return objects[index];")
  .replace("@builtin(instance_index) instance: u32","@location(0) row:u32,@location(1) primitive_base:u32")
  .replace("let object=object_at(instance);","let object=object_at(row);")
  .replace("object.range.y+occurrence/3u+1u","primitive_base+occurrence/3u+1u");
 crate::clipping_gpu::source(&source,2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("shared instance records"),entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(BYTES as u64)},count:None}]});
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(format.into()),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self {pipeline,layout,batches:BTreeMap::new(),writes:0,written_bytes:0}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=(u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size)/BYTES as u64) as usize;
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:HashMap<usize,(Rc<GpuGeometry>,Vec<Record>)>=HashMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push(row.packed());}
        let mut old=std::mem::take(&mut self.batches);let mut batches=BTreeMap::new();
        for (source,(geometry,records)) in groups {
            for (part,records) in records.chunks(limit).enumerate() {
                let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
                let slots:Vec<crate::draw_slots::Slot>=records.iter().enumerate().map(|(row,record)|[row as u32,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]).collect();
                let key=(source,part);let bytes=records.iter().flatten().copied().collect::<Vec<_>>();
                let batch=if let Some(mut batch)=old.remove(&key).filter(|b|Rc::ptr_eq(&b.geometry,&geometry)&&b.records.size()>=bytes.len() as u64) {
                    if batch.slots.set(&ctx,&slots){self.writes+=1;self.written_bytes+=(slots.len()as u64+1)*8;}
                    if batch.bytes!=bytes {queue.write_buffer(&batch.records,0,&bytes);self.writes+=1;self.written_bytes+=bytes.len() as u64;batch.bytes=bytes;}batch.count=records.len() as u32;batch
                } else {
                    let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("independent source placements"),contents:&bytes,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&self.layout,&buffer);self.writes+=1;self.written_bytes+=bytes.len() as u64;{let mut slot_buffer=crate::draw_slots::Slots::new(&ctx);slot_buffer.set(&ctx,&slots);self.writes+=2;self.written_bytes+=8+(slots.len()as u64+1)*8;Batch {geometry:geometry.clone(),records:buffer,group,bytes,count:records.len() as u32,slots:slot_buffer}}
                };batches.insert(key,batch);
            }
        }
        self.batches=batches;Ok(())
    }
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        for batch in self.batches.values() {pass.set_bind_group(1,&batch.group,&[]);batch.slots.bind(pass,0,1);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.batches.values().map(|b|b.records.size()+b.slots.allocated_bytes()).sum(),self.writes,self.written_bytes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-7.rs"
```

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-8.rs"
```

</details>

### 6. `src/renderer.rs`

Bound cumulative visible ranges below reserved cap markers.

<details>
<summary>Locate the existing block</summary>

```rust
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<=0x7fff_ffff).expect("Bounded projected triangle range");
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-9.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/global_outline_tests.rs`

Read the independent base in the existing ownership fixture.

<details>
<summary>Locate the existing block</summary>

```text
u32::from_ne_bytes(row.packed()[64..68].try_into().unwrap())>>1
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-10.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/global_outline_tests.rs`

Reject reserved addresses while keeping the original unchanged-state assertion.

<details>
<summary>Locate the existing block</summary>

```text
set_primitive_base(&gpu.queue,0x8000_0000).is_err()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/global_outline_tests.rs`

Supply full high-bit GPU readback, selection independence, cap collision and overflow assertions.

<details>
<summary>Locate the existing block</summary>

```text
()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzu-separate-primitive-addresses-from-selection-12.rs" skip=160
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.7.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for separate primitive addresses from selection.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



range record → ordinary shader or instance slot → physical primitive image.



Why check the end of the range rather than the base alone?

Every emitted address adds the triangle index and one. The last address must remain below the reserved cap interval, including when integer addition would overflow.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Select two placed copies, clear selection, hide one and pick the other. Their original identities must remain stable even when visible primitive ranges are rebuilt.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.7.1
npm --prefix ../session_tests run course -- save 20.7.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This removes the selection-bit address limit. Global slot allocation and ordinary draw-row binding still require implementation.



[Full validation scope](release.md).

</details>
