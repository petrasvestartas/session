use super::{buffers::GpuCtx, targets::Targets};
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Pipeline, PipelineDesc, Target, build, module, pipeline_layout,
};

/// Layouts and pipelines for one target; kept after ambient occlusion is turned off.
pub struct SsaoPipelines {
    target: Target, // scene color format and samples
    layout: wgpu::BindGroupLayout, // depth, uniform, gradient, triangles
    sample_layout: wgpu::BindGroupLayout, // one occlusion texture
    raw: wgpu::RenderPipeline, // computes occlusion per pixel
    filter: [Pipeline; 2], // blur in x, then in y
    composite: Pipeline, // darkens the scene with it, once per pixel
    edge: Option<Pipeline>, // at 4x, again for samples on another triangle
}

/// Compile the ambient occlusion shader and its pipelines for `target`.
pub fn pipelines(ctx: &GpuCtx, target: Target) -> SsaoPipelines {
    // group 0: depth, uniform, gradient, projected triangles
    let layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient depth"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: target.samples > 1,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: target.samples > 1,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
    // group 1: one occlusion texture and the ray positions
    let sample_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient reconstruction"),
            entries: &[0, 1].map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }),
        });
    let shader = module(ctx, "ambient", &shader_source(target.samples));
    let single = Target {
        format: wgpu::TextureFormat::R16Float,
        samples: 1,
    };
    // a quarter of the samples per frame, added into two half-float sums; the ray position beside
    let add = wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::One,
        operation: wgpu::BlendOperation::Add,
    };
    crate::engine::pipelines::count_pipeline();
    let raw = ctx
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ambient hemisphere"),
            layout: Some(&pipeline_layout(&ctx.device, "ambient hemisphere", &[&layout])),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rg16Float,
                        blend: Some(wgpu::BlendState {
                            color: add,
                            alpha: add,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(wgpu::TextureFormat::R32Float.into()),
                ],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
    let groups = [&layout, &sample_layout];
    let sampled = PipelineDesc::new(
        &shader,
        &groups,
        &[],
        wgpu::PrimitiveTopology::TriangleList,
    )
    .depth(DepthMode::Detached);
    // two blur passes, one per axis
    let filter = ["fs_filter_x", "fs_filter_y"]
        .map(|entry| build(ctx, single, &sampled.with("ambient denoise", entry)));
    // multiply the scene by the occlusion
    let shade = |entry| {
        build(
            ctx,
            target,
            &sampled
                .with("ambient reconstruction", entry)
                .color(ColorWrite::Blended),
        )
    };
    let (composite, edge) = if target.samples > 1 {
        (shade("fs_composite_pixel"), Some(shade("fs_composite_edge")))
    } else {
        (shade("fs_composite"), None)
    };

    // asked for when it is about to draw, or to warm up: compile now
    for pipeline in filter.iter().chain([&composite]).chain(&edge) {
        let _: &wgpu::RenderPipeline = pipeline;
    }

    SsaoPipelines {
        target,
        layout,
        sample_layout,
        raw,
        filter,
        composite,
        edge,
    }
}

/// The pipelines for `target` from its 1x or 4x slot, compiled on first use.
pub fn cached<'a>(
    slots: &'a mut [Option<SsaoPipelines>; 2],
    ctx: &GpuCtx,
    target: Target,
) -> &'a SsaoPipelines {
    let slot = &mut slots[usize::from(target.samples > 1)];
    if slot.as_ref().is_none_or(|pipes| pipes.target != target) {
        *slot = Some(pipelines(ctx, target));
    }
    slot.as_ref().unwrap()
}

