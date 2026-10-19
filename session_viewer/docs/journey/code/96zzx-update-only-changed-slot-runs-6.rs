(slots.instances(),0);assert_eq!(slots.allocated_bytes(),8);
 assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,8),bytes(&[[u32::MAX,0]]));
}

#[test]
fn actual_partial_slot_writes_preserve_neighbours_growth_and_sentinel(){
 let gpu=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut slots=crate::draw_slots::Slots::new(&ctx);
 let mut rows=vec![[7,16_777_217],[9,u32::MAX-1],[11,33],[13,44]];slots.set(&ctx,&rows);let capacity=slots.allocated_bytes();let buffer=slots.buffer.buf.clone();
 let check=|slots:&crate::draw_slots::Slots,rows:&[[u32;2]]|{let wanted=std::iter::once([u32::MAX,0]).chain(rows.iter().copied()).flatten().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,wanted.len()as u64),wanted);};
 rows[1]=[23,0x80000000];assert!(slots.set(&ctx,&rows));assert_eq!(slots.written_bytes,8);assert_eq!(slots.allocated_bytes(),capacity);assert_eq!(slots.buffer.buf,buffer);check(&slots,&rows);
 assert!(!slots.set(&ctx,&rows));assert_eq!(slots.written_bytes,0);check(&slots,&rows);
 rows[0]=[31,51];rows[2]=[37,61];rows[3]=[41,71];slots.set(&ctx,&rows);assert_eq!(slots.written_bytes,24);check(&slots,&rows);
 rows.extend([[43,81],[47,91],[53,101]]);slots.set(&ctx,&rows);assert_eq!(slots.written_bytes,24);check(&slots,&rows);
 rows.truncate(2);let capacity=slots.allocated_bytes();slots.set(&ctx,&rows);assert_eq!(slots.written_bytes,24);assert_eq!(slots.allocated_bytes(),capacity);check(&slots,&rows);
 slots.clear(&ctx,true);assert_eq!(slots.written_bytes,8);assert_eq!(slots.allocated_bytes(),8);check(&slots,&[]);
 let before=crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,8);let invalid=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||slots.buffer.write_bytes_at(&ctx,1,&[0;8])));assert!(invalid.is_err());assert_eq!(crate::growing_buffer_tests::read(&ctx,&slots.buffer.buf,8),before);
}
