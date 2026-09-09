//! Finite triangle visibility for ink near curved surfaces and touching solids.
//!
//! A projected polygon table and conservative screen tiles include triangles that win no
//! depth sample. Each tile owns a contiguous list of (triangle, nearest possible depth)
//! pairs. Projection, count, prefix scan and fill run only when their inputs change.
use super::buffers::{GpuCtx, ROWS, bind_group, uniform_buffer, zeroed_buffer};
use super::frame::Binds;
use crate::engine::pipelines::Layouts;

pub(super) const PROJECTED_BYTES: u64 = 96;
const MAX_TILES: u32 = 262_144;
const REFERENCES_PER_TILE: u64 = 32;

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

    /// Pooled capacity, rather than a per-tile cap: dense tiles may use spare space anywhere.
    fn buffer_bytes(self) -> u64 {
        self.header_records() * 16 + self.count() as u64 * REFERENCES_PER_TILE * 8
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
        }
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
        let layout = TileLayout::new(size);
        if self.layout != Some(layout) {
            self.buffer = zeroed_buffer(
                &ctx.device,
                "triangle.tiles",
                layout.buffer_bytes().min(limit),
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
        self.invalidate();
    }

    /// Exact owned buffer capacities and the one-byte-per-tile raster target.
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let pixels = self.layout.map_or(0, TileLayout::count) as u64;
        (
            self.buffer.size() + self.projected.size() + self.live_count.size(),
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
        let raster_pipeline_layout = pipeline_layout(
            device,
            "triangle.tiles",
            &[
                &layouts.mvp,
                &layouts.line,
                &layouts.instance,
                &raster_layout,
            ],
        );
        let count = raster_pipeline(device, &raster_pipeline_layout, &raster_shader, "fs_count");
        let fill = raster_pipeline(device, &raster_pipeline_layout, &raster_shader, "fs_fill");
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

/// Keep pipeline group order visible at each call site.
fn pipeline_layout(
    device: &wgpu::Device,
    label: &str,
    layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::PipelineLayout {
    let mut groups = Vec::with_capacity(layouts.len());
    for layout in layouts {
        groups.push(Some(*layout));
    }
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &groups,
        immediate_size: 0,
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
fn raster_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    entry: &str,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(entry),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(entry),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::R8Unorm,
                blend: None,
                write_mask: wgpu::ColorWrites::empty(),
            })],
        }),
        multiview_mask: None,
        cache: None,
    })
}

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
            assert!(layout.buffer_bytes() < 128 * 1024 * 1024);
            assert!(layout.width * layout.span >= size.0 && layout.height * layout.span >= size.1);
        }
        assert_eq!(TileLayout::new((1800, 1400)).span, 4);
        assert_eq!(TileLayout::new((2800, 1800)).span, 8);
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
        assert_eq!(initial, (128, 0));
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
