// --8<-- [start:clip-plane]
// Clipping plane = a plane object that hides everything on the side its arrow points to; up to MAX_PLANES cut at once.
// Section cap = the flat face a cut reveals inside a closed solid; without it the solid looks hollow.
// Crossing count = the surfaces behind the plane along the view ray, leaving minus entering: above zero, the pixel is inside a solid.
use super::Gpu;
use super::arena::ArenaLane;
use super::buffers::{GpuCtx, zeroed_buffer};
use super::frame::{Binds, FrameInput};
use super::objects::InstanceTable;
use super::pass::{Frame, Pass};
use super::targets::{Attachment, Targets, TextureSpec};
use super::view::View;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Target, build, instance_id_layout,
    layout, scene_module, vertex_layout,
};
use session_rust::{AABB, Xform};
use std::ops::Range;
use wgpu::PrimitiveTopology::TriangleList;

pub use super::frame::{ClipUniform, MAX_PLANES};

/// Hatch spacing, CSS px.
const HATCH_CSS_PX: f64 = 8.0;

/// Crossings behind the plane per sample, a selected solid's weighing SELECTED_CROSSING; exact small integers.
const COUNT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::R16Float;

/// Words per pick pixel and plane: count, id sum, nearest exit, owner.
const PICK_WORDS: u64 = 4;

/// The section cap shader, before its per-sample-count prefix.
const CAP: &str = shader!("cap.wgsl");

/// The face shader the section counts share.
const TRIANGLE: &str = shader!("triangle.wgsl");

/// One clipping plane in world space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ClipPlane {
    pub normal: [f64; 3], // unit normal toward the kept side
    pub offset: f64,      // kept where normal · p + offset >= 0
    pub origin: [f64; 3], // a point on the plane, where the hatch starts
    pub hatch: [f64; 3],  // unit direction across the hatch lines, in the plane
}

impl ClipPlane {
    /// Signed distance of world point `p`; below zero is cut away.
    pub fn distance(&self, p: [f64; 3]) -> f64 {
        dot(self.normal, p) + self.offset
    }

    /// True when the plane passes through the box, or so near it, with the scene origin at
    /// `anchor`, that f32 rounding on the GPU may cut a face lying on the plane.
    pub fn crosses(&self, b: &AABB, anchor: [f64; 3]) -> bool {
        let reach =
            self.normal[0].abs() * b.hx + self.normal[1].abs() * b.hy + self.normal[2].abs() * b.hz;
        let center = [b.cx, b.cy, b.cz];
        let size = b.hx + b.hy + b.hz;
        let local: f64 = (0..3).map(|i| (center[i] - anchor[i]).abs()).sum();
        let world: f64 = center.iter().map(|v| v.abs()).sum();
        // 32 times CLIP_SLACK in clip.wgsl, plus the f32 rounding of the vertices themselves
        let slack =
            (local + size + self.distance(anchor).abs()) / 4096.0 + (world + size) / 1048576.0;
        self.distance(center).abs() <= reach + slack
    }
}
// --8<-- [end:clip-plane]

// --8<-- [start:clip-resources]
/// What the uniform is built from each frame.
pub struct ClipView<'a> {
    pub view_proj: &'a Xform, // camera matrix relative to the anchor
    pub anchor: [f64; 3],     // scene origin of that matrix
    pub height: u32,          // canvas height, px
    pub pixel_scale: f64,     // canvas px per CSS px
    pub samples: u32,         // scene samples per pixel
}

/// The count texture of one plane and its binding.
struct Counts {
    texture: Attachment,    // crossings per sample
    size: (u32, u32),       // canvas size, px
    samples: u32,           // scene samples
    group: wgpu::BindGroup, // the texture, for the caps
}

/// Pick records of every plane over the pick window.
struct PickCaps {
    buffer: wgpu::Buffer,   // PICK_WORDS per plane and pixel
    target: Attachment,     // the window the counts rasterize into; nothing is written
    size: (u32, u32),       // window size, px
    group: wgpu::BindGroup, // the records, for the pick draws
}
// --8<-- [end:clip-resources]

// --8<-- [start:clip-placed]
/// Instanced solids a plane crosses: per plane, (definition face indices, placed range), and one
/// (row, plane) record per placed solid, drawn as the instances of the definition's faces.
#[derive(Default)]
struct Placed {
    runs: [Vec<(Range<u32>, Range<u32>)>; MAX_PLANES], // per plane: faces, then records
    records: Vec<[u32; 2]>,                            // row and plane per placed solid
    buffer: Option<wgpu::Buffer>,                      // the records, once uploaded
}

impl Placed {
    /// Upload the records if they changed since the last upload.
    fn upload(&mut self, ctx: &GpuCtx) {
        if self.buffer.is_some() || self.records.is_empty() {
            return;
        }

        let buffer = zeroed_buffer(
            &ctx.device,
            "clip.placed",
            self.records.len() as u64 * 8,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        );
        ctx.queue
            .write_buffer(&buffer, 0, bytemuck::cast_slice(&self.records));
        self.buffer = Some(buffer);
    }

    /// Draw plane `plane`'s placed solids with `pipeline`; returns the draw count.
    fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        arena: &ArenaLane,
        b: &Binds,
        pipeline: &Pipeline,
        group: Option<&wgpu::BindGroup>,
        plane: usize,
    ) -> u32 {
        let Some(buffer) = &self.buffer else {
            return 0;
        };
        arena.draw_placed(pass, b, pipeline, group, &self.runs[plane], buffer)
    }
}

/// Group `placed` (plane, definition faces, row) into draws: per plane, one run per definition.
fn group_placed(
    mut placed: Vec<(u32, Range<u32>, u32)>,
) -> ([Vec<(Range<u32>, Range<u32>)>; MAX_PLANES], Vec<[u32; 2]>) {
    placed.sort_by_key(|(plane, faces, row)| (*plane, faces.start, *row));
    let mut runs: [Vec<(Range<u32>, Range<u32>)>; MAX_PLANES] = Default::default();
    let mut records = Vec::with_capacity(placed.len());

    for (plane, faces, row) in placed {
        let at = records.len() as u32;
        records.push([row, plane]);
        let plane_runs = &mut runs[plane as usize];

        match plane_runs.last_mut() {
            Some((last, rows)) if *last == faces && rows.end == at => rows.end = at + 1,
            _ => plane_runs.push((faces, at..at + 1)),
        }
    }

    (runs, records)
}

/// The placed records at locations 3 and 4, stepped per instance: row and plane.
const PLACED_ATTRIBUTES: [wgpu::VertexAttribute; 2] = [
    wgpu::VertexAttribute {
        offset: 0,
        shader_location: 3,
        format: wgpu::VertexFormat::Uint32,
    },
    wgpu::VertexAttribute {
        offset: 4,
        shader_location: 4,
        format: wgpu::VertexFormat::Uint32,
    },
];

/// The placed records beside the arena vertices.
fn placed_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &PLACED_ATTRIBUTES,
    }
}
// --8<-- [end:clip-placed]

// --8<-- [start:clip-struct]
/// Pipelines at the scene's sample count.
struct CapPipelines {
    counts: wgpu::BindGroupLayout, // group 3 of the caps: the count texture
    primitives: wgpu::BindGroupLayout, // group 3 of the mask draws: the triangle ids
    count: Pipeline,               // crossings behind one plane
    count_placed: Option<Pipeline>, // the same for instanced solids, made on the first
    cap: Pipeline,                 // section caps into the face pass
    masks: Pipeline,               // caps into both outline masks
    selection: Pipeline,           // selected caps into the selection mask
}

/// Pipelines of the pick through the caps.
struct PickPipelines {
    layout: wgpu::BindGroupLayout, // group 3 of the pick draws: the records
    count: Pipeline,               // crossings into the pick records
    owner: Pipeline,               // the nearest exit names the owner
    placed: Option<[Pipeline; 2]>, // count and owner of instanced solids, made on the first
    ids: Pipeline,                 // caps into the pick
}

/// Clipping planes: what they cut, their section caps, and the caps in the pick.
pub struct Clip {
    planes: [ClipPlane; MAX_PLANES], // world planes, the first `count` in use
    count: usize,                    // planes in use
    pub enabled: bool,               // false shows everything, the planes stay
    pub fill: u32,                   // 0 black hatch, 1 solid light grey
    target: Target,                  // the scene's color format and samples
    pipes: Option<CapPipelines>,     // made on the first section, again after an MSAA flip
    pick_pipes: Option<PickPipelines>, // made on the first pick through a section
    counts: Option<Counts>,          // made on the first cap, freed with the last plane
    primitives: Option<(wgpu::TextureView, wgpu::BindGroup)>, // triangle ids bound for the masks
    pick: Option<PickCaps>,          // made on the first pick through a cap
    runs: [Vec<Range<u32>>; MAX_PLANES], // per plane, face indices of the closed solids it crosses
    placed: Placed,                  // the instanced closed solids each plane crosses
    runs_for: Option<u64>,           // the geometry revision the runs were found for
}
// --8<-- [end:clip-struct]

// --8<-- [start:clip-find]
impl Clip {
    /// No planes and no pipelines: they are made when a plane first cuts a closed solid.
    pub fn new(target: Target) -> Self {
        Self {
            planes: [ClipPlane::default(); MAX_PLANES],
            count: 0,
            enabled: true,
            fill: 1,
            target,
            pipes: None,
            pick_pipes: None,
            counts: None,
            primitives: None,
            pick: None,
            runs: Default::default(),
            placed: Placed::default(),
            runs_for: None,
        }
    }

    /// Find, per plane, the face indices of the closed solids it crosses; `faces` names a row's,
    /// true when an instance draws them from its definition. No other solid can hold a section:
    /// one beyond the plane is left as often as it is entered.
    pub fn find_solids(
        &mut self,
        rows: &InstanceTable,
        faces: impl Fn(u32) -> Option<(Range<u32>, bool)>,
    ) {
        let revision = rows.geometry_revision();

        if self.runs_for == Some(revision) {
            return;
        }

        self.runs_for = Some(revision);
        let anchor = rows.anchor();
        let mut placed = Vec::new();

        for (index, (plane, runs)) in self.planes[..self.count]
            .iter()
            .zip(&mut self.runs)
            .enumerate()
        {
            runs.clear();

            for &row in rows.closed_rows() {
                if !rows
                    .row_bounds(row)
                    .is_some_and(|b| plane.crosses(&b, anchor))
                {
                    continue;
                }

                let Some((range, instanced)) = faces(row) else {
                    continue;
                };

                if instanced {
                    placed.push((index as u32, range, row));
                    continue;
                }

                // neighbouring rows usually sit side by side in the index buffer
                match runs.last_mut() {
                    Some(last) if last.end == range.start => last.end = range.end,
                    _ => runs.push(range),
                }
            }
        }

        let (runs, records) = group_placed(placed);
        self.placed = Placed {
            runs,
            records,
            buffer: None,
        };
    }

    /// True when plane `plane` crosses a closed solid.
    fn cuts(&self, plane: usize) -> bool {
        !self.runs[plane].is_empty() || !self.placed.runs[plane].is_empty()
    }
    // --8<-- [end:clip-find]

    // --8<-- [start:clip-set]
    /// Take `planes`, at most MAX_PLANES; true when they differ from the last ones.
    pub fn set(&mut self, planes: &[ClipPlane]) -> bool {
        let count = planes.len().min(MAX_PLANES);

        if count == self.count && self.planes[..count] == planes[..count] {
            return false;
        }

        self.planes[..count].copy_from_slice(&planes[..count]);
        self.count = count;

        // the last plane gone: free the count texture and the pick records
        if count == 0 {
            self.counts = None;
            self.pick = None;
        }

        true
    }

    /// Planes cutting now.
    pub fn count(&self) -> usize {
        self.count
    }

    /// The planes cutting now, in world space.
    pub fn planes(&self) -> &[ClipPlane] {
        &self.planes[..self.count]
    }

