use crate::text_gpu_support::GpuCtx;
use crate::text::{TextDocument, TextLabel, TextPlacement, TextRun};
use crate::text_gpu_support::Target;
use glyphon::{
    Cache, Color, ColorMode, Resolution, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer,
    Viewport,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct TextFrame {
    pub mvp: [f32; 16],
    pub origin: [f64; 3],
    pub framebuffer: [u32; 2],
    pub logical: [f64; 2],
    pub ortho_half_height: f32,
    pub clip: [[f64; 4]; 6],
}


impl TextFrame {
    fn canvas(&self) -> TextFrame {
        TextFrame {
            mvp: [0.0; 16],
            origin: [0.0; 3],
            ortho_half_height: 0.0,
            clip: [[0.0; 4]; 6],
            ..self.clone()
        }
    }

    pub fn cut(&self, p: [f64; 3]) -> bool {
        self.clip
            .iter()
            .any(|k| k[0] * p[0] + k[1] * p[1] + k[2] * p[2] + k[3] < 0.0)
    }

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

struct PlacedText {
    left: f32,
    top: f32,
    scale: f32,
    depth: Option<f32>,
}

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

    if frame.cut(world) {
        return None;
    }

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

#[path = "text_plate.rs"]
mod plate;

#[derive(Clone, Debug, Default, Serialize)]
pub struct TextStats {
    pub preparations: u64,
    pub skipped_preparations: u64,
    pub shape_count: u64,
    pub shaping_ms: f64,
    pub requested_glyphs: usize,
    pub distinct_raster_keys: usize,
    pub new_raster_keys: usize,
    pub raster_images: usize,
    pub new_raster_images: usize,
    pub raster_image_capacity_bytes: usize,
    pub atlas_resets: u64,
    pub preparation_ms: f64,
    pub active_instance_bytes: usize,
    pub missing_glyphs: usize,
    pub nameplate_capacity_bytes: u64,
    pub world_plane_buffer_bytes: u64,
    pub world_plane_texture_bytes: u64,
    pub world_plane_rasterizations: u64,
}

fn text_rectangle(
    run: &TextRun,
    placed: &mut PlacedText,
    frame: &TextFrame,
    scale: f32,
) -> Option<plate::Rectangle> {
    let (mut padding, rounded, centered) = match run.label.placement {
        TextPlacement::Nameplate {
            padding, rounded, ..
        } => (padding, rounded, true),
        _ if run.label.object.is_some() => ([0.0, run.label.font_size * 2.0 / 9.0], true, false),
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

    if rounded {
        padding[0] = padding[0].max((bottom - top) * 0.5 + padding[1]);
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
    })
}

#[path = "text_plane.rs"]
mod plane;

struct PreparedText<'a> {
    overlays: Vec<TextArea<'a>>,
    anchors: Vec<TextArea<'a>>,
    depths: HashMap<usize, f32>,
    plates: Vec<plate::Rectangle>,
    overlay: bool,
    anchored: bool,
}

fn areas<'a>(
    runs: &'a [TextRun],
    frame: &TextFrame,
    scale: f32,
    stats: &mut TextStats,
    raster_keys: &mut HashSet<glyphon::CacheKey>,
) -> PreparedText<'a> {
    let mut overlay = false;
    let mut anchored = false;
    let mut overlays = Vec::new();
    let mut anchors = Vec::new();
    let mut depths = HashMap::new();
    let mut plates = Vec::new();
    stats.new_raster_keys = 0;
    stats.requested_glyphs = 0;
    stats.missing_glyphs = 0;

    for run in runs {
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
                run.label.ink_color()[0],
                run.label.ink_color()[1],
                run.label.ink_color()[2],
                run.label.ink_color()[3],
            ),
            custom_glyphs: &[],
        };

        for line in run.buffer.layout_runs() {
            for glyph in line.glyphs {
                stats.requested_glyphs += 1;
                stats.missing_glyphs += usize::from(glyph.glyph_id == 0);
                let physical = glyph.physical((placed.left, placed.top), placed.scale);
                stats.new_raster_keys += usize::from(raster_keys.insert(physical.cache_key));
            }
        }

        if !run.label.text.is_empty() {
            match run.label.placement {
                TextPlacement::Screen { .. } | TextPlacement::Nameplate { .. } => overlay = true,
                _ => anchored = true,
            }
        }

        if let Some(depth) = placed.depth {
            depths.insert(run.label.id as usize, depth);
            anchors.push(area);
        } else {
            overlays.push(area);
        }
    }
    PreparedText {
        overlays,
        anchors,
        depths,
        plates,
        overlay,
        anchored,
    }
}

