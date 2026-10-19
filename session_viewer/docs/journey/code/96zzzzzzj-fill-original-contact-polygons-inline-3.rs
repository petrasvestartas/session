    crate::layer_edit::attach_features(scene, id, prepared?)
}

pub(crate) fn contact_fill(outline:&Polyline)->Result<Option<PreparedMesh>,&'static str> {
  if outline.coords.len()%3!=0 {return Err("Contact needs complete points");}
  let Some(normal)=crate::source_mesh_topology::polygon_normal(outline.coords.len()/3,|i| {
    let p=&outline.coords[i*3..];[p[0],p[1],p[2]]
  }) else {return Ok(None)};
  let points=outline.coords.chunks_exact(3).map(|p|Point::new(p[0],p[1],p[2])).collect();
  let mut mesh=Mesh::from_polylines(vec![points],None);
  mesh.set_objectcolor(Color::new(0.9,0.1,0.1,1.));
  let mut fill=PreparedMesh::new(mesh)?;let count=fill.display.vertices().len();
  fill.display=fill.display.with_normals(vec![normal.map(|v|v as f32);count])?.with_alphas(vec![1./255.;count])?;
  Ok(Some(fill))
}
