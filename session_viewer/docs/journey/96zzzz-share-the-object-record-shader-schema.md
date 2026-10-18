# 20.10.1 · Share the object record shader schema

**Chapter 20 · Shared GPU rendering · Section 20.10 · Projection and cap consumers** · [Contents](contents.md)

**Typing: 5–9 minutes.** [Estimate](typing-load.md).

Keep one aligned object record declaration across drawing, caps and projection.

## Type

Continue from [Read ambient properties through shared records](96zzzy-read-ambient-properties-through-shared-records.md). [Save or recover your work](recovery.md).

### 1. `src/object_settings.wgsl`

Write the existing aligned record schema once.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-01.rs"
```

### 2. `src/triangle.wgsl`

Use the record declaration supplied by shared shader assembly.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

</details>

### 3. `src/cap_count.wgsl`

Cap crossings use the identical record declaration.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
    range:vec4<u32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> object: ObjectSettings;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

</details>

### 4. `src/finite_project.wgsl`

Projection uses the same byte-compatible model and range fields.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ObjectSettings {model:mat4x4<f32>,selected:u32,id:u32,pad:vec2<u32>,normals:mat4x4<f32>,colour:vec4<f32>,range:vec4<u32>}
@group(0) @binding(0) var<uniform> view:View;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> view:View;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

</details>

### 5. `src/object_record.rs`

Expose the shared declaration through the existing record owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub type Record = [u8; BYTES];
```

</details>

After that block, type:

```rust
pub fn shader()->&'static str {include_str!("object_settings.wgsl")}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-05.rs"
```

</details>

### 6. `src/clipping_gpu.rs`

