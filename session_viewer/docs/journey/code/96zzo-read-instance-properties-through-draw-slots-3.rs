pub fn set(&mut self,ctx:&GpuCtx,rows:&[Slot])->bool{
  if self.buffer.len()>0&&self.previous==rows{return false;}