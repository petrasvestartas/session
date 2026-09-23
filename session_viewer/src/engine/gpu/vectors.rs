use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::lane::{Lane, Registered, RowLane};
use super::pick::PickMode;
use super::upload::Upload;
use super::view::View;
use crate::engine::pipelines::{
    build, ink_module, ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target,
};
use wgpu::PrimitiveTopology::TriangleList;

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("vector.wgsl", include_str!("../../shaders/vector.wgsl"))];

/// Vertices per vector: a shaft quad and a head quad.
const VECTOR_VERTS: u32 = 12;

/// One vector, a shaft from `start` with an arrowhead at `end`, as the shader reads it.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VectorRow {
    // offset size
    pub start: [f32; 3],  //   0   12  tail, object space
    pub radius: f32,      //  12    4  0 = pen; > 0 world mm; < 0 pen multiplier
    pub end: [f32; 3],    //  16   12  tip, object space
    pub instance_id: u32, //  28    4  object row
    pub color: u32,       //  32    4  packed rgba, red in the low byte
    pub head: f32,        //  36    4  arrowhead length in pens; 0 = default
    pub pad: [u32; 2],    //  40    8  -> 48, a multiple of 16
}

const _: () = assert!(std::mem::size_of::<VectorRow>() == 48);

/// Vector rows of one upload; producers write them with `up.lanes.get_mut::<VectorRows>()`.
#[derive(Default)]
pub struct VectorRows {
    pub rows: Vec<VectorRow>, // one per vector
}

/// Vectors on the GPU: one instanced draw for all of them.
pub struct VectorLane {
    buf: GrowBuf,                // VectorRow rows
    group: wgpu::BindGroup,      // group 3, binds the rows
    shader: Shader,  // vector shader
    color: Pipeline, // arrows in color
    id: Pipeline,    // arrow object ids
}

/// The registry's entry: constructor, row count and merge.
pub const REGISTERED: Registered = Registered {
    make,
    rows_in,
    merge,
    stride: std::mem::size_of::<VectorRow>() as u64,
};

/// The registry's constructor.
fn make(ctx: &GpuCtx, l: &Layouts, target: Target) -> Box<dyn RowLane> {
    Box::new(VectorLane::new(ctx, l, target))
}

/// Vector rows in one upload.
fn rows_in(up: &Upload) -> u32 {
    up.lanes.get::<VectorRows>().map_or(0, |rows| rows.rows.len() as u32)
}

/// Move the vector rows of `other` after those of `up`.
fn merge(up: &mut Upload, other: &mut Upload) {
    if let Some(rows) = other.lanes.take::<VectorRows>() {
        up.lanes.get_mut::<VectorRows>().rows.extend(rows.rows);
    }
}

impl VectorLane {
    /// Create the lane: shader, pipelines, an empty table.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = ink_module(
            ctx,
            "vector.shader",
            include_str!("../../shaders/vector.wgsl"),
        );
        let (color, id) = build_pipelines(ctx, l, &shader, target);
        let buf = GrowBuf::new(
            ctx,
            "vectors",
            std::mem::size_of::<VectorRow>() as u64,
            ROWS,
        );
        let group = bind_group(ctx, &l.ink_rows, "vectors", &[&buf.buf]);
        Self {
            buf,
            group,
            shader,
            color,
            id,
        }
    }

    /// Rebuild the bind group after the buffer moved.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.group = bind_group(ctx, &l.ink_rows, "vectors", &[&self.buf.buf]);
    }

    /// Draw every vector with `pipeline`; returns the draw count.
    fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        pipeline: &Pipeline,
    ) -> u32 {
        if self.buf.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.group, &[]);
        // twelve corners per vector, one instance per row
        pass.draw(0..VECTOR_VERTS, 0..self.buf.len());
        1
    }
}

