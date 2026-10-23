# 9.5 · Oriented CAD solids, original picks and empty bores

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Checkpoints:** `59-orientation` → `59f-cad` (7 steps) · **Production source:** [app/walk/brep.rs](../../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs) · [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs)

> **Goal:** Draw the original bore and prove its empty hole.

## 1. Visual WebGPU Schematic

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Trimmed B-Rep (u,v)</text>
  <rect x="55" y="60" width="160" height="96" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <circle cx="135" cy="108" r="24" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text x="135" y="112" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Hole</text>
  <text x="135" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Outer + Inner Loops</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. Crease Split</text>
  <polyline points="338,144 419,98 500,144" fill="none" stroke="#111111" stroke-width="2.5"/>
  <circle cx="419" cy="98" r="4.5" fill="#111111"/>
  <line x1="419" y1="98" x2="386" y2="56" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <line x1="419" y1="98" x2="452" y2="56" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <text x="372" y="60" font-family="monospace" font-size="10" font-weight="700" fill="#111111">N1</text>
  <text x="458" y="60" font-family="monospace" font-size="10" font-weight="700" fill="#111111">N2</text>
  <text x="419" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Sharp Edge Normals</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Sub-ID Pick</text>
  <polygon points="650,74 758,74 780,138 628,138" fill="url(#hatch)" stroke="#111111" stroke-width="2.5"/>
  <text x="704" y="112" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">[brep, face_2]</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Face &amp; Edge IDs</text>
</svg>

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **Rust (`session_rust`)** | `BRep` Outer & Inner Trim Loops | Validates closed `(u,v)` trim loops and holes on curved CAD faces. |
| **Rust / WGSL** | Smooth `G1` vs. Crease Normals | Averages smooth normals and splits vertices across sharp creases. |
| **WebGPU (`Rg32Uint`)** | `[brep_id, face_or_edge_id]` | Unified whole-solid or sub-face/edge selection. |

![Recorded visual output for Lesson 9.5 (59f-cad)](../../../screenshots/journey/59f-cad.png)

---

## 2. Complete Code Overview (All Steps Open)

<a id="checkpoint-59-orientation"></a>

### Step 1 of 7 · Orient connected original face groups (`59-orientation`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/cad_orient.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">coherent component</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">outward winding</text>
</svg>

Reuse the earlier shared-edge comparisons. Propagate consistent signs through each connected face group, then use its signed volume to choose outward normals. Open shells keep their authored orientation. Measurements later reuse this same owner.

> **Check:** *Should an open shell be forced to use a closed-solid volume sign?* — No. Open shells retain authored orientation. A closed solid first makes adjacent face winding consistent and then chooses its outward orientation from component volume.

### 1. `src/cad_orient.rs`

Complete the single shared face-orientation algorithm.

After that block, type:

```rust
fn six_volume(fm: &Mesh) -> f64 {
    let mut keys: Vec<usize> = fm.face.keys().copied().collect();
    keys.sort_unstable();
    let mut v = 0.0;

    for k in keys {
        let verts = &fm.face[&k];

        if verts.len() < 3 {
            continue;
        }

        let (a, b, c) = (at(fm, verts[0]), at(fm, verts[1]), at(fm, verts[2]));
        v += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0]);
    }

    v
}

pub fn face_signs(b: &BRep, fms: &[Mesh], chains: &[Option<EdgeChain>]) -> Vec<f64> {
    let nf = fms.len();

    if !b.is_solid() {
        return vec![1.0; nf];
    }

    let mut adjacent: Vec<Vec<(usize, bool)>> = vec![Vec::new(); nf];

    for c in chains.iter().flatten() {
        if let (Some(other), Some(opp)) = (c.other, opposed(fms, c)) {
            adjacent[c.face].push((other, opp));
            adjacent[other].push((c.face, opp));
        }
    }

    let mut sign = vec![0.0f64; nf];

    for start in 0..nf {
        if sign[start] != 0.0 {
            continue;
        }

        sign[start] = 1.0;
        let mut group = vec![start];
        let mut head = 0;

        while head < group.len() {
            let f = group[head];
            head += 1;

            for &(g, opp) in &adjacent[f] {
                if sign[g] == 0.0 {
                    sign[g] = if opp { sign[f] } else { -sign[f] };
                    group.push(g);
                }
            }
        }

        let mut volume = 0.0;

        for &face in &group {
            volume += sign[face] * six_volume(&fms[face]);
        }

        if volume < 0.0 {
            for &f in &group {
                sign[f] = -sign[f];
            }
        }
    }

    sign
}
```

**Result after `59-orientation`:** The blue parametric roof remains visible while this preparation connects one CAD component. Draw and inspect the original bore at the final checkpoint of 59.

---

<a id="checkpoint-59a-face-data"></a>

### Step 2 of 7 · Map each display triangle to its original CAD face (`59a-face-data`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/cad_face.rs, src/lib.…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">derived display triangl…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original face index</text>
</svg>

Combine face display data while retaining the original f64 mesh positions, normals, face IDs and source surface UV addresses. Display triangle addresses remain derived from one original face.

> **Check:** *Does remeshing change the selected original face key?* — No. It changes derived triangles and compact GPU addresses. Every new triangle still resolves to its original CAD face; source surface and UV addresses remain available for later edits.

### 1. `src/cad_face.rs`

Prepare coherent face display arrays, original f64 positions, UV addresses and face indices.

Create the file and type:

```rust
--8<-- "journey/code/59a-face-data-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

After that block, type:

```rust
--8<-- "journey/code/59a-face-data-02.rs" skip=21
```

---

<a id="checkpoint-59b-display"></a>

### Step 3 of 7 · Retain source UVs and draw only original CAD boundaries (`59b-display`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/cad_display.rs, src/l…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original boundaries</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">released mesher maps</text>
</svg>

Build the derived display and compact source map, then release transient face-mesher maps. Keep original CAD ownership, UV addresses and original boundary chains without interior tessellation ink.

> **Check:** *Does the renderer need to retain every mesher HashMap for a whole-object move?* — No. The move changes placement. Original CAD ownership, compact UV addresses and boundary points remain shared; temporary face-mesher maps can be released after preparing the display.

### 1. `src/cad_display.rs`

Keep original CAD source and compact UV/border caches while discarding transient mesher faces.

Create the file and type:

```rust
--8<-- "journey/code/59b-display-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

After that block, type:

```rust
--8<-- "journey/code/59b-display-02.rs" skip=19
```

---

<a id="checkpoint-59c-source-row"></a>

### Step 4 of 7 · Attach original BRep ownership to existing rows (`59c-source-row`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/scene.rs, src/documen…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing layer and iden…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">shared placement history</text>
</svg>

Route an original BRep through the existing scene identity and current layer. Whole-object placements share original CAD data; proxy saving and unsupported direct CAD deformation are explicitly guarded.

> **Check:** *May a display proxy replace the BRep when saving?* — No. The original BRep owns its topology and geometry. Mixed saving connects in 66; the temporary display proxy must never silently be serialized as an ordinary mesh.

### 1. `src/scene.rs`

Attach original CAD ownership and preserve explicit saving and part-edit guards.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
            if row.surface.is_some() {return Err("CAD surface part editing connects after source parameter mapping");}
            let mut source=(**row.geometry().ok_or("Reload editable sources before editing a part")?).clone();
            for key in crate::part_preview::keys(&source,target.part)? {
                let point=local.transform_point(&source.vertex_point(key).ok_or("Missing source vertex")?);
```

Replace it with:

```rust
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
            if row.surface.is_some()||row.cad.is_some() {return Err("CAD surface part editing connects after source parameter mapping");}
            let mut source=(**row.geometry().ok_or("Reload editable sources before editing a part")?).clone();
            for key in crate::part_preview::keys(&source,target.part)? {
                let point=local.transform_point(&source.vertex_point(key).ok_or("Missing source vertex")?);
```

**Locate this block:**

```rust
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
        let prepared=Rc::new(crate::surface::Prepared::new(source)?);let mesh=prepared.mesh()?;let id=self.insert(mesh)?;
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
```

Replace it with:

```rust
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {cad:None,surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }

    pub fn insert_cad(&mut self,source:Rc<session_rust::BRep>)->Result<ObjectId,&'static str> {
        let (prepared,mesh)=crate::cad_display::prepare(source)?;let prepared=Rc::new(prepared);let id=self.insert(mesh)?;self.objects.iter_mut().find(|row|row.id==id).unwrap().cad=Some(prepared);Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
        let prepared=Rc::new(crate::surface::Prepared::new(source)?);let mesh=prepared.mesh()?;let id=self.insert(mesh)?;
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
```

**Locate this block:**

```rust

#[derive(Clone)]
pub struct Object {
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
```

Replace it with:

```rust

#[derive(Clone)]
pub struct Object {
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
```

**Complete updated block reference:**

```rust
use crate::{mesh::Mesh, prepared::PreparedMesh};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);

impl ObjectId { pub fn value(self) -> u32 { self.0 } }

#[derive(Clone)]
pub struct Object {
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
    pub metadata: Rc<crate::row_metadata::RowMetadata>,
    pub mesh: Rc<Mesh>,
    pub model: session_rust::Xform,
    editable: crate::edit_source::EditSource,
}

impl Object {
    pub fn visible(&self) -> bool { self.metadata.is_visible }

    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        match &self.editable { crate::edit_source::EditSource::Loaded { geometry, .. } => Some(geometry), _ => None }
    }

    pub fn source(&self) -> Option<&crate::document::Source> {
        match &self.editable { crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref(), _ => None }
    }

    pub fn origin(&self) -> Option<&Rc<crate::origin::Origin>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref().map(|source| &source.origin),
            crate::edit_source::EditSource::Released { origin, .. } => Some(origin),
        }
    }

    pub fn release_epoch(&self) -> Option<u64> {
        match &self.editable { crate::edit_source::EditSource::Released { epoch, .. } => Some(*epoch), _ => None }
    }

    pub fn can_unload(&self) -> bool {
        let Some(source) = self.source() else { return false; };
        let Some(geometry) = self.geometry() else { return false; };
        source.origin.location.is_some() && self.metadata.source_guid == source.guid
            && geometry.guid() == source.guid && self.metadata.name == geometry.name
            && self.metadata.is_visible == geometry.is_visible && self.metadata.is_locked == geometry.is_locked
            && source.document.objects.meshes.iter().any(|mesh| Rc::ptr_eq(mesh, geometry))
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
        let world = self.model.transform_point(&local);
        [world[0], world[1], world[2]]
    }
}

