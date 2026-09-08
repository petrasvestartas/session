//! Black annotation plates share the text lane's target and overlay depth policy.
use super::super::buffers::{GpuCtx, GrowBuf, VERTS};
use crate::engine::pipelines::Target;

/// Original physical plate bounds and independent clip bounds preserve corner shape when clipped.
pub(super) struct Rectangle {
    pub(super) bounds: [f32; 4],
    pub(super) clip: [f32; 4],
    pub(super) rounded: bool,
}

/// Batched physical rectangles, with no DOM, source geometry or independent device ownership.
pub(super) struct Plates {
    vertices: GrowBuf,
    pipeline: wgpu::RenderPipeline,
}

impl Plates {
    /// Start with one vertex of capacity and an empty draw list.
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 28, VERTS),
            pipeline: pipeline(ctx, target),
        }
    }

    /// Match sample count and color target when the surrounding text pass changes.
    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target);
    }

    /// Upload clipped quads with original center, half size and maximum corner radius.
    pub(super) fn prepare(&mut self, ctx: &GpuCtx, rectangles: &[Rectangle], size: [u32; 2]) {
        self.vertices.reset();
        let mut vertices = Vec::with_capacity(rectangles.len() * 6);
        for rectangle in rectangles {
            let [left, top, right, bottom] = rectangle.bounds;
            let half = [(right - left) * 0.5, (bottom - top) * 0.5];
            let center = [(left + right) * 0.5, (top + bottom) * 0.5];
            let radius = if rectangle.rounded {
                half[0].min(half[1]).max(0.0)
            } else {
                0.0
            };
            let left = left.max(rectangle.clip[0]);
            let top = top.max(rectangle.clip[1]);
            let right = right.min(rectangle.clip[2]);
            let bottom = bottom.min(rectangle.clip[3]);
            if right <= left || bottom <= top {
                continue;
            }
            for [x, y] in [
                [left, top],
                [left, bottom],
                [right, bottom],
                [left, top],
                [right, bottom],
                [right, top],
            ] {
                vertices.push([
                    2.0 * x / size[0] as f32 - 1.0,
                    1.0 - 2.0 * y / size[1] as f32,
                    x - center[0],
                    y - center[1],
                    half[0],
                    half[1],
                    radius,
                ]);
            }
        }
        self.vertices.append(ctx, &vertices);
    }

    /// Opaque plates precede overlay glyphs and never read or modify physical scene depth.
    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        if self.vertices.is_empty() {
            return 0;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(0..self.vertices.len(), 0..1);
        1
    }

    /// Hide plates immediately on label reset without waiting for another preparation.
    pub(super) fn reset(&mut self) {
        self.vertices.reset();
    }

    /// Return grown annotation allocation capacity when the scene is disposed.
    pub(super) fn release(&mut self, ctx: &GpuCtx) {
        self.vertices.release(ctx);
    }

    /// Exact application-owned GPU vertex capacity, independent of Glyphon's private atlas.
    pub(super) fn allocated_bytes(&self) -> u64 {
        self.vertices.buf.size()
    }
}

/// A fixed black quad pipeline; its Always/no-write depth state matches overlay glyphs.
fn pipeline(ctx: &GpuCtx, target: Target) -> wgpu::RenderPipeline {
    let shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text plate shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text_plate.wgsl").into()),
        });
    ctx.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text plates"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 28,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x2, 3 => Float32],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: target.samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        })
}
