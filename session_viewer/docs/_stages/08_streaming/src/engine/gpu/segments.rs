//! The segment lane: every straight piece of ink. Two tables of the same 40 B row - pipes
//! (mesh/BRep edges, the SOLID lane, tubes or flat quads with a depth prepass) and ribbons
//! (line/polyline/curve, the FLAT lane, blended camera-facing quads). `SegRows` is one upload.

use crate::engine::pipelines::{build, module, template_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, Template, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use super::view::LineStyle;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")), ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// Sides of the unit cylinder: six is the fewest that reads as round at pen widths.
const CYL_SIDES: u32 = 6;

/// Vertices per ribbon: two triangles pulled by vertex index, no vertex buffer.
const RIBBON_VERTS: u32 = 6;

/// One segment row, 40 B, the layout cylinder.wgsl and ribbon.wgsl declare. The ends are
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

/// The two shader modules the lane's pipelines are built from.
struct SegShaders {
    cylinder: wgpu::ShaderModule,
    ribbon: wgpu::ShaderModule,
}

/// The pipelines over the two tables. `ribbon` serves both lanes' colour pass: the same
/// blended, depth-read-only quad.
struct SegPipelines {
    cylinder: wgpu::RenderPipeline,
    ribbon: wgpu::RenderPipeline,
    ribbon_depth: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: two tables, the unit cylinder, the shaders, the pipelines.
pub struct SegmentLane {
    pipes: SegTable,
    ribbons: SegTable,
    template: Template,
    shaders: SegShaders,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// Two one-row tables, the unit cylinder, both shaders and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let template = Template::new(ctx, "cyl.template", &cyl_v, &cyl_i);
        let shaders = SegShaders {
            cylinder: module(&ctx.device, "cylinder.shader", include_str!("../../shaders/cylinder.wgsl")),
            ribbon: module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);

        Self { pipes: SegTable::new(ctx, l, "pipes"), ribbons: SegTable::new(ctx, l, "ribbons"), template, shaders, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }

    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        self.pipes.append(ctx, l, &up.pipes);
        self.ribbons.append(ctx, l, &up.ribbons);
    }

    /// The solid lane: mesh/BRep edges as tubes (1 draw) or as flat quads with a depth prepass
    /// (2 draws) - the prepass keeps the blended AA feather from depth-rejecting later strokes.
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, style: LineStyle) -> u32 {
        match style {
            LineStyle::Tubes => self.draw_tubes(pass, b, &self.gpu.cylinder),
            LineStyle::Flat => self.draw_table(pass, b, &self.gpu.ribbon_depth, &self.pipes) + self.draw_table(pass, b, &self.gpu.ribbon, &self.pipes),
        }
    }

    /// The flat lane's colour pass: line/polyline/curve ribbons, blended, depth read-only.
    pub fn draw_ribbons(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.ribbons)
    }

    /// The pipes as instanced cylinders through `pipeline`; 0 draws when empty.
    fn draw_tubes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline) -> u32 {
        if self.pipes.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.pipes.group, &[]);
        self.template.bind(pass);
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.pipes.buf.len());
        1
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
        self.pipes.release(ctx, l);
        self.ribbons.release(ctx, l);
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

/// Every segment pipeline for `target`. `GreaterEqual` on the ribbon is load-bearing: a mesh
/// edge sits EXACTLY on its faces' depth, and strict `Greater` shreds it.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &SegShaders, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let template = [template_layout()];
    let quad = PipelineDesc::new(&s.ribbon, &groups, &[], TriangleList);
    let tube = PipelineDesc::new(&s.cylinder, &groups, &template, TriangleList);
    let dev = &ctx.device;

    SegPipelines {
        cylinder: build(dev, target, &tube.with("cylinder", "fs_main")),
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
        ribbon_depth: build(dev, target, &quad.with("ribbon.depth", "fs_depth").color(ColorWrite::Masked)),
    }
}

/// Unit-cylinder template along +Z, radius 1, z in [0, 1], with cap fans. The shader rescales
/// xy by the pen radius and maps z along (p1 - p0).
fn unit_cylinder(sides: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut idx: Vec<u32> = Vec::new();
    for s in 0..sides {
        let a = s as f32 / sides as f32 * std::f32::consts::TAU;
        v.push([a.cos(), a.sin(), 0.0]);
        v.push([a.cos(), a.sin(), 1.0]);
    }
    for s in 0..sides {
        let b0 = 2 * s;
        let b1 = 2 * ((s + 1) % sides);
        idx.extend_from_slice(&[b0, b1, b1 + 1, b0, b1 + 1, b0 + 1]);
    }
    let cb = v.len() as u32;
    v.push([0.0, 0.0, 0.0]);
    let ct = v.len() as u32;
    v.push([0.0, 0.0, 1.0]);
    for s in 0..sides {
        let b0 = 2 * s;
        let b1 = 2 * ((s + 1) % sides);
        idx.extend_from_slice(&[cb, b1, b0, ct, b0 + 1, b1 + 1]);
    }
    (v, idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// cylinder.wgsl and ribbon.wgsl read the same 40 B segment row (ends as scalars).
    #[test]
    fn cylinder_segment_mirror() {
        let rust = ["p0x", "p0y", "p0z", "radius", "p1x", "p1y", "p1z", "instance_id", "color", "facing"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "CylinderSegment"), rust, "{name}: CylinderSegment fields");
        }
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
    }
}
