# 20.6 · Retained allocation, live lookup and safe reclamation

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 20.6** (Redraft `10.10`) · **16 checkpoints (`96zzzzi-adapt-incremental-slots-to-pulled-draws` → `96zzzzx-retire-empty-arena-ranges-after-tail-trimming`)**

> **Learning Goal:** Keep physical triangle bases correct when pulled draws begin at local occurrence zero.

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
Retained allocator run → changed members and local slot base → correct physical IDs.
Existing compute recipe plus unsigned lookup texture → prepared projection pipeline.
Retire occupied tail → arena end shrinks → release empty source association → keep the new end.
```

- Keep physical triangle bases correct when pulled draws begin at local occurrence zero.
- Add the inverse lookup texture to the existing compute binding recipe.
- Release empty source records independently of the current arena extent.
- **Why it works (Why does a local pulled draw use the full physical base?):** Its occurrence starts at zero. Subtracting the definition first triangle would shift every primitive address backward.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **WGSL (Vertex Pull)** | `@group(0) @binding(b) var<storage, read> arena_verts: array<u32>;` | Fetches packed vertex positions, normals, and colors directly from a single growable storage buffer using `@builtin(vertex_index)`. |
| **Rust/WGSL (`f64`->`f32`)** | `let rel: Vec3 = (world_f64 - anchor_f64).as_vec3();` | Subtracts the camera-area `f64` anchor on the CPU before converting to `f32`, preventing 32-bit float mantissa exhaustion far from `(0,0,0)`. |
| **WGSL (`matrix.wgsl`)** | 16-word header + 3 words/cell procedural digits | Renders matrix heatmaps and 3-decimal numeric digits procedurally inside the fragment shader with zero texture uploads per cell. |
| **Rust (`ink_ranges.rs`)** | Merged contiguous `Range<u32>` instance draws | Toggles point/line/edge/CAD-wire family visibility without rebuilding GPU buffers or invalidating original owner indices. |

**Production source (`session_viewer/src`):** [engine/gpu/arena.rs](../../../../src/engine/gpu/arena.rs), [engine/gpu/buffers.rs](../../../../src/engine/gpu/buffers.rs), [engine/gpu/objects.rs](../../../../src/engine/gpu/objects.rs), [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs), [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs), [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs), [engine/gpu/clip.rs](../../../../src/engine/gpu/clip.rs), [engine/gpu/triangle_tiles.rs](../../../../src/engine/gpu/triangle_tiles.rs), [app/scene_sync/tombs.rs](../../../../src/app/scene_sync/tombs.rs), [build.rs](../../../../build.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-96zzzzi-adapt-incremental-slots-to-pulled-draws"></a>

<details>
<summary>Implementation 96zzzzi-adapt-incremental-slots-to-pulled-draws: Adapt incremental slots to pulled draws</summary>

Keep physical triangle bases correct when pulled draws begin at local occurrence zero.

Data flow: Retained allocator run → changed members and local slot base → correct physical IDs.

### 1. `src/slot_space.rs`

Adapt physical bases to local pulled occurrences and report only changed member positions to the existing allocator.

<details>
<summary>Locate the existing block</summary>

```rust
  pub fn texel(&self, slot: u32) -> [u32; 4] {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
  pub fn local_slot(&self,slot:u32)->Slot {let texel=self.texel(slot);[texel[0],texel[1]]}
  pub fn touched(&self,definitions:&[Definition])->Vec<(u32,u32)> {
    let mut out=Vec::new();for definition in definitions {let Some(run)=self.runs.iter().find(|run|run.key==Some(definition.key))else{continue};for(position,row)in definition.rows.iter().enumerate(){if position<run.cap as usize&&self.rows.get(run.start as usize+position)!=Some(row){out.push((definition.key,position as u32));}}}out
  }
  pub fn texel(&self, slot: u32) -> [u32; 4] {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzi-adapt-incremental-slots-to-pulled-draws-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/slot_space.rs`

Supply local/global occurrence equivalence, exact physical IDs, changed-member reporting, reorder and retired-hole checks while retaining all five original allocator fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    /// Instances number their triangles one after another, past the arena's:
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzi-adapt-incremental-slots-to-pulled-draws-02.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzj-prepare-exact-arena-slot-definitions"></a>

<details>
<summary>Implementation 96zzzzj-prepare-exact-arena-slot-definitions: Prepare exact arena slot definitions</summary>

Describe allocator members using exact arena ranges and stable object rows.

Data flow: Shared arena range plus stable rows → one definition per source → existing slot allocator.

### 1. `src/instance_members.rs`

Group stable object rows by exact shared arena ranges; reject standalone buffers and mixed arenas instead of narrowing pointers into IDs.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-01.rs"
```

### 2. `src/lib.rs`

Declare the preparatory definition adapter.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod instance_gpu;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod instance_members;
mod instance_gpu;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-02.rs"
```

</details>

### 3. `src/gpu_geometry.rs`

Expose shared arena identity as a borrowed owner without changing range or allocation ownership.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn allocated_bytes(&self)->u64{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared(arena)=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/instance_members.rs`

Supply exact range keys, repeated-definition merging, stable rows, original arena count, standalone/mixed-arena refusal and empty checks.

<details>
<summary>Locate the existing block</summary>

```text
use std::{collections::BTreeMap,ops::Range,rc::Rc};
use crate::gpu_geometry::GpuGeometry;
pub struct Members {pub key:u32,pub faces:Range<u32>,pub rows:Vec<u32>}
impl Members {
 pub fn definition(&self)->crate::slot_space::Definition<'_> {
  crate::slot_space::Definition {key:self.key,rows:&self.rows,faces:self.faces.clone(),pipes:0..0,ribbons:0..0}
 }
}
pub fn collect<'a>(sources:impl IntoIterator<Item=(&'a Rc<GpuGeometry>,Vec<u32>)>)->Option<(Vec<Members>,u32)> {
 let mut arena=None;let mut count=0;let mut members=BTreeMap::<u32,Members>::new();
 for(geometry,rows)in sources {
  let current=geometry.arena()?;if arena.is_some_and(|first|!Rc::ptr_eq(first,current)){return None;}arena=Some(current);
  count=u32::try_from(current.borrow().rows.indices.len()/3).ok()?;
  let first=geometry.index_start;let end=first.checked_add(geometry.index_count)?;
  if first%3!=0||geometry.index_count%3!=0{return None;}
  let member=members.entry(first).or_insert_with(||Members {key:first,faces:first..end,rows:Vec::new()});
  if member.faces.end!=end{return None;}member.rows.extend(rows);
 }
 Some((members.into_values().collect(),count))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzj-prepare-exact-arena-slot-definitions-04.rs" skip=1140
```

</details>

</details>

<a id="checkpoint-96zzzzk-retain-live-allocator-member-runs"></a>

<details>
<summary>Implementation 96zzzzk-retain-live-allocator-member-runs: Retain live allocator member runs</summary>

Update retained source runs from the live stable object rows.

Data flow: Current stable object rows → exact definitions → bounded retained runs.

### 1. `src/instance_gpu.rs`

Keep the existing allocator under the surface lane without adding a GPU owner. Begin without retained allocation state. Update retained source runs from exact arena definitions and changed member positions; unsupported or oversized plans retain earlier drawing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
```

</details>

Replace it with:

```rust
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);space.update(&definitions,&touched,arena);
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
```

<details>
<summary>Locate this block</summary>

```rust
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
```

</details>

Replace it with:

```rust
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
```

<details>
<summary>Locate this block</summary>

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

</details>

Replace it with:

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);space.update(&definitions,&touched,arena);
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-01.rs"
```

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-02.rs"
```

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-03.rs"
```

</details>

### 2. `src/instance_members.rs`

Keep enough integer headroom for retained runs, growth and free intervals before invoking the existing allocator.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn collect<'a>(sources:impl IntoIterator<Item=(&'a Rc<GpuGeometry>,Vec<u32>)>)->Option<(Vec<Members>,u32)> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn bounded(members:&[Members],arena:u32)->bool {
 let ids=members.iter().map(|member|member.rows.len()as u64*(member.faces.len()as u64/3)).sum::<u64>();
 u64::from(arena)*2+ids*2+8192*(members.len()as u64+1)<u64::from(crate::primitive_image::CAP_START)
}
pub fn collect<'a>(sources:impl IntoIterator<Item=(&'a Rc<GpuGeometry>,Vec<u32>)>)->Option<(Vec<Members>,u32)> {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/instance_members.rs`

Supply small, empty, reserved-arena and overflowing-instance budget checks without allocating large GPU buffers.

<details>
<summary>Locate the existing block</summary>

```text
 #[test]
 fn exact_ranges_merge_members_and_reject_unrelated_storage()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-05.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply actual retained allocator locations alongside the existing exact GPU comparisons.

<details>
<summary>Locate the existing block</summary>

```text
 let retained=gpu.instances.ordinary.clone();
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-06.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Verify a surviving original retains its allocator slot through removal and insertion.

<details>
<summary>Locate the existing block</summary>

```text
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-07.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Verify complete allocator release with the existing empty-scene GPU release checks.

<details>
<summary>Locate the existing block</summary>

```text
assert!(gpu.instances.ordinary.is_empty());
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzk-retain-live-allocator-member-runs-08.rs" skip=43
```

</details>

</details>

<a id="checkpoint-96zzzzl-prepare-one-plan-for-draw-slots-and-lookup"></a>

<details>
<summary>Implementation 96zzzzl-prepare-one-plan-for-draw-slots-and-lookup: Prepare one plan for draw slots and lookup</summary>

Publish one retained plan through the existing draw-slot and integer-texture owners.

Data flow: One retained plan → local draw-slot bytes and integer lookup texels.

### 1. `src/draw_slots.rs`

Publish one retained plan through both owners. Merge slot boundary changes with allocator reports and refresh the lookup header after shrinking; reuse the existing range coalescer.

<details>
<summary>Locate the existing block</summary>

```rust
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
 pub fn publish(&mut self,ctx:&GpuCtx,space:&crate::slot_space::SlotSpace,change:&crate::slot_space::Change,table:&mut crate::primitive_slot_table::Table)->bool {
  let rows=(1..space.end()).map(|slot|space.local_slot(slot)).collect::<Vec<_>>();
  assert!(change.slots.iter().all(|range|range.start<=range.end&&range.end<=space.end()));
  let mut ranges=change.slots.clone();ranges.extend(changed_runs(&self.previous,&rows).into_iter().map(|run|(run.start+1)as u32..(run.end+1)as u32));
  if rows.len()!=self.previous.len(){ranges.push(0..1);}
  let change=crate::slot_space::Change {all:change.all,slots:crate::slot_space::coalesce(ranges,space.end())};
  if rows.is_empty()&&self.instances()>0{self.clear(ctx,true);}else{self.set(ctx,&rows);}
  table.publish(ctx,space,&change)
 }
 pub fn clear(&mut self,ctx:&GpuCtx,free:bool){if free {self.buffer.release(ctx);}self.set(ctx,&[]);}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzl-prepare-one-plan-for-draw-slots-and-lookup-01.rs"
```

</details>

### 2. `src/slot_space.rs`

Expose the range coalescer already written so publication shares its bounds and merging instead of duplicating it.

<details>
<summary>Locate the existing block</summary>

```rust
fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzl-prepare-one-plan-for-draw-slots-and-lookup-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/primitive_slot_table_tests.rs`

Supply exact mapped writes and actual lookup shader results through high IDs, hole splitting/reuse, unchanged neighbours, shrinking headers, arena following and release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{primitive_slot_table::Table,text_gpu_support::GpuCtx};
fn read(ctx:&GpuCtx,table:&Table)->Vec<[u32;4]>{
 let size=table.texture.size();let pitch=(size.width*16+255)/256*256;let bytes=u64::from(pitch)*u64::from(size.height);
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("slot texture readback"),size:bytes,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=ctx.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(table.texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&output,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(size.height)}},size);ctx.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();output.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let mapped=output.slice(..).get_mapped_range();let result=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size.width as usize*16].chunks_exact(16).map(|texel|std::array::from_fn(|i|u32::from_le_bytes(texel[i*4..i*4+4].try_into().unwrap())))).collect();drop(mapped);output.unmap();result
}
#[test]
fn actual_integer_slot_texture_preserves_ids_across_partial_row_writes_and_release(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
 let mut wanted=(0..1050).map(|i|[i,0x80000000+i,16_777_217+i,3]).collect::<Vec<_>>();table.write(&ctx,&wanted);assert_eq!([table.texture.width(),table.texture.height()],[1024,2]);assert_eq!(table.texture.format(),wgpu::TextureFormat::Rgba32Uint);let texture=table.texture.clone();wanted.resize(2048,[0;4]);assert_eq!(read(&ctx,&table),wanted);
 let changed=[[7,u32::MAX-1,31,9],[11,0x80000000,37,12],[13,16_777_217,41,15]];assert!(table.write_at(&ctx,1023,&changed));wanted[1023..1026].copy_from_slice(&changed);assert_eq!(table.texture,texture);assert_eq!(read(&ctx,&table),wanted);
 assert!(!table.write_at(&ctx,u32::MAX,&changed));assert!(!table.write_at(&ctx,2047,&changed));assert!(table.write_at(&ctx,2048,&[]));assert_eq!(read(&ctx,&table),wanted);
 table.write(&ctx,&[]);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);assert!(!table.write_at(&ctx,0,&changed));assert_eq!(read(&ctx,&table),vec![[0;4]]);
}

#[test]
fn actual_allocator_publication_keeps_high_rows_free_slots_neighbours_and_release() {
 use crate::slot_space::{Definition,SlotSpace};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();
 let mut a=[7,0x80000001];let b=[u32::MAX-1];
 fn definitions<'a>(a:&'a[u32],b:&'a[u32])->Vec<Definition<'a>>{vec![Definition {key:1,rows:a,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:b,faces:60..63,pipes:0..0,ribbons:0..0}]}
 let change=space.reset(&definitions(&a,&b),100);assert!(table.publish(&ctx,&space,&change));let texture=table.texture.clone();let bytes=read(&ctx,&table);assert_eq!(&bytes[..4],&[[3,100,0,0],[7,100,10,2],[0x80000001,102,10,2],[u32::MAX-1,104,20,1]]);
 a[0]=0x80000007;let change=space.update(&definitions(&a,&b),&[(1,0)],100);assert!(!change.all);assert!(!table.publish(&ctx,&space,&change));assert_eq!(table.texture,texture);let next=read(&ctx,&table);assert_eq!(next[1],[0x80000007,100,10,2]);assert_eq!(&next[2..],&bytes[2..]);
 let remaining=[Definition {key:2,rows:&b,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);table.publish(&ctx,&space,&change);let next=read(&ctx,&table);assert_eq!(next[1][3],0);assert_eq!(next[2][3],0);assert_eq!(next[3],[u32::MAX-1,104,20,1]);
 let change=space.reset(&[],0);assert!(table.publish(&ctx,&space,&change));assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
}

#[test]
fn actual_slot_shader_resolves_arena_instances_holes_high_rows_and_empty_table() {
 use wgpu::util::DeviceExt;use crate::slot_space::{Definition,SlotSpace};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();
 let rows=[7,0x80000001];let other=[u32::MAX-1];let definitions=[Definition {key:1,rows:&rows,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:&other,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.reset(&definitions,100);table.publish(&ctx,&space,&change);
 let source=format!("@group(0) @binding(0) var slot_table: texture_2d<u32>;\n@group(0) @binding(1) var<storage,read_write> output: array<vec2<u32>>;\n{}\n@compute @workgroup_size(1) fn run(@builtin(global_invocation_id) p: vec3<u32>) {{ let ids=array<u32,8>(0u,100u,101u,102u,103u,104u,105u,106u);output[p.x]=placed_triangle(ids[p.x],100u); }}",include_str!("primitive_slot_lookup.wgsl"));
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("actual integer slot consumer"),source:wgpu::ShaderSource::Wgsl(source.into())});let pipeline=ctx.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {label:None,layout:None,module:&shader,entry_point:Some("run"),compilation_options:Default::default(),cache:None});
 let output=ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:None,contents:&[0;64],usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC});let mapped=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:None,size:64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});
 let resolve=|table:&Table| {let group=ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {label:None,layout:&pipeline.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry {binding:0,resource:wgpu::BindingResource::TextureView(&table.view)},wgpu::BindGroupEntry {binding:1,resource:output.as_entire_binding()}]});let mut encoder=ctx.device.create_command_encoder(&Default::default());{let mut pass=encoder.begin_compute_pass(&Default::default());pass.set_pipeline(&pipeline);pass.set_bind_group(0,&group,&[]);pass.dispatch_workgroups(8,1,1);}encoder.copy_buffer_to_buffer(&output,0,&mapped,0,64);ctx.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();mapped.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=mapped.slice(..).get_mapped_range();let result=bytes.chunks_exact(8).map(|v|[u32::from_le_bytes(v[..4].try_into().unwrap()),u32::from_le_bytes(v[4..].try_into().unwrap())]).collect::<Vec<_>>();drop(bytes);mapped.unmap();result};
 let none=[u32::MAX;2];assert_eq!(resolve(&table),vec![none,[99,u32::MAX],[10,7],[11,7],[10,0x80000001],[11,0x80000001],[20,u32::MAX-1],none]);
 let mut cross=vec![[0,100,0,0];1026];cross[0]=[1025,100,0,0];cross[1024]=[0x80000001,100,30,2];cross[1025]=[u32::MAX-1,104,40,1];table.write(&ctx,&cross);assert_eq!(resolve(&table),vec![none,[99,u32::MAX],[30,0x80000001],[31,0x80000001],none,none,[40,u32::MAX-1],none]);table.write(&ctx,&(0..space.end()).map(|i|space.texel(i)).collect::<Vec<_>>());
 let remaining=[Definition {key:2,rows:&other,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);table.publish(&ctx,&space,&change);let values=resolve(&table);assert_eq!(&values[2..6],&[none;4]);assert_eq!(values[6],[20,u32::MAX-1]);
 let change=space.reset(&[],0);table.publish(&ctx,&space,&change);let values=resolve(&table);assert_eq!(values[1],[99,u32::MAX]);assert_eq!(values[0],none);assert!(values[2..].iter().all(|v|*v==none));
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzl-prepare-one-plan-for-draw-slots-and-lookup-02.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzm-prepare-one-owner-for-live-address-updates"></a>

<details>
<summary>Implementation 96zzzzm-prepare-one-owner-for-live-address-updates: Prepare one owner for live address updates</summary>

Prepare synchronized mesh and record updates without changing primitive addresses.

Data flow: Live mesh slice plus membership → one address owner → measured allocation parts.

### 1. `src/instance_gpu.rs`

Pass the live meshes to their record owner so the next connection can synchronize addresses before preparing consumers. Keep the same instance membership while accepting the complete mesh slice. Retain the existing ordinary-row capacity fallback.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set<'a>(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,rows:impl IntoIterator<Item=&'a GpuMesh>,ordinary_rows:impl IntoIterator<Item=&'a GpuMesh>)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in rows {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=ordinary_rows.into_iter().collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
```

</details>

Replace it with:

```rust
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,_geometry_changed:bool)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,_geometry_changed:bool)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);space.update(&definitions,&touched,arena);
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-01.rs"
```

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-02.rs"
```

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-03.rs"
```

</details>

### 2. `src/renderer.rs`

Distinguish source membership updates from style changes before preparing projection. Keep style-only updates distinguishable so explicit primitive addresses remain intact.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id)))?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
```

</details>

Replace it with:

```rust
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
```

<details>
<summary>Locate this block</summary>

```rust
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,self.meshes.iter().filter(|row|self.instance_rows.contains(&row.id)),self.meshes.iter().filter(|row|!self.instance_rows.contains(&row.id))).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
```

</details>

Replace it with:

```rust
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
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
    primitive:crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,
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
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&crate::clipping_gpu::layout(&device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
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
        let primitive=crate::primitive_image::Image::new(&device,[640,480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
        renderer.highlight.enable_finite(&renderer.device,format);
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.picker.cancel(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive=crate::primitive_image::Image::new(&self.device,self.size);
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

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        let planes=crate::clipping_rows::owners(scene);let flags=self.sources.mesh_ids.iter().map(|id|crate::sub_id::decode(id[1]).and_then(|(_,address)|self.sources.edges.get(address)).map_or(0,|target|if planes.contains(&target.owner){4096}else{0})).collect();self.mesh_ink.set_flags(&self.device,flags).expect("Original plane ink flags match source addresses");
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
                let geometry = self.geometry.get(&crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue), &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let mut base=0u32;
        for mesh in &mut self.meshes{
            mesh.set_primitive_base(&self.queue,base).expect("Bounded projected triangle base");
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
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
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame(&self.queue,transform,self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame(&self.queue,transform,self.size[1],self.density);
            let mut bytes=transform.iter().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();bytes.resize(80,0);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
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

    pub fn usage(&self)->[u64;4]{let mut usage=crate::memory::gpu(&self.meshes);usage[1]+=self.geometry.allocated_bytes();usage}
    pub fn arena_cpu_bytes(&self)->usize{self.geometry.cpu_bytes()}

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        if settings==self.grid_settings&&plane==self.grid_plane{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on(plane)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{if self.measurement.mark.as_ref()==mark{return Ok(())}self.measurement.set(&self.device,mark)?;self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
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
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:[0.;3],framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
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

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::camera(camera));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.picker.cancel();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid]{lane.set_opacity(value);}self.highlight.set_opacity(value);if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));owner.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}else if let Some(owner)=ambient.as_mut(){owner.clear(&self.device,&self.queue);}}
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
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame(&self.queue,*transform,self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        let display = [if normals { 1.0f32 } else { 0.0 }, self.opacity.get(), 0.0, 0.0];
        bytes.extend(display.iter().flat_map(|value| value.to_ne_bytes()));
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        self.measurement.view(&self.queue,transform,self.size,self.density);self.guides.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid]{lane.finite_frame(&self.device,&tables,&self.depth);}self.highlight.finite_frame(&self.device,&tables,&self.depth);
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
                }),Some(wgpu::RenderPassColorAttachment{
                    view:&self.primitive.view,depth_slice:None,resolve_target:None,
                    ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
                })],
                ..Default::default()
            });
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.highlight.points(&mut pass);
            self.preview.draw(&mut pass);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);self.guides.draw(&mut pass);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
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
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

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
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-04.rs"
```

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-05.rs"
```

</details>

### 3. `src/instance_gpu.rs`

Expose the existing allocation owners individually for the upcoming sparse allocation check. Derive the total from the same measured allocations instead of repeating the accounting.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.records.allocated_bytes()+if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.writes,self.written_bytes]}
}

#[cfg(test)]
```

</details>

Replace it with:

```rust
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;2] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()}]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}

#[cfg(test)]
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,_geometry_changed:bool)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);space.update(&definitions,&touched,arena);
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;2] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()}]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-06.rs"
```

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-07.rs"
```

</details>

### 4. `src/renderer.rs`

Make the two existing GPU allocation parts available through renderer diagnostics.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
```

</details>

After that block, type:

```rust
    pub fn instance_allocations(&self)->[u64;2] {self.instances.allocation_parts()}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-08.rs"
```

</details>

### 5. `src/browser.rs`

Expose measured allocation parts beside the existing browser GPU diagnostic.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-instance-gpu",&serde_json::to_string(&renderer.instance_usage()).unwrap())?;
```

</details>

After that block, type:

```rust
    canvas.set_attribute("data-instance-allocations",&serde_json::to_string(&renderer.instance_allocations()).unwrap())?;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-09.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Supply exact allocation parts alongside the unchanged two-definition GPU pixels, slots, selection and release checks.

<details>
<summary>Locate the existing block</summary>

```text
assert_eq!(&gpu.instance_usage()[..3],&[2,2,376]);
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzm-prepare-one-owner-for-live-address-updates-10.rs" skip=50
```

</details>

</details>

<a id="checkpoint-96zzzzn-adopt-retained-gpu-slots-and-physical-bases"></a>

<details>
<summary>Implementation 96zzzzn-adopt-retained-gpu-slots-and-physical-bases: Adopt retained GPU slots and physical bases</summary>

Draw retained sparse slots with synchronized original uniforms and shared records.

Data flow: Retained runs → synchronized records and uniforms → coherent sparse GPU slots and lookup.

### 1. `src/instance_gpu.rs`

