enum Storage {
    Owned {vertices:wgpu::Buffer,indices:wgpu::Buffer,group:wgpu::BindGroup},
    Shared(Rc<RefCell<crate::mesh_arena::Arena>>),
}

impl GpuGeometry {