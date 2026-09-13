//! `Targets` - the physical depth and colour attachments at the scene's sample count. The
//! face pass establishes occlusion; the ink pass samples the depth without modifying it.

use super::buffers::GpuCtx;

/// How many pixels a discrete GPU carries at 4x: 3840x2160 and change. Measured on
/// `view_local` at that size, 4x cost one 8.2 ms against 6.9 at 1x - a fifth of the frame.
const MSAA_PIXELS_DISCRETE: u32 = 9_000_000;
/// From this many physical pixels per CSS pixel the canvas stays at 1x unless forced.
const MSAA_MAX_PIXEL_SCALE: f32 = 2.0;

/// The same for an integrated or virtual GPU, which shares its bandwidth with the CPU: the
/// same scene and size cost an Intel iGPU 108.9 ms against 46.5, well over twice the frame.
/// Shrinking the canvas does not buy that back - the same adapter needed 92.4 ms for 4x at
/// 2108x1186 - so a big canvas gives up the samples rather than the pixels.
const MSAA_PIXELS_SHARED: u32 = 2_500_000;

/// The budget when the adapter will not say what it is. THE BROWSER IS ALWAYS THIS: wgpu's
/// WebGPU backend hardcodes `device_type: DeviceType::Other` for every adapter
/// (`wgpu-29.0.4/src/backend/webgpu.rs:864`), because WebGPU exposes no such field. So this
/// arm, not the ones above, is what every wasm session gets, and it may not be read as
/// "probably integrated" - it is a discrete GPU exactly as often as it is not. It keeps the
/// memory bound that has always governed here: 4x colour + 4x depth are 265 MB at 3840x2160
/// against 66 at 1x. Sending the browser to the integrated arm instead costs every canvas
/// between 2.5 and 4.2 Mpx its samples - a 2560x1440 window, or a 1440x900 one at dpr 1.5.
const MSAA_PIXELS_UNKNOWN: u32 = 4_200_000;

/// The attachments of the frame's render pass and the sample count they were made at.
/// `msaa` exists only at 4x. The ink layout binds a single-sampled AND a multisampled depth
/// view, so the one not in use is a 1x1 placeholder, kept here so it is destroyed with the rest.
pub struct Targets {
    pub depth: Attachment,
    pub msaa: Option<Attachment>,
    pub depth_single: wgpu::TextureView,
    pub depth_msaa: wgpu::TextureView,
    pub samples: u32,
    pub gradient: Attachment,
    pub gradient_single: wgpu::TextureView,
    pub gradient_msaa: wgpu::TextureView,
    _placeholders: [Attachment; 2],
}

impl Targets {
    /// Frame attachments and the opposite-sample-count placeholder binding.
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
        let msaa = (samples > 1).then(|| attachment("msaa_color", size, format, samples));

