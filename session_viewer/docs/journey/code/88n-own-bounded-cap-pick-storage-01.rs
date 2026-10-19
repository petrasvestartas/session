use crate::{cap_image::Image,pick_window::Window};
pub struct Pick{pipes:crate::cap_pick_pipeline::Pipelines,records:wgpu::Buffer,owners:wgpu::Buffer,frame:wgpu::Buffer,frame_group:wgpu::BindGroup,record_group:Option<wgpu::BindGroup>,owner_group:Option<wgpu::BindGroup>,target:Option<Image>,pub preparations:u64}
impl Pick{
 pub fn new(device:&wgpu::Device)->Self{let pipes=crate::cap_pick_pipeline::Pipelines::new(device);let buffer=|label,size,usage|device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size,usage:usage|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let records=buffer("bounded original cap crossings",16,wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC);let owners=buffer("exact original cap owners",8,wgpu::BufferUsages::STORAGE);let frame=buffer("cap physical crop frame",32,wgpu::BufferUsages::UNIFORM);let frame_group=crate::gpu_layout::group(device,&pipes.frame,&frame);Self{pipes,records,owners,frame,frame_group,record_group:None,owner_group:None,target:None,preparations:0}}
 fn capacity(device:&wgpu::Device,needed:u64,buffer:&mut wgpu::Buffer,label:&str)->Result<(),&'static str>{if needed>u64::from(device.limits().max_storage_buffer_binding_size){return Err("Cap picking exceeds the device storage binding limit")}if buffer.size()<needed{*buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some(label),size:needed,usage:buffer.usage(),mapped_at_creation:false});}Ok(())}



}
