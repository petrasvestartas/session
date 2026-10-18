# 22.6.5 · Draw original CAD wire curves

**Chapter 22 · Surface lighting, contact and display quality · Section 22.6 · Original display controls** · [Contents](contents.md)

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

Keep coherent CAD face boundaries and their original edge addresses. When an edge has no face boundary, sample its original 3D curve into the existing joined path lane. The two kinds retain independent Edges and Lines controls.

## Type

Continue from [Apply semantic view controls](96zzzzzzzp-apply-semantic-view-controls.md). [Save or recover your work](recovery.md).

### 1. `src/cad_boundary.rs`

Validate a face-free original wire without inventing a face. Preserve the general BRep validator and all coherent face-boundary guards.

<details>
<summary>Locate the existing block</summary>

```rust
        if !source.is_valid() {return Err("BRep topology contains an invalid reference");}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let b=&source;
        let index=|i:i32,n:usize|i>=0&&(i as usize)<n;
        let wire=b.m_faces.is_empty()&&b.m_shells.is_empty()&&b.m_solids.is_empty()&&!b.m_edges.is_empty()&&b.m_edges.iter().all(|e|index(e.start_vertex,b.m_vertices.len())&&index(e.end_vertex,b.m_vertices.len())&&(e.degenerated||index(e.curve_3d_index,b.m_curves_3d.len()))&&e.pcurves.iter().all(|p|index(p.surface_index,b.m_surfaces.len())&&index(p.curve_2d_index,b.m_curves_2d.len())&&(p.curve_2d_index_2<0||index(p.curve_2d_index_2,b.m_curves_2d.len()))))&&b.m_wires.iter().all(|w|!w.edges.is_empty()&&w.edges.iter().all(|r|index(r.index,b.m_edges.len())));
        if !b.is_valid()&&!wire {return Err("BRep topology contains an invalid reference");}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-01.rs"
```

</details>

### 2. `src/cad_display.rs`

Retain original fallback curve samples beside coherent boundaries. Use the existing joined path lane rather than introducing a family buffer or drawing overlapping capsules. Sample original face-free curves without inventing mesh faces or mapped boundary addresses; preserve sampled bounds. Retain the fallback curve source keys and samples. Use one existing joined-stroke helper for ordinary and selected original CAD wires, with precise placed points and no duplicate scene owner.

<details>
<summary>Locate the existing block</summary>

