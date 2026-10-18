        if scene.pick_id(self.owner.value()).is_none() { return false; }
        match self.part {
            Part::SheetEntity(id)=>scene.sheets().iter().any(|row|row.id==self.owner&&row.has_entity(id)),
            Part::CloudPoint(id)=>crate::cloud_pick::position(scene,self.owner,id).is_some(),
            Part::Object => true,
