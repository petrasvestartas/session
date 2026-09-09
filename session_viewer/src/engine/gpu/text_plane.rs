//! Fixed world-plane coverage text: retained shaped glyphs, bounded raster textures, projected quads.
use super::super::buffers::{GpuCtx, GrowBuf, VERTS};
use super::TextFrame;
use crate::engine::pipelines::Target;
use crate::engine::text::{TextDocument, TextLabel, TextPlacement, TextRun};
use glyphon::{FontSystem, SwashCache, SwashContent};

/// Hard texture payload budget, independent of adapter limits and the rest of the scene.
const TEXTURE_BUDGET: u64 = 32 * 1024 * 1024;
/// A coverage texture retains its source layout and grows resolution only when necessary.
struct CachedPlane {
    label: TextLabel,
    font_revision: u64,
    em_pixels: u32,
    size: [u32; 2],
    extent: [f32; 4],
    _texture: wgpu::Texture,
    bind: wgpu::BindGroup,
}

/// Application-owned fixed-plane resources; camera movement only updates quad vertices.
pub(super) struct Planes {
    cached: Vec<CachedPlane>,
    draws: Vec<(usize, u32)>,
    vertices: GrowBuf,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    pipeline: wgpu::RenderPipeline,
    id_pipeline: wgpu::RenderPipeline,
    target: Target,
    pub(super) rasterizations: u64,
}

impl Planes {
    /// Share the viewer device and target while keeping coverage textures explicitly owned.
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        let layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("world text texture"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("world text coverage"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let id_pipeline = pipeline(ctx, Target::ID, &layout, true);
        let pipeline = pipeline(ctx, target, &layout, false);
        Self {
            cached: Vec::new(),
            draws: Vec::new(),
            vertices: GrowBuf::new(ctx, "world text vertices", 64, VERTS),
            layout,
            sampler,
            pipeline,
            id_pipeline,
            target,
            rasterizations: 0,
        }
    }

    /// Retarget only the draw pipeline; sampled coverage survives sample-count changes.
    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.pipeline = pipeline(ctx, target, &self.layout, false);
    }

