        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {