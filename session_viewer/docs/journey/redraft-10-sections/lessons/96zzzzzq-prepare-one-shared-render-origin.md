# 21.3 · Browser ownership of frame matrices and wakeups

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 21.3** (Redraft `10.13`) · **3 checkpoints (`96zzzzzq-prepare-one-shared-render-origin` → `96zzzzzs-adopt-the-shared-origin-in-the-browser`)**

> **Learning Goal:** Adopt one origin and synchronize original draw inputs before rendering or picking.

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
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Section 10 Production Architecture: Shared GPU Storage Arena, f64 Floating Origin, Reversed-Z, 4x MSAA & Matrix Shader</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How the maintained viewer pulls vertices from one growable GPU arena, eliminates jitter at 10^12 units, and renders procedural Matrices.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="250" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="143" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. Shared GPU Arena (Ch 19–20)</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• 1 Growable Storage Buffer Arena</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• Vertex Pulling (@builtin(vertex_index))</text>
    <text x="28" y="144" font-size="10.5" fill="#1e3a8a">• 8-byte Draw Slots -&gt; ObjectRecord</text>
    <text x="28" y="162" font-size="10.5" fill="#1e3a8a">• slot_table.wgsl maps Rg32Uint</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">  primitive IDs to Element/Face owners</text>
    <text x="28" y="194" font-size="10" fill="#1e3a8a">• Free-list span reuse after GPU lease</text>

    <line x1="268" y1="138" x2="298" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="300" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="432" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. f64 Floating Origin &amp; Reversed-Z (Ch 21)</text>
    <text x="312" y="108" font-size="10.5" font-weight="600" fill="#c2410c">• rel_f32 = (world_f64 - anchor_f64) as f32</text>
    <text x="312" y="125" font-size="10" fill="#7c2d12">  Zero vertex/grid jitter even at 10^12 m!</text>
    <text x="312" y="144" font-size="10.5" font-weight="600" fill="#c2410c">• Reversed-Z Depth (GreaterEqual):</text>
    <text x="312" y="160" font-size="10" fill="#7c2d12">  Near plane = 1.0, Far plane = 0.0</text>
    <text x="312" y="178" font-size="10.5" fill="#7c2d12">• Held world base grid &amp; preserved</text>
    <text x="312" y="193" font-size="10.5" fill="#7c2d12">  live camera during Session reload</text>

    <line x1="565" y1="138" x2="595" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="597" y="68" width="223" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="708" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. 4x MSAA &amp; Matrix Shader (Ch 22)</text>
    <text x="609" y="108" font-size="10.5" fill="#064e3b">• Coherent 1x/4x MSAA sample targets</text>
    <text x="609" y="126" font-size="10.5" font-weight="600" fill="#047857">• matrix.wgsl (16w header + 3w/cell):</text>
    <text x="609" y="142" font-size="10" fill="#064e3b">  Heatmap + procedural 3-decimal digits</text>
    <text x="609" y="157" font-size="10" fill="#064e3b">  (0 cell blank; plate-less title; yellow</text>
    <text x="609" y="172" font-size="10" fill="#064e3b">  selected border; Rg32Uint whole pick)</text>
    <text x="609" y="190" font-size="10" fill="#064e3b">• Coalesced single-flight frame scheduler</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
Precise camera target → origin policy → synchronized draw inputs → matching local matrix.
Renderer origin → matching browser matrix; owned timeout → one later frame.
Browser action → shared origin → original and temporary inputs → draw/pick; pending → owned wake.
```

- Adopt one origin and synchronize original draw inputs before rendering or picking.
- Use one browser render matrix and prepare a wake owned by the runtime.
- Keep browser drawing and asynchronous interaction in one precise local frame.
- **Why it works (Why must a pending rebase schedule another frame?):** The origin policy can defer a move for its throttle interval. An event-driven browser may receive no more input after release, so it needs one later wake to finish the move and any rebased pick retry.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **WGSL (Vertex Pull)** | `@group(0) @binding(b) var<storage, read> arena_verts: array<u32>;` | Fetches packed vertex positions, normals, and colors directly from a single growable storage buffer using `@builtin(vertex_index)`. |
| **Rust/WGSL (`f64`->`f32`)** | `let rel: Vec3 = (world_f64 - anchor_f64).as_vec3();` | Subtracts the camera-area `f64` anchor on the CPU before converting to `f32`, preventing 32-bit float mantissa exhaustion far from `(0,0,0)`. |
| **WGSL (`matrix.wgsl`)** | 16-word header + 3 words/cell procedural digits | Renders matrix heatmaps and 3-decimal numeric digits procedurally inside the fragment shader with zero texture uploads per cell. |
| **Rust (`ink_ranges.rs`)** | Merged contiguous `Range<u32>` instance draws | Toggles point/line/edge/CAD-wire family visibility without rebuilding GPU buffers or invalidating original owner indices. |

**Production source (`session_viewer/src`):** [camera.rs](../../../../src/camera.rs), [state.rs](../../../../src/state.rs), [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs), [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs), [engine/gpu/targets.rs](../../../../src/engine/gpu/targets.rs), [shaders/grid.wgsl](../../../../src/shaders/grid.wgsl), [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl), [app/cplane.rs](../../../../src/app/cplane.rs), [build.rs](../../../../build.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-96zzzzzq-prepare-one-shared-render-origin"></a>

<details>
<summary>Implementation 96zzzzzq-prepare-one-shared-render-origin: Prepare one shared render origin</summary>

Adopt one origin and synchronize original draw inputs before rendering or picking.

Data flow: Precise camera target → origin policy → synchronized draw inputs → matching local matrix.

### 1. `src/render_space.rs`

Expose a finite bounded delay beyond the existing strict 200ms throttle.

<details>
<summary>Locate the existing block</summary>

```rust
 pub fn forget(&mut self){self.anchor=None;}
```

</details>

After that block, type:

```rust
 pub fn retry_after_ms(&self,now:f64)->Option<u32>{
  now.is_finite().then(||(self.last_ms+201.-now).ceil().clamp(1.,201.)as u32)
 }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzq-prepare-one-shared-render-origin-01.rs"
```

</details>

### 2. `src/renderer.rs`

Adopt one origin without source cancellation; reuse existing input owners and skip unchanged data.

<details>
<summary>Locate the existing block</summary>

```rust
    fn set_source_ink(&mut self,scene:&Scene) {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
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
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzq-prepare-one-shared-render-origin-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/render_space.rs`

Supply deterministic throttle-deadline and invalid-clock checks without requiring native tests in the learner project.

<details>
<summary>Locate the existing block</summary>

```text
pub fn relative(translation:[f64;3],anchor:[f64;3])->[f32;3] {std::array::from_fn(|i|(translation[i]-anchor[i])as f32)}
pub fn placed(place:&session_rust::Xform,anchor:[f64;3])->session_rust::Xform {
 let mut local=place.clone();for i in 0..3 {local.m[12+i]-=anchor[i];}local
}
#[derive(Default)]
pub struct Origin {anchor:Option<[f64;3]>,last_ms:f64}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rebase {pub anchor:[f64;3],pub moved:bool,pub pending:bool}
impl Origin {
 pub fn anchor(&self)->[f64;3] {self.anchor.unwrap_or([0.;3])}
 pub fn forget(&mut self){self.anchor=None;}
 pub fn retry_after_ms(&self,now:f64)->Option<u32>{
  now.is_finite().then(||(self.last_ms+201.-now).ceil().clamp(1.,201.)as u32)
 }
 pub fn update(&mut self,target:[f64;3],distance:f64,now:f64)->Option<Rebase>{
  if target.iter().any(|v|!v.is_finite())||!distance.is_finite()||distance<=0.||!now.is_finite(){return None;}
  let threshold=(distance*0.25).clamp(1000.,100000.);
  let need=self.anchor.is_none_or(|a|(0..3).map(|i|(a[i]-target[i]).powi(2)).sum::<f64>().sqrt()>threshold);
  let moved=need&&(self.anchor.is_none()||now-self.last_ms>200.);
  if moved {self.anchor=Some(target);self.last_ms=now;}
  Some(Rebase {anchor:self.anchor.unwrap(),moved,pending:need&&!moved})
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn origin_threshold_throttle_reset_and_invalid_inputs_keep_one_anchor(){
  let mut origin=Origin::default();assert_eq!(origin.update([0.;3],4000.,0.),Some(Rebase{anchor:[0.;3],moved:true,pending:false}));assert_eq!(origin.update([1000.,0.,0.],4000.,1.),Some(Rebase{anchor:[0.;3],moved:false,pending:false}));assert_eq!(origin.update([1000.25,0.,0.],4000.,200.),Some(Rebase{anchor:[0.;3],moved:false,pending:true}));assert_eq!(origin.update([1000.25,0.,0.],4000.,200.01),Some(Rebase{anchor:[1000.25,0.,0.],moved:true,pending:false}));assert!(!origin.update([99000.,0.,0.],1e15,300.).unwrap().moved);assert!(origin.update([200000.,0.,0.],1e15,500.).unwrap().moved);
  for (target,distance,now) in [([f64::NAN,0.,0.],1.,501.),([0.;3],0.,501.),([0.;3],f64::INFINITY,501.),([0.;3],1.,f64::NAN)]{assert!(origin.update(target,distance,now).is_none());}assert_eq!(origin.update([200000.,0.,0.],1.,501.).unwrap().anchor,[200000.,0.,0.]);origin.forget();assert!(origin.update([1e12,-1e12,1e12],1.,501.).unwrap().moved);
 }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzq-prepare-one-shared-render-origin-03.rs" skip=2334
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply aggregate original mesh/ink/point/cloud/sheet/text/grid pixels, precise ground, unchanged records, throttled wake timing, preserved pending flight/retry and exact restoration/source ownership.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.pipeline.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views(){
 let mut scene=Scene::empty();let mut source=session_rust::PointCloud::from_coords(vec![-0.5,0.,-0.5],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.,-0.5,0.75,0.,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let original=source.to_proto();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.distance=2.;camera.aspect=4./3.;camera.projection=crate::camera::Projection::Orthographic;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]<20&&p[2]<20));assert!(baseline.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>20);let uploads=(gpu.cloud_usage()[2],gpu.sheet_usage()[3]);assert_eq!(uploads,(1,1));
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [cloud,drawing]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);let mut far_camera=crate::camera::Camera::default();far_camera.distance=camera.distance;far_camera.aspect=camera.aspect;far_camera.projection=camera.projection;far_camera.target=anchor;gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();let local=far_camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);let prepared=gpu.cloud_usage()[3];assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.cloud_usage()[3],prepared);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];far_camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,moved).unwrap();let same_matrix=far_camera.view_projection_anchored(moved).to_f32();assert_eq!(same_matrix,local);let panned=crate::cap_gpu_tests::pixels_at(&gpu,same_matrix);assert!(panned.chunks_exact(4).all(|p|p==&[255;4]));assert!(gpu.cloud_usage()[3]>prepared,"Origin alone invalidates the equal-matrix cloud view");assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);
 far_camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);for(point,id)in[([-0.5,0.,-0.5],cloud),([0.5,0.,-0.5],drawing)]{let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));gpu.request_pick([point[0]as f32,point[1]as f32],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],id.value());}assert_eq!(source.to_proto(),original);assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.cloud_usage(),[0;5]);assert_eq!(&gpu.sheet_usage()[..2],&[0,0]);
}

fn temporary_inputs(gpu:&mut super::Renderer,world:[f64;3]){
 let point=|p:[f64;3]|std::array::from_fn(|i|p[i]+world[i]);let mut guides=crate::guide_gpu::Overlay::at(gpu.anchor());guides.wire(&[point([-0.75,0.25,-0.5]),point([0.75,0.25,-0.5])],[210./255.,40./255.,40./255.,1.],true);guides.mark(point([0.5,0.25,-0.5]),6.,[0.,0.,1.,1.]);guides.label=Some((point([0.5,0.25,-0.5]),"guide".into()));gpu.set_guides(&guides).unwrap();let mark=crate::measure_mark::Mark{points:vec![point([-0.5,-0.25,-0.5]),point([0.5,-0.25,-0.5])],label:"distance".into()};gpu.set_measurement(Some(&mark)).unwrap();let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.submit(point([0.,0.,-0.5])).unwrap();draft.hover(Some(point([0.5,0.,-0.5]))).unwrap();gpu.set_preview(Some(&draft)).unwrap();
}
#[test]
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release(){
 let scene=Scene::empty();let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();temporary_inputs(&mut gpu,[0.;3]);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]>200&&p[2]<30));assert!(baseline.chunks_exact(4).any(|p|p[0]>150&&p[1]<80&&p[2]<80));assert!(baseline.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>200);let bytes=(gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes());assert!(bytes.0>0&&bytes.1>0&&bytes.2>0);assert_eq!(gpu.text_error(),None);
 let anchor=[1e9,-1e9,1e9];assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);camera.target=anchor;temporary_inputs(&mut gpu,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes()),bytes);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));camera.target=anchor;assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert!(scene.all_ids().is_empty());gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap(),None);
 gpu.set_preview(None).unwrap();gpu.set_measurement(None).unwrap();gpu.set_guides(&crate::guide_gpu::Overlay::at(anchor)).unwrap();crate::cap_gpu_tests::pixels_at(&gpu,local);assert_eq!(gpu.preview_bytes(),0);assert_eq!(gpu.guide_bytes(),0);assert_eq!(gpu.measurement.bytes(),0);assert_eq!(gpu.text_usage()[0],0);
}

