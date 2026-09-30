use super::buffers::{bind_group, resource_group};
// Ambient occlusion (AO) = how much nearby geometry hides a point from the sky; creases and contacts come out darker.
// Screen-space = the effect reads only the depth and ids of the frame just drawn, so hidden or off-screen objects cast nothing.
// GTAO = along a few screen directions, find the highest horizon a pixel sees, then integrate the sky left above it.
// Depth pyramid = the depth image halved five times; a far sample reads a small level, so a long reach costs few reads.
use super::pass::{Frame, Pass};
use super::{Gpu, buffers::GpuCtx, targets::Targets};
use crate::engine::pipelines::bindings::{buffer_entry, texture_entry};
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Pipeline, PipelineDesc, Target, build, module, pipeline_layout,
};

/// Every AO pipeline, compiled for one colour format and one sample count (1x or 4x MSAA).
use wgpu::{BufferBindingType, ShaderStages, TextureSampleType};
mod pipelines;
pub use pipelines::{SsaoPipelines, pipelines};

// In the browser, ambient_warm.rs compiles in idle time; `#[path]` names the file because the module is called `warm`.
#[cfg(target_arch = "wasm32")]
#[path = "ambient_warm.rs"]
mod warm;

/// The pipelines for `target`, compiled on first use; slot 0 holds 1x, slot 1 holds 4x.
pub fn cached<'a>(
    slots: &'a mut [Option<SsaoPipelines>; 2],
    ctx: &GpuCtx,
    target: Target,
) -> Option<&'a SsaoPipelines> {
    let slot = &mut slots[usize::from(target.samples > 1)];
    if slot.as_ref().is_none_or(|pipes| pipes.target != target) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            *slot = Some(pipelines(ctx, target));
        }
        #[cfg(target_arch = "wasm32")]
        {
            // None until the idle callback has compiled them; the frame draws without AO meanwhile
            *slot = warm::take(ctx, target);
        }
    }
    slot.as_ref()
}

/// Schedule browser compilation outside the frame after geometry has been presented.
pub fn prewarm(slots: &mut [Option<SsaoPipelines>; 2], ctx: &GpuCtx, target: Target, idle: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        warm::schedule(ctx, target, idle);
        for samples in [1, 4] {
            cached(slots, ctx, Target { samples, ..target });
        }
    }
    // natively there is no idle callback: compile whenever the user is not dragging
    #[cfg(not(target_arch = "wasm32"))]
    if idle {
        for samples in [1, 4] {
            cached(slots, ctx, Target { samples, ..target });
        }
    }
}

/// A texture and its default view.
struct Image {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

impl Image {
    fn new(
        ctx: &GpuCtx,
        label: &str,
        size: (u32, u32),
        format: wgpu::TextureFormat,
        mips: u32,
    ) -> Self {
        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: mips,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        Self { texture, view }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        // destroy frees the GPU memory now, instead of when the last handle to the texture is gone
        self.texture.destroy();
    }
}

/// Bind the depth, radius and normal views as group 1.
fn depth_group(
    ctx: &GpuCtx,
    layout: &wgpu::BindGroupLayout,
    views: [&wgpu::TextureView; 3],
) -> wgpu::BindGroup {
    resource_group(ctx, layout, "ambient depth", std::array::from_fn::<_, 3, _>(|i| {
        (i as u32, wgpu::BindingResource::TextureView(views[i]))
    }))
}

/// A colour attachment that starts from zero, or keeps what is there.
fn attachment(
    view: &wgpu::TextureView,
    clear: bool,
) -> Option<wgpu::RenderPassColorAttachment<'_>> {
    Some(wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        depth_slice: None,
        ops: wgpu::Operations {
            load: if clear {
                wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
            } else {
                wgpu::LoadOp::Load
            },
            store: wgpu::StoreOp::Store,
        },
    })
}

// About one AO pixel per CSS pixel, never more than half the canvas: 1920 x 1080 at DPR 1 gives 960 x 540.
fn resolution(full: (u32, u32), dpr: f64) -> (u32, u32) {
    let scale = (1.0 / dpr.max(1.0)).clamp(0.25, 0.5);
    (
        (f64::from(full.0) * scale).ceil().max(1.0) as u32,
        (f64::from(full.1) * scale).ceil().max(1.0) as u32,
    )
}

