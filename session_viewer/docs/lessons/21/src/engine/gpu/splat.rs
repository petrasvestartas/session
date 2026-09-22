use super::buffers::{GpuCtx, bind_group, zeroed_buffer};
use super::cloud::{Cloud, LodNode, NO_NORMALS, PointBufs};
use super::instance::Instance;
use super::lod::{LodWalk, Projection, radius_factor};
use super::objects::InstanceTable;
// --8<-- [start:step-27a]
use super::targets::{Attachment, TextureSpec};
// --8<-- [end:step-27a]
use crate::engine::pipelines::{DepthMode, Layouts, PipelineDesc, Target, build, module};
use session_rust::Xform;
use wgpu::PrimitiveTopology::TriangleList;

/// Most point runs drawn in one frame.
pub const MAX_RECORDS: usize = 4096;

/// Color format of the point texture.
const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// Vertices per point: one quad, placed by the shader.
const POINT_VERTS: u32 = 6;

/// Bytes before the records: record count, point total, 0, 0.
const HEADER_BYTES: u64 = 16;

/// One run of points to draw, 160 bytes, as the shader reads it.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SplatRecord {
    pub mvp_model: [f32; 16], // camera matrix times object matrix
    pub tint: [f32; 4], // rgb tint; a = smallest radius in px
    pub first: u32, // first GPU point row
    pub count: u32, // points in the run
    pub cum: u32, // points drawn before this run
    pub k: f32, // radius factor; the shader divides by depth
    pub rot: [f32; 12], // object rotation, for the normals
    pub nrm_first: u32, // first normal row, or NO_NORMALS
    pub instance: u32, // object row
    pub flags: u32, // Instance::FLAG_* bits
    pub selected_point: u32, // highlighted point row + 1, or 0
}

const _: () = assert!(std::mem::size_of::<SplatRecord>() == 160);

/// Frame facts the record builder needs.
pub struct RecordCx<'a> {
    pub mvp: &'a [f32; 16], // camera matrix
    pub ortho_h: f32, // ortho half-height; 0 = perspective
    pub eye: [f32; 3], // camera position
    pub size: (u32, u32), // framebuffer size, px
    pub cloud_size: f32, // point size scale
    pub lod_px: f32, // split octree nodes wider than this
    pub objects: &'a InstanceTable, // object rows
    pub clouds: &'a [Cloud], // every cloud
    pub nodes: &'a [LodNode], // every octree node
}

/// What the last point pass depended on; same key = skip it.
#[derive(Clone, PartialEq)]
struct Key {
    mvp: [f32; 16], // camera matrix
    cloud_size: f32, // point size scale
    lod_px: f32, // LOD cutoff
    point_count: u32, // points uploaded
}

/// Textures the point pass draws into, made when the first cloud arrives.
struct SplatTargets {
    // --8<-- [start:step-27b]
    depth: Attachment, // nearest point per pixel
    color: Attachment, // its color
    // --8<-- [end:step-27b]
    size: (u32, u32), // texture size, px
    resolve_group: wgpu::BindGroup, // both textures, for the resolve
}

impl SplatTargets {
    /// Create both textures and their bind group.
    fn new(ctx: &GpuCtx, l: &Layouts, size: (u32, u32)) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        // --8<-- [start:step-27c]
        let depth = Attachment::new(
        // --8<-- [end:step-27c]
            ctx,
            "splat.depth",
            &TextureSpec {
                size,
                format: wgpu::TextureFormat::Depth32Float,
                samples: 1,
                usage,
            },
        );
        // --8<-- [start:step-27d]
        let color = Attachment::new(
        // --8<-- [end:step-27d]
            ctx,
            "splat.color",
            &TextureSpec {
                size,
                format: COLOR_FORMAT,
                samples: 1,
                usage,
            },
        );
        let resolve_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("splat.resolve.group"),
            layout: &l.resolve,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&depth),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&color),
                },
            ],
        });
        Self {
            depth,
            color,
            size,
            resolve_group,
        }
    }
}

/// Settings that differ between the color and id point pipelines.
struct PointVariant {
    target: Target, // output format
    label: &'static str, // name shown in GPU errors
    fs: &'static str, // fragment shader entry point
}

/// Draws point clouds: points into a texture, then the texture into the scene.
pub struct Splat {
    control_parent: Option<u32>, // cloud being edited, drawn without LOD
    selected_point: Option<u32>, // highlighted point row
    records: Vec<SplatRecord>, // runs to draw this frame
    walk: LodWalk, // octree walk scratch
    record_buf: wgpu::Buffer, // records on the GPU
    total: u32, // points drawn last pass
    key: Option<Key>, // what the last pass depended on
    targets: Option<SplatTargets>, // point textures
    points_group: wgpu::BindGroup, // records and point buffers, bound
    resolve_shader: wgpu::ShaderModule, // resolve shader
    point_pipeline: wgpu::RenderPipeline, // points in color
    resolve_pipeline: wgpu::RenderPipeline, // point texture into the scene
    id_pipeline: wgpu::RenderPipeline, // points as ids
}

