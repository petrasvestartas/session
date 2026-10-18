  self.buffer.reset();self.buffer.append_bytes(ctx,&bytes);
  self.previous.clear();self.previous.extend_from_slice(rows);true