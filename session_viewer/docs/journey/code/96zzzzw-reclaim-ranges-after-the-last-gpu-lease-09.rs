use crate::mesh::Mesh;
use std::{cell::RefCell,rc::Rc};
use wgpu::util::DeviceExt;

pub struct GpuGeometry {
    pub source: Rc<Mesh>,
    pub(crate) index_count: u32,
    pub(crate) index_start: u32,
    storage: Storage,
}

enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared {arena:Rc<RefCell<crate::mesh_arena::Arena>>,_lease:Rc<crate::arena_rows::MeshRange>},
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding, visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(if binding==0 {20}else{4}),
            }, count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"), entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(device: &wgpu::Device, label: &str, mut bytes: Vec<u8>, usage: wgpu::BufferUsages) -> wgpu::Buffer {
            if bytes.is_empty() { bytes.resize(4, 0); }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label), contents: &bytes, usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let mut bytes = crate::packed_vertex::bytes(&source);
        if bytes.is_empty() { bytes.resize(20, 0); }
        let vertices = buffer(device, "shared mesh vertices", bytes, wgpu::BufferUsages::VERTEX);
        let bytes = source.indices().iter().flat_map(|v| v.to_ne_bytes()).collect();
        let indices = buffer(device, "shared mesh indices", bytes, wgpu::BufferUsages::INDEX);
        let entries = [(0, &vertices), (2, &indices)].map(|(binding, buffer)| wgpu::BindGroupEntry {
            binding, resource: buffer.as_entire_binding(),
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"), layout: &Self::storage_layout(device), entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self {source,index_count,index_start:0,storage:Storage::Owned {vertices,indices,group:pull_group}}
    }

    pub fn shared(ctx:&crate::text_gpu_support::GpuCtx,source:Rc<Mesh>,arena:Rc<RefCell<crate::mesh_arena::Arena>>)->Result<Self,&'static str>{
        let range=arena.borrow_mut().acquire(ctx,&source)?;
        Ok(Self {source,index_count:range.indices.len()as u32,index_start:range.indices.start,storage:Storage::Shared {arena,_lease:range}})
    }
    pub(crate) fn buffers(&self)->(wgpu::Buffer,wgpu::Buffer){
        match &self.storage {
            Storage::Owned {vertices,indices,..}=>(vertices.clone(),indices.clone()),
            Storage::Shared {arena,..}=>{let arena=arena.borrow();(arena.vertices.buf.clone(),arena.indices.buf.clone())},
        }
    }
    fn group(&self)->wgpu::BindGroup{
        match &self.storage {Storage::Owned {group,..}=>group.clone(),Storage::Shared {arena,..}=>arena.borrow().group.clone()}
    }
    pub(crate) fn arena(&self)->Option<&Rc<RefCell<crate::mesh_arena::Arena>>> {match &self.storage {Storage::Shared {arena,..}=>Some(arena),Storage::Owned{..}=>None}}
    pub fn allocated_bytes(&self)->u64{
        match &self.storage {Storage::Owned {vertices,indices,..}=>vertices.size()+indices.size(),Storage::Shared {..}=>0}
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 { return; }
        pass.set_bind_group(3, &self.group(), &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {self.draw_instances(pass,1);}

    pub fn draw_instances(&self,pass:&mut wgpu::RenderPass<'_>,count:u32){self.draw_range(pass,0..count);}
    pub(crate) fn draw_range(&self,pass:&mut wgpu::RenderPass<'_>,instances:std::ops::Range<u32>){
        if self.index_count==0||instances.is_empty(){return;}
        let(vertices,indices)=self.buffers();
        pass.set_vertex_buffer(0,vertices.slice(..));pass.set_index_buffer(indices.slice(..),wgpu::IndexFormat::Uint32);
        pass.draw_indexed(self.index_start..self.index_start+self.index_count,0,instances);
    }
}
