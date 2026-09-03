//! The compute splatter over both cloud lanes: the per-pixel depth/colour pair both lanes
//! contest, one record table + two bind groups per lane (`SplatSlot`), the resolve group, the
//! static-skip key, and the record builder (`records`) that folds a cloud's per-frame state
//! into 144 B records the shader reads by word index. It owns no points - the lanes do.

use crate::engine::pipelines::{Layouts, Pipelines};
use super::buffers::{zeroed_buffer, GpuCtx};
use super::cloud::{CloudDraw, LodNode};
use super::frame::FrameUniforms;
use super::instance::Instance;
use super::objects::InstanceTable;

/// Words per record: `splat.wgsl` reads the table by literal word index, so this is the contract.
pub const REC_WORDS: u32 = 36;

/// Records a lane's table holds (16 B header + 256 x 144 B); the builder stops at the cap.
pub const MAX_RECORDS: u32 = 256;

/// One record = one contiguous point range at one spacing, as the shader reads it: words 0-15
/// mvp x model (column-major), 16-19 tint (.a = minimum radius px), 20 first, 21 count, 22 cum
/// (the range's first thread), 23 k (attenuation), 24-35 the model's rotation columns padded
/// to vec4 - so a thread does one mat-vec and no instance fetch.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SplatRecord {
    pub mvp_model: [f32; 16],
    pub tint: [f32; 4],
    pub first: u32,
    pub count: u32,
    pub cum: u32,
    pub k: f32,
    pub rot: [f32; 12],
}

// splat.wgsl walks the table 36 words at a time; a field added here misreads every record after the first.
const _: () = assert!(std::mem::size_of::<SplatRecord>() == REC_WORDS as usize * 4);

/// The two per-pixel u32 buffers both lanes contest: winning reverse-Z bits (0 = empty), winner's RGBA8.
pub struct PixelBufs {
    pub depth: wgpu::Buffer,
    pub color: wgpu::Buffer,
}

impl PixelBufs {
    /// Framebuffer-sized; COPY_DST so `clear_buffer` can zero them before every rebuilt frame.
    pub fn new(ctx: &GpuCtx, size: (u32, u32)) -> Self {
        let pixels = (size.0.max(1) * size.1.max(1)) as u64 * 4;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST;
        let depth = zeroed_buffer(&ctx.device, "splat.depth", pixels, usage);
        let color = zeroed_buffer(&ctx.device, "splat.color", pixels, usage);

        Self { depth, color }
    }
}

/// A lane's three point buffers, borrowed for one bind.
pub struct PointBufs<'a> {
    pub pos: &'a wgpu::Buffer,
    pub col: &'a wgpu::Buffer,
    pub nrm: &'a wgpu::Buffer,
}

/// What every splat bind group is made from besides the lane's own buffers.
pub struct SplatCx<'a> {
    pub ctx: &'a GpuCtx,
    pub layouts: &'a Layouts,
    pub frame: &'a FrameUniforms,
}

/// One frame's records for one lane: the 4-word header {n, total, 0, 0}, the records, the threads.
/// Kept between frames and refilled in place: the table and the LOD stack keep their capacity, so
/// a rebuilt frame allocates nothing.
#[derive(Default)]
pub struct Records {
    pub header: [u32; 4],
    pub recs: Vec<SplatRecord>,
    pub total: u32,
    stack: Vec<usize>,
}

impl Records {
    /// Empty the table for the next frame; capacity stays.
    fn clear(&mut self) {
        self.header = [0; 4];
        self.recs.clear();
        self.total = 0;
    }
}

/// One lane's slot: its record table on both sides, group 0 (frame + records), group 1 (points + pixels).
pub struct SplatSlot {
    recs: wgpu::Buffer,
    group0: wgpu::BindGroup,
    group1: wgpu::BindGroup,
    cpu: Records,
    pub total: u32,
}

