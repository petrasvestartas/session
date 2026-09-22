use super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group, uniform_buffer};
use super::frame::Binds;
use super::upload::drop_rows;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, ink_module,
};
use std::collections::HashSet;
use wgpu::PrimitiveTopology::TriangleList;

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// Vertices per segment: two triangles, placed by the shader.
const RIBBON_VERTS: u32 = 6;

/// One line segment, 40 bytes, as the shaders read it.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment {
    pub p0: [f32; 3], // start point
    pub radius: f32, // 0 = pen width; > 0 = world mm
    pub p1: [f32; 3], // end point
    pub instance_id: u32, // object row
    pub color: u32, // packed rgba, red in the low byte
    pub facing: u32, // packed normals of the two faces beside it
}

const _: () = assert!(std::mem::size_of::<CylinderSegment>() == 40);

/// Segment rows of one upload.
#[derive(Default)]
pub struct SegRows {
    pub pipes: Vec<CylinderSegment>, // mesh and solid edges
    pub pipe_ids: Vec<u32>, // source edge per pipe, or u32::MAX
    pub pipe_chains: Vec<std::ops::Range<u32>>, // runs of pipes that form one curve
    pub ribbon_chains: Vec<std::ops::Range<u32>>, // runs of ribbons that form one curve
    pub ribbons: Vec<CylinderSegment>, // standalone lines and curves
}

impl SegRows {
    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.pipe_ids);
        drop_rows(&mut self.pipe_chains);
        drop_rows(&mut self.ribbon_chains);
        drop_rows(&mut self.ribbons);
    }
}

/// A segment with its neighbours, so joints are drawn once.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct StrokeSegment {
    pub(super) segment: CylinderSegment, // the segment
    pub(super) previous: u32, // row of the segment before it, or u32::MAX
    pub(super) next: u32, // row of the segment after it, or u32::MAX
}

/// Link neighbouring segments inside each chain.
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
            // a closed chain wraps around
            let next = if index + 1 == chain.end {
                chain.start
            } else {
                index + 1
            };
            let a = &rows[index as usize];
            let b = &rows[next as usize];

            // join only where the ends meet and look the same
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

/// One segment buffer, its ids and their bind group.
struct SegTable {
    label: &'static str, // name shown in GPU errors
    buf: GrowBuf, // StrokeSegment rows
    ids: GrowBuf, // source id per row
    group: wgpu::BindGroup, // group 3: rows, ids, selected edge
}

impl SegTable {
    /// An empty table and its bind group.
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

    /// Rebuild the bind group after a buffer moved.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts, selection: &wgpu::Buffer) {
        self.group = bind_group(
            ctx,
            &l.segment_rows,
            self.label,
            &[&self.buf.buf, &self.ids.buf, selection],
        );
    }
}

/// The nine segment pipelines.
struct SegPipelines {
    ribbon: wgpu::RenderPipeline, // plain lines in color
    unselected: wgpu::RenderPipeline, // unselected objects' lines
    selected: wgpu::RenderPipeline, // selected objects' lines
    id_ribbon: wgpu::RenderPipeline, // object ids
    id_edge: wgpu::RenderPipeline, // source edge ids
    // --8<-- [start:step-24a]
    mask_unselected: wgpu::RenderPipeline, // unselected edges into the solid mask
    mask_selected: wgpu::RenderPipeline, // selected edges into a mask
    masks_unselected: wgpu::RenderPipeline, // unselected edges into both masks
    masks_selected: wgpu::RenderPipeline, // selected edges into both masks
    // --8<-- [end:step-24a]
}

/// Lines on the GPU: edges as pipes, curves as ribbons.
pub struct SegmentLane {
    pipes: SegTable, // mesh and solid edges
    ribbons: SegTable, // standalone lines and curves
    shader: wgpu::ShaderModule, // ribbon shader
    gpu: SegPipelines, // pipelines
    selection: wgpu::Buffer, // selected edge (object, edge), read by shaders
    selected_rows: HashSet<u32>, // selected object rows
    selected_edge: bool, // an edge is selected
}

impl SegmentLane {
    /// Bytes reserved on the GPU by this lane.
    pub fn allocated_bytes(&self) -> u64 {
        self.pipes.buf.buf.size()
            + self.pipes.ids.buf.size()
            + self.ribbons.buf.buf.size()
            + self.ribbons.ids.buf.size()
            + self.selection.size()
    }

