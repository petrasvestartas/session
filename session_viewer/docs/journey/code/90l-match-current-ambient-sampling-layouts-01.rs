}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let texture=|binding,sample|crate::text_gpu_support::texture_entry(binding,S::FRAGMENT,sample,false);let uniform=|binding|crate::tile_pipelines::entry(binding,S::FRAGMENT,B::Uniform);
  let source=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient source images and precise camera"),entries:&[texture(0,T::Depth),uniform(1),texture(2,T::Float{filterable:false}),texture(3,T::Float{filterable:false}),texture(4,T::Float{filterable:false}),uniform(5)]});
  let level=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reduced original properties"),entries:&[texture(0,T::Float{filterable:false}),texture(1,T::Uint),texture(2,T::Float{filterable:false})]});
  let sample=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient filtered samples"),entries:&[texture(0,T::Float{filterable:true}),wgpu::BindGroupLayoutEntry{binding:1,visibility:S::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None}]});
  let mut entries=(0..4).map(|i|crate::tile_pipelines::entry(i,S::FRAGMENT,B::Storage{read_only:false})).collect::<Vec<_>>();entries.push(texture(4,T::Float{filterable:true}));entries.push(texture(5,T::Float{filterable:false}));let history=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reserved current history bindings"),entries:&entries});
  let shader=crate::clipping_gpu::source(include_str!("ambient.wgsl"),0,5);let groups=[Some(&source),Some(&level),Some(&sample),Some(&history)];let prepare=pipeline(device,&shader,"fs_prepare",&[Some(&source)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint,wgpu::TextureFormat::Rg8Unorm],false);let make=|fragment|pipeline(device,&shader,fragment,&groups,&[wgpu::TextureFormat::R8Unorm],false);
  let ground=make("fs_ground");let raw=make("fs_main");let filter=[make("fs_filter_x"),make("fs_filter_y")];let upsample=make("fs_upsample");let composite=pipeline(device,include_str!("ambient_composite.wgsl"),"fs_main",&[None,Some(&sample)],&[format],true);
  let reduce=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_reduce",&[Some(&level)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint],false);let occupancy=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_occupancy",&[Some(&level)],&[wgpu::TextureFormat::R8Unorm],false);
  Self{source,level,sample,history,prepare,reduce,occupancy,ground,raw,filter,upsample,composite}
 }
}
