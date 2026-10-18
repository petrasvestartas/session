        Ok(Self { base:scene.clone(),scene:scene.clone(),ids,transform:Xform::identity() })
    }
    pub fn ids(&self) -> &[ObjectId] { &self.ids }
    pub fn scenes(&self)->[&Scene;2] {[&self.base,&self.scene]}
    pub fn matrix(&self) -> &[f64; 16] { &self.transform.m }
    pub fn set(&mut self, transform: Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&transform.m) { return Err("Invalid preview transform"); }
