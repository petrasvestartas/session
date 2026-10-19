
pub fn origin(scene:&Scene,target:Target) -> Option<[f64;3]> {
    if let Part::CloudPoint(id)=target.part {let p=crate::cloud_pick::position(scene,target.owner,id)?;return Some([p[0],p[1],p[2]]);}
    let points=points(scene,target).ok()?;
    if points.is_empty() {return None;}
