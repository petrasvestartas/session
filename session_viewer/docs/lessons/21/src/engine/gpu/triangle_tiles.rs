// --8<-- [start:step-29a]
//! Splits the screen into tiles and lists which triangles touch each tile, so a pixel tests a few triangles instead of all.
use super::buffers::{GpuCtx, ROWS, bind_group, replace_buffer, uniform_buffer, zeroed_buffer};
use super::frame::Binds;
use super::targets::{Attachment, TextureSpec};
// --8<-- [end:step-29a]
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, pipeline_layout,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

/// Bytes per projected triangle record.
pub(super) const PROJECTED_BYTES: u64 = 96;

/// Most screen tiles the grid may have.
const MAX_TILES: u32 = 262_144;

/// Most triangle references per tile on average.
const REFERENCES_PER_TILE: u64 = 32;

/// Words per reference: triangle index and depth.
const REFERENCE_WORDS: u64 = 2;

/// Smallest reference pool, in words.
const MIN_POOL_WORDS: u64 = 32 * 1024;

/// The screen tile grid: tile count and pixels per tile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TileLayout {
    width: u32, // tiles across
    height: u32, // tiles down
    span: u32, // pixels per tile side
}

impl TileLayout {
    /// Grid for a canvas size; tiles grow until the count fits.
    fn new(size: (u32, u32)) -> Self {
        let mut span = 4;

        while size.0.div_ceil(span) as u64 * size.1.div_ceil(span) as u64 > MAX_TILES as u64 {
            span *= 2;
        }

        Self {
            width: size.0.div_ceil(span).max(1),
            height: size.1.div_ceil(span).max(1),
            span,
        }
    }

    /// Tiles in the grid.
    fn count(self) -> u32 {
        self.width * self.height
    }

    /// 16-byte records before the pool: one per tile plus scan totals.
    fn header_records(self) -> u64 {
        let blocks = self.count().div_ceil(256);
        1 + self.count() as u64 + blocks as u64 + blocks.div_ceil(256) as u64
    }

    /// Bytes of the tile buffer with a pool of `pool_words`.
    fn buffer_bytes(self, pool_words: u64) -> u64 {
        self.header_records() * 16 + pool_words * 4
    }

    /// Largest pool, in words.
    fn max_pool_words(self) -> u64 {
        self.count() as u64 * REFERENCES_PER_TILE * REFERENCE_WORDS
    }

    /// First pool size for `triangles`, in words.
    fn initial_pool_words(self, triangles: u32) -> u64 {
        let references = self.count() as u64 * 2 + u64::from(triangles) * 8;
        (references * REFERENCE_WORDS)
            .max(MIN_POOL_WORDS)
            .min(self.max_pool_words())
    }
}

/// Reads back how many words the last scan needed.
struct PoolReport {
    buffer: wgpu::Buffer, // 16-byte CPU-readable copy
    ready: Arc<AtomicU8>, // 0 waiting, 1 mapped, 2 failed
    copied: bool, // a copy was encoded this frame
    inflight: bool, // the copy is being mapped
}

impl PoolReport {
    /// Create the readback buffer.
    fn new(ctx: &GpuCtx) -> Self {
        Self {
            buffer: ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("triangle.tiles.report"),
                size: 16,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
            ready: Arc::new(AtomicU8::new(0)),
            copied: false,
            inflight: false,
        }
    }

    /// Copy the first tile record into the readback buffer.
    fn copy(&mut self, encoder: &mut wgpu::CommandEncoder, tiles: &wgpu::Buffer) {
        if self.inflight {
            return;
        }

        encoder.copy_buffer_to_buffer(tiles, 0, &self.buffer, 0, 16);
        self.copied = true;
    }

