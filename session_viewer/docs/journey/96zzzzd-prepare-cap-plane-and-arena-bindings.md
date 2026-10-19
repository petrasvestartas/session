# 20.10.5 · Prepare cap plane and arena bindings

**Chapter 20 · Shared GPU rendering · Section 20.10 · Projection and cap consumers** · [Contents](contents.md)

**Typing: 21–41 minutes.** [Estimate](typing-load.md).

Prepare explicit plane parameters with current immutable arena bindings.

## Type

Continue from [Prepare shared record cap counting](96zzzzc-prepare-shared-record-cap-counting.md). [Save or recover your work](recovery.md).

### 1. `src/cap_geometry.rs`

Keep explicit plane parameters and arena binding replacement in one resource owner.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzd-prepare-cap-plane-and-arena-bindings-01.rs"
```

### 2. `src/lib.rs`

Declare the resource without connecting it to drawing yet.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod cap_pipeline;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cap_geometry;
mod cap_pipeline;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzd-prepare-cap-plane-and-arena-bindings-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/cap_geometry.rs`

Supply actual mapped plane values, exact binding reuse/replacement through arena growth, bounded refusal and full release.

<details>
<summary>Locate the existing block</summary>

```text
use std::{collections::HashMap,rc::Rc};
use wgpu::util::DeviceExt;
#[derive(Default)]
pub struct Geometry {planes:Vec<wgpu::Buffer>,groups:HashMap<(usize,u32),(wgpu::Buffer,wgpu::Buffer,wgpu::BindGroup)>}
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 use wgpu::{ShaderStages as S,BufferBindingType as B};device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("cap plane and arena"),entries:&[crate::tile_pipelines::entry(0,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::VERTEX,B::Storage{read_only:true})]})
}
impl Geometry {
 pub fn set_planes(&mut self,device:&wgpu::Device,count:u32)->Result<(),&'static str> {
  if count as usize>crate::clipping_uniform::MAX_PLANES{return Err("Too many cap planes");}if self.planes.len()==count as usize{return Ok(());}
  self.clear();self.planes=(0..count).map(|plane|{let bytes=[plane,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("cap plane selector"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_SRC})}).collect();Ok(())
 }
 pub fn group(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32)->Result<wgpu::BindGroup,&'static str> {
  let parameter=self.planes.get(plane as usize).ok_or("Missing cap plane")?;let key=(Rc::as_ptr(geometry)as usize,plane);let(vertices,indices)=geometry.buffers();
  if let Some((old_vertices,old_indices,group))=self.groups.get(&key){if *old_vertices==vertices&&*old_indices==indices{return Ok(group.clone());}}
  let group=crate::tile_pipelines::group(device,layout,&[&vertices,parameter,&indices]);self.groups.insert(key,(vertices,indices,group.clone()));Ok(group)
 }
 pub fn invalidate(&mut self){self.groups.clear();}
 pub fn clear(&mut self){*self=Self::default();}
 pub fn bytes(&self)->u64 {self.planes.iter().map(wgpu::Buffer::size).sum()}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzd-prepare-cap-plane-and-arena-bindings-03.rs" skip=2026
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.10.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Plane parameters and arena groups are prepared; cap drawing still uses its earlier path.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for prepare cap plane and arena bindings.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A shared object slot selects a surface; the cap count also needs a plane. Keep those two inputs independent. Prepare a sixteen-byte plane parameter and a geometry group that reads the current arena buffers. Cache groups while those buffers remain identical, replace them after arena growth, and release the cache when inactive. This resource connects to cap color in the next lesson.

Explicit plane plus current arena buffers → reusable cap geometry group.



Why keep the plane independent of the slot instance index?

The slot attribute selects the object record. Reusing its instance index for a plane would fetch a different slot. An explicit plane parameter preserves both identities.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the mapped plane, retained binding, grown arena and release fixture. Repeat existing cap clipping and picking in the browser; the new group is not connected yet.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.10.5
npm --prefix ../session_tests run course -- save 20.10.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Preparatory cap geometry resource only. Shared cap color and pick consumers, primitive lookup, reclamation and wider parity remain open.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.10.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
