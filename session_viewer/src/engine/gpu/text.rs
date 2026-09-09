//! Dedicated coverage-text lane, pinned to Glyphon 0.11 (wgpu 29).
//!
//! The maintained shader is `glyphon-0.11.0/src/shader.wgsl`: R8Unorm coverage,
//! straight-alpha output with ALPHA_BLENDING, integer texel footprints and fractional
//! raster cache keys. It does not use mesh lighting. `ColorMode::Web` is appropriate
//! for non-sRGB targets; the viewer's sRGB targets use Accurate mode.

use super::buffers::GpuCtx;
#[path = "text_plane.rs"]
mod plane;
#[path = "text_plate.rs"]
mod plate;
use crate::engine::pipelines::Target;
use crate::engine::text::{TextDocument, TextLabel, TextPlacement, TextRun};
use glyphon::{
    Cache, Color, ColorMode, Resolution, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer,
    Viewport,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

/// Camera matrix is column-major in the viewer's rebased frame. Logical dimensions are
/// measured from the actual canvas CSS box, not from window.devicePixelRatio.
#[derive(Clone, Debug, PartialEq)]
pub struct TextFrame {
    pub mvp: [f32; 16],
    pub origin: [f64; 3],
    pub framebuffer: [u32; 2],
    pub logical: [f64; 2],
    pub ortho_half_height: f32,
}

/// Public counters distinguish measured submissions from internals Glyphon does not expose.
#[derive(Clone, Debug, Default, Serialize)]
pub struct TextStats {
    pub preparations: u64,
    pub skipped_preparations: u64,
    pub shape_count: u64,
    pub shaping_ms: f64,
    pub requested_glyphs: usize,
    pub distinct_raster_keys: usize,
    pub new_raster_keys: usize,
    /// Actual CPU raster images retained by Swash, excluding empty glyphs.
    pub raster_images: usize,
    pub new_raster_images: usize,
    /// Allocated byte-vector payload only; excludes hash-table/font/shaper overhead and GPU atlases.
    pub raster_image_capacity_bytes: usize,
    pub atlas_resets: u64,
    pub preparation_ms: f64,
    /// Glyphon's private packed instance is 28 bytes at the pin. This is active data only,
    /// not a claim about its private capacity or GPU allocator overhead.
    pub active_instance_bytes: usize,
    pub missing_glyphs: usize,
    /// Exact owned vertex capacity for black nameplate backgrounds, not Glyphon internals.
    pub nameplate_capacity_bytes: u64,
    /// Application-owned fixed-plane vertices, R8 texture payload, and cache rebuilds.
    pub world_plane_buffer_bytes: u64,
    pub world_plane_texture_bytes: u64,
    pub world_plane_rasterizations: u64,
}

/// One shared atlas/font stack for all labels, with separate overlay and occluded draw lists.
pub struct TextLane {
    pub document: TextDocument,
    pub stats: TextStats,
    cache: Cache,
    atlas: TextAtlas,
    viewport: Viewport,
    raster: SwashCache,
    overlay: TextRenderer,
    anchored: TextRenderer,
    plates: plate::Plates,
    planes: plane::Planes,
    target: Target,
    atlas_font_revision: u64,
    prepared: Option<(u64, u64, TextFrame)>,
    raster_keys: HashSet<glyphon::CacheKey>,
    overlay_count: u32,
    anchored_count: u32,
}

impl TextLane {
    /// Exact owned nameplate buffer capacity; Glyphon's private GPU capacities are excluded.
    pub fn allocated_bytes(&self) -> u64 {
        self.plates.allocated_bytes() + self.planes.buffer_bytes()
    }

    /// Exact fixed-plane coverage payload, excluding Glyphon's private atlas allocations.
    pub fn texture_bytes(&self) -> u64 {
        self.planes.texture_bytes()
    }

    /// Reuse the viewer's device and target; no independent device or scene model is created.
    pub fn new(ctx: &GpuCtx, target: Target) -> Self {
        let cache = Cache::new(&ctx.device);
        let viewport = Viewport::new(&ctx.device, &cache);
        let mut atlas = make_atlas(ctx, &cache, target.format);
        let overlay = renderer(ctx, &mut atlas, target, wgpu::CompareFunction::Always);
        let anchored = renderer(ctx, &mut atlas, target, wgpu::CompareFunction::GreaterEqual);
        Self {
            document: TextDocument::new(),
            stats: TextStats::default(),
            cache,
            atlas,
            viewport,
            raster: SwashCache::new(),
            overlay,
            anchored,
            plates: plate::Plates::new(ctx, target),
            planes: plane::Planes::new(ctx, target),
            target,
            atlas_font_revision: 1,
            prepared: None,
            raster_keys: HashSet::new(),
            overlay_count: 0,
            anchored_count: 0,
        }
    }

    /// Replace the label document. Invalid submissions preserve the previous labels.
    pub fn set_labels(&mut self, labels: Vec<TextLabel>) -> anyhow::Result<()> {
        self.document.set_labels(labels)
    }

    /// Match the scene pass sample count without reshaping text or discarding the atlas.
    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.plates.retarget(ctx, target);
        self.planes.retarget(ctx, target);
        self.overlay = renderer(ctx, &mut self.atlas, target, wgpu::CompareFunction::Always);
        self.anchored = renderer(
            ctx,
            &mut self.atlas,
            target,
            wgpu::CompareFunction::GreaterEqual,
        );
        self.prepared = None;
    }

    /// Invalidate placements on camera/scale changes; unchanged text never shapes here.
    /// Glyphon may evict unused atlas entries only between complete preparations. Both draw
    /// lists are then rebuilt together, so no prepared vertices retain an evicted allocation.
    pub fn prepare(&mut self, ctx: &GpuCtx, frame: &TextFrame) -> anyhow::Result<()> {
        let key = (
            self.document.revision,
            self.document.font_revision,
            frame.clone(),
        );
        if self.prepared.as_ref() == Some(&key) {
            self.stats.skipped_preparations += 1;
            return Ok(());
        }
        self.overlay_count = 0;
        self.anchored_count = 0;
        self.plates.reset();
        let scale = frame.scale()?;
        let start = now_ms();
        let fonts_changed = self.atlas_font_revision != self.document.font_revision;
        if fonts_changed || self.raster_keys.len() > 4096 {
            self.rebuild_resources(ctx);
        }
        let raster_images_before = self.raster.image_cache.values().flatten().count();
        self.planes
            .prepare(ctx, &mut self.document, &mut self.raster, frame)?;
        self.atlas.trim();
        self.viewport.update(
            &ctx.queue,
            Resolution {
                width: frame.framebuffer[0],
                height: frame.framebuffer[1],
            },
        );
        let mut overlays = Vec::new();
        let mut anchors = Vec::new();
        let mut depths = HashMap::new();
        let mut plates = Vec::new();
        self.stats.new_raster_keys = 0;
        self.stats.requested_glyphs = 0;
        self.stats.missing_glyphs = 0;
        for run in &self.document.runs {
            let Some(mut placed) = place(&run.label, frame, scale) else {
                continue;
            };
            if let Some(rectangle) = text_rectangle(run, &mut placed, frame, scale) {
                plates.push(rectangle);
            }
            let area = TextArea {
                buffer: &run.buffer,
                left: placed.left,
                top: placed.top,
                scale: placed.scale,
                bounds: clip_bounds(&run.label, frame, scale),
                default_color: Color::rgba(
                    run.label.color[0],
                    run.label.color[1],
                    run.label.color[2],
                    run.label.color[3],
                ),
                custom_glyphs: &[],
            };
            for line in run.buffer.layout_runs() {
                for glyph in line.glyphs {
                    self.stats.requested_glyphs += 1;
                    self.stats.missing_glyphs += usize::from(glyph.glyph_id == 0);
                    let physical = glyph.physical((placed.left, placed.top), placed.scale);
                    self.stats.new_raster_keys +=
                        usize::from(self.raster_keys.insert(physical.cache_key));
                }
            }
            if let Some(depth) = placed.depth {
                depths.insert(run.label.id as usize, depth);
                anchors.push(area);
            } else {
                overlays.push(area);
            }
        }
        self.overlay.prepare(
            &ctx.device,
            &ctx.queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            overlays,
            &mut self.raster,
        )?;
        // Glyphon requires a callback to map shaped-run metadata to per-label clip depth.
        self.anchored.prepare_with_depth(
            &ctx.device,
            &ctx.queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            anchors,
            &mut self.raster,
            |id| depth_for(&depths, id),
        )?;
        self.plates.prepare(ctx, &plates, frame.framebuffer);
        for run in &self.document.runs {
            if place(&run.label, frame, scale).is_some() && !run.label.text.is_empty() {
                match run.label.placement {
                    TextPlacement::Screen { .. } | TextPlacement::Nameplate { .. } => {
                        self.overlay_count = 1
                    }
                    _ => self.anchored_count = 1,
                }
            }
        }
        self.stats.raster_images = 0;
        self.stats.raster_image_capacity_bytes = 0;
        for image in self.raster.image_cache.values().flatten() {
            self.stats.raster_images += 1;
            self.stats.raster_image_capacity_bytes += image.data.capacity();
        }
        self.stats.new_raster_images = self
            .stats
            .raster_images
            .saturating_sub(raster_images_before);
        self.stats.preparations += 1;
        self.stats.shape_count = self.document.shape_count;
        self.stats.shaping_ms = self.document.shaping_ms;
        self.stats.distinct_raster_keys = self.raster_keys.len();
        self.stats.active_instance_bytes = self.stats.requested_glyphs * 28;
        self.stats.nameplate_capacity_bytes = self.plates.allocated_bytes();
        self.stats.world_plane_buffer_bytes = self.planes.buffer_bytes();
        self.stats.world_plane_texture_bytes = self.planes.texture_bytes();
        self.stats.world_plane_rasterizations = self.planes.rasterizations;
        self.stats.preparation_ms = now_ms() - start;
        self.prepared = Some(key);
        Ok(())
    }

    /// Authored text IDs use the same retained plane vertices as their visible quads.
    pub fn draw_ids(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        self.planes.draw_ids(pass) + self.plates.draw_ids(pass)
    }

    /// Draw against the scene's read-only depth attachment after scene ink, overlays last.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let mut draws = self.planes.draw(pass);
        draws += self.plates.draw(pass, false);
        if self.anchored_count != 0 {
            match self.anchored.render(&self.atlas, &self.viewport, pass) {
                Ok(()) => draws += 1,
                Err(error) => log::error!("anchored text render: {error}"),
            }
        }
        draws += self.plates.draw(pass, true);
        if self.overlay_count != 0 {
            match self.overlay.render(&self.atlas, &self.viewport, pass) {
                Ok(()) => draws += 1,
                Err(error) => log::error!("overlay text render: {error}"),
            }
        }
        draws
    }

    /// Hide text immediately while releasing source strings and shape buffers.
    pub fn reset(&mut self) {
        self.document.clear();
        self.prepared = None;
        self.overlay_count = 0;
        self.anchored_count = 0;
        self.plates.reset();
        self.planes.reset();
    }

    /// Release grown atlas and instance buffers on scene disposal, retaining only small empty resources.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.plates.release(ctx);
        self.planes.release(ctx);
        self.stats.nameplate_capacity_bytes = self.plates.allocated_bytes();
        self.stats.world_plane_buffer_bytes = self.planes.buffer_bytes();
        self.stats.world_plane_texture_bytes = 0;
        self.rebuild_resources(ctx);
    }

    /// Bound continuously changing raster keys; Glyphon's atlas evicts unused glyphs between
    /// preparations, and this reset also hands its grown allocations back to the device.
    fn rebuild_resources(&mut self, ctx: &GpuCtx) {
        self.atlas = make_atlas(ctx, &self.cache, self.target.format);
        self.overlay = renderer(
            ctx,
            &mut self.atlas,
            self.target,
            wgpu::CompareFunction::Always,
        );
        self.anchored = renderer(
            ctx,
            &mut self.atlas,
            self.target,
            wgpu::CompareFunction::GreaterEqual,
        );
        self.raster = SwashCache::new();
        self.raster_keys.clear();
        self.stats.raster_images = 0;
        self.stats.new_raster_images = 0;
        self.stats.raster_image_capacity_bytes = 0;
        self.stats.atlas_resets += 1;
        self.atlas_font_revision = self.document.font_revision;
        self.prepared = None;
    }
}