pub struct Lane {
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

fn make_atlas(ctx: &GpuCtx, cache: &Cache, format: wgpu::TextureFormat) -> TextAtlas {
    let mode = if format.is_srgb() {
        ColorMode::Accurate
    } else {
        ColorMode::Web
    };
    TextAtlas::with_color_mode(&ctx.device, &ctx.queue, cache, format, mode)
}

fn depth_for(depths: &HashMap<usize, f32>, id: usize) -> f32 {
    depths.get(&id).copied().unwrap_or(0.0)
}

impl Lane {

    pub fn new(ctx: &GpuCtx, target: Target) -> Self {
        let cache = Cache::new(&ctx.device);
        let viewport = Viewport::new(&ctx.device, &cache);
        let mut atlas = make_atlas(ctx, &cache, target.format);
        let overlay = renderer(ctx, &mut atlas, target, wgpu::CompareFunction::Always);
        let anchored = renderer(ctx, &mut atlas, target, crate::depth::CLOSER_EQUAL);
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
            crate::depth::CLOSER_EQUAL,
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

    pub fn allocated_bytes(&self) -> u64 {
        self.plates.allocated_bytes() + self.planes.buffer_bytes()
    }

    pub fn texture_bytes(&self) -> u64 {
        self.planes.texture_bytes()
    }

    pub fn set_labels(&mut self, labels: Vec<TextLabel>) -> anyhow::Result<()> {
        self.document.set_labels(labels)
    }
}

impl Lane {
    fn measure_preparation(&mut self, raster_images_before: usize, start: f64) {
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
        self.stats.preparation_ms = crate::load_measure::now() - start;
    }

    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.plates.retarget(ctx, target);
        self.planes.retarget(ctx, target);
        self.overlay = renderer(ctx, &mut self.atlas, target, wgpu::CompareFunction::Always);
        self.anchored = renderer(
            ctx,
            &mut self.atlas,
            target,
            crate::depth::CLOSER_EQUAL,
        );
        self.prepared = None;
    }

    pub fn reset(&mut self) {
        self.document.clear();
        self.prepared = None;
        self.overlay_count = 0;
        self.anchored_count = 0;
        self.plates.reset();
        self.planes.reset();
    }

    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.plates.release(ctx);
        self.planes.release(ctx);
        self.stats.nameplate_capacity_bytes = self.plates.allocated_bytes();
        self.stats.world_plane_buffer_bytes = self.planes.buffer_bytes();
        self.stats.world_plane_texture_bytes = 0;
        self.rebuild_resources(ctx);
    }
}

impl Lane {

    pub fn prepare(&mut self, ctx: &GpuCtx, frame: &TextFrame) -> anyhow::Result<()> {
        let fixed = self
            .document
            .runs
            .iter()
            .all(|run| matches!(run.label.placement, TextPlacement::Screen { .. }));
        let key = (
            self.document.revision,
            self.document.font_revision,
            if fixed { frame.canvas() } else { frame.clone() },
        );

        if self.prepared.as_ref() == Some(&key) {
            self.stats.skipped_preparations += 1;
            return Ok(());
        }

        self.overlay_count = 0;
        self.anchored_count = 0;
        self.plates.reset();
        let scale = frame.scale()?;
        let start = crate::load_measure::now();
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
        let prepared = areas(
            &self.document.runs,
            frame,
            scale,
            &mut self.stats,
            &mut self.raster_keys,
        );
        self.overlay_count = u32::from(prepared.overlay);
        self.anchored_count = u32::from(prepared.anchored);

        self.overlay.prepare(
            &ctx.device,
            &ctx.queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            prepared.overlays,
            &mut self.raster,
        )?;
        self.anchored.prepare_with_depth(
            &ctx.device,
            &ctx.queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            prepared.anchors,
            &mut self.raster,
            |id| depth_for(&prepared.depths, id),
        )?;
        self.plates
            .prepare(ctx, &prepared.plates, frame.framebuffer);
        self.measure_preparation(raster_images_before, start);
        self.prepared = Some(key);
        Ok(())
    }
}

impl Lane {

    pub fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        self.planes.draw_ids(pass, pick_transform) + self.plates.draw_ids(pass, pick_transform)
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<u32> {
        let mut draws = self.planes.draw(pass);
        draws += self.plates.draw(pass, false);

        if self.anchored_count != 0 {
            self.anchored.render(&self.atlas, &self.viewport, pass)?;
            draws += 1;
        }

        draws += self.plates.draw(pass, true);

        if self.overlay_count != 0 {
            self.overlay.render(&self.atlas, &self.viewport, pass)?;
            draws += 1;
        }

        Ok(draws)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn frame(scale: f64) -> TextFrame {
        TextFrame {
            mvp: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            origin: [0.0; 3],
            framebuffer: [(800.0 * scale) as u32, (600.0 * scale) as u32],
            logical: [800.0, 600.0],
            ortho_half_height: 1.0,
            clip: [[0.0; 4]; 6],
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
}
