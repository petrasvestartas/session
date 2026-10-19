use crate::{instance_plan::Owner,scene::Scene};
use session_rust::Geometry;
use std::{collections::HashMap,rc::Rc};

pub fn prepare(owner:&Owner)->Result<Scene,&'static str> {
    let mut scene=Scene::empty();
    match &owner.definition {
        Geometry::Mesh(source)=>{scene.insert(crate::prepared::PreparedMesh::from_shared(source.clone())?)?;},
        Geometry::BRep(source)=>{scene.insert_cad(source.clone())?;},
        Geometry::NurbsSurface(source)=>{scene.insert_surface(source.clone())?;},
        Geometry::NurbsCurve(source)=>{scene.insert_path(crate::chain::PreparedChain::curve(source.clone())?)?;},
        Geometry::Polyline(source)=>{scene.insert_path(crate::chain::PreparedChain::polyline(source.clone())?)?;},
        Geometry::Line(source)=>{scene.insert_line(crate::stroke::PreparedLine::new(source.clone())?)?;},
        Geometry::Point(source)=>{scene.insert_point(crate::marker::PreparedPoint::new(source.clone())?)?;},
        Geometry::PointCloud(source)=>{scene.insert_cloud(crate::cloud::Prepared::new(source.clone())?)?;},
        _=>return Err("This definition family connects in a later subject"),
    }
    Ok(scene)
}

pub fn templates(document:Rc<session_rust::Session>)->Result<HashMap<String,Scene>,&'static str> {
    let mut templates=HashMap::new();
    for instance in &document.objects.instances {
        if !templates.contains_key(&instance.definition_guid) {
            let owner=Owner::new(document.clone(),instance.clone())?;templates.insert(instance.definition_guid.clone(),prepare(&owner)?);
        }
    }
    Ok(templates)
}

