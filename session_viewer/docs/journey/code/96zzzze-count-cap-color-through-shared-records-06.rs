  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;