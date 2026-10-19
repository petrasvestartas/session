use crate::{object_record::{BYTES,Record},text_gpu_support::{GpuCtx,GrowBuf}};
pub struct Rows {
 pub(crate) buffer:GrowBuf,
 group:wgpu::BindGroup,
 previous:Vec<Record>,
 pub written_bytes:u64,
}
impl Rows {
 pub fn new(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout)->Self {
  let buffer=GrowBuf::new(ctx,"shared object records",BYTES as u64,wgpu::BufferUsages::STORAGE);
  let group=crate::gpu_layout::group(&ctx.device,layout,&buffer.buf);
  Self {buffer,group,previous:Vec::new(),written_bytes:0}
 }
 pub fn set(&mut self,ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,rows:&[Record])->Result<bool,&'static str> {
  let limits=ctx.device.limits();let limit=limits.max_buffer_size.min(u64::from(limits.max_storage_buffer_binding_size));
  if rows.len()as u64>limit/BYTES as u64||u32::try_from(rows.len()).is_err(){return Err("Object records exceed device limits");}
  self.written_bytes=0;
  if rows.is_empty() {
   if !self.previous.is_empty(){self.buffer.release(ctx);self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);self.previous=Vec::new();}
   return Ok(false);
  }
  let bytes=|rows:&[Record]|rows.iter().flatten().copied().collect::<Vec<_>>();
  let mut grew=false;
  if rows.len()<self.previous.len() {
   self.buffer.reset();let data=bytes(rows);grew=self.buffer.append_bytes(ctx,&data);self.written_bytes=data.len()as u64;
  }else {
   for run in crate::draw_slots::changed_runs(&self.previous,rows) {
    let data=bytes(&rows[run.clone()]);self.buffer.write_bytes_at(ctx,run.start as u32,&data);self.written_bytes+=data.len()as u64;
   }
   let tail=bytes(&rows[self.previous.len()..]);
   if !tail.is_empty(){grew=self.buffer.append_bytes(ctx,&tail);self.written_bytes+=tail.len()as u64;}
  }
  if grew{self.group=crate::gpu_layout::group(&ctx.device,layout,&self.buffer.buf);}
  self.previous.clear();self.previous.extend_from_slice(rows);Ok(self.written_bytes>0)
 }
 pub fn bind(&self,pass:&mut wgpu::RenderPass<'_>){pass.set_bind_group(1,&self.group,&[]);}
 pub fn len(&self)->u32{self.previous.len()as u32}
 pub fn allocated_bytes(&self)->u64{if self.previous.is_empty(){0}else{self.buffer.buf.size()}}
}
