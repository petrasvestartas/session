pub(crate) struct Targets {
    pub depth: wgpu::TextureView,
    pub primitive: crate::primitive_image::Image,
    color: Option<wgpu::TextureView>,
    pub samples: u32,
}

impl Targets {
    pub fn new(
        device: &wgpu::Device,
        size: [u32; 2],
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert!(matches!(samples, 1 | 4) && !size.contains(&0));
        let image = |label, format| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size: wgpu::Extent3d {
                        width: size[0],
                        height: size[1],
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        Self {
            depth: image("scene depth", crate::depth::FORMAT),
            primitive: crate::primitive_image::Image::with_samples(device, size, samples),
            color: (samples == 4).then(|| image("multisampled scene colour", format)),
            samples,
        }
    }

    pub fn color<'a>(&'a self, single: &'a wgpu::TextureView) -> &'a wgpu::TextureView {
        self.color.as_ref().unwrap_or(single)
    }

    pub fn resolve(&self, encoder: &mut wgpu::CommandEncoder, single: &wgpu::TextureView) {
        let Some(color) = &self.color else {
            return;
        };
        let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("resolve scene colour"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color,
                resolve_target: Some(single),
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Discard,
                },
            })],
            ..Default::default()
        });
    }
}
