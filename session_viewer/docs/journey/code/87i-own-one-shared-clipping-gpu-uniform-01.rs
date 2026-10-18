use crate::{clipping_rows::Row,clipping_uniform::{ClipUniform,ClipView,clip_uniform}};
use wgpu::util::DeviceExt;
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout{crate::gpu_layout::uniform(device,448)}
pub fn source(base:&str,group:u32,binding:u32)->String{format!("@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",include_str!("clip.wgsl"))}
pub fn bytes(uniform:&ClipUniform)->Vec<u8>{
    let mut bytes=uniform.planes.iter().chain(&uniform.screen).chain(&uniform.hatch).chain(&uniform.hatch_w).chain(&uniform.sides).flatten().copied().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();
    bytes.extend([uniform.count,uniform.samples,uniform.fill].into_iter().flat_map(u32::to_ne_bytes));bytes.extend([uniform.spacing,uniform.width,uniform.outline,uniform.pad[0],uniform.pad[1]].into_iter().flat_map(f32::to_ne_bytes));assert_eq!(bytes.len(),448);bytes
}
pub struct Clipping{pub buffer:wgpu::Buffer,pub group:wgpu::BindGroup,pub rows:Vec<Row>,pub error:Option<&'static str>,pub enabled:bool,pub solid:bool,key:Vec<u8>,pub writes:u64}
impl Clipping{
    pub fn new(device:&wgpu::Device)->Self{let key=bytes(&ClipUniform::default());let buffer=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("current original clipping planes"),contents:&key,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});let group=crate::gpu_layout::group(device,&layout(device),&buffer);Self{buffer,group,rows:Vec::new(),error:None,enabled:true,solid:true,key,writes:0}}
    pub fn set(&mut self,scene:&crate::scene::Scene){match crate::clipping_rows::current(scene){Ok(rows)=>{self.rows=rows;self.error=None},Err(error)=>{self.rows.clear();self.error=Some(error)}}}
    pub fn frame(&mut self,queue:&wgpu::Queue,matrix:[f32;16],height:u32,density:f32){let matrix=session_rust::Xform::from_matrix(matrix.map(f64::from));let planes=if self.enabled{self.rows.iter().map(|r|r.plane).collect::<Vec<_>>()}else{Vec::new()};let uniform=clip_uniform(&planes,u32::from(self.solid),&ClipView{view_proj:&matrix,anchor:[0.;3],height,pixel_scale:f64::from(density),samples:1,forward_depth:true});let bytes=bytes(&uniform);if bytes!=self.key{queue.write_buffer(&self.buffer,0,&bytes);self.key=bytes;self.writes+=1;}}
}
