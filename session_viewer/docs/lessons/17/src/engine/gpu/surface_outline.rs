// --8<-- [start:outline-mask]
// Coverage mask = a one-byte texture: 1 where a surface covers the pixel, 0 elsewhere, a fraction along an edge.
// Outline = black on the pixels just outside a mask: each pixel searches its neighbours for coverage.
// Coarse mask = one pixel per 4 x 4 or 8 x 8 block holding its maximum, so empty blocks are skipped in one read.
use std::collections::HashSet;

use super::Gpu;
use super::buffers::GpuCtx;
use super::frame::Binds;
use super::pass::{Frame, Pass};
use super::targets::{Attachment, Targets, TextureSpec};
use crate::engine::pipelines::Layouts;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Pipeline, PipelineDesc, Target, build, module,
};

/// One coverage mask: where the surfaces are on screen.
struct Mask {
    resolved: Attachment,             // coverage, one sample per pixel
    multisampled: Option<Attachment>, // coverage at the scene's sample count
    group: wgpu::BindGroup,           // mask, radius and dilated coarse mask, for the compositor
    coarse: Attachment,               // max of each block
    dilated: Attachment,              // max of the 3x3 blocks around each; empty ones are skipped
    coarse_size: (u32, u32),          // coarse texture size
    block: u32,                       // mask pixels per coarse pixel
    pool_group: wgpu::BindGroup,      // the mask, for the coarse pass
    dilate_group: wgpu::BindGroup,    // the coarse mask, for the dilate pass
    size: (u32, u32),                 // mask size, px
    samples: u32,                     // MSAA samples
}

impl Mask {
    /// Keep the drawn samples only when there is no resolve: once averaged into `resolved`, nothing reads them again.
    fn store(&self) -> wgpu::StoreOp {
        if self.multisampled.is_some() {
            wgpu::StoreOp::Discard
        } else {
            wgpu::StoreOp::Store
        }
    }
}
// --8<-- [end:outline-mask]

// --8<-- [start:outline-tables]
// Tap = one neighbour offset the search reads; sorted nearest first, the shader can stop early.
/// Most offsets the tap table holds; must match the shader.
const TAPS: usize = 512;

/// The sorted offset table the alpha pass walks.
struct Table {
    buffer: wgpu::Buffer,   // count, then (dx, dy, weight, distance) per tap
    group: wgpu::BindGroup, // the buffer, for the alpha pass
    radius: f32,            // radius the taps were built for
}

/// Outline alpha of both masks, redrawn only when a mask changes.
struct Alpha {
    texture: Attachment,    // one byte per pixel
    group: wgpu::BindGroup, // the texture, for the compositor
    size: (u32, u32),       // texture size, px
}

/// Faces into the mask from the face pass's triangle ids, instead of drawing them again.
struct FaceCoverage {
    layout: wgpu::BindGroupLayout, // the face pass's triangle id texture
    fraction: Pipeline,            // share of face samples, straight into the one-sample mask
    with_edges: Pipeline,          // face samples inside a mask pass, before the edges
    group: Option<(wgpu::TextureView, wgpu::BindGroup)>, // the texture it binds
}
// --8<-- [end:outline-tables]

// --8<-- [start:outline-struct]
/// What a mask depends on; same key = reuse the old mask.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaskKey {
    pub mvp: [f32; 16],   // camera matrix
    pub geometry: u64,    // object change count
    pub selection: u64,   // selection change count
    pub faces: u64,       // face selection change count
    pub size: (u32, u32), // canvas size, px
    pub samples: u32,     // MSAA samples
    pub edges: bool,      // edges shown
    pub pen: u32,         // pen width bits
}

/// Which surfaces the outline goes around.
#[derive(Clone, Copy, PartialEq)]
pub enum OutlineKind {
    Selected,  // selected objects only
    AllSolids, // every solid
}

/// Draws a black outline around surfaces from a coverage mask.
pub struct SurfaceOutline {
    kind: OutlineKind,                   // which surfaces
    selected: HashSet<u32>,              // selected object rows
    layout: wgpu::BindGroupLayout,       // mask, radius, coarse mask
    pool_layout: wgpu::BindGroupLayout,  // one mask and the radius
    table_layout: wgpu::BindGroupLayout, // the tap table
    table: Option<Table>,                // taps, only on the compositing outline
    alpha_layout: wgpu::BindGroupLayout, // the outline alpha
    alpha: Option<Alpha>,                // outline alpha, only on the compositing outline
    alpha_for: Option<(bool, bool)>,     // which masks the alpha was drawn from
    alpha_pipeline: Pipeline,            // searches both masks into the alpha
    uniform: wgpu::Buffer,               // radius in px and a selected flag
    pipeline: Pipeline,                  // draws the outline
    pool_pipeline: Pipeline,             // shrinks the mask to blocks
    dilate_pipeline: Pipeline,           // grows the blocks by one
    mask: Option<Mask>,                  // current mask textures
    valid_for: Option<MaskKey>,          // what the mask was drawn for
    faces: Option<FaceCoverage>,         // faces from triangle ids, only around every solid
}
// --8<-- [end:outline-struct]

