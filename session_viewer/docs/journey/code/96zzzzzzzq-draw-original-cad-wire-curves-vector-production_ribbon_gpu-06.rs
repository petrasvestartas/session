    })
}

pub(crate) fn pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layouts: &[wgpu::BindGroupLayout; 4],
