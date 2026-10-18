    size: [u32; 2],
    density: f32,
    clipping:&wgpu::BindGroup,clip_revision:u64,
  ) {
    if !meshes.iter().any(GpuMesh::selected) {