// --8<-- [start:outline-new]
impl SurfaceOutline {
    /// Create the layouts and pipelines; textures come with the first frame.
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
        let table_layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("selection outline taps"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(16 + 16 * TAPS as u64),
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
        let alpha_layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("selection outline alpha"),
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
        let pipeline = pipeline(ctx, &alpha_layout, target);
        let alpha_pipeline = alpha_pipeline(ctx, &layout, &table_layout);
        let dilate_pipeline = pool_pipeline(ctx, &pool_layout, "fs_dilate");
        let pool_pipeline = pool_pipeline(ctx, &pool_layout, "fs_pool");
        Self {
            kind,
            selected: HashSet::new(),
            layout,
            pool_layout,
            table_layout,
            table: None,
            alpha_layout,
            alpha: None,
            alpha_for: None,
            alpha_pipeline,
            uniform,
            pipeline,
            pool_pipeline,
            dilate_pipeline,
            mask: None,
            valid_for: None,
            // `bool::then` runs the closure only when true: Some(coverage) around every solid, None around the selection
            faces: (kind == OutlineKind::AllSolids).then(|| face_coverage(ctx, target.samples)),
        }
    }
    // --8<-- [end:outline-new]

    // --8<-- [start:outline-masks]
    /// True when the mask was drawn for `key`.
    pub fn is_valid(&self, key: &MaskKey) -> bool {
        self.mask.is_some() && self.valid_for.as_ref() == Some(key)
    }

    /// Record that the mask was drawn for `key`.
    pub fn mark_valid(&mut self, key: MaskKey) {
        self.valid_for = Some(key);
    }

