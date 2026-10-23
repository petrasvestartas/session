# 22.3 · Original boundary pens, joins and vertex markers

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Checkpoints:** `96zzzzzzn-respect-original-boundary-pens` → `96zzzzzzq-taper-crowded-original-mesh-vertices` (4 steps) · **Production `session_viewer/src` target:** [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs) · [engine/gpu/pass.rs](../../../../src/engine/gpu/pass.rs) · [engine/gpu/targets.rs](../../../../src/engine/gpu/targets.rs) · [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs) · [engine/gpu/view.rs](../../../../src/engine/gpu/view.rs) · [engine/pipelines/mod.rs](../../../../src/engine/pipelines/mod.rs) · [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl) · [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl) · [app/scene.rs](../../../../src/app/scene.rs) (`session_viewer/src/lib.rs`) · **Goal:** Keep dense source vertices distinguishable while retaining explicit point sizes, editing controls and original ownership.

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 22.3 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/mesh_pens.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/mesh_parts.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">96zzzzzzn-respect-original-boundary-pens -&gt; 96zzzzzzq-taper-crowded-original-mesh-vertices</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Respect original boundary…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Vertex RGB &amp; Uniforms</text>
  <polygon points="610,56 690,56 610,126" fill="#bfdbfe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="690,56 690,126 610,126" fill="#fde68a" stroke="#111111" stroke-width="1.75"/>
  <circle cx="610" cy="56" r="4" fill="#ef4444"/><text x="590" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v0</text>
  <circle cx="690" cy="56" r="4" fill="#22c55e"/><text x="696" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v1</text>
  <circle cx="610" cy="126" r="4" fill="#3b82f6"/><text x="590" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v2</text>
  <circle cx="690" cy="126" r="4" fill="#f59e0b"/><text x="696" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v3</text>
  <rect x="720" y="64" width="90" height="54" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="765" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">view_proj</text>
  <text x="765" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">mat4x4&lt;f32&gt;</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Original boundary pens, joi…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Depth32Float &amp; Ray Pick</text>
  <rect x="936" y="54" width="96" height="62" rx="4" fill="#93c5fd" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="892" y="74" width="96" height="62" rx="4" fill="#fdba74" stroke="#111111" stroke-width="2"/>
  <line x1="864" y1="138" x2="932" y2="102" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="936" cy="100" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <text x="896" y="68" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 &lt; z=0.7</text>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Taper crowded original me…</text>
</svg>
</div>

## <a id="checkpoint-96zzzzzzn-respect-original-boundary-pens"></a>Step 1 of 4 · Respect original boundary pens (`96zzzzzzn-respect-original-boundary-pens`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1/4: 96zzzzzzn-respect…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/mesh_pens.rs, src/lib.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original topology and sty…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing boundary stroke…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">actual source edge queries</text>
</svg>
</div>

A mesh with an authored seven-pixel pen currently looks identical to its one-pixel version. Prepare pens in the topology producer’s edge order, then feed the existing stroke lane. Use the same preparation to suppress coplanar seams while preserving source face identity.

A Mesh width of zero hides an edge; a single zero width denotes an undecorated printed fill. BRep and NURBS widths instead use the default pen for nonpositive values. Dense meshes above the maintained triangle budget also keep their faces without edge decoration. Store only numeric pens on the existing shared display so source unloading does not change appearance.

> **Check:** *Why must edge colors and widths use topology encounter order instead of a sorted edge list?* — The source stores edge style streams in the order seeded while walking faces and their rings. Sorting endpoint pairs changes that order and assigns styles to different original edges. Preserve actual endpoint keys for source queries.

### 1. `src/mesh_pens.rs`

Prepare only numeric pens using the already-taught topology and original encounter order.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-01.rs"
```

### 2. `src/lib.rs`

Connect the prepared style producer to the independent learner runtime.

After that block, type:

```rust
pub mod mesh_pens;
```

### 3. `src/mesh_parts.rs`

Keep lightweight pens beside the existing shared source-key display. Reuse one topology preparation for styling and coplanar edge omission. Retain pens after the editable source is unloaded. Decode packed original colors only when preparing the existing stroke stream. Count actual retained numeric pen capacity.

Edit the existing block to match this code; keep unchanged lines:

```rust
#[derive(Clone,Default)]
pub struct Parts {
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
    pub endpoints: Vec<[[f64; 3]; 2]>,
}

impl Parts {
    // Render triangles can split one source polygon; retain its original key rather than treating each display triangle as a new face.
    pub fn new(source: &session_rust::Mesh, count: usize) -> Result<Self, &'static str> {
        let faces = source.faces(); let mut triangles = Vec::new();
        for (slot, &face) in faces.iter().enumerate() {
            let slot = u32::try_from(slot).map_err(|_| "Too many source faces")?;
            let cached = source.get_triangulation().get(&face).filter(|tris| !tris.is_empty());
            let indices = if let Some(cached) = cached { cached.clone() } else {
                let vertices = source.face_vertices(face).ok_or("Missing source face")?;
                (1..vertices.len().saturating_sub(1)).map(|i| [vertices[0], vertices[i], vertices[i + 1]]).collect()
            };
            for tri in indices { if tri.iter().all(|&key| source.vertex_point(key).is_some()) { triangles.push(slot); } }
        }
        if triangles.len() != count { return Err("Source faces do not match the rendered triangles"); }
        let (edges,pens)=crate::mesh_pens::prepare(source,count); let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source.vertex_point(a).ok_or("Missing edge endpoint")?;
            let b = source.vertex_point(b).ok_or("Missing edge endpoint")?;
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self { pens,faces, triangles, edges, endpoints })
    }

    pub fn pen(&self,index:usize)->(f32,[f32;4]) {
        let &(width,colour)=self.pens.get(if self.pens.len()==1 {0} else {index}).unwrap_or(&(1.,crate::source_topology_colour::BLACK));
        (width,colour.to_le_bytes().map(|v|v as f32/255.))
    }

    pub fn bytes(&self) -> usize {
        self.pens.capacity()*std::mem::size_of::<crate::mesh_pens::Pen>()+self.faces.capacity() * std::mem::size_of::<usize>() + self.triangles.capacity() * 4
            + self.edges.capacity() * std::mem::size_of::<(usize, usize)>() + self.endpoints.capacity() * 48
    }
}
```

### 4. `src/bounds_original.rs`

Existing analytic bounds edges retain their default black pen.

After that block, type:

```rust
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-08.rs" skip=19
```

### 5. `src/pick_sources.rs`

Keep source endpoint keys, boundary addresses and pen slots aligned. Respect each original geometry’s width semantics before emitting visible ink. Preserve source edge colors unless the existing display override wins. Use the existing CSS-pixel stroke path rather than another render lane.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (&(a, b), ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: scene.view.edges.get(&row.id).map_or([0.,0.,0.,1.],|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]), width: 1. });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
```

Replace it with:

```rust
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
```