#[test]
fn actual_source_grid_origin_cache_keeps_surface_pixels_owners_and_release(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let settings=Some(crate::grid::Settings{spacing:0.1,half_cells:5});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.space.update(anchor,2.,0.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let far_plane=plane([anchor[0],anchor[1],anchor[2]-0.75]);gpu.set_grid_on(settings,far_plane).unwrap();camera.target=anchor;let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);let grid_uploads=gpu.grid_usage()[2];gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(gpu.grid_usage()[2],grid_uploads);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];gpu.space.update(moved,2.,300.).unwrap();camera.target=moved;for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,moved);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert!(gpu.grid_usage()[2]>grid_uploads);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));gpu.space.update(anchor,2.,600.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_grid(None).unwrap();gpu.set_scene(&Scene::empty(),None);assert_eq!(&gpu.grid_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_initial_object_records_keep_precise_local_pixels_updates_and_sources(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.2,-0.25,-0.5],[0.3,-0.25,-0.5],[0.1,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload(&gpu.device,&layout,&scene.objects()[0],false);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let original=gpu.meshes[0].packed();let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload_at(&gpu.device,&layout,&far.objects()[0],false,anchor);assert_eq!(gpu.meshes[0].packed(),original);let geometry=gpu.meshes[0].geometry.clone();let style=crate::gpu_mesh::Style{selected:false,pickable:true,colour:None};gpu.meshes[0]=crate::gpu_mesh::GpuMesh::with_geometry_at(&gpu.device,&layout,&far.objects()[0],style,geometry.clone(),anchor);assert_eq!(gpu.meshes[0].packed(),original);assert!(Rc::ptr_eq(&gpu.meshes[0].geometry,&geometry));gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));camera.target=anchor;let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[0,0]);let selected=crate::gpu_mesh::Style{selected:true,..style};assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],selected),[1,16]);assert_eq!(&gpu.meshes[0].packed()[..64],&original[..64]);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[1,16]);assert_eq!(gpu.meshes[0].packed(),original);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);assert_eq!(gpu.stats()[0],uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_original_highlights_keep_local_pixels_addresses_and_sources_after_rebasing(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let mesh=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.4,0.5,-0.5,0.4,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.4,-0.5,-0.5),session_rust::Point::new(0.,-0.3,-0.5),session_rust::Point::new(0.4,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let mut proto=session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![],vec![]).to_proto();proto.point_ids=vec![u32::MAX];let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_proto(proto))).unwrap()).unwrap();scene.owner_controls(path).unwrap();scene.cloud_controls(Some(cloud));let curve=scene.insert_path(crate::chain::PreparedChain::curve(Rc::new(crate::freeform_curve::curve(&[session_rust::Point::new(-0.4,0.8,-0.5),session_rust::Point::new(0.,0.65,-0.5),session_rust::Point::new(0.4,0.8,-0.5)]))).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let edge=scene.objects()[0].mesh.parts.edges[0];let control=scene.control_markers().into_iter().find(|c|c.parent==path).unwrap().part;let targets=[Target{owner:mesh,part:Part::Face(key)},Target{owner:mesh,part:Part::MeshEdge(edge.0,edge.1)},Target{owner:line,part:Part::Curve},Target{owner:curve,part:Part::Curve},Target{owner:path,part:Part::PathEdge(0)},Target{owner:path,part:control},Target{owner:cloud,part:Part::CloudPoint(u32::MAX)}];let mut gpu=renderer(&scene);let mut camera=crate::camera::Camera::default();camera.projection=crate::camera::Projection::Orthographic;camera.distance=3.;camera.aspect=4./3.;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let mut pixels=Vec::new();for target in targets{assert!(target.valid(&scene));gpu.set_highlight(&scene,Some(target)).unwrap();let selected=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(selected!=baseline,"A valid original highlight must change actual pixels: {target:?}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);pixels.push(selected);}gpu.set_highlight(&scene,None).unwrap();gpu.select_objects(&scene,&[line,path,point]).unwrap();let objects=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert_ne!(objects,baseline);gpu.select_objects(&scene,&[]).unwrap();gpu.set_highlight(&scene,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,transform),baseline);
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [mesh,line,path,point,cloud,curve]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.space.update(anchor,3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&far);gpu.set_clouds(&far,&[]).unwrap();camera.target=anchor;gpu.cloud_camera(&camera);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);
 gpu.mesh_ink.set(&gpu.device,crate::pick_sources::Sources::anchored(&far,anchor).unwrap().mesh_ink).unwrap();for(target,expected)in targets.into_iter().zip(pixels){assert!(target.valid(&far));gpu.set_highlight(&far,Some(target)).unwrap();let actual=crate::cap_gpu_tests::pixels_at(&gpu,local);assert!(actual==expected,"Distant original highlight pixels must match exactly: {target:?}; differing bytes: {}",actual.iter().zip(&expected).filter(|(a,b)|a!=b).count());assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);}gpu.set_highlight(&far,None).unwrap();gpu.select_objects(&far,&[line,path,point]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),objects);gpu.select_objects(&far,&[]).unwrap();gpu.set_highlight(&far,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_distant_widgets_keep_local_pixels_scale_picks_and_tile_release(){
 let scene=Scene::empty();let anchor=[1e12,-1e12,1e12];let pivot=[0.125,0.25,-0.5];let mut renderer=renderer(&scene);for projection in [crate::camera::Projection::Perspective,crate::camera::Projection::Orthographic]{for density in [1u32,2]{let mut camera=crate::camera::Camera::default();camera.isometric();camera.projection=projection;camera.aspect=4./3.;let size=[320*density,240*density];renderer.resize(crate::viewport::Viewport{width:size[0],height:size[1]});renderer.set_density(density as f32);renderer.space.forget();let scale=crate::widget_layout::camera_scale(&camera,pivot,240.).unwrap();renderer.set_gizmo(&camera,Some(pivot),-1.);let usage=renderer.gizmo_usage();let extent=renderer.gizmo_extent();let transform=camera.uniform();let baseline=crate::widget_tests::pixels(&mut renderer,&transform,size);assert!(baseline.chunks_exact(4).any(|p|p[..3]!=[255;3]));renderer.space.update(anchor,3.,0.).unwrap();camera.target=anchor;let distant=std::array::from_fn(|i|anchor[i]+pivot[i]);assert_eq!(crate::widget_layout::camera_scale(&camera,distant,240.).unwrap(),scale);renderer.set_gizmo(&camera,Some(distant),-1.);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);let actual=crate::widget_tests::pixels(&mut renderer,&local,size);assert!(actual==baseline,"Precise distant widget pixels differ at density {density}");assert_eq!(renderer.gizmo_extent(),extent);assert_eq!(renderer.gizmo_usage()[2],usage[2]);renderer.request_pick([0.,0.],local).unwrap();crate::cap_gpu_tests::wait(&renderer);assert!(renderer.take_pick(local).unwrap().unwrap().is_none());renderer.set_gizmo(&camera,None,-1.);assert_eq!(renderer.gizmo_usage()[1],0);assert!(renderer.gizmo_extent().is_none());}}
}

#[test]
fn actual_origin_only_pick_views_discard_old_hits_and_restore_owners(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.5],[0.4,-0.4,-0.5],[0.,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);let transform=matrix();let uploads=gpu.stats()[0];gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,gpu.space.anchor());}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert!(gpu.take_pick(transform).is_none(),"Identical matrix in a new origin must not accept the old owner");assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none(),"The automatic current-frame retry is empty");gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none());gpu.space.update([0.;3],3.,300.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[0.;3]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(transform).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_rebased_pick_finishes_a_released_drag_without_reuploading_sources(){
 use crate::editor::{Editor,Action,Change};let mut editor=Editor::default();editor.scene.clear();let owner=editor.scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.5],[0.4,-0.4,-0.5],[0.,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();editor.scene.translate(owner,[2000.,0.,0.]).unwrap();editor.camera.target=[2000.,0.,0.];let original=editor.scene.placement(owner).unwrap().m;let source=editor.scene.objects()[0].geometry().unwrap().clone();let mut gpu=renderer(&editor.scene);let uploads=gpu.stats()[0];let transform=editor.camera.uniform();let size=[320.,240.];editor.apply(Action::CanvasStart{from:[0.,0.],to:[0.2,0.],size,modified:false,pointer:1}).unwrap();gpu.request_pick([0.,0.],transform).unwrap();editor.apply(Action::CanvasRelease{screen:[0.4,0.],size,add:false,remove:false}).unwrap();assert_eq!(editor.scene.placement(owner).unwrap().m,original);gpu.space.update(editor.camera.target,editor.camera.distance,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,editor.camera.target);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let local=editor.camera.view_projection_anchored(gpu.anchor()).to_f32();assert_ne!(local,transform);wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);assert_eq!(editor.scene.placement(owner).unwrap().m,original);wait(&gpu);let hit=gpu.take_pick(local).unwrap().unwrap().and_then(|pixel|gpu.resolve_pick(&editor.scene,pixel));assert_eq!(hit.unwrap().owner,owner);assert_eq!(editor.apply(Action::CanvasHit(hit.map(|target|target.owner))).unwrap(),Change::Scene);assert_ne!(editor.scene.placement(owner).unwrap().m,original);assert!(Rc::ptr_eq(&source,editor.scene.objects()[0].geometry().unwrap()));gpu.set_scene(&editor.scene,editor.selected);assert_eq!(gpu.stats()[0],uploads);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.placement(owner).unwrap().m,original);
}
#[test]
fn actual_rebased_pending_cursor_stays_newest_and_cancellation_never_revives_it(){
 let mut scene=Scene::empty();let make=||PreparedMesh::triangle([[-0.18,-0.18,-0.5],[0.18,-0.18,-0.5],[0.,0.18,-0.5]],[0.2,0.4,0.8]).unwrap();let a=scene.insert(make()).unwrap();let b=scene.insert(make()).unwrap();scene.translate(a,[1999.5,0.,0.]).unwrap();scene.translate(b,[2000.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;let local=matrix();gpu.request_pick([-0.5,0.],old).unwrap();gpu.request_pick([0.5,0.],old).unwrap();assert_eq!(&gpu.pick_usage()[..2],&[1,1]);gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],b.value());assert_eq!(gpu.pick_usage()[3],2);assert!(gpu.take_pick(local).is_none());gpu.request_pick([-0.5,0.],local).unwrap();gpu.cancel_pick();gpu.space.update([0.;3],3.,300.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[0.;3]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));wait(&gpu);assert!(gpu.take_pick(old).is_none());assert_eq!(&gpu.pick_usage()[..2],&[0,0]);gpu.request_pick([-0.5,0.],old).unwrap();gpu.set_scene(&Scene::empty(),None);gpu.space.update([2000.,0.,0.],3.,600.).unwrap();wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_origin_refresh_reuses_selected_face_geometry_and_original_ink(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();scene.translate(owner,[2000.,0.,0.]).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let target=Some(Target{owner,part:Part::Face(key)});let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;gpu.set_highlight(&scene,target).unwrap();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,old);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let geometry=gpu.highlight.geometry_for_test().unwrap().clone();let uploads=gpu.stats()[0];gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&scene);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));let actual=crate::cap_gpu_tests::pixels_at(&gpu,matrix());assert!(actual==baseline,"Rebased original face and ink pixels must match exactly");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);drop(geometry);gpu.set_highlight(&scene,None).unwrap();assert!(gpu.highlight.geometry_for_test().is_none());assert!(!gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzq-prepare-one-shared-render-origin-04.rs" skip=70899
```

</details>

</details>

<a id="checkpoint-96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes"></a>

<details>
<summary>Implementation 96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes: Prepare browser frame matrices and owned wakes</summary>

Use one browser render matrix and prepare a wake owned by the runtime.

Data flow: Renderer origin → matching browser matrix; owned timeout → one later frame.

### 1. `src/timer.rs`

Retain the kind of owned browser timer for cancellation. Reuse the existing callback owner for both repeating reports and one-shot wakes. Schedule the requested timer kind through the same owned callback. Keep the callback alive exactly as long as its timer owner. Cancel the matching interval or timeout when its owner exits.

<details>
<summary>Locate the existing block</summary>

```rust
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

pub struct Timer {
    window: web_sys::Window,
    id: i32,
    _callback: Closure<dyn FnMut()>,
}

impl Timer {
    pub fn every(milliseconds: i32, tick: impl FnMut() + 'static) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let callback = Closure::wrap(Box::new(tick) as Box<dyn FnMut()>);
        let id = window.set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), milliseconds)?;
        Ok(Self { window, id, _callback: callback })
    }
}