impl Splat {
    /// Bytes reserved on the GPU: (buffers, textures).
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let pixels = match &self.targets {
            Some(target) => u64::from(target.size.0) * u64::from(target.size.1),
            None => 0,
        };
        (self.record_buf.size(), pixels * 8)
    }

    /// Create the record buffer, bind group and pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target, bufs: PointBufs) -> Self {
        let record_buf = zeroed_buffer(
            &ctx.device,
            "splat.records",
            HEADER_BYTES + MAX_RECORDS as u64 * 160,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        let points_group = points_group(ctx, l, &record_buf, &bufs);
        let point_shader = module(
            &ctx.device,
            "splat.shader",
            include_str!("../../shaders/splat.wgsl"),
        );
        let resolve_shader = module(
            &ctx.device,
            "splat.resolve.shader",
            include_str!("../../shaders/splat_resolve.wgsl"),
        );
        let point_pipeline = build_point(
            ctx,
            l,
            &point_shader,
            &PointVariant {
                target: Target {
                    format: COLOR_FORMAT,
                    samples: 1,
                },
                label: "splat.points",
                fs: "fs_point",
            },
        );
        let id_pipeline = build_point(
            ctx,
            l,
            &point_shader,
            &PointVariant {
                target: Target::ID,
                label: "splat.points.id",
                fs: "fs_point_id",
            },
        );
        let resolve_pipeline = build_resolve(ctx, l, &resolve_shader, target);

        Self {
            control_parent: None,
            selected_point: None,
            records: Vec::new(),
            walk: LodWalk::default(),
            record_buf,
            total: 0,
            key: None,
            targets: None,
            points_group,
            resolve_shader,
            point_pipeline,
            resolve_pipeline,
            id_pipeline,
        }
    }

    /// Draw cloud `parent` without LOD while it is edited.
    pub fn set_controls(&mut self, parent: Option<u32>) {
        self.control_parent = parent;
        self.selected_point = None;
        self.invalidate();
    }

    /// Highlight one point row; None clears it.
    pub fn set_point(&mut self, point: Option<u32>) {
        self.selected_point = point;
        self.invalidate();
    }

    /// Rebuild the resolve pipeline for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.resolve_pipeline = build_resolve(ctx, l, &self.resolve_shader, target);
    }

    /// Drop the textures; the next pass remakes them.
    pub fn resize(&mut self) {
        self.targets = None;
        self.key = None;
    }

    /// Rebuild the bind group after a point buffer moved.
    pub fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts, bufs: PointBufs) {
        self.points_group = points_group(ctx, l, &self.record_buf, &bufs);
        self.key = None;
    }

    /// Make the next frame redraw the points.
    pub fn invalidate(&mut self) {
        self.key = None;
    }

    /// Drop the textures and forget the points.
    pub fn release(&mut self) {
        self.targets = None;
        self.total = 0;
        self.key = None;
    }

    /// Points drawn last pass.
    pub fn total(&self) -> u32 {
        self.total
    }

    /// Draw the points into their texture, unless nothing changed.
    pub fn prelude(
        &mut self,
        ctx: &GpuCtx,
        l: &Layouts,
        encoder: &mut wgpu::CommandEncoder,
        cx: &RecordCx,
        cloud_group: &wgpu::BindGroup,
    ) {
        let mut point_count = 0u32;

        for c in cx.clouds {
            point_count += c.resident;
        }

        // skip the pass when the inputs are unchanged
        let key = Key {
            mvp: *cx.mvp,
            cloud_size: cx.cloud_size,
            lod_px: cx.lod_px,
            point_count,
        };

        if self.key.as_ref() == Some(&key) {
            return;
        }

        self.key = Some(key);
        self.build_records(cx);

        if self.total == 0 {
            return;
        }

        // remake the textures when the size changed
        if !matches!(&self.targets, Some(targets) if targets.size == cx.size) {
            self.targets = Some(SplatTargets::new(ctx, l, cx.size));
        }

        // upload the header and the records
        let header = [self.records.len() as u32, self.total, 0, 0];
        ctx.queue
            .write_buffer(&self.record_buf, 0, bytemuck::bytes_of(&header));
        ctx.queue.write_buffer(
            &self.record_buf,
            HEADER_BYTES,
            bytemuck::cast_slice(&self.records),
        );

        let Some(targets) = &self.targets else { return };
        let mut pass = begin_point_pass(encoder, targets);
        pass.set_pipeline(&self.point_pipeline);
        pass.set_bind_group(0, cloud_group, &[]);
        pass.set_bind_group(1, &self.points_group, &[]);
        pass.draw(0..POINT_VERTS * self.total, 0..1);
    }

    /// Draw the point texture into the scene; returns the draw count.
    pub fn draw_resolve(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        cloud_group: &wgpu::BindGroup,
    ) -> u32 {
        let Some(targets) = &self.targets else {
            return 0;
        };

        if self.total == 0 {
            return 0;
        }

        pass.set_pipeline(&self.resolve_pipeline);
        pass.set_bind_group(0, cloud_group, &[]);
        pass.set_bind_group(1, &targets.resolve_group, &[]);
        pass.draw(0..3, 0..1);
        1
    }

    /// Draw the points as (object row, point row) ids.
    pub fn draw_ids(&self, pass: &mut wgpu::RenderPass<'_>, cloud_group: &wgpu::BindGroup) -> u32 {
        if self.total == 0 {
            return 0;
        }

        pass.set_pipeline(&self.id_pipeline);
        pass.set_bind_group(0, cloud_group, &[]);
        pass.set_bind_group(1, &self.points_group, &[]);
        pass.draw(0..POINT_VERTS * self.total, 0..1);
        1
    }

    /// Build one record per run of points to draw.
    fn build_records(&mut self, cx: &RecordCx) {
        self.records.clear();
        let mut p = Projection {
            eye: cx.eye,
            ortho_h: cx.ortho_h,
            height_px: cx.size.1,
            lod_px: cx.lod_px,
            nodes: cx.nodes,
        };
        let mut cum = 0u32;

        for c in cx.clouds {
            let Some(row) = cx.objects.row(c.instance) else {
                continue;
            };

            if row.flags & Instance::FLAG_HIDDEN != 0 {
                continue;
            }

            let Some(model) = cx.objects.anchored_model(c.instance) else {
                continue;
            };
            // point size in px; 3 when the cloud named none
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * cx.cloud_size;

            // no LOD while the cloud is edited
            p.lod_px = if self.control_parent == Some(c.instance) {
                0.0
            } else {
                cx.lod_px
            };
            self.walk.select(&p, c, &model);
            // camera times object, one matrix per point
            let m = (&Xform::from_matrix(cx.mvp.map(f64::from))
                * &Xform::from_matrix(model.map(f64::from)))
                .to_f32();
            // rotation only, for the normals
            let rot = [
                model[0], model[1], model[2], 0.0, model[4], model[5], model[6], 0.0, model[8],
                model[9], model[10], 0.0,
            ];
            let scale = Xform::from_matrix(model.map(f64::from)).uniform_scale();
            let selected = row.flags & Instance::FLAG_SELECTED != 0;
            // yellow when selected
            let tint = if selected {
                [1.0, 1.0, 0.0, (px * 0.5).max(0.5)]
            } else {
                [
                    row.color[0],
                    row.color[1],
                    row.color[2],
                    (px * 0.5).max(0.5),
                ]
            };

            for r in &self.walk.ranges {
                let k = radius_factor(r, px, scale, cx.ortho_h);

                // clip the run to each uploaded chunk
                for chunk in &c.chunks {
                    let a = r.first.max(chunk.from);
                    let b = (r.first + r.count).min(chunk.to);

                    if a >= b || self.records.len() >= MAX_RECORDS {
                        continue;
                    }

                    let nrm_first = if c.nrm_first == NO_NORMALS {
                        NO_NORMALS
                    } else {
                        c.nrm_first + a
                    };
                    self.records.push(SplatRecord {
                        mvp_model: m,
                        tint,
                        first: chunk.row_of(a),
                        count: b - a,
                        cum,
                        k,
                        rot,
                        nrm_first,
                        instance: c.instance,
                        flags: row.flags,
                        selected_point: match self.selected_point {
                            Some(point) => point + 1,
                            None => 0,
                        },
                    });
                    cum += b - a;
                }
            }
        }

        self.total = cum;
    }
}