    /// Keep current source textures, grow resolution in powers of two, and project exact plane axes.
    pub(super) fn prepare(
        &mut self,
        ctx: &GpuCtx,
        document: &mut TextDocument,
        raster: &mut SwashCache,
        frame: &TextFrame,
    ) -> anyhow::Result<()> {
        self.draws.clear();
        self.vertices.reset();
        let mut retained = Vec::new();
        for cached in self.cached.drain(..) {
            for run in &document.runs {
                if run.label.id == cached.label.id
                    && matches!(run.label.placement, TextPlacement::WorldPlane { .. })
                {
                    retained.push(cached);
                    break;
                }
            }
        }
        self.cached = retained;
        let mut vertices = Vec::new();
        for run in &document.runs {
            if !matches!(run.label.placement, TextPlacement::WorldPlane { .. })
                || run.label.text.is_empty()
            {
                continue;
            }
            let em_pixels = raster_em(&run.label, frame);
            let mut index = None;
            for (at, cached) in self.cached.iter().enumerate() {
                if cached.label.id == run.label.id {
                    index = Some(at);
                    break;
                }
            }
            let rebuild = match index {
                Some(at) => !same_raster(
                    &self.cached[at],
                    &run.label,
                    document.font_revision,
                    em_pixels,
                ),
                None => true,
            };
            if rebuild {
                let (pixels, size, extent) =
                    rasterize(run, &mut document.fonts, raster, em_pixels)?;
                let mut allocated = u64::from(size[0]) * u64::from(size[1]);
                for (at, cached) in self.cached.iter().enumerate() {
                    if Some(at) != index {
                        allocated += u64::from(cached.size[0]) * u64::from(cached.size[1]);
                    }
                }
                anyhow::ensure!(
                    allocated <= TEXTURE_BUDGET,
                    "world text coverage exceeds 32 MiB budget"
                );
                let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("world text coverage"),
                    size: wgpu::Extent3d {
                        width: size[0],
                        height: size[1],
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::R8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });
                ctx.queue.write_texture(
                    texture.as_image_copy(),
                    &pixels,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(size[0]),
                        rows_per_image: Some(size[1]),
                    },
                    texture.size(),
                );
                let view = texture.create_view(&Default::default());
                let bind = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("world text coverage"),
                    layout: &self.layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&self.sampler),
                        },
                    ],
                });
                let cached = CachedPlane {
                    label: run.label.clone(),
                    font_revision: document.font_revision,
                    em_pixels,
                    size,
                    extent,
                    _texture: texture,
                    bind,
                };
                match index {
                    Some(at) => self.cached[at] = cached,
                    None => {
                        index = Some(self.cached.len());
                        self.cached.push(cached);
                    }
                }
                self.rasterizations += 1;
                // Completed plane textures no longer depend on Swash images; bound the shared
                // CPU image cache before preparing another source or Glyphon's draw lists.
                if raster.image_cache.len() > 4096 {
                    *raster = SwashCache::new();
                }
            }
            let index = index.expect("a prepared world plane has a cache entry");
            let start = vertices.len() as u32;
            append_quad(
                &mut vertices,
                &run.label,
                self.cached[index].extent,
                frame,
                self.target.format.is_srgb(),
            );
            self.draws.push((index, start));
        }
        self.vertices.append(ctx, &vertices);
        Ok(())
    }

    /// Render fixed-plane quads with interpolated perspective UVs and physical reverse-Z occlusion.
    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        if self.draws.is_empty() {
            return 0;
        }
        self.draw_run(pass, &self.pipeline)
    }

    /// Picking uses exactly the visible plane footprint, excluding annotations with no owner;
    /// the pick pass's window transform sits at group 1.
    pub(super) fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        pass.set_bind_group(1, pick_transform, &[]);
        self.draw_run(pass, &self.id_pipeline)
    }

    /// Both passes use retained projected vertices and the same clipping/depth test.
    fn draw_run(&self, pass: &mut wgpu::RenderPass<'_>, pipeline: &wgpu::RenderPipeline) -> u32 {
        if self.draws.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        for &(index, start) in &self.draws {
            pass.set_bind_group(0, &self.cached[index].bind, &[]);
            pass.draw(start..start + 6, 0..1);
        }
        self.draws.len() as u32
    }

    /// Clear source textures immediately; retained buffers carry no live draws.
    pub(super) fn reset(&mut self) {
        self.cached.clear();
        self.draws.clear();
        self.vertices.reset();
    }
    /// Return both coverage textures and grown vertex capacity on scene disposal.
    pub(super) fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.vertices.release(ctx);
    }
    /// Exact owned vertex-buffer capacity, separate from sampled texture payload.
    pub(super) fn buffer_bytes(&self) -> u64 {
        self.vertices.buf.size()
    }
    /// Exact R8 texture payload; excludes driver allocation granularity.
    pub(super) fn texture_bytes(&self) -> u64 {
        let mut total = 0;
        for cached in &self.cached {
            total += u64::from(cached.size[0]) * u64::from(cached.size[1]);
        }
        total
    }
}

/// Placement and color changes keep coverage; source/font changes invalidate it.
fn same_raster(cached: &CachedPlane, label: &TextLabel, revision: u64, em_pixels: u32) -> bool {
    cached.font_revision == revision
        && cached.label.text == label.text
        && cached.label.font_size == label.font_size
        && cached.label.line_height == label.line_height
        && cached.em_pixels >= em_pixels
}

/// Project one rebased world point without discarding clip W, which the GPU needs for perspective.
fn project(world: [f64; 3], frame: &TextFrame) -> [f32; 4] {
    let point = [
        (world[0] - frame.origin[0]) as f32,
        (world[1] - frame.origin[1]) as f32,
        (world[2] - frame.origin[2]) as f32,
        1.0,
    ];
    let mut clip = [0.0; 4];
    for (row, out) in clip.iter_mut().enumerate() {
        for (column, value) in point.iter().enumerate() {
            *out += frame.mvp[column * 4 + row] * value;
        }
    }
    clip
}

