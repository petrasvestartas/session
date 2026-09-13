use super::buffers::{GpuCtx, bind_group, uniform_buffer};
use super::targets::{Attachment, Targets, TextureSpec};
use super::widget_mesh;
use crate::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};
use session_rust::Xform;
use wgpu::util::DeviceExt;

pub struct Widget {
    vertices: wgpu::Buffer,
    count: u32,
    uniform: wgpu::Buffer,
    group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
    tile: Option<Tile>,
    composite: wgpu::RenderPipeline,
    format: wgpu::TextureFormat,
    texture_layout: wgpu::BindGroupLayout,
    visible: bool,
    pub placement: Option<([f64; 3], f64)>,
    pub active: f32,
}

impl Widget {
    pub fn new(ctx: &GpuCtx, target: Target) -> Self {
        let mesh = widget_mesh::vertices();
        let vertices = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("widget mesh"),
                contents: bytemuck::cast_slice(&mesh),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("widget uniform"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(96),
                    },
                    count: None,
                }],
            });
        let uniform = uniform_buffer(&ctx.device, "widget placement", &[0.0_f32; 24]);
        let group = bind_group(ctx, &layout, "widget", &[&uniform]);
        let texture_layout = texture_layout(ctx);
        let pipeline = pipeline(ctx, &layout, MESH_TARGET);
        let composite = composite_pipeline(ctx, &layout, &texture_layout, target.format);
        Self {
            vertices,
            count: mesh.len() as u32,
            uniform,
            group,
            layout,
            pipeline,
            tile: None,
            composite,
            format: target.format,
            texture_layout,
            visible: false,
            placement: None,
            active: -1.0,
        }
    }

    pub fn clear(&mut self) {
        self.placement = None;
        self.tile = None;
        self.visible = false;
        self.active = -1.0;
    }

    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        if self.format != target.format {
            self.composite =
                composite_pipeline(ctx, &self.layout, &self.texture_layout, target.format);
            self.format = target.format;
        }
    }

    pub fn allocated_bytes(&self) -> (u64, u64) {
        let textures = self.tile.as_ref().map_or(0, |tile| {
            u64::from(tile.size.0) * u64::from(tile.size.1) * 36
        });
        (self.vertices.size() + self.uniform.size(), textures)
    }

    pub fn prepare(
        &mut self,
        ctx: &GpuCtx,
        mvp: &Xform,
        anchor: [f64; 3],
        _eye: [f32; 3],
        size: (u32, u32),
    ) {
        self.visible = false;
        let Some((origin, scale)) = self.placement else {
            return;
        };
        let position = std::array::from_fn::<_, 3, _>(|i| origin[i] - anchor[i]);
        let model = &Xform::translation(position[0], position[1], position[2])
            * &Xform::scale_xyz(scale, scale, scale);
        let matrix = mvp * &model;
        let Some(rect) = bounds(&matrix.m, size) else {
            return;
        };
        let extent = ((rect[2] * 2.0).ceil() as u32, (rect[3] * 2.0).ceil() as u32);
        let extent = (
            extent.0.div_ceil(64).clamp(1, 16) * 64,
            extent.1.div_ceil(64).clamp(1, 16) * 64,
        );
        if self.tile.as_ref().is_none_or(|tile| tile.size != extent) {
            self.tile = None;
            self.tile = Some(Tile::new(ctx, &self.texture_layout, extent));
        }
        let mut uniform = [0.0_f32; 24];
        let [x, y, width, height] = rect;
        let sx = size.0 as f64 / width;
        let sy = size.1 as f64 / height;
        let tx = (size.0 as f64 - 2.0 * x - width) / width;
        let ty = (2.0 * y + height - size.1 as f64) / height;
        for column in 0..4 {
            let at = column * 4;
            uniform[at] = (sx * matrix.m[at] + tx * matrix.m[at + 3]) as f32;
            uniform[at + 1] = (sy * matrix.m[at + 1] + ty * matrix.m[at + 3]) as f32;
            uniform[at + 2] = matrix.m[at + 2] as f32;
            uniform[at + 3] = matrix.m[at + 3] as f32;
        }
        uniform[16] = (2.0 * x / size.0 as f64 - 1.0) as f32;
        uniform[17] = (1.0 - 2.0 * y / size.1 as f64) as f32;
        uniform[18] = (2.0 * width / size.0 as f64) as f32;
        uniform[19] = (-2.0 * height / size.1 as f64) as f32;
        uniform[20] = self.active;
        ctx.queue
            .write_buffer(&self.uniform, 0, bytemuck::bytes_of(&uniform));
        self.visible = true;
    }

    pub fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        _targets: &Targets,
    ) -> u32 {
        let Some(tile) = self.tile.as_ref().filter(|_| self.visible) else {
            return 0;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("widget overlay"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &tile.color,
                resolve_target: Some(&tile.resolved),
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &tile.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.0),
                    store: wgpu::StoreOp::Discard,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.draw(0..self.count, 0..1);
        drop(pass);
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("widget composite"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&self.composite);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_bind_group(1, &tile.group, &[]);
        pass.draw(0..6, 0..1);
        2
    }
}

