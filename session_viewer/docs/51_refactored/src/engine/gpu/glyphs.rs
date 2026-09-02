//! The glyph family - every vertex-sized piece of ink. Two tables of the same 48 B row: spheres
//! (mesh/BRep vertex markers, the SOLID lane, drawn on a quad template) and dots (free points,
//! the FLAT lane, three verts per dot). `GlyphRows` is one upload; `GlyphLane` the GPU.

use crate::engine::pipelines::{Layouts, Pipelines};
use super::buffers::{rows_group, GpuCtx, GrowBuf, Template};
use super::frame::Binds;

/// One marker or dot row, 48 B (three 16 B rows), the layout sphere.wgsl and glyph.wgsl declare.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3], // 12 B - mesh-local
    pub radius: f32, // 4 B - 0.0 - screen-constant px; 0 - world mm
    pub color:  [f32; 4],
    pub instance_id: u32, // 4 B - row in instances[]
    // Up to SIX incident face normals (oct16 pairs), widest incident edge's two first - the same
    // adjacency CylinderSegment carries one word of. A marker that hugs only the widest edge's
    // two faces still loses a sector of its disc to the THIRD face's band at a trihedral corner
    // (measured on a box corner); all-ones (FACING_UNKNOWN) means "no adjacency / no more".
    pub facing: u32,
    pub facing_ext: [u32; 2],
} // 48 B total, three 16-byte rows

// The WGSL GlyphPoint (glyph.wgsl AND sphere.wgsl - same table) is exactly this layout; the
// array stride is the struct's, so a drift here misreads every row.
const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

/// One upload's glyphs: the solid lane's vertex markers and the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    /// Solid lane: mesh/BRep vertices, radius matched to the pipes.
    pub spheres: Vec<GlyphPoint>,
    /// Flat lane: points, drawn as SDF dots.
    pub dots: Vec<GlyphPoint>,
}

/// Vertex ink, split like the segments: spheres are mesh/BRep vertices, dots are flat points.
/// Same layout and same table shape; each lane indexes from row 0 and grows by appending.
pub struct GlyphLane {
    spheres: GrowBuf,
    dots: GrowBuf,
    sphere_group: wgpu::BindGroup,
    dot_group: wgpu::BindGroup,
    template: Template,
}

impl GlyphLane {
    /// Two one-row tables and the camera-facing quad, uploaded once.
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stride = std::mem::size_of::<GlyphPoint>() as u64;
        let spheres = GrowBuf::new(ctx, "spheres.buffer", stride, rows);
        let dots = GrowBuf::new(ctx, "glyphs.buffer", stride, rows);
        let sphere_group = rows_group(ctx, &l.glyph, "spheres.bind_group", &spheres.buf);
        let dot_group = rows_group(ctx, &l.glyph, "glyphs.bind_group", &dots.buf);
        let (sph_v, sph_i) = unit_quad();
        let template = Template::new(ctx, "sph.template", &sph_v, &sph_i);

        Self { spheres, dots, sphere_group, dot_group, template }
    }

    /// Append one file's rows (a DELTA); a bind group is rebuilt only when its buffer grew.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        if self.spheres.append(ctx, &up.spheres) {
            self.sphere_group = rows_group(ctx, &l.glyph, "spheres.bind_group", &self.spheres.buf);
        }
        if self.dots.append(ctx, &up.dots) {
            self.dot_group = rows_group(ctx, &l.glyph, "glyphs.bind_group", &self.dots.buf);
        }
    }

    /// Vertex markers are drawn LAST of the solid lane, after the bands, and their
    /// pipeline compares GreaterEqual. Drawn FIRST (the previous arrangement) the marker
    /// had to win STRICTLY - the band, testing GreaterEqual against the marker's depth,
    /// takes the pixel on any tie - so every pixel where the two computed the same depth
    /// went to the band, and the disc lost a bite of its rim wherever a band cap crossed
    /// it. Ordering it last inverts that: the marker only has to MATCH the band's depth to
    /// keep the pixel, which is a strictly weaker condition, so it can only ever draw more
    /// of the disc. Real occlusion is untouched - anything genuinely nearer still has a
    /// higher depth and still wins.
    ///
    /// Faces are already down by this point, so a vertex hidden inside the solid stays
    /// hidden, which was the reason markers went early in the first place.
    ///
    /// Prepass + colour = 2 draws; the caller gates on `show_mesh_edges && markers`.
    pub fn draw_spheres(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.spheres.is_empty() {
            return 0;
        }

        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.sphere_group, &[]);
        pass.set_vertex_buffer(0, self.template.vbo.slice(..));
        pass.set_index_buffer(self.template.ibo.slice(..), wgpu::IndexFormat::Uint32);
        // Same prepass split as the solid ribbons - see `SegmentLane::draw_pipes`.
        pass.set_pipeline(&p.sphere_depth);
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.spheres.len());
        pass.set_pipeline(&p.sphere);
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.spheres.len()); // one template, N glyphs
        2
    }

    /// The flat lane's colour pass: SDF dots, three verts each, no template.
    pub fn draw_dots(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.dots.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.glyph);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.dot_group, &[]);
        pass.draw(0..3 * self.dots.len(), 0..1); // 3 verts/dot, no template
        1
    }

    /// The flat lane's depth prepass (`INK_DEPTH_PREPASS`): the same dots, depth only.
    pub fn draw_dot_depth(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.dots.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.glyph_depth);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.dot_group, &[]);
        pass.draw(0..3 * self.dots.len(), 0..1);
        1
    }

    /// Forget every row; the buffers keep their capacity.
    pub fn reset(&mut self) {
        self.spheres.reset();
        self.dots.reset();
    }

    /// Hand both buffers back (one-row tables again) and re-point the groups at them.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.spheres.release(ctx);
        self.dots.release(ctx);
        self.sphere_group = rows_group(ctx, &l.glyph, "spheres.bind_group", &self.spheres.buf);
        self.dot_group = rows_group(ctx, &l.glyph, "glyphs.bind_group", &self.dots.buf);
    }

    /// Solid-lane rows on the GPU - the MSAA test reads it.
    pub fn sphere_count(&self) -> u32 {
        self.spheres.len()
    }

    /// Flat-lane rows on the GPU.
    pub fn dot_count(&self) -> u32 {
        self.dots.len()
    }
}

/// Camera-facing quad template (positions + indices) for the instanced vertex markers. The
/// shader expands it in SCREEN space and trims to a circle in the fragment with a 1px AA ramp,
/// so the silhouette is a perfect circle at any radius. This replaced a tessellated unit sphere:
/// 6x3 segments was a comment-era choice ("a few pixels across") that reads as a hexagon at the
/// sizes world-mm pens reach, and any fixed tessellation is still a polygon when you zoom in -
/// the SDF is exact and cheaper (2 triangles instead of 36+).
fn unit_quad() -> (Vec<[f32; 3]>, Vec<u32>) {
    let v = vec![
        [-1.0, -1.0, 0.0],
        [ 1.0, -1.0, 0.0],
        [ 1.0,  1.0, 0.0],
        [-1.0,  1.0, 0.0],
    ];
    let idx = vec![0u32, 1, 2, 0, 2, 3];
    (v, idx)
}