Retain the inverse lookup under the same live surface owner. Allocate the lookup only when a complete shared plan is active. Use source versus style updates to preserve explicit primitive addresses. Keep style-only explicit address overrides on the earlier path. Activate inverse lookup only for a complete supported shared table. Retain the allocator change report for the GPU writers. Preserve existing fallback behavior when the complete plan cannot be used. Publish the same retained change that assigned physical intervals. Synchronize final mesh uniforms and shared records before any consumer, using sparse locations or unchanged legacy addresses. Use the existing coherent GPU publication, with full resource release on the legacy or empty path.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,_geometry_changed:bool)->Result<(),&'static str> {
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let own=if own.len()<=limit-count {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);space.update(&definitions,&touched,arena);
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;2] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()}]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) lookup:Option<crate::primitive_slot_table::Table>,pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {lookup:None,space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,geometry_changed:bool)->Result<(),&'static str> {
        let compatible=geometry_changed||self.space.as_ref().is_some_and(|space|meshes.iter().all(|mesh|self.ordinary.get(&mesh.id).is_some_and(|(slot,row)|*slot>0&&*slot<space.end()&&space.local_slot(*slot)==[*row,mesh.primitive_base()])));
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let complete=own.len()<=limit-count;let own=if complete {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut change=None;let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|complete&&compatible&&crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);change=Some(space.update(&definitions,&touched,arena));
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        if let Some(space)=&self.space {let locations=space.draws().into_iter().flat_map(|draw|draw.slots).map(|slot|{let[row,base]=space.local_slot(slot);(row,(slot,base))}).collect::<std::collections::HashMap<_,_>>();for(first,row)in ordinary.values_mut(){*first=locations[&*row].0;}for batch in batches.values_mut(){batch.first_slot=locations[&all_slots[(batch.first_slot-1)as usize][0]].0;}}
        let mut dense=0u32;for mesh in meshes {let entry=ordinary.get(&mesh.id);let base=if let(Some(space),Some((first,_)))=(&self.space,entry){space.local_slot(*first)[1]}else if geometry_changed {dense}else{mesh.primitive_base()};mesh.set_primitive_base(queue,base)?;if let Some((first,row))=entry {all_records[*row as usize]=mesh.packed();if self.space.is_none(){all_slots[(*first-1)as usize]=[*row,base];}}dense=dense.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).ok_or("Primitive range overlaps reserved caps")?;}
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if let(Some(space),Some(change))=(&self.space,&change){let table=self.lookup.get_or_insert_with(||crate::primitive_slot_table::Table::new(&ctx));self.slots.publish(&ctx,space,change,table);let bytes=table.written_bytes.get();if bytes>0{self.writes+=1;self.written_bytes+=bytes;}}else{self.lookup=None;if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;2] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()}]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-01.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-02.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-03.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-04.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-05.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-06.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-07.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-08.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-09.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-10.rs"
```

</details>

### 2. `src/primitive_slot_table.rs`

Measure actual lookup upload bytes, including padded full writes. Begin with zero lookup upload bytes. Count only validated texture writes. Measure each publication independently.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::text_gpu_support::GpuCtx;
pub const WIDTH:u32=1024;
pub struct Table {pub(crate) texture:wgpu::Texture,pub view:wgpu::TextureView}
fn dimensions(count:u32)->(u32,u32){if count<=1{(1,1)}else{(WIDTH,count.div_ceil(WIDTH))}}
fn texture(ctx:&GpuCtx,width:u32,height:u32)->wgpu::Texture{
 ctx.device.create_texture(&wgpu::TextureDescriptor {label:Some("primitive slot lookup"),size:wgpu::Extent3d {width,height,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba32Uint,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_DST|wgpu::TextureUsages::COPY_SRC,view_formats:&[]})
}
impl Table {
 pub fn new(ctx:&GpuCtx)->Self{let texture=texture(ctx,1,1);let view=texture.create_view(&Default::default());Self {texture,view}}
 pub fn write(&mut self,ctx:&GpuCtx,texels:&[[u32;4]]){
  let count=u32::try_from(texels.len()).expect("Bounded slot table");let(width,height)=dimensions(count);
  assert!(height<=ctx.device.limits().max_texture_dimension_2d);
  if self.texture.width()!=width||self.texture.height()!=height{self.texture=texture(ctx,width,height);self.view=self.texture.create_view(&Default::default());}
  let mut data=texels.to_vec();data.resize((width*height)as usize,[0;4]);self.write_at(ctx,0,&data);
 }
 pub fn write_at(&self,ctx:&GpuCtx,first:u32,texels:&[[u32;4]])->bool{
  let width=self.texture.width();if u64::from(first)+texels.len()as u64>u64::from(width)*u64::from(self.texture.height()){return false;}
  let mut at=first;let mut rest=texels;
  while !rest.is_empty(){
   let(x,y)=(at%width,at/width);let(row,tail)=rest.split_at(((width-x)as usize).min(rest.len()));let bytes:Vec<u8>=row.iter().flatten().flat_map(|v|v.to_le_bytes()).collect();
   ctx.queue.write_texture(wgpu::TexelCopyTextureInfo {origin:wgpu::Origin3d {x,y,z:0},..self.texture.as_image_copy()},&bytes,wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(row.len()as u32*16),rows_per_image:Some(1)},wgpu::Extent3d {width:row.len()as u32,height:1,depth_or_array_layers:1});
   at+=row.len()as u32;rest=tail;
  }true
 }
 pub fn publish(&mut self,ctx:&GpuCtx,space:&crate::slot_space::SlotSpace,change:&crate::slot_space::Change)->bool {
  let previous=self.texture.clone();let (width,height)=dimensions(space.end());
  if change.all||self.texture.width()!=width||self.texture.height()!=height {
   let texels=(0..space.end()).map(|slot|space.texel(slot)).collect::<Vec<_>>();self.write(ctx,&texels);
  }else {
   for range in &change.slots {
    assert!(range.start<=range.end&&range.end<=space.end());
    let texels=range.clone().map(|slot|space.texel(slot)).collect::<Vec<_>>();assert!(self.write_at(ctx,range.start,&texels));
   }
  }
  self.texture!=previous
 }
 pub fn allocated_bytes(&self)->u64{u64::from(self.texture.width())*u64::from(self.texture.height())*16}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::text_gpu_support::GpuCtx;
pub const WIDTH:u32=1024;
pub struct Table {pub written_bytes:std::cell::Cell<u64>,pub(crate) texture:wgpu::Texture,pub view:wgpu::TextureView}
fn dimensions(count:u32)->(u32,u32){if count<=1{(1,1)}else{(WIDTH,count.div_ceil(WIDTH))}}
fn texture(ctx:&GpuCtx,width:u32,height:u32)->wgpu::Texture{
 ctx.device.create_texture(&wgpu::TextureDescriptor {label:Some("primitive slot lookup"),size:wgpu::Extent3d {width,height,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba32Uint,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_DST|wgpu::TextureUsages::COPY_SRC,view_formats:&[]})
}
impl Table {
 pub fn new(ctx:&GpuCtx)->Self{let texture=texture(ctx,1,1);let view=texture.create_view(&Default::default());Self {written_bytes:Default::default(),texture,view}}
 pub fn write(&mut self,ctx:&GpuCtx,texels:&[[u32;4]]){
  let count=u32::try_from(texels.len()).expect("Bounded slot table");let(width,height)=dimensions(count);
  assert!(height<=ctx.device.limits().max_texture_dimension_2d);
  if self.texture.width()!=width||self.texture.height()!=height{self.texture=texture(ctx,width,height);self.view=self.texture.create_view(&Default::default());}
  let mut data=texels.to_vec();data.resize((width*height)as usize,[0;4]);self.write_at(ctx,0,&data);
 }
 pub fn write_at(&self,ctx:&GpuCtx,first:u32,texels:&[[u32;4]])->bool{
  let width=self.texture.width();if u64::from(first)+texels.len()as u64>u64::from(width)*u64::from(self.texture.height()){return false;}
  self.written_bytes.set(self.written_bytes.get()+texels.len()as u64*16);
  let mut at=first;let mut rest=texels;
  while !rest.is_empty(){
   let(x,y)=(at%width,at/width);let(row,tail)=rest.split_at(((width-x)as usize).min(rest.len()));let bytes:Vec<u8>=row.iter().flatten().flat_map(|v|v.to_le_bytes()).collect();
   ctx.queue.write_texture(wgpu::TexelCopyTextureInfo {origin:wgpu::Origin3d {x,y,z:0},..self.texture.as_image_copy()},&bytes,wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(row.len()as u32*16),rows_per_image:Some(1)},wgpu::Extent3d {width:row.len()as u32,height:1,depth_or_array_layers:1});
   at+=row.len()as u32;rest=tail;
  }true
 }
 pub fn publish(&mut self,ctx:&GpuCtx,space:&crate::slot_space::SlotSpace,change:&crate::slot_space::Change)->bool {
  self.written_bytes.set(0);let previous=self.texture.clone();let (width,height)=dimensions(space.end());
  if change.all||self.texture.width()!=width||self.texture.height()!=height {
   let texels=(0..space.end()).map(|slot|space.texel(slot)).collect::<Vec<_>>();self.write(ctx,&texels);
  }else {
   for range in &change.slots {
    assert!(range.start<=range.end&&range.end<=space.end());
    let texels=range.clone().map(|slot|space.texel(slot)).collect::<Vec<_>>();assert!(self.write_at(ctx,range.start,&texels));
   }
  }
  self.texture!=previous
 }
 pub fn allocated_bytes(&self)->u64{u64::from(self.texture.width())*u64::from(self.texture.height())*16}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-11.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-12.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-13.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-14.rs"
```

</details>

### 3. `src/instance_gpu.rs`

Include the actual inverse lookup allocation in the same accounting. Count lookup bytes once alongside existing record and slot buffers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;2] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()}]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}
```

</details>

Replace it with:

```rust
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;3] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.lookup.as_ref().map_or(0,|table|table.allocated_bytes())]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh};
use std::{collections::BTreeMap,rc::Rc};
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) lookup:Option<crate::primitive_slot_table::Table>,pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
 let candidates=rows.iter().filter(|(_,key,declared)|*declared||uses[key]>1).map(|(id,_,_)|*id).collect::<std::collections::HashSet<_>>();
 if candidates.len()<=limit {candidates}else{rows.into_iter().filter(|(_,_,declared)|*declared).map(|(id,_,_)|id).collect()}
}
pub(crate) fn stable_rows(previous:&std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,ids:&[crate::scene::ObjectId],limit:usize)->Result<std::collections::HashMap<crate::scene::ObjectId,u32>,&'static str> {
 let mut rows=std::collections::HashMap::new();let mut used=std::collections::HashSet::new();
 for id in ids {if let Some(&(_,row))=previous.get(id) {if row as usize>=limit||!used.insert(row){return Err("Invalid retained object row");}rows.insert(*id,row);}}
 let mut next=0usize;
 for id in ids {if rows.contains_key(id){continue;}while next<limit&&used.contains(&(next as u32)){next+=1;}if next>=limit||u32::try_from(next).is_err(){return Err("Object records exceed device limits");}let row=next as u32;rows.insert(*id,row);used.insert(row);}
 Ok(rows)
}
pub(crate) fn source()->String {
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {lookup:None,space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,geometry_changed:bool)->Result<(),&'static str> {
        let compatible=geometry_changed||self.space.as_ref().is_some_and(|space|meshes.iter().all(|mesh|self.ordinary.get(&mesh.id).is_some_and(|(slot,row)|*slot>0&&*slot<space.end()&&space.local_slot(*slot)==[*row,mesh.primitive_base()])));
        let limit=crate::object_rows::capacity(device);
        if limit==0 {return Err("Device cannot hold one instance record");}
        let mut groups:BTreeMap<usize,(Rc<GpuGeometry>,Vec<(crate::scene::ObjectId,Record)>)>=BTreeMap::new();
        for row in meshes.iter().filter(|row|instances.contains(&row.id)) {let key=Rc::as_ptr(&row.geometry) as usize;groups.entry(key).or_insert_with(||(row.geometry.clone(),Vec::new())).1.push((row.id,row.packed()));}
        let count=groups.values().try_fold(0usize,|n,(_,rows)|n.checked_add(rows.len())).ok_or("Instance record count")?;
        if count>limit{return Err("Shared instance records exceed device limits");}
        let own=meshes.iter().filter(|row|!instances.contains(&row.id)).collect::<Vec<_>>();let complete=own.len()<=limit-count;let own=if complete {own}else{Vec::new()};
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        let indices=stable_rows(&self.ordinary,&ids,limit)?;
        let mut change=None;let members=crate::instance_members::collect(groups.values().map(|(geometry,records)|(geometry,records.iter().map(|(id,_)|indices[id]).collect())).chain(own.iter().map(|mesh|(&mesh.geometry,vec![indices[&mesh.id]]))));
        if ids.is_empty(){self.space=None;}else if let Some((members,arena))=members.filter(|(members,arena)|complete&&compatible&&crate::instance_members::bounded(members,self.space.as_ref().and_then(|space|space.triangle_count()).unwrap_or(*arena).max(*arena))) {
            let definitions=members.iter().map(crate::instance_members::Members::definition).collect::<Vec<_>>();let space=self.space.get_or_insert_with(Default::default);let touched=space.touched(&definitions);change=Some(space.update(&definitions,&touched,arena));
        }else{self.space=None;}
        let mut batches=BTreeMap::new();let mut ordinary=std::collections::HashMap::new();let mut all_slots=Vec::new();let mut all_records=vec![[0;BYTES];indices.values().max().map_or(0,|row|*row as usize+1)];
        for (source,(geometry,records)) in groups {
            let first_slot=u32::try_from(all_slots.len()+1).map_err(|_|"Instance slot range")?;
            for (id,record) in &records {
                let row=indices[id];
                all_slots.push([row,u32::from_ne_bytes(record[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())]);
                ordinary.insert(*id,(u32::try_from(all_slots.len()).map_err(|_|"Instance slot range")?,row));
                all_records[row as usize]=*record;
            }
            batches.insert(source,Batch {geometry,count:records.len()as u32,first_slot});
        }
        for mesh in own {
            let first=u32::try_from(all_slots.len()+1).map_err(|_|"Ordinary slot range")?;let row=indices[&mesh.id];
            all_slots.push([row,mesh.primitive_base()]);all_records[row as usize]=mesh.packed();ordinary.insert(mesh.id,(first,row));
        }
        if let Some(space)=&self.space {let locations=space.draws().into_iter().flat_map(|draw|draw.slots).map(|slot|{let[row,base]=space.local_slot(slot);(row,(slot,base))}).collect::<std::collections::HashMap<_,_>>();for(first,row)in ordinary.values_mut(){*first=locations[&*row].0;}for batch in batches.values_mut(){batch.first_slot=locations[&all_slots[(batch.first_slot-1)as usize][0]].0;}}
        let mut dense=0u32;for mesh in meshes {let entry=ordinary.get(&mesh.id);let base=if let(Some(space),Some((first,_)))=(&self.space,entry){space.local_slot(*first)[1]}else if geometry_changed {dense}else{mesh.primitive_base()};mesh.set_primitive_base(queue,base)?;if let Some((first,row))=entry {all_records[*row as usize]=mesh.packed();if self.space.is_none(){all_slots[(*first-1)as usize]=[*row,base];}}dense=dense.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).ok_or("Primitive range overlaps reserved caps")?;}
        let ctx=crate::text_gpu_support::GpuCtx::new(device,queue);
        self.records.set(&ctx,&self.layout,&all_records)?;
        if self.records.written_bytes>0{self.writes+=1;self.written_bytes+=self.records.written_bytes;}
        if let(Some(space),Some(change))=(&self.space,&change){let table=self.lookup.get_or_insert_with(||crate::primitive_slot_table::Table::new(&ctx));self.slots.publish(&ctx,space,change,table);let bytes=table.written_bytes.get();if bytes>0{self.writes+=1;self.written_bytes+=bytes;}}else{self.lookup=None;if all_slots.is_empty()&&self.slots.instances()>0{self.slots.clear(&ctx,true);}else{self.slots.set(&ctx,&all_slots);}}
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
    pub fn current_row(&self,mesh:&GpuMesh)->Option<(u32,u32)> {
        let &(first,row)=self.ordinary.get(&mesh.id)?;self.records.matches(row,&mesh.packed()).then_some((first,row))
    }
    pub fn bind_record(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {
        let Some((first,_))=self.current_row(mesh)else{return false};self.records.bind(pass);self.slots.bind(pass,0,first);true
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
    pub(crate) fn record_buffer(&self)->&wgpu::Buffer{&self.records.buffer.buf}
    pub fn allocation_parts(&self)->[u64;3] {[self.records.allocated_bytes(),if self.slots.instances()==0{0}else{self.slots.allocated_bytes()},self.lookup.as_ref().map_or(0,|table|table.allocated_bytes())]}
    pub fn usage(&self)->[u64;5] {[self.batches.len() as u64,self.batches.values().map(|b|u64::from(b.count)).sum(),self.allocation_parts().into_iter().sum(),self.writes,self.written_bytes]}
}

#[cfg(test)]
mod stable_row_tests {
 use super::*;
 #[test]
 fn retained_ids_reuse_holes_and_reject_capacity_overflow() {
  let mut scene=crate::scene::Scene::empty();let mesh=||crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap();let a=scene.insert(mesh()).unwrap();let b=scene.insert(mesh()).unwrap();let c=scene.insert(mesh()).unwrap();
  let old=std::collections::HashMap::from([(a,(1,0)),(b,(2,1)),(c,(3,2))]);let rows=stable_rows(&old,&[c,a,b],3).unwrap();assert_eq!((rows[&a],rows[&b],rows[&c]),(0,1,2));
  let d=scene.insert(mesh()).unwrap();let rows=stable_rows(&old,&[c,d,a],3).unwrap();assert_eq!((rows[&a],rows[&d],rows[&c]),(0,1,2));assert!(!rows.contains_key(&b));assert!(stable_rows(&old,&[a,b,c,d],3).is_err());assert!(stable_rows(&old,&[c],2).is_err());assert!(stable_rows(&old,&[],0).unwrap().is_empty());
 }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-15.rs"
```

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-16.rs"
```

</details>

### 4. `src/renderer.rs`

Expose all three measured allocation owners. The lane now synchronizes addresses once before record upload and projection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;2] {self.instances.allocation_parts()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }
```

</details>

Replace it with:

```rust
    pub fn tile_usage(&self)->[u64;7]{self.tiles.borrow().usage()}
    pub fn sheet_usage(&self)->[u64;5] {self.sheets.usage()}
    pub fn instance_usage(&self)->[u64;5] {self.instances.usage()}
    pub fn instance_allocations(&self)->[u64;3] {self.instances.allocation_parts()}
    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }
```

<details>
<summary>Locate this block</summary>

```rust
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let mut base=0u32;
        for mesh in &mut self.meshes{
            mesh.set_primitive_base(&self.queue,base).expect("Bounded projected triangle base");
            base=base.checked_add(mesh.geometry.index_count/3).filter(|n|*n<crate::primitive_image::CAP_START).expect("Bounded projected triangle range");
        }
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
```

</details>

Replace it with:

```rust
        drop(old);
        self.caps.borrow_mut().set_scene(&self.queue,scene,&mut self.meshes,&self.clipping.borrow().rows);
        for mesh in &mut self.meshes{mesh.set_xray(&self.queue,self.opacity.get()==0.);}
        let radii=self.ambient_rows.specs.iter().copied().collect::<std::collections::HashMap<_,_>>();for mesh in &mut self.meshes {mesh.set_ambient_radius(&self.queue,radii.get(&mesh.id.value()).copied().unwrap_or(0.));}
        self.instance_rows=crate::instance_gpu::batch_rows(self.meshes.iter().map(|row|(row.id,std::rc::Rc::as_ptr(&row.geometry)as usize,scene.instances.contains_key(&row.id))),crate::object_rows::capacity(&self.device));
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,true).expect("Validated instance records fit device limits");
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
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
    primitive:crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    grid_plane:crate::cplane::CPlane,
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
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&crate::clipping_gpu::layout(&device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
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
        let primitive=crate::primitive_image::Image::new(&device,[640,480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        for lane in [&mut renderer.mesh_ink,&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers,&mut renderer.grid]{lane.enable_finite(&renderer.device,format);}
        renderer.highlight.enable_finite(&renderer.device,format);
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.picker.cancel(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive=crate::primitive_image::Image::new(&self.device,self.size);
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

    #[cfg(test)]
    pub(crate) fn use_uniform_surface_records(&mut self) {self.instances.ordinary.clear();}

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        let planes=crate::clipping_rows::owners(scene);let flags=self.sources.mesh_ids.iter().map(|id|crate::sub_id::decode(id[1]).and_then(|(_,address)|self.sources.edges.get(address)).map_or(0,|target|if planes.contains(&target.owner){4096}else{0})).collect();self.mesh_ink.set_flags(&self.device,flags).expect("Original plane ink flags match source addresses");
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
                let geometry = self.geometry.get(&crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue), &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
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
        self.sheets.set(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases).expect("Prepared original sheet batches fit device limits");
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
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame(&self.queue,transform,self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame(&self.queue,transform,self.size[1],self.density);
            let mut bytes=transform.iter().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();bytes.resize(80,0);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
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

    pub fn usage(&self)->[u64;4]{let mut usage=crate::memory::gpu(&self.meshes);usage[1]+=self.geometry.allocated_bytes();usage}
    pub fn arena_cpu_bytes(&self)->usize{self.geometry.cpu_bytes()}

    pub fn uploaded_bytes(&self) -> u64 {
        self.instances.written_bytes + self.geometry.uploaded_bytes + self.settings_allocations as u64 * crate::object_record::BYTES as u64 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self,settings:Option<crate::grid::Settings>)->Result<(),&'static str>{self.set_grid_on(settings,crate::cplane::CPlane::Xy)}
    pub fn set_grid_on(&mut self,settings:Option<crate::grid::Settings>,plane:crate::cplane::CPlane)->Result<(),&'static str>{
        if settings==self.grid_settings&&plane==self.grid_plane{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on(plane)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases)?;
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{if self.measurement.mark.as_ref()==mark{return Ok(())}self.measurement.set(&self.device,mark)?;self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
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
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:[0.;3],framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
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

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::camera(camera));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.picker.cancel();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid]{lane.set_opacity(value);}self.highlight.set_opacity(value);if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));owner.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}else if let Some(owner)=ambient.as_mut(){owner.clear(&self.device,&self.queue);}}
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
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame(&self.queue,*transform,self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        let display = [if normals { 1.0f32 } else { 0.0 }, self.opacity.get(), 0.0, 0.0];
        bytes.extend(display.iter().flat_map(|value| value.to_ne_bytes()));
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        self.measurement.view(&self.queue,transform,self.size,self.density);self.guides.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers,&self.grid]{lane.finite_frame(&self.device,&tables,&self.depth);}self.highlight.finite_frame(&self.device,&tables,&self.depth);
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
                }),Some(wgpu::RenderPassColorAttachment{
                    view:&self.primitive.view,depth_slice:None,resolve_target:None,
                    ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},
                })],
                ..Default::default()
            });
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.primitive.view,&self.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_bind_group(2,&clip_group,&[]);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.highlight.points(&mut pass);
            self.preview.draw(&mut pass);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass);self.guides.draw(&mut pass);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
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
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground,[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.face_rgba(id)}}

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
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-17.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Include the actual one-row 1024-texel lookup in exact two-definition allocation accounting.

<details>
<summary>Locate the existing block</summary>

```text
assert_eq!(&gpu.instance_usage()[..3],&[2,2,376]);assert_eq!(gpu.instance_allocations(),[352,24]);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-19.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Keep the original projection relocation fixture explicitly based at zero, preserving every gap, byte, shrink and fallback assertion.

<details>
<summary>Locate the existing block</summary>

```text
let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-20.rs"
```

</details>

### 7. `src/slot_space.rs`

Report newly active retained members even when their row number is unchanged; inverse lookup counts must refresh after hide/show.

<details>
<summary>Locate the existing block</summary>

```rust
    let mut out=Vec::new();for definition in definitions {let Some(run)=self.runs.iter().find(|run|run.key==Some(definition.key))else{continue};for(position,row)in definition.rows.iter().enumerate(){if position<run.cap as usize&&self.rows.get(run.start as usize+position)!=Some(row){out.push((definition.key,position as u32));}}}out
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut out=Vec::new();for definition in definitions {let Some(run)=self.runs.iter().find(|run|run.key==Some(definition.key))else{continue};for(position,row)in definition.rows.iter().enumerate(){if position<run.cap as usize&&(position>=run.len as usize||self.rows.get(run.start as usize+position)!=Some(row)){out.push((definition.key,position as u32));}}}out
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-21.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/pick_gpu_tests.rs`

Retain all original copied/placed/nested owner, pixel, history and release assertions while adding the actual padded 16384-byte inverse lookup to exact allocation totals.

<details>
<summary>Locate the existing block</summary>

```text
use super::*;
use crate::{renderer::Renderer, viewport::Viewport};

fn renderer(scene: &Scene) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::VULKAN, flags: Default::default(), memory_budget_thresholds: Default::default(), backend_options: Default::default(), display: None });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).expect("Actual GPU adapter");
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).expect("Actual GPU device");
    let mut renderer = Renderer::new(device, queue, wgpu::TextureFormat::Rgba8Unorm, scene);
    renderer.resize(Viewport { width: 320, height: 240 }); renderer
}

fn finish(renderer: &mut Renderer) -> Option<[u32; 2]> {
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    renderer.take_pick(identity()).expect("Map callback completed").unwrap()
}

fn identity() -> [f32; 16] { session_rust::Xform::identity().m.map(|n| n as f32) }
fn triangle(z: f64) -> PreparedMesh {
    PreparedMesh::triangle([[-0.8, -0.8, z], [0.8, -0.8, z], [0.0, 0.8, z]], [0.8, 0.2, 0.1]).unwrap()
}


#[test]
fn actual_projected_feature_pixel_selects_original_element_not_display_path() {
    let mut scene = Scene::empty();
    let mut element = session_rust::Element::new("feature pick");
    element.set_geometry(session_rust::Mesh::create_box(0.2,0.2,0.2));
    let outline = session_rust::Polyline::new(vec![session_rust::Point::new(-0.8,0.4,0.2),session_rust::Point::new(0.8,0.4,0.2)]);
    element.add_feature(session_rust::element::ElementFeature::new("cut", 0, vec![outline], "pick line"));
    let original = Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, original.clone()).unwrap();
    crate::feature_display::append(&mut scene,owner,&session_rust::Session::new("test"),&original).unwrap();
    assert_eq!(scene.paths().len(),1); let path=scene.paths()[0].id;
    let mut renderer=renderer(&scene); renderer.request_pick([0.,0.4],identity()).unwrap();
    let pixel=finish(&mut renderer).unwrap();assert_eq!(pixel,[owner.value(),0]);assert_ne!(pixel[0],path.value());
    assert_eq!(renderer.resolve_pick(&scene,pixel),Some(crate::pick_sources::Target {owner,part:crate::pick_sources::Part::Object}));
    Rc::make_mut(&mut scene.view).lock(&[owner],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.,0.4],identity()).unwrap();assert_eq!(finish(&mut renderer),None);
}

#[test]
fn actual_feature_point_disc_uses_original_owner_and_css_diameter() {
    let mut scene = Scene::empty();
    let mut element = session_rust::Element::new("feature dot");
    element.set_geometry(session_rust::Mesh::create_box(0.2, 0.2, 0.2));
    let mut dot = session_rust::Polyline::new(vec![session_rust::Point::new(0.5, 0.4, 0.2), session_rust::Point::new(0.5, 0.4, 0.2)]);
    dot.linecolor = session_rust::Color::new(0.9, 0.1, 0.1, 1.);
    element.add_feature(session_rust::element::ElementFeature::new("drill", -1, vec![dot], "dot"));
    let original = Rc::new(element);
    let owner = crate::element_original::insert(&mut scene, original.clone()).unwrap();
    crate::feature_display::append(&mut scene, owner, &session_rust::Session::new("dot"), &original).unwrap();
    assert_eq!(scene.points().len(), 1);
    let mut renderer = renderer(&scene);
    for density in [1., 2.] {
        renderer.resize(Viewport {width: 320, height: 240});
        renderer.set_density(density);
        let image = colour_pixels(&mut renderer);
        let pixels: Vec<_> = image.chunks_exact(4).enumerate().filter_map(|(index, pixel)| {
            let (x, y) = (index % 320, index / 320);
            (x > 210 && (40..105).contains(&y) && u16::from(pixel[0]) > 2 * u16::from(pixel[1])
                && u16::from(pixel[0]) > 2 * u16::from(pixel[2])).then_some((x, y))
        }).collect();
        assert!(!pixels.is_empty());
        for axis in [0, 1] {
            let values: Vec<_> = pixels.iter().map(|&(x, y)| if axis == 0 { x } else { y }).collect();
            let span = values.iter().max().unwrap() - values.iter().min().unwrap() + 1;
            assert!((span as f32 - 12. * density).abs() <= 1., "DPR {density}, axis {axis}, physical span {span}");
        }
        // Picking searches the already-taught six-CSS-pixel neighbourhood around the disc.
        for (offset, expected) in [(0., Some([owner.value(), 0])), (5., Some([owner.value(), 0])), (13., None)] {
            renderer.request_pick([0.5 + 2. * offset * density / 320., 0.4], identity()).unwrap();
            assert_eq!(finish(&mut renderer), expected, "DPR {density}, CSS offset {offset}");
        }
    }
}

#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion() {
    let mut scene = Scene::demo(); scene.clear(); scene.next_id = 16_777_217;
    let back = scene.insert(triangle(0.6)).unwrap(); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([back.value(), 0]));
    let front = scene.insert(triangle(0.2)).unwrap(); renderer.set_scene(&scene, None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([front.value(), 0]));
    Rc::make_mut(&mut scene.view).lock(&[front],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), None);
    assert_eq!(scene.pick_id(front.value()), None);
    Rc::make_mut(&mut scene.view).hide(&[front],true);renderer.set_scene(&scene,None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([back.value(), 0]));
    renderer.request_pick([0.95, 0.95], identity()).unwrap(); assert_eq!(finish(&mut renderer), None);
}

