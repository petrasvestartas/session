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
pub const SHADERS: &[(&str, &str)] = &[("matrix.wgsl", shader!("matrix.wgsl"))];

/// Vertices per matrix: one quad over all its cells.
const MATRIX_VERTS: u32 = 6;

/// Words before a matrix's values; the shader reads the header at these offsets.
pub const HEADER_WORDS: usize = 16;
/// Words per value: its f32, its whole part, its thousandths with the sign in the top bit.
pub const VALUE_WORDS: usize = 3;
/// The sign in a value's third word.
pub const SIGN_BIT: u32 = 0x8000_0000;
pub const ROWS_AT: usize = 0; // row count
pub const COLS_AT: usize = 1; // column count
pub const INSTANCE_AT: usize = 2; // object row
pub const CELL_AT: usize = 3; // edge length of a cell, f32 bits
pub const LOW_AT: usize = 4; // smallest non-zero entry, f32 bits
pub const HIGH_AT: usize = 5; // largest non-zero entry, f32 bits
pub const LINE_AT: usize = 6; // edge colour, packed rgba
pub const MIN_AT: usize = 8; // colour of the smallest entry, packed rgba
pub const MAX_AT: usize = 9; // colour of the largest entry, packed rgba
pub const ZERO_AT: usize = 10; // colour of a zero entry, packed rgba
pub const TEXT_AT: usize = 11; // colour of the values, packed rgba

/// Matrices of one upload as words: per matrix a header, then its values row by row, VALUE_WORDS each.
#[derive(Default)]
pub struct MatrixRows {
    pub words: Vec<u32>, // block after block
}

/// The first word of every block in `words`, plus `base`.
fn block_starts(words: &[u32], base: u32) -> Vec<u32> {
    let mut starts = Vec::new();
    let mut at = 0usize;

    while at + HEADER_WORDS <= words.len() {
        starts.push(base + at as u32);
        at += HEADER_WORDS + words[at + ROWS_AT] as usize * words[at + COLS_AT] as usize * VALUE_WORDS;
    }

    starts
}

/// Matrices on the GPU: one quad per matrix, the fragment shader does the rest.
pub struct MatrixLane {
    buf: GrowBuf,           // MatrixRows words
    starts: Vec<u32>,       // first word of every matrix: its draw's instance
    group: wgpu::BindGroup, // group 3, binds the words
    shader: Shader,         // matrix shader
    color: Pipeline,        // matrices in color
    id: Pipeline,           // matrix object ids
}

/// The registry's entry: constructor, row count and merge.
pub const REGISTERED: Registered = Registered {
    make,
    rows_in,
    merge,
    stride: std::mem::size_of::<u32>() as u64,
};

/// The registry's constructor.
fn make(ctx: &GpuCtx, l: &Layouts, target: Target) -> Box<dyn RowLane> {
    Box::new(MatrixLane::new(ctx, l, target))
}

/// Matrix words in one upload.
fn rows_in(up: &Upload) -> u32 {
    up.lanes
        .get::<MatrixRows>()
        .map_or(0, |rows| rows.words.len() as u32)
}

/// Move the matrix words of `other` after those of `up`.
fn merge(up: &mut Upload, other: &mut Upload) {
    if let Some(rows) = other.lanes.take::<MatrixRows>() {
        up.lanes.get_mut::<MatrixRows>().words.extend(rows.words);
    }
}

impl MatrixLane {
    /// Create the lane: shader, pipelines, an empty table.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shader = ink_module(ctx, "matrix.shader", shader!("matrix.wgsl"));
        let (color, id) = build_pipelines(ctx, l, &shader, target);
        let buf = GrowBuf::new(ctx, "matrices", std::mem::size_of::<u32>() as u64, ROWS);
        let group = bind_group(ctx, &l.ink_rows, "matrices", &[&buf.buf]);
        Self {
            buf,
            starts: Vec::new(),
            group,
            shader,
            color,
            id,
        }
    }

    /// Rebuild the bind group after the buffer moved.
    fn rebind(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.group = bind_group(ctx, &l.ink_rows, "matrices", &[&self.buf.buf]);
    }

    /// Draw every matrix with `pipeline`, one instance each; returns the draw count.
    fn draw(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &Pipeline) -> u32 {
        if self.starts.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.group, &[]);

        for &start in &self.starts {
            pass.draw(0..MATRIX_VERTS, start..start + 1);
        }

        self.starts.len() as u32
    }
}

