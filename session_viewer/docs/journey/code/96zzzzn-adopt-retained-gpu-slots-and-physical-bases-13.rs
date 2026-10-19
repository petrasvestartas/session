  self.written_bytes.set(self.written_bytes.get()+texels.len()as u64*16);
  let mut at=first;let mut rest=texels;