impl TextFrame {
    /// Derive one isotropic font raster scale from actual framebuffer/CSS dimensions. The
    /// one-pixel rounding difference between axes is allowed; stretched canvases are rejected.
    pub fn scale(&self) -> anyhow::Result<f32> {
        anyhow::ensure!(
            self.framebuffer[0] > 0 && self.framebuffer[1] > 0,
            "empty text framebuffer"
        );
        anyhow::ensure!(
            self.logical[0].is_finite()
                && self.logical[1].is_finite()
                && self.logical[0] > 0.0
                && self.logical[1] > 0.0,
            "empty text CSS box"
        );
        let x = self.framebuffer[0] as f64 / self.logical[0];
        let y = self.framebuffer[1] as f64 / self.logical[1];
        let tolerance = 1.0 / self.logical[0] + 1.0 / self.logical[1];
        anyhow::ensure!(
            (x - y).abs() <= tolerance,
            "text canvas is stretched non-uniformly"
        );
        Ok(y as f32)
    }
}

/// The glyph raster size and placement in physical framebuffer pixels.
struct PlacedText {
    left: f32,
    top: f32,
    scale: f32,
    depth: Option<f32>,
}

/// Project only the anchor. Camera-facing glyphs keep the anchor's clip depth; near/far and
/// behind-camera anchors are culled instead of producing inverted or infinitely large text.
fn place(label: &TextLabel, frame: &TextFrame, scale: f32) -> Option<PlacedText> {
    let (world, offset, world_height) = match label.placement {
        TextPlacement::WorldPlane { .. } => return None,
        TextPlacement::Screen { left, top } => {
            return Some(PlacedText {
                left: left * scale,
                top: top * scale,
                scale,
                depth: None,
            });
        }
        TextPlacement::Anchor { world, offset } => (world, offset, None),
        TextPlacement::Nameplate { world, .. } => (world, [0.0; 2], None),
        TextPlacement::WorldBillboard {
            world,
            world_height,
        } => (world, [0.0; 2], Some(world_height)),
    };
    let p = [
        (world[0] - frame.origin[0]) as f32,
        (world[1] - frame.origin[1]) as f32,
        (world[2] - frame.origin[2]) as f32,
    ];
    let m = &frame.mvp;
    let w = m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15];
    if !w.is_finite() || w <= 0.0 {
        return None;
    }
    let z = (m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14]) / w;
    if !(0.0..=1.0).contains(&z) {
        return None;
    }
    let x = (m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12]) / w;
    let y = (m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13]) / w;
    let raster_scale = match world_height {
        Some(height) => {
            let projection = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt();
            height as f32 * projection * frame.framebuffer[1] as f32 / (2.0 * w * label.font_size)
        }
        None => scale,
    };
    if !raster_scale.is_finite() || raster_scale <= 0.0 {
        return None;
    }
    Some(PlacedText {
        left: (x + 1.0) * 0.5 * frame.framebuffer[0] as f32 + offset[0] * scale,
        top: (1.0 - y) * 0.5 * frame.framebuffer[1] as f32 + offset[1] * scale,
        scale: raster_scale,
        depth: match label.placement {
            TextPlacement::Nameplate { .. } => None,
            _ => Some(z),
        },
    })
}

