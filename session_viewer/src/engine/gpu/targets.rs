//! `Targets` - physical depth, face identity and colour attachments at the scene's sample
//! count. The face pass establishes occlusion; the ink pass samples it without modifying it.

use super::buffers::GpuCtx;

/// Above this many pixels the frame stays at 1x: 4x colour + 4x depth scale with DPR², and at
/// 3840x2160 they were 266 MiB against 36 at 1x.
const MSAA_MAX_PIXELS: u32 = 4_200_000;

/// The attachments of the frame's render pass and the sample count they were made at.
/// `msaa` exists only at 4x.
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: Option<wgpu::TextureView>,
    pub faces: wgpu::TextureView,
    pub depth_single: wgpu::TextureView,
    pub depth_msaa: wgpu::TextureView,
    pub faces_single: wgpu::TextureView,
    pub faces_msaa: wgpu::TextureView,
    pub samples: u32,
}

impl Targets {
    /// Frame attachments and opposite-sample-count placeholder bindings.
    pub fn new(ctx: &GpuCtx, size: (u32, u32), format: wgpu::TextureFormat, samples: u32) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let depth = texture_view(ctx, "depth", &TextureSpec { size, format: wgpu::TextureFormat::Depth32Float, samples, usage });
        let msaa = if samples > 1 {
            Some(texture_view(ctx, "msaa_color", &TextureSpec { size, format, samples, usage }))
        } else {
            None
        };

        let faces = texture_view(ctx, "physical.faces", &TextureSpec { size, format: wgpu::TextureFormat::Rg16Uint, samples, usage });
        let other_samples = if samples == 1 { 4 } else { 1 };
        let empty_depth = texture_view(ctx, "unused.depth", &TextureSpec { size: (1, 1), format: wgpu::TextureFormat::Depth32Float, samples: other_samples, usage });
        let empty_faces = texture_view(ctx, "unused.faces", &TextureSpec { size: (1, 1), format: wgpu::TextureFormat::Rg16Uint, samples: other_samples, usage });
        let (depth_single, depth_msaa, faces_single, faces_msaa) = if samples == 1 {
            (depth.clone(), empty_depth, faces.clone(), empty_faces)
        } else {
            (empty_depth, depth.clone(), empty_faces, faces.clone())
        };
        Self { depth, msaa, faces, depth_single, depth_msaa, faces_single, faces_msaa, samples }
    }

    /// The sample count a frame gets: 4x only when SOLID geometry (faces, pipes, spheres) is on
    /// the GPU AND the canvas is at most `MSAA_MAX_PIXELS`, else 1x. Hard edges are the only
    /// thing MSAA smooths; ribbons, dots and splats antialias themselves. `forced` wins outright.
    pub fn samples_for(solid: bool, pixels: u32, forced: Option<u32>) -> u32 {
        if let Some(s) = forced {
            return if s == 4 { 4 } else { 1 };
        }
        if solid && pixels <= MSAA_MAX_PIXELS { 4 } else { 1 }
    }

    /// Clear physical depth to reverse-Z far and record the nearest face identity.
    /// Multisampled colour resolves only after the following ink pass.
    pub fn begin_faces<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView, clear: wgpu::Color) -> wgpu::RenderPass<'a> {
        let target = self.msaa.as_ref().unwrap_or(view);
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("physical face pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear), store: wgpu::StoreOp::Store },
            }), Some(wgpu::RenderPassColorAttachment {
                view: &self.faces,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }
    /// Ink samples physical depth while the read-only attachment preserves depth-tested sheets.
    pub fn begin_ink<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView) -> wgpu::RenderPass<'a> {
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
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
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

/// What a 2D texture is made of: pixels, format, sample count, and what it is used for.
pub struct TextureSpec {
    pub size: (u32, u32),
    pub format: wgpu::TextureFormat,
    pub samples: u32,
    pub usage: wgpu::TextureUsages,
}

/// A 2D texture to `spec`.
pub fn texture(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> wgpu::Texture {
    ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d { width: spec.size.0.max(1), height: spec.size.1.max(1), depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: spec.samples,
        dimension: wgpu::TextureDimension::D2,
        format: spec.format,
        usage: spec.usage,
        view_formats: &[],
    })
}

/// A 2D texture's default view, the texture itself dropped (wgpu keeps it alive).
pub fn texture_view(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> wgpu::TextureView {
    texture(ctx, label, spec).create_view(&wgpu::TextureViewDescriptor::default())
}
