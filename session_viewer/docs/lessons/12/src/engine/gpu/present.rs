// --8<-- [start:step-2a]
//! Owns the surface and hands out the texture for this frame.
use super::Gpu;
use super::frame::{FrameCx, FrameInput};
#[cfg(not(target_arch = "wasm32"))]
use super::targets::{TextureSpec, texture};

impl Gpu {
    /// Write this frame's uniforms and prepare the widget and text.
    fn write_frame_uniforms(&mut self, input: &FrameInput) {
        let size = (self.config.width, self.config.height);
        let cx = FrameCx {
            view: &self.view,
            anchor: self.objects.anchor_f32(),
            size,
            // framebuffer pixels per CSS pixel
            pixel_scale: size.0 as f32 / self.logical_size[0].max(1.0) as f32,
        };
        self.frame.write(&self.ctx, input, &cx);
        self.objects
            .update_inside(&self.ctx, self.frame.eye, &self.bounds);
        let frame = super::text::TextFrame {
            mvp: self.frame.mvp_f32,
            origin: self.objects.anchor(),
            framebuffer: [size.0, size.1],
            logical: self.logical_size,
            ortho_half_height: self.frame.ortho_h,
        };

        if let Err(error) = self.text.prepare(&self.ctx, &frame) {
            log::warn!("text preparation: {error}");
        }
    }

    /// Draw one frame to the canvas; returns encode time in ms.
    pub fn present(&mut self, input: &FrameInput) -> Option<f64> {
        self.write_frame_uniforms(input);
        let surface = self.surface.as_ref()?;
        // this frame's canvas texture; None means try again
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => {
                surface.configure(&self.ctx.device, &self.config);
                return None;
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame"),
            });

        let t0 = crate::engine::performance::now_ms();
        let (draws, objects) = self.encode_frame(&mut encoder, &view, input.clear);
        let encode_ms = crate::engine::performance::now_ms() - t0;
        self.ctx.queue.submit([encoder.finish()]);
        self.pick.map();
        output.present();
        self.performance
            .frame(draws, objects, input.now_ms, self.view.perf);
        Some(encode_ms)
    }

// --8<-- [end:step-2a]
    // --8<-- [start:step-2b]
    /// Run only the id pass for a pick; nothing is shown.
    pub fn pick_frame(&mut self, input: &FrameInput, at: (u32, u32)) {
        self.write_frame_uniforms(input);
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("pick"),
            });
        self.point_pass(&mut encoder);
        self.id_pass(&mut encoder, Some(at));
        self.ctx.queue.submit([encoder.finish()]);
        self.pick.map();
    }

    // --8<-- [end:step-2b]
    // --8<-- [start:step-2c]
    /// Draw one frame into a texture and return its RGBA8 pixels; native only.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_offscreen(&mut self, input: &FrameInput) -> Vec<u8> {
        let (w, h) = (self.config.width, self.config.height);
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
        let tex = texture(
            &self.ctx,
            "headless.color",
            &TextureSpec {
                size: (w, h),
                format: self.config.format,
                samples: 1,
                usage,
            },
        );
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        // bytes per row, 256-aligned as wgpu requires
        let padded = (w * 4).div_ceil(256) * 256;
        let readback = self.ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("headless.readback"),
            size: (padded * h) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        self.write_frame_uniforms(input);
        let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
        let (draws, objects) = self.encode_frame(&mut encoder, &view, input.clear);
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded),
                    rows_per_image: Some(h),
                },
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );
        self.ctx.queue.submit([encoder.finish()]);
        self.pick.map();
        log::info!("headless frame: {draws} draws, {objects} objects, {w}x{h}");

        let slice = readback.slice(..);
        // wait for the copy to land on the CPU
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.ctx.device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity((w * 4 * h) as usize);

        // drop the row padding
        for row in 0..h {
            let a = (row * padded) as usize;
            out.extend_from_slice(&data[a..a + (w * 4) as usize]);
        }

        drop(data);
        readback.unmap();
        out
    }

    /// Draw one frame and return (object, sub) per pixel; native only.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_ids_offscreen(&mut self, input: &FrameInput) -> Vec<[u32; 2]> {
        let size = (self.config.width, self.config.height);
        let texture = texture(
            &self.ctx,
            "headless.ids.color",
            &TextureSpec {
                size,
                format: self.config.format,
                samples: 1,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.write_frame_uniforms(input);
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("headless.ids"),
            });
        self.encode_frame(&mut encoder, &view, input.clear);
        self.id_pass(&mut encoder, None);
        let readback = self.pick.copy_frame(&self.ctx, &mut encoder);
        self.ctx.queue.submit([encoder.finish()]);
        readback.read(&self.ctx)
    }
}
// --8<-- [end:step-2c]
