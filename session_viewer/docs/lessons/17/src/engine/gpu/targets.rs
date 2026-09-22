use super::buffers::GpuCtx;

/// Pixels a discrete GPU may draw at 4x MSAA.
const MSAA_PIXELS_DISCRETE: u32 = 9_000_000;

// --8<-- [start:step-39a]
/// Device scale from which MSAA is off unless forced.
const MSAA_MAX_PIXEL_SCALE: f32 = 2.0;

/// Pixels an integrated GPU may draw at 4x MSAA.
// --8<-- [end:step-39a]
const MSAA_PIXELS_SHARED: u32 = 2_500_000;

/// Pixels at 4x when the GPU type is unknown; every browser lands here.
const MSAA_PIXELS_UNKNOWN: u32 = 4_200_000;

/// The frame's depth and color textures at one sample count.
pub struct Targets {
    pub depth: wgpu::TextureView, // scene depth
    pub msaa: Option<wgpu::TextureView>, // 4x color, if on
    pub depth_single: wgpu::TextureView, // depth at 1x, or a 1x1 placeholder
    pub depth_msaa: wgpu::TextureView, // depth at 4x, or a 1x1 placeholder
    pub samples: u32, // MSAA samples, 1 or 4
    pub gradient: wgpu::TextureView, // depth slope per pixel
    pub gradient_single: wgpu::TextureView, // gradient at 1x, or a placeholder
    pub gradient_msaa: wgpu::TextureView, // gradient at 4x, or a placeholder
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
                size: (1, 1), // resized on the first frame
                format: wgpu::TextureFormat::Depth32Float,
                samples: other_samples, // 1 or 4
                usage,
            },
        );
        let (depth_single, depth_msaa) = if samples == 1 {
            (depth.clone(), empty_depth)
        } else {
            (empty_depth, depth.clone())
        };
        let gradient = texture_view(
            ctx,
            "physical.gradient",
            &TextureSpec {
                size,
                format: wgpu::TextureFormat::Rg16Float, // two half floats
                samples,
                usage,
            },
        );
        let empty_gradient = texture_view(
            ctx,
            "unused.gradient",
            &TextureSpec {
                size: (1, 1), // resized on the first frame
                format: wgpu::TextureFormat::Rg16Float, // two half floats
                samples: other_samples, // 1 or 4
                usage,
            },
        );
        let (gradient_single, gradient_msaa) = if samples == 1 {
            (gradient.clone(), empty_gradient)
        } else {
            (empty_gradient, gradient.clone())
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

    // --8<-- [start:step-39b]
    /// Sample count: 4x only with solids, within budget, below device scale 2.
    pub fn samples_for(
        solid: bool,
        pixels: u32,
        forced: Option<u32>,
        budget: Option<u32>,
        pixel_scale: f32,
    ) -> u32 {
        if let Some(s) = forced {
            return if s == 4 { 4 } else { 1 };
        }

        if super::view::reduced() || pixel_scale >= MSAA_MAX_PIXEL_SCALE {
            return 1;
        }

// --8<-- [end:step-39b]
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
        let target = self.msaa.as_ref().unwrap_or(view);
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

/// A texture's default view.
pub fn texture_view(ctx: &GpuCtx, label: &str, spec: &TextureSpec) -> wgpu::TextureView {
    texture(ctx, label, spec).create_view(&wgpu::TextureViewDescriptor::default())
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
            // --8<-- [start:step-39c]
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
        // --8<-- [end:step-39c]
    }

    /// The unknown-GPU budget keeps 4x at 2560x1440 but not at 4K.
    #[test]
    fn the_browser_arm_is_not_the_integrated_one() {
        let browser = Targets::msaa_budget(wgpu::DeviceType::Other);
        let shared = Targets::msaa_budget(wgpu::DeviceType::IntegratedGpu);
        // --8<-- [start:step-39d]
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
        // --8<-- [end:step-39d]
    }
}
