use super::buffers::GpuCtx;

/// The depth texture remembers how far away each pixel already is, which is what lets a nearer triangle cover a farther one.
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: Option<wgpu::TextureView>,
    pub depth_single: wgpu::TextureView, // depth at 1x, or a 1x1 placeholder
    pub depth_msaa: wgpu::TextureView, // depth at 4x, or a 1x1 placeholder
    pub samples: u32, // MSAA draws each pixel 4 times at slightly different spots and averages them, to soften edges
}

impl Targets {
    /// Create the textures for `size` at `samples`.
    pub fn new(ctx: &GpuCtx, size: (u32, u32), format: wgpu::TextureFormat, samples: u32) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let depth = texture_view(
            ctx,
            "depth",
            &TextureSpec {
                size,
                format: wgpu::TextureFormat::Depth32Float,
                samples,
                usage,
            },
        );
        let msaa = if samples > 1 {
            Some(texture_view(
                ctx,
                "msaa_color",
                &TextureSpec {
                    size,
                    format,
                    samples,
                    usage,
                },
            ))
        } else {
            None
        };

        // shaders bind both sample counts; the unused one is 1x1
        let other_samples = if samples == 1 { 4 } else { 1 };
        let empty_depth = texture_view(
            ctx,
            "unused.depth",
            &TextureSpec {
                size: (1, 1),
                format: wgpu::TextureFormat::Depth32Float,
                samples: other_samples,
                usage,
            },
        );
        let (depth_single, depth_msaa) = if samples == 1 {
            (depth.clone(), empty_depth)
        } else {
            (empty_depth, depth.clone())
        };
        Self {
            depth,
            msaa,
            depth_single,
            depth_msaa,
            samples,
        }
    }

    /// Open the face pass: color and depth cleared.
    pub fn begin_faces<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        clear: wgpu::Color,
    ) -> wgpu::RenderPass<'a> {
        let target = self.msaa.as_ref().unwrap_or(view);
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("physical face pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth,
                depth_ops: Some(wgpu::Operations {
                    // reverse-Z: 0 is the far plane
                    load: wgpu::LoadOp::Clear(0.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    /// Open the ink pass over the faces; depth is read, not written.
    pub fn begin_ink<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        let (target, resolve) = match &self.msaa {
            Some(msaa) => (msaa, Some(view)),
            None => (view, None),
        };
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("visible ink pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: resolve,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }
}

/// Settings for one 2D texture.
pub struct TextureSpec {
    pub size: (u32, u32), // width and height, px
    pub format: wgpu::TextureFormat, // pixel format
    pub samples: u32, // MSAA samples
    pub usage: wgpu::TextureUsages, // how the GPU may use it
}

/// Create a 2D texture from `spec`.
pub fn texture(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> wgpu::Texture {
    ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: spec.size.0.max(1),
            height: spec.size.1.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: spec.samples,
        dimension: wgpu::TextureDimension::D2,
        format: spec.format,
        usage: spec.usage,
        view_formats: &[],
    })
}

/// A texture's default view; wgpu keeps the texture alive.
pub fn texture_view(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> wgpu::TextureView {
    texture(ctx, label, spec).create_view(&wgpu::TextureViewDescriptor::default())
}
