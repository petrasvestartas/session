use super::buffers::{GpuCtx, GrowBuf, INDICES, VERTS};
use super::frame::Binds;
use super::instanced::{arena_slot_layout, clamp};
use super::text_outline::{OutlineBuffers, OutlineTextLane};
use super::upload::drop_rows;
use crate::engine::pipelines::{
    Layouts, Pipeline, PipelineDesc, Shader, Target, build, instance_id_layout, scene_module,
    vertex_layout,
};
use session_rust::RenderVertex;
use wgpu::PrimitiveTopology::TriangleList;

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("triangle.wgsl", include_str!("../../shaders/triangle.wgsl")),
    (
        "text_outline.wgsl",
        include_str!("../../shaders/text_outline.wgsl"),
    ),
];

/// The normal word of a vertex without a normal; no encoded normal reaches it.
const NO_NORMAL: u32 = 0x8000_8000;

/// An arena vertex as the GPU stores it: 20 bytes, half the kernel's RenderVertex.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuVertex {
    position: [f32; 3], // object-space position; offset 0
    normal: u32,        // octahedral normal in two snorm16 halves, or NO_NORMAL; offset 12
    color: u32,         // rgba, one unorm byte each; offset 16, 20 bytes in all
}

impl GpuVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32, 2 => Unorm8x4];

    /// The vertex buffer layout of the arena.
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl From<&RenderVertex> for GpuVertex {
    /// Pack the normal into octahedral halves and the color into bytes.
    fn from(vertex: &RenderVertex) -> Self {
        let bytes = vertex.color.map(|c| (c.clamp(0.0, 1.0) * 255.0).round() as u8);
        Self {
            position: vertex.position,
            normal: octahedral(vertex.normal),
            color: u32::from_le_bytes(bytes),
        }
    }
}

/// A normal as two octahedral snorm16 halves, as `oct32_decode` reads them; NO_NORMAL when zero.
fn octahedral(normal: [f32; 3]) -> u32 {
    let length = normal[0].abs() + normal[1].abs() + normal[2].abs();

    if length == 0.0 || length.is_nan() {
        return NO_NORMAL;
    }

    let (x, y) = (normal[0] / length, normal[1] / length);
    let sign = |v: f32| if v < 0.0 { -1.0 } else { 1.0 };
    // the lower half folds over the diagonals
    let (x, y) = if normal[2] < 0.0 {
        ((1.0 - y.abs()) * sign(x), (1.0 - x.abs()) * sign(y))
    } else {
        (x, y)
    };
    // -32767 and -32768 stay free for NO_NORMAL
    let half = |v: f32| u32::from((v * 32767.0).round().clamp(-32766.0, 32767.0) as i16 as u16);
    half(x) | half(y) << 16
}

/// The GPU form of `vertices`.
fn gpu_vertices(vertices: &[RenderVertex]) -> Vec<GpuVertex> {
    vertices.iter().map(GpuVertex::from).collect()
}

/// Mesh rows of one upload, ready for the GPU.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>, // one vertex per row
    pub vids: Vec<u32>, // object row of each vertex
    pub idx: Vec<u32>, // triangle indices of solid faces
    pub idx_print: Vec<u32>, // triangle indices of sheet fills
    pub idx_text: Vec<u32>, // triangle indices of sheet lettering
    pub face_ids: Vec<u32>, // source face of each solid triangle
    pub face_sources: Vec<super::faces::FaceSource>, // where each face came from
    pub surface_boundaries: Vec<(u32, [u32; 2])>, // pipe and sample range per surface edge
    pub surface_samples: Vec<crate::app::surface_preview::Sample>, // surface points for previews
}

impl ArenaRows {
    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
        drop_rows(&mut self.face_ids);
        drop_rows(&mut self.face_sources);
        drop_rows(&mut self.surface_samples);
        drop_rows(&mut self.surface_boundaries);
    }
}