    /// The mask as a render target, resolving MSAA into `resolved`.
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
                store: mask.store(),
            },
        })
    }

    /// Open one pass that writes both masks at once.
    pub fn begin_masks<'a>(
        solid: &'a Self,
        selected: &'a Self,
        encoder: &'a mut wgpu::CommandEncoder,
        targets: &'a Targets,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("visible coverage masks"),
            // two targets in one pass: the shader writes @location(0) to the solid mask, @location(1) to the selection mask
            color_attachments: &[solid.attachment(), selected.attachment()],
            // the scene depth, read only: a surface behind another fails the depth test and leaves no coverage
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

    /// Remember whether object `row` is selected.
    pub fn set_selected(&mut self, row: u32, selected: bool) {
        if selected {
            self.selected.insert(row);
        } else {
            self.selected.remove(&row);
        }

        // nothing selected: free the mask
        if self.kind == OutlineKind::Selected && self.selected.is_empty() {
            self.mask = None;
        }
    }

    /// Forget the selection and drop the mask.
    pub fn reset(&mut self) {
        self.selected.clear();
        self.mask = None;
        self.free_alpha();
    }

    /// Rebuild the pipeline for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, &self.alpha_layout, target);

        if self.faces.is_some() {
            self.faces = Some(face_coverage(ctx, target.samples));
        }

        self.mask = None;
        self.free_alpha();
    }
    // --8<-- [end:outline-masks]

    // --8<-- [start:outline-prepare]
    /// Make sure the mask textures exist; returns false when no outline is due.
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

        let radius = radius(size, css_width);
        let block = block(radius);
        // remake the textures when size, samples or block changed
        let changed = match &self.mask {
            Some(mask) => mask.size != size || mask.samples != samples || mask.block != block,
            None => true,
        };

        if changed {
            let usage =
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
            let spec = TextureSpec {
                size,
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                samples: 1,
                usage,
            };
            let resolved = Attachment::new(ctx, "selection coverage", &spec);
            let multisampled = if samples > 1 {
                // only resolved, never read: tilers may keep it on chip
                let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TRANSIENT;
                Some(Attachment::new(
                    ctx,
                    "selection coverage MSAA",
                    &TextureSpec {
                        samples,
                        usage,
                        ..spec
                    },
                ))
            } else {
                None
            };
            let coarse_size = (size.0.div_ceil(block).max(1), size.1.div_ceil(block).max(1));
            let coarse_spec = TextureSpec {
                size: coarse_size,
                ..spec
            };
            let coarse = Attachment::new(ctx, "selection coverage coarse", &coarse_spec);
            let dilated = Attachment::new(ctx, "selection coverage dilated", &coarse_spec);
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
                        resource: wgpu::BindingResource::TextureView(&dilated),
                    },
                ],
            });
            let pool_group = self.pool_group(ctx, &resolved);
            let dilate_group = self.pool_group(ctx, &coarse);
            self.mask = Some(Mask {
                resolved,
                multisampled,
                group,
                coarse,
                dilated,
                coarse_size,
                block,
                pool_group,
                dilate_group,
                size,
                samples,
            });
            self.valid_for = None;
        }

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
                block as f32,
                0.0,
            ]),
        );
        true
    }

    /// Bind `source` and the radius for a pool or dilate pass.
    fn pool_group(&self, ctx: &GpuCtx, source: &wgpu::TextureView) -> wgpu::BindGroup {
        ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("selection outline pool"),
            layout: &self.pool_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(source),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.uniform.as_entire_binding(),
                },
            ],
        })
    }
    // --8<-- [end:outline-prepare]

    // --8<-- [start:outline-alpha]
    /// Free the tap table and alpha texture.
    fn free_alpha(&mut self) {
        self.table = None;
        self.alpha = None;
        self.alpha_for = None;
    }

    /// Make the tap table and alpha texture for `radius` and `size`, or free them when None.
    pub fn prepare_alpha(&mut self, ctx: &GpuCtx, radius: Option<f32>, size: (u32, u32)) {
        let Some(radius) = radius else {
            self.free_alpha();
            return;
        };

        if self.alpha.as_ref().is_none_or(|alpha| alpha.size != size) {
            self.alpha = None;
            let spec = TextureSpec {
                size,
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                samples: 1,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
            };
            let texture = Attachment::new(ctx, "selection outline alpha", &spec);
            let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("selection outline alpha"),
                layout: &self.alpha_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture),
                }],
            });
            self.alpha = Some(Alpha {
                texture,
                group,
                size,
            });
            self.alpha_for = None;
        }

        if self
            .table
            .as_ref()
            .is_some_and(|table| table.radius == radius)
        {
            return;
        }

        self.alpha_for = None;

        let buffer = self
            .table
            .take()
            .map(|table| table.buffer)
            .unwrap_or_else(|| {
                ctx.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("selection outline taps"),
                    size: 16 + 16 * TAPS as u64,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                })
            });
        let taps = taps(radius);
        let mut data = vec![[0.0_f32; 4]; TAPS + 1];
        data[0][0] = f32::from_bits(taps.len() as u32);
        data[1..=taps.len()].copy_from_slice(&taps);
        ctx.queue
            .write_buffer(&buffer, 0, bytemuck::cast_slice(&data));
        let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("selection outline taps"),
            layout: &self.table_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        self.table = Some(Table {
            buffer,
            group,
            radius,
        });
    }
    // --8<-- [end:outline-alpha]

    // --8<-- [start:outline-passes]
    /// Open the pass that draws this mask, cleared, against the scene depth.
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
                    store: mask.store(),
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

    /// Bind the face pass's triangle ids for the face coverage draws.
    pub fn bind_faces(&mut self, ctx: &GpuCtx, targets: &Targets) {
        let Some(faces) = self.faces.as_mut() else {
            return;
        };

        if faces
            .group
            .as_ref()
            .is_some_and(|(view, _)| *view == targets.gradient.view)
        {
            return;
        }

        let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("face coverage"),
            layout: &faces.layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&targets.gradient),
            }],
        });
        faces.group = Some((targets.gradient.view.clone(), group));
    }

    /// Faces into an open mask pass from the triangle ids; returns the draw count.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let Some(FaceCoverage {
            with_edges,
            group: Some((_, group)),
            ..
        }) = &self.faces
        else {
            return 0;
        };
        pass.set_pipeline(with_edges);
        pass.set_bind_group(0, group, &[]);
        pass.draw(0..3, 0..1);
        1
    }

    /// Faces straight into the one-sample mask from the triangle ids; returns the draw count.
    pub fn encode_faces(&self, encoder: &mut wgpu::CommandEncoder) -> u32 {
        let (
            Some(mask),
            Some(FaceCoverage {
                fraction,
                group: Some((_, group)),
                ..
            }),
        ) = (&self.mask, &self.faces)
        else {
            return 0;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("face coverage"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &mask.resolved,
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
        pass.set_pipeline(fraction);
        pass.set_bind_group(0, group, &[]);
        pass.draw(0..3, 0..1);
        1
    }

    /// Shrink the mask to its block maxima, then grow those by one block.
    pub fn encode_pool(&self, encoder: &mut wgpu::CommandEncoder) {
        let Some(mask) = self.mask.as_ref() else {
            return;
        };

        for (target, pipeline, group) in [
            (&mask.coarse, &self.pool_pipeline, &mask.pool_group),
            (&mask.dilated, &self.dilate_pipeline, &mask.dilate_group),
        ] {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("selection coverage pool"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
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
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, group, &[]);
            pass.draw(0..3, 0..1);
        }
    }
    // --8<-- [end:outline-passes]

    // --8<-- [start:outline-composite]
    /// Search both masks into the alpha texture, when forced or the masks in use changed.
    pub fn encode_alpha(
        &mut self,
        selected: &Self,
        encoder: &mut wgpu::CommandEncoder,
        force: bool,
    ) {
        let from = (self.mask.is_some(), selected.mask.is_some());

        if !force && self.alpha_for == Some(from) {
            return;
        }

        let Some(normal) = self.mask.as_ref().or(selected.mask.as_ref()) else {
            return;
        };
        let (Some(alpha), Some(table)) = (self.alpha.as_ref(), self.table.as_ref()) else {
            return;
        };
        let selection = selected.mask.as_ref().unwrap_or(normal);
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("selection outline alpha"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &alpha.texture,
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
        pass.set_pipeline(&self.alpha_pipeline);
        pass.set_bind_group(0, &normal.group, &[]);
        pass.set_bind_group(1, &selection.group, &[]);
        pass.set_bind_group(2, &table.group, &[]);
        pass.draw(0..3, 0..1);
        // the pass borrows `self.alpha`; `drop` ends it here so the next line may write `self`
        drop(pass);
        self.alpha_for = Some(from);
    }

    /// Blend the outline alpha over the scene; returns the draw count.
    pub fn draw_combined(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let Some(alpha) = self.alpha.as_ref().filter(|_| self.alpha_for.is_some()) else {
            return 0;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &alpha.group, &[]);
        pass.draw(0..3, 0..1);
        1
    }

    /// Bytes reserved on the GPU: (buffers, textures).
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let textures = match &self.mask {
            Some(mask) => {
                // MSAA keeps every sample plus the one-sample resolve: at 4x a mask costs 5 bytes a pixel
                let samples = if mask.samples > 1 {
                    u64::from(mask.samples) + 1
                } else {
                    1
                };
                u64::from(mask.size.0) * u64::from(mask.size.1) * samples
                    + 2 * u64::from(mask.coarse_size.0) * u64::from(mask.coarse_size.1)
            }
            None => 0,
        };
        let table = self.table.as_ref().map_or(0, |table| table.buffer.size());
        let alpha = self
            .alpha
            .as_ref()
            .map_or(0, |alpha| u64::from(alpha.size.0) * u64::from(alpha.size.1));
        (self.uniform.size() + table, textures + alpha)
    }
}
// --8<-- [end:outline-composite]

// --8<-- [start:outline-radius]
/// Outline width in framebuffer pixels for a canvas `size` shown `css_width` CSS pixels wide.
pub fn radius(size: (u32, u32), css_width: f64) -> f32 {
    // outline width in CSS pixels
    let css_radius = 1.6875;
    // at most 12 px, so the tap table fits
    (css_radius * f64::from(size.0) / css_width.max(1.0)).clamp(1.0, 12.0) as f32
}

/// Mask pixels per coarse pixel: a power of two, at least the search extent.
fn block(radius: f32) -> u32 {
    ((radius + 0.5).ceil() as u32).next_power_of_two().max(4)
}

/// Offsets within `radius + 0.5` px, nearest first: (dx, dy, weight, distance).
fn taps(radius: f32) -> Vec<[f32; 4]> {
    let extent = (radius + 0.5).ceil() as i32;
    let mut taps = Vec::new();

    for y in -extent..=extent {
        for x in -extent..=extent {
            let distance = ((x * x + y * y) as f32).sqrt();

            if distance >= radius + 0.5 {
                continue;
            }

            // 1 - smoothstep(radius - 0.5, radius + 0.5, distance), as in WGSL
            let t = (distance - (radius - 0.5)).clamp(0.0, 1.0);
            taps.push([x as f32, y as f32, 1.0 - t * t * (3.0 - 2.0 * t), distance]);
        }
    }

    taps.sort_by(|a, b| a[3].total_cmp(&b[3]));
    taps
}
// --8<-- [end:outline-radius]

// --8<-- [start:outline-pipelines]
/// Outline pipeline: fullscreen, the alpha blended over the scene.
fn pipeline(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, target: Target) -> Pipeline {
    let shader = module(ctx, "selection outline", shader!("surface_outline.wgsl"));
    let groups = [layout];
    let desc = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .with("black selection outline", "fs_main")
        .depth(DepthMode::Always)
        .color(ColorWrite::Blended);
    build(ctx, target, &desc)
}

/// Alpha pipeline: fullscreen, both masks searched into one byte per pixel.
fn alpha_pipeline(
    ctx: &GpuCtx,
    layout: &wgpu::BindGroupLayout,
    table: &wgpu::BindGroupLayout,
) -> Pipeline {
    let shader = module(
        ctx,
        "selection outline alpha",
        shader!("surface_outline.wgsl"),
    );
    let groups = [layout, layout, table];
    let desc = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .with("selection outline alpha", "fs_alpha")
        .depth(DepthMode::Detached);
    let target = Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
        samples: 1,
    };
    build(ctx, target, &desc)
}

