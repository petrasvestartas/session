//! `Targets` - the depth and MSAA colour attachments a frame renders into, sized to the
//! surface at the sample count the scene chose (`samples_for`), and the one render pass that
//! clears them. Nothing here knows what is drawn; it only opens the pass.

use super::buffers::GpuCtx;

/// Above this many pixels the frame stays at 1x: 4x colour + 4x depth scale with DPR², and at
/// 3840x2160 they were 266 MiB against 36 at 1x. 4.2 M = 2560x1440 DPR 1.1, the common laptop.
const MSAA_MAX_PIXELS: u32 = 4_200_000;

/// The attachments of the frame's render pass, and the sample count they were made at. `msaa`
/// exists only at 4x - the 1x texture used to be allocated and never bound (8-127 MiB).
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: Option<wgpu::TextureView>,
    pub samples: u32,
}

impl Targets {
    /// Both attachments for `config`'s size and format at `samples` (1 or 4).
    pub fn new(ctx: &GpuCtx, config: &wgpu::SurfaceConfiguration, samples: u32) -> Self {
        let depth = depth_view(&ctx.device, config, samples);
        let msaa = if samples > 1 { Some(msaa_view(&ctx.device, config, samples)) } else { None };

        Self { depth, msaa, samples }
    }

    /// The sample count a frame gets: 4x only when SOLID geometry (faces, pipes, spheres) is on
    /// the GPU AND the canvas is at most `MSAA_MAX_PIXELS`, else 1x. Hard edges are the only
    /// thing MSAA smooths; ribbons, dots and splats antialias themselves, so a pure sheet pays
    /// nothing. `override` (`VIEWER_MSAA` / `?msaa=`) wins outright: 4 is 4x, anything else 1x.
    pub fn samples_for(solid: bool, pixels: u32, override_samples: Option<u32>) -> u32 {
        if let Some(s) = override_samples {
            return if s == 4 { 4 } else { 1 };
        }
        if solid && pixels <= MSAA_MAX_PIXELS { 4 } else { 1 }
    }

    /// Open the frame's render pass: colour cleared to `clear`, depth cleared to 0 (reverse-Z
    /// far). At 1x the pass draws straight into `view`.
    pub fn begin_pass<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        clear: wgpu::Color,
    ) -> wgpu::RenderPass<'a> {
        // MSAA off: draw straight to the swapchain view - a 1-sample attachment must NOT
        // carry a resolve target.
        let (target, resolve) = match &self.msaa {
            Some(msaa) => (msaa, Some(view)),
            None => (view, None),
        };
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: resolve,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth,
                depth_ops: Some(
                    wgpu::Operations{load: wgpu::LoadOp::Clear(0.0),
                    store:wgpu::StoreOp::Store,
                }),
                stencil_ops: None }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }
}

/// Create the reverse-Z depth texture view, sized to the surface at the MSAA sample count.
fn depth_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> wgpu::TextureView {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth"),
        size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
        mip_level_count: 1,
        sample_count: samples,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

/// Create the multisampled color target the frame renders into (resolved to the surface each frame).
fn msaa_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> wgpu::TextureView {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("msaa_color"),
        size: wgpu::Extent3d{ width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
        mip_level_count: 1,
        sample_count: samples,
        dimension: wgpu::TextureDimension::D2,
        format: config.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}
