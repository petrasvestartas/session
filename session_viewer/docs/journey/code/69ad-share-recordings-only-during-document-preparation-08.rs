use std::{collections::HashMap,rc::Rc};

pub fn prepare(owner:&Owner,shapes:&mut crate::cad_shape_cache::Cache)->Result<Scene,&'static str> {
    let mut scene=Scene::empty();
    match &owner.definition {
        Geometry::Mesh(source)=>{scene.insert(crate::prepared::PreparedMesh::from_shared(source.clone())?)?;},
        Geometry::BRep(source)=>{scene.insert_prepared_cad(shapes.prepare(source.clone())?)?;},
        Geometry::NurbsSurface(source)=>{scene.insert_surface(source.clone())?;},
        Geometry::NurbsCurve(source)=>{scene.insert_path(crate::chain::PreparedChain::curve(source.clone())?)?;},
