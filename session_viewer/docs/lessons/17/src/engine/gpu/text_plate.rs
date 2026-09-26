//! Plates: the box drawn behind a label, with no texture: the shader works out the rounded shape per pixel.
use super::super::buffers::{GpuCtx, GrowBuf, VERTS};
use crate::engine::pipelines::Target;

/// One plate, in framebuffer pixels.
pub(super) struct Rectangle {
    pub(super) bounds: [f32; 4], // left, top, right, bottom
    pub(super) clip: [f32; 4], // the label's clip box
    pub(super) rounded: bool, // corner radius = half the height: a pill
    // --8<-- [start:step-18a]
    pub(super) depth: Option<f32>, // scene depth, or None for an overlay
    pub(super) object: Option<crate::engine::text::TextObject>, // object it belongs to, for picks
}

/// Every plate of the frame, in one vertex buffer.
pub(super) struct Plates {
    vertices: GrowBuf, // six per plate: two triangles
    pipeline: wgpu::RenderPipeline,
    id_pipeline: wgpu::RenderPipeline, // object ids
    physical_vertices: u32, // vertices of depth-tested plates; overlays follow
}

impl Plates {
    /// Starts empty; the buffer grows with the first plates.
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            // 40 bytes per vertex; must match the shader and `pipeline`
            vertices: GrowBuf::new(ctx, "text.plates", 40, VERTS),
            pipeline: pipeline(ctx, target, false),
            id_pipeline: pipeline(ctx, Target::ID, true),
            physical_vertices: 0,
        }
    }

    /// Rebuild the color pipeline for a new MSAA sample count.
    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target, false);
    }

    /// Two triangles per plate, cut to its clip box.
    pub(super) fn prepare(&mut self, ctx: &GpuCtx, rectangles: &[Rectangle], size: [u32; 2]) {
        self.vertices.reset();
        let mut vertices = Vec::with_capacity(rectangles.len() * 6);
        self.physical_vertices = 0;

        for overlay in [false, true] {
            for rectangle in rectangles {
                if rectangle.depth.is_none() != overlay {
                    continue;
                }

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
                        rectangle.depth.unwrap_or(1.0),
                        x - center[0],
                        y - center[1],
                        half[0],
                        half[1],
                        radius,
                        f32::from_bits(rectangle.object.map_or(0, |object| object.row + 1)),
                        if rectangle.object.is_some_and(|object| object.selected) {
                            1.0
                        } else {
                            0.0
                        },
                    ]);
                }
            }

            if !overlay {
                self.physical_vertices = vertices.len() as u32;
            }
        }

        self.vertices.append(ctx, &vertices);
    }

    /// Draw the depth-tested plates, or the overlay ones; returns the draw count.
    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>, overlay: bool) -> u32 {
        let range = if overlay {
            self.physical_vertices..self.vertices.len()
        } else {
            0..self.physical_vertices
        };

        if range.is_empty() {
            return 0;
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(range, 0..1);
        1
    }

    /// Draw every plate's object id; group 0 is the pick transform.
    pub(super) fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        if self.vertices.is_empty() {
            return 0;
        }

        pass.set_pipeline(&self.id_pipeline);
        pass.set_bind_group(0, pick_transform, &[]);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        // --8<-- [end:step-18a]
        pass.draw(0..self.vertices.len(), 0..1);
        1
    }

    /// Forget every rectangle.
    pub(super) fn reset(&mut self) {
        self.vertices.reset();
        // --8<-- [start:step-18b]
        self.physical_vertices = 0;
        // --8<-- [end:step-18b]
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

// --8<-- [start:step-18c]
/// Build the color or id pipeline; depth is read, not written.
fn pipeline(ctx: &GpuCtx, target: Target, ids: bool) -> wgpu::RenderPipeline {
    let shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text plate shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text_plate.wgsl").into()),
        });
    let pick = crate::engine::gpu::frame::pick_transform_layout(ctx);
    let id_layout = ctx
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text plate ids"),
            bind_group_layouts: &[Some(&pick)],
            immediate_size: 0,
        });
    ctx.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text plates"),
            layout: if ids { Some(&id_layout) } else { None },
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some(if ids { "vs_id" } else { "vs_main" }),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 40,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x2, 3 => Float32, 4 => Uint32, 5 => Float32],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some(if ids { "fs_id" } else { "fs_main" }),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target.format,
                    blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) },
                    // --8<-- [end:step-18c]
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                // --8<-- [start:step-18d]
                depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
                // --8<-- [end:step-18d]
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