        // The ink layout binds a single-sampled and a multisampled view of both physical
        // targets; the pair not in use is a 1x1 placeholder.
        let other_samples = if samples == 1 { 4 } else { 1 };
        let empty_depth = attachment(
            "unused.depth",
            (1, 1),
            wgpu::TextureFormat::Depth32Float,
            other_samples,
        );
        let (depth_single, depth_msaa) = if samples == 1 {
            (depth.view.clone(), empty_depth.view.clone())
        } else {
            (empty_depth.view.clone(), depth.view.clone())
        };
        let gradient = attachment(
            "physical.gradient",
            size,
            wgpu::TextureFormat::Rgba16Float,
            samples,
        );
        let empty_gradient = attachment(
            "unused.gradient",
            (1, 1),
            wgpu::TextureFormat::Rgba16Float,
            other_samples,
        );
        let (gradient_single, gradient_msaa) = if samples == 1 {
            (gradient.view.clone(), empty_gradient.view.clone())
        } else {
            (empty_gradient.view.clone(), gradient.view.clone())
        };
        Self {
            gradient,
            gradient_single,
            gradient_msaa,
            depth,
            msaa,
            depth_single,
            depth_msaa,
            samples,
            _placeholders: [empty_depth, empty_gradient],
        }
    }

    /// Destroy every attachment ahead of the set that replaces this one, so the peak during
    /// a resize is one set and not two - at 4x on a 4 Mpx canvas the difference is 265 MB.
    pub fn destroy(&self) {
        self.depth.destroy();
        self.gradient.destroy();
        if let Some(msaa) = &self.msaa {
            msaa.destroy();
        }
        for placeholder in &self._placeholders {
            placeholder.destroy();
        }
    }

    /// How many pixels this adapter carries at 4x, or `None` when 4x is never worth it. 4x
    /// colour + 4x depth scale with DPR², and at 3840x2160 they were 265 MB against 66 at 1x,
    /// but what decides the frame is the adapter: the same scene cost a discrete GPU a fifth
    /// more and an integrated one more than twice as much.
    pub fn msaa_budget(gpu: wgpu::DeviceType) -> Option<u32> {
        match gpu {
            wgpu::DeviceType::DiscreteGpu => Some(MSAA_PIXELS_DISCRETE),
            wgpu::DeviceType::IntegratedGpu | wgpu::DeviceType::VirtualGpu => {
                Some(MSAA_PIXELS_SHARED)
            }
            wgpu::DeviceType::Cpu => None,
            wgpu::DeviceType::Other => Some(MSAA_PIXELS_UNKNOWN),
        }
    }

    /// The sample count a frame gets: 4x only when SOLID geometry (faces, pipes, spheres) is on
    /// the GPU AND the canvas is within this adapter's `budget` AND the canvas is below two
    /// physical pixels per CSS pixel, else 1x. Hard edges are the only thing MSAA smooths;
    /// ribbons, dots and splats antialias themselves, and at device scale 2 the pixel density
    /// already halves the stair-steps, for a quarter of the attachment memory. `forced` wins,
    /// except over the reduction: a page that lost its device draws at 1x whatever it asked.
    pub fn samples_for(
        solid: bool,
        pixels: u32,
        forced: Option<u32>,
        budget: Option<u32>,
        pixel_scale: f32,
    ) -> u32 {
        if super::view::reduced() {
            return 1;
        }
        if let Some(s) = forced {
            return if s == 4 { 4 } else { 1 };
        }
        if pixel_scale >= MSAA_MAX_PIXEL_SCALE {
            return 1;
        }
        match budget {
            Some(max) if solid && pixels <= max => 4,
            _ => 1,
        }
    }

    /// Clear physical depth to reverse-Z far and write the faces. Multisampled colour
    /// resolves only after the following ink pass.
    pub fn begin_faces<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        clear: wgpu::Color,
    ) -> wgpu::RenderPass<'a> {
        let target = self.msaa.as_deref().unwrap_or(view);
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("physical face pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &self.gradient,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth,
                depth_ops: Some(wgpu::Operations {
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
    /// Ink samples physical depth while the read-only attachment preserves depth-tested sheets.
    pub fn begin_ink<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        let (target, resolve) = match self.msaa.as_deref() {
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

/// A texture the viewer owns, with its default view. Dropping it DESTROYS the texture: on the
/// web wgpu's drop is a no-op and the memory would wait for the JavaScript garbage collector,
/// which nothing in a wasm frame loop hurries - a window drag remade the frame's attachments
/// every frame, a quarter of a gigabyte each at 4x on a 4 Mpx canvas, until the device was
/// lost. Destroying after the last submit that used it is safe: queued work still completes.
pub struct Attachment {
    texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl Attachment {
    /// A 2D texture to `spec` and its default view.
    pub fn new(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> Self {
        let texture = texture(ctx, label, spec);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }

    /// The texture, for a copy out of it.
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    /// Give the memory back now rather than on drop: a caller about to make the replacement
    /// keeps one set alive instead of two. Destroying twice is allowed, so the drop stays.
    pub fn destroy(&self) {
        self.texture.destroy();
    }
}

impl std::ops::Deref for Attachment {
    type Target = wgpu::TextureView;

    fn deref(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl Drop for Attachment {
    fn drop(&mut self) {
        self.texture.destroy();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The sample count follows the ADAPTER, not the pixel count alone: a discrete GPU keeps
    /// 4x at 4K, a shared one gives it up well before, and a software one never has it.
    #[test]
    fn msaa_follows_the_adapter() {
        let discrete = Targets::msaa_budget(wgpu::DeviceType::DiscreteGpu);
        let shared = Targets::msaa_budget(wgpu::DeviceType::IntegratedGpu);
        let software = Targets::msaa_budget(wgpu::DeviceType::Cpu);
        assert_eq!(
            Targets::samples_for(true, 3840 * 2160, None, discrete, 1.0),
            4
        );
        assert_eq!(
            Targets::samples_for(true, 3840 * 2160, None, shared, 1.0),
            1
        );
        assert_eq!(
            Targets::samples_for(true, 1920 * 1080, None, shared, 1.0),
            4
        );
        assert_eq!(Targets::samples_for(true, 1, None, software, 1.0), 1);
        assert_eq!(Targets::samples_for(false, 1, None, discrete, 1.0), 1);
        assert_eq!(
            Targets::samples_for(true, 3840 * 2160, Some(1), discrete, 1.0),
            1
        );
        assert_eq!(
            Targets::samples_for(false, u32::MAX, Some(4), software, 1.0),
            4
        );
        // Device scale 2 halves the stair-steps already: 1x unless forced.
        assert_eq!(
            Targets::samples_for(true, 1920 * 1080, None, discrete, 2.0),
            1
        );
        assert_eq!(
            Targets::samples_for(true, 1920 * 1080, None, discrete, 1.5),
            4
        );
        assert_eq!(
            Targets::samples_for(true, 1920 * 1080, Some(4), discrete, 2.0),
            4
        );
    }

    /// THE BROWSER'S ARM. wgpu's WebGPU backend reports `DeviceType::Other` for every adapter
    /// there, so this is the only budget a wasm session can reach. Reading it as "integrated"
    /// costs an ordinary 2560x1440 window its samples; the memory bound still takes them away
    /// at 4K, where 4x really is 265 MB.
    #[test]
    fn the_browser_arm_is_not_the_integrated_one() {
        let browser = Targets::msaa_budget(wgpu::DeviceType::Other);
        let shared = Targets::msaa_budget(wgpu::DeviceType::IntegratedGpu);
        assert_eq!(
            Targets::samples_for(true, 2560 * 1440, None, browser, 1.0),
            4
        );
        assert_eq!(
            Targets::samples_for(true, 2560 * 1440, None, shared, 1.0),
            1
        );
        assert_eq!(
            Targets::samples_for(true, 3840 * 2160, None, browser, 1.0),
            1
        );
    }
}
