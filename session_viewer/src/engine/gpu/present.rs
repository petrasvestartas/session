//! The three ways a frame leaves `Gpu`: presented to the swapchain (`present`), read back from
//! an offscreen texture (`render_offscreen`, the native harness), or timed in a batch
//! (`bench_frames`). Each writes the uniforms, encodes through `encode_frame`, and submits.

use super::frame::{FrameCx, FrameInput};
#[cfg(not(target_arch = "wasm32"))]
use super::targets::{texture, TextureSpec};
use super::Gpu;

impl Gpu {
    /// Per-frame uniforms, then the inside-flag refresh, which reads the eye just solved.
    fn write_frame_uniforms(&mut self, input: &FrameInput) {
        let size = (self.config.width, self.config.height);
        let occluder_rect = self.objects.occluder_rect(&input.view_proj.to_f32(), size, self.view.cloud_size);
        let cx = FrameCx { view: &self.view, anchor: self.objects.anchor_f32(), size, occluder_rect };
        self.frame.write(&self.ctx, input, &cx);
        self.objects.update_inside(&self.ctx, self.frame.eye, &self.bounds);
    }

    /// Draw one frame to the swapchain. Returns the encode time in ms, or `None` when the
    /// surface had no texture to give (it was reconfigured; the caller asks for another frame).
    pub fn present(&mut self, input: &FrameInput) -> Option<f64> {
        self.write_frame_uniforms(input);
        let surface = self.surface.as_ref()?;
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => {
                surface.configure(&self.ctx.device, &self.config);
                return None;
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });

        let t0 = crate::engine::performance::now_ms();
        let (draws, objects) = self.encode_frame(&mut encoder, &view, input.clear);
        let encode_ms = crate::engine::performance::now_ms() - t0;
        self.ctx.queue.submit([encoder.finish()]);
        self.pick.map();
        output.present();
        self.performance.frame(draws, objects, input.now_ms, self.view.perf);
        Some(encode_ms)
    }

    /// Render one frame into an offscreen texture and read the pixels back (RGBA8, tightly
    /// packed, top row first). Native only: the harness behind every measured number.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_offscreen(&mut self, input: &FrameInput) -> Vec<u8> {
        let (w, h) = (self.config.width, self.config.height);
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
        let tex = texture(&self.ctx, "headless.color", &TextureSpec { size: (w, h), format: self.config.format, samples: 1, usage });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
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
            wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(padded), rows_per_image: Some(h) } },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.ctx.queue.submit([encoder.finish()]);
        self.pick.map();
        log::info!("headless frame: {draws} draws, {objects} objects, {w}x{h}");

        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity((w * 4 * h) as usize);
        for row in 0..h {
            let a = (row * padded) as usize;
            out.extend_from_slice(&data[a..a + (w * 4) as usize]);
        }
        drop(data);
        readback.unmap();
        out
    }

    /// Capture picking's object IDs against this exact frame's physical surfaces and camera.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_ids_offscreen(&mut self, input: &FrameInput) -> Vec<[u32; 2]> {
        let size = (self.config.width, self.config.height);
        let texture = texture(&self.ctx, "headless.ids.color", &TextureSpec {
            size, format: self.config.format, samples: 1, usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.write_frame_uniforms(input);
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("headless.ids") });
        self.encode_frame(&mut encoder, &view, input.clear);
        self.id_pass(&mut encoder, None);
        let readback = self.pick.copy_frame(&self.ctx, &mut encoder);
        self.ctx.queue.submit([encoder.finish()]);
        readback.read(&self.ctx)
    }

    /// Time `frames` full frames into one offscreen target, GPU drained after each; returns
    /// seconds for the batch. The caller warms the caches with a first call it discards.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bench_frames(&mut self, input: &FrameInput, frames: u32) -> f64 {
        let (w, h) = (self.config.width, self.config.height);
        let tex = texture(&self.ctx, "bench.color", &TextureSpec { size: (w, h), format: self.config.format, samples: 1, usage: wgpu::TextureUsages::RENDER_ATTACHMENT });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        self.write_frame_uniforms(input);

        let t0 = std::time::Instant::now();
        for _ in 0..frames {
            let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
            self.encode_frame(&mut encoder, &view, input.clear);
            self.ctx.queue.submit([encoder.finish()]);
            let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        }
        t0.elapsed().as_secs_f64()
    }
}