/// Pool or dilate pipeline: fullscreen into a coarse texture.
fn pool_pipeline(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, fs: &str) -> Pipeline {
    let shader = module(
        ctx,
        "selection outline pool",
        shader!("surface_outline.wgsl"),
    );
    let groups = [layout];
    let desc = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .with("selection outline pool", fs)
        .depth(DepthMode::Detached);
    let target = Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
        samples: 1,
    };
    build(ctx, target, &desc)
}

// WGSL has no generics, so Rust writes the texture type for one or four samples in front of the shared text.
/// The face coverage shader for a face pass at `samples`.
fn face_coverage_source(samples: u32) -> String {
    let texture = if samples > 1 {
        "texture_multisampled_2d<u32>"
    } else {
        "texture_2d<u32>"
    };
    format!(
        "@group(0) @binding(0) var physical: {texture}; // triangle index + 1 per sample\nconst SAMPLES: u32 = {samples}u; // samples per pixel of `physical`\n{}",
        shader!("face_coverage.wgsl")
    )
}

/// Face coverage pipelines for a face pass at `samples`.
fn face_coverage(ctx: &GpuCtx, samples: u32) -> FaceCoverage {
    // the same layout on every retarget, so the cached pipelines match again
    let layout = crate::engine::pipelines::layout(
        ctx,
        "face coverage",
        &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Uint,
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: samples > 1,
            },
            count: None,
        }],
    );
    let shader = module(ctx, "face coverage", &face_coverage_source(samples));
    let groups = [&layout];
    let base = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList);
    let mask = |samples| Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
        samples,
    };
    let fraction = build(
        ctx,
        mask(1),
        &base
            .with("face coverage", "fs_fraction")
            .depth(DepthMode::Detached),
    );
    // inside a mask pass: the scene depth is attached, and at 4x every sample is written
    let entry = if samples > 1 {
        "fs_samples"
    } else {
        "fs_fraction"
    };
    let with_edges = build(
        ctx,
        mask(samples),
        &base
            .with("face coverage beside edges", entry)
            .depth(DepthMode::Always),
    );
    FaceCoverage {
        layout,
        fraction,
        with_edges,
        group: None,
    }
}
// --8<-- [end:outline-pipelines]