/// Pipelines that draw solid faces as masks.
struct ArenaPipelines {
    selection_mask: Pipeline, // marks selected faces
    masks: Pipeline, // both masks in one pass
}

/// All mesh geometry on the GPU, in five growing buffers.
pub struct ArenaLane {
    pub tiles: super::triangle_tiles::TriangleTiles, // screen tiles for visibility tests
    verts: GrowBuf, // vertex buffer
    vids: GrowBuf, // object row per vertex
    faces: GrowBuf, // solid face indices
    print: GrowBuf, // sheet fill indices
    text: GrowBuf, // sheet lettering indices
    shader: Shader, // triangle shader
    pipes: ArenaPipelines, // mask pipelines
    outline_text: OutlineTextLane, // draws sheet fills and lettering
    pub source_faces: super::faces::Faces, // solid faces with their source ids
}

impl ArenaLane {
    /// Geometry buffers for read-only GPU passes: vertices, owners, solid indices.
    pub fn geometry_buffers(&self) -> [&wgpu::Buffer; 3] {
        [&self.verts.buf, &self.vids.buf, &self.faces.buf]
    }

    /// Reproject the triangles for this camera; bin them into screen tiles too when `lists`.
    pub fn prepare_visibility(
        &mut self,
        ctx: &GpuCtx,
        encoder: &mut wgpu::CommandEncoder,
        binds: &Binds,
        matrix: [f32; 16],
        objects_revision: u64,
        lists: bool,
    ) {
        // instanced triangles are in no tile list: their ink tests the fitted planes alone
        let instanced = self.source_faces.slots.any();
        self.tiles.encode(
            ctx,
            encoder,
            super::triangle_tiles::TileInput {
                binds,
                geometry: [&self.verts.buf, &self.vids.buf, &self.faces.buf],
                matrix,
                objects_revision,
            },
            lists && !instanced,
        );

        if lists && instanced {
            self.tiles.drop_lists(encoder);
        }
    }

    /// Bytes reserved on the GPU by this lane.
    pub fn allocated_bytes(&self) -> u64 {
        self.verts.buf.size()
            + self.vids.buf.size()
            + self.faces.buf.size()
            + self.print.buf.size()
            + self.text.buf.size()
            + self.source_faces.allocated_bytes()
            + self.tiles.allocated_bytes().0
    }

