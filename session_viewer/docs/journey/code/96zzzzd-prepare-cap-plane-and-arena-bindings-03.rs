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

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn planes_keep_current_arena_bindings_and_release() {
  let renderer=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&renderer.device,&renderer.queue);let layout=layout(&ctx.device);let mut groups=Geometry::default();groups.set_planes(&ctx.device,2).unwrap();assert_eq!(groups.bytes(),32);
  for(plane,buffer)in groups.planes.iter().enumerate(){assert_eq!(crate::growing_buffer_tests::read(&ctx,buffer,16),[plane as u32,0,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());}
  let mut scene=crate::scene::Scene::empty();scene.insert(crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let arena=Rc::new(std::cell::RefCell::new(crate::mesh_arena::Arena::new(&ctx)));let geometry=Rc::new(crate::gpu_geometry::GpuGeometry::shared(&ctx,scene.objects()[0].mesh.clone(),arena.clone()).unwrap());let old=groups.group(&ctx.device,&layout,&geometry,1).unwrap();assert_eq!(groups.group(&ctx.device,&layout,&geometry,1).unwrap(),old);assert!(groups.group(&ctx.device,&layout,&geometry,2).is_err());
  scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();arena.borrow_mut().upload(&ctx,&scene.objects()[1].mesh).unwrap();let fresh=groups.group(&ctx.device,&layout,&geometry,1).unwrap();assert_ne!(fresh,old);assert_eq!(groups.group(&ctx.device,&layout,&geometry,1).unwrap(),fresh);
  assert!(groups.set_planes(&ctx.device,crate::clipping_uniform::MAX_PLANES as u32+1).is_err());assert_eq!(groups.bytes(),32);groups.invalidate();assert!(groups.groups.is_empty());groups.clear();assert_eq!(groups.bytes(),0);assert_eq!(groups.planes.capacity(),0);assert_eq!(groups.groups.capacity(),0);
 }
}
