pub fn points(plane:&Plane)->Result<Vec<Point>,&'static str>{
    if !valid(plane){return Err("Invalid original plane frame")}
    let o=plane.origin();let x=plane.x_axis();let y=plane.y_axis();let z=x.cross(&y).normalized();
    if !crate::clipping_geometry::is_clipping(plane) {
        let mut rows=crate::production_plane::PlaneRows::default();
        let bounds=crate::production_plane::walk_plane(&mut rows,plane,0).bounds;
        let a=bounds.min_point();let b=bounds.max_point();
        return Ok((0..8).map(|i|Point::new(if i&1==0{a[0]}else{b[0]},if i&2==0{a[1]}else{b[1]},if i&4==0{a[2]}else{b[2]})).collect());
    }
    let arrow=0.2*x.magnitude().min(y.magnitude());let tip=&o+&z*arrow;let back=&o+&z*(arrow*0.7);let wing=x.normalized()*(arrow*0.25);
    Ok(vec![&o+&x+y.clone(),&o-&x+y.clone(),&o-&x-y.clone(),&o+&x-y,o.clone(),tip,&back+&wing,&back-&wing,&o-&z*arrow])
}
pub fn prepared(plane:&Plane)->Result<crate::prepared::PreparedMesh,&'static str>{
    let points=points(plane)?;let edges=if crate::clipping_geometry::is_clipping(plane){vec![(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(5,7)]}else{Vec::new()};
    let mut source=Mesh::from_vertices_and_faces(points.clone(),Vec::new());source.set_guid(plane.guid().into());source.name=plane.name.clone();source.set_objectcolor(plane.linecolor.clone());source.is_visible=plane.is_visible;source.is_locked=plane.is_locked;
    let mut display=crate::mesh::Mesh::new(points.iter().map(|p|[p[0] as f32,p[1] as f32,p[2] as f32,0.,0.,0.]).collect(),Vec::new())?;
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{dots:Vec::new(),dot_colours:Vec::new(),pens:Vec::new(),faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
