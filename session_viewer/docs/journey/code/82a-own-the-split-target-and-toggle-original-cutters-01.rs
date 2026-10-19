use crate::{
    scene::{ObjectId, Scene},
    source_edit::Owner,
};
use session_rust::{Geometry, Xform};
#[derive(Clone)]
pub struct Draft {
    pub target: Owner,
    pub face: Option<usize>,
    pub cutters: Vec<Owner>,
}
impl Draft {
    pub fn new(scene: &Scene, id: ObjectId, face: Option<usize>) -> Result<Self, String> {
        let target = Owner::capture(scene, id).map_err(str::to_owned)?;
        if !crate::split_geometry::is_cutter(&target.source) {
            return Err("Select a curve; surface-face splitting connects next".into());
        }
        let face = crate::split_geometry::face_index(&target.source, face)?;
        Ok(Self {
            target,
            face,
            cutters: Vec::new(),
        })
    }
    pub fn pick(&mut self, scene: &Scene, id: Option<ObjectId>) -> Result<(), String> {
        let id = id.ok_or("Choose a cutter curve")?;
        if id == self.target.id {
            return Err("Choose a cutter distinct from the target".into());
        }
        let owner = Owner::capture(scene, id).map_err(str::to_owned)?;
        if !crate::split_geometry::is_cutter(&owner.source) {
            return Err("Choose a line, polyline or NURBS curve as cutter".into());
        }
        if let Some(at) = self.cutters.iter().position(|c| c.id == id) {
            self.cutters.remove(at);
        } else if self.cutters.len() < 64 {
            self.cutters.push(owner);
        }
        Ok(())
    }
}