**Complete updated block reference:**

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize), SourceControl(crate::source_controls::ControlId), CloudPoint(u32), SheetEntity(u32) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode { #[default] Object, Face, Edge, Control }

#[derive(Default)]
pub struct Sources {
    pub sheet_bases:std::collections::HashMap<(ObjectId,usize),u32>,
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(rows: &mut Vec<Target>, kind: Kind, target: Target) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?; rows.push(target); Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene:&Scene)->Result<Self,&'static str> {Self::anchored(scene,[0.;3])}
    pub fn anchored(scene:&Scene,anchor:[f64;3])->Result<Self,&'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !scene.forest.derived(row.id) {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
            }
            let mut facing=crate::packed_vertex::NO_NORMAL;
            if row.mesh.alphas().first().is_some_and(|a|(*a-1./255.).abs()<0.5/255.) {
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
            let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part: Part::Curve })?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points_at(anchor).iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        for row in scene.sheets().iter().filter(|row|scene.visible(row.id)) {for page in row.prepared.pages() {sources.sheet_bases.insert((row.id,page.key()),u32::try_from(sources.edges.len()).map_err(|_|"Too many sheet pick addresses")?);for entity in &page.entities {append(&mut sources.edges,Kind::Edge,Target {owner:row.id,part:if *entity==u32::MAX {Part::Object} else {Part::SheetEntity(*entity)}})?;}}}
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }

    pub fn resolve(&self, scene: &Scene, pixel: [u32; 2], mode: Mode) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode==Mode::Control && kind==Kind::Control && (scene.clouds().iter().any(|r|r.id==owner)||scene.streams().iter().any(|r|r.id==owner)) {
            return crate::cloud_pick::resolve(scene,owner,address);
        }
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address), _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/source_pen_tests.rs`

Supply real GPU/source/archive, sparse style-order and source-unload controls.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-13.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/renderer.rs`

Connect Native validation outside the browser runtime.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-14.rs" skip=29
```

### 8. `src/plane_original.rs`

Existing analytic plane boundaries retain their default black pen.

Edit the existing block to match this code; keep unchanged lines:

```rust
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{pens:Vec::new(),faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
```

### 9. `src/pick_sources.rs`

Resolve immutable mesh-row ownership and explicit display color once. Share the existing source-face eligibility classification. Resolve the original surface/BRep scalar pen once per row, outside its boundary segments. Retain identical original pick addresses without repeated per-segment hierarchy lookups.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
```

Replace it with:

```rust
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push([owner,if derived{0}else{ids[1]}]);
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
```

**Locate this block:**

```rust
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
```

Replace it with:

```rust
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
```

**Locate this block:**

```rust
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !scene.forest.derived(row.id) {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
```

Replace it with:

```rust
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let derived=scene.forest.derived(row.id);
            let owner=crate::feature_owner::display_owner(scene,row.id).map_or(0,ObjectId::value);
            let edge_colour=scene.view.edges.get(&row.id).map(|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !derived {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
```

**Complete updated block reference:**

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize), SourceControl(crate::source_controls::ControlId), CloudPoint(u32), SheetEntity(u32) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode { #[default] Object, Face, Edge, Control }

#[derive(Default)]
pub struct Sources {
    pub sheet_bases:std::collections::HashMap<(ObjectId,usize),u32>,
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(rows: &mut Vec<Target>, kind: Kind, target: Target) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?; rows.push(target); Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene:&Scene)->Result<Self,&'static str> {Self::anchored(scene,[0.;3])}
    pub fn anchored(scene:&Scene,anchor:[f64;3])->Result<Self,&'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let derived=scene.forest.derived(row.id);
            let owner=crate::feature_owner::display_owner(scene,row.id).map_or(0,ObjectId::value);
            let edge_colour=scene.view.edges.get(&row.id).map(|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !derived {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
            }
            let mut facing=crate::packed_vertex::NO_NORMAL;
            if row.mesh.alphas().first().is_some_and(|a|(*a-1./255.).abs()<0.5/255.) {
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push([owner,if derived{0}else{ids[1]}]);
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
            let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part: Part::Curve })?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points_at(anchor).iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        for row in scene.sheets().iter().filter(|row|scene.visible(row.id)) {for page in row.prepared.pages() {sources.sheet_bases.insert((row.id,page.key()),u32::try_from(sources.edges.len()).map_err(|_|"Too many sheet pick addresses")?);for entity in &page.entities {append(&mut sources.edges,Kind::Edge,Target {owner:row.id,part:if *entity==u32::MAX {Part::Object} else {Part::SheetEntity(*entity)}})?;}}}
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }

    pub fn resolve(&self, scene: &Scene, pixel: [u32; 2], mode: Mode) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode==Mode::Control && kind==Kind::Control && (scene.clouds().iter().any(|r|r.id==owner)||scene.streams().iter().any(|r|r.id==owner)) {
            return crate::cloud_pick::resolve(scene,owner,address);
        }
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address), _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/mesh_arena_tests.rs`

Keep the supplied stale-binding GPU probe inside its actual bound arrays; preserve all pixel, identity and write-count assertions.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzn-respect-original-boundary-pens-20.rs"
```

**Result after `96zzzzzzn-respect-original-boundary-pens`:** The authored pen changes the actual GPU border. Sparse original keys and complete source records remain unchanged; coplanar seams and hidden mesh edges do not become visible ink.

## <a id="checkpoint-96zzzzzzo-close-original-boundary-joins"></a>Step 2 of 4 · Close original boundary joins (`96zzzzzzo-close-original-boundary-joins`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2/4: 96zzzzzzo-close-o…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_quad.wgsl, src/f…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original endpoints</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">distance coverage</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">matching integer query</text>
</svg>
</div>

A wide source border leaves a white gap outside each corner. Expand the existing stroke quad along its length, then measure fragment distance to the finite segment. Share this footprint with the existing query shaders.

Original endpoints receive round caps; a camera near-plane cut stays flat. A nonzero line viewed end-on uses its nearest reverse-depth endpoint. Identical source endpoints remain hidden. Both existing finite and legacy lanes consume the same helper without new geometry owners, buffers or metadata.

> **Check:** *Why must a near-plane cut remain flat even when original line endpoints are round?* — Clipping creates a camera boundary rather than another original endpoint. A round cap there would invent ink beyond the visible source segment and allow queries on that invented shape.

### 1. `src/stroke_quad.wgsl`

Share near-clipped round-stroke projection without new records or buffers.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-01.rs"
```

### 2. `src/finite_project_source.rs`

Expand the one shared projection helper in both consumers.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-02.rs"
```

### 3. `src/stroke.wgsl`

Use one shared stroke shape for ordinary and finite rendering. Distinguish genuine round ends from flat near-plane cuts. Carry only screen coordinates needed by existing fragment coverage. Replace duplicate projection with the shared clipped capsule.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4f,
    @location(11) @interpolate(linear) finite_pixel:vec2f,
}

fn geometry(start:vec3f,end:vec3f,colour:vec4f,width:f32,index:u32)->Output {
    let s=stroke_quad(view.transform*vec4f(start,1.),view.transform*vec4f(end,1.),width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,s.axis,s.pixel);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

### 4. `src/finite_stroke.wgsl`

Use one shared stroke shape for ordinary and finite rendering. Distinguish genuine round ends from flat near-plane cuts. Let the shared projection own hidden, near-clipped and end-on cases. Retain contact rise and reuse the shared screen-space shape.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
#include "contact_depth.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32, display:vec4<f32>, eye:vec4<f32> }
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    let s=stroke_quad(a,b,width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,flags,s.axis,s.depths,s.pixel,rise);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) metadata:InkRecord) -> Output { return geometry(start, end, colour, width, index,metadata); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}

#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

### 5. `src/stroke_coverage.wgsl`