#[test]
fn actual_ink_pick_uses_coverage_depth_density_and_bounded_coalescing() {
    let mut scene = Scene::demo(); scene.clear();
    let mesh = scene.insert(triangle(0.5)).unwrap();
    let mut point = session_rust::Point::new(0.0, 0.0, 0.2); point.width = 12.0;
    let point = scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(point)).unwrap()).unwrap();
    let mut line = session_rust::Line::new(-0.4, 0.4, 0.2, 0.4, 0.4, 0.2); line.width = 5.0;
    let line = scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();
    let mut path = session_rust::Polyline::new(vec![session_rust::Point::new(-0.4, -0.4, 0.2), session_rust::Point::new(0.4, -0.4, 0.2)]);
    path.width = 5.0;
    let path = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(path)).unwrap()).unwrap();
    let mut renderer = renderer(&scene); let geometry = renderer.stats();
    for density in [1.0, 2.0] {
        renderer.set_density(density); renderer.resize(Viewport { width: (320.0 * density) as u32, height: (240.0 * density) as u32 });
        for (cursor, expected) in [([0.0, 0.0], point), ([0.0, 0.4], line), ([0.0, -0.4], path), ([0.0, 0.65], mesh)] {
            renderer.request_pick(cursor, identity()).unwrap(); assert_eq!(finish(&mut renderer).unwrap()[0], expected.value());
        }
    }
    assert_eq!(renderer.stats(), geometry, "Picking and resize reuse visible geometry");
    renderer.request_pick([0.0, 0.0], identity()).unwrap();
    for _ in 0..20 { renderer.request_pick([0.95, 0.95], identity()).unwrap(); }
    assert_eq!(&renderer.pick_usage()[..2], &[1, 1]); assert_eq!(renderer.pick_usage()[4], 19);
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none(), "A newer cursor supersedes the first result");
    assert_eq!(finish(&mut renderer), None);
    assert_eq!(&renderer.pick_usage()[..2], &[0, 0]);
    assert!(renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 16 + 192 + 3 * 52, "Readback does not grow with the full drawing");
}

#[test]
fn cancelled_readback_cannot_select_a_new_document_or_keep_its_source_alive() {
    let mut scene = Scene::demo(); scene.clear(); let old = scene.insert(triangle(0.2)).unwrap();
    let weak = Rc::downgrade(&scene.objects[0].mesh); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); scene.clear(); renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none()); assert_eq!(&renderer.pick_usage()[..3], &[0, 0, 160]);
    let new = scene.insert(triangle(0.3)).unwrap(); assert_ne!(old, new); renderer.set_scene(&scene, None);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert_eq!(finish(&mut renderer), Some([new.value(), 0]));
    assert!(renderer.take_pick(identity()).is_none(), "The cancelled map has no result for the replacement");
    let uploads = renderer.stats(); renderer.request_pick([0.0, 0.0], identity()).unwrap(); renderer.cancel_pick();
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none()); assert_eq!(renderer.stats(), uploads);
}

#[test]
fn a_real_readback_from_an_old_camera_cannot_select_in_the_current_view() {
    let mut scene = Scene::demo(); scene.clear(); scene.insert(triangle(0.2)).unwrap(); let mut renderer = renderer(&scene);
    renderer.request_pick([0.0, 0.0], identity()).unwrap();
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    let mut moved = identity(); moved[12] = 0.2;
    assert!(renderer.take_pick(moved).is_none(), "Same scene, different camera: no accepted hit");
    assert_eq!(&renderer.pick_usage()[..2], &[0, 0]);
    let submitted = renderer.pick_usage()[3]; let mut invalid = moved; invalid[0] = f32::NAN;
    assert!(renderer.request_pick([0.0, 0.0], invalid).is_err()); assert_eq!(renderer.pick_usage()[3], submitted);
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); assert!(finish(&mut renderer).is_some());
    renderer.request_pick([0.0, 0.0], identity()).unwrap(); renderer.set_density(2.0);
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    assert!(renderer.take_pick(identity()).is_none(), "Density changed while the map was pending");
}

#[test]
fn actual_face_and_edge_ids_preserve_sparse_source_keys_and_exclude_a_rendering_diagonal() {
    let mut source = session_rust::Mesh::new();
    for (key, p) in [(10, [-0.5, -0.5, 0.5]), (20, [0.5, -0.5, 0.5]), (30, [0.5, 0.5, 0.5]), (40, [-0.5, 0.5, 0.5])] {
        source.add_vertex(session_rust::Point::new(p[0], p[1], p[2]), Some(key));
    }
    source.add_face(vec![10, 20, 30, 40], Some(37)); source.set_face_triangulation(37, vec![[10, 20, 40], [20, 30, 40]]);
    source.set_facecolors(vec![session_rust::Color::new(0.8, 0.2, 0.1, 1.)]);
    let mut scene = Scene::demo(); scene.clear(); let id = scene.insert(PreparedMesh::new(source).unwrap()).unwrap(); let mut renderer = renderer(&scene);
    let before = renderer.stats(); renderer.set_pick_mode(crate::pick_sources::Mode::Face);
    renderer.request_pick([0., 0.], identity()).unwrap(); let hit = finish(&mut renderer).unwrap();
    assert_eq!(renderer.resolve_pick(&scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Face(37) }));
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge); renderer.request_pick([0., -0.5], identity()).unwrap(); let hit = finish(&mut renderer).unwrap();
    assert_eq!(renderer.resolve_pick(&scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::MeshEdge(10, 20) }));
    renderer.request_pick([0., 0.], identity()).unwrap(); assert!(finish(&mut renderer).is_none(), "The triangulation diagonal is not an editable edge");
    renderer.set_pick_mode(crate::pick_sources::Mode::Object); renderer.request_pick([0., 0.], identity()).unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value()); assert_eq!(renderer.stats(), before);
}

#[test]
fn actual_control_hits_resolve_the_original_curve_control_instead_of_a_sample() {
    let mut editor = crate::editor::Editor::default(); editor.apply(crate::editor::Action::Close).unwrap();
    editor.apply(crate::editor::Action::AddCurve).unwrap(); editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let id = editor.scene.paths()[0].id; let mut renderer = renderer(&editor.scene);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control); renderer.request_pick([0., 0.6], identity()).unwrap();
    let hit = finish(&mut renderer).unwrap(); assert_eq!(renderer.resolve_pick(&editor.scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Control(1) }));
    renderer.set_pick_mode(crate::pick_sources::Mode::Edge); renderer.request_pick([0., 0.1], identity()).unwrap();
    let hit = finish(&mut renderer).unwrap(); assert_eq!(renderer.resolve_pick(&editor.scene, hit), Some(crate::pick_sources::Target { owner: id, part: crate::pick_sources::Part::Curve }));
    editor.scene.show_controls(false); renderer.set_scene(&editor.scene, None);
    renderer.set_pick_mode(crate::pick_sources::Mode::Control); renderer.request_pick([0., 0.6], identity()).unwrap(); assert!(finish(&mut renderer).is_none());
}

fn colour_pixels(renderer: &mut Renderer) -> Vec<u8> {colour_pixels_at(renderer,identity())}
fn colour_pixels_at(renderer:&mut Renderer,transform:[f32;16])->Vec<u8> {colour_pixels_display(renderer,transform,false)}
fn colour_pixels_display(renderer:&mut Renderer,transform:[f32;16],normals:bool)->Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: 320, height: 240, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), &transform, normals);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: 320 * 240 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1280), rows_per_image: Some(240) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec(); bytes.unmap(); result
}

#[test]
fn actual_part_highlights_are_local_leave_black_edges_and_release_derived_data() {
    use crate::pick_sources::{Part, Target};
    let mut source = session_rust::Mesh::new();
    for (key, p) in [(10, [-0.8, -0.6, 0.5]), (20, [0., -0.6, 0.5]), (30, [0., 0.6, 0.5]), (40, [-0.8, 0.6, 0.5]),
                     (50, [0.8, -0.6, 0.5]), (60, [0.8, 0.6, 0.5])] {
        source.add_vertex(session_rust::Point::new(p[0], p[1], p[2]), Some(key));
    }
    source.add_face(vec![10, 20, 30, 40], Some(37)); source.add_face(vec![20, 50, 60, 30], Some(91));
    let mut scene = Scene::demo(); scene.clear(); let owner = scene.insert(PreparedMesh::new(source).unwrap()).unwrap();
    let weak = Rc::downgrade(&scene.objects()[0].mesh); let mut renderer = renderer(&scene); let stats = renderer.stats();
    let before = colour_pixels(&mut renderer);
    renderer.set_highlight(&scene, Some(Target { owner, part: Part::Face(37) })).unwrap();
    let face = colour_pixels(&mut renderer); assert!(renderer.highlight_bytes() > 0); let mut changed = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(face.chunks_exact(4)).enumerate() { if a != b {
        let x = index % 320; let y = index / 320; assert!((31..=160).contains(&x) && (47..=192).contains(&y)); changed += 1;
    } }
    assert!(changed > 10000, "The original face, rather than the whole object, is visibly selected");
    renderer.set_highlight(&scene, Some(Target { owner, part: Part::MeshEdge(10, 20) })).unwrap();
    let edge = colour_pixels(&mut renderer); let mut changed = 0; let mut black = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(edge.chunks_exact(4)).enumerate() {
        let x = index % 320; let y = index / 320;
        if a != b { assert!((30..=161).contains(&x) && (190..=193).contains(&y)); changed += 1; }
        if (35..150).contains(&x) && (191..=192).contains(&y) && b[0] < 160 && b[1] < 140 && b[2] < 80 { black += 1; }
    }
    assert!(changed > 80 && black > 80, "A yellow fringe surrounds the original black edge: changed={changed}, dark={black}");
    assert_eq!(renderer.stats(), stats, "Highlights reuse primary geometry");
    renderer.set_highlight(&scene, None).unwrap(); assert_eq!(renderer.highlight_bytes(), 0); assert_eq!(colour_pixels(&mut renderer), before);
    scene.clear(); renderer.set_scene(&scene, None); assert!(weak.upgrade().is_none()); assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_control_highlight_surrounds_only_the_original_marker_and_keeps_source_precision() {
    let mut editor = crate::editor::Editor::default(); editor.apply(crate::editor::Action::Close).unwrap();
    editor.apply(crate::editor::Action::AddCurve).unwrap(); editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let owner = editor.scene.paths()[0].id; let original = editor.scene.paths()[0].points(); let mut renderer = renderer(&editor.scene);
    let before = colour_pixels(&mut renderer); renderer.set_highlight(&editor.scene, Some(crate::pick_sources::Target {
        owner, part: crate::pick_sources::Part::Control(1) })).unwrap(); let after = colour_pixels(&mut renderer); let mut changed = 0;
    for (index, (a, b)) in before.chunks_exact(4).zip(after.chunks_exact(4)).enumerate() { if a != b {
        let x = index % 320; let y = index / 320; assert!((151..=168).contains(&x) && (39..=56).contains(&y)); changed += 1;
    } }
    assert!(changed > 20, "The halo belongs to the original control rather than a curve sample");
    assert_eq!(editor.scene.paths()[0].points(), original);
    editor.apply(crate::editor::Action::Close).unwrap(); renderer.set_scene(&editor.scene, None); assert_eq!(renderer.highlight_bytes(), 0);
}

#[test]
fn actual_multiple_mesh_selection_reuses_geometry_and_pick_ownership() {
    let mut scene = Scene::demo(); scene.clear();
    let panel = |x: f64| PreparedMesh::triangle([[x - 0.25, -0.4, 0.5], [x + 0.25, -0.4, 0.5], [x, 0.4, 0.5]], [0.5, 0.5, 0.5]).unwrap();
    let a = scene.insert(panel(-0.4)).unwrap(); let b = scene.insert(panel(0.4)).unwrap(); let mut renderer = renderer(&scene);
    let baseline = colour_pixels(&mut renderer); let geometry = renderer.stats();
    renderer.select_objects(&scene, &[a, b]).unwrap(); let selected = colour_pixels(&mut renderer);
    let pixel = |bytes: &[u8], x: usize| bytes[(120 * 320 + x) * 4..(120 * 320 + x) * 4 + 3].to_vec();
    for x in [96, 224] { let p = pixel(&selected, x); assert!(p[0] > 0 && p[0] == p[1] && p[2] == 0); }
    renderer.select_objects(&scene, &[b]).unwrap(); let one = colour_pixels(&mut renderer);
    assert_eq!(pixel(&one, 96), pixel(&baseline, 96)); assert_eq!(pixel(&one, 224), pixel(&selected, 224));
    assert_eq!(&renderer.stats()[..2], &geometry[..2], "Only selected settings change, not primary geometry");
    for (x, id) in [(-0.4, a), (0.4, b)] { renderer.request_pick([x, 0.], identity()).unwrap(); assert_eq!(finish(&mut renderer).unwrap()[0], id.value()); }
}

#[test]
fn actual_tool_preview_is_temporary_bounded_and_absent_from_object_picking() {
    let mut scene = Scene::demo(); scene.clear(); let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer); let stats = renderer.stats();
    let mut draft = crate::tool::Draft::new(crate::tool::Shape::Line);
    draft.submit([-0.5, 0., 0.2]).unwrap(); draft.hover(Some([0.5, 0., 0.2])).unwrap();
    renderer.set_preview(Some(&draft)).unwrap(); assert_eq!(renderer.preview_bytes(), 76);
    let after = colour_pixels(&mut renderer);
    let changed = before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(a,b)| a!=b).count();
    assert!(changed > 100, "The actual unfinished rubber band and cursor render");
    let p = &after[(120*320+160)*4..(120*320+160)*4+3]; assert_eq!(p, &[255,255,0], "The preview uses current selection yellow: {p:?}");
    renderer.request_pick([0., 0.], identity()).unwrap(); assert!(finish(&mut renderer).is_none(), "Drafts have no saved-object pick owner");
    assert_eq!(renderer.stats(), stats); assert!(scene.lines().is_empty());
    renderer.set_preview(None).unwrap(); assert_eq!(renderer.preview_bytes(), 0); assert_eq!(colour_pixels(&mut renderer), before);
    renderer.set_preview(Some(&draft)).unwrap(); scene.clear(); renderer.set_scene(&scene, None); assert_eq!(renderer.preview_bytes(), 0);
}

#[test]
fn actual_layer_colours_change_display_settings_without_editing_or_reuploading_original_geometry() {
    let mut scene=Scene::demo();scene.clear();let id=scene.insert(triangle(0.5)).unwrap();let source=Rc::clone(scene.objects()[0].geometry().unwrap());let original=source.to_proto();
    let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let allocations=renderer.stats();
    Rc::make_mut(&mut scene.view).faces.insert(id,[0,0,255]);renderer.set_scene(&scene,None);let blue=colour_pixels(&mut renderer);
    assert_ne!(blue,before);let centre=(120*320+160)*4;assert!(blue[centre+2]>blue[centre].saturating_add(100)&&blue[centre+2]>blue[centre+1].saturating_add(100));
    assert_eq!(&renderer.stats()[..2],&allocations[..2],"Layer colour reuses primary geometry and its settings allocation");assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
    Rc::make_mut(&mut scene.view).edges.insert(id,[0,255,0]);renderer.set_scene(&scene,None);let green_edges=colour_pixels(&mut renderer);let count=green_edges.chunks_exact(4).filter(|p|p[1]>p[0].saturating_add(100)&&p[1]>p[2].saturating_add(100)).count();assert!(count>100,"Actual source boundary colour is independent: {count}");
    Rc::make_mut(&mut scene.view).faces.remove(&id);renderer.set_scene(&scene,None);let restored_faces=colour_pixels(&mut renderer);assert_eq!(&restored_faces[centre..centre+4],&before[centre..centre+4]);assert_ne!(restored_faces,before,"Restoring Faces leaves the independent edge override");
    Rc::make_mut(&mut scene.view).edges.remove(&id);renderer.set_scene(&scene,None);assert_eq!(colour_pixels(&mut renderer),before);assert!(Rc::ptr_eq(&source,scene.objects()[0].geometry().unwrap()));
}

#[test]
fn actual_delayed_gpu_answer_finishes_a_released_drag_and_reuses_its_geometry() {
    use crate::editor::{Editor,Action,Change};
    let mut e=Editor::default();e.scene.clear();let id=e.scene.insert(triangle(0.2)).unwrap();let original=e.scene.placement(id).unwrap().m;let source=e.scene.objects()[0].geometry().unwrap().clone();
    let mut renderer=renderer(&e.scene);let transform=e.camera.uniform();let gpu=renderer.stats();let size=[320.,240.];
    e.apply(Action::CanvasStart {from:[0.,0.],to:[0.2,0.],size,modified:false,pointer:1}).unwrap();renderer.request_pick([0.,0.],transform).unwrap();
    e.apply(Action::CanvasRelease {screen:[0.4,0.],size,add:false,remove:false}).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,original);
    renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();let hit=renderer.take_pick(transform).unwrap().unwrap().and_then(|pixel|renderer.resolve_pick(&e.scene,pixel));assert_eq!(hit.unwrap().owner,id);
    assert_eq!(e.apply(Action::CanvasHit(hit.map(|target|target.owner))).unwrap(),Change::Scene);assert_ne!(e.scene.placement(id).unwrap().m,original);assert!(Rc::ptr_eq(&source,e.scene.objects()[0].geometry().unwrap()));
    renderer.set_scene(&e.scene,e.selected);assert_eq!(&renderer.stats()[..2],&gpu[..2]);e.apply(Action::Undo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,original);
}

#[test]
fn actual_surface_pick_keeps_one_cad_face_and_four_domain_edge_ids() {
    use crate::pick_sources::{Mode,Part};
    let mut scene=Scene::demo();scene.clear();let source=Rc::new(crate::surface::example());let _=source.guid();let before=source.to_proto();let id=scene.insert_surface(source.clone()).unwrap();let mut renderer=renderer(&scene);let camera=crate::camera::Camera::default();let transform=camera.uniform();let gpu=renderer.stats();
    let finish_at=|renderer:&mut Renderer|{renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();renderer.take_pick(transform).unwrap().unwrap().and_then(|hit|renderer.resolve_pick(&scene,hit))};
    renderer.set_pick_mode(Mode::Face);renderer.request_pick([0.,0.],transform).unwrap();assert_eq!(finish_at(&mut renderer).unwrap().part,Part::Face(0));
    renderer.set_pick_mode(Mode::Edge);for (point,edge) in [([-1.,0.,0.],0),([1.,0.,0.],1),([0.,-1.,0.],2),([0.,1.,0.],3)] {
        let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));renderer.request_pick([point[0] as f32,point[1] as f32],transform).unwrap();let target=finish_at(&mut renderer).unwrap();assert_eq!(target.owner,id);assert_eq!(target.part,Part::SurfaceEdge(edge));
    }
    assert_eq!(renderer.stats(),gpu);assert_eq!(source.to_proto(),before);
}

#[test]
fn actual_cad_hole_has_no_face_pick_and_integer_faces_retain_original_identity() {
    use crate::pick_sources::{Mode,Part};
    let mut scene=Scene::demo();scene.clear();let mut original=session_rust::BRep::create_block_with_hole(1.6,1.2,0.24,0.26);original.surfacecolor=session_rust::Color::new(0.35,0.65,0.85,1.);let source=Rc::new(original);let _=source.guid();let before=source.to_proto();let id=scene.insert_cad(source.clone()).unwrap();let mut renderer=renderer(&scene);let camera=crate::camera::Camera::default();let transform=camera.uniform();let gpu=renderer.stats();
    let finish_at=|renderer:&mut Renderer|{renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();renderer.take_pick(transform).unwrap().unwrap().and_then(|hit|renderer.resolve_pick(&scene,hit))};
    renderer.set_pick_mode(Mode::Face);renderer.request_pick([0.,0.],transform).unwrap();assert!(finish_at(&mut renderer).is_none(),"The actual shaded hole stays empty");
    let p=camera.view_projection().transform_point(&session_rust::Point::new(0.5,0.,0.12));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();let target=finish_at(&mut renderer).unwrap();assert_eq!(target.owner,id);assert_eq!(target.part,Part::Face(6),"The visible top cap retains its original CAD identity");renderer.set_pick_mode(Mode::Edge);let mut seen=0;
    for border in &scene.objects()[0].cad.as_ref().unwrap().edges {
        let points=&border.points;if !points.iter().all(|p|(p[2]-0.12).abs()<1e-10) {continue;}
        let middle=points.windows(2).nth((points.len()-1)/2).unwrap();let p=std::array::from_fn::<_,3,_>(|i|(middle[0][i]+middle[1][i])*0.5);let p=camera.view_projection().transform_point(&session_rust::Point::new(p[0],p[1],p[2]));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();assert_eq!(finish_at(&mut renderer).unwrap_or_else(||panic!("Missing original CAD edge {} at {:?}",border.edge,p)).part,Part::BRepEdge(border.edge));seen+=1;
    }
    assert_eq!(seen,5,"All four outer edges and the original hole rim resolve");assert_eq!(renderer.stats(),gpu);assert_eq!(source.to_proto(),before);
}

#[test]
fn actual_screen_text_overlay_does_not_create_scene_pick_owners_or_reupload_geometry() {
    let mut scene=Scene::demo();scene.clear();let id=scene.insert(triangle(0.2)).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let gpu=renderer.stats();renderer.text_demo().unwrap();let with_text=colour_pixels(&mut renderer);assert_ne!(with_text,before);assert_eq!(renderer.text_error(),None);assert_eq!(renderer.stats(),gpu);assert_eq!(scene.objects().len(),1);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
    renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());renderer.request_pick([-0.85,0.8],identity()).unwrap();assert!(finish(&mut renderer).is_none());assert_eq!(renderer.text_usage()[0],1);renderer.set_text(Vec::new()).unwrap();assert_eq!(renderer.text_usage(),[0;6]);assert_eq!(colour_pixels(&mut renderer),before);
}

#[test]
fn actual_owned_text_plane_respects_depth_integer_picks_selection_cache_and_close() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let gpu=renderer.stats();
    let mut source=example(TextPlacement::WorldPlane {world:[-0.1,0.05,0.6],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.08});source.label.text="Hi".into();let source=Rc::new(source);let original=serde_json::to_string(&*source).unwrap();let weak=Rc::downgrade(&source);let id=scene.insert_text(Prepared::new(source.clone()).unwrap()).unwrap();drop(source);renderer.set_scene(&scene,None);
    assert_eq!(colour_pixels(&mut renderer),before,"Actual fixed text behind the opaque solid is hidden");renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],mesh.value());
    scene.place_text(id,session_rust::Xform::translation(0.,0.,-0.45)).unwrap();renderer.set_scene(&scene,None);let front=colour_pixels(&mut renderer);assert_ne!(front,before);assert_eq!(renderer.text_error(),None);assert!(front.chunks_exact(4).filter(|p|p[0]<10&&p[1]<10&&p[2]<10).count()>20);let raster=renderer.text_scene_usage()[2];assert_eq!(raster,1,"Placement reuses already rasterized original words");assert_eq!(renderer.text_usage()[0],1);
    renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap(),[id.value(),0]);renderer.select_objects(&scene,&[id]).unwrap();let selected=colour_pixels(&mut renderer);assert!(selected.chunks_exact(4).filter(|p|p[0]>200&&p[1]>200&&p[2]<40).count()>20);assert_eq!(renderer.text_scene_usage()[2],raster);assert_eq!(renderer.text_usage()[0],1);assert_eq!(&renderer.stats()[..2],&gpu[..2]);assert_eq!(serde_json::to_string(&*scene.texts()[0].prepared.source).unwrap(),original);
    Rc::make_mut(&mut scene.view).lock(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.06,-0.008],identity()).unwrap();assert!(finish(&mut renderer).is_none(),"The locked plate remains an occluder without a selectable id");Rc::make_mut(&mut scene.view).hide(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.06,0.02],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],mesh.value());
    scene.clear();renderer.set_scene(&scene,None);assert!(weak.upgrade().is_none());assert_eq!(renderer.text_scene_usage(),[0;4]);assert_eq!(renderer.text_usage(),[0;6]);
}

#[test]
fn actual_anchor_nameplate_and_billboard_follow_the_current_depth_and_plate_rules() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    for (placement,overlay) in [(TextPlacement::Anchor {world:[0.,0.,0.6],offset:[0.;2]},false),(TextPlacement::Nameplate {world:[0.,0.,0.6],padding:[17.,4.],rounded:true},true),(TextPlacement::WorldBillboard {world:[0.,0.,0.6],world_height:0.08},false)] {
        let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);let mut source=example(placement);source.label.text="Hi".into();let id=scene.insert_text(Prepared::new(Rc::new(source)).unwrap()).unwrap();renderer.set_scene(&scene,None);let after=colour_pixels(&mut renderer);assert_eq!(after!=before,overlay,"Only the nameplate overlays its own solid");renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],if overlay {id.value()} else {mesh.value()});assert_eq!(renderer.text_error(),None);
        let fonts=vec![crate::text::FONT_BYTES.to_vec(),crate::text::FALLBACK_BYTES.to_vec(),crate::text::SYMBOL_BYTES.to_vec()];assert!(renderer.replace_text_fonts(vec![vec![0;8]]).is_err());assert_eq!(colour_pixels(&mut renderer),after);renderer.replace_text_fonts(fonts).unwrap();assert_eq!(colour_pixels(&mut renderer),after);assert_eq!(renderer.text_usage()[5],1);
    }
}

