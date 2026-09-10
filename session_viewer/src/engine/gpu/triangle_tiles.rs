//! Finite triangle visibility for ink near curved surfaces and touching solids.
//!
//! A projected polygon table and conservative screen tiles include triangles that win no
//! depth sample. Each tile owns a contiguous list of (triangle, nearest possible depth)
//! pairs. Projection, count, prefix scan and fill run only when their inputs change.
use super::buffers::{GpuCtx, ROWS, bind_group, uniform_buffer, zeroed_buffer};
use super::frame::Binds;
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, pipeline_layout,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

pub(super) const PROJECTED_BYTES: u64 = 96;
const MAX_TILES: u32 = 262_144;
/// The most references a tile may hold on average: the pool's ceiling, never its size.
const REFERENCES_PER_TILE: u64 = 32;
/// Words per (triangle, depth) reference.
const REFERENCE_WORDS: u64 = 2;
/// The smallest pool: enough for a screen-filling triangle in every tile of a small canvas.
const MIN_POOL_WORDS: u64 = 32 * 1024;

/// Framebuffer pixels per tile grow only when needed to bound the grid's storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TileLayout {
    width: u32,
    height: u32,
    span: u32,
}

impl TileLayout {
    /// Matches `visibility_tile_span` in the shared WGSL contract.
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

    /// Number of tile headers; each header has count, offset, cursor and overflow words.
    fn count(self) -> u32 {
        self.width * self.height
    }

    /// Extra records hold the two levels of block totals for the parallel prefix scan.
    fn header_records(self) -> u64 {
        let blocks = self.count().div_ceil(256);
        1 + self.count() as u64 + blocks as u64 + blocks.div_ceil(256) as u64
    }

    /// The headers plus a pool of `pool_words` reference words: dense tiles may use spare
    /// space anywhere, so the pool is sized for the scene, not per tile.
    fn buffer_bytes(self, pool_words: u64) -> u64 {
        self.header_records() * 16 + pool_words * 4
    }

    /// The pool's ceiling: the former fixed allocation.
    fn max_pool_words(self) -> u64 {
        self.count() as u64 * REFERENCES_PER_TILE * REFERENCE_WORDS
    }

    /// A first pool that fits a screen-filling triangle in every tile plus eight tiles per
    /// triangle; the scan reports what it really needs and the pool grows to that.
    fn initial_pool_words(self, triangles: u32) -> u64 {
        let references = self.count() as u64 * 2 + u64::from(triangles) * 8;
        (references * REFERENCE_WORDS)
            .max(MIN_POOL_WORDS)
            .min(self.max_pool_words())
    }
}

/// The scan's own report of the words its lists need, read back a frame later: a pool too
/// small keeps the conservative rejection for that frame and grows before the next.
struct PoolReport {
    buffer: wgpu::Buffer,
    ready: Arc<AtomicU8>,
    copied: bool,
    inflight: bool,
}

impl PoolReport {
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

    /// Copy the first record (valid flag, words needed) once the fill has run.
    fn copy(&mut self, encoder: &mut wgpu::CommandEncoder, tiles: &wgpu::Buffer) {
        if self.inflight {
            return;
        }
        encoder.copy_buffer_to_buffer(tiles, 0, &self.buffer, 0, 16);
        self.copied = true;
    }

    /// After the submit: map the copy once.
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

    /// The words the last scan needed, when its report has landed.
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

/// Selection changes do not change physical geometry; hiding, placement and rebasing do.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ProjectionKey {
    matrix: [f32; 16],
    objects: u64,
}

/// Shader layouts and immutable pipelines stay device-owned when scene data is released.
struct TilePipelines {
    project_layout: wgpu::BindGroupLayout,
    raster_layout: wgpu::BindGroupLayout,
    scan_layout: wgpu::BindGroupLayout,
    project: wgpu::ComputePipeline,
    count: wgpu::RenderPipeline,
    fill: wgpu::RenderPipeline,
    scans: [wgpu::ComputePipeline; 3],
}

/// One owner for all temporary geometry used by the finite visibility fallback.
pub struct TriangleTiles {
    pub buffer: wgpu::Buffer,
    pub projected: wgpu::Buffer,
    requested_triangles: u32,
    layout: Option<TileLayout>,
    target: Option<wgpu::TextureView>,
    live_count: wgpu::Buffer,
    key: Option<ProjectionKey>,
    pipes: TilePipelines,
    /// Reference words the pool holds; grows from the scan's report, resets with the scene.
    pool_words: u64,
    report: PoolReport,
}

impl TriangleTiles {
    /// Start with zeroed placeholders: an empty scene requests no projected table or tiles.
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

    /// After a submit that ran the scan: start reading its report.
    pub fn map_report(&mut self) {
        self.report.map();
    }

    /// The words the header and pool occupy for `layout` at the current pool size.
    fn pool_capacity(&self, layout: TileLayout) -> u64 {
        layout.header_records() * 4 + self.pool_words
    }

    /// Geometry may change without changing its count; never reuse a previous projection then.
    pub fn invalidate(&mut self) {
        self.key = None;
    }

