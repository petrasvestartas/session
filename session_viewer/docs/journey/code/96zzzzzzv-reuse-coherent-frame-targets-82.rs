);
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|word|*word==0));
}

#[t