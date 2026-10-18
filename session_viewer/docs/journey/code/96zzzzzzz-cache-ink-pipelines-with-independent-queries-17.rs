 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool)->Self{Self::with_samples(device,format,stride,grid,varying,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool,samples:u32)->Self{
  assert!(matches!(samples,1|4));