/// Center the shaped line box and its padding on the projected source anchor. Both glyphs
/// and black background obey the same CSS clip; a nameplate is an overlay annotation.
fn text_rectangle(
    run: &TextRun,
    placed: &mut PlacedText,
    frame: &TextFrame,
    scale: f32,
) -> Option<plate::Rectangle> {
    let (padding, rounded, centered) = match run.label.placement {
        TextPlacement::Nameplate {
            padding, rounded, ..
        } => (padding, rounded, true),
        _ if run.label.object.is_some() => ([4.0, 4.0], false, false),
        _ => return None,
    };
    if run.label.text.is_empty() {
        return None;
    }
    let mut width = 0.0f32;
    let mut top = f32::INFINITY;
    let mut bottom = f32::NEG_INFINITY;
    for line in run.buffer.layout_runs() {
        width = width.max(line.line_w);
        top = top.min(line.line_top);
        bottom = bottom.max(line.line_top + line.line_height);
    }
    if !top.is_finite() || !bottom.is_finite() {
        return None;
    }
    let raster = placed.scale;
    if centered {
        placed.left -= width * raster * 0.5;
        placed.top -= (top + bottom) * raster * 0.5;
    }
    let bounds = clip_bounds(&run.label, frame, scale);
    let bounds = [
        bounds.left as f32,
        bounds.top as f32,
        bounds.right as f32,
        bounds.bottom as f32,
    ];
    Some(plate::Rectangle {
        bounds: [
            placed.left - padding[0] * raster,
            placed.top + (top - padding[1]) * raster,
            placed.left + (width + padding[0]) * raster,
            placed.top + (bottom + padding[1]) * raster,
        ],
        clip: bounds,
        rounded,
        depth: placed.depth,
        object: run.label.object,
        border: 2.0 * scale,
    })
}

