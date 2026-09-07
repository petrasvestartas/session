//! The segment lane: every straight piece of ink. Two tables of the same 40 B row - pipes
//! (mesh/BRep edges, the SOLID lane, culled by facing) and ribbons (line/polyline/curve, the
//! FLAT lane, always drawn) - through one blended camera-facing quad. `SegRows` is one upload.

use crate::engine::pipelines::{build, ink_module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// Vertices per ribbon: two triangles pulled by vertex index, no vertex buffer.
const RIBBON_VERTS: u32 = 6;

/// One segment row, 40 B, the layout ribbon.wgsl declares. The ends are flat f32s: a `vec3`
/// would pad the row to 48 B. Offsets: p0 0, radius 12, p1 16, instance_id 28, color 32,
/// facing 36.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment {
    pub p0: [f32; 3],
    /// 0 = the screen-constant pen; > 0 = a world-mm radius.
    pub radius: f32,
    pub p1: [f32; 3],
    pub instance_id: u32,
    /// RGBA8, low byte red.
    pub color: u32,
    /// Two oct16 adjacent face normals; `FACING_UNKNOWN` = no adjacency, always drawn.
    pub facing: u32,
}

const _: () = assert!(std::mem::size_of::<CylinderSegment>() == 40);

/// One upload's segments: the solid lane's pipes and the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    pub pipes: Vec<CylinderSegment>,
    pub ribbons: Vec<CylinderSegment>,
}

impl SegRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.ribbons);
    }
}

/// One segment table on the GPU with the group 3 that binds it.
struct SegTable {
    label: &'static str,
    buf: GrowBuf,
    group: wgpu::BindGroup,
}

impl SegTable {
    /// A one-row table and its bind group.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<CylinderSegment>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Rebind after the backing buffer changed.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.group = bind_group(ctx, &l.ink_rows, self.label, &[&self.buf.buf]);
    }
}

/// The pipelines over the two tables: the same blended quad for both, and its id twin.
struct SegPipelines {
    ribbon: wgpu::RenderPipeline,
    id_ribbon: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: two tables, the shader, the pipelines.
pub struct SegmentLane {
    pipes: SegTable,
    ribbons: SegTable,
    shader: wgpu::ShaderModule,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// Two one-row tables, the shader and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = ink_module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl"));
        let gpu = build_pipelines(ctx, l, &shader, target);
        let pipes = SegTable::new(ctx, l, "pipes");
        let ribbons = SegTable::new(ctx, l, "ribbons");
        Self { pipes, ribbons, shader, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shader, target);
    }

    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        if self.pipes.buf.append(ctx, &up.pipes) {
            self.pipes.rebind(ctx, l);
        }
        if self.ribbons.buf.append(ctx, &up.ribbons) {
            self.ribbons.rebind(ctx, l);
        }
    }

    /// Mesh/BRep edges: camera-facing quads against physical depth.
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.pipes)
    }

    /// The flat lane's colour pass: line/polyline/curve ribbons, blended.
    pub fn draw_ribbons(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.ribbons)
    }

    /// The id pass for the solid lane: opaque quads.
    pub fn draw_pipe_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.pipes)
    }

    /// The id pass for the flat lane: opaque quads.
    pub fn draw_ribbon_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.ribbons)
    }

    /// One table as ribbons through `pipeline`; 0 draws when empty.
    fn draw_table(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline, table: &SegTable) -> u32 {
        if table.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &table.group, &[]);
        pass.draw(0..RIBBON_VERTS * table.buf.len(), 0..1);
        1
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.pipes.buf.reset();
        self.ribbons.buf.reset();
    }

    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.pipes.buf.release(ctx);
        self.ribbons.buf.release(ctx);
        self.pipes.rebind(ctx, l);
        self.ribbons.rebind(ctx, l);
    }

    /// Solid-lane rows on the GPU - the MSAA policy reads it.
    pub fn pipe_count(&self) -> u32 {
        self.pipes.buf.len()
    }

    /// Flat-lane rows on the GPU.
    pub fn ribbon_count(&self) -> u32 {
        self.ribbons.buf.len()
    }
}

/// Both segment pipelines read physical visibility without writing or biasing that depth.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.ink_rows];
    let quad = PipelineDesc::new(shader, &groups, &[], TriangleList).scene_samples(target.samples).depth(DepthMode::Always);
    let dev = &ctx.device;

    SegPipelines {
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended)),
        id_ribbon: build(dev, Target::ID, &quad.with("ribbon.id", "fs_id")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// ribbon.wgsl reads the 40 B segment row (ends as scalars).
    #[test]
    fn cylinder_segment_mirror() {
        let rust = ["p0x", "p0y", "p0z", "radius", "p1x", "p1y", "p1z", "instance_id", "color", "facing"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "CylinderSegment"), rust, "{name}: CylinderSegment fields");
        }
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
        assert_eq!(std::mem::offset_of!(CylinderSegment, facing), 36);
    }
}
