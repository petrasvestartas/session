pub(crate) const FORMAT:wgpu::TextureFormat=wgpu::TextureFormat::Rg16Uint;
pub(crate) struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView}
impl Image{
    pub fn new(device:&wgpu::Device,size:[u32;2])->Self{
        let texture=device.create_texture(&wgpu::TextureDescriptor{
            label:Some("physical triangle addresses"),
            size:wgpu::Extent3d{width:size[0].max(1),height:size[1].max(1),depth_or_array_layers:1},
            mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:FORMAT,
            usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_SRC,
            view_formats:&[],
        });
        let view=texture.create_view(&Default::default());Self{texture,view}
    }
}
pub(crate) fn target(write:bool)->wgpu::ColorTargetState{
    wgpu::ColorTargetState{format:FORMAT,blend:None,write_mask:if write{wgpu::ColorWrites::ALL}else{wgpu::ColorWrites::empty()}}
}
