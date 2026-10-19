pub struct GpuCtx {pub device:wgpu::Device,pub queue:wgpu::Queue}
impl GpuCtx {pub fn new(device:&wgpu::Device,queue:&wgpu::Queue)->Self {Self {device:device.clone(),queue:queue.clone()}}}
pub type Pipeline=wgpu::RenderPipeline;
#[derive(Clone,Copy)]
pub struct Target {pub format:wgpu::TextureFormat,pub samples:u32}
impl Target {pub const ID:Self=Self {format:wgpu::TextureFormat::Rg32Uint,samples:1};}
pub const VERTS:wgpu::BufferUsages=wgpu::BufferUsages::VERTEX.union(wgpu::BufferUsages::COPY_DST);
pub struct GrowBuf {pub buf:wgpu::Buffer,stride:u64,len:u32,label:&'static str,usage:wgpu::BufferUsages}
impl GrowBuf {
 pub fn new(ctx:&GpuCtx,label:&'static str,stride:u64,usage:wgpu::BufferUsages)->Self {Self {buf:ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some(label),size:stride,usage,mapped_at_creation:false}),stride,len:0,label,usage}}
 pub fn append<const N:usize>(&mut self,ctx:&GpuCtx,vertices:&[[f32;N]]) {assert_eq!(self.stride,N as u64*4);if vertices.is_empty() {return;}
  let count=u32::try_from(vertices.len()).expect("Bounded text vertices");let size=(self.len as u64+count as u64)*self.stride;
  if size>self.buf.size() {assert_eq!(self.len,0,"Text upload starts at an empty buffer");self.buf=ctx.device.create_buffer(&wgpu::BufferDescriptor {label:Some(self.label),size:size.next_power_of_two(),usage:self.usage,mapped_at_creation:false});}
  let bytes:Vec<u8>=vertices.iter().flatten().flat_map(|n|n.to_ne_bytes()).collect();ctx.queue.write_buffer(&self.buf,self.len as u64*self.stride,&bytes);self.len+=count;
 }
 pub fn reset(&mut self) {self.len=0;}
 pub fn release(&mut self,ctx:&GpuCtx) {*self=Self::new(ctx,self.label,self.stride,self.usage);}
 pub fn len(&self)->u32 {self.len}
 pub fn is_empty(&self)->bool {self.len==0}
}
pub fn texture_entry(binding:u32,visibility:wgpu::ShaderStages,sample_type:wgpu::TextureSampleType,multisampled:bool)->wgpu::BindGroupLayoutEntry {wgpu::BindGroupLayoutEntry {binding,visibility,ty:wgpu::BindingType::Texture {sample_type,view_dimension:wgpu::TextureViewDimension::D2,multisampled},count:None}}
pub fn resource_group<'a,const N:usize>(ctx:&GpuCtx,layout:&wgpu::BindGroupLayout,label:&str,entries:[(u32,wgpu::BindingResource<'a>);N])->wgpu::BindGroup {ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {label:Some(label),layout,entries:&entries.into_iter().map(|(binding,resource)|wgpu::BindGroupEntry {binding,resource}).collect::<Vec<_>>()})}
