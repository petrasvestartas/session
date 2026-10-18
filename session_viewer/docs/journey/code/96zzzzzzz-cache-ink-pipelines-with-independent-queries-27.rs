use super::*;
#[test]
fn actual_colour_variants_reuse_pipelines_and_keep_fresh_queries_single_sample() {
 let renderer=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let device=&renderer.device;
 device.on_uncaptured_error(std::sync::Arc::new(|e|panic!("sample-aware ink: {e}")));
 let buffer=|size|device.create_buffer(&wgpu::BufferDescriptor {label:Some("actual ink binding witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});
 let projected=buffer(96);let tiles=buffer(32);
 let one=crate::frame_targets::Targets::new(device,[16,16],wgpu::TextureFormat::Rgba8Unorm,1);
 let four=crate::frame_targets::Targets::new(device,[16,16],wgpu::TextureFormat::Rgba8Unorm,4);
 let table=|target:&crate::frame_targets::Targets|Tables {projected:projected.clone(),tiles:tiles.clone(),primitive:target.primitive.view.clone()};
 for (stride,grid,varying) in [(44,false,false),(72,false,false),(32,false,false),(44,true,false),(44,false,true),(32,false,true)] {
  let mut ink=Ink::new(device,wgpu::TextureFormat::Rgba8Unorm,stride,grid,varying);let first=ink.color.active.0.clone();let query=ink.pick.clone();
  assert!(ink.color.alternate.is_none());ink.set(device,&table(&one),&one.depth);assert!(ink.bindings.borrow().is_some());
  ink.set_samples(device,4);assert!(ink.bindings.borrow().is_none());let second=ink.color.active.0.clone();assert_ne!(second,first);assert_eq!(ink.pick,query);
  ink.set(device,&table(&four),&four.depth);assert!(ink.bindings.borrow().is_some());
  let _query=ink.group(device,&ink.query,&table(&one),&one.depth);
  ink.set_samples(device,4);assert_eq!(ink.color.active.0,second);assert!(ink.bindings.borrow().is_some());
  ink.set_samples(device,1);assert_eq!(ink.color.active.0,first);assert_eq!(ink.pick,query);assert!(ink.bindings.borrow().is_none());
  ink.set_samples(device,4);assert_eq!(ink.color.active.0,second);assert_eq!(ink.pick,query);
  let direct=Ink::with_samples(device,wgpu::TextureFormat::Rgba8Unorm,stride,grid,varying,4);assert!(direct.color.alternate.is_none());
  direct.set(device,&table(&four),&four.depth);let _query=direct.group(device,&direct.query,&table(&one),&one.depth);
 }
 renderer.device.poll(wgpu::PollType::Wait {submission_index:None,timeout:Some(std::time::Duration::from_secs(5))}).unwrap();
}
