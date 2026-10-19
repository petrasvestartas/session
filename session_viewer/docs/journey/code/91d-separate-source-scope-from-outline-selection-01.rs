  revision: u64,
  clipping:u64,
  all:bool,
}

pub struct Outline {
  mask_pipeline: wgpu::RenderPipeline,
  caps: crate::outline_caps::Caps,
  pool_pipeline: wgpu::RenderPipeline,
  dilate_pipeline: wgpu::RenderPipeline,