    /// Start mapping the copy; call once after submit.
    fn map(&mut self) {
        if !self.copied || self.inflight {
            return;
        }

        self.copied = false;
        self.inflight = true;
        let flag = self.ready.clone();
        self.buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                flag.store(if result.is_ok() { 1 } else { 2 }, Ordering::Release);
            });
    }

    /// Words the last scan needed, once the copy is readable.
    fn poll(&mut self) -> Option<u64> {
        if !self.inflight {
            return None;
        }

        let status = self.ready.load(Ordering::Acquire);

        if status == 0 {
            return None;
        }

        self.ready.store(0, Ordering::Release);
        self.inflight = false;

        if status != 1 {
            return None;
        }

        let words = {
            let bytes = self.buffer.slice(..).get_mapped_range();
            u32::from_le_bytes(bytes[4..8].try_into().expect("report words"))
        };
        self.buffer.unmap();
        Some(u64::from(words))
    }
}

/// Next pool size: at least `floor`; doubled when the report overflowed.
fn next_pool_words(
    current: u64,
    floor: u64,
    capacity: u64,
    report: Option<u64>,
    ceiling: u64,
) -> u64 {
    let mut want = current.max(floor);

    // a report at or past capacity means the lists did not fit
    if let Some(needed) = report
        && needed >= capacity
    {
        want = want.max(current.saturating_mul(2));
    }

    want.min(ceiling)
}

/// What the tile lists were built for; same key = reuse them.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ProjectionKey {
    matrix: [f32; 16],
    objects: u64, // object change count
}

/// The layouts and pipelines of the tile passes.
struct TilePipelines {
    project_layout: wgpu::BindGroupLayout, // vertices, rows, indices, projected, count
    raster_layout: wgpu::BindGroupLayout, // projected, tiles
    scan_layout: wgpu::BindGroupLayout, // tiles
    project: wgpu::ComputePipeline, // triangles to screen space
    count: wgpu::RenderPipeline, // count triangles per tile
    fill: wgpu::RenderPipeline, // write triangle lists per tile
    scans: [wgpu::ComputePipeline; 3], // prefix sum of the counts, three levels
}

/// Which triangles cover which screen tiles, rebuilt when the camera moves.
pub struct TriangleTiles {
    pub buffer: wgpu::Buffer, // tile headers and reference pool
    pub projected: wgpu::Buffer, // one screen-space record per triangle
    requested_triangles: u32,
    layout: Option<TileLayout>, // current grid, None when empty
    // --8<-- [start:step-29b]
    target: Option<Attachment>, // one pixel per tile, drawn into but never read
    // --8<-- [end:step-29b]
    live_count: wgpu::Buffer, // triangle count, for the shaders
    key: Option<ProjectionKey>, // what the lists were built for
    pipes: TilePipelines,
    pool_words: u64,
    report: PoolReport, // readback of the words needed
}

impl TriangleTiles {
    /// Create with tiny placeholder buffers.
    pub fn new(ctx: &GpuCtx, layouts: &Layouts) -> Self {
        Self {
            buffer: zeroed_buffer(&ctx.device, "triangle.tiles", 16, ROWS),
            projected: zeroed_buffer(&ctx.device, "triangle.projected", PROJECTED_BYTES, ROWS),
            requested_triangles: 0,
            layout: None,
            target: None,
            live_count: uniform_buffer(&ctx.device, "triangle.project.count", &[0u32; 4]),
            key: None,
            pipes: TilePipelines::new(ctx, layouts),
            pool_words: 0,
            report: PoolReport::new(ctx),
        }
    }

    /// Start reading the scan's report; call after submit.
    pub fn map_report(&mut self) {
        self.report.map();
    }

    /// Words in the buffer: headers plus pool.
    fn pool_capacity(&self, layout: TileLayout) -> u64 {
        layout.header_records() * 4 + self.pool_words
    }

    /// Force a rebuild on the next frame.
    pub fn invalidate(&mut self) {
        self.key = None;
    }

