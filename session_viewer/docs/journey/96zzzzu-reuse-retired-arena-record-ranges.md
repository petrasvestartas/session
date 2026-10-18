# 20.12.2 · Reuse retired arena record ranges

**Chapter 20 · Shared GPU rendering · Section 20.12 · Arena reclamation** · [Contents](contents.md)

**Typing: 21–42 minutes.** [Estimate](typing-load.md).

Connect bounded span plans to CPU arena records while preserving live offsets.

## Type

Continue from [Plan reusable arena spans](96zzzzt-plan-reusable-arena-spans.md). [Save or recover your work](recovery.md).

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

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.12.2
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

CPU arena records can reuse retired spans; application GPU allocation remains unchanged until retirement is connected.

**Verified checkpoint in Chrome.**

**Feature-specific visual review pending.** Scripted acceptance and the final main-page capture are separate evidence. A checker may restore its general fixture after closing the feature-test page; this image does not establish the feature’s visual correctness.

![General browser fixture after Reuse retired arena record ranges; feature-scene visual acceptance pending.](../screenshots/journey/96zzzzn-adopt-retained-gpu-slots-and-physical-bases-browser.png)

[Verification scope](release.md).

This reference image is shared with [20.11.6 · Adopt retained GPU slots and physical bases](96zzzzn-adopt-retained-gpu-slots-and-physical-bases.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Keep free vertex and index spans beside the existing source map. Plan both requests and validate rebased indices before taking either. A retired source loses its range association even when history still retains its precise mesh. Reuse holes and trim only unused tails. GPU retirement is connected after in-place uploads are prepared.

Retired source association → free spans → checked vertex/index plans → current packed records.



Why remove the source association while its mesh is still alive?

History can retain the mesh after its GPU range is retired. Returning to it must allocate a current range rather than recover bytes reused by another source.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Keep an original in Undo history, remove it, then add another original. At this preparatory checkpoint drawing stays unchanged; trace how a reused range must get a new source association.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.12.2
npm --prefix ../session_tests run course -- save 20.12.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

CPU range reuse is prepared. Bounded in-place GPU uploads and actual geometry lease retirement remain required before reclamation is complete.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.12.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
