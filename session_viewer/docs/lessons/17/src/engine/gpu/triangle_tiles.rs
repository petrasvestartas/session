// --8<-- [start:tile-layout]
// Finite visibility = ink hides only behind a triangle that really covers its pixel, not behind that plane extended past the edges.
// Screen tile = a square of 4 x 4 pixels (8 x 8 on large canvases) that keeps a list of the triangles that may cover it.
// Reference pool = one buffer with every tile's list back to back; a prefix sum gives each list its start.
// Prefix sum = each entry's total of the entries before it: counts 3, 0, 2 give starts 0, 3, 3.
use super::buffers::{GpuCtx, ROWS, bind_group, resource_group, replace_buffer, uniform_buffer, zeroed_buffer};
use super::frame::Binds;
use super::targets::{Attachment, TextureSpec};
use crate::engine::pipelines::bindings::{buffer_entry, texture_entry};
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, Lazy, Pipeline, PipelineDesc, Shader, Target, build,
    count_pipeline, pipeline_layout, wgsl,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

/// Bytes per projected triangle record.
use wgpu::{BufferBindingType, TextureSampleType};
pub(super) const PROJECTED_BYTES: u64 = 96;

/// Projection workgroups a dispatch row, under the 65535 limit; 64 triangles each.
const PROJECT_ROW_GROUPS: u32 = 32768;

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
    width: u32,  // tiles across
    height: u32, // tiles down
    span: u32,   // pixels per tile side
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
// --8<-- [end:tile-layout]