    /// Create the lane with empty buffers.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = scene_module(
            ctx,
            "triangle.shader",
            include_str!("../../shaders/triangle.wgsl"),
        );
        let pipes = build_pipelines(ctx, l, &shader, target);

        let source_faces = super::faces::Faces::new(ctx, l, &shader, target);
        Self {
            source_faces,
            tiles: super::triangle_tiles::TriangleTiles::new(ctx, l),
            verts: GrowBuf::new(
                ctx,
                "arena.vbo",
                std::mem::size_of::<GpuVertex>() as u64,
                VERTS | wgpu::BufferUsages::STORAGE,
            ),
            vids: GrowBuf::new(ctx, "arena.vids", 4, VERTS | wgpu::BufferUsages::STORAGE),
            faces: GrowBuf::new(ctx, "arena.ibo", 4, INDICES | wgpu::BufferUsages::STORAGE),
            print: GrowBuf::new(ctx, "arena.ibo.print", 4, INDICES),
            text: GrowBuf::new(ctx, "arena.ibo.text", 4, INDICES),
            shader,
            pipes,
            outline_text: OutlineTextLane::new(ctx, l, target),
        }
    }

    /// Rebuild the pipelines for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.pipes = build_pipelines(ctx, l, &self.shader, target);
        self.outline_text.retarget(ctx, l, target);
        self.source_faces.retarget(ctx, l, &self.shader, target);
    }

    /// Append one upload's rows to every buffer.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.tiles.invalidate();
        self.verts.append(ctx, &gpu_vertices(&up.verts));
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
        self.source_faces
            .append(ctx, up, [&self.verts.buf, &self.vids.buf, &self.faces.buf]);
    }

    /// Overwrite vertices starting at row `first`.
    pub(crate) fn patch_vertices(&mut self, ctx: &GpuCtx, first: u32, vertices: &[RenderVertex]) {
        self.tiles.invalidate();
        self.verts.write_at(ctx, first, &gpu_vertices(vertices));
    }

    /// Overwrite one object's rows in place.
    pub(crate) fn patch(&mut self, ctx: &GpuCtx, at: super::patch::Counts, up: &ArenaRows) {
        self.tiles.invalidate();
        self.verts.write_at(ctx, at.verts, &gpu_vertices(&up.verts));
        self.vids.write_at(ctx, at.verts, &up.vids);
        self.faces.write_at(ctx, at.faces, &up.idx);
        self.print.write_at(ctx, at.print, &up.idx_print);
        self.text.write_at(ctx, at.text, &up.idx_text);
        self.source_faces.patch(ctx, at, up);
    }

    /// Hand `count` rows of `lane` from `first` to the hidden row `sink`: vertices and source faces
    /// directly, face indices by their face ids; print and text indices die with their vertices.
    pub(crate) fn kill(
        &mut self,
        ctx: &GpuCtx,
        lane: super::patch::LaneId,
        first: u32,
        count: u32,
        sink: u32,
    ) {
        use super::patch::LaneId;

        if count == 0 {
            return;
        }

        self.tiles.invalidate();

        match lane {
            LaneId::Verts => self.vids.fill(ctx, first, count, &sink),
            LaneId::Faces => self.source_faces.kill_ids(ctx, first / 3, count / 3),
            LaneId::Sources => self.source_faces.kill_sources(first, count),
            _ => {}
        }
    }

    /// Turn `count` indices of a face, print or text run from `first` into empty triangles on `vertex`.
    pub(crate) fn degenerate(
        &mut self,
        ctx: &GpuCtx,
        lane: super::patch::LaneId,
        first: u32,
        count: u32,
        vertex: u32,
    ) {
        use super::patch::LaneId;

        if count == 0 {
            return;
        }

        self.tiles.invalidate();

        match lane {
            LaneId::Faces => {
                self.faces.fill(ctx, first, count, &vertex);
                self.source_faces.kill_ids(ctx, first / 3, count / 3);
            }
            LaneId::Print => self.print.fill(ctx, first, count, &vertex),
            LaneId::Text => self.text.fill(ctx, first, count, &vertex),
            _ => {}
        }
    }

    /// Draw the solid faces; `opaque` when nothing is see-through, `clipped` while planes cut.
    pub fn draw_faces(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        opaque: bool,
        clipped: bool,
    ) -> u32 {
        self.source_faces.draw_physical(pass, b, opaque, clipped)
    }

    /// Draw the selected faces into a mask.
    pub fn draw_selection_mask(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.selection_mask, &self.faces)
    }

    /// Draw sheet fills.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw(pass, b, &self.outline_buffers(&self.print))
    }

    /// Draw sheet lettering.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw(pass, b, &self.outline_buffers(&self.text))
    }

    /// Draw object ids of faces and sheet fills.
    pub fn draw_face_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.source_faces.draw_object_ids(pass, b)
            + self
                .outline_text
                .draw_physical_ids(pass, b, &self.outline_buffers(&self.print))
    }

    /// Draw face ids of faces and sheet fills.
    pub fn draw_component_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.source_faces.draw_ids(pass, b)
            + self
                .outline_text
                .draw_physical_ids(pass, b, &self.outline_buffers(&self.print))
    }

    /// Both coverage masks from one pass over the faces.
    pub fn draw_masks(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.masks, &self.faces)
    }

    /// Draw object ids of sheet lettering.
    pub fn draw_text_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw_ids(pass, b, &self.outline_buffers(&self.text))
    }

    /// Bundle the buffers one outline draw needs.
    fn outline_buffers<'a>(&'a self, indices: &'a GrowBuf) -> OutlineBuffers<'a> {
        OutlineBuffers {
            vertices: &self.verts,
            objects: &self.vids,
            indices,
        }
    }

    /// Index count of sheet fills and lettering together.
    pub fn sheet_count(&self) -> u32 {
        self.text.len().saturating_add(self.print.len())
    }

    /// Draw one index buffer with `pipeline`; returns the draw count.
    fn draw_run(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &Pipeline,
        run: &GrowBuf,
    ) -> u32 {
        if run.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_vertex_buffer(0, self.verts.buf.slice(..));
        pass.set_vertex_buffer(1, self.vids.buf.slice(..));
        self.source_faces.slots.bind(pass, 2);
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.len(), 0, 0..1);
        let mut draws = 1;

        // each definition once more per instance, its rows from the slots
        for draw in self.source_faces.slots.draws() {
            let faces = clamp(&draw.faces, run.len());

            if !faces.is_empty() {
                pass.draw_indexed(faces, 0, draw.slots.clone());
                draws += 1;
            }
        }

        draws
    }

    /// Draw the solid face index `runs` with `pipeline` as instance `instance`, with an optional group 3.
    pub fn draw_solids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &Pipeline,
        group: Option<&wgpu::BindGroup>,
        runs: &[std::ops::Range<u32>],
        instance: u32,
    ) -> u32 {
        if self.faces.is_empty() || runs.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);

        if let Some(group) = group {
            pass.set_bind_group(3, group, &[]);
        }

        pass.set_vertex_buffer(0, self.verts.buf.slice(..));
        pass.set_vertex_buffer(1, self.vids.buf.slice(..));
        pass.set_index_buffer(self.faces.buf.slice(..), wgpu::IndexFormat::Uint32);

        for run in runs {
            let end = run.end.min(self.faces.len());

            // a run past a shrunk buffer would wrap the index count
            if run.start < end {
                pass.draw_indexed(run.start..end, 0, instance..instance + 1);
            }
        }

        runs.len() as u32
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self, ctx: &GpuCtx) {
        self.tiles.invalidate();
        self.source_faces.reset(ctx);
        self.verts.reset();
        self.vids.reset();
        self.faces.reset();
        self.print.reset();
        self.text.reset();
    }

    /// Free every buffer.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.tiles.release(ctx);
        self.source_faces.release(ctx);
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
        self.print.release(ctx);
        self.text.release(ctx);
    }

    /// Vertices on the GPU.
    pub fn vert_count(&self) -> u32 {
        self.verts.len()
    }

    /// Index count of the solid faces.
    pub fn face_count(&self) -> u32 {
        self.faces.len()
    }
}