```rust
use std::rc::Rc;
use session_rust::Xform;

#[derive(Clone)]
pub struct Boundary {pub edge:usize,pub points:Vec<[f64;3]>}
#[derive(Clone)]
pub struct Prepared {pub source:Rc<session_rust::BRep>,pub edges:Vec<Boundary>,pub samples:Vec<crate::cad_face::Sample>,pub face_count:usize,pub edge_ids:Vec<usize>}

pub fn prepare(source:Rc<session_rust::BRep>)->Result<(Prepared,crate::prepared::PreparedMesh),&'static str> {
    let boundary=crate::cad_boundary::Prepared::new(source.clone())?;let data=crate::cad_face::prepare(&boundary)?;
    let mut proxy=session_rust::Mesh::from_vertices_and_faces(data.points,data.faces);proxy.set_guid(source.guid().to_owned());proxy.name=source.name.clone();proxy.is_visible=source.is_visible;proxy.is_locked=source.is_locked;proxy.set_objectcolor(source.surfacecolor.clone());
    let mut display=crate::mesh::Mesh::new(data.vertices,data.indices)?.with_normals(data.normals)?;display.parts.faces=(0..boundary.faces.len()).collect();display.parts.triangles=data.face_ids;
    let (mut edges,mut edge_ids)=(Vec::new(),Vec::new());
    for chain in boundary.chains.iter().flatten() {
        let points=boundary.points(chain.edge).ok_or("Missing CAD boundary sample")?;
        for pair in chain.keys.windows(2) {
            let mesh=&boundary.faces[chain.face];let ends=[pair[0],pair[1]].map(|key|{let v=&mesh.vertex[&key];[v.x,v.y,v.z]});display.parts.edges.push((data.keys[chain.face][&pair[0]] as usize,data.keys[chain.face][&pair[1]] as usize));display.parts.endpoints.push(ends);edge_ids.push(chain.edge);
        }
        edges.push(Boundary {edge:chain.edge,points});
    }
    let prepared=Prepared {source,edges,samples:data.samples,face_count:boundary.faces.len(),edge_ids};
    let mesh=crate::prepared::PreparedMesh {geometry:Rc::new(proxy),display,source:None,model:Xform::identity()};Ok((prepared,mesh))
}

pub fn example()->session_rust::BRep {
    let mut source=session_rust::BRep::create_block_with_hole(4.,3.,0.6,0.65);source.name="Original CAD bore".into();source.surfacecolor=session_rust::Color::new(0.35,0.65,0.85,1.);source
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::rc::Rc;
use session_rust::Xform;

#[derive(Clone)]
pub struct Boundary {pub edge:usize,pub points:Vec<[f64;3]>}
#[derive(Clone)]
pub struct Prepared {pub source:Rc<session_rust::BRep>,pub edges:Vec<Boundary>,pub samples:Vec<crate::cad_face::Sample>,pub wires:Vec<Boundary>,pub face_count:usize,pub edge_ids:Vec<usize>}

pub fn prepare(source:Rc<session_rust::BRep>)->Result<(Prepared,crate::prepared::PreparedMesh),&'static str> {
    let boundary=crate::cad_boundary::Prepared::new(source.clone())?;let mut data=crate::cad_face::prepare(&boundary)?;
    let mut wires=Vec::new();let mut seen=std::collections::HashSet::new();for(edge,chain)in boundary.chains.iter().enumerate(){let e=&source.m_edges[edge];if chain.is_some()||e.degenerated||e.curve_3d_index<0||!seen.insert(e.curve_3d_index){continue;}
        let points=crate::curve::sample(&source.m_curves_3d[e.curve_3d_index as usize])?;if data.vertices.len()+points.len()>1<<20{return Err("CAD wire sample budget exceeded")}
        for p in &points{data.points.push(session_rust::Point::new(p[0],p[1],p[2]));data.vertices.push([p[0]as f32,p[1]as f32,p[2]as f32,0.,0.,0.]);data.normals.push([0.,0.,1.]);}wires.push(Boundary{edge,points});
    }
    let mut proxy=session_rust::Mesh::from_vertices_and_faces(data.points,data.faces);proxy.set_guid(source.guid().to_owned());proxy.name=source.name.clone();proxy.is_visible=source.is_visible;proxy.is_locked=source.is_locked;proxy.set_objectcolor(source.surfacecolor.clone());
    let mut display=crate::mesh::Mesh::new(data.vertices,data.indices)?.with_normals(data.normals)?;display.parts.faces=(0..boundary.faces.len()).collect();display.parts.triangles=data.face_ids;
    let (mut edges,mut edge_ids)=(Vec::new(),Vec::new());
    for chain in boundary.chains.iter().flatten() {
        let points=boundary.points(chain.edge).ok_or("Missing CAD boundary sample")?;
        for pair in chain.keys.windows(2) {
            let mesh=&boundary.faces[chain.face];let ends=[pair[0],pair[1]].map(|key|{let v=&mesh.vertex[&key];[v.x,v.y,v.z]});display.parts.edges.push((data.keys[chain.face][&pair[0]] as usize,data.keys[chain.face][&pair[1]] as usize));display.parts.endpoints.push(ends);edge_ids.push(chain.edge);
        }
        edges.push(Boundary {edge:chain.edge,points});
    }
    let prepared=Prepared {source,edges,samples:data.samples,face_count:boundary.faces.len(),edge_ids,wires};
    let mesh=crate::prepared::PreparedMesh {geometry:Rc::new(proxy),display,source:None,model:Xform::identity()};Ok((prepared,mesh))
}

pub fn wires(row:&crate::scene::Object,anchor:[f64;3],colour:Option<[f32;4]>,extra:f32)->Vec<crate::chain::Segment>{
    let Some(cad)=&row.cad else{return Vec::new()};let model=crate::render_space::placed(&row.model,anchor);
    cad.wires.iter().flat_map(|wire|{let mut points=Vec::new();for p in &wire.points{let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32();if points.last()!=Some(&p){points.push(p);}}
        crate::chain::segments(&points,colour.unwrap_or([0.,0.,0.,1.]),crate::mesh_pens::width(cad.source.width)+extra,0)
    }).collect()
}

pub fn example()->session_rust::BRep {
    let mut source=session_rust::BRep::create_block_with_hole(4.,3.,0.6,0.65);source.name="Original CAD bore".into();source.surfacecolor=session_rust::Color::new(0.35,0.65,0.85,1.);source
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-05.rs"
```

</details>

### 3. `src/chain.rs`

Share the existing joined-stroke helper, retaining its complete original join and closed-chain body. Use the shared source pen parameters. Retain start arrow semantics. Retain end arrow semantics. Close the shared helper after the unchanged segment iterator.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
#[derive(Clone, Copy)]
pub struct Segment { pub stroke: Stroke, pub previous: [f32; 3], pub next: [f32; 3], pub heads: u32 }