/// Open the point pass: color cleared, depth cleared to far.
fn begin_point_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    t: &'a SplatTargets,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("splat.points"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &t.color,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &t.depth,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(0.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    })
}

/// Bind group 1 of the point pass: records, positions, colors, normals.
fn points_group(
    ctx: &GpuCtx,
    l: &Layouts,
    records: &wgpu::Buffer,
    bufs: &PointBufs,
) -> wgpu::BindGroup {
    bind_group(
        ctx,
        &l.points,
        "splat.points.group",
        &[records, bufs.pos, bufs.col, bufs.nrm],
    )
}

/// Point pipeline: quads, nearest point wins, no blending.
fn build_point(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &wgpu::ShaderModule,
    v: &PointVariant,
) -> wgpu::RenderPipeline {
    let groups = [&l.line, &l.points];
    let desc = PipelineDesc::new(shader, &groups, &[], TriangleList)
        .with(v.label, v.fs)
        .vertex("vs_point");
    let desc = if v.target == Target::ID {
        desc.physical()
    } else {
        desc
    };
    build(&ctx.device, v.target, &desc)
}

/// Resolve pipeline: a fullscreen triangle writing color and depth.
fn build_resolve(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> wgpu::RenderPipeline {
    let groups = [&l.line, &l.resolve];
    let desc = PipelineDesc::new(shader, &groups, &[], TriangleList)
        .with("splat.resolve", "fs_main")
        .physical()
        .depth(DepthMode::Opaque);
    build(&ctx.device, target, &desc)
}
