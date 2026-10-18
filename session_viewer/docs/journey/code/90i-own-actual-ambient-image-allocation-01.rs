pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub bytes:u64,pub size:[u32;2]}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat)->Self{let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("ambient actual image"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());let bytes=u64::from(size[0])*u64::from(size[1])*u64::from(format.block_copy_size(None).unwrap());Self{texture,view,bytes,size}}
}
