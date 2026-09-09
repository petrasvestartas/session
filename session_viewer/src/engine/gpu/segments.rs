//! The segment lane: every straight piece of ink. Two tables of the same connected stroke row - pipes
//! (mesh/BRep edges, the SOLID lane, culled by facing) and ribbons (line/polyline/curve, the
//! FLAT lane, always drawn) - through one blended camera-facing quad. `SegRows` is one upload.
//! A streamed sheet's ribbons arrive in CHUNKS interleaved with other uploads, so the lane
//! maps a global ribbon row back to (sheet row, segment index) through the sheet's chunk list.

use super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group, uniform_buffer};
use super::frame::Binds;
use super::upload::drop_rows;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, ink_module,
};
use std::collections::HashSet;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// Vertices per ribbon: two triangles pulled by vertex index, no vertex buffer.
const RIBBON_VERTS: u32 = 6;

/// One source segment row, 40 B. Upload adds two GPU neighbor indices. The ends are flat f32s: a `vec3`
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

/// One upload's contribution to a sheet: segments `[from, from + count)` of the sheet on
/// object row `instance`, landing at upload-local ribbon row `first`. `from == 0` opens the
/// sheet; a later `from` extends the one already open on the same row.
pub struct SegDraw {
    pub instance: u32,
    pub from: u32,
    pub count: u32,
    pub first: u32,
}

/// One upload's segments: the solid lane's pipes and the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    pub pipes: Vec<CylinderSegment>,
    /// Source edge index per pipe, or u32::MAX when no CAD edge identity is available.
    pub pipe_ids: Vec<u32>,
    /// Source curve ranges in this upload; independent mesh wires remain unjoined.
    pub pipe_chains: Vec<std::ops::Range<u32>>,
    /// Consecutive spans belonging to one source polyline or NURBS curve.
    pub ribbon_chains: Vec<std::ops::Range<u32>>,
    pub ribbons: Vec<CylinderSegment>,
    /// Source entity id per ribbon, or u32::MAX; shorter than `ribbons` means no identity.
    pub ribbon_ids: Vec<u32>,
    /// The sheet slices among this upload's ribbons.
    pub sheets: Vec<SegDraw>,
}

impl SegRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.pipe_ids);
        drop_rows(&mut self.pipe_chains);
        drop_rows(&mut self.ribbon_chains);
        drop_rows(&mut self.ribbons);
        drop_rows(&mut self.ribbon_ids);
        drop_rows(&mut self.sheets);
    }
}

/// Segments `[from, to)` of a sheet, resident at global ribbon rows starting at `row`.
#[derive(Clone, Copy)]
pub struct SegChunk {
    pub from: u32,
    pub to: u32,
    pub row: u32,
}

/// One sheet as the lane knows it: its object row, the chunks resident so far (contiguous
/// from 0) and the entity id of every resident segment, for picks.
pub struct SegSheet {
    pub instance: u32,
    pub resident: u32,
    pub chunks: Vec<SegChunk>,
    pub ids: Vec<u32>,
}

/// Open a sheet, or add a chunk to the one on its object row; a chunk that does not continue
/// the resident prefix is dropped with a warning (the walk cannot address it).
fn push_chunk(sheets: &mut Vec<SegSheet>, instance: u32, chunk: SegChunk, ids: &[u32]) {
    if chunk.from == 0 {
        sheets.push(SegSheet {
            instance,
            resident: chunk.to,
            chunks: vec![chunk],
            ids: ids.to_vec(),
        });
        return;
    }
    for sheet in sheets.iter_mut() {
        if sheet.instance != instance {
            continue;
        }
        if chunk.from != sheet.resident {
            log::warn!(
                "sheet chunk [{}, {}) does not continue the {} resident segments; dropped",
                chunk.from,
                chunk.to,
                sheet.resident
            );
            return;
        }
        sheet.resident = chunk.to;
        sheet.chunks.push(chunk);
        sheet.ids.extend_from_slice(ids);
        return;
    }
    log::warn!("sheet chunk for row {instance} arrived before its sheet; dropped");
}

