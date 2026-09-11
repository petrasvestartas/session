//! Picking by id pass: on request the lanes redraw ONCE at 1x into an `Rg32Uint` target -
//! (object row + 1, sub-object id + 1) per pixel - scissored to a small window about the
//! cursor, which is copied out and mapped asynchronously. `poll` answers with the nearest
//! ink hit in the window (a hairline or a dot is hard to land on exactly), else the nearest
//! face. No CPU ray cast, and it works for streamed clouds that never existed on the CPU.

use super::buffers::GpuCtx;
use super::frame::PickView;
use super::targets::{TextureSpec, texture, texture_view};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

/// What a pixel answered: the object row, and a sub-object id that is a TAGGED union rather
/// than one number. 0 is the object itself; bit 31 set is a segment, the low bits its row;
/// the top three bits equal to `faces::FACE_TAG` is a source face address; top bits `01`
/// (`DISC_ID_TAG` in `scene.wgsl`) is a control dot, its index in the low 30; a cloud answers
/// with its point row. The tags are disjoint so one channel carries every kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pick {
    pub row: u32,
    pub sub: u32,
}

/// The id pass's attachments: the size of the pick window plus its halo, not the canvas,
/// made on the first pick and kept while the window keeps that size.
struct IdTargets {
    id: wgpu::Texture,
    id_view: wgpu::TextureView,
    depth: wgpu::TextureView,
    gradient: wgpu::TextureView,
    size: (u32, u32),
}

/// Default selection tolerance in CSS pixels, independent of display stroke width.
pub const PICK_RADIUS: u32 = 6;
/// Texels rendered around the readback window: the ink visibility test fits planes from
/// neighbouring texels, so the attachment extends this far past what is copied out.
pub const PICK_HALO: u32 = 3;
/// Bounds readback allocations even at unusual browser zoom factors.
const MAX_RADIUS: u32 = 128;

/// The specialized pass renders only eligible targets against the complete scene depth.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PickMode {
    #[default]
    Object,
    Edge,
    /// Original edges take precedence; otherwise pick the visible source face.
    Component,
    Controls {
        parent: u32,
        cloud: bool,
    },
}

/// The source query's attachment lifetime; only later pages load accumulated candidate depth.
#[derive(Clone, Copy, PartialEq, Eq)]
enum SourcePhase {
    Inactive,
    FirstPage,
    MorePages,
}

/// The readback window: a `PICK_RADIUS` square about the cursor, clamped into the target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    /// The cursor's place inside the window.
    pub cx: u32,
    pub cy: u32,
    pub radius: u32,
}

impl Window {
    /// The window about `at` inside a target of `size` (both at least 1 px).
    pub fn about(at: (u32, u32), size: (u32, u32)) -> Self {
        Self::with_radius(at, size, PICK_RADIUS)
    }

    /// A bounded circular tolerance in framebuffer pixels; size is at least one pixel.
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

    /// The attachment that serves this window: the window and its halo, inside the canvas.
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

/// The row pitch of the window copy: `2 * MAX_RADIUS + 1` texels of 8 B, rounded up to wgpu's
/// 256 B copy alignment. It is sized for the widest window `configure` can ask for, not for
/// `PICK_RADIUS`, so one readback buffer serves every tolerance.
const ROW_BYTES: u32 = ((2 * MAX_RADIUS + 1) * 8).div_ceil(256) * 256;

/// The pending request, the targets, the readback buffer and its completion flag.
pub struct Picker {
    pending: Option<(u32, u32)>,
    inflight: bool,
    /// The window the in-flight copy covers.
    window: Window,
    /// A copy was encoded this frame and its buffer must be mapped once the submit is in.
    copied: bool,
    ready: Arc<AtomicU8>,
    generation: u64,
    submitted: u64,
    pub mode: PickMode,
    radius: u32,
    source_phase: SourcePhase,
    readback: Option<wgpu::Buffer>,
    targets: Option<IdTargets>,
    /// Where the targets sit in the canvas, set by `begin_pass`.
    view: PickView,
}

/// A native full-frame ID capture awaiting queue submission and readback.
#[cfg(not(target_arch = "wasm32"))]
pub(super) struct IdReadback {
    buffer: wgpu::Buffer,
    size: (u32, u32),
    row_bytes: u32,
}

#[cfg(not(target_arch = "wasm32"))]
impl IdReadback {
    /// Drain the submitted copy and retain each pixel's object ID, rejecting map failures.
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
    /// Pick staging capacity and integer-color/depth texture estimate, excluding driver overhead.
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

