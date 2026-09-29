use super::buffers::resource_group;
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
use crate::engine::pipelines::bindings::{buffer_entry, texture_entry};
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Target, build, instance_id_layout,
    layout, scene_module, vertex_layout,
};
use session_rust::{AABB, Xform};
use std::ops::Range;
use wgpu::PrimitiveTopology::TriangleList;

use wgpu::{BufferBindingType, ShaderStages, TextureSampleType};
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
        let group = resource_group(
            ctx,
            &pipes.counts,
            "clip.counts",
            [
                (0, wgpu::BindingResource::TextureView(&texture)),
            ],
        );
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

        let group = resource_group(
            ctx,
            &pipes.primitives,
            "clip.primitives",
            [
                (1, wgpu::BindingResource::TextureView(&targets.gradient)),
            ],
        );
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
            let group = resource_group(
                ctx,
                &pipes.layout,
                "clip.pick",
                [
                    (5, buffer.as_entire_binding()),
                ],
            );
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

mod pipelines;
use pipelines::*;



#[cfg(test)]
mod tests;
