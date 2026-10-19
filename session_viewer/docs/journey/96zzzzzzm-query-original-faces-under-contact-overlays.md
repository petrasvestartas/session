# 22.2.5 · Query original faces under contact overlays

**Chapter 22 · Surface lighting, contact and display quality · Section 22.2 · Contact rendering and shared frames** · [Contents](contents.md)

**Typing: 8–15 minutes.** [Estimate](typing-load.md).

The contact selects its original parent as an object, but its generated face address cannot resolve as an original face. Exclude derived display rows from the face query and its depth pass. Keep original locked geometry as an occluder and preserve real source keys.

## Type

Continue from [Layer original contact boundaries](96zzzzzzl-layer-original-contact-boundaries.md). [Save or recover your work](recovery.md).

### 1. `src/gpu_mesh.rs`

Retain source-query eligibility on the existing CPU mesh. Classify original and derived display rows explicitly. Standalone original geometry and the existing highlight adapter remain valid. Keep the flag out of the packed GPU record. Update eligibility without a GPU setting write.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        self.translation=object.model.m[12..15].try_into().unwrap();
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start,self.anchor);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
```

</details>

Replace it with:

```rust
    pub fn settings_bytes(&self) -> u64 { self.model.size() }

    pub fn update(&mut self, queue: &wgpu::Queue, object: &Object, style:Style) -> [usize; 2] {
        self.source_faces=style.source_faces;
        self.translation=object.model.m[12..15].try_into().unwrap();
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start,self.anchor);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
```

<details>
<summary>Locate this block</summary>

```rust
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { translation:object.model.m[12..15].try_into().unwrap(),anchor,geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
```

</details>

Replace it with:

```rust
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: model.as_entire_binding() }],
        });
        Self { source_faces:style.source_faces,translation:object.model.m[12..15].try_into().unwrap(),anchor,geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
```

<details>
<summary>Locate this block</summary>

```rust
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{selected,owner:if object.metadata.is_locked {0}else{object.id.value()},colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
```

</details>

Replace it with:

```rust
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{source_faces:true,selected,owner:if object.metadata.is_locked {0}else{object.id.value()},colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
```

<details>
<summary>Locate this block</summary>

```rust
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    model: wgpu::Buffer,
    translation:[f64;3],anchor:[f64;3],
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub selected:bool,pub owner:u32,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
```

</details>

Replace it with:

```rust
    pub geometry: Rc<GpuGeometry>,
    model_group: wgpu::BindGroup,
    pub id: crate::scene::ObjectId,
    pub source_faces:bool,
    model: wgpu::Buffer,
    translation:[f64;3],anchor:[f64;3],
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub source_faces:bool,pub selected:bool,pub owner:u32,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
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
    pub source_faces:bool,
    model: wgpu::Buffer,
    translation:[f64;3],anchor:[f64;3],
    previous: Record,primitive_base:u32,xray:bool,cap_flags:u32,cap_token:u32,
}

#[derive(Clone,Copy)]
pub struct Style {pub source_faces:bool,pub selected:bool,pub owner:u32,pub colour:Option<[f32;4]>}

fn settings(object: &Object, style:Style,cap_flags:u32,cap_token:u32,primitive_base:u32,index_start:u32,anchor:[f64;3]) -> Record {
    let mut bytes = [0; BYTES];
    let model=crate::render_space::placed(&object.model,anchor).to_f32();
    for (index, value) in model.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
    // Integer settings preserve every object identity, including 16,777,217, which a single-precision float rounds to another value.
    bytes[64..68].copy_from_slice(&u32::from(style.selected).to_ne_bytes());
    let id=style.owner;
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
        Self::upload_at(device,layout,object,selected,[0.;3])
    }
    pub fn upload_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,selected:bool,anchor:[f64;3])->Self {
        let geometry=Rc::new(GpuGeometry::upload(device,Rc::clone(&object.mesh)));
        Self::with_geometry_at(device,layout,object,Style{source_faces:true,selected,owner:if object.metadata.is_locked {0}else{object.id.value()},colour:None},geometry,anchor)
    }

    pub fn with_geometry(device: &wgpu::Device, layout: &wgpu::BindGroupLayout,
        object: &Object, style:Style, geometry: Rc<GpuGeometry>) -> Self
    {
        Self::with_geometry_at(device,layout,object,style,geometry,[0.;3])
    }
    pub fn with_geometry_at(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,object:&Object,style:Style,geometry:Rc<GpuGeometry>,anchor:[f64;3])->Self {
        let bytes=settings(object,style,0,0,0,geometry.index_start,anchor);
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
        Self { source_faces:style.source_faces,translation:object.model.m[12..15].try_into().unwrap(),anchor,geometry, model_group, id: object.id, model, previous: bytes,primitive_base:0,xray:false,cap_flags:0,cap_token:0 }
    }

    pub fn rebase(&mut self,queue:&wgpu::Queue,anchor:[f64;3])->[usize;2] {
        assert!(anchor.iter().all(|v|v.is_finite()));self.anchor=anchor;
        let bytes=crate::render_space::relative(self.translation,anchor).into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
        if self.previous[48..60]==bytes {return [0,0];}
        self.previous[48..60].copy_from_slice(&bytes);queue.write_buffer(&self.model,48,&bytes);[1,12]
    }
    pub(crate) fn model_buffer(&self)->&wgpu::Buffer{&self.model}
    pub(crate) fn packed(&self)->Record {self.previous}
    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())&1!=0}

    pub fn set_ambient_radius(&mut self,queue:&wgpu::Queue,radius:f32){let bytes=radius.to_ne_bytes();if self.previous[RANGE+8..RANGE+12]!=bytes {self.previous[RANGE+8..RANGE+12].copy_from_slice(&bytes);queue.write_buffer(&self.model,(RANGE+8)as u64,&bytes);}}
    pub fn primitive_base(&self)->u32{self.primitive_base}
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
        self.source_faces=style.source_faces;
        self.translation=object.model.m[12..15].try_into().unwrap();
        let mut next = settings(object, style,self.cap_flags|if self.xray&&self.cap_flags&8192!=0{32768}else{0},self.cap_token,self.primitive_base,self.geometry.index_start,self.anchor);
        next[RANGE+8..RANGE+12].copy_from_slice(&self.previous[RANGE+8..RANGE+12]);
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
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-05.rs"
```

</details>

### 2. `src/renderer.rs`

Instances and locked originals are not generated attribute display pieces.

<details>
<summary>Locate the existing block</summary>

```rust
fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {source_faces:!scene.forest.derived(id),selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-06.rs"
```

</details>

### 3. `src/pick_sources.rs`

Classify the source row once, then append only its actual original faces.

<details>
<summary>Locate the existing block</summary>

```rust
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-07.rs"
```

### 4. `src/face_pick.rs`

Do not allocate generated display-face metadata; preserve one row per mesh.

<details>
<summary>Locate the existing block</summary>

```rust
            if geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            if !mesh.source_faces||geometry.source.parts.triangles.is_empty() { self.rows.push(None); continue; }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-08.rs"
```

</details>

### 5. `src/pick_encode.rs`

Ignore derived depth only for source-face queries; keep all original occluders.

<details>
<summary>Locate the existing block</summary>

```rust
            for mesh in meshes {if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-09.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Preserve the unchanged original-source uniform/reference test.

<details>
<summary>Locate the existing block</summary>

```text
Style{selected:false,owner:owner.value(),colour:None}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-10.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/contact_feature_tests.rs`

Reuse the unchanged original contact ring validation fixture.

<details>
<summary>Locate the existing block</summary>

```text
fn polygon()->Polyline
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/source_face_policy_tests.rs`

Supply actual original face GPU/source/archive, locked occlusion, instance and BRep controls.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/renderer.rs`

Connect optional Native validation outside the browser runtime.

<details>
<summary>Locate the existing block</summary>

```text
mod contact_boundary_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzm-query-original-faces-under-contact-overlays-13.rs" skip=27
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.2.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Face picks resolve the original Mesh or BRep face under the contact. Beyond the host and the cursor pick radius there is no original face to select. Object picking and the contact drawing are unchanged.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Critical actual original face queries: sparse original Mesh faces 17 and 93 highlight separately in Top and Isometric at DPR1/2 while the readonly contact stays red. Beyond the host and cursor radius, Face selection is empty and Object selection still identifies the original parent. The actual original BRep top face 1 also resolves. Cropped GPU readback and exact clearing/zoom/MoveUndo/source/complete archive restoration pass. Source pen, facing and default vertex-marker policy, current ambient inputs/order, multisampling, remaining view controls, runtime efficiency and final whole-course validation remain unfinished.

![Critical actual original face queries: sparse original Mesh faces 17 and 93 highlight separately in Top and Isometric at DPR1/2 while the readonly contact stays red. Beyond the host and cursor radius, Face selection is empty and Object selection still identifies the original parent. The actual original BRep top face 1 also resolves. Cropped GPU readback and exact clearing/zoom/MoveUndo/source/complete archive restoration pass. Source pen, facing and default vertex-marker policy, current ambient inputs/order, multisampling, remaining view controls, runtime efficiency and final whole-course validation remain unfinished.](../screenshots/journey/96zzzzzzm-query-original-faces-under-contact-overlays-browser.png)

[Verification scope](release.md).

<details>
<summary>Code explanation and diagram</summary>

A contact can span several original faces or extend outside its host. Aliasing every generated polygon to parent face zero would invent source identity. Query the actual original beneath it instead. Instances retain their original face identities; selection does not grant permission to edit a readonly definition. The readonly red overlay can remain above the selected original-face highlight.

Original versus derived row → existing CPU mesh eligibility → original depth and source-face passes → actual source key.



Why must the original depth pass also ignore derived contact rows in Face mode?

The contact is layered nearer than its host. Skipping only its face-ID draw would leave that nearer generated depth blocking the original face beneath it. Original locked or missing-face-metadata geometry still retains depth and occlusion.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Pick both wings of a contact spanning two sparse original face keys. Inspect the original-face highlight, outside-host clicks beyond the cursor pick radius, locked occlusion, shared instances, both projections/densities and exact source/archive/pixel restoration.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.2.5
npm --prefix ../session_tests run course -- save 22.2.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

One CPU eligibility flag in the existing mesh style distinguishes original rows from generated display pieces. Face-mode depth and metadata skip only generated rows; locked and missing-face-metadata originals retain occlusion. The source-address table preserves actual sparse Mesh/BRep keys and instances. Eligibility is computed once per source row, outside the face loop. No new packed GPU field, geometry owner, face alias or selection subsystem is added. This deliberately queries the actual underlying original face, rather than assigning every contact to parent face zero. Selecting a readonly instance component does not grant edit permission. Fresh 971 native canonical/WebAssembly/optimized/frame and full composed Chrome/stored checks pass. Source pen, facing and default vertex-marker policy, current ambient inputs/order, multisampling, remaining view controls, runtime efficiency and final whole-course validation remain unfinished.

Critical actual original face queries: sparse original Mesh faces 17 and 93 highlight separately in Top and Isometric at DPR1/2 while the readonly contact stays red. Beyond the host and cursor radius, Face selection is empty and Object selection still identifies the original parent. The actual original BRep top face 1 also resolves. Cropped GPU readback and exact clearing/zoom/MoveUndo/source/complete archive restoration pass. Source pen, facing and default vertex-marker policy, current ambient inputs/order, multisampling, remaining view controls, runtime efficiency and final whole-course validation remain unfinished.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.2.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
