use session_rust::proto;
use std::collections::HashSet;

pub fn components(source:&proto::Session)->Option<proto::Objects>{
    let objects=source.objects.as_ref()?;
    (!objects.components.is_empty()).then(||proto::Objects{name:objects.name.clone(),guid:objects.guid.clone(),components:objects.components.clone(),..Default::default()})
}

pub fn unused_definitions(source:&proto::Session)->Option<proto::Objects>{
    let definitions=source.definitions.as_ref()?;
    let used=source.objects.as_ref().map(|o|o.instances.iter().map(|i|i.definition_guid.as_str()).collect::<HashSet<_>>()).unwrap_or_default();
    let mut out=proto::Objects{name:definitions.name.clone(),guid:definitions.guid.clone(),..Default::default()};let mut count=0;
    macro_rules! keep{($($field:ident),*)=>{$(out.$field=definitions.$field.iter().filter(|v|!used.contains(v.guid.as_str())).cloned().collect();count+=out.$field.len();)*};}
    keep!(points,lines,planes,bboxes,polylines,pointclouds,meshes,nurbscurves,nurbssurfaces,breps,elements,components,sheets,instances);
    (count>0).then_some(out)
}

pub fn raw_plane(plane:&proto::Plane)->Result<(),String>{
    if plane.frame.len()!=12||plane.frame.iter().any(|v|!v.is_finite())||!plane.width.is_finite()||plane.width<0.{return Err("Invalid original plane frame".into());}
    if !crate::plane_original::valid(&session_rust::Plane::from_proto(plane.clone())){return Err("Invalid original plane frame".into());}
    Ok(())
}

pub fn usage<'a>(scenes:impl IntoIterator<Item=&'a crate::scene::Scene>)->[usize;2]{
    use prost::Message;
    let mut origins=HashSet::new();let mut usage=[0,0];
    for scene in scenes{for doc in &scene.forest.documents{if let Some(origin)=doc.header.as_ref(){if origins.insert(std::rc::Rc::as_ptr(origin)){usage[0]+=1;usage[1]+=origin.header.encoded_len();}}}}
    usage
}