    /// Size the buffers for `size` and `triangles`; returns true if a buffer moved.
    pub fn prepare(&mut self, ctx: &GpuCtx, size: (u32, u32), triangles: u32) -> bool {
        let limit = ctx.device.limits().max_storage_buffer_binding_size;

        // no triangles, or too many for the device: drop the tables
        if triangles == 0 || triangles as u64 * PROJECTED_BYTES > limit {
            let changed = self.release_data(ctx);

            if triangles != self.requested_triangles && triangles > 0 {
                log::warn!(
                    "Finite triangle visibility exceeds the device storage limit for {triangles} triangles; retaining physical depth occlusion"
                );
            }

            self.requested_triangles = triangles;
            return changed;
        }

        let mut changed = false;
        let layout = TileLayout::new(size);
        // only a report for this grid counts
        let report = match self.report.poll() {
            Some(needed) if self.layout == Some(layout) => Some(needed),
            // a different grid's report is stale
            _ => None,
        };
        let pool_words = next_pool_words(
            self.pool_words,
            layout.initial_pool_words(triangles),
            self.pool_capacity(layout),
            report,
            layout.max_pool_words(),
        );
        let grow = pool_words > self.pool_words;

        // new triangle count: new projected table
        if triangles != self.requested_triangles || self.layout.is_none() {
            // --8<-- [start:step-29c]
            replace_buffer(
                &mut self.projected,
                zeroed_buffer(
                    &ctx.device,
                    "triangle.projected",
                    triangles as u64 * PROJECTED_BYTES,
                    ROWS,
                ),
                // --8<-- [end:step-29c]
            );
            ctx.queue.write_buffer(
                &self.live_count,
                0,
                bytemuck::cast_slice(&[triangles, 0u32, 0, 0]),
            );
            self.requested_triangles = triangles;
            self.invalidate();
            changed = true;
        }

        // new grid or bigger pool: new tile buffer
        if self.layout != Some(layout) || grow {
            self.pool_words = pool_words;
            // --8<-- [start:step-29d]
            replace_buffer(
                &mut self.buffer,
                zeroed_buffer(
                    &ctx.device,
                    "triangle.tiles",
                    layout.buffer_bytes(pool_words).min(limit),
                    ROWS,
                ),
            );
            self.target = Some(Attachment::new(
                ctx,
                "triangle.tiles.target",
                &TextureSpec {
                    size: (layout.width, layout.height),
                    format: wgpu::TextureFormat::R8Unorm,
                    samples: 1,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                },
            ));
            // --8<-- [end:step-29d]
            self.layout = Some(layout);
            self.invalidate();
            changed = true;
        }

        changed
    }

    /// Rebuild the tile lists unless camera and objects are unchanged.
    pub(super) fn encode(
        &mut self,
        ctx: &GpuCtx,
        encoder: &mut wgpu::CommandEncoder,
        input: TileInput<'_>,
    ) {
        let Some(layout) = self.layout else {
            return;
        };
        let key = ProjectionKey {
            matrix: input.matrix,
            objects: input.objects_revision,
        };

        if self.key == Some(key) {
            return;
        }

        let group = bind_group(
            ctx,
            &self.pipes.project_layout,
            "triangle.project.bindings",
            &[
                input.geometry[0],
                input.geometry[1],
                input.geometry[2],
                &self.projected,
                &self.live_count,
            ],
        );
        {
            // 1: project every triangle to the screen
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("triangle.project"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipes.project);
            pass.set_bind_group(0, input.binds.mvp, &[]);
            pass.set_bind_group(1, input.binds.line, &[]);
            pass.set_bind_group(2, input.binds.instances, &[]);
            pass.set_bind_group(3, &group, &[]);
            pass.dispatch_workgroups(self.requested_triangles.div_ceil(64), 1, 1);
        }
        // 2: zero the tile headers
        encoder.clear_buffer(&self.buffer, 0, Some(layout.header_records() * 16));
        let raster = bind_group(
            ctx,
            &self.pipes.raster_layout,
            "triangle.tiles.bindings",
            &[&self.projected, &self.buffer],
        );
        // 3: count triangles per tile
        self.bin(encoder, input.binds, &raster, &self.pipes.count);
        let scan = bind_group(
            ctx,
            &self.pipes.scan_layout,
            "triangle.scan.bindings",
            &[&self.buffer],
        );
        let blocks = layout.count().div_ceil(256);

        // 4: prefix sum gives each tile its list offset
        for (index, count) in [blocks, blocks.div_ceil(256), blocks]
            .into_iter()
            .enumerate()
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("triangle.scan"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipes.scans[index]);
            pass.set_bind_group(0, input.binds.line, &[]);
            pass.set_bind_group(1, &scan, &[]);
            pass.dispatch_workgroups(count, 1, 1);
        }

