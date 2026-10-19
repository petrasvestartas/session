)->Self{Self::with_samples(device,format,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->Self{assert!(matches!(samples,1|4));
  let t