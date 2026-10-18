use session_rust::proto;

pub fn merge(unused:Option<&proto::Objects>,used:Option<proto::Objects>)->Result<Option<proto::Objects>,String>{
    let Some(original)=unused else{return Ok(used)};
    let mut out=used.unwrap_or_default();out.name=original.name.clone();out.guid=original.guid.clone();
    macro_rules! append{($($field:ident),*)=>{$(for source in &original.$field{
        if let Some(existing)=out.$field.iter().find(|v|v.guid==source.guid){if existing!=source{return Err("Different original definitions reuse a GUID".into());}}
        else{out.$field.push(source.clone());}
    }out.$field.sort_by(|a,b|a.guid.cmp(&b.guid));)*};}
    append!(points,lines,planes,bboxes,polylines,pointclouds,meshes,nurbscurves,nurbssurfaces,breps,elements,components,sheets,instances);
    Ok(Some(out))
}