/// Screen-space ambient occlusion: soft shadows where surfaces meet.
pub struct Ssao {
    full: (u32, u32), // canvas size the textures were made for
    samples: u32, // canvas samples they were made for
    filtered: wgpu::Texture, // half-blurred occlusion
    filtered_view: wgpu::TextureView, // view of it
    filtered_group: wgpu::BindGroup, // binds it
    history: wgpu::Texture, // near and far sums over the frames so far
    history_view: wgpu::TextureView, // view of it
    history_group: wgpu::BindGroup, // binds it
    linear: wgpu::Texture, // position along each pixel's ray, signed by surface kind
    linear_view: wgpu::TextureView, // view of it
    quarter: u32, // last quarter of the samples added, 0..3
    inverse: wgpu::Buffer, // inverse camera, camera, ground, size, quarter
    texture: wgpu::Texture, // occlusion per pixel
    view: wgpu::TextureView, // view of it
    sampled: wgpu::BindGroup, // binds it
    size: (u32, u32), // occlusion texture size, px
    extent: (u32, u32), // texels the occlusion fills: the size, or half of it in a drag
    written: Option<[f32; 64]>, // uniform in the buffer
    group: Option<(wgpu::TextureView, wgpu::TextureView, wgpu::Buffer, wgpu::BindGroup)>, // depth, gradient and triangles it binds
    cached: Option<([f32; 38], u64)>, // uniform, extent and geometry the occlusion was computed for
    receiver_bounds: Option<(u64, session_rust::AABB, f32)>, // geometry revision, box of shadow receivers, largest radius
}

impl Ssao {
    /// Ground height and contact radius from the visible solids.
    pub fn receiver(&mut self, objects: &super::objects::InstanceTable) -> [f32; 2] {
        let revision = objects.geometry_revision();
        // recompute when the objects changed
        if self
            .receiver_bounds
            .as_ref()
            .is_none_or(|(r, _, _)| *r != revision)
        {
            let mut bounds = session_rust::AABB::empty();
            let mut radius = 0.01_f32;
            for i in 0..objects.len() {
                let flags = objects.row(i).unwrap().flags;
                if flags & super::Instance::FLAG_HAS_FACES != 0
                    && flags & (super::Instance::FLAG_HIDDEN | super::Instance::FLAG_SHEET) == 0
                    && let Some(b) = objects.row_bounds(i)
                {
                    bounds.union_with(&b);
                    radius = radius.max(objects.row(i).unwrap().ao_radius);
                }
            }
            self.receiver_bounds = Some((revision, bounds, radius));
        }
        let b = &self.receiver_bounds.as_ref().unwrap().1;
        let radius = self.receiver_bounds.as_ref().unwrap().2;
        // bottom of the box, relative to the scene origin
        [(b.cz - b.hz - objects.anchor()[2]) as f32, radius]
    }

