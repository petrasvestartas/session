// Storage indices address current GPU columns; original IDs survive octree reordering and may use every bit of a u32.
pub fn resolve(scene:&Scene,owner:ObjectId,index:usize)->Option<Target> {
    if let Some(row)=scene.streams().iter().find(|r|r.id==owner&&r.controls_visible) {
        scene.pick_id(owner.value())?;let mut address=0usize;
        for page in row.prepared.pages() {let count=page.value.ids.len();if index>=address&&index-address<count {return Some(Target {owner,part:Part::CloudPoint(page.value.ids[index-address])});}address=address.checked_add(count)?;}return None;
    }
    let row=scene.clouds().iter().find(|r|r.id==owner && r.controls_visible)?;
    if scene.pick_id(owner.value()).is_none() || index>=row.prepared.source.point_count() {return None;}
