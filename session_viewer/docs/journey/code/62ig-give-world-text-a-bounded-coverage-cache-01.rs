use super::TextFrame;
use crate::text::{TextDocument, TextLabel, TextPlacement, TextRun};
use crate::text_gpu_support::resource_group;
use crate::text_gpu_support::texture_entry;
use crate::text_gpu_support::{GpuCtx, GrowBuf, VERTS};
use crate::text_gpu_support::{Pipeline, Target};
use glyphon::{FontSystem, SwashCache, SwashContent};
use wgpu::{ShaderStages, TextureSampleType};

const TEXTURE_BUDGET: u64 = 32 * 1024 * 1024;
struct CachedPlane {
    label: TextLabel,
    font_revision: u64,
    em_pixels: u32,
    size: [u32; 2],
    extent: [f32; 4],
    texture: wgpu::Texture,
    bind: wgpu::BindGroup,
}
impl Drop for CachedPlane {
    fn drop(&mut self) {
        self.texture.destroy();
    }
}
pub(super) struct Planes {
    cached: Vec<CachedPlane>,
    draws: Vec<(usize, u32)>,
    vertices: GrowBuf,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    pipeline: Pipeline,
    id_pipeline: Pipeline,
    target: Target,
    pub(super) rasterizations: u64,
}

fn same_raster(cached: &CachedPlane, label: &TextLabel, revision: u64, em_pixels: u32) -> bool {
    cached.font_revision == revision
        && cached.label.text == label.text
        && cached.label.font_size == label.font_size
        && cached.label.line_height == label.line_height
        && cached.em_pixels >= em_pixels
}

fn project(world: [f64; 3], frame: &TextFrame) -> [f32; 4] {
    let point = [
        (world[0] - frame.origin[0]) as f32,
        (world[1] - frame.origin[1]) as f32,
        (world[2] - frame.origin[2]) as f32,
        1.0,
    ];
    let mut clip = [0.0; 4];

    for (row, out) in clip.iter_mut().enumerate() {
        for (column, value) in point.iter().enumerate() {
            *out += frame.mvp[column * 4 + row] * value;
        }
    }

    clip
}