    /// Create the textures and uniform for a `full`-sized canvas.
    pub fn new(ctx: &GpuCtx, pipes: &SsaoPipelines, full: (u32, u32)) -> Self {
        // 64 floats: inverse camera, camera, ground, size, quarter, pixel rays
        let inverse = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ambient inverse and ground"),
            size: 256,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        // occlusion at most 1920 px wide
        let scale = 1.0_f64.min(1920.0 / f64::from(full.0.max(full.1).max(1)));
        let size = (
            (f64::from(full.0) * scale).ceil().max(1.0) as u32,
            (f64::from(full.1) * scale).ceil().max(1.0) as u32,
        );
        let descriptor = wgpu::TextureDescriptor {
            label: Some("ambient half-float display pixels"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let linear = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ambient ray positions"),
            format: wgpu::TextureFormat::R32Float,
            ..descriptor
        });
        let linear_view = linear.create_view(&Default::default());
        // an occlusion texture beside the ray positions
        let bind = |label, view: &wgpu::TextureView| {
            ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
                layout: &pipes.sample_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&linear_view),
                    },
                ],
            })
        };
        let texture = ctx.device.create_texture(&descriptor);
        let filtered = ctx.device.create_texture(&descriptor);
        let filtered_view = filtered.create_view(&Default::default());
        let filtered_group = bind("ambient filtered texture", &filtered_view);
        let history = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ambient sums"),
            format: wgpu::TextureFormat::Rg16Float,
            ..descriptor
        });
        let history_view = history.create_view(&Default::default());
        let history_group = bind("ambient sums", &history_view);
        let view = texture.create_view(&Default::default());
        let sampled = bind("ambient texture", &view);
        Self {
            full,
            samples: pipes.target.samples,
            filtered,
            filtered_view,
            filtered_group,
            history,
            history_view,
            history_group,
            linear,
            linear_view,
            quarter: 3,
            inverse,
            texture,
            view,
            sampled,
            size,
            extent: size,
            written: None,
            group: None,
            cached: None,
            receiver_bounds: None,
        }
    }

    /// Bytes of the two occlusion textures, the sums and the ray positions.
    pub fn texture_bytes(&self) -> u64 {
        12 * u64::from(self.size.0) * u64::from(self.size.1)
    }

    /// True while the occlusion still lacks some of its samples.
    pub fn pending(&self) -> bool {
        self.quarter < 3
    }

    /// True when the occlusion on screen was computed at half resolution, in a drag.
    pub fn half(&self) -> bool {
        self.extent != self.size
    }

    /// True when the textures fit a `full` canvas drawn by `pipes`.
    pub fn fits(&self, pipes: &SsaoPipelines, full: (u32, u32)) -> bool {
        self.full == full && self.samples == pipes.target.samples
    }

    /// Compute occlusion if needed, at half resolution in a `drag`, then darken the scene;
    /// returns the draw count.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        ctx: &GpuCtx,
        pipes: &SsaoPipelines,
        targets: &Targets,
        projected: &wgpu::Buffer,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        mvp: [f32; 16],
        ground: [f32; 2],
        revision: u64,
        drag: bool,
    ) -> u32 {
        let size = self.full;
        // a drag fills a half-size corner of the textures; the next still frame all of them
        let extent = if drag {
            (self.size.0.div_ceil(2), self.size.1.div_ceil(2))
        } else {
            self.size
        };
        // clip space back to view space
        let Some(inverse) = inverse_projection(mvp) else {
            return 0;
        };
        let mut key = [0.0; 38];
        key[..16].copy_from_slice(&inverse);
        key[16..32].copy_from_slice(&mvp);
        key[32..].copy_from_slice(&[
            ground[0],
            ground[1],
            size.0 as f32,
            size.1 as f32,
            extent.0 as f32,
            extent.1 as f32,
        ]);
        // a new view starts over; a still one adds the next quarter
        let quarter = if self.cached != Some((key, revision)) {
            Some(0)
        } else {
            (self.quarter < 3).then_some(self.quarter + 1)
        };
        let mut uniform = [0.0; 64];
        uniform[..36].copy_from_slice(&key[..36]);
        uniform[36] = quarter.unwrap_or(self.quarter) as f32;
        uniform[37..39].copy_from_slice(&key[36..]);
        uniform[39] = f32::from(u8::from(drag));
        uniform[40..].copy_from_slice(&pixel_rays(&inverse, size));
        if self.written != Some(uniform) {
            ctx.queue
                .write_buffer(&self.inverse, 0, bytemuck::cast_slice(&uniform));
            self.written = Some(uniform);
        }
        // rebind only when depth, gradient or triangles moved
        if self.group.as_ref().is_none_or(|(depth, gradient, triangles, _)| {
            *depth != targets.depth.view || *gradient != targets.gradient.view || triangles != projected
        }) {
            let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("ambient"),
                layout: &pipes.layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&targets.depth),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.inverse.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&targets.gradient),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: projected.as_entire_binding(),
                    },
                ],
            });
            self.group = Some((
                targets.depth.view.clone(),
                targets.gradient.view.clone(),
                projected.clone(),
                group,
            ));
        }
        let group = &self.group.as_ref().unwrap().3;
        let mut draws = 0;
        // work only while the camera or geometry moved, or samples are missing
        if let Some(quarter) = quarter {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ambient horizons"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.history_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: if quarter == 0 {
                            wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                        } else {
                            wgpu::LoadOp::Load
                        },
                        store: wgpu::StoreOp::Store,
                    },
                }), Some(wgpu::RenderPassColorAttachment {
                    view: &self.linear_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_viewport(0.0, 0.0, extent.0 as f32, extent.1 as f32, 0.0, 1.0);
            pass.set_pipeline(&pipes.raw);
            pass.set_bind_group(0, group, &[]);
            pass.draw(0..3, 0..1);
            drop(pass);
            // blur the sums along x into filtered, then y into view
            for (pipeline, output, input) in [
                (&pipes.filter[0], &self.filtered_view, &self.history_group),
                (&pipes.filter[1], &self.view, &self.filtered_group),
            ] {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient denoise"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: output,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_viewport(0.0, 0.0, extent.0 as f32, extent.1 as f32, 0.0, 1.0);
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, group, &[]);
                pass.set_bind_group(1, input, &[]);
                pass.draw(0..3, 0..1);
            }
            self.cached = Some((key, revision));
            self.quarter = quarter;
            self.extent = extent;
            draws += 3;
        }
        // darken the scene color
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ambient composite"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: targets.msaa.as_deref().unwrap_or(view),
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_bind_group(0, group, &[]);
        pass.set_bind_group(1, &self.sampled, &[]);
        // a drag shades every sample from the first; a still frame each triangle's samples
        let edge = pipes.edge.as_ref().filter(|_| !drag);

        for pipeline in std::iter::once(&pipes.composite).chain(edge) {
            pass.set_pipeline(pipeline);
            pass.draw(0..3, 0..1);
            draws += 1;
        }
        draws
    }
}