impl PreparedChain {
    pub fn segments(&self) -> Vec<Segment> {
        let points = &self.points; let last = points.len().saturating_sub(1);
        let closed = last > 1 && points.first() == points.last();
        points.windows(2).enumerate().map(|(i, ends)| Segment {
            stroke: Stroke { start: ends[0], end: ends[1], colour: self.colour, width: self.width },
            previous: if i > 0 { points[i - 1] } else if closed { points[last - 1] } else { ends[0] },
            next: if i + 2 <= last { points[i + 2] } else if closed { points[1] } else { ends[1] },
            heads: if closed { 0 } else { (if i == 0 { self.heads & 2 } else { 0 }) | (if i + 1 == last { self.heads & 1 } else { 0 }) },
        }).collect()
    }
}

impl Segment {
```

</details>

Replace it with:

```rust
#[derive(Clone, Copy)]
pub struct Segment { pub stroke: Stroke, pub previous: [f32; 3], pub next: [f32; 3], pub heads: u32 }

impl PreparedChain {pub fn segments(&self)->Vec<Segment>{segments(&self.points,self.colour,self.width,self.heads)}}
pub fn segments(points:&[[f32;3]],colour:[f32;4],width:f32,heads:u32)->Vec<Segment>{
        let last = points.len().saturating_sub(1);
        let closed = last > 1 && points.first() == points.last();
        points.windows(2).enumerate().map(|(i, ends)| Segment {
            stroke: Stroke { start: ends[0], end: ends[1], colour, width },
            previous: if i > 0 { points[i - 1] } else if closed { points[last - 1] } else { ends[0] },
            next: if i + 2 <= last { points[i + 2] } else if closed { points[1] } else { ends[1] },
            heads: if closed { 0 } else { (if i == 0 { heads & 2 } else { 0 }) | (if i + 1 == last { heads & 1 } else { 0 }) },
        }).collect()
}

impl Segment {
```

<details>
<summary>Complete updated block reference</summary>

```rust
pub fn shader() -> String { crate::finite_project_source::expand(include_str!("chain.wgsl")) }
use std::rc::Rc;
use session_rust::{Arrowhead, Line, Polyline};
use crate::stroke::{PreparedLine};

#[derive(Clone)]
pub enum Source { Line(Rc<Line>), Polyline(Rc<Polyline>), Curve(Rc<session_rust::NurbsCurve>, Rc<Vec<[f64; 3]>>) }

#[derive(Clone)]
pub struct PreparedChain { pub source: Source, pub points: Rc<Vec<[f32; 3]>>, pub colour: [f32; 4], pub width: f32, pub heads: u32 }

impl Source {
    pub fn name(&self)->&str {match self {Self::Line(source)=>&source.name,Self::Polyline(source)=>&source.name,Self::Curve(source,_)=>&source.name}}
    pub fn locked(&self) -> bool {
        match self { Self::Line(r) => r.is_locked, Self::Polyline(r) => r.is_locked, Self::Curve(r, _) => r.is_locked }
    }

    pub fn visible(&self) -> bool {
        match self { Self::Line(line) => line.is_visible, Self::Polyline(line) => line.is_visible, Self::Curve(curve, _) => curve.is_visible }
    }

    pub fn guid(&self) -> &str { match self { Self::Line(line) => line.guid(), Self::Polyline(line) => line.guid(), Self::Curve(curve, _) => curve.guid() } }

    pub fn owner(&self) -> (u8, usize) {
        match self { Self::Line(line) => (0, Rc::as_ptr(line) as usize), Self::Polyline(line) => (1, Rc::as_ptr(line) as usize), Self::Curve(curve, _) => (2, Rc::as_ptr(curve) as usize) }
    }

    pub fn controls(&self) -> Vec<crate::controls::Control> {
        match self {
            Self::Curve(curve, _) => (0..curve.m_cv_count).map(|index|
                crate::controls::Control::new(Rc::clone(curve), index).expect("Validated original curve control")).collect(),
            _ => Vec::new(),
        }
    }

    pub fn bounds_coordinates(&self) -> Vec<[f64; 3]> {
        if matches!(self, Self::Curve(..)) { self.controls().iter().map(|c| c.position().unwrap()).collect() }
        else { self.coordinates() }
    }

