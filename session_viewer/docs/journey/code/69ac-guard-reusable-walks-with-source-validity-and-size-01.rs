    }
}
impl Cache {
    // Rigid-frame keys normalize coordinates, so additionally compare physical size before accepting a reusable walk of another original BRep.
    pub fn prepare(&mut self,source:Rc<BRep>)->Result<(Prepared,PreparedMesh),&'static str> {
        crate::cad_boundary::validate(&source)?;let key=ShapeKey::of(&source);if let Some(key)=key.as_ref() {let size=key.step/SAME_POINT;if let Some(record)=self.records.get(&key.words).and_then(|records|records.iter().find(|r|(r.size-size).abs()<=SAME_POINT*r.size.max(size))) {self.replays+=1;return record.replay(source,&key.frame);}}
        self.walks+=1;let (prepared,mesh)=crate::cad_display::prepare(source)?;
        if let Some(key)=key {self.records.entry(key.words).or_default().push(Recording {frame:key.frame,size:key.step/SAME_POINT,prepared:prepared.clone(),mesh:Rc::new(mesh.display.clone()),proxy:mesh.geometry.clone()});}
        Ok((prepared,mesh))
    }
    pub fn recordings(&self)->usize {self.records.values().map(Vec::len).sum()}
}
