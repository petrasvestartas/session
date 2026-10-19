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

#[cfg(test)]
mod tests {
 use super::*;
#[test]
fn shared_object_rows_keep_neighbours_growth_and_release() {
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let layout=gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:None,entries:&[wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Storage {read_only:true},has_dynamic_offset:false,min_binding_size:std::num::NonZeroU64::new(crate::object_record::BYTES as u64)},count:None}]});
 let mut table=Rows::new(&ctx,&layout);let mut rows=vec![[7;BYTES],[9;BYTES],[11;BYTES]];
 let check=|table:&Rows,rows:&[Record]|{let bytes=rows.iter().flatten().copied().collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,&table.buffer.buf,bytes.len()as u64),bytes);};
 table.set(&ctx,&layout,&rows).unwrap();check(&table,&rows);let buffer=table.buffer.buf.clone();let group=table.group.clone();
 rows[1][68..72].copy_from_slice(&0x80000001u32.to_ne_bytes());table.set(&ctx,&layout,&rows).unwrap();assert_eq!(table.written_bytes,BYTES as u64);assert_eq!(table.buffer.buf,buffer);assert_eq!(table.group,group);check(&table,&rows);
 assert!(!table.set(&ctx,&layout,&rows).unwrap());assert_eq!(table.written_bytes,0);
 rows.extend([[13;BYTES],[17;BYTES],[19;BYTES],[23;BYTES]]);table.set(&ctx,&layout,&rows).unwrap();assert_ne!(table.buffer.buf,buffer);assert_ne!(table.group,group);check(&table,&rows);let capacity=table.allocated_bytes();
 rows.truncate(2);table.set(&ctx,&layout,&rows).unwrap();assert_eq!(table.allocated_bytes(),capacity);check(&table,&rows);
 table.set(&ctx,&layout,&[]).unwrap();assert_eq!(table.len(),0);assert_eq!(table.allocated_bytes(),0);assert_eq!(table.previous.capacity(),0);assert_eq!(table.buffer.buf.size(),BYTES as u64);
 table.set(&ctx,&layout,&rows).unwrap();check(&table,&rows);
}
}
