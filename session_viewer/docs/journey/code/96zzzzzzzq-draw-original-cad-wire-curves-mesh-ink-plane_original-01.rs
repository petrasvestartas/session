    let points=points(plane)?;let edges=if crate::clipping_geometry::is_clipping(plane){vec![(0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(5,7)]}else{Vec::new()};
    let mut source=Mesh::from_vertices_and_faces(points.clone(),Vec::new());source.set_guid(plane.guid().into());source.name=plane.name.clone();source.set_objectcolor(plane.linecolor.clone());source.is_visible=plane.is_visible;source.is_locked=plane.is_locked;
    let mut display=crate::mesh::Mesh::new(points.iter().map(|p|[p[0] as f32,p[1] as f32,p[2] as f32,0.,0.,0.]).collect(),Vec::new())?;
    let endpoints=edges.iter().map(|&(a,b)|[&points[a],&points[b]].map(|p|[p[0],p[1],p[2]])).collect();display.parts=crate::mesh_parts::Parts{dots:Vec::new(),dot_styles:Vec::new(),glyph_flags:0,ink:Default::default(),dot_colours:Vec::new(),pens:Vec::new(),faces:Vec::new(),triangles:Vec::new(),edges,endpoints};
    Ok(crate::prepared::PreparedMesh{geometry:Rc::new(source),display,source:None,model:Xform::identity()})
}
pub fn placed(plane:&Plane,model:&Xform)->Option<crate::bounds::Bounds>{let points=points(plane).ok()?;let p=points.first()?.transformed(model);let mut bounds=crate::bounds::Bounds::point([p[0],p[1],p[2]]);for p in points{let p=p.transformed(model);bounds.include([p[0],p[1],p[2]])}Some(bounds)}