impl Lane for MatrixLane {
    fn on_retarget(&mut self, ctx: &GpuCtx, layouts: &Layouts, target: Target) {
        (self.color, self.id) = build_pipelines(ctx, layouts, &self.shader, target);
    }

    fn on_reset(&mut self, _ctx: &GpuCtx) {
        self.buf.reset();
        self.starts.clear();
    }

    fn on_release(&mut self, ctx: &GpuCtx, layouts: &Layouts) {
        self.buf.release(ctx);
        self.starts = Vec::new();
        self.rebind(ctx, layouts);
    }

    fn bytes(&self) -> (u64, u64) {
        (self.buf.buf.size(), 0)
    }

    fn on_append(&mut self, ctx: &GpuCtx, layouts: &Layouts, up: &Upload) {
        let Some(rows) = up.lanes.get::<MatrixRows>() else {
            return;
        };

        self.starts.extend(block_starts(&rows.words, self.buf.len()));

        if self.buf.append(ctx, &rows.words) {
            self.rebind(ctx, layouts);
        }
    }

    fn draw_ink(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, _view: &View) -> u32 {
        self.draw(pass, b, &self.color)
    }

    fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        b: &Binds,
        _view: &View,
        mode: PickMode,
    ) -> u32 {
        if mode != PickMode::Object {
            return 0;
        }

        self.draw(pass, b, &self.id)
    }
}

impl RowLane for MatrixLane {
    fn write_at(&mut self, ctx: &GpuCtx, _l: &Layouts, first: u32, up: &Upload) {
        if let Some(rows) = up.lanes.get::<MatrixRows>() {
            self.buf.write_at(ctx, first, &rows.words);
        }
    }

    // a killed block reads zero rows and columns: its quad is never placed
    fn kill(&mut self, ctx: &GpuCtx, first: u32, count: u32, _sink: u32) {
        self.buf.fill(ctx, first, count, &0u32);
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
        .vertex("vs_matrix")
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    let color = build(
        ctx,
        target,
        &quad.with("matrix", "fs_main").color(ColorWrite::Blended),
    );
    let id = build(
        ctx,
        Target::ID,
        &quad.with("matrix.id", "fs_id").scene_samples(1),
    );
    (color, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shader reads the header at the Rust offsets.
    #[test]
    fn header_offsets_match_the_shader() {
        let (_, src) = SHADERS[0];

        for (name, at) in [
            ("HEADER_WORDS", HEADER_WORDS),
            ("VALUE_WORDS", VALUE_WORDS),
            ("ROWS", ROWS_AT),
            ("COLS", COLS_AT),
            ("INSTANCE", INSTANCE_AT),
            ("CELL", CELL_AT),
            ("LOW", LOW_AT),
            ("HIGH", HIGH_AT),
            ("LINE", LINE_AT),
            ("MIN", MIN_AT),
            ("MAX", MAX_AT),
            ("ZERO", ZERO_AT),
            ("TEXT", TEXT_AT),
        ] {
            assert!(src.contains(&format!("const {name}: u32 = {at}u;")), "{name}");
        }

        assert!(src.contains(&format!("const SIGN_BIT: u32 = {SIGN_BIT:#x}u;")));
    }

    /// Blocks are found from their own row and column counts.
    #[test]
    fn blocks_start_after_their_values() {
        let first = HEADER_WORDS + 6 * VALUE_WORDS;
        let mut words = vec![0u32; first + HEADER_WORDS + VALUE_WORDS];
        words[ROWS_AT] = 2;
        words[COLS_AT] = 3;
        words[first + ROWS_AT] = 1;
        words[first + COLS_AT] = 1;
        assert_eq!(block_starts(&words, 10), [10, 10 + first as u32]);
    }
}
