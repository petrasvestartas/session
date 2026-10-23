    })
}

pub(crate) fn group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    resources: Vec<(u32, wgpu::BindingResource<'_>)>,
