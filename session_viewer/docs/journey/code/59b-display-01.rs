use std::rc::Rc;
use session_rust::Xform;

pub struct Boundary {pub edge:usize,pub points:Vec<[f64;3]>}
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
