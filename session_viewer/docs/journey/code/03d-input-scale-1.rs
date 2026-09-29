            top: f32::INFINITY,
            context,
            model: CommandLine {
                focus_command: true,
                ..Default::default()
            },
            painter: egui_wgpu::Renderer::new(&renderer.device, format, Default::default()),
