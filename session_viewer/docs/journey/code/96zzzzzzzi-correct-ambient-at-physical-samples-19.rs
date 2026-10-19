tes+=1;
   if self.pipes.samples==4 {encoder.clear_buffer(&state.groups.buffers[1],0,None);encoder.clear_buffer(&state.groups.buffers[3],4,Some(4));}
   let i