    /// Every plane as (normal, offset) in world space; unused ones are zero and cut nothing.
    pub fn world(&self) -> [[f64; 4]; MAX_PLANES] {
        let mut out = [[0.0; 4]; MAX_PLANES];

        for (slot, plane) in out.iter_mut().zip(self.planes()) {
            *slot = [
                plane.normal[0],
                plane.normal[1],
                plane.normal[2],
                plane.offset,
            ];
        }

        out
    }

    /// This frame's uniform: the planes relative to the anchor, over clip space, with their hatch.
    pub fn uniform(&self, v: &ClipView) -> ClipUniform {
        clip_uniform(self.planes(), self.fill, v)
    }
    // --8<-- [end:clip-set]

    // --8<-- [start:clip-counts]
    /// Make the pipelines, then the count texture for a `size` canvas.
    pub fn prepare_counts(&mut self, ctx: &GpuCtx, l: &Layouts, size: (u32, u32)) {
        let target = self.target;
        let pipes = self
            .pipes
            .get_or_insert_with(|| cap_pipelines(ctx, l, target));
        self.placed.upload(ctx);

        if self.placed.buffer.is_some() && pipes.count_placed.is_none() {
            pipes.count_placed = Some(count_placed_pipeline(ctx, l, target));
        }

        if self
            .counts
            .as_ref()
            .is_some_and(|counts| counts.size == size)
        {
            return;
        }

        let samples = target.samples;
        let texture = Attachment::new(
            ctx,
            "clip.counts",
            &TextureSpec {
                size,
                format: COUNT_FORMAT,
                samples,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
            },
        );
        let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("clip.counts"),
            layout: &pipes.counts,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture),
            }],
        });
        self.counts = Some(Counts {
            texture,
            size,
            samples,
            group,
        });
    }

    /// Count the crossings of closed solids behind plane `plane` into the count texture.
    pub fn encode_count(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        arena: &ArenaLane,
        b: &Binds,
        plane: u32,
    ) -> u32 {
        let (Some(counts), Some(pipes)) = (&self.counts, &self.pipes) else {
            return 0;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("section counts"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &counts.texture,
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
        // additive blending makes the draw a counter: each surface adds +1 leaving or -1 entering at every sample
        arena.draw_solids(
            &mut pass,
            b,
            &pipes.count,
            None,
            &self.runs[plane as usize],
            plane,
        ) + pipes.count_placed.as_ref().map_or(0, |pipeline| {
            self.placed
                .draw(&mut pass, arena, b, pipeline, None, plane as usize)
        })
    }

    /// Draw plane `plane`'s section caps into the open face pass, before the faces.
    pub fn draw_cap(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, plane: u32) -> u32 {
        let (Some(counts), Some(pipes)) = (&self.counts, &self.pipes) else {
            return 0;
        };
        pass.set_pipeline(&pipes.cap);
        b.set(pass);
        pass.set_bind_group(3, &counts.group, &[]);
        // the instance index carries the plane number into the shader
        pass.draw(0..3, plane..plane + 1);
        1
    }
    // --8<-- [end:clip-counts]

    // --8<-- [start:clip-masks]
    /// Bind the face pass's triangle ids for the mask draws.
    pub fn bind_primitives(&mut self, ctx: &GpuCtx, targets: &Targets) {
        let Some(pipes) = &self.pipes else {
            return;
        };

        if self
            .primitives
            .as_ref()
            .is_some_and(|(view, _)| *view == targets.gradient.view)
        {
            return;
        }

        let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("clip.primitives"),
            layout: &pipes.primitives,
            entries: &[wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&targets.gradient),
            }],
        });
        self.primitives = Some((targets.gradient.view.clone(), group));
    }

    /// The caps into both outline masks of an open mask pass.
    pub fn draw_cap_masks(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.pipes
            .as_ref()
            .map_or(0, |pipes| self.draw_primitives(pass, b, &pipes.masks, 2))
    }

    /// The selected caps into the open selection mask pass.
    pub fn draw_cap_selection(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.pipes.as_ref().map_or(0, |pipes| {
            self.draw_primitives(pass, b, &pipes.selection, 1)
        })
    }

    /// A fullscreen draw over the triangle ids, `instances` times.
    fn draw_primitives(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &Pipeline,
        instances: u32,
    ) -> u32 {
        let Some((_, group)) = &self.primitives else {
            return 0;
        };
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, group, &[]);
        pass.draw(0..3, 0..instances);
        1
    }
    // --8<-- [end:clip-masks]

    // --8<-- [start:clip-pick]
    /// Count every plane's crossings over the pick window `size`, then name each pixel's owner.
    pub fn encode_pick(
        &mut self,
        ctx: &GpuCtx,
        l: &Layouts,
        encoder: &mut wgpu::CommandEncoder,
        arena: &ArenaLane,
        b: &Binds,
        size: (u32, u32),
    ) -> u32 {
        let pipes = self
            .pick_pipes
            .get_or_insert_with(|| pick_pipelines(ctx, l));
        self.placed.upload(ctx);

        if self.placed.buffer.is_some() && pipes.placed.is_none() {
            pipes.placed = Some(pick_placed_pipelines(ctx, l, &pipes.layout));
        }
        let words = self.count as u64 * u64::from(size.0) * u64::from(size.1) * PICK_WORDS;

        if !self
            .pick
            .as_ref()
            .is_some_and(|pick| pick.size == size && pick.buffer.size() >= words * 4)
        {
            // records for the planes cutting now; a plane added later grows it
            let buffer = zeroed_buffer(
                &ctx.device,
                "clip.pick",
                words * 4,
                wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            );
            let target = Attachment::new(
                ctx,
                "clip.pick.window",
                &TextureSpec {
                    size,
                    format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                    samples: 1,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                },
            );
            let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("clip.pick"),
                layout: &pipes.layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 5,
                    resource: buffer.as_entire_binding(),
                }],
            });
            self.pick = Some(PickCaps {
                buffer,
                target,
                size,
                group,
            });
        }

        let Some(pick) = &self.pick else {
            return 0;
        };
        encoder.clear_buffer(&pick.buffer, 0, Some(words * 4));
        let mut draws = 0;

        // counts first; the owners compare against the finished nearest exits
        for (k, pipeline) in [&pipes.count, &pipes.owner].into_iter().enumerate() {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("section pick"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &pick.target,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            for (plane, runs) in self.runs[..self.count].iter().enumerate() {
                draws += arena.draw_solids(
                    &mut pass,
                    b,
                    pipeline,
                    Some(&pick.group),
                    runs,
                    plane as u32,
                );

                if let Some(placed) = &pipes.placed {
                    let group = Some(&pick.group);
                    draws += self
                        .placed
                        .draw(&mut pass, arena, b, &placed[k], group, plane);
                }
            }
        }

        draws
    }

    /// Every plane's caps into the open pick pass, as their owners' ids.
    pub fn draw_cap_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let (Some(pick), Some(pipes)) = (&self.pick, &self.pick_pipes) else {
            return 0;
        };
        pass.set_pipeline(&pipes.ids);
        b.set(pass);
        pass.set_bind_group(3, &pick.group, &[]);
        pass.draw(0..3, 0..self.count as u32);
        1
    }

    /// Bytes reserved on the GPU: (buffers, textures).
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let texture = self.counts.as_ref().map_or(0, |counts| {
            u64::from(counts.size.0) * u64::from(counts.size.1) * u64::from(counts.samples) * 2
        });
        let (buffer, window) = self.pick.as_ref().map_or((0, 0), |pick| {
            (
                pick.buffer.size(),
                u64::from(pick.size.0) * u64::from(pick.size.1),
            )
        });
        (buffer, texture + window)
    }
}
// --8<-- [end:clip-pick]

// --8<-- [start:clip-gpu]
impl Clip {
    /// The planes whose sections this frame draws, and how many: each crosses a closed solid.
    fn cap_planes(&self, view: &View) -> ([u32; MAX_PLANES], usize) {
        let mut planes = [0; MAX_PLANES];
        let mut count = 0;

        // x-ray draws no faces, so no sections either
        if view.opacity <= 0.0 {
            return (planes, 0);
        }

        for plane in 0..self.count {
            if self.cuts(plane) {
                planes[count] = plane as u32;
                count += 1;
            }
        }

        (planes, count)
    }
}

impl super::Gpu {
    /// Cut the scene with `planes`; the cached passes run again only when they changed, and the
    /// ink pipelines swap to their clip tests when cutting starts and back when it stops.
    pub fn set_clip_planes(&mut self, planes: &[ClipPlane]) {
        if !self.pass_mut::<Clip>().set(planes) {
            return;
        }

        self.objects.geometry_changed();
        let cutting = self.pass::<Clip>().count > 0;

        // `replace` stores the new value and returns the old one: true only when cutting starts or stops
        if self.ctx.cache.clipping.replace(cutting) != cutting {
            self.rebuild_pipelines();
        }
    }

    /// Find, per plane, the closed solids it crosses; `faces` names a row's face indices.
    pub fn find_solids(&mut self, faces: impl Fn(u32) -> Option<(Range<u32>, bool)>) {
        super::pass::find_mut::<Clip>(&mut self.passes).find_solids(&self.objects, faces);
    }
}

/// The clip pass, without planes.
pub fn pass(_ctx: &GpuCtx, target: Target) -> Box<dyn Pass> {
    Box::new(Clip::new(target))
}
// --8<-- [end:clip-gpu]

// --8<-- [start:clip-pass]
impl Pass for Clip {
    fn clip_world(&self) -> Option<[[f64; 4]; MAX_PLANES]> {
        Some(self.world())
    }

    fn write_frame(&mut self, g: &mut Gpu, input: &FrameInput) {
        let size = (g.config.width, g.config.height);
        let clip = self.uniform(&ClipView {
            view_proj: &input.view_proj,
            anchor: g.objects.anchor(),
            height: size.1,
            pixel_scale: f64::from(size.0) / g.logical_size[0].max(1.0),
            samples: g.targets.samples,
        });
        g.frame.write_clip(&g.ctx, &clip);
    }

    /// For each plane that crosses a closed solid, the crossings are counted first and its
    /// section caps drawn before the faces; every plane but the last in a face pass of its own.
    fn before_faces(
        &mut self,
        g: &mut Gpu,
        encoder: &mut wgpu::CommandEncoder,
        f: &Frame,
        drew: &mut bool,
    ) -> u32 {
        let (planes, count) = self.cap_planes(&g.view);
        let size = (g.config.width, g.config.height);
        let mut draws = 0;

        if count > 0 {
            self.prepare_counts(&g.ctx, &g.layouts, size);
        }

        for &plane in &planes[..count.saturating_sub(1)] {
            let b = g.frame.binds(&g.objects.group);
            draws += self.encode_count(encoder, &g.arena, &b, plane);
            let mut pass = g
                .targets
                .begin_faces(encoder, f.view, (!*drew).then_some(f.clear));

            if !*drew {
                draws += g.backdrop_list(&mut pass, &b);
            }

            *drew = true;
            draws += self.draw_cap(&mut pass, &b, plane);
        }

        if count > 0 {
            let b = g.frame.binds(&g.objects.group);
            draws += self.encode_count(encoder, &g.arena, &b, planes[count - 1]);
            // --8<-- [start:32-mark-counts]
            g.mark(encoder, "counts"); // register:gtao
            // --8<-- [end:32-mark-counts]
        }

        draws
    }

    /// The last plane's caps, into the face pass the faces follow in.
    fn in_faces(&self, g: &Gpu, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let (planes, count) = self.cap_planes(&g.view);

        if count == 0 {
            return 0;
        }

        self.draw_cap(pass, b, planes[count - 1])
    }

    fn clips(&self) -> bool {
        self.count() > 0
    }

    fn bind_masks(&mut self, g: &Gpu) {
        if self.cap_planes(&g.view).1 > 0 {
            self.bind_primitives(&g.ctx, &g.targets);
        }
    }