// --8<-- [start:pool-report]
/// Reads back how many words the last scan needed.
// The GPU knows how long the lists got; the CPU learns it frames later and grows the pool for the next build.
struct PoolReport {
    buffer: wgpu::Buffer, // 16-byte CPU-readable copy
    ready: Arc<AtomicU8>, // 0 waiting, 1 mapped, 2 failed
    copied: bool,         // a copy was encoded this frame
    inflight: bool,       // the copy is being mapped
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
// --8<-- [end:pool-report]

// --8<-- [start:pool-sizing]
/// Projection workgroups across and down for `triangles`, 64 each, rows of PROJECT_ROW_GROUPS.
fn project_groups(triangles: u32) -> (u32, u32) {
    let groups = triangles.div_ceil(64);
    (
        groups.min(PROJECT_ROW_GROUPS),
        groups.div_ceil(PROJECT_ROW_GROUPS),
    )
}

/// Records for a table of `need` triangles, None while `capacity` fits: exact after a load jump, an eighth spare after an edit.
fn projected_records(need: u64, capacity: u64, most: u64) -> Option<u64> {
    if need <= capacity && need * 4 >= capacity {
        return None;
    }

    let spare = if need > capacity * 3 / 2 { 0 } else { need / 8 };
    Some((need + spare).min(most).max(need))
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
// --8<-- [end:pool-sizing]

// --8<-- [start:tile-struct]
/// What the tile lists were built for; same key = reuse them.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ProjectionKey {
    matrix: [f32; 16], // camera matrix
    objects: u64,      // object change count
}

/// The layouts and pipelines of the tile passes.
struct TilePipelines {
    project_layout: wgpu::BindGroupLayout, // vertices, rows, indices, projected, count, slot table
    raster_layout: wgpu::BindGroupLayout,  // projected, tiles
    scan_layout: wgpu::BindGroupLayout,    // tiles
    project: Lazy<wgpu::ComputePipeline>,  // triangles to screen space
    count: Pipeline,                       // count triangles per tile
    fill: Pipeline,                        // write triangle lists per tile
    scans: [Lazy<wgpu::ComputePipeline>; 3], // prefix sum of the counts, three levels
}

/// Which triangles cover which screen tiles, rebuilt when the camera moves.
pub struct TriangleTiles {
    pub buffer: wgpu::Buffer,      // tile headers and reference pool
    pub projected: wgpu::Buffer,   // one screen-space record per triangle
    requested_triangles: u32,      // triangles in the scene
    layout: Option<TileLayout>,    // current grid, None when empty
    target: Option<Attachment>,    // one pixel per tile, drawn into but never read
    live_count: wgpu::Buffer,      // triangle count, for the shaders
    key: Option<ProjectionKey>,    // what the projection was built for
    binned: Option<ProjectionKey>, // what the tile lists were built for
    pipes: TilePipelines,          // pipelines
    project_group: Option<(wgpu::BindGroup, [wgpu::Buffer; 3], wgpu::TextureView)>, // projection bindings and the geometry they bind
    raster: wgpu::BindGroup, // projected records and tiles, for binning
    scan: wgpu::BindGroup,   // tiles, for the prefix sum
    pool_words: u64,         // reference pool size, words
    report: PoolReport,      // readback of the words needed
}
// --8<-- [end:tile-struct]

// --8<-- [start:tiles-prepare]
impl TriangleTiles {
    /// Create with tiny placeholder buffers.
    pub fn new(ctx: &GpuCtx, layouts: &Layouts) -> Self {
        let buffer = zeroed_buffer(&ctx.device, "triangle.tiles", 16, ROWS);
        let projected = zeroed_buffer(&ctx.device, "triangle.projected", PROJECTED_BYTES, ROWS);
        let pipes = TilePipelines::new(ctx, layouts);
        let (raster, scan) = pipes.groups(ctx, &buffer, &projected);
        Self {
            buffer,
            projected,
            requested_triangles: 0,
            layout: None,
            target: None,
            live_count: uniform_buffer(&ctx.device, "triangle.project.count", &[0u32; 4]),
            key: None,
            binned: None,
            pipes,
            project_group: None,
            raster,
            scan,
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
        self.binned = None;
    }

    /// Rebind the tables after a buffer moved.
    fn rebind(&mut self, ctx: &GpuCtx) {
        (self.raster, self.scan) = self.pipes.groups(ctx, &self.buffer, &self.projected);
        self.project_group = None;
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

        // new triangle count: a new table only when it no longer fits or is mostly empty
        if triangles != self.requested_triangles || self.layout.is_none() {
            let capacity = self.projected.size() / PROJECTED_BYTES;

            if let Some(records) =
                projected_records(u64::from(triangles), capacity, limit / PROJECTED_BYTES)
            {
                replace_buffer(
                    &mut self.projected,
                    zeroed_buffer(
                        &ctx.device,
                        "triangle.projected",
                        records * PROJECTED_BYTES,
                        ROWS,
                    ),
                );
                changed = true;
            }

            ctx.queue.write_buffer(
                &self.live_count,
                0,
                bytemuck::cast_slice(&[triangles, 0u32, 0, 0]),
            );
            self.requested_triangles = triangles;
            self.invalidate();
        }

        // new grid or bigger pool: new tile buffer
        if self.layout != Some(layout) || grow {
            self.pool_words = pool_words;
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
                    format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                    samples: 1,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                },
            ));
            self.layout = Some(layout);
            self.invalidate();
            changed = true;
        }

        if changed {
            self.rebind(ctx);
        }

