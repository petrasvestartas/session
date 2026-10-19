        self.trim=Some(trim);if finish{self.commit_trim()}else{Ok(Change::Preview)}
    }
    fn explode_selected(&mut self)->Result<Change,&'static str>{if self.selected_part.is_some(){return Err("Explode works on whole objects")}
        let plan=crate::explode_edit::Plan::prepare(&self.scene,&self.selected_objects()).map_err(|message|self.primitive_failure(message))?;let mut made=None;self.history.try_edit(&mut self.scene,|scene|{made=Some(plan.adopt(scene)?);Ok(())})?;self.select_objects(made.unwrap(),false);self.snap_index=None;Ok(Change::Scene)
    }
    fn curve_heads(&mut self,head:session_rust::Arrowhead)->Result<Change,&'static str>{
        let mut edits=Vec::new();let mut curves=0;
        for id in self.measured_ids(){let owner=crate::source_edit::Owner::capture(&self.scene,id)?;if let Some((next,changed))=crate::curve_heads::headed(&owner.source,head){curves+=1;if changed{edits.push((id,next))}}}
        if curves==0{return Err("Arrowhead needs a selected line, polyline or curve")}if edits.is_empty(){return Ok(Change::View)}
        self.history.try_edit(&mut self.scene,|scene|{for(id,source)in edits{scene.replace_original(id,source)?;}Ok(())})?;self.snap_index=None;Ok(Change::Scene)
    }
    fn begin_text(&mut self,words:String)->Result<Change,&'static str>{let text=crate::text_draft::Draft::new(words,self.camera.distance,self.last_text_height)?;self.apply(Action::BeginTool(crate::tool::Shape::Text))?;self.draft.as_mut().unwrap().text=Some(text);self.tool_status=self.draft.as_ref().unwrap().prompt();Ok(Change::View)}
    fn commit_text(&mut self,point:[f64;3])->Result<Change,&'static str>{let draft=self.draft.as_ref().ok_or("Start Text first")?;let text=draft.text.as_ref().ok_or("Start Text first")?;let prepared=crate::scene_label::Prepared::new(std::rc::Rc::new(text.source(point,draft.frame,&self.camera)))?;let height=text.height;self.history.try_edit(&mut self.scene,|scene|scene.insert_text(prepared).map(|_|()))?;self.last_text_height=height;self.draft=None;self.tool_status="Created original scene text. Undo removes it.".into();Ok(Change::Scene)}