Measure distance to the finite segment; keep near cuts flat.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-11.rs"
```

### 6. `src/stroke_pick.wgsl`

Keep color and integer queries on the same end policy. Carry the actual clipping-end policy into the cropped request. Reuse the exact color coverage in integer picking. Use full-viewport pixel coordinates for cropped coverage. Cropping changes clip position, not the original screen-space footprint. Pass the shared footprint to the ordinary pick fragment.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4f,
    @location(11) @interpolate(linear) finite_pixel:vec2f,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.flat_ends = source.flat_ends;
    output.half_width = source.half_width;
    output.finite_axis_value=source.finite_axis_value;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.flat_ends, input.half_width,input.finite_axis_value,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
}
```

### 7. `src/finite_stroke_pick.wgsl`

Keep color and integer queries on the same end policy. Carry the actual clipping-end policy into the cropped request. Reuse the exact color coverage in integer picking.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) metadata:InkRecord) -> PickOutput {
    let source = geometry(start, end, colour, width, index,metadata);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.flat_ends = source.flat_ends;
    output.half_width = source.half_width;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.rise=source.rise;output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.flat_ends, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel,input.rise);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.))<=0.){discard;}
    return input.id;
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/round_caps_tests.rs`

Copy actual GPU validation fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-21.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/renderer.rs`

Register Native validation only.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-22.rs" skip=21
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/stroke_gpu.rs`

Exercise the actual existing legacy pipeline in Native fixtures.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-23.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/cap_gpu_tests.rs`

Sample the unoccluded original back face, away from the now-visible end-on construction-plane marker. Keep the zero section-fill assertion.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-24.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/finite_visibility_tests.rs`

Retain exact clipped headed-chain comparison; distinguish genuine round source caps with exact body, flat-cut and bounded-cap pixel assertions.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzo-close-original-boundary-joins-25.rs"
```

**Result after `96zzzzzzo-close-original-boundary-joins`:** Original borders close at wide corners; end-on original lines remain visible and queryable. Near cuts stay flat, complete source records remain unchanged, and shared geometry survives placement and source unload.

## <a id="checkpoint-96zzzzzzp-draw-default-mesh-vertex-dots"></a>Step 3 of 4 · Draw default mesh vertex dots (`96zzzzzzp-draw-default-mesh-vertex-dots`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3/4: 96zzzzzzp-draw-de…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/mesh_pens.rs, src/mesh_…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original nonzero edge inc…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">anchored marker</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">existing point upload</text>
</svg>
</div>

The original mesh border has ink but its default vertex dots are missing. Reuse its prepared topology to reference eligible endpoints, then upload their markers through the existing point lane.

A nonzero authored edge makes its endpoint vertices eligible even when a coplanar seam is omitted from the ink. Printed and dense fills have no dots. Preserve exact authored point colors when requested; otherwise use the maintained dark default. These dots select their object and remain separate from active editing controls.

> **Check:** *Why retain an endpoint address rather than another original mesh or a second list of vertex positions?* — The existing shared display owner already retains precise endpoints after source unloading. A compact address reuses those positions and keeps transformed instances independent without introducing another geometry owner.

### 1. `src/mesh_pens.rs`

Prepare numeric endpoint references and only authored point colors on the existing display owner. Printed and dense fills have no default vertex decoration. Reuse the same source topology and sparse slot map once. A nonzero authored source edge makes its endpoints eligible, including a removed coplanar seam. Default dark color needs no per-vertex color allocation; common authored colors retain one entry.

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::source_mesh_topology::{SlotMap,mesh_topology};
use session_rust::Mesh;
pub type Pen=(f32,u32);

pub fn width(value:f64)->f32 {
    let value=value as f32;
    if value.is_finite()&&value>0. {value} else {1.}
}

pub fn prepare(source:&Mesh,count:usize)->(Vec<(usize,usize)>,Vec<Pen>,Vec<u32>,Vec<[f32;4]>) {
    if count>200_000||source.widths()==[0.] {return (Vec::new(),Vec::new(),Vec::new(),Vec::new());}
    let keys=source.vertices();
    let points=keys.iter().map(|key|{let p=&source.vertex[key];[p.x,p.y,p.z]}).collect::<Vec<_>>();
    let slots=SlotMap::new(&keys);
    let topology=mesh_topology(source,&keys,&points,&slots);
    let mut vertices=vec![u32::MAX;keys.len()];
    let mut pens=Vec::with_capacity(topology.edges.len());
    for (i,&(_,_,colour)) in topology.edges.iter().enumerate() {
        let widths=source.widths();
        let value=if widths.len()==1 {widths[0]} else {widths.get(i).copied().unwrap_or(1.)};
        if value!=0. {for(side,key)in [topology.edges[i].0,topology.edges[i].1].into_iter().enumerate(){let dot=&mut vertices[slots.slot(key)];if *dot==u32::MAX {*dot=(i*2+side)as u32;}}}
        let pair=topology.edge_faces[i].map(|face|topology.normals.get(face as usize).copied().flatten());
        let flat=match pair {
            [Some(a),Some(b)]=>a.iter().zip(b).map(|(a,b)|a*b).sum::<f64>()*if topology.opposed[i]{1.}else{-1.}>=1.-1e-9,
            _=>false,
        };
        pens.push((if value==0.||flat {0.} else {width(value)},if topology.edges.len()>=10_000 {crate::source_topology_colour::BLACK} else {colour}));
    }
    if pens.first().is_some_and(|first|pens.iter().all(|pen|pen==first)) {pens=vec![pens[0]];}
    let mut dots=Vec::new();let mut colours=Vec::new();let pc=source.get_pointcolors();
    let coloured=source.color_mode==session_rust::mesh::ColorMode::POINTCOLORS&&pc.len()==keys.len();
    for(slot,edge)in vertices.into_iter().enumerate(){if edge!=u32::MAX{dots.push(edge);if coloured{colours.push(pc[slot].to_f32());}}}
    if colours.first().is_some_and(|c|colours.iter().all(|v|v==c)){colours=vec![colours[0]];}
    (topology.edges.into_iter().map(|(a,b,_)|(a,b)).collect(),pens,dots,colours)
}
```

### 2. `src/mesh_parts.rs`

Retain four-byte references into existing f64 endpoints rather than duplicate positions or original meshes. Carry the prepared numerical decoration through source release. Keep the original shared display owner. Read authored point colors or the maintained default exactly, without byte quantization. Count actual retained numeric decoration capacity.

Edit the existing block to match this code; keep unchanged lines:

```rust
#[derive(Clone,Default)]
pub struct Parts {
    pub dots:Vec<u32>,
    pub dot_colours:Vec<[f32;4]>,
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
    pub endpoints: Vec<[[f64; 3]; 2]>,
}

