}
pub fn model(world:&HashMap<String,Xform>,place:&Xform,guid:&str)->Xform {world.get(guid).cloned().unwrap_or_else(||place.clone())}

#[derive(Clone)]
pub struct Node {pub local:Xform,pub world:Xform,pub parent:Xform}
impl Default for Node {fn default()->Self {Self {local:Xform::identity(),world:Xform::identity(),parent:Xform::identity()}}}
// Preserve unchanged canonical local transforms exactly; inverse-derived locals are needed only after a row or its parent placement changes.
pub fn saved_local(original:&Node,parent:&Xform,world:&Xform)->Result<Xform,&'static str> {
    if original.parent.m==parent.m&&original.world.m==world.m {return Ok(original.local.clone());}
    Ok(&parent.inverse().ok_or("Parent placement cannot be inverted for Save")?*world)
}
