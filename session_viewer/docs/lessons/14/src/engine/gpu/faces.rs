use super::buffers::{GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::slots::{Slots, clamp, slot_layout};
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build,
};

pub use super::arena::{FACE_TAG, FaceSource};

/// Solid faces: their source ids, the selected one, and the pipelines.
pub struct Faces {
    pub sources: Vec<FaceSource>,   // one entry per face
    ids: GrowBuf,                   // face id per triangle
    selected: wgpu::Buffer,         // selected face id, read by shaders
    active: Option<u32>,            // selected face id, if any
    revision: u64,                  // bumps on every selection change
    layout: wgpu::BindGroupLayout,  // shape of the face bind group
    group: Option<wgpu::BindGroup>, // the face buffers, bound
    pipes: FacePipelines,           // face pipelines
    pub slots: Slots,               // instance slots and the per-definition draws
}

/// The face pipelines.
struct FacePipelines {
    physical: Pipeline,       // colored faces, blended for glass
    opaque: Pipeline,         // colored faces at full opacity, no blending
    clipped: Pipeline,        // colored faces cut by clipping planes, blended
    clipped_opaque: Pipeline, // the same at full opacity
    object_ids: Pipeline,     // object id per pixel
    pick: Pipeline,           // face id per pixel
    highlight: Pipeline,      // selected face in color
    mask: Pipeline,           // selected face into a mask
    masks: Pipeline,          // selected face into both masks
}

impl Faces {
    /// Create the layout, the selection buffer and the pipelines.
    pub fn new(ctx: &GpuCtx, layouts: &Layouts, shader: &Shader, target: Target) -> Self {
        // bindings 0-3 are storage buffers, 4 is the selection
        let entries: Vec<_> = (0..5)
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: if binding == 4 {
                        wgpu::BufferBindingType::Uniform
                    } else {
                        wgpu::BufferBindingType::Storage { read_only: true }
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("source faces"),
                entries: &entries,
            });
        // 16 bytes: selected face id plus padding
        let selected = super::buffers::zeroed_buffer(
            &ctx.device,
            "selected source face",
            16,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );
        let pipes = pipelines(ctx, layouts, shader, target, &layout);
        Self {
            sources: Vec::new(),
            ids: GrowBuf::new(ctx, "source face ids", 4, ROWS),
            selected,
            active: None,
            revision: 0,
            layout,
            group: None,
            pipes,
            slots: Slots::new(ctx),
        }
    }

    /// Rebuild the pipelines for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, shader: &Shader, target: Target) {
        self.pipes = pipelines(ctx, layouts, shader, target, &self.layout);
    }

    /// Append one upload's faces and rebuild the bind group.
    pub fn append(
        &mut self,
        ctx: &GpuCtx,
        up: &super::arena::ArenaRows,
        buffers: [&wgpu::Buffer; 3],
    ) {
        // faces before this upload
        let base = self.sources.len() as u32;
        self.sources.extend_from_slice(&up.face_sources);
        // one face id per triangle, u32::MAX = none
        let ids: Vec<u32> = (0..up.idx.len() / 3)
            .map(|i| match up.face_ids.get(i) {
                Some(&id) if id != u32::MAX => base + id,
                _ => u32::MAX,
            })
            .collect();
        self.ids.append(ctx, &ids);
        let buffers = [
            buffers[0],
            buffers[1],
            buffers[2],
            &self.ids.buf,
            &self.selected,
        ];
        let entries: Vec<_> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: buffer.as_entire_binding(),
            })
            .collect();
        self.group = Some(ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("source faces"),
            layout: &self.layout,
            entries: &entries,
        }));
    }

    /// Overwrite one object's faces in place.
    pub(crate) fn patch(
        &mut self,
        ctx: &GpuCtx,
        at: super::patch::Counts,
        up: &super::arena::ArenaRows,
    ) {
        let first = at.sources as usize;
        self.sources[first..first + up.face_sources.len()].copy_from_slice(&up.face_sources);
        let ids: Vec<u32> = (0..up.idx.len() / 3)
            .map(|i| match up.face_ids.get(i) {
                Some(&id) if id != u32::MAX => at.sources + id,
                _ => u32::MAX,
            })
            .collect();
        self.ids.write_at(ctx, at.faces / 3, &ids);
        self.revision = self.revision.wrapping_add(1);
    }

    /// Give `count` triangles from `first` no face.
    pub(crate) fn kill_ids(&mut self, ctx: &GpuCtx, first: u32, count: u32) {
        self.ids.fill(ctx, first, count, &u32::MAX);
        self.revision = self.revision.wrapping_add(1);
    }

    /// Detach `count` source faces from `first`, so no pick or address finds them.
    pub(crate) fn kill_sources(&mut self, first: u32, count: u32) {
        let end = (first + count) as usize;

        for source in &mut self.sources[first as usize..end] {
            source.parent = u32::MAX;
        }

        self.revision = self.revision.wrapping_add(1);
    }

    /// Draw the colored faces; `opaque` skips blending, which full opacity does not need, and
    /// `clipped` cuts them sample by sample while clipping planes are active.
    pub fn draw_physical(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        opaque: bool,
        clipped: bool,
    ) -> u32 {
        let pipeline = match (opaque, clipped) {
            (true, false) => &self.pipes.opaque,
            (false, false) => &self.pipes.physical,
            (true, true) => &self.pipes.clipped_opaque,
            (false, true) => &self.pipes.clipped,
        };
        self.draw(pass, binds, pipeline)
    }

    /// Draw object ids.
    pub fn draw_object_ids(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        self.draw(pass, binds, &self.pipes.object_ids)
    }

    /// Draw face ids.
    pub fn draw_ids(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        self.draw(pass, binds, &self.pipes.pick)
    }

    /// Draw the selected face into the selection mask.
    pub fn draw_mask(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        if self.active.is_none() {
            return 0;
        }

        self.draw(pass, binds, &self.pipes.mask)
    }

    /// Draw the selected face into both masks.
    pub fn draw_masks(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        if self.active.is_none() {
            return 0;
        }

        self.draw(pass, binds, &self.pipes.masks)
    }

    /// Draw every face with `pipeline`; returns the draw count.
    fn draw(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds, pipeline: &Pipeline) -> u32 {
        let Some(group) = &self.group else {
            return 0;
        };

        if self.ids.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        binds.set(pass);
        pass.set_bind_group(3, group, &[]);
        self.slots.bind(pass, 0);
        // three vertices per triangle, read by index in the shader
        pass.draw(0..self.ids.len() * 3, 0..1);
        let mut draws = 1;

        // each definition once more per instance, its rows from the slots
        for draw in self.slots.draws() {
            let faces = clamp(&draw.faces, self.ids.len() * 3);

            if !faces.is_empty() {
                pass.draw(faces, draw.slots.clone());
                draws += 1;
            }
        }

        draws
    }

    /// Select a face; None clears the selection.
    pub fn select(&mut self, ctx: &GpuCtx, face: Option<u32>) {
        self.active = face;
        self.revision = self.revision.wrapping_add(1);
        ctx.queue.write_buffer(
            &self.selected,
            0,
            bytemuck::cast_slice(&[face.unwrap_or(u32::MAX), 0, 0, 0]),
        );
    }

    /// Forget every face; keep the buffers.
    pub fn reset(&mut self, ctx: &GpuCtx) {
        self.select(ctx, None);
        self.slots.clear_draws();
        self.ids.reset();
        self.sources.clear();
        self.group = None;
    }

    /// Forget every face and free the buffers.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset(ctx);
        self.slots.clear(ctx, true);
        self.ids.release(ctx);
        self.sources.shrink_to_fit();
    }

    /// Bytes reserved on the GPU by this struct.
    pub fn allocated_bytes(&self) -> u64 {
        self.ids.buf.size() + self.selected.size() + self.slots.allocated_bytes()
    }
}