/// Choose a bounded supersampled em bucket; small camera motion reuses the existing texture.
fn raster_em(label: &TextLabel, frame: &TextFrame) -> u32 {
    let TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
        ..
    } = label.placement
    else {
        return 32;
    };
    let a = project(world, frame);
    if a[3] <= 0.0 {
        return 32;
    }
    let mut projected_em = 0.0f32;
    for direction in [right, up] {
        let mut end = world;
        for axis in 0..3 {
            end[axis] += direction[axis] * world_height;
        }
        let b = project(end, frame);
        if b[3] <= 0.0 {
            continue;
        }
        let x = (a[0] / a[3] - b[0] / b[3]) * frame.framebuffer[0] as f32 * 0.5;
        let y = (a[1] / a[3] - b[1] / b[3]) * frame.framebuffer[1] as f32 * 0.5;
        projected_em = projected_em.max(x.hypot(y));
    }
    let needed = (projected_em * 2.0).clamp(32.0, 256.0);
    let mut bucket = 32;
    while (bucket as f32) < needed {
        bucket *= 2;
    }
    bucket
}

/// Rasterize positioned shaped glyphs, including bearings and baseline, into one coverage texture.
fn rasterize(
    run: &TextRun,
    fonts: &mut FontSystem,
    raster: &mut SwashCache,
    em_pixels: u32,
) -> anyhow::Result<(Vec<u8>, [u32; 2], [f32; 4])> {
    let scale = em_pixels as f32 / run.label.font_size;
    let mut glyphs = Vec::new();
    let mut bounds = [0i32; 4];
    for line in run.buffer.layout_runs() {
        bounds[2] = bounds[2].max((line.line_w * scale).ceil() as i32);
        bounds[3] = bounds[3].max(((line.line_top + line.line_height) * scale).ceil() as i32);
        anyhow::ensure!(
            bounds[2] <= 4092 && bounds[3] <= 4092,
            "world text layout exceeds 4096px extent"
        );
        for glyph in line.glyphs {
            let physical = glyph.physical((0.0, 0.0), scale);
            let Some(image) = raster.get_image(fonts, physical.cache_key) else {
                continue;
            };
            let x = physical.x + image.placement.left;
            let y = (line.line_y * scale).round() as i32 + physical.y - image.placement.top;
            bounds[0] = bounds[0].min(x);
            bounds[1] = bounds[1].min(y);
            bounds[2] = bounds[2].max(x + image.placement.width as i32);
            bounds[3] = bounds[3].max(y + image.placement.height as i32);
            glyphs.push((physical.cache_key, x, y));
        }
    }
    // Match the selection-name plate proportions. Whole caps stay outside the glyph box.
    let vertical_padding = (run.label.font_size * (2.0 / 9.0) * scale).ceil() as i32;
    let horizontal_padding = (bounds[3] - bounds[1] + 2 * vertical_padding + 1) / 2;
    bounds[0] -= horizontal_padding;
    bounds[1] -= vertical_padding;
    bounds[2] += horizontal_padding;
    bounds[3] += vertical_padding;
    let size = [
        (bounds[2] - bounds[0]) as u32,
        (bounds[3] - bounds[1]) as u32,
    ];
    anyhow::ensure!(
        size[0] <= 4096 && size[1] <= 4096,
        "world text texture exceeds 4096px extent"
    );
    let mut pixels = vec![0u8; (size[0] * size[1]) as usize];
    for (key, x, y) in glyphs {
        let Some(image) = raster.get_image(fonts, key) else {
            continue;
        };
        for row in 0..image.placement.height {
            for column in 0..image.placement.width {
                let source = (row * image.placement.width + column) as usize;
                let coverage = match image.content {
                    SwashContent::Mask => image.data[source],
                    SwashContent::Color => image.data[source * 4 + 3],
                    SwashContent::SubpixelMask => {
                        ((u16::from(image.data[source * 4])
                            + u16::from(image.data[source * 4 + 1])
                            + u16::from(image.data[source * 4 + 2]))
                            / 3) as u8
                    }
                };
                let target = ((y - bounds[1] + row as i32) as u32 * size[0]
                    + (x - bounds[0] + column as i32) as u32) as usize;
                let previous = u16::from(pixels[target]);
                pixels[target] = (previous + u16::from(coverage) * (255 - previous) / 255) as u8;
            }
        }
    }
    Ok((
        pixels,
        size,
        [
            bounds[0] as f32 / scale,
            bounds[1] as f32 / scale,
            bounds[2] as f32 / scale,
            bounds[3] as f32 / scale,
        ],
    ))
}

