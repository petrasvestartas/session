// --8<-- [start:arena-vertex]
// The arena = one growing vertex buffer shared by every mesh, a parallel buffer with each vertex's
// object row, and index buffers; a mesh is just a run of indices into it, so all meshes draw at once.
use super::buffers::{GpuCtx, GrowBuf, INDICES, VERTS};
use super::frame::Binds;
use super::slots::{SlotSpace, SlotTable, arena_slot_layout, clamp};
use super::text_outline::{OutlineBuffers, OutlineTextLane};
use super::upload::drop_rows;
use crate::engine::pipelines::{
    Layouts, Pipeline, PipelineDesc, Shader, Target, build, instance_id_layout, scene_module,
    vertex_layout,
};
use session_rust::RenderVertex;
use wgpu::PrimitiveTopology::TriangleList;

/// #[cfg(test)] compiles this only for `cargo test`, where a test validates every listed shader.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[
    ("triangle.wgsl", shader!("triangle.wgsl")),
    ("text_outline.wgsl", shader!("text_outline.wgsl")),
];

/// Both halves -32768, a value `octahedral` never writes, so it can mean "this vertex has no normal".
const NO_NORMAL: u32 = 0x8000_8000;

/// 20 bytes a vertex instead of the kernel's 40: the normal packed into 4 bytes, the colour into 4.
/// A million-vertex mesh then takes 20 MB of GPU memory, not 40.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuVertex {
    position: [f32; 3], // byte 0, object space
    normal: u32,        // byte 12, octahedral: two 16-bit halves, or NO_NORMAL
    color: u32,         // byte 16, rgba: one byte a channel
}

impl GpuVertex {
    // vertex_attr_array! numbers the locations and sums the offsets from the formats: 0, 12, 16.
    // Unorm8x4 = 4 bytes the shader receives as 4 floats 0..1, so 255 arrives as 1.0.
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32, 2 => Unorm8x4];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

// Implementing the From trait gives `GpuVertex::from(&v)`, and lets `.map(GpuVertex::from)` convert a list.
impl From<&RenderVertex> for GpuVertex {
    fn from(vertex: &RenderVertex) -> Self {
        let bytes = vertex
            .color
            .map(|c| (c.clamp(0.0, 1.0) * 255.0).round() as u8); // array map: [f32; 4] in, [u8; 4] out
        Self {
            position: vertex.position,
            normal: octahedral(vertex.normal),
            color: u32::from_le_bytes(bytes), // little-endian: red in the lowest byte, as Unorm8x4 reads it
        }
    }
}
// --8<-- [end:arena-vertex]

// --8<-- [start:arena-octahedral]
/// The inverse of `oct32_decode` in normals.wgsl: 12 bytes of normal into one u32.
fn octahedral(normal: [f32; 3]) -> u32 {
    let length = normal[0].abs() + normal[1].abs() + normal[2].abs();

    if length == 0.0 || length.is_nan() {
        return NO_NORMAL;
    }

    let (x, y) = (normal[0] / length, normal[1] / length);
    let sign = |v: f32| if v < 0.0 { -1.0 } else { 1.0 };
    // dividing by |x| + |y| + |z| puts the point on the octahedron; the lower half folds over its diagonals
    let (x, y) = if normal[2] < 0.0 {
        ((1.0 - y.abs()) * sign(x), (1.0 - x.abs()) * sign(y))
    } else {
        (x, y)
    };
    // `as i16 as u16` keeps the bits of a negative number: -1 becomes 0xffff; -32768 stays free for NO_NORMAL
    let half = |v: f32| u32::from((v * 32767.0).round().clamp(-32766.0, 32767.0) as i16 as u16);
    half(x) | half(y) << 16
}

fn gpu_vertices(vertices: &[RenderVertex]) -> Vec<GpuVertex> {
    vertices.iter().map(GpuVertex::from).collect()
}
// --8<-- [end:arena-octahedral]

// --8<-- [start:arena-rows]
/// The CPU side of one upload: plain lists the walk fills, one entry per GPU element.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,                           // object row of each vertex
    pub idx: Vec<u32>,                            // three per triangle: solid faces
    pub idx_print: Vec<u32>,                      // sheet fills
    pub idx_text: Vec<u32>,                       // sheet lettering
    pub face_ids: Vec<u32>,                       // source face of each solid triangle
    pub face_sources: Vec<FaceSource>,
    pub surface_boundaries: Vec<(u32, [u32; 2])>, // pipe and sample range per surface edge
    pub surface_samples: Vec<Sample>,             // surface points for previews
}

