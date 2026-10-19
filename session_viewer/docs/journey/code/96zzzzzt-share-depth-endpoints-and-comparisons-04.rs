pub fn texture_layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label:Some("handle tile"),entries:&[
            wgpu::BindGroupLayoutEntry { binding:0,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture { sample_type:wgpu::TextureSampleType::Float {filterable:true},view_dimension:wgpu::TextureViewDimension::D2,multisampled:false },count:None },
            wgpu::BindGroupLayoutEntry { binding:1,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None },
        ] })
}

pub fn mesh(device:&wgpu::Device,layout:&wgpu::BindGroupLayout)->wgpu::RenderPipeline {
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("handles"),source:wgpu::ShaderSource::Wgsl(include_str!("widget.wgsl").into())});
        let mesh_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("handle mesh"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:Some("handle mesh"),layout:Some(&mesh_layout),
            vertex:wgpu::VertexState { module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32]}] },
            fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::TextureFormat::Rgba8Unorm.into())]}),
            primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState {format:wgpu::TextureFormat::Depth32Float,depth_write_enabled:Some(true),depth_compare:Some(crate::depth::CLOSER_EQUAL),stencil:Default::default(),bias:Default::default()}),
            multisample:wgpu::MultisampleState {count:4,..Default::default()},multiview_mask:None,cache:None,
        })
}

pub fn composite(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,texture_layout:&wgpu::BindGroupLayout,format:wgpu::TextureFormat)->wgpu::RenderPipeline {
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("handles"),source:wgpu::ShaderSource::Wgsl(include_str!("widget.wgsl").into())});
        let composite_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("handle composite"),bind_group_layouts:&[Some(&layout),Some(&texture_layout)],immediate_size:0});
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:Some("handle composite"),layout:Some(&composite_layout),
            vertex:wgpu::VertexState {module:&shader,entry_point:Some("vs_composite"),compilation_options:Default::default(),buffers:&[]},
            fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fs_composite"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})]}),
            primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None,
        })
}
