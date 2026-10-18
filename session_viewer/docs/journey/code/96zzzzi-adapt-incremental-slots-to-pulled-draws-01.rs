  pub fn local_slot(&self,slot:u32)->Slot {let texel=self.texel(slot);[texel[0],texel[1]]}
  pub fn touched(&self,definitions:&[Definition])->Vec<(u32,u32)> {
    let mut out=Vec::new();for definition in definitions {let Some(run)=self.runs.iter().find(|run|run.key==Some(definition.key))else{continue};for(position,row)in definition.rows.iter().enumerate(){if position<run.cap as usize&&self.rows.get(run.start as usize+position)!=Some(row){out.push((definition.key,position as u32));}}}out
  }
  pub fn texel(&self, slot: u32) -> [u32; 4] {