impl ArenaRows {
    /// Once the GPU holds the rows the CPU copy is waste: free it.
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
// --8<-- [end:arena-rows]

// --8<-- [start:arena-lane]
/// The mask pipelines; faces.rs draws the shaded faces.
struct ArenaPipelines {
    selection_mask: Pipeline,
    masks: Pipeline, // solid and selection masks in one pass
}

/// An index buffer lists triangle corners as vertex numbers, so a corner shared by six triangles
/// is stored once; here three index buffers share one vertex buffer.
pub struct ArenaLane {
    // --8<-- [start:18-tiles-field]
    // --8<-- [start:tiles-field]
    pub tiles: super::triangle_tiles::TriangleTiles, // screen tiles for visibility tests; register:tiles
    // --8<-- [end:tiles-field]
    // --8<-- [end:18-tiles-field]
    verts: GrowBuf,                                  // GpuVertex rows
    vids: GrowBuf,                                   // object row per vertex
    faces: GrowBuf,                                  // indices of solid faces
    print: GrowBuf,                                  // indices of sheet fills
    text: GrowBuf,                                   // indices of sheet lettering
    shader: Shader,                                  // triangle.wgsl
    pipes: ArenaPipelines,
    outline_text: OutlineTextLane,
    pub source_faces: super::faces::Faces,           // solid faces with their source ids
    pub table: SlotTable,
    pub space: SlotSpace,
}

impl ArenaLane {
    /// The three buffers passes read as storage: vertices, object rows, solid indices.
    pub fn geometry_buffers(&self) -> [&wgpu::Buffer; 3] {
        [&self.verts.buf, &self.vids.buf, &self.faces.buf]
    }

    /// Triangle ids in use: the arena's, then one per instance triangle.
    pub fn triangle_count(&self) -> u32 {
        // unwrap_or: the value inside Some, or this fallback for None
        self.space.triangle_count().unwrap_or(self.face_count() / 3)
    }

    /// Bytes reserved on the GPU by this lane.
    pub fn allocated_bytes(&self) -> u64 {
        self.verts.buf.size()
            + self.vids.buf.size()
            + self.faces.buf.size()
            + self.print.buf.size()
            + self.text.buf.size()
            + self.source_faces.allocated_bytes()
            // --8<-- [start:18-tiles-bytes]
            // --8<-- [start:tiles-bytes]
            + self.tiles.allocated_bytes().0 // register:tiles
            // --8<-- [end:tiles-bytes]
            // --8<-- [end:18-tiles-bytes]
    }

    /// Bytes of the lane's textures: the tile target and the slot table.
    pub fn texture_bytes(&self) -> u64 {
        let mut bytes = self.table.allocated_bytes();
        // --8<-- [start:18-tiles-texture-bytes]
        // --8<-- [start:tiles-texture-bytes]
        bytes += self.tiles.allocated_bytes().1; // register:tiles
        // --8<-- [end:tiles-texture-bytes]
        // --8<-- [end:18-tiles-texture-bytes]
        bytes
    }

    /// Every buffer starts empty and grows as meshes are appended.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = scene_module(ctx, "triangle.shader", shader!("triangle.wgsl"));
        let pipes = build_pipelines(ctx, l, &shader, target);

        let source_faces = super::faces::Faces::new(ctx, l, &shader, target);
        Self {
            source_faces,
            // --8<-- [start:18-tiles-new]
            // --8<-- [start:tiles-new]
            tiles: super::triangle_tiles::TriangleTiles::new(ctx, l), // register:tiles
            // --8<-- [end:tiles-new]
            // --8<-- [end:18-tiles-new]
            // STORAGE too: faces.rs and later passes read the same bytes by index
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
            table: SlotTable::new(ctx),
            space: SlotSpace::default(),
        }
    }

    /// New MSAA sample count: rebuild every pipeline, keep the buffers.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.pipes = build_pipelines(ctx, l, &self.shader, target);
        self.outline_text.retarget(ctx, l, target);
        self.source_faces.retarget(ctx, l, &self.shader, target);
    }
// --8<-- [end:arena-lane]