impl Parts {
    // Render triangles can split one source polygon; retain its original key rather than treating each display triangle as a new face.
    pub fn new(source: &session_rust::Mesh, count: usize) -> Result<Self, &'static str> {
        let faces = source.faces(); let mut triangles = Vec::new();
        for (slot, &face) in faces.iter().enumerate() {
            let slot = u32::try_from(slot).map_err(|_| "Too many source faces")?;
            let cached = source.get_triangulation().get(&face).filter(|tris| !tris.is_empty());
            let indices = if let Some(cached) = cached { cached.clone() } else {
                let vertices = source.face_vertices(face).ok_or("Missing source face")?;
                (1..vertices.len().saturating_sub(1)).map(|i| [vertices[0], vertices[i], vertices[i + 1]]).collect()
            };
            for tri in indices { if tri.iter().all(|&key| source.vertex_point(key).is_some()) { triangles.push(slot); } }
        }
        if triangles.len() != count { return Err("Source faces do not match the rendered triangles"); }
        let (edges,pens,dots,dot_colours)=crate::mesh_pens::prepare(source,count); let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source.vertex_point(a).ok_or("Missing edge endpoint")?;
            let b = source.vertex_point(b).ok_or("Missing edge endpoint")?;
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self { dots,dot_colours,pens,faces, triangles, edges, endpoints })
    }

    pub fn pen(&self,index:usize)->(f32,[f32;4]) {
        let &(width,colour)=self.pens.get(if self.pens.len()==1 {0} else {index}).unwrap_or(&(1.,crate::source_topology_colour::BLACK));
        (width,colour.to_le_bytes().map(|v|v as f32/255.))
    }

    pub fn dot_colour(&self,index:usize)->[f32;4]{
        self.dot_colours.get(if self.dot_colours.len()==1{0}else{index}).copied().unwrap_or([0.1,0.1,0.1,1.])
    }

    pub fn bytes(&self) -> usize {
        self.dots.capacity()*4+self.dot_colours.capacity()*16+self.pens.capacity()*std::mem::size_of::<crate::mesh_pens::Pen>()+self.faces.capacity() * std::mem::size_of::<usize>() + self.triangles.capacity() * 4
            + self.edges.capacity() * std::mem::size_of::<(usize, usize)>() + self.endpoints.capacity() * 48
    }
}
```

### 3. `src/bounds_original.rs`

Bounding frames do not invent mesh vertex markers.

Edit the existing block to match this code; keep unchanged lines:

```rust
        dots:Vec::new(),dot_colours:Vec::new(),pens:Vec::new(),
