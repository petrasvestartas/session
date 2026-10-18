before));assert_eq!(&scene.objects().iter().find(|o|o.id==mesh.id).unwrap().geometry().unwrap().to_proto(),source);}
}


#[test]
fn a_colour_sample_count_change_preserves_an_inflight_independent_owner_query() {
 let (scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=crate::cap_gpu_tests::renderer(&scene);let matrix=crate::cap_gpu_tests::matrix();assert_eq!(r.samples(),1);r.request_pick([0.3,0.2],matrix).unwrap();let usage=r.pick_usage();r.set_samples(4);assert_eq!(r.samples(),4);assert_eq!(r.pick_usage(),usage,"Colour-only retarget must retain its independent1x query and ticket");crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());let first=crate::cap_gpu_tests::pixels(&r);let targets=(r.targets.depth.clone(),r.targets.primitive.view.clone());let stats=r.stats();r.set_samples(4);assert_eq!((&r.targets.depth,&r.targets.primitive.view),(&targets.0,&targets.1));assert_eq!(r.stats(),stats);assert!(crate::cap_gpu_tests::pixels(&r)==first);
 r.request_pick([-0.3,0.2],matrix).unwrap();let usage=r.pick_usage();r.set_samples(1);assert_eq!(r.pick_usage(),usage);crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());
}

