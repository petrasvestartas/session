use session_rust::brep::BRepOrientation;
pub struct EdgeUse {pub face:usize,pub orientation:BRepOrientation,pub pcurve:usize,pub keys:Option<Vec<usize>>}

pub fn uses(prepared:&crate::cad_boundary::Prepared,edge:usize)->Result<Vec<EdgeUse>,&'static str> {
    if edge>=prepared.source.m_edges.len() {return Err("Missing original CAD edge");}
    prepared.source.edge_faces(edge).into_iter().map(|use_| {
        let face=usize::try_from(use_.index).map_err(|_|"Invalid incident face")?;let mesh=prepared.faces.get(face).ok_or("Missing incident face mesh")?;
        let pcurve=usize::try_from(prepared.source.pcurve_index(edge,face,use_.orientation)).map_err(|_|"Missing edge pcurve")?;
        let keys=crate::cad_edges::iso_chain(&prepared.source,mesh,&crate::cad_edges::EdgeUse {edge,face,orientation:use_.orientation});
        Ok(EdgeUse {face,orientation:use_.orientation,pcurve,keys})
    }).collect()
}
