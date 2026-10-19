# 22.4.3 · Keep contact ambient inputs on originals

**Chapter 22 · Surface lighting, contact and display quality · Section 22.4 · Geometric ambient inputs and pass order** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

Read ambient properties at the contact surface’s actual display depth and use its original object’s radius. Reuse the same geometry function as the face pass so contact shading stays correct without new GPU resources.

## Type

Continue from [Shade surfaces before drawing ink](96zzzzzzs-shade-surfaces-before-drawing-ink.md). [Save or recover your work](recovery.md).

### 1. `src/feature_owner.rs`

Resolve the existing original visual ancestor before applying pickability. A locked original still owns its visible contact shading. Keep existing pick and highlight filtering while exposing raw visual ownership. Use the same original hierarchy for visual shading. Preserve the existing original picking contract.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{hierarchy::Document, scene::{ObjectId, Scene}};

fn ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    let mut parent = doc.nodes[index].parent;
    while let Some(index) = parent {
        let node = &doc.nodes[index];
        if let Some(id) = node.object {
            if scene.instances.contains_key(&id) || scene.objects().iter().any(|row| row.id == id
                && matches!(row.original.as_ref(), Some(session_rust::Geometry::Element(_)))) {
                return scene.pick_id(id.value());
            }
        }
        parent = node.parent;
    }
    None
}

pub fn display_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    if !scene.forest.derived(id) { return scene.pick_id(id.value()); }
    for doc in &scene.forest.documents {
        if let Some(index) = doc.nodes.iter().position(|node| node.object == Some(id)) {
            return ancestor(scene, doc, index);
        }
    }
    None
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
    let (document, guid) = key.split_once('/')?;
    let document = document.parse::<u32>().ok()?;
    let doc = scene.forest.documents.iter().find(|doc| doc.key == document)?;
    let index = doc.nodes.iter().position(|node| node.source.guid == guid)?;
    if !doc.nodes[index].readonly { return None; }
    ancestor(scene, doc, index)
}

pub fn highlighted(scene: &Scene, selected: &[ObjectId]) -> Vec<ObjectId> {
    let mut ids = selected.to_vec();
    for doc in &scene.forest.documents {
        for (index, node) in doc.nodes.iter().enumerate() {
            if !node.derived { continue; }
            let Some(id) = node.object.filter(|id| scene.visible(*id)) else { continue; };
            if ancestor(scene, doc, index).is_some_and(|owner| selected.contains(&owner)) {
                ids.push(id);
            }
        }
    }
    ids.sort_unstable_by_key(|id| id.value());
    ids.dedup();
    ids
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{hierarchy::Document, scene::{ObjectId, Scene}};

fn visual_ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    let mut parent = doc.nodes[index].parent;
    while let Some(index) = parent {
        let node = &doc.nodes[index];
        if let Some(id) = node.object {
            if scene.instances.contains_key(&id) || scene.objects().iter().any(|row| row.id == id
                && matches!(row.original.as_ref(), Some(session_rust::Geometry::Element(_)))) {
                return Some(id);
            }
        }
        parent = node.parent;
    }
    None
}

fn ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    visual_ancestor(scene,doc,index).and_then(|id|scene.pick_id(id.value()))
}

pub fn visual_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    if !scene.forest.derived(id) { return Some(id); }
    for doc in &scene.forest.documents {
        if let Some(index) = doc.nodes.iter().position(|node| node.object == Some(id)) {
            return visual_ancestor(scene, doc, index);
        }
    }
    None
}

pub fn display_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    visual_owner(scene,id).and_then(|id|scene.pick_id(id.value()))
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
    let (document, guid) = key.split_once('/')?;
    let document = document.parse::<u32>().ok()?;
    let doc = scene.forest.documents.iter().find(|doc| doc.key == document)?;
    let index = doc.nodes.iter().position(|node| node.source.guid == guid)?;
    if !doc.nodes[index].readonly { return None; }
    ancestor(scene, doc, index)
}

