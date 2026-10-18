use session_rust::Xform;

// A placement preview shares original geometry, but owns temporary row placements outside the saved document and its history.
pub struct Preview { base: Scene, pub scene: Scene, ids: Vec<ObjectId>, roots:Vec<ObjectId>, transform: Xform }

impl Preview {
    pub fn new(scene: &Scene, mut ids: Vec<ObjectId>) -> Result<Self, &'static str> {
