    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.paths.set_chains(&self.device, scene.paths().iter().flat_map(crate::scene::PathObject::segments))
            .expect("Prepared path placement fits the display range");