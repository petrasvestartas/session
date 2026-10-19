            if scene.pick_id(id.value())!=Some(*id) { return Err("Move needs visible unlocked objects"); }
            if scene.objects().iter().any(|r| r.id==*id && r.geometry().is_none()) { return Err("Reload editable sources before Move"); }
        }
        let roots=ids;let ids=scene.with_descendants(&roots);
        Ok(Self { base:scene.clone(),scene:scene.clone(),ids,roots,transform:Xform::identity() })
    }
    pub fn ids(&self) -> &[ObjectId] { &self.ids }
    pub fn scenes(&self)->[&Scene;2] {[&self.base,&self.scene]}
