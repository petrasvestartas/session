//! The segment lane: every straight piece of ink. One table of 40 B rows - ribbons
//! (line/polyline/curve, the FLAT lane, blended camera-facing quads). `SegRows` is one upload.

use crate::engine::pipelines::{build, module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// Vertices per ribbon: two triangles pulled by vertex index, no vertex buffer.
const RIBBON_VERTS: u32 = 6;

/// One segment row, 40 B, the layout ribbon.wgsl declares. The ends are
/// flat f32s: a `vec3` would pad the row to 48 B.
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

/// One upload's segments: the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    pub ribbons: Vec<CylinderSegment>,
}

impl SegRows {
    /// Empty the table and hand the allocation back.
    pub fn drop_rows(&mut self) {
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
    /// A one-row table.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<CylinderSegment>() as u64, ROWS);
        let group = bind_group(ctx, &l.rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Append rows; the group is rebuilt only when the buffer grew.
    fn append(&mut self, ctx: &GpuCtx, l: &Layouts, rows: &[CylinderSegment]) {
        if self.buf.append(ctx, rows) {
            self.group = bind_group(ctx, &l.rows, self.label, &[&self.buf.buf]);
        }
    }

    /// Hand the buffer back and re-point the group at the one-row table.
    fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.buf.release(ctx);
        self.group = bind_group(ctx, &l.rows, self.label, &[&self.buf.buf]);
    }
}

/// The shader module the lane's pipeline is built from.
struct SegShaders {
    ribbon: wgpu::ShaderModule,
}

/// The pipeline over the table: the blended, depth-read-only quad.
struct SegPipelines {
    ribbon: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: one table, the shader, the pipeline.
pub struct SegmentLane {
    ribbons: SegTable,
    shaders: SegShaders,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// One one-row table, the shader and the pipeline.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shaders = SegShaders {
            ribbon: module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);

        Self { ribbons: SegTable::new(ctx, l, "ribbons"), shaders, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }

    /// Append one file's rows to the table.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        self.ribbons.append(ctx, l, &up.ribbons);
    }

    /// The flat lane's colour pass: line/polyline/curve ribbons, blended, depth read-only.
    pub fn draw_ribbons(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.ribbons)
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
        self.ribbons.buf.reset();
    }

    /// Hand the buffer back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.ribbons.release(ctx, l);
    }

    /// Flat-lane rows on the GPU.
    pub fn ribbon_count(&self) -> u32 {
        self.ribbons.buf.len()
    }
}

/// Every segment pipeline for `target`. `GreaterEqual` on the ribbon is load-bearing: a mesh
/// edge sits EXACTLY on its faces' depth, and strict `Greater` shreds it.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &SegShaders, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let quad = PipelineDesc::new(&s.ribbon, &groups, &[], TriangleList);
    let dev = &ctx.device;

    SegPipelines {
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
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
    }
}
