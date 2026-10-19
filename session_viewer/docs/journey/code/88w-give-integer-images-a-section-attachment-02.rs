        let readback = device.create_buffer(&wgpu::BufferDescriptor { label: Some("bounded pick readback"), size: window.bytes(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ, mapped_at_creation: false });
        let section=device.create_texture(&wgpu::TextureDescriptor{label:Some("bounded section depth"),size,mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rg32Float,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,view_formats:&[]}).create_view(&Default::default());
        Self { texture, colour, depth, section,readback, window }
    }

