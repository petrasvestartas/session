pub struct Sample {pub key:usize,pub surface:usize,pub uv:[f64;2],pub point:[f64;3],pub normal:[f64;3]}

pub fn face(prepared:&crate::cad_boundary::Prepared,index:usize)->Result<Vec<Sample>,&'static str> {
    let original=prepared.source.m_faces.get(index).ok_or("Missing original CAD face")?;
    let surface=prepared.source.m_surfaces.get(original.surface_index as usize).ok_or("Missing original face surface")?;
    let mesh=prepared.faces.get(index).ok_or("Missing prepared face mesh")?;let mut rows=mesh.vertex.iter().collect::<Vec<_>>();rows.sort_unstable_by_key(|(key,_)|**key);
    let mut out=Vec::new();
    for (&key,vertex) in rows {
        let (Some(&u),Some(&v))=(vertex.attributes.get("u"),vertex.attributes.get("v")) else {continue;};
        let point=surface.point_at(u,v).ok_or("Face UV sample cannot be evaluated")?;let normal=surface.normal_at(u,v);let point=[point[0],point[1],point[2]];let normal=[normal[0],normal[1],normal[2]];
        if point.iter().chain(&normal).any(|v|!v.is_finite()) {return Err("Face samples must be finite");}
        out.push(Sample {key,surface:original.surface_index as usize,uv:[u,v],point,normal});
    }
    Ok(out)
}
