pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub levels:[wgpu::TextureView;6],pub bytes:u64}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat)->Self{
  let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("ambient six-level image"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:6,sample_count:1,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=texture.create_view(&Default::default());let levels=std::array::from_fn(|i|texture.create_view(&wgpu::TextureViewDescriptor{base_mip_level:i as u32,mip_level_count:Some(1),..Default::default()}));let bytes=(0..6).map(|i|u64::from(size[0]>>i)*u64::from(size[1]>>i)*u64::from(format.block_copy_size(None).unwrap())).sum();Self{texture,view,levels,bytes}
 }
}
