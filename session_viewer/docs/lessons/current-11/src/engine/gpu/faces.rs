use super::buffers::{GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use crate::engine::pipelines::{ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build};

/// Bit that marks a pick id as a face.
pub const FACE_TAG: u32 = 0x2000_0000;

/// Which object and face a triangle came from.
#[derive(Clone, Copy)]
pub struct FaceSource {
    pub parent: u32, // object row
    pub face: usize, // face index in that object
}

/// Solid faces: their source ids, the selected one, and the pipelines.
pub struct Faces {
    pub sources: Vec<FaceSource>, // one entry per face
    ids: GrowBuf, // face id per triangle
    selected: wgpu::Buffer, // selected face id, read by shaders
    active: Option<u32>, // selected face id, if any
    revision: u64, // bumps on every selection change
    layout: wgpu::BindGroupLayout, // shape of the face bind group
    group: Option<wgpu::BindGroup>, // the face buffers, bound
    pipes: FacePipelines, // face pipelines
}

/// The six face pipelines.
struct FacePipelines {
    physical: wgpu::RenderPipeline, // colored faces
    object_ids: wgpu::RenderPipeline, // object id per pixel
    pick: wgpu::RenderPipeline, // face id per pixel
    highlight: wgpu::RenderPipeline, // selected face in color
    mask: wgpu::RenderPipeline, // selected face into a mask
    masks: wgpu::RenderPipeline, // selected face into both masks
}

impl Faces {
    /// Create the layout, the selection buffer and the pipelines.
    pub fn new(
        ctx: &GpuCtx,
        layouts: &Layouts,
        shader: &wgpu::ShaderModule,
        target: Target,
    ) -> Self {
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
        }
    }

    /// Rebuild the pipelines for a new MSAA sample count.
    pub fn retarget(
        &mut self,
        ctx: &GpuCtx,
        layouts: &Layouts,
        shader: &wgpu::ShaderModule,
        target: Target,
    ) {
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

    /// Face behind a pick id, if it belongs to object `row`.
    pub fn source(&self, row: u32, sub: u32) -> Option<(u32, FaceSource)> {
        // top three bits say what kind of pick
        if sub & 0xe000_0000 != FACE_TAG {
            return None;
        }

        let address = sub & !FACE_TAG;
        let source = *self.sources.get(address as usize)?;
        (source.parent == row).then_some((address, source))
    }

    /// Face id of `face` on object `parent`.
    pub fn address(&self, parent: u32, face: usize) -> Option<u32> {
        self.sources
            .iter()
            .position(|source| source.parent == parent && source.face == face)
            .map(|i| i as u32)
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

    /// Draw the colored faces.
    pub fn draw_physical(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        self.draw(pass, binds, &self.pipes.physical)
    }

    /// Draw object ids.
    pub fn draw_object_ids(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        self.draw(pass, binds, &self.pipes.object_ids)
    }

    /// Draw face ids.
    pub fn draw_ids(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        self.draw(pass, binds, &self.pipes.pick)
    }

    /// Draw the selected face highlighted.
    pub fn draw_highlight(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        if self.active.is_none() {
            return 0;
        }

        self.draw(pass, binds, &self.pipes.highlight)
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

    /// Selection change count.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Draw every face with `pipeline`; returns the draw count.
    fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        binds: &Binds,
        pipeline: &wgpu::RenderPipeline,
    ) -> u32 {
        let Some(group) = &self.group else {
            return 0;
        };

        if self.ids.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        binds.set(pass);
        pass.set_bind_group(3, group, &[]);
        // three vertices per triangle, read by index in the shader
        pass.draw(0..self.ids.len() * 3, 0..1);
        1
    }

    /// Forget every face; keep the buffers.
    pub fn reset(&mut self, ctx: &GpuCtx) {
        self.select(ctx, None);
        self.ids.reset();
        self.sources.clear();
        self.group = None;
    }

    /// Forget every face and free the buffers.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset(ctx);
        self.ids.release(ctx);
        self.sources.shrink_to_fit();
    }

    /// Bytes reserved on the GPU by this struct.
    pub fn allocated_bytes(&self) -> u64 {
        self.ids.buf.size() + self.selected.size()
    }
}

/// Build the six face pipelines.
fn pipelines(
    ctx: &GpuCtx,
    layouts: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
    layout: &wgpu::BindGroupLayout,
) -> FacePipelines {
    let groups = [&layouts.mvp, &layouts.line, &layouts.instance, layout];
    // no vertex buffers: the shader reads vertices by index
    let base = PipelineDesc::new(shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .vertex("vs_face");
    let pick = build(
        &ctx.device,
        Target::ID,
        &base.with("source face IDs", "fs_id").physical(),
    );
    let highlight = build(
        &ctx.device,
        target,
        &base
            .with("selected face", "fs_face_highlight")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let mask = build(
        &ctx.device,
        Target {
            format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
            samples: target.samples,
        },
        &base
            .with("selected face mask", "fs_selection_mask")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let masks = build(
        &ctx.device,
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
        &ctx.device,
        target,
        &object_base
            .with("physical triangle", "fs_main")
            .color(ColorWrite::Blended)
            .physical(),
    );
    let object_ids = build(
        &ctx.device,
        Target::ID,
        &object_base.with("object triangle IDs", "fs_id").physical(),
    );
    FacePipelines {
        pick,
        highlight,
        mask,
        masks,
        physical,
        object_ids,
    }
}