/// The images and buffers of the effect at one canvas size; turning AO off drops them all.
pub struct Ssao {
    full: (u32, u32),
    samples: u32,     // 1 or 4
    size: (u32, u32), // AO size, px
    linear: Image,    // distance along each pixel's ray, six levels
    radius: Image,    // each pixel's object AO radius, packed, six levels
    normals: Image,   // octahedral normal, two bytes
    occupancy: Image,
    occupancy_group: wgpu::BindGroup,
    ground: Image, // floor shadows at half the AO size
    ground_group: wgpu::BindGroup,
    levels: [[wgpu::TextureView; 2]; 6], // one view per level, to render into it
    depth_group: wgpu::BindGroup,
    reduce_groups: [wgpu::BindGroup; 5], // level i, read while writing level i + 1
    ao: [Image; 3],
    sampled: [wgpu::BindGroup; 3],
    history: Image, // this frame's depth, checked by the next frame
    history_group: wgpu::BindGroup,
    #[cfg(test)]
    history_enabled: bool, // tests switch reuse off to compare
    inverse: wgpu::Buffer,           // the camera uniform, 320 bytes
    edge_buffers: [wgpu::Buffer; 4], // 4x corrections, tile flags, tile list, indirect draw
    edge_write: wgpu::BindGroup,
    edge_read: wgpu::BindGroup,
    // the scene group, and the views it was made from, to see when it is stale
    group: Option<(
        wgpu::TextureView,
        wgpu::TextureView,
        [wgpu::Buffer; 4],
        wgpu::TextureView,
        wgpu::BindGroup,
    )>,
    cached: Option<([f32; 64], u64)>, // last uniform and geometry revision
    receiver_bounds: Option<(u64, session_rust::AABB, f32)>, // floor bounds and largest radius, per revision
    receiver_box: [f32; 6],
}