impl Lane for VectorLane {
    fn on_retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        (self.color, self.id) = build_pipelines(ctx, layouts, &self.shader, target);
    }

    fn on_reset(&mut self, _ctx: &GpuCtx) {
        self.buf.reset();
    }

    fn on_release(&mut self, ctx: &GpuCtx, layouts: &Layouts) {
        self.buf.release(ctx);
        self.rebind(ctx, layouts);
    }

    fn bytes(&self) -> (u64, u64) {
        (self.buf.buf.size(), 0)
    }

    fn on_append(&mut self, ctx: &GpuCtx, layouts: &Layouts, up: &Upload) {
        let Some(rows) = up.lanes.get::<VectorRows>() else {
            return;
        };

        if self.buf.append(ctx, &rows.rows) {
            self.rebind(ctx, layouts);
        }
    }

    fn draw_ink(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, view: &View) -> u32 {
        if !view.show_lines {
            return 0;
        }

        self.draw(pass, b, &self.color)
    }

    fn reads_tiles(&self, view: &View) -> bool {
        view.show_lines && !self.buf.is_empty()
    }

    fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        view: &View,
        mode: PickMode,
    ) -> u32 {
        if mode != PickMode::Object || !view.show_lines {
            return 0;
        }

        self.draw(pass, b, &self.id)
    }
}

impl RowLane for VectorLane {
    fn write_at(&mut self, ctx: &GpuCtx, _l: &Layouts, first: u32, up: &Upload) {
        if let Some(rows) = up.lanes.get::<VectorRows>() {
            self.buf.write_at(ctx, first, &rows.rows);
        }
    }

    fn kill(&mut self, ctx: &GpuCtx, first: u32, count: u32, sink: u32) {
        let dead = VectorRow {
            instance_id: sink,
            ..bytemuck::Zeroable::zeroed()
        };
        self.buf.fill(ctx, first, count, &dead);
    }
}

