use std::{collections::HashMap,rc::Rc};
use wgpu::util::DeviceExt;
#[derive(Default)]
pub struct Geometry {planes:Vec<wgpu::Buffer>,groups:HashMap<(usize,u32),(wgpu::Buffer,wgpu::Buffer,wgpu::BindGroup)>}
pub fn layout(device:&wgpu::Device)->wgpu::BindGroupLayout {
 use wgpu::{ShaderStages as S,BufferBindingType as B};device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("cap plane and arena"),entries:&[crate::tile_pipelines::entry(0,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::VERTEX,B::Storage{read_only:true})]})
}
impl Geometry {
 pub fn set_planes(&mut self,device:&wgpu::Device,count:u32)->Result<(),&'static str> {
  if count as usize>crate::clipping_uniform::MAX_PLANES{return Err("Too many cap planes");}if self.planes.len()==count as usize{return Ok(());}
  self.clear();self.planes=(0..count).map(|plane|{let bytes=[plane,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("cap plane selector"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_SRC})}).collect();Ok(())
 }
 pub fn group(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,geometry:&Rc<crate::gpu_geometry::GpuGeometry>,plane:u32)->Result<wgpu::BindGroup,&'static str> {
  let parameter=self.planes.get(plane as usize).ok_or("Missing cap plane")?;let key=(Rc::as_ptr(geometry)as usize,plane);let(vertices,indices)=geometry.buffers();
  if let Some((old_vertices,old_indices,group))=self.groups.get(&key){if *old_vertices==vertices&&*old_indices==indices{return Ok(group.clone());}}
  let group=crate::tile_pipelines::group(device,layout,&[&vertices,parameter,&indices]);self.groups.insert(key,(vertices,indices,group.clone()));Ok(group)
 }
 pub fn invalidate(&mut self){self.groups.clear();}
 pub fn clear(&mut self){*self=Self::default();}
 pub fn bytes(&self)->u64 {self.planes.iter().map(wgpu::Buffer::size).sum()}
}
