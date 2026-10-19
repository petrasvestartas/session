roto(),cloud_before);assert!(Rc::ptr_eq(&scene.sheets()[0].prepared.original,&sheet));crate::sample_test_proof::write(root.join("four.rgba"),four).unwrap();
}


#[test]
fn actual_text_sample_switches_keep_font_raster_sources_and_fresh_owner_queries() {
 use crate::{scene_label::{Prepared,example},text::TextPlacement};
 let mut scene=Scene::empty();let mesh=scene.insert(crate::prepared::PreparedMesh::triangle([[-0.8,-0.7,0.6],[0.8,-0.7,0.6],[0.,0.8,0.6]],[0.1,0.3,0.8]).unwrap()).unwrap();let mut r=crate::cap_gpu_tests::renderer(&scene);retarget_basic_fixture(&mut r,4);
 let mut label=example(TextPlacement::WorldPlane {world:[-0.5,0.25,0.2],right:[1.,0.,0.],up:[0.,1.,0.],world_height:0.18});label.label.text="Original".into();let original=Rc::new(label);scene.insert_text(Prepared::new(original.clone()).unwrap()).unwrap();
 let mut name=example(TextPlacement::Nameplate {world:[0.4,-0.35,0.2],padding:[8.,3.],rounded:true});name.label.text="Owner".into();let owner=scene.insert_text(Prepared::new(Rc::new(name)).unwrap()).unwrap();r.set_scene(&scene,None);let source=serde_json::to_string(&*original).unwrap();
 let mut overlay=crate::text_example::label();overlay.text="Sampled text".into();overlay.font_size=18.;r.set_text(vec![overlay]).unwrap();let transform=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));let first=crate::cap_gpu_tests::pixels_at(&r,transform);assert_eq!(r.text_error(),None);let stats=r.text_stats().unwrap();assert!(stats.world_plane_rasterizations>0&&stats.raster_images>0);let usage=r.text_scene_usage();let gpu=r.stats();let geometry=r.meshes[0].geometry.clone();
 let root=std::path::Path::new("text-pixels");crate::sample_test_proof::write(root.join("four.rgba"),&first).unwrap();let mut single=Vec::new();
 for count in [4,1,4] {retarget_basic_fixture(&mut r,count);let pixels=crate::cap_gpu_tests::pixels_at(&r,transform);assert_eq!(r.text_error(),None);if count==4 {assert!(pixels==first);}else{single=pixels;}let current=r.text_stats().unwrap();assert_eq!(current.shape_count,stats.shape_count);assert_eq!(current.world_plane_rasterizations,stats.world_plane_rasterizations);assert_eq!(current.raster_images,stats.raster_images);assert_eq!(current.atlas_resets,stats.atlas_resets);assert_eq!(r.text_scene_usage(),usage);assert_eq!(r.stats(),gpu);assert!(Rc::ptr_eq(&r.meshes[0].geometry,&geometry));
  for(at,id)in [([0.4,-0.35],owner),([-0.2,-0.2],mesh)] {r.request_pick(at,transform).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(transform).unwrap().unwrap().unwrap()[0],id.value(),"Original1x owner query after{count}x text");}
 }
 assert_eq!(serde_json::to_string(&*original).unwrap(),source);crate::sample_test_proof::write(root.join("one.rgba"),single).unwrap();
}