#[test]
fn actual_world_text_clip_uses_canvas_coordinates_in_the_small_integer_pick_window() {
    use crate::{scene_label::{Prepared,example},text::TextPlacement};
    let mut scene=Scene::demo();scene.clear();let mut source=example(TextPlacement::WorldPlane {world:[-0.2,0.1,0.3],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.2});source.label.text="Hi".into();source.label.clip=Some([140.,100.,160.,140.]);let id=scene.insert_text(Prepared::new(Rc::new(source)).unwrap()).unwrap();let mut renderer=renderer(&scene);let before=colour_pixels(&mut renderer);assert_eq!(renderer.text_error(),None);
    let mut clean=Scene::demo();clean.clear();let mut no_text=super::pick_gpu_tests::renderer(&clean);let baseline=colour_pixels(&mut no_text);let mut changed=0;
    for (i,(p,b)) in before.chunks_exact(4).zip(baseline.chunks_exact(4)).enumerate() {let (x,y)=(i%320,i/320);if x<140||x>=160||y<100||y>=140 {assert_eq!(p,b,"Clipped text does not affect pixels outside the CSS clip");} else {changed+=usize::from(p!=b);}}
    assert!(changed>50);renderer.request_pick([-0.0625,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());renderer.request_pick([0.125,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_selected_silhouette_has_one_union_no_internal_diagonal_and_keeps_source_and_picks() {
    let mut scene=Scene::demo();scene.clear();let mesh=|x:f64|{let mut source=session_rust::Mesh::create_box(0.6,0.8,0.1);source.set_objectcolor(session_rust::Color::new(0.5,0.5,0.5,1.));let mut prepared=PreparedMesh::new(source).unwrap();prepared.model=session_rust::Xform::translation(x,0.,0.5);prepared};let a=scene.insert(mesh(-0.3)).unwrap();let b=scene.insert(mesh(0.3)).unwrap();let sources=scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>();let mut renderer=renderer(&scene);let geometry=renderer.stats();renderer.select_objects(&scene,&[a,b]).unwrap();let first=colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert_eq!(alpha.len(),320*240);assert!(alpha.iter().filter(|v|**v>0).count()>300);for y in 90..150 {for x in 90..230 {assert_eq!(alpha[y*320+x],0,"The combined coverage has no outline along the shared object join or tessellation diagonals");}}
    let stats=renderer.outline_usage();assert_eq!(colour_pixels(&mut renderer),first);assert_eq!(renderer.outline_alpha(),alpha);assert_eq!(renderer.outline_usage()[1],stats[1]);assert!(renderer.outline_usage()[2]>stats[2]);assert_eq!(&renderer.stats()[..2],&geometry[..2]);assert_eq!(scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>(),sources);renderer.request_pick([-0.3,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],a.value());renderer.request_pick([0.3,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],b.value());
    renderer.select_objects(&scene,&[]).unwrap();colour_pixels(&mut renderer);assert!(renderer.outline_alpha().is_empty());assert_eq!(renderer.outline_usage()[0],0);
}

#[test]
fn actual_surface_outline_culls_occluded_selection_and_selected_edge_ink_cannot_narrow_its_alpha() {
    use crate::pick_sources::{Target,Part};
    let mut scene=Scene::demo();scene.clear();let back=scene.insert(triangle(0.6)).unwrap();let front=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);renderer.select_objects(&scene,&[back]).unwrap();colour_pixels(&mut renderer);assert!(renderer.outline_alpha().iter().all(|v|*v==0),"A selected surface wholly behind the current opaque depth adds no silhouette");renderer.select_objects(&scene,&[front]).unwrap();let plain=colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert!(alpha.iter().any(|v|*v>128));let edge=scene.objects().iter().find(|row|row.id==front).unwrap().mesh.parts.edges[0];renderer.set_highlight(&scene,Some(Target {owner:front,part:Part::MeshEdge(edge.0,edge.1)})).unwrap();let inked=colour_pixels(&mut renderer);assert_eq!(renderer.outline_alpha(),alpha,"Source edge highlighting cannot change the actual outline width");let mut opaque=0;for ((a,b),coverage) in plain.chunks_exact(4).zip(inked.chunks_exact(4)).zip(&alpha) {if *coverage>=250&&a[..3].iter().all(|c|*c<8) {opaque+=1;assert!(b[..3].iter().all(|c|*c<12),"Yellow source edge ink cannot paint over black outline pixels");}}assert!(opaque>100);scene.clear();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.outline_usage()[0],0);
}

#[test]
fn actual_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.select_objects(&scene,&[id]).unwrap();let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_selected_bore_keeps_its_open_center_and_black_inner_contour() {
    let source=Rc::new(session_rust::BRep::create_block_with_hole(1.2,1.2,0.4,0.25));
    let _=source.guid();let original=source.to_proto();let mut scene=Scene::demo();scene.clear();
    let id=scene.insert_cad(source.clone()).unwrap();let mut renderer=renderer(&scene);
    renderer.select_objects(&scene,&[id]).unwrap();let pixels=colour_pixels(&mut renderer);
    let alpha=renderer.outline_alpha();
    for y in 112..128 {for x in 152..168 {assert_eq!(alpha[y*320+x],0,"The bore center stays open");}}
    let mut inner=0;
    for y in 80..160 {for x in 110..210 {
        let i=y*320+x;
        if alpha[i]>=250 {inner+=1;assert!(pixels[i*4..i*4+3].iter().all(|c|*c<12),"The actual inner silhouette is black");}
    }}
    assert!(inner>100,"The real bore boundary has an inner outline: {inner}");
    renderer.request_pick([0.,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    renderer.request_pick([0.4,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());
    assert_eq!(source.to_proto(),original,"Coverage cannot change original topology");
}

#[test]
fn actual_cloud_pixels_and_whole_ids_keep_cached_source_uploads_density_and_locked_occlusion() {
    let mut source=session_rust::PointCloud::from_coords(vec![0.,0.,0.2],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let _=source.guid();let before=source.to_proto();
    let mut scene=Scene::demo();scene.clear();let mesh=scene.insert(triangle(0.6)).unwrap();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    for density in [1.,2.] {renderer.set_density(density);let first=colour_pixels(&mut renderer);let center=&first[(120*320+160)*4..(120*320+160)*4+4];assert!(center[0]>240&&center[1]<10&&center[2]<10,"Original cloud color covers the solid behind it");let usage=renderer.cloud_usage();assert_eq!(usage[0],1);assert_eq!(usage[2],1);assert!(usage[1]>320*240*16);assert_eq!(colour_pixels(&mut renderer),first);assert_eq!(renderer.cloud_usage()[3],usage[3]);assert!(renderer.cloud_usage()[4]>usage[4]);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([cloud.value(),0]));}
    renderer.set_density(1.);Rc::make_mut(&mut scene.view).lock(&[cloud],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),None,"The locked cloud writes zero while occluding the mesh");
    Rc::make_mut(&mut scene.view).hide(&[cloud],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,0.],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([mesh.value(),0]));assert_eq!(source.to_proto(),before);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}
#[test]
fn actual_cloud_copy_and_normal_shading_preserve_one_source_and_release_all_derived_owners() {
    let source=Rc::new(session_rust::PointCloud::from_coords(vec![-0.3,0.,0.2,0.3,0.,0.2],vec![0,255,0,255,0,255,0,255],vec![0.,0.,1.,0.,0.,1.]));let weak=Rc::downgrade(&source);let mut scene=Scene::demo();scene.clear();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).any(|p|p[1]>160&&p[0]<10&&p[2]<10));assert_eq!(renderer.cloud_usage()[2],1);
    let copy=scene.copy_row(cloud,false).unwrap();scene.translate(copy,[0.,0.3,0.]).unwrap();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.cloud_usage()[2],1,"Copies share one original upload");assert_eq!(renderer.cloud_usage()[0],4);renderer.request_pick([-0.3,0.3],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],copy.value());drop(source);scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);assert!(weak.upgrade().is_none());
}

fn camera_pixels(renderer: &mut Renderer,camera:&crate::camera::Camera) -> Vec<u8> {
    let texture = renderer.device.create_texture(&wgpu::TextureDescriptor { label: Some("selection pixels"),
        size: wgpu::Extent3d { width: 320, height: 240, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[] });
    renderer.cloud_camera(camera);renderer.draw_mode(&texture.create_view(&Default::default()), &Default::default(), &camera.uniform(), false);
    let bytes = renderer.device.create_buffer(&wgpu::BufferDescriptor { label: Some("selection readback"), size: 320 * 240 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
    let mut encoder = renderer.device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(texture.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &bytes,
        layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(1280), rows_per_image: Some(240) } }, texture.size());
    renderer.queue.submit([encoder.finish()]); let (send, receive) = std::sync::mpsc::channel();
    bytes.slice(..).map_async(wgpu::MapMode::Read, move |result| { let _ = send.send(result); });
    renderer.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(5)) }).unwrap();
    receive.recv_timeout(std::time::Duration::from_secs(5)).unwrap().unwrap();
    let result = bytes.slice(..).get_mapped_range().to_vec(); bytes.unmap(); result
}

#[test]
fn actual_large_cloud_changes_detail_with_the_view_and_reuses_its_original_upload() {
    let source=Rc::new(crate::cloud::large_example());assert_eq!(source.len(),2_000_064);let mut scene=Scene::demo();scene.clear();scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    let mut camera=crate::camera::Camera::default();camera.aspect=320./240.;camera.projection=crate::camera::Projection::Orthographic;camera.distance=100./std::f64::consts::FRAC_PI_6.tan();
    let coarse=camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);let uploads=renderer.cloud_usage()[2];let prepared=renderer.cloud_usage()[3];assert_eq!(camera_pixels(&mut renderer,&camera),coarse);assert_eq!(renderer.cloud_usage()[3],prepared);
    camera.distance=0.7/std::f64::consts::FRAC_PI_6.tan();let fine=camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],2_000_064);assert_ne!(fine,coarse);assert_eq!(renderer.cloud_usage()[2],uploads);assert_eq!(source.point_id(64),64);assert_eq!(source.point_id(source.len()-1),2_000_063);
    camera.distance=100./std::f64::consts::FRAC_PI_6.tan();assert_eq!(camera_pixels(&mut renderer,&camera),coarse);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}

#[test]
fn actual_cloud_control_pick_preserves_full_u32_source_ids_and_local_highlight() {
    use crate::pick_sources::{Mode,Part};
    let mut proto=session_rust::PointCloud::from_coords(vec![-0.4,0.,0.2,0.4,0.,0.2],vec![],vec![]).to_proto();
    proto.point_size=16.;proto.point_ids=vec![u32::MAX,16_777_217];let source=Rc::new(session_rust::PointCloud::from_proto(proto));let _=source.guid();let original=source.to_proto();
    let mut scene=Scene::demo();scene.clear();let owner=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();scene.cloud_controls(Some(owner));let mut renderer=renderer(&scene);renderer.set_pick_mode(Mode::Control);
    let uploaded=renderer.cloud_usage()[2];let geometry=renderer.stats();
    for density in [1.,2.] {
        renderer.set_density(density);renderer.resize(Viewport {width:(320.*density) as u32,height:(240.*density) as u32});
        for (x,id) in [(-0.4,u32::MAX),(0.4,16_777_217)] {
            renderer.request_pick([x,0.],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();let target=renderer.resolve_pick(&scene,pixel).unwrap();assert_eq!(target.part,Part::CloudPoint(id));assert_eq!(target.owner,owner);
            renderer.set_highlight(&scene,Some(target)).unwrap();assert!(renderer.highlight_bytes()>0);assert_eq!(renderer.stats(),geometry);
        }
    }
    assert_eq!(renderer.cloud_usage()[2],uploaded);assert_eq!(source.to_proto(),original);
    Rc::make_mut(&mut scene.view).lock(&[owner],true);renderer.set_density(1.);renderer.resize(Viewport {width:320,height:240});renderer.set_scene(&scene,None);
    renderer.request_pick([-0.4,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    scene.clear();renderer.set_scene(&scene,None);renderer.set_highlight(&scene,None).unwrap();assert_eq!(renderer.cloud_usage(),[0;5]);assert_eq!(renderer.highlight_bytes(),0);
}

#[test]
fn actual_resident_cloud_controls_query_original_points_outside_the_coarse_display_subset() {
    use crate::pick_sources::{Mode,Part};
    let mut proto=crate::cloud::large_example().to_proto();for xyz in proto.coords[..64*3].chunks_exact_mut(3) {xyz[2]=0.;}
    let source=Rc::new(session_rust::PointCloud::from_proto(proto));let mut scene=Scene::demo();scene.clear();let owner=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let mut renderer=renderer(&scene);
    let mut camera=crate::camera::Camera::default();camera.aspect=320./240.;camera.projection=crate::camera::Projection::Orthographic;camera.distance=100./std::f64::consts::FRAC_PI_6.tan();
    camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);let uploads=renderer.cloud_usage()[2];
    scene.cloud_controls(Some(owner));renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Control);camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],2_000_064);
    let transform=camera.view_projection().to_f32();let p=camera.view_projection().transform_point(&session_rust::Point::new(0.25,0.25,0.2));renderer.request_pick([p[0] as f32,p[1] as f32],transform).unwrap();renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();
    let pixel=renderer.take_pick(transform).unwrap().unwrap().unwrap();let target=renderer.resolve_pick(&scene,pixel).unwrap();let Part::CloudPoint(id)=target.part else {panic!("Original cloud point identity");};assert!(id>=64);assert!(target.valid(&scene));assert_eq!(renderer.cloud_usage()[2],uploads);
    renderer.request_pick([0.,0.],transform).unwrap();renderer.cancel_pick();assert!(renderer.take_pick(transform).is_none());
    scene.cloud_controls(None);renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Object);camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}

#[test]
fn actual_streamed_pages_pick_original_ids_share_uploads_and_release_after_eviction() {
    use crate::pick_sources::{Mode,Part};
    let prepared=crate::range_example::prepared();let weak=Rc::downgrade(&prepared.source);
    let mut scene=Scene::empty();let owner=scene.insert_stream(prepared.clone(),session_rust::Xform::identity(),12.).unwrap();scene.cloud_controls(Some(owner));
    let mut renderer=renderer(&scene);renderer.set_pick_mode(Mode::Control);
    for (cursor,id) in [([-0.5,0.5],u32::MAX),([0.5,0.5],16_777_217),([-0.5,-0.5],10)] {
        renderer.request_pick(cursor,identity()).unwrap();let hit=finish(&mut renderer).expect("Actual streamed point coverage");let target=renderer.resolve_pick(&scene,hit).unwrap();assert_eq!(target.owner,owner);assert_eq!(target.part,Part::CloudPoint(id));
    }
    let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).any(|p|p[2]>160&&p[0]<10));assert!(pixels.chunks_exact(4).any(|p|p[0]>160&&p[2]<10));
    let uploads=renderer.cloud_usage()[2];let copy=scene.copy_row(owner,false).unwrap();scene.translate(copy,[0.2,0.,0.]).unwrap();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.cloud_usage()[2],uploads,"Placed owners share original page uploads");
    Rc::make_mut(&mut scene.view).lock(&[owner,copy],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.5,0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).lock(&[owner,copy],false);let held=Rc::downgrade(&prepared.page(2).unwrap());prepared.pages.borrow_mut().clear();assert!(held.upgrade().is_some(),"GPU page owner retains precise pick identities");renderer.set_scene(&scene,None);assert!(held.upgrade().is_none());renderer.request_pick([-0.5,0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none(),"Evicted detailed points are absent");
    renderer.request_pick([-0.5,-0.5],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit).unwrap().part,Part::CloudPoint(10));
    scene.clear();renderer.set_scene(&scene,None);drop(prepared);assert_eq!(renderer.cloud_usage(),[0;5]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_definition_instances_batch_one_geometry_keep_integer_picks_and_release_records() {
    let mut original=crate::instance_specimen::message();original.objects.as_mut().unwrap().instances[0].xform=None;original.objects.as_mut().unwrap().instances[1].flags=0;
    original.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.5,0.,0.3).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.5,0.,0.3).to_proto())}];
    let mut scene=Scene::empty();scene.next_id=16_777_217;crate::scene_document::import(&mut scene,&prost::Message::encode_to_vec(&original),"",&session_rust::Xform::identity(),0.).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[1].id;let weak=Rc::downgrade(scene.objects()[0].geometry().unwrap());let mut renderer=renderer(&scene);assert_eq!(&renderer.instance_usage()[..3],&[1,2,376]);let source_uploads=renderer.stats()[0];let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count()>1000);
    for (cursor,id) in [([-0.5,-0.1],a),([0.5,-0.1],b)] {renderer.request_pick(cursor,identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());}
    let mut coloured=original.clone();let first=&mut coloured.objects.as_mut().unwrap().instances[0];first.flags|=session_rust::InstanceRef::FLAG_COLOR;first.color=Some(session_rust::proto::Color {r:0.9,g:0.08,b:0.05,a:0.5,..Default::default()});let mut colour_scene=Scene::empty();crate::scene_document::import(&mut colour_scene,&prost::Message::encode_to_vec(&coloured),"",&session_rust::Xform::identity(),0.).unwrap();let mut colour_renderer=super::pick_gpu_tests::renderer(&colour_scene);let colours=colour_pixels(&mut colour_renderer);assert!(colours.chunks_exact(4).filter(|p|p[0]>p[1].saturating_add(50)&&p[0]>p[2].saturating_add(50)).count()>500);assert!(colours.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)).count()>500);
    let writes=renderer.instance_usage()[3];colour_pixels(&mut renderer);assert_eq!(renderer.instance_usage()[3],writes);scene.translate(a,[0.,0.2,0.]).unwrap();renderer.set_scene(&scene,None);assert_eq!(renderer.stats()[0],source_uploads);assert!(renderer.instance_usage()[3]>writes);renderer.select_objects(&scene,&[a]).unwrap();let selected=colour_pixels(&mut renderer);assert!(selected.chunks_exact(4).any(|p|p[0]>140&&p[1]>90&&p[2]<40));
    Rc::make_mut(&mut scene.view).hide(&[b],true);renderer.set_scene(&scene,None);assert_eq!(renderer.instance_usage()[1],1);scene.clear();renderer.set_scene(&scene,None);assert_eq!(&renderer.instance_usage()[..3],&[0,0,0]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_congruent_cad_replay_matches_independent_faces_ink_and_owner_picks() {
    let first=Rc::new(session_rust::BRep::create_block_with_hole(0.7,0.6,0.2,0.12).transformed(&session_rust::Xform::translation(-0.35,-0.3,0.3)));let mut cache=crate::cad_shape_cache::Cache::default();cache.prepare(first.clone()).unwrap();let motion=&session_rust::Xform::translation(0.1,-0.1,0.)*&session_rust::Xform::rotation_z(90.,true);let copy=Rc::new(first.transformed(&motion));let mut shared=Scene::empty();let id=shared.insert_prepared_cad(cache.prepare(copy.clone()).unwrap()).unwrap();assert_eq!(cache.replays,1);let mut alone=Scene::empty();alone.insert_cad(copy.clone()).unwrap();let mut shared_renderer=renderer(&shared);let mut independent_renderer=renderer(&alone);let a=colour_pixels(&mut shared_renderer);let b=colour_pixels(&mut independent_renderer);let changed=a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(a,b)|a[..3].iter().zip(&b[..3]).any(|(a,b)|a.abs_diff(*b)>3)).count();assert!(changed<100,"Congruent faces and source ink differed at {changed} pixels");assert!(a.chunks_exact(4).filter(|p|p[0]<180&&p[1]<180&&p[2]<180).count()>500);
    let mut hits=0;for point in [[-0.1,-0.05],[0.25,-0.05],[0.1,0.15]] {shared_renderer.request_pick(point,identity()).unwrap();independent_renderer.request_pick(point,identity()).unwrap();let actual=finish(&mut shared_renderer);let independent=finish(&mut independent_renderer);assert_eq!(actual,independent);if let Some(actual)=actual {assert_eq!(actual[0],id.value());hits+=1;}}assert!(hits>0,"Actual original CAD ownership must be picked");
    let weak=Rc::downgrade(&copy);drop(copy);shared.clear();alone.clear();shared_renderer.set_scene(&shared,None);independent_renderer.set_scene(&alone,None);drop(cache);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_nested_rotated_and_mirrored_definition_placements_keep_independent_integer_owners() {
    let mut scene=Scene::empty();scene.next_id=16_777_217;crate::scene_document::import(&mut scene,&crate::nested_example::bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);assert_eq!(&gpu.instance_usage()[..3],&[1,2,376]);for row in scene.objects() {let point=row.model.transform_point(&session_rust::Point::new((0.123456789012345+0.8)/3.,0.8/3.,0.));gpu.request_pick([point[0] as f32,point[1] as f32],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],row.id.value());}
    let id=scene.objects()[1].id;gpu.select_objects(&scene,&[id]).unwrap();let pixels=colour_pixels(&mut gpu);assert!(pixels.chunks_exact(4).filter(|p|p[0]>140&&p[1]>90&&p[2]<40).count()>500);let bytes=crate::scene_archive::save(&scene).unwrap();let mut restored=Scene::empty();crate::scene_document::open(&mut restored,&bytes).unwrap();for a in scene.objects() {let guid=scene.instances[&a.id].instance.guid();let b=restored.objects().iter().find(|row|restored.instances[&row.id].instance.guid()==guid).unwrap();for (a,b) in a.model.m.iter().zip(&b.model.m) {assert!((a-b).abs()<1e-12,"Canonical placement for {guid}: {a} {b}");}}
}

#[test]
fn actual_sheet_batches_share_source_uploads_keep_world_pens_full_entity_ids_and_locked_occlusion() {
    use crate::pick_sources::{Part,Mode,Target};
    let mut source=crate::sheet_example::source();source.entity_count=u32::MAX;source.source_ids=vec![u32::MAX-1;5];let mut scene=Scene::empty();scene.next_id=16_777_217;let id=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(Rc::new(source)).unwrap())).unwrap();let copy=scene.copy_row(id,false).unwrap();scene.place_sheet(copy,session_rust::Xform::translation(0.,0.1,0.)).unwrap();let weak=Rc::downgrade(&scene.sheets()[0].prepared);let mut renderer=renderer(&scene);assert_eq!(renderer.sheet_usage()[..2],[2,1]);let uploads=renderer.sheet_usage()[3];renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[3],uploads);
    renderer.set_pick_mode(Mode::Edge);renderer.request_pick([0.,-0.5],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit),Some(Target {owner:id,part:Part::SheetEntity(u32::MAX-1)}));renderer.request_pick([0.,-0.4],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit),Some(Target {owner:copy,part:Part::SheetEntity(u32::MAX-1)}));assert!(hit[1]<u32::MAX-1);
    let pixels=colour_pixels(&mut renderer);let black=(0..240).filter(|y| {let p=&pixels[(*y*320+160)*4..][..4];p[0]<30&&p[1]<30&&p[2]<30}).count();assert!(black>=8&&black<20,"Physical .025mm pens draw separate three-pixel boundaries: {black}");scene.place_sheet(copy,session_rust::Xform::translation(0.,0.2,0.)).unwrap();renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[3],uploads);
    let front=scene.insert(triangle(0.1)).unwrap();renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Object);renderer.request_pick([0.,-0.5],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],front.value());Rc::make_mut(&mut scene.view).lock(&[front],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());Rc::make_mut(&mut scene.view).hide(&[front],true);Rc::make_mut(&mut scene.view).lock(&[id],true);renderer.set_scene(&scene,None);renderer.request_pick([0.,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.sheet_usage()[..2],[0,0]);assert!(weak.upgrade().is_none());
}

#[test]
fn actual_evicted_sheet_pages_remain_charged_until_renderer_refresh_cancels_old_picks() {
    let budget=crate::page_cache::Budget::new(256);let source=Rc::new(crate::sheet_range::Source {guid:crate::sheet_example::source().guid,name:"paged drawing".into(),url:"plan.pb".into(),fields:crate::stream::SheetFields {count:2,entities:u32::MAX,..Default::default()}});let stream=crate::sheet_stream::Stream::new(source,budget.clone());let raw=[-0.5f64,0.,0.3,0.5,0.,0.3].into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();let page=crate::sheet_page::Page::decode(0..1,2,u32::MAX,&raw,None,None,Some(&(u32::MAX-1).to_le_bytes())).unwrap();let held=stream.adopt(page,budget.reserve(192).unwrap()).unwrap();let weak=Rc::downgrade(&held);let mut scene=Scene::empty();let id=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::streamed(stream.clone()))).unwrap();let mut renderer=renderer(&scene);drop(held);renderer.set_pick_mode(crate::pick_sources::Mode::Edge);renderer.request_pick([0.,0.],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(hit[0],id.value());assert!(renderer.resolve_pick(&scene,hit).is_some());assert_eq!(budget.used(),84);stream.pages.borrow_mut().clear();assert_eq!(budget.used(),84);assert!(budget.reserve(192).is_err());renderer.request_pick([0.,0.],identity()).unwrap();renderer.set_scene(&scene,None);assert!(renderer.take_pick(identity()).is_none());assert_eq!(budget.used(),0);assert!(weak.upgrade().is_none());assert!(renderer.resolve_pick(&scene,hit).is_none());renderer.request_pick([0.,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_sheet_pen_projection_uses_camera_lens_independently_of_panel_frame_shift() {
    let source=session_rust::proto::Sheet {guid:crate::sheet_example::source().guid,coords:vec![-0.6,0.,0.3,0.6,0.,0.3],widths:vec![0.35],segment_count:1,..Default::default()};let mut scene=Scene::empty();scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(Rc::new(source)).unwrap())).unwrap();let mut renderer=renderer(&scene);let mut camera=crate::camera::Camera::default();let pen_rows=|pixels:Vec<u8>|pixels.chunks_exact(320*4).filter(|row|row[160*4..][..3].iter().all(|v|*v<30)).count();renderer.sheet_camera(&camera);let first=pen_rows(colour_pixels_at(&mut renderer,camera.uniform()));assert!(first>20);camera.frame=[0.,0.,1.,0.2];renderer.sheet_camera(&camera);let shifted=pen_rows(colour_pixels_at(&mut renderer,camera.uniform()));assert_eq!(shifted,first,"A panel shifts the frame without changing the original world pen");
}

#[test]
fn actual_mesh_control_pixel_keeps_sparse_vertex_key_at_both_densities() {
    let mut mesh=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.5,-0.5,0.25]),(30,[0.,0.5,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![10,20,30],Some(37));let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();scene.show_controls(true);
    let source=crate::source_edit::original(&scene,owner).unwrap();let model=session_rust::Xform::translation(0.125,0.25,0.);scene.place(owner,model).unwrap();
    for density in [1.,2.] {
        let mut renderer=renderer(&scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);renderer.request_pick([-0.375,-0.25],identity()).unwrap();
        let pixel=finish(&mut renderer).expect("Actual placed control pixel");
        assert_eq!(renderer.resolve_pick(&scene,pixel),Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::SourceControl(crate::source_controls::ControlId::Vertex(10))}));
        scene.show_controls(false);renderer.set_scene(&scene,None);renderer.request_pick([-0.375,-0.25],identity()).unwrap();assert!(finish(&mut renderer).is_none());scene.show_controls(true);
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,owner).unwrap()));
}

#[test]
fn actual_controls_are_blue_seven_css_pixels_and_selected_yellow_at_both_densities() {
    let mut mesh=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.1,0.1,0.25]),(30,[0.8,0.1,0.25]),(40,[0.1,0.8,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}
    mesh.add_face(vec![20,30,40],Some(37));let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(mesh).unwrap()).unwrap();scene.show_controls(true);
    assert!(scene.control_markers().iter().all(|m|m.display.diameter==7.&&m.display.colour==[0.15,0.35,0.9,1.]));
    let source=crate::source_edit::original(&scene,owner).unwrap();let target=crate::pick_sources::Target{owner,part:crate::pick_sources::Part::SourceControl(crate::source_controls::ControlId::Vertex(10))};
    for density in [1.,2.] {
        let mut renderer=renderer(&scene);renderer.set_density(density);
        let pixels=colour_pixels(&mut renderer);let center=(180*320+80)*4;
        for (actual,expected) in pixels[center..center+4].iter().zip([38_i16,89,230,255]){assert!((*actual as i16-expected).abs()<=2,"Actual blue control pixel {:?}",&pixels[center..center+4]);}
        let coverage:f32=(60..100).map(|x|(255.-pixels[(180*320+x)*4]as f32)/217.).sum();
        assert!((coverage/density-7.).abs()<0.6,"Seven CSS pixel row coverage at density {density}: {coverage}");
        renderer.set_highlight(&scene,Some(target)).unwrap();let selected=colour_pixels(&mut renderer);
        assert_eq!(&selected[center..center+4],&[255,255,0,255]);
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,owner).unwrap()));
}