/// Expand a shaped line into its fixed right/up axes; every vertex retains full clip coordinates.
fn append_quad(
    vertices: &mut Vec<[f32; 16]>,
    label: &TextLabel,
    extent: [f32; 4],
    frame: &TextFrame,
    srgb: bool,
) {
    let TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
    } = label.placement
    else {
        return;
    };
    let unit = world_height / f64::from(label.font_size);
    let mut color = [0.0; 4];
    for (index, component) in color.iter_mut().enumerate() {
        let value = f32::from(label.ink_color()[index]) / 255.0;
        *component = if srgb && index < 3 {
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        } else {
            value
        };
    }
    let scale = frame.framebuffer[0] as f32 / frame.logical[0] as f32;
    let mut bounds = [
        0.0,
        0.0,
        frame.framebuffer[0] as f32,
        frame.framebuffer[1] as f32,
    ];
    if let Some(clip) = label.clip {
        for (index, value) in bounds.iter_mut().enumerate() {
            *value = clip[index] * scale;
        }
    }
    for [u, v] in [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 1.0],
        [1.0, 0.0],
    ] {
        let x = f64::from(extent[0] + (extent[2] - extent[0]) * u) * unit;
        let y = f64::from(extent[1] + (extent[3] - extent[1]) * v) * unit;
        let mut point = world;
        for axis in 0..3 {
            point[axis] += right[axis] * x - up[axis] * y;
        }
        let clip = project(point, frame);
        vertices.push([
            clip[0],
            clip[1],
            clip[2],
            clip[3],
            u,
            v,
            color[0],
            color[1],
            color[2],
            color[3],
            bounds[0],
            bounds[1],
            bounds[2],
            bounds[3],
            f32::from_bits(label.object.map_or(0, |object| object.row + 1)),
            if label.object.is_some_and(|object| object.selected) {
                1.0
            } else {
                0.0
            },
        ]);
    }
}