```

### 4. `src/plane_original.rs`

Original planes retain their own frame glyph policy.

Edit the existing block to match this code; keep unchanged lines:

```rust
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{dots:Vec::new(),dot_colours:Vec::new(),pens:Vec::new(),faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
```

### 5. `src/pick_sources.rs`

Build ordinary point instances with object ownership and no new editable component domain. Transform numerical endpoints after origin anchoring; BRep, NURBS and derived contact shapes keep their own display policy. Account for temporary marker preparation; move it into the existing upload.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
```

Replace it with:

```rust
    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_markers.capacity()*std::mem::size_of::<crate::marker::Marker>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
```

**Locate this block:**

```rust
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
```

Replace it with:

```rust
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            if row.cad.is_none()&&row.surface.is_none()&&!derived {
                for(i,&address)in row.mesh.parts.dots.iter().enumerate(){
                    let p=row.mesh.parts.endpoints[address as usize/2][address as usize%2];
                    sources.mesh_markers.push(crate::marker::Marker{center:model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),diameter:1.,colour:edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i))});
                    sources.marker_ids.push([owner,0]);
                }
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
```

**Locate this block:**

```rust
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}
```

Replace it with:

```rust
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub mesh_markers:Vec<crate::marker::Marker>,
    pub marker_ids: Vec<[u32; 2]>,
}
```

**Complete updated block reference:**

```rust
use crate::{scene::{ObjectId, Scene}, sub_id::{self, Kind}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part { Object, Face(usize), SurfaceEdge(usize), BRepEdge(usize), MeshEdge(usize, usize), PathEdge(usize), Curve, Control(usize), SourceControl(crate::source_controls::ControlId), CloudPoint(u32), SheetEntity(u32) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target { pub owner: ObjectId, pub part: Part }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum Mode { #[default] Object, Face, Edge, Control }

#[derive(Default)]
pub struct Sources {
    pub sheet_bases:std::collections::HashMap<(ObjectId,usize),u32>,
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub mesh_markers:Vec<crate::marker::Marker>,
    pub marker_ids: Vec<[u32; 2]>,
}

fn append(rows: &mut Vec<Target>, kind: Kind, target: Target) -> Result<[u32; 2], &'static str> {
    let tag = sub_id::encode(kind, rows.len())?; rows.push(target); Ok([target.owner.value(), tag])
}

impl Sources {
    // A source table translates compact GPU addresses to original keys; addresses belong to this scene snapshot, not the document's saved identity.
    pub fn new(scene:&Scene)->Result<Self,&'static str> {Self::anchored(scene,[0.;3])}
    pub fn anchored(scene:&Scene,anchor:[f64;3])->Result<Self,&'static str> {
        let mut sources = Self::default();
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
        let planes=crate::clipping_rows::owners(scene);
        for row in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let derived=scene.forest.derived(row.id);
            let owner=crate::feature_owner::display_owner(scene,row.id).map_or(0,ObjectId::value);
            let edge_colour=scene.view.edges.get(&row.id).map(|rgb|[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);
            let model=crate::render_space::placed(&row.model,anchor);
            sources.mesh_bases.push((row.id, u32::try_from(sources.faces.len()).map_err(|_| "Too many source faces")?));
            if !derived {
                for &key in &row.mesh.parts.faces {
                    append(&mut sources.faces,Kind::Face,Target{owner:row.id,part:Part::Face(key)})?;
                }
            }
            let mut facing=crate::packed_vertex::NO_NORMAL;
            if row.mesh.alphas().first().is_some_and(|a|(*a-1./255.).abs()<0.5/255.) {
                let n=row.mesh.normals()[0];let m=crate::normals::matrix(&row.model)?;
                facing=crate::packed_vertex::oct32(std::array::from_fn(|i|(0..3).map(|j|m[j*4+i]*n[j]).sum()));
            }
            if row.cad.is_none()&&row.surface.is_none()&&!derived {
                for(i,&address)in row.mesh.parts.dots.iter().enumerate(){
                    let p=row.mesh.parts.endpoints[address as usize/2][address as usize%2];
                    sources.mesh_markers.push(crate::marker::Marker{center:model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),diameter:1.,colour:edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i))});
                    sources.marker_ids.push([owner,0]);
                }
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push([owner,if derived{0}else{ids[1]}]);
                sources.mesh_metadata.extend([flag,facing]);
            }
        }
        for row in scene.lines().iter().filter(|row| scene.visible(row.id)) {
            let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part: Part::Curve })?;
            sources.line_ids.push(id(row.id, ids[1]));
        }
        for row in scene.paths().iter().filter(|row| scene.visible(row.id)) {
            let points:Vec<_>=row.points_at(anchor).iter().map(session_rust::Point::to_f32).collect();
            for (index, _) in points.windows(2).enumerate().filter(|(_, pair)| pair[0] != pair[1]) {
                let part = if matches!(row.prepared.source, crate::chain::Source::Polyline(_)) { Part::PathEdge(index) } else { Part::Curve };
                let ids = append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?;
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, 1)); }
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        for row in scene.sheets().iter().filter(|row|scene.visible(row.id)) {for page in row.prepared.pages() {sources.sheet_bases.insert((row.id,page.key()),u32::try_from(sources.edges.len()).map_err(|_|"Too many sheet pick addresses")?);for entity in &page.entities {append(&mut sources.edges,Kind::Edge,Target {owner:row.id,part:if *entity==u32::MAX {Part::Object} else {Part::SheetEntity(*entity)}})?;}}}
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_markers.capacity()*std::mem::size_of::<crate::marker::Marker>()
            + self.mesh_metadata.capacity()*4
            + self.mesh_ink.capacity() * std::mem::size_of::<crate::stroke::Stroke>()
            + (self.mesh_ids.capacity() + self.line_ids.capacity() + self.path_ids.capacity() + self.marker_ids.capacity()) * 8
    }

    pub fn resolve(&self, scene: &Scene, pixel: [u32; 2], mode: Mode) -> Option<Target> {
        let owner = scene.pick_id(pixel[0])?;
        if mode == Mode::Object { return Some(Target { owner, part: Part::Object }); }
        let (kind, address) = sub_id::decode(pixel[1])?;
        if mode==Mode::Control && kind==Kind::Control && (scene.clouds().iter().any(|r|r.id==owner)||scene.streams().iter().any(|r|r.id==owner)) {
            return crate::cloud_pick::resolve(scene,owner,address);
        }
        let target = match (mode, kind) {
            (Mode::Face, Kind::Face) => self.faces.get(address),
            (Mode::Edge, Kind::Edge) => self.edges.get(address),
            (Mode::Control, Kind::Control) => self.controls.get(address), _ => None,
        }?;
        (target.owner == owner && target.valid(scene)).then_some(*target)
    }
}
```

### 6. `src/renderer.rs`

Upload through the existing point buffer once, releasing temporary CPU instances. Match default mesh, original point and active control ownership order.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker}).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
```

Replace it with:

```rust
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, std::mem::take(&mut self.sources.mesh_markers).into_iter().chain(scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker})).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
```

**Complete updated block reference:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    surface_options:std::cell::Cell<[bool;2]>,
    world_bounds:std::cell::RefCell<Option<crate::bounds::Bounds>>,
    space:crate::render_space::Origin,
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
    grid_plane:crate::cplane::CPlane,grid_anchor:[f64;3],
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode:crate::pick_sources::Mode,
    pick_cursor:Option<([f32;2],[u64;3])>,
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
        let view_layout = crate::surface_view::layout(&device);
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
            size: crate::surface_view::BYTES,
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
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
        for lane in [&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format,false);}
        renderer.highlight.enable_finite(&renderer.device,format);
        renderer.set_scene(scene, None);
        renderer
    }

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

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
        if self.size != [size.width, size.height] { self.cancel_pick(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
        self.primitive=crate::primitive_image::Image::new(&self.device,self.size);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.cancel_pick(); self.density = density; }
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
        let anchor=self.anchor();
        self.ambient_rows.set(scene);if let Some(ambient)=self.ambient.borrow_mut().as_mut(){if self.ambient_enabled.get(){ambient.sources.set(&self.device,&self.queue,&self.ambient_rows.specs);}}
        self.pyramid_revision=self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
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
                GpuMesh::with_geometry_at(&self.device,&layout,object,
                    style(scene,object.id,selected==Some(object.id)),geometry,anchor)
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
        self.sheets.set_anchored(&self.device,&self.queue,scene,selected.as_slice(),&self.sources.sheet_bases,anchor).expect("Prepared original sheet batches fit device limits");
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        if let Err(error)=self.set_clouds(scene,selected.as_slice()) {*self.cloud_error.borrow_mut()=Some(error.to_string());}
        if let Err(error)=self.set_scene_text(scene,selected.as_slice()) {*self.text_error.borrow_mut()=Some(error.to_string());}
        *self.world_bounds.borrow_mut()=scene.bounds();
    }

    pub fn grow_camera(&self,camera:&mut crate::camera::Camera) {
        if let Some(bounds)=self.world_bounds.borrow().as_ref() {camera.grow_extent(bounds);}
    }

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
        let anchor=self.anchor();
        self.sources = crate::pick_sources::Sources::anchored(scene,anchor).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.mesh_ink.set_records(&self.device,std::mem::take(&mut self.sources.mesh_metadata)).expect("Original ink metadata");
        self.markers.set_points(&self.device, std::mem::take(&mut self.sources.mesh_markers).into_iter().chain(scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {marker.colour=colour;}marker})).chain(scene.control_markers_at(anchor).into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {stroke.colour=colour;}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        let origin=self.anchor();self.picker.queue_anchored(crate::pick_gpu::Request{window,transform,density:self.density,mode:self.pick_mode},origin)?;
        self.pick_cursor=Some((screen,origin.map(f64::to_bits)));self.pump_pick(transform);Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) else { self.cancel_pick(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases,&self.clipping.borrow().buffer);
        }
        if let Some(lane)=self.text.borrow_mut().as_mut() {if let Err(error)=lane.prepare(&self.text_context(),&self.text_frame(transform)) {*self.text_error.borrow_mut()=Some(error.to_string());return;}}
        self.clipping.borrow_mut().frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut clipping=self.clipping.borrow_mut();clipping.frame_anchored(&self.queue,transform,self.anchor(),self.size[1],self.density);
            let bytes=crate::surface_view::bytes(transform,[0.;4],self.size,self.density);self.queue.write_buffer(&self.uniform,0,&bytes);
            let tables={let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);let mut encoder=self.device.create_command_encoder(&Default::default());tiles.encode(&self.device,&mut encoder,&self.view_group,transform);self.queue.submit([encoder.finish()]);tiles.map_report();crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()}};
            self.picker.submit_finite(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces,self.text.borrow().as_ref(),self.clouds.borrow().as_ref(),Some(&self.sheets),Some(&tables),&clipping.group,&clipping.buffer,Some(&mut self.caps.borrow_mut()),Some(&self.instances)); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::anchored(transform,self.size,self.density,self.anchor()) { Ok(view) => view, Err(error) => {
            self.cancel_pick(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit)))=>{self.pick_cursor=None;Some(Ok(hit))},
            Some(Ok(crate::pick_gpu::Reply::Stale))=>{
                if let Some((screen,_))=self.pick_cursor.filter(|(_,origin)|*origin!=self.anchor().map(f64::to_bits)) {
                    if let Err(error)=self.request_pick(screen,transform){self.cancel_pick();return Some(Err(error));}
                } else {
                    self.pump_pick(transform);
                    if self.picker.usage()[0]==0&&!self.picker.queued(){self.pick_cursor=None;}
                }
                None
            },
            Some(Err(error))=>{self.cancel_pick();Some(Err(error))},None=>None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.cancel_pick(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) {self.pick_cursor=None;self.picker.cancel();}

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
        let anchor=self.anchor();
        if settings==self.grid_settings&&plane==self.grid_plane&&anchor==self.grid_anchor{return Ok(())}
        let strokes=settings.map(|s|s.strokes_on_at(plane,anchor)).transpose()?.unwrap_or_default();
        self.grid.set(&self.device,strokes)?;self.grid_settings=settings;self.grid_plane=plane;self.grid_anchor=anchor;Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.pipeline.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        let anchor=self.anchor();
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        self.instances.set(&self.device,&self.queue,&mut self.meshes,&self.instance_rows,false)?;
        self.tiles.borrow_mut().set_records(&self.device,&self.meshes,Some(&self.instances));
        if !self.meshes.iter().any(GpuMesh::selected) {self.outline.borrow_mut().take();}
        self.set_clouds(scene,ids)?;self.sheets.set_anchored(&self.device,&self.queue,scene,ids,&self.sources.sheet_bases,anchor)?;
        if !ids.is_empty() { self.highlight.objects_at(&self.device,scene,ids,anchor)?; }self.set_scene_text(scene,ids).map_err(|_|"Cannot prepare scene labels")?; Ok(())
    }

    pub fn set_guides(&mut self,overlay:&crate::guide_gpu::Overlay)->Result<(),&'static str>{if self.guides.set(&self.device,overlay)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare editing guide label")?}Ok(())}
    pub fn guide_bytes(&self)->u64{self.guides.bytes()}
    pub fn set_measurement(&mut self,mark:Option<&crate::measure_mark::Mark>)->Result<(),&'static str>{let anchor=self.anchor();if self.measurement.set_at(&self.device,mark,anchor)?{self.set_scene_measure_text().map_err(|_|"Cannot prepare measurement annotation")?;}Ok(())}
    pub fn measurement_bytes(&self)->u64{self.measurement.bytes()}
    fn merge_measurement(&self,mut labels:Vec<crate::text::TextLabel>)->Vec<crate::text::TextLabel>{if let Some(mark)=&self.measurement.mark{labels.extend(mark.labels(&labels));}labels.extend(self.guides.overlay.labels(&labels));labels}
    fn set_scene_measure_text(&self)->anyhow::Result<()>{let mut labels=self.scene_text.borrow().clone();labels.extend(self.presentation_text.borrow().clone());self.adopt_text(self.merge_measurement(labels))}
    pub fn set_preview(&mut self,draft:Option<&crate::tool::Draft>)->Result<(),&'static str>{let anchor=self.anchor();self.preview.set_at(&self.device,draft,anchor)}
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let anchor=self.anchor();let matrix=camera.view_projection_anchored(anchor);
        let layout=origin.map(|p|std::array::from_fn(|i|p[i]-anchor[i])).and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    fn text_context(&self)->crate::text_gpu_support::GpuCtx {crate::text_gpu_support::GpuCtx::new(&self.device,&self.queue)}
    fn text_frame(&self,transform:[f32;16])->crate::scene_text_gpu::TextFrame {crate::scene_text_gpu::TextFrame {mvp:transform,origin:self.anchor(),framebuffer:self.size,logical:[self.size[0] as f64/self.density as f64,self.size[1] as f64/self.density as f64],ortho_half_height:0.,clip:{let clip=self.clipping.borrow();let mut planes=[[0.;4];6];if clip.enabled {for (p,row) in planes.iter_mut().zip(&clip.rows){*p=[row.plane.normal[0],row.plane.normal[1],row.plane.normal[2],row.plane.offset];}}planes}}}
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
    pub fn refresh_text_bounds(&self,scene:&mut Scene)->bool {
        let mut changed=false;
        if let Some(lane)=self.text.borrow().as_ref() {for row in scene.texts_mut() {if let Some(run)=lane.document.runs.iter().find(|run|run.label.id==row.id.value()) {
            let extent=run.buffer.layout_runs().fold([0f64;2],|[w,h],line|[w.max(line.line_w as f64),h.max((line.line_top+line.line_height) as f64)]);
            if row.prepared.extent!=extent {row.prepared.extent=extent;changed=true;}
        }}}
        if changed {*self.world_bounds.borrow_mut()=scene.bounds();}
        changed
    }
    pub fn text_stats(&self)->Option<crate::scene_text_gpu::TextStats> {self.text.borrow().as_ref().map(|lane|lane.stats.clone())}
    pub fn text_scene_usage(&self)->[u64;4] {self.text.borrow().as_ref().map_or([0;4],|lane|[lane.allocated_bytes(),lane.texture_bytes(),lane.stats.world_plane_rasterizations,lane.document.runs.len() as u64])}
    pub fn replace_text_fonts(&self,fonts:Vec<Vec<u8>>)->anyhow::Result<()> {if let Some(lane)=self.text.borrow_mut().as_mut() {lane.document.replace_fonts(fonts.clone())?;} else {let mut document=crate::text::TextDocument::new();document.replace_fonts(fonts.clone())?;}*self.full_text_fonts.borrow_mut()=Some(fonts);Ok(())}
    pub fn clear_text_fonts(&self) {self.full_text_fonts.borrow_mut().take();}

    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{[lane.document.shape_count,lane.stats.preparations,lane.stats.skipped_preparations,lane.stats.raster_images as u64,lane.stats.raster_image_capacity_bytes as u64,lane.stats.atlas_resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn cloud_camera(&self,camera:&crate::camera::Camera) {self.cloud_view.set(crate::cloud_gpu::View::anchored(camera,self.anchor()));self.sheets.camera(camera);}
    pub fn cloud_usage(&self)->[u64;5] {self.clouds.borrow().as_ref().map_or([0;5],|lane|lane.usage())}
    pub fn cloud_error(&self)->Option<String> {self.cloud_error.borrow().clone()}
    fn set_clouds(&self,scene:&Scene,selected:&[ObjectId])->Result<(),&'static str> {
        if scene.clouds().is_empty()&&scene.streams().is_empty() {self.clouds.borrow_mut().take();self.cloud_error.borrow_mut().take();return Ok(());}
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
    }
    fn prepare_clouds(&self,transform:[f32;16]) {
        if let Some(lane)=self.clouds.borrow_mut().as_mut() {lane.set_clipping(self.clipping.borrow().writes);if let Err(error)=lane.prepare(&self.text_context(),transform,self.size,self.density,self.cloud_view.get()) {lane.failed();*self.cloud_error.borrow_mut()=Some(error.to_string());} else {self.cloud_error.borrow_mut().take();}}
    }
    pub fn set_opacity(&mut self,value:f32){if !value.is_finite(){return;}let value=value.clamp(0.,1.);if self.opacity.get()==value{return;}self.opacity.set(value);self.cancel_pick();self.pyramid_revision=self.pyramid_revision.wrapping_add(1);for mesh in &mut self.meshes{mesh.set_xray(&self.queue,value==0.);}if let Some(o)=self.outline.borrow_mut().as_mut(){o.invalidate();}if let Some(o)=self.global_outline.borrow_mut().as_mut(){o.invalidate();}}
    pub fn opacity(&self)->f32{self.opacity.get()}
    pub fn set_outlines(&self,on:bool){self.outlines_enabled.set(on);if !on{self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();}}
    pub fn global_outline_usage(&self)->[u64;4]{self.global_outline.borrow().as_ref().map_or([0;4],|o|o.usage())}
    pub fn outline_usage(&self)->[u64;4] {self.outline.borrow().as_ref().map_or([0;4],|outline|outline.usage())}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn sheet_camera(&self,camera:&crate::camera::Camera) {self.sheets.camera(camera);}
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self)->[wgpu::Buffer;2]{let tiles=self.tiles.borrow();[tiles.projected.clone(),tiles.buffer.clone()]}
    pub fn set_clipping(&self,enabled:bool,solid:bool){let mut clip=self.clipping.borrow_mut();clip.enabled=enabled;clip.solid=solid;self.caps.borrow_mut().enabled=enabled;}
    pub fn set_lighting(&self,on:bool,backfaces:bool){self.surface_options.set([on,backfaces]);}
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
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
        let options=self.surface_options.get();
        let display=[f32::from(normals),self.opacity.get(),if self.ambient_enabled.get(){2.}else{f32::from(options[0])},f32::from(options[1])];
        let bytes=crate::surface_view::bytes(*transform,display,self.size,self.density);
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.sheets.view(&self.queue,transform,self.size,self.density);
        let [r, g, b] = if self.ambient_enabled.get() { [0.94; 3] } else { background.rgb() };
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut tiles=self.tiles.borrow_mut();tiles.prepare(&self.device,&self.queue,self.size);tiles.encode(&self.device,&mut encoder,&self.view_group,*transform);
        let tables=crate::finite_ink::Tables{projected:tiles.projected.clone(),tiles:tiles.buffer.clone(),primitive:self.primitive.view.clone()};for lane in [&self.mesh_ink,&self.strokes,&self.paths,&self.markers]{lane.finite_frame(&self.device,&tables,&self.depth);}self.highlight.finite_frame(&self.device,&tables,&self.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
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
            self.grid.draw(&mut pass,&self.view_group);
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
            self.highlight.ink(&mut pass,&self.view_group);
            self.mesh_ink.draw(&mut pass,&self.view_group);
            self.strokes.draw(&mut pass,&self.view_group);self.sheets.draw(&mut pass);
            self.paths.draw(&mut pass,&self.view_group);
            self.markers.draw(&mut pass,&self.view_group);
            self.highlight.points(&mut pass,&self.view_group);
            self.preview.draw(&mut pass,&self.view_group);
        }
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
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
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.depth,&self.view_group,&clip_group,&clip.buffer,&self.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {source_faces:!scene.forest.derived(id),selected,owner:crate::feature_owner::display_owner(scene,id).map_or(0,ObjectId::value),colour:scene.face_rgba(id)}}

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

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="camera_extent_tests.rs"]
mod camera_extent_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_lighting_tests.rs"]
mod surface_lighting_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="surface_controls_gpu_tests.rs"]
mod surface_controls_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="shared_view_tests.rs"]
mod shared_view_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="contact_feature_tests.rs"]
mod contact_feature_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_metadata_tests.rs"]
mod ink_metadata_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="contact_boundary_tests.rs"]
mod contact_boundary_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="source_face_policy_tests.rs"]
mod source_face_policy_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="source_pen_tests.rs"]
mod source_pen_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="round_caps_tests.rs"]
mod round_caps_tests;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/mesh_marker_tests.rs`