impl Drop for Widget {
    fn drop(&mut self) {
        self.vertices.destroy();
        self.uniform.destroy();
    }
}

fn pipeline(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, target: Target) -> wgpu::RenderPipeline {
    let shader = module(
        &ctx.device,
        "widget",
        include_str!("../../shaders/widget.wgsl"),
    );
    let vertex = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<widget_mesh::Vertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32, 2 => Uint32],
    };
    build(
        &ctx.device,
        target,
        &PipelineDesc::new(
            &shader,
            &[layout],
            &[vertex],
            wgpu::PrimitiveTopology::TriangleList,
        ),
    )
}

const MESH_TARGET: Target = Target {
    format: wgpu::TextureFormat::Rgba8Unorm,
    samples: 4,
};

struct Tile {
    size: (u32, u32),
    color: Attachment,
    depth: Attachment,
    resolved: Attachment,
    group: wgpu::BindGroup,
}

impl Tile {
    fn new(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, size: (u32, u32)) -> Self {
        let attachment = |label, format, samples, usage| {
            Attachment::new(
                ctx,
                label,
                &TextureSpec {
                    size,
                    format,
                    samples,
                    usage,
                },
            )
        };
        let color = attachment(
            "widget color",
            MESH_TARGET.format,
            4,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        let depth = attachment(
            "widget depth",
            wgpu::TextureFormat::Depth32Float,
            4,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        let resolved = attachment(
            "widget resolved",
            MESH_TARGET.format,
            1,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("widget image"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&resolved),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        Self {
            size,
            color,
            depth,
            resolved,
            group,
        }
    }
}

fn texture_layout(ctx: &GpuCtx) -> wgpu::BindGroupLayout {
    ctx.device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("widget image"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                },
            ],
        })
}

fn composite_pipeline(
    ctx: &GpuCtx,
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = module(
        &ctx.device,
        "widget composite",
        include_str!("../../shaders/widget.wgsl"),
    );
    let groups = [layout, texture];
    let desc = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .vertex("vs_composite")
        .with("widget composite", "fs_composite")
        .depth(DepthMode::Detached)
        .color(ColorWrite::Blended);
    build(&ctx.device, Target { format, samples: 1 }, &desc)
}

fn bounds(m: &[f64; 16], size: (u32, u32)) -> Option<[f64; 4]> {
    let radius = crate::app::gizmo::ARM + 2.0;
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    for corner in 0..8 {
        let p = std::array::from_fn::<_, 3, _>(|i| {
            if corner & (1 << i) == 0 {
                -radius
            } else {
                radius
            }
        });
        let c = std::array::from_fn::<_, 4, _>(|i| {
            m[i] * p[0] + m[4 + i] * p[1] + m[8 + i] * p[2] + m[12 + i]
        });
        if c[3] <= 0.0 {
            return None;
        }
        let screen = [
            (c[0] / c[3] * 0.5 + 0.5) * size.0 as f64,
            (0.5 - c[1] / c[3] * 0.5) * size.1 as f64,
        ];
        for i in 0..2 {
            min[i] = min[i].min(screen[i]);
            max[i] = max[i].max(screen[i]);
        }
    }
    let x = (min[0] - 2.0).floor().max(0.0);
    let y = (min[1] - 2.0).floor().max(0.0);
    let width = (max[0] + 2.0).ceil().min(size.0 as f64) - x;
    let height = (max[1] + 2.0).ceil().min(size.1 as f64) - y;
    (width > 0.0 && height > 0.0).then_some([x, y, width, height])
}
