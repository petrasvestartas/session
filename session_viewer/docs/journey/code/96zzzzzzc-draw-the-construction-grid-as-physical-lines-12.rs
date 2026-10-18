use crate::{camera::{Camera,Projection,View},grid::Settings,renderer::Renderer,scene::Scene,viewport::Viewport};
use sha2::{Digest,Sha256};
fn renderer(scene:&Scene,format:wgpu::TextureFormat)->Renderer{
 let instance=wgpu::Instance::new(wgpu::InstanceDescriptor{backends:wgpu::Backends::VULKAN,flags:Default::default(),memory_budget_thresholds:Default::default(),backend_options:Default::default(),display:None});
 let adapter=pollster::block_on(instance.request_adapter(&Default::default())).unwrap();let(device,queue)=pollster::block_on(adapter.request_device(&Default::default())).unwrap();
 device.on_uncaptured_error(std::sync::Arc::new(|error|panic!("Actual hardware grid: {error}")));
 Renderer::new(device,queue,format,scene)
}
#[test]
fn actual_hardware_grid_matches_independent_current_native_frames(){
 let manifest:serde_json::Value=serde_json::from_str(include_str!("grid_reference_fixture.json")).unwrap();
 for density in [1u32,2]{
  let size=256*density;let scene=Scene::empty();let format=wgpu::TextureFormat::Rgba8UnormSrgb;let mut gpu=renderer(&scene,format);
  gpu.resize(Viewport{width:size,height:size});gpu.set_density(density as f32);
  let mut camera=Camera::default();camera.set_view(View::Top);camera.projection=Projection::Orthographic;camera.distance=12000.;camera.aspect=1.;camera.grid=Some(Settings::default());
  gpu.set_grid_on(camera.grid,crate::cplane::CPlane::Xy).unwrap();let usage=gpu.grid_usage();assert_eq!(&usage[..2],&[25,1100]);assert_eq!(gpu.clipping_ink_bytes(),[0,0]);
  let actual=crate::widget_tests::pixels_format(&mut gpu,&camera.uniform(),[size,size],format);
  let frame=&manifest["frames"][(density-1)as usize];let bytes=std::fs::read(frame["asset"].as_str().unwrap()).unwrap();
  assert_eq!(format!("{:x}",Sha256::digest(&bytes)),frame["sha256"].as_str().unwrap());
  let header=format!("P6\n{size} {size}\n255\n");let expected=bytes.strip_prefix(header.as_bytes()).unwrap();let rgb=actual.chunks_exact(4).flat_map(|p|p[..3].iter().copied()).collect::<Vec<_>>();
  assert!(rgb==expected,"Actual hardware grid differs from the independent current renderer at physical size {size}");
  gpu.set_grid_on(camera.grid,crate::cplane::CPlane::Xy).unwrap();assert_eq!(gpu.grid_usage(),usage);
  gpu.request_pick([0.,0.],camera.uniform()).unwrap();crate::cap_gpu_tests::wait(&gpu);assert!(gpu.take_pick(camera.uniform()).unwrap().unwrap().is_none());
  gpu.set_grid_on(None,crate::cplane::CPlane::Xy).unwrap();assert_eq!(&gpu.grid_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
 }
}
#[test]
fn grid_stays_behind_original_solid_color_and_never_owns_a_pick(){
 let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().clone();
 let mut gpu=crate::cap_gpu_tests::renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();camera.set_view(View::Top);camera.aspect=4./3.;let matrix=camera.uniform();let uploads=gpu.stats()[0];
 let settings=Settings{spacing:0.25,half_cells:5};let pixels=|gpu:&mut Renderer|crate::widget_tests::pixels(gpu,&matrix,[320,240]);
 let solid=pixels(&mut gpu);gpu.set_grid_on(Some(settings),crate::cplane::CPlane::Xy).unwrap();let opaque=pixels(&mut gpu);let center=(100*320+159)*4;assert_eq!(&opaque[center..center+4],&solid[center..center+4]);assert!(opaque!=solid,"The grid must draw outside the opaque solid");
 gpu.set_opacity(0.5);let glass_grid=pixels(&mut gpu);gpu.set_grid_on(None,crate::cplane::CPlane::Xy).unwrap();let glass_plain=pixels(&mut gpu);assert_ne!(&glass_grid[center..center+4],&glass_plain[center..center+4],"The hardware grid is behind the transparent original");
 gpu.set_grid_on(Some(settings),crate::cplane::CPlane::Xy).unwrap();gpu.request_pick([0.,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&gpu);let pick=gpu.take_pick(matrix).unwrap().unwrap().unwrap();assert_eq!(pick[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert!(std::rc::Rc::ptr_eq(&source,scene.objects()[0].geometry().unwrap()));
 gpu.set_scene(&Scene::empty(),None);gpu.request_pick([0.,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&gpu);assert!(gpu.take_pick(matrix).unwrap().unwrap().is_none());gpu.set_grid_on(None,crate::cplane::CPlane::Xy).unwrap();assert_eq!(gpu.usage(),[0;4]);
}