/// Which sheet a global ribbon row belongs to: (sheet index, segment index within it).
fn sheet_of(sheets: &[SegSheet], row: u32) -> Option<(usize, u32)> {
    for (index, sheet) in sheets.iter().enumerate() {
        for k in &sheet.chunks {
            if row >= k.row && row < k.row + (k.to - k.from) {
                return Some((index, k.from + (row - k.row)));
            }
        }
    }
    None
}

/// GPU connectivity partitions the shared rounded cap instead of blending it twice.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct StrokeSegment {
    pub(super) segment: CylinderSegment,
    pub(super) previous: u32,
    pub(super) next: u32,
}

/// Only explicitly connected source chains join; separate wires keep their own end caps.
fn joined_rows(
    rows: &[CylinderSegment],
    chains: &[std::ops::Range<u32>],
    base: u32,
) -> Vec<StrokeSegment> {
    let mut result = Vec::with_capacity(rows.len());
    for segment in rows {
        result.push(StrokeSegment {
            segment: *segment,
            previous: u32::MAX,
            next: u32::MAX,
        });
    }
    for chain in chains {
        if chain.end > rows.len() as u32 || chain.end.saturating_sub(chain.start) < 2 {
            continue;
        }
        for index in chain.clone() {
            let next = if index + 1 == chain.end {
                chain.start
            } else {
                index + 1
            };
            let a = &rows[index as usize];
            let b = &rows[next as usize];
            if a.p1 == b.p0
                && a.instance_id == b.instance_id
                && a.color == b.color
                && a.radius == b.radius
            {
                result[index as usize].next = next + base;
                result[next as usize].previous = index + base;
            }
        }
    }
    result
}

/// One segment table on the GPU with the group 3 that binds it.
struct SegTable {
    label: &'static str,
    buf: GrowBuf,
    ids: GrowBuf,
    group: wgpu::BindGroup,
}

impl SegTable {
    /// A one-row table and its bind group.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str, selection: &wgpu::Buffer) -> Self {
        let buf = GrowBuf::new(
            ctx,
            label,
            std::mem::size_of::<StrokeSegment>() as u64,
            ROWS,
        );
        let ids = GrowBuf::new(ctx, "segment.sources", 4, ROWS);
        let group = bind_group(
            ctx,
            &l.segment_rows,
            label,
            &[&buf.buf, &ids.buf, selection],
        );
        Self {
            label,
            buf,
            ids,
            group,
        }
    }

    /// Rebind after the backing buffer changed.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts, selection: &wgpu::Buffer) {
        self.group = bind_group(
            ctx,
            &l.segment_rows,
            self.label,
            &[&self.buf.buf, &self.ids.buf, selection],
        );
    }
}

/// The pipelines over the two tables: the same blended quad for both, and its id twin.
struct SegPipelines {
    ribbon: wgpu::RenderPipeline,
    unselected: wgpu::RenderPipeline,
    selected: wgpu::RenderPipeline,
    id_ribbon: wgpu::RenderPipeline,
    id_edge: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: two tables, the shader, the pipelines, the sheets.
pub struct SegmentLane {
    pipes: SegTable,
    ribbons: SegTable,
    shader: wgpu::ShaderModule,
    gpu: SegPipelines,
    selection: wgpu::Buffer,
    selected_rows: HashSet<u32>,
    selected_edge: bool,
    sheets: Vec<SegSheet>,
}

impl SegmentLane {
    /// Application-owned buffer allocation capacity in bytes; excludes driver overhead.
    pub fn allocated_bytes(&self) -> u64 {
        self.pipes.buf.buf.size()
            + self.pipes.ids.buf.size()
            + self.ribbons.buf.buf.size()
            + self.ribbons.ids.buf.size()
            + self.selection.size()
    }