/// Build the color and id pipelines.
fn build_pipelines(
    ctx: &GpuCtx,
    l: &Layouts,
    shader: &Shader,
    target: Target,
) -> (Pipeline, Pipeline) {
    let groups = [&l.mvp, &l.line, &l.ink_instance, &l.ink_rows];
    // no vertex buffer, always drawn; the shader tests depth
    let quad = PipelineDesc::new(shader, &groups, &[], TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    let color = build(
        ctx,
        target,
        &quad.with("vector", "fs_main").color(ColorWrite::Blended),
    );
    let id = build(
        ctx,
        Target::ID,
        &quad.with("vector.id", "fs_id").scene_samples(1),
    );
    (color, id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// A pen-wide vector from `start` to `end` on object `row`, default head.
    fn arrow(row: u32, start: [f32; 3], end: [f32; 3], color: [f32; 4]) -> VectorRow {
        let byte = |v: f32| ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff;
        VectorRow {
            start,
            radius: 0.0,
            end,
            instance_id: row,
            color: byte(color[0])
                | byte(color[1]) << 8
                | byte(color[2]) << 16
                | byte(color[3]) << 24,
            head: 0.0,
            pad: [0; 2],
        }
    }

    /// The shader declares VectorRow with the Rust fields at the Rust offsets.
    #[test]
    fn vector_row_mirror() {
        use std::mem::{offset_of, size_of};
        let rust = [
            "start",
            "radius",
            "end",
            "instance_id",
            "color",
            "head",
            "pad",
        ];
        let (_, src) = SHADERS[0];
        assert_eq!(wgsl_fields(src, "VectorRow"), rust, "VectorRow fields");

        let source = crate::engine::pipelines::shared(&format!(
            "{src}\n{}\n{}\n{}",
            crate::engine::pipelines::INK,
            crate::engine::pipelines::SCENE,
            crate::engine::pipelines::CLIP
        ));
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        let (_, ty) = module
            .types
            .iter()
            .find(|(_, ty)| ty.name.as_deref() == Some("VectorRow"))
            .expect("VectorRow in the shader");
        let naga::TypeInner::Struct { members, span } = &ty.inner else {
            panic!("VectorRow is not a struct")
        };
        let offsets = [
            offset_of!(VectorRow, start),
            offset_of!(VectorRow, radius),
            offset_of!(VectorRow, end),
            offset_of!(VectorRow, instance_id),
            offset_of!(VectorRow, color),
            offset_of!(VectorRow, head),
            offset_of!(VectorRow, pad),
        ];
        assert_eq!(*span as usize, size_of::<VectorRow>());
        assert_eq!(members.len(), offsets.len());

        for (member, expected) in members.iter().zip(offsets) {
            assert_eq!(
                member.offset as usize, expected,
                "VectorRow.{:?}",
                member.name
            );
        }
    }

    /// Colors pack red in the low byte, like the other lanes.
    #[test]
    fn vector_row_color() {
        let row = arrow(3, [0.0; 3], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(row.color, 0xff00_00ff);
        assert_eq!(row.instance_id, 3);
    }

    /// Draws, highlights, hides, picks and releases one vector on a headless device.
    #[test]
    fn vector_renders_selects_hides_and_picks() {
        use crate::camera::Camera;
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow};
        use session_rust::{Xform, AABB};

        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(256, 256)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.show_grid = false;
        let mut up = Upload::default();
        let mut bounds = AABB::empty();
        bounds.union_with_point(0.0, 0.0, 0.0);
        bounds.union_with_point(100.0, 100.0, 0.0);
        let mut row = ObjectRow::new(Xform::identity(), 0);
        row.bounds = bounds;
        up.obj.rows.push(row);
        up.bounds = bounds;
        let vector = arrow(
            0,
            [0.0, 0.0, 0.0],
            [100.0, 100.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );
        up.lanes.get_mut::<VectorRows>().rows.push(vector);
        gpu.set_scene(&up);

        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, 1.0);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let ink = |rgba: &[u8]| {
            rgba.chunks_exact(4)
                .filter(|p| p[0] < 128 && p[1] < 128)
                .count()
        };
        let yellow = |rgba: &[u8]| {
            rgba.chunks_exact(4)
                .filter(|p| p[0] > 200 && p[1] > 200 && p[2] < 100)
                .count()
        };

        for samples in [1, 4] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(256, 256);
            let plain = gpu.render_offscreen(&input);
            let drawn = ink(&plain);
            assert!(drawn > 60, "{samples}x: the arrow draws ink: {drawn}");

            gpu.set_selected(0, true);
            let selected = gpu.render_offscreen(&input);
            assert!(
                yellow(&selected) > 60,
                "{samples}x: a selected arrow is yellow"
            );
            gpu.set_selected(0, false);

            gpu.set_hidden(0, true);
            let hidden = gpu.render_offscreen(&input);
            assert_eq!(ink(&hidden), 0, "{samples}x: a hidden arrow draws nothing");
            gpu.set_hidden(0, false);
        }

        let ids = gpu.render_ids_offscreen(&input);
        let hits = ids.iter().filter(|id| id[0] == 1).count();
        assert!(hits > 30, "the id pass answers row 0: {hits}");

        let tagged = ids
            .iter()
            .filter(|id| id[0] == 1 && id[1] == 0x4000_0000)
            .count();
        assert_eq!(tagged, hits, "vector picks carry the marker tag");

        // a stub head draws less ink than the default one
        let with_head = ink(&gpu.render_offscreen(&input));
        gpu.reset();
        up.lanes.get_mut::<VectorRows>().rows[0].head = 0.1;
        gpu.set_scene(&up);
        let stub = ink(&gpu.render_offscreen(&input));
        assert!(
            with_head > stub + 15,
            "the head adds ink: {with_head} vs {stub}"
        );

        gpu.release();
        let empty = gpu.render_offscreen(&input);
        assert_eq!(ink(&empty), 0, "released vectors are gone");
    }
}
