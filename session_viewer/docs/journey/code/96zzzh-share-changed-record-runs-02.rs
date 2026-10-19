  let inside=self.previous.len().min(rows.len());
  for run in changed_runs(&self.previous,rows) {
   self.buffer.write_bytes_at(ctx,(run.start+1)as u32,&bytes(&rows[run.clone()]));
   self.written_bytes+=run.len()as u64*8;
  }