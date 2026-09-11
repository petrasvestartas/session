//! Black surface silhouettes from visible coverage. Combined solid silhouettes and
//! selected boundaries share allocation, depth testing and compositing.
//! The mask never changes geometry or picking, and cannot include hidden surfaces.

use std::collections::HashSet;

use super::buffers::GpuCtx;
use super::targets::{Targets, TextureSpec, texture_view};
use crate::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

struct Mask {
    resolved: wgpu::TextureView,
    multisampled: Option<wgpu::TextureView>,
    group: wgpu::BindGroup,
    /// The maximum of each `POOL`-square block of `resolved`: the compositor skips every
    /// pixel whose neighbourhood of blocks is empty, which is most of the frame.
    coarse: wgpu::TextureView,
    coarse_size: (u32, u32),
    pool_group: wgpu::BindGroup,
    size: (u32, u32),
    samples: u32,
}

/// Mask texels per coarse texel; matches `POOL` in the shader and exceeds the largest kernel.
const POOL: u32 = 16;

/// What a coverage mask depends on. While none of it changes, the mask passes are skipped
/// and the previous masks are composited again: a still view costs no rasterization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaskKey {
    pub mvp: [f32; 16],
    pub geometry: u64,
    pub selection: u64,
    pub faces: u64,
    pub size: (u32, u32),
    pub samples: u32,
    /// The edges are part of the coverage, so the mask follows their toggle and pen.
    pub edges: bool,
    pub pen: u32,
}

/// Which visible surfaces contribute to a shared, antialiased coverage mask.
#[derive(Clone, Copy, PartialEq)]
pub enum OutlineKind {
    Selected,
    AllSolids,
}

/// Visible surface coverage uses the existing arena geometry and read-only physical depth.
pub struct SurfaceOutline {
    kind: OutlineKind,
    selected: HashSet<u32>,
    layout: wgpu::BindGroupLayout,
    pool_layout: wgpu::BindGroupLayout,
    uniform: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    pool_pipeline: wgpu::RenderPipeline,
    mask: Option<Mask>,
    /// The key the current mask contents were rendered for.
    valid_for: Option<MaskKey>,
}

