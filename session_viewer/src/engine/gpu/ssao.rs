use super::{buffers::GpuCtx, targets::Targets};
use crate::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

/// Hemisphere/contact lighting with two capped half-float textures.
/// The hemisphere sampling pass is cached while camera and geometry remain unchanged.
pub struct Ssao {
    target: Target,
    layout: wgpu::BindGroupLayout,
    raw: wgpu::RenderPipeline,
    composite: wgpu::RenderPipeline,
    filter: [wgpu::RenderPipeline; 2],
    filtered: wgpu::Texture,
    filtered_view: wgpu::TextureView,
    filtered_group: wgpu::BindGroup,
    inverse: wgpu::Buffer,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampled: wgpu::BindGroup,
    size: (u32, u32),
    cached: Option<([f32; 36], u64)>,
    receiver_bounds: Option<(u64, session_rust::AABB, f32)>,
}

impl Ssao {
    /// Hidden solids and non-surface drawings cannot lower the shadow receiver.
    pub fn receiver(&mut self, objects: &super::objects::InstanceTable) -> [f32; 2] {
        let revision = objects.geometry_revision();
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
        [(b.cz - b.hz - objects.anchor()[2]) as f32, radius]
    }

    pub fn new(ctx: &GpuCtx, target: Target, full: (u32, u32)) -> Self {
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
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
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
        let sample_layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient reconstruction"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }],
            });
        let shader = module(&ctx.device, "ambient", &shader_source(target.samples));
        let raw = build(
            &ctx.device,
            Target {
                format: wgpu::TextureFormat::R16Float,
                samples: 1,
            },
            &PipelineDesc::new(
                &shader,
                &[&layout],
                &[],
                wgpu::PrimitiveTopology::TriangleList,
            )
            .with("ambient hemisphere", "fs_main")
            .depth(DepthMode::Detached),
        );
        let filter = ["fs_filter_x", "fs_filter_y"].map(|entry| {
            build(
                &ctx.device,
                Target {
                    format: wgpu::TextureFormat::R16Float,
                    samples: 1,
                },
                &PipelineDesc::new(
                    &shader,
                    &[&layout, &sample_layout],
                    &[],
                    wgpu::PrimitiveTopology::TriangleList,
                )
                .with("ambient denoise", entry)
                .depth(DepthMode::Detached),
            )
        });
        let composite = build(
            &ctx.device,
            target,
            &PipelineDesc::new(
                &shader,
                &[&layout, &sample_layout],
                &[],
                wgpu::PrimitiveTopology::TriangleList,
            )
            .with("ambient reconstruction", "fs_composite")
            .depth(DepthMode::Detached)
            .color(ColorWrite::Blended),
        );
        let inverse = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ambient inverse and ground"),
            size: 144,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
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
        let texture = ctx.device.create_texture(&descriptor);
        let filtered = ctx.device.create_texture(&descriptor);
        let filtered_view = filtered.create_view(&Default::default());
        let filtered_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient filtered texture"),
            layout: &sample_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&filtered_view),
            }],
        });
        let view = texture.create_view(&Default::default());
        let sampled = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient texture"),
            layout: &sample_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
        });
        Self {
            target,
            layout,
            raw,
            composite,
            filter,
            filtered,
            filtered_view,
            filtered_group,
            inverse,
            texture,
            view,
            sampled,
            size,
            cached: None,
            receiver_bounds: None,
        }
    }

    pub fn texture_bytes(&self) -> u64 {
        4 * u64::from(self.size.0) * u64::from(self.size.1)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        ctx: &GpuCtx,
        target: Target,
        targets: &Targets,
        projected: &wgpu::Buffer,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        mvp: [f32; 16],
        ground: [f32; 2],
        revision: u64,
    ) -> u32 {
        let size = (
            targets.depth.texture().width(),
            targets.depth.texture().height(),
        );
        if self.target != target
            || self
                .cached
                .is_some_and(|(key, _)| key[34] != size.0 as f32 || key[35] != size.1 as f32)
        {
            *self = Self::new(ctx, target, size);
        }
        let Some(inverse) = inverse_projection(mvp) else {
            return 0;
        };
        let mut uniform = [0.0; 36];
        uniform[..16].copy_from_slice(&inverse);
        uniform[16..32].copy_from_slice(&mvp);
        uniform[32..].copy_from_slice(&[ground[0], ground[1], size.0 as f32, size.1 as f32]);
        ctx.queue
            .write_buffer(&self.inverse, 0, bytemuck::cast_slice(&uniform));
        let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient"),
            layout: &self.layout,
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
        let mut draws = 1;
        if self.cached != Some((uniform, revision)) {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ambient horizons"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.view,
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
            pass.set_pipeline(&self.raw);
            pass.set_bind_group(0, &group, &[]);
            pass.draw(0..3, 0..1);
            drop(pass);
            for (pipeline, output, input) in [
                (&self.filter[0], &self.filtered_view, &self.sampled),
                (&self.filter[1], &self.view, &self.filtered_group),
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
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &group, &[]);
                pass.set_bind_group(1, input, &[]);
                pass.draw(0..3, 0..1);
            }
            self.cached = Some((uniform, revision));
            draws += 3;
        }
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
        pass.set_pipeline(&self.composite);
        pass.set_bind_group(0, &group, &[]);
        pass.set_bind_group(1, &self.sampled, &[]);
        pass.draw(0..3, 0..1);
        draws
    }
}

impl Drop for Ssao {
    fn drop(&mut self) {
        self.texture.destroy();
        self.filtered.destroy();
    }
}

fn shader_source(samples: u32) -> String {
    let source = include_str!("../../shaders/ssao.wgsl");
    if samples > 1 {
        source
            .replace("texture_depth_2d", "texture_depth_multisampled_2d")
            .replace(
                "fn fs_composite(@builtin(position) pixel: vec4<f32>)",
                "fn fs_composite(@builtin(position) pixel: vec4<f32>, @builtin(sample_index) sample: u32)",
            )
            .replace("reconstruct(pixel.xy,vec2<i32>(0),0)", "reconstruct(pixel.xy,vec2<i32>(0),i32(sample))")
            .replace(
                "physical: texture_2d<f32>",
                "physical: texture_multisampled_2d<f32>",
            )
    } else {
        source.to_owned()
    }
}

// Camera matrices include millimetre-to-metre conversion. Normalize each equation
// before the kernel inverse's absolute determinant test, then undo the row scaling.
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
            let shaded = gpu.render_offscreen(&input);
            if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
                std::fs::write(format!("target/review/ambient-{samples}x-off.rgba"), &plain)
                    .unwrap();
                std::fs::write(format!("target/review/ambient-{samples}x-on.rgba"), &shaded)
                    .unwrap();
            }
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
            assert_eq!(
                gpu.allocated_bytes(),
                (memory.0 + 144, memory.1 + 4 * 256 * 256)
            );
            gpu.view.ssao = false;
            assert_eq!(gpu.render_offscreen(&input), plain);
            assert_eq!(gpu.allocated_bytes(), memory);
        }
    }

    #[test]
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
