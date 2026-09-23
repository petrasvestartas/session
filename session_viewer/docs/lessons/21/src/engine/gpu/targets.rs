use super::buffers::GpuCtx;

/// Pixels a discrete GPU may draw at 4x MSAA.
const MSAA_PIXELS_DISCRETE: u32 = 9_000_000;

/// Device scale from which MSAA is off unless forced.
const MSAA_MAX_PIXEL_SCALE: f32 = 2.0;

/// Pixels an integrated GPU may draw at 4x MSAA.
const MSAA_PIXELS_SHARED: u32 = 2_500_000;

/// Pixels at 4x when the GPU type is unknown; every browser lands here.
const MSAA_PIXELS_UNKNOWN: u32 = 4_200_000;

// --8<-- [start:step-24a]
/// The frame's depth and color textures at one sample count.
pub struct Targets {
    pub depth: Attachment,
    pub msaa: Option<Attachment>, // multisampled color, only at 4x
    pub depth_single: wgpu::TextureView, // depth at 1x, or a 1x1 placeholder
    pub depth_msaa: wgpu::TextureView, // depth at 4x, or a 1x1 placeholder
    pub samples: u32, // MSAA samples, 1 or 4
    pub gradient: Attachment, // depth slope per pixel
    pub gradient_single: wgpu::TextureView, // gradient at 1x, or a placeholder
    pub gradient_msaa: wgpu::TextureView, // gradient at 4x, or a placeholder
    _placeholders: [Attachment; 2], // the 1x1 textures, freed with the rest
    // --8<-- [end:step-24a]
}

impl Targets {
    /// Create the textures for `size` at `samples`.
    pub fn new(ctx: &GpuCtx, size: (u32, u32), format: wgpu::TextureFormat, samples: u32) -> Self {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        let attachment = |label, size, format, samples| {
            // --8<-- [start:step-24b]
            Attachment::new(
            // --8<-- [end:step-24b]
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

        // shaders bind both sample counts; the unused one is 1x1
        let other_samples = if samples == 1 { 4 } else { 1 };
        let empty_depth = attachment(
            "unused.depth",
            (1, 1),
            wgpu::TextureFormat::Depth32Float,
            other_samples,
        );
        let (depth_single, depth_msaa) = if samples == 1 {
            // --8<-- [start:step-24c]
            (depth.view.clone(), empty_depth.view.clone())
        } else {
            (empty_depth.view.clone(), depth.view.clone())
            // --8<-- [end:step-24c]
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
            // --8<-- [start:step-24d]
            (gradient.view.clone(), empty_gradient.view.clone())
        } else {
            (empty_gradient.view.clone(), gradient.view.clone())
            // --8<-- [end:step-24d]
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
            // --8<-- [start:step-24e]
            _placeholders: [empty_depth, empty_gradient],
        }
    }

    /// Free every texture now.
    pub fn destroy(&self) {
        self.depth.destroy();
        self.gradient.destroy();

        if let Some(msaa) = &self.msaa {
            msaa.destroy();
        }

        for placeholder in &self._placeholders {
            placeholder.destroy();
            // --8<-- [end:step-24e]
        }
    }

    /// Pixels this GPU type may draw at 4x; None = never.
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

    // --8<-- [start:step-24f]
    /// Sample count: 4x only with solids, within budget, below device scale 2.
    // --8<-- [end:step-24f]
    pub fn samples_for(
        solid: bool,
        pixels: u32,
        forced: Option<u32>,
        budget: Option<u32>,
        pixel_scale: f32,
    ) -> u32 {
        // --8<-- [start:step-24g]
        // a page that lost its device stays at 1x
        if super::view::reduced() {
            return 1;
        }

        if let Some(s) = forced {
            return if s == 4 { 4 } else { 1 };
        }

        if pixel_scale >= MSAA_MAX_PIXEL_SCALE {
        // --8<-- [end:step-24g]
            return 1;
        }

        match budget {
            Some(max) if solid && pixels <= max => 4,
            _ => 1,
        }
    }

    /// Open the face pass: color and depth cleared.
    pub fn begin_faces<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        clear: wgpu::Color,
    ) -> wgpu::RenderPass<'a> {
        // --8<-- [start:step-24h]
        let target = self.msaa.as_deref().unwrap_or(view);
        // --8<-- [end:step-24h]
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
        // --8<-- [start:step-24i]
        // at 4x the pass resolves into the canvas here
        let (target, resolve) = match self.msaa.as_deref() {
        // --8<-- [end:step-24i]
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
    pub samples: u32,
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

// --8<-- [start:step-24j]
/// A texture and its view; dropping it frees the memory at once.
pub struct Attachment {
    texture: wgpu::Texture,
    pub view: wgpu::TextureView,
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

/// An Attachment can be used wherever a view is expected.
impl std::ops::Deref for Attachment {
    type Target = wgpu::TextureView;

    fn deref(&self) -> &wgpu::TextureView {
        &self.view
    }
}

/// Free the texture on drop.
impl Drop for Attachment {
    fn drop(&mut self) {
        self.texture.destroy();
    }
    // --8<-- [end:step-24j]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The budget follows the GPU type, not the pixel count alone.
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
        // device scale 2: 1x unless forced
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

    /// The unknown-GPU budget keeps 4x at 2560x1440 but not at 4K.
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
