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
            if !edge.degenerated&&!source.edge_faces(index).is_empty()&&chain.is_none()&&edge.curve_3d_index<0 {return Err("A CAD edge has no coherent face boundary");}
        }
        Ok(Self {source,faces,chains})
    }
    pub fn points(&self,edge:usize)->Option<Vec<[f64;3]>> {
        let chain=self.chains.get(edge)?.as_ref()?;let face=self.faces.get(chain.face)?;
        chain.keys.iter().map(|key|face.vertex.get(key).map(|v|[v.x,v.y,v.z])).collect()
    }
}

pub fn validate(source:&BRep)->Result<(),&'static str> {
        let b=&source;
        let index=|i:i32,n:usize|i>=0&&(i as usize)<n;
        let wire=b.m_faces.is_empty()&&b.m_shells.is_empty()&&b.m_solids.is_empty()&&!b.m_edges.is_empty()&&b.m_edges.iter().all(|e|index(e.start_vertex,b.m_vertices.len())&&index(e.end_vertex,b.m_vertices.len())&&(e.degenerated||index(e.curve_3d_index,b.m_curves_3d.len()))&&e.pcurves.iter().all(|p|index(p.surface_index,b.m_surfaces.len())&&index(p.curve_2d_index,b.m_curves_2d.len())&&(p.curve_2d_index_2<0||index(p.curve_2d_index_2,b.m_curves_2d.len()))))&&b.m_wires.iter().all(|w|!w.edges.is_empty()&&w.edges.iter().all(|r|index(r.index,b.m_edges.len())));
        if !b.is_valid()&&!wire {return Err("BRep topology contains an invalid reference");}
        if source.m_faces.len()>65_536||source.m_edges.len()>262_144 {return Err("BRep topology exceeds the display budget");}
        for surface in &source.m_surfaces {crate::surface::validate(surface)?;}
        for curve in source.m_curves_3d.iter().chain(&source.m_curves_2d) {crate::curve::validate(curve)?;}
    Ok(())
}