Exercise actual GPU dots, source keys, authored colors, incidence and shared source unloading.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-18.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/renderer.rs`

Register the three actual mesh marker GPU checks.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-19.rs" skip=21
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/source_pen_tests.rs`

Extend the existing real 200000-triangle budget case without duplicating its dense fixture.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-20.rs" skip=55
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/global_outline_tests.rs`

This existing fixture manually isolates surface-buffer rebasing; suppress both original ink and new default dots while preserving every surface/grid/cap/pick/source assertion. Production complete-lane marker rebasing is separately exercised at a billion units.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-21.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/global_outline_tests.rs`

This existing fixture manually isolates surface-buffer rebasing; suppress both original ink and new default dots while preserving every surface/grid/cap/pick/source assertion. Production complete-lane marker rebasing is separately exercised at a billion units.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-22.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/global_outline_tests.rs`

This existing fixture manually isolates surface-buffer rebasing; suppress both original ink and new default dots while preserving every surface/grid/cap/pick/source assertion. Production complete-lane marker rebasing is separately exercised at a billion units.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-23.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/scene_origin_tests.rs`

Keep the original point/control count exact after the new eight default Box vertex markers; verify all marker values, owners, ordering and large-origin equality independently.

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzp-draw-default-mesh-vertex-dots-24.rs"
```

**Result after `96zzzzzzp-draw-default-mesh-vertex-dots`:** Eligible original mesh vertices have their dark default or authored dot colors, original source queries remain correct, and shared placed geometry survives unloading and restoration.

## <a id="checkpoint-96zzzzzzq-taper-crowded-original-mesh-vertices"></a>Step 4 of 4 · Taper crowded original mesh vertices (`96zzzzzzq-taper-crowded-original-mesh-vertices`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4/4: 96zzzzzzq-taper-c…</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/mesh.rs, src/marker.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Render bounds and source…</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">signed GPU size</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">common radius and query c…</text>
</svg>
</div>

Dense original mesh dots overlap until the blue face becomes an opaque dark sheet. Estimate spacing once, then taper each default dot at its projected depth while keeping a half-physical-pixel minimum.

Reuse the existing 32-byte point layout. Positive GPU sizes retain explicit CSS diameters; negative values carry prepared mesh spacing in millimetres. A named constructor makes this render-only mode explicit, and original source widths remain unchanged. One helper serves both existing finite and legacy color/query paths. The original mesh owner retains one scalar after source release; no per-vertex distance array, extra buffer or neighbor-normal table is added.

> **Check:** *Why apply the half-physical-pixel minimum after the crowding taper?* — Crowded default dots must become smaller before the lower bound is applied. This preserves a visible footprint without keeping an unnecessarily large radius at a higher pixel density.

### 1. `src/mesh.rs`

Retain one prepared mesh spacing scalar after original source release. Only actual original default-dot meshes need a spacing estimate. Use actual render bounds and original vertex count once, matching the maintained per-mesh estimate; skip dense/printed fills without dots.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self { vertices, normals, alphas, indices, parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        display.parts = parts; Ok(display)
    }
```

