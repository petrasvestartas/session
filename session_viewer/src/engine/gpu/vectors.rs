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
pub const SHADERS: &[(&str, &str)] = &[("vector.wgsl", shader!("vector.wgsl"))];

/// Vertices per vector: a shaft quad and a quad per head.
const VECTOR_VERTS: u32 = 18;

/// One vector as the shader reads it: a head's tip lands on its point, a bare end is cut flat there.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VectorRow {
    // offset size
    pub start: [f32; 3],  //   0   12  start point, object space
    pub radius: f32,      //  12    4  0 = pen; > 0 world mm; < 0 pen multiplier
    pub end: [f32; 3],    //  16   12  end point, object space
    pub instance_id: u32, //  28    4  object row
    pub color: u32,       //  32    4  packed rgba, red in the low byte
    pub head: f32,        //  36    4  arrowhead length in pens; 0 = default
    pub heads: u32,       //  40    4  HEAD_END | HEAD_START | HEAD_ONLY
    pub pad: u32,         //  44    4  -> 48, a multiple of 16
}

impl VectorRow {
    pub const HEAD_END: u32 = 1; // a head whose tip is `end`
    pub const HEAD_START: u32 = 2; // a head whose tip is `start`
    pub const HEAD_ONLY: u32 = 4; // no shaft: `start` only aims the heads, e.g. a curve's end tangent
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
            shader!("vector.wgsl"),
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
        // eighteen corners per vector, one instance per row
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
            heads: VectorRow::HEAD_END,
            pad: 0,
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
            "heads",
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
            offset_of!(VectorRow, heads),
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