    fn in_masks(&self, g: &Gpu, pass: &mut wgpu::RenderPass<'_>, b: &Binds, both: bool) -> u32 {
        if self.cap_planes(&g.view).1 == 0 {
            0
        } else if both {
            self.draw_cap_masks(pass, b)
        } else {
            self.draw_cap_selection(pass, b)
        }
    }

    /// Which solid each section cap pixel belongs to.
    fn before_ids(
        &mut self,
        g: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        b: &Binds,
        size: (u32, u32),
    ) {
        if self.cap_planes(&g.view).1 > 0 {
            self.encode_pick(&g.ctx, &g.layouts, encoder, &g.arena, b, size);
        }
    }

    fn in_ids(&self, g: &Gpu, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        if self.cap_planes(&g.view).1 == 0 {
            return 0;
        }

        self.draw_cap_ids(pass, b)
    }
}

impl super::lane::Lane for Clip {
    fn on_retarget(&mut self, _ctx: &GpuCtx, _layouts: &Layouts, target: Target) {
        self.target = target;
        self.pipes = None;
        self.counts = None;
        self.primitives = None;
    }

    fn on_release(&mut self, _ctx: &GpuCtx, _layouts: &Layouts) {
        self.counts = None;
        self.primitives = None;
        self.pick = None;
    }

    fn bytes(&self) -> (u64, u64) {
        self.allocated_bytes()
    }
}
// --8<-- [end:clip-pass]

// --8<-- [start:clip-uniform]
/// The uniform of `planes`: relative to the anchor, over clip space, with their hatch.
pub fn clip_uniform(planes: &[ClipPlane], fill: u32, v: &ClipView) -> ClipUniform {
    let mut u = ClipUniform::default();
    if planes.is_empty() {
        return u;
    }

    let Some(inverse) = inverse(v.view_proj) else {
        return u;
    };
    let m = &v.view_proj.m;
    let ortho = v.view_proj.ortho_half_height() > 0.0;
    let eye = v.view_proj.eye();
    let toward = [m[2], m[6], m[10]]; // depth grows toward the eye
    // scene units per pixel at the anchor; the hatch phase wraps far above that
    let rows = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt();
    let per_px = 2.0 * m[15].abs() / (rows * f64::from(v.height.max(1)));
    let spacing = HATCH_CSS_PX * v.pixel_scale;
    let period = (spacing * per_px * 65536.0).log2().ceil().exp2();
    u.count = planes.len().min(MAX_PLANES) as u32;
    u.samples = v.samples;
    u.fill = fill;
    u.spacing = spacing as f32;
    u.width = v.pixel_scale.max(1.5) as f32;
    u.outline = (2.0 * v.pixel_scale) as f32;

    for (i, plane) in planes.iter().take(MAX_PLANES).enumerate() {
        let k = plane.normal;
        let p = [k[0], k[1], k[2], plane.offset + dot(k, v.anchor)];
        let s: [f64; 4] =
            std::array::from_fn(|j| (0..4).map(|r| inverse.m[j * 4 + r] * p[r]).sum());
        let side = if ortho {
            dot(k, toward)
        } else {
            dot(k, [eye[0], eye[1], eye[2]]) + p[3]
        };
        u.planes[i] = p.map(|value| value as f32);
        u.screen[i] = s.map(|value| value as f32);
        u.sides[i / 4][i % 4] = if side > 0.0 {
            1.0
        } else if side < 0.0 {
            -1.0
        } else {
            0.0
        };

        // edge on: no cap to hatch
        if s[2] == 0.0 {
            continue;
        }

        // a plane point over canvas (x, y) is inverse · (x, y, depth(x, y), 1), homogeneous
        let columns = [
            [1.0, 0.0, -s[0] / s[2], 0.0],
            [0.0, 1.0, -s[1] / s[2], 0.0],
            [0.0, 0.0, -s[3] / s[2], 1.0],
        ];
        let from = [
            v.anchor[0] - plane.origin[0],
            v.anchor[1] - plane.origin[1],
            v.anchor[2] - plane.origin[2],
        ];
        let phase = dot(plane.hatch, from);
        let phase = if period.is_normal() {
            phase.rem_euclid(period)
        } else {
            phase
        };

        for (c, column) in columns.iter().enumerate() {
            let h: [f64; 4] =
                std::array::from_fn(|r| (0..4).map(|q| inverse.m[q * 4 + r] * column[q]).sum());
            u.hatch[i][c] = (dot(plane.hatch, [h[0], h[1], h[2]]) + phase * h[3]) as f32;
            u.hatch_w[i][c] = h[3] as f32;
        }
    }

    u
}

/// The inverse of `m`, its rows scaled to one first: the kernel refuses a determinant below
/// 1e-12, which a camera in millimetres falls under.
fn inverse(m: &Xform) -> Option<Xform> {
    let scales: [f64; 4] = std::array::from_fn(|row| {
        (0..4)
            .map(|col| m.m[col * 4 + row].abs())
            .fold(0.0, f64::max)
    });

    if scales.iter().any(|scale| !scale.is_normal()) {
        return None;
    }

    let mut inverse =
        Xform::from_matrix(std::array::from_fn(|i| m.m[i] / scales[i % 4])).inverse()?;

    for (i, value) in inverse.m.iter_mut().enumerate() {
        *value /= scales[i / 4];
    }

    Some(inverse)
}

/// Dot product of two 3-vectors.
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
// --8<-- [end:clip-uniform]

// --8<-- [start:clip-pipelines]
/// The cap shader for a face pass at `samples`: its textures and their loaders first.
fn cap_source(samples: u32) -> String {
    let (counts, primitives, count, primitive) = if samples > 1 {
        (
            "texture_multisampled_2d<f32>",
            "texture_multisampled_2d<u32>",
            "textureLoad(counts, at, i32(k))",
            "textureLoad(primitives, at, i32(k))",
        )
    } else {
        (
            "texture_2d<f32>",
            "texture_2d<u32>",
            "textureLoad(counts, at, 0)",
            "textureLoad(primitives, at, 0)",
        )
    };
    format!(
        "@group(3) @binding(0) var counts: {counts}; // crossings behind the plane per sample
@group(3) @binding(1) var primitives: {primitives}; // the face pass's triangle ids
const SAMPLES: u32 = {samples}u; // samples per pixel
// Crossings behind the plane at sample `k`, a selected solid's weighing SELECTED_CROSSING.
fn count_at(at: vec2<i32>, k: u32) -> f32 {{ return {count}.x; }}
// Triangle id at sample `k`.
fn primitive_at(at: vec2<i32>, k: u32) -> u32 {{ let h = {primitive}.xy; return h.x | (h.y << 16u); }}
{CAP}"
    )
}

/// One texture binding of the fragment stage.
fn texture_entry(
    binding: u32,
    sample_type: wgpu::TextureSampleType,
    samples: u32,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: samples > 1,
        },
        count: None,
    }
}

/// The pipelines of the pick through the caps.
fn pick_pipelines(ctx: &GpuCtx, l: &Layouts) -> PickPipelines {
    let records = layout(
        ctx,
        "clip.pick",
        &[wgpu::BindGroupLayoutEntry {
            binding: 5,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    );
    let triangle = scene_module(ctx, "triangle.shader", TRIANGLE);
    let groups = [&l.mvp, &l.line, &l.instance, &records];
    let buffers = [vertex_layout(), instance_id_layout()];
    let window = Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel, never written
        samples: 1,
    };
    let solids = PipelineDesc::new(&triangle, &groups, &buffers, TriangleList)
        .vertex("vs_count")
        .color(ColorWrite::Nothing)
        .depth(DepthMode::Detached);
    let count = build(
        ctx,
        window,
        &solids.with("section pick count", "fs_pick_count"),
    );
    let owner = build(
        ctx,
        window,
        &solids.with("section pick owner", "fs_pick_owner"),
    );
    let shader = scene_module(ctx, "clip.cap", &cap_source(1));
    let ids = build(
        ctx,
        Target::ID,
        &PipelineDesc::new(&shader, &groups, &[], TriangleList)
            .vertex("vs_cap")
            .with("section cap ids", "fs_cap_id")
            .physical(),
    );
    PickPipelines {
        layout: records,
        count,
        owner,
        placed: None,
        ids,
    }
}

/// The pick's count and owner pipelines for instanced solids, `records` at group 3.
fn pick_placed_pipelines(
    ctx: &GpuCtx,
    l: &Layouts,
    records: &wgpu::BindGroupLayout,
) -> [Pipeline; 2] {
    let triangle = scene_module(ctx, "triangle.shader", TRIANGLE);
    let groups = [&l.mvp, &l.line, &l.instance, records];
    let buffers = [vertex_layout(), placed_layout()];
    let window = Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel, never written
        samples: 1,
    };
    let placed = PipelineDesc::new(&triangle, &groups, &buffers, TriangleList)
        .vertex("vs_count_placed")
        .color(ColorWrite::Nothing)
        .depth(DepthMode::Detached);
    [
        build(
            ctx,
            window,
            &placed.with("section pick count placed", "fs_pick_count"),
        ),
        build(
            ctx,
            window,
            &placed.with("section pick owner placed", "fs_pick_owner"),
        ),
    ]
}

/// The crossing count of instanced closed solids at the scene's sample count.
fn count_placed_pipeline(ctx: &GpuCtx, l: &Layouts, target: Target) -> Pipeline {
    let triangle = scene_module(ctx, "triangle.shader", TRIANGLE);
    let groups = [&l.mvp, &l.line, &l.instance];
    let buffers = [vertex_layout(), placed_layout()];
    let counted = Target {
        format: COUNT_FORMAT,
        samples: target.samples,
    };
    build(
        ctx,
        counted,
        &PipelineDesc::new(&triangle, &groups, &buffers, TriangleList)
            .with("section count placed", "fs_count")
            .vertex("vs_count_placed")
            .color(ColorWrite::Add)
            .depth(DepthMode::Detached),
    )
}

/// The count, cap and mask pipelines at the scene's sample count.
fn cap_pipelines(ctx: &GpuCtx, l: &Layouts, target: Target) -> CapPipelines {
    let counts = layout(
        ctx,
        "clip.counts",
        &[texture_entry(
            0,
            wgpu::TextureSampleType::Float { filterable: false },
            target.samples,
        )],
    );
    let primitives = layout(
        ctx,
        "clip.primitives",
        &[texture_entry(
            1,
            wgpu::TextureSampleType::Uint,
            target.samples,
        )],
    );
    let triangle = scene_module(ctx, "triangle.shader", TRIANGLE);
    let groups = [&l.mvp, &l.line, &l.instance];
    let buffers = [vertex_layout(), instance_id_layout()];
    let count = build(
        ctx,
        Target {
            format: COUNT_FORMAT,
            samples: target.samples,
        },
        &PipelineDesc::new(&triangle, &groups, &buffers, TriangleList)
            .with("section count", "fs_count")
            .vertex("vs_count")
            .color(ColorWrite::Add)
            .depth(DepthMode::Detached),
    );
    let shader = scene_module(ctx, "clip.cap", &cap_source(target.samples));
    let cap_groups = [&l.mvp, &l.line, &l.instance, &counts];
    let cap = build(
        ctx,
        target,
        &PipelineDesc::new(&shader, &cap_groups, &[], TriangleList)
            .with("section caps", "fs_cap")
            .vertex("vs_cap")
            .physical(),
    );
    let mask_groups = [&l.mvp, &l.line, &l.instance, &primitives];
    let base = PipelineDesc::new(&shader, &mask_groups, &[], TriangleList)
        .vertex("vs_cap")
        .depth(DepthMode::Always);
    let mask = Target {
        format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
        samples: target.samples,
    };
    let masks = build(
        ctx,
        mask,
        &base.with("section cap masks", "fs_cap_masks").masks(),
    );
    let selection = build(
        ctx,
        mask,
        &base.with("section cap selection", "fs_cap_selection"),
    );
    CapPipelines {
        counts,
        primitives,
        count,
        count_placed: None,
        cap,
        masks,
        selection,
    }
}
// --8<-- [end:clip-pipelines]