    /// Create the lane: shader, pipelines, empty tables.
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
        }
    }

    /// Rebuild the pipelines for a new MSAA sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shader, target);
    }

    /// Append one upload's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        // pad missing ids with u32::MAX
        let mut ids = up.pipe_ids.clone();
        ids.resize(up.pipes.len(), u32::MAX);
        let pipes = joined_rows(&up.pipes, &up.pipe_chains, self.pipes.buf.len());
        let pipes_changed = self.pipes.buf.append(ctx, &pipes);

        if self.pipes.ids.append(ctx, &ids) || pipes_changed {
            self.pipes.rebind(ctx, l, &self.selection);
        }

        let ribbons = joined_rows(&up.ribbons, &up.ribbon_chains, self.ribbons.buf.len());
        let ribbons_changed = self.ribbons.buf.append(ctx, &ribbons);

        if self
            .ribbons
            .ids
            .append(ctx, &vec![u32::MAX; up.ribbons.len()])
            || ribbons_changed
        {
            self.ribbons.rebind(ctx, l, &self.selection);
        }
    }

    /// Select one source edge (object, edge); None clears it.
    pub fn set_edge(&mut self, ctx: &GpuCtx, edge: Option<(u32, u32)>) {
        self.selected_edge = edge.is_some();
        let (parent, edge) = edge.unwrap_or((u32::MAX, u32::MAX));
        ctx.queue.write_buffer(
            &self.selection,
            0,
            bytemuck::cast_slice(&[parent, edge, 0, 0]),
        );
    }

    /// Remember whether object `row` is selected.
    pub fn set_selected(&mut self, row: u32, selected: bool) {
        if selected {
            self.selected_rows.insert(row);
        } else {
            self.selected_rows.remove(&row);
        }
    }

    /// Draw the unselected objects' lines.
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

    /// Draw the selected objects' lines.
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

    // --8<-- [start:step-24b]
    /// Draw every edge into the solid mask.
    pub fn draw_solid_mask(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.mask_unselected, &self.pipes)
            + self.draw_table(pass, b, &self.gpu.mask_selected, &self.pipes)
    }

    /// Draw the selected objects' edges into a mask.
    pub fn draw_selection_mask(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        if self.selected_rows.is_empty() {
            return 0;
        }

        self.draw_table(pass, b, &self.gpu.mask_selected, &self.pipes)
    }

    /// Draw every edge into both masks at once.
    pub fn draw_masks(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.masks_unselected, &self.pipes)
            + self.draw_table(pass, b, &self.gpu.masks_selected, &self.pipes)
    }

    /// Draw the edges in color.
// --8<-- [end:step-24b]
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.pipes)
    }

    /// Draw the lines and curves in color.
    pub fn draw_ribbons(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.ribbons)
    }

    /// Draw edge object ids.
    pub fn draw_pipe_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.pipes)
    }

    /// Draw source edge ids.
    pub fn draw_edge_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_edge, &self.pipes)
    }

    /// Draw line object ids.
    pub fn draw_ribbon_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.ribbons)
    }

    /// Draw one table with `pipeline`; returns the draw count.
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
        // six vertices per segment, placed by the shader
        pass.draw(0..RIBBON_VERTS * table.buf.len(), 0..1);
        1
    }

    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        self.selected_rows.clear();
        self.selected_edge = false;
        self.pipes.buf.reset();
        self.pipes.ids.reset();
        self.ribbons.buf.reset();
        self.ribbons.ids.reset();
    }

    /// Forget every row and free the buffers.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.selected_rows.clear();
        self.selected_edge = false;
        self.pipes.buf.release(ctx);
        self.pipes.ids.release(ctx);
        self.ribbons.buf.release(ctx);
        self.ribbons.ids.release(ctx);
        self.pipes.rebind(ctx, l, &self.selection);
        self.ribbons.rebind(ctx, l, &self.selection);
    }

    /// Pipe rows on the GPU.
    pub fn pipe_count(&self) -> u32 {
        self.pipes.buf.len()
    }

    /// Ribbon rows on the GPU.
    pub fn ribbon_count(&self) -> u32 {
        self.ribbons.buf.len()
    }
}

/// Build the nine segment pipelines.
fn build_pipelines(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &wgpu::ShaderModule,
    target: Target,
) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.segment_rows];
    // no vertex buffer, always drawn; the shader tests depth
    let quad = PipelineDesc::new(shader, &groups, &[], TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    // --8<-- [start:step-24c]
    // masks are one-channel textures
    let mask = Target {
        format: wgpu::TextureFormat::R8Unorm,
        samples: target.samples,
    };
    // --8<-- [end:step-24c]
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
        // --8<-- [start:step-24d]
        mask_unselected: build(
            dev,
            mask,
            &quad
                .with("ribbon.mask", "fs_mask")
                .vertex("vs_unselected")
                .color(ColorWrite::Max),
        ),
        mask_selected: build(
            dev,
            mask,
            &quad
                .with("ribbon.mask.selected", "fs_mask")
                .vertex("vs_selected")
                .color(ColorWrite::Max),
        ),
        masks_unselected: build(
            dev,
            mask,
            &quad
                .with("ribbon.masks", "fs_masks")
                .vertex("vs_unselected")
                .masks(),
        ),
        masks_selected: build(
            dev,
            mask,
            &quad
                .with("ribbon.masks.selected", "fs_masks_selected")
                .vertex("vs_selected")
                .masks(),
        ),
        // --8<-- [end:step-24d]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// The shader declares StrokeSegment with the Rust fields.
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
}
