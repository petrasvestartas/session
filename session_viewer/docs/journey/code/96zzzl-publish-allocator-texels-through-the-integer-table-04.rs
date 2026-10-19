use crate::{primitive_slot_table::Table,text_gpu_support::GpuCtx};
fn read(ctx:&GpuCtx,table:&Table)->Vec<[u32;4]>{
 let size=table.texture.size();let pitch=(size.width*16+255)/256*256;let bytes=u64::from(pitch)*u64::from(size.height);
 let output=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some("slot texture readback"),size:bytes,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});let mut encoder=ctx.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(table.texture.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&output,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(size.height)}},size);ctx.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();output.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());ctx.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();rx.recv().unwrap().unwrap();let mapped=output.slice(..).get_mapped_range();let result=mapped.chunks_exact(pitch as usize).flat_map(|row|row[..size.width as usize*16].chunks_exact(16).map(|texel|std::array::from_fn(|i|u32::from_le_bytes(texel[i*4..i*4+4].try_into().unwrap())))).collect();drop(mapped);output.unmap();result
}
#[test]
fn actual_integer_slot_texture_preserves_ids_across_partial_row_writes_and_release(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
 let mut wanted=(0..1050).map(|i|[i,0x80000000+i,16_777_217+i,3]).collect::<Vec<_>>();table.write(&ctx,&wanted);assert_eq!([table.texture.width(),table.texture.height()],[1024,2]);assert_eq!(table.texture.format(),wgpu::TextureFormat::Rgba32Uint);let texture=table.texture.clone();wanted.resize(2048,[0;4]);assert_eq!(read(&ctx,&table),wanted);
 let changed=[[7,u32::MAX-1,31,9],[11,0x80000000,37,12],[13,16_777_217,41,15]];assert!(table.write_at(&ctx,1023,&changed));wanted[1023..1026].copy_from_slice(&changed);assert_eq!(table.texture,texture);assert_eq!(read(&ctx,&table),wanted);
 assert!(!table.write_at(&ctx,u32::MAX,&changed));assert!(!table.write_at(&ctx,2047,&changed));assert!(table.write_at(&ctx,2048,&[]));assert_eq!(read(&ctx,&table),wanted);
 table.write(&ctx,&[]);assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);assert!(!table.write_at(&ctx,0,&changed));assert_eq!(read(&ctx,&table),vec![[0;4]]);
}

#[test]
fn actual_allocator_publication_keeps_high_rows_free_slots_neighbours_and_release() {
 use crate::slot_space::{Definition,SlotSpace};
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=GpuCtx::new(&gpu.device,&gpu.queue);let mut table=Table::new(&ctx);let mut space=SlotSpace::default();
 let mut a=[7,0x80000001];let b=[u32::MAX-1];
 fn definitions<'a>(a:&'a[u32],b:&'a[u32])->Vec<Definition<'a>>{vec![Definition {key:1,rows:a,faces:30..36,pipes:0..0,ribbons:0..0},Definition {key:2,rows:b,faces:60..63,pipes:0..0,ribbons:0..0}]}
 let change=space.reset(&definitions(&a,&b),100);assert!(table.publish(&ctx,&space,&change));let texture=table.texture.clone();let bytes=read(&ctx,&table);assert_eq!(&bytes[..4],&[[3,100,0,0],[7,100,10,2],[0x80000001,102,10,2],[u32::MAX-1,104,20,1]]);
 a[0]=0x80000007;let change=space.update(&definitions(&a,&b),&[(1,0)],100);assert!(!change.all);assert!(!table.publish(&ctx,&space,&change));assert_eq!(table.texture,texture);let next=read(&ctx,&table);assert_eq!(next[1],[0x80000007,100,10,2]);assert_eq!(&next[2..],&bytes[2..]);
 let remaining=[Definition {key:2,rows:&b,faces:60..63,pipes:0..0,ribbons:0..0}];let change=space.update(&remaining,&[],100);table.publish(&ctx,&space,&change);let next=read(&ctx,&table);assert_eq!(next[1][3],0);assert_eq!(next[2][3],0);assert_eq!(next[3],[u32::MAX-1,104,20,1]);
 let change=space.reset(&[],0);assert!(table.publish(&ctx,&space,&change));assert_eq!(table.allocated_bytes(),16);assert_eq!(read(&ctx,&table),vec![[0;4]]);
}
