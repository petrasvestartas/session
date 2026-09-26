use super::buffers::GpuCtx;
use super::frame::PickView;
use super::targets::{Attachment, TextureSpec};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

/// What the pixel under the cursor holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pick {
    pub row: u32, // object row
    pub sub: u32, // 0 = the object; else a tagged edge, face, dot or point id
}

/// Textures the id pass draws into, sized to the pick window.
struct IdTargets {
    id: Attachment,       // object and sub id per pixel
    depth: Attachment,    // depth per pixel
    gradient: Attachment, // triangle index + 1 per pixel, 0 for none
    size: (u32, u32),     // texture size, px
}

/// Default click tolerance, CSS pixels.
pub const PICK_RADIUS: u32 = 6;

/// Extra pixels drawn around the window, for the visibility test.
pub const PICK_HALO: u32 = 3;

/// Largest tolerance, framebuffer pixels.
const MAX_RADIUS: u32 = 128;

pub use super::lane::PickMode;

/// Stage of a multi-page source point query.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SourcePhase {
    Inactive,  // no query running
    FirstPage, // first page clears the ids
    MorePages, // later pages keep them
}

/// The square of pixels read back around the cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub x: u32,      // left edge, px
    pub y: u32,      // top edge, px
    pub w: u32,      // width, px
    pub h: u32,      // height, px
    pub cx: u32,     // cursor x inside the window
    pub cy: u32,     // cursor y inside the window
    pub radius: u32, // tolerance, px
}

impl Window {
    /// The default window around `at` inside a `size` canvas.
    pub fn about(at: (u32, u32), size: (u32, u32)) -> Self {
        Self::with_radius(at, size, PICK_RADIUS)
    }

    /// A window of `radius` around `at`, kept inside the canvas.
    pub fn with_radius(at: (u32, u32), size: (u32, u32), radius: u32) -> Self {
        let radius = radius.min(MAX_RADIUS);
        let size = (size.0.max(1), size.1.max(1));
        let (w, h) = ((2 * radius + 1).min(size.0), (2 * radius + 1).min(size.1));
        let x = at.0.saturating_sub(radius).min(size.0 - w);
        let y = at.1.saturating_sub(radius).min(size.1 - h);
        Self {
            x,
            y,
            w,
            h,
            cx: at.0.min(size.0 - 1) - x,
            cy: at.1.min(size.1 - 1) - y,
            radius,
        }
    }

    /// The window plus its halo, as the pass draws it.
    pub fn view(&self, size: (u32, u32)) -> PickView {
        let size = (size.0.max(1), size.1.max(1));
        let x = self.x.saturating_sub(PICK_HALO);
        let y = self.y.saturating_sub(PICK_HALO);
        let right = (self.x + self.w + PICK_HALO).min(size.0);
        let bottom = (self.y + self.h + PICK_HALO).min(size.1);
        PickView {
            x,
            y,
            w: (right - x).max(1),
            h: (bottom - y).max(1),
        }
    }
}

/// Bytes per row of the readback buffer, 256-aligned as wgpu requires.
const ROW_BYTES: u32 = ((2 * MAX_RADIUS + 1) * 8).div_ceil(256) * 256;

/// Runs picks: request, draw, copy, map, read.
pub struct Picker {
    pending: Option<(u32, u32)>,    // cursor position waiting to be picked
    inflight: bool,                 // a copy is on the GPU
    window: Window,                 // window of the copy in flight
    copied: bool,                   // a copy was encoded this frame, map it after submit
    ready: Arc<AtomicU8>,           // 0 waiting, 1 mapped, 2 failed
    generation: u64,                // bumps on every request or cancel
    submitted: u64,                 // generation of the copy in flight
    pub mode: PickMode,             // what to answer with
    radius: u32,                    // tolerance, framebuffer px
    source_phase: SourcePhase,      // stage of a source point query
    readback: Option<wgpu::Buffer>, // CPU-readable copy of the window
    targets: Option<IdTargets>,     // id textures
    view: PickView,                 // where the textures sit in the canvas
}

/// A whole-frame id copy, native only.
#[cfg(not(target_arch = "wasm32"))]
pub(super) struct IdReadback {
    buffer: wgpu::Buffer, // CPU-readable copy
    size: (u32, u32),     // frame size, px
    row_bytes: u32,       // bytes per row, 256-aligned
}

