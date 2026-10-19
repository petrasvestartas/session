use crate::{
  gpu_mesh::GpuMesh,
  text_gpu_support::{GpuCtx, resource_group, texture_entry},
};
use wgpu::{ShaderStages as Stages, TextureSampleType as Sample};

struct Image {
  texture: wgpu::Texture,
  view: wgpu::TextureView,
  size: [u32; 2],
}
impl Image {
  fn new(device: &wgpu::Device, size: [u32; 2], label: &str) -> Self {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: Some(label),
      size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::R8Unorm,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT
        | wgpu::TextureUsages::TEXTURE_BINDING
        | wgpu::TextureUsages::COPY_SRC,
      view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    Self { texture, view, size }
  }
  fn bytes(&self) -> u64 {
    self.size[0] as u64 * self.size[1] as u64
  }
}
impl Drop for Image {
  fn drop(&mut self) {
    self.texture.destroy();
  }
}
struct Masks {
  coverage: Image,
  coarse: Image,
  dilated: Image,
  alpha: Image,
  block: u32,
  pool: wgpu::BindGroup,
  dilate: wgpu::BindGroup,
  search: wgpu::BindGroup,
  composite: wgpu::BindGroup,
}
#[derive(Clone, PartialEq)]
struct Key {
  transform: [f32; 16],
  size: [u32; 2],
  density: u32,
  revision: u64,
}

pub struct Outline {
  mask_pipeline: wgpu::RenderPipeline,
  pool_pipeline: wgpu::RenderPipeline,
  dilate_pipeline: wgpu::RenderPipeline,
  alpha_pipeline: wgpu::RenderPipeline,
  composite_pipeline: wgpu::RenderPipeline,
  pool_layout: wgpu::BindGroupLayout,
  search_layout: wgpu::BindGroupLayout,
  composite_layout: wgpu::BindGroupLayout,
  radius: wgpu::Buffer,
  taps: wgpu::Buffer,
  tap_group: wgpu::BindGroup,
  masks: Option<Masks>,
  key: Option<Key>,
  tap_radius: Option<f32>,
  revision: u64,
  pub rebuilds: u64,
  pub skips: u64,
  pub draws: u64,
}
impl Outline {
}
fn raster(
  encoder: &mut wgpu::CommandEncoder,
  view: &wgpu::TextureView,
  pipeline: &wgpu::RenderPipeline,
  groups: &[&wgpu::BindGroup],
) {
  let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    label: Some("outline preparation"),
    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
      view,
      depth_slice: None,
      resolve_target: None,
      ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
    })],
    ..Default::default()
  });
  pass.set_pipeline(pipeline);
  for (i, group) in groups.iter().enumerate() {
    pass.set_bind_group(i as u32, *group, &[]);
  }
  pass.draw(0..3, 0..1);
}
