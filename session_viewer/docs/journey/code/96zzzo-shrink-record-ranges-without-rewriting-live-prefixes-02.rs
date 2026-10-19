  if rows.len()<self.previous.len(){self.buffer.truncate(rows.len()as u32);}
  for run in crate::draw_slots::changed_runs(&self.previous,rows) {
   let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
  }
  let tail=bytes(&rows[self.previous.len().min(rows.len())..]);
  if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}