// --8<-- [start:arena-write]
    /// Loading a second file appends; nothing already on the GPU moves.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        // --8<-- [start:18-tiles-append]
        // --8<-- [start:tiles-append]
        self.tiles.invalidate(); // register:tiles
        // --8<-- [end:tiles-append]
        // --8<-- [end:18-tiles-append]
        self.verts.append(ctx, &gpu_vertices(&up.verts));
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
        self.source_faces
            .append(ctx, up, [&self.verts.buf, &self.vids.buf, &self.faces.buf]);
    }

    /// pub(crate) = visible anywhere in this crate, but not to other crates.
    pub(crate) fn patch_vertices(&mut self, ctx: &GpuCtx, first: u32, vertices: &[RenderVertex]) {
        // --8<-- [start:18-tiles-patch-vertices]
        // --8<-- [start:tiles-patch-vertices]
        self.tiles.invalidate(); // register:tiles
        // --8<-- [end:tiles-patch-vertices]
        // --8<-- [end:18-tiles-patch-vertices]
        self.verts.write_at(ctx, first, &gpu_vertices(vertices));
    }

    /// Overwrite one object's rows in place.
    pub(crate) fn patch(&mut self, ctx: &GpuCtx, at: super::patch::Counts, up: &ArenaRows) {
        // --8<-- [start:18-tiles-patch]
        // --8<-- [start:tiles-patch]
        self.tiles.invalidate(); // register:tiles
        // --8<-- [end:tiles-patch]
        // --8<-- [end:18-tiles-patch]
        self.verts.write_at(ctx, at.verts, &gpu_vertices(&up.verts));
        self.vids.write_at(ctx, at.verts, &up.vids);
        self.faces.write_at(ctx, at.faces, &up.idx);
        self.print.write_at(ctx, at.print, &up.idx_print);
        self.text.write_at(ctx, at.text, &up.idx_text);
        self.source_faces.patch(ctx, at, up);
    }

    /// Delete and undo hide rows instead of freeing them: each vertex moves to object row `sink`,
    /// which is hidden, so it draws nothing but keeps its place for an instant undo.
    pub(crate) fn kill(
        &mut self,
        ctx: &GpuCtx,
        lane: super::patch::LaneId,
        first: u32,
        count: u32,
        sink: u32,
    ) {
        use super::patch::LaneId; // a `use` inside a function is visible only in it

        if count == 0 {
            return;
        }

        // --8<-- [start:18-tiles-kill]
        // --8<-- [start:tiles-kill]
        self.tiles.invalidate(); // register:tiles
        // --8<-- [end:tiles-kill]
        // --8<-- [end:18-tiles-kill]
        match lane {
            LaneId::Verts => self.vids.fill(ctx, first, count, &sink),
            LaneId::Faces => self.source_faces.kill_ids(ctx, first / 3, count / 3),
            LaneId::Sources => self.source_faces.kill_sources(first, count),
            _ => {}
        }
    }

    /// All three corners on one vertex make a triangle with no area, which the GPU skips.
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

        // --8<-- [start:18-tiles-degenerate]
        // --8<-- [start:tiles-degenerate]
        self.tiles.invalidate(); // register:tiles
        // --8<-- [end:tiles-degenerate]
        // --8<-- [end:18-tiles-degenerate]
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
// --8<-- [end:arena-write]

// --8<-- [start:arena-draw]
    /// faces.rs draws the shaded solids by vertex pulling; `opaque` when nothing is see-through,
    /// `clipped` while planes cut. Every draw method returns its draw count for the statistics.
    pub fn draw_faces(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        opaque: bool,
        clipped: bool,
    ) -> u32 {
        self.source_faces.draw_physical(pass, b, opaque, clipped)
    }

    pub fn draw_selection_mask(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.selection_mask, &self.faces)
    }

    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw(pass, b, &self.outline_buffers(&self.print))
    }

    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw(pass, b, &self.outline_buffers(&self.text))
    }

    /// Object ids: a click selects the whole object.
    pub fn draw_face_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.source_faces.draw_object_ids(pass, b)
            + self
                .outline_text
                .draw_physical_ids(pass, b, &self.outline_buffers(&self.print))
    }

    /// Face ids: a click selects one face of a solid.
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

    pub fn draw_text_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.outline_text
            .draw_ids(pass, b, &self.outline_buffers(&self.text))
    }

    /// The same lifetime 'a on self and `indices`: the bundle may live only as long as both.
    fn outline_buffers<'a>(&'a self, indices: &'a GrowBuf) -> OutlineBuffers<'a> {
        OutlineBuffers {
            vertices: &self.verts,
            objects: &self.vids,
            indices,
        }
    }

    pub fn sheet_count(&self) -> u32 {
        self.text.len().saturating_add(self.print.len())
    }

    /// The first mesh draw: vertex buffers 0-2, one index buffer, one draw_indexed call.
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
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32); // Uint32: 4 bytes an index
        // draw_indexed(which indices, number added to each index, which instances): all of them, once
        pass.draw_indexed(0..run.len(), 0, 0..1);
        let mut draws = 1;

        // from lesson 18a: each shared mesh once more per instance, its rows from the slots
        for draw in self.source_faces.slots.draws() {
            let faces = clamp(&draw.faces, run.len());

            if !faces.is_empty() {
                pass.draw_indexed(faces, 0, draw.slots.clone());
                draws += 1;
            }
        }

        draws
    }
