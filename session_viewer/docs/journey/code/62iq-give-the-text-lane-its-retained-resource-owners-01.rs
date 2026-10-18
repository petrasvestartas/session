        anchored,
    }
}

pub struct Lane {
    pub document: TextDocument,
    pub stats: TextStats,
    cache: Cache,
    atlas: TextAtlas,
    viewport: Viewport,
    raster: SwashCache,
    overlay: TextRenderer,
    anchored: TextRenderer,
    plates: plate::Plates,
    planes: plane::Planes,
    target: Target,
    atlas_font_revision: u64,
    prepared: Option<(u64, u64, TextFrame)>,
    raster_keys: HashSet<glyphon::CacheKey>,
    overlay_count: u32,
    anchored_count: u32,
}

fn renderer(
    ctx: &GpuCtx,
    atlas: &mut TextAtlas,
    target: Target,
    compare: wgpu::CompareFunction,
) -> TextRenderer {
    TextRenderer::new(
        atlas,
        &ctx.device,
        wgpu::MultisampleState {
            count: target.samples,
            ..Default::default()
        },
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false),
            depth_compare: Some(compare),
            stencil: Default::default(),
            bias: Default::default(),
        }),
    )
}

fn make_atlas(ctx: &GpuCtx, cache: &Cache, format: wgpu::TextureFormat) -> TextAtlas {
    let mode = if format.is_srgb() {
        ColorMode::Accurate
    } else {
        ColorMode::Web
    };
    TextAtlas::with_color_mode(&ctx.device, &ctx.queue, cache, format, mode)
}

fn depth_for(depths: &HashMap<usize, f32>, id: usize) -> f32 {
    depths.get(&id).copied().unwrap_or(0.0)
}