// --8<-- [start:outline-tests]
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
    use session_rust::{RenderVertex, Xform};

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Selected edges never thin the black outline.
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
                        gpu.view.show_mesh_edges = true;
                        gpu.segments.set_selected(0, false);
                        let silhouette = gpu.render_offscreen(&input);
                        gpu.segments.set_selected(0, true);
                        let edged = gpu.render_offscreen(&input);
                        let mut coverage = 0_u32;

                        for (plain, inked) in silhouette.chunks_exact(4).zip(edged.chunks_exact(4))
                        {
                            let lo = *plain[..3].iter().min().unwrap();
                            let hi = *plain[..3].iter().max().unwrap();

                            if hi - lo <= 2 {
                                coverage += u32::from(255 - hi);
                            }

                            if hi < 8 {
                                assert!(
                                    inked[..3].iter().all(|channel| *channel < 12),
                                    "yellow CAD strokes must not narrow the black border: {plain:?} -> {inked:?}; samples={samples}, DPR={dpr}, orbit={orbit:?}"
                                );
                            }
                        }

                        assert!(
                            coverage > gpu.config.height * 255 / 2,
                            "visible silhouette includes fractional coverage: {coverage}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Two touching solids get one outline, none between them.
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

    /// Add one square object to the upload.
    fn quad(upload: &mut Upload, extent: f32, depth: f32) {
        let row = upload.obj.rows.len() as u32;
        let first = upload.arena.verts.len() as u32;
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), 0));

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
    /// A hidden selection has no outline; clearing it frees the mask.
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
            // with only the selected mask the picture is the same
            gpu.pass_mut::<super::Outline>().solid.kind = super::OutlineKind::Selected;
            assert_eq!(
                selected,
                gpu.render_offscreen(&input),
                "selection must not receive a second overlapping outline"
            );
            gpu.pass_mut::<super::Outline>().solid.kind = super::OutlineKind::AllSolids;
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
            let capacity = gpu.pass::<super::Outline>().selection.allocated_bytes().1;
            assert_eq!(
                capacity,
                200 * 200 * if samples > 1 { 5 } else { 1 } + 2 * 50 * 50
            );
            gpu.set_selected(0, false);
            assert_eq!(
                gpu.pass::<super::Outline>().selection.allocated_bytes().1,
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
            assert_eq!(
                plain_black, black,
                "ordinary and selected outlines have the same width"
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
        assert_eq!(
            gpu.pass::<super::Outline>().selection.allocated_bytes(),
            (16, 0)
        );
        assert_eq!(
            gpu.pass::<super::Outline>().solid.allocated_bytes(),
            (16, 0),
            "release frees the outline alpha"
        );
    }

    #[test]
    /// The face coverage shader validates at one and at four samples.
    fn face_coverage_shader_validates() {
        for samples in [1, 4] {
            let source = crate::engine::pipelines::shared(&super::face_coverage_source(samples));
            let module = naga::front::wgsl::parse_str(&source)
                .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::default()
                    | naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32,
            )
            .validate(&module)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        }
    }

    #[test]
    /// Taps fit the table, run nearest first and the block covers the search.
    fn taps_are_sorted_and_blocks_cover_the_extent() {
        for radius in [1.0, 1.6875, 3.375, 5.0625, 12.0] {
            let taps = super::taps(radius);
            assert!(taps.len() <= super::TAPS);
            assert_eq!(taps[0], [0.0, 0.0, 1.0, 0.0]);
            assert!(taps.windows(2).all(|pair| pair[0][2] >= pair[1][2]));
            assert!(super::block(radius) >= (radius + 0.5).ceil() as u32);
        }

        assert_eq!(super::block(3.375), 4);
        assert_eq!(super::block(5.0625), 8);
    }

    /// A headless canvas with a grid of cones and cylinders, rows 0..5 selectable.
    fn solids_scene(
        width: u32,
        height: u32,
        count: usize,
    ) -> (Gpu, crate::camera::Camera, session_rust::Point) {
        use crate::app::scene::{FileDoc, Scene};
        use crate::camera::Camera;
        use session_rust::{BRep, Session};
        use std::rc::Rc;
        let mut gpu = pollster::block_on(Gpu::new_headless(width, height)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.show_outlines = true;
        gpu.view.markers = false;
        let mut source = Session::new("outline bench");
        let side = (count as f64).sqrt().ceil() as usize;

        for i in 0..count {
            let shape = if i % 2 == 0 {
                BRep::create_cone(40.0, 100.0)
            } else {
                BRep::create_cylinder(40.0, 100.0)
            };
            let solid = source.add_brep(shape, None).unwrap();
            let (x, y) = ((i % side) as f64 * 120.0, (i / side) as f64 * 120.0);
            source.set_xform(&solid.borrow().name, Xform::translation(x, y, 0.0));
        }

        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "solids".into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, width as f64 / height as f64);
        camera.orbit(60.0, -80.0);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        (gpu, camera, rebase.anchor)
    }

    /// Milliseconds of one offscreen frame read back to the CPU, and its pixels.
    fn timed(
        gpu: &mut Gpu,
        camera: &crate::camera::Camera,
        anchor: &session_rust::Point,
    ) -> (f64, Vec<u8>) {
        let aspect = gpu.config.width as f64 / gpu.config.height as f64;
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(aspect, anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let start = std::time::Instant::now();
        let pixels = gpu.render_offscreen(&input);
        (start.elapsed().as_secs_f64() * 1000.0, pixels)
    }

    /// 0: `o` only, 1: `o` and a selection, 2: the selection only, 3: no outline.
    fn outline_mode(gpu: &mut Gpu, mode: usize) {
        gpu.view.show_outlines = mode != 3;
        gpu.pass_mut::<super::Outline>().solid.kind = if mode == 2 {
            super::OutlineKind::Selected
        } else {
            super::OutlineKind::AllSolids
        };

        for row in 0..5 {
            gpu.set_selected(row, mode == 1 || mode == 2);
        }
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Once each outline mode drew at 1x and 4x, toggles, flips and resizes compile nothing.
    fn toggles_flips_and_resizes_compile_nothing_twice() {
        let (mut gpu, camera, anchor) = solids_scene(320, 200, 8);
        let cycle = |gpu: &mut Gpu, width: u32| {
            for samples in [4, 1] {
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(width, 200);

                for mode in [0, 1, 2, 3] {
                    outline_mode(gpu, mode);
                    timed(gpu, &camera, &anchor);
                }
            }
        };
        cycle(&mut gpu, 320);
        let compiled = crate::engine::pipelines::created();

        for width in 321..331 {
            cycle(&mut gpu, width);
        }

        assert_eq!(crate::engine::pipelines::created(), compiled);
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Writes outline captures at DPR 1/2/3, MSAA 1/4 for 'o', selection and both.
    fn outline_captures() {
        let (mut gpu, camera, anchor) = solids_scene(960, 600, 50);
        std::fs::create_dir_all("target/review/outline").unwrap();

        for dpr in [1.0, 2.0, 3.0] {
            for samples in [1, 4] {
                gpu.logical_size = [960.0 / dpr, 600.0 / dpr];
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(960, 600);

                for (mode, edges) in [(0, false), (1, false), (2, false), (0, true), (1, true)] {
                    outline_mode(&mut gpu, mode);
                    gpu.view.show_mesh_edges = edges;
                    let pixels = timed(&mut gpu, &camera, &anchor).1;
                    let name = format!("dpr{dpr}-msaa{samples}-mode{mode}-edges{edges}");
                    std::fs::write(format!("target/review/outline/{name}.rgba"), &pixels).unwrap();
                }
            }
        }
    }

    #[test]
    #[ignore = "benchmark, requires a native GPU adapter"]
    /// Orbit and still frame times at 2880x1800, DPR 2, MSAA 4, outlines off and on.
    fn bench_outline() {
        let (mut gpu, mut camera, anchor) = solids_scene(2880, 1800, 50);
        gpu.logical_size = [1440.0, 900.0];
        gpu.view.msaa_forced = Some(4);
        gpu.resize(2880, 1800);
        let mut report = String::new();

        for (mode, edges) in [
            (3, false),
            (0, false),
            (1, false),
            (2, false),
            (3, true),
            (0, true),
        ] {
            outline_mode(&mut gpu, mode);
            gpu.view.show_mesh_edges = edges;

            for _ in 0..5 {
                timed(&mut gpu, &camera, &anchor);
            }

            let still: f64 = (0..200)
                .map(|_| timed(&mut gpu, &camera, &anchor).0)
                .sum::<f64>()
                / 200.0;
            let mut orbit = 0.0;

            for _ in 0..200 {
                camera.orbit(3.49, 0.0);
                orbit += timed(&mut gpu, &camera, &anchor).0;
            }

            camera.orbit(-3.49 * 200.0, 0.0);
            report += &format!(
                "mode {mode}, edges {edges}: still {still:.2} ms, orbit {:.2} ms\n",
                orbit / 200.0
            );
        }

        std::fs::create_dir_all("target/review").unwrap();
        std::fs::write("target/review/bench-outline.txt", &report).unwrap();
    }
}
// --8<-- [end:outline-tests]

// --8<-- [start:outline-pass]
impl super::lane::Lane for SurfaceOutline {
    fn on_retarget(&mut self, ctx: &GpuCtx, _layouts: &Layouts, target: Target) {
        self.retarget(ctx, target);
    }

    fn on_reset(&mut self, _ctx: &GpuCtx) {
        self.reset();
    }

    fn bytes(&self) -> (u64, u64) {
        self.allocated_bytes()
    }
}

// Two outlines, one pass: the selection keeps its own mask, so a selected solid touching another still gets a border between them.
/// The outlines around the selection and around every solid.
pub struct Outline {
    pub selection: SurfaceOutline, // around the selection
    pub solid: SurfaceOutline,     // around every solid; also composites both
}

/// The outline pass.
pub fn pass(ctx: &GpuCtx, target: Target) -> Box<dyn Pass> {
    Box::new(Outline {
        selection: SurfaceOutline::new(ctx, target, OutlineKind::Selected),
        solid: SurfaceOutline::new(ctx, target, OutlineKind::AllSolids),
    })
}

impl super::lane::Lane for Outline {
    fn on_retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        self.selection.on_retarget(ctx, layouts, target);
        self.solid.on_retarget(ctx, layouts, target);
    }

    fn on_reset(&mut self, ctx: &GpuCtx) {
        self.selection.on_reset(ctx);
        self.solid.on_reset(ctx);
    }

    fn bytes(&self) -> (u64, u64) {
        let (a, b) = self.selection.bytes();
        let (c, d) = self.solid.bytes();
        (a + c, b + d)
    }
}

// The frame calls each registered pass at fixed points: this one masks after the faces and blends over the ink.
impl Pass for Outline {
    /// Redraw the masks only when something changed, then search them into the alpha texture.
    fn after_faces(&mut self, g: &mut Gpu, encoder: &mut wgpu::CommandEncoder, f: &Frame) -> u32 {
        let size = (g.config.width, g.config.height);
        // no outlines in x-ray
        let faces = g.view.show_outlines && g.view.opacity > 0.0 && g.live_faces() > 0;
        let selected =
            self.selection
                .prepare(&g.ctx, size, g.targets.samples, g.logical_size[0], faces);
        let solid = self
            .solid
            .prepare(&g.ctx, size, g.targets.samples, g.logical_size[0], faces);
        // the compositing outline holds the taps and alpha for both masks
        let radius = radius(size, g.logical_size[0]);
        self.solid
            .prepare_alpha(&g.ctx, (solid || selected).then_some(radius), size);
        let key = MaskKey {
            mvp: g.frame.mvp_f32,
            geometry: g.objects.geometry_revision(),
            selection: g.selection_revision,
            faces: g.arena.source_faces.revision(),
            size,
            samples: g.targets.samples,
            edges: g.view.show_mesh_edges && f.tier < 2,
            pen: g.view.thickness_px.to_bits(),
        };
        let stale =
            (solid && !self.solid.is_valid(&key)) || (selected && !self.selection.is_valid(&key));
        let mut draws = 0;

        if stale {
            self.solid.bind_faces(&g.ctx, &g.targets);
            g.each_pass(|pass, g| pass.bind_masks(g));
            let b = g.frame.binds(&g.objects.group);
            // edges widen the mask, except in a slow drag
            let ink = g.frame.binds(g.objects.ink_group());
            let edges = key.edges && g.live_pipes() > 0;

            if solid && selected {
                // one pass writes both masks
                let mut pass =
                    SurfaceOutline::begin_masks(&self.solid, &self.selection, encoder, &g.targets);
                draws += g.arena.draw_masks(&mut pass, &b);
                draws += g.arena.source_faces.draw_masks(&mut pass, &b);
                for other in &g.passes {
                    draws += other.in_masks(g, &mut pass, &b, true);
                }

                if edges {
                    draws += g.segments.draw_masks(&mut pass, &ink);
                }
            } else if solid && edges {
                // faces from the face pass's triangle ids, then the edges over them
                let mut pass = self.solid.begin_mask(encoder, &g.targets);
                draws += self.solid.draw_faces(&mut pass);
                draws += g.segments.draw_solid_mask(&mut pass, &ink);
            } else if solid {
                draws += self.solid.encode_faces(encoder);
            } else if selected {
                let mut pass = self.selection.begin_mask(encoder, &g.targets);
                draws += g.arena.draw_selection_mask(&mut pass, &b);
                draws += g.arena.source_faces.draw_mask(&mut pass, &b);
                for other in &g.passes {
                    draws += other.in_masks(g, &mut pass, &b, false);
                }

                if edges {
                    draws += g.segments.draw_selection_mask(&mut pass, &ink);
                }
            }

            if solid {
                self.solid.encode_pool(encoder);
                self.solid.mark_valid(key);
            }

            if selected {
                self.selection.encode_pool(encoder);
                self.selection.mark_valid(key);
            }

        }

        self.solid.encode_alpha(&self.selection, encoder, stale);
        draws
    }

    fn over_ink(&self, _g: &Gpu, pass: &mut wgpu::RenderPass<'_>, _b: &Binds) -> u32 {
        self.solid.draw_combined(pass)
    }

    fn on_select(&mut self, row: u32, on: bool) {
        self.selection.set_selected(row, on);
    }
}
// --8<-- [end:outline-pass]
