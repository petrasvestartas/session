//! The point lane's renderer: one pixel-aligned quad per point into the lane's OWN 1x depth +
//! colour targets (the hardware depth test keeps the nearest point), then a fullscreen resolve
//! into the scene pass with EDL and `frag_depth`. A record per visible cloud (or octree node)
//! folds camera x placement, tint and radius; the point pass is skipped while nothing changed.

use crate::engine::pipelines::{build, module, DepthMode, Layouts, PipelineDesc, Target};
use crate::math::{mat_mul_f32, mat_scale};
use super::buffers::{bind_group, zeroed_buffer, GpuCtx};
use super::cloud::{Cloud, LodNode, PointBufs, NO_NORMALS};
use super::instance::Instance;
use super::lod::{radius_factor, LodWalk, Projection};
use super::objects::InstanceTable;
use super::targets::{texture_view, TextureSpec};
use wgpu::PrimitiveTopology::TriangleList;

/// Records the lane can hold in one frame: one per cloud, or one per selected octree node.
pub const MAX_RECORDS: usize = 4096;

/// The point pass draws into linear RGBA8; the resolve reads it back as-is.
const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// Vertices per point: one quad pulled by vertex index.
const POINT_VERTS: u32 = 6;

/// Header words before the records: {count, total points, 0, 0}.
const HEADER_BYTES: u64 = 16;

/// One record, 160 B (40 words), read as raw words by splat.wgsl.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SplatRecord {
    /// mvp x anchored model: one mat-vec per point.
    pub mvp_model: [f32; 16],
    /// Instance tint; `.a` = the minimum radius in px.
    pub tint: [f32; 4],
    pub first: u32,
    pub count: u32,
    /// Points before this record: the vertex index minus `cum` is the offset into the range.
    pub cum: u32,
    /// Radius factor: screen radius = k * vp_h / clip.w (perspective) or k * vp_h (ortho).
    pub k: f32,
    /// The model's rotation columns (translation-free), three vec4 slots, for the normals.
    pub rot: [f32; 12],
    /// First row in the normals table, or `NO_NORMALS`.
    pub nrm_first: u32,
    /// The object row, written by the id pass.
    pub instance: u32,
    pub flags: u32,
    pub _pad: u32,
}

const _: () = assert!(std::mem::size_of::<SplatRecord>() == 160);

/// What the record builder needs from the frame: camera facts, size, the two cloud knobs,
/// the object rows and the cloud lane's clouds and nodes.
pub struct RecordCx<'a> {
    pub mvp: &'a [f32; 16],
    pub ortho_h: f32,
    pub eye: [f32; 3],
    pub size: (u32, u32),
    pub cloud_size: f32,
    pub lod_px: f32,
    pub objects: &'a InstanceTable,
    pub clouds: &'a [Cloud],
    pub nodes: &'a [LodNode],
}

/// The key the point pass was last drawn for; a frame with the same key skips it.
#[derive(Clone, PartialEq)]
struct Key {
    mvp: [f32; 16],
    cloud_size: f32,
    lod_px: f32,
    point_count: u32,
}

/// The two point-pass targets, 1x, sized to the surface, and the resolve group over them.
/// Made on the first frame that has points and dropped on resize, so a scene without a
/// cloud never pays 8 B/px for them.
struct SplatTargets {
    depth: wgpu::TextureView,
    color: wgpu::TextureView,
    size: (u32, u32),
    resolve_group: wgpu::BindGroup,
}

impl SplatTargets {
    /// Depth (nearest point per pixel, 0 = empty) and its colour, both bindable.
    fn new(ctx: &GpuCtx, l: &Layouts, size: (u32, u32)) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let depth = texture_view(ctx, "splat.depth", &TextureSpec { size, format: wgpu::TextureFormat::Depth32Float, samples: 1, usage });
        let color = texture_view(ctx, "splat.color", &TextureSpec { size, format: COLOR_FORMAT, samples: 1, usage });
        let resolve_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("splat.resolve.group"),
            layout: &l.resolve,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&depth) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&color) },
            ],
        });
        Self { depth, color, size, resolve_group }
    }
}