    /// How much of a pixel ink covers, from its darkest channel over the empty frame's; both sRGB.
    fn covered(pixel: &[u8], paper: &[u8]) -> f64 {
        let linear = |c: &[u8]| {
            let v = f64::from(c[0].min(c[1]).min(c[2])) / 255.0;

            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        (1.0 - linear(pixel) / linear(paper).max(1e-6)).clamp(0.0, 1.0)
    }

    /// How far one arrow's ink reaches past each end point, along the arrow, in px.
    #[derive(Clone, Copy)]
    struct Reach {
        far: f64,  // center of the farthest inked pixel
        edge: f64, // the ink's edge: a pixel of coverage c has it c - 0.5 px past its center
    }

    /// Reach of the ink in `rgba` over the empty frame `paper` past `b` (along a to b) and past `a` (back).
    fn reach(rgba: &[u8], paper: &[u8], width: usize, a: [f64; 2], b: [f64; 2]) -> [Reach; 2] {
        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
        let len = (dx * dx + dy * dy).sqrt();
        let dir = [dx / len, dy / len];
        let none = Reach {
            far: f64::NEG_INFINITY,
            edge: f64::NEG_INFINITY,
        };
        let mut out = [none; 2];

        for (i, (p, q)) in rgba.chunks_exact(4).zip(paper.chunks_exact(4)).enumerate() {
            let dark = |c: &[u8]| f64::from(c[0].min(c[1]).min(c[2]));

            // ink: a visible step darker than the empty frame
            if dark(q) - dark(p) < 3.0 {
                continue;
            }

            let covered = covered(p, q);
            let x = (i % width) as f64 + 0.5;
            let y = (i / width) as f64 + 0.5;
            let past = [
                (x - b[0]) * dir[0] + (y - b[1]) * dir[1],
                (a[0] - x) * dir[0] + (a[1] - y) * dir[1],
            ];

            for (end, t) in out.iter_mut().zip(past) {
                end.far = end.far.max(t);
                end.edge = end.edge.max(t + covered - 0.5);
            }
        }

        out
    }

    /// Least ink along the center line from `a` to `b`, sampled every 0.1 px between `from` and `to` px after `a`.
    fn thinnest(
        rgba: &[u8],
        paper: &[u8],
        width: usize,
        a: [f64; 2],
        b: [f64; 2],
        from: f64,
        to: f64,
    ) -> f64 {
        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
        let len = (dx * dx + dy * dy).sqrt();
        let at = |x: usize, y: usize| {
            covered(&rgba[(y * width + x) * 4..], &paper[(y * width + x) * 4..])
        };
        let mut least: f64 = 1.0;
        let mut t = from;

        while t <= to {
            // bilinear between the four pixel centers around the point
            let x = a[0] + dx / len * t - 0.5;
            let y = a[1] + dy / len * t - 0.5;
            let (i, j) = (x.floor() as usize, y.floor() as usize);
            let (u, v) = (x - x.floor(), y - y.floor());
            let ink = at(i, j) * (1.0 - u) * (1.0 - v)
                + at(i + 1, j) * u * (1.0 - v)
                + at(i, j + 1) * (1.0 - u) * v
                + at(i + 1, j + 1) * u * v;
            least = least.min(ink);
            t += 0.1;
        }

        least
    }

    /// Where the drawn tip sits past `tip` along `dir`, px: the shift that best fits the ink within `r` to an ideal head.
    fn tip_offset(
        rgba: &[u8],
        paper: &[u8],
        width: usize,
        tip: [f64; 2],
        dir: [f64; 2],
        r: f64,
    ) -> f64 {
        let side = (1.0 + 0.16_f64).sqrt();
        let mut near = Vec::new(); // (u, v, coverage) per pixel, apex at the point

        for (i, (p, q)) in rgba.chunks_exact(4).zip(paper.chunks_exact(4)).enumerate() {
            let x = (i % width) as f64 + 0.5 - tip[0];
            let y = (i / width) as f64 + 0.5 - tip[1];

            if x * x + y * y > r * r {
                continue;
            }

            near.push((
                x * dir[0] + y * dir[1],
                (x * dir[1] - y * dir[0]).abs(),
                covered(p, q),
            ));
        }

        let misfit = |shift: f64| -> f64 {
            near.iter()
                .map(|(u, v, covered)| {
                    let u = u - shift;
                    // past the apex's normal cone: the apex is nearest, else the side line
                    let along_side = (-u + 0.4 * v) / side;
                    let distance = if along_side >= 0.0 {
                        (0.4 * u + v) / side
                    } else {
                        (u * u + v * v).sqrt()
                    };
                    let model = (0.5 - distance).clamp(0.0, 1.0);
                    (model - covered) * (model - covered)
                })
                .sum()
        };

        (-300..=300)
            .map(|k| f64::from(k) * 0.005)
            .min_by(|a, b| misfit(*a).total_cmp(&misfit(*b)))
            .unwrap_or(0.0)
    }

    /// The head's tip lands on the end point and nothing reaches past either end.
    #[test]
    fn arrow_is_exactly_as_long_as_its_line() {
        use crate::camera::Camera;
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow};
        use session_rust::{Point, Xform, AABB};

        const SIZE: u32 = 256;
        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(SIZE, SIZE)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.show_grid = false;
        let mut bounds = AABB::empty();
        bounds.union_with_point(-100.0, -100.0, 0.0);
        bounds.union_with_point(100.0, 100.0, 0.0);
        let row = |start: [f32; 3], end: [f32; 3], radius: f32, heads: u32| VectorRow {
            start,
            radius,
            end,
            color: 0xff00_0000,
            heads,
            ..bytemuck::Zeroable::zeroed()
        };
        let (one, both) = (
            VectorRow::HEAD_END,
            VectorRow::HEAD_END | VectorRow::HEAD_START,
        );
        // name, row, selected
        let cases = [
            (
                "thin",
                row([-70.0, -40.0, 0.0], [60.0, 35.0, 0.0], 0.0, one),
                false,
            ),
            (
                "thick",
                row([-70.0, 40.0, 0.0], [65.0, -30.0, 0.0], 6.0, one),
                false,
            ),
            (
                "selected",
                row([-60.0, -60.0, 0.0], [40.0, 70.0, 0.0], 0.0, one),
                true,
            ),
            (
                "both heads",
                row([80.0, -70.0, 0.0], [-50.0, 60.0, 0.0], 0.0, both),
                false,
            ),
            (
                "both thick",
                row([-80.0, 10.0, 0.0], [70.0, 20.0, 0.0], 5.0, both),
                true,
            ),
            (
                "bare thick",
                row([-60.0, -20.0, 0.0], [50.0, 50.0, 0.0], 6.0, 0),
                false,
            ),
            (
                "head only",
                row(
                    [30.0, -50.0, 0.0],
                    [40.0, -20.0, 0.0],
                    0.0,
                    one | VectorRow::HEAD_ONLY,
                ),
                false,
            ),
            (
                "short",
                row([10.0, 10.0, 0.0], [16.0, 13.0, 0.0], 0.0, one),
                false,
            ),
            (
                "capped thick",
                row([30.0, 60.0, 0.0], [50.0, 75.0, 0.0], 4.0, one),
                false,
            ),
            (
                "capped selected",
                row([-90.0, -80.0, 0.0], [-70.0, -90.0, 0.0], 0.0, one),
                true,
            ),
            (
                "short thick",
                row([-20.0, 0.0, 0.0], [-12.0, 6.0, 0.0], 4.0, one),
                false,
            ),
        ];
        // worst tip error, the farthest ink past a head's tip or a flat end, the least ink on a center line
        let (mut worst_tip, mut worst_far, mut worst_flat): (f64, f64, f64) = (0.0, 0.0, 0.0);
        let mut worst_seam: f64 = 1.0;

        for perspective in [true, false] {
            let mut camera = Camera::new();
            camera.perspective = perspective;
            camera.fit(&bounds, 1.0);
            let px = |p: [f32; 3]| {
                let at = Point::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2]));
                let m = camera.view_proj_anchored(1.0, &at).m;
                let s = f64::from(SIZE);
                [
                    (m[12] / m[15] * 0.5 + 0.5) * s,
                    (0.5 - m[13] / m[15] * 0.5) * s,
                ]
            };

            for (dpr, samples) in [(1.0, 1), (1.0, 4), (2.0, 1), (2.0, 4)] {
                gpu.logical_size = [f64::from(SIZE) / dpr; 2];
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(SIZE, SIZE);

                for (name, vector, selected) in &cases {
                    gpu.reset();
                    let mut up = Upload::default();
                    let mut object = ObjectRow::new(Xform::identity(), 0);
                    object.bounds = bounds;
                    up.obj.rows.push(object);
                    up.bounds = bounds;
                    up.lanes.get_mut::<VectorRows>().rows.push(*vector);
                    gpu.set_scene(&up);
                    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
                    let input = FrameInput {
                        view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
                        clear: wgpu::Color::WHITE,
                        now_ms: 0.0,
                    };
                    gpu.set_hidden(0, true);
                    let paper = gpu.render_offscreen(&input);
                    gpu.set_hidden(0, false);
                    gpu.set_selected(0, *selected);
                    let rgba = gpu.render_offscreen(&input);
                    let (a, b) = (px(vector.start), px(vector.end));
                    let [tip, tail] = reach(&rgba, &paper, SIZE as usize, a, b);
                    let len = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
                    let dir = [(b[0] - a[0]) / len, (b[1] - a[1]) / len];
                    // near the tip only the head: within half the shortest head
                    let r = (0.3 * len).min(2.5);
                    let offsets = [
                        tip_offset(&rgba, &paper, SIZE as usize, b, dir, r),
                        tip_offset(&rgba, &paper, SIZE as usize, a, [-dir[0], -dir[1]], r),
                    ];
                    let label =
                        format!("perspective {perspective}, dpr {dpr}, msaa {samples}, {name}");
                    let shaft = vector.heads & VectorRow::HEAD_ONLY == 0;
                    let headed = [
                        (
                            tip,
                            offsets[0],
                            vector.heads & VectorRow::HEAD_END != 0,
                            true,
                        ),
                        (
                            tail,
                            offsets[1],
                            vector.heads & VectorRow::HEAD_START != 0,
                            shaft,
                        ),
                    ];

                    for (end, offset, head, drawn) in headed {
                        if !drawn {
                            continue;
                        }

                        if head {
                            // the tip on the point; no ink, shaft or head, past half a pixel
                            assert!(offset.abs() <= 0.25, "{label}: tip off by {offset:+.3}");
                            assert!(end.far <= 0.5, "{label}: ink {:+.3} past the tip", end.far);
                            worst_tip = worst_tip.max(offset.abs());
                            worst_far = worst_far.max(end.far);
                        } else {
                            // a flat end: the edge on the point, only the fringe past it
                            assert!(end.edge.abs() <= 0.5, "{label}: flat edge {:+.3}", end.edge);
                            assert!(
                                end.far <= 1.0,
                                "{label}: ink {:+.3} past a flat end",
                                end.far
                            );
                            worst_flat = worst_flat.max(end.far);
                        }
                    }

                    // no seam across a shaft at least a pixel wide: away from a tip the center line is inked
                    if shaft && (vector.radius > 0.0 || *selected) {
                        let margin = |bit: u32| if vector.heads & bit != 0 { 3.0 } else { 1.5 };
                        let from = margin(VectorRow::HEAD_START);
                        let to = len - margin(VectorRow::HEAD_END);
                        let least = thinnest(&rgba, &paper, SIZE as usize, a, b, from, to);
                        assert!(least >= 0.9, "{label}: center line only {least:.3} inked");
                        worst_seam = worst_seam.min(least);
                    }
                }
            }
        }

        eprintln!("worst tip error {worst_tip:.3}, ink past a tip {worst_far:.3}, past a flat end {worst_flat:.3}, center line {worst_seam:.3}");
    }

    /// Curve heads: each tip lands on its curve's end point and no ink, curve or head, reaches past it.
    #[test]
    fn curve_heads_land_on_their_ends() {
        use crate::app::walk::{Walk, WalkCx, walk_geometry};
        use crate::camera::{Camera, View};
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow};
        use session_rust::{AABB, Arrowhead, Geometry, NurbsCurve, Point, Polyline, Xform};
        use std::rc::Rc;

        const SIZE: u32 = 256;
        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(SIZE, SIZE)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.show_grid = false;
        let mut bounds = AABB::empty();
        bounds.union_with_point(-100.0, -100.0, 0.0);
        bounds.union_with_point(100.0, 100.0, 0.0);
        let p = |x: f64, y: f64| Point::new(x, y, 0.0);
        let polyline = |head: Arrowhead, width: f64| {
            let mut pl = Polyline::new(vec![
                p(-70.0, -30.0),
                p(-20.0, 30.0),
                p(20.0, -30.0),
                p(70.0, 10.0),
            ]);
            pl.arrowhead = head;
            pl.width = width;
            Geometry::Polyline(Rc::new(pl))
        };
        let curve = |head: Arrowhead, width: f64| {
            let cvs = [p(-70.0, 0.0), p(-30.0, 60.0), p(30.0, -60.0), p(70.0, 0.0)];
            let mut c = NurbsCurve::create(false, 3, &cvs);
            c.arrowhead = head;
            c.width = width;
            Geometry::NurbsCurve(Rc::new(c))
        };
        // a last segment shorter than its head, and a curve turning sharply into its end
        let mut short = Polyline::new(vec![
            p(-70.0, -30.0),
            p(20.0, -30.0),
            p(70.0, 10.0),
            p(73.0, 10.0),
        ]);
        short.arrowhead = Arrowhead::END;
        let short = Geometry::Polyline(Rc::new(short));
        let cvs = [p(-70.0, 0.0), p(-30.0, 60.0), p(60.0, 0.0), p(70.0, -40.0)];
        let mut sharp = NurbsCurve::create(false, 3, &cvs);
        sharp.arrowhead = Arrowhead::END;
        let sharp = Geometry::NurbsCurve(Rc::new(sharp));
        // name, curve, selected
        let cases = [
            ("polyline both", polyline(Arrowhead::BOTH, 1.0), false),
            ("polyline thick", polyline(Arrowhead::END, 6.0), false),
            ("polyline selected", polyline(Arrowhead::START, 1.0), true),
            ("curve end", curve(Arrowhead::END, 1.0), false),
            ("curve both thick", curve(Arrowhead::BOTH, 4.0), true),
            ("polyline short end", short, false),
            ("curve sharp end", sharp, false),
        ];
        let (mut worst_tip, mut worst_far, mut worst_flat): (f64, f64, f64) = (0.0, 0.0, 0.0);
        // the default view, top in both projections, iso
        let views = [
            ("default", None, true),
            ("top", Some(View::Top), false),
            ("top", Some(View::Top), true),
            ("iso", Some(View::Iso), false),
        ];

        for (shown, view, perspective) in views {
            let mut camera = Camera::new();

            if let Some(view) = view {
                camera.set_view(view);
            }

            camera.perspective = perspective;
            camera.fit(&bounds, 1.0);
            let px = |q: [f32; 3]| {
                let at = Point::new(f64::from(q[0]), f64::from(q[1]), f64::from(q[2]));
                let m = camera.view_proj_anchored(1.0, &at).m;
                let s = f64::from(SIZE);
                [
                    (m[12] / m[15] * 0.5 + 0.5) * s,
                    (0.5 - m[13] / m[15] * 0.5) * s,
                ]
            };

            for (dpr, samples) in [(1.0, 1), (1.0, 4), (2.0, 1), (2.0, 4)] {
                gpu.logical_size = [f64::from(SIZE) / dpr; 2];
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(SIZE, SIZE);

                for (name, geometry, selected) in &cases {
                    gpu.reset();
                    let mut up = Upload::default();
                    let cx = WalkCx {
                        vert_base: 0,
                        cloud_px: 0.0,
                        row: 0,
                        attributes: false,
                    };
                    let walked = walk_geometry(&mut Walk::of(&mut up), &cx, geometry);
                    let mut object = ObjectRow::new(Xform::identity(), walked.flags);
                    object.bounds = bounds;
                    up.obj.rows.push(object);
                    up.bounds = bounds;
                    let heads = up
                        .lanes
                        .get::<VectorRows>()
                        .expect("head rows")
                        .rows
                        .clone();
                    gpu.set_scene(&up);
                    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
                    let input = FrameInput {
                        view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
                        clear: wgpu::Color::WHITE,
                        now_ms: 0.0,
                    };
                    gpu.set_hidden(0, true);
                    let paper = gpu.render_offscreen(&input);
                    gpu.set_hidden(0, false);
                    gpu.set_selected(0, *selected);
                    let rgba = gpu.render_offscreen(&input);
                    let label = format!(
                        "{shown}, perspective {perspective}, dpr {dpr}, msaa {samples}, {name}"
                    );

                    // only the ink near `tip`: the rest of the curve may lie past it on screen
                    let near = |tip: [f64; 2]| -> Vec<u8> {
                        rgba.chunks_exact(4)
                            .zip(paper.chunks_exact(4))
                            .enumerate()
                            .flat_map(|(i, (ink, blank))| {
                                let x = (i % SIZE as usize) as f64 + 0.5 - tip[0];
                                let y = (i / SIZE as usize) as f64 + 0.5 - tip[1];
                                if x * x + y * y < 40.0 * 40.0 {
                                    ink
                                } else {
                                    blank
                                }
                                .to_vec()
                            })
                            .collect()
                    };

                    for head in &heads {
                        let (aim, tip) = (px(head.start), px(head.end));
                        let len = ((tip[0] - aim[0]).powi(2) + (tip[1] - aim[1]).powi(2)).sqrt();
                        let dir = [(tip[0] - aim[0]) / len, (tip[1] - aim[1]) / len];
                        let [past, _] = reach(&near(tip), &paper, SIZE as usize, aim, tip);
                        let offset = tip_offset(&rgba, &paper, SIZE as usize, tip, dir, 2.5);
                        assert!(offset.abs() <= 0.5, "{label}: tip off by {offset:+.3}");
                        assert!(
                            past.far <= 0.5,
                            "{label}: ink {:+.3} past the tip",
                            past.far
                        );
                        worst_tip = worst_tip.max(offset.abs());
                        worst_far = worst_far.max(past.far);

                        // a thick curve runs on under its head's base: no seam
                        if *name == "polyline thick" {
                            let least =
                                thinnest(&rgba, &paper, SIZE as usize, aim, tip, 1.5, len - 3.0);
                            assert!(least >= 0.9, "{label}: center line only {least:.3} inked");
                        }
                    }

                    // a bare end of a headed curve is flat on its point, as a vector's
                    let ribbons = &up.seg.ribbons;
                    let (first, last) = (ribbons[0], ribbons[ribbons.len() - 1]);

                    for (aim, end) in [(first.p1, first.p0), (last.p0, last.p1)] {
                        if heads.iter().any(|head| head.end == end) {
                            continue;
                        }

                        let (aim, tip) = (px(aim), px(end));
                        let [past, _] = reach(&near(tip), &paper, SIZE as usize, aim, tip);
                        assert!(past.edge.abs() <= 0.5, "{label}: flat edge {:+.3}", past.edge);
                        assert!(past.far <= 1.0, "{label}: ink {:+.3} past a flat end", past.far);
                        worst_flat = worst_flat.max(past.far);
                    }
                }
            }
        }

        eprintln!(
            "curve heads: worst tip error {worst_tip:.3}, ink past a tip {worst_far:.3}, past a flat end {worst_flat:.3}"
        );
    }
}