        // 5: write the triangle lists
        self.bin(encoder, input.binds, &raster, &self.pipes.fill);
        self.report.copy(encoder, &self.buffer);
        self.key = Some(key);
    }

    /// Draw every triangle over the tile grid with `pipeline`.
    fn bin(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        binds: &Binds,
        group: &wgpu::BindGroup,
        pipeline: &wgpu::RenderPipeline,
    ) {
        let Some(view) = &self.target else {
            return;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("triangle.tiles"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Discard,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(pipeline);
        binds.set(&mut pass);
        pass.set_bind_group(3, group, &[]);
        // a quad per triangle, covering its tiles
        pass.draw(0..6, 0..self.requested_triangles);
    }

    /// Shrink the buffers back to placeholders; returns true if they were bigger.
    fn release_data(&mut self, ctx: &GpuCtx) -> bool {
        if self.layout.take().is_none() {
            return false;
        }

        // --8<-- [start:step-29e]
        replace_buffer(
            &mut self.buffer,
            zeroed_buffer(&ctx.device, "triangle.tiles", 16, ROWS),
        );
        replace_buffer(
            &mut self.projected,
            zeroed_buffer(&ctx.device, "triangle.projected", PROJECTED_BYTES, ROWS),
        );
        // --8<-- [end:step-29e]
        self.target = None;
        self.invalidate();
        true
    }

    /// Forget the scene and free the buffers.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.release_data(ctx);
        self.requested_triangles = 0;
        self.pool_words = 0;
        self.invalidate();
    }

    /// Bytes reserved on the GPU: (buffers, textures).
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let pixels = self.layout.map_or(0, TileLayout::count) as u64;
        (
            self.buffer.size()
                + self.projected.size()
                + self.live_count.size()
                + self.report.buffer.size(),
            pixels,
        )
    }
}

/// What `encode` needs from the frame and the arena.
pub(super) struct TileInput<'a> {
    pub binds: &'a Binds<'a>, // bind groups 0-2
    pub geometry: [&'a wgpu::Buffer; 3], // vertices, object rows, indices
    pub matrix: [f32; 16],
    pub objects_revision: u64, // object change count
}

/// One buffer binding for a layout.
fn entry(
    binding: u32,
    visibility: wgpu::ShaderStages,
    ty: wgpu::BufferBindingType,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

impl TilePipelines {
    /// Create the layouts, shaders and pipelines.
    fn new(ctx: &GpuCtx, layouts: &Layouts) -> Self {
        use wgpu::BufferBindingType::{Storage, Uniform};
        use wgpu::ShaderStages as Stages;
        let device = &ctx.device;
        let project_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("triangle.project.layout"),
            entries: &[
                entry(0, Stages::COMPUTE, Storage { read_only: true }),
                entry(1, Stages::COMPUTE, Storage { read_only: true }),
                entry(2, Stages::COMPUTE, Storage { read_only: true }),
                entry(3, Stages::COMPUTE, Storage { read_only: false }),
                entry(4, Stages::COMPUTE, Uniform),
            ],
        });
        let raster_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("triangle.tiles.layout"),
            entries: &[
                entry(0, Stages::VERTEX, Storage { read_only: true }),
                entry(1, Stages::FRAGMENT, Storage { read_only: false }),
            ],
        });
        let scan_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("triangle.scan.layout"),
            entries: &[entry(0, Stages::COMPUTE, Storage { read_only: false })],
        });
        let project_shader = shader(
            device,
            "triangle.project",
            include_str!("../../shaders/project_triangles.wgsl"),
        );
        let project_pipeline_layout = pipeline_layout(
            device,
            "triangle.project",
            &[
                &layouts.mvp,
                &layouts.line,
                &layouts.instance,
                &project_layout,
            ],
        );
        let project =
            compute_pipeline(device, &project_pipeline_layout, &project_shader, "cs_main");
        let raster_shader = shader(
            device,
            "triangle.tiles",
            include_str!("../../shaders/triangle_tiles.wgsl"),
        );
        // the fragment shader writes buffers, not pixels
        let raster_groups = [
            &layouts.mvp,
            &layouts.line,
            &layouts.instance,
            &raster_layout,
        ];
        let raster = PipelineDesc::new(
            &raster_shader,
            &raster_groups,
            &[],
            wgpu::PrimitiveTopology::TriangleList,
        )
        .depth(DepthMode::Detached)
        .color(ColorWrite::Nothing);
        let tile_target = Target {
            format: wgpu::TextureFormat::R8Unorm,
            samples: 1,
        };
        let count = build(device, tile_target, &raster.with("fs_count", "fs_count"));
        let fill = build(device, tile_target, &raster.with("fs_fill", "fs_fill"));
        let scan_shader = shader(
            device,
            "triangle.scan",
            include_str!("../../shaders/scan_triangle_tiles.wgsl"),
        );
        let scan_pipeline_layout =
            pipeline_layout(device, "triangle.scan", &[&layouts.line, &scan_layout]);
        let scans = [
            compute_pipeline(device, &scan_pipeline_layout, &scan_shader, "scan_tiles"),
            compute_pipeline(device, &scan_pipeline_layout, &scan_shader, "scan_blocks"),
            compute_pipeline(
                device,
                &scan_pipeline_layout,
                &scan_shader,
                "finish_offsets",
            ),
        ];
        Self {
            project_layout,
            raster_layout,
            scan_layout,
            project,
            count,
            fill,
            scans,
        }
    }
}

