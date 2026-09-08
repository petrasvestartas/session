//! Selected surface silhouettes, following the archive's coverage-mask algorithm.
//! Source mesh positions, physical depth and picking are unchanged. Only a visible
//! selected surface contributes to the mask; curves and control markers do not.

use std::collections::HashSet;

use super::buffers::GpuCtx;
use super::targets::{Targets, TextureSpec, texture_view};
use crate::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

struct Mask {
    resolved: wgpu::TextureView,
    multisampled: Option<wgpu::TextureView>,
    group: wgpu::BindGroup,
    size: (u32, u32),
    samples: u32,
}

/// Allocates full-frame coverage only while a selection exists. One R8 byte per
/// sample, plus the resolved R8 image at MSAA, with no copied mesh or ID table.
pub struct SelectionOutline {
    selected: HashSet<u32>,
    layout: wgpu::BindGroupLayout,
    uniform: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    mask: Option<Mask>,
}

impl SelectionOutline {
    /// Create the shared mask layout and compositor without allocating framebuffer textures.
    pub fn new(ctx: &GpuCtx, target: Target) -> Self {
        let layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("selection outline"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(16),
                        },
                        count: None,
                    },
                ],
            });
        let uniform = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection outline radius"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let pipeline = pipeline(ctx, &layout, target);
        Self {
            selected: HashSet::new(),
            layout,
            uniform,
            pipeline,
            mask: None,
        }
    }

    /// Track selected source rows; clearing the final row releases coverage immediately.
    pub fn set_selected(&mut self, row: u32, selected: bool) {
        if selected {
            self.selected.insert(row);
        } else {
            self.selected.remove(&row);
        }
        if self.selected.is_empty() {
            self.mask = None;
        }
    }

    /// Clear source selection and its size-dependent coverage resources.
    pub fn reset(&mut self) {
        self.selected.clear();
        self.mask = None;
    }

    /// Rebuild the compositor for the current sample count and discard obsolete coverage.
    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, &self.layout, target);
        self.mask = None;
    }

    /// Return whether to render selected surface coverage this frame.
    pub fn prepare(
        &mut self,
        ctx: &GpuCtx,
        size: (u32, u32),
        samples: u32,
        css_width: f64,
        faces: bool,
    ) -> bool {
        if self.selected.is_empty() || !faces {
            self.mask = None;
            return false;
        }
        let changed = match &self.mask {
            Some(mask) => mask.size != size || mask.samples != samples,
            None => true,
        };
        if changed {
            let usage =
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
            let spec = TextureSpec {
                size,
                format: wgpu::TextureFormat::R8Unorm,
                samples: 1,
                usage,
            };
            let resolved = texture_view(ctx, "selection coverage", &spec);
            let multisampled = if samples > 1 {
                Some(texture_view(
                    ctx,
                    "selection coverage MSAA",
                    &TextureSpec { samples, ..spec },
                ))
            } else {
                None
            };
            let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("selection outline"),
                layout: &self.layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&resolved),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.uniform.as_entire_binding(),
                    },
                ],
            });
            self.mask = Some(Mask {
                resolved,
                multisampled,
                group,
                size,
                samples,
            });
        }
        let radius = (1.5 * f64::from(size.0) / css_width.max(1.0)).clamp(1.0, 8.0) as f32;
        ctx.queue.write_buffer(
            &self.uniform,
            0,
            bytemuck::cast_slice(&[radius, 0.0, 0.0, 0.0]),
        );
        true
    }

    /// Clear coverage while testing the same immutable depth used by the color frame.
    pub fn begin_mask<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        targets: &'a Targets,
    ) -> wgpu::RenderPass<'a> {
        let mask = self
            .mask
            .as_ref()
            .expect("prepare enabled selected coverage");
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("visible selection coverage"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: mask.multisampled.as_ref().unwrap_or(&mask.resolved),
                resolve_target: if mask.multisampled.is_some() {
                    Some(&mask.resolved)
                } else {
                    None
                },
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &targets.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    /// Composite the outside border before control markers and labels.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let Some(mask) = &self.mask else {
            return 0;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &mask.group, &[]);
        pass.draw(0..3, 0..1);
        1
    }

    /// Exact owned buffer and texture payload, excluding driver allocation overhead.
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let textures = match &self.mask {
            Some(mask) => {
                let samples = if mask.samples > 1 {
                    u64::from(mask.samples) + 1
                } else {
                    1
                };
                u64::from(mask.size.0) * u64::from(mask.size.1) * samples
            }
            None => 0,
        };
        (self.uniform.size(), textures)
    }
}