// --8<-- [end:arena-draw]

// --8<-- [start:arena-instanced]
    /// Later passes (outlines, shadows, caps) draw chosen runs of solid faces with their own pipeline.
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

        // `if let Some(group)` runs the block only when there is a group, with `group` unwrapped
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

    /// The same for instanced solids: buffer 1 is `placed`, one (row, plane) record per instance.
    pub fn draw_placed(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &Pipeline,
        group: Option<&wgpu::BindGroup>,
        runs: &[(std::ops::Range<u32>, std::ops::Range<u32>)],
        placed: &wgpu::Buffer,
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
        pass.set_vertex_buffer(1, placed.slice(..));
        pass.set_index_buffer(self.faces.buf.slice(..), wgpu::IndexFormat::Uint32);
        let mut draws = 0;

        for (faces, records) in runs {
            let faces = clamp(faces, self.faces.len());

            if !faces.is_empty() {
                pass.draw_indexed(faces, 0, records.clone());
                draws += 1;
            }
        }

        draws
    }
// --8<-- [end:arena-instanced]

// --8<-- [start:arena-reset]
    /// A new scene: forget every row but keep the GPU memory for it.
    pub fn reset(&mut self, ctx: &GpuCtx) {
        // --8<-- [start:18-tiles-reset]
        // --8<-- [start:tiles-reset]
        self.tiles.invalidate(); // register:tiles
        // --8<-- [end:tiles-reset]
        // --8<-- [end:18-tiles-reset]
        self.source_faces.reset(ctx);
        self.space = SlotSpace::default();
        self.table.write(ctx, &[]);
        self.verts.reset();
        self.vids.reset();
        self.faces.reset();
        self.print.reset();
        self.text.reset();
    }

    /// Give the GPU memory back.
    pub fn release(&mut self, ctx: &GpuCtx) {
        // --8<-- [start:18-tiles-release]
        // --8<-- [start:tiles-release]
        self.tiles.release(ctx); // register:tiles
        // --8<-- [end:tiles-release]
        // --8<-- [end:18-tiles-release]
        self.source_faces.release(ctx);
        self.space = SlotSpace::default();
        self.table.write(ctx, &[]);
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
        self.print.release(ctx);
        self.text.release(ctx);
    }

    pub fn vert_count(&self) -> u32 {
        self.verts.len()
    }

    /// Indices, not triangles: three per triangle.
    pub fn face_count(&self) -> u32 {
        self.faces.len()
    }
}
// --8<-- [end:arena-reset]

// --8<-- [start:arena-pipelines]
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &Shader, target: Target) -> ArenaPipelines {
    let groups = [&l.mvp, &l.line, &l.instance];
    // the order is the vertex buffer number draw_run binds: 0 vertices, 1 object rows, 2 slots
    let buffers = [vertex_layout(), instance_id_layout(), arena_slot_layout()];
    let base = PipelineDesc::new(shader, &groups, &buffers, TriangleList);

    ArenaPipelines {
        // ReadOnlyEqual: draw only where this surface is the one the depth buffer already holds
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

// The Gpu keeps its lanes in one list and calls these hooks on each; the arena forwards them.
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
        (self.allocated_bytes(), self.texture_bytes())
    }
}
// --8<-- [end:arena-pipelines]

// --8<-- [start:arena-tests]
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
// --8<-- [end:arena-tests]

// --8<-- [start:arena-faces]
/// Bit 29 marks a pick id as a face, not a whole object; triangle.wgsl holds the same value.
pub const FACE_TAG: u32 = 0x2000_0000;

/// A solid's triangles come from its faces; this remembers which, so lesson 17 can select one face.
#[derive(Clone, Copy)]
pub struct FaceSource {
    pub parent: u32, // object row
    pub face: usize, // face index in that object
}

/// Where a GPU vertex lies on its surface, so a preview can evaluate the surface there again.
#[derive(Clone, Copy)]
pub struct Sample {
    pub index: u32,   // GPU vertex index
    pub surface: u32, // surface index in the BRep
    pub uv: [f64; 2], // parameter on that surface
    pub sign: f32,    // +1 or -1 on the normal
}
// --8<-- [end:arena-faces]

// --8<-- [start:18-arena-visibility]
// --8<-- [start:arena-visibility]
impl ArenaLane {
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
        self.tiles.encode(
            ctx,
            encoder,
            super::triangle_tiles::TileInput {
                binds,
                geometry: [&self.verts.buf, &self.vids.buf, &self.faces.buf],
                table: &self.table.view,
                matrix,
                objects_revision,
            },
            lists,
        );
    }
}
// --8<-- [end:arena-visibility]
// --8<-- [end:18-arena-visibility]
