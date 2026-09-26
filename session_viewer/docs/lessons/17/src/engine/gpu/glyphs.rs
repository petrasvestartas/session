use super::buffers::{GpuCtx, GrowBuf, ROWS, Template, bind_group};
use super::frame::Binds;
use super::upload::drop_rows;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build, ink_module,
    template_layout,
};
use wgpu::PrimitiveTopology::TriangleList;

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("sphere.wgsl", shader!("sphere.wgsl")),
    ("glyph.wgsl", shader!("glyph.wgsl")),
];

/// Vertices per dot: one triangle around the disc.
const DOT_VERTS: u32 = 3;

/// One marker or dot, 48 bytes, as the shaders read it.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3],     // world position
    pub radius: f32,          // 0 = pen width; > 0 world mm; < 0 screen px
    pub color: [f32; 4],      // rgba
    pub instance_id: u32,     // object row
    pub facing: u32,          // packed normals of the faces around it
    pub facing_ext: [u32; 2], // more packed normals
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

/// Marker and dot rows of one upload.
#[derive(Default)]
pub struct GlyphRows {
    pub spheres: Vec<GlyphPoint>, // vertex markers, shaded
    pub dots: Vec<GlyphPoint>,    // flat dots
}

impl GlyphRows {
    /// Empty both tables and free their memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.dots);
    }
}

/// One glyph buffer and its bind group.
struct GlyphTable {
    label: &'static str,    // name shown in GPU errors
    buf: GrowBuf,           // the rows
    group: wgpu::BindGroup, // group 3, binds the rows
}

impl GlyphTable {
    /// An empty table and its bind group.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<GlyphPoint>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Rebuild the bind group after the buffer moved.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.group = bind_group(ctx, &l.ink_rows, self.label, &[&self.buf.buf]);
    }
}

/// The two glyph shaders.
struct GlyphShaders {
    sphere: Shader, // shaded markers
    dot: Shader,    // flat dots
}

/// The five glyph pipelines.
struct GlyphPipelines {
    sphere: Pipeline,     // markers in color
    dot: Pipeline,        // dots in color
    id_sphere: Pipeline,  // marker object ids
    id_dot: Pipeline,     // dot object ids
    source_dot: Pipeline, // dot pick ids
}

/// Markers and dots on the GPU.
pub struct GlyphLane {
    spheres: GlyphTable,   // marker rows
    dots: GlyphTable,      // dot rows
    template: Template,    // one quad, drawn per marker
    shaders: GlyphShaders, // shader modules
    gpu: GlyphPipelines,   // pipelines
}

impl GlyphLane {
    /// Overwrite one marker or dot row.
    pub(crate) fn patch_marker(
        &mut self,
        ctx: &GpuCtx,
        index: u32,
        sphere: bool,
        glyph: GlyphPoint,
    ) {
        let table = if sphere {
            &mut self.spheres
        } else {
            &mut self.dots
        };
        table.buf.write_at(ctx, index, &[glyph]);
    }

    /// Overwrite one object's rows in place.
    pub(crate) fn patch(&mut self, ctx: &GpuCtx, at: super::patch::Counts, up: &GlyphRows) {
        self.spheres.buf.write_at(ctx, at.spheres, &up.spheres);
        self.dots.buf.write_at(ctx, at.dots, &up.dots);
    }

    /// Hand `count` marker or dot rows from `first` to the hidden row `sink`.
    pub(crate) fn kill(&mut self, ctx: &GpuCtx, spheres: bool, first: u32, count: u32, sink: u32) {
        let table = if spheres { &self.spheres } else { &self.dots };
        let dead = GlyphPoint {
            instance_id: sink,
            ..bytemuck::Zeroable::zeroed()
        };
        table.buf.fill(ctx, first, count, &dead);
    }

    /// Bytes reserved on the GPU by this lane.
    pub fn allocated_bytes(&self) -> u64 {
        self.spheres.buf.buf.size()
            + self.dots.buf.buf.size()
            + self.template.vbo.size()
            + self.template.ibo.size()
    }