impl SurfaceOutline {
    /// Create the shared mask layout and compositor without allocating framebuffer textures.
    pub fn new(ctx: &GpuCtx, target: Target, kind: OutlineKind) -> Self {
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
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });
        let pool_layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("selection outline pool"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
            });
        let uniform = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection outline radius"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let pipeline = pipeline(ctx, &layout, target);
        let pool_pipeline = pool_pipeline(ctx, &pool_layout);
        Self {
            kind,
            selected: HashSet::new(),
            layout,
            pool_layout,
            uniform,
            pipeline,
            pool_pipeline,
            mask: None,
            valid_for: None,
        }
    }

    /// Whether the mask holds the coverage for `key`.
    pub fn is_valid(&self, key: &MaskKey) -> bool {
        self.mask.is_some() && self.valid_for.as_ref() == Some(key)
    }

    /// The mask was just rendered for `key`.
    pub fn mark_valid(&mut self, key: MaskKey) {
        self.valid_for = Some(key);
    }

    /// This mask's attachment: the multisampled view when there is one, resolving into the
    /// single-sample view the compositor reads.
    fn attachment(&self) -> Option<wgpu::RenderPassColorAttachment<'_>> {
        let mask = self.mask.as_ref()?;
        Some(wgpu::RenderPassColorAttachment {
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
        })
    }

    /// One pass writing the solid mask and the selected mask together, so the faces are
    /// rasterized once for both.
    pub fn begin_masks<'a>(
        solid: &'a Self,
        selected: &'a Self,
        encoder: &'a mut wgpu::CommandEncoder,
        targets: &'a Targets,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("visible coverage masks"),
            color_attachments: &[solid.attachment(), selected.attachment()],
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

    /// Track selected source rows; clearing the final row releases coverage immediately.
    pub fn set_selected(&mut self, row: u32, selected: bool) {
        if selected {
            self.selected.insert(row);
        } else {
            self.selected.remove(&row);
        }
        if self.kind == OutlineKind::Selected && self.selected.is_empty() {
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

    /// Allocate one R8 byte per sample plus its MSAA resolve while this outline is enabled.
    pub fn prepare(
        &mut self,
        ctx: &GpuCtx,
        size: (u32, u32),
        samples: u32,
        css_width: f64,
        faces: bool,
    ) -> bool {
        if (self.kind == OutlineKind::Selected && self.selected.is_empty()) || !faces {
            self.mask = None;
            self.valid_for = None;
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
            let coarse_size = (size.0.div_ceil(POOL).max(1), size.1.div_ceil(POOL).max(1));
            let coarse = texture_view(
                ctx,
                "selection coverage coarse",
                &TextureSpec {
                    size: coarse_size,
                    ..spec
                },
            );
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
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&coarse),
                    },
                ],
            });
            let pool_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("selection outline pool"),
                layout: &self.pool_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&resolved),
                }],
            });
            self.mask = Some(Mask {
                resolved,
                multisampled,
                group,
                coarse,
                coarse_size,
                pool_group,
                size,
                samples,
            });
            self.valid_for = None;
        }
        // One radius for every silhouette: an ordinary solid's outline is as heavy as a
        // selected one's, the selection differing by its yellow fill, not its border.
        let css_radius = 1.6875;
        let radius = (css_radius * f64::from(size.0) / css_width.max(1.0)).clamp(1.0, 12.0) as f32;
        ctx.queue.write_buffer(
            &self.uniform,
            0,
            bytemuck::cast_slice(&[
                radius,
                if self.kind == OutlineKind::Selected {
                    1.0
                } else {
                    0.0
                },
                0.0,
                0.0,
            ]),
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

    /// After the mask pass: reduce the resolved mask to its block maxima, so the compositor
    /// can skip the pixels that no covered texel can reach.
    pub fn encode_pool(&self, encoder: &mut wgpu::CommandEncoder) {
        let Some(mask) = self.mask.as_ref() else {
            return;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("selection coverage pool"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &mask.coarse,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.pool_pipeline);
        pass.set_bind_group(0, &mask.pool_group, &[]);
        pass.draw(0..3, 0..1);
    }

    /// Composite both silhouettes once. Maximum coverage lets the selected
    /// border win without painting the overlapping antialias fringe twice.
    pub fn draw_combined(&self, selected: &Self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let Some(normal) = self.mask.as_ref().or(selected.mask.as_ref()) else {
            return 0;
        };
        let selection = selected.mask.as_ref().unwrap_or(normal);
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &normal.group, &[]);
        pass.set_bind_group(1, &selection.group, &[]);
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
                    + u64::from(mask.coarse_size.0) * u64::from(mask.coarse_size.1)
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
        include_str!("../../shaders/surface_outline.wgsl"),
    );
    let groups = [layout, layout];
    let desc = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .with("black selection outline", "fs_main")
        .depth(DepthMode::Always)
        .color(ColorWrite::Blended);
    build(&ctx.device, target, &desc)
}

