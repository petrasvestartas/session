use session_rust::{Mesh,AABB};
use crate::production_curve_walk::{CylinderSegment,StrokeSegment,joined_rows};
use crate::production_ribbon::encode_width;
use crate::production_glyph::oct16;
use crate::source_mesh_topology::{MeshTopo,SlotMap,mesh_topology};
pub const COPLANAR_DOT:f64=1.0-1e-9;
const CREASE_COS:f64=0.906_307_787;
const WIREFRAME_BLACK_MIN:usize=10_000;
pub const SILHOUETTE_SAG:f32=-0.0;
const BLACK:u32=0xff00_0000;
const FACING_UNKNOWN:u32=u32::MAX;
pub const PROVENANCE:&str=include_str!("production_mesh_ink_provenance.json");
mod knobs {pub fn all_edges()->bool{false}}
#[derive(Default)]
pub struct SegRows {pub pipes:Vec<CylinderSegment>,pub pipe_ids:Vec<u32>,pub pipe_sags:Vec<f32>,pub pipe_chains:Vec<std::ops::Range<u32>>}
struct Ink<'a>{seg:&'a mut SegRows}
struct InkCx<'a>{row:u32,vpos:&'a [[f32;3]],slots:&'a SlotMap,smooth:bool}
#[derive(Clone,Default)]
pub struct Prepared {pub active:bool,pub segments:Vec<StrokeSegment>,pub ids:Vec<u32>,pub flags:u32,pub normals:Vec<crate::production_brep_ink::FacingPair>}
impl Prepared {pub fn bytes(&self)->usize{self.segments.capacity()*52+self.ids.capacity()*4+self.normals.capacity()*std::mem::size_of::<crate::production_brep_ink::FacingPair>()}}
/// Two face normals in one word; one face is used twice.
pub fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(a), oct16(b)),
        (Some(a), None) | (None, Some(a)) => (oct16(a), oct16(a)),
        _ => (None, None),
    };

    match pair {
        (Some(a), Some(b)) => {
            let v = a | b << 16;

            if v == FACING_UNKNOWN { v ^ 1 } else { v } // never the reserved code
        }
        _ => FACING_UNKNOWN,
    }
}

/// Pen width of edge `i`; one entry applies to all.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 {
        w[0]
    } else {
        w.get(i).copied().unwrap_or(1.0)
    }
}

/// Width 0 hides the edge.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// Normal of one of an edge's two faces.
fn normal_of(topo: &MeshTopo, faces: [u32; 2], side: usize) -> Option<[f64; 3]> {
    if faces[side] == u32::MAX {
        return None;
    }

    topo.normals[faces[side] as usize]
}

/// The two outward normals of edge `ei`.
fn edge_normals(topo: &MeshTopo, ei: usize) -> (Option<[f64; 3]>, Option<[f64; 3]>) {
    let f = topo.edge_faces[ei];
    let n0 = normal_of(topo, f, 0);
    let n1 = normal_of(topo, f, 1);

    if topo.opposed[ei] {
        return (n0, n1);
    }

    (n0, n1.map(reversed_normal)) // badly wound: flip the second
}

/// Dot product of two vectors.
fn dot3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// True for a border or crease edge of a smooth mesh.
fn smooth_feature(topo: &MeshTopo, ei: usize, pair: (Option<[f64; 3]>, Option<[f64; 3]>)) -> bool {
    if topo.edge_faces[ei][1] == u32::MAX {
        return true; // border
    }

    match pair {
        (Some(n0), Some(n1)) => dot3(&n0, &n1) < CREASE_COS,
        _ => true,
    }
}

