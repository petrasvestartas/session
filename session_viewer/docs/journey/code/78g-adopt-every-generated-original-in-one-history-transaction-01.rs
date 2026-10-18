        let draft=self.draft.as_ref().ok_or("No transform tool is running")?;let mut next=draft.transform.clone().ok_or("No transform tool is running")?;
        let outcome=next.text(text,draft.cursor,self.construction_plane).map_err(|message|self.primitive_failure(message))?;self.transform_outcome(next,outcome)
    }
    pub fn highlighted_objects(&self)->Vec<ObjectId>{self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||self.selected_objects(),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
    pub fn gathering_curves(&self)->bool{self.draft.as_ref().is_some_and(|d|d.plane_points.is_none()&&d.gather.as_ref().is_some_and(|g|g.picks()))}
    fn gather_adopt(&mut self,next:crate::gather_draft::Draft,made:Option<crate::gather_input::Made>)->Result<Change,&'static str>{
        next.valid(&self.scene).map_err(|message|self.primitive_failure(message))?;
        if let Some(made)=made{let mut ids=Vec::new();self.history.try_edit(&mut self.scene,|scene|{ids=made.geometries.into_iter().map(|geometry|crate::primitive_geometry::insert(scene,geometry)).collect::<Result<Vec<_>,_>>()?;Ok(())})?;self.select_objects(ids,false);self.draft=None;self.tool_status=made.message;return Ok(Change::Scene)}
        let draft=self.draft.as_mut().ok_or("No gathering tool is running")?;draft.points=next.answers.iter().filter_map(|answer|if let crate::gather_input::Answer::Point(p)=answer{Some([p[0],p[1],p[2]])}else{None}).collect();draft.gather=Some(next);draft.cursor=None;draft.snapped=None;self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
