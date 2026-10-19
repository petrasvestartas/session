    ))
}


fn upload_plane(
    ctx: &GpuCtx,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    label: &TextLabel,
    font_revision: u64,
    em_pixels: u32,
    data: (Vec<u8>, [u32; 2], [f32; 4]),
) -> CachedPlane {
    let (pixels, size, extent) = data;
    let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("world text coverage"),
        size: wgpu::Extent3d {
            width: size[0],
            height: size[1],
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    ctx.queue.write_texture(
        texture.as_image_copy(),
        &pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(size[0]),
            rows_per_image: Some(size[1]),
        },
        texture.size(),
    );
    let view = texture.create_view(&Default::default());
    let bind = resource_group(
        ctx,
        &layout,
        "world text coverage",
        [
            (0, wgpu::BindingResource::TextureView(&view)),
            (1, wgpu::BindingResource::Sampler(&sampler)),
        ],
    );
    CachedPlane {
        label: label.clone(),
        font_revision: font_revision,
        em_pixels,
        size,
        extent,
        texture,
        bind,
    }
}