#[test]
fn actual_control_net_pixel_addresses_parent_without_a_fake_edge_or_control() {
    let mut editor=crate::editor::Editor::default();editor.apply(crate::editor::Action::Close).unwrap();editor.apply(crate::editor::Action::AddCurve).unwrap();editor.apply(crate::editor::Action::Controls(true)).unwrap();
    let owner=editor.scene.paths()[0].id;let nets=editor.scene.control_nets();assert_eq!(nets.len(),2);let markers=editor.scene.control_markers();
    for(parent,line)in &nets{assert_eq!(*parent,owner);assert!(markers.iter().any(|m|m.display.center==line.start));assert!(markers.iter().any(|m|m.display.center==line.end));assert_eq!(line.width,1.);}
    let source=crate::source_edit::original(&editor.scene,owner).unwrap();
    for density in [1.,2.] {
        let mut renderer=renderer(&editor.scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});renderer.set_pick_mode(crate::pick_sources::Mode::Object);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([owner.value(),0]));
        for mode in [crate::pick_sources::Mode::Edge,crate::pick_sources::Mode::Control]{renderer.set_pick_mode(mode);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert!(finish(&mut renderer).is_none());}
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,owner).unwrap()));editor.scene.show_controls(false);assert!(editor.scene.control_nets().is_empty());
}

#[test]
fn actual_edited_element_controls_move_their_integer_pixels_and_undo() {
    use crate::pick_sources::{Mode,Part,Target};
    let mut mesh=session_rust::Mesh::new();for(key,p)in[(10,[-0.5,-0.5,0.25]),(20,[0.5,-0.5,0.25]),(30,[0.,0.5,0.25])]{mesh.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));}mesh.add_face(vec![10,20,30],Some(37));
    let mut element=session_rust::Element::new("edited control pixel");element.set_geometry(mesh);let mut base=Scene::empty();let owner=crate::element_original::insert(&mut base,std::rc::Rc::new(element)).unwrap();base.owner_controls(owner).unwrap();let original=crate::source_edit::original(&base,owner).unwrap();let target=Target{owner,part:Part::SourceControl(crate::source_controls::ControlId::Vertex(10))};
    for density in [1.,2.] {
        let mut scene=base.clone();let mut renderer=renderer(&scene);renderer.set_density(density);renderer.resize(Viewport{width:(320.*density)as u32,height:(240.*density)as u32});renderer.set_pick_mode(Mode::Control);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
        let mut preview=crate::part_preview::Preview::new(&scene,target).unwrap();preview.set(session_rust::Xform::translation(0.25,0.,0.)).unwrap();renderer.set_scene(&preview.scene,None);renderer.request_pick([-0.25,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&preview.scene,pixel),Some(target));renderer.request_pick([-0.5,-0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());
        let mut history=crate::history::History::default();assert!(preview.commit(&mut scene,&mut history).unwrap());renderer.set_scene(&scene,None);renderer.request_pick([-0.25,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
        assert!(history.undo(&mut scene));renderer.set_scene(&scene,None);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));assert!(crate::source_edit::same(&original,&crate::source_edit::original(&scene,owner).unwrap()));
    }
}

#[test]
fn actual_pulled_source_normals_keep_shared_reflected_placements_and_original_picks() {
    let mut source=session_rust::Mesh::new();
    for(key,p)in[(10,[-0.2,-0.4,0.3]),(20,[0.2,-0.4,0.3]),(30,[0.,0.4,0.3])]{source.add_vertex(session_rust::Point::new(p[0],p[1],p[2]),Some(key));for(name,value)in[("nx",4.),("ny",0.),("nz",3.)]{source.set_vertex_attribute(key,name,value);}}
    source.add_face(vec![30,10,20],Some(37));let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let b=scene.copy_row(a,false).unwrap();
    scene.place(a,session_rust::Xform::translation(-0.5,0.,0.)).unwrap();let mut reflected=session_rust::Xform::identity();reflected.m[0]=-1.;reflected.m[12]=0.5;scene.place(b,reflected).unwrap();assert!(Rc::ptr_eq(scene.objects()[0].geometry().unwrap(),scene.objects()[1].geometry().unwrap()));
    let mut gpu=renderer(&scene);let usage=gpu.stats();
    for density in [1.,2.] {gpu.set_density(density);let pixels=colour_pixels_display(&mut gpu,identity(),true);
        for(x,expected,owner,position)in[(80,[230u8,128,204],a,-0.5),(240,[230u8,128,51],b,0.5)]{let pixel=&pixels[(132*320+x)*4..][..3];for(actual,expected)in pixel.iter().zip(expected){assert!(actual.abs_diff(expected)<=2,"Actual placed source normal: {pixel:?}");}gpu.request_pick([position,-0.1],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],owner.value());}
        assert_eq!(&gpu.stats()[..2],&usage[..2]);
    }
    assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(scene.objects()[1].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_copied_originals_share_instance_records_keep_original_picks_and_return_to_ordinary_drawing() {
 let mut scene=Scene::empty();scene.next_id=16_777_217;let a=scene.insert(PreparedMesh::triangle([[-0.18,-0.2,0.4],[0.18,-0.2,0.4],[0.,0.2,0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(a,[-0.45,0.,0.]).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(b,[0.9,0.,0.]).unwrap();assert!(scene.instances.is_empty());assert!(Rc::ptr_eq(&scene.objects()[0].mesh,&scene.objects()[1].mesh));let original=scene.objects()[0].geometry().unwrap().to_proto();let weak=Rc::downgrade(&scene.objects()[0].mesh);
 {let classify=|limit,declared|crate::instance_gpu::batch_rows(scene.objects().iter().map(|row|(row.id,Rc::as_ptr(&row.mesh)as usize,declared==Some(row.id))),limit);assert!(classify(1,None).is_empty());assert_eq!(classify(2,None).len(),2);assert_eq!(classify(1,Some(b)),[b].into_iter().collect());}
 let mut gpu=renderer(&scene);gpu.set_opacity(1.);assert_eq!(&gpu.instance_usage()[..3],&[1,2,376]);let uploads=gpu.stats()[0];let plain=colour_pixels(&mut gpu);let mut blue=[0;2];for line in plain.chunks_exact(320*4){for(x,p)in line.chunks_exact(4).enumerate(){if p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10){blue[x/160]+=1;}}}assert!(blue.into_iter().all(|n|n>100));
 for(id,x)in[(a,-0.45),(b,0.45)]{gpu.request_pick([x,0.],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],id.value());}
 gpu.select_objects(&scene,&[b]).unwrap();let selected=colour_pixels(&mut gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>140&&p[1]>90&&p[2]<40));gpu.select_objects(&scene,&[]).unwrap();assert_eq!(colour_pixels(&mut gpu),plain);
 Rc::make_mut(&mut scene.view).hide(&[b],true);gpu.set_scene(&scene,None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,376]);gpu.request_pick([-0.45,0.],identity()).unwrap();assert_eq!(finish(&mut gpu).unwrap()[0],a.value());
 Rc::make_mut(&mut scene.view).hide(&[b],false);gpu.set_scene(&scene,None);assert_eq!(&gpu.instance_usage()[..3],&[1,2,376]);assert_eq!(colour_pixels(&mut gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert!(scene.instances.is_empty());for row in scene.objects(){assert_eq!(row.geometry().unwrap().to_proto(),original);}
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert!(weak.upgrade().is_none());
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-22.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/primitive_slot_table_tests.rs`

Supply actual lookup shader readback for inactive and reactivated retained rows, while checking unchanged draw-slot bytes and all earlier reuse, shrink, high-ID and release assertions.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{primitive_slot_table::Table,text_gpu_support::GpuCtx};
fn read(ctx:&GpuCtx,table:&Table)->Vec<[u32;4]>{
 let size=table.texture.size();let pitch=(size.width*16+255)/256*256;let bytes=u64::from(pitch)*u64::from(size.height);
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("slot texture readback"),size:bytes,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=ctx.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(table.texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&output,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(size.height)}},size);ctx.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();output.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let mapped=output.slice(..).get_mapped_range();let result=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size.width as usize*16].chunks_exact(16).map(|texel|std::array::from_fn(|i|u32::from_le_bytes(texel[i*4..i*4+4].try_into().unwrap())))).collect();drop(mapped);output.unmap();result
}
#[test]
fn actual_integer_slot_texture_preserves_ids_across_partial_row_writes_and_release(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
 let mut wanted=(0..1050).map(|i|[i,0x80000000+i,16_777_217+i,3]).collect::<Vec<_>>();table.write(&ctx,&wanted);assert_eq!([table.texture.width(),table.texture.height()],[1024,2]);assert_eq!(table.texture.format(),wgpu::TextureFormat::Rgba32Uint);let texture=table.texture.clone();wanted.resize(2048,[0;4]);assert_eq!(read(&ctx,&table),wanted);
 let changed=[[7,u32::MAX-1,31,9],[11,0x80000000,37,12],[13,16_777_217,41,15]];assert!(table.write_at(&ctx,1023,&changed));wanted[1023..1026].copy_from_slice(&changed);assert_eq!(table.texture,texture);assert_eq!(read(&ctx,&table),wanted);
 assert!(!table.write_at(&ctx,u32::MAX,&changed));assert!(!table.write_at(&ctx,2047,&changed));assert!(table.write_at(&ctx,2048,&[]));assert_eq!(read(&ctx,&table),wanted);
 table.write(&ctx,&[]);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);assert!(!table.write_at(&ctx,0,&changed));assert_eq!(read(&ctx,&table),vec![[0;4]]);
}

#[test]
fn actual_allocator_publication_keeps_high_rows_free_slots_neighbours_and_release() {
 use crate::slot_space::{Definition,SlotSpace};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();
 let mut a=[7,0x80000001];let b=[u32::MAX-1];
 fn definitions<'a>(a:&'a[u32],b:&'a[u32])->Vec<Definition<'a>>{vec![Definition {key:1,rows:a,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:b,faces:60..63,pipes:0..0,ribbons:0..0}]}
 let change=space.reset(&definitions(&a,&b),100);assert!(table.publish(&ctx,&space,&change));let texture=table.texture.clone();let bytes=read(&ctx,&table);assert_eq!(&bytes[..4],&[[3,100,0,0],[7,100,10,2],[0x80000001,102,10,2],[u32::MAX-1,104,20,1]]);
 a[0]=0x80000007;let change=space.update(&definitions(&a,&b),&[(1,0)],100);assert!(!change.all);assert!(!table.publish(&ctx,&space,&change));assert_eq!(table.texture,texture);let next=read(&ctx,&table);assert_eq!(next[1],[0x80000007,100,10,2]);assert_eq!(&next[2..],&bytes[2..]);
 let remaining=[Definition {key:2,rows:&b,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);table.publish(&ctx,&space,&change);let next=read(&ctx,&table);assert_eq!(next[1][3],0);assert_eq!(next[2][3],0);assert_eq!(next[3],[u32::MAX-1,104,20,1]);
 let change=space.reset(&[],0);assert!(table.publish(&ctx,&space,&change));assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
}

#[test]
fn actual_slot_shader_resolves_arena_instances_holes_high_rows_and_empty_table() {
 use wgpu::util::DeviceExt;use crate::slot_space::{Definition,SlotSpace};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();let mut slots=crate::draw_slots::Slots::new(&ctx);
 let rows=[7,0x80000001];let other=[u32::MAX-1];let definitions=[Definition {key:1,rows:&rows,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:&other,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.reset(&definitions,100);slots.publish(&ctx,&space,&change,&mut table);
 let source=format!("@group(0) @binding(0) var slot_table: texture_2d<u32>;\n@group(0) @binding(1) var<storage,read_write> output: array<vec2<u32>>;\n{}\n@compute @workgroup_size(1) fn run(@builtin(global_invocation_id) p: vec3<u32>) {{ let ids=array<u32,8>(0u,100u,101u,102u,103u,104u,105u,106u);output[p.x]=placed_triangle(ids[p.x],100u); }}",include_str!("primitive_slot_lookup.wgsl"));
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("actual integer slot consumer"),source:wgpu::ShaderSource::Wgsl(source.into())});let pipeline=ctx.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {label:None,layout:None,module:&shader,entry_point:Some("run"),compilation_options:Default::default(),cache:None});
 let output=ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:None,contents:&[0;64],usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC});let mapped=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:None,size:64,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});
 let resolve=|table:&Table| {let group=ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {label:None,layout:&pipeline.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry {binding:0,resource:wgpu::BindingResource::TextureView(&table.view)},wgpu::BindGroupEntry {binding:1,resource:output.as_entire_binding()}]});let mut encoder=ctx.device.create_command_encoder(&Default::default());{let mut pass=encoder.begin_compute_pass(&Default::default());pass.set_pipeline(&pipeline);pass.set_bind_group(0,&group,&[]);pass.dispatch_workgroups(8,1,1);}encoder.copy_buffer_to_buffer(&output,0,&mapped,0,64);ctx.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();mapped.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=mapped.slice(..).get_mapped_range();let result=bytes.chunks_exact(8).map(|v|[u32::from_le_bytes(v[..4].try_into().unwrap()),u32::from_le_bytes(v[4..].try_into().unwrap())]).collect::<Vec<_>>();drop(bytes);mapped.unmap();result};
 let none=[u32::MAX;2];assert_eq!(resolve(&table),vec![none,[99,u32::MAX],[10,7],[11,7],[10,0x80000001],[11,0x80000001],[20,u32::MAX-1],none]);
 let mut cross=vec![[0,100,0,0];1026];cross[0]=[1025,100,0,0];cross[1024]=[0x80000001,100,30,2];cross[1025]=[u32::MAX-1,104,40,1];table.write(&ctx,&cross);assert_eq!(resolve(&table),vec![none,[99,u32::MAX],[30,0x80000001],[31,0x80000001],none,none,[40,u32::MAX-1],none]);table.write(&ctx,&(0..space.end()).map(|i|space.texel(i)).collect::<Vec<_>>());
 let remaining=[Definition {key:2,rows:&other,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);slots.publish(&ctx,&space,&change,&mut table);let values=resolve(&table);assert_eq!(&values[2..6],&[none;4]);assert_eq!(values[6],[20,u32::MAX-1]);
 let joined=[Definition {key:2,rows:&other,faces:60..63,pipes:0..0,ribbons:0..0},Definition {key:3,rows:&[11],faces:90..93,pipes:0..0,ribbons:0..0}];let change=space.update(&joined,&[],100);slots.publish(&ctx,&space,&change,&mut table);assert_eq!(resolve(&table),vec![none,[99,u32::MAX],[30,11],none,none,none,[20,u32::MAX-1],none]);
 let shortened=[Definition {key:3,rows:&[11],faces:90..93,pipes:0..0,ribbons:0..0}];let change=space.update(&shortened,&[],100);slots.publish(&ctx,&space,&change,&mut table);assert_eq!(read(&ctx,&table)[0],[1,100,0,0]);assert_eq!(resolve(&table),vec![none,[99,u32::MAX],[30,11],none,none,none,none,none]);
 let change=space.reset(&[],0);slots.publish(&ctx,&space,&change,&mut table);let values=resolve(&table);assert_eq!(values[1],[99,u32::MAX]);assert_eq!(values[0],none);assert!(values[2..].iter().all(|v|*v==none));
}

#[test]
fn actual_local_slots_and_lookup_share_one_plan_through_changes_holes_growth_and_release() {
 use crate::{draw_slots::{Slots,OWN_ROW},slot_space::{Definition,SlotSpace}};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut slots=Slots::new(&ctx);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();let mut a=[7,0x80000001];let b=[u32::MAX-1];
 fn definitions<'a>(a:&'a[u32],b:&'a[u32])->Vec<Definition<'a>>{vec![Definition {key:1,rows:a,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:b,faces:60..63,pipes:0..0,ribbons:0..0}]}
 let change=space.reset(&definitions(&a,&b),100);assert!(slots.publish(&ctx,&space,&change,&mut table));let buffer=slots.buffer.buf.clone();let texture=table.texture.clone();let wanted=[[OWN_ROW,0],[7,100],[0x80000001,102],[u32::MAX-1,104]].into_iter().flatten().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,&buffer,32),wanted);assert_eq!(&read(&ctx,&table)[..4],&[[3,100,0,0],[7,100,10,2],[0x80000001,102,10,2],[u32::MAX-1,104,20,1]]);
 a[0]=0x80000007;let specs=definitions(&a,&b);let touched=space.touched(&specs);let change=space.update(&specs,&touched,100);assert!(!slots.publish(&ctx,&space,&change,&mut table));assert_eq!(slots.buffer.buf,buffer);assert_eq!(table.texture,texture);assert_eq!(slots.written_bytes,8);let changed=crate::growing_buffer_tests::read(&ctx,&buffer,32);assert_eq!(&changed[..8],&wanted[..8]);assert_eq!(&changed[16..],&wanted[16..]);assert_eq!(u32::from_ne_bytes(changed[8..12].try_into().unwrap()),0x80000007);
 let change=space.update(&specs,&[],100);slots.publish(&ctx,&space,&change,&mut table);assert_eq!(slots.written_bytes,0);
 let remaining=[Definition {key:2,rows:&b,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);slots.publish(&ctx,&space,&change,&mut table);let values=read(&ctx,&table);assert_eq!(values[1][3],0);assert_eq!(values[2][3],0);assert_eq!(values[3],[u32::MAX-1,104,20,1]);assert_eq!(slots.instances(),3);
 let joined=[Definition {key:2,rows:&b,faces:60..63,pipes:0..0,ribbons:0..0},Definition {key:3,rows:&[11],faces:90..93,pipes:0..0,ribbons:0..0}];let change=space.update(&joined,&[],100);slots.publish(&ctx,&space,&change,&mut table);assert_eq!(space.local_slot(1),[11,100]);let lookup=read(&ctx,&table);assert_eq!(lookup[1],[11,100,30,1]);assert_eq!(lookup[2],[0x80000001,101,0,0]);assert_eq!(slots.written_bytes,16);assert_eq!(slots.buffer.buf,buffer);let wanted=[[OWN_ROW,0],[11,100],[0x80000001,101],[u32::MAX-1,104]].into_iter().flatten().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,&buffer,32),wanted);
 let shortened=[Definition {key:3,rows:&[11],faces:90..93,pipes:0..0,ribbons:0..0}];let change=space.update(&shortened,&[],100);slots.publish(&ctx,&space,&change,&mut table);assert_eq!(slots.instances(),1);assert_eq!(slots.written_bytes,0);assert_eq!(slots.buffer.buf,buffer);assert_eq!(read(&ctx,&table)[0],[1,100,0,0]);
 let change=space.follow(120);assert!(change.all);slots.publish(&ctx,&space,&change,&mut table);let wanted=(0..space.end()).map(|i|space.texel(i)).collect::<Vec<_>>();assert_eq!(&read(&ctx,&table)[..wanted.len()],wanted);let wanted=std::iter::once([OWN_ROW,0]).chain((1..space.end()).map(|i|space.local_slot(i))).flatten().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,wanted.len()as u64),wanted);
 let change=space.reset(&[],0);assert!(slots.publish(&ctx,&space,&change,&mut table));assert_eq!(slots.instances(),0);assert_eq!(slots.allocated_bytes(),8);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);let sentinel=slots.buffer.buf.clone();slots.publish(&ctx,&space,&change,&mut table);assert_eq!(slots.buffer.buf,sentinel);assert_eq!(slots.written_bytes,0);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-23.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/global_outline_tests.rs`

Retain all original owner, background, selection, overlay, crop, density and reserved-ID guards while testing placed physical intervals after the arena range; compare later placements with their assigned non-overlapping object records.

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
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![0,1]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![0,1]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![0,1]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![0]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],1);assert_eq!(before[120*320+240],2);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(placed[120*320+64],1);assert_eq!(placed[120*320+256],2);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,1u32),([0.5,0.],b,2)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],1,"Visible intervals must rebuild without a color frame");
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
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-24.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/tile_gpu_tests.rs`

Read the first placed projected polygon after the two original demo arena triangles; retain exact clipping-corner, tile-depth, list bounds, cache and release checks.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(not(target_arch="wasm32"))]
#[test]
fn actual_finite_gpu_lists_cache_projection_and_drop_deleted_geometry(){
 use crate::{renderer::Renderer,scene::Scene,viewport::Viewport};
 let scene=Scene::demo();let instance=wgpu::Instance::new(wgpu::InstanceDescriptor{backends:wgpu::Backends::VULKAN,flags:Default::default(),memory_budget_thresholds:Default::default(),backend_options:Default::default(),display:None});let adapter=pollster::block_on(instance.request_adapter(&Default::default())).unwrap();let(device,queue)=pollster::block_on(adapter.request_device(&Default::default())).unwrap();device.on_uncaptured_error(std::sync::Arc::new(|error|panic!("actual finite GPU: {error}")));
 let mut renderer=Renderer::new(device,queue,wgpu::TextureFormat::Rgba8Unorm,&scene);renderer.resize(Viewport{width:320,height:240});let texture=renderer.device.create_texture(&wgpu::TextureDescriptor{label:Some("finite actual frame"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});let view=texture.create_view(&Default::default());let camera=crate::camera::Camera::default();let matrix=camera.uniform();
 renderer.draw(&view,&Default::default(),&matrix);renderer.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();let buffers=renderer.tile_buffers();let read=renderer.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual projected record and tile header"),size:96+buffers[1].size(),usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=renderer.device.create_command_encoder(&Default::default());encoder.copy_buffer_to_buffer(&buffers[0],0,&read,0,96);encoder.copy_buffer_to_buffer(&buffers[1],0,&read,96,buffers[1].size());renderer.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());renderer.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let corners=f32::from_ne_bytes(bytes[60..64].try_into().unwrap());assert!((3.0..=4.0).contains(&corners),"actual near-clipped polygon: {corners}");assert_eq!(u32::from_ne_bytes(bytes[96..100].try_into().unwrap()),1);assert!(u64::from(u32::from_ne_bytes(bytes[100..104].try_into().unwrap()))<=buffers[1].size()/4);let words=|offset:usize|u32::from_ne_bytes(bytes[offset..offset+4].try_into().unwrap());let mut occupied=0;for tile in 1..=4800{let at=96+tile*16;let count=words(at);if count==0{continue}occupied+=1;let start=words(at+4)as usize;assert_eq!(words(at+8),count);assert_eq!(words(at+12),0);assert!((start as u64+u64::from(count)*2)*4<=buffers[1].size());for i in 0..count as usize{let primitive=words(96+(start+i*2)*4);assert!(primitive>0&&u64::from(primitive)<=renderer.tile_usage()[3]);let depth=f32::from_ne_bytes(bytes[96+(start+i*2+1)*4..96+(start+i*2+2)*4].try_into().unwrap());assert!(depth.is_finite());}}assert!(occupied>0,"actual rasterized tile lists contain triangles");drop(bytes);read.unmap();
 let first=renderer.tile_usage();assert!(first[3]>0&&first[0]>=first[3]*96);assert_eq!(first[5..],[1,1]);
 renderer.draw(&view,&Default::default(),&matrix);renderer.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();assert_eq!(renderer.tile_usage()[5..],[1,1]);
 let mut camera=camera;camera.orbit(0.1,0.05);renderer.draw(&view,&Default::default(),&camera.uniform());renderer.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();assert_eq!(renderer.tile_usage()[5..],[2,2]);
 drop(buffers);renderer.set_scene(&Scene::empty(),None);renderer.draw(&view,&Default::default(),&camera.uniform());renderer.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();let empty=renderer.tile_usage();assert_eq!(empty[..5],[96,16,48,0,0]);
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-25.rs"
```

</details>

### 12. `src/instance_gpu.rs`

Assign new record rows in original-ID order instead of geometry pointer order; retained rows and the original scene draw order remain unchanged.

<details>
<summary>Locate the existing block</summary>

```rust
        let ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut ids=groups.values().flat_map(|(_,rows)|rows.iter().map(|(id,_)|*id)).chain(own.iter().map(|mesh|mesh.id)).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-26.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzo-separate-projection-rows-and-arena-offsets"></a>

<details>
<summary>Implementation 96zzzzo-separate-projection-rows-and-arena-offsets: Separate projection rows and arena offsets</summary>

Make source addressing explicit before projection consumes the inverse lookup.

Data flow: Explicit object row and first index → shared projection helper → unchanged projected records.

### 1. `src/finite_project.wgsl`

Pass the model row and index offset explicitly instead of closing over one dispatch interval. Read exactly the requested shared record or the unchanged uniform object. Keep arena addressing separate from object-row selection. Let the same projection algorithm accept either local or global arena triangles. Use the same explicit address contract for every triangle corner. Preserve the current local draw behavior while preparing lookup-selected arena inputs.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
        return;
    }

    let polygon = project_physical_triangle(local+1u, 0u);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
```

</details>

Replace it with:

```wgsl
        return;
    }

    let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
```

<details>
<summary>Locate this block</summary>

```wgsl

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base), physical_clip_corner(base+1u), physical_clip_corner(base+2u));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
```

</details>

Replace it with:

```wgsl

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base,row,first), physical_clip_corner(base+1u,row,first), physical_clip_corner(base+2u,row,first));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
```

<details>
<summary>Locate this block</summary>

```wgsl
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let object=object_at(live_count.z);
 let point=packed_position(physical_vertices[physical_indices[index+object.range.x]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}

fn project_physical_triangle(primitive: u32, row: u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
```

</details>

Replace it with:

```wgsl
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32,row:u32,first:u32)->vec4<f32>{
 let object=object_at(row);
 let point=packed_position(physical_vertices[physical_indices[index+first]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}

fn project_physical_triangle(primitive: u32, row: u32, first:u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
```

<details>
<summary>Complete updated block reference</summary>