#[cfg(not(target_arch = "wasm32"))]
impl IdReadback {
    /// Wait for the copy and return (object, sub) per pixel.
    pub(super) fn read(self, ctx: &GpuCtx) -> Vec<[u32; 2]> {
        let (send, receive) = std::sync::mpsc::sync_channel(1);
        self.buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                send_id_map(&send, result)
            });
        ctx.device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            })
            .expect("ID GPU poll");
        receive
            .recv()
            .expect("ID map callback")
            .expect("ID buffer map");
        let bytes = self.buffer.slice(..).get_mapped_range();
        let mut ids = Vec::with_capacity((self.size.0 * self.size.1) as usize);

        for y in 0..self.size.1 {
            let start = (y * self.row_bytes) as usize;

            for pixel in bytes[start..start + (self.size.0 * 8) as usize].chunks_exact(8) {
                ids.push([
                    u32::from_le_bytes(pixel[..4].try_into().expect("object ID bytes")),
                    u32::from_le_bytes(pixel[4..8].try_into().expect("sub-object ID bytes")),
                ]);
            }
        }

        drop(bytes);
        self.buffer.unmap();
        ids
    }
}

impl Picker {
    /// Bytes reserved on the GPU: (buffer, textures).
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let buffer = match &self.readback {
            Some(buffer) => buffer.size(),
            None => 0,
        };
        let pixels = match &self.targets {
            Some(target) => u64::from(target.size.0) * u64::from(target.size.1),
            None => 0,
        };
        (buffer, pixels * 20)
    }

    /// An idle picker with nothing allocated.
    pub fn new() -> Self {
        Self {
            pending: None,
            inflight: false,
            window: Window {
                x: 0,
                y: 0,
                w: 1,
                h: 1,
                cx: 0,
                cy: 0,
                radius: PICK_RADIUS,
            },
            copied: false,
            ready: Arc::new(AtomicU8::new(0)),
            generation: 0,
            submitted: 0,
            mode: PickMode::Object,
            radius: PICK_RADIUS,
            source_phase: SourcePhase::Inactive,
            readback: None,
            targets: None,
            view: PickView::whole((1, 1)),
        }
    }

    /// Ask for a pick at canvas pixel (x, y).
    pub fn request(&mut self, x: u32, y: u32) {
        self.generation = self.generation.wrapping_add(1);
        self.pending = Some((x, y));
    }

    /// Set the mode and the tolerance in CSS pixels.
    pub fn configure(&mut self, mode: PickMode, radius_css: f64, scale: f64) {
        self.mode = mode;
        self.radius = (radius_css * scale)
            .ceil()
            .clamp(1.0, f64::from(MAX_RADIUS)) as u32;
    }

    /// Drop the pending request and ignore any answer in flight.
    pub fn cancel(&mut self) {
        self.source_phase = SourcePhase::Inactive;
        self.generation = self.generation.wrapping_add(1);
        self.pending = None;
    }

    /// Start a source point query.
    pub fn start_source_query(&mut self) {
        self.cancel();
        self.source_phase = SourcePhase::FirstPage;
    }

    /// True while a source point query runs.
    pub fn source_query(&self) -> bool {
        self.source_phase != SourcePhase::Inactive
    }

    /// True once the first page of the query was drawn.
    pub fn source_initialized(&self) -> bool {
        self.source_phase == SourcePhase::MorePages
    }

    /// Open a pass for one query page; the first page clears the ids.
    pub fn begin_source<'a>(
        &mut self,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> wgpu::RenderPass<'a> {
        let first = self.source_phase == SourcePhase::FirstPage;
        self.source_phase = SourcePhase::MorePages;
        let target = self
            .targets
            .as_ref()
            .expect("physical query pass initializes targets");
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("source points"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target.id,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: if first {
                        wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                    } else {
                        wgpu::LoadOp::Load
                    },
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    /// The readback window around `at` at the current tolerance.
    pub fn window(&self, at: (u32, u32), size: (u32, u32)) -> Window {
        Window::with_radius(at, size, self.radius)
    }

    /// Area the id pass draws: the window plus halo, or the whole canvas.
    pub fn view_for(&self, at: Option<(u32, u32)>, size: (u32, u32)) -> PickView {
        match at {
            Some(at) => self.window(at, size).view(size),
            None => PickView::whole(size),
        }
    }

    /// True while a pick is requested or in flight.
    pub fn busy(&self) -> bool {
        self.inflight || self.pending.is_some()
    }

    /// The request to draw this frame, unless one is in flight.
    pub fn take_pending(&mut self) -> Option<(u32, u32)> {
        if self.inflight {
            None
        } else {
            self.pending.take()
        }
    }

    /// Open the id pass over textures sized to `view`, cleared.
    pub fn begin_pass<'a>(
        &mut self,
        ctx: &GpuCtx,
        encoder: &'a mut wgpu::CommandEncoder,
        view: PickView,
    ) -> wgpu::RenderPass<'a> {
        let size = (view.w, view.h);
        self.view = view;

        // remake the textures when the size changed
        if !matches!(&self.targets, Some(targets) if targets.size == size) {
            let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
            let id = Attachment::new(
                ctx,
                "pick.id",
                &TextureSpec {
                    size,
                    format: wgpu::TextureFormat::Rg32Uint,
                    samples: 1,
                    usage,
                },
            );
            let depth = Attachment::new(
                ctx,
                "pick.depth",
                &TextureSpec {
                    size,
                    format: wgpu::TextureFormat::Depth32Float,
                    samples: 1,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                },
            );
            let gradient = Attachment::new(
                ctx,
                "pick.primitive",
                &TextureSpec {
                    size,
                    format: wgpu::TextureFormat::Rg16Uint,
                    samples: 1,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                },
            );
            self.targets = Some(IdTargets {
                id,
                depth,
                gradient,
                size,
            });
        }

        let t = self.targets.as_ref().unwrap();
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("pick pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &t.id,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &t.gradient,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &t.depth,
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

    /// The triangle id texture of the id pass.
    pub fn gradient(&self) -> &wgpu::TextureView {
        &self.targets.as_ref().expect("physical ID targets").gradient
    }

    /// The depth texture of the id pass, once it exists.
    pub fn depth(&self) -> Option<&wgpu::TextureView> {
        match &self.targets {
            Some(targets) => Some(&targets.depth),
            None => None,
        }
    }

    /// Open a second pass that adds ink ids over the same textures.
    pub fn begin_ink<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder) -> wgpu::RenderPass<'a> {
        let targets = self
            .targets
            .as_ref()
            .expect("physical ID pass initializes targets");
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("pick ink"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &targets.id,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &targets.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    /// Copy the window around `at` into the readback buffer.
    pub fn copy_window(
        &mut self,
        ctx: &GpuCtx,
        encoder: &mut wgpu::CommandEncoder,
        at: (u32, u32),
        size: (u32, u32),
    ) {
        let Some(t) = &self.targets else { return };
        let win = self.window(at, size);

        if self.readback.is_none() {
            self.readback = Some(readback_buffer(ctx));
        }

        let buf = self.readback.as_ref().expect("readback initialized above");
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: t.id.texture(),
                mip_level: 0,
                // window position inside the textures
                origin: wgpu::Origin3d {
                    x: win.x.saturating_sub(self.view.x),
                    y: win.y.saturating_sub(self.view.y),
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: buf,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(ROW_BYTES),
                    rows_per_image: Some(win.h),
                },
            },
            wgpu::Extent3d {
                width: win.w,
                height: win.h,
                depth_or_array_layers: 1,
            },
        );
        self.window = win;
        self.submitted = self.generation;
        self.inflight = true;
        self.copied = true;
    }

    /// Copy the whole id texture, native only.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn copy_frame(
        &self,
        ctx: &GpuCtx,
        encoder: &mut wgpu::CommandEncoder,
    ) -> IdReadback {
        let target = self.targets.as_ref().expect("ID pass must precede capture");
        let row_bytes = (target.size.0 * 8).div_ceil(256) * 256;
        let buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("pick.frame.readback"),
            size: u64::from(row_bytes) * u64::from(target.size.1),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: target.id.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row_bytes),
                    rows_per_image: Some(target.size.1),
                },
            },
            wgpu::Extent3d {
                width: target.size.0,
                height: target.size.1,
                depth_or_array_layers: 1,
            },
        );
        IdReadback {
            buffer,
            size: target.size,
            row_bytes,
        }
    }

    /// Start mapping the readback buffer; call once after submit.
    pub fn map(&mut self) {
        if !self.copied {
            return;
        }

        self.copied = false;
        let Some(buf) = &self.readback else { return };
        let flag = self.ready.clone();
        buf.slice(..)
            .map_async(wgpu::MapMode::Read, move |result| finish_map(&flag, result));
    }

    /// Result of the last pick: None while in flight, Some(None) for background.
    pub fn poll(&mut self) -> Option<Option<Pick>> {
        let status = self.ready.load(Ordering::Acquire);

        if !self.inflight || status == 0 {
            return None;
        }

        if status == 2 {
            self.ready.store(0, Ordering::Release);
            self.inflight = false;
            log::warn!("selection readback failed; click again to retry");
            return None;
        }

        let buf = self.readback.as_ref()?;
        let win = self.window;
        let best = {
            let bytes = buf.slice(..).get_mapped_range();
            nearest_hit(&bytes, win)
        };
        buf.unmap();
        self.ready.store(0, Ordering::Release);
        self.inflight = false;

        // a newer request made this answer stale
        if self.submitted != self.generation {
            return None;
        }

        Some(best.map(decode_pick))
    }

    /// Drop the textures; the next pick remakes them.
    pub fn resize(&mut self) {
        self.cancel();
        self.targets = None;
    }
}

