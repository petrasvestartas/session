r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value(),"Single-sample closed-cap owner query is independent of4x crossing resources");
}


fn retarget_basic_fixture(r:&mut Renderer,count:u32) {
 super::cloud_address_tests::retarget(r,count);
 r.sheets.set_samples(&r.device,count);
}
#[test]
fn cached_clouds_and_original_sheets_draw_in_the_actual_four_sample_main_frame() {
 let mut scene=Scene::empty();let mesh=scene.insert(crate::prepared::PreparedMesh::triangle([[-0.7,-0.7,0.6],[0.7,-0.7,0.6],[0.,0.7,0.6]],[0.1,0.3,0.8]).unwrap()).unwrap();
 let mut original=session_rust::PointCloud::from_coords(vec![0.,0.,0.2],vec![255,0,0,255],vec![]);original.point_size=16.;let original=Rc::new(original);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(original.clone()).unwrap()).unwrap();let cloud_before=original.to_proto();let sheet=Rc::new(crate::sheet_example::source());let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();
 let mut r=crate::cap_gpu_tests::renderer(&scene);let transform=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));let first=crate::cap_gpu_tests::pixels_at(&r,transform);let uploads=(r.stats()[0],r.sheet_usage()[3],r.cloud_usage()[2]);let preparations=r.cloud_usage()[3];let geometry=r.meshes.iter().map(|m|Rc::clone(&m.geometry)).collect::<Vec<_>>();
 let root=std::path::Path::new("sheet-cloud-pixels");crate::sample_test_proof::write(root.join("one.rgba"),&first).unwrap();let mut four=Vec::new();
 for count in [4,1,4] {
  retarget_basic_fixture(&mut r,count);let pixels=crate::cap_gpu_tests::pixels_at(&r,transform);let centre=crate::cap_gpu_tests::at(&pixels,160,120);assert!(centre[0]>240&&centre[1]<10&&centre[2]<10,"Original cloud correctly covers the face and sheet at{count}x");assert!(pixels.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>400,"Drawing sheet remains visible");
  assert_eq!((r.stats()[0],r.sheet_usage()[3],r.cloud_usage()[2]),uploads);assert_eq!(r.cloud_usage()[3],preparations,"Changing main sample count must not rebuild cached cloud pixels");if count==1 {assert!(pixels==first);}else if four.is_empty(){four=pixels;}else{assert!(pixels==four);}
  for (at,id) in [([0.,0.],cloud),([0.,-0.5],drawing),([0.,-0.2],mesh)] {r.request_pick(at,transform).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(transform).unwrap().unwrap().unwrap()[0],id.value(),"Original owner after main{count}x switch");}
 }
 for (mesh,before)in r.meshes.iter().zip(&geometry){assert!(Rc::ptr_eq(&mesh.geometry,before));}assert_eq!(original.to_proto(),cloud_before);assert!(Rc::ptr_eq(&scene.sheets()[0].prepared.original,&sheet));crate::sample_test_proof::write(root.join("four.rgba"),four).unwrap();
}

