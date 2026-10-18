use crate::{
    scene::{ObjectId, Scene},
    source_edit::Owner,
};
use session_rust::{Geometry, Plane, Point};
#[derive(Clone)]
pub struct Draft {
    pub targets: Vec<Owner>,
    pub unsupported: usize,
    pub before: Vec<ObjectId>,
    pub points: Vec<[f64; 3]>,
}
impl Draft {
    pub fn new(scene: &Scene, before: Vec<ObjectId>) -> Result<Self, &'static str> {
        if before.is_empty() {
            return Err("Select original objects to project");
        }
        let mut targets = Vec::new();
        let mut unsupported = 0;
        for id in &before {
            if scene.pick_id(id.value()) != Some(*id) {
                return Err("Project visible unlocked originals");
            }
            if crate::source_edit::original(scene, *id).is_some() {
                targets.push(Owner::capture(scene, *id)?)
            } else {
                unsupported += 1
            }
        }
        Ok(Self {
            targets,
            unsupported,
            before,
            points: Vec::new(),
        })
    }
}