/// Compile a shader with the shared projected-triangle code appended.
fn shader(device: &wgpu::Device, label: &str, source: &str) -> wgpu::ShaderModule {
    let source = format!(
        "{source}\n{}",
        include_str!("../../shaders/projected_triangle.wgsl")
    );
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}

/// A compute pipeline for one entry point.
fn compute_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    entry: &str,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some(entry),
        layout: Some(layout),
        module: shader,
        entry_point: Some(entry),
        compilation_options: Default::default(),
        cache: None,
    })
}

/// Pool sizing and grid tests.
#[cfg(test)]
mod tests {
    use super::*;

    /// Past or at capacity doubles; below it keeps the pool.
    #[test]
    fn a_saturated_report_grows_the_pool() {
        let capacity = 1_000;
        let pool = 800;
        assert_eq!(
            next_pool_words(pool, 0, capacity, Some(capacity + 64), u64::MAX),
            1_600,
            "a report past capacity doubles"
        );
        assert_eq!(
            next_pool_words(pool, 0, capacity, Some(capacity), u64::MAX),
            1_600,
            "a pool exactly filled has also run out"
        );
        assert_eq!(
            next_pool_words(pool, 0, capacity, Some(capacity - 1), u64::MAX),
            pool,
            "a report below capacity measured lists that fitted"
        );
        assert_eq!(
            next_pool_words(pool, 0, capacity, None, u64::MAX),
            pool,
            "no report, no change"
        );
    }

    /// Growth stops at the ceiling.
    #[test]
    fn growth_stops_at_the_ceiling() {
        let ceiling = 2_048;
        let mut pool = 512;

        for _ in 0..8 {
            pool = next_pool_words(pool, 0, pool, Some(pool), ceiling);
        }

        assert_eq!(pool, ceiling);
        assert_eq!(next_pool_words(pool, 0, pool, Some(pool), ceiling), ceiling);
    }

    /// Doubling reaches ten times the size in four frames.
    #[test]
    fn doubling_converges_in_a_few_frames() {
        let ceiling = u64::MAX;
        let mut pool = 1_000;
        let mut frames = 0;

        while pool < 10_000 {
            pool = next_pool_words(pool, 0, pool, Some(pool), ceiling);
            frames += 1;
        }

        assert_eq!(frames, 4);
    }

    /// The floor wins when the scene grew.
    #[test]
    fn the_initial_pool_is_a_floor() {
        assert_eq!(next_pool_words(100, 4_096, 100, None, u64::MAX), 4_096);
        assert_eq!(next_pool_words(8_192, 4_096, 8_192, None, u64::MAX), 8_192);
    }

