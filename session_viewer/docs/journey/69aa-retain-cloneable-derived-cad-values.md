# 13.1.19 · Retain cloneable derived CAD values

**Chapter 13 · Shared definitions and drawing sheets · Section 13.1 · Shared CAD definitions** · [Contents](contents.md)

**Typing: 6–12 minutes.** [Estimate](typing-load.md).

Keep derived display values cloneable and validate original topology/control storage separately from tessellation.

## Type

Continue from [Assemble an original rigid shape key](69a-describe-a-rigid-frame-and-original-shape-key.md). [Save or recover your work](recovery.md).

### 1. `src/mesh.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Mesh {
    vertices: Vec<[f32; 6]>,
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-01.rs"
```

### 2. `src/mesh_parts.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
#[derive(Default)]
pub struct Parts {
    pub faces: Vec<usize>,
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-02.rs"
```

### 3. `src/cad_face.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::Point;
pub struct Sample {pub vertex:u32,pub surface:usize,pub uv:[f64;2],pub sign:f64}
pub struct Data {pub points:Vec<Point>,pub faces:Vec<Vec<usize>>,pub vertices:Vec<[f32;6]>,pub normals:Vec<[f32;3]>,pub indices:Vec<u32>,pub face_ids:Vec<u32>,pub keys:Vec<std::collections::HashMap<usize,u32>>,pub samples:Vec<Sample>}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-03.rs"
```

### 4. `src/cad_display.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::Xform;

pub struct Boundary {pub edge:usize,pub points:Vec<[f64;3]>}
pub struct Prepared {pub source:Rc<session_rust::BRep>,pub edges:Vec<Boundary>,pub samples:Vec<crate::cad_face::Sample>,pub face_count:usize,pub edge_ids:Vec<usize>}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-04.rs"
```

### 5. `src/cad_boundary.rs`

Separate original validation and cloneable derived CAD values.

<details>
<summary>Locate the existing block</summary>

```rust
use std::rc::Rc;
use session_rust::{BRep,Mesh};

pub const QUALITY:(f64,f64)=(5.,0.001);
pub struct Prepared {pub source:Rc<BRep>,pub faces:Vec<Mesh>,pub chains:Vec<Option<crate::cad_edges::EdgeChain>>}
impl Prepared {
    pub fn new(source:Rc<BRep>)->Result<Self,&'static str> {
        if !source.is_valid() {return Err("BRep topology contains an invalid reference");}
        if source.m_faces.len()>65_536||source.m_edges.len()>262_144 {return Err("BRep topology exceeds the display budget");}
        for surface in &source.m_surfaces {crate::surface::validate(surface)?;}
        for curve in source.m_curves_3d.iter().chain(&source.m_curves_2d) {crate::curve::validate(curve)?;}
        let _=source.guid();let faces=source.face_meshes_q(Some(QUALITY));
        if faces.len()!=source.m_faces.len()||faces.iter().any(|face|face.face.is_empty()) {return Err("BRep face could not be tessellated");}
        let chains=crate::cad_edges::edge_chains(&source,&faces);
        for (index,(edge,chain)) in source.m_edges.iter().zip(&chains).enumerate() {
            if !edge.degenerated&&!source.edge_faces(index).is_empty()&&chain.is_none() {return Err("A CAD edge has no coherent face boundary");}
        }
        Ok(Self {source,faces,chains})
    }
    pub fn points(&self,edge:usize)->Option<Vec<[f64;3]>> {
        let chain=self.chains.get(edge)?.as_ref()?;let face=self.faces.get(chain.face)?;
        chain.keys.iter().map(|key|face.vertex.get(key).map(|v|[v.x,v.y,v.z])).collect()
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::rc::Rc;
use session_rust::{BRep,Mesh};

pub const QUALITY:(f64,f64)=(5.,0.001);
pub struct Prepared {pub source:Rc<BRep>,pub faces:Vec<Mesh>,pub chains:Vec<Option<crate::cad_edges::EdgeChain>>}
impl Prepared {
    pub fn new(source:Rc<BRep>)->Result<Self,&'static str> {
        validate(&source)?;
        let _=source.guid();let faces=source.face_meshes_q(Some(QUALITY));
        if faces.len()!=source.m_faces.len()||faces.iter().any(|face|face.face.is_empty()) {return Err("BRep face could not be tessellated");}
        let chains=crate::cad_edges::edge_chains(&source,&faces);
        for (index,(edge,chain)) in source.m_edges.iter().zip(&chains).enumerate() {
            if !edge.degenerated&&!source.edge_faces(index).is_empty()&&chain.is_none() {return Err("A CAD edge has no coherent face boundary");}
        }
        Ok(Self {source,faces,chains})
    }
    pub fn points(&self,edge:usize)->Option<Vec<[f64;3]>> {
        let chain=self.chains.get(edge)?.as_ref()?;let face=self.faces.get(chain.face)?;
        chain.keys.iter().map(|key|face.vertex.get(key).map(|v|[v.x,v.y,v.z])).collect()
    }
}

pub fn validate(source:&BRep)->Result<(),&'static str> {
        if !source.is_valid() {return Err("BRep topology contains an invalid reference");}
        if source.m_faces.len()>65_536||source.m_edges.len()>262_144 {return Err("BRep topology exceeds the display budget");}
        for surface in &source.m_surfaces {crate::surface::validate(surface)?;}
        for curve in source.m_curves_3d.iter().chain(&source.m_curves_2d) {crate::curve::validate(curve)?;}
    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-05.rs"
```

```text
--8<-- "journey/code/69aa-retain-cloneable-derived-cad-values-06.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 13.1.19
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

Independent yellow and blue shared-definition placements remain visible beside original CAD borders.

![Native renderer output for retain cloneable derived cad values.](../screenshots/journey/69n-inspect-and-exercise-original-definition-instances.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.1.14 · Inspect and exercise original definition instances](69n-inspect-and-exercise-original-definition-instances.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original source validation → derived values → scoped recording.

![Retain cloneable derived CAD values.](../illustrations/69aa-retain-cloneable-derived-cad-values.svg)

Are copied display values another editable BRep?

No. Original BRep records remain the editable source. Sampled vertices, normals, boundary chords and source mappings are derived display values.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect original source sharing beside copied display arrays and their exact source face and edge mappings.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 13.1.19
npm --prefix ../session_tests run course -- save 13.1.19
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited instance acceptance remains authored and unrun. The final congruent-CAD exercise inspects one walk/one replay, original identities, Save/Open, Delete/Undo/Redo and Close; actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 13.1.19
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