/// Build outside-coverage compositing for the pass's color format and sample count.
fn pipeline(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, target: Target) -> wgpu::RenderPipeline {
    let shader = module(
        &ctx.device,
        "selection outline",
        include_str!("../../shaders/selection_outline.wgsl"),
    );
    let groups = [layout];
    let desc = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .with("black selection outline", "fs_main")
        .depth(DepthMode::Always)
        .color(ColorWrite::Blended);
    build(&ctx.device, target, &desc)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
    use session_rust::{RenderVertex, Xform};

    fn quad(upload: &mut Upload, extent: f32, depth: f32) {
        let row = upload.obj.rows.len() as u32;
        let first = upload.arena.verts.len() as u32;
        upload.obj.rows.push(ObjectRow::new(Xform::identity().m, 0));
        for [x, y] in [
            [-extent, -extent],
            [extent, -extent],
            [extent, extent],
            [-extent, extent],
        ] {
            upload.arena.verts.push(RenderVertex {
                position: [x, y, depth],
                normal: [0.0, 0.0, 1.0],
                color: [0.3, 0.5, 0.7, 1.0],
            });
            upload.arena.vids.push(row);
        }
        upload
            .arena
            .idx
            .extend([0, 1, 2, 0, 2, 3].map(|i| first + i));
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn selected_silhouette_is_black_visible_only_and_releases_coverage() {
        let mut gpu = pollster::block_on(Gpu::new_headless(200, 200)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.lit = false;
        let mut upload = Upload::default();
        quad(&mut upload, 0.5, 0.5);
        quad(&mut upload, 0.8, 0.7);
        gpu.set_scene(&upload);
        let input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        for samples in [1, 4, 1] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(200, 200);
            let hidden = gpu.render_offscreen(&input);
            gpu.set_selected(0, true);
            assert_eq!(
                hidden,
                gpu.render_offscreen(&input),
                "an entirely occluded selection has no outline or yellow pixels"
            );
            gpu.set_hidden(1, true);
            let selected = gpu.render_offscreen(&input);
            let black = selected
                .chunks_exact(4)
                .filter(|p| p[..3].iter().all(|c| *c < 8))
                .count();
            let yellow = selected
                .chunks_exact(4)
                .filter(|p| p[0] > 240 && p[1] > 240 && p[2] < 8)
                .count();
            assert!(
                black > 350,
                "selected silhouette must have a continuous black border: {black}"
            );
            assert!(
                yellow > 9500,
                "the original selection fill stays yellow: {yellow}"
            );
            let selected_ids = gpu.render_ids_offscreen(&input);
            let capacity = gpu.selection_outline.allocated_bytes().1;
            assert_eq!(capacity, 200 * 200 * if samples > 1 { 5 } else { 1 });
            gpu.set_selected(0, false);
            assert_eq!(
                gpu.selection_outline.allocated_bytes().1,
                0,
                "clearing selection releases coverage immediately"
            );
            let plain = gpu.render_offscreen(&input);
            assert_eq!(
                selected_ids,
                gpu.render_ids_offscreen(&input),
                "a visual border must not add pickable geometry"
            );
            assert!(
                plain
                    .chunks_exact(4)
                    .all(|p| p[0] > 8 || p[1] > 8 || p[2] > 8)
            );
            gpu.set_hidden(1, false);
        }
        gpu.release();
        assert_eq!(gpu.selection_outline.allocated_bytes(), (16, 0));
    }
}