/// One of the two point pipelines: the colour pass or the id pass.
struct PointVariant {
    target: Target,
    label: &'static str,
    fs: &'static str,
}

/// The point lane's renderer.
pub struct Splat {
    records: Vec<SplatRecord>,
    walk: LodWalk,
    record_buf: wgpu::Buffer,
    total: u32,
    key: Option<Key>,
    targets: Option<SplatTargets>,
    points_group: wgpu::BindGroup,
    resolve_shader: wgpu::ShaderModule,
    point_pipeline: wgpu::RenderPipeline,
    resolve_pipeline: wgpu::RenderPipeline,
}

impl Splat {
    /// The record buffer, the points group over the lane's placeholder buffers, and the
    /// two pipelines; the targets wait for the first cloud.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target, bufs: PointBufs) -> Self {
        let record_buf = zeroed_buffer(&ctx.device, "splat.records", HEADER_BYTES + MAX_RECORDS as u64 * 160, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let points_group = points_group(ctx, l, &record_buf, &bufs);
        let point_shader = module(&ctx.device, "splat.shader", include_str!("../../shaders/splat.wgsl"));
        let resolve_shader = module(&ctx.device, "splat.resolve.shader", include_str!("../../shaders/splat_resolve.wgsl"));
        let point_pipeline = build_point(ctx, l, &point_shader, &PointVariant { target: Target { format: COLOR_FORMAT, samples: 1 }, label: "splat.points", fs: "fs_point" });
        let resolve_pipeline = build_resolve(ctx, l, &resolve_shader, target);

        Self {
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
        }
    }

