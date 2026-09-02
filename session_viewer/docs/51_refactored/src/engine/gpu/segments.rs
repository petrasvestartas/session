//! The segment family - every straight piece of ink. Two tables of the same 40 B row: pipes
//! (mesh/BRep edges, the SOLID lane) and ribbons (line/polyline/curve, the FLAT lane), plus
//! the unit cylinder the tube style instances. `SegRows` is one upload; `SegmentLane` the GPU.

use crate::engine::pipelines::{Layouts, Pipelines};
use super::buffers::{rows_group, GpuCtx, GrowBuf, Template};
use super::frame::Binds;

/// Sides of the unit cylinder: six is the fewest that reads as round at pen widths.
const CYL_SIDES: u32 = 6;

/// One segment row, 40 B, the layout cylinder.wgsl and ribbon.wgsl both declare.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment {
    // The two ends are FLAT f32s, not `[f32; 3]`, and that is deliberate. WGSL gives `vec3<f32>`
    // an alignment of 16, so any struct containing one is padded to a multiple of 16 - this table
    // was 48 B and could not have been 40 whatever else was packed. Scalars align to 4, so the
    // stride is the honest sum of the fields. Costs one `vec3<f32>(..)` per end in the shaders.
    pub p0: [f32; 3],   // 12 B - start point
    pub radius: f32,    // 4 B - 0.0 = screen-constant px (default); > 0 = world mm override
    pub p1: [f32; 3],   // 12 B - end point (p0..instance_id = 32 B of geometry)
    pub instance_id: u32,  // 4 B - row in instances[]: object model + flags (hide/select later)
    // Was `[f32; 4]` - 16 B carrying what is really 8-bit RGBA. Packing it paid for `facing`
    // AND took 8 B off every segment: 48 -> 40, which is 20% of the biggest table in the viewer
    // (118 MB at mesh-stress scale).
    pub color: u32,     // 4 B - RGBA8, low byte red
    // The two faces this edge belongs to, as octahedral unit normals, 16 bits each - about 1.4
    // degrees, when all that is asked of them is the SIGN of a dot product (the facing cull) and
    // a plane to hug (the flat lane's depth solve). This is what lets the shader answer "is this
    // edge facing the camera" without the depth buffer: both faces facing away means the edge is
    // hidden and must not be drawn at all. FACING_UNKNOWN = unknown, always draw (polylines,
    // drawing linework, BRep edges with no adjacency); 0 is a real value - a +Z/+Z face pair.
    pub facing: u32,    // 4 B
}                       // 40 B

// The WGSL CylinderSegment (cylinder.wgsl AND ribbon.wgsl - same table) is exactly this layout;
// the array stride is the struct's, so a drift here misreads every row.
const _: () = assert!(std::mem::size_of::<CylinderSegment>() == 40);

/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
/// switching costs one branch at the draw site and nothing in memory - which is the whole reason
/// the two lanes were built over one buffer. Easy3D ships exactly this pair
/// (`lines_cylinders_*` against `lines_plain_*_width_control`) and lets you pick.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: 12 triangles, and the radius lifts the ink off the surface it
    /// decorates so silhouette edges never lose the depth test.
    Tubes,
    /// A camera-facing quad per edge: 6 vertices, the flat lane's own shader. Cheaper, and it
    /// lies IN the surface rather than proud of it.
    Flat,
}

/// One upload's segments: the solid lane's pipes and the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    /// Solid lane: mesh/BRep edges, drawn as 3D cylinders or as ribbons with a depth prepass.
    pub pipes: Vec<CylinderSegment>,
    /// Flat lane: line/polyline/curve, drawn as camera-facing ribbons.
    pub ribbons: Vec<CylinderSegment>,
}

/// Linework lane is per GEOMETRY TYPE, not global (both stay screen-constant px):
/// SOLID (cylinder + sphere) for mesh/BRep, whose ink lies ON a surface - the tube radius lifts
///   it off that surface, so a silhouette edge cannot lose the depth test to its own face.
/// FLAT (ribbon + glyph) for line/polyline/point, which float free and have nothing to fight.
/// Routing lives in `app::scene::Scene`; one draw per lane here.
///
/// The two tables used to share one buffer, solid rows first. One buffer meant one splice point,
/// and a splice point moves whenever either half grows - so appending a file was impossible and
/// every upload rebuilt the whole table. Two buffers, same layout and same shader (each lane
/// indexes from row 0), and both grow by appending.
pub struct SegmentLane {
    pipes: GrowBuf,
    ribbons: GrowBuf,
    pipe_group: wgpu::BindGroup,
    ribbon_group: wgpu::BindGroup,
    template: Template,
}