    pub fn coordinates(&self) -> Vec<[f64; 3]> {
        match self {
            Self::Line(line) => [line.start(), line.end()].map(|p| [p[0], p[1], p[2]]).to_vec(),
            Self::Polyline(line) => line.coords.chunks_exact(3).map(|p| [p[0], p[1], p[2]]).collect(),
            Self::Curve(_, samples) => samples.as_ref().clone(),
        }
    }
}

impl PreparedChain {
    pub fn curve(source: Rc<session_rust::NurbsCurve>) -> Result<Self, &'static str> {
        let samples = Rc::new(crate::curve::sample(&source)?); let _ = source.guid();
        for index in 0..source.m_cv_count { crate::controls::Control::new(Rc::clone(&source), index)?.marker()?; }
        let mut line = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        line.width = source.width;
        line.linecolor = source.linecolors.first().cloned().unwrap_or_else(|| session_rust::Color::new(0.0, 0.0, 0.0, 1.0));
        let display = PreparedLine::new(Rc::new(line))?.display;
        Self::new(Source::Curve(source, samples), display.colour, display.width)
    }

    pub fn line(source: Rc<Line>) -> Result<Self, &'static str> {
        let display = PreparedLine::new(Rc::clone(&source))?.display;
        Self::new(Source::Line(source), display.colour, display.width)
    }

    pub fn polyline(source: Rc<Polyline>) -> Result<Self, &'static str> {
        if source.coords.len() % 3 != 0 || !(6..=30_000).contains(&source.coords.len()) {
            return Err("Polyline needs 2–10,000 complete points");
        }
        let _ = source.guid();
        let mut line = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        line.linecolor = source.linecolor.clone(); line.width = source.width;
        let display = PreparedLine::new(Rc::new(line))?.display;
        Self::new(Source::Polyline(source), display.colour, display.width)
    }

    fn new(source: Source, colour: [f32; 4], width: f32) -> Result<Self, &'static str> {
        let heads = match &source { Source::Line(line) => line.arrowhead, Source::Polyline(line) => line.arrowhead, Source::Curve(curve, _) => curve.arrowhead };
        let heads = match heads { Arrowhead::NONE => 0, Arrowhead::END => 1, Arrowhead::START => 2, Arrowhead::BOTH => 3 };
        let mut points = Vec::new();
        for p in source.coordinates() {
            let p = p.map(|v| v as f32);
            if p.iter().any(|v| !v.is_finite()) { return Err("Chain exceeds the display range"); }
            if points.last() != Some(&p) { points.push(p); }
        }
        Ok(Self { source, points: Rc::new(points), colour, width, heads })
    }

}
use crate::stroke::Stroke;
#[derive(Clone, Copy)]
pub struct Segment { pub stroke: Stroke, pub previous: [f32; 3], pub next: [f32; 3], pub heads: u32 }

impl PreparedChain {pub fn segments(&self)->Vec<Segment>{segments(&self.points,self.colour,self.width,self.heads)}}
pub fn segments(points:&[[f32;3]],colour:[f32;4],width:f32,heads:u32)->Vec<Segment>{
        let last = points.len().saturating_sub(1);
        let closed = last > 1 && points.first() == points.last();
        points.windows(2).enumerate().map(|(i, ends)| Segment {
            stroke: Stroke { start: ends[0], end: ends[1], colour, width },
            previous: if i > 0 { points[i - 1] } else if closed { points[last - 1] } else { ends[0] },
            next: if i + 2 <= last { points[i + 2] } else if closed { points[1] } else { ends[1] },
            heads: if closed { 0 } else { (if i == 0 { heads & 2 } else { 0 }) | (if i + 1 == last { heads & 1 } else { 0 }) },
        }).collect()
}

