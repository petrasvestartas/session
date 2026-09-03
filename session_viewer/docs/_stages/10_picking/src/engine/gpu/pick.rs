//! Picking by id pass: on request the lanes redraw ONCE at 1x into an `Rg32Uint` target -
//! (object row + 1, sub-object id + 1) per pixel - and one texel is copied out and mapped
//! asynchronously. The answer arrives a frame later from `poll`. No CPU ray cast, and it
//! works for streamed clouds that never existed on the CPU.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use super::buffers::GpuCtx;
use super::targets::{TextureSpec, texture, texture_view};

/// What a pixel answered: the object row and the sub-object id (point row for clouds, 0 else).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pick {
    pub row: u32,
    pub sub: u32,
}

/// The id pass's attachments, made on the first pick and kept until the canvas resizes.
struct IdTargets {
    id: wgpu::Texture,
    id_view: wgpu::TextureView,
    depth: wgpu::TextureView,
    size: (u32, u32),
}

/// The pending request, the targets, the readback buffer and its completion flag.
pub struct Picker {
    pending: Option<(u32, u32)>,
    inflight: bool,
    /// A copy was encoded this frame and its buffer must be mapped once the submit is in.
    copied: bool,
    ready: Arc<AtomicBool>,
    readback: Option<wgpu::Buffer>,
    targets: Option<IdTargets>,
}

impl Picker {
    /// Nothing requested, nothing allocated.
    pub fn new() -> Self {
        Self { pending: None, inflight: false, copied: false, ready: Arc::new(AtomicBool::new(false)), readback: None, targets: None }
    }

    /// Ask for the ids under pixel (x, y). Ignored while an earlier pick is still in flight.
    pub fn request(&mut self, x: u32, y: u32) {
        if !self.inflight {
            self.pending = Some((x, y));
        }
    }

    /// Whether a pick is waiting for its answer (the shell keeps frames coming until it lands).
    pub fn busy(&self) -> bool {
        self.inflight || self.pending.is_some()
    }

    /// The request to serve this frame, if any.
    pub fn take_pending(&mut self) -> Option<(u32, u32)> {
        self.pending.take()
    }

    /// Open the id pass over targets of `size`, cleared to 0 (= nothing) and reverse-Z far.
    pub fn begin_pass<'a>(&'a mut self, ctx: &GpuCtx, encoder: &'a mut wgpu::CommandEncoder, size: (u32, u32)) -> wgpu::RenderPass<'a> {
        if self.targets.as_ref().map(|t| t.size) != Some(size) {
            let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
            let id = texture(ctx, "pick.id", &TextureSpec { size, format: wgpu::TextureFormat::Rg32Uint, samples: 1, usage });
            let id_view = id.create_view(&wgpu::TextureViewDescriptor::default());
            let depth = texture_view(ctx, "pick.depth", &TextureSpec { size, format: wgpu::TextureFormat::Depth32Float, samples: 1, usage: wgpu::TextureUsages::RENDER_ATTACHMENT });
            self.targets = Some(IdTargets { id, id_view, depth, size });
        }
        let t = self.targets.as_ref().unwrap();
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("pick pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &t.id_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &t.depth,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    /// Copy the texel at (x, y) into the readback buffer and start mapping it.
    pub fn copy_texel(&mut self, ctx: &GpuCtx, encoder: &mut wgpu::CommandEncoder, at: (u32, u32)) {
        let Some(t) = &self.targets else { return };
        let (x, y) = (at.0.min(t.size.0 - 1), at.1.min(t.size.1 - 1));
        let buf = self.readback.get_or_insert_with(|| readback_buffer(ctx));
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &t.id, mip_level: 0, origin: wgpu::Origin3d { x, y, z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(256), rows_per_image: Some(1) } },
            wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        );
        self.inflight = true;
        self.copied = true;
    }

    /// After the copy was submitted: map the buffer ONCE; `ready` flips when the map completes.
    /// A second `map_async` on a buffer still mapped is a wgpu panic, so this is a no-op until
    /// the next copy.
    pub fn map(&mut self) {
        if !self.copied {
            return;
        }
        self.copied = false;
        let Some(buf) = &self.readback else { return };
        let flag = self.ready.clone();
        buf.slice(..).map_async(wgpu::MapMode::Read, move |_| flag.store(true, Ordering::Release));
    }

    /// Collect a pick asked for earlier: `None` while in flight, `Some(None)` for background.
    pub fn poll(&mut self) -> Option<Option<Pick>> {
        if !self.inflight || !self.ready.load(Ordering::Acquire) {
            return None;
        }
        let buf = self.readback.as_ref()?;
        let (object, sub) = {
            let bytes = buf.slice(..).get_mapped_range();
            let object = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let sub = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            (object, sub)
        };
        buf.unmap();
        self.ready.store(false, Ordering::Release);
        self.inflight = false;
        if object == 0 {
            return Some(None);
        }
        Some(Some(Pick { row: object - 1, sub: sub.saturating_sub(1) }))
    }

    /// Drop the targets (the canvas resized); they are remade on the next pick.
    pub fn resize(&mut self) {
        self.targets = None;
    }
}

/// A 256 B readback buffer - one row of copy alignment holds our 8 bytes.
fn readback_buffer(ctx: &GpuCtx) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("pick.readback"),
        size: 256,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    })
}