/// Convert clip edges once, conservatively enclosing fractional CSS rectangles.
fn clip_bounds(label: &TextLabel, frame: &TextFrame, scale: f32) -> TextBounds {
    match label.clip {
        Some(c) => TextBounds {
            left: (c[0] * scale).floor() as i32,
            top: (c[1] * scale).floor() as i32,
            right: (c[2] * scale).ceil() as i32,
            bottom: (c[3] * scale).ceil() as i32,
        },
        None => TextBounds {
            left: 0,
            top: 0,
            right: frame.framebuffer[0] as i32,
            bottom: frame.framebuffer[1] as i32,
        },
    }
}

/// The pinned library owns all shader descriptors; the pass retains reversed-Z depth.
fn renderer(
    ctx: &GpuCtx,
    atlas: &mut TextAtlas,
    target: Target,
    compare: wgpu::CompareFunction,
) -> TextRenderer {
    TextRenderer::new(
        atlas,
        &ctx.device,
        wgpu::MultisampleState {
            count: target.samples,
            ..Default::default()
        },
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false),
            depth_compare: Some(compare),
            stencil: Default::default(),
            bias: Default::default(),
        }),
    )
}

/// Match color encoding to the configured render target; mask coverage always remains linear.
fn make_atlas(ctx: &GpuCtx, cache: &Cache, format: wgpu::TextureFormat) -> TextAtlas {
    let mode = if format.is_srgb() {
        ColorMode::Accurate
    } else {
        ColorMode::Web
    };
    TextAtlas::with_color_mode(&ctx.device, &ctx.queue, cache, format, mode)
}