    #[test]
    /// Every canvas size gets a grid under the tile and memory limits.
    fn viewport_grid_fits_core_storage_limits_without_fixed_per_tile_caps() {
        for size in [
            (1, 1),
            (1400, 900),
            (1800, 1400),
            (2800, 1800),
            (7680, 4320),
            (16384, 16384),
        ] {
            let layout = TileLayout::new(size);
            assert!(layout.count() <= MAX_TILES);
            assert!(layout.buffer_bytes(layout.max_pool_words()) < 128 * 1024 * 1024);
            assert!(layout.width * layout.span >= size.0 && layout.height * layout.span >= size.1);
        }

        assert_eq!(TileLayout::new((1800, 1400)).span, 4);
        assert_eq!(TileLayout::new((2800, 1800)).span, 8);
    }

    /// A small scene starts with a small pool.
    #[test]
    fn pool_starts_small_and_is_capped() {
        let layout = TileLayout::new((3200, 2000));
        let small = layout.initial_pool_words(6_000);
        assert!(small < layout.max_pool_words() / 8);
        assert_eq!(layout.initial_pool_words(u32::MAX), layout.max_pool_words());
        assert!(layout.buffer_bytes(small) < 4 * 1024 * 1024);
        assert_eq!(
            layout.buffer_bytes(layout.max_pool_words()),
            layout.header_records() * 16 + layout.count() as u64 * REFERENCES_PER_TILE * 8
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Lists are reused until camera, hiding or scene change.
    fn projection_cache_tracks_camera_hidden_state_replacement_and_release() {
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
        use session_rust::{RenderVertex, Xform};
        let mut gpu = pollster::block_on(Gpu::new_headless(128, 128)).unwrap();
        gpu.view.show_grid = false;
        let initial = gpu.arena.tiles.allocated_bytes();
        // placeholders: tiles, one record, count, report
        assert_eq!(initial, (16 + PROJECTED_BYTES + 16 + 16, 0));
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), 0));

        for position in [[-0.7, -0.7, 0.5], [0.7, -0.7, 0.5], [0.0, 0.7, 0.5]] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [0.0, 0.0, 1.0],
                color: [0.5; 4],
            });
            upload.arena.vids.push(0);
        }

        upload.arena.idx.extend([0, 1, 2]);
        gpu.set_scene(&upload);
        let mut input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let visible = gpu.render_offscreen(&input);
        let key = gpu
            .arena
            .tiles
            .key
            .expect("a populated scene prepares visibility");
        assert!(gpu.arena.tiles.allocated_bytes().0 > initial.0);
        assert_eq!(visible, gpu.render_offscreen(&input));
        assert_eq!(gpu.arena.tiles.key, Some(key));
        gpu.set_selected(0, true);
        gpu.render_offscreen(&input);
        assert_eq!(
            gpu.arena.tiles.key,
            Some(key),
            "highlighting must not reproject source triangles"
        );
        gpu.set_hidden(0, true);
        let hidden = gpu.render_offscreen(&input);
        assert_ne!(hidden, visible);
        assert_ne!(
            gpu.arena.tiles.key,
            Some(key),
            "a hidden occluder must leave the tile lists"
        );
        let hidden_key = gpu.arena.tiles.key;
        gpu.set_hidden(0, false);
        input.view_proj.m[12] = 0.1;
        gpu.render_offscreen(&input);
        assert_ne!(gpu.arena.tiles.key, hidden_key);
        gpu.resize(160, 96);
        gpu.render_offscreen(&input);
        assert_eq!(gpu.arena.tiles.layout, Some(TileLayout::new((160, 96))));
        gpu.reset();
        gpu.set_scene(&upload);
        assert!(
            gpu.arena.tiles.key.is_none(),
            "same-count replacement invalidates projected positions"
        );
        gpu.render_ids_offscreen(&input);
        assert!(
            gpu.arena.tiles.key.is_some(),
            "ID-only rendering prepares its own current geometry"
        );
        gpu.release();
        assert_eq!(gpu.arena.tiles.allocated_bytes(), initial);
        assert!(gpu.arena.tiles.key.is_none());
    }
}