pub fn highlighted(scene: &Scene, selected: &[ObjectId]) -> Vec<ObjectId> {
    let mut ids = selected.to_vec();
    for doc in &scene.forest.documents {
        for (index, node) in doc.nodes.iter().enumerate() {
            if !node.derived { continue; }
            let Some(id) = node.object.filter(|id| scene.visible(*id)) else { continue; };
            if ancestor(scene, doc, index).is_some_and(|owner| selected.contains(&owner)) {
                ids.push(id);
            }
        }
    }
    ids.sort_unstable_by_key(|id| id.value());
    ids.dedup();
    ids
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-05.rs"
```

</details>

### 2. `src/ambient_rows.rs`

Remove the obsolete maximum before visual-owner mapping. Compute the maximum after resolving original visual radii. Map temporary numeric radii to original visual owners, without retaining another source or property cache.

<details>
<summary>Locate the existing block</summary>

```rust
#[derive(Default)]
pub struct Rows{bounds:crate::cap_source_bounds::Bounds,pub specs:Vec<(u32,f32)>,pub ground:[f64;2]}
impl Rows{
 pub fn ground_at(&self,anchor:[f64;3])->[f32;2] {[(self.ground[0]-anchor[2])as f32,self.ground[1]as f32]}
 pub fn set(&mut self,scene:&crate::scene::Scene){self.specs.clear();let mut floor=f64::INFINITY;let mut radius=0f32;for object in scene.objects().iter().filter(|r|scene.visible(r.id)&&!r.mesh.indices().is_empty()){if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let r=crate::ambient_radius::ambient_radius(&bounds);self.specs.push((object.id.value(),r));floor=floor.min(bounds.cz-bounds.hz);radius=radius.max(r);}}self.ground=[if floor.is_finite(){floor}else{0.},f64::from(radius)];self.bounds.prune();if self.specs.is_empty(){self.specs.shrink_to_fit();}}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
#[derive(Default)]
pub struct Rows{bounds:crate::cap_source_bounds::Bounds,pub specs:Vec<(u32,f32)>,pub ground:[f64;2]}
impl Rows{
 pub fn ground_at(&self,anchor:[f64;3])->[f32;2] {[(self.ground[0]-anchor[2])as f32,self.ground[1]as f32]}
 pub fn set(&mut self,scene:&crate::scene::Scene){self.specs.clear();let mut floor=f64::INFINITY;for object in scene.objects().iter().filter(|r|scene.visible(r.id)&&!r.mesh.indices().is_empty()){if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let r=crate::ambient_radius::ambient_radius(&bounds);self.specs.push((object.id.value(),r));floor=floor.min(bounds.cz-bounds.hz);}}let mut radii=self.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for object in scene.objects().iter().filter(|r|scene.forest.derived(r.id)){if let Some(radius)=crate::feature_owner::visual_owner(scene,object.id).and_then(|id|radii.get(&id.value())).copied(){radii.insert(object.id.value(),radius);}}for(id,radius)in &mut self.specs{*radius=radii[id];}let radius=self.specs.iter().fold(0f32,|radius,row|radius.max(row.1));self.ground=[if floor.is_finite(){floor}else{0.},f64::from(radius)];self.bounds.prune();if self.specs.is_empty(){self.specs.shrink_to_fit();}}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-08.rs"
```

</details>

### 3. `src/ambient_sources.wgsl`

Reuse the actual geometry function, including contact depth and clipping flags. Read existing packed colour and normal words for the shared contact-depth function. Match contact coverage in the existing shared object path too.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32}
fn ambient_vertex(position:vec3<f32>,object:ObjectSettings)->Vertex {
 let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,object.selection.z,bitcast<f32>(object.range.z));
}
@vertex fn vs_main(@location(0) position:vec3<f32>)->Vertex {return ambient_vertex(position,object_at(0u));}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,object);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(face,-face,front));
 return Source(in.radius,encode_normal(normal)*0.5+0.5);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32}
fn ambient_vertex(position:vec3<f32>,colour:vec4<f32>,normal:vec3<f32>,object:ObjectSettings)->Vertex {
 let input=geometry(position,colour,normal,object);return Vertex(input.position,input.world,input.clip_flags,bitcast<f32>(object.range.z));
}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(1) normal:u32,@location(2) colour:u32)->Vertex {return ambient_vertex(position,unpack4x8unorm(colour),oct32_decode(normal),object_at(0u));}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,input.colour,input.normal,object);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(face,-face,front));
 return Source(in.radius,encode_normal(normal)*0.5+0.5);
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-11.rs"
```

</details>

### 4. `src/ambient_source_pipeline.rs`

Use the existing 20-byte vertex layout; no additional buffer or attribute storage.

<details>
<summary>Locate the existing block</summary>

```rust
  let attributes=wgpu::vertex_attr_array![0=>Float32x3];let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some(if shared {"vs_record"}else{"vs_main"}),compilation_options:Default::default(),buffers:&[if shared {crate::draw_slots::layout()}else{wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});draw};let draw=create(false);let record_draw=create(true);Self{draw,record_draw}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  let attributes=wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32];let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some(if shared {"vs_record"}else{"vs_main"}),compilation_options:Default::default(),buffers:&[if shared {crate::draw_slots::layout()}else{wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});draw};let draw=create(false);let record_draw=create(true);Self{draw,record_draw}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/ambient_contact_tests.rs`

Check real contact coverage/radius, both object paths and device budgets, locks, original picks, source semantics and placed definitions.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-13.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/renderer.rs`

Register the two actual original contact-input GPU checks.

<details>
<summary>Locate the existing block</summary>

```text
mod ambient_order_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzt-keep-contact-ambient-inputs-on-originals-14.rs" skip=24
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.4.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Actual contact pixels retain their original radius and receive ambient shading. Uniform and storage paths share the existing contact geometry function, including device fallback and original ownership.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual original contact fill receives ambient shading near its original occluder. Source vertices reuse the same contact placement as the face pass, and the current object record carries the original visual radius. Both object paths, projections and densities retain original face ownership and complete source values; Move/Undo restores camera, pixels and geometry. Multisampling, remaining view controls, runtime efficiency, independence and final whole-course validation remain unfinished. Broader facing and NURBS quality review also remains required.

![Actual original contact fill receives ambient shading near its original occluder. Source vertices reuse the same contact placement as the face pass, and the current object record carries the original visual radius. Both object paths, projections and densities retain original face ownership and complete source values; Move/Undo restores camera, pixels and geometry. Multisampling, remaining view controls, runtime efficiency, independence and final whole-course validation remain unfinished. Broader facing and NURBS quality review also remains required.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

<details>
<summary>Code explanation and diagram</summary>

A real original panel and contact feature expose a zero ambient radius even though an independent GPU query resolves the original face. The contact depth is seven float steps closer than the original plane; the separate source vertex path fails the exact depth test. Its separate radius also differs from the original object’s radius.

Call the existing geometry function from both source vertex paths. It already handles contact placement and clipping flags. Reuse the packed colour and normal words in the existing 20-byte vertex layout, while ambient normals still come from geometric face derivatives. Resolve visual ancestry before applying picking filters, so locking never removes visible contact shading.

Original visual owner → existing radius word → shared contact geometry → exact source depth → geometric ambient normal.



Why can a visually aligned contact surface still lose its ambient input?

The displayed contact uses a deliberate depth shift. A second vertex path without that shift fails exact depth matching. Sharing geometry fixes coverage, and the original visual owner supplies the correct radius independently of picking locks.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect actual source-radius readbacks and matching whole frames, both projections and densities, real 64 KiB device fallback, locked original owners, mirrored nonuniform definition placements and complete source values.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.4.3
npm --prefix ../session_tests run course -- save 22.4.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Reuse the existing geometry() function in both ambient source vertex paths, including contact depth and clipping flags. Read the correct packed normal and colour words from the existing 20-byte vertex layout; ambient normals still use geometric face derivatives. Resolve original visual ancestry before applying picking locks, then update existing radius words through a temporary numeric map. No extra GPU buffer, texture, pipeline, bias tolerance or persistent property/source owner is introduced. All 990 native tests, WebAssembly and optimized browser builds, the frame check, declared Chrome checks and stored evidence pass. Eight Native and five canonical Chrome pictures were individually inspected. Native checks verify exact uniform/storage property buffers, actual 64 KiB device fallback, locked visual shading, actual original picks, complete decoded embedded-mesh/source/presentation values, true mirrored nonuniform definitions, restoration without geometry uploads and disposal of original sources. Definition-instance unload retains the existing explicit ineligible rejection. Main physical triangles may belong to derived contact geometry, while request-owned face queries resolve the original Element; those identities remain distinct. Chrome independently measures contact shading against actual unshaded fill, verifies actual camera modes, original ownership, full archives and Move/Undo. All heavy checks use a 2 GiB memory cap, no swap and two CPU cores with software adapters. Multisampling, remaining view controls, runtime efficiency, independence and final whole-course validation remain unfinished. Broader facing and NURBS quality review also remains required. Fresh physical-pixel disc correction in existing22.3.4 and its inherited source. All 990 Native tests, WASM, optimized web, frame, declared canonical Chrome and stored checks passed. Every current Native and canonical browser image was individually directly inspected and bound to this exact application/checker. Earlier input fixtures and their old Native references remain historical producer evidence; current GPU images and full runs are separate. The unchanged half-physical-pixel minimum still visibly crowds extreme zoom-out. No new geometry buffer, pipeline, radius/padding policy or tolerance change. Software checks do not establish hardware FPS. Full viewer course remains unfinished.

The supplied arena stale-binding probe checks actual GPU array lengths before reading indices and vertices. Its pixel, buffer-identity and exact write-count assertions remain unchanged. A stale binding is deliberately retained, but the fixture no longer relies on implementation-dependent out-of-bounds shader behavior.

Actual original contact fill receives ambient shading near its original occluder. Source vertices reuse the same contact placement as the face pass, and the current object record carries the original visual radius. Both object paths, projections and densities retain original face ownership and complete source values; Move/Undo restores camera, pixels and geometry. Multisampling, remaining view controls, runtime efficiency, independence and final whole-course validation remain unfinished. Broader facing and NURBS quality review also remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.4.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