impl Drop for Timer {
    fn drop(&mut self) { self.window.clear_interval_with_handle(self.id); }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

pub struct Timer {
    window: web_sys::Window,
    id:i32,
    repeat:bool,
    _callback: Closure<dyn FnMut()>,
}

impl Timer {
    pub fn every(milliseconds:i32,tick:impl FnMut()+'static)->Result<Self,JsValue>{Self::start(milliseconds,tick,true)}
    pub fn after(milliseconds:i32,tick:impl FnMut()+'static)->Result<Self,JsValue>{Self::start(milliseconds,tick,false)}
    fn start(milliseconds:i32,tick:impl FnMut()+'static,repeat:bool)->Result<Self,JsValue>{
        let window = web_sys::window().ok_or("No browser window")?;
        let callback = Closure::wrap(Box::new(tick) as Box<dyn FnMut()>);
        let id=if repeat {window.set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(),milliseconds)?}
            else {window.set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(),milliseconds)?};
        Ok(Self{window,id,repeat,_callback:callback})
    }
}

impl Drop for Timer {
    fn drop(&mut self){if self.repeat{self.window.clear_interval_with_handle(self.id);}else{self.window.clear_timeout_with_handle(self.id);}}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes-01.rs"
```

```text
--8<-- "journey/code/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes-02.rs"
```

```text
--8<-- "journey/code/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes-03.rs"
```

```text
--8<-- "journey/code/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes-04.rs"
```

```text
--8<-- "journey/code/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes-05.rs"
```

</details>

### 2. `src/browser.rs`

Use the matching renderer matrix for every existing browser draw, request and readback path. Expose the actual local frame without replacing the existing precise world-camera diagnostic. Report the matrix after initial viewport layout so it matches the first presented frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    canvas.set_attribute("data-grid-gpu", &serde_json::to_string(&renderer.grid_usage()).unwrap())?;
    canvas.set_attribute("data-normal-view", if editor.normal_view { "true" } else { "false" })?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
```

</details>

Replace it with:

```rust
    canvas.set_attribute("data-grid-gpu", &serde_json::to_string(&renderer.grid_usage()).unwrap())?;
    canvas.set_attribute("data-normal-view", if editor.normal_view { "true" } else { "false" })?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-render-origin",&serde_json::to_string(&renderer.anchor()).unwrap())?;
    canvas.set_attribute("data-render-matrix",&serde_json::to_string(&renderer.camera_matrix(&editor.camera)).unwrap())?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
```

<details>
<summary>Locate this block</summary>

```rust
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                editor.normal_view,
                &mut panel,
            ) {
```

</details>

Replace it with:

```rust
                &surface,
                &renderer,
                &editor.background,
                &renderer.camera_matrix(&editor.camera),
                editor.normal_view,
                &mut panel,
            ) {
```

<details>
<summary>Locate this block</summary>

```rust
            Some(Action::Pick(screen)) => {
                pick_remove.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.ctrl_key()||p.meta_key()));
                pick_additive.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p| p.shift_key() && !p.ctrl_key() && !p.meta_key()));
                if let Err(error) = renderer.request_pick(screen, editor.camera.uniform()) { report(error); }
                None
            }
            Some(action) => { renderer.cancel_pick(); Some(action) },
```

</details>

Replace it with:

```rust
            Some(Action::Pick(screen)) => {
                pick_remove.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.ctrl_key()||p.meta_key()));
                pick_additive.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p| p.shift_key() && !p.ctrl_key() && !p.meta_key()));
                if let Err(error) = renderer.request_pick(screen, renderer.camera_matrix(&editor.camera)) { report(error); }
                None
            }
            Some(action) => { renderer.cancel_pick(); Some(action) },
```

<details>
<summary>Locate this block</summary>

```rust
                let modified=modified||editor.pick_mode!=crate::pick_sources::Mode::Object;
                let start=Action::CanvasStart {from,to,size,modified,pointer};
                let release=event.dyn_ref::<web_sys::PointerEvent>().filter(|_|event.type_()=="pointerup").map(|pointer|Action::CanvasRelease {screen:to,size,add:pointer.shift_key(),remove:pointer.ctrl_key()||pointer.meta_key()});
                if !modified {if let Err(error)=renderer.request_pick(from,editor.camera.uniform()) {report(error);return;}}
                Some(Action::Touch(std::iter::once(start).chain(release).collect()))
            },
            Some(Action::CanvasStart {..}|Action::CanvasMove {..}|Action::CanvasRelease {..}) if editor.draft.is_some()=>None,
```

</details>

Replace it with:

```rust
                let modified=modified||editor.pick_mode!=crate::pick_sources::Mode::Object;
                let start=Action::CanvasStart {from,to,size,modified,pointer};
                let release=event.dyn_ref::<web_sys::PointerEvent>().filter(|_|event.type_()=="pointerup").map(|pointer|Action::CanvasRelease {screen:to,size,add:pointer.shift_key(),remove:pointer.ctrl_key()||pointer.meta_key()});
                if !modified {if let Err(error)=renderer.request_pick(from,renderer.camera_matrix(&editor.camera)) {report(error);return;}}
                Some(Action::Touch(std::iter::once(start).chain(release).collect()))
            },
            Some(Action::CanvasStart {..}|Action::CanvasMove {..}|Action::CanvasRelease {..}) if editor.draft.is_some()=>None,
```

<details>
<summary>Locate this block</summary>

```rust
            Some(Action::Touch(actions))=> {
                let mut applied=Vec::new();
                for action in actions {match action {
                    Action::Pick(screen)=>{if editor.trimming_parts(){let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true});}else if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()){report(error)}}else if editor.draft.is_some() {let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::ToolScreen {screen,size:[r.width(),r.height()],commit:true});} else {pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()) {report(error);}}},
                    action=>{renderer.cancel_pick();applied.push(action);}
                }}Some(Action::Touch(applied))
            },
```

</details>

Replace it with:

```rust
            Some(Action::Touch(actions))=> {
                let mut applied=Vec::new();
                for action in actions {match action {
                    Action::Pick(screen)=>{if editor.trimming_parts(){let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true});}else if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)){report(error)}}else if editor.draft.is_some() {let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::ToolScreen {screen,size:[r.width(),r.height()],commit:true});} else {pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)) {report(error);}}},
                    action=>{renderer.cancel_pick();applied.push(action);}
                }}Some(Action::Touch(applied))
            },
```

<details>
<summary>Locate this block</summary>

```rust
            }
        } else { match action {
            Some(Action::Pick(screen)) if editor.trimming_parts()=>{let r=pointer_canvas.get_bounding_client_rect();Some(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true})},
            Some(Action::Pick(screen)) if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves()=>{pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()){report(error)}None},
            Some(Action::Pick(screen)) if editor.draft.is_some() => {
                Some(Action::ToolScreen { screen,size:[pointer_canvas.get_bounding_client_rect().width(),pointer_canvas.get_bounding_client_rect().height()],commit:true })
            }
```

</details>

Replace it with:

```rust
            }
        } else { match action {
            Some(Action::Pick(screen)) if editor.trimming_parts()=>{let r=pointer_canvas.get_bounding_client_rect();Some(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true})},
            Some(Action::Pick(screen)) if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves()=>{pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)){report(error)}None},
            Some(Action::Pick(screen)) if editor.draft.is_some() => {
                Some(Action::ToolScreen { screen,size:[pointer_canvas.get_bounding_client_rect().width(),pointer_canvas.get_bounding_client_rect().height()],commit:true })
            }
```

<details>
<summary>Locate this block</summary>

```rust
        } else if let Some(handle_action)=handle_action {
            renderer.cancel_pick();gesture.cancel();Some(handle_action)
        } else if event.type_() == "viewer-pick" {
            match renderer.take_pick(editor.camera.uniform()) {
                Some(Ok(hit)) => {
                    let target=hit.and_then(|hit|renderer.resolve_pick(&editor.scene,hit));
                    if editor.element.is_some(){Some(Action::ElementPick(target.map(|t|t.owner)))}else if editor.trimming_picks(){Some(Action::TrimPick(target.map(|t|t.owner)))}else if editor.split.is_some(){Some(Action::SplitPick(target.map(|t|t.owner)))}else if editor.editing_boundaries(){Some(Action::ExtendPick(target.map(|t|t.owner)))}else if editor.gathering_curves(){Some(Action::GatherPick(target.map(|t|t.owner)))}
```

</details>

Replace it with:

```rust
        } else if let Some(handle_action)=handle_action {
            renderer.cancel_pick();gesture.cancel();Some(handle_action)
        } else if event.type_() == "viewer-pick" {
            match renderer.take_pick(renderer.camera_matrix(&editor.camera)) {
                Some(Ok(hit)) => {
                    let target=hit.and_then(|hit|renderer.resolve_pick(&editor.scene,hit));
                    if editor.element.is_some(){Some(Action::ElementPick(target.map(|t|t.owner)))}else if editor.trimming_picks(){Some(Action::TrimPick(target.map(|t|t.owner)))}else if editor.split.is_some(){Some(Action::SplitPick(target.map(|t|t.owner)))}else if editor.editing_boundaries(){Some(Action::ExtendPick(target.map(|t|t.owner)))}else if editor.gathering_curves(){Some(Action::GatherPick(target.map(|t|t.owner)))}
```

<details>
<summary>Locate this block</summary>

```rust
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            editor.normal_view,
            &mut panel,
        )?;
```

</details>

Replace it with:

```rust
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        inspect(&editor,&renderer)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &renderer.camera_matrix(&editor.camera),
            editor.normal_view,
            &mut panel,
        )?;
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) {
            crate::browser_report::fatal(&message);
            if crate::browser_recovery::request(&message) { return; }
            report(&format!("Cannot draw: {message}"));
        }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> {
    let shared=crate::drawing_document::new();
    crate::browser_defaults::initialize(&mut shared.borrow_mut());
    run_with(shared).await
}

pub async fn run_with(shared: crate::drawing_document::Shared) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let recovery = crate::browser_recovery::adopt()?;
    crate::browser_report::start()?;
    let startup = crate::browser_startup::Guard::new()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter_started = crate::browser_phase::now();
    let probe = crate::browser_adapter::Probe::new().ok();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await;
    if !startup.permits() { return Ok(()); }
    crate::browser_phase::finish("adapter request", adapter_started, 0, "GPU");
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) { let _ = crate::browser_report::adapter(info); }
    else { let _ = crate::browser_report::observe("diagnostic", "Adapter identity unavailable"); }
    drop(probe);
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
    if !startup.permits() {
        if let Ok((device, _)) = device { device.destroy(); }
        return Ok(());
    }
    crate::browser_phase::finish("device request", device_started, 0, "GPU");
    let (device, queue) = device.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| failed(errors.clone(), format!("WebGPU error: {error}"))));
    let lost = fault.clone();
    device.set_device_lost_callback(move |reason, message| failed(lost.clone(), format!("WebGPU device lost ({reason:?}): {message}")));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = shared.borrow_mut();
    let pipeline_started = crate::browser_phase::now();
    let text_fonts=std::rc::Rc::new(crate::text_fonts::Inbox::default());
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    crate::browser_phase::finish("renderer setup", pipeline_started, 0, "GPU");
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        crate::commands::names(),
    );
    if let Some(message) = crate::browser_report::previous_notice() { panel.result(message); }
    if let Some(message) = &recovery { panel.result(message); }
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        inspect(&editor,&renderer)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &renderer.camera_matrix(&editor.camera),
            editor.normal_view,
            &mut panel,
        )?;
        let completing = fault.clone();
        // Queue completion runs after submitted GPU work; checking the current device prevents a late result from reviving an exited runtime.
        renderer.queue.on_submitted_work_done(move || {
            if crate::browser_runtime::owns(&completing) && completing.message().is_none() {
                crate::browser_phase::finish("first frame complete", frame_started, 0, "GPU");
                let _ = crate::browser_report::observe("milestone", "geometry on screen");
            }
        });
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let mut touch=crate::touch::Touch::default();
    let pick_remove=std::cell::Cell::new(false);
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let publication=crate::browser_publication::Publication::new(std::rc::Rc::clone(&shared),fault.clone())?;
    let publication_control=publication.control();let start_publication=publication_control.clone();
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let sheet_query=crate::browser_sheet::Control::default();
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        let mut editor = shared.borrow_mut();
        if matches!(event.type_().as_str(), "blur" | "pagehide" | "viewer-reload") { renderer.cancel_pick(); }
        if event.type_() == "pagehide" {
            gesture.cancel();touch.cancel();let _=editor.apply(Action::CanvasCancel); let _=editor.apply(Action::GizmoCancel);
            crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());renderer.set_gizmo(&editor.camera,None,-1.);
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
        if matches!(event.type_().as_str(),"blur"|"resize"|"pointercancel")||event.dyn_ref::<web_sys::KeyboardEvent>().is_some_and(|key|key.key()=="Escape"&&!key.is_composing()) {
            gesture.cancel();touch.cancel();renderer.cancel_pick();if editor.apply(Action::CanvasCancel)==Ok(Change::Preview) {crate::browser_upload::prepare(&mut renderer,&editor.scene,editor.whole_selection(),active_fault.clone());}
        }
        if event.type_()=="lostpointercapture"&&event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|editor.canvas_drag.as_ref().is_some_and(|d|d.pointer==p.pointer_id()&&d.released.is_none())) {
            gesture.cancel();renderer.cancel_pick();if editor.apply(Action::CanvasCancel)==Ok(Change::Preview) {crate::browser_upload::prepare(&mut renderer,&editor.scene,editor.whole_selection(),active_fault.clone());}
        }
        if event.type_()=="pointerdown"&&event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.is_primary())&&editor.canvas_drag.as_ref().is_none_or(|d|d.released.is_some()) {renderer.cancel_pick();let _=editor.apply(Action::CanvasCancel);}
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let navigation=if line.is_none()&&!panel.consumed {navigation_action(&event,&pointer_canvas,&mut gesture,&mut touch)} else {None};
        let line=if gesture.take_repeat() {event.prevent_default();if editor.draft.is_some()||editor.gizmo_edit.is_some() {Some(String::new())} else {panel.repeat_line()}} else {line};
        let line = if (editor.lasso.is_some() || editor.trim.is_some() || editor.mark.is_some() || editor.split.is_some() || editor.draft.is_some() || editor.gizmo_edit.is_some()) && event.dyn_ref::<web_sys::KeyboardEvent>()
            .is_some_and(|key| event.type_() == "keydown" && key.key() == "Escape" && !key.is_composing()) {
            event.prevent_default(); gesture.cancel(); Some("Escape".into())
        } else { line };

        let mut highlight_changed = false;
        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.as_ref().map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        highlight_changed = true;
                        crate::browser_upload::prepare(&mut renderer, editor.display_scene(), editor.whole_selection(), active_fault.clone());
                        crate::browser_phase::revision(&editor.scene);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
        } else if event.type_() == "viewer-stream-page" {
            renderer.cancel_pick();Some(Action::StreamRefresh)
        } else if event.type_() == "viewer-publication" {
            match publication_control.take() {Ok(action)=>action,Err(error)=>{let message=format!("Cannot load scene: {error}");report(&message);panel.result(&message);None}}
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery, &editor.scene) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot open file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            let command = editor.tool_line(&line).map(|result| result.map(crate::commands::Command::Editor).map_err(str::to_owned))
                .unwrap_or_else(|| crate::commands::parse(&line));
            editor.mark=None;
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { crate::stream::reset_range_gate();publication_control.cancel();request.borrow_mut().cancel();let _={text_fonts.clear();renderer.clear_text_fonts();renderer.set_text(Vec::new())}; }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
            }
            Ok(crate::commands::Command::Host(host)) => {
            use crate::commands::Host;
            match host {
            Host::TextDemo=>{if let Err(error)=renderer.text_demo() {panel.result(&error.to_string());}None},
            Host::Help => { panel.result(&crate::commands::names().join(" · ")); None }
            Host::Selection => {
                let rows = editor.selected_objects();
                panel.result(&if rows.len() > 1 { format!("Objects {:?}", rows.iter().map(|id| id.value()).collect::<Vec<_>>()) }
                    else { editor.selection().map_or_else(|| "Nothing selected".to_owned(), |target| target.describe()) });
                None
            }
            Host::Reload => {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            }
            Host::CancelReload => {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            }
            Host::CancelOpen => {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            }
            Host::Open | Host::Replace => {
                publication_control.cancel();
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if host == Host::Replace { reload.borrow_mut().cancel(); }
                read_mode.set(if host == Host::Replace { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            }
            Host::Report | Host::PreviousReport => {
                let output = if host == Host::PreviousReport { crate::browser_report::download_previous() }
                    else { crate::browser_report::download() };
                let result = output.map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
            }
            Host::LayersOn=>{panel.tree.open=true;None},
            Host::LayersOff=>{panel.tree.open=false;None},
            Host::LayersAll=>{panel.tree.all();None},
            Host::Save => {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::scene_archive::save(&editor.scene)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            }
            }
            }
            }
        } else if !panel.consumed {
            navigation
        } else {
            None
        };
        let handle_action=lasso_action(&event,&pointer_canvas,&editor,panel.consumed).or_else(||gizmo_action(&event,&pointer_canvas,&editor,panel.consumed));
        let action = if let Some((key,edit))=panel.tree.edit.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::LayerEdit {key,edit})
        } else if let Some((key,property))=panel.tree.property.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::LayerProperty {key,property})
        } else if let Some((ids,additive))=panel.tree.action.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::TreePick {ids,additive})
        } else if let Some(handle_action)=handle_action {
            renderer.cancel_pick();gesture.cancel();Some(handle_action)
        } else if event.type_() == "viewer-pick" {
            match renderer.take_pick(renderer.camera_matrix(&editor.camera)) {
                Some(Ok(hit)) => {
                    let target=hit.and_then(|hit|renderer.resolve_pick(&editor.scene,hit));
                    if editor.element.is_some(){Some(Action::ElementPick(target.map(|t|t.owner)))}else if editor.trimming_picks(){Some(Action::TrimPick(target.map(|t|t.owner)))}else if editor.split.is_some(){Some(Action::SplitPick(target.map(|t|t.owner)))}else if editor.editing_boundaries(){Some(Action::ExtendPick(target.map(|t|t.owner)))}else if editor.gathering_curves(){Some(Action::GatherPick(target.map(|t|t.owner)))}
                    else if editor.canvas_drag.is_some() {Some(Action::CanvasHit(target.map(|t|t.owner)))}
                    else if pick_remove.get() {Some(Action::SelectRows {ids:target.filter(|t|t.part==crate::pick_sources::Part::Object).map(|t|t.owner).into_iter().collect(),add:false,remove:true})}
                    else {Some(Action::PickedMany {target,additive:pick_additive.get()})}
                },
                Some(Err(error)) => { report(error); None },
                None => None,
            }
        } else { match action {
            Some(Action::Pick(screen)) if editor.trimming_parts()=>{let r=pointer_canvas.get_bounding_client_rect();Some(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true})},
            Some(Action::Pick(screen)) if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves()=>{pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)){report(error)}None},
            Some(Action::Pick(screen)) if editor.draft.is_some() => {
                Some(Action::ToolScreen { screen,size:[pointer_canvas.get_bounding_client_rect().width(),pointer_canvas.get_bounding_client_rect().height()],commit:true })
            }
            None if editor.trimming_parts()&&event.type_()=="pointermove"=>{let r=pointer_canvas.get_bounding_client_rect();event.dyn_ref::<web_sys::PointerEvent>().and_then(|e|(r.width()>0.&&r.height()>0.).then(||[(2.*(e.client_x()as f64-r.left())/r.width()-1.)as f32,(1.-2.*(e.client_y()as f64-r.top())/r.height())as f32])).map(|screen|Action::TrimScreen{screen,size:[r.width(),r.height()],commit:false})},
            None if editor.draft.is_some() && matches!(event.type_().as_str(), "blur" | "resize" | "pointercancel" | "lostpointercapture") => Some(Action::ToolHover(None)),
            None if editor.draft.is_some() && !panel.consumed && event.type_() == "pointermove" => {
                event.dyn_ref::<web_sys::PointerEvent>().filter(|p| p.buttons() == 0).map(|p| {
                    let r = pointer_canvas.get_bounding_client_rect();
                    let screen = [(2. * (p.client_x() as f64-r.left())/r.width()-1.) as f32, (1.-2.*(p.client_y() as f64-r.top())/r.height()) as f32];
                    Action::ToolScreen { screen,size:[r.width(),r.height()],commit:false }
                })
            }
            Some(Action::Touch(actions))=> {
                let mut applied=Vec::new();
                for action in actions {match action {
                    Action::Pick(screen)=>{if editor.trimming_parts(){let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true});}else if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)){report(error)}}else if editor.draft.is_some() {let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::ToolScreen {screen,size:[r.width(),r.height()],commit:true});} else {pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)) {report(error);}}},
                    action=>{renderer.cancel_pick();applied.push(action);}
                }}Some(Action::Touch(applied))
            },
            Some(Action::CanvasStart {from,to,size,modified,pointer}) if editor.draft.is_none()=> {
                renderer.cancel_pick();
                let modified=modified||editor.pick_mode!=crate::pick_sources::Mode::Object;
                let start=Action::CanvasStart {from,to,size,modified,pointer};
                let release=event.dyn_ref::<web_sys::PointerEvent>().filter(|_|event.type_()=="pointerup").map(|pointer|Action::CanvasRelease {screen:to,size,add:pointer.shift_key(),remove:pointer.ctrl_key()||pointer.meta_key()});
                if !modified {if let Err(error)=renderer.request_pick(from,renderer.camera_matrix(&editor.camera)) {report(error);return;}}
                Some(Action::Touch(std::iter::once(start).chain(release).collect()))
            },
            Some(Action::CanvasStart {..}|Action::CanvasMove {..}|Action::CanvasRelease {..}) if editor.draft.is_some()=>None,
            Some(action @ (Action::CanvasMove {..}|Action::CanvasRelease {..}))=>Some(action),
            Some(Action::Pick(screen)) => {
                pick_remove.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.ctrl_key()||p.meta_key()));
                pick_additive.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p| p.shift_key() && !p.ctrl_key() && !p.meta_key()));
                if let Err(error) = renderer.request_pick(screen, renderer.camera_matrix(&editor.camera)) { report(error); }
                None
            }
            Some(action) => { renderer.cancel_pick(); Some(action) },
            None => None,
        } };
        if let Some(action) = action {
            highlight_changed |= matches!(&action, Action::SelectAll);
            let group_action=matches!(&action,Action::AddGroup(_));let graph_action=matches!(&action,Action::AddEdge);
            let rename_action=matches!(&action,Action::LayerEdit {edit:crate::layer_edit::Edit::Rename(_),..});
            let tool_action = matches!(&action, Action::ElementBegin{..}|Action::ElementText(_)|Action::ElementPick(_)|Action::ElementAttributes(_)| Action::AddGroup(_) | Action::AddEdge | Action::SelectLasso | Action::LassoRelease{..} | Action::LassoCancel | Action::TextBegin(_) | Action::Trim | Action::TrimPick(_) | Action::TrimScreen{commit:true,..} | Action::CancelTrim | Action::Arrowhead(_) | Action::Explode | Action::Query(_) | Action::Measure(_) | Action::MeasureDistance(_) | Action::BoundingBox | Action::Split | Action::SplitPick(_) | Action::CancelSplit | Action::BeginTool(_) | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::CancelTool | Action::Snap(_) | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::ToolScreen { commit:true,.. });
            if matches!(&action, Action::CancelTool | Action::CancelSplit | Action::CancelTrim) { panel.cancel_input(); }
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Live(_) | Action::Undo | Action::Redo
                | Action::Adopt {..} | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            if closing_action(&action) {publication_control.cancel();}
            let replacement = matches!(&action, Action::Adopt {replace:true,..} | Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = crate::edit_intent::Intent::capture_for(&action,&editor)
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            if change.is_ok()&&(group_action||graph_action){panel.tree.open=true;if graph_action{panel.tree.graph_open=true;}}
            if rename_action {panel.tree.rename_result(change.as_ref().err().map(String::as_str));}
            if tool_action && change.is_ok() {highlight_changed=true;panel.result(&editor.tool_status); }
            match change {
                Ok(Some(Change::Scene)) => {
                    highlight_changed = true;
                    crate::browser_upload::prepare(&mut renderer, &editor.scene, editor.whole_selection(), active_fault.clone());
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-publication" {
                        crate::browser_phase::revision(&editor.scene);let _=crate::browser_report::observe("live-reload","source bytes changed");report("URL scene replaced.");panel.result("URL scene replaced.");
                    } else if event.type_() == "viewer-file" {
                        crate::browser_phase::revision(&editor.scene);
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::Preview)) => {
                    highlight_changed=true;
                    crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    let error=if error=="Shape input invalid" {editor.tool_status.as_str()}else{error.as_str()};
                    report(error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        if let Some(element)=&editor.element{panel.tool(element.prompt(),element.buttons());}
        else if editor.plane_menu {panel.tool(editor.tool_status.clone(),crate::drawing_input::buttons());}
        else if let Some(trim)=&editor.trim{panel.tool(trim.prompt(),vec![("Enter".into(),"Trim".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(split)=&editor.split{panel.tool(split.prompt(),vec![("Split".into(),"Split".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(draft) = editor.draft.as_ref() {let mut buttons=draft.tool.buttons();if draft.text.is_some(){buttons.push(("Height".into(),"Height".into()));}buttons.extend(crate::drawing_input::buttons());panel.tool(draft.prompt(),buttons);}
        else { panel.tool(String::new(), Vec::new()); }
        if let Some(reply)=text_fonts.reply.borrow_mut().take() {match reply {Ok(fonts)=>{if let Err(error)=renderer.replace_text_fonts(fonts) {text_fonts.failed();panel.result(&format!("Text: {error}"));}},Err(error)=>panel.result(&format!("Text: {error}"))}}
        sheet_query.observe(&editor.scene,editor.selection(),&active_fault);
        if let Some(reply)=sheet_query.take() {match reply {Ok(entity)=>panel.result(&format!("{} · {} · {}",entity.name,entity.kind,entity.guid)),Err(error)=>panel.result(&error)}}
        let guides=editor.guide_overlay_at(renderer.anchor());
        if let Err(error)=renderer.set_guides(&guides){panel.result(error);}
        if let Err(error)=renderer.set_measurement(editor.mark.as_ref()){panel.result(error);}
        renderer.refresh_text_bounds(&mut editor.scene);
        text_fonts.need(editor.scene.texts().iter().filter(|row|editor.scene.visible(row.id)).map(|row|row.prepared.source.label.text.as_str()));
        let rect=pointer_canvas.get_bounding_client_rect();
        let number=editor.number_prompt([rect.width(),rect.height()]);
        if editor.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some()) && number.is_none() {
            if editor.apply(Action::GizmoCancel).is_ok() {highlight_changed=true;crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());}
        }
        panel.rectangle=editor.canvas_drag.as_ref().filter(|d|d.missed).and_then(|d|d.rectangle());
        panel.number(number);
        panel.lasso=editor.lasso.as_ref().map_or_else(Vec::new,|l|l.points.iter().map(|p|[(p.0/l.density) as f32,(p.1/l.density) as f32]).collect());
        panel.tree.refresh_cached(&editor.scene,editor.selected_objects(),editor.interaction_epoch);
        if let Err(error) = renderer.set_preview(editor.draft.as_ref()) { panel.result(error); }
        renderer.set_pick_mode(if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){crate::pick_sources::Mode::Object}else{editor.pick_mode});
        if highlight_changed {
            if let Err(error) = renderer.set_highlight(editor.display_scene(), editor.selection())
                .and_then(|()| renderer.select_objects(editor.display_scene(), &editor.highlighted_objects())) { report(error); return; }
        }
        renderer.set_clipping(editor.clipping_enabled,editor.clipping_solid);
        renderer.set_opacity(editor.opacity);renderer.set_ambient(editor.arctic);renderer.set_outlines(editor.outlines);
        if let Err(error) = renderer.set_grid_on(editor.camera.grid,editor.drawing_plane()) {
            report(error); return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Some(frame)=panel.free_frame([rect.width(),rect.height()]) {if editor.camera.frame(frame) {renderer.cancel_pick();let _=editor.apply(Action::CanvasCancel);}}
            let hover=event.dyn_ref::<web_sys::PointerEvent>().filter(|p|event.type_()=="pointermove" && p.buttons()==0 && !panel.consumed).map(|p| {
                let r=pointer_canvas.get_bounding_client_rect();
                ([(2.*(p.client_x() as f64-r.left())/r.width()-1.) as f32,(1.-2.*(p.client_y() as f64-r.top())/r.height()) as f32],[r.width(),r.height()])
            });
            editor.hover_gizmo(hover);
            renderer.set_gizmo(&editor.camera,editor.gizmo_origin(),editor.gizmo_active());
            renderer.cloud_camera(&editor.camera);
            if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &renderer.camera_matrix(&editor.camera),
                editor.normal_view,
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    let mut listeners = crate::listeners::Listeners::new(update);
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        listeners.listen(canvas.as_ref(), name)?;
    }
    listeners.listen(canvas.as_ref(), "wheel")?;
    listeners.listen(window.as_ref(), "resize")?;
    listeners.listen(window.as_ref(), "blur")?;
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
    listeners.listen(window.as_ref(), "viewer-stream-page")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault, owned_editor, publication);
    report(&recovery.unwrap_or_else(|| "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.".into()));
    start_publication.begin();
    Ok(())
}

fn closing_action(action:&Action)->bool {matches!(action,Action::Close|Action::Adopt {..}|Action::Replace(_)|Action::ReplaceAt(_, _))}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    let view_ids=editor.scene.objects().iter().map(|row|row.id).chain(editor.scene.lines().iter().map(|row|row.id)).chain(editor.scene.paths().iter().map(|row|row.id)).chain(editor.scene.points().iter().map(|row|row.id)).chain(editor.scene.texts().iter().map(|row|row.id)).chain(editor.scene.clouds().iter().map(|row|row.id));
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    let local=editor.scene.forest.documents.iter().flat_map(|doc|doc.nodes.iter().map(move |node|serde_json::json!({"document":doc.key,"name":node.source.name,"object":node.object.map(|id|id.value()),"local":node.placement.local.m,"original_world":node.placement.world.m}))).collect::<Vec<_>>();
    canvas.set_attribute("data-original-tree-placements",&serde_json::to_string(&local).unwrap())?;
    canvas.set_attribute("data-camera-frame",&serde_json::to_string(&editor.camera.frame).unwrap())?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-source-metadata",&serde_json::to_string(&crate::source_metadata::usage(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-groups",&serde_json::json!(editor.scene.groups.iter().collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-graph",&serde_json::json!(crate::graph_edit::rows(&editor.scene)).to_string())?;
    canvas.set_attribute("data-lasso",&editor.lasso.as_ref().map_or_else(||"null".into(),|l|serde_json::json!({"points":l.points.len(),"spacing":l.spacing,"density":l.density}).to_string()))?;
    canvas.set_attribute("data-tree",&serde_json::json!(crate::hierarchy::rows(&editor.scene).iter().map(|r|serde_json::json!({"key":r.key,"label":r.label,"depth":r.depth,"end":r.end,"readonly":r.readonly,"ids":r.ids.iter().map(|id|id.value()).collect::<Vec<_>>()})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute("data-triangle-tiles",&serde_json::to_string(&renderer.tile_usage()).unwrap())?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-cloud-cpu",&serde_json::to_string(&crate::memory::clouds(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-cloud-gpu",&serde_json::to_string(&renderer.cloud_usage()).unwrap())?;
    canvas.set_attribute("data-cloud-error",&renderer.cloud_error().unwrap_or_default())?;
    canvas.set_attribute("data-cloud-sources",&serde_json::json!(editor.scene.clouds().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid(),"point_size":row.prepared.source.point_size,"point_px":row.point_px,"points":row.prepared.source.len(),"nodes":row.prepared.nodes.len(),"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-surface-outline",&serde_json::to_string(&renderer.outline_usage()).unwrap())?;
    canvas.set_attribute("data-scene-text-stats",&serde_json::to_string(&renderer.text_stats()).unwrap())?;
    canvas.set_attribute("data-scene-text-cpu",&serde_json::to_string(&crate::memory::texts(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-scene-text-gpu",&serde_json::to_string(&renderer.text_scene_usage()).unwrap())?;
    canvas.set_attribute("data-scene-texts",&serde_json::json!(editor.scene.texts().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source":row.prepared.source,"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-text-error",&renderer.text_error().unwrap_or_default())?;
    canvas.set_attribute("data-text-atlas",&serde_json::to_string(&renderer.text_usage()).unwrap())?;
    canvas.set_attribute("data-cad-walks",&serde_json::json!({"scope":"last imported original document","walks":editor.scene.cad_walks[0],"replays":editor.scene.cad_walks[1]}).to_string())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
    let cads=editor.scene.objects().iter().filter_map(|row|row.cad.as_ref().map(|cad|serde_json::json!({"id":row.id.value(),"source":cad.source.guid(),"faces":row.mesh.parts.faces,"edges":cad.edges.iter().map(|border|border.edge).collect::<Vec<_>>(),"vertices":cad.samples.len()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-cads",&serde_json::to_string(&cads).unwrap())?;
    canvas.set_attribute("data-surface-cpu",&serde_json::to_string(&crate::memory::surfaces(editor.presentation_scenes())).unwrap())?;
    let surfaces=editor.scene.objects().iter().filter_map(|row|row.surface.as_ref().map(|s|serde_json::json!({"id":row.id.value(),"source":s.source.guid(),"controls":s.source.m_cv_count,"steps":s.grid.steps,"samples":s.grid.samples.len(),"faces":row.mesh.parts.faces,"edges":s.grid.boundaries.iter().map(|b|b.edge).collect::<Vec<_>>()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-surfaces",&serde_json::to_string(&surfaces).unwrap())?;
    canvas.set_attribute("data-curve-cpu", &serde_json::to_string(&crate::memory::curves(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-control-count", &editor.scene.control_markers().len().to_string())?;
    canvas.set_attribute("data-point-cpu", &serde_json::to_string(&crate::memory::points(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-path-gpu", &serde_json::to_string(&renderer.path_usage()).unwrap())?;
    canvas.set_attribute("data-path-cpu", &serde_json::to_string(&crate::memory::paths(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-stroke-usage", &serde_json::to_string(&renderer.stroke_usage()).unwrap())?;
    canvas.set_attribute("data-line-usage", &serde_json::to_string(&crate::memory::lines(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-edit-preview", &editor.edit_preview.as_ref().map_or_else(|| "null".into(),|p| serde_json::json!({"ids":p.ids().iter().map(|id| id.value()).collect::<Vec<_>>(),"matrix":p.matrix()}).to_string()))?;
    let originals=serde_json::json!({"points":editor.scene.points().iter().map(|row|(0..3).map(|i|row.prepared.source[i]).collect::<Vec<_>>()).collect::<Vec<_>>(),"lines":editor.scene.lines().iter().map(|row|{let s=&row.prepared.source;[s.start(),s.end()].map(|p|[p[0],p[1],p[2]])}).collect::<Vec<_>>(),"polylines":editor.scene.paths().iter().filter(|row|matches!(row.prepared.source,crate::chain::Source::Polyline(_))).map(|row|row.prepared.source.coordinates()).collect::<Vec<_>>()});
    canvas.set_attribute("data-curve-originals",&serde_json::to_string(&editor.scene.paths().iter().filter_map(|row|match &row.prepared.source {crate::chain::Source::Curve(source,_)=>Some(serde_json::json!({"guid":source.guid(),"controls":source.m_cv,"knots":source.m_nurbsknot,"rational":source.m_is_rat,"count":source.cv_count()})),_=>None}).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-drawing-originals",&originals.to_string())?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute("data-measurement",&editor.mark.as_ref().map_or_else(||"null".into(),|m|serde_json::json!({"points":m.points,"label":m.label}).to_string()))?;
    canvas.set_attribute("data-measurement-gpu-bytes",&renderer.measurement_bytes().to_string())?;
    canvas.set_attribute("data-guide-gpu-bytes",&renderer.guide_bytes().to_string())?;
    canvas.set_attribute("data-trim",&editor.trim.as_ref().map_or_else(||"null".into(),|t|serde_json::json!({"phase":format!("{:?}",t.phase),"targets":t.targets.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"cutters":t.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"parts":t.cuts.iter().map(|c|c.parts.count()).collect::<Vec<_>>(),"removed":t.cuts.iter().map(|c|&c.removed).collect::<Vec<_>>(),"hover":t.hover}).to_string()))?;
    canvas.set_attribute("data-split", &editor.split.as_ref().map_or_else(||"null".into(),|s|serde_json::json!({"target":s.target.id.value(),"face":s.face,"cutters":s.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>()}).to_string()))?;
    canvas.set_attribute("data-project", &editor.draft.as_ref().and_then(|d|d.project.as_ref()).map_or_else(||"null".into(), |p|serde_json::json!({"targets":p.targets.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"points":p.points,"unsupported":p.unsupported}).to_string()))?;
    canvas.set_attribute("data-extend", &editor.draft.as_ref().and_then(|d|d.extend.as_ref()).map_or_else(||"null".into(), |e|serde_json::json!({"boundaries":e.boundaries.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"curves":e.curves,"distance":e.distance,"asking":e.asking,"candidates":e.candidates.len(),"hover":e.hover.as_ref().map(|h|serde_json::json!({"owner":h.owner.id.value(),"end":format!("{:?}",h.end),"tail":h.tail})),"error":e.error}).to_string()))?;
    canvas.set_attribute("data-gather",&editor.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||"null".into(),|g|serde_json::json!({"name":g.recipe.name,"step":g.progress.step,"rows":g.progress.rows,"answered":g.progress.answered,"choice":g.choice}).to_string()))?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
    canvas.set_attribute("data-snap-settings", &serde_json::json!({"objects":editor.snapping.objects,"modes":editor.snapping.modes,"grid":editor.snapping.grid,"step":editor.snapping.step}).to_string())?;
    canvas.set_attribute("data-snap-usage", &serde_json::to_string(&editor.snap_usage()).unwrap())?;
    canvas.set_attribute("data-preview-bytes", &renderer.preview_bytes().to_string())?;
    canvas.set_attribute("data-highlight-bytes", &renderer.highlight_bytes().to_string())?;
    canvas.set_attribute("data-selection-part", &editor.selection().map_or_else(String::new, |target| target.describe()))?;
    let sheets=editor.scene.sheets().iter().map(|row|serde_json::json!({"row":row.id.value(),"guid":row.guid,"source":row.prepared.original.guid,"model":row.model.m,"segments":row.prepared.original.segment_count,"entities":row.prepared.original.entity_count,"meta":row.prepared.original.meta,"streamed":row.prepared.streamed.is_some(),"resident":row.prepared.pages().iter().map(|page|page.rows.end-page.rows.start).sum::<u32>()})).collect::<Vec<_>>();canvas.set_attribute("data-sheets",&serde_json::to_string(&sheets).unwrap())?;canvas.set_attribute("data-sheet-cpu",&serde_json::to_string(&crate::memory::sheets(editor.scenes())).unwrap())?;canvas.set_attribute("data-sheet-gpu",&serde_json::to_string(&renderer.sheet_usage()).unwrap())?;
    canvas.set_attribute("data-instance-cpu",&serde_json::to_string(&crate::memory::instances(editor.scenes())).unwrap())?;
    canvas.set_attribute("data-instance-gpu",&serde_json::to_string(&renderer.instance_usage()).unwrap())?;
    canvas.set_attribute("data-instance-allocations",&serde_json::to_string(&renderer.instance_allocations()).unwrap())?;
    let instances=editor.scene.instances.iter().map(|(id,owner)|serde_json::json!({"id":id.value(),"guid":editor.scene.row_guid(*id),"source_guid":owner.instance.guid(),"definition_guid":owner.definition.guid(),"document_guid":owner.document.guid(),"model":editor.scene.placement(*id).map(|m|m.m),"flags":owner.instance.flags,"colour":editor.scene.face_rgba(*id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-instances",&serde_json::to_string(&instances).unwrap())?;
    let stream_usage=crate::memory::streams(editor.scenes());
    canvas.set_attribute("data-range-sources",&serde_json::to_string(&stream_usage).unwrap())?;
    let streamed=editor.scene.streams().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid,"name":row.prepared.source.name,"url":row.prepared.source.url,"revision":row.prepared.source.fields.revision,"total":row.prepared.source.fields.count,"model":row.model.m,"point_px":row.point_px,"controls":row.controls_visible})).collect::<Vec<_>>();
    canvas.set_attribute("data-range-documents",&serde_json::to_string(&streamed).unwrap())?;

    canvas.set_attribute("data-pick-mode", &format!("{:?}", editor.pick_mode))?;
    canvas.set_attribute("data-part-usage", &serde_json::to_string(&renderer.part_usage()).unwrap())?;
    canvas.set_attribute("data-selected-ids", &serde_json::to_string(&editor.selected_objects().iter().map(|id| id.value()).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-selected-id", &editor.selected.map(|id| format!("{id:?}")).unwrap_or_default())?;
    canvas.set_attribute("data-grid-plane",&serde_json::json!({"origin":(0..3).map(|i|editor.drawing_plane().origin()[i]).collect::<Vec<_>>(),"matrix":editor.drawing_plane().matrix()}).to_string())?;
    canvas.set_attribute("data-grid-enabled", if editor.camera.grid.is_some() { "true" } else { "false" })?;
    canvas.set_attribute("data-grid-gpu", &serde_json::to_string(&renderer.grid_usage()).unwrap())?;
    canvas.set_attribute("data-normal-view", if editor.normal_view { "true" } else { "false" })?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-render-origin",&serde_json::to_string(&renderer.anchor()).unwrap())?;
    canvas.set_attribute("data-render-matrix",&serde_json::to_string(&renderer.camera_matrix(&editor.camera)).unwrap())?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let mut cpu = crate::memory::cpu(editor.presentation_scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    cpu[4]+=renderer.arena_cpu_bytes();
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
    touch:&mut crate::touch::Touch,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            let rect=canvas.get_bounding_client_rect();
            let Action::Zoom(factor)=crate::navigation::wheel(wheel.delta_y(),mode,rect.height())? else {return None;};
            let screen=[(2.*(wheel.client_x() as f64-rect.left())/rect.width()-1.) as f32,(1.-2.*(wheel.client_y() as f64-rect.top())/rect.height()) as f32];
            Some(Action::ZoomAt {factor,screen})
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _=>{
            if let Some(pointer)=event.dyn_ref::<web_sys::PointerEvent>().filter(|p|p.pointer_type()=="touch") {
                event.prevent_default();let r=canvas.get_bounding_client_rect();let rect=[r.left(),r.top(),r.width(),r.height()];let at=[pointer.client_x() as f64,pointer.client_y() as f64];let id=pointer.pointer_id();
                let actions=match event.type_().as_str() {
                    "pointerdown"=>{gesture.cancel();let multiple=touch.count()>0;if touch.press(id,at,event.time_stamp()) {if canvas.set_pointer_capture(id).is_err() {touch.cancel();}}if multiple {vec![Action::GizmoCancel,Action::CanvasCancel]} else {Vec::new()}},
                    "pointermove"=>touch.move_to(id,at,rect),
                    "pointerup"=>{let actions=touch.release(id,at,event.time_stamp(),rect);let _=canvas.release_pointer_capture(id);actions},
                    "pointercancel"=>{touch.cancel();Vec::new()},"lostpointercapture"=>{if touch.contains(id) {touch.cancel();}Vec::new()},_=>Vec::new(),
                };return (!actions.is_empty()).then_some(Action::Touch(actions));
            }
            return pointer_action(event,canvas,gesture);
        },
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press_at(id, pointer.button(), position,pointer.ctrl_key()||pointer.meta_key(),pointer.time_stamp()) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_at(id, position,pointer.time_stamp()),
        "pointerup" => {
            let motion = gesture.release_at(id, position,pointer.time_stamp());
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    let mut action=motion.action([rect.left(),rect.top(),rect.width(),rect.height()])?;
    match &mut action {
        Action::CanvasStart {modified,pointer:id,..}=>{*id=pointer.pointer_id();*modified=pointer.shift_key()||pointer.ctrl_key()||pointer.meta_key();},
        Action::CanvasRelease {add,remove,..}=>{*add=pointer.shift_key();*remove=pointer.ctrl_key()||pointer.meta_key();},_=>{}
    }Some(action)
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let density = crate::browser_recovery::density(crate::browser_defaults::scale(window.device_pixel_ratio()));
    renderer.set_density(density as f32);
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        density,
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    normals: bool,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw_mode(&view, background, transform, normals);
    if let Some(error)=renderer.cloud_error() {panel.result(&format!("Cloud: {error}"));}
    if let Some(error)=renderer.text_error() {panel.result(&format!("Text: {error}"));}
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}

fn gizmo_action(event:&web_sys::Event,canvas:&web_sys::HtmlCanvasElement,editor:&Editor,consumed:bool)->Option<Action> {
    if matches!(event.type_().as_str(),"blur"|"resize") && editor.gizmo_edit.is_some() {return Some(Action::GizmoCancel);}
    let pointer=event.dyn_ref::<web_sys::PointerEvent>()?;
    if matches!(event.type_().as_str(),"pointercancel"|"lostpointercapture") {
        return editor.gizmo_edit.as_ref().filter(|e|e.pointer==Some(pointer.pointer_id())).map(|_|Action::GizmoCancel);
    }
    let rect=canvas.get_bounding_client_rect();let size=[rect.width(),rect.height()];
    if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
    let screen=[(2.*(pointer.client_x() as f64-rect.left())/size[0]-1.) as f32,(1.-2.*(pointer.client_y() as f64-rect.top())/size[1]) as f32];
    let id=pointer.pointer_id();
    if editor.gizmo_edit.as_ref().is_some_and(|e|e.pointer==Some(id)) {
        event.prevent_default();
        return match event.type_().as_str() {
            "pointermove"=>Some(Action::GizmoMove {pointer:id,screen,size,shift:pointer.shift_key()}),
            "pointerup"=>{let _=canvas.release_pointer_capture(id);Some(Action::GizmoRelease {pointer:id,screen,size,shift:pointer.shift_key()})},
            _=>None,
        };
    }
    if consumed || event.type_()!="pointerdown" || !pointer.is_primary() || pointer.button()!=0 {return None;}
    if pointer.shift_key() || pointer.ctrl_key() || pointer.meta_key() {return None;}
    let finger=pointer.pointer_type()=="touch";
    if editor.gizmo_hit_radius(screen,size,if finger {18.} else {8.}).is_some() {
        event.prevent_default();
        if canvas.set_pointer_capture(id).is_err() {return Some(Action::GizmoCancel);}
        Some(if finger {Action::GizmoTouch {pointer:id,screen,size}} else {Action::GizmoPress {pointer:id,screen,size,modified:false}})
    } else if editor.gizmo_edit.is_some() {Some(Action::GizmoCancel)} else {None}
}

fn lasso_action(event:&web_sys::Event,canvas:&web_sys::HtmlCanvasElement,editor:&Editor,consumed:bool)->Option<Action>{
    let lasso=editor.lasso.as_ref()?;if matches!(event.type_().as_str(),"resize"|"blur"){return Some(Action::LassoCancel)}
    let pointer=event.dyn_ref::<web_sys::PointerEvent>()?;let id=pointer.pointer_id();let r=canvas.get_bounding_client_rect();let at=[pointer.client_x() as f64-r.left(),pointer.client_y() as f64-r.top()];
    if lasso.pointer==Some(id){event.prevent_default();return match event.type_().as_str(){"pointermove"=>Some(Action::LassoMove{at,pointer:id}),"pointerup"=>{let _=canvas.release_pointer_capture(id);Some(Action::LassoRelease{at,pointer:id,add:pointer.shift_key(),remove:pointer.ctrl_key()||pointer.meta_key()})},"pointercancel"|"lostpointercapture"=>Some(Action::LassoCancel),_=>None}}
    if consumed||event.type_()!="pointerdown"||pointer.button()!=0||!pointer.is_primary(){return None}event.prevent_default();if canvas.set_pointer_capture(id).is_err(){return Some(Action::LassoCancel)}Some(Action::LassoPress{at,size:[r.width(),r.height()],density:canvas.width() as f64/r.width(),pointer:id})
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes-06.rs"
```

```text
--8<-- "journey/code/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes-07.rs"
```

```text
--8<-- "journey/code/96zzzzzr-prepare-browser-frame-matrices-and-owned-wakes-08.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzzs-adopt-the-shared-origin-in-the-browser"></a>

<details>
<summary>Implementation 96zzzzzs-adopt-the-shared-origin-in-the-browser: Adopt the shared origin in the browser</summary>

Keep browser drawing and asynchronous interaction in one precise local frame.

Data flow: Browser action → shared origin → original and temporary inputs → draw/pick; pending → owned wake.

### 1. `src/browser_rebase.rs`

Connect the existing origin policy and timer owner without retaining an editor or scene in a wake.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzs-adopt-the-shared-origin-in-the-browser-01.rs"
```

### 2. `src/lib.rs`

Compile the browser-only frame and wake adapter.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
mod timer;
```

</details>

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_rebase;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzs-adopt-the-shared-origin-in-the-browser-02.rs"
```

</details>

### 3. `src/browser.rs`

Prepare actual origin-relative inputs before the initial frame and its diagnostics. Keep the wake owner inside the runtime-owned input closure. Use the current origin before judging or queueing asynchronous picks. Adopt action changes before temporary inputs, avoid duplicate selection uploads, and schedule only a pending origin move. Deliver the owned wake through the existing runtime listener owner. Expose the actual precise camera target used by the origin policy for controlled browser frame checks.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    canvas.set_attribute("data-normal-view", if editor.normal_view { "true" } else { "false" })?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-render-origin",&serde_json::to_string(&renderer.anchor()).unwrap())?;
    canvas.set_attribute("data-render-matrix",&serde_json::to_string(&renderer.camera_matrix(&editor.camera)).unwrap())?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
```

</details>

Replace it with:

```rust
    canvas.set_attribute("data-normal-view", if editor.normal_view { "true" } else { "false" })?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-render-origin",&serde_json::to_string(&renderer.anchor()).unwrap())?;
    canvas.set_attribute("data-camera-target",&serde_json::to_string(&editor.camera.target).unwrap())?;
    canvas.set_attribute("data-render-matrix",&serde_json::to_string(&renderer.camera_matrix(&editor.camera)).unwrap())?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
```

<details>
<summary>Locate this block</summary>

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
```

</details>

Replace it with:

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(),"viewer-origin")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
```

<details>
<summary>Locate this block</summary>

```rust
        if let Some(reply)=text_fonts.reply.borrow_mut().take() {match reply {Ok(fonts)=>{if let Err(error)=renderer.replace_text_fonts(fonts) {text_fonts.failed();panel.result(&format!("Text: {error}"));}},Err(error)=>panel.result(&format!("Text: {error}"))}}
        sheet_query.observe(&editor.scene,editor.selection(),&active_fault);
        if let Some(reply)=sheet_query.take() {match reply {Ok(entity)=>panel.result(&format!("{} · {} · {}",entity.name,entity.kind,entity.guid)),Err(error)=>panel.result(&error)}}
        let guides=editor.guide_overlay_at(renderer.anchor());
        if let Err(error)=renderer.set_guides(&guides){panel.result(error);}
        if let Err(error)=renderer.set_measurement(editor.mark.as_ref()){panel.result(error);}
```

</details>

Replace it with:

```rust
        if let Some(reply)=text_fonts.reply.borrow_mut().take() {match reply {Ok(fonts)=>{if let Err(error)=renderer.replace_text_fonts(fonts) {text_fonts.failed();panel.result(&format!("Text: {error}"));}},Err(error)=>panel.result(&format!("Text: {error}"))}}
        sheet_query.observe(&editor.scene,editor.selection(),&active_fault);
        if let Some(reply)=sheet_query.take() {match reply {Ok(entity)=>panel.result(&format!("{} · {} · {}",entity.name,entity.kind,entity.guid)),Err(error)=>panel.result(&error)}}
        let origin_frame=match crate::browser_rebase::prepare(&mut renderer,&editor){Ok(frame)=>frame,Err(error)=>{report(&format!("Cannot prepare view: {error:?}"));return;}};
        if origin_frame.moved{highlight_changed=false;}
        if let Err(error)=origin_wake.sync(origin_frame.pending,renderer.rebase_delay(crate::browser_phase::now()).unwrap_or(1)){report(&format!("Cannot schedule view: {error:?}"));return;}
        let _=pointer_canvas.set_attribute("data-origin-pending",if origin_wake.pending(){"true"}else{"false"});
        let guides=editor.guide_overlay_at(renderer.anchor());
        if let Err(error)=renderer.set_guides(&guides){panel.result(error);}
        if let Err(error)=renderer.set_measurement(editor.mark.as_ref()){panel.result(error);}
```

<details>
<summary>Locate this block</summary>

```rust
            event.prevent_default(); gesture.cancel(); Some("Escape".into())
        } else { line };

        let mut highlight_changed = false;
        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
