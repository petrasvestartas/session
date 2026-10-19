        crate::finite_project_source::check(&device, crate::finite_project_source::source(), "Finite projection");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {