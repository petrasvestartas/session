use super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group};
use super::frame::Binds;
use super::lane::PickMode;
use super::lane::{Lane, Registered, RowLane};
use super::upload::Upload;
use super::view::View;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Pipeline, PipelineDesc, Shader, Target, build, ink_module,
};
use std::sync::atomic::{AtomicU64, Ordering};
use wgpu::PrimitiveTopology::TriangleList;

/// Shader sources the tests compare against the files.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("plane.wgsl", shader!("plane.wgsl"))];

/// Vertices per plane: six per grid line, eighteen per axis arrow.
const PLANE_VERTS: u32 = 22 * 6 + 3 * 18;

/// Half side of every plane's grid and length of its arrows, scene units, as f64 bits; `View Plane Size` sets it.
static PLANE_SIZE: AtomicU64 = AtomicU64::new(100.0_f64.to_bits());

/// Half side of every plane's grid, scene units.
pub fn plane_size() -> f64 {
    f64::from_bits(PLANE_SIZE.load(Ordering::Relaxed))
}

/// Set the half side of every plane's grid; the next frame draws it, no walk needed.
pub fn set_plane_size(size: f64) {
    PLANE_SIZE.store(size.to_bits(), Ordering::Relaxed);
}

/// One plane as the shader reads it: its frame; the grid and the arrows come from the vertex index.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlaneRow {
    // offset size
    pub origin: [f32; 3], //   0   12  object space
    pub instance_id: u32, //  12    4  object row
    pub x: [f32; 3],      //  16   12  unit x axis
    pub radius: f32,      //  28    4  pen: 0 = default; > 0 world mm; < 0 px
    pub y: [f32; 3],      //  32   12  unit y axis
    pub pad: u32,         //  44    4  -> 48, a multiple of 16
}

const _: () = assert!(std::mem::size_of::<PlaneRow>() == 48);

/// Plane rows of one upload; the walk writes them with `up.lanes.get_mut::<PlaneRows>()`.
#[derive(Default)]
pub struct PlaneRows {
    pub rows: Vec<PlaneRow>, // one per plane
}

/// Planes on the GPU: one instanced draw for all of them.
pub struct PlaneLane {
    buf: GrowBuf,           // PlaneRow rows
    group: wgpu::BindGroup, // group 3, binds the rows
    shader: Shader,         // plane shader
    color: Pipeline,        // grids and arrows in color
    id: Pipeline,           // plane object ids
}

/// The registry's entry: constructor, row count and merge.
pub const REGISTERED: Registered = Registered {
    make,
    rows_in,
    merge,
    stride: std::mem::size_of::<PlaneRow>() as u64,
};

/// The registry's constructor.
fn make(ctx: &GpuCtx, l: &Layouts, target: Target) -> Box<dyn RowLane> {
    Box::new(PlaneLane::new(ctx, l, target))
}

/// Plane rows in one upload.
fn rows_in(up: &Upload) -> u32 {
    up.lanes
        .get::<PlaneRows>()
        .map_or(0, |rows| rows.rows.len() as u32)
}

/// Move the plane rows of `other` after those of `up`.
fn merge(up: &mut Upload, other: &mut Upload) {
    if let Some(rows) = other.lanes.take::<PlaneRows>() {
        up.lanes.get_mut::<PlaneRows>().rows.extend(rows.rows);
    }
}

impl PlaneLane {
    /// Create the lane: shader, pipelines, an empty table.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = ink_module(ctx, "plane.shader", shader!("plane.wgsl"));
        let (color, id) = build_pipelines(ctx, l, &shader, target);
        let buf = GrowBuf::new(ctx, "planes", std::mem::size_of::<PlaneRow>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, "planes", &[&buf.buf]);
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
        self.group = bind_group(ctx, &l.ink_rows, "planes", &[&self.buf.buf]);
    }

    /// Draw every plane with `pipeline`; returns the draw count.
    fn draw(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &Pipeline) -> u32 {
        if self.buf.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.group, &[]);
        pass.draw(0..PLANE_VERTS, 0..self.buf.len());
        1
    }
}

impl Lane for PlaneLane {
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
        let Some(rows) = up.lanes.get::<PlaneRows>() else {
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

impl RowLane for PlaneLane {
    fn write_at(&mut self, ctx: &GpuCtx, _l: &Layouts, first: u32, up: &Upload) {
        if let Some(rows) = up.lanes.get::<PlaneRows>() {
            self.buf.write_at(ctx, first, &rows.rows);
        }
    }

    fn kill(&mut self, ctx: &GpuCtx, first: u32, count: u32, sink: u32) {
        let dead = PlaneRow {
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
        .vertex("vs_plane")
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    let color = build(
        ctx,
        target,
        &quad.with("plane", "fs_main").color(ColorWrite::Blended),
    );
    let id = build(
        ctx,
        Target::ID,
        &quad.with("plane.id", "fs_id").scene_samples(1),
    );
    (color, id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// The shader declares PlaneRow with the Rust fields at the Rust offsets.
    #[test]
    fn plane_row_mirror() {
        use std::mem::{offset_of, size_of};
        let (_, src) = SHADERS[0];
        assert_eq!(
            wgsl_fields(src, "PlaneRow"),
            ["origin", "instance_id", "x", "radius", "y", "pad"]
        );

        let source = crate::engine::pipelines::shared(&crate::engine::pipelines::ink_source(src));
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        let (_, ty) = module
            .types
            .iter()
            .find(|(_, ty)| ty.name.as_deref() == Some("PlaneRow"))
            .expect("PlaneRow in the shader");
        let naga::TypeInner::Struct { members, span } = &ty.inner else {
            panic!("PlaneRow is not a struct")
        };
        let offsets = [
            offset_of!(PlaneRow, origin),
            offset_of!(PlaneRow, instance_id),
            offset_of!(PlaneRow, x),
            offset_of!(PlaneRow, radius),
            offset_of!(PlaneRow, y),
            offset_of!(PlaneRow, pad),
        ];
        assert_eq!(*span as usize, size_of::<PlaneRow>());

        for (member, expected) in members.iter().zip(offsets) {
            assert_eq!(
                member.offset as usize, expected,
                "PlaneRow.{:?}",
                member.name
            );
        }
    }

    /// The shader's grey and axis colors are the ground grid's: grey 0.55, x pink, y yellow-green, z blue.
    #[test]
    fn plane_colors_are_the_grid_colors() {
        let (_, src) = SHADERS[0];
        let pack = |c: [f32; 3]| {
            let byte = |v: f32| ((v * 255.0 + 0.5) as u32) & 0xff;
            format!(
                "{:#010x}u",
                0xff00_0000 | byte(c[2]) << 16 | byte(c[1]) << 8 | byte(c[0])
            )
        };
        assert!(src.contains(&format!("const PLANE_GREY: u32 = {};", pack([0.55; 3]))));

        for color in [
            [0.910, 0.278, 0.545],
            [0.604, 0.804, 0.196],
            [0.129, 0.588, 0.918],
        ] {
            assert!(src.contains(&pack(color)), "{color:?}");
        }
    }
}