// --8<-- [start:clip-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::clipping::{Mode, clip_plane, plane_from};
    use crate::engine::gpu::Instance;
    use session_rust::{Point, Vector};

    /// A camera at `eye` looking at `target`, relative to `anchor`, in scene units; reverse depth.
    pub(super) fn view(
        eye: [f64; 3],
        target: [f64; 3],
        anchor: [f64; 3],
        perspective: bool,
    ) -> Xform {
        let at = |p: [f64; 3]| Point::new(p[0] - anchor[0], p[1] - anchor[1], p[2] - anchor[2]);
        let d = [target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]];
        let distance = dot(d, d).sqrt();
        // straight up or down: y is up on screen
        let up = if d[0].abs() + d[1].abs() < 1e-9 * distance {
            Vector::new(0.0, 1.0, 0.0)
        } else {
            Vector::new(0.0, 0.0, 1.0)
        };
        let look = Xform::look_at_right_handed(&at(eye), &at(target), &up);
        let projection = if perspective {
            Xform::perspective(60f64.to_radians(), 1.0, distance * 10.0, distance * 1e-3)
        } else {
            let h = distance * 30f64.to_radians().tan();
            Xform::orthographic(-h, h, -h, h, distance * 3.0, -distance * 3.0)
        };
        &projection * &look
    }

    /// The uniform for one plane under a camera.
    fn uniform_for(plane: &ClipPlane, view_proj: &Xform, anchor: [f64; 3]) -> ClipUniform {
        clip_uniform(
            std::slice::from_ref(plane),
            0,
            &ClipView {
                view_proj,
                anchor,
                height: 512,
                pixel_scale: 1.0,
                samples: 4,
            },
        )
    }

    /// The shader declares ClipUniform with the Rust fields at the Rust offsets, and the same flags.
    #[test]
    fn clip_uniform_mirror() {
        use std::mem::{offset_of, size_of};
        let source = format!(
            "@group(1) @binding(1) var<uniform> clipping: ClipUniform;\n{}",
            crate::engine::pipelines::CLIP
        );
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::default(),
        )
        .validate(&module)
        .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        let (_, ty) = module
            .types
            .iter()
            .find(|(_, ty)| ty.name.as_deref() == Some("ClipUniform"))
            .expect("ClipUniform in the shader");
        let naga::TypeInner::Struct { members, span } = &ty.inner else {
            panic!("ClipUniform is not a struct")
        };
        let offsets: Vec<u32> = members.iter().map(|member| member.offset).collect();
        let pad = offset_of!(ClipUniform, pad) as u32;
        let rust = [
            offset_of!(ClipUniform, planes),
            offset_of!(ClipUniform, screen),
            offset_of!(ClipUniform, hatch),
            offset_of!(ClipUniform, hatch_w),
            offset_of!(ClipUniform, sides),
            offset_of!(ClipUniform, count),
            offset_of!(ClipUniform, samples),
            offset_of!(ClipUniform, fill),
            offset_of!(ClipUniform, spacing),
            offset_of!(ClipUniform, width),
            offset_of!(ClipUniform, outline),
        ]
        .map(|offset| offset as u32);
        assert_eq!(offsets[..11], rust);
        assert_eq!(offsets[11..], [pad, pad + 4]);
        assert_eq!(*span as usize, size_of::<ClipUniform>());

        for (name, bit) in [
            ("FLAG_CLIPPING_PLANE", Instance::FLAG_CLIPPING_PLANE),
            ("FLAG_CLOSED", Instance::FLAG_CLOSED),
            ("FLAG_INWARD", Instance::FLAG_INWARD),
        ] {
            assert!(
                crate::engine::pipelines::CLIP.contains(&format!("const {name}: u32 = {bit}u;")),
                "{name} matches Instance::{name}"
            );
        }
    }

    /// Fragment stages never touch the object rows and vertex stages never touch fragment-only
    /// tables: the layouts show each binding to those stages only, and WebGPU rejects the pipeline.
    #[test]
    fn every_stage_stays_within_its_bindings() {
        use crate::engine::pipelines::{scene_source, shared};
        let scene = |source: &str| shared(&scene_source(source));
        // (group, binding) of the storage each stage may not use, per shader
        let triangle: &[((u32, u32), naga::ShaderStage)] = &[
            ((2, 0), naga::ShaderStage::Fragment),
            ((2, 1), naga::ShaderStage::Fragment),
            ((3, 0), naga::ShaderStage::Fragment),
            ((3, 1), naga::ShaderStage::Fragment),
            ((3, 2), naga::ShaderStage::Fragment),
            ((3, 3), naga::ShaderStage::Fragment),
            ((3, 4), naga::ShaderStage::Fragment),
            ((3, 5), naga::ShaderStage::Vertex),
        ];
        let rows: &[((u32, u32), naga::ShaderStage)] = &[
            ((2, 0), naga::ShaderStage::Fragment),
            ((2, 1), naga::ShaderStage::Fragment),
            ((3, 0), naga::ShaderStage::Vertex),
            ((3, 1), naga::ShaderStage::Vertex),
            ((3, 5), naga::ShaderStage::Vertex),
        ];
        let text_outline = shader!("text_outline.wgsl");

        for (name, source, forbidden) in [
            ("triangle.wgsl", scene(TRIANGLE), triangle),
            ("text_outline.wgsl", scene(text_outline), rows),
            ("cap.wgsl 1x", scene(&cap_source(1)), rows),
            ("cap.wgsl 4x", scene(&cap_source(4)), rows),
        ] {
            let module = naga::front::wgsl::parse_str(&source)
                .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
            let info = naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::default()
                    | naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32,
            )
            .validate(&module)
            .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));

            for (index, entry) in module.entry_points.iter().enumerate() {
                let uses = info.get_entry_point(index);

                for (handle, global) in module.global_variables.iter() {
                    let Some(binding) = &global.binding else {
                        continue;
                    };
                    let used = !uses[handle].is_empty();
                    let banned = forbidden.iter().any(|(at, stage)| {
                        *at == (binding.group, binding.binding) && *stage == entry.stage
                    });
                    assert!(
                        !(used && banned),
                        "{name}: {} uses {:?} at group {} binding {}",
                        entry.name,
                        global.name,
                        binding.group,
                        binding.binding
                    );
                }
            }
        }
    }

    /// Only a box the plane passes through can hold a section; touching counts, within rounding.
    #[test]
    fn a_plane_crosses_the_boxes_it_passes_through() {
        let plane = clip_plane(
            &plane_from(Mode::Xy, &[[0.0, 0.0, 50.0]], 10.0).unwrap(),
            &Xform::identity(),
        )
        .unwrap();
        let tilted = clip_plane(
            &plane_from(Mode::Normal, &[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]], 10.0).unwrap(),
            &Xform::identity(),
        )
        .unwrap();
        let block = |z: f64| AABB::new(0.0, 0.0, z, 10.0, 10.0, 10.0);
        let origin = [0.0; 3];
        assert!(plane.crosses(&block(45.0), origin));
        assert!(plane.crosses(&block(60.0), origin), "touching from above");
        assert!(!plane.crosses(&block(61.0), origin));
        assert!(!plane.crosses(&block(-100.0), origin));
        assert!(tilted.crosses(&block(0.0), origin));
        assert!(!tilted.crosses(&AABB::new(40.0, 40.0, 40.0, 5.0, 5.0, 5.0), origin));

        // a face on the plane whose box rounds a hair short of it still counts
        let bottom = clip_plane(
            &plane_from(Mode::Normal, &[[0.0, 0.0, 5.9], [0.0, 0.0, -4.1]], 10.0).unwrap(),
            &Xform::identity(),
        )
        .unwrap();
        let mut b = AABB::empty();
        b.union_with_point(13.7, -21.3, 5.9 + 1e-9);
        b.union_with_point(113.7, 58.7, 65.9);
        assert!(bottom.crosses(&b, [63.7, 18.7, 35.9]));
    }

    /// No plane: an all-zero uniform, written once and never again.
    #[test]
    fn no_plane_is_all_zero() {
        let eye = view([0.0, -500.0, 300.0], [0.0; 3], [0.0; 3], true);
        let u = clip_uniform(
            &[],
            1,
            &ClipView {
                view_proj: &eye,
                anchor: [0.0; 3],
                height: 512,
                pixel_scale: 2.0,
                samples: 4,
            },
        );
        assert_eq!(u, ClipUniform::default());
    }

    /// Far from the origin the planes relative to the anchor still cut where the world planes do.
    #[test]
    fn anchored_planes_match_the_world() {
        let anchor = [1.0e6, -3.0e5, 2.0e3];
        let origin = [anchor[0] + 3.0, anchor[1] - 2.0, anchor[2] + 1.0];
        let normal = [origin[0] + 1.0, origin[1] + 2.0, origin[2] + 2.0];
        let plane = plane_from(Mode::Normal, &[origin, normal], 100.0).unwrap();
        let clip = clip_plane(&plane, &Xform::identity()).unwrap();
        let camera = view(
            [anchor[0] + 400.0, anchor[1] - 900.0, anchor[2] + 500.0],
            anchor,
            anchor,
            true,
        );
        let u = uniform_for(&clip, &camera, anchor);

        for i in 0..50 {
            let t = f64::from(i);
            let p = [
                anchor[0] + (t * 7.3).sin() * 50.0,
                anchor[1] + (t * 3.1).cos() * 50.0,
                anchor[2] + (t * 1.7).sin() * 50.0,
            ];
            let local = [
                (p[0] - anchor[0]) as f32,
                (p[1] - anchor[1]) as f32,
                (p[2] - anchor[2]) as f32,
            ];
            let k = u.planes[0];
            let gpu = k[0] * local[0] + k[1] * local[1] + k[2] * local[2] + k[3];
            assert!(
                (f64::from(gpu) - clip.distance(p)).abs() < 1e-3,
                "point {i}"
            );
        }
    }

    /// The clip-space plane is zero on the plane, positive on the kept side, in both projections.
    #[test]
    fn screen_planes_vanish_on_the_plane() {
        let plane = plane_from(
            Mode::Normal,
            &[[10.0, 20.0, 30.0], [11.0, 21.0, 32.0]],
            100.0,
        )
        .unwrap();
        let clip = clip_plane(&plane, &Xform::identity()).unwrap();
        let anchor = [5.0, 5.0, 5.0];

        for perspective in [true, false] {
            let camera = view(
                [300.0, -400.0, 250.0],
                [10.0, 20.0, 30.0],
                anchor,
                perspective,
            );
            let u = uniform_for(&clip, &camera, anchor);
            let s = u.screen[0].map(f64::from);
            let ndc = |p: [f64; 3]| {
                camera.transform_point(&Point::new(
                    p[0] - anchor[0],
                    p[1] - anchor[1],
                    p[2] - anchor[2],
                ))
            };
            let value = |p: [f64; 3]| {
                let q = ndc(p);
                s[0] * q[0] + s[1] * q[1] + s[2] * q[2] + s[3]
            };
            let x = array(&plane.x_axis());
            let y = array(&plane.y_axis());

            for (a, b) in [(0.0, 0.0), (0.3, -0.7), (-0.9, 0.2)] {
                let on = [
                    10.0 + a * x[0] + b * y[0],
                    20.0 + a * x[1] + b * y[1],
                    30.0 + a * x[2] + b * y[2],
                ];
                assert!(
                    value(on).abs() < 1e-4,
                    "on the plane, perspective {perspective}"
                );
            }

            assert!(
                value([10.0, 20.0, 29.0]) > 0.0,
                "kept below the normal point"
            );
            assert!(
                value([10.0, 20.0, 31.0]) < 0.0,
                "cut toward the normal point"
            );
        }
    }

    /// The hatch coordinate over the screen is the scene distance across the lines, up to its period.
    #[test]
    fn hatch_coordinate_follows_the_plane() {
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 50.0]], 200.0).unwrap();
        let clip = clip_plane(&plane, &Xform::identity()).unwrap();
        let anchor = [30.0, -10.0, 0.0];
        let camera = view([250.0, -300.0, 400.0], [0.0, 0.0, 50.0], anchor, true);
        let inverse = camera.inverse().unwrap();
        let u = uniform_for(&clip, &camera, anchor);
        let s = u.screen[0].map(f64::from);
        // the scene point under canvas point (x, y), and the hatch coordinate there
        let point = |x: f64, y: f64| {
            let z = -(s[0] * x + s[1] * y + s[3]) / s[2];
            let p = inverse.transform_point(&Point::new(x, y, z));
            [p[0] + anchor[0], p[1] + anchor[1], p[2] + anchor[2]]
        };
        let coordinate = |x: f64, y: f64| {
            let h = [x, y, 1.0];
            let n: f64 = (0..3).map(|c| f64::from(u.hatch[0][c]) * h[c]).sum();
            let w: f64 = (0..3).map(|c| f64::from(u.hatch_w[0][c]) * h[c]).sum();
            n / w
        };
        let a = point(-0.2, 0.1);
        let b = point(0.3, -0.25);
        assert!((a[2] - 50.0).abs() < 1e-3 && (b[2] - 50.0).abs() < 1e-3);
        let across = |p: [f64; 3]| dot(clip.hatch, p);
        let expected = across(b) - across(a);
        let found = coordinate(0.3, -0.25) - coordinate(-0.2, 0.1);
        assert!((expected - found).abs() < 1e-2, "{expected} vs {found}");
    }

    /// A real millimetre camera, perspective and ortho, far from the origin: the planes still reach
    /// the shaders, and a point on the plane lands on the clip-space plane.
    #[test]
    fn millimetre_cameras_keep_their_planes() {
        use crate::camera::Camera;
        use session_rust::AABB;

        let plane = plane_from(Mode::Xy, &[[2.0e5, -1.0e5, 839.0]], 8000.0).unwrap();
        let clip = clip_plane(&plane, &Xform::identity()).unwrap();
        let mut bounds = AABB::empty();
        bounds.union_with_point(2.0e5 - 7000.0, -1.0e5 - 4000.0, -1500.0);
        bounds.union_with_point(2.0e5 + 7000.0, -1.0e5 + 4000.0, 3000.0);

        for ortho in [false, true] {
            let mut camera = Camera::new();
            camera.fit(&bounds, 1.5);

            if ortho {
                camera.toggle_projection();
            }

            let origin = camera.origin();
            let anchor = [origin[0], origin[1], origin[2]];
            let view_proj = camera.view_proj_anchored(1.5, &origin);
            let u = uniform_for(&clip, &view_proj, anchor);
            assert_eq!(u.count, 1, "ortho {ortho}: the plane reaches the shaders");
            let s = u.screen[0].map(f64::from);
            let q = view_proj.transform_point(&Point::new(
                2.0e5 + 1000.0 - anchor[0],
                -1.0e5 - 500.0 - anchor[1],
                839.0 - anchor[2],
            ));
            // clip-space distance to the plane: the f32 coefficients round at their own scale
            let value = (s[0] * q[0] + s[1] * q[1] + s[2] * q[2] + s[3])
                / dot([s[0], s[1], s[2]], [s[0], s[1], s[2]]).sqrt();
            assert!(value.abs() < 1e-6, "ortho {ortho}: {value}, plane {s:?}");
        }
    }

    /// The eye side flips as the eye crosses the plane, in perspective and in ortho.
    #[test]
    fn eye_sides_follow_the_eye() {
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 0.0]], 100.0).unwrap();
        let clip = clip_plane(&plane, &Xform::identity()).unwrap();

        for (eye, side) in [([0.0, -300.0, 200.0], -1.0), ([0.0, -300.0, -200.0], 1.0)] {
            for perspective in [true, false] {
                let camera = view(eye, [0.0; 3], [0.0; 3], perspective);
                let u = uniform_for(&clip, &camera, [0.0; 3]);
                assert_eq!(u.sides[0][0], side, "{eye:?}, perspective {perspective}");
            }
        }
    }

    /// Hatch spacing and line width follow the device scale.
    #[test]
    fn hatch_spacing_follows_the_device_scale() {
        let plane = clip_plane(
            &plane_from(Mode::Xy, &[[0.0; 3]], 10.0).unwrap(),
            &Xform::identity(),
        )
        .unwrap();
        let camera = view([0.0, -300.0, 200.0], [0.0; 3], [0.0; 3], true);
        let u = clip_uniform(
            &[plane],
            0,
            &ClipView {
                view_proj: &camera,
                anchor: [0.0; 3],
                height: 512,
                pixel_scale: 2.0,
                samples: 1,
            },
        );
        assert_eq!(
            (u.count, u.samples, u.spacing, u.width, u.outline),
            (1, 1, 16.0, 2.0, 4.0)
        );
    }

    /// A point or vector as three numbers.
    fn array<T: std::ops::Index<usize, Output = f64>>(v: &T) -> [f64; 3] {
        [v[0], v[1], v[2]]
    }
}
// --8<-- [end:clip-tests]

