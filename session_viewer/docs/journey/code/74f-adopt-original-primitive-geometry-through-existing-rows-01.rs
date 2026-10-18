use crate::scene::{Scene,ObjectId};
use session_rust::Geometry;

pub fn insert(scene:&mut Scene,geometry:Geometry)->Result<ObjectId,&'static str> {
    match geometry {
        Geometry::BRep(source)=>scene.insert_cad(source),
        Geometry::Mesh(source)=>scene.insert(crate::prepared::PreparedMesh::new((*source).clone())?),
        Geometry::NurbsCurve(source)=>scene.insert_path(crate::chain::PreparedChain::curve(source)?),
        _=>Err("This primitive geometry is not supported"),
    }
}
