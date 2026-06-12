use crate::pipelines::Pipelines;

/// Per-resolution Arctic render targets + the bind groups that reference them.
/// Recreated on resize. Only exists when the backend can bind MSAA depth
/// (`ssao_supported`).
pub struct ArcticTargets {
    pub scene_color_view: wgpu::TextureView,
    pub ao_raw_view: wgpu::TextureView,
    pub ao_blur_view: wgpu::TextureView,
    pub ssao_bg: wgpu::BindGroup,
    /// Blur pass 1: reads ao_raw -> writes ao_blur.
    pub blur_bg: wgpu::BindGroup,
    /// Blur pass 2 (ping-pong): reads ao_blur -> writes ao_raw.
    pub blur2_bg: wgpu::BindGroup,
    /// Composite reads ao_raw (the twice-blurred result).
    pub composite_bg: wgpu::BindGroup,
}

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
    pub ssao_supported: bool,
    /// Adapter backend name ("BrowserWebGpu", "Gl", "Vulkan", ...) for the HUD.
    pub backend: String,
    pub arctic_buf: wgpu::Buffer,
    pub ground_vbo: wgpu::Buffer,
    pub arctic_targets: Option<ArcticTargets>,
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
        let (raw, view) = create_depth_texture(&self.device, width, height, 4, self.ssao_supported);
        self.depth_tex_raw = raw;
        self.depth_view = view;
        self.msaa_view = create_msaa_texture(&self.device, width, height, self.config.format, 4);
        self.arctic_targets = create_arctic_targets(
            &self.device, width, height, self.config.format,
            &self.pipelines, &self.arctic_buf, &self.depth_view,
        );
    }
}

pub fn create_depth_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    sample_count: u32,
    sampleable: bool,
) -> (wgpu::Texture, wgpu::TextureView) {
    let mut usage = wgpu::TextureUsages::RENDER_ATTACHMENT;
    if sampleable {
        usage |= wgpu::TextureUsages::TEXTURE_BINDING;
    }
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth_texture"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage,
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

fn create_target(
    device: &wgpu::Device,
    label: &str,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
) -> wgpu::TextureView {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    tex.create_view(&wgpu::TextureViewDescriptor::default())
}

/// Returns `None` when the backend has no SSAO support (`pipelines.arctic` is `None`).
pub fn create_arctic_targets(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    surface_format: wgpu::TextureFormat,
    pipelines: &Pipelines,
    arctic_buf: &wgpu::Buffer,
    depth_view: &wgpu::TextureView,
) -> Option<ArcticTargets> {
    let ap = pipelines.arctic.as_ref()?;
    let w = width.max(1);
    let h = height.max(1);
    // R16Float, not R8Unorm: the wide soft ground gradients band visibly at 8 bits.
    let scene_color_view = create_target(device, "arctic.scene_color", w, h, surface_format);
    let ao_raw_view = create_target(device, "arctic.ao_raw", w, h, wgpu::TextureFormat::R16Float);
    let ao_blur_view = create_target(device, "arctic.ao_blur", w, h, wgpu::TextureFormat::R16Float);
    let ssao_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("ssao.bg"),
        layout: &ap.ssao_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: arctic_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(depth_view) },
        ],
    });
    let blur_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("ssao_blur.bg"),
        layout: &ap.blur_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&ao_raw_view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(depth_view) },
        ],
    });
    let blur2_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("ssao_blur2.bg"),
        layout: &ap.blur_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&ao_blur_view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(depth_view) },
        ],
    });
    let composite_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("composite.bg"),
        layout: &ap.composite_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&scene_color_view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&ao_raw_view) },
        ],
    });
    Some(ArcticTargets {
        scene_color_view,
        ao_raw_view,
        ao_blur_view,
        ssao_bg,
        blur_bg,
        blur2_bg,
        composite_bg,
    })
}

pub fn create_ground_vbo(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("arctic.ground"),
        size: 6 * 12, // 6 vertices x vec3<f32>
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}