```wgsl
struct View { transform:mat4x4<f32>, size:vec2<f32>, density:f32 }
struct TileLine {thickness:f32,proj_y:f32,ortho_h:f32,vp_h:f32,vp_w:f32}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<PackedVertex>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
struct ProjectedPolygon {
    points: array<vec3<f32>,
    4>,
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32,row:u32,first:u32)->vec4<f32>{
 let object=object_at(row);
 let point=packed_position(physical_vertices[physical_indices[index+first]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}

fn project_physical_triangle(primitive: u32, row: u32, first:u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
        return polygon;
    }

    let base = (primitive-1u)*3u;

    let input = array<vec4<f32>, 3>(physical_clip_corner(base,row,first), physical_clip_corner(base+1u,row,first), physical_clip_corner(base+2u,row,first));
    // Homogeneous barycentric coordinates give z_ndc as an affine plane without
    // dividing corners by w. Its constant is the depth at ndc (0, 0).
    let edge0 = cross(input[1].xyw, input[2].xyw);
    let determinant = dot(edge0, input[0].xyw);

    if (determinant == 0.0) {
        return polygon;
    }

    let plane = (input[0].z * edge0 + input[1].z * cross(input[2].xyw, input[0].xyw)
        + input[2].z * cross(input[0].xyw, input[1].xyw)) / determinant;
    polygon.depth = vec3<f32>(2.0 * plane.x / line.vp_w, -2.0 * plane.y / line.vp_h, plane.z);
    var clipped: array<vec4<f32>, 4>;
    var previous = input[2];
    var previous_distance = previous.w - previous.z;

    // clip each edge against the near plane
    for (var i = 0u; i<3u; i++) {
        let current = input[i];
        let distance = current.w - current.z;

        if ((distance > 0.0 && previous_distance < 0.0) || (distance < 0.0 && previous_distance > 0.0)) {
            let t = previous_distance / (previous_distance-distance);
            clipped[polygon.count] = mix(previous, current, t);
            polygon.count++;
        }

        if (distance >= 0.0) {
            clipped[polygon.count] = current;
            polygon.count++;
        }

        previous = current;
        previous_distance = distance;
    }

    // clip space to screen pixels
    for (var i = 0u; i<polygon.count; i++) {
        let clip = clipped[i];

        if (clip.w <= 0.0) {
            polygon.count = 0u;
            return polygon;
        }

        let ndc = clip.xyz/clip.w;
        polygon.points[i] = vec3<f32>((ndc.x*0.5+0.5)*line.vp_w, (0.5-ndc.y*0.5)*line.vp_h, ndc.z);
    }

    return polygon;
}
fn physical_cross(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x*b.y-a.y*b.x;
}
fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // zero only for a degenerate polygon
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // rows of 32768 workgroups (PROJECT_ROW_GROUPS): a dispatch dimension holds at most 65535
    let local = id.x + id.y * 2097152u;
    let index = local + live_count.y;

    if (local>=live_count.x) {
        return;
    }

    let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
    if (polygon.count>=3u) {
        // screen box
        var lo = polygon.points[0].xy;
        var hi = lo;

        for (var i = 1u;i<polygon.count;i++) {
            lo = min(lo, polygon.points[i].xy);
            hi = max(hi, polygon.points[i].xy);
        }

        let area = physical_polygon_area(polygon);

        // degenerate: empty record
        if (abs(area)<1e-12) {
            projected[index] = out;
            return;
        }

        // one inward line equation per edge
        var equations: array<vec4<f32>, 4>;

        for (var i = 0u;i<polygon.count;i++) {
            let a = polygon.points[i].xy;
            let edge = polygon.points[(i+1u)%polygon.count].xy-a;
            let normal = sign(area)*vec2<f32>(-edge.y, edge.x)/length(edge);
            equations[i] = vec4<f32>(normal, -dot(normal, a), 0.0);
        }

        // Small far triangles need a nearby reference; extrapolating their homogeneous
        // plane to the viewport centre magnifies cancellation. Huge near-clipped faces
        // still use that centre, keeping their million-pixel corners out of the depth fit.
        var reference = vec3<f32>(line.vp_w * 0.5, line.vp_h * 0.5, polygon.depth.z);
        var gradient = polygon.depth.xy;
        let viewport = vec2<f32>(line.vp_w, line.vp_h);
        if (polygon.count == 3u && all(lo >= -viewport) && all(hi <= 2.0 * viewport)) {
            let ab = polygon.points[1] - polygon.points[0];
            let ac = polygon.points[2] - polygon.points[0];
            gradient = vec2<f32>(ab.z * ac.y - ac.z * ab.y, ab.x * ac.z - ac.x * ab.z) / area;
            reference = polygon.points[0];
        }
        out.edge0 = vec4<f32>(equations[0].xyz, reference.x);
        out.edge1 = vec4<f32>(equations[1].xyz, reference.y);
        out.edge2 = vec4<f32>(equations[2].xyz, reference.z);
        out.edge3 = vec4<f32>(equations[3].xyz, f32(polygon.count));
        // nearest corner depth
        var nearest = polygon.points[0].z;

        for (var i = 1u;i<polygon.count;i++) {
            nearest = max(nearest, polygon.points[i].z);
        }

        out.gradient = vec4<f32>(gradient, nearest, 0.0);
        out.bounds = vec4<f32>(lo, hi);
    }

    projected[index] = out;
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzo-separate-projection-rows-and-arena-offsets-01.rs"
```

```text
--8<-- "journey/code/96zzzzo-separate-projection-rows-and-arena-offsets-02.rs"
```

```text
--8<-- "journey/code/96zzzzo-separate-projection-rows-and-arena-offsets-03.rs"
```

```text
--8<-- "journey/code/96zzzzo-separate-projection-rows-and-arena-offsets-04.rs"
```

```text
--8<-- "journey/code/96zzzzo-separate-projection-rows-and-arena-offsets-05.rs"
```

```text
--8<-- "journey/code/96zzzzo-separate-projection-rows-and-arena-offsets-06.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzp-prepare-lookup-selected-triangle-projection"></a>

<details>
<summary>Implementation 96zzzzp-prepare-lookup-selected-triangle-projection: Prepare lookup-selected triangle projection</summary>

Resolve physical primitive IDs to their shared source triangle and object row.

Data flow: Physical primitive ID → exact unsigned lookup → arena triangle and object row → shared projection.

### 1. `src/finite_project_source.rs`

Assemble lookup-selected projection from the existing shader and unsigned lookup helper; keep one clipping and projection algorithm.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn lookup()->String {
 let source=records().replace("let index = local + live_count.y;","let index = local;").replace("let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);","let source=placed_triangle(index+1u,live_count.y);if(source.x==NO_TRIANGLE||source.y==SLOT_OWN_ROW){return;}let polygon=project_physical_triangle(source.x+1u,source.y,0u);");
 format!("@group(2) @binding(5) var slot_table:texture_2d<u32>;\n{}\n{}",include_str!("primitive_slot_lookup.wgsl"),source)
}

#[cfg(test)]
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzp-prepare-lookup-selected-triangle-projection-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/finite_project_source.rs`

Supply validation of uniform, shared per-mesh and lookup-selected compute interfaces.

<details>
<summary>Locate the existing block</summary>

```text
[super::source(),super::records()]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzp-prepare-lookup-selected-triangle-projection-02.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzq-prepare-the-lookup-projection-pipeline"></a>

<details>
<summary>Implementation 96zzzzq-prepare-the-lookup-projection-pipeline: Prepare the lookup projection pipeline</summary>

Add the inverse lookup texture to the existing compute binding recipe.

Data flow: Existing compute recipe plus unsigned lookup texture → prepared projection pipeline.

### 1. `src/tile_pipelines.rs`

Keep the lookup projection layout beside the two existing resource contracts. Own the lookup state with the existing tile pipelines. Extend the existing binding recipe with one unsigned texture input instead of duplicating the compute descriptor. Select the shared projection algorithm assembled with the inverse lookup. Prepare uniform, shared per-mesh and lookup states through one constructor. Retain the new layout under the existing pipeline lifetime. Retain the prepared compute state without changing dispatches yet.

<details>
<summary>Locate the existing block</summary>

```rust
fn compute(device:&wgpu::Device,source:&str,entry_point:&str,groups:&[Option<&wgpu::BindGroupLayout>])->wgpu::ComputePipeline{let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some(entry_point),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some(entry_point),bind_group_layouts:groups,immediate_size:0});device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{label:Some(entry_point),layout:Some(&layout),module:&shader,entry_point:Some(entry_point),compilation_options:Default::default(),cache:None})}
fn raster(device:&wgpu::Device,shader:&wgpu::ShaderModule,layout:&wgpu::PipelineLayout,entry:&str)->wgpu::RenderPipeline{device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some(entry),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(entry),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()})]}),primitive:wgpu::PrimitiveState{topology:wgpu::PrimitiveTopology::TriangleStrip,..Default::default()},depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})}
use wgpu::{BufferBindingType as Type,ShaderStages as Stage};
pub fn entry(binding:u32,visibility:Stage,ty:Type)->wgpu::BindGroupLayoutEntry{wgpu::BindGroupLayoutEntry{binding,visibility,ty:wgpu::BindingType::Buffer{ty,has_dynamic_offset:false,min_binding_size:None},count:None}}
pub fn group(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,buffers:&[&wgpu::Buffer])->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite triangle buffers"),layout,entries:&buffers.iter().enumerate().map(|(binding,buffer)|wgpu::BindGroupEntry{binding:binding as u32,resource:buffer.as_entire_binding()}).collect::<Vec<_>>()})}
pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub record_project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub record_project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let layout=|label,entries:&[wgpu::BindGroupLayoutEntry]|device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some(label),entries});
  let line=layout("finite view dimensions",&[entry(0,Stage::VERTEX_FRAGMENT|Stage::COMPUTE,Type::Uniform)]);
  let create=|shared| {let project_layout=layout("finite original projection",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:true}),entry(1,Stage::COMPUTE,Type::Storage{read_only:true}),entry(2,Stage::COMPUTE,if shared {Type::Storage{read_only:true}}else{Type::Uniform}),entry(3,Stage::COMPUTE,Type::Storage{read_only:false}),entry(4,Stage::COMPUTE,Type::Uniform)]);
   let source=if shared {crate::finite_project_source::records()}else{crate::finite_project_source::source()};let project=compute(device,&source,"cs_main",&[Some(view),Some(&line),Some(&project_layout)]);(project_layout,project)};
  let(project_layout,project)=create(false);let(record_project_layout,record_project)=create(true);
  let raster_layout=layout("finite triangle tiles",&[entry(0,Stage::VERTEX,Type::Storage{read_only:true}),entry(1,Stage::FRAGMENT,Type::Storage{read_only:false})]);
  let scan_layout=layout("finite prefix scan",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:false})]);

  let scans=["scan_tiles","scan_blocks","finish_offsets"].map(|entry|compute(device,&crate::finite_project_source::scan(),entry,&[Some(&line),Some(&scan_layout)]));
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite bins"),source:wgpu::ShaderSource::Wgsl(crate::finite_project_source::tiles().into())});
  let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite bins"),bind_group_layouts:&[Some(&line),Some(&raster_layout)],immediate_size:0});

  let count=raster(device,&shader,&layout,"fs_count");let fill=raster(device,&shader,&layout,"fs_fill");Self{line,project_layout,record_project_layout,raster_layout,scan_layout,project,record_project,count,fill,scans}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
fn compute(device:&wgpu::Device,source:&str,entry_point:&str,groups:&[Option<&wgpu::BindGroupLayout>])->wgpu::ComputePipeline{let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some(entry_point),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some(entry_point),bind_group_layouts:groups,immediate_size:0});device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{label:Some(entry_point),layout:Some(&layout),module:&shader,entry_point:Some(entry_point),compilation_options:Default::default(),cache:None})}
fn raster(device:&wgpu::Device,shader:&wgpu::ShaderModule,layout:&wgpu::PipelineLayout,entry:&str)->wgpu::RenderPipeline{device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some(entry),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(entry),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()})]}),primitive:wgpu::PrimitiveState{topology:wgpu::PrimitiveTopology::TriangleStrip,..Default::default()},depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})}
use wgpu::{BufferBindingType as Type,ShaderStages as Stage};
pub fn entry(binding:u32,visibility:Stage,ty:Type)->wgpu::BindGroupLayoutEntry{wgpu::BindGroupLayoutEntry{binding,visibility,ty:wgpu::BindingType::Buffer{ty,has_dynamic_offset:false,min_binding_size:None},count:None}}
pub fn group(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,buffers:&[&wgpu::Buffer])->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite triangle buffers"),layout,entries:&buffers.iter().enumerate().map(|(binding,buffer)|wgpu::BindGroupEntry{binding:binding as u32,resource:buffer.as_entire_binding()}).collect::<Vec<_>>()})}
pub struct Pipelines{pub line:wgpu::BindGroupLayout,pub project_layout:wgpu::BindGroupLayout,pub record_project_layout:wgpu::BindGroupLayout,pub lookup_project_layout:wgpu::BindGroupLayout,pub raster_layout:wgpu::BindGroupLayout,pub scan_layout:wgpu::BindGroupLayout,pub project:wgpu::ComputePipeline,pub record_project:wgpu::ComputePipeline,pub lookup_project:wgpu::ComputePipeline,pub count:wgpu::RenderPipeline,pub fill:wgpu::RenderPipeline,pub scans:[wgpu::ComputePipeline;3]}
impl Pipelines{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let layout=|label,entries:&[wgpu::BindGroupLayoutEntry]|device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some(label),entries});
  let line=layout("finite view dimensions",&[entry(0,Stage::VERTEX_FRAGMENT|Stage::COMPUTE,Type::Uniform)]);
  let create=|shared,lookup| {let mut entries=vec![entry(0,Stage::COMPUTE,Type::Storage{read_only:true}),entry(1,Stage::COMPUTE,Type::Storage{read_only:true}),entry(2,Stage::COMPUTE,if shared {Type::Storage{read_only:true}}else{Type::Uniform}),entry(3,Stage::COMPUTE,Type::Storage{read_only:false}),entry(4,Stage::COMPUTE,Type::Uniform)];if lookup{entries.push(crate::text_gpu_support::texture_entry(5,Stage::COMPUTE,wgpu::TextureSampleType::Uint,false));}let project_layout=layout("finite original projection",&entries);
   let source=if lookup {crate::finite_project_source::lookup()}else if shared {crate::finite_project_source::records()}else{crate::finite_project_source::source()};let project=compute(device,&source,"cs_main",&[Some(view),Some(&line),Some(&project_layout)]);(project_layout,project)};
  let(project_layout,project)=create(false,false);let(record_project_layout,record_project)=create(true,false);let(lookup_project_layout,lookup_project)=create(true,true);
  let raster_layout=layout("finite triangle tiles",&[entry(0,Stage::VERTEX,Type::Storage{read_only:true}),entry(1,Stage::FRAGMENT,Type::Storage{read_only:false})]);
  let scan_layout=layout("finite prefix scan",&[entry(0,Stage::COMPUTE,Type::Storage{read_only:false})]);

  let scans=["scan_tiles","scan_blocks","finish_offsets"].map(|entry|compute(device,&crate::finite_project_source::scan(),entry,&[Some(&line),Some(&scan_layout)]));
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite bins"),source:wgpu::ShaderSource::Wgsl(crate::finite_project_source::tiles().into())});
  let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite bins"),bind_group_layouts:&[Some(&line),Some(&raster_layout)],immediate_size:0});

  let count=raster(device,&shader,&layout,"fs_count");let fill=raster(device,&shader,&layout,"fs_fill");Self{line,project_layout,record_project_layout,lookup_project_layout,raster_layout,scan_layout,project,record_project,lookup_project,count,fill,scans}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzq-prepare-the-lookup-projection-pipeline-01.rs"
```

```text
--8<-- "journey/code/96zzzzq-prepare-the-lookup-projection-pipeline-02.rs"
```

```text
--8<-- "journey/code/96zzzzq-prepare-the-lookup-projection-pipeline-03.rs"
```

```text
--8<-- "journey/code/96zzzzq-prepare-the-lookup-projection-pipeline-04.rs"
```

```text
--8<-- "journey/code/96zzzzq-prepare-the-lookup-projection-pipeline-05.rs"
```

```text
--8<-- "journey/code/96zzzzq-prepare-the-lookup-projection-pipeline-06.rs"
```

```text
--8<-- "journey/code/96zzzzq-prepare-the-lookup-projection-pipeline-07.rs"
```

</details>

</details>

<a id="checkpoint-96zzzzr-own-current-lookup-projection-bindings"></a>

<details>
<summary>Implementation 96zzzzr-own-current-lookup-projection-bindings: Own current lookup projection bindings</summary>

Keep current shared inputs and replace immutable projection bindings when needed.

Data flow: Current shared inputs → one dispatch interval → output-aware immutable bind group.

### 1. `src/projection_lookup.rs`

Own immutable current arena/record/lookup bindings and one bounded dispatch interval; replace the group when the output buffer changes.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzr-own-current-lookup-projection-bindings-01.rs"
```

### 2. `src/lib.rs`

Declare the binding owner without changing live projection dispatch.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod finite_project_source;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod projection_lookup;
mod finite_project_source;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzr-own-current-lookup-projection-bindings-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/global_outline_tests.rs`

Supply actual mapped dispatch words, exact immutable-group reuse and replacement, missing/current-record refusal and empty checks.

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
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzr-own-current-lookup-projection-bindings-03.rs" skip=34426
```

</details>

</details>

<a id="checkpoint-96zzzzs-project-live-triangles-through-the-lookup"></a>

<details>
<summary>Implementation 96zzzzs-project-live-triangles-through-the-lookup: Project live triangles through the lookup</summary>

Resolve shared source triangles and object rows on the GPU for live projection.

Data flow: Physical IDs → GPU inverse lookup → original arena triangle and row → projected records → existing tile bins.

### 1. `src/projection_lookup.rs`

Release cached output references when tile resources are replaced or released.

<details>
<summary>Locate the existing block</summary>

```rust
 pub fn bytes(&self)->u64 {self.parameters.size()}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
 pub fn invalidate(&mut self){self.group=None;}
 pub fn bytes(&self)->u64 {self.parameters.size()}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-01.rs"
```

</details>

### 2. `src/tile_gpu.rs`