// --8<-- [start:21-clip-scene-tests]
#[cfg(test)]
mod editing_tests {
    use super::tests::view;

    #[cfg(not(target_arch = "wasm32"))]
    mod gpu {
        use super::view;
        use crate::app::clipping::{Mode, clip_plane, plane_from};
        use crate::app::scene::{FileDoc, Scene};
        use crate::engine::gpu::clip::ClipPlane;
        use crate::engine::gpu::{FrameInput, Gpu, Instance};
        use session_rust::{Color, Mesh, Point, Session, Xform};
        use std::rc::Rc;

        /// Canvas side, px.
        const SIZE: usize = 256;

        /// Face colors as the canvas stores them.
        const GREY: [u8; 3] = [188, 188, 188];
        const RED: [u8; 3] = [255, 0, 0];
        const BLUE: [u8; 3] = [0, 0, 255];
        const YELLOW: [u8; 3] = [255, 255, 0];

        /// A box from `lo` to `hi` in one color, faces outward; `top` false leaves its top open.
        fn block(lo: [f64; 3], hi: [f64; 3], color: Color, top: bool) -> Mesh {
            // corners as Mesh::create_box orders them: bottom then top, counterclockwise
            let corners = (0..8)
                .map(|i: usize| {
                    let x = if i % 4 == 1 || i % 4 == 2 {
                        hi[0]
                    } else {
                        lo[0]
                    };
                    let y = if i % 4 >= 2 { hi[1] } else { lo[1] };
                    let z = if i >= 4 { hi[2] } else { lo[2] };
                    Point::new(x, y, z)
                })
                .collect();
            let mut faces = vec![
                vec![0, 3, 2, 1],
                vec![0, 1, 5, 4],
                vec![2, 3, 7, 6],
                vec![0, 4, 7, 3],
                vec![1, 2, 6, 5],
            ];

            if top {
                faces.push(vec![4, 5, 6, 7]);
            }

            let mut mesh = Mesh::from_vertices_and_faces(corners, faces);
            mesh.set_objectcolor(color);
            mesh
        }

        /// The meshes as one document on a headless GPU, flat faces, no edges; None without an adapter.
        fn solids(meshes: Vec<Mesh>) -> Option<(Gpu, Scene)> {
            crate::app::clipping::verify_solids();
            let mut gpu = pollster::block_on(Gpu::new_headless(SIZE as u32, SIZE as u32)).ok()?;
            gpu.pass_mut::<super::super::Clip>().fill = 0; // existing hatch regressions explicitly choose Hatch
            gpu.view.show_grid = false;
            gpu.view.show_mesh_edges = false;
            gpu.view.show_points = false;
            gpu.view.lit = false;
            let mut session = Session::new("solids");

            for mesh in meshes {
                session.add_mesh(mesh, None);
            }

            let mut scene = Scene::new();
            scene.add_file(FileDoc {
                name: "solids".into(),
                session: Rc::new(session),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            });
            scene.upload_to(&mut gpu);
            Some((gpu, scene))
        }

        /// Cut with `planes` and find the solids they cross, as a frame of the viewer does.
        fn cut(gpu: &mut Gpu, scene: &Scene, planes: &[ClipPlane]) {
            gpu.set_clip_planes(planes);
            gpu.find_solids(|row| scene.solid_faces(row));
        }

        /// The world XY plane through height `z`, everything above cut away.
        fn cut_above(z: f64) -> ClipPlane {
            let plane = plane_from(Mode::Xy, &[[0.0, 0.0, z]], 200.0).unwrap();
            clip_plane(&plane, &Xform::identity()).unwrap()
        }

        /// The frame seen from `eye` toward `target`, and a scene point's pixel in it.
        fn frame(
            gpu: &mut Gpu,
            eye: [f64; 3],
            target: [f64; 3],
            perspective: bool,
        ) -> (FrameInput, impl Fn([f64; 3]) -> (usize, usize) + use<>) {
            let d: f64 = (0..3)
                .map(|i| (target[i] - eye[i]).powi(2))
                .sum::<f64>()
                .sqrt();
            let rebase = gpu.rebase_anchor(&Point::new(target[0], target[1], target[2]), d, 0.0);
            let anchor = [rebase.anchor[0], rebase.anchor[1], rebase.anchor[2]];
            let view_proj = view(eye, target, anchor, perspective);
            let map = view_proj.clone();
            let pixel = move |p: [f64; 3]| {
                let q = map.transform_point(&Point::new(
                    p[0] - anchor[0],
                    p[1] - anchor[1],
                    p[2] - anchor[2],
                ));
                (
                    ((q[0] * 0.5 + 0.5) * SIZE as f64) as usize,
                    ((0.5 - q[1] * 0.5) * SIZE as f64) as usize,
                )
            };
            let input = FrameInput {
                view_proj,
                clear: wgpu::Color {
                    r: 0.2,
                    g: 0.2,
                    b: 0.2,
                    a: 1.0,
                },
                now_ms: 0.0,
            };
            (input, pixel)
        }

        /// Shares of hatch ink, cap paper and `color` among the pixels within `r` px of `at`.
        fn patch(rgba: &[u8], at: (usize, usize), r: usize, color: [u8; 3]) -> [f64; 3] {
            let mut counts = [0.0; 3];
            let mut total = 0.0_f64;

            for y in at.1.saturating_sub(r)..(at.1 + r + 1).min(SIZE) {
                for x in at.0.saturating_sub(r)..(at.0 + r + 1).min(SIZE) {
                    let p = &rgba[(y * SIZE + x) * 4..][..3];
                    let luma = p.iter().map(|c| u32::from(*c)).sum::<u32>() / 3;
                    let grey = p.iter().max().unwrap() - p.iter().min().unwrap() < 30;
                    total += 1.0;
                    // sRGB: a line pixel of 0.75 coverage reads 137
                    counts[0] += f64::from(u8::from(grey && luma < 170));
                    counts[1] += f64::from(u8::from(p.iter().all(|c| *c > 225)));
                    counts[2] += f64::from(u8::from(
                        (0..3).all(|k| (i32::from(p[k]) - i32::from(color[k])).abs() < 30),
                    ));
                }
            }

            counts.map(|count| count / total.max(1.0))
        }

        /// True for black lines on white paper.
        fn hatched(shares: [f64; 3]) -> bool {
            shares[0] > 0.04 && shares[0] < 0.6 && shares[1] > 0.3
        }