impl SplatSlot {
    /// A zeroed record table (`label`) and its two groups over `points` and `pixels`.
    pub fn new(cx: &SplatCx, label: &str, points: PointBufs, pixels: &PixelBufs) -> Self {
        let recs = zeroed_buffer(&cx.ctx.device, label, 16 + (MAX_RECORDS * REC_WORDS * 4) as u64, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let group0 = group0(cx, &recs);
        let group1 = group1(cx, points, pixels);

        Self { recs, group0, group1, cpu: Records::default(), total: 0 }
    }

    /// Re-point both groups at the current buffers - the lane grew or the canvas resized.
    pub fn rebind(&mut self, cx: &SplatCx, points: PointBufs, pixels: &PixelBufs) {
        self.group0 = group0(cx, &self.recs);
        self.group1 = group1(cx, points, pixels);
    }

    /// Rebuild this lane's records for the frame in `cx`, in place; `total` follows.
    pub fn build(&mut self, cx: &RecordCx, draws: &[CloudDraw], nodes: &[LodNode]) {
        records(cx, draws, nodes, &mut self.cpu);
        self.total = self.cpu.total;
    }

    /// Upload the records just built: the header at 0, the records at 16.
    pub fn write(&self, ctx: &GpuCtx) {
        ctx.queue.write_buffer(&self.recs, 0, bytemuck::bytes_of(&self.cpu.header));
        ctx.queue.write_buffer(&self.recs, 16, bytemuck::cast_slice(&self.cpu.recs));
    }

    /// Bind this lane and run the set pipeline over its threads; nothing when the lane is empty.
    pub(super) fn dispatch(&self, cp: &mut wgpu::ComputePass<'_>) {
        if self.total == 0 {
            return;
        }
        let (gx, gy) = dispatch_grid(self.total);
        cp.set_bind_group(0, &self.group0, &[]);
        cp.set_bind_group(1, &self.group1, &[]);
        cp.dispatch_workgroups(gx, gy, 1);
    }
}

/// 2D grid for `n` threads at 64 per group: a 1D dispatch caps at 65535 workgroups (~4.2M threads)
/// and an oversized dispatch silently invalidates the WHOLE command buffer. The rows are as
/// narrow as the count allows: a full 4096-wide last row ran 53% idle threads on the lion.
fn dispatch_grid(n: u32) -> (u32, u32) {
    let g = n.div_ceil(64);
    let gy = g.div_ceil(4096);
    (g.div_ceil(gy), gy)
}

/// Splat group 0 for one lane: the frame uniforms and that lane's record table.
fn group0(cx: &SplatCx, recs: &wgpu::Buffer) -> wgpu::BindGroup {
    cx.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("splat.group0"),
        layout: &cx.layouts.splat_group0,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: cx.frame.mvp_buffer.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: cx.frame.cloud_buffer.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: recs.as_entire_binding()},
        ],
    })
}

/// Splat group 1 for one lane: its point buffers and the shared per-pixel depth/colour pair.
fn group1(cx: &SplatCx, points: PointBufs, pixels: &PixelBufs) -> wgpu::BindGroup {
    cx.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("splat.group1"),
        layout: &cx.layouts.splat_group1,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: points.pos.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: points.col.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: pixels.depth.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 3, resource: pixels.color.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 4, resource: points.nrm.as_entire_binding()},
        ],
    })
}

/// The resolve pass's view of the per-pixel splat buffers.
fn resolve_group(cx: &SplatCx, pixels: &PixelBufs) -> wgpu::BindGroup {
    cx.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("splat.resolve.group"),
        layout: &cx.layouts.splat_resolve,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: pixels.depth.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: pixels.color.as_entire_binding()},
        ],
    })
}

/// The splatter over both lanes; `state` = the (mvp, cloud_size) the pixel buffers hold, None = stale.
pub struct Splat {
    pub pixels: PixelBufs,
    pub walked: SplatSlot,
    pub streamed: SplatSlot,
    pub resolve_group: wgpu::BindGroup,
    state: Option<([f32; 16], f32)>,
}

impl Splat {
    /// Per-pixel buffers for `size`, one slot per lane, the resolve group; nothing splatted yet.
    pub fn new(cx: &SplatCx, size: (u32, u32), walked: PointBufs, streamed: PointBufs) -> Self {
        let pixels = PixelBufs::new(cx.ctx, size);
        let walked = SplatSlot::new(cx, "splat.rescales", walked, &pixels);
        let streamed = SplatSlot::new(cx, "splat.stream.recs", streamed, &pixels);
        let resolve_group = resolve_group(cx, &pixels);

        Self { pixels, walked, streamed, resolve_group, state: None }
    }

    /// Re-point the five bind groups at the current buffers (set_scene, resize, stream growth).
    pub fn rebind(&mut self, cx: &SplatCx, walked: PointBufs, streamed: PointBufs) {
        self.walked.rebind(cx, walked, &self.pixels);
        self.streamed.rebind(cx, streamed, &self.pixels);
        self.resolve_group = resolve_group(cx, &self.pixels);
    }

    /// True when the buffers already hold the frame for (mvp, cloud_size) - the static skip.
    pub fn is_current(&self, mvp: &[f32; 16], cloud_size: f32) -> bool {
        self.state == Some((*mvp, cloud_size))
    }

    /// The buffers now hold the frame for (mvp, cloud_size).
    pub fn mark_current(&mut self, mvp: &[f32; 16], cloud_size: f32) {
        self.state = Some((*mvp, cloud_size));
    }

    /// Points, instances or pixels changed under the buffers: splat again next frame.
    pub fn invalidate(&mut self) {
        self.state = None;
    }