/// Free the textures now, not when the browser collects them.
impl Drop for Ssao {
    /// Destroy the occlusion textures.
    fn drop(&mut self) {
        self.texture.destroy();
        self.filtered.destroy();
        self.history.destroy();
        self.linear.destroy();
    }
}

/// The shader text, rewritten for multisampled depth when needed.
fn shader_source(samples: u32) -> String {
    let source = include_str!("../../shaders/ssao.wgsl");
    if samples > 1 {
        source
            .replace("texture_depth_2d", "texture_depth_multisampled_2d")
            .replace(
                "physical: texture_2d<u32>",
                "physical: texture_multisampled_2d<u32>",
            )
    } else {
        source.to_owned()
    }
}

/// Near plane point and ray to depth 0.5 at pixel (0,0), and their steps per pixel in x and y.
fn pixel_rays(inverse: &[f32; 16], size: (u32, u32)) -> [f32; 24] {
    // world point at a pixel and depth, as the shader's `world`
    let world = |x: f64, y: f64, z: f64| -> [f64; 3] {
        let ndc = [x / f64::from(size.0) * 2.0 - 1.0, 1.0 - y / f64::from(size.1) * 2.0, z, 1.0];
        let p: [f64; 4] = std::array::from_fn(|row| {
            (0..4).map(|col| f64::from(inverse[col * 4 + row]) * ndc[col]).sum()
        });
        [p[0] / p[3], p[1] / p[3], p[2] / p[3]]
    };
    let near = [world(0.0, 0.0, 1.0), world(1.0, 0.0, 1.0), world(0.0, 1.0, 1.0)];
    let half = [world(0.0, 0.0, 0.5), world(1.0, 0.0, 0.5), world(0.0, 1.0, 0.5)];
    let ray: [[f64; 3]; 3] = std::array::from_fn(|i| std::array::from_fn(|k| half[i][k] - near[i][k]));
    let mut out = [0.0; 24];
    for (i, base) in [near, ray].iter().enumerate() {
        for k in 0..3 {
            out[i * 12 + k] = base[0][k] as f32;
            out[i * 12 + 4 + k] = (base[1][k] - base[0][k]) as f32;
            out[i * 12 + 8 + k] = (base[2][k] - base[0][k]) as f32;
        }
    }
    out
}

/// Invert the camera matrix; rows are scaled first to keep precision.
fn inverse_projection(matrix: [f32; 16]) -> Option<[f32; 16]> {
    let mut normalized = matrix.map(f64::from);
    let scales: [f64; 4] = std::array::from_fn(|row| {
        (0..4)
            .map(|col| normalized[col * 4 + row].abs())
            .fold(0.0, f64::max)
    });
    if scales.iter().any(|s| !s.is_finite() || *s == 0.0) {
        return None;
    }
    for row in 0..4 {
        for col in 0..4 {
            normalized[col * 4 + row] /= scales[row];
        }
    }
    let mut inverse = session_rust::Xform::from_matrix(normalized).inverse()?.m;
    for col in 0..4 {
        for row in 0..4 {
            inverse[col * 4 + row] /= scales[col];
        }
    }
    Some(inverse.map(|v| v as f32))
}