/// Best pixel in the window: ink before faces, then nearest to the cursor.
fn nearest_hit(bytes: &[u8], win: Window) -> Option<(u32, u32)> {
    let mut best: Option<(bool, u64, u32, u32)> = None;

    for y in 0..win.h {
        for x in 0..win.w {
            let at = (y * ROW_BYTES + x * 8) as usize;
            let object = u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());

            if object == 0 {
                continue;
            }

            let sub = u32::from_le_bytes(bytes[at + 4..at + 8].try_into().unwrap());
            let (dx, dy) = (x as i64 - win.cx as i64, y as i64 - win.cy as i64);
            let distance = (dx * dx + dy * dy) as u64;

            // outside the circle
            if distance > u64::from(win.radius).pow(2) {
                continue;
            }

            let face = sub == 0 || sub.wrapping_sub(1) & 0xe000_0000 == super::faces::FACE_TAG;
            // smallest tuple wins: ink first, then nearest
            let key = (face, distance, object, sub);

            match best {
                Some(previous) if key >= previous => {}
                _ => best = Some(key),
            }
        }
    }

    best.map(hit_ids)
}

/// The CPU-readable buffer for the largest window.
fn readback_buffer(ctx: &GpuCtx) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("pick.readback"),
        size: u64::from(ROW_BYTES) * u64::from(2 * MAX_RADIUS + 1),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    })
}

