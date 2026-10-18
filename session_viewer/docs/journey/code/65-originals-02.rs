use crate::{pick_sources::{Part,Target},scene::{ObjectId,Scene}};

// Storage indices address current GPU columns; original IDs survive octree reordering and may use every bit of a u32.
pub fn resolve(scene:&Scene,owner:ObjectId,index:usize)->Option<Target> {
    let row=scene.clouds().iter().find(|r|r.id==owner && r.controls_visible)?;
    if scene.pick_id(owner.value()).is_none() || index>=row.prepared.source.point_count() {return None;}
    let target=Target {owner,part:Part::CloudPoint(row.prepared.source.point_id(index))};
    Some(target)
}

pub fn position(scene:&Scene,owner:ObjectId,id:u32)->Option<session_rust::Point> {
    let row=scene.clouds().iter().find(|r|r.id==owner && r.controls_visible)?;
    scene.pick_id(owner.value())?;
    let index=row.prepared.source.index_of_id(id)?;
    Some(row.model.transform_point(&row.prepared.source.get_point(index)))
}
