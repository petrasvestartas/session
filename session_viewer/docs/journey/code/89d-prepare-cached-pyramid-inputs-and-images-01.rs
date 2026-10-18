use crate::pyramid_image::Image;
struct Images{size:[u32;2],pub linear:Image,pub radii:Image,groups:[wgpu::BindGroup;5]}
pub struct Pyramid{pipes:crate::pyramid_pipeline::Pipelines,frame:wgpu::Buffer,images:Option<Images>,input:Option<(wgpu::TextureView,wgpu::BindGroup)>,cached:Option<([u32;16],[u32;2],u32,[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Pyramid{
 pub fn new(device:&wgpu::Device)->Self{Self{pipes:crate::pyramid_pipeline::Pipelines::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("ambient earlier camera"),size:112,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),images:None,input:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 fn prepare(&mut self,device:&wgpu::Device,depth:&wgpu::TextureView,full:[u32;2],density:f32){
  let (width,height)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[width.max(32),height.max(32)];
  if self.images.as_ref().is_none_or(|images|images.size!=size){let linear=Image::new(device,size,wgpu::TextureFormat::R32Float);let radii=Image::new(device,size,wgpu::TextureFormat::R16Uint);let groups=std::array::from_fn(|i|device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient previous level"),layout:&self.pipes.level,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&linear.levels[i])},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(&radii.levels[i])}]}));self.images=Some(Images{size,linear,radii,groups});self.cached=None;self.preparations+=1;}
  if self.input.as_ref().is_none_or(|(old,_)|old!=depth){let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient source depth"),layout:&self.pipes.input,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:1,resource:self.frame.as_entire_binding()}]});self.input=Some((depth.clone(),group));self.cached=None;}
 }




}