impl SegmentLane {
    /// Two one-row tables (VERTEX-visible, read-only storage) and the unit cylinder, uploaded once.
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stride = std::mem::size_of::<CylinderSegment>() as u64;
        let pipes = GrowBuf::new(ctx, "pipes.buffer", stride, rows);
        let ribbons = GrowBuf::new(ctx, "segments.buffer", stride, rows);
        let pipe_group = rows_group(ctx, &l.segment, "pipes.bind_group", &pipes.buf);
        let ribbon_group = rows_group(ctx, &l.segment, "segments.bind_group", &ribbons.buf);
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let template = Template::new(ctx, "cyl.template", &cyl_v, &cyl_i);

        Self { pipes, ribbons, pipe_group, ribbon_group, template }
    }

    /// Append one file's rows (a DELTA); a bind group is rebuilt only when its buffer grew.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        if self.pipes.append(ctx, &up.pipes) {
            self.pipe_group = rows_group(ctx, &l.segment, "pipes.bind_group", &self.pipes.buf);
        }
        if self.ribbons.append(ctx, &up.ribbons) {
            self.ribbon_group = rows_group(ctx, &l.segment, "segments.bind_group", &self.ribbons.buf);
        }
    }

    /// The solid lane: mesh/BRep edges as real cylinders (the tube radius lifts the ink off the
    /// surface it sits on, so silhouette edges never lose the depth test), or as flat ribbons
    /// with a depth prepass. Tubes = 1 draw, Flat = prepass + colour = 2.
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds, style: LineStyle) -> u32 {
        if self.pipes.is_empty() {
            return 0;
        }

        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.pipe_group, &[]);
        match style {
            LineStyle::Tubes => {
                pass.set_pipeline(&p.cylinder);
                pass.set_vertex_buffer(0, self.template.vbo.slice(..));
                pass.set_index_buffer(self.template.ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.template.index_count, 0, 0..self.pipes.len()); // one template, N edges
                1
            }
            // The flat lane's own shader over the SOLID table. DEPTH PREPASS
            // first (binary at half coverage): the blended colour pass writes no depth,
            // so its AA feather can never depth-reject a later stroke's opaque core -
            // that rejection read as pale flecks inside the bunny's wireframe.
            LineStyle::Flat => {
                pass.set_pipeline(&p.ribbon_solid_depth);
                pass.draw(0..4, 0..self.pipes.len());
                pass.set_pipeline(&p.ribbon_solid);
                pass.draw(0..4, 0..self.pipes.len());
                2
            }
        }
    }

    /// The flat lane's colour pass: line/polyline/curve ribbons, blended, depth read-only.
    pub fn draw_ribbons(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.ribbons.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.ribbon);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.ribbon_group, &[]);
        // instance_index IS the row: this table holds nothing but flat-lane segments
        pass.draw(0..4, 0..self.ribbons.len());
        1
    }

    /// The flat lane's depth prepass (`INK_DEPTH_PREPASS`): the same ribbons, depth only.
    pub fn draw_ribbon_depth(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.ribbons.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.ribbon_depth);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.ribbon_group, &[]);
        pass.draw(0..4, 0..self.ribbons.len());
        1
    }

    /// Forget every row; the buffers keep their capacity.
    pub fn reset(&mut self) {
        self.pipes.reset();
        self.ribbons.reset();
    }

    /// Hand both buffers back (one-row tables again) and re-point the groups at them.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.pipes.release(ctx);
        self.ribbons.release(ctx);
        self.pipe_group = rows_group(ctx, &l.segment, "pipes.bind_group", &self.pipes.buf);
        self.ribbon_group = rows_group(ctx, &l.segment, "segments.bind_group", &self.ribbons.buf);
    }

    /// Solid-lane rows on the GPU - the MSAA test reads it.
    pub fn pipe_count(&self) -> u32 {
        self.pipes.len()
    }

    /// Flat-lane rows on the GPU.
    pub fn ribbon_count(&self) -> u32 {
        self.ribbons.len()
    }
}

/// Unit-cylinder template mesh (positions + indices) along +Z, radius 1, z in [0,1], with cap fans.
/// The shader rescales xy by the screen-constant radius and maps z along (p1-p0), so it's registered ONCE.
fn unit_cylinder(sides: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut idx: Vec<u32> = Vec::new();
    for s in 0..sides{
        let a = s as f32 / sides as f32 * std::f32::consts::TAU;
        v.push([a.cos(), a.sin(), 0.0]);
        v.push([a.cos(), a.sin(), 1.0]);
    }
    for s in 0..sides{
        let b0 = 2 * s;
        let b1 = 2 * ((s+1) % sides);
        idx.extend_from_slice(&[b0, b1, b1 + 1, b0, b1+1, b0+1]); // Two triangles per side face
    }
    let cb = v.len() as u32;
    v.push([0.0, 0.0, 0.0]);
    let ct = v.len() as u32;
    v.push([0.0, 0.0, 1.0]);
    for s in 0..sides{
        let b0 = 2 * s;
        let b1 = 2 * ((s+1)%sides);
        idx.extend_from_slice(&[cb, b1, b0, ct, b0 + 1, b1 + 1]); // bottom + top fan
    }
    (v, idx)
}
