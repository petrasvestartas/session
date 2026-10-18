#[test]
fn actual_slots_keep_integer_ids_sentinel_capacity_and_release(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut slots=crate::draw_slots::Slots::new(&ctx);
 let bytes=|rows:&[[u32;2]]|rows.iter().flatten().flat_map(|v|v.to_ne_bytes()).collect::<Vec<_>>();
 assert_eq!(slots.instances(),0);assert_eq!(slots.allocated_bytes(),8);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,8),bytes(&[[u32::MAX,0]]));
 slots.set(&ctx,&[[7,16_777_217],[9,u32::MAX-1]]);let capacity=slots.allocated_bytes();assert_eq!(slots.instances(),2);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,24),bytes(&[[u32::MAX,0],[7,16_777_217],[9,u32::MAX-1]]));
 slots.clear(&ctx,false);assert_eq!(slots.instances(),0);assert_eq!(slots.allocated_bytes(),capacity);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,8),bytes(&[[u32::MAX,0]]));
 slots.set(&ctx,&[[13,21]]);assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,16),bytes(&[[u32::MAX,0],[13,21]]));
 slots.clear(&ctx,true);assert_eq!(slots.instances(),0);assert_eq!(slots.allocated_bytes(),8);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,8),bytes(&[[u32::MAX,0]]));
}