    /// Nothing requested, nothing allocated.
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

    /// Supersede an earlier request while retaining its buffer until mapping has completed.
    pub fn request(&mut self, x: u32, y: u32) {
        self.generation = self.generation.wrapping_add(1);
        self.pending = Some((x, y));
    }

    /// Set interaction mode and CSS tolerance using the actual logical-to-framebuffer scale.
    pub fn configure(&mut self, mode: PickMode, radius_css: f64, scale: f64) {
        self.mode = mode;
        self.radius = (radius_css * scale)
            .ceil()
            .clamp(1.0, f64::from(MAX_RADIUS)) as u32;
    }

    /// Invalidate pending and mapped results on camera, viewport, scene or mode changes.
    pub fn cancel(&mut self) {
        self.source_phase = SourcePhase::Inactive;
        self.generation = self.generation.wrapping_add(1);
        self.pending = None;
    }

    /// One source query retains physical point depth and IDs across every bounded page.
    pub fn start_source_query(&mut self) {
        self.cancel();
        self.source_phase = SourcePhase::FirstPage;
    }

    /// Whether the current pick accumulates source points rather than ordinary scene IDs.
    pub fn source_query(&self) -> bool {
        self.source_phase != SourcePhase::Inactive
    }
    /// Whether physical scene depth and the first source page have been encoded.
    pub fn source_initialized(&self) -> bool {
        self.source_phase == SourcePhase::MorePages
    }

    /// Clear physical object IDs once while keeping scene depth; later pages load both.
    pub fn begin_source<'a>(
        &'a mut self,
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
                view: &target.id_view,
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

    /// The same bounds must be used for both scissoring and copying.
    pub fn window(&self, at: (u32, u32), size: (u32, u32)) -> Window {
        Window::with_radius(at, size, self.radius)
    }

    /// The attachment rectangle the pass draws for a pick at `at`, or the whole canvas.
    pub fn view_for(&self, at: Option<(u32, u32)>, size: (u32, u32)) -> PickView {
        match at {
            Some(at) => self.window(at, size).view(size),
            None => PickView::whole(size),
        }
    }

    /// Whether a pick is waiting for its answer (the shell keeps frames coming until it lands).
    pub fn busy(&self) -> bool {
        self.inflight || self.pending.is_some()
    }

    /// The request to serve this frame, if any.
    pub fn take_pending(&mut self) -> Option<(u32, u32)> {
        if self.inflight {
            None
        } else {
            self.pending.take()
        }
    }