/// Build the two mask pipelines for `target`.
fn build_pipelines(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &Shader,
    target: Target,
) -> ArenaPipelines {
    // bind groups every mask pipeline uses
    let groups = [&l.mvp, &l.line, &l.instance];
    // vertex buffer 0: vertices, 1: object rows, 2: instance slots
    let buffers = [vertex_layout(), instance_id_layout(), arena_slot_layout()];
    let base = PipelineDesc::new(shader, &groups, &buffers, TriangleList);

    ArenaPipelines {
        // masks write to a one-channel texture at the scene depth
        selection_mask: build(
            ctx,
            Target {
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                samples: target.samples,
            },
            &base
                .with("triangle.selection_mask", "fs_selection_mask")
                .depth(crate::engine::pipelines::DepthMode::ReadOnlyEqual),
        ),
        masks: build(
            ctx,
            Target {
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                samples: target.samples,
            },
            &base
                .with("triangle.masks", "fs_masks")
                .depth(crate::engine::pipelines::DepthMode::ReadOnlyEqual)
                .masks(),
        ),
    }
}

impl super::lane::Lane for ArenaLane {
    fn on_retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        self.retarget(ctx, layouts, target);
    }

    fn on_reset(&mut self, ctx: &GpuCtx) {
        self.reset(ctx);
    }

    fn on_release(&mut self, ctx: &GpuCtx, _layouts: &Layouts) {
        self.release(ctx);
    }

    fn bytes(&self) -> (u64, u64) {
        (self.allocated_bytes(), self.tiles.allocated_bytes().1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `oct32_decode` from normals.wgsl, for the round trip.
    fn decode(word: u32) -> [f32; 3] {
        if word == NO_NORMAL {
            return [0.0; 3];
        }

        let half = |bits: u32| (f32::from(bits as u16 as i16) / 32767.0).max(-1.0);
        let (x, y) = (half(word & 0xffff), half(word >> 16));
        let z = 1.0 - x.abs() - y.abs();
        let sign = |v: f32| if v < 0.0 { -1.0 } else { 1.0 };
        let (x, y) = if z < 0.0 {
            ((1.0 - y.abs()) * sign(x), (1.0 - x.abs()) * sign(y))
        } else {
            (x, y)
        };
        let length = (x * x + y * y + z * z).sqrt();
        [x / length, y / length, z / length]
    }

    /// Angle between two unit vectors, exact for small angles.
    fn angle_to(a: [f32; 3], b: [f32; 3]) -> f32 {
        let chord: f32 = (0..3).map(|k| (a[k] - b[k]).powi(2)).sum::<f32>().sqrt();
        2.0 * (chord / 2.0).asin()
    }

    /// Packed vertices are 20 bytes and match the attribute offsets.
    #[test]
    fn packed_vertex_is_twenty_bytes() {
        assert_eq!(std::mem::size_of::<GpuVertex>(), 20);
        assert_eq!(std::mem::offset_of!(GpuVertex, normal), 12);
        assert_eq!(std::mem::offset_of!(GpuVertex, color), 16);
        let offsets: Vec<u64> = GpuVertex::ATTRIBS.iter().map(|a| a.offset).collect();
        assert_eq!(offsets, [0, 12, 16]);
        assert_eq!(GpuVertex::layout().array_stride, 20);
    }

    /// Normals survive packing within a ten-thousandth of a radian; zero stays zero.
    #[test]
    fn octahedral_normals_round_trip() {
        let mut worst = 0.0_f32;

        for i in 0..2000 {
            // points spread over the sphere, poles and the fold included
            let z = 1.0 - 2.0 * (i as f32 + 0.5) / 2000.0;
            let angle = i as f32 * 2.399_963;
            let ring = (1.0 - z * z).sqrt();
            let normal = [ring * angle.cos(), ring * angle.sin(), z];
            worst = worst.max(angle_to(normal, decode(octahedral(normal))));
        }

        // the poles, a point beside the south pole's fold, the equator
        let edges = [
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
            [-1e-7, -1e-7, -1.0],
            [1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
        ];

        for normal in edges {
            worst = worst.max(angle_to(normal, decode(octahedral(normal))));
        }

        assert!(worst < 1e-4, "largest angle {worst}");
        assert_eq!(octahedral([0.0; 3]), NO_NORMAL);
        assert_eq!(decode(octahedral([0.0; 3])), [0.0; 3]);
        // a scaled normal packs like its unit direction
        assert_eq!(octahedral([0.0, 3.0, 4.0]), octahedral([0.0, 0.6, 0.8]));
    }

    /// Colors pack to the nearest byte, clamped to the unit range.
    #[test]
    fn colors_pack_to_bytes() {
        let vertex = RenderVertex {
            position: [1.0, 2.0, 3.0],
            normal: [0.0; 3],
            color: [1.0, 0.5, -0.25, 2.0],
        };
        let packed = GpuVertex::from(&vertex);
        assert_eq!(packed.position, [1.0, 2.0, 3.0]);
        assert_eq!(packed.color.to_le_bytes(), [255, 128, 0, 255]);
        assert_eq!(packed.normal, NO_NORMAL);
    }
}