impl Segment {
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = self.stroke.bytes();
        bytes.extend(self.previous.into_iter().chain(self.next).flat_map(f32::to_ne_bytes));
        bytes.extend(self.heads.to_ne_bytes()); bytes
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-10.rs"
```

</details>

### 4. `src/pick_sources.rs`

Prepare fallback path records once alongside their original Object-only query IDs. Append fallback IDs in the same order as their joined path records. They follow Lines and never create BRepEdge subparts.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        let vertex_end=sources.marker_ids.len()as u32;
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, sub_id::OBJECT_INK)); }
        let point_end=sources.marker_ids.len()as u32;
```

</details>

Replace it with:

```rust
                sources.path_ids.push(id(row.id, ids[1]));
            }
        }
        for row in scene.objects().iter().filter(|r|scene.visible(r.id)){
            let colour=scene.view.edges.get(&row.id).map(|c|[c[0]as f32/255.,c[1]as f32/255.,c[2]as f32/255.,1.]);
            let wires=crate::cad_display::wires(row,anchor,colour,0.);sources.path_ids.extend(std::iter::repeat_n(id(row.id,sub_id::OBJECT_INK),wires.len()));sources.wire_segments.extend(wires);
        }
        let vertex_end=sources.marker_ids.len()as u32;
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, sub_id::OBJECT_INK)); }
        let point_end=sources.marker_ids.len()as u32;
```

<details>
<summary>Locate this block</summary>

```rust
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub path_ids: Vec<[u32; 2]>,
    pub mesh_markers:Vec<crate::marker::Marker>,
    pub marker_ids: Vec<[u32; 2]>,
```

</details>

Replace it with:

```rust
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub wire_segments:Vec<crate::chain::Segment>,
    pub path_ids: Vec<[u32; 2]>,
    pub mesh_markers:Vec<crate::marker::Marker>,
    pub marker_ids: Vec<[u32; 2]>,
```

<details>
<summary>Complete updated block reference</summary>

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
    pub mesh_runs:crate::ink_ranges::Runs,
    pub marker_runs:crate::ink_ranges::Runs,
    pub sheet_bases:std::collections::HashMap<(ObjectId,usize),u32>,
    pub faces: Vec<Target>,
    pub edges: Vec<Target>,
    pub controls: Vec<Target>,
    pub mesh_bases: Vec<(ObjectId, u32)>,
    pub mesh_ink: Vec<crate::stroke::Stroke>,
    pub mesh_ids: Vec<[u32; 2]>,
    pub mesh_metadata:Vec<u32>,
    pub line_ids: Vec<[u32; 2]>,
    pub wire_segments:Vec<crate::chain::Segment>,
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
            let ink_start=sources.mesh_ink.len()as u32;
            let line_frame=matches!(&row.original,Some(session_rust::Geometry::Plane(_)|session_rust::Geometry::OBB(_)));
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
                    sources.mesh_markers.push(crate::marker::Marker::mesh_dot(model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32(),edge_colour.unwrap_or_else(||row.mesh.parts.dot_colour(i)),row.mesh.spacing));
                    sources.marker_ids.push([owner,0]);
                }
            }
            let flag=if planes.contains(&row.id){4096}else{0};
            let boundary_width=row.cad.as_ref().map(|c|c.source.width).or_else(||row.surface.as_ref().map(|s|s.source.width)).map(crate::mesh_pens::width);
            let mut cad_edges=row.cad.as_ref().map(|cad|cad.edge_ids.iter());
            let mut surface_edges=row.surface.as_ref().map(|surface|surface.edge_ids());
            for (index,(&(a,b),ends)) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints).enumerate() {
                let part=cad_edges.as_mut().and_then(|edges|edges.next()).map(|edge|Part::BRepEdge(*edge)).unwrap_or_else(||surface_edges.as_mut().and_then(|edges|edges.next()).map_or(Part::MeshEdge(a,b),Part::SurfaceEdge));
                let ids = if line_frame {[owner,sub_id::OBJECT_INK]}else{append(&mut sources.edges, Kind::Edge, Target { owner: row.id, part })?};
                let (width,colour)=row.mesh.parts.pen(index);
                let width=boundary_width.unwrap_or(width);
                if width==0. {continue;}
                let ends=ends.map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());
                sources.mesh_ink.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: edge_colour.unwrap_or(colour), width });
                sources.mesh_ids.push([owner,if derived{0}else{ids[1]}]);
                sources.mesh_metadata.extend([flag,facing]);
            }
            let kind=if line_frame {crate::ink_ranges::LINES}else{crate::ink_ranges::EDGES};
            sources.mesh_runs.push(kind,ink_start,sources.mesh_ink.len()as u32);
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
        for row in scene.objects().iter().filter(|r|scene.visible(r.id)){
            let colour=scene.view.edges.get(&row.id).map(|c|[c[0]as f32/255.,c[1]as f32/255.,c[2]as f32/255.,1.]);
            let wires=crate::cad_display::wires(row,anchor,colour,0.);sources.path_ids.extend(std::iter::repeat_n(id(row.id,sub_id::OBJECT_INK),wires.len()));sources.wire_segments.extend(wires);
        }
        let vertex_end=sources.marker_ids.len()as u32;
        for row in scene.points().iter().filter(|row| scene.visible(row.id)) { sources.marker_ids.push(id(row.id, sub_id::OBJECT_INK)); }
        let point_end=sources.marker_ids.len()as u32;
        let control_start=sources.mesh_ink.len()as u32;
        for (owner, stroke) in scene.control_nets_at(anchor) {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
            sources.mesh_metadata.extend([0,crate::packed_vertex::NO_NORMAL]);
        }
        sources.mesh_runs.push(crate::ink_ranges::CONTROLS,control_start,sources.mesh_ink.len()as u32);
        for row in scene.control_markers_at(anchor) {
            let ids = append(&mut sources.controls, Kind::Control, Target { owner: row.parent, part: row.part })?;
            sources.marker_ids.push(id(row.parent, ids[1]));
        }
        sources.marker_runs.push(crate::ink_ranges::VERTICES,0,vertex_end);
        sources.marker_runs.push(crate::ink_ranges::POINTS,vertex_end,point_end);
        sources.marker_runs.push(crate::ink_ranges::CONTROLS,point_end,sources.marker_ids.len()as u32);
        for row in scene.sheets().iter().filter(|row|scene.visible(row.id)) {for page in row.prepared.pages() {sources.sheet_bases.insert((row.id,page.key()),u32::try_from(sources.edges.len()).map_err(|_|"Too many sheet pick addresses")?);for entity in &page.entities {append(&mut sources.edges,Kind::Edge,Target {owner:row.id,part:if *entity==u32::MAX {Part::Object} else {Part::SheetEntity(*entity)}})?;}}}
        Ok(sources)
    }

    pub fn bytes(&self) -> usize {
        self.mesh_runs.bytes()+self.marker_runs.bytes()
            +(self.faces.capacity() + self.edges.capacity() + self.controls.capacity()) * std::mem::size_of::<Target>()
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

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-12.rs"
```

</details>

### 5. `src/renderer.rs`

Upload the prepared fallback spans through the existing path buffer and visibility/query policy.

<details>
<summary>Locate the existing block</summary>

```rust
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}))
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments_at(anchor);if let Some(colour)=scene.face_rgba(row.id) {for segment in &mut segments {segment.stroke.colour=colour;}}segments}).chain(std::mem::take(&mut self.sources.wire_segments)))
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-13.rs"
```

</details>

### 6. `src/highlight.rs`

Selected mixed and face-free CAD wires use the same joined highlight lane and source pen width, keeping black centers and Lines masks.

<details>
<summary>Locate the existing block</summary>

```rust
            r.segments_at(anchor).into_iter().map(|mut s| { s.stroke.colour = yellow; s.stroke.width += 2.; s })))?;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            r.segments_at(anchor).into_iter().map(|mut s| { s.stroke.colour = yellow; s.stroke.width += 2.; s })).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).flat_map(|r|crate::cad_display::wires(r,anchor,Some(yellow),2.))))?;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-14.rs"