/// Build the face pipelines.
fn pipelines(
    ctx: &GpuCtx,
    layouts: &Layouts,
    shader: &Shader,
    target: Target,
    layout: &wgpu::BindGroupLayout,
) -> FacePipelines {
    let groups = [&layouts.mvp, &layouts.line, &layouts.instance, layout];
    // the shader reads vertices by index; the one vertex buffer is the instance slots
    let slots = [slot_layout()];
    let base = PipelineDesc::new(
        shader,
        &groups,
        &slots,
        wgpu::PrimitiveTopology::TriangleList,
    )
    .vertex("vs_face");
    let pick = build(
        ctx,
        Target::ID,
        &base.with("source face IDs", "fs_id").physical(),
    );
    let highlight = build(
        ctx,
        target,
        &base
            .with("selected face", "fs_face_highlight")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let mask = build(
        ctx,
        Target {
            format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
            samples: target.samples,
        },
        &base
            .with("selected face mask", "fs_selection_mask")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let masks = build(
        ctx,
        Target {
            format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
            samples: target.samples,
        },
        &base
            .with("selected face masks", "fs_masks")
            .depth(DepthMode::ReadOnlyEqual)
            .masks(),
    );
    // same faces, plain vertex shader
    let object_base = base.vertex("vs_triangle");
    let physical = build(
        ctx,
        target,
        &object_base
            .with("physical triangle", "fs_main")
            .color(ColorWrite::Blended)
            .physical(),
    );
    let opaque = build(
        ctx,
        target,
        &object_base
            .with("physical triangle opaque", "fs_main")
            .physical(),
    );
    let object_ids = build(
        ctx,
        Target::ID,
        &object_base.with("object triangle IDs", "fs_id").physical(),
    );
    // cut by clipping planes: only compiled once a plane cuts
    let cut = object_base
        .with("clipped triangle", "fs_clipped")
        .physical();
    let clipped = build(ctx, target, &cut.clone().color(ColorWrite::Blended));
    let clipped_opaque = build(ctx, target, &cut);
    FacePipelines {
        pick,
        highlight,
        mask,
        masks,
        physical,
        opaque,
        clipped,
        clipped_opaque,
        object_ids,
    }
}
