use crate::pipelines::Pipelines;

pub struct GpuCtx {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,
    pub clear_color: wgpu::Color,
    pub pipelines: Pipelines,
    pub camera_buf: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub depth_tex_raw: wgpu::Texture,
    pub depth_view: wgpu::TextureView,
    pub msaa_view: wgpu::TextureView,
    #[allow(dead_code)]
    pub font_atlas_view: wgpu::TextureView,
    #[allow(dead_code)]
    pub font_sampler: wgpu::Sampler,
    pub glyph_bind_group: wgpu::BindGroup,
}

impl GpuCtx {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.is_surface_configured = true;
        let (raw, view) = create_depth_texture(&self.device, width, height, 4);
        self.depth_tex_raw = raw;
        self.depth_view = view;
        self.msaa_view = create_msaa_texture(&self.device, width, height, self.config.format, 4);
    }
}

pub fn create_depth_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    sample_count: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    (tex, view)
}

pub fn create_msaa_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::TextureView {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("msaa_texture"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    tex.create_view(&wgpu::TextureViewDescriptor::default())
}