    /// Open the id pass over targets the size of `view`, cleared to 0 (= nothing) and
    /// reverse-Z far.
    pub fn begin_pass<'a>(
        &'a mut self,
        ctx: &GpuCtx,
        encoder: &'a mut wgpu::CommandEncoder,
        view: PickView,
    ) -> wgpu::RenderPass<'a> {
        let size = (view.w, view.h);
        self.view = view;
        if !matches!(&self.targets, Some(targets) if targets.size == size) {
            let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
            let id = texture(
                ctx,
                "pick.id",
                &TextureSpec {
                    size,
                    format: wgpu::TextureFormat::Rg32Uint,
                    samples: 1,
                    usage,
                },
            );
            let id_view = id.create_view(&wgpu::TextureViewDescriptor::default());
            let depth = texture_view(
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
            let gradient = texture_view(
                ctx,
                "pick.gradient",
                &TextureSpec {
                    size,
                    format: wgpu::TextureFormat::Rgba16Float,
                    samples: 1,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                },
            );
            self.targets = Some(IdTargets {
                id,
                id_view,
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
                    view: &t.id_view,
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

    /// The same physical primitive gradient used by visible ink in the one-sample ID pass.
    pub fn gradient(&self) -> &wgpu::TextureView {
        &self.targets.as_ref().expect("physical ID targets").gradient
    }

    /// The ID pass's single-sample depth, read only after the physical pass has ended.
    pub fn depth(&self) -> Option<&wgpu::TextureView> {
        match &self.targets {
            Some(targets) => Some(&targets.depth),
            None => None,
        }
    }

    /// Append ink IDs using pixel-center visibility from this same single-sample target.
    pub fn begin_ink<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder) -> wgpu::RenderPass<'a> {
        let targets = self
            .targets
            .as_ref()
            .expect("physical ID pass initializes targets");
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("pick ink"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &targets.id_view,
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

    /// Copy the window about `at` (in canvas pixels of a `size` canvas) into the readback
    /// buffer; `map` starts the mapping after the submit.
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
                texture: &t.id,
                mip_level: 0,
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

    /// Capture the unchanged ID pass for a native census with exact object attribution.
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
                texture: &target.id,
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
        buf.slice(..)
            .map_async(wgpu::MapMode::Read, move |result| finish_map(&flag, result));
    }

    /// Collect a pick asked for earlier: `None` while in flight, `Some(None)` for background.
    /// Ink (a stroke, a marker, a cloud point) beats a face anywhere in the window; among
    /// equals the nearest to the cursor wins, so a click on a curve lying across a face still
    /// picks the curve.
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
        if self.submitted != self.generation {
            return None;
        }
        Some(best.map(decode_pick))
    }

    /// Drop the targets (the canvas resized); they are remade on the next pick.
    pub fn resize(&mut self) {
        self.cancel();
        self.targets = None;
    }
}

/// The (object, sub) texel of `win` to answer with: ink first, then the nearest to the cursor.
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
            if distance > u64::from(win.radius).pow(2) {
                continue;
            }
            let face = sub == 0 || sub.wrapping_sub(1) & 0xe000_0000 == super::faces::FACE_TAG;
            // The rule, as a tuple compared left to right: `false < true` in Rust, so ink
            // (face == false) beats a face outright, and only then does the nearer win. The
            // last two fields decide nothing real - they make the order total, so the same
            // pixels always answer the same way.
            let key = (face, distance, object, sub);
            match best {
                Some(previous) if key >= previous => {}
                _ => best = Some(key),
            }
        }
    }
    best.map(hit_ids)
}

/// The readback buffer: one aligned row per window line.
fn readback_buffer(ctx: &GpuCtx) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("pick.readback"),
        size: u64::from(ROW_BYTES) * u64::from(2 * MAX_RADIUS + 1),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    })
}

/// Publish mapping success or failure to the next frame without touching picker ownership.
fn finish_map(flag: &AtomicU8, result: Result<(), wgpu::BufferAsyncError>) {
    flag.store(if result.is_ok() { 1 } else { 2 }, Ordering::Release);
}

/// Forward native ID mapping completion to the waiting capture caller.
#[cfg(not(target_arch = "wasm32"))]
fn send_id_map(
    send: &std::sync::mpsc::SyncSender<Result<(), wgpu::BufferAsyncError>>,
    result: Result<(), wgpu::BufferAsyncError>,
) {
    send.send(result).expect("ID map receiver");
}

/// Convert one-based texture identities to the viewer's zero-based object and source IDs.
fn decode_pick((object, sub): (u32, u32)) -> Pick {
    Pick {
        row: object - 1,
        sub: sub.saturating_sub(1),
    }
}

/// Discard the ranking fields after the nearest eligible ID texel has been chosen.
fn hit_ids((_, _, object, sub): (bool, u64, u32, u32)) -> (u32, u32) {
    (object, sub)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pack synthetic ID texels with the production readback stride.
    fn texels(win: Window, hits: &[(u32, u32, u32, u32)]) -> Vec<u8> {
        let mut bytes = vec![0u8; (ROW_BYTES * win.h) as usize];
        for &(x, y, object, sub) in hits {
            let at = (y * ROW_BYTES + x * 8) as usize;
            bytes[at..at + 4].copy_from_slice(&object.to_le_bytes());
            bytes[at + 4..at + 8].copy_from_slice(&sub.to_le_bytes());
        }
        bytes
    }

    /// The window hugs the target's edges and remembers where the cursor sits in it.
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

    /// Ink beats a face under the cursor; nearer ink beats farther ink; a face alone is found.
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