    /// Threads this frame over both lanes; 0 = no cloud on screen, the resolve is skipped.
    pub fn total(&self) -> u32 {
        self.walked.total + self.streamed.total
    }

    /// The cloud lane's one draw: the compute prelude resolved every cloud into the per-pixel
    /// buffers, so one fullscreen triangle composites them, writing frag_depth against the solids.
    pub fn draw_resolve(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, cloud: &wgpu::BindGroup) -> u32 {
        if self.total() == 0 {
            return 0;
        }

        pass.set_pipeline(&p.splat_resolve);
        pass.set_bind_group(0, cloud, &[]);
        pass.set_bind_group(1, &self.resolve_group, &[]);
        pass.draw(0..3, 0..1);
        1
    }
}

/// The per-frame facts the record builder needs, gathered once.
pub struct RecordCx<'a> {
    pub mvp: &'a [f32; 16],
    pub ortho_h: f32,
    pub eye: [f32; 3],
    pub size: (u32, u32),
    pub cloud_size: f32,
    pub lod_split_px: f32,
    pub objects: &'a InstanceTable,
}

/// One cloud's constants, shared by every record it emits: the folded matrix, the model (the
/// LOD walk places nodes with it), tint, scale, size factor, the draw's first row and spacing.
struct CloudCx {
    m: [f32; 16],
    model: [f32; 16],
    tint: [f32; 4],
    mscale: f64,
    px: f32,
    first: u32,
    spacing: f32,
}

impl CloudCx {
    /// The constants for one draw; `None` when its instance is missing or hidden, or px is zero.
    fn new(cx: &RecordCx, d: &CloudDraw) -> Option<Self> {
        let Some(row) = cx.objects.row(d.instance) else { return None };
        let Some(model) = cx.objects.anchored_model(d.instance) else { return None };
        if row.flags & Instance::FLAG_HIDDEN != 0 {
            return None;
        }
        let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * cx.cloud_size;
        if px <= 0.0 {
            return None;
        }

        // column-major 4x4: combined = mvp x model - one per cloud, shared by every
        // record the cloud emits
        let (a, b) = (cx.mvp, &model);
        let mut m = [0.0f32; 16];
        for col in 0..4 {
            for r in 0..4 {
                m[col * 4 + r] = (0..4).map(|k| a[k * 4 + r] * b[col * 4 + k]).sum();
            }
        }
        // tint.a smuggles the MINIMUM radius (the manifest px, halved): without a
        // floor, attenuation turns distant clouds to dust. With octree LOD a far node
        // carries BIGGER spacing (Potree's answer), but the floor still guards leaves.
        let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];
        // spacing is in the cloud's LOCAL units; col0's length is the model scale
        let mscale = ((model[0] as f64).powi(2) + (model[1] as f64).powi(2) + (model[2] as f64).powi(2)).sqrt();

        Some(Self { m, model, tint, mscale, px, first: d.first, spacing: d.spacing })
    }

    /// One record = one contiguous range at one spacing. world radius = spacing x
    /// (px/6); k folds the projection so the shader only divides by clip.w:
    ///   perspective: r_px = world_r * cot(fov/2) * (vp_h/2) / w
    ///   ortho:       r_px = world_r * vp_h / (2*ortho_h), and w = 1
    fn record(&self, cx: &RecordCx, first: u32, count: u32, sp: f32) -> SplatRecord {
        let world_r = (sp as f64).max(1.0e-9) * self.mscale * 0.001 * (self.px as f64) / 6.0; // metres
        let k = if cx.ortho_h > 0.0 { world_r / (2.0 * cx.ortho_h as f64) }
                else { world_r * 1.7320508 * 0.5 }; // cot(30 deg) / 2
        // the MODEL rotation columns (translation-free), so a cloud with
        // normals can rotate them into world space for the lambert term
        let b = &self.model;
        let rot = [
            b[0], b[1], b[2], 0.0f32,
            b[4], b[5], b[6], 0.0,
            b[8], b[9], b[10], 0.0,
        ];

        SplatRecord { mvp_model: self.m, tint: self.tint, first, count, cum: 0, k: k as f32, rot }
    }
}

/// Append a record with its thread offset; a full table drops it.
fn push_record(out: &mut Records, mut rec: SplatRecord) {
    if out.header[0] >= MAX_RECORDS {
        return;
    }
    rec.cum = out.total;
    out.header[0] += 1;
    out.total += rec.count;
    out.recs.push(rec);
}

