use super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group, uniform_buffer};
use super::frame::Binds;
use super::upload::drop_rows;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, ink_module,
};
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
    pub ribbons: Vec<CylinderSegment>, // standalone lines and curves
}

impl SegRows {
    /// Empty every table and free its memory.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.pipe_ids);
        drop_rows(&mut self.ribbons);
    }
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
            std::mem::size_of::<CylinderSegment>() as u64,
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
    id_ribbon: wgpu::RenderPipeline, // object ids
    id_edge: wgpu::RenderPipeline, // source edge ids
}

/// Lines on the GPU: edges as pipes, curves as ribbons.
pub struct SegmentLane {
    pipes: SegTable, // mesh and solid edges
    ribbons: SegTable, // standalone lines and curves
    shader: wgpu::ShaderModule, // ribbon shader
    gpu: SegPipelines, // pipelines
    selection: wgpu::Buffer, // selected edge (object, edge), read by shaders
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
        let pipes_changed = self.pipes.buf.append(ctx, &up.pipes);

        if self.pipes.ids.append(ctx, &ids) || pipes_changed {
            self.pipes.rebind(ctx, l, &self.selection);
        }

        let ribbons_changed = self.ribbons.buf.append(ctx, &up.ribbons);

        if self
            .ribbons
            .ids
            .append(ctx, &vec![u32::MAX; up.ribbons.len()])
            || ribbons_changed
        {
            self.ribbons.rebind(ctx, l, &self.selection);
        }
    }

    /// Highlight one edge without reuploading it.
    pub fn set_edge(&self, ctx: &GpuCtx, edge: Option<(u32, u32)>) {
        let (parent, edge) = edge.unwrap_or((u32::MAX, u32::MAX));
        ctx.queue.write_buffer(
            &self.selection,
            0,
            bytemuck::cast_slice(&[parent, edge, 0, 0]),
        );
    }

    /// Draw the edges in color.
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
        self.pipes.buf.reset();
        self.pipes.ids.reset();
        self.ribbons.buf.reset();
        self.ribbons.ids.reset();
    }

    /// Forget every row and free the buffers.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
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
    let dev = &ctx.device;

    SegPipelines {
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
        ];

        for (name, src) in SHADERS {
            assert_eq!(
                wgsl_fields(src, "CylinderSegment"),
                rust,
                "{name}: CylinderSegment fields"
            );
        }

        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
        assert_eq!(std::mem::offset_of!(CylinderSegment, facing), 36);
    }
}
