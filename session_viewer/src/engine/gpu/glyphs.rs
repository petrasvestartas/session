//! The glyph lane: every vertex-sized piece of ink. Two tables of the same 64 B row - spheres
//! (mesh/BRep vertex markers, the SOLID lane, on a quad template) and
//! dots (free points, the FLAT lane, three verts per dot). `GlyphRows` is one upload.

use crate::engine::pipelines::{build, ink_module, template_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, Template, ROWS};
use super::frame::Binds;
use super::segments::InkSupport;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")), ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl"))];

/// Vertices per dot: one triangle whose incircle is the disc.
const DOT_VERTS: u32 = 3;

/// One marker or dot row, 64 B, the layout sphere.wgsl and glyph.wgsl declare.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3],
    /// 0 = the screen-constant pen; > 0 = a world-mm radius.
    pub radius: f32,
    pub color: [f32; 4],
    pub instance_id: u32,
    /// Up to SIX incident face normals as oct16 pairs, widest edge's two first;
    /// `FACING_UNKNOWN` = no adjacency / no more.
    pub facing: u32,
    pub facing_ext: [u32; 2],
    pub support_start: u32,
    pub support_count: u32,
    pub _pad: [u32; 2],
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 64);

/// One upload's glyphs: the solid lane's vertex markers and the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    pub spheres: Vec<GlyphPoint>,
    pub dots: Vec<GlyphPoint>,
    pub supports: Vec<InkSupport>,
}

impl GlyphRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.dots);
        drop_rows(&mut self.supports);
    }
}

/// One glyph table on the GPU with the group 3 that binds it.
struct GlyphTable {
    label: &'static str,
    buf: GrowBuf,
    group: wgpu::BindGroup,
}

impl GlyphTable {
    /// A one-row table sharing the lane's exact support identities.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str, supports: &GrowBuf) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<GlyphPoint>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, label, &[&buf.buf, &supports.buf]);
        Self { label, buf, group }
    }

    /// Rebind both tables after either backing buffer changes.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts, supports: &GrowBuf) {
        self.group = bind_group(ctx, &l.ink_rows, self.label, &[&self.buf.buf, &supports.buf]);
    }

}

/// The two shader modules the lane's pipelines are built from.
struct GlyphShaders {
    sphere: wgpu::ShaderModule,
    dot: wgpu::ShaderModule,
}

/// The pipelines over the two tables.
struct GlyphPipelines {
    sphere: wgpu::RenderPipeline,
    dot: wgpu::RenderPipeline,
    id_sphere: wgpu::RenderPipeline,
    id_dot: wgpu::RenderPipeline,
}

/// The glyph lane on the GPU: two tables, the marker quad, the shaders, the pipelines.
pub struct GlyphLane {
    spheres: GlyphTable,
    dots: GlyphTable,
    supports: GrowBuf,
    template: Template,
    shaders: GlyphShaders,
    gpu: GlyphPipelines,
}

impl GlyphLane {
    /// Two one-row tables, the marker quad, both shaders and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let (q_v, q_i) = unit_quad();
        let template = Template::new(ctx, "quad.template", &q_v, &q_i);
        let shaders = GlyphShaders {
            sphere: ink_module(&ctx.device, "sphere.shader", include_str!("../../shaders/sphere.wgsl")),
            dot: ink_module(&ctx.device, "glyph.shader", include_str!("../../shaders/glyph.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);

        let supports = GrowBuf::new(ctx, "glyphs.supports", std::mem::size_of::<InkSupport>() as u64, ROWS);
        let spheres = GlyphTable::new(ctx, l, "spheres", &supports);
        let dots = GlyphTable::new(ctx, l, "dots", &supports);
        Self { spheres, dots, supports, template, shaders, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }

    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        let base = self.supports.len();
        let supports_grew = self.supports.append(ctx, &up.supports);
        let spheres = rebase_supports(&up.spheres, base);
        let dots = rebase_supports(&up.dots, base);
        let spheres_grew = self.spheres.buf.append(ctx, &spheres);
        let dots_grew = self.dots.buf.append(ctx, &dots);
        if supports_grew || spheres_grew {
            self.spheres.rebind(ctx, l, &self.supports);
        }
        if supports_grew || dots_grew {
            self.dots.rebind(ctx, l, &self.supports);
        }
    }

    /// Vertex markers draw after mesh edges so their complete footprint remains on top.
    pub fn draw_spheres(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_markers(pass, b, &self.gpu.sphere)
    }

    /// The flat lane's colour pass: SDF dots, three verts each, no template.
    pub fn draw_dots(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.dot)
    }

    /// The id pass for the markers: the template, opaque.
    pub fn draw_sphere_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_markers(pass, b, &self.gpu.id_sphere)
    }