Assemble every clipped raster consumer with the shared record declaration. Keep the shared declaration separate from shader resource bindings.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{clipping_rows::Row,clipping_uniform::{ClipUniform,ClipView,clip_uniform}};
use wgpu::util::DeviceExt;
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout{crate::gpu_layout::uniform(device,448)}
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),crate::draw_slots::sentinel_shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
pub fn bytes(uniform:&ClipUniform)->Vec<u8>{
    let mut bytes=uniform.planes.iter().chain(&uniform.screen).chain(&uniform.hatch).chain(&uniform.hatch_w).chain(&uniform.sides).flatten().copied().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
    bytes.extend([uniform.count,uniform.samples,uniform.fill].into_iter().flat_map(u32::to_ne_bytes));bytes.extend([uniform.spacing,uniform.width,uniform.outline,uniform.pad[0],uniform.pad[1]].into_iter().flat_map(f32::to_ne_bytes));assert_eq!(bytes.len(),448);bytes
}
pub struct Clipping{pub buffer:wgpu::Buffer,pub group:wgpu::BindGroup,pub rows:Vec<Row>,pub error:Option<&'static str>,pub enabled:bool,pub solid:bool,key:Vec<u8>,pub writes:u64}
impl Clipping{
    pub fn new(device:&wgpu::Device)->Self{let key=bytes(&ClipUniform::default());let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("current original clipping planes"),contents:&key,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&layout(device),&buffer);Self{buffer,group,rows:Vec::new(),error:None,enabled:true,solid:true,key,writes:0}}
    pub fn set(&mut self,scene:&crate::scene::Scene){match crate::clipping_rows::current(scene){Ok(rows)=>{self.rows=rows;self.error=None},Err(error)=>{self.rows.clear();self.error=Some(error)}}}
    pub fn frame(&mut self,queue:&wgpu::Queue,matrix:[f32;16],height:u32,density:f32){let matrix=session_rust::Xform::from_matrix(matrix.map(f64::from));let planes=if self.enabled{self.rows.iter().map(|r|r.plane).collect::<Vec<_>>()}else{Vec::new()};let uniform=clip_uniform(&planes,u32::from(self.solid),&ClipView{view_proj:&matrix,anchor:[0.;3],height,pixel_scale:f64::from(density),samples:1,forward_depth:true});let bytes=bytes(&uniform);if bytes!=self.key{queue.write_buffer(&self.buffer,0,&bytes);self.key=bytes;self.writes+=1;}}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{clipping_rows::Row,clipping_uniform::{ClipUniform,ClipView,clip_uniform}};
use wgpu::util::DeviceExt;
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout{crate::gpu_layout::uniform(device,448)}
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n{}\n{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),crate::draw_slots::sentinel_shader(),crate::object_record::shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
pub fn bytes(uniform:&ClipUniform)->Vec<u8>{
    let mut bytes=uniform.planes.iter().chain(&uniform.screen).chain(&uniform.hatch).chain(&uniform.hatch_w).chain(&uniform.sides).flatten().copied().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
    bytes.extend([uniform.count,uniform.samples,uniform.fill].into_iter().flat_map(u32::to_ne_bytes));bytes.extend([uniform.spacing,uniform.width,uniform.outline,uniform.pad[0],uniform.pad[1]].into_iter().flat_map(f32::to_ne_bytes));assert_eq!(bytes.len(),448);bytes
}
pub struct Clipping{pub buffer:wgpu::Buffer,pub group:wgpu::BindGroup,pub rows:Vec<Row>,pub error:Option<&'static str>,pub enabled:bool,pub solid:bool,key:Vec<u8>,pub writes:u64}
impl Clipping{
    pub fn new(device:&wgpu::Device)->Self{let key=bytes(&ClipUniform::default());let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("current original clipping planes"),contents:&key,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&layout(device),&buffer);Self{buffer,group,rows:Vec::new(),error:None,enabled:true,solid:true,key,writes:0}}
    pub fn set(&mut self,scene:&crate::scene::Scene){match crate::clipping_rows::current(scene){Ok(rows)=>{self.rows=rows;self.error=None},Err(error)=>{self.rows.clear();self.error=Some(error)}}}
    pub fn frame(&mut self,queue:&wgpu::Queue,matrix:[f32;16],height:u32,density:f32){let matrix=session_rust::Xform::from_matrix(matrix.map(f64::from));let planes=if self.enabled{self.rows.iter().map(|r|r.plane).collect::<Vec<_>>()}else{Vec::new()};let uniform=clip_uniform(&planes,u32::from(self.solid),&ClipView{view_proj:&matrix,anchor:[0.;3],height,pixel_scale:f64::from(density),samples:1,forward_depth:true});let bytes=bytes(&uniform);if bytes!=self.key{queue.write_buffer(&self.buffer,0,&bytes);self.key=bytes;self.writes+=1;}}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-06.rs"
```

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-07.rs"
```

</details>

### 7. `src/finite_project_source.rs`

Compute projection includes the same declaration before its packed geometry helper.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source() -> String { format!("{}\n{}",include_str!("packed_vertex.wgsl"),expand(include_str!("finite_project.wgsl"))) }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source() -> String { format!("{}\n{}\n{}",crate::object_record::shader(),include_str!("packed_vertex.wgsl"),expand(include_str!("finite_project.wgsl"))) }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-08.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/object_record_tests.rs`

Verify that projection uses the same aligned members as both raster states after consolidating the flag vector.

<details>
<summary>Locate the existing block</summary>

```text
(crate::finite_project_source::source(), vec![0, 64, 68, 72, 80, 144, 160])
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzz-share-the-object-record-shader-schema-layout-fixture.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.10.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Surface, cap and projection shaders use one object schema with unchanged rendering and picking.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for share the object record shader schema.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Surface and cap shaders already read the same record bytes, while projection spells the equivalent fields separately. Extract the aligned declaration once and assemble each shader with that declaration. Keep every offset and resource binding unchanged. This avoids repeating the schema before the projection and cap consumers connect to shared records.

One aligned object declaration → raster and compute shader assembly → unchanged GPU bytes.



Why centralize the declaration before connecting more consumers?

A duplicated layout can drift even when the buffers are shared. One declaration keeps model, flags, normal transform, presentation and range offsets consistent.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the single declaration, then repeat ordinary and copied-object picks, clipping and Arctic restoration. Compare the existing exact byte and pixel fixtures.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.10.1
npm --prefix ../session_tests run course -- save 20.10.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Object layout duplication is removed. Projection and cap record consumers, primitive lookup, coordinates and wider viewer parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.10.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
