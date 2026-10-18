fn ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    visual_ancestor(scene,doc,index).and_then(|id|scene.pick_id(id.value()))
}

pub fn visual_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    if !scene.forest.derived(id) { return Some(id); }