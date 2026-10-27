use crate::production_ribbon_gpu::{buffer,group,storage};
pub const SOURCE:&str=include_str!("production_vector.wgsl");
pub(crate) struct Heads {
    source:&'static str,vertex:&'static str,vertices:u32,
    layouts:[wgpu::BindGroupLayout;4],shader:RefCell<Option<wgpu::ShaderModule>>,
    color:RefCell<Option<crate::sample_pipelines::Variants<wgpu::RenderPipeline>>>,pick:RefCell<Option<wgpu::RenderPipeline>>,
    group:wgpu::BindGroup,bytes:Vec<u8>,owners:Vec<u32>,count:u32,format:wgpu::TextureFormat,
}
impl Heads {
    pub fn new(device:&wgpu::Device,shared:&[wgpu::BindGroupLayout;4],format:wgpu::TextureFormat)->Self {
        Self::with_source(device,shared,format,SOURCE,"vs_main",18)
    }
    pub fn planes(device:&wgpu::Device,shared:&[wgpu::BindGroupLayout;4],format:wgpu::TextureFormat)->Self {
        Self::with_source(device,shared,format,crate::production_plane::SOURCE,"vs_plane",22*6+3*18)
    }
    fn with_source(device:&wgpu::Device,shared:&[wgpu::BindGroupLayout;4],format:wgpu::TextureFormat,source:&'static str,vertex:&'static str,vertices:u32)->Self {
        let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("production vector rows and owner ABI"),entries:&[storage(0,wgpu::ShaderStages::VERTEX_FRAGMENT),storage(1,wgpu::ShaderStages::FRAGMENT)]});
        let layouts=[shared[0].clone(),shared[1].clone(),shared[2].clone(),layout];
        let rows=buffer(device,&[],48,wgpu::BufferUsages::STORAGE);let owners=buffer(device,&[],4,wgpu::BufferUsages::STORAGE);
        let group=group(device,&layouts[3],vec![(0,rows.as_entire_binding()),(1,owners.as_entire_binding())]);
        Self{source,vertex,vertices,layouts,shader:Default::default(),color:Default::default(),pick:Default::default(),group,bytes:Vec::new(),owners:Vec::new(),count:0,format}
    }
    pub fn set(&mut self,device:&wgpu::Device,bytes:&[u8],owners:&[u32]) {
        if self.bytes==bytes&&self.owners==owners{return;}