    /// The id pass for the dots: triangles, opaque.
    pub fn draw_dot_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.id_dot)
    }

    /// The marker table on the quad template through `pipeline`; 0 draws when empty.
    fn draw_markers(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline) -> u32 {
        if self.spheres.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.spheres.group, &[]);
        self.template.bind(pass);
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.spheres.buf.len());
        1
    }

    /// The dot table through `pipeline`; 0 draws when empty.
    fn draw_dot_table(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline) -> u32 {
        if self.dots.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.dots.group, &[]);
        pass.draw(0..DOT_VERTS * self.dots.buf.len(), 0..1);
        1
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.spheres.buf.reset();
        self.dots.buf.reset();
        self.supports.reset();
    }

    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.spheres.buf.release(ctx);
        self.dots.buf.release(ctx);
        self.supports.release(ctx);
        self.spheres.rebind(ctx, l, &self.supports);
        self.dots.rebind(ctx, l, &self.supports);
    }

    /// Solid-lane rows on the GPU - the MSAA policy reads it.
    pub fn sphere_count(&self) -> u32 {
        self.spheres.buf.len()
    }

    /// Flat-lane rows on the GPU.
    pub fn dot_count(&self) -> u32 {
        self.dots.buf.len()
    }
}

/// Every glyph pipeline for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &GlyphShaders, target: Target) -> GlyphPipelines {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.ink_rows];
    let template = [template_layout()];
    let marker = PipelineDesc::new(&s.sphere, &groups, &template, TriangleList).scene_samples(target.samples).depth(DepthMode::Always);
    let disc = PipelineDesc::new(&s.dot, &groups, &[], TriangleList).scene_samples(target.samples).depth(DepthMode::Always);
    let dev = &ctx.device;

    GlyphPipelines {
        sphere: build(dev, target, &marker.with("sphere", "fs_main").color(ColorWrite::Blended)),
        dot: build(dev, target, &disc.with("glyph", "fs_main").color(ColorWrite::Blended)),
        id_sphere: build(dev, Target::ID, &marker.with("sphere.id", "fs_id")),
        id_dot: build(dev, Target::ID, &disc.with("glyph.id", "fs_id")),
    }
}

/// Rebase upload-local support ranges while preserving the caller's append-only rows.
fn rebase_supports(rows: &[GlyphPoint], base: u32) -> Vec<GlyphPoint> {
    let mut rows = rows.to_vec();
    for row in &mut rows {
        row.support_start = row.support_start.checked_add(base).expect("glyph support index overflow");
    }
    rows
}

/// Camera-facing quad template for the markers; the fragment trims it to a circle.
fn unit_quad() -> (Vec<[f32; 3]>, Vec<u32>) {
    let v = vec![[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [1.0, 1.0, 0.0], [-1.0, 1.0, 0.0]];
    let idx = vec![0u32, 1, 2, 0, 2, 3];
    (v, idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// sphere.wgsl and glyph.wgsl read the same 64 B glyph row.
    #[test]
    fn glyph_point_mirror() {
        let rust = ["center", "radius", "color", "instance_id", "facing", "facing_ext", "support_start", "support_count", "_pad"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "GlyphPoint"), rust, "{name}: GlyphPoint fields");
        }
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 64);
        assert_eq!(std::mem::offset_of!(GlyphPoint, support_start), 48);
        assert_eq!(std::mem::offset_of!(GlyphPoint, support_count), 52);
        assert_eq!(std::mem::offset_of!(GlyphPoint, _pad), 56);
    }
}
