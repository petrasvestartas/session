)->Self{Self::with_samples(device,size,format,1)}
 pub fn with_samples(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,samples:u32)->Self{assert!(matches!(samples,1|4));let text