Replace it with:

```rust
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self { vertices, normals, alphas, indices, spacing:0.,parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        if !parts.dots.is_empty(){
            let mut b=session_rust::AABB::empty();for p in display.vertices(){b.union_with_point(p[0]as f64,p[1]as f64,p[2]as f64);}
            display.spacing=(b.diagonal().min(f32::MAX as f64)as f32)/(mesh.number_of_vertices()as f32).sqrt();
        }
        display.parts = parts; Ok(display)
    }
```

**Locate this block:**

```rust
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}
```

Replace it with:

```rust
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub spacing:f32,
    pub parts: crate::mesh_parts::Parts,
}
```

**Complete updated block reference:**

```rust
#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<[f32; 6]>,
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub spacing:f32,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(vertices: Vec<[f32; 6]>, indices: Vec<u32>) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err("Indices must describe complete triangles within the draw limit");
        }
        if indices.iter().any(|&index| index as usize >= vertices.len()) {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices.iter().flatten().any(|value| !value.is_finite()) {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self { vertices, normals, alphas, indices, spacing:0.,parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        if !parts.dots.is_empty(){
            let mut b=session_rust::AABB::empty();for p in display.vertices(){b.union_with_point(p[0]as f64,p[1]as f64,p[2]as f64);}
            display.spacing=(b.diagonal().min(f32::MAX as f64)as f32)/(mesh.number_of_vertices()as f32).sqrt();
        }
        display.parts = parts; Ok(display)
    }

    pub fn from_render(render:session_rust::render_mesh::RenderMesh)->Result<Self,&'static str>{
        let vertices = render.vertices.iter().map(|vertex| {
            let [x, y, z] = vertex.position;
            let [r, g, b, _] = vertex.color;
            [x, y, z, r, g, b]
        }).collect();
        let normals = render.vertices.iter().map(|vertex| vertex.normal).collect();
        let alphas = render.vertices.iter().map(|vertex| vertex.color[3]).collect();
        Self::new(vertices, render.indices)?.with_normals(normals)?.with_alphas(alphas)
    }

    pub fn with_normals(mut self, normals: Vec<[f32; 3]>) -> Result<Self, &'static str> {
        if normals.len() != self.vertices.len() { return Err("Each display vertex needs one normal"); }
        self.normals = normals.into_iter().map(crate::normals::unit).collect::<Result<_, _>>()?; Ok(self)
    }

    pub fn with_alphas(mut self, alphas:Vec<f32>)->Result<Self,&'static str>{
        if alphas.len()!=self.vertices.len()||alphas.iter().any(|v|!v.is_finite()){
            return Err("Each display vertex needs finite alpha");
        }
        self.alphas=alphas;Ok(self)
    }
    pub fn alphas(&self)->&[f32]{&self.alphas}
    pub fn normals(&self) -> &[[f32; 3]] { &self.normals }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
            + self.alphas.capacity() * std::mem::size_of::<f32>() + self.parts.bytes()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kernel_box_becomes_eight_vertices_and_twelve_triangles() {
        let source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        let mesh = Mesh::from_kernel(&source).unwrap();
        assert_eq!(mesh.vertices().len(), 8);
        assert_eq!(mesh.indices().len(), 36);
    }

    #[test]
    fn invalid_connections_and_nonfinite_vertices_are_rejected() {
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 3]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1]).is_err());
        assert!(Mesh::new(vec![[f32::NAN; 6]; 3], vec![0, 1, 2]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2]).is_ok());
    }
}
```