#[derive(Clone)]
pub struct LineObject {
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
}

impl LineObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [self.model.transform_point(&line.start()), self.model.transform_point(&line.end())]
    }

    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke { start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32), ..self.prepared.display }
    }
}

#[derive(Clone)]
pub struct PathObject { pub controls_visible: bool, pub id: ObjectId, pub guid: String, pub prepared: crate::chain::PreparedChain, pub model: session_rust::Xform }

impl PathObject {
    pub fn visible(&self) -> bool { self.prepared.source.visible() }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = std::array::from_fn(|i| p[i] as f32);
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
    }

    pub fn bounds_points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.bounds_coordinates().into_iter().map(|p|
            self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.coordinates().into_iter().map(|p| self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

}

pub struct ControlMarker { pub parent: ObjectId, pub control: crate::controls::Control, pub display: crate::marker::Marker }

#[derive(Clone)]
pub struct PointObject { pub id: ObjectId, pub guid: String, pub prepared: crate::marker::PreparedPoint, pub model: session_rust::Xform }

impl PointObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self) -> session_rust::Point { self.model.transform_point(&self.prepared.source) }
    pub fn marker(&self) -> crate::marker::Marker { crate::marker::Marker { center: self.point().to_f32(), ..self.prepared.display } }
}

#[derive(Clone)]
pub struct Scene {
    pub(crate) created:Option<u32>,
    ids:Rc<std::collections::HashSet<ObjectId>>,
    pub view:Rc<crate::view_flags::Flags>,
    pub forest:crate::hierarchy::Forest,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn pick_id(&self,value:u32)->Option<ObjectId> {
        let id=ObjectId(value);(self.visible(id) && !self.view.locked.contains(&id) && !self.forest.readonly(id)).then_some(id)
    }

    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle([
            [-0.7, -0.6, 0.25], [0.5, -0.6, 0.25], [-0.1, 0.6, 0.25],
        ], [0.9, 0.25, 0.45]).expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle([
            [-0.4, -0.2, 0.75], [0.8, -0.2, 0.75], [0.2, 0.8, 0.75],
        ], [0.05, 0.7, 0.7]).expect("Valid turquoise triangle");
        let mut scene = Self { created:None,ids:Default::default(), view:Default::default(), forest:Default::default(), objects: Vec::new(), points: Vec::new(), paths: Vec::new(), lines: Vec::new(), next_id: 1, extra: None };
        scene.insert(pink).expect("ID available");
        scene.insert(turquoise).expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn visible(&self,id:ObjectId)->bool {self.contains(id)&&!self.view.is_hidden(&id)}

    pub(crate) fn restore_view(&mut self,mut saved:Self) {saved.view=Rc::clone(&self.view);self.restore(saved);Rc::make_mut(&mut self.view).refresh(&self.forest);}

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in self.objects.iter().filter(|row| self.visible(row.id)) {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for line in self.lines.iter().filter(|line| self.visible(line.id)) {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self.paths.iter().filter(|row| self.visible(row.id)) {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds { Some(bounds) => bounds.include(point), None => bounds = Some(crate::bounds::Bounds::point(point)) }
            }
        }
        for row in self.points.iter().filter(|row| self.visible(row.id)) {
            let p = row.point(); let p = [p[0], p[1], p[2]];
            match &mut bounds { Some(bounds) => bounds.include(p), None => bounds = Some(crate::bounds::Bounds::point(p)) }
        }
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for id in ids { if let Some(next) = self.selected_bounds(*id) {
            if let Some(b) = &mut bounds { b.include(next.min); b.include(next.max); } else { bounds = Some(next); }
        } } bounds
    }

    pub fn placement(&self, id: ObjectId) -> Option<session_rust::Xform> {
        self.objects.iter().find(|r| r.id==id).map(|r| r.model.clone())
            .or_else(|| self.lines.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.paths.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.points.iter().find(|r| r.id==id).map(|r| r.model.clone()))
    }

    pub fn with_descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {self.forest.descendants(ids).into_iter().filter(|id|self.contains(*id)).collect()}

    pub fn place_owner(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_following(id,model)
    }