/// One pipe per drawn edge.
fn push_pipes(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &InkCx) {
    let w = m.widths();
    let black_wire = topo.edges.len() >= WIREFRAME_BLACK_MIN; // dense mesh: black edges
    ink.seg.pipes.reserve(topo.edges.len());
    ink.seg.pipe_ids.resize(ink.seg.pipes.len(), u32::MAX);
    ink.seg.pipe_sags.resize(ink.seg.pipes.len(), 0.0);

    for (i, (a, b, col)) in topo.edges.iter().enumerate() {
        let (na, nb) = edge_normals(topo, i);
        let facing = pack_facing(na.as_ref(), nb.as_ref());

        if hidden(w, i) {
            continue;
        }

        // skip a diagonal inside a flat region
        if let (Some(n0), Some(n1)) = (na, nb)
            && dot3(&n0, &n1) >= COPLANAR_DOT
            && !knobs::all_edges()
        {
            continue;
        }

        // a smooth edge: drawn only where it is the silhouette
        let silhouette = cx.smooth && !smooth_feature(topo, i, (na, nb));

        ink.seg
            .pipe_ids
            .push(if cx.smooth { u32::MAX } else { i as u32 }); // edge id for picking
        ink.seg.pipe_sags.push(if silhouette { SILHOUETTE_SAG } else { 0.0 });
        ink.seg.pipes.push(CylinderSegment {
            p0: cx.vpos[cx.slots.slot(*a)],
            radius: encode_width(width_at(w, i)),
            p1: cx.vpos[cx.slots.slot(*b)],
            instance_id: cx.row,
            color: if black_wire { BLACK } else { *col },
            facing,
        });
    }
}

/// The opposite direction.
fn reversed_normal(n: [f64; 3]) -> [f64; 3] {
    [-n[0], -n[1], -n[2]]
}


