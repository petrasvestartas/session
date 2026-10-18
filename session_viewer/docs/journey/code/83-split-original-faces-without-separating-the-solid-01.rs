    pub fn new(scene: &Scene, id: ObjectId, face: Option<usize>) -> Result<Self, String> {
        let target = Owner::capture(scene, id).map_err(str::to_owned)?;
        let face = crate::split_geometry::face_index(&target.source, face)?;
        Ok(Self {
