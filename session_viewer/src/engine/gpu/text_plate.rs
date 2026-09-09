//! Annotation plates share the text lane's target and depth policy; selected sources are yellow.
use super::super::buffers::{GpuCtx, GrowBuf, VERTS};
use crate::engine::pipelines::Target;

/// Original physical plate bounds and independent clip bounds preserve corner shape when clipped.
pub(super) struct Rectangle {
    pub(super) bounds: [f32; 4],
    pub(super) clip: [f32; 4],
    pub(super) rounded: bool,
    pub(super) depth: Option<f32>,
    pub(super) object: Option<crate::engine::text::TextObject>,
}

/// Batched physical rectangles, with no DOM, source geometry or independent device ownership.
pub(super) struct Plates {
    vertices: GrowBuf,
    pipeline: wgpu::RenderPipeline,
    id_pipeline: wgpu::RenderPipeline,
    physical_vertices: u32,
}

impl Plates {
    /// Start with one vertex of capacity and an empty draw list.
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 40, VERTS),
            pipeline: pipeline(ctx, target, false),
            id_pipeline: pipeline(ctx, Target::ID, true),
            physical_vertices: 0,
        }
    }

    /// Match sample count and color target when the surrounding text pass changes.
    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target, false);
    }

    /// Upload clipped quads with original center, half size and maximum corner radius.
    pub(super) fn prepare(&mut self, ctx: &GpuCtx, rectangles: &[Rectangle], size: [u32; 2]) {
        self.vertices.reset();
        let mut vertices = Vec::with_capacity(rectangles.len() * 6);
        self.physical_vertices = 0;
        for overlay in [false, true] {
            for rectangle in rectangles {
                if rectangle.depth.is_none() != overlay {
                    continue;
                }
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

    /// Draw the depth-tested plates before anchored glyphs, then overlay plates before overlay glyphs.
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

    /// The same clipped, rounded quads provide IDs for every selectable camera-facing label.
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
        pass.draw(0..self.vertices.len(), 0..1);
        1
    }

    /// Hide plates immediately on label reset without waiting for another preparation.
    pub(super) fn reset(&mut self) {
        self.vertices.reset();
        self.physical_vertices = 0;
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

/// Read-only physical depth; overlays use near depth 1 and source labels use their anchor depth.
/// The ID pipeline maps its clip-space vertices through the pick pass's window transform.
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
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
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
