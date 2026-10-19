use crate::{gpu_mesh::GpuMesh,instance_gpu::Lane};
use wgpu::util::DeviceExt;
pub struct Bindings {vertices:wgpu::Buffer,indices:wgpu::Buffer,objects:wgpu::Buffer,pub(crate) table:wgpu::TextureView,pub(crate) parameters:wgpu::Buffer,group:Option<(wgpu::Buffer,wgpu::BindGroup)>}
impl Bindings {
 pub fn source<'a>(meshes:&[GpuMesh],records:Option<&'a Lane>)->Option<&'a Lane> {
  records.filter(|records|records.lookup.is_some()&&records.space.is_some()&&meshes.iter().any(|mesh|mesh.geometry.index_count>0)&&meshes.iter().all(|mesh|records.current_row(mesh).is_some()))
 }
 pub fn new(device:&wgpu::Device,meshes:&[GpuMesh],records:Option<&Lane>,count:u32)->Option<Self> {
  if count==0{return None;}let records=Self::source(meshes,records)?;let mesh=meshes.iter().find(|mesh|mesh.geometry.index_count>0)?;let(vertices,indices)=mesh.geometry.buffers();let table=records.lookup.as_ref()?.view.clone();let arena=records.space.as_ref()?.texel(0)[1];
  let bytes=[count,arena,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();let parameters=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("lookup projection interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_SRC});
  Some(Self {vertices,indices,objects:records.record_buffer().clone(),table,parameters,group:None})
 }
 pub fn group(&mut self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,projected:&wgpu::Buffer)->wgpu::BindGroup {
  if let Some((old,group))=&self.group {if old==projected{return group.clone();}}
  let buffers=[&self.vertices,&self.indices,&self.objects,projected,&self.parameters];let mut entries=buffers.into_iter().enumerate().map(|(i,buffer)|wgpu::BindGroupEntry{binding:i as u32,resource:buffer.as_entire_binding()}).collect::<Vec<_>>();entries.push(wgpu::BindGroupEntry{binding:5,resource:wgpu::BindingResource::TextureView(&self.table)});
  let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("current lookup projection"),layout,entries:&entries});self.group=Some((projected.clone(),group.clone()));group
 }
 pub fn bytes(&self)->u64 {self.parameters.size()}
}
