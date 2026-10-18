struct Tile { size:[u32;2],color:wgpu::TextureView,depth:wgpu::TextureView,resolved:wgpu::TextureView,group:wgpu::BindGroup }

impl Tile {
    fn new(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,size:[u32;2]) -> Self {
        let texture=|format,sample_count,usage| device.create_texture(&wgpu::TextureDescriptor {label:Some("handle tile"),size:wgpu::Extent3d {width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count,dimension:wgpu::TextureDimension::D2,format,usage,view_formats:&[]}).create_view(&Default::default());
        let color=texture(wgpu::TextureFormat::Rgba8Unorm,4,wgpu::TextureUsages::RENDER_ATTACHMENT);
        let depth=texture(wgpu::TextureFormat::Depth32Float,4,wgpu::TextureUsages::RENDER_ATTACHMENT);
        let resolved=texture(wgpu::TextureFormat::Rgba8Unorm,1,wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING);
        let sampler=device.create_sampler(&wgpu::SamplerDescriptor {mag_filter:wgpu::FilterMode::Linear,min_filter:wgpu::FilterMode::Linear,..Default::default()});
        let group=device.create_bind_group(&wgpu::BindGroupDescriptor {label:Some("handle tile"),layout,entries:&[
            wgpu::BindGroupEntry {binding:0,resource:wgpu::BindingResource::TextureView(&resolved)},
            wgpu::BindGroupEntry {binding:1,resource:wgpu::BindingResource::Sampler(&sampler)},
        ]}); Self {size,color,depth,resolved,group}
    }
}