/// Coverage blending uses perspective-correct UVs and the same physical depth convention as solids.
fn pipeline(
    ctx: &GpuCtx,
    target: Target,
    layout: &wgpu::BindGroupLayout,
    ids: bool,
) -> wgpu::RenderPipeline {
    let shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("world text shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text_plane.wgsl").into()),
        });
    let pick = crate::engine::gpu::frame::pick_transform_layout(ctx);
    let colour_groups = [Some(layout)];
    let id_groups = [Some(layout), Some(&pick)];
    let pipeline_layout = ctx
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("world text"),
            bind_group_layouts: if ids { &id_groups } else { &colour_groups },
            immediate_size: 0,
        });
    ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("world text"), layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState { module: &shader, entry_point: Some(if ids { "vs_id" } else { "vs_main" }), buffers: &[wgpu::VertexBufferLayout { array_stride: 64, step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4, 4 => Uint32, 5 => Float32] }], compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some(if ids { "fs_id" } else { "fs_main" }), targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) }, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
        primitive: Default::default(), depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(false), depth_compare: Some(wgpu::CompareFunction::GreaterEqual), stencil: Default::default(), bias: Default::default() }),
        multisample: wgpu::MultisampleState { count: target.samples, ..Default::default() }, multiview_mask: None, cache: None,
    })
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
    use session_rust::{RenderVertex, Xform};

    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn fixed_plane_obeys_solid_depth_orientation_cache_and_release() {
        let mut gpu = pollster::block_on(Gpu::new_headless(320, 160)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.lit = false;
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity().m, 0));
        for position in [
            [-1.0, -1.0, 0.7],
            [1.0, -1.0, 0.7],
            [1.0, 1.0, 0.7],
            [-1.0, 1.0, 0.7],
        ] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [0.0, 0.0, 1.0],
                color: [0.3, 0.5, 0.7, 1.0],
            });
            upload.arena.vids.push(0);
        }
        upload.arena.idx = vec![0, 1, 2, 0, 2, 3];
        gpu.set_scene(&upload);
        let input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let baseline = gpu.render_offscreen(&input);
        let mut label = TextLabel {
            object: None,
            id: 7,
            text: "Fixed plane".into(),
            font_size: 18.0,
            line_height: 26.0,
            color: [255; 4],
            placement: TextPlacement::WorldPlane {
                world: [-0.8, 0.6, 0.5],
                right: [1.0, 0.0, 0.0],
                up: [0.0, 1.0, 0.0],
                world_height: 0.25,
            },
            clip: None,
        };
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        assert_eq!(
            baseline,
            gpu.render_offscreen(&input),
            "a solid fully hides the text plane"
        );
        let rasterizations = gpu.text.stats.world_plane_rasterizations;
        label.placement = TextPlacement::WorldPlane {
            world: [-0.8, 0.6, 0.8],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            world_height: 0.25,
        };
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        let front = gpu.render_offscreen(&input);
        let mut white = 0;
        let mut black = 0;
        for pixel in front.chunks_exact(4) {
            white += usize::from(pixel[0] > 240 && pixel[1] > 240 && pixel[2] > 240);
            black += usize::from(pixel[0] < 8 && pixel[1] < 8 && pixel[2] < 8);
        }
        assert!(
            white > 40 && black > 500,
            "foreground text has white glyphs on a black plane"
        );
        assert_eq!(gpu.text.stats.world_plane_rasterizations, rasterizations);
        label.object = Some(crate::engine::text::TextObject {
            row: 0,
            selected: true,
        });
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        let selected = gpu.render_offscreen(&input);
        let yellow = selected
            .chunks_exact(4)
            .filter(|pixel| pixel[0] > 240 && pixel[1] > 240 && pixel[2] < 8)
            .count();
        let black_ink = selected
            .chunks_exact(4)
            .filter(|pixel| pixel[0] < 8 && pixel[1] < 8 && pixel[2] < 8)
            .count();
        assert!(
            yellow > 500 && black_ink > 40,
            "selected fixed text has a yellow backing and black glyphs"
        );
        assert_eq!(
            gpu.text.stats.world_plane_rasterizations, rasterizations,
            "selection reuses the coverage texture"
        );
        label.object = None;
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        assert_eq!(
            front,
            gpu.render_offscreen(&input),
            "deselect restores the original text colors"
        );
        let diagonal = std::f64::consts::FRAC_1_SQRT_2;
        label.placement = TextPlacement::WorldPlane {
            world: [-0.8, 0.6, 0.8],
            right: [diagonal, 0.0, -diagonal],
            up: [0.0, 1.0, 0.0],
            world_height: 0.25,
        };
        gpu.text.set_labels(vec![label]).unwrap();
        assert_ne!(
            front,
            gpu.render_offscreen(&input),
            "the fixed axes change projection and partial occlusion"
        );
        assert_eq!(gpu.text.stats.shape_count, 1);
        assert_eq!(gpu.text.stats.world_plane_rasterizations, rasterizations);
        assert!(gpu.text.texture_bytes() > 0);
        gpu.text.release(&gpu.ctx);
        assert_eq!(gpu.text.texture_bytes(), 0);
        assert_eq!(baseline, gpu.render_offscreen(&input));
    }
}