Own the current global lookup binding state under the existing tile lifetime. Begin with the existing per-mesh fallback. Drop global inputs when the physical projection interval exceeds the device budget. Include inverse lookup identity in the existing source/model witness without invalidating projection for selection-only bytes. Allocate one global dispatch interval, or retain the existing guarded per-mesh intervals. Keep one active projection resource path. Release stale immutable output groups during resize and hidden-viewport release. Resolve all live physical IDs through the lookup in one dispatch, preserving the earlier projection algorithm and fallback. Count only the active interval owner; geometry, object and lookup allocations remain counted by their existing owners.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_mesh::GpuMesh,tile_math::{TileLayout,PROJECTED_BYTES,projected_records,next_pool_words,project_groups},tile_pipelines::{Pipelines,group},tile_report::PoolReport};
use wgpu::util::DeviceExt;
type Witness=(wgpu::Buffer,wgpu::Buffer,wgpu::Buffer,[u8;64],u32,u32,u32,Option<u32>);
struct Row{vertices:wgpu::Buffer,indices:wgpu::Buffer,model:wgpu::Buffer,count:wgpu::Buffer,triangles:u32,shared:bool,group:Option<wgpu::BindGroup>}
pub struct Tiles{pub projected:wgpu::Buffer,pub buffer:wgpu::Buffer,pipes:Pipelines,line:wgpu::Buffer,line_group:wgpu::BindGroup,raster:wgpu::BindGroup,scan:wgpu::BindGroup,rows:Vec<Row>,witness:Vec<Witness>,triangles:u32,viewport:[u32;2],layout:Option<TileLayout>,target:Option<wgpu::TextureView>,pool:u64,report:PoolReport,key:Option<[f32;16]>,pub projections:u64,pub bins:u64,reported:Option<u64>,pub fallback:bool}
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let pipes=Pipelines::new(device,view);let projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);let buffer=buffer(device,"finite tile placeholder",16);
  let line=device.create_buffer(&wgpu::BufferDescriptor{label:Some("finite view dimensions"),size:32,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
  let line_group=group(device,&pipes.line,&[&line]);let raster=group(device,&pipes.raster_layout,&[&projected,&buffer]);let scan=group(device,&pipes.scan_layout,&[&buffer]);
  Self{projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
 pub fn set(&mut self,device:&wgpu::Device,meshes:&[GpuMesh]){self.set_records(device,meshes,None)}
 pub fn set_records(&mut self,device:&wgpu::Device,meshes:&[GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let count=meshes.iter().filter(|row|row.geometry.index_count>0).try_fold(0u32,|end,row|row.primitive_base().checked_add(row.geometry.index_count/3).map(|next|end.max(next)));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let current=|mesh:&GpuMesh|records.and_then(|rows|rows.current_row(mesh)).map(|(_,row)|row);
  let model=|mesh:&GpuMesh|if current(mesh).is_some(){records.unwrap().record_buffer().clone()}else{mesh.model_buffer().clone()};
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|{let(vertices,indices)=row.geometry.buffers();(vertices,indices,model(row),row.packed()[..64].try_into().unwrap(),row.primitive_base(),row.geometry.index_start,row.geometry.index_count,current(row))}).collect::<Vec<_>>();
  if witness==self.witness{return}
  self.rows.clear();
  for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,row.primitive_base(),current(row).unwrap_or(0),0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.buffers().0,indices:row.geometry.buffers().1,model:model(row),count,triangles,shared:current(row).is_some(),group:None});
  }
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }
 fn rebind(&mut self,device:&wgpu::Device){self.raster=group(device,&self.pipes.raster_layout,&[&self.projected,&self.buffer]);self.scan=group(device,&self.pipes.scan_layout,&[&self.buffer]);for row in &mut self.rows{row.group=None;}}
 fn release(&mut self,device:&wgpu::Device){if self.layout.take().is_none()&&self.projected.size()==PROJECTED_BYTES&&self.buffer.size()==16{return}self.projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);self.buffer=buffer(device,"finite tile placeholder",16);self.pool=0;self.target=None;self.key=None;self.rebind(device);}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,size:[u32;2]){
  if self.triangles==0||size.contains(&0){self.release(device);return}
  let layout=TileLayout::new((size[0],size[1]));let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let report=self.report.poll().filter(|_|self.layout==Some(layout));let capacity=self.buffer.size()/4;
  let ceiling=layout.max_pool_words().min(limit.saturating_sub(layout.header_records()*16)/4);
  let pool=next_pool_words(self.pool,layout.initial_pool_words(self.triangles),capacity,report,ceiling);
  if pool<layout.initial_pool_words(self.triangles)||layout.header_records()*16>limit{self.fallback=true;self.release(device);return}
  let mut moved=false;
  if let Some(records)=projected_records(u64::from(self.triangles),self.projected.size()/PROJECTED_BYTES,limit/PROJECTED_BYTES){self.projected=buffer(device,"finite projected triangles",records*PROJECTED_BYTES);moved=true;}
  if self.layout!=Some(layout)||pool>self.pool{self.buffer=buffer(device,"finite triangle tile lists",layout.buffer_bytes(pool));self.pool=pool;moved=true;}
  if self.layout!=Some(layout){self.target=Some(device.create_texture(&wgpu::TextureDescriptor{label:Some("finite tile raster"),size:wgpu::Extent3d{width:layout.dimensions()[0],height:layout.dimensions()[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::R8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]}).create_view(&Default::default()));}
  if self.viewport!=size{let bytes=[0.,0.,0.,size[1]as f32,size[0]as f32,0.,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.line,0,&bytes);self.viewport=size;self.key=None;}
  self.layout=Some(layout);if moved{self.key=None;self.rebind(device);}
 }
 pub fn encode(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,view:&wgpu::BindGroup,matrix:[f32;16]){
  let Some(layout)=self.layout else{return};if self.key==Some(matrix){if self.reported!=Some(self.bins)&&self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}return}
  encoder.clear_buffer(&self.projected,0,None);
  for row in &mut self.rows{let bindings=row.group.get_or_insert_with(||group(device,if row.shared {&self.pipes.record_project_layout}else{&self.pipes.project_layout},&[&row.vertices,&row.indices,&row.model,&self.projected,&row.count]));let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite original projection"),timestamp_writes:None});pass.set_pipeline(if row.shared {&self.pipes.record_project}else{&self.pipes.project});pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&*bindings,&[]);let(x,y)=project_groups(row.triangles);pass.dispatch_workgroups(x,y,1);}
  self.projections+=1;encoder.clear_buffer(&self.buffer,0,Some(layout.header_records()*16));self.bin(encoder,&self.pipes.count);
  let blocks=layout.count().div_ceil(256);
  for(pipeline,count)in self.pipes.scans.iter().zip([blocks,blocks.div_ceil(256),blocks]){let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite prefix scan"),timestamp_writes:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.scan,&[]);pass.dispatch_workgroups(count,1,1);}
  self.bin(encoder,&self.pipes.fill);self.bins+=1;if self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}self.key=Some(matrix);
 }
 fn bin(&self,encoder:&mut wgpu::CommandEncoder,pipeline:&wgpu::RenderPipeline){let Some(target)=&self.target else{return};let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite tile lists"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.raster,&[]);pass.draw(0..4,0..self.triangles);}
 pub fn map_report(&mut self){self.report.map();}
 pub fn usage(&self)->[u64;7]{[self.projected.size(),self.buffer.size(),32+16+16*self.rows.len()as u64,self.triangles as u64,self.pool,self.projections,self.bins]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_mesh::GpuMesh,tile_math::{TileLayout,PROJECTED_BYTES,projected_records,next_pool_words,project_groups},tile_pipelines::{Pipelines,group},tile_report::PoolReport};
use wgpu::util::DeviceExt;
type Witness=(wgpu::Buffer,wgpu::Buffer,wgpu::Buffer,[u8;64],u32,u32,u32,Option<u32>);
struct Row{vertices:wgpu::Buffer,indices:wgpu::Buffer,model:wgpu::Buffer,count:wgpu::Buffer,triangles:u32,shared:bool,group:Option<wgpu::BindGroup>}
pub struct Tiles{pub(crate) lookup:Option<crate::projection_lookup::Bindings>,pub projected:wgpu::Buffer,pub buffer:wgpu::Buffer,pipes:Pipelines,line:wgpu::Buffer,line_group:wgpu::BindGroup,raster:wgpu::BindGroup,scan:wgpu::BindGroup,rows:Vec<Row>,witness:Vec<Witness>,triangles:u32,viewport:[u32;2],layout:Option<TileLayout>,target:Option<wgpu::TextureView>,pool:u64,report:PoolReport,key:Option<[f32;16]>,pub projections:u64,pub bins:u64,reported:Option<u64>,pub fallback:bool}
fn buffer(device:&wgpu::Device,label:&str,size:u64)->wgpu::Buffer{device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false})}
impl Tiles{
 pub fn new(device:&wgpu::Device,view:&wgpu::BindGroupLayout)->Self{
  let pipes=Pipelines::new(device,view);let projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);let buffer=buffer(device,"finite tile placeholder",16);
  let line=device.create_buffer(&wgpu::BufferDescriptor{label:Some("finite view dimensions"),size:32,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
  let line_group=group(device,&pipes.line,&[&line]);let raster=group(device,&pipes.raster_layout,&[&projected,&buffer]);let scan=group(device,&pipes.scan_layout,&[&buffer]);
  Self{lookup:None,projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
 pub fn set(&mut self,device:&wgpu::Device,meshes:&[GpuMesh]){self.set_records(device,meshes,None)}
 pub fn set_records(&mut self,device:&wgpu::Device,meshes:&[GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let count=meshes.iter().filter(|row|row.geometry.index_count>0).try_fold(0u32,|end,row|row.primitive_base().checked_add(row.geometry.index_count/3).map(|next|end.max(next)));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.lookup=None;self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let current=|mesh:&GpuMesh|records.and_then(|rows|rows.current_row(mesh)).map(|(_,row)|row);
  let model=|mesh:&GpuMesh|if current(mesh).is_some(){records.unwrap().record_buffer().clone()}else{mesh.model_buffer().clone()};
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|{let(vertices,indices)=row.geometry.buffers();(vertices,indices,model(row),row.packed()[..64].try_into().unwrap(),row.primitive_base(),row.geometry.index_start,row.geometry.index_count,current(row))}).collect::<Vec<_>>();
  let lookup=crate::projection_lookup::Bindings::source(meshes,records);let lookup_view=lookup.and_then(|records|records.lookup.as_ref()).map(|table|&table.view);
  if witness==self.witness&&self.lookup.as_ref().map(|state|&state.table)==lookup_view{return}
  self.rows.clear();self.lookup=crate::projection_lookup::Bindings::new(device,meshes,lookup,count.unwrap());
  if self.lookup.is_none(){for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,row.primitive_base(),current(row).unwrap_or(0),0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.buffers().0,indices:row.geometry.buffers().1,model:model(row),count,triangles,shared:current(row).is_some(),group:None});
  }
  }
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }
 fn rebind(&mut self,device:&wgpu::Device){self.raster=group(device,&self.pipes.raster_layout,&[&self.projected,&self.buffer]);self.scan=group(device,&self.pipes.scan_layout,&[&self.buffer]);for row in &mut self.rows{row.group=None;}if let Some(lookup)=&mut self.lookup{lookup.invalidate();}}
 fn release(&mut self,device:&wgpu::Device){if self.layout.take().is_none()&&self.projected.size()==PROJECTED_BYTES&&self.buffer.size()==16{return}self.projected=buffer(device,"finite projected placeholder",PROJECTED_BYTES);self.buffer=buffer(device,"finite tile placeholder",16);self.pool=0;self.target=None;self.key=None;self.rebind(device);}
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,size:[u32;2]){
  if self.triangles==0||size.contains(&0){self.release(device);return}
  let layout=TileLayout::new((size[0],size[1]));let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let report=self.report.poll().filter(|_|self.layout==Some(layout));let capacity=self.buffer.size()/4;
  let ceiling=layout.max_pool_words().min(limit.saturating_sub(layout.header_records()*16)/4);
  let pool=next_pool_words(self.pool,layout.initial_pool_words(self.triangles),capacity,report,ceiling);
  if pool<layout.initial_pool_words(self.triangles)||layout.header_records()*16>limit{self.fallback=true;self.release(device);return}
  let mut moved=false;
  if let Some(records)=projected_records(u64::from(self.triangles),self.projected.size()/PROJECTED_BYTES,limit/PROJECTED_BYTES){self.projected=buffer(device,"finite projected triangles",records*PROJECTED_BYTES);moved=true;}
  if self.layout!=Some(layout)||pool>self.pool{self.buffer=buffer(device,"finite triangle tile lists",layout.buffer_bytes(pool));self.pool=pool;moved=true;}
  if self.layout!=Some(layout){self.target=Some(device.create_texture(&wgpu::TextureDescriptor{label:Some("finite tile raster"),size:wgpu::Extent3d{width:layout.dimensions()[0],height:layout.dimensions()[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::R8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]}).create_view(&Default::default()));}
  if self.viewport!=size{let bytes=[0.,0.,0.,size[1]as f32,size[0]as f32,0.,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.line,0,&bytes);self.viewport=size;self.key=None;}
  self.layout=Some(layout);if moved{self.key=None;self.rebind(device);}
 }
 pub fn encode(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,view:&wgpu::BindGroup,matrix:[f32;16]){
  let Some(layout)=self.layout else{return};if self.key==Some(matrix){if self.reported!=Some(self.bins)&&self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}return}
  encoder.clear_buffer(&self.projected,0,None);
  if let Some(lookup)=&mut self.lookup {let bindings=lookup.group(device,&self.pipes.lookup_project_layout,&self.projected);let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("shared lookup triangle projection"),timestamp_writes:None});pass.set_pipeline(&self.pipes.lookup_project);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&bindings,&[]);let(x,y)=project_groups(self.triangles);pass.dispatch_workgroups(x,y,1);}else{for row in &mut self.rows{let bindings=row.group.get_or_insert_with(||group(device,if row.shared {&self.pipes.record_project_layout}else{&self.pipes.project_layout},&[&row.vertices,&row.indices,&row.model,&self.projected,&row.count]));let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite original projection"),timestamp_writes:None});pass.set_pipeline(if row.shared {&self.pipes.record_project}else{&self.pipes.project});pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,&self.line_group,&[]);pass.set_bind_group(2,&*bindings,&[]);let(x,y)=project_groups(row.triangles);pass.dispatch_workgroups(x,y,1);}}
  self.projections+=1;encoder.clear_buffer(&self.buffer,0,Some(layout.header_records()*16));self.bin(encoder,&self.pipes.count);
  let blocks=layout.count().div_ceil(256);
  for(pipeline,count)in self.pipes.scans.iter().zip([blocks,blocks.div_ceil(256),blocks]){let mut pass=encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label:Some("finite prefix scan"),timestamp_writes:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.scan,&[]);pass.dispatch_workgroups(count,1,1);}
  self.bin(encoder,&self.pipes.fill);self.bins+=1;if self.report.copy(encoder,&self.buffer){self.reported=Some(self.bins)}self.key=Some(matrix);
 }
 fn bin(&self,encoder:&mut wgpu::CommandEncoder,pipeline:&wgpu::RenderPipeline){let Some(target)=&self.target else{return};let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite tile lists"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:target,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Discard}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});pass.set_pipeline(pipeline);pass.set_bind_group(0,&self.line_group,&[]);pass.set_bind_group(1,&self.raster,&[]);pass.draw(0..4,0..self.triangles);}
 pub fn map_report(&mut self){self.report.map();}
 pub fn usage(&self)->[u64;7]{[self.projected.size(),self.buffer.size(),32+16+16*self.rows.len()as u64+self.lookup.as_ref().map_or(0,|state|state.bytes()),self.triangles as u64,self.pool,self.projections,self.bins]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-02.rs"
```

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-03.rs"
```

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-04.rs"
```

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-05.rs"
```

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-06.rs"
```

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-07.rs"
```

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-08.rs"
```

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-09.rs"
```

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-10.rs"
```

</details>

### 3. `src/renderer.rs`

Observe explicit address overrides after style changes while the unchanged witness retains cached projections for ordinary selection.

<details>
<summary>Locate the existing block</summary>

```rust
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
```

</details>

After that block, type:

```rust
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/global_outline_tests.rs`

Supply exact global/per-mesh projected bytes and pixels across distinct and copied definitions, selection cache reuse, unchanged geometry uploads and full release.

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
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzs-project-live-triangles-through-the-lookup-12.rs" skip=36256
```

</details>

</details>

<a id="checkpoint-96zzzzt-plan-reusable-arena-spans"></a>

<details>
<summary>Implementation 96zzzzt-plan-reusable-arena-spans: Plan reusable arena spans</summary>

Prepare bounded hole reuse without moving any surviving geometry.

Data flow: Free intervals plus tail → bounded plan → take after both ranges validate.

### 1. `src/arena_spans.rs`

Plan reusable spans before committing either geometry range, and reuse the existing coalescer for retirement.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzt-plan-reusable-arena-spans-01.rs"
```

### 2. `src/lib.rs`

Declare the complete preparatory span helper.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod arena_rows;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod arena_spans;
mod arena_rows;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzt-plan-reusable-arena-spans-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/arena_spans.rs`

Supply hole reuse, unchanged preflight state, overflow refusal, zero spans, coalescing and free-tail checks.

<details>
<summary>Locate the existing block</summary>

```text
use std::ops::Range;
pub struct Plan {pub range:Range<u32>,at:Option<usize>}
pub fn plan(free:&[Range<u32>],count:u32,end:u32)->Option<Plan> {
 if count>0 {if let Some(at)=free.iter().position(|span|span.end-span.start>=count){let first=free[at].start;return Some(Plan {range:first..first+count,at:Some(at)});}}
 Some(Plan {range:end..end.checked_add(count)?,at:None})
}
impl Plan {
 pub fn take(self,free:&mut Vec<Range<u32>>)->Range<u32> {
  if let Some(at)=self.at {free[at].start=self.range.end;if free[at].is_empty(){free.remove(at);}}
  self.range
 }
}
pub fn retire(free:&mut Vec<Range<u32>>,range:Range<u32>,mut end:u32)->u32 {
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
 *free=crate::slot_space::coalesce(std::mem::take(free),end);
 while free.last().is_some_and(|span|span.end==end){end=free.pop().unwrap().start;}end
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzt-plan-reusable-arena-spans-03.rs" skip=854
```

</details>

</details>

<a id="checkpoint-96zzzzu-reuse-retired-arena-record-ranges"></a>

<details>
<summary>Implementation 96zzzzu-reuse-retired-arena-record-ranges: Reuse retired arena record ranges</summary>

Connect bounded span plans to CPU arena records while preserving live offsets.

Data flow: Retired source association → free spans → checked vertex/index plans → current packed records.

### 1. `src/arena_rows.rs`

Keep free vertex and index spans under the existing arena record owner. Validate both ranges and rebased addresses before taking spans. Retire the old source mapping and trim only a free tail; surviving addresses remain fixed. Reset free-span metadata with the existing logical arena reset.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::mesh::Mesh;
use std::{collections::HashMap,ops::Range,rc::{Rc,Weak}};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct MeshRange {pub vertices:Range<u32>,pub indices:Range<u32>}
#[derive(Default)]
pub struct Rows {
    pub vertices:Vec<u8>,pub indices:Vec<u32>,
    sources:HashMap<usize,(Weak<Mesh>,MeshRange)>,
}
impl Rows {
    pub fn append(&mut self,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let key=Rc::as_ptr(source) as usize;
        if let Some((weak,range))=self.sources.get(&key) {
            if weak.upgrade().is_some_and(|saved|Rc::ptr_eq(&saved,source)){return Ok(range.clone());}
        }
        let start=u32::try_from(self.vertices.len()/20).map_err(|_|"Arena vertex range")?;
        let count=u32::try_from(source.vertices().len()).map_err(|_|"Arena vertex count")?;
        let end=start.checked_add(count).ok_or("Arena vertex range")?;
        let first=u32::try_from(self.indices.len()).map_err(|_|"Arena index range")?;
        let count=u32::try_from(source.indices().len()).map_err(|_|"Arena index count")?;
        let last=first.checked_add(count).ok_or("Arena index range")?;
        let range=MeshRange {vertices:start..end,indices:first..last};
        self.vertices.extend(crate::packed_vertex::bytes(source));
        self.indices.extend(source.indices().iter().map(|i|i+start));
        self.sources.insert(key,(Rc::downgrade(source),range.clone()));Ok(range)
    }
    pub fn index_bytes(&self)->Vec<u8>{self.index_bytes_from(0)}
    pub fn index_bytes_from(&self,start:usize)->Vec<u8>{self.indices[start..].iter().flat_map(|i|i.to_le_bytes()).collect()}
    pub fn source_count(&self)->usize{self.sources.len()}
    pub fn reset(&mut self){self.vertices.clear();self.indices.clear();self.sources.clear();}
    pub fn release(&mut self){*self=Self::default();}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::mesh::Mesh;
use std::{collections::HashMap,ops::Range,rc::{Rc,Weak}};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct MeshRange {pub vertices:Range<u32>,pub indices:Range<u32>}
#[derive(Default)]
pub struct Rows {
    pub vertices:Vec<u8>,pub indices:Vec<u32>,
    sources:HashMap<usize,(Weak<Mesh>,MeshRange)>,
    free_vertices:Vec<Range<u32>>,free_indices:Vec<Range<u32>>,
}
impl Rows {
    pub fn append(&mut self,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let key=Rc::as_ptr(source) as usize;
        if let Some((weak,range))=self.sources.get(&key) {
            if weak.upgrade().is_some_and(|saved|Rc::ptr_eq(&saved,source)){return Ok(range.clone());}
        }
        let vertex_count=u32::try_from(source.vertices().len()).map_err(|_|"Arena vertex count")?;
        let index_count=u32::try_from(source.indices().len()).map_err(|_|"Arena index count")?;
        let vertex_end=u32::try_from(self.vertices.len()/20).map_err(|_|"Arena vertex range")?;
        let index_end=u32::try_from(self.indices.len()).map_err(|_|"Arena index range")?;
        let vertices=crate::arena_spans::plan(&self.free_vertices,vertex_count,vertex_end).ok_or("Arena vertex range")?;
        let indices=crate::arena_spans::plan(&self.free_indices,index_count,index_end).ok_or("Arena index range")?;
        let byte_end=usize::try_from(u64::from(vertices.range.end)*20).map_err(|_|"Arena vertex bytes")?;
        let rebased=source.indices().iter().map(|i|i.checked_add(vertices.range.start).ok_or("Arena index address")).collect::<Result<Vec<_>,_>>()?;
        let bytes=crate::packed_vertex::bytes(source);
        let range=MeshRange {vertices:vertices.take(&mut self.free_vertices),indices:indices.take(&mut self.free_indices)};
        self.vertices.resize(self.vertices.len().max(byte_end),0);self.indices.resize(self.indices.len().max(range.indices.end as usize),0);
        self.vertices[range.vertices.start as usize*20..byte_end].copy_from_slice(&bytes);
        self.indices[range.indices.start as usize..range.indices.end as usize].copy_from_slice(&rebased);
        self.sources.insert(key,(Rc::downgrade(source),range.clone()));Ok(range)
    }
    pub(crate) fn retire(&mut self,key:usize)->bool {
        let Some((_,range))=self.sources.remove(&key)else{return false};
        let vertices=crate::arena_spans::retire(&mut self.free_vertices,range.vertices,(self.vertices.len()/20)as u32);
        let indices=crate::arena_spans::retire(&mut self.free_indices,range.indices,self.indices.len()as u32);
        self.vertices.truncate(vertices as usize*20);self.indices.truncate(indices as usize);true
    }
    pub fn index_bytes(&self)->Vec<u8>{self.index_bytes_from(0)}
    pub fn index_bytes_from(&self,start:usize)->Vec<u8>{self.indices[start..].iter().flat_map(|i|i.to_le_bytes()).collect()}
    pub fn source_count(&self)->usize{self.sources.len()}
    pub fn reset(&mut self){self.vertices.clear();self.indices.clear();self.sources.clear();self.free_vertices.clear();self.free_indices.clear();}
    pub fn release(&mut self){*self=Self::default();}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzu-reuse-retired-arena-record-ranges-01.rs"
```

```text
--8<-- "journey/code/96zzzzu-reuse-retired-arena-record-ranges-02.rs"
```

```text
--8<-- "journey/code/96zzzzu-reuse-retired-arena-record-ranges-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/arena_rows_tests.rs`

Supply exact packed bytes, rebased indices, surviving offsets, retained CPU history, safe source restoration and coalesced tail retirement.

<details>
<summary>Locate the existing block</summary>

```text
use std::rc::Rc;
fn source()->Rc<crate::mesh::Mesh>{Rc::new(crate::mesh::Mesh::new(vec![[0.,0.,0.,0.9,0.1,0.1],[1.,0.,0.,0.9,0.1,0.1],[0.,1.,0.,0.9,0.1,0.1]],vec![0,1,2]).unwrap().with_alphas(vec![1./255.;3]).unwrap())}
#[test]
fn ranges_share_identity_and_rebase_indices_without_changing_packed_positions(){
 let a=source();let b=source();let mut rows=crate::arena_rows::Rows::default();
 let first=rows.append(&a).unwrap();assert_eq!(first.vertices,0..3);assert_eq!(first.indices,0..3);
 assert_eq!(rows.append(&Rc::clone(&a)).unwrap(),first);assert_eq!(rows.source_count(),1);
 let second=rows.append(&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert_eq!(rows.source_count(),2);
 assert_eq!(rows.indices,[0,1,2,3,4,5]);assert_eq!(rows.vertices,[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(rows.vertices.len(),6*20);assert_eq!(rows.vertices[19],1);assert_eq!(rows.index_bytes(),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());
 assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);
}
#[test]
fn arena_records_do_not_pin_sources_and_reset_reuses_storage(){
 let mesh=source();let weak=Rc::downgrade(&mesh);let mut rows=crate::arena_rows::Rows::default();rows.append(&mesh).unwrap();
 let cap=[rows.vertices.capacity(),rows.indices.capacity()];drop(mesh);assert!(weak.upgrade().is_none());
 rows.reset();assert_eq!(rows.source_count(),0);assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],cap);
 let next=source();assert_eq!(rows.append(&next).unwrap().vertices,0..3);rows.release();assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],[0,0]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzu-reuse-retired-arena-record-ranges-04.rs" skip=1687
```

</details>

</details>

<a id="checkpoint-96zzzzv-upload-reused-arena-spans-in-place"></a>

<details>
<summary>Implementation 96zzzzv-upload-reused-arena-spans-in-place: Upload reused arena spans in place</summary>

Write reused geometry ranges without replacing unchanged GPU buffers.

Data flow: Current arena range → bounded overwrite or tail → retained/grown buffers → exact upload totals.

### 1. `src/text_gpu_support.rs`

Use the same device-limit calculation for append and bounded overwrite operations. Validate alignment, start and complete end before any write, then reuse existing bounded overwrite and growth operations.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        }),stride,len:0,label,usage}
    }

    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let limits=ctx.device.limits();
        let limit=if self.usage.contains(wgpu::BufferUsages::STORAGE){
            limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))
        }else{limits.max_buffer_size};
        let most=limit/self.stride;assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
```

</details>

Replace it with:

```rust
        }),stride,len:0,label,usage}
    }

    fn max_rows(&self,ctx:&GpuCtx)->u64 {
        let limits=ctx.device.limits();let bytes=if self.usage.contains(wgpu::BufferUsages::STORAGE){limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))}else{limits.max_buffer_size};bytes/self.stride
    }
    pub fn write_or_append(&mut self,ctx:&GpuCtx,at:u32,bytes:&[u8])->bool {
        assert_eq!(bytes.len()as u64%self.stride,0);assert!(at<=self.len);
        let rows=u32::try_from(bytes.len()as u64/self.stride).expect("Bounded GPU records");let end=at.checked_add(rows).expect("GPU row count");assert!(u64::from(end)<=self.max_rows(ctx));
        let inside=usize::try_from(u64::from((self.len-at).min(rows))*self.stride).unwrap();
        self.write_bytes_at(ctx,at,&bytes[..inside]);self.append_bytes(ctx,&bytes[inside..])
    }
    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let most=self.max_rows(ctx);assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
```

<details>
<summary>Complete updated block reference</summary>

```rust
pub struct GpuCtx {pub device:wgpu::Device,pub queue:wgpu::Queue}
impl GpuCtx {pub fn new(device:&wgpu::Device,queue:&wgpu::Queue)->Self {Self {device:device.clone(),queue:queue.clone()}}}
pub type Pipeline=wgpu::RenderPipeline;
#[derive(Clone,Copy)]
pub struct Target {pub format:wgpu::TextureFormat,pub samples:u32}
impl Target {pub const ID:Self=Self {format:wgpu::TextureFormat::Rg32Uint,samples:1};}
pub const VERTS:wgpu::BufferUsages=wgpu::BufferUsages::VERTEX.union(wgpu::BufferUsages::COPY_DST);
pub struct GrowBuf {
    pub buf:wgpu::Buffer,
    stride:u64,len:u32,label:&'static str,usage:wgpu::BufferUsages,
}
impl GrowBuf {
    pub fn new(ctx:&GpuCtx,label:&'static str,stride:u64,usage:wgpu::BufferUsages)->Self {
        assert!(stride>0&&stride%4==0);
        let usage=usage|wgpu::BufferUsages::COPY_SRC|wgpu::BufferUsages::COPY_DST;
        Self {buf:ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label:Some(label),size:stride,usage,mapped_at_creation:false,
        }),stride,len:0,label,usage}
    }

    fn max_rows(&self,ctx:&GpuCtx)->u64 {
        let limits=ctx.device.limits();let bytes=if self.usage.contains(wgpu::BufferUsages::STORAGE){limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size))}else{limits.max_buffer_size};bytes/self.stride
    }
    pub fn write_or_append(&mut self,ctx:&GpuCtx,at:u32,bytes:&[u8])->bool {
        assert_eq!(bytes.len()as u64%self.stride,0);assert!(at<=self.len);
        let rows=u32::try_from(bytes.len()as u64/self.stride).expect("Bounded GPU records");let end=at.checked_add(rows).expect("GPU row count");assert!(u64::from(end)<=self.max_rows(ctx));
        let inside=usize::try_from(u64::from((self.len-at).min(rows))*self.stride).unwrap();
        self.write_bytes_at(ctx,at,&bytes[..inside]);self.append_bytes(ctx,&bytes[inside..])
    }
    pub fn append_bytes(&mut self,ctx:&GpuCtx,bytes:&[u8])->bool {
        assert_eq!(bytes.len() as u64%self.stride,0);
        if bytes.is_empty(){return false;}
        let rows=u32::try_from(bytes.len() as u64/self.stride).expect("Bounded GPU records");
        let end=self.len.checked_add(rows).expect("GPU row count");
        let most=self.max_rows(ctx);assert!(u64::from(end)<=most);
        let grew=u64::from(end)*self.stride>self.buf.size();
        if grew {
            let cap=self.buf.size()/self.stride;
            let cap=(cap+cap/2).max(u64::from(end)).min(most);
            let next=ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label:Some(self.label),size:cap*self.stride,usage:self.usage,mapped_at_creation:false,
            });
            if self.len>0 {
                let mut encoder=ctx.device.create_command_encoder(&Default::default());
                encoder.copy_buffer_to_buffer(&self.buf,0,&next,0,u64::from(self.len)*self.stride);
                ctx.queue.submit([encoder.finish()]);
            }
            self.buf=next;
        }
        ctx.queue.write_buffer(&self.buf,u64::from(self.len)*self.stride,bytes);
        self.len=end;grew
    }

    pub fn truncate(&mut self,len:u32){assert!(len<=self.len);self.len=len;}

    pub fn write_bytes_at(&self,ctx:&GpuCtx,at:u32,bytes:&[u8]) {
        assert_eq!(bytes.len()as u64%self.stride,0);
        let rows=bytes.len()as u64/self.stride;
        assert!(u64::from(at)+rows<=u64::from(self.len));
        if !bytes.is_empty(){ctx.queue.write_buffer(&self.buf,u64::from(at)*self.stride,bytes);}
    }

    pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {
        assert_eq!(self.stride,N as u64*4);
        let bytes:Vec<u8>=vertices.iter().flatten().flat_map(|v|v.to_ne_bytes()).collect();
        self.append_bytes(ctx,&bytes);
    }
    pub fn reset(&mut self){self.len=0;}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx,self.label,self.stride,self.usage);}
    pub fn len(&self)->u32{self.len}
    pub fn is_empty(&self)->bool{self.len==0}
}
pub fn texture_entry(binding:u32,visibility:wgpu::ShaderStages,sample_type:wgpu::TextureSampleType,multisampled:bool)->wgpu::BindGroupLayoutEntry {wgpu::BindGroupLayoutEntry {binding,visibility,ty:wgpu::BindingType::Texture {sample_type,view_dimension:wgpu::TextureViewDimension::D2,multisampled},count:None}}
pub fn resource_group<'a,const N:usize>(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,label:&str,entries:[(u32,wgpu::BindingResource<'a>);N])->wgpu::BindGroup {ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {label:Some(label),layout,entries:&entries.into_iter().map(|(binding,resource)|wgpu::BindGroupEntry {binding,resource}).collect::<Vec<_>>()})}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-01.rs"
```

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-02.rs"
```

</details>

### 2. `src/mesh_arena.rs`

Measure real new geometry uploads and written source bytes under the arena owner. Start upload counters with the arena. Write exact reused ranges and count real writes; preserve current groups unless a buffer allocation changes.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::rc::Rc;

