use crate::{ambient_image::Image,pyramid_image::Image as Pyramid};
pub struct Images{pub full:[u32;2],pub size:[u32;2],pub source_radius:Image,pub source_normals:Image,pub linear:Pyramid,pub radii:Pyramid,pub normals:Image,pub occupancy:Image,pub ground:Image,pub ao:[Image;3],pub history:Image,pub history_depth:Image}
impl Images{
 pub fn new(device:&wgpu::Device,full:[u32;2],density:f32)->Self{
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];let image=|size,format|Image::new(device,size,format);
  let source_radius=image(full,wgpu::TextureFormat::R32Float);let source_normals=image(full,wgpu::TextureFormat::Rg8Unorm);let linear=Pyramid::new(device,size,wgpu::TextureFormat::R32Float);let radii=Pyramid::new(device,size,wgpu::TextureFormat::R16Uint);let normals=image(size,wgpu::TextureFormat::Rg8Unorm);let occupancy=image(size,wgpu::TextureFormat::R8Unorm);let ground=image(size,wgpu::TextureFormat::R8Unorm);let ao=std::array::from_fn(|i|image(if i==2{full}else{size},wgpu::TextureFormat::R8Unorm));let history=image(size,wgpu::TextureFormat::R8Unorm);let history_depth=image(size,wgpu::TextureFormat::R32Float);Self{full,size,source_radius,source_normals,linear,radii,normals,occupancy,ground,ao,history,history_depth}
 }
 pub fn bytes(&self)->u64{self.source_radius.bytes+self.source_normals.bytes+self.linear.bytes+self.radii.bytes+self.normals.bytes+self.occupancy.bytes+self.ground.bytes+self.ao.iter().map(|i|i.bytes).sum::<u64>()+self.history.bytes+self.history_depth.bytes}
}