        /// Every sample count and projection, with the frame drawn from `eye` toward `target`.
        fn each_view(
            gpu: &mut Gpu,
            eye: [f64; 3],
            target: [f64; 3],
            projections: &[bool],
            check: impl Fn(&[u8], &dyn Fn([f64; 3]) -> (usize, usize), &str),
        ) {
            for samples in [1, 4] {
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(SIZE as u32, SIZE as u32);

                for &perspective in projections {
                    let (input, pixel) = frame(gpu, eye, target, perspective);
                    let rgba = gpu.render_offscreen(&input);
                    let label = format!("{samples}x, perspective {perspective}");
                    check(&rgba, &pixel, &label);
                }
            }
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn sections_default_to_light_grey_with_black_boundaries() {
            let (mut gpu, scene) = solids(vec![block([-50.0; 3], [50.0; 3], Color::blue(), true)])
                .expect("native GPU");
            gpu.pass_mut::<super::super::Clip>().fill = super::super::Clip::new(gpu.target()).fill;
            assert_eq!(gpu.pass::<super::super::Clip>().fill, 1);
            cut(&mut gpu, &scene, &[cut_above(0.0)]);
            for samples in [1, 4] {
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(SIZE as u32, SIZE as u32);
                for dpr in [1.0, 2.0] {
                    gpu.logical_size = [SIZE as f64 / dpr; 2];
                    for perspective in [false, true] {
                        let (input, pixel) =
                            frame(&mut gpu, [0.0, 0.0, 250.0], [0.0; 3], perspective);
                        let rgba = gpu.render_offscreen(&input);
                        let center = pixel([0.0; 3]);
                        for y in center.1 - 8..=center.1 + 8 {
                            for x in center.0 - 8..=center.0 + 8 {
                                let rgb = &rgba[(y * SIZE + x) * 4..][..3];
                                assert!(
                                    rgb.iter().all(|v| (202..=204).contains(v)),
                                    "solid light grey, without hatch: {rgb:?}"
                                );
                            }
                        }
                        for edge in [
                            [-50.0, 0.0, 0.0],
                            [50.0, 0.0, 0.0],
                            [0.0, -50.0, 0.0],
                            [0.0, 50.0, 0.0],
                        ] {
                            let at = pixel(edge);
                            let black = (at.1 - 4..=at.1 + 4)
                                .flat_map(|y| {
                                    (at.0 - 4..=at.0 + 4).map(move |x| (y * SIZE + x) * 4)
                                })
                                .filter(|at| rgba[*at..*at + 3].iter().all(|v| *v < 16))
                                .count();
                            assert!(
                                black >= 5,
                                "black cut edge: {edge:?}, {samples}x, DPR {dpr}, perspective {perspective}"
                            );
                        }
                        let ids = gpu.render_ids_offscreen(&input);
                        assert_eq!(
                            ids[center.1 * SIZE + center.0][0],
                            1,
                            "the cap still picks its solid"
                        );
                    }
                }
            }
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// A cut cube seen from the cut side: hatch on the section, the kept top face stays plain,
        /// and the removed half shows the section behind it.
        fn a_cut_cube_caps_its_section_and_nothing_else() {
            let Some((mut gpu, scene)) =
                solids(vec![block([0.0; 3], [100.0; 3], Color::grey(), true)])
            else {
                return;
            };
            assert_ne!(gpu.objects.row(0).unwrap().flags & Instance::FLAG_CLOSED, 0);
            let plane = plane_from(Mode::Normal, &[[50.0, 0.0, 0.0], [40.0, 0.0, 0.0]], 100.0);
            cut(
                &mut gpu,
                &scene,
                &[clip_plane(&plane.unwrap(), &Xform::identity()).unwrap()],
            );
            each_view(
                &mut gpu,
                [-200.0, 50.0, 300.0],
                [50.0, 50.0, 50.0],
                &[true, false],
                |rgba, pixel, label| {
                    let cap = patch(rgba, pixel([50.0, 50.0, 50.0]), 8, GREY);
                    assert!(hatched(cap), "{label}: the section is hatched: {cap:?}");
                    let top = patch(rgba, pixel([75.0, 50.0, 100.0]), 4, GREY);
                    assert!(
                        top[2] > 0.95,
                        "{label}: the kept top face is plain: {top:?}"
                    );
                    let removed = patch(rgba, pixel([10.0, 50.0, 100.0]), 8, GREY);
                    assert!(
                        hatched(removed),
                        "{label}: the removed half shows the section: {removed:?}"
                    );
                },
            );

            cut(&mut gpu, &scene, &[]);
            let (input, pixel) = frame(&mut gpu, [-200.0, 50.0, 300.0], [50.0, 50.0, 50.0], true);
            let whole = gpu.render_offscreen(&input);
            let top = patch(&whole, pixel([10.0, 50.0, 100.0]), 4, GREY);
            assert!(
                top[2] > 0.95,
                "no plane: the whole top face is back: {top:?}"
            );
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// A dowel through a beam, no boolean hole: both sections hatched, the dowel never shows
        /// through the beam's section, and a pick names the solid under the cap.
        fn a_dowel_inside_a_beam_never_shows_through() {
            let Some((mut gpu, scene)) = solids(vec![
                block(
                    [-100.0, -50.0, -50.0],
                    [100.0, 50.0, 50.0],
                    Color::grey(),
                    true,
                ),
                block(
                    [-10.0, -150.0, -10.0],
                    [10.0, 150.0, 10.0],
                    Color::red(),
                    true,
                ),
            ]) else {
                return;
            };
            cut(&mut gpu, &scene, &[cut_above(0.0)]);
            let eye = [150.0, -250.0, 250.0];
            each_view(
                &mut gpu,
                eye,
                [0.0; 3],
                &[true, false],
                |rgba, pixel, label| {
                    // the dowel runs through the beam's middle; no red may show anywhere on the beam's cap
                    for x in [-80.0, -40.0, -5.0, 0.0, 5.0, 40.0, 80.0] {
                        for y in [-25.0, 0.0, 25.0] {
                            let shares = patch(rgba, pixel([x, y, 0.0]), 6, RED);
                            assert!(
                                shares[2] == 0.0,
                                "{label}: no dowel through the beam at {x},{y}: {shares:?}"
                            );
                            assert!(
                                hatched(shares),
                                "{label}: beam section at {x},{y}: {shares:?}"
                            );
                        }
                    }

                    let dowel = patch(rgba, pixel([0.0, 120.0, 0.0]), 6, RED);
                    assert!(
                        hatched(dowel),
                        "{label}: the dowel's own section: {dowel:?}"
                    );
                },
            );

            // picks: the beam where only it holds the cap point, the dowel outside the beam
            gpu.view.msaa_forced = Some(1);
            gpu.resize(SIZE as u32, SIZE as u32);
            let (input, pixel) = frame(&mut gpu, eye, [0.0; 3], true);
            let ids = gpu.render_ids_offscreen(&input);
            let id = |p: [f64; 3]| {
                let (x, y) = pixel(p);
                ids[y * SIZE + x]
            };
            assert_eq!(
                id([15.0, 0.0, 0.0]),
                [1, 0],
                "the beam's cap picks the beam"
            );
            assert_eq!(
                id([60.0, -30.0, 0.0]),
                [1, 0],
                "the beam's cap picks the beam"
            );
            assert_eq!(
                id([0.0, 120.0, 0.0]),
                [2, 0],
                "the dowel's cap picks the dowel"
            );
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// Two overlapping boxes: the union is hatched, the overlap too; a selected one's cap turns yellow.
        fn overlapping_boxes_are_capped_as_their_union() {
            let Some((mut gpu, scene)) = solids(vec![
                block(
                    [-100.0, -50.0, -50.0],
                    [30.0, 50.0, 50.0],
                    Color::red(),
                    true,
                ),
                block(
                    [-30.0, -50.0, -50.0],
                    [100.0, 50.0, 50.0],
                    Color::blue(),
                    true,
                ),
            ]) else {
                return;
            };
            cut(&mut gpu, &scene, &[cut_above(500.0)]);
            assert_eq!(
                gpu.pass::<super::super::Clip>().cap_planes(&gpu.view).1,
                0,
                "a plane above both boxes draws no section pass"
            );
            cut(&mut gpu, &scene, &[cut_above(0.0)]);
            assert_eq!(
                gpu.pass::<super::super::Clip>().cap_planes(&gpu.view).1,
                1,
                "the plane through both boxes does"
            );
            each_view(
                &mut gpu,
                [100.0, -250.0, 250.0],
                [0.0; 3],
                &[true, false],
                |rgba, pixel, label| {
                    for x in [-70.0, 0.0, 70.0] {
                        let red = patch(rgba, pixel([x, 0.0, 0.0]), 8, RED);
                        let blue = patch(rgba, pixel([x, 0.0, 0.0]), 8, BLUE);
                        assert!(hatched(red), "{label}: section at {x}: {red:?}");
                        assert!(
                            red[2] + blue[2] < 0.01,
                            "{label}: no face shows through at {x}"
                        );
                    }
                },
            );

            // the selected box's cap turns yellow where it holds the cap point, the overlap too
            gpu.set_selected(0, true);
            let (input, pixel) = frame(&mut gpu, [100.0, -250.0, 250.0], [0.0; 3], true);
            let rgba = gpu.render_offscreen(&input);
            let yellow = |x: f64| patch(&rgba, pixel([x, 0.0, 0.0]), 8, YELLOW)[2];
            assert!(yellow(-70.0) > 0.3, "selected alone: {}", yellow(-70.0));
            assert!(yellow(0.0) > 0.3, "selected and not: {}", yellow(0.0));
            assert_eq!(yellow(70.0), 0.0, "the other box's cap stays white");
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// An open box is cut but never capped: its inside shows.
        fn an_open_mesh_gets_no_cap() {
            let Some((mut gpu, scene)) =
                solids(vec![block([-50.0; 3], [50.0; 3], Color::grey(), false)])
            else {
                return;
            };
            assert_eq!(gpu.objects.row(0).unwrap().flags & Instance::FLAG_CLOSED, 0);
            cut(&mut gpu, &scene, &[cut_above(0.0)]);
            each_view(
                &mut gpu,
                [100.0, -150.0, 250.0],
                [0.0; 3],
                &[true, false],
                |rgba, pixel, label| {
                    let inside = patch(rgba, pixel([0.0, 0.0, 0.0]), 8, GREY);
                    assert!(
                        !hatched(inside) && inside[0] < 0.01,
                        "{label}: no cap: {inside:?}"
                    );
                },
            );
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// Seen from the kept side the kept faces hide the cap; from inside the solid it shows.
        fn from_the_kept_side_the_cap_hides_behind_kept_faces() {
            let Some((mut gpu, scene)) =
                solids(vec![block([-50.0; 3], [50.0; 3], Color::grey(), true)])
            else {
                return;
            };
            cut(&mut gpu, &scene, &[cut_above(0.0)]);
            each_view(
                &mut gpu,
                [100.0, -150.0, -250.0],
                [0.0; 3],
                &[true, false],
                |rgba, pixel, label| {
                    let bottom = patch(rgba, pixel([0.0, 0.0, -50.0]), 8, GREY);
                    assert!(
                        bottom[2] > 0.95,
                        "{label}: the kept bottom face: {bottom:?}"
                    );
                },
            );
            each_view(
                &mut gpu,
                [10.0, 10.0, -30.0],
                [0.0, 0.0, 10.0],
                &[true],
                |rgba, _pixel, label| {
                    let center = patch(rgba, (SIZE / 2, SIZE / 2), 8, GREY);
                    assert!(
                        hatched(center),
                        "{label}: inside the solid the cap shows: {center:?}"
                    );
                },
            );
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// Lines, dots and arrows lose the part a plane cuts away, in color and in the pick.
        fn ink_lanes_lose_the_cut_part() {
            use crate::engine::gpu::vectors::{VectorRow, VectorRows};
            use crate::engine::gpu::{CylinderSegment, GlyphPoint, ObjectRow, Upload};
            use session_rust::AABB;

            let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(SIZE as u32, SIZE as u32))
            else {
                return;
            };
            gpu.view.show_grid = false;
            let mut up = Upload::default();
            let mut bounds = AABB::empty();
            bounds.union_with_point(-100.0, -40.0, 0.0);
            bounds.union_with_point(100.0, 40.0, 0.0);

            for _ in 0..3 {
                let mut row = ObjectRow::new(Xform::identity(), 0);
                row.bounds = bounds;
                up.obj.rows.push(row);
            }

            up.bounds = bounds;
            up.seg.ribbons.push(CylinderSegment {
                p0: [-100.0, 30.0, 0.0],
                radius: 3.0,
                p1: [100.0, 30.0, 0.0],
                instance_id: 0,
                color: 0xff00_0000,
                facing: u32::MAX,
            });

            for x in [-60.0, 60.0] {
                up.glyph.dots.push(GlyphPoint {
                    center: [x, 0.0, 0.0],
                    radius: -6.0,
                    color: [0.0, 0.0, 0.0, 1.0],
                    instance_id: 1,
                    facing: u32::MAX,
                    facing_ext: [u32::MAX; 2],
                });
            }

            up.lanes.get_mut::<VectorRows>().rows.push(VectorRow {
                start: [-100.0, -30.0, 0.0],
                radius: 3.0,
                end: [100.0, -30.0, 0.0],
                instance_id: 2,
                color: 0xff00_0000,
                head: 0.0,
                heads: VectorRow::HEAD_END,
                pad: 0,
            });
            gpu.set_scene(&up);
            // ink left and right of x = 0, and pick ids right of it
            let halves = |rgba: &[u8]| {
                let mut ink = [0_i32; 2];

                for (i, p) in rgba.chunks_exact(4).enumerate() {
                    if p[..3].iter().any(|c| *c < 230) {
                        ink[usize::from(i % SIZE >= SIZE / 2)] += 1;
                    }
                }

                ink
            };
            let right = |ids: &[[u32; 2]]| {
                ids.iter()
                    .enumerate()
                    .filter(|(i, id)| i % SIZE > SIZE / 2 + 2 && id[0] != 0)
                    .count()
            };
            let yz = plane_from(Mode::Yz, &[[0.0; 3]], 200.0).unwrap();

            for samples in [1, 4] {
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(SIZE as u32, SIZE as u32);

                for perspective in [true, false] {
                    let (input, _) = frame(&mut gpu, [0.0, 0.0, 250.0], [0.0; 3], perspective);
                    gpu.set_clip_planes(&[]);
                    let whole = halves(&gpu.render_offscreen(&input));
                    let picked = right(&gpu.render_ids_offscreen(&input));
                    gpu.set_clip_planes(&[clip_plane(&yz, &Xform::identity()).unwrap()]);
                    let cut = halves(&gpu.render_offscreen(&input));
                    let label = format!("{samples}x, perspective {perspective}");
                    assert!(
                        whole[0] > 150 && whole[1] > 150,
                        "{label}: ink both sides: {whole:?}"
                    );
                    assert!(
                        (cut[0] - whole[0]).abs() <= 4,
                        "{label}: the kept half is untouched: {cut:?} {whole:?}"
                    );
                    assert!(cut[1] <= 8, "{label}: the cut half is empty: {cut:?}");
                    assert!(picked > 60, "{label}: the right half picks uncut: {picked}");
                    let picked = right(&gpu.render_ids_offscreen(&input));
                    assert_eq!(picked, 0, "{label}: nothing cut away can be picked");
                }
            }

            // cutting again and stopping again compiles nothing: both ink variants are cached
            let (input, _) = frame(&mut gpu, [0.0, 0.0, 250.0], [0.0; 3], false);
            let compiled = crate::engine::pipelines::created().0;
            gpu.set_clip_planes(&[]);
            let whole = halves(&gpu.render_offscreen(&input));
            gpu.set_clip_planes(&[clip_plane(&yz, &Xform::identity()).unwrap()]);
            let cut = halves(&gpu.render_offscreen(&input));
            assert!(whole[1] > 150 && cut[1] <= 8, "{whole:?} {cut:?}");
            assert_eq!(crate::engine::pipelines::created().0, compiled);
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// Seen along the cut, the removed half is empty: nothing to see, nothing to pick.
        fn cut_away_geometry_cannot_be_picked() {
            let Some((mut gpu, scene)) =
                solids(vec![block([0.0; 3], [100.0; 3], Color::grey(), true)])
            else {
                return;
            };
            let plane = plane_from(Mode::Normal, &[[50.0, 0.0, 0.0], [40.0, 0.0, 0.0]], 100.0);
            cut(
                &mut gpu,
                &scene,
                &[clip_plane(&plane.unwrap(), &Xform::identity()).unwrap()],
            );

            for perspective in [true, false] {
                let (input, pixel) = frame(
                    &mut gpu,
                    [50.0, -300.0, 50.0],
                    [50.0, 50.0, 50.0],
                    perspective,
                );
                let ids = gpu.render_ids_offscreen(&input);
                let id = |p: [f64; 3]| {
                    let (x, y) = pixel(p);
                    ids[y * SIZE + x]
                };
                assert_eq!(id([20.0, 0.0, 50.0])[0], 0, "cut away: nothing to pick");
                assert_eq!(id([80.0, 0.0, 50.0])[0], 1, "kept: the cube");
            }
        }

        /// Meshes and breps as one document placed at `place`, back faces red so an inside reads red.
        fn placed(
            meshes: Vec<Mesh>,
            breps: Vec<session_rust::BRep>,
            place: Xform,
        ) -> Option<(Gpu, Scene)> {
            crate::app::clipping::verify_solids();
            let mut gpu = pollster::block_on(Gpu::new_headless(SIZE as u32, SIZE as u32)).ok()?;
            gpu.pass_mut::<super::super::Clip>().fill = 0;
            gpu.view.show_grid = false;
            gpu.view.show_mesh_edges = false;
            gpu.view.show_points = false;
            gpu.view.lit = false;
            gpu.view.backface = true;
            let mut session = Session::new("placed");

            for mesh in meshes {
                session.add_mesh(mesh, None);
            }

            for brep in breps {
                session.add_brep(brep, None);
            }

            let mut scene = Scene::new();
            scene.add_file(FileDoc {
                name: "placed".into(),
                session: Rc::new(session),
                place,
                point_px: 0.0,
                display_only: false,
            });
            scene.upload_to(&mut gpu);
            Some((gpu, scene))
        }

        /// Shares of neutral (section or background), blue, red and other pixels within `r` px of `at`.
        fn tints(rgba: &[u8], at: (usize, usize), r: usize) -> [f64; 4] {
            let mut counts = [0.0; 4];
            let mut total = 0.0_f64;

            for y in at.1.saturating_sub(r)..(at.1 + r + 1).min(SIZE) {
                for x in at.0.saturating_sub(r)..(at.0 + r + 1).min(SIZE) {
                    let p = &rgba[(y * SIZE + x) * 4..][..3];
                    let spread = p.iter().max().unwrap() - p.iter().min().unwrap();
                    let class = if spread < 40 {
                        0
                    } else if p[2] > 150 && p[0] < 110 && p[1] < 110 {
                        1
                    } else if p[0] > 150 && p[1] < 120 && p[2] < 120 {
                        2
                    } else {
                        3
                    };
                    counts[class] += 1.0;
                    total += 1.0;
                }
            }

            counts.map(|count| count / total.max(1.0))
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// A plane lying on a face shows that face or the section, one of them, never a speckle.
        fn a_plane_on_a_face_shows_one_thing() {
            let lo = [13.7, -21.3, 5.9];
            let hi = [113.7, 58.7, 65.9];
            let Some((mut gpu, scene)) = placed(
                vec![block(lo, hi, Color::blue(), true)],
                vec![],
                Xform::identity(),
            ) else {
                return;
            };
            let mid = [
                (lo[0] + hi[0]) * 0.5,
                (lo[1] + hi[1]) * 0.5,
                (lo[2] + hi[2]) * 0.5,
            ];
            // the top, +x and bottom faces, each seen from the side its plane cuts away
            let cases = [
                (
                    plane_from(Mode::Xy, &[[0.0, 0.0, hi[2]]], 300.0),
                    [mid[0], mid[1], hi[2]],
                    [mid[0] - 150.0, mid[1] - 200.0, hi[2] + 250.0],
                ),
                (
                    plane_from(Mode::Yz, &[[hi[0], 0.0, 0.0]], 300.0),
                    [hi[0], mid[1], mid[2]],
                    [hi[0] + 250.0, mid[1] - 200.0, mid[2] + 150.0],
                ),
                (
                    plane_from(
                        Mode::Normal,
                        &[[0.0, 0.0, lo[2]], [0.0, 0.0, lo[2] - 10.0]],
                        300.0,
                    ),
                    [mid[0], mid[1], lo[2]],
                    [mid[0] - 150.0, mid[1] - 200.0, lo[2] - 250.0],
                ),
            ];

            for (plane, face, eye) in cases {
                let plane = clip_plane(&plane.unwrap(), &Xform::identity()).unwrap();
                cut(&mut gpu, &scene, &[plane]);
                each_view(&mut gpu, eye, mid, &[true, false], |rgba, pixel, label| {
                    let shares = tints(rgba, pixel(face), 12);
                    let section = shares[0] > 0.99 && hatched(patch(rgba, pixel(face), 12, BLUE));
                    assert!(
                        section || shares[1] > 0.99,
                        "{label}: the face or its section at {face:?}, not both: {shares:?}"
                    );
                });
            }
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// Two boxes stacked and cut where they meet: one clean section, no speckle of either face.
        fn stacked_boxes_cut_where_they_meet() {
            let Some((mut gpu, scene)) = placed(
                vec![
                    block([0.0; 3], [100.0, 100.0, 50.0], Color::blue(), true),
                    block([0.0, 0.0, 50.0], [100.0; 3], Color::red(), true),
                ],
                vec![],
                Xform::identity(),
            ) else {
                return;
            };
            cut(&mut gpu, &scene, &[cut_above(50.0)]);
            each_view(
                &mut gpu,
                [-80.0, -150.0, 300.0],
                [40.0, 40.0, 40.0],
                &[true, false],
                |rgba, pixel, label| {
                    let at = pixel([50.0, 50.0, 50.0]);
                    let shares = tints(rgba, at, 12);
                    let section = shares[0] > 0.99 && hatched(patch(rgba, at, 12, BLUE));
                    assert!(
                        section || shares[1] > 0.99 || shares[2] > 0.99,
                        "{label}: one clean surface where the boxes meet: {shares:?}"
                    );
                },
            );
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// A mirrored placement and inward faces flip the crossings, not the section.
        fn mirrored_and_inward_solids_cap_like_any_other() {
            let outward = block([-50.0; 3], [50.0; 3], Color::blue(), true);
            let mut inward = block([-50.0; 3], [50.0; 3], Color::blue(), true);

            for corners in inward.face.values_mut() {
                corners.reverse();
            }

            for (mesh, place, name) in [
                (outward, Xform::scale_xyz(-1.0, 1.0, 1.0), "mirrored"),
                (inward, Xform::identity(), "inward"),
            ] {
                let Some((mut gpu, scene)) = placed(vec![mesh], vec![], place) else {
                    return;
                };
                assert_ne!(gpu.objects.row(0).unwrap().flags & Instance::FLAG_CLOSED, 0);
                cut(&mut gpu, &scene, &[cut_above(0.0)]);
                each_view(
                    &mut gpu,
                    [100.0, -150.0, 250.0],
                    [0.0; 3],
                    &[true, false],
                    |rgba, pixel, label| {
                        let shares = tints(rgba, pixel([0.0; 3]), 12);
                        let cap = patch(rgba, pixel([0.0; 3]), 12, BLUE);
                        assert!(
                            hatched(cap) && shares[0] > 0.99,
                            "{name}, {label}: a clean section: {shares:?} {cap:?}"
                        );
                    },
                );
            }
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// Curved breps with seams and poles, and a block with a hole, cap without leaks.
        fn brep_sections_are_watertight() {
            use session_rust::BRep;

            let solids: [(BRep, [f64; 3], [f64; 3], [f64; 3]); 4] = [
                (
                    BRep::create_sphere(50.0),
                    [0.0; 3],
                    [0.0; 3],
                    [120.0, -160.0, 250.0],
                ),
                (
                    BRep::create_torus(50.0, 20.0),
                    [50.0, 0.0, 0.0],
                    [0.0; 3],
                    [120.0, -160.0, 250.0],
                ),
                (
                    BRep::create_block_with_hole(120.0, 100.0, 80.0, 25.0),
                    [-42.0, 0.0, 0.0],
                    [0.0; 3],
                    [60.0, -90.0, 320.0],
                ),
                (
                    BRep::create_cylinder(40.0, 100.0),
                    [0.0, 0.0, 50.0],
                    [0.0; 3],
                    [-220.0, 0.5, 120.0],
                ),
            ];

            for (mut brep, inside, hole, eye) in solids {
                brep.surfacecolor = Color::blue();
                let name = brep.name.clone();
                let Some((mut gpu, scene)) = placed(vec![], vec![brep], Xform::identity()) else {
                    return;
                };
                assert_ne!(
                    gpu.objects.row(0).unwrap().flags & Instance::FLAG_CLOSED,
                    0,
                    "{name} is closed"
                );
                // the cylinder stands on z = 0: cut through its axis, the rays leaving along its seam
                let plane = if name == "cylinder" {
                    let p = plane_from(Mode::Normal, &[[0.0; 3], [-10.0, 0.0, 0.0]], 300.0);
                    clip_plane(&p.unwrap(), &Xform::identity()).unwrap()
                } else {
                    cut_above(0.0)
                };
                cut(&mut gpu, &scene, &[plane]);
                each_view(
                    &mut gpu,
                    eye,
                    inside,
                    &[true, false],
                    |rgba, pixel, label| {
                        let shares = tints(rgba, pixel(inside), 6);
                        let cap = patch(rgba, pixel(inside), 6, BLUE);
                        assert!(
                            hatched(cap) && shares[0] > 0.99,
                            "{name}, {label}: a clean section at {inside:?}: {shares:?} {cap:?}"
                        );

                        // the torus and the block keep their hole open
                        if name != "sphere" && name != "cylinder" {
                            let open = patch(rgba, pixel(hole), 5, BLUE);
                            assert!(
                                open[0] < 0.02,
                                "{name}, {label}: no hatch in the hole: {open:?}"
                            );
                        }
                    },
                );
            }
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// Two planes cut a corner off: both sections hatched up to the edge they share, nothing leaks.
        fn two_planes_cap_the_corner() {
            let Some((mut gpu, scene)) = placed(
                vec![block([-50.0; 3], [50.0; 3], Color::blue(), true)],
                vec![],
                Xform::identity(),
            ) else {
                return;
            };
            let yz = plane_from(Mode::Yz, &[[0.0; 3]], 300.0).unwrap();
            cut(
                &mut gpu,
                &scene,
                &[cut_above(0.0), clip_plane(&yz, &Xform::identity()).unwrap()],
            );
            each_view(
                &mut gpu,
                [200.0, -150.0, 250.0],
                [0.0; 3],
                &[true, false],
                |rgba, pixel, label| {
                    for at in [[-25.0, -10.0, 0.0], [0.0, -10.0, -25.0]] {
                        let cap = patch(rgba, pixel(at), 8, BLUE);
                        assert!(hatched(cap), "{label}: section at {at:?}: {cap:?}");
                    }

                    for at in [[-4.0, -10.0, 0.0], [0.0, -10.0, -4.0], [-4.0, 30.0, 0.0]] {
                        let shares = tints(rgba, pixel(at), 2);
                        assert!(
                            shares[0] > 0.99,
                            "{label}: nothing leaks at {at:?}: {shares:?}"
                        );
                    }
                },
            );
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// A millimetre box far from the origin: the cut and its section hold at f32.
        fn far_from_the_origin_the_section_holds() {
            let lo = [2.0e5, -1.0e5, 800.0];
            let hi = [lo[0] + 3000.0, lo[1] + 2000.0, lo[2] + 1500.0];
            let Some((mut gpu, scene)) = placed(
                vec![block(lo, hi, Color::blue(), true)],
                vec![],
                Xform::identity(),
            ) else {
                return;
            };
            let mid = [
                (lo[0] + hi[0]) * 0.5,
                (lo[1] + hi[1]) * 0.5,
                (lo[2] + hi[2]) * 0.5,
            ];
            cut(&mut gpu, &scene, &[cut_above(mid[2] + 0.3)]);
            each_view(
                &mut gpu,
                [mid[0] + 3000.0, mid[1] - 4500.0, mid[2] + 6000.0],
                mid,
                &[true, false],
                |rgba, pixel, label| {
                    let at = [mid[0], mid[1], mid[2] + 0.3];
                    let shares = tints(rgba, pixel(at), 12);
                    let cap = patch(rgba, pixel(at), 12, BLUE);
                    assert!(
                        hatched(cap) && shares[0] > 0.99,
                        "{label}: a clean section far out: {shares:?} {cap:?}"
                    );
                },
            );
        }

        #[test]
        #[ignore = "requires a native GPU adapter"]
        /// Where a dowel sits inside a beam, the section picks the dowel, the innermost solid.
        fn a_nested_section_picks_the_innermost_solid() {
            let Some((mut gpu, scene)) = placed(
                vec![
                    block(
                        [-100.0, -50.0, -50.0],
                        [100.0, 50.0, 50.0],
                        Color::blue(),
                        true,
                    ),
                    block(
                        [-10.0, -150.0, -10.0],
                        [10.0, 150.0, 10.0],
                        Color::red(),
                        true,
                    ),
                ],
                vec![],
                Xform::identity(),
            ) else {
                return;
            };
            cut(&mut gpu, &scene, &[cut_above(0.0)]);
            gpu.view.msaa_forced = Some(1);
            gpu.resize(SIZE as u32, SIZE as u32);

            for perspective in [true, false] {
                let (input, pixel) = frame(&mut gpu, [150.0, -250.0, 250.0], [0.0; 3], perspective);
                let ids = gpu.render_ids_offscreen(&input);
                let id = |p: [f64; 3]| {
                    let (x, y) = pixel(p);
                    ids[y * SIZE + x]
                };

                for at in [[0.0, 0.0, 0.0], [3.0, -30.0, 0.0], [-3.0, 30.0, 0.0]] {
                    assert_eq!(
                        id(at),
                        [2, 0],
                        "perspective {perspective}: the dowel at {at:?}"
                    );
                }

                assert_eq!(
                    id([40.0, 0.0, 0.0]),
                    [1, 0],
                    "perspective {perspective}: the beam"
                );
            }
        }

        /// A block defined once and placed three times, straight, turned and mirrored: each
        /// placed solid gets its section like the same blocks baked, and a pick through a cap
        /// names the instance.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn instanced_solids_cap_like_baked_ones() {
            use session_rust::{Geometry, InstanceRef};

            let places = [
                Xform::translation(-60.0, 0.0, 0.0),
                &Xform::translation(60.0, 0.0, 0.0) * &Xform::rotation_z(30.0, true),
                &Xform::translation(0.0, 70.0, 0.0) * &Xform::scale_xyz(-1.0, 1.0, 1.0),
            ];
            let mut source = Session::new("instanced");
            let cube = block([-25.0; 3], [25.0; 3], Color::blue(), true);
            let definition = source.add_definition(Geometry::Mesh(Rc::new(cube)));

            for place in &places {
                let instance = InstanceRef::new(&definition, Xform::identity());
                source.add_instance(instance, place.clone(), None);
            }

            let baked: Vec<Mesh> = source
                .get_geometry()
                .meshes
                .iter()
                .map(|mesh| (**mesh).clone())
                .collect();
            let Some((mut gpu, scene)) = shown(source) else {
                return;
            };
            let Some((mut want, want_scene)) = placed(baked, vec![], Xform::identity()) else {
                return;
            };
            // baking a mirror keeps the winding, so the baked mirrored block would show red backs
            gpu.view.backface = false;
            want.view.backface = false;
            cut(&mut gpu, &scene, &[cut_above(0.0)]);
            cut(&mut want, &want_scene, &[cut_above(0.0)]);
            assert_eq!(
                gpu.pass::<super::super::Clip>().placed.records.len(),
                3,
                "every instance is cut"
            );
            let centers = [[-60.0, 0.0, 0.0], [60.0, 0.0, 0.0], [0.0, 70.0, 0.0]];

            for samples in [1, 4] {
                for target in [&mut gpu, &mut want] {
                    target.view.msaa_forced = Some(samples);
                    target.resize(SIZE as u32, SIZE as u32);
                }

                for perspective in [true, false] {
                    let eye = [40.0, -200.0, 260.0];
                    let (input, pixel) = frame(&mut gpu, eye, [0.0, 20.0, 0.0], perspective);
                    let rgba = gpu.render_offscreen(&input);
                    let (input, _) = frame(&mut want, eye, [0.0, 20.0, 0.0], perspective);
                    let expected = want.render_offscreen(&input);
                    let label = format!("{samples}x, perspective {perspective}");

                    for at in centers {
                        let shares = tints(&rgba, pixel(at), 8);
                        let cap = patch(&rgba, pixel(at), 8, BLUE);
                        assert!(
                            hatched(cap) && shares[0] > 0.99,
                            "{label}: the section at {at:?}: {shares:?} {cap:?}"
                        );
                    }

                    let far = rgba
                        .chunks_exact(4)
                        .zip(expected.chunks_exact(4))
                        .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > 24))
                        .count();
                    eprintln!("{label}: {far} pixels unlike baked");

                    if let Some(dir) = std::env::var_os("INSTANCING_SHOTS") {
                        for (name, pixels) in [("instanced", &rgba), ("baked", &expected)] {
                            let mut ppm = format!("P6 {SIZE} {SIZE} 255\n").into_bytes();
                            ppm.extend(pixels.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]));
                            let file = format!("caps_{samples}_{perspective}_{name}.ppm");
                            std::fs::write(std::path::Path::new(&dir).join(file), ppm).unwrap();
                        }
                    }

                    assert!(far * 400 <= SIZE * SIZE, "{label}: {far} unlike baked");
                }
            }

            gpu.view.msaa_forced = Some(1);
            gpu.resize(SIZE as u32, SIZE as u32);
            let (input, pixel) = frame(&mut gpu, [40.0, -200.0, 260.0], [0.0, 20.0, 0.0], true);
            let ids = gpu.render_ids_offscreen(&input);

            for (at, place) in centers.iter().zip(&places) {
                let (x, y) = pixel(*at);
                let row = ids[y * SIZE + x][0].checked_sub(1).expect("a cap pick");
                let placement = scene.placement_of(row).expect("an instance row");
                let same = (0..16).all(|i| (placement.m[i] - place.m[i]).abs() < 1e-9);
                assert!(same, "the cap at {at:?} picks its instance");
            }
        }

        /// A session's instances on a headless GPU, as `placed` sets it up.
        fn shown(session: Session) -> Option<(Gpu, Scene)> {
            crate::app::clipping::verify_solids();
            let mut gpu = pollster::block_on(Gpu::new_headless(SIZE as u32, SIZE as u32)).ok()?;
            gpu.pass_mut::<super::super::Clip>().fill = 0;
            gpu.view.show_grid = false;
            gpu.view.show_mesh_edges = false;
            gpu.view.show_points = false;
            gpu.view.lit = false;
            gpu.view.backface = true;
            let mut scene = Scene::new();
            scene.add_file(FileDoc {
                name: "instanced".into(),
                session: Rc::new(session),
                place: Xform::identity(),
                point_px: 0.0,
                display_only: false,
            });
            scene.upload_to(&mut gpu);
            Some((gpu, scene))
        }
    }
}
// --8<-- [end:21-clip-scene-tests]