/// Record whether the map succeeded: 1 ok, 2 failed.
fn finish_map(flag: &AtomicU8, result: Result<(), wgpu::BufferAsyncError>) {
    flag.store(if result.is_ok() { 1 } else { 2 }, Ordering::Release);
}

/// Send the map result to the waiting thread.
#[cfg(not(target_arch = "wasm32"))]
fn send_id_map(
    send: &std::sync::mpsc::SyncSender<Result<(), wgpu::BufferAsyncError>>,
    result: Result<(), wgpu::BufferAsyncError>,
) {
    send.send(result).expect("ID map receiver");
}

/// Texture ids are one-based; 0 means nothing.
fn decode_pick((object, sub): (u32, u32)) -> Pick {
    Pick {
        row: object - 1,
        sub: sub.saturating_sub(1),
    }
}

/// Keep only the ids of the chosen pixel.
fn hit_ids((_, _, object, sub): (bool, u64, u32, u32)) -> (u32, u32) {
    (object, sub)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a readback buffer with the given hits.
    fn texels(win: Window, hits: &[(u32, u32, u32, u32)]) -> Vec<u8> {
        let mut bytes = vec![0u8; (ROW_BYTES * win.h) as usize];

        for &(x, y, object, sub) in hits {
            let at = (y * ROW_BYTES + x * 8) as usize;
            bytes[at..at + 4].copy_from_slice(&object.to_le_bytes());
            bytes[at + 4..at + 8].copy_from_slice(&sub.to_le_bytes());
        }

        bytes
    }

    /// The window stays inside the canvas and tracks the cursor.
    #[test]
    fn window_clamps() {
        let r = PICK_RADIUS;
        assert_eq!(
            Window::about((100, 100), (400, 300)),
            Window {
                x: 100 - r,
                y: 100 - r,
                w: 2 * r + 1,
                h: 2 * r + 1,
                cx: r,
                cy: r,
                radius: r
            }
        );
        assert_eq!(
            Window::about((0, 299), (400, 300)),
            Window {
                x: 0,
                y: 300 - (2 * r + 1),
                w: 2 * r + 1,
                h: 2 * r + 1,
                cx: 0,
                cy: 2 * r,
                radius: r
            }
        );
        assert_eq!(
            Window::about((5, 5), (4, 4)),
            Window {
                x: 0,
                y: 0,
                w: 4,
                h: 4,
                cx: 3,
                cy: 3,
                radius: r
            }
        );
    }

    /// Pick order: an edge or point wins over a face, then the nearest wins.
    #[test]
    fn ink_first_then_nearest() {
        let win = Window::about((50, 50), (200, 200));
        let (c, seg) = (PICK_RADIUS, 0x8000_0000 | 3);
        assert_eq!(
            nearest_hit(&texels(win, &[(c, c, 7, 0), (c + 5, c, 9, seg)]), win),
            Some((9, seg))
        );
        assert_eq!(
            nearest_hit(
                &texels(win, &[(c + 5, c, 9, seg), (c - 2, c + 1, 4, 12)]),
                win
            ),
            Some((4, 12))
        );
        assert_eq!(
            nearest_hit(&texels(win, &[(c + 3, c, 7, 0), (c - 1, c, 2, 0)]), win),
            Some((2, 0))
        );
        assert_eq!(nearest_hit(&texels(win, &[]), win), None);
    }
}

impl super::lane::Lane for Picker {
    fn on_reset(&mut self, _ctx: &GpuCtx) {
        self.cancel();
    }

    fn bytes(&self) -> (u64, u64) {
        self.allocated_bytes()
    }
}