/// The block-maximum reduction: a full-screen triangle over the coarse texture, no depth.
fn pool_pipeline(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    let shader = module(
        &ctx.device,
        "selection outline pool",
        include_str!("../../shaders/surface_outline.wgsl"),
    );
    let groups = [layout];
    let desc = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .with("selection outline pool", "fs_pool")
        .depth(DepthMode::Detached);
    let target = Target {
        format: wgpu::TextureFormat::R8Unorm,
        samples: 1,
    };
    build(&ctx.device, target, &desc)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
    use session_rust::{RenderVertex, Xform};

    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn selected_cad_edges_do_not_paint_over_the_black_silhouette() {
        use crate::app::scene::{FileDoc, Scene};
        use crate::camera::Camera;
        use session_rust::{BRep, Session};
        use std::rc::Rc;

        let mut gpu = pollster::block_on(Gpu::new_headless(480, 480)).unwrap();
        gpu.view.show_outlines = true;
        gpu.view.show_grid = false;
        gpu.view.markers = false;
        for shape in [
            BRep::create_cone(150.0, 400.0),
            BRep::create_cylinder(150.0, 400.0),
        ] {
            gpu.reset();
            let mut source = Session::new("selected silhouette regression");
            source.add_brep(shape, None);
            let mut scene = Scene::new();
            scene.add_file(FileDoc {
                name: "solid".into(),
                session: Rc::new(source),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            });
            scene.upload_to(&mut gpu);
            gpu.set_selected(0, true);
            for samples in [1, 4] {
                for dpr in [1.0, 2.0] {
                    gpu.logical_size = [480.0 / dpr; 2];
                    gpu.view.msaa_forced = Some(samples);
                    gpu.resize(480, 480);
                    for orbit in [(0.0, 0.0), (95.0, -65.0), (-80.0, 130.0)] {
                        let mut camera = Camera::new();
                        camera.fit(&gpu.bounds, 1.0);
                        camera.orbit(orbit.0, orbit.1);
                        let rebase =
                            gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
                        let input = FrameInput {
                            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
                            clear: wgpu::Color::WHITE,
                            now_ms: 0.0,
                        };
                        gpu.view.show_mesh_edges = false;
                        let silhouette = gpu.render_offscreen(&input);
                        gpu.view.show_mesh_edges = true;
                        let edged = gpu.render_offscreen(&input);
                        let mut black = 0;
                        for (plain, inked) in silhouette.chunks_exact(4).zip(edged.chunks_exact(4))
                        {
                            if plain[..3].iter().all(|channel| *channel < 8) {
                                black += 1;
                                assert!(
                                    inked[..3].iter().all(|channel| *channel < 12),
                                    "yellow CAD strokes must not narrow the black border: {plain:?} -> {inked:?}; samples={samples}, DPR={dpr}, orbit={orbit:?}"
                                );
                            }
                        }
                        assert!(
                            black > 500,
                            "the perspective silhouette must remain visible"
                        );
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn touching_and_overlapping_solids_have_one_continuous_outline() {
        let mut gpu = pollster::block_on(Gpu::new_headless(200, 200)).unwrap();
        gpu.view.show_outlines = true;
        gpu.view.show_grid = false;
        gpu.view.lit = false;
        let input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        for samples in [1, 4] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(200, 200);
            for overlap in [false, true] {
                gpu.reset();
                let mut upload = Upload::default();
                quad(&mut upload, 0.4, 0.5);
                quad(&mut upload, 0.4, 0.6);
                for (i, vertex) in upload.arena.verts.iter_mut().enumerate() {
                    vertex.position[0] += if i < 4 {
                        -0.4
                    } else if overlap {
                        0.2
                    } else {
                        0.4
                    };
                }
                gpu.set_scene(&upload);
                let outlined = gpu.render_offscreen(&input);
                gpu.view.show_outlines = false;
                let plain = gpu.render_offscreen(&input);
                let ids = gpu.render_ids_offscreen(&input);
                gpu.view.show_outlines = true;
                assert_eq!(ids, gpu.render_ids_offscreen(&input));
                for y in 65..135 {
                    for x in 45..150 {
                        let at = (y * 200 + x) * 4;
                        assert_eq!(
                            &outlined[at..at + 4],
                            &plain[at..at + 4],
                            "no outline inside the combined silhouette, including object joins"
                        );
                    }
                }
                assert_ne!(
                    outlined, plain,
                    "the combined outside silhouette must still be outlined"
                );
            }
        }
    }

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
        gpu.view.show_outlines = true;
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
            // Suppress only the ordinary mask: the selected object's pixels must stay
            // identical, including fractional antialias coverage along its silhouette.
            gpu.solid_outline.kind = super::OutlineKind::Selected;
            assert_eq!(
                selected,
                gpu.render_offscreen(&input),
                "selection must not receive a second overlapping outline"
            );
            gpu.solid_outline.kind = super::OutlineKind::AllSolids;
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
            assert_eq!(
                capacity,
                200 * 200 * if samples > 1 { 5 } else { 1 } + 13 * 13
            );
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
            let plain_black = plain
                .chunks_exact(4)
                .filter(|p| p[..3].iter().all(|c| *c < 8))
                .count();
            assert!(
                plain_black > 0 && plain_black < black,
                "ordinary and selected outlines both stay visible"
            );
            gpu.view.show_outlines = false;
            let disabled = gpu.render_offscreen(&input);
            assert!(
                disabled
                    .chunks_exact(4)
                    .all(|p| p[0] > 8 || p[1] > 8 || p[2] > 8)
            );
            assert_eq!(
                selected_ids,
                gpu.render_ids_offscreen(&input),
                "outline toggling never changes source picking"
            );
            gpu.view.show_outlines = true;
            gpu.set_hidden(1, false);
        }
        gpu.release();
        assert_eq!(gpu.selection_outline.allocated_bytes(), (16, 0));
    }
}
