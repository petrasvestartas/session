elf{Self::with_samples(device,format,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,count:u32)->Self{Self{pipes:V