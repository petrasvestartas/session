// --8<-- [start:003-targets]
use super::buffers::GpuCtx;


// A render target is a texture a pass draws into; the depth texture keeps, per pixel, how near the closest surface is.
/// The frame's depth and color textures at one sample count.
pub struct Targets {
    pub depth: Attachment,                  // scene depth
    pub samples: u32,                       // MSAA samples, 1 or 4
}

impl Targets {
    /// Create the textures for `size` at `samples`.
    pub fn new(ctx: &GpuCtx, size: (u32, u32), format: wgpu::TextureFormat, samples: u32) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let attachment = |label, size, format, samples| {
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
        let depth = attachment("depth", size, wgpu::TextureFormat::Depth32Float, samples);
        Self {
            depth,
            samples,
        }
    }

    /// Free every texture now.
    pub fn destroy(&self) {
        self.depth.destroy();

    }
    // A render pass is one run of draws into a set of targets: `load` says what they start from, `store` whether the result is kept.
    // `'a` ties the pass to the encoder and views it borrows: it may not outlive them.
    /// Open the face pass: color and depth cleared, or kept when `clear` is None.
    pub fn begin_faces<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        clear: Option<wgpu::Color>,
    ) -> wgpu::RenderPass<'a> {
        let target = view;
        // a pass after the first keeps what the one before drew
        let load = |color| match clear {
            Some(_) => wgpu::LoadOp::Clear(color),
            None => wgpu::LoadOp::Load,
        };
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("physical face pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: load(clear.unwrap_or(wgpu::Color::TRANSPARENT)),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth,
                depth_ops: Some(wgpu::Operations {
                    // reverse-Z: near is 1 and far is 0, which keeps float depth precise far away
                    load: match clear {
                        Some(_) => wgpu::LoadOp::Clear(0.0),
                        None => wgpu::LoadOp::Load,
                    },
                    store: wgpu::StoreOp::Store,
                }),
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
    pub size: (u32, u32),            // width and height, px
    pub format: wgpu::TextureFormat, // pixel format
    pub samples: u32,                // MSAA samples
    pub usage: wgpu::TextureUsages,  // how the GPU may use it
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

/// A texture and its view; dropping it frees the memory at once.
pub struct Attachment {
    texture: wgpu::Texture,
    pub view: wgpu::TextureView, // its default view
}

impl Attachment {
    /// Create a texture from `spec` and its default view.
    pub fn new(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> Self {
        let texture = texture(ctx, label, spec);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }

    /// The texture itself, for copies.
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// Free the memory now; destroying twice is fine.
    pub fn destroy(&self) {
        self.texture.destroy();
    }
}

// Deref lets `&attachment` stand in for `&TextureView`, the way a smart pointer stands in for its value.
/// An Attachment can be used wherever a view is expected.
impl std::ops::Deref for Attachment {
    type Target = wgpu::TextureView;

    fn deref(&self) -> &wgpu::TextureView {
        &self.view
    }
}

/// Free the texture on drop.
impl Drop for Attachment {
    /// Rust runs this when the value goes out of scope.
    fn drop(&mut self) {
        self.texture.destroy();
    }
}
// --8<-- [end:003-targets]