/// Build the record table for one cloud lane. A record folds the cloud's whole per-frame
/// state: mvp x rebased model as ONE matrix, the tint, the attenuation constant and the
/// model rotation - so a thread does one mat-vec, no instance fetch.
/// Attenuated (world-sized) dots, Potree-style: the record carries k such that the
/// shader's radius is clamp(k * vp_h / clip.w, ...) px - a point covers its own
/// world-space footprint, so near surfaces close up gap-free and far points shrink.
/// The manifest px is a size FACTOR on the measured spacing.
pub fn records(cx: &RecordCx, draws: &[CloudDraw], nodes: &[LodNode], out: &mut Records) {
    out.clear();
    for d in draws {
        let Some(cloud) = CloudCx::new(cx, d) else { continue };
        if cx.lod_split_px > 0.0 && d.node_count > 0 {
            let slice = &nodes[d.node_first as usize..(d.node_first + d.node_count) as usize];
            walk_nodes(cx, &cloud, slice, out);
        } else {
            push_record(out, cloud.record(cx, d.first, d.count, d.spacing));
        }
    }
    out.header[1] = out.total;
}

/// Octree LOD, Potree-style screen-error selection: every VISITED node
/// contributes its own subsample, and the walk descends while the node's
/// projected point spacing is coarser than the cutoff - far nodes stop at
/// the root (a handful of coarse points), near nodes go deep. Coarse nodes
/// carry big spacing, so attenuation grows their dots to close the gaps.
fn walk_nodes(cx: &RecordCx, cloud: &CloudCx, slice: &[LodNode], out: &mut Records) {
    let ortho_h = cx.ortho_h as f64;
    let vp_h = cx.size.1 as f64;
    let aspect = cx.size.0 as f64 / cx.size.1 as f64;
    let eye = cx.eye;
    let (m, mscale) = (&cloud.m, cloud.mscale);
    out.stack.clear();
    out.stack.push(0);
    while let Some(ni) = out.stack.pop() {
        if out.header[0] >= MAX_RECORDS {
            break;
        }
        let nd = slice[ni];
        let c = nd.center;
        // FRUSTUM CULL on the node's bounding sphere, in clip space through the
        // folded matrix: an off-screen subtree costs nothing - and without this
        // a close zoom would visit every node and starve the 256-record table.
        let r_m = nd.size as f64 * 0.8660254 * mscale * 0.001; // sphere radius, metres
        let cw = (m[3] * c[0] + m[7] * c[1] + m[11] * c[2] + m[15]) as f64;
        if ortho_h <= 0.0 && cw < -r_m { continue; } // fully behind the eye
        let clip_x = (m[0] * c[0] + m[4] * c[1] + m[8] * c[2] + m[12]) as f64;
        let clip_y = (m[1] * c[0] + m[5] * c[1] + m[9] * c[2] + m[13]) as f64;
        let (ndc_x, ndc_y, ry) = if ortho_h > 0.0 {
            (clip_x, clip_y, r_m / ortho_h)
        } else {
            let w = cw.max(1.0e-9);
            (clip_x / w, clip_y / w, r_m * 1.7320508 / w)
        };
        if ndc_x.abs() > 1.0 + ry / aspect.min(1.0) || ndc_y.abs() > 1.0 + ry {
            continue; // the whole subtree is outside the view
        }
        // node centre in anchored world units - the eye's space
        let w = [
            cloud.model[0] * c[0] + cloud.model[4] * c[1] + cloud.model[8] * c[2] + cloud.model[12],
            cloud.model[1] * c[0] + cloud.model[5] * c[1] + cloud.model[9] * c[2] + cloud.model[13],
            cloud.model[2] * c[0] + cloud.model[6] * c[1] + cloud.model[10] * c[2] + cloud.model[14],
        ];
        let dist_m = (((w[0] - eye[0]).powi(2) + (w[1] - eye[1]).powi(2) + (w[2] - eye[2]).powi(2)) as f64).sqrt() * 0.001;
        let sp_m = nd.spacing as f64 * mscale * 0.001;
        let sp_px = if ortho_h > 0.0 { sp_m * vp_h / (2.0 * ortho_h) }
                    else { sp_m * 1.7320508 * 0.5 * vp_h / dist_m.max(1.0e-9) };
        let leaf = nd.children.iter().all(|&ch| ch < 0);
        let refine = !leaf && sp_px > cx.lod_split_px as f64;
        // Dot size: a REFINED node's region also receives all its deeper
        // points, so its own subsample renders at the cloud's measured
        // spacing - otherwise coarse dots blob over the fine layer under
        // them. Only the unrefined FRINGE keeps its coarse node spacing
        // (its points are the only ink there - big dots close the gaps);
        // a node can never be DENSER than the raw cloud, so the measured
        // spacing is also the floor there. Leaves hold raw points.
        let sp = if refine || leaf { cloud.spacing } else { nd.spacing.max(cloud.spacing) };
        // `nd.first` is relative to this cloud's own first point
        push_record(out, cloud.record(cx, cloud.first + nd.first, nd.count, sp));
        if refine {
            for &ch in &nd.children {
                if ch >= 0 { out.stack.push(ch as usize); }
            }
        }
    }
}
