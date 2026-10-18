        Self::with_formats(device, size, format, format, samples)
    }

    pub fn with_formats(
        device: &wgpu::Device,
        size: [u32; 2],
        base: wgpu::TextureFormat,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert_eq!(base.remove_srgb_suffix(), format.remove_srgb_suffix());
        assert!(matches!(samples, 1 | 4) && !size.contains(&0));