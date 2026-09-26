// --8<-- [start:003-targets]
use super::buffers::GpuCtx;


// A render target is a texture a pass draws into; the depth texture keeps, per pixel, how near the closest surface is.
/// The frame's depth and color textures at one sample count.
pub struct Targets {
    pub depth: Attachment,                  // scene depth
// --8<-- [start:005-fields]
    pub msaa: Option<Attachment>,           // multisampled color, only at 4x; register:msaa
    pub depth_single: wgpu::TextureView,    // depth at 1x, or a 1x1 placeholder; register:msaa
    pub depth_msaa: wgpu::TextureView,      // depth at 4x, or a 1x1 placeholder; register:msaa
// --8<-- [end:005-fields]
    pub samples: u32,                       // MSAA samples, 1 or 4
    pub gradient: Attachment, // triangle index + 1 per sample in two 16-bit halves, 0 for none; register:physical
    pub gradient_single: wgpu::TextureView, // triangle ids at 1x, or a placeholder; register:physical
    pub gradient_msaa: wgpu::TextureView, // triangle ids at 4x, or a placeholder; register:physical
    _placeholders: [Attachment; 2], // the 1x1 textures, freed with the rest; register:physical
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
// --8<-- [start:005-textures]
        let msaa = (samples > 1).then(|| attachment("msaa_color", size, format, samples)); // register:msaa

        // shaders bind both sample counts; the unused one is 1x1
        let other_samples = if samples == 1 { 4 } else { 1 }; // register:msaa
        let empty_depth = attachment( // register:msaa
            "unused.depth",
            (1, 1),
            wgpu::TextureFormat::Depth32Float,
            other_samples,
        );
        let (depth_single, depth_msaa) = if samples == 1 { // register:msaa
            (depth.view.clone(), empty_depth.view.clone())
        } else {
            (empty_depth.view.clone(), depth.view.clone())
        };
// --8<-- [end:005-textures]
        let gradient = attachment( // register:physical
            "physical.primitive",
            size,
            wgpu::TextureFormat::Rg16Uint,
            samples,
        );
        let empty_gradient = attachment( // register:physical
            "unused.primitive",
            (1, 1),
            wgpu::TextureFormat::Rg16Uint,
            other_samples,
        );
        let (gradient_single, gradient_msaa) = if samples == 1 { // register:physical
            (gradient.view.clone(), empty_gradient.view.clone())
        } else {
            (empty_gradient.view.clone(), gradient.view.clone())
        };
        Self {
            gradient,        // register:physical
            gradient_single, // register:physical
            gradient_msaa,   // register:physical
            depth,
// --8<-- [start:005-init]
            msaa,         // register:msaa
            depth_single, // register:msaa
            depth_msaa,   // register:msaa
// --8<-- [end:005-init]
            samples,
            _placeholders: [empty_depth, empty_gradient], // register:physical
        }
    }

    /// Free every texture now.
    pub fn destroy(&self) {
        self.depth.destroy();
        self.gradient.destroy(); // register:physical

// --8<-- [start:005-destroy]
        if let Some(msaa) = &self.msaa { // register:msaa
            msaa.destroy();
        }
// --8<-- [end:005-destroy]

        for placeholder in &self._placeholders { // register:physical
            placeholder.destroy();
        }
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
// --8<-- [start:005-target]
        let target = self.msaa.as_deref().unwrap_or(target); // register:msaa
// --8<-- [end:005-target]
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
                Some(wgpu::RenderPassColorAttachment { // register:physical
                    view: &self.gradient,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: load(wgpu::Color::TRANSPARENT),
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

// --8<-- [start:005-msaa]
impl Targets {
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
}

// MSAA = multisample anti-aliasing: 4 samples per pixel smooth the edges and cost 4 times the memory.
/// Pixels a discrete GPU may draw at 4x MSAA.
const MSAA_PIXELS_DISCRETE: u32 = 9_000_000;

/// Device scale from which MSAA is off unless forced.
const MSAA_MAX_PIXEL_SCALE: f32 = 2.0;

/// Pixels an integrated GPU may draw at 4x MSAA.
const MSAA_PIXELS_SHARED: u32 = 2_500_000;

/// Pixels at 4x when the GPU type is unknown; every browser lands here.
const MSAA_PIXELS_UNKNOWN: u32 = 4_200_000;
impl Targets {
    /// Sample count: 4x only with solids, within budget, below device scale 2.
    pub fn samples_for(
        solid: bool,
        pixels: u32,
        forced: Option<u32>,
        budget: Option<u32>,
        pixel_scale: f32,
    ) -> u32 {
        // a page that lost its device stays at 1x
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
}
// --8<-- [end:005-msaa]
// --8<-- [start:04a-tail]

impl Targets {

    /// Open the ink pass over the faces; depth is read, not written.
    pub fn begin_ink<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        // at 4x the pass resolves into the canvas: each pixel's 4 samples are averaged into one
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
// --8<-- [end:04a-tail]
