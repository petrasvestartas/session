//! The glyph lane: every vertex-sized piece of ink. Two tables of the same 48 B row - spheres
//! (mesh/BRep vertex markers, the SOLID lane, on a quad template) and
//! dots (free points, the FLAT lane, three verts per dot). `GlyphRows` is one upload.

use super::buffers::{GpuCtx, GrowBuf, ROWS, Template, bind_group};
use super::frame::Binds;
use super::upload::drop_rows;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, ink_module, template_layout,
};
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
    ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
];

/// Vertices per dot: one triangle whose incircle is the disc.
const DOT_VERTS: u32 = 3;

/// One marker or dot row, 48 B, the layout sphere.wgsl and glyph.wgsl declare. Offsets:
/// center 0 (vec3, 16-aligned), radius 12, color 16, instance_id 32, facing 36, facing_ext 40.
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
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

/// One upload's glyphs: the solid lane's vertex markers and the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    pub spheres: Vec<GlyphPoint>,
    pub dots: Vec<GlyphPoint>,
}

impl GlyphRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.dots);
    }
}

/// One glyph table on the GPU with the group 3 that binds it.
struct GlyphTable {
    label: &'static str,
    buf: GrowBuf,
    group: wgpu::BindGroup,
}

impl GlyphTable {
    /// A one-row table and its bind group.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<GlyphPoint>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Rebind after the backing buffer changed.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.group = bind_group(ctx, &l.ink_rows, self.label, &[&self.buf.buf]);
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
    source_dot: wgpu::RenderPipeline,
}

/// The glyph lane on the GPU: two tables, the marker quad, the shaders, the pipelines.
pub struct GlyphLane {
    spheres: GlyphTable,
    dots: GlyphTable,
    template: Template,
    shaders: GlyphShaders,
    gpu: GlyphPipelines,
}

impl GlyphLane {
    /// Application-owned buffer allocation capacity in bytes; excludes driver overhead.
    pub fn allocated_bytes(&self) -> u64 {
        self.spheres.buf.buf.size()
            + self.dots.buf.buf.size()
            + self.template.vbo.size()
            + self.template.ibo.size()
    }

    /// Two one-row tables, the marker quad, both shaders and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let (q_v, q_i) = unit_quad();
        let template = Template::new(ctx, "quad.template", &q_v, &q_i);
        let shaders = GlyphShaders {
            sphere: ink_module(
                &ctx.device,
                "sphere.shader",
                include_str!("../../shaders/sphere.wgsl"),
            ),
            dot: ink_module(
                &ctx.device,
                "glyph.shader",
                include_str!("../../shaders/glyph.wgsl"),
            ),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);
        let spheres = GlyphTable::new(ctx, l, "spheres");
        let dots = GlyphTable::new(ctx, l, "dots");
        Self {
            spheres,
            dots,
            template,
            shaders,
            gpu,
        }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }

    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        if self.spheres.buf.append(ctx, &up.spheres) {
            self.spheres.rebind(ctx, l);
        }
        if self.dots.buf.append(ctx, &up.dots) {
            self.dots.rebind(ctx, l);
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

    /// Opaque source candidates accumulate physical depth across bounded query pages.
    pub fn draw_source_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.source_dot)
    }

    /// The marker table on the quad template through `pipeline`; 0 draws when empty.
    fn draw_markers(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &wgpu::RenderPipeline,
    ) -> u32 {
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
    fn draw_dot_table(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &wgpu::RenderPipeline,
    ) -> u32 {
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
    }

    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.spheres.buf.release(ctx);
        self.dots.buf.release(ctx);
        self.spheres.rebind(ctx, l);
        self.dots.rebind(ctx, l);
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
    let marker = PipelineDesc::new(&s.sphere, &groups, &template, TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    let disc = PipelineDesc::new(&s.dot, &groups, &[], TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    let dev = &ctx.device;

    GlyphPipelines {
        sphere: build(
            dev,
            target,
            &marker.with("sphere", "fs_main").color(ColorWrite::Blended),
        ),
        dot: build(
            dev,
            target,
            &disc.with("glyph", "fs_main").color(ColorWrite::Blended),
        ),
        id_sphere: build(
            dev,
            Target::ID,
            &marker.with("sphere.id", "fs_id").scene_samples(1),
        ),
        id_dot: build(
            dev,
            Target::ID,
            &disc.with("glyph.id", "fs_id").scene_samples(1),
        ),
        source_dot: build(
            dev,
            Target::ID,
            &disc
                .with("glyph.source", "fs_source_id")
                .vertex("vs_source")
                .scene_samples(1)
                .depth(DepthMode::OpaqueEqual),
        ),
    }
}

/// Camera-facing quad template for the markers; the fragment trims it to a circle.
fn unit_quad() -> (Vec<[f32; 3]>, Vec<u32>) {
    let v = vec![
        [-1.0, -1.0, 0.0],
        [1.0, -1.0, 0.0],
        [1.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0],
    ];
    let idx = vec![0u32, 1, 2, 0, 2, 3];
    (v, idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// sphere.wgsl and glyph.wgsl read the same 48 B glyph row.
    #[test]
    fn glyph_point_mirror() {
        let rust = [
            "center",
            "radius",
            "color",
            "instance_id",
            "facing",
            "facing_ext",
        ];
        for (name, src) in SHADERS {
            assert_eq!(
                wgsl_fields(src, "GlyphPoint"),
                rust,
                "{name}: GlyphPoint fields"
            );
        }
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 48);
        assert_eq!(std::mem::offset_of!(GlyphPoint, facing_ext), 40);
    }
}
