# 11.4 · Original point identity and manifest source records

**Goal:** Validate world text, heights and plane axes. · **Steps:** 6 · **Source:** [engine/gpu/surface_outline.rs](../../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/cloud.rs](../../../../src/engine/gpu/cloud.rs) · [engine/gpu/lod.rs](../../../../src/engine/gpu/lod.rs) · [engine/gpu/splat.rs](../../../../src/engine/gpu/splat.rs) · [shaders/splat.wgsl](../../../../src/shaders/splat.wgsl) · [shaders/splat_resolve.wgsl](../../../../src/shaders/splat_resolve.wgsl) · [app/cloud_query.rs](../../../../src/app/cloud_query.rs) (`session_viewer/src/lib.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 11.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/cloud.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/cloud_pick.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/scene.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">6 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Resolve source point iden…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Two-Pass CommandEncoder</text>
  <rect x="594" y="48" width="212" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Pass 1: LoadOp::Clear</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="80" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">Clear Canvas + Pink Triangle</text>
  <line x1="700" y1="88" x2="700" y2="102" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="594" y="104" width="212" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="121" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Pass 2: LoadOp::Load</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="136" text-anchor="middle" font-family="sans-serif" font-size="9" fill="#111111">Overlay Command Dock UI</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Original point identity and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Clip Space (-1..+1) Output</text>
  <rect x="900" y="46" width="148" height="96" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="900" y1="94" x2="1048" y2="94" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <line x1="974" y1="46" x2="974" y2="142" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <polygon points="974,56 926,132 1022,132" fill="#ed5c87" stroke="#111111" stroke-width="2"/>
  <circle cx="974" cy="56" r="3.5" fill="#111111"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="982" y="59" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(0,+0.5)</text>
  <circle cx="926" cy="132" r="3.5" fill="#111111"/>
  <circle cx="1022" cy="132" r="3.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Validate manifest text an…</text>
</svg>
</div>

## <a id="checkpoint-65-originals"></a>1. Resolve source point identities and precise positions

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud.rs, src/cloud_pic…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">stored source row</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed f64 point</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local highlight</text>
</svg>
</div>

**What this code does:** Resolve source point identities and precise positions.

### 1. `src/cloud.rs`

Retain source point identity, one control owner and precise placement without rewriting the original cloud.

**Locate this existing block:**

```rust
#[derive(Clone)]
pub struct Row {
  pub id: crate::scene::ObjectId,
  pub guid: String,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/65-originals-01.rs"
```

### 2. `src/cloud_pick.rs`

Retain source point identity, one control owner and precise placement without rewriting the original cloud.

Create the file and type:

```rust
--8<-- "journey/code/65-originals-02.rs"
```

### 3. `src/scene.rs`

Retain source point identity, one control owner and precise placement without rewriting the original cloud.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn deform_part(&mut self,target:crate::pick_sources::Target,delta:&session_rust::Xform) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
```

Replace it with:

```rust
    pub fn deform_part(&mut self,target:crate::pick_sources::Target,delta:&session_rust::Xform) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
```

**Locate this block:**

```rust
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        else if let Some(row)=self.clouds.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.clouds.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
```

Replace it with:

```rust
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        else if let Some(row)=self.clouds.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.clouds.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
```

**Locate this block:**

```rust

    pub fn line_capacity_bytes(&self) -> usize { self.lines.capacity() * std::mem::size_of::<LineObject>() }

    pub fn clouds(&self)->&[crate::cloud::Row] {&self.clouds}
    pub fn insert_cloud(&mut self,prepared:crate::cloud::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid().to_string();while self.clouds.iter().any(|r|r.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let name=prepared.source.name.clone();let visible=prepared.source.is_visible;let locked=prepared.source.is_locked;let row=crate::cloud::Row {id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.clouds.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_cloud(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
```

Replace it with:

```rust

    pub fn line_capacity_bytes(&self) -> usize { self.lines.capacity() * std::mem::size_of::<LineObject>() }

    pub fn cloud_controls(&mut self,parent:Option<ObjectId>) {
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn clouds(&self)->&[crate::cloud::Row] {&self.clouds}
    pub fn insert_cloud(&mut self,prepared:crate::cloud::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid().to_string();while self.clouds.iter().any(|r|r.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let name=prepared.source.name.clone();let visible=prepared.source.is_visible;let locked=prepared.source.is_locked;let row=crate::cloud::Row {controls_visible:false,id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.clouds.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_cloud(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
```

### 4. `src/pick_sources.rs`

Retain source point identity, one control owner and precise placement without rewriting the original cloud.

**Locate this existing block:**

```rust

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/65-originals-06.rs"
```

### 5. `src/selection.rs`

Retain source point identity, one control owner and precise placement without rewriting the original cloud.

Replace the entire file with:

```rust
use crate::{pick_sources::{Part, Target}, scene::Scene};

impl Target {
    pub fn valid(self, scene: &Scene) -> bool {
        if scene.pick_id(self.owner.value()).is_none() { return false; }
        match self.part {
            Part::CloudPoint(id)=>crate::cloud_pick::position(scene,self.owner,id).is_some(),
            Part::Object => true,
            Part::Face(key) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.faces.contains(&key)),
            Part::BRepEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.cad.as_ref().is_some_and(|cad|cad.edges.iter().any(|border|border.edge==edge))),
            Part::SurfaceEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.surface.as_ref().is_some_and(|s|s.grid.boundaries.iter().any(|b|b.edge==edge))),
            Part::MeshEdge(a, b) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.edges.contains(&(a, b))),
            Part::PathEdge(index) => scene.paths().iter().any(|r| r.id == self.owner && matches!(&r.prepared.source,
                crate::chain::Source::Polyline(line) if line.get_points().windows(2).nth(index).is_some_and(|pair| pair[0].to_f32() != pair[1].to_f32()))),
            Part::Curve => scene.lines().iter().any(|r| r.id == self.owner) || scene.paths().iter().any(|r| r.id == self.owner && !matches!(r.prepared.source, crate::chain::Source::Polyline(_))),
            Part::Control(index) => scene.paths().iter().any(|r| r.id == self.owner && r.controls_visible && matches!(&r.prepared.source,
                crate::chain::Source::Curve(curve, _) if crate::controls::Control::new(std::rc::Rc::clone(curve), index).is_ok())),
        }
    }

    pub fn describe(self) -> String {
        match self.part {
            Part::CloudPoint(id)=>format!("Object {} · original point {id}",self.owner.value()),
            Part::Object => format!("Object {}", self.owner.value()),
            Part::Face(key) => format!("Object {} · source face {key}", self.owner.value()),
            Part::BRepEdge(edge)=>format!("Object {} · original BRep edge {edge}",self.owner.value()),
            Part::SurfaceEdge(edge)=>format!("Object {} · surface domain edge {edge}",self.owner.value()),
            Part::MeshEdge(a, b) => format!("Object {} · source edge {a}–{b}", self.owner.value()),
            Part::PathEdge(index) => format!("Object {} · source segment {index}", self.owner.value()),
            Part::Curve => format!("Object {} · original curve", self.owner.value()),
            Part::Control(index) => format!("Object {} · original control {index}", self.owner.value()),
        }
    }
}
```

### 6. `src/highlight.rs`

Retain source point identity, one control owner and precise placement without rewriting the original cloud.

**Locate this existing block:**

```rust
                markers.push(crate::marker::Marker { diameter: marker.display.diameter + 4., colour: yellow, ..marker.display });
            }
            Part::Object => {}
        } }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/65-originals-09.rs"
```

### 7. `src/part_preview.rs`

Retain source point identity, one control owner and precise placement without rewriting the original cloud.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
}

pub fn origin(scene:&Scene,target:Target) -> Option<[f64;3]> {
    let points=points(scene,target).ok()?;
    if points.is_empty() {return None;}
    Some(std::array::from_fn(|i|points.iter().map(|p|p[i]/points.len() as f64).sum()))
```

Replace it with:

```rust
}

pub fn origin(scene:&Scene,target:Target) -> Option<[f64;3]> {
    if let Part::CloudPoint(id)=target.part {let p=crate::cloud_pick::position(scene,target.owner,id)?;return Some([p[0],p[1],p[2]]);}
    let points=points(scene,target).ok()?;
    if points.is_empty() {return None;}
    Some(std::array::from_fn(|i|points.iter().map(|p|p[i]/points.len() as f64).sum()))
```

**Locate this block:**

```rust

pub fn points(scene:&Scene,target:Target) -> Result<Vec<Point>, &'static str> {
    if !target.valid(scene) {return Err("Selected part is no longer editable");}
    let model=scene.placement(target.owner).ok_or("Part owner is missing")?;
    let points=if let Some(row)=scene.objects().iter().find(|r|r.id==target.owner) {
        if row.surface.is_some()||row.cad.is_some() {return Err("CAD surface part editing connects after source parameter mapping");}
```

Replace it with:

```rust

pub fn points(scene:&Scene,target:Target) -> Result<Vec<Point>, &'static str> {
    if !target.valid(scene) {return Err("Selected part is no longer editable");}
    if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
    let model=scene.placement(target.owner).ok_or("Part owner is missing")?;
    let points=if let Some(row)=scene.objects().iter().find(|r|r.id==target.owner) {
        if row.surface.is_some()||row.cad.is_some() {return Err("CAD surface part editing connects after source parameter mapping");}
```

### 8. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust

pub mod cloud_gpu;
```

After that block, type:

```rust
--8<-- "journey/code/65-originals-12.rs" skip=20
```

## <a id="checkpoint-65a-addresses"></a>2. Recover IDs from integer cloud pixels

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/pick_sources.rs, src/cl…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">active original parent</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">integer stored address</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original ID lookup</text>
</svg>
</div>

**What this code does:** Recover original IDs from integer cloud pixels.

### 1. `src/pick_sources.rs`

Return original integer storage addresses and query the active parent’s complete resident points.

**Locate this existing block:**

```rust
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/65a-addresses-01.rs"
```

### 2. `src/cloud_gpu.rs`

Return original integer storage addresses and query the active parent’s complete resident points.

**Locate this existing block:**

```rust
        ortho_h: view.ortho_h,
        height_px: size[1],
        lod_px: view.lod_px,
        nodes: &row.prepared.nodes,
      };
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/65a-addresses-02.rs"
```

### 3. `src/cloud_resolve.wgsl`

Return original integer storage addresses and query the active parent’s complete resident points.

**Locate this existing block:**

```wgsl
    let q=vec2<i32>(p.xy)+vec2<i32>(window.origin);let d=textureLoad(sdepth,q,0);if (d>=1.0) {discard;}
    let ids=textureLoad(sids,q,0).xy;
    return Pick(select(vec2<u32>(0u),vec2<u32>(ids.x,0u),window.mode==0u),d);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/65a-addresses-03.rs"
```

## <a id="checkpoint-65b-controls"></a>3. Enter source controls with the current command or F10

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/commands…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">selected cloud</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one original parent</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">point selection</text>
</svg>
</div>

**What this code does:** Enter source controls with the current command or F10.

### 1. `src/editor.rs`

Route the current Controls command and canvas-owned F10 to one original cloud parent.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
```

Replace it with:

```rust
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => {
                let parent=if visible {self.selected.filter(|id|self.scene.clouds().iter().any(|r|r.id==*id))} else {None};
                if parent.is_some() && self.pick_mode==crate::pick_sources::Mode::Control && self.scene.clouds().iter().any(|r|Some(r.id)==parent && r.controls_visible) {return Ok(Change::View);}
                self.scene.cloud_controls(parent);
                self.scene.show_controls(visible);
                if parent.is_some() {self.pick_mode=crate::pick_sources::Mode::Control;self.objects.clear();self.selected_part=None;}
                else if !visible && self.pick_mode==crate::pick_sources::Mode::Control {self.pick_mode=crate::pick_sources::Mode::Object;self.selected_part=None;self.select_objects(self.selected,false);}
            },
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
```

**Locate this block:**

```rust
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
```

Replace it with:

```rust
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { if mode!=crate::pick_sources::Mode::Control {self.scene.cloud_controls(None);} self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
```

**Locate this block:**

```rust

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
```

Replace it with:

```rust

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if self.scene.clouds().iter().any(|r|r.controls_visible) {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
```

### 2. `src/commands.rs`

Route the current Controls command and canvas-owned F10 to one original cloud parent.

**Locate this existing block:**

```rust
    ("Example Smooth", || Action::AddNormals(crate::normal_example::Mode::Smooth)),
    ("Example Winding", || Action::AddNormals(crate::normal_example::Mode::Winding)),
    ("Controls On", || Action::Controls(true)),
    ("Controls Off", || Action::Controls(false)),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/65b-controls-04.rs"
```

### 3. `src/shortcuts.rs`

Route the current Controls command and canvas-owned F10 to one original cloud parent.

Replace the entire file with:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shortcut { Undo, Redo, Fit, Hide, Show, Delete, SelectAll, Controls }

impl Shortcut {
    pub fn command(self) -> &'static str {
        match self { Self::Undo => "Undo", Self::Redo => "Redo", Self::Fit => "Fit", Self::Hide=>"Hide",Self::Show=>"Show",Self::Delete=>"Delete",Self::SelectAll=>"Select All",Self::Controls=>"Controls" }
    }
}

pub fn key(text: &str, command: bool, shift: bool, alt: bool, composing: bool,
    focused: bool, text_empty: bool) -> Option<Shortcut> {
    if alt || composing { return None; }
    if command {
        if focused && !text_empty { return None; }
        match text.to_ascii_lowercase().as_str() {
            "a" => Some(Shortcut::SelectAll),
            "z" if shift => Some(Shortcut::Redo),
            "z" => Some(Shortcut::Undo),
            "y" => Some(Shortcut::Redo),
            _ => None,
        }
    } else if !focused {match text.to_ascii_lowercase().as_str() {"f"=>Some(Shortcut::Fit),"h"=>Some(Shortcut::Hide),"s"=>Some(Shortcut::Show),"delete"=>Some(Shortcut::Delete),"f10"=>Some(Shortcut::Controls),_=>None}}
    else {None}
}

pub fn context_key(context:&egui::Context,command_text:&str,text:&str,command:bool,shift:bool,alt:bool,composing:bool)->Option<Shortcut> {
    if egui::Popup::is_any_open(context) {return None;}
    let empty_command=context.memory(|memory|memory.focused())==Some(egui::Id::new("command-input"))&&command_text.is_empty();
    key(text,command,shift,alt,composing,context.egui_wants_keyboard_input(),empty_command)
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/shortcuts_tests.rs`

Copy the earlier shortcut checks plus focused F10 ownership coverage.

**Locate this existing block:**

```text
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/65b-controls-07.rs" skip=8
```

## <a id="checkpoint-65c-cloud-point"></a>4. Pick points and prove source query reuse

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud_pick_tests.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original sparse IDs</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">local marker</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cancellation and source r…</text>
</svg>
</div>

**What this code does:** Pick original points and prove source query reuse.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/cloud_pick_tests.rs`

Copy original point-ID, f64 placement, eligibility, parent and source/history invariants.

Copy this supplied file:

```text
--8<-- "journey/code/65c-cloud-point-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```text

pub mod cloud_pick;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/65c-cloud-point-02.rs" skip=21
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/pick_gpu_tests.rs`

Copy actual full-u32 integer picks, local highlights, locked coverage, resident source queries, cancellation and shared-upload checks.

**Locate this existing block:**

```text
    camera.distance=100./std::f64::consts::FRAC_PI_6.tan();assert_eq!(camera_pixels(&mut renderer,&camera),coarse);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/65c-cloud-point-03.rs" skip=297
```

## <a id="checkpoint-65za-read-original-manifest-records-and-placements"></a>5. Read manifest records and placements

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Cargo.toml, src/manifest.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">result or refusal</text>
</svg>
</div>

**What this code does:** Pin the parser dependencies and write geometry/text records with their defaults.

### 1. `Cargo.toml`

Edit the named browser features and pinned manifest dependencies; keep existing entries.

Replace the entire file with:

```toml
[package]
name = "viewer_journey"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[workspace]

[dependencies]
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
prost = "=0.14.4"
sha2 = "=0.10.8"
uuid = { version = "=1.26.1", features = ["v4", "js"] }
js-sys = "=0.3.105"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"
session_rust = { path = "../../../session_rust", default-features = false }
getrandom = { version = "=0.2.17", features = ["js"] }

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }

serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", features = ["float_roundtrip"] }

glyphon = "=0.11.0"
anyhow = "=1.0.104"
ttf-parser = {version="=0.25.1",default-features=false}

serde_yaml_ng = "=0.10.0"
toml = { version = "=0.8.23", optional = true }

[features]
json-sessions = []
toml-manifests = ["dep:toml"]

[dev-dependencies]
pollster = "=0.4.0"
log = "=0.4.34"
```

### 2. `src/manifest.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/65za-read-original-manifest-records-and-placements-5.rs"
```

### 3. `src/lib.rs`

Register the original manifest records and helpers.

**Locate this existing block:**

```rust
#[cfg(test)]
mod cloud_pick_tests;
```

After that block, type:

```rust
--8<-- "journey/code/65za-read-original-manifest-records-and-placements-6.rs" skip=35
```

## <a id="checkpoint-65zb-validate-original-manifest-text-and-source-keys"></a>6. Validate manifest text and source keys

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 6</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/manifest.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original family</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">pure calculation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">result or refusal</text>
</svg>
</div>

**What this code does:** Validate world text, heights and plane axes.

### 1. `src/manifest.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
fn text_right() -> [f64; 3] {
    [1.0, 0.0, 0.0]
}

/// World +Y.
fn text_up() -> [f64; 3] {
    [0.0, 1.0, 0.0]
}

/// A translation matrix.
fn translation(at: [f64; 3]) -> Xform {
    Xform::translation(at[0], at[1], at[2])
}

/// True when any value is not finite.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/65zb-validate-original-manifest-text-and-source-keys-1.rs"
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 11.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 11.4
npm --prefix ../session_tests run course -- save 11.4
```

Continue with [12.1 · Complete viewer files and published scene sources](../../consolidation/66-read-scene-manifests.md).