// The `impl Ssao` block stays open over the next three steps.
impl Ssao {
    /// Floor height and largest AO radius; the virtual floor lies under the lowest visible solid.
    pub fn receiver(&mut self, objects: &super::objects::InstanceTable) -> [f32; 2] {
        let revision = objects.geometry_revision();
        // the loop over every row runs once per edit, not once per frame
        if self
            .receiver_bounds
            .as_ref()
            .is_none_or(|(r, _, _)| *r != revision)
        {
            let mut bounds = session_rust::AABB::empty();
            let mut radius = 0.01_f32;
            for i in 0..objects.len() {
                let flags = objects.row(i).unwrap().flags;
                // lines, points, sheets and hidden rows do not lower the floor
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
        let (_, b, radius) = self.receiver_bounds.as_ref().unwrap();
        self.receiver_box = [
            (b.cx - b.hx - objects.anchor()[0]) as f32,
            (b.cy - b.hy - objects.anchor()[1]) as f32,
            (b.cz - b.hz - objects.anchor()[2]) as f32,
            (b.cx + b.hx - objects.anchor()[0]) as f32,
            (b.cy + b.hy - objects.anchor()[1]) as f32,
            (b.cz + b.hz - objects.anchor()[2]) as f32,
        ];
        [self.receiver_box[2], *radius]
    }

    /// Allocate every image and buffer for a canvas of `full` pixels.
    pub fn new(ctx: &GpuCtx, pipes: &SsaoPipelines, full: (u32, u32), dpr: f64) -> Self {
        let size = resolution(full, dpr);
        // 32 px halved five times is 1 px, so even a tiny window keeps six valid levels
        let size = (size.0.max(32), size.1.max(32));
        let inverse = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ambient camera"),
            size: 320, // 64 floats of camera and rays plus the previous matrix: 80 x 4 bytes
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let linear = Image::new(
            ctx,
            "ambient linear depth",
            size,
            wgpu::TextureFormat::R32Float,
            6,
        );
        let radius = Image::new(ctx, "ambient radius", size, wgpu::TextureFormat::R16Uint, 6);
        let normals = Image::new(
            ctx,
            "ambient normals",
            size,
            wgpu::TextureFormat::Rg8Unorm,
            1,
        );
        // a view of one mip level alone, so a pass can render into that level
        let levels = std::array::from_fn(|level| {
            [&linear, &radius].map(|image| {
                image.texture.create_view(&wgpu::TextureViewDescriptor {
                    base_mip_level: level as u32,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
        });
        let depth_group = depth_group(
            ctx,
            &pipes.depth_layout,
            [&linear.view, &radius.view, &normals.view],
        );
        let reduce_groups = std::array::from_fn(|i| {
            self::depth_group(
                ctx,
                &pipes.depth_layout,
                [&levels[i][0], &levels[i][1], &normals.view],
            )
        });
        // images 0 and 1 at AO size take turns through the filters; image 2 is the full-size cache the composite reads
        let ao = std::array::from_fn(|i| {
            Image::new(
                ctx,
                "ambient occlusion",
                if i == 2 { full } else { size },
                wgpu::TextureFormat::R8Unorm,
                1,
            )
        });
        // bilinear = blend the four nearest texels, used when a small image is read at a larger size
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("ambient interpolation"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let sample = |view: &wgpu::TextureView| {
            resource_group(
                ctx,
                &pipes.sample_layout,
                "ambient samples",
                [
                    (0, wgpu::BindingResource::TextureView(view)),
                    (1, wgpu::BindingResource::Sampler(&sampler)),
                ],
            )
        };
        let sampled = std::array::from_fn(|i| sample(&ao[i].view));
        let occupancy = Image::new(
            ctx,
            "ambient ground occupancy",
            (size.0 >> 4, size.1 >> 4), // one texel per 16 x 16 AO pixels
            wgpu::TextureFormat::R8Unorm,
            1,
        );
        let occupancy_group = sample(&occupancy.view);
        let ground = Image::new(
            ctx,
            "ambient ground contacts",
            (size.0.div_ceil(2), size.1.div_ceil(2)),
            wgpu::TextureFormat::R8Unorm,
            1,
        );
        let ground_group = sample(&ground.view);
        let history = Image::new(
            ctx,
            "ambient history depth",
            (size.0.div_ceil(2), size.1.div_ceil(2)),
            wgpu::TextureFormat::R32Float,
            1,
        );
        let history_group = resource_group(
            ctx,
            &pipes.history_layout,
            "ambient history",
            [
                (4, wgpu::BindingResource::TextureView(&ao[2].view)),
                (5, wgpu::BindingResource::TextureView(&history.view)),
            ],
        );
        // at 4x: one packed word per pixel for its four samples, one flag per 16 x 16 tile, and the list of flagged tiles
        let pixels = u64::from(full.0) * u64::from(full.1);
        let tiles = u64::from(full.0.div_ceil(16)) * u64::from(full.1.div_ceil(16));
        let edge_buffers = std::array::from_fn(|i| {
            ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient edge corrections"),
                size: if i == 3 {
                    16
                } else if pipes.target.samples == 1 {
                    4
                } else if i == 0 {
                    pixels * 4
                } else {
                    tiles * 4
                },
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | if i == 3 {
                        wgpu::BufferUsages::INDIRECT
                    } else {
                        wgpu::BufferUsages::empty()
                    },
                mapped_at_creation: false,
            })
        });
        // indirect draw = the GPU reads the draw's counts from a buffer: 6 vertices per tile, instances added by the upsample shader
        ctx.queue
            .write_buffer(&edge_buffers[3], 0, bytemuck::cast_slice(&[6_u32, 0, 0, 0]));
        let edge_write = bind_group(
            ctx, &pipes.write_layout, "ambient edge output", &edge_buffers.each_ref(),
        );
        let edge_read = bind_group(
            ctx, &pipes.composite_layout, "ambient edge input",
            &[&inverse, &edge_buffers[0], &edge_buffers[2]],
        );
        Self {
            full,
            samples: pipes.target.samples,
            size,
            linear,
            radius,
            normals,
            occupancy,
            occupancy_group,
            ground,
            ground_group,
            levels,
            depth_group,
            reduce_groups,
            ao,
            sampled,
            history,
            history_group,
            #[cfg(test)]
            history_enabled: true,
            inverse,
            edge_buffers,
            edge_write,
            edge_read,
            group: None,
            cached: None,
            receiver_bounds: None,
            receiver_box: [0.0; 6],
        }
    }

    /// GPU bytes of the images, for the memory counters.
    pub fn texture_bytes(&self) -> u64 {
        let pixels = u64::from(self.size.0) * u64::from(self.size.1);
        let pyramid: u64 = (0..self.linear.texture.mip_level_count())
            .map(|i| u64::from(self.size.0 >> i) * u64::from(self.size.1 >> i))
            .sum();
        (4 + u64::from(self.radius.texture.format().block_copy_size(None).unwrap())) * pyramid
            + 4 * pixels
            + u64::from(self.full.0) * u64::from(self.full.1)
            + 5 * u64::from(self.ground.texture.width()) * u64::from(self.ground.texture.height())
            + u64::from(self.occupancy.texture.width()) * u64::from(self.occupancy.texture.height())
    }

    /// GPU bytes of the uniform and the correction buffers.
    pub fn buffer_bytes(&self) -> u64 {
        320 + self
            .edge_buffers
            .iter()
            .map(wgpu::Buffer::size)
            .sum::<u64>()
    }

    /// False after a resize, a DPR change or a switch between 1x and 4x; then the images are rebuilt.
    pub fn fits(&self, pipes: &SsaoPipelines, full: (u32, u32), dpr: f64) -> bool {
        let size = resolution(full, dpr);
        self.full == full
            && self.samples == pipes.target.samples
            && self.size == (size.0.max(32), size.1.max(32))
    }

    /// Record the AO passes and return the draw count; with the camera and scene unchanged it only blends the cache.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        ctx: &GpuCtx,
        pipes: &SsaoPipelines,
        targets: &Targets,
        geometry: [&wgpu::Buffer; 4],
        table: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        mvp: [f32; 16],
        ground: [f32; 2],
        revision: u64,
        mut timer: Option<&mut super::timing::PassTimer>,
    ) -> u32 {
        // let-else: bind the value or leave the function; a degenerate camera matrix has no inverse
        let Some(inverse) = inverse_projection(mvp) else {
            return 0;
        };
        // the same layout as `Ambient` in ssao.wgsl: 16 floats per matrix, 4 per vec4
        let mut uniform = [0.0; 64];
        uniform[..16].copy_from_slice(&inverse);
        uniform[16..32].copy_from_slice(&mvp);
        // params = floor height, largest radius, canvas size; extent = ground width, AO size, ground height
        uniform[32..40].copy_from_slice(&[
            ground[0],
            ground[1],
            self.full.0 as f32,
            self.full.1 as f32,
            self.ground.texture.width() as f32,
            self.size.0 as f32,
            self.size.1 as f32,
            self.ground.texture.height() as f32,
        ]);
        uniform[40..].copy_from_slice(&pixel_rays(&inverse, self.full));
        // same camera, same geometry: last frame's cache is still right
        let changed = self.cached != Some((uniform, revision));
        if changed {
            let mut data = [0.0; 80];
            data[..64].copy_from_slice(&uniform);
            // same geometry, moved camera: pass the previous matrix, and near.w = 1 tells the shader it may reuse last frame
            if let Some((previous, key)) = &self.cached {
                if *key == revision {
                    data[64..].copy_from_slice(&previous[16..32]);
                    data[43] = 1.0;
                }
            }
            #[cfg(test)]
            if !self.history_enabled {
                data[43] = 0.0;
            }
            ctx.queue
                .write_buffer(&self.inverse, 0, bytemuck::cast_slice(&data));
        }
        // rebuild the bind group only when a target or buffer was replaced, after a resize or an upload
        if self
            .group
            .as_ref()
            .is_none_or(|(depth, ids, buffer, bound, _)| {
                *depth != targets.depth.view
                    || *ids != targets.gradient.view
                    || bound != table
                    || buffer.iter().zip(geometry).any(|(a, b)| a != b)
            })
        {
            let group = resource_group(
                ctx,
                &pipes.layout,
                "ambient scene",
                [
                    (0, wgpu::BindingResource::TextureView(&targets.depth)),
                    (1, self.inverse.as_entire_binding()),
                    (2, wgpu::BindingResource::TextureView(&targets.gradient)),
                    (3, geometry[0].as_entire_binding()),
                    (4, geometry[1].as_entire_binding()),
                    (5, geometry[2].as_entire_binding()),
                    (6, geometry[3].as_entire_binding()),
                    (7, wgpu::BindingResource::TextureView(table)),
                ],
            );
            self.group = Some((
                targets.depth.view.clone(),
                targets.gradient.view.clone(),
                geometry.map(Clone::clone),
                table.clone(),
                group,
            ));
        }
        let group = &self.group.as_ref().unwrap().4;
        if changed {
            // 4x: clear the tile flags and the instance count before the upsample fills them
            if self.samples > 1 {
                encoder.clear_buffer(&self.edge_buffers[1], 0, None);
                encoder.clear_buffer(&self.edge_buffers[3], 4, Some(4));
            }
            // only the screen rectangle around the visible solids, plus a margin, is shaded
            let rectangle = projected_bounds(mvp, self.receiver_box, self.full);
            // level 0 from the frame's depth, then each level from the one below it
            for level in 0..6 {
                // `as_deref_mut` lends the timer for this call and leaves the Option usable afterwards
                if level == 1
                    && let Some(timer) = timer.as_deref_mut()
                {
                    timer.mark(encoder, "ao.prepare");
                }
                let attachments = [
                    attachment(&self.levels[level][0], true),
                    attachment(&self.levels[level][1], true),
                    attachment(&self.normals.view, true),
                ];
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient depth"),
                    color_attachments: &attachments[..if level == 0 { 3 } else { 2 }],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                if level == 0 {
                    let lo = [
                        (rectangle[0] * self.size.0 as f32).floor() as u32,
                        (rectangle[1] * self.size.1 as f32).floor() as u32,
                    ];
                    let hi = [
                        (rectangle[2] * self.size.0 as f32).ceil() as u32,
                        (rectangle[3] * self.size.1 as f32).ceil() as u32,
                    ];
                    if hi[0] > lo[0] && hi[1] > lo[1] {
                        // a scissor rectangle limits drawing to these pixels; the rest of the pass costs nothing
                        pass.set_scissor_rect(lo[0], lo[1], hi[0] - lo[0], hi[1] - lo[1]);
                    }
                }
                pass.set_pipeline(if level == 0 {
                    &pipes.prepare
                } else {
                    &pipes.reduce
                });
                pass.set_bind_group(
                    0,
                    if level == 0 {
                        group
                    } else {
                        &self.reduce_groups[level - 1]
                    },
                    &[],
                );
                pass.draw(0..3, 0..1);
            }
            // the braces end `pass`, which borrows `encoder`, so the next pass can begin
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient ground occupancy"),
                    color_attachments: &[attachment(&self.occupancy.view, true)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&pipes.occupancy);
                pass.set_bind_group(0, &self.reduce_groups[4], &[]);
                pass.draw(0..3, 0..1);
            }
            if let Some(timer) = timer.as_deref_mut() {
                timer.mark(encoder, "ao.depth");
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient ground contacts"),
                    color_attachments: &[attachment(&self.ground.view, true)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&pipes.ground);
                pass.set_bind_group(0, group, &[]);
                pass.set_bind_group(1, &self.depth_group, &[]);
                pass.set_bind_group(2, &self.occupancy_group, &[]);
                pass.draw(0..3, 0..1);
            }
            if let Some(timer) = timer.as_deref_mut() {
                timer.mark(encoder, "ao.ground");
            }
            // horizons, blur x, blur y with history, upsample to full size; each reads what the previous one wrote
            for (index, pipe) in [
                &pipes.raw,
                &pipes.filter[0],
                &pipes.filter[1],
                &pipes.upsample,
            ]
            .into_iter()
            .enumerate()
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient shading"),
                    color_attachments: &[attachment(&self.ao[[0, 1, 0, 2][index]].view, true)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                let size = if index == 3 { self.full } else { self.size };
                // snapped to 16 px tiles, the tiles of the 4x correction
                let lo: [u32; 2] = std::array::from_fn(|i| {
                    ((rectangle[i] * [size.0, size.1][i] as f32).floor() as u32) / 16 * 16
                });
                let hi: [u32; 2] = std::array::from_fn(|i| {
                    ((rectangle[i + 2] * [size.0, size.1][i] as f32).ceil() as u32)
                        .max(lo[i] + 1)
                        .div_ceil(16)
                        .saturating_mul(16)
                        .min([size.0, size.1][i])
                });
                pass.set_scissor_rect(lo[0], lo[1], hi[0] - lo[0], hi[1] - lo[1]);
                pass.set_pipeline(pipe);
                pass.set_bind_group(0, group, &[]);
                pass.set_bind_group(1, &self.depth_group, &[]);
                pass.set_bind_group(
                    2,
                    if index == 0 {
                        &self.ground_group
                    } else {
                        &self.sampled[usize::from(index == 2)]
                    },
                    &[],
                );
                if index == 2 {
                    pass.set_bind_group(3, &self.history_group, &[]);
                } else if index == 3 {
                    pass.set_bind_group(3, &self.edge_write, &[]);
                }
                pass.draw(0..3, 0..1);
                // end the pass before the timer writes into the encoder
                drop(pass);
                if let Some(timer) = timer.as_deref_mut() {
                    timer.mark(
                        encoder,
                        ["ao.horizons", "ao.blur_x", "ao.blur_y", "ao.upsample"][index],
                    );
                }
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient history depth"),
                    color_attachments: &[attachment(&self.history.view, true)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&pipes.history);
                pass.set_bind_group(0, group, &[]);
                pass.set_bind_group(1, &self.depth_group, &[]);
                pass.draw(0..3, 0..1);
            }
            if let Some(timer) = timer.as_deref_mut() {
                timer.mark(encoder, "ao.history");
            }
            self.cached = Some((uniform, revision));
        }
        // runs every frame: blend the cached AO over the scene, before the MSAA resolve
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ambient composite"),
            color_attachments: &[attachment(targets.msaa.as_deref().unwrap_or(view), false)],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&pipes.composite);
        pass.set_bind_group(0, &self.edge_read, &[]);
        pass.set_bind_group(1, &self.sampled[2], &[]);
        let rectangle = projected_bounds(mvp, self.receiver_box, self.full);
        let lo = [
            (rectangle[0] * self.full.0 as f32).floor() as u32,
            (rectangle[1] * self.full.1 as f32).floor() as u32,
        ];
        let hi = [
            (rectangle[2] * self.full.0 as f32).ceil() as u32,
            (rectangle[3] * self.full.1 as f32).ceil() as u32,
        ];
        // 13 draws to recompute: 6 levels, occupancy, ground, 4 shading passes, history
        let mut draws = if changed { 13 } else { 0 };
        if hi[0] > lo[0] && hi[1] > lo[1] {
            pass.set_scissor_rect(lo[0], lo[1], hi[0] - lo[0], hi[1] - lo[1]);
            pass.draw(0..3, 0..1);
            draws += 1;
            if self.samples > 1 {
                pass.set_pipeline(&pipes.edges);
                // how many tile squares to draw comes from the buffer the upsample shader filled
                pass.draw_indirect(&self.edge_buffers[3], 0);
                draws += 1;
            }
        }
        draws
    }
}

/// The screen rectangle, 0 to 1, of a world box; the whole screen when a corner is behind the camera.
fn projected_bounds(matrix: [f32; 16], bounds: [f32; 6], full: (u32, u32)) -> [f32; 4] {
    let mut rect = [1.0_f32, 1.0, 0.0, 0.0];
    for corner in 0..8 {
        let p = [
            bounds[if corner & 1 == 0 { 0 } else { 3 }],
            bounds[if corner & 2 == 0 { 1 } else { 4 }],
            bounds[if corner & 4 == 0 { 2 } else { 5 }],
            1.0,
        ];
        let clip: [f32; 4] =
            // column-major: element (row, col) sits at col * 4 + row
            std::array::from_fn(|row| (0..4).map(|col| matrix[col * 4 + row] * p[col]).sum());
        if clip[3] <= 0.0 {
            return [0.0, 0.0, 1.0, 1.0];
        }
        let uv = [clip[0] / clip[3] * 0.5 + 0.5, 0.5 - clip[1] / clip[3] * 0.5];
        for i in 0..2 {
            rect[i] = rect[i].min(uv[i]);
            rect[i + 2] = rect[i + 2].max(uv[i]);
        }
    }
    // Ground probes reach 64 canvas pixels; leave room for filtering and reconstruction.
    for i in 0..2 {
        let margin = 84.0 / [full.0, full.1][i].max(1) as f32;
        rect[i] = (rect[i] - margin).clamp(0.0, 1.0 - 1.0 / [full.0, full.1][i].max(1) as f32);
        rect[i + 2] = (rect[i + 2] + margin).clamp(0.0, 1.0);
    }
    rect
}

/// The AO shader for 1x or 4x: WGSL has separate types for multisampled textures, so the source is patched.
fn shader_source(samples: u32) -> String {
    let source = shader!("ssao.wgsl");
    if samples > 1 {
        source
            .replace("const MSAA: bool = false;", "const MSAA: bool = true;")
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
        let ndc = [
            x / f64::from(size.0) * 2.0 - 1.0,
            1.0 - y / f64::from(size.1) * 2.0,
            z,
            1.0,
        ];
        let p: [f64; 4] = std::array::from_fn(|row| {
            (0..4)
                .map(|col| f64::from(inverse[col * 4 + row]) * ndc[col])
                .sum()
        });
        [p[0] / p[3], p[1] / p[3], p[2] / p[3]]
    };
    // the shader rebuilds any pixel's ray as base + x * step_x + y * step_y, with no matrix product per pixel
    let near = [
        world(0.0, 0.0, 1.0),
        world(1.0, 0.0, 1.0),
        world(0.0, 1.0, 1.0),
    ];
    let half = [
        world(0.0, 0.0, 0.5),
        world(1.0, 0.0, 0.5),
        world(0.0, 1.0, 0.5),
    ];
    let ray: [[f64; 3]; 3] =
        std::array::from_fn(|i| std::array::from_fn(|k| half[i][k] - near[i][k]));
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
mod tests;

/// Ambient occlusion over the faces; its pipelines at 1x and 4x stay while it is off.
pub struct Ambient {
    ssao: Option<Ssao>,                // the textures and buffers, when on
    pipes: [Option<SsaoPipelines>; 2], // at 1x and 4x
}

// PASSES calls this at start-up; nothing is allocated until AO is switched on
/// The ambient pass, off.
pub fn pass(_ctx: &GpuCtx, _target: Target) -> Box<dyn Pass> {
    Box::new(Ambient {
        ssao: None,
        pipes: [None, None],
    })
}

impl Ambient {
    /// The pass's textures and buffers; it must be on.
    #[cfg(test)]
    fn ambient(&self) -> &Ssao {
        self.ssao.as_ref().expect("ambient occlusion is on")
    }
}

// `bytes` feeds the memory counters, so the AO images show in ?inspect=1
impl super::lane::Lane for Ambient {
    fn bytes(&self) -> (u64, u64) {
        (
            self.ssao.as_ref().map_or(0, Ssao::buffer_bytes),
            self.ssao.as_ref().map_or(0, Ssao::texture_bytes),
        )
    }
}

impl Pass for Ambient {
    /// The same quality throughout navigation.
    fn after_faces(&mut self, g: &mut Gpu, encoder: &mut wgpu::CommandEncoder, f: &Frame) -> u32 {
        // opacity 0 hides the faces, so there is nothing to occlude
        let ambient = g.view.ssao && g.view.opacity > 0.0 && g.live_faces() > 0;
        let mut draws = 0;

        if ambient {
            let full = (g.config.width, g.config.height);
            // device pixels per CSS pixel, e.g. 2 on a HiDPI laptop
            let dpr = f64::from(full.0) / g.logical_size[0].max(1.0);
            let target = g.target();
            if let Some(pipes) = cached(&mut self.pipes, &g.ctx, target) {
                // textures follow the canvas; pipelines stay
                if self
                    .ssao
                    .as_ref()
                    .is_some_and(|ssao| !ssao.fits(pipes, full, dpr))
                {
                    self.ssao = None;
                }
                // `get_or_insert_with` runs the closure only when the Option is None
                let ssao = self
                    .ssao
                    .get_or_insert_with(|| Ssao::new(&g.ctx, pipes, full, dpr));
                let receiver = ssao.receiver(&g.objects);
                let [vertices, owners, indices] = g.arena.geometry_buffers();
                draws += ssao.draw(
                    &g.ctx,
                    pipes,
                    &g.targets,
                    [vertices, owners, indices, g.objects.instance_buffer()],
                    &g.arena.table.view,
                    encoder,
                    f.view,
                    g.frame.mvp_f32,
                    receiver,
                    g.objects.geometry_revision(),
                    g.timer.as_mut(),
                );
            }
        } else {
            // off: drop the images at once, keep the compiled pipelines
            self.ssao = None;
        }

        g.mark(encoder, "ssao");
        draws
    }

    fn pending(&self, g: &Gpu) -> bool {
        g.view.ssao
            && g.view.opacity > 0.0
            && g.live_faces() > 0
            && self.pipes[usize::from(g.targets.samples > 1)].is_none()
    }

    // after the frame is on screen, compile ahead; natively only while not dragging
    fn after_present(&mut self, g: &mut Gpu) {
        if g.live_faces() > 0 {
            let target = g.target();
            prewarm(&mut self.pipes, &g.ctx, target, !g.performance.interacting);
        }
    }
}
