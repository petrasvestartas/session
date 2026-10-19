use session_rust::{Geometry,Mesh,Plane,Point,Xform};
use std::rc::Rc;
pub fn valid(plane:&Plane)->bool{
    let o=plane.origin();let x=plane.x_axis();let y=plane.y_axis();
    (0..3).all(|i|o[i].is_finite()&&x[i].is_finite()&&y[i].is_finite())&&x.magnitude()>0.&&y.magnitude()>0.&&x.cross(&y).magnitude()>1e-12&&plane.width.is_finite()&&plane.width>=0.
}
pub fn points(plane:&Plane)->Result<Vec<Point>,&'static str>{
    if !valid(plane){return Err("Invalid original plane frame")}
    let o=plane.origin();let x=plane.x_axis();let y=plane.y_axis();let z=x.cross(&y).normalized();
    let arrow=0.2*x.magnitude().min(y.magnitude());let tip=&o+&z*arrow;let back=&o+&z*(arrow*0.7);let wing=x.normalized()*(arrow*0.25);
    Ok(vec![&o+&x+y.clone(),&o-&x+y.clone(),&o-&x-y.clone(),&o+&x-y,o.clone(),tip,&back+&wing,&back-&wing,&o-&z*arrow])
}
pub fn prepared(plane:&Plane)->Result<crate::prepared::PreparedMesh,&'static str>{
    let points=points(plane)?;let edges=vec![(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(5,7)];
    let mut source=Mesh::from_vertices_and_faces(points.clone(),Vec::new());source.set_guid(plane.guid().into());source.name=plane.name.clone();source.set_objectcolor(plane.linecolor.clone());source.is_visible=plane.is_visible;source.is_locked=plane.is_locked;
    let mut display=crate::mesh::Mesh::new(points.iter().map(|p|[p[0] as f32,p[1] as f32,p[2] as f32,0.,0.,0.]).collect(),Vec::new())?;
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
    Ok(crate::prepared::PreparedMesh{geometry:Rc::new(source),display,source:None,model:Xform::identity()})
}
pub fn placed(plane:&Plane,model:&Xform)->Option<crate::bounds::Bounds>{let points=points(plane).ok()?;let p=points.first()?.transformed(model);let mut bounds=crate::bounds::Bounds::point([p[0],p[1],p[2]]);for p in points{let p=p.transformed(model);bounds.include([p[0],p[1],p[2]])}Some(bounds)}
pub fn insert(scene:&mut crate::scene::Scene,plane:Rc<Plane>)->Result<crate::scene::ObjectId,&'static str>{let id=scene.insert(prepared(&plane)?)?;scene.objects_mut().iter_mut().find(|r|r.id==id).unwrap().original=Some(Geometry::Plane(plane));Ok(id)}