    pub(crate) fn place_following(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id==id) { self.place_mesh(id,model) }
        else if self.lines.iter().any(|r| r.id==id) { self.place_line(id,model) }
        else if self.paths.iter().any(|r| r.id==id) { self.place_path(id,model) }
        else { self.place_point(id,model) }
    }

    pub fn translate(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<(), &'static str> {
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        if let Some(row) = self.objects().iter().find(|r| r.id == id) {
            if row.geometry().is_none() { return Err("Reload editable sources before Move"); }
            let model = &shift * &row.model; return self.place(id, model);
        }
        if let Some(row) = self.lines().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_line(id, model); }
        if let Some(row) = self.paths().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_path(id, model); }
        if let Some(row) = self.points().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_point(id, model); }
        Err("Object not found")
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) { return None; }
        let object = self.objects.iter().find(|object| object.id == id)?;
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn line_capacity_bytes(&self) -> usize { self.lines.capacity() * std::mem::size_of::<LineObject>() }

    pub fn points(&self) -> &[PointObject] { &self.points }

    pub fn point_capacity_bytes(&self) -> usize { self.points.capacity() * std::mem::size_of::<PointObject>() }

    pub fn insert_point(&mut self, prepared: crate::marker::PreparedPoint) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.points.iter().any(|row| row.guid == guid) || self.paths.iter().any(|row| row.guid == guid)
            || self.objects.iter().any(|row| row.guid == guid) || self.lines.iter().any(|row| row.prepared.source.guid() == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let label=prepared.source.name.clone();let node_guid=guid.clone();self.points.push(PointObject { id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_point(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid point placement"); }
        let row = self.points.iter_mut().find(|row| row.id == id).ok_or("Point not found")?;
        let point = model.transform_point(&row.prepared.source);
        if (0..3).any(|i| !(point[i] as f32).is_finite()) { return Err("Point placement exceeds the display range"); }
        row.model = model; Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        for row in &mut self.paths { row.controls_visible = visible && matches!(row.prepared.source, crate::chain::Source::Curve(..)); }
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self.paths.iter().filter(|row| row.controls_visible && self.visible(row.id)) {
            for control in row.prepared.source.controls() {
                let p = control.position().expect("Validated original control");
                let p = row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker { center: p.to_f32(), ..control.marker().expect("Prepared control marker") };
                markers.push(ControlMarker { parent: row.id, control, display });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] { &self.paths }

    pub fn path_capacity_bytes(&self) -> usize { self.paths.capacity() * std::mem::size_of::<PathObject>() }

    pub fn insert_path(&mut self, prepared: crate::chain::PreparedChain) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.paths.iter().any(|row| row.guid == guid) || self.objects.iter().any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.prepared.source.guid() == guid) { guid = uuid::Uuid::new_v4().to_string(); }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.visible(),prepared.source.locked());
        let label=prepared.source.name().to_owned();let node_guid=guid.clone();self.paths.push(PathObject { controls_visible: false, id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_path(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid path placement"); }
        let row = self.paths.iter_mut().find(|row| row.id == id).ok_or("Path not found")?;
        for p in row.prepared.source.coordinates().into_iter().chain(row.prepared.source.bounds_coordinates()) {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) { return Err("Path placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn lines(&self) -> &[LineObject] { &self.lines }

    pub fn insert_line(&mut self, prepared: crate::stroke::PreparedLine) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let guid=prepared.source.guid().to_owned();let label=prepared.source.name.clone();
        self.lines.push(LineObject { id, prepared, model: session_rust::Xform::identity() });
        crate::layer_edit::attach_generated(self,id,guid,label)?;
        Ok(id)
    }

    pub fn place_line(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid line placement"); }
        let row = self.lines.iter_mut().find(|row| row.id == id).ok_or("Line not found")?;
        for point in [row.prepared.source.start(), row.prepared.source.end()] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) { return Err("Line placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut [Object] { &mut self.objects }

    pub(crate) fn unload(&mut self, origins: &std::collections::HashSet<uuid::Uuid>, epoch: u64) {
        for row in &mut self.objects {
            if let Some(origin) = row.origin().filter(|origin| origins.contains(&origin.id)).cloned() {
                row.editable = crate::edit_source::EditSource::Released { origin, epoch };
            }
        }
    }

    pub(crate) fn hydrate(&mut self, prepared: &crate::rehydrate::PreparedReload) {
        for row in &mut self.objects {
            if !prepared.key.matches(row) { continue; }
            let Some(geometry) = prepared.geometry(&row.metadata.source_guid).cloned() else { continue; };
            let source = crate::document::Source { document: Rc::clone(&prepared.document),
                guid: row.metadata.source_guid.clone(), origin: Rc::clone(&prepared.key.origin) };
            row.editable = crate::edit_source::EditSource::Loaded { geometry, source: Some(source) };
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&prepared.model.m) { return Err("Placement must be finite, affine and fit the display range"); }
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self.objects.iter().any(|object| object.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.geometry.is_visible,prepared.geometry.is_locked);
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(&prepared.geometry));
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {cad:None,surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }

    pub fn insert_cad(&mut self,source:Rc<session_rust::BRep>)->Result<ObjectId,&'static str> {
        let (prepared,mesh)=crate::cad_display::prepare(source)?;let prepared=Rc::new(prepared);let id=self.insert(mesh)?;self.objects.iter_mut().find(|row|row.id==id).unwrap().cad=Some(prepared);Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
        let prepared=Rc::new(crate::surface::Prepared::new(source)?);let mesh=prepared.mesh()?;let id=self.insert(mesh)?;
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
    }

    pub(crate) fn copy_row(&mut self,id:ObjectId,all:bool)->Result<ObjectId,&'static str> {
        if self.objects.iter().any(|row|row.id==id&&row.geometry().is_none()) {return Err("Reload editable sources before copying");}
        if !self.contains(id) {return Err("Object is gone");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let copy=ObjectId(self.next_id);let guid=uuid::Uuid::new_v4().to_string();
        if let Some(row)=self.objects.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.objects.push(row);}
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
        for channel in [&mut view.faces,&mut view.edges] {if let Some(rgb)=channel.get(&id).copied() {channel.insert(copy,rgb);}}
        Ok(copy)
    }

    pub(crate) fn row_guid(&self,id:ObjectId)->Option<&str> {
        self.objects.iter().find(|row|row.id==id).map(|row|row.guid.as_str())
            .or_else(||self.paths.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.points.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.lines.iter().find(|row|row.id==id).map(|row|row.prepared.source.guid()))
    }

    pub fn clear(&mut self) {
        self.created=None;
        self.ids=Default::default();
        self.view=Default::default();
        self.forest=Default::default();
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
        self.objects = Vec::new();
        self.extra = None;
    }

    pub fn replace(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        self.clear();
        self.import(loaded)
    }

    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        let mut objects=Vec::new();let key=self.next_id;
        for prepared in loaded.meshes {
            let guid=prepared.geometry.guid().to_owned();let name=prepared.geometry.name.clone();
            let id=self.insert(prepared)?;objects.push((guid,id,name));
        }
        self.forest.add(key,loaded.document.name.clone(),loaded.tree,&objects);let colours=self.forest.colours(key);Rc::make_mut(&mut self.view).faces.extend(colours);Ok(())
    }

    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_mesh(id,model)
    }

    fn place_mesh(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if !crate::placement::valid(&model.m)
        {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        if object.geometry().is_none() { return Err("Reload editable sources before Move"); }
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id); self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id); self.points.retain(|row| row.id != id);
        if self.extra == Some(id) { self.extra = None; }
        existed
    }

    pub fn contains(&self,id:ObjectId)->bool {self.ids.contains(&id)}

    pub fn next(&self,current:Option<ObjectId>)->Option<ObjectId> {
        let mut ids=self.objects.iter().map(|r|r.id).chain(self.lines.iter().map(|r|r.id)).chain(self.paths.iter().map(|r|r.id)).chain(self.points.iter().map(|r|r.id)).filter(|id|self.pick_id(id.value()).is_some()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());if ids.is_empty() {return None;}
        let start=ids.iter().position(|id|Some(*id)==current).map_or(0,|i|i+1);Some(ids[start%ids.len()])
    }

    pub fn add_box(&mut self) -> Result<ObjectId, &'static str> {
        let mut source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        source.set_objectcolor(session_rust::Color::new(0.65, 0.65, 0.65, 1.0));
        let id = self.insert(PreparedMesh::new(source)?)?;
        self.place(id, session_rust::Xform::translation(0.9, 0.0, 0.4))?;
        Ok(id)
    }

    pub fn toggle_extra(&mut self) {
        if let Some(id) = self.extra.take() {
            self.remove(id);
        } else {
            let extra = PreparedMesh::triangle([
                [-0.9, 0.3, 0.5], [-0.4, 0.3, 0.5], [-0.65, 0.9, 0.5],
            ], [0.2, 0.8, 0.3]).expect("Valid extra triangle");
            self.extra = Some(self.insert(extra).expect("ID available"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unloading_policy_protects_generated_unlocated_and_modified_sources() {
        let mut scene = Scene::demo(); assert!(!scene.objects[0].can_unload());
        let bytes = crate::specimen::bytes();
        scene.replace(crate::document::load(&bytes).unwrap()).unwrap(); assert!(!scene.objects[0].can_unload());
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:policy".into()));
        scene.replace(crate::document::load_at(&bytes, Some(url)).unwrap()).unwrap();
        assert!(scene.objects.iter().all(Object::can_unload));
        let row = &mut scene.objects[0];
        Rc::make_mut(&mut row.metadata).is_locked = true; assert!(!row.can_unload());
        Rc::make_mut(&mut row.metadata).is_locked = row.geometry().unwrap().is_locked;
        assert!(row.can_unload());
        let source = row.source().cloned(); let geometry = Rc::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0));
        row.editable = crate::edit_source::EditSource::Loaded { geometry, source }; assert!(!row.can_unload());
    }

    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
        let mut scene = Scene::demo();
        let before = scene.objects.clone();
        scene.next_id = u32::MAX - 1;
        let loaded = crate::document::load(&crate::specimen::bytes()).unwrap();
        let mut history = crate::history::History::default();
        assert!(history.try_edit(&mut scene, |scene| scene.replace(loaded)).is_err());
        assert_eq!(scene.next_id, u32::MAX);
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(object.geometry().unwrap(), old.geometry().unwrap()));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
        }
        assert!(!history.undo(&mut scene));
    }

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
        let second = scene.objects()[1].id;
        assert!(scene.remove(first));
        assert_eq!(scene.objects()[0].id, second);
        assert_eq!(scene.next(None), Some(second));
        assert!(!scene.contains(first));
        scene.toggle_extra();
        assert_ne!(scene.objects()[1].id, first);
        assert_ne!(scene.objects()[1].id, second);
    }

    #[test]
    fn toggling_the_extra_object_uses_its_id_after_other_removals() {
        let mut scene = Scene::demo();
        scene.toggle_extra();
        let extra = scene.objects()[2].id;
        scene.remove(scene.objects()[0].id);
        scene.toggle_extra();
        assert!(!scene.contains(extra));
        assert_eq!(scene.objects().len(), 1);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "pick_gpu_tests.rs"]
mod pick_gpu_tests;

impl Scene {
    pub fn deform_part(&mut self,target:crate::pick_sources::Target,delta:&session_rust::Xform) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
            if row.surface.is_some()||row.cad.is_some() {return Err("CAD surface part editing connects after source parameter mapping");}
            let mut source=(**row.geometry().ok_or("Reload editable sources before editing a part")?).clone();
            for key in crate::part_preview::keys(&source,target.part)? {
                let point=local.transform_point(&source.vertex_point(key).ok_or("Missing source vertex")?);
                source.vertex.get_mut(&key).ok_or("Missing source vertex")?.set_position(point);
            }
            source.clear_triangle_bvh();
            let prepared=crate::prepared::PreparedMesh::new(source)?;
            for vertex in prepared.display.vertices() {let point=model.transform_point(&session_rust::Point::new(vertex[0] as f64,vertex[1] as f64,vertex[2] as f64));
                if (0..3).any(|i| !(point[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}}
            row.mesh=Rc::new(prepared.display); row.editable=crate::edit_source::EditSource::Loaded {geometry:prepared.geometry,source:None}; return Ok(());
        }
        let row=self.paths.iter_mut().find(|r|r.id==target.owner).ok_or("Part owner is missing")?;
        let (crate::chain::Source::Curve(curve,_),Part::Control(index))=(&row.prepared.source,target.part) else {return Err("Select a curve control point");};
        let mut source=(**curve).clone(); let point=local.transform_point(&source.get_cv(index).ok_or("Missing curve control")?);
        let weight=source.get_cv_4d(index).ok_or("Missing curve control")?.3;
        let set=if source.m_is_rat {source.set_cv_4d(index,point[0]*weight,point[1]*weight,point[2]*weight,weight)} else {source.set_cv_point(index,&point)};
        if !set {return Err("Cannot set curve control");}
        let prepared=crate::chain::PreparedChain::curve(Rc::new(source))?;
        for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()) {
            let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}
        }
        row.prepared=prepared;Ok(())
    }
}
```

### 2. `src/document.rs`

Attach original CAD ownership and preserve explicit saving and part-edit guards.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/59c-source-row-04.rs"
```

