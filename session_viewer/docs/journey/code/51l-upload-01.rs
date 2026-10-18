use wgpu::util::DeviceExt;

pub struct Widget {
    vertices:wgpu::Buffer, count:u32, uniform:wgpu::Buffer, group:wgpu::BindGroup,
    pipeline:wgpu::RenderPipeline, composite:wgpu::RenderPipeline,
    texture_layout:wgpu::BindGroupLayout, tile:Option<Tile>, visible:bool,
    pub allocations:u64, pub writes:u64,
}

impl Widget {
    pub fn new(device:&wgpu::Device, format:wgpu::TextureFormat) -> Self {
        let mesh=crate::widget_mesh::vertices();
        let bytes:Vec<u8>=mesh.iter().copied().flat_map(crate::widget_mesh::Vertex::bytes).collect();
        let vertices=device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label:Some("handle mesh"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX });
        let layout=crate::gpu_layout::uniform(device,96);
        let uniform=device.create_buffer(&wgpu::BufferDescriptor { label:Some("handle placement"),size:96,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false });
        let group=crate::gpu_layout::group(device,&layout,&uniform);
        let texture_layout=crate::widget_pipelines::texture_layout(device);
        let pipeline=crate::widget_pipelines::mesh(device,&layout);
        let composite=crate::widget_pipelines::composite(device,&layout,&texture_layout,format);
        Self {vertices,count:mesh.len() as u32,uniform,group,pipeline,composite,texture_layout,tile:None,visible:false,allocations:0,writes:0}
    }

    pub fn clear(&mut self) { self.tile=None; self.visible=false; }
    pub fn usage(&self) -> [u64;4] { [self.vertices.size()+self.uniform.size(),self.tile.as_ref().map_or(0,|t|u64::from(t.size[0])*u64::from(t.size[1])*36),self.allocations,self.writes] }
    pub fn extent(&self) -> Option<[u32;2]> { self.tile.as_ref().map(|t|t.size) }

}

impl Drop for Widget { fn drop(&mut self) {self.vertices.destroy();self.uniform.destroy();} }

struct Tile { size:[u32;2],color:wgpu::TextureView,depth:wgpu::TextureView,resolved:wgpu::TextureView,group:wgpu::BindGroup }

impl Tile {
