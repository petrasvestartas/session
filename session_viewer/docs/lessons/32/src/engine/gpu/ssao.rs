use super::{buffers::GpuCtx, targets::Targets};
use crate::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

/// Screen-space ambient occlusion: soft shadows where surfaces meet.
pub struct Ssao {
    target: Target, // scene color format and samples
    layout: wgpu::BindGroupLayout, // depth, uniform, gradient, triangles
    raw: wgpu::RenderPipeline, // computes occlusion per pixel
    composite: wgpu::RenderPipeline, // darkens the scene with it
    filter: wgpu::RenderPipeline, // the blur pass
    filtered: wgpu::Texture, // half-blurred occlusion
    filtered_view: wgpu::TextureView, // view of it
    filtered_group: wgpu::BindGroup, // binds it
    inverse: wgpu::Buffer, // inverse camera, camera, ground, size
    texture: wgpu::Texture, // occlusion per pixel
    view: wgpu::TextureView, // view of it
    sampled: wgpu::BindGroup, // binds it
    size: (u32, u32), // occlusion texture size, px
    cached: Option<([f32; 36], u64)>, // uniform and geometry the occlusion was computed for
    receiver_bounds: Option<(u64, session_rust::AABB)>, // bounds the radius was fitted to
}

impl Ssao {
    /// Ground height and contact radius from the visible solids.
    pub fn receiver(&mut self, objects: &super::objects::InstanceTable) -> [f32; 2] {
        let revision = objects.geometry_revision();
        // recompute when the objects changed
        if self
            .receiver_bounds
            .as_ref()
            .is_none_or(|(r, _)| *r != revision)
        {
            let mut bounds = session_rust::AABB::empty();
            for i in 0..objects.len() {
                let flags = objects.row(i).unwrap().flags;
                if flags & super::Instance::FLAG_HAS_FACES != 0
                    && flags & (super::Instance::FLAG_HIDDEN | super::Instance::FLAG_SHEET) == 0
                    && let Some(b) = objects.row_bounds(i)
                {
                    bounds.union_with(&b);
                }
            }
            self.receiver_bounds = Some((revision, bounds));
        }
        let b = &self.receiver_bounds.as_ref().unwrap().1;
        let radius = (2.0 * (b.hx * b.hx + b.hy * b.hy + b.hz * b.hz).sqrt() * 0.01).max(0.01);
        [(b.cz - b.hz - objects.anchor()[2]) as f32, radius as f32]
    }

    /// Create the pipelines and textures for a `full`-sized canvas.
    pub fn new(ctx: &GpuCtx, target: Target, full: (u32, u32)) -> Self {
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
                ],
            });
        // group 1: one occlusion texture
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
        let source = include_str!("../../shaders/ssao.wgsl").replace(
            "texture_depth_2d",
            if target.samples > 1 {
                "texture_depth_multisampled_2d"
            } else {
                "texture_depth_2d"
            },
        );
        let shader = module(&ctx.device, "ambient", &source);
        // occlusion into a one-channel half-float texture
        let raw = build(
            &ctx.device,
            Target {
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
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
        let filter = build(
            &ctx.device,
            Target {
                format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
                samples: 1,
            },
            &PipelineDesc::new(
                &shader,
                &[&layout, &sample_layout],
                &[],
                wgpu::PrimitiveTopology::TriangleList,
            )
            .with("ambient denoise", "fs_filter")
            .depth(DepthMode::Detached),
        );
        // multiply the scene by the occlusion
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
        // 36 floats: inverse camera, camera, ground, size
        let inverse = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ambient inverse and ground"),
            size: 144,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let scale = 0.5_f64.min(960.0 / f64::from(full.0.max(full.1).max(1)));
        let size = (
            (f64::from(full.0) * scale).ceil().max(1.0) as u32,
            (f64::from(full.1) * scale).ceil().max(1.0) as u32,
        );
        let descriptor = wgpu::TextureDescriptor {
            label: Some("ambient R8 quarter pixels"), // debug name
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm, // one byte per pixel
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

    /// Bytes of the two occlusion textures.
    pub fn texture_bytes(&self) -> u64 {
        2 * u64::from(self.size.0) * u64::from(self.size.1)
    }

    /// Compute occlusion if needed, then darken the scene; returns the draw count.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        ctx: &GpuCtx,
        target: Target,
        targets: &Targets,
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
        // remake everything when format or size changed
        if self.target != target
            || self
                .cached
                .is_some_and(|(key, _)| key[18] != size.0 as f32 || key[19] != size.1 as f32)
        {
            *self = Self::new(ctx, target, size);
        }
        // clip space back to view space
        let Some(inverse) = inverse_projection(mvp) else {
            return 0;
        };
        let mut uniform = [0.0; 36];
        uniform[..16].copy_from_slice(&inverse);
        uniform[16..32].copy_from_slice(&mvp);
        uniform[32..].copy_from_slice(&[ground[0], ground[1], size.0 as f32, size.1 as f32]);
        ctx.queue
            .write_buffer(&self.inverse, 0, bytemuck::cast_slice(&uniform));
        // this frame's depth and gradient
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
            ],
        });
        let mut draws = 1;
        // recompute only when camera or geometry moved
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
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ambient denoise"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.filtered_view, // the blurred occlusion
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
            pass.set_pipeline(&self.filter);
            pass.set_bind_group(0, &group, &[]);
            pass.set_bind_group(1, &self.sampled, &[]);
            pass.draw(0..3, 0..1);
            self.cached = Some((uniform, revision));
            draws += 2;
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
        pass.set_pipeline(&self.composite);
        pass.set_bind_group(0, &group, &[]);
        pass.set_bind_group(1, &self.filtered_group, &[]);
        pass.draw(0..3, 0..1);
        draws
    }
}

/// Free the textures now, not when the browser collects them.
impl Drop for Ssao {
    /// Remove the DOM listener.
    fn drop(&mut self) {
        self.texture.destroy();
        self.filtered.destroy();
    }
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
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        for samples in [1, 4] {
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
                (memory.0 + 144, memory.1 + 2 * 128 * 128)
            );
            gpu.view.ssao = false;
            assert_eq!(gpu.render_offscreen(&input), plain);
            assert_eq!(gpu.allocated_bytes(), memory);
        }
    }

    #[test]
    /// The shader compiles at 1x and 4x.
    fn shader_validates_for_both_depth_sample_counts() {
        for texture in ["texture_depth_2d", "texture_depth_multisampled_2d"] {
            let source =
                include_str!("../../shaders/ssao.wgsl").replace("texture_depth_2d", texture);
            let module = naga::front::wgsl::parse_str(&source).unwrap();
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap();
        }
    }
}