fn prepared(seg:SegRows,flags:u32)->Prepared {
    Prepared{active:true,segments:joined_rows(&seg.pipes,&seg.pipe_chains,&[],0,&seg.pipe_sags),ids:seg.pipe_ids,flags,normals:Vec::new()}
}
pub fn mesh(source:&Mesh,triangles:usize)->Prepared {
    if triangles>200_000||source.widths()==[0.]{return Prepared{active:true,..Default::default()};}
    let keys=source.vertices();let points:Vec<_>=keys.iter().map(|key|{let p=&source.vertex[key];[p.x,p.y,p.z]}).collect();
    let slots=SlotMap::new(&keys);let topo=mesh_topology(source,&keys,&points,&slots);let points:Vec<_>=points.into_iter().map(|p|p.map(|v|v as f32)).collect();
    let mut seg=SegRows::default();push_pipes(&mut Ink{seg:&mut seg},source,&topo,&InkCx{row:0,vpos:&points,slots:&slots,smooth:false});
    prepared(seg,1024|if topo.closed{8192}else{16}|if source.number_of_faces()==1{128}else{0})
}
pub fn brep(boundary:&crate::cad_boundary::Prepared)->Prepared {
    use crate::production_brep_ink::{EdgePen,display_chain,push_edge_pipes};
    let source=&boundary.source;let signs=crate::cad_orient::face_signs(source,&boundary.faces,&boundary.chains);
    let pen=crate::production_curve_walk::Pen{row:0,radius:encode_width(source.width),color:BLACK};
    let ep=EdgePen::new(&boundary.faces,&signs,pen).recording();let mut seg=SegRows::default();let mut bounds=AABB::empty();
    for chain in boundary.chains.iter().flatten(){let display=display_chain(source,chain,&ep.fms[chain.face]);push_edge_pipes(&mut seg,chain,&display,&ep,&mut bounds);}
    ep.push_silhouette_pipes(&mut seg,&mut bounds);
    let mut out=prepared(seg,1024|64|if source.is_solid(){8192}else{16}|if source.face_count()==1{128}else{0});out.normals=ep.take_normals();out
}
pub fn replay(ink:&mut Prepared,motion:&crate::cad_shape_key::Motion){
    assert_eq!(ink.segments.len(),ink.normals.len());
    for(segment,(first,second))in ink.segments.iter_mut().zip(&mut ink.normals){
        let pipe=&mut segment.segment;pipe.p0=motion.position(pipe.p0);pipe.p1=motion.position(pipe.p1);
        *first=first.map(|n|motion.turn(n));*second=second.map(|n|motion.turn(n));pipe.facing=pack_facing(first.as_ref(),second.as_ref());
    }
}
pub fn surface(source:&session_rust::NurbsSurface,grid:&crate::surface::Grid)->Prepared {
    let mut seg=SegRows::default();
    for boundary in &grid.boundaries{let start=seg.pipes.len()as u32;
        for ends in &boundary.segments{seg.pipes.push(CylinderSegment{p0:grid.samples[ends[0]as usize].point.map(|v|v as f32),p1:grid.samples[ends[1]as usize].point.map(|v|v as f32),radius:encode_width(source.width),instance_id:0,color:BLACK,facing:FACING_UNKNOWN});seg.pipe_ids.push(boundary.edge as u32);}
        seg.pipe_chains.push(start..seg.pipes.len()as u32);
    }
    prepared(seg,1024|128|64|16)
}
pub fn source_edge(scene:&crate::scene::Scene,target:crate::pick_sources::Target)->Option<u32>{
    use crate::pick_sources::Part;
    match target.part{Part::PathEdge(i)|Part::BRepEdge(i)|Part::SurfaceEdge(i)=>u32::try_from(i).ok(),Part::MeshEdge(a,b)=>scene.objects().iter().find(|r|r.id==target.owner)?.mesh.parts.edges.iter().position(|e|*e==(a,b)).and_then(|i|u32::try_from(i).ok()),_=>None}
}
pub fn append(scene:&crate::scene::Scene,out:&mut crate::production_joined::Rows,sources:&crate::pick_sources::Sources,anchor:[f64;3],selected:&[crate::scene::ObjectId])->Result<(),&'static str>{
    use crate::pick_sources::Part;
    let key=|owner:crate::scene::ObjectId,part:Part|match part{Part::MeshEdge(a,b)=>Some((owner.value(),0,a,b)),Part::BRepEdge(edge)=>Some((owner.value(),1,edge,0)),Part::SurfaceEdge(edge)=>Some((owner.value(),2,edge,0)),_=>None};
    let mut picks=std::collections::HashMap::new();
    for(i,target)in sources.edges.iter().enumerate(){if let Some(key)=key(target.owner,target.part){picks.entry(key).or_insert(crate::sub_id::encode(crate::sub_id::Kind::Edge,i)?);}}
    for row in scene.objects().iter().filter(|r|scene.visible(r.id)&&r.mesh.parts.ink.active&&!r.mesh.parts.ink.segments.is_empty()){
        let ink=&row.mesh.parts.ink;
        let index=if let Some(index)=out.object_ids.iter().position(|id|*id==row.id){index as u32}else{
            let index=out.object_ids.len()as u32;let(mut object,translation)=crate::production_joined::object(scene,row.id,&row.model,anchor,selected.contains(&row.id),ink.flags);
            object[88..92].copy_from_slice(&row.mesh.spacing.to_ne_bytes());
            if let Some(rgb)=scene.view.edges.get(&row.id){let flags=u32::from_ne_bytes(object[80..84].try_into().unwrap())|512;object[80..84].copy_from_slice(&flags.to_ne_bytes());object[92..96].copy_from_slice(&crate::production_ribbon::pack_rgba([rgb[0]as f32/255.,rgb[1]as f32/255.,rgb[2]as f32/255.,1.]).to_ne_bytes());}
            out.objects.extend(object);out.translations.extend(translation);out.object_ids.push(row.id);out.pick_owners.push(crate::feature_owner::display_owner(scene,row.id).map_or(0,crate::scene::ObjectId::value));
            if let Some(bounds)=scene.selected_bounds(row.id){out.inside.push((index,bounds));}index
        };
        let at=index as usize*96+80;let old=u32::from_ne_bytes(out.objects[at..at+4].try_into().unwrap());out.objects[at..at+4].copy_from_slice(&((old&!(1024|16|8192|128|64))|ink.flags).to_ne_bytes());
        if !out.inside.iter().any(|(i,_)|*i==index){if let Some(bounds)=scene.selected_bounds(row.id){out.inside.push((index,bounds));}}
        let first=out.owners.len()as u32;
        for(segment,&edge)in ink.segments.iter().zip(&ink.ids){let mut segment=*segment;segment.segment.instance_id=index;
            for neighbour in[&mut segment.previous,&mut segment.next]{if *neighbour<u32::MAX-1{*neighbour+=first;}}
            let part=if edge==u32::MAX{Part::Object}else if row.cad.is_some(){Part::BRepEdge(edge as usize)}else if row.surface.is_some(){Part::SurfaceEdge(edge as usize)}else{let(a,b)=row.mesh.parts.edges[edge as usize];Part::MeshEdge(a,b)};
            let owner=crate::feature_owner::display_owner(scene,row.id).map_or(0,crate::scene::ObjectId::value);
            let tag=if scene.forest.derived(row.id){0}else if let Some(key)=key(row.id,part){*picks.get(&key).ok_or("Missing original production mesh edge address")?}else{crate::sub_id::OBJECT_INK};
            out.ids.push([owner,tag]);out.edges.push(edge);out.owners.push(row.id.value());crate::production_joined::segment_bytes(&segment,&mut out.segments);
        }
    }
    Ok(())
}