### 3. `src/part_preview.rs`

Attach original CAD ownership and preserve explicit saving and part-edit guards.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/59c-source-row-05.rs"
```

---

<a id="checkpoint-59d-identities"></a>

### Step 5 of 7 · Pick and highlight an original CAD border (`59d-identities`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/pick_sources.rs, src/…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original face or edge</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">complete source highlig…</text>
</svg>

Resolve every display border segment to its original BRep edge. A selected edge highlights its complete chain; each shaded triangle resolves to the original face index.

> **Check:** *Should each segment of a circular rim become a separate CAD edge?* — No. Many display segments approximate one original rim. They all resolve to one original edge identity, and highlighting follows the complete sampled border.

### 1. `src/pick_sources.rs`

Resolve original BRep edge identities and highlight complete original borders.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge);
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends = ends.map(|p| row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
```

Replace it with:

```rust
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends = ends.map(|p| row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
```

**Locate this block:**

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }
```

Replace it with:

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }
```

**Complete updated block reference:**

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode { #[default] Object, Face, Edge, Control }

#[derive(Default)]
pub struct Sources {
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(rows: &mut Vec<Target>, kind: Kind, target: Target) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?; rows.push(target); Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene: &Scene) -> Result<Self, &'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [scene.pick_id(owner.value()).map_or(0, ObjectId::value), tag];
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            for &key in &row.mesh.parts.faces { append(&mut sources.faces, Kind::Face, Target { owner: row.id, part: Part::Face(key) })?; }
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends = ends.map(|p| row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
            let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part: Part::Curve })?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points: Vec<_> = row.points().iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for row in scene.control_markers() {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: Part::Control(row.control.index) })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }

    pub fn resolve(&self, scene: &Scene, pixel: [u32; 2], mode: Mode) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address), _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

### 2. `src/selection.rs`

Resolve original BRep edge identities and highlight complete original borders.

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{pick_sources::{Part, Target}, scene::Scene};

