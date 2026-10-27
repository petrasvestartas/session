        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
        for row in &mut self.streamed {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn matrices(&self)->&[crate::production_matrix::Row] {&self.matrices}
    pub fn insert_matrix(&mut self,source:Rc<session_rust::Component>)->Result<ObjectId,&'static str> {
        crate::production_matrix::validate(&source)?;
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);
        let mut guid=source.guid.clone();let used=self.used_guids();while used.contains(&guid){guid=uuid::Uuid::new_v4().to_string();}
        let name=source.name.clone();self.matrices.push(crate::production_matrix::Row{id,guid:guid.clone(),source,model:session_rust::Xform::identity()});
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_matrix(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m){return Err("Invalid matrix placement");}
        let row=self.matrices.iter_mut().find(|r|r.id==id).ok_or("Matrix is gone")?;let mut candidate=row.clone();candidate.model=model;
        let bounds=candidate.bounds();if bounds.min.into_iter().chain(bounds.max).any(|v|!(v as f32).is_finite()){return Err("Matrix placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn sheets(&self)->&[crate::sheet_source::Row] {&self.sheets}
    pub fn insert_sheet(&mut self,prepared:Rc<crate::sheet_source::Prepared>)->Result<ObjectId,&'static str> {