/// Adapter for Glyphon's required metadata-to-depth callback.
fn depth_for(depths: &HashMap<usize, f32>, id: usize) -> f32 {
    depths.get(&id).copied().unwrap_or(0.0)
}

/// Browser monotonic timing; native unit tests do not interpret preparation timing.
fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window()
        && let Some(performance) = window.performance()
    {
        return performance.now();
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build an identity camera with one known logical-to-physical scale.
    fn frame(scale: f64) -> TextFrame {
        TextFrame {
            mvp: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            origin: [0.0; 3],
            framebuffer: [(800.0 * scale) as u32, (600.0 * scale) as u32],
            logical: [800.0, 600.0],
            ortho_half_height: 1.0,
        }
    }

    #[test]
    fn logical_to_physical_scale_is_applied_once() {
        for scale in [1.0, 1.25, 1.5, 2.0] {
            let frame = frame(scale);
            assert_eq!(frame.scale().unwrap(), scale as f32);
            let label = TextLabel {
                object: None,
                id: 1,
                text: "A".into(),
                font_size: 16.0,
                line_height: 24.0,
                color: [255; 4],
                placement: TextPlacement::Screen {
                    left: 12.25,
                    top: 8.5,
                },
                clip: None,
            };
            let placed = place(&label, &frame, frame.scale().unwrap()).unwrap();
            assert_eq!(placed.left, 12.25 * scale as f32);
            assert_eq!(placed.scale * label.font_size, 16.0 * scale as f32);
        }
    }

    #[test]
    fn scene_anchor_preserves_rebased_depth_and_culls_near_plane() {
        let mut frame = frame(2.0);
        frame.origin = [1_000_000.0, 0.0, 0.0];
        let mut label = TextLabel {
            object: None,
            id: 1,
            text: "A".into(),
            font_size: 16.0,
            line_height: 24.0,
            color: [255; 4],
            placement: TextPlacement::Anchor {
                world: [1_000_000.0, 0.0, 0.25],
                offset: [0.0; 2],
            },
            clip: None,
        };
        let placed = place(&label, &frame, 2.0).unwrap();
        assert_eq!((placed.left, placed.top), (800.0, 600.0));
        assert_eq!(placed.depth, Some(0.25));
        label.placement = TextPlacement::Anchor {
            world: [1_000_000.0, 0.0, 1.01],
            offset: [0.0; 2],
        };
        assert!(place(&label, &frame, 2.0).is_none());
    }
    #[test]
    fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system() {
        let mut document = TextDocument::new();
        let mut label = TextLabel {
            object: None,
            id: 1,
            text: "Sphere Ø25".into(),
            font_size: 18.0,
            line_height: 26.0,
            color: [255; 4],
            placement: TextPlacement::Nameplate {
                world: [0.0, 0.0, 0.25],
                padding: [6.0, 4.0],
                rounded: false,
            },
            clip: None,
        };
        document.set_labels(vec![label.clone()]).unwrap();
        for scale in [1.0, 1.25, 2.0] {
            let frame = frame(scale);
            let run = &document.runs[0];
            let mut placed = place(&run.label, &frame, scale as f32).unwrap();
            let rectangle = text_rectangle(run, &mut placed, &frame, scale as f32)
                .unwrap()
                .bounds;
            assert_eq!(
                placed.depth, None,
                "a source-center annotation overlays its own solid"
            );
            assert!(((rectangle[0] + rectangle[2]) * 0.5 - 400.0 * scale as f32).abs() < 0.001);
            assert!(((rectangle[1] + rectangle[3]) * 0.5 - 300.0 * scale as f32).abs() < 0.001);
            assert!((rectangle[3] - rectangle[1] - 34.0 * scale as f32).abs() < 0.001);
        }
        label.clip = Some([390.0, 290.0, 410.0, 310.0]);
        label.color = [255, 255, 0, 255];
        document.set_labels(vec![label.clone()]).unwrap();
        assert_eq!(
            document.shape_count, 1,
            "annotation style and clip reuse shaping"
        );
        let frame = frame(2.0);
        let run = &document.runs[0];
        let mut placed = place(&run.label, &frame, 2.0).unwrap();
        assert_eq!(
            text_rectangle(run, &mut placed, &frame, 2.0).unwrap().clip,
            [780.0, 580.0, 820.0, 620.0]
        );
        label.placement = TextPlacement::Nameplate {
            world: [0.0, 0.0, 0.25],
            padding: [f32::NAN, 4.0],
            rounded: false,
        };
        assert!(document.set_labels(vec![label]).is_err());
        assert_eq!(document.shape_count, 1);
        assert_eq!(
            document.runs.len(),
            1,
            "invalid padding preserves the previous document"
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    #[ignore = "requires a native GPU adapter"]
    fn nameplate_has_black_background_white_ink_centering_and_clean_release() {
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
        use session_rust::{RenderVertex, Xform};
        let mut gpu = pollster::block_on(Gpu::new_headless(320, 160)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.lit = false;
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity().m, 0));
        for position in [
            [-1.0, -1.0, 0.7],
            [1.0, -1.0, 0.7],
            [1.0, 1.0, 0.7],
            [-1.0, 1.0, 0.7],
        ] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [0.0, 0.0, 1.0],
                color: [0.3, 0.5, 0.7, 1.0],
            });
            upload.arena.vids.push(0);
        }
        upload.arena.idx = vec![0, 1, 2, 0, 2, 3];
        gpu.set_scene(&upload);
        let mut input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let baseline = gpu.render_offscreen(&input);
        let label = TextLabel {
            object: None,
            id: 1,
            text: "Sphere Ø25".into(),
            font_size: 13.5,
            line_height: 19.5,
            color: [255; 4],
            placement: TextPlacement::Nameplate {
                world: [0.0, 0.0, 0.2],
                padding: [12.75, 3.0],
                rounded: true,
            },
            clip: None,
        };
        gpu.text.set_labels(vec![label]).unwrap();
        let pixels = gpu.render_offscreen(&input);
        assert!(
            white_pixels(&pixels) > 40,
            "normal-sized glyphs remain visible over their solid's center"
        );
        let mut left = 320;
        let mut right = 0;
        let mut top = 160;
        let mut bottom = 0;
        let mut black = 0;
        for (index, pixel) in pixels.chunks_exact(4).enumerate() {
            if pixel[0] < 5 && pixel[1] < 5 && pixel[2] < 5 {
                let x = index % 320;
                let y = index / 320;
                left = left.min(x);
                right = right.max(x);
                top = top.min(y);
                bottom = bottom.max(y);
                black += 1;
            }
        }
        assert!(
            black > 600,
            "the nameplate has a real opaque black GPU background"
        );
        assert!(((left + right) as f32 * 0.5 - 160.0).abs() <= 1.0);
        assert!(((top + bottom) as f32 * 0.5 - 80.0).abs() <= 1.0);
        let mut line_width = 0.0f32;
        for line in gpu.text.document.runs[0].buffer.layout_runs() {
            line_width = line_width.max(line.line_w);
        }
        assert!(
            (right - left + 1) as f32 - 25.5 >= line_width - 2.0,
            "the complete shaped line fits in the straight section between both caps"
        );
        let corner = (top * 320 + left) * 4;
        assert_eq!(
            &pixels[corner..corner + 4],
            &baseline[corner..corner + 4],
            "the maximum-radius corner exposes the original solid"
        );
        input.view_proj.m[12] = 0.25;
        let moved = gpu.render_offscreen(&input);
        assert_ne!(
            pixels, moved,
            "the nameplate follows its projected source center"
        );
        assert_eq!(
            gpu.text.stats.shape_count, 1,
            "camera movement does not reshape"
        );
        gpu.text.release(&gpu.ctx);
        input.view_proj = Xform::identity();
        assert_eq!(
            baseline,
            gpu.render_offscreen(&input),
            "clearing selection releases plate and text together"
        );
        assert_eq!(gpu.text.stats.nameplate_capacity_bytes, 40);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    #[ignore = "requires a native GPU adapter"]
    fn actual_glyph_coverage_obeys_depth_clip_motion_and_release() {
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
        use session_rust::{RenderVertex, Xform};
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 128)).unwrap();
        gpu.view.show_grid = false;
        gpu.view.lit = false;
        let mut upload = Upload::default();
        upload
            .obj
            .rows
            .push(ObjectRow::new(Xform::identity().to_f32().map(f64::from), 0));
        for position in [
            [-1.0, -1.0, 0.7],
            [1.0, -1.0, 0.7],
            [1.0, 1.0, 0.7],
            [-1.0, 1.0, 0.7],
        ] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [0.0, 0.0, 1.0],
                color: [0.1, 0.1, 0.1, 1.0],
            });
            upload.arena.vids.push(0);
        }
        upload.arena.idx = vec![0, 1, 2, 0, 2, 3];
        gpu.set_scene(&upload);
        let mut frame = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::BLACK,
            now_ms: 0.0,
        };
        let baseline = gpu.render_offscreen(&frame);
        let mut label = TextLabel {
            object: None,
            id: 1,
            text: "AV office Ø25".into(),
            font_size: 14.0,
            line_height: 21.0,
            color: [255; 4],
            placement: TextPlacement::Anchor {
                world: [-0.8, 0.8, 0.5],
                offset: [0.0; 2],
            },
            clip: None,
        };
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        assert_eq!(
            baseline,
            gpu.render_offscreen(&frame),
            "a label behind a solid must be fully occluded"
        );
        label.placement = TextPlacement::Anchor {
            world: [-0.8, 0.8, 0.9],
            offset: [0.0; 2],
        };
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        let front = gpu.render_offscreen(&frame);
        assert!(
            white_pixels(&front) > 50,
            "normal-size foreground glyphs must render actual opaque interiors"
        );
        assert_eq!(gpu.text.stats.shape_count, 1);
        let mut translated = Xform::identity();
        translated.m[12] = 0.125;
        frame.view_proj = translated;
        let moved = gpu.render_offscreen(&frame);
        assert_ne!(front, moved, "camera motion must refresh glyph placement");
        assert_eq!(
            gpu.text.stats.shape_count, 1,
            "camera motion must reuse shaping"
        );
        label.placement = TextPlacement::WorldBillboard {
            world: [-0.5, 0.4, 0.9],
            world_height: 0.25,
        };
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        assert!(white_pixels(&gpu.render_offscreen(&frame)) > 50);
        label.placement = TextPlacement::Screen {
            left: 8.25,
            top: 8.5,
        };
        label.clip = Some([0.0, 0.0, 24.0, 128.0]);
        gpu.text.set_labels(vec![label]).unwrap();
        let clipped = gpu.render_offscreen(&frame);
        assert!(white_pixels(&clipped) > 10);
        for (index, pixel) in clipped.chunks_exact(4).enumerate() {
            if index % 256 >= 24 {
                assert!(
                    pixel[0] < 240,
                    "clip must exclude glyph pixels outside its CSS rectangle"
                );
            }
        }
        gpu.text.release(&gpu.ctx);
        assert_eq!(
            baseline,
            gpu.render_offscreen(&FrameInput {
                view_proj: Xform::identity(),
                clear: wgpu::Color::BLACK,
                now_ms: 0.0
            })
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    #[ignore = "requires a native GPU adapter"]
    fn continuous_world_scale_evicts_without_reshaping_or_stale_instances() {
        use crate::engine::gpu::{FrameInput, Gpu};
        use session_rust::Xform;
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 128)).unwrap();
        gpu.view.show_grid = false;
        let frame = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::BLACK,
            now_ms: 0.0,
        };
        let mut label = TextLabel {
            object: None,
            id: 1,
            text: (33u8..127).map(char::from).collect(),
            font_size: 14.0,
            line_height: 21.0,
            color: [255; 4],
            placement: TextPlacement::WorldBillboard {
                world: [-0.9, 0.5, 0.5],
                world_height: 0.2,
            },
            clip: None,
        };
        for step in 0..60 {
            label.placement = TextPlacement::WorldBillboard {
                world: [-0.9, 0.5, 0.5],
                world_height: 0.2 + f64::from(step) * 0.001,
            };
            gpu.text.set_labels(vec![label.clone()]).unwrap();
            assert!(
                white_pixels(&gpu.render_offscreen(&frame)) > 20,
                "every zoom frame must contain opaque glyph interiors"
            );
            assert_eq!(
                gpu.text.stats.shape_count, 1,
                "world zoom must not reshape unchanged text"
            );
            assert!(
                gpu.text.stats.distinct_raster_keys <= 4096 + 94,
                "cache growth is limited to the budget plus one preparation"
            );
        }
        assert!(
            gpu.text.stats.atlas_resets > 0,
            "this continuous-scale fixture must actually cross the eviction threshold"
        );
        assert_eq!(gpu.text.stats.missing_glyphs, 0);
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Count opaque interiors without treating anti-aliased edges as full glyph coverage.
    fn white_pixels(pixels: &[u8]) -> usize {
        let mut count = 0;
        for pixel in pixels.chunks_exact(4) {
            count += usize::from(pixel[0] > 240 && pixel[1] > 240 && pixel[2] > 240);
        }
        count
    }
}