```

</details>

Replace it with:

```rust
            event.prevent_default(); gesture.cancel(); Some("Escape".into())
        } else { line };

        if let Err(error)=crate::browser_rebase::prepare(&mut renderer,&editor){report(&format!("Cannot prepare view: {error:?}"));return;}
        let mut highlight_changed=false;
        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
```

<details>
<summary>Locate this block</summary>

```rust
    drop(editor);
    let sheet_query=crate::browser_sheet::Control::default();
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        let mut editor = shared.borrow_mut();
```

</details>

Replace it with:

```rust
    drop(editor);
    let sheet_query=crate::browser_sheet::Control::default();
    let owned_editor = std::rc::Rc::clone(&shared);
    let mut origin_wake=crate::browser_rebase::Wake::new(fault.clone());
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        let mut editor = shared.borrow_mut();
```

<details>
<summary>Locate this block</summary>

```rust
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        inspect(&editor,&renderer)?;
        let frame_started = crate::browser_phase::now();
        present(
```

</details>

Replace it with:

```rust
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None,&canvas)?;
        crate::browser_rebase::prepare(&mut renderer,&editor)?;
        canvas.set_attribute("data-origin-pending","false")?;
        inspect(&editor,&renderer)?;
        let frame_started = crate::browser_phase::now();
        present(
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) {
            crate::browser_report::fatal(&message);
            if crate::browser_recovery::request(&message) { return; }
            report(&format!("Cannot draw: {message}"));
        }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> {
    let shared=crate::drawing_document::new();
    crate::browser_defaults::initialize(&mut shared.borrow_mut());
    run_with(shared).await
}