    /// Two one-row tables, the shader and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = ink_module(
            &ctx.device,
            "ribbon.shader",
            include_str!("../../shaders/ribbon.wgsl"),
        );
        let gpu = build_pipelines(ctx, l, &shader, target);
        let selection = uniform_buffer(&ctx.device, "edge.selection", &[u32::MAX; 4]);
        let pipes = SegTable::new(ctx, l, "pipes", &selection);
        let ribbons = SegTable::new(ctx, l, "ribbons", &selection);
        Self {
            pipes,
            ribbons,
            shader,
            gpu,
            selection,
            selected_rows: HashSet::new(),
            selected_edge: false,
            sheets: Vec::new(),
        }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shader, target);
    }

    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        let mut ids = up.pipe_ids.clone();
        ids.resize(up.pipes.len(), u32::MAX);
        let pipes = joined_rows(&up.pipes, &up.pipe_chains, self.pipes.buf.len());
        let pipes_changed = self.pipes.buf.append(ctx, &pipes);
        if self.pipes.ids.append(ctx, &ids) || pipes_changed {
            self.pipes.rebind(ctx, l, &self.selection);
        }
        let ribbon_base = self.ribbons.buf.len();
        let mut ribbon_ids = up.ribbon_ids.clone();
        ribbon_ids.resize(up.ribbons.len(), u32::MAX);
        let ribbons = joined_rows(&up.ribbons, &up.ribbon_chains, ribbon_base);
        let ribbons_changed = self.ribbons.buf.append(ctx, &ribbons);
        if self.ribbons.ids.append(ctx, &ribbon_ids) || ribbons_changed {
            self.ribbons.rebind(ctx, l, &self.selection);
        }
        for d in &up.sheets {
            let Some(ids) = ribbon_ids.get(d.first as usize..(d.first + d.count) as usize) else {
                continue;
            };
            let chunk = SegChunk {
                from: d.from,
                to: d.from + d.count,
                row: ribbon_base + d.first,
            };
            push_chunk(&mut self.sheets, d.instance, chunk, ids);
        }
    }

    /// Which sheet a global ribbon row belongs to: (object row, segment index within it).
    pub fn row_of(&self, row: u32) -> Option<(u32, u32)> {
        let (index, local) = sheet_of(&self.sheets, row)?;
        Some((self.sheets[index].instance, local))
    }

    /// The entity id of the segment at a global ribbon row; `None` off every sheet.
    pub fn source_id(&self, row: u32) -> Option<u32> {
        let (index, local) = sheet_of(&self.sheets, row)?;
        self.sheets[index].ids.get(local as usize).copied()
    }

    /// Highlight one source edge without reuploading its tessellation or the parent geometry.
    pub fn set_edge(&mut self, ctx: &GpuCtx, edge: Option<(u32, u32)>) {
        self.selected_edge = edge.is_some();
        let (parent, edge) = edge.unwrap_or((u32::MAX, u32::MAX));
        ctx.queue.write_buffer(
            &self.selection,
            0,
            bytemuck::cast_slice(&[parent, edge, 0, 0]),
        );
    }

    /// Skip the final selection pass when no source object or edge is selected.
    pub fn set_selected(&mut self, row: u32, selected: bool) {
        if selected {
            self.selected_rows.insert(row);
        } else {
            self.selected_rows.remove(&row);
        }
    }

    /// Ordinary scene ink precedes silhouettes and the selected stroke overlay.
    pub fn draw_unselected(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipes: bool,
        ribbons: bool,
    ) -> u32 {
        let mut draws = 0;
        if pipes {
            draws += self.draw_table(pass, b, &self.gpu.unselected, &self.pipes);
        }
        if ribbons {
            draws += self.draw_table(pass, b, &self.gpu.unselected, &self.ribbons);
        }
        draws
    }

    /// Selected ink wins coincident stroke coverage, while still testing physical occlusion.
    pub fn draw_selected(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipes: bool,
        ribbons: bool,
    ) -> u32 {
        if self.selected_rows.is_empty() && !self.selected_edge {
            return 0;
        }
        let mut draws = 0;
        if pipes {
            draws += self.draw_table(pass, b, &self.gpu.selected, &self.pipes);
        }
        if ribbons {
            draws += self.draw_table(pass, b, &self.gpu.selected, &self.ribbons);
        }
        draws
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

    /// Ctrl selection includes only edges with original source identity.
    pub fn draw_edge_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_edge, &self.pipes)
    }

    /// The id pass for the flat lane: opaque quads.
    pub fn draw_ribbon_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.ribbons)
    }

    /// One table as ribbons through `pipeline`; 0 draws when empty.
    fn draw_table(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &wgpu::RenderPipeline,
        table: &SegTable,
    ) -> u32 {
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
        self.selected_rows.clear();
        self.selected_edge = false;
        self.sheets.clear();
        self.pipes.buf.reset();
        self.pipes.ids.reset();
        self.ribbons.buf.reset();
        self.ribbons.ids.reset();
    }

    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.selected_rows.clear();
        self.selected_edge = false;
        self.sheets = Vec::new();
        self.pipes.buf.release(ctx);
        self.pipes.ids.release(ctx);
        self.ribbons.buf.release(ctx);
        self.ribbons.ids.release(ctx);
        self.pipes.rebind(ctx, l, &self.selection);
        self.ribbons.rebind(ctx, l, &self.selection);
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
fn build_pipelines(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.segment_rows];
    let quad = PipelineDesc::new(shader, &groups, &[], TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    let dev = &ctx.device;

    SegPipelines {
        unselected: build(
            dev,
            target,
            &quad
                .with("ribbon.unselected", "fs_main")
                .vertex("vs_unselected")
                .color(ColorWrite::Blended),
        ),
        selected: build(
            dev,
            target,
            &quad
                .with("ribbon.selected", "fs_main")
                .vertex("vs_selected")
                .color(ColorWrite::Blended),
        ),
        ribbon: build(
            dev,
            target,
            &quad.with("ribbon", "fs_main").color(ColorWrite::Blended),
        ),
        id_ribbon: build(
            dev,
            Target::ID,
            &quad.with("ribbon.id", "fs_id").scene_samples(1),
        ),
        id_edge: build(
            dev,
            Target::ID,
            &quad.with("edge.id", "fs_edge_id").scene_samples(1),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// ribbon.wgsl reads the 48 B connected GPU row (ends as scalars).
    #[test]
    fn cylinder_segment_mirror() {
        let rust = [
            "p0x",
            "p0y",
            "p0z",
            "radius",
            "p1x",
            "p1y",
            "p1z",
            "instance_id",
            "color",
            "facing",
            "previous",
            "next",
        ];
        for (name, src) in SHADERS {
            assert_eq!(
                wgsl_fields(src, "StrokeSegment"),
                rust,
                "{name}: StrokeSegment fields"
            );
        }
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
        assert_eq!(std::mem::size_of::<StrokeSegment>(), 48);
        assert_eq!(std::mem::offset_of!(CylinderSegment, facing), 36);
    }

    /// Chunks interleaved with other uploads still map global rows to sheet segments and
    /// their ids; a chunk that skips ahead is dropped, one for an unknown sheet too.
    #[test]
    fn sheet_chunks_map_global_ribbon_rows_to_segments_and_ids() {
        let mut sheets = Vec::new();
        let chunk = |from, to, row| SegChunk { from, to, row };
        push_chunk(&mut sheets, 7, chunk(0, 3, 10), &[100, 101, 102]);
        push_chunk(&mut sheets, 9, chunk(0, 1, 13), &[u32::MAX]);
        push_chunk(&mut sheets, 7, chunk(3, 5, 20), &[103, 104]);
        push_chunk(&mut sheets, 7, chunk(6, 8, 30), &[9, 9]);
        push_chunk(&mut sheets, 8, chunk(2, 4, 40), &[9, 9]);
        assert_eq!(sheets.len(), 2);
        assert_eq!(sheets[0].resident, 5);
        assert_eq!(sheet_of(&sheets, 12), Some((0, 2)));
        assert_eq!(sheet_of(&sheets, 13), Some((1, 0)));
        assert_eq!(sheet_of(&sheets, 21), Some((0, 4)));
        assert_eq!(sheet_of(&sheets, 9), None);
        assert_eq!(sheet_of(&sheets, 22), None);
        assert_eq!(sheet_of(&sheets, 30), None);
        assert_eq!(sheets[0].ids[4], 104);
        assert_eq!(sheets[1].ids[0], u32::MAX);
    }
}
