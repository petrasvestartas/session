// --8<-- [start:step-1a]
//! Text that always faces the viewer at a fixed pixel size, for labels that must stay readable from any angle.
use super::super::buffers::{GpuCtx, GrowBuf, VERTS};
use crate::engine::pipelines::Target;

/// One label background rectangle, in screen pixels.
pub(super) struct Rectangle {
    pub(super) bounds: [f32; 4], // left, top, right, bottom
    pub(super) clip: [f32; 4], // screen box it is cut to
    pub(super) rounded: bool,
}

/// Draws label backgrounds as rounded rectangles.
pub(super) struct Plates {
    vertices: GrowBuf, // six vertices per rectangle
    pipeline: wgpu::RenderPipeline, // in color
}

impl Plates {
    /// Create the buffer and pipelines.
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 28, VERTS), // 7 floats per vertex
            pipeline: pipeline(ctx, target),
        }
    }

    /// Rebuild the color pipeline for a new MSAA sample count.
    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target);
    }

    /// Build six vertices per rectangle; depth-tested ones first.
    pub(super) fn prepare(&mut self, ctx: &GpuCtx, rectangles: &[Rectangle], size: [u32; 2]) {
        self.vertices.reset();
        let mut vertices = Vec::with_capacity(rectangles.len() * 6);

        for rectangle in rectangles {
            // shape before clipping, for the rounded corners
            let [left, top, right, bottom] = rectangle.bounds;
            let half = [(right - left) * 0.5, (bottom - top) * 0.5];
            let center = [(left + right) * 0.5, (top + bottom) * 0.5];
            let radius = if rectangle.rounded {
                half[0].min(half[1]).max(0.0)
            } else {
                0.0
            };
            // cut to the clip box
            let left = left.max(rectangle.clip[0]);
            let top = top.max(rectangle.clip[1]);
            let right = right.min(rectangle.clip[2]);
            let bottom = bottom.min(rectangle.clip[3]);

            if right <= left || bottom <= top {
                continue;
            }

            // two triangles in clip space
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

// --8<-- [end:step-1a]
    // --8<-- [start:step-1b]
    /// Plates draw before glyphs, ignoring scene depth.
    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        if self.vertices.is_empty() {
            return 0;
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(0..self.vertices.len(), 0..1);
        1
    }

    /// Forget every rectangle.
    pub(super) fn reset(&mut self) {
        self.vertices.reset();
    }

    /// Forget every rectangle and free the buffer.
    pub(super) fn release(&mut self, ctx: &GpuCtx) {
        self.vertices.release(ctx);
    }

    /// Bytes reserved by the vertex buffer.
    pub(super) fn allocated_bytes(&self) -> u64 {
        self.vertices.buf.size()
    }
}

// --8<-- [end:step-1b]
// --8<-- [start:step-1c]
/// A black quad pipeline behind overlay text.
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
                    array_stride: 28, // 7 floats per vertex
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
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // plates are translucent
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
// --8<-- [end:step-1c]