pub async fn run_with(shared: crate::drawing_document::Shared) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let recovery = crate::browser_recovery::adopt()?;
    crate::browser_report::start()?;
    let startup = crate::browser_startup::Guard::new()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter_started = crate::browser_phase::now();
    let probe = crate::browser_adapter::Probe::new().ok();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await;
    if !startup.permits() { return Ok(()); }
    crate::browser_phase::finish("adapter request", adapter_started, 0, "GPU");
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) { let _ = crate::browser_report::adapter(info); }
    else { let _ = crate::browser_report::observe("diagnostic", "Adapter identity unavailable"); }
    drop(probe);
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
    if !startup.permits() {
        if let Ok((device, _)) = device { device.destroy(); }
        return Ok(());
    }
    crate::browser_phase::finish("device request", device_started, 0, "GPU");
    let (device, queue) = device.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| failed(errors.clone(), format!("WebGPU error: {error}"))));
    let lost = fault.clone();
    device.set_device_lost_callback(move |reason, message| failed(lost.clone(), format!("WebGPU device lost ({reason:?}): {message}")));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = shared.borrow_mut();
    let pipeline_started = crate::browser_phase::now();
    let text_fonts=std::rc::Rc::new(crate::text_fonts::Inbox::default());
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    crate::browser_phase::finish("renderer setup", pipeline_started, 0, "GPU");
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        crate::commands::names(),
    );
    if let Some(message) = crate::browser_report::previous_notice() { panel.result(message); }
    if let Some(message) = &recovery { panel.result(message); }
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None,&canvas)?;
        crate::browser_rebase::prepare(&mut renderer,&editor)?;
        canvas.set_attribute("data-origin-pending","false")?;
        inspect(&editor,&renderer)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &renderer.camera_matrix(&editor.camera),
            editor.normal_view,
            &mut panel,
        )?;
        let completing = fault.clone();
        // Queue completion runs after submitted GPU work; checking the current device prevents a late result from reviving an exited runtime.
        renderer.queue.on_submitted_work_done(move || {
            if crate::browser_runtime::owns(&completing) && completing.message().is_none() {
                crate::browser_phase::finish("first frame complete", frame_started, 0, "GPU");
                let _ = crate::browser_report::observe("milestone", "geometry on screen");
            }
        });
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let mut touch=crate::touch::Touch::default();
    let pick_remove=std::cell::Cell::new(false);
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let publication=crate::browser_publication::Publication::new(std::rc::Rc::clone(&shared),fault.clone())?;
    let publication_control=publication.control();let start_publication=publication_control.clone();
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let sheet_query=crate::browser_sheet::Control::default();
    let owned_editor = std::rc::Rc::clone(&shared);
    let mut origin_wake=crate::browser_rebase::Wake::new(fault.clone());
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        let mut editor = shared.borrow_mut();
        if matches!(event.type_().as_str(), "blur" | "pagehide" | "viewer-reload") { renderer.cancel_pick(); }
        if event.type_() == "pagehide" {
            gesture.cancel();touch.cancel();let _=editor.apply(Action::CanvasCancel); let _=editor.apply(Action::GizmoCancel);
            crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());renderer.set_gizmo(&editor.camera,None,-1.);
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
        if matches!(event.type_().as_str(),"blur"|"resize"|"pointercancel")||event.dyn_ref::<web_sys::KeyboardEvent>().is_some_and(|key|key.key()=="Escape"&&!key.is_composing()) {
            gesture.cancel();touch.cancel();renderer.cancel_pick();if editor.apply(Action::CanvasCancel)==Ok(Change::Preview) {crate::browser_upload::prepare(&mut renderer,&editor.scene,editor.whole_selection(),active_fault.clone());}
        }
        if event.type_()=="lostpointercapture"&&event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|editor.canvas_drag.as_ref().is_some_and(|d|d.pointer==p.pointer_id()&&d.released.is_none())) {
            gesture.cancel();renderer.cancel_pick();if editor.apply(Action::CanvasCancel)==Ok(Change::Preview) {crate::browser_upload::prepare(&mut renderer,&editor.scene,editor.whole_selection(),active_fault.clone());}
        }
        if event.type_()=="pointerdown"&&event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.is_primary())&&editor.canvas_drag.as_ref().is_none_or(|d|d.released.is_some()) {renderer.cancel_pick();let _=editor.apply(Action::CanvasCancel);}
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let navigation=if line.is_none()&&!panel.consumed {navigation_action(&event,&pointer_canvas,&mut gesture,&mut touch)} else {None};
        let line=if gesture.take_repeat() {event.prevent_default();if editor.draft.is_some()||editor.gizmo_edit.is_some() {Some(String::new())} else {panel.repeat_line()}} else {line};
        let line = if (editor.lasso.is_some() || editor.trim.is_some() || editor.mark.is_some() || editor.split.is_some() || editor.draft.is_some() || editor.gizmo_edit.is_some()) && event.dyn_ref::<web_sys::KeyboardEvent>()
            .is_some_and(|key| event.type_() == "keydown" && key.key() == "Escape" && !key.is_composing()) {
            event.prevent_default(); gesture.cancel(); Some("Escape".into())
        } else { line };

        if let Err(error)=crate::browser_rebase::prepare(&mut renderer,&editor){report(&format!("Cannot prepare view: {error:?}"));return;}
        let mut highlight_changed=false;
        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.as_ref().map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        highlight_changed = true;
                        crate::browser_upload::prepare(&mut renderer, editor.display_scene(), editor.whole_selection(), active_fault.clone());
                        crate::browser_phase::revision(&editor.scene);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
        } else if event.type_() == "viewer-stream-page" {
            renderer.cancel_pick();Some(Action::StreamRefresh)
        } else if event.type_() == "viewer-publication" {
            match publication_control.take() {Ok(action)=>action,Err(error)=>{let message=format!("Cannot load scene: {error}");report(&message);panel.result(&message);None}}
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery, &editor.scene) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot open file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            let command = editor.tool_line(&line).map(|result| result.map(crate::commands::Command::Editor).map_err(str::to_owned))
                .unwrap_or_else(|| crate::commands::parse(&line));
            editor.mark=None;
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { crate::stream::reset_range_gate();publication_control.cancel();request.borrow_mut().cancel();let _={text_fonts.clear();renderer.clear_text_fonts();renderer.set_text(Vec::new())}; }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
            }
            Ok(crate::commands::Command::Host(host)) => {
            use crate::commands::Host;
            match host {
            Host::TextDemo=>{if let Err(error)=renderer.text_demo() {panel.result(&error.to_string());}None},
            Host::Help => { panel.result(&crate::commands::names().join(" · ")); None }
            Host::Selection => {
                let rows = editor.selected_objects();
                panel.result(&if rows.len() > 1 { format!("Objects {:?}", rows.iter().map(|id| id.value()).collect::<Vec<_>>()) }
                    else { editor.selection().map_or_else(|| "Nothing selected".to_owned(), |target| target.describe()) });
                None
            }
            Host::Reload => {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            }
            Host::CancelReload => {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            }
            Host::CancelOpen => {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            }
            Host::Open | Host::Replace => {
                publication_control.cancel();
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if host == Host::Replace { reload.borrow_mut().cancel(); }
                read_mode.set(if host == Host::Replace { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            }
            Host::Report | Host::PreviousReport => {
                let output = if host == Host::PreviousReport { crate::browser_report::download_previous() }
                    else { crate::browser_report::download() };
                let result = output.map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
            }
            Host::LayersOn=>{panel.tree.open=true;None},
            Host::LayersOff=>{panel.tree.open=false;None},
            Host::LayersAll=>{panel.tree.all();None},
            Host::Save => {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::scene_archive::save(&editor.scene)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            }
            }
            }
            }
        } else if !panel.consumed {
            navigation
        } else {
            None
        };
        let handle_action=lasso_action(&event,&pointer_canvas,&editor,panel.consumed).or_else(||gizmo_action(&event,&pointer_canvas,&editor,panel.consumed));
        let action = if let Some((key,edit))=panel.tree.edit.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::LayerEdit {key,edit})
        } else if let Some((key,property))=panel.tree.property.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::LayerProperty {key,property})
        } else if let Some((ids,additive))=panel.tree.action.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::TreePick {ids,additive})
        } else if let Some(handle_action)=handle_action {
            renderer.cancel_pick();gesture.cancel();Some(handle_action)
        } else if event.type_() == "viewer-pick" {
            match renderer.take_pick(renderer.camera_matrix(&editor.camera)) {
                Some(Ok(hit)) => {
                    let target=hit.and_then(|hit|renderer.resolve_pick(&editor.scene,hit));
                    if editor.element.is_some(){Some(Action::ElementPick(target.map(|t|t.owner)))}else if editor.trimming_picks(){Some(Action::TrimPick(target.map(|t|t.owner)))}else if editor.split.is_some(){Some(Action::SplitPick(target.map(|t|t.owner)))}else if editor.editing_boundaries(){Some(Action::ExtendPick(target.map(|t|t.owner)))}else if editor.gathering_curves(){Some(Action::GatherPick(target.map(|t|t.owner)))}
                    else if editor.canvas_drag.is_some() {Some(Action::CanvasHit(target.map(|t|t.owner)))}
                    else if pick_remove.get() {Some(Action::SelectRows {ids:target.filter(|t|t.part==crate::pick_sources::Part::Object).map(|t|t.owner).into_iter().collect(),add:false,remove:true})}
                    else {Some(Action::PickedMany {target,additive:pick_additive.get()})}
                },
                Some(Err(error)) => { report(error); None },
                None => None,
            }
        } else { match action {
            Some(Action::Pick(screen)) if editor.trimming_parts()=>{let r=pointer_canvas.get_bounding_client_rect();Some(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true})},
            Some(Action::Pick(screen)) if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves()=>{pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)){report(error)}None},
            Some(Action::Pick(screen)) if editor.draft.is_some() => {
                Some(Action::ToolScreen { screen,size:[pointer_canvas.get_bounding_client_rect().width(),pointer_canvas.get_bounding_client_rect().height()],commit:true })
            }
            None if editor.trimming_parts()&&event.type_()=="pointermove"=>{let r=pointer_canvas.get_bounding_client_rect();event.dyn_ref::<web_sys::PointerEvent>().and_then(|e|(r.width()>0.&&r.height()>0.).then(||[(2.*(e.client_x()as f64-r.left())/r.width()-1.)as f32,(1.-2.*(e.client_y()as f64-r.top())/r.height())as f32])).map(|screen|Action::TrimScreen{screen,size:[r.width(),r.height()],commit:false})},
            None if editor.draft.is_some() && matches!(event.type_().as_str(), "blur" | "resize" | "pointercancel" | "lostpointercapture") => Some(Action::ToolHover(None)),
            None if editor.draft.is_some() && !panel.consumed && event.type_() == "pointermove" => {
                event.dyn_ref::<web_sys::PointerEvent>().filter(|p| p.buttons() == 0).map(|p| {
                    let r = pointer_canvas.get_bounding_client_rect();
                    let screen = [(2. * (p.client_x() as f64-r.left())/r.width()-1.) as f32, (1.-2.*(p.client_y() as f64-r.top())/r.height()) as f32];
                    Action::ToolScreen { screen,size:[r.width(),r.height()],commit:false }
                })
            }
            Some(Action::Touch(actions))=> {
                let mut applied=Vec::new();
                for action in actions {match action {
                    Action::Pick(screen)=>{if editor.trimming_parts(){let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::TrimScreen{screen,size:[r.width(),r.height()],commit:true});}else if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)){report(error)}}else if editor.draft.is_some() {let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::ToolScreen {screen,size:[r.width(),r.height()],commit:true});} else {pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,renderer.camera_matrix(&editor.camera)) {report(error);}}},
                    action=>{renderer.cancel_pick();applied.push(action);}
                }}Some(Action::Touch(applied))
            },
            Some(Action::CanvasStart {from,to,size,modified,pointer}) if editor.draft.is_none()=> {
                renderer.cancel_pick();
                let modified=modified||editor.pick_mode!=crate::pick_sources::Mode::Object;
                let start=Action::CanvasStart {from,to,size,modified,pointer};
                let release=event.dyn_ref::<web_sys::PointerEvent>().filter(|_|event.type_()=="pointerup").map(|pointer|Action::CanvasRelease {screen:to,size,add:pointer.shift_key(),remove:pointer.ctrl_key()||pointer.meta_key()});
                if !modified {if let Err(error)=renderer.request_pick(from,renderer.camera_matrix(&editor.camera)) {report(error);return;}}
                Some(Action::Touch(std::iter::once(start).chain(release).collect()))
            },
            Some(Action::CanvasStart {..}|Action::CanvasMove {..}|Action::CanvasRelease {..}) if editor.draft.is_some()=>None,
            Some(action @ (Action::CanvasMove {..}|Action::CanvasRelease {..}))=>Some(action),
            Some(Action::Pick(screen)) => {
                pick_remove.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.ctrl_key()||p.meta_key()));
                pick_additive.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p| p.shift_key() && !p.ctrl_key() && !p.meta_key()));
                if let Err(error) = renderer.request_pick(screen, renderer.camera_matrix(&editor.camera)) { report(error); }
                None
            }
            Some(action) => { renderer.cancel_pick(); Some(action) },
            None => None,
        } };
        if let Some(action) = action {
            highlight_changed |= matches!(&action, Action::SelectAll);
            let group_action=matches!(&action,Action::AddGroup(_));let graph_action=matches!(&action,Action::AddEdge);
            let rename_action=matches!(&action,Action::LayerEdit {edit:crate::layer_edit::Edit::Rename(_),..});
            let tool_action = matches!(&action, Action::ElementBegin{..}|Action::ElementText(_)|Action::ElementPick(_)|Action::ElementAttributes(_)| Action::AddGroup(_) | Action::AddEdge | Action::SelectLasso | Action::LassoRelease{..} | Action::LassoCancel | Action::TextBegin(_) | Action::Trim | Action::TrimPick(_) | Action::TrimScreen{commit:true,..} | Action::CancelTrim | Action::Arrowhead(_) | Action::Explode | Action::Query(_) | Action::Measure(_) | Action::MeasureDistance(_) | Action::BoundingBox | Action::Split | Action::SplitPick(_) | Action::CancelSplit | Action::BeginTool(_) | Action::GatherBegin{..} | Action::GatherPick(_) | Action::Project(_) | Action::ExtendBegin{..} | Action::ExtendPick(_) | Action::Interval(_) | Action::ToolPoint(_) | Action::CancelTool | Action::Snap(_) | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::ToolScreen { commit:true,.. });
            if matches!(&action, Action::CancelTool | Action::CancelSplit | Action::CancelTrim) { panel.cancel_input(); }
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Live(_) | Action::Undo | Action::Redo
                | Action::Adopt {..} | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            if closing_action(&action) {publication_control.cancel();}
            let replacement = matches!(&action, Action::Adopt {replace:true,..} | Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = crate::edit_intent::Intent::capture_for(&action,&editor)
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            if change.is_ok()&&(group_action||graph_action){panel.tree.open=true;if graph_action{panel.tree.graph_open=true;}}
            if rename_action {panel.tree.rename_result(change.as_ref().err().map(String::as_str));}
            if tool_action && change.is_ok() {highlight_changed=true;panel.result(&editor.tool_status); }
            match change {
                Ok(Some(Change::Scene)) => {
                    highlight_changed = true;
                    crate::browser_upload::prepare(&mut renderer, &editor.scene, editor.whole_selection(), active_fault.clone());
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-publication" {
                        crate::browser_phase::revision(&editor.scene);let _=crate::browser_report::observe("live-reload","source bytes changed");report("URL scene replaced.");panel.result("URL scene replaced.");
                    } else if event.type_() == "viewer-file" {
                        crate::browser_phase::revision(&editor.scene);
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::Preview)) => {
                    highlight_changed=true;
                    crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    let error=if error=="Shape input invalid" {editor.tool_status.as_str()}else{error.as_str()};
                    report(error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        if let Some(element)=&editor.element{panel.tool(element.prompt(),element.buttons());}
        else if editor.plane_menu {panel.tool(editor.tool_status.clone(),crate::drawing_input::buttons());}
        else if let Some(trim)=&editor.trim{panel.tool(trim.prompt(),vec![("Enter".into(),"Trim".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(split)=&editor.split{panel.tool(split.prompt(),vec![("Split".into(),"Split".into()),("Cancel".into(),"Escape".into())]);}
        else if let Some(draft) = editor.draft.as_ref() {let mut buttons=draft.tool.buttons();if draft.text.is_some(){buttons.push(("Height".into(),"Height".into()));}buttons.extend(crate::drawing_input::buttons());panel.tool(draft.prompt(),buttons);}
        else { panel.tool(String::new(), Vec::new()); }
        if let Some(reply)=text_fonts.reply.borrow_mut().take() {match reply {Ok(fonts)=>{if let Err(error)=renderer.replace_text_fonts(fonts) {text_fonts.failed();panel.result(&format!("Text: {error}"));}},Err(error)=>panel.result(&format!("Text: {error}"))}}
        sheet_query.observe(&editor.scene,editor.selection(),&active_fault);
        if let Some(reply)=sheet_query.take() {match reply {Ok(entity)=>panel.result(&format!("{} · {} · {}",entity.name,entity.kind,entity.guid)),Err(error)=>panel.result(&error)}}
        let origin_frame=match crate::browser_rebase::prepare(&mut renderer,&editor){Ok(frame)=>frame,Err(error)=>{report(&format!("Cannot prepare view: {error:?}"));return;}};
        if origin_frame.moved{highlight_changed=false;}
        if let Err(error)=origin_wake.sync(origin_frame.pending,renderer.rebase_delay(crate::browser_phase::now()).unwrap_or(1)){report(&format!("Cannot schedule view: {error:?}"));return;}
        let _=pointer_canvas.set_attribute("data-origin-pending",if origin_wake.pending(){"true"}else{"false"});
        let guides=editor.guide_overlay_at(renderer.anchor());
        if let Err(error)=renderer.set_guides(&guides){panel.result(error);}
        if let Err(error)=renderer.set_measurement(editor.mark.as_ref()){panel.result(error);}
        renderer.refresh_text_bounds(&mut editor.scene);
        text_fonts.need(editor.scene.texts().iter().filter(|row|editor.scene.visible(row.id)).map(|row|row.prepared.source.label.text.as_str()));
        let rect=pointer_canvas.get_bounding_client_rect();
        let number=editor.number_prompt([rect.width(),rect.height()]);
        if editor.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some()) && number.is_none() {
            if editor.apply(Action::GizmoCancel).is_ok() {highlight_changed=true;crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());}
        }
        panel.rectangle=editor.canvas_drag.as_ref().filter(|d|d.missed).and_then(|d|d.rectangle());
        panel.number(number);
        panel.lasso=editor.lasso.as_ref().map_or_else(Vec::new,|l|l.points.iter().map(|p|[(p.0/l.density) as f32,(p.1/l.density) as f32]).collect());
        panel.tree.refresh_cached(&editor.scene,editor.selected_objects(),editor.interaction_epoch);
        if let Err(error) = renderer.set_preview(editor.draft.as_ref()) { panel.result(error); }
        renderer.set_pick_mode(if editor.element.is_some()||editor.trimming_picks()||editor.split.is_some()||editor.editing_boundaries()||editor.gathering_curves(){crate::pick_sources::Mode::Object}else{editor.pick_mode});
        if highlight_changed {
            if let Err(error) = renderer.set_highlight(editor.display_scene(), editor.selection())
                .and_then(|()| renderer.select_objects(editor.display_scene(), &editor.highlighted_objects())) { report(error); return; }
        }
        renderer.set_clipping(editor.clipping_enabled,editor.clipping_solid);
        renderer.set_opacity(editor.opacity);renderer.set_ambient(editor.arctic);renderer.set_outlines(editor.outlines);
        if let Err(error) = renderer.set_grid_on(editor.camera.grid,editor.drawing_plane()) {
            report(error); return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Some(frame)=panel.free_frame([rect.width(),rect.height()]) {if editor.camera.frame(frame) {renderer.cancel_pick();let _=editor.apply(Action::CanvasCancel);}}
            let hover=event.dyn_ref::<web_sys::PointerEvent>().filter(|p|event.type_()=="pointermove" && p.buttons()==0 && !panel.consumed).map(|p| {
                let r=pointer_canvas.get_bounding_client_rect();
                ([(2.*(p.client_x() as f64-r.left())/r.width()-1.) as f32,(1.-2.*(p.client_y() as f64-r.top())/r.height()) as f32],[r.width(),r.height()])
            });
            editor.hover_gizmo(hover);
            renderer.set_gizmo(&editor.camera,editor.gizmo_origin(),editor.gizmo_active());
            renderer.cloud_camera(&editor.camera);
            if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &renderer.camera_matrix(&editor.camera),
                editor.normal_view,
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    let mut listeners = crate::listeners::Listeners::new(update);
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        listeners.listen(canvas.as_ref(), name)?;
    }
    listeners.listen(canvas.as_ref(), "wheel")?;
    listeners.listen(window.as_ref(), "resize")?;
    listeners.listen(window.as_ref(), "blur")?;
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(),"viewer-origin")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
    listeners.listen(window.as_ref(), "viewer-stream-page")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault, owned_editor, publication);
    report(&recovery.unwrap_or_else(|| "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.".into()));
    start_publication.begin();
    Ok(())
}

fn closing_action(action:&Action)->bool {matches!(action,Action::Close|Action::Adopt {..}|Action::Replace(_)|Action::ReplaceAt(_, _))}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    let view_ids=editor.scene.objects().iter().map(|row|row.id).chain(editor.scene.lines().iter().map(|row|row.id)).chain(editor.scene.paths().iter().map(|row|row.id)).chain(editor.scene.points().iter().map(|row|row.id)).chain(editor.scene.texts().iter().map(|row|row.id)).chain(editor.scene.clouds().iter().map(|row|row.id));
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    let local=editor.scene.forest.documents.iter().flat_map(|doc|doc.nodes.iter().map(move |node|serde_json::json!({"document":doc.key,"name":node.source.name,"object":node.object.map(|id|id.value()),"local":node.placement.local.m,"original_world":node.placement.world.m}))).collect::<Vec<_>>();
    canvas.set_attribute("data-original-tree-placements",&serde_json::to_string(&local).unwrap())?;
    canvas.set_attribute("data-camera-frame",&serde_json::to_string(&editor.camera.frame).unwrap())?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-source-metadata",&serde_json::to_string(&crate::source_metadata::usage(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-groups",&serde_json::json!(editor.scene.groups.iter().collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-graph",&serde_json::json!(crate::graph_edit::rows(&editor.scene)).to_string())?;
    canvas.set_attribute("data-lasso",&editor.lasso.as_ref().map_or_else(||"null".into(),|l|serde_json::json!({"points":l.points.len(),"spacing":l.spacing,"density":l.density}).to_string()))?;
    canvas.set_attribute("data-tree",&serde_json::json!(crate::hierarchy::rows(&editor.scene).iter().map(|r|serde_json::json!({"key":r.key,"label":r.label,"depth":r.depth,"end":r.end,"readonly":r.readonly,"ids":r.ids.iter().map(|id|id.value()).collect::<Vec<_>>()})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute("data-triangle-tiles",&serde_json::to_string(&renderer.tile_usage()).unwrap())?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-cloud-cpu",&serde_json::to_string(&crate::memory::clouds(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-cloud-gpu",&serde_json::to_string(&renderer.cloud_usage()).unwrap())?;
    canvas.set_attribute("data-cloud-error",&renderer.cloud_error().unwrap_or_default())?;
    canvas.set_attribute("data-cloud-sources",&serde_json::json!(editor.scene.clouds().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid(),"point_size":row.prepared.source.point_size,"point_px":row.point_px,"points":row.prepared.source.len(),"nodes":row.prepared.nodes.len(),"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-surface-outline",&serde_json::to_string(&renderer.outline_usage()).unwrap())?;
    canvas.set_attribute("data-scene-text-stats",&serde_json::to_string(&renderer.text_stats()).unwrap())?;
    canvas.set_attribute("data-scene-text-cpu",&serde_json::to_string(&crate::memory::texts(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-scene-text-gpu",&serde_json::to_string(&renderer.text_scene_usage()).unwrap())?;
    canvas.set_attribute("data-scene-texts",&serde_json::json!(editor.scene.texts().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source":row.prepared.source,"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-text-error",&renderer.text_error().unwrap_or_default())?;
    canvas.set_attribute("data-text-atlas",&serde_json::to_string(&renderer.text_usage()).unwrap())?;
    canvas.set_attribute("data-cad-walks",&serde_json::json!({"scope":"last imported original document","walks":editor.scene.cad_walks[0],"replays":editor.scene.cad_walks[1]}).to_string())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
    let cads=editor.scene.objects().iter().filter_map(|row|row.cad.as_ref().map(|cad|serde_json::json!({"id":row.id.value(),"source":cad.source.guid(),"faces":row.mesh.parts.faces,"edges":cad.edges.iter().map(|border|border.edge).collect::<Vec<_>>(),"vertices":cad.samples.len()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-cads",&serde_json::to_string(&cads).unwrap())?;
    canvas.set_attribute("data-surface-cpu",&serde_json::to_string(&crate::memory::surfaces(editor.presentation_scenes())).unwrap())?;
    let surfaces=editor.scene.objects().iter().filter_map(|row|row.surface.as_ref().map(|s|serde_json::json!({"id":row.id.value(),"source":s.source.guid(),"controls":s.source.m_cv_count,"steps":s.grid.steps,"samples":s.grid.samples.len(),"faces":row.mesh.parts.faces,"edges":s.grid.boundaries.iter().map(|b|b.edge).collect::<Vec<_>>()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-surfaces",&serde_json::to_string(&surfaces).unwrap())?;
    canvas.set_attribute("data-curve-cpu", &serde_json::to_string(&crate::memory::curves(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-control-count", &editor.scene.control_markers().len().to_string())?;
    canvas.set_attribute("data-point-cpu", &serde_json::to_string(&crate::memory::points(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-path-gpu", &serde_json::to_string(&renderer.path_usage()).unwrap())?;
    canvas.set_attribute("data-path-cpu", &serde_json::to_string(&crate::memory::paths(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-stroke-usage", &serde_json::to_string(&renderer.stroke_usage()).unwrap())?;
    canvas.set_attribute("data-line-usage", &serde_json::to_string(&crate::memory::lines(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-edit-preview", &editor.edit_preview.as_ref().map_or_else(|| "null".into(),|p| serde_json::json!({"ids":p.ids().iter().map(|id| id.value()).collect::<Vec<_>>(),"matrix":p.matrix()}).to_string()))?;
    let originals=serde_json::json!({"points":editor.scene.points().iter().map(|row|(0..3).map(|i|row.prepared.source[i]).collect::<Vec<_>>()).collect::<Vec<_>>(),"lines":editor.scene.lines().iter().map(|row|{let s=&row.prepared.source;[s.start(),s.end()].map(|p|[p[0],p[1],p[2]])}).collect::<Vec<_>>(),"polylines":editor.scene.paths().iter().filter(|row|matches!(row.prepared.source,crate::chain::Source::Polyline(_))).map(|row|row.prepared.source.coordinates()).collect::<Vec<_>>()});
    canvas.set_attribute("data-curve-originals",&serde_json::to_string(&editor.scene.paths().iter().filter_map(|row|match &row.prepared.source {crate::chain::Source::Curve(source,_)=>Some(serde_json::json!({"guid":source.guid(),"controls":source.m_cv,"knots":source.m_nurbsknot,"rational":source.m_is_rat,"count":source.cv_count()})),_=>None}).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-drawing-originals",&originals.to_string())?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute("data-measurement",&editor.mark.as_ref().map_or_else(||"null".into(),|m|serde_json::json!({"points":m.points,"label":m.label}).to_string()))?;
    canvas.set_attribute("data-measurement-gpu-bytes",&renderer.measurement_bytes().to_string())?;
    canvas.set_attribute("data-guide-gpu-bytes",&renderer.guide_bytes().to_string())?;
    canvas.set_attribute("data-trim",&editor.trim.as_ref().map_or_else(||"null".into(),|t|serde_json::json!({"phase":format!("{:?}",t.phase),"targets":t.targets.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"cutters":t.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"parts":t.cuts.iter().map(|c|c.parts.count()).collect::<Vec<_>>(),"removed":t.cuts.iter().map(|c|&c.removed).collect::<Vec<_>>(),"hover":t.hover}).to_string()))?;
    canvas.set_attribute("data-split", &editor.split.as_ref().map_or_else(||"null".into(),|s|serde_json::json!({"target":s.target.id.value(),"face":s.face,"cutters":s.cutters.iter().map(|o|o.id.value()).collect::<Vec<_>>()}).to_string()))?;
    canvas.set_attribute("data-project", &editor.draft.as_ref().and_then(|d|d.project.as_ref()).map_or_else(||"null".into(), |p|serde_json::json!({"targets":p.targets.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"points":p.points,"unsupported":p.unsupported}).to_string()))?;
    canvas.set_attribute("data-extend", &editor.draft.as_ref().and_then(|d|d.extend.as_ref()).map_or_else(||"null".into(), |e|serde_json::json!({"boundaries":e.boundaries.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"curves":e.curves,"distance":e.distance,"asking":e.asking,"candidates":e.candidates.len(),"hover":e.hover.as_ref().map(|h|serde_json::json!({"owner":h.owner.id.value(),"end":format!("{:?}",h.end),"tail":h.tail})),"error":e.error}).to_string()))?;
    canvas.set_attribute("data-gather",&editor.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||"null".into(),|g|serde_json::json!({"name":g.recipe.name,"step":g.progress.step,"rows":g.progress.rows,"answered":g.progress.answered,"choice":g.choice}).to_string()))?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
    canvas.set_attribute("data-snap-settings", &serde_json::json!({"objects":editor.snapping.objects,"modes":editor.snapping.modes,"grid":editor.snapping.grid,"step":editor.snapping.step}).to_string())?;
    canvas.set_attribute("data-snap-usage", &serde_json::to_string(&editor.snap_usage()).unwrap())?;
    canvas.set_attribute("data-preview-bytes", &renderer.preview_bytes().to_string())?;
    canvas.set_attribute("data-highlight-bytes", &renderer.highlight_bytes().to_string())?;
    canvas.set_attribute("data-selection-part", &editor.selection().map_or_else(String::new, |target| target.describe()))?;
    let sheets=editor.scene.sheets().iter().map(|row|serde_json::json!({"row":row.id.value(),"guid":row.guid,"source":row.prepared.original.guid,"model":row.model.m,"segments":row.prepared.original.segment_count,"entities":row.prepared.original.entity_count,"meta":row.prepared.original.meta,"streamed":row.prepared.streamed.is_some(),"resident":row.prepared.pages().iter().map(|page|page.rows.end-page.rows.start).sum::<u32>()})).collect::<Vec<_>>();canvas.set_attribute("data-sheets",&serde_json::to_string(&sheets).unwrap())?;canvas.set_attribute("data-sheet-cpu",&serde_json::to_string(&crate::memory::sheets(editor.scenes())).unwrap())?;canvas.set_attribute("data-sheet-gpu",&serde_json::to_string(&renderer.sheet_usage()).unwrap())?;
    canvas.set_attribute("data-instance-cpu",&serde_json::to_string(&crate::memory::instances(editor.scenes())).unwrap())?;
    canvas.set_attribute("data-instance-gpu",&serde_json::to_string(&renderer.instance_usage()).unwrap())?;
    canvas.set_attribute("data-instance-allocations",&serde_json::to_string(&renderer.instance_allocations()).unwrap())?;
    let instances=editor.scene.instances.iter().map(|(id,owner)|serde_json::json!({"id":id.value(),"guid":editor.scene.row_guid(*id),"source_guid":owner.instance.guid(),"definition_guid":owner.definition.guid(),"document_guid":owner.document.guid(),"model":editor.scene.placement(*id).map(|m|m.m),"flags":owner.instance.flags,"colour":editor.scene.face_rgba(*id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-instances",&serde_json::to_string(&instances).unwrap())?;
    let stream_usage=crate::memory::streams(editor.scenes());
    canvas.set_attribute("data-range-sources",&serde_json::to_string(&stream_usage).unwrap())?;
    let streamed=editor.scene.streams().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source_guid":row.prepared.source.guid,"name":row.prepared.source.name,"url":row.prepared.source.url,"revision":row.prepared.source.fields.revision,"total":row.prepared.source.fields.count,"model":row.model.m,"point_px":row.point_px,"controls":row.controls_visible})).collect::<Vec<_>>();
    canvas.set_attribute("data-range-documents",&serde_json::to_string(&streamed).unwrap())?;

    canvas.set_attribute("data-pick-mode", &format!("{:?}", editor.pick_mode))?;
    canvas.set_attribute("data-part-usage", &serde_json::to_string(&renderer.part_usage()).unwrap())?;
    canvas.set_attribute("data-selected-ids", &serde_json::to_string(&editor.selected_objects().iter().map(|id| id.value()).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-selected-id", &editor.selected.map(|id| format!("{id:?}")).unwrap_or_default())?;
    canvas.set_attribute("data-grid-plane",&serde_json::json!({"origin":(0..3).map(|i|editor.drawing_plane().origin()[i]).collect::<Vec<_>>(),"matrix":editor.drawing_plane().matrix()}).to_string())?;
    canvas.set_attribute("data-grid-enabled", if editor.camera.grid.is_some() { "true" } else { "false" })?;
    canvas.set_attribute("data-grid-gpu", &serde_json::to_string(&renderer.grid_usage()).unwrap())?;
    canvas.set_attribute("data-normal-view", if editor.normal_view { "true" } else { "false" })?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-render-origin",&serde_json::to_string(&renderer.anchor()).unwrap())?;
    canvas.set_attribute("data-camera-target",&serde_json::to_string(&editor.camera.target).unwrap())?;
    canvas.set_attribute("data-render-matrix",&serde_json::to_string(&renderer.camera_matrix(&editor.camera)).unwrap())?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let mut cpu = crate::memory::cpu(editor.presentation_scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    cpu[4]+=renderer.arena_cpu_bytes();
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
    touch:&mut crate::touch::Touch,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            let rect=canvas.get_bounding_client_rect();
            let Action::Zoom(factor)=crate::navigation::wheel(wheel.delta_y(),mode,rect.height())? else {return None;};
            let screen=[(2.*(wheel.client_x() as f64-rect.left())/rect.width()-1.) as f32,(1.-2.*(wheel.client_y() as f64-rect.top())/rect.height()) as f32];
            Some(Action::ZoomAt {factor,screen})
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _=>{
            if let Some(pointer)=event.dyn_ref::<web_sys::PointerEvent>().filter(|p|p.pointer_type()=="touch") {
                event.prevent_default();let r=canvas.get_bounding_client_rect();let rect=[r.left(),r.top(),r.width(),r.height()];let at=[pointer.client_x() as f64,pointer.client_y() as f64];let id=pointer.pointer_id();
                let actions=match event.type_().as_str() {
                    "pointerdown"=>{gesture.cancel();let multiple=touch.count()>0;if touch.press(id,at,event.time_stamp()) {if canvas.set_pointer_capture(id).is_err() {touch.cancel();}}if multiple {vec![Action::GizmoCancel,Action::CanvasCancel]} else {Vec::new()}},
                    "pointermove"=>touch.move_to(id,at,rect),
                    "pointerup"=>{let actions=touch.release(id,at,event.time_stamp(),rect);let _=canvas.release_pointer_capture(id);actions},
                    "pointercancel"=>{touch.cancel();Vec::new()},"lostpointercapture"=>{if touch.contains(id) {touch.cancel();}Vec::new()},_=>Vec::new(),
                };return (!actions.is_empty()).then_some(Action::Touch(actions));
            }
            return pointer_action(event,canvas,gesture);
        },
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press_at(id, pointer.button(), position,pointer.ctrl_key()||pointer.meta_key(),pointer.time_stamp()) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_at(id, position,pointer.time_stamp()),
        "pointerup" => {
            let motion = gesture.release_at(id, position,pointer.time_stamp());
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    let mut action=motion.action([rect.left(),rect.top(),rect.width(),rect.height()])?;
    match &mut action {
        Action::CanvasStart {modified,pointer:id,..}=>{*id=pointer.pointer_id();*modified=pointer.shift_key()||pointer.ctrl_key()||pointer.meta_key();},
        Action::CanvasRelease {add,remove,..}=>{*add=pointer.shift_key();*remove=pointer.ctrl_key()||pointer.meta_key();},_=>{}
    }Some(action)
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let density = crate::browser_recovery::density(crate::browser_defaults::scale(window.device_pixel_ratio()));
    renderer.set_density(density as f32);
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        density,
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    normals: bool,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw_mode(&view, background, transform, normals);
    if let Some(error)=renderer.cloud_error() {panel.result(&format!("Cloud: {error}"));}
    if let Some(error)=renderer.text_error() {panel.result(&format!("Text: {error}"));}
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}

fn gizmo_action(event:&web_sys::Event,canvas:&web_sys::HtmlCanvasElement,editor:&Editor,consumed:bool)->Option<Action> {
    if matches!(event.type_().as_str(),"blur"|"resize") && editor.gizmo_edit.is_some() {return Some(Action::GizmoCancel);}
    let pointer=event.dyn_ref::<web_sys::PointerEvent>()?;
    if matches!(event.type_().as_str(),"pointercancel"|"lostpointercapture") {
        return editor.gizmo_edit.as_ref().filter(|e|e.pointer==Some(pointer.pointer_id())).map(|_|Action::GizmoCancel);
    }
    let rect=canvas.get_bounding_client_rect();let size=[rect.width(),rect.height()];
    if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
    let screen=[(2.*(pointer.client_x() as f64-rect.left())/size[0]-1.) as f32,(1.-2.*(pointer.client_y() as f64-rect.top())/size[1]) as f32];
    let id=pointer.pointer_id();
    if editor.gizmo_edit.as_ref().is_some_and(|e|e.pointer==Some(id)) {
        event.prevent_default();
        return match event.type_().as_str() {
            "pointermove"=>Some(Action::GizmoMove {pointer:id,screen,size,shift:pointer.shift_key()}),
            "pointerup"=>{let _=canvas.release_pointer_capture(id);Some(Action::GizmoRelease {pointer:id,screen,size,shift:pointer.shift_key()})},
            _=>None,
        };
    }
    if consumed || event.type_()!="pointerdown" || !pointer.is_primary() || pointer.button()!=0 {return None;}
    if pointer.shift_key() || pointer.ctrl_key() || pointer.meta_key() {return None;}
    let finger=pointer.pointer_type()=="touch";
    if editor.gizmo_hit_radius(screen,size,if finger {18.} else {8.}).is_some() {
        event.prevent_default();
        if canvas.set_pointer_capture(id).is_err() {return Some(Action::GizmoCancel);}
        Some(if finger {Action::GizmoTouch {pointer:id,screen,size}} else {Action::GizmoPress {pointer:id,screen,size,modified:false}})
    } else if editor.gizmo_edit.is_some() {Some(Action::GizmoCancel)} else {None}
}

fn lasso_action(event:&web_sys::Event,canvas:&web_sys::HtmlCanvasElement,editor:&Editor,consumed:bool)->Option<Action>{
    let lasso=editor.lasso.as_ref()?;if matches!(event.type_().as_str(),"resize"|"blur"){return Some(Action::LassoCancel)}
    let pointer=event.dyn_ref::<web_sys::PointerEvent>()?;let id=pointer.pointer_id();let r=canvas.get_bounding_client_rect();let at=[pointer.client_x() as f64-r.left(),pointer.client_y() as f64-r.top()];
    if lasso.pointer==Some(id){event.prevent_default();return match event.type_().as_str(){"pointermove"=>Some(Action::LassoMove{at,pointer:id}),"pointerup"=>{let _=canvas.release_pointer_capture(id);Some(Action::LassoRelease{at,pointer:id,add:pointer.shift_key(),remove:pointer.ctrl_key()||pointer.meta_key()})},"pointercancel"|"lostpointercapture"=>Some(Action::LassoCancel),_=>None}}
    if consumed||event.type_()!="pointerdown"||pointer.button()!=0||!pointer.is_primary(){return None}event.prevent_default();if canvas.set_pointer_capture(id).is_err(){return Some(Action::LassoCancel)}Some(Action::LassoPress{at,size:[r.width(),r.height()],density:canvas.width() as f64/r.width(),pointer:id})
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzs-adopt-the-shared-origin-in-the-browser-03.rs"
```

```text
--8<-- "journey/code/96zzzzzs-adopt-the-shared-origin-in-the-browser-04.rs"
```

```text
--8<-- "journey/code/96zzzzzs-adopt-the-shared-origin-in-the-browser-05.rs"
```

```text
--8<-- "journey/code/96zzzzzs-adopt-the-shared-origin-in-the-browser-06.rs"
```

```text
--8<-- "journey/code/96zzzzzs-adopt-the-shared-origin-in-the-browser-07.rs"
```

```text
--8<-- "journey/code/96zzzzzs-adopt-the-shared-origin-in-the-browser-08.rs"
```

</details>

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 96zzzzzs-adopt-the-shared-origin-in-the-browser
npm --prefix ../session_tests run course -- check 96zzzzzs-adopt-the-shared-origin-in-the-browser
npm --prefix ../session_tests run course -- save 96zzzzzs-adopt-the-shared-origin-in-the-browser
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

The browser uses real anchored coordinates and finishes deferred rebases after input stops.

- **Try this:** Create a unit Box, move it by 1000000000,-1000000000,1000000000 and Fit. Select its original, clear selection and Undo/Redo the move. Zoom out and pan twice quickly, then stop input; the deferred frame must settle without another gesture. Save/reopen and confirm the original source and view.

Continue with [Reversed depth and homogeneous near clipping](96zzzzzt-share-depth-endpoints-and-comparisons.md).
