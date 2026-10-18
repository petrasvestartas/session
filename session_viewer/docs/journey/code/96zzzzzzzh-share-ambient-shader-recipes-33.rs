ample");
  let composite_layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient sparse sample corrections"),entries:&[entry(0,S::VERTEX_FRAGMENT,B::Uniform),entry(1,S::FRAGMENT,B::Storage{read_only:true}),entry(2,S::VERTEX,B::Storage{read_only:true})]});
  let comp