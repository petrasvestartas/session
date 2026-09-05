//! Cull the existing angular band by degenerating complete triangles before rasterization.
//! Physical faces and their ID pass share this output. Source indices are immutable.

use super::buffers::{bind_group, uniform_buffer, GpuCtx, GrowBuf, INDICES};
use super::frame::FrameUniforms;
use super::objects::InstanceTable;
use crate::engine::pipelines::module;

#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("face_filter.wgsl", include_str!("../../shaders/face_filter.wgsl"))];

const WORKGROUP_SIZE: u32 = 64;

/// Explicit live index count and 2D dispatch pitch. WGSL offsets 0/4/8; size 16.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct FaceFilterParams {
    pub index_count: u32,
    pub row_width: u32,
    pub _pad: [u32; 2],
}

const _: () = {
    assert!(std::mem::size_of::<FaceFilterParams>() == 16);
    assert!(std::mem::offset_of!(FaceFilterParams, row_width) == 4);
    assert!(std::mem::offset_of!(FaceFilterParams, _pad) == 8);
};

/// The original tables and the exact uniforms/translations used by the following face draw.
pub(super) struct FaceFilterInputs<'a> {
    pub frame: &'a FrameUniforms,
    pub objects: &'a InstanceTable,
    pub planes: &'a wgpu::Buffer,
    pub source: &'a GrowBuf,
    pub vertex_faces: &'a wgpu::Buffer,
}

/// Every camera value the predicate reads, plus the exact f64 anchor that produced the
/// translation buffer. Source edits invalidate this key independently through append/reset.
#[derive(Clone, Copy, PartialEq, Eq)]
struct FilterKey {
    mvp: [u32; 16],
    eye: [u32; 3],
    ortho: u32,
    origin: Option<[u64; 3]>,
}

/// Separate compute-writable index output under the existing append/growth policy.
pub(super) struct FaceFilter {
    indices: GrowBuf,
    params: wgpu::Buffer,
    layout: wgpu::BindGroupLayout,
    pipeline: wgpu::ComputePipeline,
    previous: Option<FilterKey>,
}

impl FaceFilter {
    /// Build the portable compute pipeline and an initially empty filtered index table.
    pub fn new(ctx: &GpuCtx) -> Self {
        let entries: Vec<_> = (0..8).map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: if matches!(binding, 0 | 1 | 7) { wgpu::BufferBindingType::Uniform }
                    else { wgpu::BufferBindingType::Storage { read_only: binding != 6 } },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }).collect();
        let layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("face.filter.layout"), entries: &entries,
        });
        let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("face.filter.pipeline.layout"), bind_group_layouts: &[Some(&layout)], immediate_size: 0,
        });
        let shader = module(&ctx.device, "face.filter.shader", include_str!("../../shaders/face_filter.wgsl"));
        let pipeline = ctx.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("face.filter"), layout: Some(&pipeline_layout), module: &shader,
            entry_point: Some("cs_main"), compilation_options: Default::default(), cache: None,
        });
        Self {
            indices: GrowBuf::new(ctx, "arena.filtered.ibo", 4, INDICES | wgpu::BufferUsages::STORAGE),
            params: uniform_buffer(&ctx.device, "face.filter.params", &FaceFilterParams {
                index_count: 0, row_width: 0, _pad: [0; 2],
            }),
            layout, pipeline, previous: None,
        }
    }

    /// Initial values establish the live prefix; the next uncached frame rewrites it.
    pub fn append(&mut self, ctx: &GpuCtx, indices: &[u32]) {
        assert!(indices.len().is_multiple_of(3), "physical triangle index count");
        self.indices.append(ctx, indices);
        self.previous = None;
    }

    /// Filtered physical index rows shared by color and picking.
    pub fn indices(&self) -> &GrowBuf { &self.indices }
    /// Forget the live prefix and the camera that produced it.
    pub fn reset(&mut self) { self.indices.reset(); self.previous = None; }
    pub fn release(&mut self, ctx: &GpuCtx) { self.indices.release(ctx); self.previous = None; }

    /// Encode once before any face/color/ID draws for this frame. Fresh bindings intentionally
    /// follow all append/release/reanchor buffer replacements without cached-handle invalidation.
    pub fn encode(&mut self, ctx: &GpuCtx, encoder: &mut wgpu::CommandEncoder, inputs: &FaceFilterInputs) {
        let count = self.indices.len();
        assert_eq!(count, inputs.source.len(), "filtered/source live index counts");
        if count == 0 { return; }
        let key = FilterKey {
            mvp: inputs.frame.mvp_f32.map(f32::to_bits),
            eye: inputs.frame.eye.map(f32::to_bits), ortho: inputs.frame.ortho_h.to_bits(),
            origin: inputs.objects.translation_origin_bits(),
        };
        if self.previous == Some(key) { return; }
        let limit = ctx.device.limits().max_compute_workgroups_per_dimension;
        let groups = (count / 3).div_ceil(WORKGROUP_SIZE);
        let width = groups.min(limit);
        let height = groups.div_ceil(width);
        assert!(height <= limit, "face filter dispatch exceeds device limit");
        let params = FaceFilterParams { index_count: count, row_width: width * WORKGROUP_SIZE, _pad: [0; 2] };
        ctx.queue.write_buffer(&self.params, 0, bytemuck::bytes_of(&params));
        let (mvp, line) = inputs.frame.face_filter_uniforms();
        let group = bind_group(ctx, &self.layout, "face.filter.bindings", &[
            mvp, line, inputs.objects.translation_buffer(), inputs.planes,
            &inputs.source.buf, inputs.vertex_faces, &self.indices.buf, &self.params,
        ]);
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("physical face angular filter"), timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(width, height, 1);
        // Frame encoders are submitted by the renderer; still frames reuse this exact output.
        self.previous = Some(key);
    }
}
