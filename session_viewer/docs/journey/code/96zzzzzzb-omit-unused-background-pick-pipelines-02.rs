#[test]
fn grid_omits_its_base_integer_pipeline_without_stealing_original_picks(){
 let scene=crate::scene::Scene::empty();let mut gpu=crate::cap_gpu_tests::renderer(&scene);
 let grid=super::Lane::grid(&gpu.device,wgpu::TextureFormat::Rgba8Unorm);
 let original=super::Lane::new(&gpu.device,wgpu::TextureFormat::Rgba8Unorm);
 assert!(grid.pick_pipeline.is_none());assert!(original.pick_pipeline.is_some());
 gpu.set_grid_on(Some(crate::grid::Settings{spacing:0.25,half_cells:5}),crate::cplane::CPlane::Xy).unwrap();
 let matrix=crate::cap_gpu_tests::matrix();let image=crate::cap_gpu_tests::pixels_at(&gpu,matrix);
 assert!(image.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>100);
 gpu.request_pick([0.,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&gpu);assert!(gpu.take_pick(matrix).unwrap().unwrap().is_none());
 let mut scene=scene;let owner=scene.insert(crate::prepared::PreparedMesh::triangle([[-0.8,-0.8,-0.5],[0.8,-0.8,-0.5],[0.,0.8,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();
 let source=scene.objects()[0].geometry().unwrap().clone();gpu.set_scene(&scene,None);let uploads=gpu.stats()[0];
 gpu.request_pick([0.,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&gpu);let pick=gpu.take_pick(matrix).unwrap().unwrap().unwrap();assert_eq!(pick[0],owner.value());
 assert_eq!(gpu.stats()[0],uploads);assert!(std::rc::Rc::ptr_eq(&source,scene.objects()[0].geometry().unwrap()));
 gpu.set_scene(&crate::scene::Scene::empty(),None);gpu.set_grid_on(None,crate::cplane::CPlane::Xy).unwrap();assert_eq!(gpu.usage(),[0;4]);
}