```

</details>

### 7. `src/cad_shape_cache.rs`

Rigid-source replay must transform fallback curve samples together with mapped edges; retain original source records and precise models.

<details>
<summary>Locate the existing block</summary>

```rust
        for edge in &mut prepared.edges {for p in &mut edge.points {*p=motion.point(*p);}}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        for edge in prepared.edges.iter_mut().chain(&mut prepared.wires) {for p in &mut edge.points {*p=motion.point(*p);}}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-15.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/cap_gpu_tests.rs`

Supply a format-parameterized real GPU readback helper while keeping every existing Unorm fixture unchanged; matched maintained comparisons need actual sRGB targets, not edited pixels.

<details>
<summary>Locate the existing block</summary>

```text
use crate::{scene::Scene,renderer::Renderer,viewport::Viewport};
use std::rc::Rc;
pub(crate) fn source(open:bool)->session_rust::Mesh{
 let corners=(0..8).map(|i|session_rust::Point::new(if i%4==1||i%4==2{0.6}else{-0.6},if i%4>=2{0.6}else{-0.6},if i>=4{-0.2}else{-0.8})).collect();let mut faces=vec![vec![0,3,2,1],vec![0,1,5,4],vec![2,3,7,6],vec![0,4,7,3],vec![1,2,6,5]];if !open{faces.push(vec![4,5,6,7]);}let mut mesh=session_rust::Mesh::from_vertices_and_faces(corners,faces);mesh.set_objectcolor(session_rust::Color::new(1.,0.,0.,1.));mesh
}
pub(crate) fn scene(open:bool)->(Scene,crate::scene::ObjectId){let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(source(open)).unwrap()).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();(scene,owner)}
pub(crate) fn matrix()->[f32;16]{session_rust::Xform::orthographic(-1.,1.,-1.,1.,1.,0.).m.map(|v|v as f32)}
pub(crate) fn renderer(scene:&Scene)->Renderer{let instance=wgpu::Instance::new(wgpu::InstanceDescriptor{backends:wgpu::Backends::VULKAN,flags:Default::default(),memory_budget_thresholds:Default::default(),backend_options:Default::default(),display:None});let adapter=pollster::block_on(instance.request_adapter(&Default::default())).unwrap();let(device,queue)=pollster::block_on(adapter.request_device(&Default::default())).unwrap();device.on_uncaptured_error(std::sync::Arc::new(|e|panic!("actual original cap: {e}")));let mut renderer=Renderer::new(device,queue,wgpu::TextureFormat::Rgba8Unorm,scene);renderer.resize(Viewport{width:320,height:240});renderer}
pub(crate) fn wait(r:&Renderer){r.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();}
pub(crate) fn pixels(r:&Renderer)->Vec<u8>{pixels_at(r,matrix())}
pub(crate) fn pixels_at(r:&Renderer,transform:[f32;16])->Vec<u8>{let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("original cap pixel fixture"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&transform);let pitch=1280;let read=r.device.create_buffer(&wgpu::BufferDescriptor{label:None,size:pitch*240,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=r.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch as u32),rows_per_image:Some(240)}},texture.size());r.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());wait(r);rx.recv().unwrap().unwrap();let data=read.slice(..).get_mapped_range().to_vec();read.unmap();data}
pub(crate) fn at(data:&[u8],x:usize,y:usize)->&[u8]{&data[(y*320+x)*4..(y*320+x+1)*4]}
#[test]
fn actual_closed_original_sections_draw_grey_hatch_and_selected_color_then_release_large_images(){
 let(mut scene,owner)=scene(false);let mut r=renderer(&scene);let solid=pixels(&r);let center=at(&solid,208,96);assert!(center[..3].iter().all(|v|(150..=155).contains(v)),"solid section: {center:?}");assert_eq!(r.cap_usage()[1],1);assert_eq!(r.cap_usage()[2],1);assert_eq!(r.cap_usage()[0],16+16+320*240*2);let uploads=r.stats()[0];r.set_clipping(true,false);let hatch=pixels(&r);assert_ne!(hatch,solid);assert!(hatch.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>400,"hatch has actual black ink");r.set_clipping(true,true);r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);let center=at(&selected,208,96);assert!(center[0]>240&&center[1]>240&&center[2]<20,"selected original section: {center:?}");assert_eq!(r.stats()[0],uploads);assert_eq!(r.cap_usage()[2],1,"selection retains original closedness cache");let stored=selected.clone();r.use_uniform_surface_records();assert_eq!(pixels(&r),stored,"Exact uniform and shared cap crossing pixels");r.set_scene(&scene,None);r.select_objects(&scene,&[owner]).unwrap();assert_eq!(pixels(&r),stored);assert_eq!(r.stats()[0],uploads);scene.clear();r.set_scene(&scene,None);pixels(&r);assert_eq!(r.cap_usage()[0],16);assert_eq!(r.cap_usage()[1],0);
}
#[test]
fn actual_open_originals_get_no_section_fill(){let(scene,_)=scene(true);let r=renderer(&scene);let image=pixels(&r);assert_eq!(r.cap_usage()[1],0);let center=at(&image,208,96);assert!(center[0]>center[1]+80&&center[0]>center[2]+80,"open back face remains red: {center:?}");}

