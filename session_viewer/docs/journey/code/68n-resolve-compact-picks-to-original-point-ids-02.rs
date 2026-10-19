
pub fn position(scene:&Scene,owner:ObjectId,id:u32)->Option<session_rust::Point> {
    if let Some(row)=scene.streams().iter().find(|r|r.id==owner&&r.controls_visible) {
        scene.pick_id(owner.value())?;let p=row.prepared.position(id)?;return Some(row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])));
    }
    let row=scene.clouds().iter().find(|r|r.id==owner && r.controls_visible)?;
    scene.pick_id(owner.value())?;
