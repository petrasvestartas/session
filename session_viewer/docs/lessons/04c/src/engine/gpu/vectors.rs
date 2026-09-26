// --8<-- [start:04b-vector-row]
// --8<-- [start:vector-row]
use super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group};
use super::frame::Binds;
use super::lane::PickMode;
use super::lane::{Lane, Registered, RowLane};
use super::upload::Upload;
use super::view::View;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build, ink_module,
};
use wgpu::PrimitiveTopology::TriangleList;

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("vector.wgsl", shader!("vector.wgsl"))];

/// Vertices per vector: a shaft quad and a quad per head, 3 x 6.
const VECTOR_VERTS: u32 = 18;

// An arrow is exactly as long as its line: the tip lands on the end point, the shaft stops under the head's base,
// and an end without a head is cut flat at its point, with no round cap poking past it.
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

// Associated constants live on the type, so callers write VectorRow::HEAD_END; the bits combine with |.
impl VectorRow {
    pub const HEAD_END: u32 = 1; // a head whose tip is `end`
    pub const HEAD_START: u32 = 2; // a head whose tip is `start`
    pub const HEAD_ONLY: u32 = 4; // no shaft: `start` only aims the heads, e.g. a curve's end tangent
}

const _: () = assert!(std::mem::size_of::<VectorRow>() == 48);

/// Vector rows of one upload; producers write them with `up.lanes.get_mut::<VectorRows>()`.
/// The upload finds these rows by their type, so adding a lane adds no field to Upload.
#[derive(Default)]
pub struct VectorRows {
    pub rows: Vec<VectorRow>, // one per vector
}
// --8<-- [end:vector-row]
// --8<-- [end:04b-vector-row]

// --8<-- [start:04b-vector-lane]
// --8<-- [start:vector-lane]
/// Vectors on the GPU: one instanced draw for all of them.
pub struct VectorLane {
    buf: GrowBuf,           // VectorRow rows
    group: wgpu::BindGroup, // group 3, binds the rows
    shader: Shader,         // vector shader
    color: Pipeline,        // arrows in color
    id: Pipeline,           // arrow object ids
}

/// The registry's entry: constructor, row count and merge.
/// One line in lane.rs lists this constant; the Gpu then builds, fills and draws the lane without naming its type.
pub const REGISTERED: Registered = Registered {
    make,
    rows_in,
    merge,
    stride: std::mem::size_of::<VectorRow>() as u64,
};

/// The registry's constructor.
// Box<dyn RowLane> = a lane of any type behind one pointer; calls go through the trait, decided at run time.
fn make(ctx: &GpuCtx, l: &Layouts, target: Target) -> Box<dyn RowLane> {
    Box::new(VectorLane::new(ctx, l, target))
}

/// Vector rows in one upload.
fn rows_in(up: &Upload) -> u32 {
    up.lanes
        .get::<VectorRows>()
        .map_or(0, |rows| rows.rows.len() as u32)
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
        let shader = ink_module(ctx, "vector.shader", shader!("vector.wgsl"));
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
    fn draw(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &Pipeline) -> u32 {
        if self.buf.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.group, &[]);
        // instancing: the same 18 corners run once per row, and the shader reads row `instance_index`
        pass.draw(0..VECTOR_VERTS, 0..self.buf.len());
        1
    }
}
// --8<-- [end:vector-lane]
// --8<-- [end:04b-vector-lane]

// --8<-- [start:04b-vector-traits]
// --8<-- [start:vector-traits]
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

// RowLane adds what an editable lane needs: overwrite one object's rows, and hide them for undo.
impl RowLane for VectorLane {
    fn write_at(&mut self, ctx: &GpuCtx, _l: &Layouts, first: u32, up: &Upload) {
        if let Some(rows) = up.lanes.get::<VectorRows>() {
            self.buf.write_at(ctx, first, &rows.rows);
        }
    }

    fn kill(&mut self, ctx: &GpuCtx, first: u32, count: u32, sink: u32) {
        // `..zeroed()` fills every field not named with zero; `sink` is an object row that is always hidden
        let dead = VectorRow {
            instance_id: sink,
            ..bytemuck::Zeroable::zeroed()
        };
        self.buf.fill(ctx, first, count, &dead);
    }
}
// --8<-- [end:vector-traits]
// --8<-- [end:04b-vector-traits]

// --8<-- [start:04b-vector-pipelines]
// --8<-- [start:vector-pipelines]
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
// --8<-- [end:vector-pipelines]
// --8<-- [end:04b-vector-pipelines]

// --8<-- [start:04b-vector-tests]
// --8<-- [start:vector-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// A pen-wide vector from `start` to `end` on object `row`, default head.
    pub(super) fn arrow(row: u32, start: [f32; 3], end: [f32; 3], color: [f32; 4]) -> VectorRow {
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

        let source = crate::engine::pipelines::shared(&crate::engine::pipelines::ink_source(src));
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

    /// How much of a pixel ink covers, from its darkest channel over the empty frame's; both sRGB.
    pub(super) fn covered(pixel: &[u8], paper: &[u8]) -> f64 {
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
    pub(super) struct Reach {
        pub(super) far: f64,  // center of the farthest inked pixel
        pub(super) edge: f64, // the ink's edge: a pixel of coverage c has it c - 0.5 px past its center
    }

    /// Reach of the ink in `rgba` over the empty frame `paper` past `b` (along a to b) and past `a` (back).
    pub(super) fn reach(
        rgba: &[u8],
        paper: &[u8],
        width: usize,
        a: [f64; 2],
        b: [f64; 2],
    ) -> [Reach; 2] {
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
    pub(super) fn thinnest(
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
    pub(super) fn tip_offset(
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
        use session_rust::{AABB, Point, Xform};

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

        eprintln!(
            "worst tip error {worst_tip:.3}, ink past a tip {worst_far:.3}, past a flat end {worst_flat:.3}, center line {worst_seam:.3}"
        );
    }
}
// --8<-- [end:vector-tests]
// --8<-- [end:04b-vector-tests]