### 2. `src/marker.rs`

Name the render-only size mode explicitly; original point widths and editing-control diameters remain unchanged.

After that block, type:

```rust
    // The signed GPU size is a CSS diameter, or negative mesh spacing in mm.
    pub fn mesh_dot(center:[f32;3],colour:[f32;4],spacing:f32)->Self{
        Self{center,colour,diameter:if spacing>0.{-spacing}else{1.}}
    }
```

### 3. `src/pick_sources.rs`

Reuse the existing32byte point instance, endpoint addresses and upload; no distance array, extra metadata or buffer.

Edit the existing block to match this code; keep unchanged lines:

```rust
                    sources.mesh_markers.push(crate::marker::Marker::mesh_dot(model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i)),row.mesh.spacing));
```

### 4. `src/stroke_gpu.rs`

Accept the explicit finite mesh-spacing size mode while rejecting zero and nonfinite display data.

Edit the existing block to match this code; keep unchanged lines:

```rust
            if marker.diameter == 0.0 || marker.values().iter().any(|v| !v.is_finite()) { return Err("Invalid marker display data"); }
```

### 5. `src/marker_radius.wgsl`

Project one instance spacing at each vertex depth; retain the actual three-diameter taper and half-physical-pixel minimum.

Create the file and type:

```wgsl
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-07.rs"
```

### 6. `src/finite_project_source.rs`

Expand one shared radius helper for both color/pick and existing finite/legacy paths.

After that block, type:

```rust
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-08.rs" skip=92
```

### 7. `src/marker.wgsl`

Use the same radius rule in the existing marker paths. Color and integer picking retain the same projected marker footprint.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

### 8. `src/finite_marker.wgsl`

Use the same radius rule in the existing marker paths. Color and integer picking retain the same projected marker footprint.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,flags,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}

#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/mesh_density_tests.rs`

Exercise actual dense physical footprints, explicit point/control independence, correct rear-edge visibility, source release and shared geometry.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-13.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/renderer.rs`

Register the three actual source-bound density GPU checks.

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-14.rs" skip=22
```

### 11. `src/marker_coverage.wgsl`

Measure the circle at the actual physical fragment pixel relative to its fixed centre; keep one shared colour/query coverage function.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/96zzzzzzq-taper-crowded-original-mesh-vertices-physical-centre-01.wgsl"
```

### 12. `src/marker.wgsl`

Carry a fixed screen centre instead of interpolating the expanded quad offset.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(1) @interpolate(flat) centre: vec2<f32>,
```

### 13. `src/marker_pick.wgsl`

Carry a fixed screen centre instead of interpolating the expanded quad offset.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(1) @interpolate(flat) centre: vec2<f32>,
```

### 14. `src/marker.wgsl`

Compute the centre before expanding the existing quad; no new vertex buffer or radius policy. Pass the fixed screen centre through the existing location. Use the fragment built-in pixel rather than interpolated quad coordinates.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) centre: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let centre=(position.xy/position.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, centre, radius);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input.colour,input.radius,input.position.xy-input.centre); }
```

### 15. `src/marker_pick.wgsl`

Give the cropped query the same centre in its local physical-pixel coordinates. Remove the unnecessary reconstructed colour-output record. Apply the same analytic disc coverage to integer queries.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) centre: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(center, diameter, colour, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.centre = source.centre-vec2<f32>(vec2<u32>(crop._a,crop._b));
    output.radius = source.radius;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(input.colour,input.radius,input.position.xy-input.centre).a <= 0.0) { discard; }
    return input.id;
}
```

### 16. `src/finite_marker.wgsl`

Reuse the existing fixed axis centre; drop the redundant interpolated offset. Use the actual fragment pixel and remove the second redundant varying.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,flags,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}

#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

### 17. `src/finite_marker_pick.wgsl`

Reuse the existing fixed axis centre; drop the redundant interpolated offset. Use the actual fragment pixel and remove the second redundant varying.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) flags:u32) -> PickOutput {
    let source = geometry(center, diameter, colour, index,flags);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
}
```

### 18. `src/finite_marker.wgsl`

Keep the rejected-vertex result aligned with the smaller output. Reuse existing fixed source centre/depth fields and omit redundant pixel interpolation. Read visibility at the actual full-frame physical pixel. Evaluate colour coverage from the existing flat centre.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, radius,flags,axis,depths);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.position.xy,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input.colour,input.radius,input.position.xy-input.finite_axis_value.xy);return vec4<f32>(c.rgb,c.a*alpha);}

#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

### 19. `src/finite_marker_pick.wgsl`

Remove the deleted output varying from the query vertex. Keep the existing fixed axis/depth values without another varying. Restore the query fragment to the full physical frame once, then share it with coverage and visibility. Keep the integer-query disc identical to colour coverage. Use the same restored full-frame pixel for depth/section eligibility.

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> crop: Crop;
struct PickOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) flags:u32) -> PickOutput {
    let source = geometry(center, diameter, colour, index,flags);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.radius = source.radius;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let pixel=input.position.xy+vec2<f32>(vec2<u32>(crop._a,crop._b));
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(input.colour,input.radius,pixel-input.finite_axis_value.xy).a <= 0.0) { discard; }
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
}
```

**Result after `96zzzzzzq-taper-crowded-original-mesh-vertices`:** Crowding-aware dots retain more of the face at readable zoom levels. At extreme zoom-out, the half-physical-pixel minimum can still overlap. Explicit point sizes, active controls, original ownership and shared geometry remain stable.

## Build, Verify & Visual Result

```sh
npm --prefix ../session_tests run course -- dependencies 22.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 22.3
npm --prefix ../session_tests run course -- save 22.3
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../../recovery.md).

Continue with [22.4 · Original ambient inputs and real pass ordering](../../consolidation/96zzzzzzr-share-geometric-ambient-inputs.md).
