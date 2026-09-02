//! The three ways a frame leaves `Gpu`: presented to the swapchain (`clear`), read back from
//! an offscreen texture (`render_offscreen`, the native harness), or timed in a batch
//! (`bench_frames`). Each writes the uniforms (`write_frame_uniforms`), encodes through
//! `encode_frame` (render.rs), and submits.

use session_rust::Xform;
use crate::engine::performance::now_ms;
use super::Gpu;
use super::frame::{FrameCx, FrameInput};

impl Gpu {
    /// Per-frame uniforms through `FrameUniforms::write`, then the inside-flag refresh, which
    /// reads the eye it solved.
    fn write_frame_uniforms(&mut self, input: &FrameInput) {
        let anchor = self.objects.anchor_f32();
        let cx = FrameCx { view: &self.view, anchor, size: (self.config.width, self.config.height) };
        self.frame.write(&self.ctx, input, &cx);
        self.objects.update_inside(&self.ctx, self.frame.eye, &self.bounds);
    }

    /// Draw one frame to the swapchain. The frame ENCODING lives in `encode_frame` so a
    /// headless harness can aim the same code at an offscreen texture and read the pixels back -
    /// see `selftest.rs`. Shader work that is only ever checked in a browser is shader work
    /// checked by somebody else's eyes.
    pub fn clear(&mut self, input: &FrameInput) -> anyhow::Result<()> {
        self.write_frame_uniforms(input);

        // wgpu 29: get_current_texture() returns an enum, not a Result.
        let Some(surface) = &self.surface else { return Ok(()) }; // headless: nothing to present
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => { surface.configure(&self.ctx.device, &self.config); return Ok(()); }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("clear encoder"),
        });
        let (draws, objects) = self.encode_frame(&mut encoder, &view, input.clear);
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        self.performance.frame(draws, objects, input.now_ms);
        Ok(())
    }

    /// Render one frame into an offscreen texture and read the pixels back (RGBA8, tightly
    /// packed, top row first). Native only - this is the harness that lets a shader be looked at
    /// on this machine before it is shipped to a browser.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_offscreen(&mut self, input: &FrameInput) -> Vec<u8> {
        let (w, h) = (self.config.width, self.config.height);
        let tex = self.ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("headless.color"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        // copy_texture_to_buffer needs each row padded to 256 B
        let unpadded = w * 4;
        let pad = (256 - unpadded % 256) % 256;
        let padded = unpadded + pad;
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
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(padded), rows_per_image: Some(h) },
            },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.ctx.queue.submit([encoder.finish()]);
        log::info!("headless frame: {draws} draws, {objects} objects, {w}x{h}");

        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity((unpadded * h) as usize);
        for row in 0..h {
            let a = (row * padded) as usize;
            out.extend_from_slice(&data[a..a + unpadded as usize]);
        }
        drop(data);
        readback.unmap();
        out
    }

    /// Time `frames` full frames (encode + submit), reusing one offscreen target, and wait for
    /// the GPU to drain. Native bench helper: returns seconds for the whole batch, warmup
    /// excluded, so two line styles can be compared on the same scene.
    pub fn bench_frames(&mut self, view_proj: &Xform, frames: u32) -> f64 {
        let (w, h) = (self.config.width, self.config.height);
        let tex = self.ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("bench.color"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let clear = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };
        let input = FrameInput { view_proj: view_proj.clone(), clear, now_ms: now_ms() };
        self.write_frame_uniforms(&input);
        for _ in 0..3 { // warmup: pipeline/driver caches
            let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
            self.encode_frame(&mut encoder, &view, clear);
            self.ctx.queue.submit([encoder.finish()]);
        }
        let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        let t0 = std::time::Instant::now();
        for _ in 0..frames {
            let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
            self.encode_frame(&mut encoder, &view, clear);
            self.ctx.queue.submit([encoder.finish()]);
            let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        }
        t0.elapsed().as_secs_f64()
    }
}