pub(crate) fn reversed(mut matrix:[f32;16])->[f32;16]{for column in 0..4{matrix[column*4+2]=matrix[column*4+3]-matrix[column*4+2];}matrix}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-16.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/cad_wire_tests.rs`

Supply independent original-wire topology validation and actual mixed/family/query/shared-source/selection/archive GPU witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-17.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/lib.rs`

Supply original CAD wire regressions without production code.

<details>
<summary>Locate the existing block</summary>

```text
mod view_command_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzq-draw-original-cad-wire-curves-18.rs" skip=23
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.6.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Original CAD fallback curves share the existing joined path buffer, source ownership, family masks and selected-ink path. Actual complete application, maintained comparison, feature pictures and published UI remain mandatory.

### Actual Native original CAD wires

Actual original face-free and mixed face/wire BReps, each with a shared mirrored nonuniform placement. Captions distinguish one/four samples, ordinary family flags and selected source curves. White output when all face-free wires are hidden is an intentional hidden-family state, paired with visible-source and original-owner checks. The existing bore, mapped boundary and black-core assertions remain mandatory. Matched maintained frames and actual Browser proof are separate gates. The source’s four-pixel pen leaves small white bore centers in these 320×240 frames. The maintained comparison uses additional renders with matching sRGB output.

**Original face-free CAD wires and shared placement, one sample — actual Native original CAD wire frame.**

![Actual Native component: Original face-free CAD wires and shared placement, one sample](../screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-cad-wire-0.png)

**Original face-free CAD wires and shared placement, four samples — actual Native original CAD wire frame.**

![Actual Native component: Original face-free CAD wires and shared placement, four samples](../screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-cad-wire-1.png)

**Hide Lines removes face-free wires; original owners and source archive retained, four samples — actual Native original CAD wire frame.**

![Actual Native component: Hide Lines removes face-free wires; original owners and source archive retained, four samples](../screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-cad-wire-2.png)

**Selected original face-free wires retain their black centers and yellow fringe, four samples — actual Native original CAD wire frame.**

![Actual Native component: Selected original face-free wires retain their black centers and yellow fringe, four samples](../screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-cad-wire-3.png)

**Original bore plus detached wire and shared placement, four samples — actual Native original CAD wire frame.**

![Actual Native component: Original bore plus detached wire and shared placement, four samples](../screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-cad-wire-4.png)

**Hide Edges removes mapped CAD borders while detached original wires remain, four samples — actual Native original CAD wire frame.**

![Actual Native component: Hide Edges removes mapped CAD borders while detached original wires remain, four samples](../screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-cad-wire-5.png)

**Hide Lines removes detached wires while original faces and mapped borders remain, four samples — actual Native original CAD wire frame.**

![Actual Native component: Hide Lines removes detached wires while original faces and mapped borders remain, four samples](../screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-cad-wire-6.png)

**Selected mixed original CAD objects and their detached wires, four samples — actual Native original CAD wire frame.**

![Actual Native component: Selected mixed original CAD objects and their detached wires, four samples](../screenshots/journey/review/96zzzzzzzq-draw-original-cad-wire-curves-cad-wire-7.png)


**Verification pending.** Canonical build and browser acceptance have not yet been recorded for this checkpoint.

<details>
<summary>Code explanation and diagram</summary>

A face-free BRep needs explicit reference validation because the kernel general validator requires faces. Validate every referenced vertex, curve, wire and pcurve; reject faceless shells/solids and malformed input. Do not create a fake face or relax the guard that rejects an incoherent boundary on an actual CAD face.

Share the existing joined-stroke helper for ordinary and selected curves. Prepare path records once with their Object-only IDs, then move them into the existing path buffer. No new GPU lane, family table or proxy scene owner is needed.

Retain fallback samples in the original prepared CAD record and transform them during rigid source replay. Shared placements retain the same original source. Actual bounds, source archives, owner queries, selected black centers/fringes and one/four-sample returns need independent checks.

Supplied Native witnesses cover malformed topology, source-preserving rigid replay, and actual face-free/mixed/shared display and queries at both projections and sample counts. The browser uses their exported original archives at both densities/counts. Matched maintained frames remain a separate acceptance requirement.

Sample each repeated fallback curve reference once. Bound aggregate display samples before appending them or allocating a GPU path buffer; a topology count limit alone does not bound sampled geometry. Keep all original source references and archive fields.

The same-source, same-pose comparison uses separately rendered sRGB targets. Fine terminal and antialias coverage remain part of the following global pen work; this checkpoint establishes original fallback curves, ownership and independent family controls.

Original BRep curve → retained samples → existing joined path records and Object-only IDs.



Does a detached original CAD curve acquire a mapped BRepEdge query address?

No. It remains selectable as its original object. Only a coherent mapped face boundary receives the original BRepEdge subpart address.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Hide Edges and Lines independently on the mixed original bore/wire fixture. Restore exact pixels and query both shared owners without changing source archives or geometry uploads.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.6.5
npm --prefix ../session_tests run course -- save 22.6.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Original fallback 3D curves use the existing joined path buffer and its Lines mask; coherent mapped borders keep Edges and original BRepEdge addresses. Fallbacks retain Object-only source ownership. Wire-only validation checks every referenced vertex, curve, wire and pcurve without inventing faces or changing the kernel validator. Rigid cache replay transforms both mapped and fallback samples. Full procedural Plane display and pen/cloud/runtime coverage, independence, final chapter feature review and later camera/base-grid fixes remain pending.

Run every supplied topology/rigid-replay/GPU case plus the existing selected-bore and congruent-CAD regressions. Retain all inherited assertions, whole Native/WASM/optimized/canonical checks, matched maintained frames and individual critical review of every actual image.

Actual controlled single-sample original contact regression fixture. Dedicated original CAD wire and mixed-source figures establish the labelled wire behavior; this contact picture alone does not prove CAD fallback coverage.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.6.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