impl Target {
    pub fn valid(self, scene: &Scene) -> bool {
        if scene.pick_id(self.owner.value()).is_none() { return false; }
        match self.part {
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

### 3. `src/highlight.rs`

Resolve original BRep edge identities and highlight complete original borders.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/59d-identities-05.rs"
```

---

<a id="checkpoint-59e-cache"></a>

### Step 6 of 7 · Inspect shared CAD owners and compact cache counts (`59e-cache`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/memory.rs, src/browse…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">compact UV and border c…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">diagnostics</text>
</svg>

Deduplicate original CAD and preparation owners across live and history scenes. Expose face, edge and UV-cache counts without mistaking sample counts for bytes or new topology.

> **Check:** *Does Undo require another complete original BRep and UV cache?* — No. Unchanged history scenes share both owners. Row counts can increase across snapshots while unique source and preparation counts stay at one.

### 1. `src/memory.rs`

Expose unique CAD owners and original identity/UV-cache counts for acceptance.

After that block, type:

```rust
--8<-- "journey/code/59e-cache-01.rs" skip=59
```

### 2. `src/browser.rs`

Expose unique CAD owners and original identity/UV-cache counts for acceptance.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/59e-cache-02.rs"
```

---

<a id="checkpoint-59f-cad"></a>

### Step 7 of 7 · Draw the original bore and prove its empty hole (`59f-cad`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/editor.rs, src/comman…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">empty hole GPU readback</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">original cap and rim id…</text>
</svg>

Connect Example CAD and copy all original-source, placement, orientation and actual GPU checks. A rendered hole has no shaded face target; its five visible cap boundaries retain original edge IDs.

> **Check:** *What proves a visible hole is absent from face picking too?* — Actual integer GPU readback through the hole returns no face target. A point on the cap resolves to its original face, and every visible cap border resolves to its original CAD edge.

### 1. `src/editor.rs`

Create the original CAD bore through one checked document transaction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Replace(bytes) => {
```

Replace it with:

```rust
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Replace(bytes) => {
```

**Locate this block:**

```rust
    Show,
    AddBox,
    AddNested,
    AddSurface,
    AddLine,
    AddPoint,
```

Replace it with:

```rust
    Show,
    AddBox,
    AddNested,
    AddCad,
    AddSurface,
    AddLine,
    AddPoint,
```

**Complete updated block reference:**

```rust
use crate::{background::Background, camera::Camera, history::History, scene::{ObjectId, Scene}};

pub enum Action {
    CanvasStart {from:[f32;2],to:[f32;2],size:[f64;2],modified:bool,pointer:i32},
    CanvasMove {screen:[f32;2],size:[f64;2]},
    CanvasRelease {screen:[f32;2],size:[f64;2],add:bool,remove:bool},
    CanvasHit(Option<ObjectId>),CanvasCancel,
    SelectRows {ids:Vec<ObjectId>,add:bool,remove:bool},
    PanScreen {delta:[f64;2],size:[f64;2]},ZoomAt {factor:f32,screen:[f32;2]},
    Touch(Vec<Action>),
    LayerEdit {key:String,edit:crate::layer_edit::Edit},
    LayerProperty {key:String,property:crate::layer_properties::Property},
    TreePick {ids:Vec<ObjectId>,additive:bool},
    GizmoPress {pointer:i32,screen:[f32;2],size:[f64;2],modified:bool},
    GizmoTouch {pointer:i32,screen:[f32;2],size:[f64;2]},
    GizmoMove {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoRelease {pointer:i32,screen:[f32;2],size:[f64;2],shift:bool},
    GizmoCancel,
    GizmoNumber(String),
    Snap(crate::snap::Setting),
    ToolScreen { screen: [f32; 2], size: [f64; 2], commit: bool },
    BeginTool(crate::tool::Shape),
    ToolPoint([f64; 3]),
    ToolHover(Option<[f64; 3]>),
    CancelTool,
    Pick([f32; 2]),
    Picked(Option<ObjectId>),
    PickedTarget(Option<crate::pick_sources::Target>),
    PickedMany { target: Option<crate::pick_sources::Target>, additive: bool },
    SelectAll,
    PickMode(crate::pick_sources::Mode),
    Import(Vec<u8>),
    Replace(Vec<u8>),
    ImportAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    ReplaceAt(Vec<u8>, std::rc::Rc<crate::reload_url::ReloadUrl>),
    Close,
    UnloadSources,
    Hide,
    Show,
    AddBox,
    AddNested,
    AddCad,
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
    Delete,
    Undo,
    Redo,
    Background,
    Zoom(f32),
    Pan(f32, f32),
    Orbit(f64, f64),
    Isometric,
    ResetView,
    Fit,
    FitSelected,
    Projection(crate::camera::Projection),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Change {
    View,
    Scene,
    Preview,
}

pub struct Editor {
    pub canvas_drag:Option<crate::canvas_drag::Drag>,
    pub gizmo_edit:Option<crate::gizmo_edit::Edit>,
    gizmo_hover:Option<crate::gizmo::Handle>,
    pub edit_preview: Option<crate::edit_preview::Preview>,
    pub snapping: crate::snap::Settings,
    snap_index: Option<crate::snap_index::Index>,
    pub draft: Option<crate::tool::Draft>,
    pub tool_status: String,
    pub scene: Scene,
    pub selected: Option<ObjectId>,
    pub selected_part: Option<crate::pick_sources::Part>,
    pub pick_mode: crate::pick_sources::Mode,
    objects: crate::object_selection::Objects,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            canvas_drag:None, gizmo_edit:None, gizmo_hover:None, edit_preview: None, snapping: Default::default(), snap_index: None,
            draft: None, tool_status: String::new(),
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(), objects: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
    }
}

impl Editor {
    pub fn display_scene(&self) -> &Scene {
        self.canvas_drag.as_ref().and_then(|d|d.preview.as_ref()).map_or_else(||self.gizmo_edit.as_ref().map_or_else(||self.edit_preview.as_ref().map_or(&self.scene,|p| &p.scene),|g|g.preview.scene()),|p|&p.scene)
    }

    pub fn gizmo_origin(&self)->Option<[f64;3]> {
        if self.draft.is_some()||self.canvas_drag.is_some() {return None;}
        if let Some(edit)=self.gizmo_edit.as_ref() {return Some(edit.origin());}
        if let Some(target)=self.selection().filter(|_|self.selected_part.is_some()) {return crate::part_preview::origin(&self.scene,target);}
        self.scene.bounds_of(&self.selected_objects()).map(|b|b.centre())
    }

    pub fn gizmo_hit(&self,screen:[f32;2],size:[f64;2])->Option<crate::gizmo::Handle> {self.gizmo_hit_radius(screen,size,8.)}

    pub fn gizmo_hit_radius(&self,screen:[f32;2],size:[f64;2],radius:f64)->Option<crate::gizmo::Handle> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
        let origin=self.gizmo_origin()?;
        let scale=crate::widget_layout::world_per_pixel(&self.camera.view_projection(),origin,size[1])?;
        let ray=self.camera.ray(screen)?;
        crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2])).hit_with_radius(&ray.origin,&ray.direction,scale,radius)
    }

    pub fn number_prompt(&self,size:[f64;2])->Option<crate::number_box::NumberPrompt> {
        let edit=self.gizmo_edit.as_ref()?;let handle=edit.gizmo.typing?;
        let origin=edit.origin();let matrix=self.camera.view_projection();let scale=crate::widget_layout::world_per_pixel(&matrix,origin,size[1])?;
        let point=edit.gizmo.handle_point(handle,scale);
        let at=crate::snap_math::Screen {matrix:matrix.m,size}.point([point[0],point[1],point[2]])?;
        let unit=handle.labels().2;
        Some(crate::number_box::NumberPrompt {handle,title:handle.title(),unit,hint:if unit=="factor" {"1"} else {"0"},at:at.map(|v|v as f32)})
    }

    pub fn gizmo_active(&self)->f32 {crate::gizmo_edit::code(self.gizmo_edit.as_ref().and_then(|e|e.active()).or(self.gizmo_hover))}

    pub fn hover_gizmo(&mut self,screen:Option<([f32;2],[f64;2])>) {
        self.gizmo_hover=if self.gizmo_edit.is_some() {None} else {screen.and_then(|(point,size)|self.gizmo_hit(point,size))};
    }

    fn gizmo_press(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool)->Result<Change,&'static str> {
        self.gizmo_press_radius(pointer,screen,size,modified,8.)
    }

    fn gizmo_press_radius(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],modified:bool,radius:f64)->Result<Change,&'static str> {
        if modified || self.draft.is_some() || self.gizmo_edit.as_ref().is_some_and(|g|g.pointer.is_some()) {return Ok(Change::View);}
        let Some(handle)=self.gizmo_hit_radius(screen,size,radius) else {return Ok(Change::View);};
        let origin=self.gizmo_origin().ok_or("No editable selection")?;
        let preview=crate::gizmo_edit::Preview::new(&self.scene,self.selected_objects(),self.selection().filter(|_|self.selected_part.is_some()))?;
        let ray=self.camera.ray(screen).ok_or("Invalid handle ray")?;
        let gizmo=crate::gizmo::Gizmo::new(session_rust::Point::new(origin[0],origin[1],origin[2]));
        self.gizmo_edit=crate::gizmo_edit::Edit::begin(gizmo,handle,&ray,preview,pointer,screen);Ok(Change::View)
    }

    fn gizmo_move(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if size.iter().any(|v| !v.is_finite() || *v<=0.) {return Err("Invalid handle viewport");}
        let Some(ray)=self.camera.ray(screen) else {return Err("Invalid handle ray");};
        let Some(edit)=self.gizmo_edit.as_mut() else {return Ok(Change::View);};
        Ok(if edit.update(pointer,screen,size,&ray,shift)? {Change::Preview} else {Change::View})
    }

    fn gizmo_release(&mut self,pointer:i32,screen:[f32;2],size:[f64;2],shift:bool)->Result<Change,&'static str> {
        if self.gizmo_edit.as_ref().is_none_or(|g|g.pointer!=Some(pointer)) {return Ok(Change::View);}
        self.gizmo_move(pointer,screen,size,shift)?;
        let edit=self.gizmo_edit.as_mut().unwrap();
        if edit.click() {self.tool_status=format!("{}: type a value in {}, Enter applies, Esc closes",edit.active().unwrap().title(),edit.active().unwrap().labels().2);return Ok(Change::Preview);}
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?; self.gizmo_edit=None;
        self.tool_status="Handle edit completed.".into();Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn gizmo_number(&mut self,text:&str)->Result<Change,&'static str> {
        let edit=self.gizmo_edit.as_mut().ok_or("No handle number is being requested")?;
        let handle=edit.gizmo.typing.ok_or("No handle number is being requested")?;
        let value=crate::gizmo::typed_value(handle,text).map_err(|_|"Enter a finite number; scale needs a positive factor")?;
        let Some(value)=value else {self.gizmo_edit=None;return Ok(Change::Preview);};
        edit.preview.set(edit.gizmo.typed(handle,value))?;
        let changed=edit.preview.commit(&mut self.scene,&mut self.history)?;self.gizmo_edit=None;
        self.tool_status=format!("{} {value} {}",handle.title(),handle.labels().2);Ok(if changed {Change::Scene} else {Change::Preview})
    }

    fn update_edit_preview(&mut self) -> Result<Change, &'static str> {
        let Some(preview)=self.edit_preview.as_mut() else { return Ok(Change::View); };
        let offset=self.draft.as_ref().and_then(|d| d.points.first().zip(d.cursor)).map(|(a,b)| std::array::from_fn::<_,3,_>(|i| b[i]-a[i])).unwrap_or([0.;3]);
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?; Ok(Change::Preview)
    }

    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.scene).chain(self.history.scenes())
    }

    pub fn presentation_scenes(&self)->impl Iterator<Item=&Scene> {
        self.scenes().chain(self.canvas_drag.iter().filter_map(|d|d.preview.as_ref()).flat_map(|p|p.scenes())).chain(self.edit_preview.iter().flat_map(|p|p.scenes())).chain(self.gizmo_edit.iter().flat_map(|g|g.preview.scenes()))
    }

    pub fn reload_keys(&self) -> Vec<crate::rehydrate::ReloadKey> {
        let mut seen = std::collections::HashSet::new();
        self.scene.objects().iter().filter_map(crate::rehydrate::ReloadKey::of)
            .filter(|key| seen.insert(key.origin.id)).collect()
    }

    fn scenes_mut(&mut self) -> impl Iterator<Item = &mut Scene> {
        std::iter::once(&mut self.scene).chain(self.history.scenes_mut())
    }

    pub fn hydrate(&mut self, values: Vec<(crate::rehydrate::ReloadKey, Vec<u8>)>) -> Result<bool, &'static str> {
        if values.is_empty() { return Ok(false); }
        let mut seen = std::collections::HashSet::new();
        for (key, _) in &values {
            if !seen.insert(key.origin.id) || !self.scene.objects().iter().any(|row| key.matches(row)) {
                return Ok(false);
            }
        }
        let prepared: Vec<_> = values.into_iter().map(|(key, bytes)|
            crate::rehydrate::PreparedReload::new(key, &bytes)).collect::<Result<_, _>>()?;
        for plan in &prepared {
            if self.scenes().flat_map(|scene| scene.objects()).filter(|row| plan.key.matches(row))
                .any(|row| !plan.validates(row)) { return Err("Reloaded source GUID or metadata does not match"); }
        }
        for scene in self.scenes_mut() {
            for plan in &prepared { scene.hydrate(plan); }
        }
        if self.edit_preview.take().is_some() { self.draft=None; } self.gizmo_edit=None; self.snap_index = None; Ok(true)
    }

    fn unload_sources(&mut self) -> Result<(), &'static str> {
        let mut origins: std::collections::HashSet<_> = self.scenes().flat_map(|scene| scene.objects())
            .filter(|row| row.can_unload()).filter_map(|row| row.origin().map(|origin| origin.id)).collect();
        for row in self.scenes().flat_map(|scene| scene.objects()) {
            if let Some(origin) = row.origin() {
                if !row.can_unload() { origins.remove(&origin.id); }
            }
        }
        if origins.is_empty() { return Err("No eligible imported sources to unload"); }
        let epoch = self.last_release.checked_add(1).ok_or("Release epochs exhausted")?;
        for scene in self.scenes_mut() { scene.unload(&origins, epoch); }
        self.last_release = epoch;
        Ok(())
    }

    pub fn move_object(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
        let object=self.scene.objects().iter().find(|row|row.id==id).ok_or("Object not found")?;
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if object.geometry().is_none() {return Err("Reload editable sources before Move");}
        if offset.iter().all(|&value| value == 0.0) { return Ok(Change::View); }
        let ids=self.scene.with_descendants(&[id]);let delta=session_rust::Xform::translation(offset[0],offset[1],offset[2]);
        self.history.try_edit(&mut self.scene,|scene| {for id in ids {let matrix=&delta*&scene.placement(id).ok_or("Object not found")?;scene.place_following(id,matrix)?;}Ok(())})?;
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn delete_object(&mut self, id: ObjectId) -> Result<Change, &'static str> {
        if !self.scene.contains(id) {return Err("Object not found");}
        if self.scene.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        if self.scene.objects().iter().find(|row| row.id == id).is_some_and(|row| row.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&[id]);
        self.history.edit(&mut self.scene, |scene| {for id in ids {scene.remove(id);}});
        if self.edit_preview.take().is_some() { self.draft=None; }
        self.objects.retain(&self.scene);
        if self.selected == Some(id) { self.selected = self.objects.primary(); self.selected_part = None; }
        self.gizmo_edit=None; self.snap_index = None; Ok(Change::Scene)
    }

    pub fn selected_objects(&self) -> Vec<ObjectId> {
        if self.selected_part.is_some() { return Vec::new(); }
        if self.objects.primary() == self.selected { self.objects.rows().to_vec() }
        else { self.selected.into_iter().filter(|id| self.scene.pick_id(id.value()).is_some()).collect() }
    }

    pub fn select_objects(&mut self, rows: impl IntoIterator<Item = ObjectId>, additive: bool) {
        if additive { let old = self.selected_objects(); self.objects.set(&self.scene, old, false); }
        self.selected_part = None; self.objects.set(&self.scene, rows, additive); self.selected = self.objects.primary();
    }

    pub fn select_rows(&mut self,ids:Vec<ObjectId>,add:bool,remove:bool) {
        if remove {let old=self.selected_objects();self.objects.set(&self.scene,old,false);self.objects.remove(&self.scene,ids);self.selected=self.objects.primary();self.selected_part=None;}
        else {self.select_objects(ids,add);}
    }

    fn finish_canvas(&mut self)->Result<Change,&'static str> {
        let Some(mut drag)=self.canvas_drag.take() else {return Ok(Change::View);};
        let (add,remove)=drag.released.unwrap_or((false,false));
        if let Some(preview)=drag.preview.take() {let changed=preview.commit(&mut self.scene,&mut self.history)?;if changed {self.snap_index=None;}return Ok(if changed {Change::Scene} else {Change::Preview});}
        if drag.missed {if let Some(rectangle)=drag.rectangle() {let ids=crate::rect_select::found(&self.scene,&self.camera.view_projection().m,drag.size,rectangle);self.select_rows(ids,add,remove);}}
        Ok(Change::View)
    }

    fn repair_selection(&mut self) {
        if self.objects.primary() == self.selected && self.selected_part.is_none() {
            self.objects.retain(&self.scene); self.selected = self.objects.primary();
        } else { self.objects.clear(); }
    }

    fn move_selection(&mut self, offset: [f64; 3]) -> Result<Change, &'static str> {
        if !offset.iter().all(|n| n.is_finite()) { return Err("Move needs finite coordinates"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Err("Select an object before Move"); }
        if offset.iter().all(|&n| n == 0.) { return Ok(Change::View); }
        let mut preview=crate::edit_preview::Preview::new(&self.scene,ids)?;
        preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
        preview.commit(&mut self.scene,&mut self.history)?;Ok(Change::Scene)
    }

    fn delete_selection(&mut self) -> Result<Change, &'static str> {
        if self.selected_part.is_some() { return Err("Use Select Object before Delete"); }
        let ids = self.selected_objects(); if ids.is_empty() { return Ok(Change::View); }
        if self.scene.objects().iter().any(|r| ids.contains(&r.id) && r.geometry().is_none()) {
            return Err("Reload editable sources before Delete");
        }
        let ids=self.scene.with_descendants(&ids);
        self.history.edit(&mut self.scene, |scene| { for id in ids { scene.remove(id); } });
        self.objects.clear(); self.selected = None; Ok(Change::Scene)
    }

    pub fn selection(&self) -> Option<crate::pick_sources::Target> {
        self.selected.map(|owner| crate::pick_sources::Target { owner, part: self.selected_part.unwrap_or(crate::pick_sources::Part::Object) })
    }

    pub fn whole_selection(&self) -> Option<ObjectId> { self.selected.filter(|_| self.selected_part.is_none()) }

    pub fn tool_line(&self, text: &str) -> Option<Result<Action, &'static str>> {
        if self.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some() || text.trim().eq_ignore_ascii_case("Escape")) {
            return Some(Ok(if text.trim().eq_ignore_ascii_case("Escape") {Action::GizmoCancel} else {Action::GizmoNumber(text.to_owned())}));
        }
        self.draft.as_ref()?;
        if text.trim().is_empty() || text.eq_ignore_ascii_case("Escape") { return Some(Ok(Action::CancelTool)); }
        text.contains(',').then(|| crate::tool_coords::parse(text).map(Action::ToolPoint))
    }

    fn place_tool_point(&mut self, point: [f64; 3]) -> Result<Change, &'static str> {
        let draft = self.draft.as_mut().ok_or("No point tool is running")?;
        let name = draft.tool.name();
        let Some(built) = draft.submit(point)? else { self.tool_status = draft.prompt(); self.snap_index=None; return self.update_edit_preview(); };
        if let crate::tool::Built::Move(offset)=built {
            let preview=self.edit_preview.as_mut().ok_or("No placement preview is running")?;
            preview.set(session_rust::Xform::translation(offset[0],offset[1],offset[2]))?;
            let changed=preview.commit(&mut self.scene,&mut self.history)?;
            if self.edit_preview.take().is_some() { self.draft=None; } self.tool_status="Move completed.".into();
            return Ok(if changed { Change::Scene } else { Change::Preview });
        }
        self.history.try_edit(&mut self.scene, |scene| match built {
            crate::tool::Built::Point(point) => scene.insert_point(point).map(|_| ()),
            crate::tool::Built::Line(line) => scene.insert_line(line).map(|_| ()),
            crate::tool::Built::Move(_) => unreachable!("Move uses its placement preview"),
        })?;
        self.draft = None; self.tool_status = format!("{name} created."); Ok(Change::Scene)
    }

    pub fn snap_point(&mut self, screen: [f32; 2], size: [f64; 2]) -> Option<crate::snap::Hit> {
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
            let exclude=self.edit_preview.as_ref().filter(|_| from.is_some()).map_or_else(Vec::new,|p| p.ids().to_vec());
            let index = self.snap_index.get_or_insert_with(|| crate::snap_index::Index::new(&self.scene).excluding(&exclude));
            let viewport = crate::snap_math::Screen { matrix: self.camera.view_projection().m, size };
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        hit.or_else(|| crate::snap_index::fallback(self.snapping, &self.camera, screen))
    }

    pub fn snap_usage(&self) -> [u64; 2] { self.snap_index.as_ref().map_or([0,0], |index| [index.builds,index.visits as u64]) }

    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        match action {
            Action::Touch(actions)=> {let mut change=Change::View;for action in actions {let next=self.apply(action)?;if next!=Change::View {change=next;}}return Ok(change);},
            Action::CanvasStart {from,to,size,modified,pointer}=>{self.canvas_drag=crate::canvas_drag::Drag::new(from,to,size,modified);if let Some(drag)=self.canvas_drag.as_mut() {drag.pointer=pointer;}return Ok(Change::View);},
            Action::CanvasMove {screen,size}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.released.is_none()&&drag.size==size&&screen.iter().all(|v|v.is_finite()) {drag.to=screen;drag.update(&self.camera)?;}if drag.preview.is_some() {return Ok(Change::Preview);}}return Ok(Change::View);},
            Action::CanvasRelease {screen,size,add,remove}=>{if let Some(drag)=self.canvas_drag.as_mut() {if drag.size!=size {self.canvas_drag=None;return Ok(Change::Preview);}drag.to=screen;drag.released=Some((add,remove));drag.update(&self.camera)?;if drag.preview.is_some()||drag.missed {return self.finish_canvas();}}return Ok(Change::View);},
            Action::CanvasHit(hit)=>{
                let Some(drag)=self.canvas_drag.as_ref() else {return Ok(Change::View);};let ids=hit.filter(|id|!drag.modified&&self.scene.pick_id(id.value()).is_some());
                if let Some(id)=ids {let mut rows=self.selected_objects();if !rows.contains(&id) {self.select_objects([id],false);rows=vec![id];}if let Err(error)=self.canvas_drag.as_mut().unwrap().grab(&self.scene,rows,&self.camera) {self.canvas_drag=None;return Err(error);}}
                else {self.canvas_drag.as_mut().unwrap().missed=true;}
                if self.canvas_drag.as_ref().unwrap().released.is_some() {return self.finish_canvas();}return Ok(Change::Preview);
            },
            Action::CanvasCancel=>{let preview=self.canvas_drag.take().is_some_and(|d|d.preview.is_some());return Ok(if preview {Change::Preview} else {Change::View});},
            Action::SelectRows {ids,add,remove}=>{self.select_rows(ids,add,remove);return Ok(Change::View);},
            _=>{}
        }
        self.canvas_drag=None;
        let keep_gizmo=matches!(&action,Action::GizmoPress {..}|Action::GizmoTouch {..}|Action::GizmoMove {..}|Action::GizmoRelease {..}|Action::GizmoCancel|Action::GizmoNumber(_)|Action::Zoom(_)|Action::ZoomAt {..}|Action::PanScreen {..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Background|Action::NormalView(_)|Action::Grid(_)|Action::Snap(_));
        let keep_edit=matches!(&action, Action::GizmoPress {..} | Action::GizmoTouch {..} | Action::GizmoMove {..} | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::BeginTool(_) | Action::ToolPoint(_) | Action::ToolScreen {..} | Action::ToolHover(_) | Action::CancelTool | Action::Snap(_) | Action::Zoom(_) | Action::ZoomAt {..} | Action::PanScreen {..} | Action::Pan(_,_) | Action::Orbit(_,_) | Action::Fit | Action::FitSelected | Action::Projection(_) | Action::ResetView | Action::Background | Action::NormalView(_) | Action::Grid(_));
        let mut change=self.apply_action(action)?;
        self.gizmo_hover=None;
        if !keep_gizmo && self.gizmo_edit.take().is_some() && change==Change::View {change=Change::Preview;}
        if !keep_edit && self.edit_preview.take().is_some() { self.draft=None; if change==Change::View { change=Change::Preview; } }
        if change==Change::Scene { self.snap_index=None; }
        Ok(change)
    }

    fn apply_action(&mut self, action: Action) -> Result<Change, &'static str> {

        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None; }
        match action {
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
                self.history.try_edit(&mut self.scene,|scene|crate::layer_edit::apply(scene,&key,edit))?;self.repair_selection();
            },
            Action::LayerProperty {key,property}=>{crate::layer_properties::apply(&mut self.scene,&key,property)?;self.repair_selection();if self.selected.is_some_and(|id|self.scene.pick_id(id.value()).is_none()) {self.selected=None;self.selected_part=None;}self.snap_index=None;return Ok(Change::Preview);},
            Action::Hide=>{let ids=self.scene.with_descendants(&self.selected_objects());std::rc::Rc::make_mut(&mut self.scene.view).hide(&ids,true);self.select_objects([],false);self.snap_index=None;return Ok(Change::Preview);},
            Action::Show=>{std::rc::Rc::make_mut(&mut self.scene.view).show_all();self.snap_index=None;return Ok(Change::Preview);},
            Action::TreePick {ids,additive}=>{let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
            Action::GizmoTouch {pointer,screen,size}=>return self.gizmo_press_radius(pointer,screen,size,false,18.),
            Action::GizmoPress {pointer,screen,size,modified}=>return self.gizmo_press(pointer,screen,size,modified),
            Action::GizmoMove {pointer,screen,size,shift}=>return self.gizmo_move(pointer,screen,size,shift),
            Action::GizmoRelease {pointer,screen,size,shift}=>return self.gizmo_release(pointer,screen,size,shift),
            Action::GizmoCancel=>{self.gizmo_edit=None;self.tool_status="Handle edit cancelled.".into();return Ok(Change::Preview);},
            Action::GizmoNumber(text)=>return self.gizmo_number(&text),
            Action::Snap(setting) => {
                self.tool_status = self.snapping.apply(setting)?;
                if let Some(draft) = self.draft.as_mut() { draft.cursor = None; draft.snapped = None; }
                return self.update_edit_preview();
            }
            Action::ToolScreen { screen, size, commit } => {
                let hit = self.snap_point(screen,size);
                if let Some(draft) = self.draft.as_mut() { draft.snapped = hit.filter(|h| h.owner.is_some() || h.kind == crate::snap::Kind::Grid).map(|h| h.kind); }
                if commit { return hit.map_or(Ok(Change::View), |hit| self.place_tool_point(hit.point)); }
                if let Some(draft) = self.draft.as_mut() { draft.hover(hit.map(|h| h.point))?; }
                return self.update_edit_preview();
            }
            Action::BeginTool(shape) => {
                let preview=if matches!(shape,crate::tool::Shape::Move) {
                    if self.selected_part.is_some() { return Err("Use Select Object before Move"); }
                    Some(crate::edit_preview::Preview::new(&self.scene,self.selected_objects())?)
                } else { None };
                let changed=preview.is_some() || self.edit_preview.is_some(); self.edit_preview=preview;
                self.snap_index=None; self.draft=Some(crate::tool::Draft::new(shape)); self.tool_status=self.draft.as_ref().unwrap().prompt();
                return Ok(if changed { Change::Preview } else { Change::View });
            }
            Action::ToolPoint(point) => return self.place_tool_point(point),
            Action::ToolHover(point) => { if let Some(draft) = self.draft.as_mut() { draft.snapped=None; draft.hover(point)?; } return self.update_edit_preview(); }
            Action::CancelTool => { let changed=self.edit_preview.take().is_some(); self.draft=None; self.tool_status="Tool cancelled.".into(); return Ok(if changed { Change::Preview } else { Change::View }); }
            Action::Picked(id) => self.select_objects(id, false),
            Action::PickedMany { target, additive } => {
                if self.pick_mode != crate::pick_sources::Mode::Object { return self.apply(Action::PickedTarget(target)); }
                self.select_objects(target.filter(|t| t.part == crate::pick_sources::Part::Object && t.valid(&self.scene)).map(|t| t.owner), additive);
            }
            Action::SelectAll => {
                let ids: Vec<_> = self.scene.objects().iter().map(|r| r.id).chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id)).chain(self.scene.points().iter().map(|r| r.id)).collect();
                self.select_objects(ids, false);return Ok(Change::View);
            }
            Action::PickedTarget(target) => {
                let target = target.filter(|target| target.valid(&self.scene));
                self.objects.clear();
                self.selected = target.map(|target| target.owner);
                if let Some(target) = target.filter(|t| t.part == crate::pick_sources::Part::Object) { self.objects.set(&self.scene, [target.owner], false); }
                self.selected_part = target.map(|target| target.part).filter(|part| *part != crate::pick_sources::Part::Object);
            }
            Action::PickMode(mode) => { self.pick_mode = mode; self.selected = None; self.selected_part = None; self.objects.clear(); }
            Action::Pick(screen) => {
                let id = self.camera.ray(screen).and_then(|ray| crate::picking::pick(&self.scene, &ray));
                self.select_objects(id, false);
            }
            Action::UnloadSources => self.unload_sources()?,
            Action::Close => {
                self.scene.clear();
                self.history.clear();
                self.objects.clear(); self.selected = None;
            }
            Action::AddCad=>{let source=std::rc::Rc::new(crate::cad_display::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_cad(source).map(|_|()))?;},
            Action::AddSurface=>{let source=std::rc::Rc::new(crate::surface::example());self.history.try_edit(&mut self.scene,|scene|scene.insert_surface(source).map(|_|()))?;},
            Action::AddNested=>{let loaded=crate::document::load(&crate::tree_specimen::nested(true))?;self.history.try_edit(&mut self.scene,|scene|scene.import(loaded))?;},
            Action::Replace(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::Import(bytes) => {
                let loaded = crate::document::load(&bytes)?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ImportAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.import(loaded))?;
            }
            Action::ReplaceAt(bytes, location) => {
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history.try_edit(&mut self.scene, |scene| scene.replace(loaded))?;
            }
            Action::AddPolyline | Action::AddArrow => {
                let prepared = if matches!(action, Action::AddPolyline) {
                    let mut line = session_rust::Polyline::new(vec![session_rust::Point::new(-0.6, -0.4, 0.0),
                        session_rust::Point::new(0.0, 0.0, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                    line.width = 6.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                    crate::chain::PreparedChain::polyline(std::rc::Rc::new(line))?
                } else {
                    let mut line = session_rust::Line::new(-0.6, 0.3, 0.0, 0.6, 0.3, 0.0);
                    line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                    line.arrowhead = session_rust::Arrowhead::BOTH;
                    crate::chain::PreparedChain::line(std::rc::Rc::new(line))?
                };
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::normal_example::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddColours(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(crate::colour::specimen(mode))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert(prepared).map(|_| ()))?;
            }
            Action::AddCurve => {
                let mut curve = session_rust::NurbsCurve::create(false, 2, &[session_rust::Point::new(-0.6, -0.4, 0.0),
                    session_rust::Point::new(0.0, 0.6, 0.0), session_rust::Point::new(0.6, -0.4, 0.0)]);
                curve.width = 5.0; curve.linecolors = vec![session_rust::Color::new(0.0, 0.0, 0.0, 0.5)];
                let prepared = crate::chain::PreparedChain::curve(std::rc::Rc::new(curve))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_path(prepared).map(|_| ()))?;
            }
            Action::AddPoint => {
                let mut point = session_rust::Point::new(0.0, 0.0, 0.0);
                point.width = 12.0; point.pointcolor = session_rust::Color::new(0.0, 0.0, 0.0, 0.5);
                let prepared = crate::marker::PreparedPoint::new(std::rc::Rc::new(point))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_point(prepared).map(|_| ()))?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0; line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared = crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history.try_edit(&mut self.scene, |scene| scene.insert_line(prepared).map(|_| ()))?;
            }
            Action::AddBox => {
                self.history.try_edit(&mut self.scene, |scene| scene.add_box().map(|_| ()))?;
            }
            Action::Translate(offset) => return self.move_selection(offset),
            Action::ToggleExtra => self.history.edit(&mut self.scene, Scene::toggle_extra),
            Action::SelectNext => { let id = self.scene.next(self.selected); self.select_objects(id, false); }
            Action::Delete => return self.delete_selection(),
            Action::Undo => { self.history.undo(&mut self.scene); }
            Action::Redo => { self.history.redo(&mut self.scene); }
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings { grid.half()?; grid.reach(self.camera.target)?; }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::ZoomAt {factor,screen}=>self.camera.zoom_at(factor,screen),
                    Action::PanScreen {delta,size}=>self.camera.pan_screen(delta,size),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
                    Action::Isometric => self.camera.isometric(),
                    Action::Projection(mode) => self.camera.projection = mode,
                    Action::Fit => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(||self.display_scene().bounds()) {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.display_scene().bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
                    }
                    Action::ResetView => {
                        let aspect = self.camera.aspect;
                        self.camera = Camera { aspect, grid: self.camera.grid,frame:self.camera.frame, ..Camera::default() };
                    }
                    _ => unreachable!("Document actions were handled above"),
                }
                return Ok(Change::View);
            }
        }
        // Deletion and history travel can invalidate selection; repair it in one place.
        self.repair_selection();
        self.selected = self.selected.filter(|id| self.scene.contains(*id));
        if let (Some(owner), Some(part)) = (self.selected, self.selected_part) {
            if !(crate::pick_sources::Target { owner, part }).valid(&self.scene) { self.selected = None; }
        }
        if self.selected.is_none() { self.selected_part = None; }
        Ok(Change::Scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_modified_history_row_protects_only_its_own_import_origin() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let first = Rc::new(crate::reload_url::ReloadUrl::new("test:first".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), first)).unwrap();
        let origin = editor.scene.objects()[0].origin().unwrap().id;
        let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        let second = Rc::new(crate::reload_url::ReloadUrl::new("test:second".into()));
        editor.apply(Action::ImportAt(bytes, second)).unwrap();
        let copy = Rc::downgrade(editor.scene.objects()[3].geometry().unwrap());
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        editor.apply(Action::Undo).unwrap();
        let saved = editor.history.scenes_mut().last().unwrap();
        assert_eq!(saved.objects()[0].origin().unwrap().id, origin);
        Rc::make_mut(&mut saved.objects_mut()[0].metadata).is_locked = true;
        assert!(editor.scene.objects()[0].can_unload());
        editor.apply(Action::UnloadSources).unwrap();
        assert!(original.upgrade().is_some()); assert!(copy.upgrade().is_none());
        assert!(editor.scene.objects()[0].geometry().is_some());
        assert!(editor.scene.objects()[3].geometry().is_none());
        assert_eq!(editor.last_release, 1);
        assert!(editor.apply(Action::UnloadSources).is_err()); assert_eq!(editor.last_release, 1);
    }

    #[test]
    fn release_epochs_do_not_wrap_or_reset_on_close() {
        use std::rc::Rc;
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:epoch".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        editor.last_release = u64::MAX;
        assert_eq!(editor.apply(Action::UnloadSources), Err("Release epochs exhausted"));
        assert!(editor.scene.objects()[0].geometry().is_some());
        editor.last_release = 0; editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        assert_eq!(editor.scene.objects()[0].release_epoch(), Some(2));
    }

    #[test]
    fn close_ends_history_and_keeps_issued_ids_and_view_settings() {
        let mut editor = Editor::default();
        editor.apply(Action::Import(crate::specimen::bytes())).unwrap();
        editor.apply(Action::AddBox).unwrap(); editor.apply(Action::Undo).unwrap();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap(); editor.apply(Action::Background).unwrap();
        let view = editor.camera.uniform(); let background = editor.background.rgb();
        editor.apply(Action::Close).unwrap();
        assert!(editor.scene.objects().is_empty() && editor.selected.is_none());
        assert_eq!(editor.scenes().count(), 1);
        editor.apply(Action::Undo).unwrap(); editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert_eq!(editor.camera.uniform(), view); assert_eq!(editor.background.rgb(), background);
        editor.apply(Action::AddBox).unwrap();
        assert!(!old.contains(&editor.scene.objects()[0].id));
    }

    #[test]
    fn repeated_close_is_empty_and_drops_sources_without_gpu_owners() {
        let mut editor = Editor::default();
        let display = std::rc::Rc::downgrade(&editor.scene.objects()[0].mesh);
        let source = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Close).unwrap();
        assert!(display.upgrade().is_none() && source.upgrade().is_none());
        assert_eq!(crate::memory::cpu(editor.scenes(), std::iter::empty()), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn close_releases_imported_history_documents_and_reopening_uses_fresh_ids() {
        use std::rc::Rc;
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        let displays: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(&row.mesh))).collect();
        let sources: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .map(|row| Rc::downgrade(row.geometry().unwrap()))).collect();
        let documents: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter()
            .filter_map(|row| row.source().map(|source| Rc::downgrade(&source.document)))).collect();
        let old: Vec<_> = editor.scenes().flat_map(|scene| scene.objects().iter().map(|row| row.id)).collect();
        assert!(!documents.is_empty());
        editor.apply(Action::Close).unwrap();
        assert!(displays.iter().all(|owner| owner.upgrade().is_none()));
        assert!(sources.iter().all(|owner| owner.upgrade().is_none()));
        assert!(documents.iter().all(|owner| owner.upgrade().is_none()));
        editor.apply(Action::Import(bytes)).unwrap();
        assert!(editor.scene.objects().iter().all(|row| !old.contains(&row.id)));
    }

    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
        let mut editor = Editor::default();
        editor.camera.aspect = 2.0;
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::ResetView).unwrap();
        assert_eq!(editor.camera.aspect, 2.0);
        assert_eq!(editor.camera.distance, 3.0);
    }

    #[test]
    fn undo_changes_the_document_without_rewinding_the_view() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        editor.apply(Action::Zoom(2.0)).unwrap();
        editor.apply(Action::Background).unwrap();
        editor.apply(Action::Undo).unwrap();
        assert_eq!(editor.scene.objects().len(), 2);
        assert_eq!(editor.camera.distance, 1.5);
        assert_eq!(editor.background.rgb(), [0.9, 0.9, 0.9]);
    }

    #[test]
    fn undo_repairs_selection_and_empty_delete_preserves_redo() {
        let mut editor = Editor::default();
        editor.apply(Action::AddBox).unwrap();
        for _ in 0..3 {
            editor.apply(Action::SelectNext).unwrap();
        }
        let box_id = editor.selected.unwrap();
        editor.apply(Action::Undo).unwrap();
        assert!(editor.selected.is_none());
        editor.apply(Action::Delete).unwrap();
        editor.apply(Action::Redo).unwrap();
        assert!(editor.scene.contains(box_id));
    }

    #[test]
    fn picking_a_moved_view_uses_the_editors_current_camera() {
        let mut editor = Editor::default();
        editor.apply(Action::Pan(0.2, 0.1)).unwrap();
        let point = session_rust::Point::new(0.2, 0.3, 0.75);
        let screen = editor.camera.view_projection().transform_point(&point);
        assert_eq!(editor.apply(Action::Pick([screen[0] as f32, screen[1] as f32])).unwrap(), Change::Scene);
        assert_eq!(editor.selected, Some(editor.scene.objects()[1].id));
        assert_eq!(editor.apply(Action::Zoom(2.0)).unwrap(), Change::View);
    }
}
```

### 2. `src/commands.rs`

Create the original CAD bore through one checked document transaction.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/59f-cad-03.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/cad_display_tests.rs`

Copy original face/edge identity, precise shared placement, source Close and outward-normal regressions.

Copy this supplied file:

```text
--8<-- "journey/code/59f-cad-04.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the source component.

After that block, append this supplied code:

```text
--8<-- "journey/code/59f-cad-05.rs" skip=22
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/pick_gpu_tests.rs`

Copy actual empty-hole, original cap face and five visible border edge GPU readbacks with source/GPU reuse assertions.

After that block, append this supplied code:

```text
--8<-- "journey/code/59f-cad-06.rs" skip=77
```

**Result after `59f-cad`:** Type Close, Example CAD, Fit and View Isometric. Inspect the open hole, blue faces and black original borders. Select All, Move 0.123456789012345 0 0, then Undo once. Use Select Face and Select Edge to inspect original CAD identities; return to Select Object for whole-placement editing.

---

## 3. Build, Verify & Save

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 9.5
npm --prefix ../session_tests run course -- save 9.5
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../../recovery.md).

Continue with [10.1 · Explicit fonts and source label placement](../../consolidation/60-fonts.md).