    /// Rebuild the resolve pipeline for a new scene sample count (the point pass stays 1x).
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.resolve_pipeline = build_resolve(ctx, l, &self.resolve_shader, target);
    }

    /// Drop the targets: the next point pass makes them at the new size.
    pub fn resize(&mut self) {
        self.targets = None;
        self.key = None;
    }

    /// Re-point the points group at the lane's current buffers (a table grew or was released).
    pub fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts, bufs: PointBufs) {
        self.points_group = points_group(ctx, l, &self.record_buf, &bufs);
        self.key = None;
    }

    /// Force the next frame to rebuild the records and redraw the point pass.
    pub fn invalidate(&mut self) {
        self.key = None;
    }

    /// Drop the targets with the scene.
    pub fn release(&mut self) {
        self.targets = None;
        self.total = 0;
        self.key = None;
    }

    /// Points the last point pass drew; 0 = nothing to resolve.
    pub fn total(&self) -> u32 {
        self.total
    }

    /// The point pass: skipped while the key matches, else records rebuilt, written and drawn.
    /// `cloud_group` is the cloud uniform (group 0).
    pub fn prelude(&mut self, ctx: &GpuCtx, l: &Layouts, encoder: &mut wgpu::CommandEncoder, cx: &RecordCx, cloud_group: &wgpu::BindGroup) {
        let mut point_count = 0u32;
        for c in cx.clouds {
            point_count += c.count;
        }
        let key = Key { mvp: *cx.mvp, cloud_size: cx.cloud_size, lod_px: cx.lod_px, point_count };
        if self.key.as_ref() == Some(&key) {
            return;
        }
        self.key = Some(key);
        self.build_records(cx);
        if self.total == 0 {
            return;
        }
        if self.targets.as_ref().map(|t| t.size) != Some(cx.size) {
            self.targets = Some(SplatTargets::new(ctx, l, cx.size));
        }

        let header = [self.records.len() as u32, self.total, 0, 0];
        ctx.queue.write_buffer(&self.record_buf, 0, bytemuck::bytes_of(&header));
        ctx.queue.write_buffer(&self.record_buf, HEADER_BYTES, bytemuck::cast_slice(&self.records));

        let Some(targets) = &self.targets else { return };
        let mut pass = begin_point_pass(encoder, targets);
        pass.set_pipeline(&self.point_pipeline);
        pass.set_bind_group(0, cloud_group, &[]);
        pass.set_bind_group(1, &self.points_group, &[]);
        pass.draw(0..POINT_VERTS * self.total, 0..1);
    }

    /// The fullscreen resolve inside the scene pass: 1 draw, or 0 with no points.
    pub fn draw_resolve(&self, pass: &mut wgpu::RenderPass<'_>, cloud_group: &wgpu::BindGroup) -> u32 {
        let Some(targets) = &self.targets else { return 0 };
        if self.total == 0 {
            return 0;
        }
        pass.set_pipeline(&self.resolve_pipeline);
        pass.set_bind_group(0, cloud_group, &[]);
        pass.set_bind_group(1, &targets.resolve_group, &[]);
        pass.draw(0..3, 0..1);
        1
    }

    /// One record per visible cloud, or per selected octree node when the LOD walk is on.
    fn build_records(&mut self, cx: &RecordCx) {
        self.records.clear();
        let p = Projection { eye: cx.eye, ortho_h: cx.ortho_h, height_px: cx.size.1, lod_px: cx.lod_px, nodes: cx.nodes };
        let mut cum = 0u32;
        for c in cx.clouds {
            let Some(row) = cx.objects.row(c.instance) else { continue };
            if row.flags & Instance::FLAG_HIDDEN != 0 {
                continue;
            }
            let Some(model) = cx.objects.anchored_model(c.instance) else { continue };
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * cx.cloud_size;

            self.walk.select(&p, c, &model);
            let m = mat_mul_f32(cx.mvp, &model);
            let rot = [model[0], model[1], model[2], 0.0, model[4], model[5], model[6], 0.0, model[8], model[9], model[10], 0.0];
            let scale = mat_scale(&model);
            let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];

            for r in &self.walk.ranges {
                if self.records.len() >= MAX_RECORDS {
                    break;
                }
                let k = radius_factor(r, px, scale, cx.ortho_h);
                let nrm_first = if c.nrm_first == NO_NORMALS { NO_NORMALS } else { c.nrm_first + r.first };
                self.records.push(SplatRecord {
                    mvp_model: m,
                    tint,
                    first: c.first + r.first,
                    count: r.count,
                    cum,
                    k,
                    rot,
                    nrm_first,
                    instance: c.instance,
                    flags: row.flags,
                    _pad: 0,
                });
                cum += r.count;
            }
        }
        self.total = cum;
    }
}

/// The point pass over the lane's own targets: colour cleared transparent, depth to 0.
/// Group 0 (the cloud uniform) is set by the caller.
fn begin_point_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, t: &'a SplatTargets) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("splat.points"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &t.color,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &t.depth,
            depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    })
}

/// Group 1 of the point pass: the records, then positions, colours, normals.
fn points_group(ctx: &GpuCtx, l: &Layouts, records: &wgpu::Buffer, bufs: &PointBufs) -> wgpu::BindGroup {
    bind_group(ctx, &l.points, "splat.points.group", &[records, bufs.pos, bufs.col, bufs.nrm])
}

/// The point pass pipeline: quads, depth written (nearest wins), no blending.
fn build_point(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, v: &PointVariant) -> wgpu::RenderPipeline {
    let groups = [&l.line, &l.points];
    let desc = PipelineDesc::new(shader, &groups, &[], TriangleList).with(v.label, v.fs).vertex("vs_point");
    build(&ctx.device, v.target, &desc)
}

/// The resolve pipeline: a fullscreen triangle writing colour and `frag_depth` under the
/// scene's depth test.
fn build_resolve(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> wgpu::RenderPipeline {
    let groups = [&l.line, &l.resolve];
    let desc = PipelineDesc::new(shader, &groups, &[], TriangleList).with("splat.resolve", "fs_main").depth(DepthMode::Opaque);
    build(&ctx.device, target, &desc)
}
