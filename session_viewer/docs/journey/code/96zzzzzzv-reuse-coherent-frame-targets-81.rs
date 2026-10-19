ty});
  assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|word|*word==0));
  gpu.