    /// Resize storage before its next binding. Return whether either ink buffer was replaced.
    pub fn prepare(&mut self, ctx: &GpuCtx, size: (u32, u32), triangles: u32) -> bool {
        let limit = ctx.device.limits().max_storage_buffer_binding_size;
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
        // The scan reported what its lists needed last time: grow past it with headroom, and
        // a saturated report (the lists did not fit) doubles instead.
        let mut pool_words = self.pool_words.max(layout.initial_pool_words(triangles));
        if let Some(needed) = self.report.poll()
            && self.layout == Some(layout)
            && needed > self.pool_capacity(layout)
        {
            let saturated = needed >= self.pool_capacity(layout);
            let grown = if saturated {
                self.pool_words * 2
            } else {
                (needed - layout.header_records() * 4) * 3 / 2
            };
            pool_words = pool_words.max(grown);
        }
        let pool_words = pool_words.min(layout.max_pool_words());
        let grow = pool_words > self.pool_words;
        if triangles != self.requested_triangles || self.layout.is_none() {
            self.projected = zeroed_buffer(
                &ctx.device,
                "triangle.projected",
                triangles as u64 * PROJECTED_BYTES,
                ROWS,
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
        if self.layout != Some(layout) || grow {
            self.pool_words = pool_words;
            self.buffer = zeroed_buffer(
                &ctx.device,
                "triangle.tiles",
                layout.buffer_bytes(pool_words).min(limit),
                ROWS,
            );
            self.target = Some(
                ctx.device
                    .create_texture(&wgpu::TextureDescriptor {
                        label: Some("triangle.tiles.target"),
                        size: wgpu::Extent3d {
                            width: layout.width,
                            height: layout.height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::R8Unorm,
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    })
                    .create_view(&Default::default()),
            );
            self.layout = Some(layout);
            self.invalidate();
            changed = true;
        }
        changed
    }

    /// Project and bin each physical triangle once for this camera and object revision.
    /// The caller submits this encoder before any subsequent frame can reuse the cached data.
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
        encoder.clear_buffer(&self.buffer, 0, Some(layout.header_records() * 16));
        let raster = bind_group(
            ctx,
            &self.pipes.raster_layout,
            "triangle.tiles.bindings",
            &[&self.projected, &self.buffer],
        );
        self.bin(encoder, input.binds, &raster, &self.pipes.count);
        let scan = bind_group(
            ctx,
            &self.pipes.scan_layout,
            "triangle.scan.bindings",
            &[&self.buffer],
        );
        let blocks = layout.count().div_ceil(256);
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
        self.bin(encoder, input.binds, &raster, &self.pipes.fill);
        self.report.copy(encoder, &self.buffer);
        self.key = Some(key);
    }

    /// Rasterize conservative tile coverage, first counting and then writing the same pairs.
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
        pass.draw(0..6, 0..self.requested_triangles);
    }

    /// Release scene-sized tables without rebuilding pipelines or repeatedly replacing placeholders.
    fn release_data(&mut self, ctx: &GpuCtx) -> bool {
        if self.layout.take().is_none() {
            return false;
        }
        self.buffer = zeroed_buffer(&ctx.device, "triangle.tiles", 16, ROWS);
        self.projected = zeroed_buffer(&ctx.device, "triangle.projected", PROJECTED_BYTES, ROWS);
        self.target = None;
        self.invalidate();
        true
    }

    /// Scene replacement releases every variable-sized visibility allocation.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.release_data(ctx);
        self.requested_triangles = 0;
        self.pool_words = 0;
        self.invalidate();
    }

    /// Exact owned buffer capacities and the one-byte-per-tile raster target.
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

/// Borrowed frame and source buffers; no source topology is copied or retained here.
pub(super) struct TileInput<'a> {
    pub binds: &'a Binds<'a>,
    pub geometry: [&'a wgpu::Buffer; 3],
    pub matrix: [f32; 16],
    pub objects_revision: u64,
}

/// One explicit storage/uniform binding; each pass exposes only the stages that use it.
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
    /// Create the finite visibility passes. Their storage layouts are private to this owner.
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
        // The tile passes rasterize into an R8 target they never write: the fragment side
        // effects (counts, then list fills) are the output.
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

/// Every preparation shader shares the projected record and tile-grid arithmetic with ink.
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

/// One named compute entry; no optional features or atomics beyond core WebGPU are needed.
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

/// Tile binning is an ordinary single-sample raster pass without a depth attachment.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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

    /// A small scene starts far below the ceiling; the ceiling is the former fixed pool.
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
    fn projection_cache_tracks_camera_hidden_state_replacement_and_release() {
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
        use session_rust::{RenderVertex, Xform};
        let mut gpu = pollster::block_on(Gpu::new_headless(128, 128)).unwrap();
        gpu.view.show_grid = false;
        let initial = gpu.arena.tiles.allocated_bytes();
        // Placeholders: 16 B tiles, one projected record, the live count and the pool report.
        assert_eq!(initial, (16 + PROJECTED_BYTES + 16 + 16, 0));
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity().m, 0));
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