    /// Create the lane: quad, shaders, pipelines, empty tables.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let (q_v, q_i) = unit_quad();
        let template = Template::new(ctx, "quad.template", &q_v, &q_i);
        let shaders = GlyphShaders {
            sphere: ink_module(ctx, "sphere.shader", shader!("sphere.wgsl")),
            dot: ink_module(ctx, "glyph.shader", shader!("glyph.wgsl")),
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

    /// Rebuild the pipelines for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }

    /// Append one upload's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        if self.spheres.buf.append(ctx, &up.spheres) {
            self.spheres.rebind(ctx, l);
        }

        if self.dots.buf.append(ctx, &up.dots) {
            self.dots.rebind(ctx, l);
        }
    }

    /// Draw the markers in color.
    pub fn draw_spheres(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_markers(pass, b, &self.gpu.sphere)
    }

    /// Draw the dots in color.
    pub fn draw_dots(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.dot)
    }

    /// Draw marker object ids.
    pub fn draw_sphere_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_markers(pass, b, &self.gpu.id_sphere)
    }

    /// Draw dot object ids.
    pub fn draw_dot_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.id_dot)
    }

    /// Draw dot pick ids.
    pub fn draw_source_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.source_dot)
    }

    /// Draw every marker with `pipeline`; returns the draw count.
    fn draw_markers(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &Pipeline) -> u32 {
        if self.spheres.buf.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.spheres.group, &[]);
        self.template.bind(pass);
        // one quad per marker row
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.spheres.buf.len());
        1
    }

    /// Draw every dot with `pipeline`; returns the draw count.
    fn draw_dot_table(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &Pipeline,
    ) -> u32 {
        if self.dots.buf.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.dots.group, &[]);
        // three vertices per dot, placed by the shader
        pass.draw(0..DOT_VERTS * self.dots.buf.len(), 0..1);
        1
    }

    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        self.spheres.buf.reset();
        self.dots.buf.reset();
    }

    /// Free both buffers.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.spheres.buf.release(ctx);
        self.dots.buf.release(ctx);
        self.spheres.rebind(ctx, l);
        self.dots.rebind(ctx, l);
    }

    /// Marker rows on the GPU.
    pub fn sphere_count(&self) -> u32 {
        self.spheres.buf.len()
    }

    /// Dot rows on the GPU.
    pub fn dot_count(&self) -> u32 {
        self.dots.buf.len()
    }
}

/// Build the five glyph pipelines.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &GlyphShaders, target: Target) -> GlyphPipelines {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.ink_rows];
    let template = [template_layout()];
    // markers: quad template, always drawn
    let marker = PipelineDesc::new(&s.sphere, &groups, &template, TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    // dots: no vertex buffer, always drawn
    let disc = PipelineDesc::new(&s.dot, &groups, &[], TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);

    GlyphPipelines {
        sphere: build(
            ctx,
            target,
            &marker.with("sphere", "fs_main").color(ColorWrite::Blended),
        ),
        dot: build(
            ctx,
            target,
            &disc.with("glyph", "fs_main").color(ColorWrite::Blended),
        ),
        id_sphere: build(
            ctx,
            Target::ID,
            &marker
                .with("sphere.id", "fs_id")
                .vertex("vs_front")
                .scene_samples(1),
        ),
        id_dot: build(
            ctx,
            Target::ID,
            &disc.with("glyph.id", "fs_id").scene_samples(1),
        ),
        source_dot: build(
            ctx,
            Target::ID,
            &disc
                .with("glyph.source", "fs_source_id")
                .vertex("vs_source")
                .scene_samples(1)
                .depth(DepthMode::OpaqueEqual),
        ),
    }
}

/// A square from -1 to 1; the shader cuts it to a circle.
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

    /// Both shaders declare the same GlyphPoint fields.
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

impl super::lane::Lane for GlyphLane {
    fn on_retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        self.retarget(ctx, layouts, target);
    }

    fn on_reset(&mut self, _ctx: &GpuCtx) {
        self.reset();
    }

    fn on_release(&mut self, ctx: &GpuCtx, layouts: &Layouts) {
        self.release(ctx, layouts);
    }

    fn bytes(&self) -> (u64, u64) {
        (self.allocated_bytes(), 0)
    }
}
