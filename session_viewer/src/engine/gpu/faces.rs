//! Physical face drawing and original face identities over the arena's shared triangles.
use super::buffers::{GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use crate::engine::pipelines::{DepthMode, Layouts, PipelineDesc, Target, build};

/// Sub-selection tag; source face addresses remain separate from edges and controls.
pub const FACE_TAG: u32 = 0x2000_0000;

/// One source face, shared by every display triangle belonging to it.
#[derive(Clone, Copy)]
pub struct FaceSource {
    pub parent: u32,
    pub face: usize,
}

/// Face picking pulls existing vertices by index: no duplicate mesh or per-face draw calls.
pub struct Faces {
    pub sources: Vec<FaceSource>,
    ids: GrowBuf,
    selected: wgpu::Buffer,
    active: Option<u32>,
    layout: wgpu::BindGroupLayout,
    group: Option<wgpu::BindGroup>,
    pipes: FacePipelines,
}

/// All passes share the same pulled vertices and primitive numbering.
struct FacePipelines {
    physical: wgpu::RenderPipeline,
    object_ids: wgpu::RenderPipeline,
    pick: wgpu::RenderPipeline,
    highlight: wgpu::RenderPipeline,
    mask: wgpu::RenderPipeline,
}

impl Faces {
    /// Compile source-face passes using the same transforms and physical triangle positions.
    pub fn new(
        ctx: &GpuCtx,
        layouts: &Layouts,
        shader: &wgpu::ShaderModule,
        target: Target,
    ) -> Self {
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
            layout,
            group: None,
            pipes,
        }
    }

    /// Rebuild only sample-dependent passes after the target changes.
    pub fn retarget(
        &mut self,
        ctx: &GpuCtx,
        layouts: &Layouts,
        shader: &wgpu::ShaderModule,
        target: Target,
    ) {
        self.pipes = pipelines(ctx, layouts, shader, target, &self.layout);
    }

    /// Map upload-local source addresses into the append-only scene address space.
    pub fn append(
        &mut self,
        ctx: &GpuCtx,
        up: &super::arena::ArenaRows,
        buffers: [&wgpu::Buffer; 3],
    ) {
        let base = self.sources.len() as u32;
        self.sources.extend_from_slice(&up.face_sources);
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

    /// Resolve a GPU face address back to its original parent and source key.
    pub fn source(&self, row: u32, sub: u32) -> Option<(u32, FaceSource)> {
        if sub & 0xe000_0000 != FACE_TAG {
            return None;
        }
        let address = sub & !FACE_TAG;
        let source = *self.sources.get(address as usize)?;
        (source.parent == row).then_some((address, source))
    }

    /// Switch the highlighted source face without editing geometry or normals.
    pub fn select(&mut self, ctx: &GpuCtx, face: Option<u32>) {
        self.active = face;
        ctx.queue.write_buffer(
            &self.selected,
            0,
            bytemuck::cast_slice(&[face.unwrap_or(u32::MAX), 0, 0, 0]),
        );
    }

    /// Draw opaque surfaces with the same primitive addresses used by finite visibility.
    pub fn draw_physical(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        self.draw(pass, binds, &self.pipes.physical)
    }
    /// Preserve parent-object identity while carrying physical triangle provenance.
    pub fn draw_object_ids(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        self.draw(pass, binds, &self.pipes.object_ids)
    }
    /// One source-ID draw over the shared index run.
    pub fn draw_ids(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        self.draw(pass, binds, &self.pipes.pick)
    }
    /// Color only the selected source face, against immutable physical depth.
    pub fn draw_highlight(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        if self.active.is_none() {
            return 0;
        }
        self.draw(pass, binds, &self.pipes.highlight)
    }
    /// Add just the selected face to the normal selection outline mask.
    pub fn draw_mask(&self, pass: &mut wgpu::RenderPass<'_>, binds: &Binds) -> u32 {
        if self.active.is_none() {
            return 0;
        }
        self.draw(pass, binds, &self.pipes.mask)
    }
    /// Vertex pulling uses the arena's exact position, normal, color and triangle indices.
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
        pass.draw(0..self.ids.len() * 3, 0..1);
        1
    }
    /// Forget scene identities and references before arena buffers are reused.
    pub fn reset(&mut self, ctx: &GpuCtx) {
        self.select(ctx, None);
        self.ids.reset();
        self.sources.clear();
        self.group = None;
    }
    /// Release all variable-size storage with the rest of the arena.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset(ctx);
        self.ids.release(ctx);
        self.sources.shrink_to_fit();
    }
    /// Exact owned GPU buffer capacity; the vertex/index buffers remain owned by the arena.
    pub fn allocated_bytes(&self) -> u64 {
        self.ids.buf.size() + self.selected.size()
    }
}

/// Shared shader entries retain identical clip-space arithmetic for color and ID passes.
fn pipelines(
    ctx: &GpuCtx,
    layouts: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
    layout: &wgpu::BindGroupLayout,
) -> FacePipelines {
    let groups = [&layouts.mvp, &layouts.line, &layouts.instance, layout];
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
            format: wgpu::TextureFormat::R8Unorm,
            samples: target.samples,
        },
        &base
            .with("selected face mask", "fs_selection_mask")
            .depth(DepthMode::ReadOnlyEqual),
    );
    let object_base = base.vertex("vs_triangle");
    let physical = build(
        &ctx.device,
        target,
        &object_base.with("physical triangle", "fs_main").physical(),
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
        physical,
        object_ids,
    }
}