#[cfg(test)]
mod tests {
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Contact darkens pixels, far ground stays bright, memory returns.
    fn occlusion_darkens_contact_and_releases_its_small_uniform() {
        use crate::app::scene::{FileDoc, Scene};
        use crate::camera::Camera;
        use crate::engine::gpu::{FrameInput, Gpu};
        use session_rust::{BRep, Session, Xform};
        use std::rc::Rc;
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).unwrap();
        gpu.view.show_grid = false;
        let mut source = Session::new("contact");
        source.add_brep(BRep::create_box(100.0, 100.0, 10.0), None);
        let tower = source
            .add_brep(BRep::create_box(30.0, 30.0, 80.0), None)
            .unwrap();
        source.set_xform(&tower.borrow().name, Xform::translation(0.0, 0.0, 35.0));
        let raised = source
            .add_brep(BRep::create_box(40.0, 40.0, 10.0), None)
            .unwrap();
        source.set_xform(&raised.borrow().name, Xform::translation(110.0, 0.0, 12.0));
        if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
            std::fs::create_dir_all("target/review").unwrap();
            std::fs::write("target/review/ambient-contact.pb", source.pb_dumps()).unwrap();
        }
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "contact".into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, 1.0);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        for (samples, perspective) in [(1, true), (4, true), (1, false), (4, false)] {
            camera.perspective = perspective;
            let input = FrameInput {
                view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
                clear: wgpu::Color::WHITE,
                now_ms: 0.0,
            };
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(256, 256);
            gpu.view.ssao = false;
            let plain = gpu.render_offscreen(&input);
            let memory = gpu.allocated_bytes();
            gpu.view.ssao = true;
            let partial = gpu.render_offscreen(&input);
            // three more frames add the other quarters of the samples
            let shaded = (0..3).fold(Vec::new(), |_, _| gpu.render_offscreen(&input));
            if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
                std::fs::write(format!("target/review/ambient-{samples}x-off.rgba"), &plain)
                    .unwrap();
                std::fs::write(format!("target/review/ambient-{samples}x-on.rgba"), &shaded)
                    .unwrap();
            }
            for shaded in [&partial, &shaded] {
                let ground_shadow = plain
                    .chunks_exact(4)
                    .zip(shaded.chunks_exact(4))
                    .filter(|(a, b)| {
                        a[0] == 255 && a[1] == 255 && a[2] == 255 && b[0] < shaded[0].saturating_sub(3)
                    })
                    .count();
                assert!(
                    ground_shadow > 20,
                    "virtual ground receives soft shadows: {ground_shadow}"
                );
                let mut distant_ground = 0;
                for (i, (a, b)) in plain
                    .chunks_exact(4)
                    .zip(shaded.chunks_exact(4))
                    .enumerate()
                {
                    if a[..3] != [255, 255, 255] {
                        continue;
                    }
                    let (origin, direction) = camera
                        .ray(
                            ((i % 256) as f64 + 0.5, (i / 256) as f64 + 0.5),
                            (256.0, 256.0),
                        )
                        .unwrap();
                    if direction[2] >= 0.0 {
                        continue;
                    }
                    let t = (-5.0 - origin[2]) / direction[2];
                    let x = origin[0] + t * direction[0];
                    let y = origin[1] + t * direction[1];
                    let distance = (x.abs() - 50.0).max(0.0).hypot((y.abs() - 50.0).max(0.0));
                    if t > 0.0 && distance > 25.0 {
                        distant_ground += 1;
                        assert!(
                            b[0] >= shaded[0].saturating_sub(2),
                            "ground halo away from base at ({x}, {y}), {samples}x, perspective={perspective}: {}",
                            b[0]
                        );
                    }
                }
                assert!(
                    distant_ground > 1000,
                    "check exposed ground around raised geometry"
                );
                let darkened = plain
                    .chunks_exact(4)
                    .zip(shaded.chunks_exact(4))
                    .filter(|(a, b)| a[0] > b[0].saturating_add(2))
                    .count();
                assert!(
                    darkened > 20,
                    "contact occlusion changes pixels at {samples}x: {darkened}"
                );
            }
            assert_eq!(
                gpu.allocated_bytes(),
                (memory.0 + 256, memory.1 + 12 * 256 * 256)
            );
            gpu.view.ssao = false;
            assert_eq!(gpu.render_offscreen(&input), plain);
            assert_eq!(gpu.allocated_bytes(), memory);
        }
    }

    /// A headless canvas showing the contact scene, and its camera.
    #[cfg(not(target_arch = "wasm32"))]
    fn contact_scene(
        width: u32,
        height: u32,
    ) -> (crate::engine::gpu::Gpu, crate::camera::Camera, session_rust::Point) {
        use crate::app::scene::{FileDoc, Scene};
        use crate::camera::Camera;
        use crate::engine::gpu::Gpu;
        use session_rust::{BRep, Session, Xform};
        use std::rc::Rc;
        let mut gpu = pollster::block_on(Gpu::new_headless(width, height)).unwrap();
        gpu.view.show_grid = false;
        let mut source = Session::new("contact");
        source.add_brep(BRep::create_box(100.0, 100.0, 10.0), None);
        let tower = source
            .add_brep(BRep::create_box(30.0, 30.0, 80.0), None)
            .unwrap();
        source.set_xform(&tower.borrow().name, Xform::translation(0.0, 0.0, 35.0));
        let raised = source
            .add_brep(BRep::create_box(40.0, 40.0, 10.0), None)
            .unwrap();
        source.set_xform(&raised.borrow().name, Xform::translation(110.0, 0.0, 12.0));
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "contact".into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, width as f64 / height as f64);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        (gpu, camera, rebase.anchor)
    }

    /// Milliseconds of one offscreen frame, read back to the CPU.
    #[cfg(not(target_arch = "wasm32"))]
    fn timed(gpu: &mut crate::engine::gpu::Gpu, camera: &crate::camera::Camera, anchor: &session_rust::Point) -> (f64, Vec<u8>) {
        let aspect = gpu.config.width as f64 / gpu.config.height as f64;
        let input = crate::engine::gpu::FrameInput {
            view_proj: camera.view_proj_anchored(aspect, anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let start = std::time::Instant::now();
        let pixels = gpu.render_offscreen(&input);
        (start.elapsed().as_secs_f64() * 1000.0, pixels)
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "benchmark, requires a native GPU adapter"]
    /// Toggle, resize, still and orbit timings at 1920x1080; writes captures.
    fn bench_ambient() {
        let (mut gpu, mut camera, anchor) = contact_scene(1920, 1080);
        let mut report = String::new();
        gpu.view.msaa_forced = Some(4);
        gpu.resize(1920, 1080);
        timed(&mut gpu, &camera, &anchor);
        let mut toggles = Vec::new();
        for _ in 0..10 {
            gpu.view.ssao = true;
            toggles.push(timed(&mut gpu, &camera, &anchor).0);
            gpu.view.ssao = false;
            timed(&mut gpu, &camera, &anchor);
        }
        report += &format!("toggle on ms: {toggles:.1?}\n");
        gpu.view.ssao = true;
        let resizes: Vec<f64> = (1..=10)
            .map(|k| {
                gpu.resize(1920 + k, 1080);
                timed(&mut gpu, &camera, &anchor).0
            })
            .collect();
        report += &format!("resize ms: {resizes:.1?}\n");
        for samples in [1, 4] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(1920, 1080);
            for ssao in [false, true] {
                gpu.view.ssao = ssao;
                let pixels = (0..4).fold(Vec::new(), |_, _| timed(&mut gpu, &camera, &anchor).1);
                if ssao {
                    std::fs::create_dir_all("target/review").unwrap();
                    std::fs::write(format!("target/review/bench-{samples}x.rgba"), &pixels).unwrap();
                }
                let still: f64 = (0..120).map(|_| timed(&mut gpu, &camera, &anchor).0).sum::<f64>() / 120.0;
                let mut orbit = 0.0;
                for _ in 0..120 {
                    camera.orbit(1.745, 0.0);
                    orbit += timed(&mut gpu, &camera, &anchor).0;
                }
                for _ in 0..120 {
                    camera.orbit(-1.745, 0.0);
                }
                // the same orbit as a drag
                gpu.performance.interacting = true;
                let mut drag = 0.0;
                for _ in 0..120 {
                    camera.orbit(1.745, 0.0);
                    drag += timed(&mut gpu, &camera, &anchor).0;
                }
                for _ in 0..120 {
                    camera.orbit(-1.745, 0.0);
                }
                gpu.performance.interacting = false;
                report += &format!(
                    "{samples}x ssao={ssao}: still {still:.2} ms, orbit {:.2} ms, drag {:.2} ms\n",
                    orbit / 120.0,
                    drag / 120.0
                );
            }
        }
        std::fs::write("target/review/bench-ambient.txt", &report).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// After the first frame with occlusion, toggles and resizes compile no pipeline or shader.
    fn toggles_and_resizes_compile_nothing_after_the_first_frame() {
        let (mut gpu, camera, anchor) = contact_scene(256, 256);
        gpu.view.msaa_forced = Some(4);
        gpu.resize(256, 256);
        gpu.view.ssao = true;
        timed(&mut gpu, &camera, &anchor);
        let compiled = crate::engine::pipelines::created();

        for _ in 0..20 {
            gpu.view.ssao = false;
            timed(&mut gpu, &camera, &anchor);
            gpu.view.ssao = true;
            timed(&mut gpu, &camera, &anchor);
        }

        for step in 1..=20 {
            gpu.resize(256 + step, 256);
            timed(&mut gpu, &camera, &anchor);
        }

        assert_eq!(crate::engine::pipelines::created(), compiled);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A drag shades from half-resolution occlusion in the same textures; the still frames after it
    /// are the ones a view that never dragged shows.
    fn drags_shade_at_half_resolution_and_the_frames_after_restore_the_still_image() {
        for samples in [1, 4] {
            let (mut gpu, camera, anchor) = contact_scene(256, 256);
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(256, 256);
            gpu.view.ssao = false;
            let plain = timed(&mut gpu, &camera, &anchor).1;
            gpu.view.ssao = true;
            let still = (0..4).fold(Vec::new(), |_, _| timed(&mut gpu, &camera, &anchor).1);
            let memory = gpu.allocated_bytes();

            if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
                std::fs::create_dir_all("target/review").unwrap();
                std::fs::write(format!("target/review/ambient-{samples}x-still.rgba"), &still)
                    .unwrap();
            }

            gpu.performance.interacting = true;
            let dragged = timed(&mut gpu, &camera, &anchor).1;
            assert!(gpu.ssao.as_ref().unwrap().half());

            if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
                std::fs::create_dir_all("target/review").unwrap();
                std::fs::write(format!("target/review/ambient-{samples}x-drag.rgba"), &dragged)
                    .unwrap();
            }

            assert_eq!(gpu.allocated_bytes(), memory, "no texture for the drag");
            let darkened = plain
                .chunks_exact(4)
                .zip(dragged.chunks_exact(4))
                .filter(|(a, b)| a[0] > b[0].saturating_add(2))
                .count();
            assert!(darkened > 20, "a drag still shades contact at {samples}x: {darkened}");
            // close to the still image on average
            let difference: u64 = still
                .iter()
                .zip(&dragged)
                .map(|(a, b)| u64::from(a.abs_diff(*b)))
                .sum();
            assert!(
                difference < still.len() as u64,
                "mean difference {:.3} at {samples}x",
                difference as f64 / still.len() as f64
            );

            gpu.performance.interacting = false;
            let restored = (0..4).fold(Vec::new(), |_, _| timed(&mut gpu, &camera, &anchor).1);
            assert!(!gpu.ssao.as_ref().unwrap().half());
            assert!(restored == still, "the still image returns at {samples}x");
        }
    }

    #[test]
    /// The shader compiles at 1x and 4x.
    fn shader_validates_for_both_depth_sample_counts() {
        for samples in [1, 4] {
            let module = naga::front::wgsl::parse_str(&super::shader_source(samples)).unwrap();
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap();
        }
    }
}

impl super::lane::Lane for Option<Ssao> {
    fn bytes(&self) -> (u64, u64) {
        (
            if self.is_some() { 256 } else { 0 },
            self.as_ref().map_or(0, Ssao::texture_bytes),
        )
    }
}
