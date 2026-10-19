pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub size:[u32;2],pub bytes:u64}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,bytes:u64)->Self{let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("original section data"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});let view=texture.create_view(&Default::default());Self{texture,view,size,bytes:bytes*u64::from(size[0])*u64::from(size[1])}}
}
