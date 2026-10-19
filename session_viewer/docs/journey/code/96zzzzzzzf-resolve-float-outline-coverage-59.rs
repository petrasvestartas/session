r{count}x text");}
 }
 assert_eq!(serde_json::to_string(&*original).unwrap(),source);crate::sample_test_proof::write(root.join("one.rgba"),single).unwrap();
}


#[test]
fn actual_four_sample_outline_and_section_masks_keep_single_blend_and_original_owners() {
 let (scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=crate::cap_gpu_tests::renderer(&scene);r.select_objects(&scene,&[owner]).unwrap();r.set_outlines(true);let first=crate::cap_gpu_tests::pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);assert!(r.outline_alpha()==alpha);let usage=r.global_outline_usage()[0];let geometry=r.meshes.iter().map(|m|Rc::clone(&m.geometry)).collect::<Vec<_>>();let uploads=r.stats()[0];let original=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let root=std::path::Path::new("outline-pixels");crate::sample_test_proof::write(root.join("selected-one.rgba"),&first).unwrap();let mut four=Vec::new();
 for count in [4,1,4] {retarget_basic_fixture(&mut r,count);let pixels=crate::cap_gpu_tests::pixels(&r);assert!(r.outline_alpha()==r.global_outline_alpha(),"Selected and global actual{count}x cap/face coverage remains identical");let expected=usage+if count==4 {4*320*240}else{0};assert_eq!(r.global_outline_usage()[0],expected,"Only the active4x coverage image adds memory;1x search images persist");assert_eq!(r.stats()[0],uploads);if count==1 {assert!(pixels==first);}else if four.is_empty(){four=pixels;}else{assert!(pixels==four);}
  let matrix=crate::cap_gpu_tests::matrix();r.request_pick([0.3,0.2],matrix).unwrap();crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());
 }
 crate::sample_test_proof::write(root.join("selected-four.rgba"),&four).unwrap();r.select_objects(&scene,&[]).unwrap();let global=crate::cap_gpu_tests::pixels(&r);crate::sample_test_proof::write(root.join("global-four.rgba"),&global).unwrap();let alpha=r.global_outline_alpha();let mut boundary=0;for (i,coverage)in alpha.iter().enumerate(){if *coverage==255 {boundary+=1;assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(four[i*4..i*4+3].iter().all(|v|*v<12));}}assert!(boundary>300);
 for ((mesh,geometry),source)in r.meshes.iter().zip(&geometry).zip(&original){assert!(Rc::ptr_eq(&mesh.geometry,geometry));assert_eq!(&scene.objects().iter().find(|o|o.id==mesh.id).unwrap().geometry().unwrap().to_proto(),source);}
 r.set_clipping(false,true);let uncut=crate::cap_gpu_tests::pixels(&r);assert!(r.global_outline_alpha()==alpha,"Actual4x horizontal section and full solid share the same outer silhouette");crate::sample_test_proof::write(root.join("uncut-four.rgba"),uncut).unwrap();
}