        changed
    }
    // --8<-- [end:tiles-prepare]

    // --8<-- [start:tiles-encode]
    /// Reproject the triangles unless camera and objects are unchanged; bin them into the tile
    /// lists too when `lists`.
    pub(super) fn encode(
        &mut self,
        ctx: &GpuCtx,
        encoder: &mut wgpu::CommandEncoder,
        input: TileInput<'_>,
        lists: bool,
    ) {
        let Some(layout) = self.layout else {
            return;
        };
        let key = ProjectionKey {
            matrix: input.matrix,
            objects: input.objects_revision,
        };

        if self.key != Some(key) {
            self.project(ctx, encoder, &input);
            self.key = Some(key);
        }

        if !lists || self.binned == Some(key) {
            return;
        }

        // zero the tile headers
        encoder.clear_buffer(&self.buffer, 0, Some(layout.header_records() * 16));
        // count triangles per tile
        self.bin(encoder, input.binds, &self.pipes.count);
        let blocks = layout.count().div_ceil(256);

        // prefix sum gives each tile its list offset
        for (index, count) in [blocks, blocks.div_ceil(256), blocks]
            .into_iter()
            .enumerate()
        {
            // a compute pass runs a shader over a grid of workgroups: no vertices, no pixels, only buffers
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("triangle.scan"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipes.scans[index]);
            pass.set_bind_group(0, input.binds.line, &[]);
            pass.set_bind_group(1, &self.scan, &[]);
            pass.dispatch_workgroups(count, 1, 1);
        }

        // write the triangle lists
        self.bin(encoder, input.binds, &self.pipes.fill);
        self.report.copy(encoder, &self.buffer);
        self.binned = Some(key);
    }

    /// Test ink against the fitted planes alone until the lists are built again.
    pub(super) fn drop_lists(&mut self, encoder: &mut wgpu::CommandEncoder) {
        // a zero first record reads as no lists; an invalidated buffer still holds the old ones
        self.binned = None;
        encoder.clear_buffer(&self.buffer, 0, Some(16));
    }

    /// Project every triangle to the screen.
    fn project(&mut self, ctx: &GpuCtx, encoder: &mut wgpu::CommandEncoder, input: &TileInput<'_>) {
        // the geometry buffers move when the scene grows
        let stale = self.project_group.as_ref().is_none_or(|(_, bound, table)| {
            table != input.table || bound.iter().zip(input.geometry).any(|(a, b)| a != b)
        });

        if stale {
            let group = resource_group(ctx, &self.pipes.project_layout, "triangle.project.bindings", [
                (0, input.geometry[0].as_entire_binding()),
                (1, input.geometry[1].as_entire_binding()),
                (2, input.geometry[2].as_entire_binding()),
                (3, self.projected.as_entire_binding()),
                (4, self.live_count.as_entire_binding()),
                (5, wgpu::BindingResource::TextureView(input.table)),
            ]);
            self.project_group = Some((
                group,
                input.geometry.map(|buffer| buffer.clone()),
                input.table.clone(),
            ));
        }

        let Some((group, _, _)) = &self.project_group else {
            return;
        };
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("triangle.project"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&self.pipes.project);
        pass.set_bind_group(0, input.binds.mvp, &[]);
        pass.set_bind_group(1, input.binds.line, &[]);
        pass.set_bind_group(2, input.binds.instances, &[]);
        pass.set_bind_group(3, group, &[]);
        let (x, y) = project_groups(self.requested_triangles);
        pass.dispatch_workgroups(x, y, 1);
    }

    /// Draw every triangle over the tile grid with `pipeline`.
    fn bin(&self, encoder: &mut wgpu::CommandEncoder, binds: &Binds, pipeline: &Pipeline) {
        // the rasterizer bins: the target has one pixel per tile, so each fragment is one (triangle, tile) pair
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
        pass.set_bind_group(3, &self.raster, &[]);
        // a four-corner strip per triangle, covering its tiles
        pass.draw(0..4, 0..self.requested_triangles);
    }
    // --8<-- [end:tiles-encode]

    // --8<-- [start:tiles-release]
    /// Shrink the buffers back to placeholders; returns true if they were bigger.
    fn release_data(&mut self, ctx: &GpuCtx) -> bool {
        if self.layout.take().is_none() {
            return false;
        }

        replace_buffer(
            &mut self.buffer,
            zeroed_buffer(&ctx.device, "triangle.tiles", 16, ROWS),
        );
        replace_buffer(
            &mut self.projected,
            zeroed_buffer(&ctx.device, "triangle.projected", PROJECTED_BYTES, ROWS),
        );
        self.target = None;
        self.invalidate();
        self.rebind(ctx);
        true
    }

    /// Free the tables while nothing reads them; true when a buffer moved.
    pub fn release_unread(&mut self, ctx: &GpuCtx) -> bool {
        self.release_data(ctx)
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
// --8<-- [end:tiles-release]

// --8<-- [start:tile-input]
/// What `encode` needs from the frame and the arena.
pub(super) struct TileInput<'a> {
    pub binds: &'a Binds<'a>,            // bind groups 0-2
    pub geometry: [&'a wgpu::Buffer; 3], // vertices, object rows, indices
    pub table: &'a wgpu::TextureView,    // where the instances' triangle ids lie
    pub matrix: [f32; 16],               // camera matrix
    pub objects_revision: u64,           // object change count
}

/// One buffer binding for a layout.
// --8<-- [end:tile-input]

// --8<-- [start:tile-pipelines]
impl TilePipelines {
    /// Create the layouts, shaders and pipelines.
    fn new(ctx: &GpuCtx, layouts: &Layouts) -> Self {
        use BufferBindingType::{Storage, Uniform};
        use wgpu::ShaderStages as Stages;
        let device = &ctx.device;
        let project_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("triangle.project.layout"),
            entries: &[
                buffer_entry(0, Stages::COMPUTE, Storage { read_only: true }),
                buffer_entry(1, Stages::COMPUTE, Storage { read_only: true }),
                buffer_entry(2, Stages::COMPUTE, Storage { read_only: true }),
                buffer_entry(3, Stages::COMPUTE, Storage { read_only: false }),
                buffer_entry(4, Stages::COMPUTE, Uniform),
                texture_entry(5, Stages::COMPUTE, TextureSampleType::Uint, false),
            ],
        });
        let raster_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("triangle.tiles.layout"),
            entries: &[
                buffer_entry(0, Stages::VERTEX, Storage { read_only: true }),
                buffer_entry(1, Stages::FRAGMENT, Storage { read_only: false }),
            ],
        });
        let scan_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("triangle.scan.layout"),
            entries: &[buffer_entry(
                0,
                Stages::COMPUTE,
                Storage { read_only: false },
            )],
        });
        let project_shader = wgsl(
            ctx,
            "triangle.project",
            shader!("project_triangles.wgsl").to_owned(),
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
        let project = compute_pipeline(ctx, &project_pipeline_layout, &project_shader, "cs_main");
        let raster_shader = wgsl(
            ctx,
            "triangle.tiles",
            shader!("triangle_tiles.wgsl").to_owned(),
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
            wgpu::PrimitiveTopology::TriangleStrip,
        )
        .depth(DepthMode::Detached)
        .color(ColorWrite::Nothing);
        let tile_target = Target {
            format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
            samples: 1,
        };
        let count = build(ctx, tile_target, &raster.with("fs_count", "fs_count"));
        let fill = build(ctx, tile_target, &raster.with("fs_fill", "fs_fill"));
        let scan_shader = wgsl(
            ctx,
            "triangle.scan",
            shader!("scan_triangle_tiles.wgsl").to_owned(),
        );
        let scan_pipeline_layout =
            pipeline_layout(device, "triangle.scan", &[&layouts.line, &scan_layout]);
        let scans = [
            compute_pipeline(ctx, &scan_pipeline_layout, &scan_shader, "scan_tiles"),
            compute_pipeline(ctx, &scan_pipeline_layout, &scan_shader, "scan_blocks"),
            compute_pipeline(ctx, &scan_pipeline_layout, &scan_shader, "finish_offsets"),
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

    /// The binning and scan bindings of `tiles` and `projected`.
    fn groups(
        &self,
        ctx: &GpuCtx,
        tiles: &wgpu::Buffer,
        projected: &wgpu::Buffer,
    ) -> (wgpu::BindGroup, wgpu::BindGroup) {
        let raster = bind_group(
            ctx,
            &self.raster_layout,
            "triangle.tiles.bindings",
            &[projected, tiles],
        );
        let scan = bind_group(ctx, &self.scan_layout, "triangle.scan.bindings", &[tiles]);
        (raster, scan)
    }
}

/// A compute pipeline for one entry point, compiled on first use.
fn compute_pipeline(
    ctx: &GpuCtx,
    layout: &wgpu::PipelineLayout,
    shader: &Shader,
    entry: &'static str,
) -> Lazy<wgpu::ComputePipeline> {
    let device = ctx.device.clone();
    let layout = layout.clone();
    let shader = shader.clone();
    Lazy::new(move || {
        count_pipeline();
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(entry),
            layout: Some(&layout),
            module: &shader,
            entry_point: Some(entry),
            compilation_options: Default::default(),
            cache: None,
        })
    })
}
// --8<-- [end:tile-pipelines]