pub struct Arena {
    pub rows:Rows,pub vertices:GrowBuf,pub indices:GrowBuf,
    pub layout:wgpu::BindGroupLayout,pub group:wgpu::BindGroup,pub revision:u64,
}
impl Arena {
    pub fn new(ctx:&GpuCtx)->Self {
        let vertices=GrowBuf::new(ctx,"arena compact vertices",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
        let indices=GrowBuf::new(ctx,"arena indices",4,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::INDEX);
        let layout=crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group=Self::bind(ctx,&layout,&vertices,&indices);
        Self {rows:Rows::default(),vertices,indices,layout,group,revision:0}
    }
    fn bind(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,vertices:&GrowBuf,indices:&GrowBuf)->wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("shared arena reads"),layout,
            entries:&[(0,&vertices.buf),(2,&indices.buf)].map(|(binding,buffer)|wgpu::BindGroupEntry {binding,resource:buffer.as_entire_binding()}),
        })
    }
    pub fn upload(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let vertex_start=self.rows.vertices.len();let index_start=self.rows.indices.len();
        let range=self.rows.append(source)?;
        let vertices=self.vertices.append_bytes(ctx,&self.rows.vertices[vertex_start..]);
        let indices=self.indices.append_bytes(ctx,&self.rows.index_bytes_from(index_start));
        if vertices||indices {
            self.group=Self::bind(ctx,&self.layout,&self.vertices,&self.indices);
            self.revision+=1;
        }
        Ok(range)
    }
    pub fn reset(&mut self){self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::rc::Rc;

pub struct Arena {
    pub rows:Rows,pub vertices:GrowBuf,pub indices:GrowBuf,
    pub uploads:usize,pub uploaded_bytes:u64,
    pub layout:wgpu::BindGroupLayout,pub group:wgpu::BindGroup,pub revision:u64,
}
impl Arena {
    pub fn new(ctx:&GpuCtx)->Self {
        let vertices=GrowBuf::new(ctx,"arena compact vertices",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
        let indices=GrowBuf::new(ctx,"arena indices",4,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::INDEX);
        let layout=crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group=Self::bind(ctx,&layout,&vertices,&indices);
        Self {rows:Rows::default(),vertices,indices,layout,group,revision:0,uploads:0,uploaded_bytes:0}
    }
    fn bind(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,vertices:&GrowBuf,indices:&GrowBuf)->wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("shared arena reads"),layout,
            entries:&[(0,&vertices.buf),(2,&indices.buf)].map(|(binding,buffer)|wgpu::BindGroupEntry {binding,resource:buffer.as_entire_binding()}),
        })
    }
    pub fn upload(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let sources=self.rows.source_count();let range=self.rows.append(source)?;
        if sources==self.rows.source_count(){return Ok(range);}
        let bytes=&self.rows.vertices[range.vertices.start as usize*20..range.vertices.end as usize*20];
        let index_bytes=self.rows.indices[range.indices.start as usize..range.indices.end as usize].iter().flat_map(|i|i.to_le_bytes()).collect::<Vec<_>>();
        let vertices=self.vertices.write_or_append(ctx,range.vertices.start,bytes);
        let indices=self.indices.write_or_append(ctx,range.indices.start,&index_bytes);
        self.uploads+=1;self.uploaded_bytes+=(bytes.len()+index_bytes.len())as u64;
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
        if vertices||indices {self.group=Self::bind(ctx,&self.layout,&self.vertices,&self.indices);self.revision+=1;}
        Ok(range)
    }
    pub fn reset(&mut self){self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-03.rs"
```

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-04.rs"
```

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-05.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/mesh_arena_tests.rs`

Supply mapped reused bytes, exact upload totals, retained CPU source, real blue/green/yellow pixels, old/fresh immutable bindings and refused-write preservation.

<details>
<summary>Locate the existing block</summary>

```text
use crate::text_gpu_support::GpuCtx;
use std::rc::Rc;
fn mesh(center:f32,color:[f32;3])->Rc<crate::mesh::Mesh>{
 Rc::new(crate::mesh::Mesh::new(vec![[center-0.4,-0.6,0.,color[0],color[1],color[2]],[center+0.4,-0.6,0.,color[0],color[1],color[2]],[center,0.6,0.,color[0],color[1],color[2]]],vec![0,1,2]).unwrap())
}
#[test]
fn actual_gpu_arena_appends_exact_records_reuses_identity_and_releases(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let first=arena.upload(&ctx,&a).unwrap();let revision=arena.revision;
 assert_eq!(arena.upload(&ctx,&a).unwrap(),first);assert_eq!(arena.revision,revision);
 let second=arena.upload(&ctx,&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert!(arena.revision>revision);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),arena.rows.index_bytes());
 let allocated=arena.allocated_bytes();let revision=arena.revision;arena.reset();assert_eq!(arena.rows.source_count(),0);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
 assert_eq!(arena.upload(&ctx,&b).unwrap().vertices,0..3);assert_eq!(arena.allocated_bytes(),allocated);assert_eq!(arena.revision,revision);
 arena.release(&ctx);assert_eq!(arena.allocated_bytes(),24);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
}
fn render(ctx:&GpuCtx,arena:&crate::mesh_arena::Arena,group:&wgpu::BindGroup)->Vec<u8>{
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("arena binding fixture"),source:wgpu::ShaderSource::Wgsl(format!("{}\n{}",include_str!("packed_vertex.wgsl"),r#"
 @group(0) @binding(0) var<storage,read> vertices:array<PackedVertex>;
 @group(0) @binding(2) var<storage,read> indices:array<u32>;
 struct Out{@builtin(position) position:vec4<f32>,@location(0) color:vec4<f32>}
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
 @fragment fn fragment(input:Out)->@location(0) vec4<f32>{return input.color;}
 "#).into())});
 let layout=ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(&arena.layout)],immediate_size:0});
 let pipeline=ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:None,layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rgba8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None});
 let texture=ctx.device.create_texture(&wgpu::TextureDescriptor{label:Some("actual arena colors"),size:wgpu::Extent3d{width:64,height:64,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());
 let mut encoder=ctx.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipeline);pass.set_bind_group(0,group,&[]);pass.draw(0..arena.indices.len(),0..1);
 }
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:64*256,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});
 encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&output,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(256),rows_per_image:Some(64)}},texture.size());ctx.queue.submit([encoder.finish()]);crate::growing_buffer_tests::read(ctx,&output,64*256)
}
#[test]
fn actual_arena_draw_rebuilds_immutable_bindings_after_growth(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);arena.upload(&ctx,&a).unwrap();let old=arena.group.clone();arena.upload(&ctx,&b).unwrap();
 let fresh=render(&ctx,&arena,&arena.group);let left=(32*64+16)*4;let right=(32*64+48)*4;
 assert_eq!(&fresh[left..left+4],&[255,0,0,255]);assert_eq!(&fresh[right..right+4],&[0,255,0,255]);
 let stale=render(&ctx,&arena,&old);assert_eq!(&stale[left..left+4],&[255,0,0,255]);assert_eq!(&stale[right..right+4],&[255;4],"An old binding cannot read the appended right-hand range");
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzv-upload-reused-arena-spans-in-place-06.rs" skip=5276
```

</details>

</details>

<a id="checkpoint-96zzzzw-reclaim-ranges-after-the-last-gpu-lease"></a>

<details>
<summary>Implementation 96zzzzw-reclaim-ranges-after-the-last-gpu-lease: Reclaim ranges after the last GPU lease</summary>

Retire GPU ranges independently of precise CPU history ownership.

Data flow: Last shared GPU owner drops → weak range lease expires → retire source/span → later geometry reuses the hole.

### 1. `src/mesh_arena.rs`

Use weak range tokens independently of CPU source ownership. Keep weak range leases under the existing arena owner. Begin without live range leases. Acquire one shared range token and retire only expired GPU leases; synchronize logical GPU lengths after free-tail trimming. Reset lease metadata with the existing explicit arena reset.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::rc::Rc;

pub struct Arena {
    pub rows:Rows,pub vertices:GrowBuf,pub indices:GrowBuf,
    pub uploads:usize,pub uploaded_bytes:u64,
    pub layout:wgpu::BindGroupLayout,pub group:wgpu::BindGroup,pub revision:u64,
}
impl Arena {
    pub fn new(ctx:&GpuCtx)->Self {
        let vertices=GrowBuf::new(ctx,"arena compact vertices",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
        let indices=GrowBuf::new(ctx,"arena indices",4,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::INDEX);
        let layout=crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group=Self::bind(ctx,&layout,&vertices,&indices);
        Self {rows:Rows::default(),vertices,indices,layout,group,revision:0,uploads:0,uploaded_bytes:0}
    }
    fn bind(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,vertices:&GrowBuf,indices:&GrowBuf)->wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("shared arena reads"),layout,
            entries:&[(0,&vertices.buf),(2,&indices.buf)].map(|(binding,buffer)|wgpu::BindGroupEntry {binding,resource:buffer.as_entire_binding()}),
        })
    }
    pub fn upload(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let sources=self.rows.source_count();let range=self.rows.append(source)?;
        if sources==self.rows.source_count(){return Ok(range);}
        let bytes=&self.rows.vertices[range.vertices.start as usize*20..range.vertices.end as usize*20];
        let index_bytes=self.rows.indices[range.indices.start as usize..range.indices.end as usize].iter().flat_map(|i|i.to_le_bytes()).collect::<Vec<_>>();
        let vertices=self.vertices.write_or_append(ctx,range.vertices.start,bytes);
        let indices=self.indices.write_or_append(ctx,range.indices.start,&index_bytes);
        self.uploads+=1;self.uploaded_bytes+=(bytes.len()+index_bytes.len())as u64;
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
        if vertices||indices {self.group=Self::bind(ctx,&self.layout,&self.vertices,&self.indices);self.revision+=1;}
        Ok(range)
    }
    pub fn reset(&mut self){self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{arena_rows::{Rows,MeshRange},mesh::Mesh,text_gpu_support::{GpuCtx,GrowBuf}};
use std::{collections::HashMap,rc::{Rc,Weak}};

pub struct Arena {
    leases:HashMap<usize,Weak<MeshRange>>,
    pub rows:Rows,pub vertices:GrowBuf,pub indices:GrowBuf,
    pub uploads:usize,pub uploaded_bytes:u64,
    pub layout:wgpu::BindGroupLayout,pub group:wgpu::BindGroup,pub revision:u64,
}
impl Arena {
    pub fn new(ctx:&GpuCtx)->Self {
        let vertices=GrowBuf::new(ctx,"arena compact vertices",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
        let indices=GrowBuf::new(ctx,"arena indices",4,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::INDEX);
        let layout=crate::gpu_geometry::GpuGeometry::storage_layout(&ctx.device);
        let group=Self::bind(ctx,&layout,&vertices,&indices);
        Self {leases:HashMap::new(),rows:Rows::default(),vertices,indices,layout,group,revision:0,uploads:0,uploaded_bytes:0}
    }
    fn bind(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,vertices:&GrowBuf,indices:&GrowBuf)->wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:Some("shared arena reads"),layout,
            entries:&[(0,&vertices.buf),(2,&indices.buf)].map(|(binding,buffer)|wgpu::BindGroupEntry {binding,resource:buffer.as_entire_binding()}),
        })
    }
    pub fn upload(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<MeshRange,&'static str>{
        let sources=self.rows.source_count();let range=self.rows.append(source)?;
        if sources==self.rows.source_count(){return Ok(range);}
        let bytes=&self.rows.vertices[range.vertices.start as usize*20..range.vertices.end as usize*20];
        let index_bytes=self.rows.indices[range.indices.start as usize..range.indices.end as usize].iter().flat_map(|i|i.to_le_bytes()).collect::<Vec<_>>();
        let vertices=self.vertices.write_or_append(ctx,range.vertices.start,bytes);
        let indices=self.indices.write_or_append(ctx,range.indices.start,&index_bytes);
        self.uploads+=1;self.uploaded_bytes+=(bytes.len()+index_bytes.len())as u64;
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
        if vertices||indices {self.group=Self::bind(ctx,&self.layout,&self.vertices,&self.indices);self.revision+=1;}
        Ok(range)
    }
    pub fn acquire(&mut self,ctx:&GpuCtx,source:&Rc<Mesh>)->Result<Rc<MeshRange>,&'static str>{
        self.prune();let key=Rc::as_ptr(source)as usize;
        if let Some(range)=self.leases.get(&key).and_then(Weak::upgrade){return Ok(range);}
        let range=Rc::new(self.upload(ctx,source)?);self.leases.insert(key,Rc::downgrade(&range));Ok(range)
    }
    pub fn prune(&mut self){
        let retired=self.leases.iter().filter(|(_,range)|range.strong_count()==0).map(|(&key,_)|key).collect::<Vec<_>>();
        for key in retired {self.leases.remove(&key);self.rows.retire(key);}
        self.vertices.truncate((self.rows.vertices.len()/20)as u32);self.indices.truncate(self.rows.indices.len()as u32);
    }
    pub fn reset(&mut self){self.leases.clear();self.rows.reset();self.vertices.reset();self.indices.reset();}
    pub fn release(&mut self,ctx:&GpuCtx){*self=Self::new(ctx);}
    pub fn allocated_bytes(&self)->u64{self.vertices.buf.size()+self.indices.buf.size()}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-01.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-02.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-03.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-04.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-05.rs"
```

</details>

### 2. `src/gpu_geometry.rs`

Keep the range reserved for every shared GPU geometry owner, including direct construction outside the cache. Acquire a range lease before publishing its immutable draw offsets. Hold the range token for the geometry lifetime. Read the same arena buffers/groups through leased storage without changing standalone preview ownership.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().upload(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared(arena)})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared(arena)=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared(arena)=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared(arena)=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared(_)=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
```

</details>

Replace it with:

```rust
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().acquire(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared {arena,_lease:range}})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared {arena,..}=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared {arena,..}=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared {arena,..}=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared {..}=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
```

<details>
<summary>Locate this block</summary>

```rust

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared(Rc<RefCell<crate::mesh_arena::Arena>>),
}

impl GpuGeometry {
```

</details>

Replace it with:

```rust

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared {arena:Rc<RefCell<crate::mesh_arena::Arena>>,_lease:Rc<crate::arena_rows::MeshRange>},
}

impl GpuGeometry {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::mesh::Mesh;
use std::{cell::RefCell,rc::Rc};
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) index_count: u32,
    pub(crate) index_start: u32,
    storage: Storage,
}

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared {arena:Rc<RefCell<crate::mesh_arena::Arena>>,_lease:Rc<crate::arena_rows::MeshRange>},
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(if binding==0 {20}else{4}),
            }, count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"), entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(device: &wgpu::Device, label: &str, mut bytes: Vec<u8>, usage: wgpu::BufferUsages) -> wgpu::Buffer {
            if bytes.is_empty() { bytes.resize(4, 0); }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label), contents: &bytes, usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let mut bytes = crate::packed_vertex::bytes(&source);
        if bytes.is_empty() { bytes.resize(20, 0); }
        let vertices = buffer(device, "shared mesh vertices", bytes, wgpu::BufferUsages::VERTEX);
        let bytes = source.indices().iter().flat_map(|v| v.to_ne_bytes()).collect();
        let indices = buffer(device, "shared mesh indices", bytes, wgpu::BufferUsages::INDEX);
        let entries = [(0, &vertices), (2, &indices)].map(|(binding, buffer)| wgpu::BindGroupEntry {
            binding, resource: buffer.as_entire_binding(),
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"), layout: &Self::storage_layout(device), entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self {source,index_count,index_start:0,storage:Storage::Owned {vertices,indices,group:pull_group}}
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().acquire(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared {arena,_lease:range}})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared {arena,..}=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared {arena,..}=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared {arena,..}=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared {..}=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 { return; }
        pass.set_bind_group(3, &self.group(), &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {self.draw_instances(pass,1);}

    pub fn draw_instances(&self,pass:&mut wgpu::RenderPass<'_>,count:u32){self.draw_range(pass,0..count);}
    pub(crate) fn draw_range(&self,pass:&mut wgpu::RenderPass<'_>,instances:std::ops::Range<u32>){
        if self.index_count==0||instances.is_empty(){return;}
        let(vertices,indices)=self.buffers();
        pass.set_vertex_buffer(0,vertices.slice(..));pass.set_index_buffer(indices.slice(..),wgpu::IndexFormat::Uint32);
        pass.draw_indexed(self.index_start..self.index_start+self.index_count,0,instances);
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-06.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-07.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-08.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-09.rs"
```

</details>

### 3. `src/geometry_cache.rs`

Capture real arena upload counters before acquisition, including hole reuse. Count actual new geometry uploads and written bytes once instead of inferring them from arena growth. Retire expired leases after scene replacement and retain the existing empty-owner release.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{cell::RefCell,collections::HashMap, rc::{Rc, Weak}};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    arena:Option<Rc<RefCell<crate::mesh_arena::Arena>>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, ctx:&crate::text_gpu_support::GpuCtx, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self.entries.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let arena=self.arena.get_or_insert_with(||Rc::new(RefCell::new(crate::mesh_arena::Arena::new(ctx)))).clone();
        let (sources,bytes)={let arena=arena.borrow();(arena.rows.source_count(),arena.rows.vertices.len()+arena.rows.indices.len()*4)};
        let geometry=Rc::new(GpuGeometry::shared(ctx,Rc::clone(source),arena.clone()).expect("Prepared shared mesh ranges"));
        let arena=arena.borrow();
        self.uploads+=usize::from(arena.rows.source_count()!=sources);
        self.uploaded_bytes+=(arena.rows.vertices.len()+arena.rows.indices.len()*4-bytes)as u64;
        self.entries.insert(key, Rc::downgrade(&geometry));

        geometry
    }

    pub fn prune(&mut self) {
        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
        if self.entries.is_empty(){self.arena=None;}
    }

    pub fn entries(&self) -> usize { self.entries.len() }
    pub fn allocated_bytes(&self)->u64{self.arena.as_ref().map_or(0,|arena|arena.borrow().allocated_bytes())}
    pub fn cpu_bytes(&self)->usize{self.arena.as_ref().map_or(0,|arena|{let arena=arena.borrow();arena.rows.vertices.capacity()+arena.rows.indices.capacity()*4})}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{cell::RefCell,collections::HashMap, rc::{Rc, Weak}};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    arena:Option<Rc<RefCell<crate::mesh_arena::Arena>>>,
    pub uploads: usize,
    pub uploaded_bytes: u64,
}

impl GeometryCache {
    pub fn get(&mut self, ctx:&crate::text_gpu_support::GpuCtx, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self.entries.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let arena=self.arena.get_or_insert_with(||Rc::new(RefCell::new(crate::mesh_arena::Arena::new(ctx)))).clone();
        let (uploads,bytes)={let arena=arena.borrow();(arena.uploads,arena.uploaded_bytes)};
        let geometry=Rc::new(GpuGeometry::shared(ctx,Rc::clone(source),arena.clone()).expect("Prepared shared mesh ranges"));
        let arena=arena.borrow();
        self.uploads+=arena.uploads-uploads;
        self.uploaded_bytes+=arena.uploaded_bytes-bytes;
        self.entries.insert(key, Rc::downgrade(&geometry));

        geometry
    }

    pub fn prune(&mut self) {
        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
        if let Some(arena)=&self.arena {arena.borrow_mut().prune();}
        if self.entries.is_empty(){self.arena=None;}
    }

    pub fn entries(&self) -> usize { self.entries.len() }
    pub fn allocated_bytes(&self)->u64{self.arena.as_ref().map_or(0,|arena|arena.borrow().allocated_bytes())}
    pub fn cpu_bytes(&self)->usize{self.arena.as_ref().map_or(0,|arena|{let arena=arena.borrow();arena.rows.vertices.capacity()+arena.rows.indices.capacity()*4})}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-10.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-11.rs"
```

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/mesh_arena_tests.rs`

Supply direct shared-construction aliases, last-owner retirement despite retained CPU history, exact reused offsets/bytes, safe restoration and logical empty release.

<details>
<summary>Locate the existing block</summary>

```text
use crate::text_gpu_support::GpuCtx;
use std::rc::Rc;
fn mesh(center:f32,color:[f32;3])->Rc<crate::mesh::Mesh>{
 Rc::new(crate::mesh::Mesh::new(vec![[center-0.4,-0.6,0.,color[0],color[1],color[2]],[center+0.4,-0.6,0.,color[0],color[1],color[2]],[center,0.6,0.,color[0],color[1],color[2]]],vec![0,1,2]).unwrap())
}
#[test]
fn actual_gpu_arena_appends_exact_records_reuses_identity_and_releases(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let first=arena.upload(&ctx,&a).unwrap();let revision=arena.revision;
 assert_eq!(arena.upload(&ctx,&a).unwrap(),first);assert_eq!(arena.revision,revision);
 let second=arena.upload(&ctx,&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert!(arena.revision>revision);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),arena.rows.index_bytes());
 let allocated=arena.allocated_bytes();let revision=arena.revision;arena.reset();assert_eq!(arena.rows.source_count(),0);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
 assert_eq!(arena.upload(&ctx,&b).unwrap().vertices,0..3);assert_eq!(arena.allocated_bytes(),allocated);assert_eq!(arena.revision,revision);
 arena.release(&ctx);assert_eq!(arena.allocated_bytes(),24);assert!(arena.vertices.is_empty());assert!(arena.indices.is_empty());
}
fn render(ctx:&GpuCtx,arena:&crate::mesh_arena::Arena,group:&wgpu::BindGroup)->Vec<u8>{
 let shader=ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("arena binding fixture"),source:wgpu::ShaderSource::Wgsl(format!("{}\n{}",include_str!("packed_vertex.wgsl"),r#"
 @group(0) @binding(0) var<storage,read> vertices:array<PackedVertex>;
 @group(0) @binding(2) var<storage,read> indices:array<u32>;
 struct Out{@builtin(position) position:vec4<f32>,@location(0) color:vec4<f32>}
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
 @fragment fn fragment(input:Out)->@location(0) vec4<f32>{return input.color;}
 "#).into())});
 let layout=ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:None,bind_group_layouts:&[Some(&arena.layout)],immediate_size:0});
 let pipeline=ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:None,layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rgba8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None});
 let texture=ctx.device.create_texture(&wgpu::TextureDescriptor{label:Some("actual arena colors"),size:wgpu::Extent3d{width:64,height:64,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());
 let mut encoder=ctx.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipeline);pass.set_bind_group(0,group,&[]);pass.draw(0..arena.indices.len(),0..1);
 }
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:64*256,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});
 encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&output,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(256),rows_per_image:Some(64)}},texture.size());ctx.queue.submit([encoder.finish()]);crate::growing_buffer_tests::read(ctx,&output,64*256)
}
#[test]
fn actual_arena_draw_rebuilds_immutable_bindings_after_growth(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);
 let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);arena.upload(&ctx,&a).unwrap();let old=arena.group.clone();arena.upload(&ctx,&b).unwrap();
 let fresh=render(&ctx,&arena,&arena.group);let left=(32*64+16)*4;let right=(32*64+48)*4;
 assert_eq!(&fresh[left..left+4],&[255,0,0,255]);assert_eq!(&fresh[right..right+4],&[0,255,0,255]);
 let stale=render(&ctx,&arena,&old);assert_eq!(&stale[left..left+4],&[255,0,0,255]);assert_eq!(&stale[right..right+4],&[255;4],"An old binding cannot read the appended right-hand range");
}

#[test]
fn actual_gpu_arena_reuses_ranges_preserves_bindings_and_counts_exact_writes(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut arena=crate::mesh_arena::Arena::new(&ctx);let a=mesh(-0.45,[1.,0.,0.]);let b=mesh(0.45,[0.,1.,0.]);let history=Rc::clone(&a);let first=arena.upload(&ctx,&a).unwrap();let survivor=arena.upload(&ctx,&b).unwrap();let group=arena.group.clone();let buffers=[arena.vertices.buf.clone(),arena.indices.buf.clone()];let revision=arena.revision;assert_eq!((arena.uploads,arena.uploaded_bytes),(2,144));
 assert!(arena.rows.retire(Rc::as_ptr(&a)as usize));let c=mesh(-0.45,[0.,0.,1.]);assert_eq!(arena.upload(&ctx,&c).unwrap(),first);assert_eq!(arena.upload(&ctx,&b).unwrap(),survivor);assert_eq!(arena.group,group);assert_eq!(arena.revision,revision);assert_eq!([arena.vertices.buf.clone(),arena.indices.buf.clone()],buffers);assert_eq!((arena.uploads,arena.uploaded_bytes),(3,216));assert_eq!(history.indices(),&[0,1,2]);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,120),[crate::packed_vertex::bytes(&c),crate::packed_vertex::bytes(&b)].concat());assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.indices.buf,24),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());let image=render(&ctx,&arena,&group);let left=(32*64+16)*4;let right=(32*64+48)*4;let center=(32*64+32)*4;assert_eq!(&image[left..left+4],&[0,0,255,255]);assert_eq!(&image[right..right+4],&[0,255,0,255]);
 let d=mesh(0.,[1.,1.,0.]);arena.upload(&ctx,&d).unwrap();assert_ne!(arena.group,group);assert!(arena.revision>revision);assert_eq!((arena.uploads,arena.uploaded_bytes),(4,288));let fresh=render(&ctx,&arena,&arena.group);let stale=render(&ctx,&arena,&group);assert_eq!(&fresh[center..center+4],&[255,255,0,255]);assert_eq!(&stale[center..center+4],&[255;4]);assert_eq!(&stale[left..left+4],&[0,0,255,255]);
 let before=crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,180);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||arena.vertices.write_or_append(&ctx,u32::MAX,&[0;20]))).is_err());assert_eq!(crate::growing_buffer_tests::read(&ctx,&arena.vertices.buf,180),before);assert!(!arena.vertices.write_or_append(&ctx,9,&[]));
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-13.rs" skip=7560
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/global_outline_tests.rs`

Supply actual scene deletion/addition/Undo with retained precise CPU history, reused GPU ranges, stable survivor pixels/owners, exact uploads, original identity and empty release.

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
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzw-reclaim-ranges-after-the-last-gpu-lease-14.rs" skip=37938
```

</details>

</details>

<a id="checkpoint-96zzzzx-retire-empty-arena-ranges-after-tail-trimming"></a>

<details>
<summary>Implementation 96zzzzx-retire-empty-arena-ranges-after-tail-trimming: Retire empty arena ranges after tail trimming</summary>

Release empty source records independently of the current arena extent.

Data flow: Retire occupied tail → arena end shrinks → release empty source association → keep the new end.

### 1. `src/arena_spans.rs`

Empty ranges reserve no bytes; a previous free-tail trim cannot invalidate their release.

<details>
<summary>Locate the existing block</summary>

```rust
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
 if range.start==range.end{return end;}
 assert!(range.start<=range.end&&range.end<=end);free.push(range);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzx-retire-empty-arena-ranges-after-tail-trimming-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/arena_rows_tests.rs`

Supply both source retirement orders after tail trimming, exact empty endpoints, retained originals, subsequent zero-based reuse and unchanged rejection of reversed/out-of-bounds nonempty ranges.

<details>
<summary>Locate the existing block</summary>

```text
use std::rc::Rc;
fn source()->Rc<crate::mesh::Mesh>{Rc::new(crate::mesh::Mesh::new(vec![[0.,0.,0.,0.9,0.1,0.1],[1.,0.,0.,0.9,0.1,0.1],[0.,1.,0.,0.9,0.1,0.1]],vec![0,1,2]).unwrap().with_alphas(vec![1./255.;3]).unwrap())}
#[test]
fn ranges_share_identity_and_rebase_indices_without_changing_packed_positions(){
 let a=source();let b=source();let mut rows=crate::arena_rows::Rows::default();
 let first=rows.append(&a).unwrap();assert_eq!(first.vertices,0..3);assert_eq!(first.indices,0..3);
 assert_eq!(rows.append(&Rc::clone(&a)).unwrap(),first);assert_eq!(rows.source_count(),1);
 let second=rows.append(&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert_eq!(rows.source_count(),2);
 assert_eq!(rows.indices,[0,1,2,3,4,5]);assert_eq!(rows.vertices,[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(rows.vertices.len(),6*20);assert_eq!(rows.vertices[19],1);assert_eq!(rows.index_bytes(),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());
 assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);
}
#[test]
fn arena_records_do_not_pin_sources_and_reset_reuses_storage(){
 let mesh=source();let weak=Rc::downgrade(&mesh);let mut rows=crate::arena_rows::Rows::default();rows.append(&mesh).unwrap();
 let cap=[rows.vertices.capacity(),rows.indices.capacity()];drop(mesh);assert!(weak.upgrade().is_none());
 rows.reset();assert_eq!(rows.source_count(),0);assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],cap);
 let next=source();assert_eq!(rows.append(&next).unwrap().vertices,0..3);rows.release();assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],[0,0]);
}

#[test]
fn retired_ranges_reuse_holes_without_pinning_or_moving_surviving_sources() {
 let a=source();let b=source();let history=Rc::clone(&a);let weak=Rc::downgrade(&a);let mut rows=crate::arena_rows::Rows::default();let first=rows.append(&a).unwrap();let survivor=rows.append(&b).unwrap();let held=rows.vertices[60..120].to_vec();
 assert!(!rows.retire(usize::MAX));assert!(rows.retire(Rc::as_ptr(&a)as usize));assert!(weak.upgrade().is_some());assert_eq!(rows.source_count(),1);
 let c=Rc::new((*source()).clone().with_alphas(vec![0.5;3]).unwrap());let reused=rows.append(&c).unwrap();assert_eq!(reused,first);assert_eq!(rows.append(&b).unwrap(),survivor);assert_eq!(rows.vertices.len(),120);assert_eq!(&rows.vertices[..60],crate::packed_vertex::bytes(&c));assert_eq!(&rows.vertices[60..120],held);assert_eq!(rows.indices,[0,1,2,3,4,5]);
 let restored=rows.append(&history).unwrap();assert_eq!(restored.vertices,6..9);assert_eq!(restored.indices,6..9);assert_ne!(restored,reused);assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);assert_eq!(c.indices(),&[0,1,2]);
 assert!(rows.retire(Rc::as_ptr(&c)as usize));assert!(rows.retire(Rc::as_ptr(&a)as usize));assert_eq!(rows.vertices.len(),120);assert_eq!(rows.append(&b).unwrap(),survivor);assert!(rows.retire(Rc::as_ptr(&b)as usize));assert!(rows.vertices.is_empty());assert!(rows.indices.is_empty());assert_eq!(rows.source_count(),0);assert_eq!(rows.append(&history).unwrap().vertices,0..3);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzx-retire-empty-arena-ranges-after-tail-trimming-02.rs" skip=3150
```

</details>

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 96zzzzx-retire-empty-arena-ranges-after-tail-trimming
npm --prefix ../session_tests run course -- check 96zzzzx-retire-empty-arena-ranges-after-tail-trimming
npm --prefix ../session_tests run course -- save 96zzzzx-retire-empty-arena-ranges-after-tail-trimming
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

Empty geometry retires safely in either order while preserving nonempty range bounds and hole reuse.

- **Try this:** Run Example CAD, Clipping Plane XY 0,0,0, Clipping Plane Solid and Fit. Run Select All, Delete, Undo and Select None repeatedly. Confirm exact restoration and no runtime error. The supplied optional fixture separately checks both empty/nonempty retirement orders and nonempty bounds.
