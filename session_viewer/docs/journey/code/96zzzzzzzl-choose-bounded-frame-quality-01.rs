pub fn budget(gpu:wgpu::DeviceType)->Option<u32> {
    match gpu {
        wgpu::DeviceType::DiscreteGpu=>Some(9_000_000),
        wgpu::DeviceType::IntegratedGpu|wgpu::DeviceType::VirtualGpu=>Some(2_500_000),
        wgpu::DeviceType::Other=>Some(4_200_000),
        wgpu::DeviceType::Cpu=>None,
    }
}

pub fn samples(solid:bool,size:[u32;2],forced:Option<u32>,budget:Option<u32>,pixel_scale:f32,reduced:bool)->u32 {
    if reduced {return 1;}
    if let Some(count)=forced {return if count==4 {4}else{1};}
    if pixel_scale>=2. {return 1;}
    let pixels=u64::from(size[0])*u64::from(size[1]);
    if solid&&budget.is_some_and(|max|pixels<=u64::from(max)) {4}else{1}
}

