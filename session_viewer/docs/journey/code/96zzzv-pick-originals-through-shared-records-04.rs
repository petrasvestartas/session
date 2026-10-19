        let source=concat!(include_str!("triangle.wgsl"), "
", include_str!("pick_crop.wgsl"), "
", include_str!("triangle_pick.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("mesh IDs"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source,2,1).into())});let slots=[crate::draw_slots::layout()];
