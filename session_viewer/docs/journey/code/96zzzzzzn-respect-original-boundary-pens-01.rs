use crate::source_mesh_topology::{SlotMap,mesh_topology};
use session_rust::Mesh;
pub type Pen=(f32,u32);

pub fn width(value:f64)->f32 {
    let value=value as f32;
    if value.is_finite()&&value>0. {value} else {1.}
}

pub fn prepare(source:&Mesh,count:usize)->(Vec<(usize,usize)>,Vec<Pen>) {
    if count>200_000||source.widths()==[0.] {return (Vec::new(),Vec::new());}
    let keys=source.vertices();
    let points=keys.iter().map(|key|{let p=&source.vertex[key];[p.x,p.y,p.z]}).collect::<Vec<_>>();
    let topology=mesh_topology(source,&keys,&points,&SlotMap::new(&keys));
    let mut pens=Vec::with_capacity(topology.edges.len());
    for (i,&(_,_,colour)) in topology.edges.iter().enumerate() {
        let widths=source.widths();
        let value=if widths.len()==1 {widths[0]} else {widths.get(i).copied().unwrap_or(1.)};
        let pair=topology.edge_faces[i].map(|face|topology.normals.get(face as usize).copied().flatten());
        let flat=match pair {
            [Some(a),Some(b)]=>a.iter().zip(b).map(|(a,b)|a*b).sum::<f64>()*if topology.opposed[i]{1.}else{-1.}>=1.-1e-9,
            _=>false,
        };
        pens.push((if value==0.||flat {0.} else {width(value)},if topology.edges.len()>=10_000 {crate::source_topology_colour::BLACK} else {colour}));
    }
    if pens.first().is_some_and(|first|pens.iter().all(|pen|pen==first)) {pens=vec![pens[0]];}
    (topology.edges.into_iter().map(|(a,b,_)|(a,b)).collect(),pens)
}
