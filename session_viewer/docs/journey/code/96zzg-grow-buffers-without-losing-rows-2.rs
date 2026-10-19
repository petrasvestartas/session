use crate::text_gpu_support::{GpuCtx,GrowBuf};
fn read(ctx:&GpuCtx,buffer:&wgpu::Buffer,size:u64)->Vec<u8>{
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor{label:Some("growth readback"),size,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
 let mut encoder=ctx.device.create_command_encoder(&Default::default());encoder.copy_buffer_to_buffer(buffer,0,&output,0,size);ctx.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();output.slice(..).map_async(wgpu::MapMode::Read,move|r|tx.send(r).unwrap());ctx.device.poll(wgpu::PollType::Wait{submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let bytes=output.slice(..).get_mapped_range().to_vec();output.unmap();bytes
}
#[test]
fn actual_gpu_growth_preserves_packed_records_and_reset_reuses_capacity(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);
 let mut buffer=GrowBuf::new(&ctx,"packed arena preparation",20,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::VERTEX);
 let records=[vec![11;20],vec![22;40],vec![33;60]];let mut wanted=Vec::new();
 for record in records {let before=buffer.len();let replaced=buffer.append_bytes(&ctx,&record);if before>0{assert!(replaced);}wanted.extend(record);assert_eq!(read(&ctx,&buffer.buf,wanted.len()as u64),wanted);}
 assert_eq!(buffer.len(),6);let size=buffer.buf.size();assert!(!buffer.append_bytes(&ctx,&[]));assert_eq!(buffer.len(),6);
 buffer.reset();assert!(!buffer.append_bytes(&ctx,&[44;20]));assert_eq!(buffer.buf.size(),size);assert_eq!(read(&ctx,&buffer.buf,20),[44;20]);
 buffer.release(&ctx);assert!(buffer.is_empty());assert_eq!(buffer.buf.size(),20);
 let mut floats=GrowBuf::new(&ctx,"existing float adapter",8,wgpu::BufferUsages::VERTEX);floats.append(&ctx,&[[1.,2.]]);floats.append(&ctx,&[[3.,4.]]);
 assert_eq!(read(&ctx,&floats.buf,16),[1f32,2.,3.,4.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());
}
