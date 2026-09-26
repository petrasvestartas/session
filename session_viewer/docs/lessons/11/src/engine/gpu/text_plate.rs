// --8<-- [start:step-1a]
//! Plates: the box drawn behind a label, with no texture: the shader works out the rounded shape per pixel.
use super::super::buffers::{GpuCtx, GrowBuf, VERTS};
use crate::engine::pipelines::Target;

/// One plate, in framebuffer pixels.
pub(super) struct Rectangle {
    pub(super) bounds: [f32; 4], // left, top, right, bottom
    pub(super) clip: [f32; 4], // the label's clip box
    pub(super) rounded: bool, // corner radius = half the height: a pill
}

/// Every plate of the frame, in one vertex buffer.
pub(super) struct Plates {
    vertices: GrowBuf, // six per plate: two triangles
    pipeline: wgpu::RenderPipeline,
}

impl Plates {
    /// Starts empty; the buffer grows with the first plates.
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            // 7 floats = 28 bytes: clip position 2, offset from the center 2, half size 2, radius 1
            vertices: GrowBuf::new(ctx, "text.plates", 28, VERTS),
            pipeline: pipeline(ctx, target),
        }
    }

    /// Rebuild the color pipeline for a new MSAA sample count.
    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target);
    }

    /// Two triangles per plate, cut to its clip box.
    pub(super) fn prepare(&mut self, ctx: &GpuCtx, rectangles: &[Rectangle], size: [u32; 2]) {
        self.vertices.reset();
        let mut vertices = Vec::with_capacity(rectangles.len() * 6);

        for rectangle in rectangles {
            // measure before cutting, so a cut plate keeps its true corners
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

            // pixels to clip space: x 0..width becomes -1..1, and y flips because clip y points up
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
    /// Draw before the overlay text, so the glyphs land on top.
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
/// Depth test Always: a plate is never hidden by the scene.
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
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // alpha = edge coverage: a smooth rim
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
