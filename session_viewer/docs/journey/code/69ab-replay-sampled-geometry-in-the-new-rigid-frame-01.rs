use crate::{cad_shape_key::{ShapeKey,Frame,SAME_POINT},cad_display::Prepared,prepared::PreparedMesh,mesh::Mesh};
use session_rust::{BRep,Xform};
use std::{collections::HashMap,rc::Rc};

struct Recording {frame:Frame,size:f64,prepared:Prepared,mesh:Rc<Mesh>,proxy:Rc<session_rust::Mesh>}
#[derive(Default)]
pub struct Cache {records:HashMap<Vec<u64>,Vec<Recording>>,pub walks:usize,pub replays:usize}
impl Recording {
    fn replay(&self,source:Rc<BRep>,frame:&Frame)->Result<(Prepared,PreparedMesh),&'static str> {
        let motion=self.frame.motion_to(frame);let mut prepared=self.prepared.clone();prepared.source=source.clone();
        for edge in &mut prepared.edges {for p in &mut edge.points {*p=motion.point(*p);}}
        let vertices=self.mesh.vertices().iter().map(|v| {let p=motion.position([v[0],v[1],v[2]]);[p[0],p[1],p[2],v[3],v[4],v[5]]}).collect();let normals=self.mesh.normals().iter().map(|n|motion.normal(*n)).collect();
        let mut display=Mesh::new(vertices,self.mesh.indices().to_vec())?.with_normals(normals)?;display.parts=self.mesh.parts.clone();for edge in &mut display.parts.endpoints {for p in edge {*p=motion.point(*p);}}
        let columns=[motion.turn([1.,0.,0.]),motion.turn([0.,1.,0.]),motion.turn([0.,0.,1.])];let origin=motion.point([0.;3]);let matrix=Xform::from_matrix([columns[0][0],columns[0][1],columns[0][2],0.,columns[1][0],columns[1][1],columns[1][2],0.,columns[2][0],columns[2][1],columns[2][2],0.,origin[0],origin[1],origin[2],1.]);
        let mut proxy=(*self.proxy).clone();proxy.transform(&matrix);proxy.set_guid(source.guid().into());proxy.name=source.name.clone();proxy.is_visible=source.is_visible;proxy.is_locked=source.is_locked;
        Ok((prepared,PreparedMesh {geometry:Rc::new(proxy),display,source:None,model:Xform::identity()}))
    }
}
