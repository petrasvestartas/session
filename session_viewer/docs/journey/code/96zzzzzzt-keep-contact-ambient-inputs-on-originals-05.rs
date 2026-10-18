pub fn display_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    visual_owner(scene,id).and_then(|id|scene.pick_id(id.value()))
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {