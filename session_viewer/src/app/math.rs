use crate::app::manifest::TextItem;
use crate::app::scene::FileDoc;
use crate::engine::text::{
    FALLBACK_BYTES, FONT_BYTES, MATH_FONT_BYTES, MATH_SERIF_ID, STIX_ITALIC_BYTES,
    STIX_REGULAR_BYTES, SYMBOL_BYTES, TextDocument, TextLabel, TextPlacement,
};
use session_rust::{Color, Line, Session, Xform};
use std::cell::RefCell;
use std::rc::Rc;

/// Fixed-size spatial labels use CSS pixels as their layout unit.
pub const SPATIAL_FONT_SCALE: f64 = 1.0;

/// A TeX expression expanded into world-plane glyph labels and vector rules.
pub struct Output {
    pub texts: Vec<TextItem>,
    pub documents: Vec<FileDoc>,
}

/// Parse and lay out TeX items entirely inside the Rust/WGSL viewer.
pub fn typeset(items: &[TextItem]) -> Output {
    let mut out = Output {
        texts: Vec::new(),
        documents: Vec::new(),
    };
    let texture_baseline = texture_baseline();
    for item in items {
        if !item.math {
            out.texts.push(item.clone());
            continue;
        }
        let block = cached_layout(&item.text);
        let scale = item.height;
        let ink_center = if item.center_ink {
            block_ink_center(&block)
        } else {
            None
        };
        let name = if item.name.is_empty() {
            item.text.clone()
        } else {
            item.name.clone()
        };
        let color = item.color.unwrap_or([0, 0, 0, 255]);
        let mut session = Session::new(&format!("math/{name}"));
        for run in &block.runs {
            let color = run.color.unwrap_or(color);
            let (x, y) =
                place_with_ink_center(item, &block, run.x, run.baseline, scale, ink_center);
            let baseline = if item.center_ink {
                glyph_metrics(&run.text)
                    .map_or(texture_baseline, |metrics| metrics.texture_baseline)
            } else {
                texture_baseline
            };
            let local = (x, y + baseline * run.size * scale);
            let (at, offset) = if item.fixed && item.spatial && item.camera_facing {
                let scale = SPATIAL_FONT_SCALE;
                (
                    item.at,
                    [
                        ((local.0 - item.at[0]) * scale) as f32,
                        (-(local.1 - item.at[1]) * scale) as f32,
                    ],
                )
            } else {
                (oriented_position(item, local), [0.0; 2])
            };
            out.texts.push(TextItem {
                text: run.text.clone(),
                math: false,
                serif: true,
                center: false,
                center_ink: false,
                fixed: item.fixed,
                max_width: 0.0,
                name: name.clone(),
                outline: item.outline,
                at,
                offset,
                right: item.right,
                up: item.up,
                height: run.size * scale,
                camera_facing: item.camera_facing,
                spatial: item.spatial,
                plate: false,
                color: Some(color),
            });
        }
        for rule in &block.lines {
            let (x0, y0) =
                place_with_ink_center(item, &block, rule.from[0], rule.from[1], scale, ink_center);
            let (x1, y1) =
                place_with_ink_center(item, &block, rule.to[0], rule.to[1], scale, ink_center);
            let from = oriented_position(item, (x0, y0));
            let to = oriented_position(item, (x1, y1));
            let mut line = Line::new(from[0], from[1], from[2], to[0], to[1], to[2]);
            line.name = name.clone();
            line.width = (rule.width * scale).max(0.25);
            line.linecolor = Color::new(
                f32::from(color[0]) / 255.0,
                f32::from(color[1]) / 255.0,
                f32::from(color[2]) / 255.0,
                f32::from(color[3]) / 255.0,
            );
            session.add_line(line, None);
        }
        if !session.lookup.is_empty() {
            out.documents.push(FileDoc {
                name: format!("math/{name}"),
                place: Xform::identity(),
                session: Rc::new(session),
                point_px: 0.0,
                display_only: false,
            });
        }
    }
    out
}

/// Place a local baseline or rule point at the expression's top-left or center.
#[cfg(test)]
fn place(item: &TextItem, block: &Block, x: f64, baseline: f64, scale: f64) -> (f64, f64) {
    place_with_ink_center(item, block, x, baseline, scale, None)
}

fn place_with_ink_center(
    item: &TextItem,
    block: &Block,
    x: f64,
    baseline: f64,
    scale: f64,
    ink_center: Option<(f64, f64)>,
) -> (f64, f64) {
    if item.center {
        let (horizontal, vertical) = ink_center.map_or(
            (
                block.width * 0.5,
                (block.ascent - block.depth) * 0.5 + baseline,
            ),
            |(horizontal, vertical)| (horizontal, baseline - vertical),
        );
        (
            item.at[0] + (x - horizontal) * scale,
            item.at[1] + vertical * scale,
        )
    } else {
        (
            item.at[0] + x * scale,
            item.at[1] + (block.ascent + baseline) * scale,
        )
    }
}

/// Lay out a whole billboard expression on the camera plane, not along world X and Y.
fn oriented_position(item: &TextItem, at: (f64, f64)) -> [f64; 3] {
    let dx = at.0 - item.at[0];
    let dy = at.1 - item.at[1];
    if item.camera_facing {
        std::array::from_fn(|axis| item.at[axis] + item.right[axis] * dx + item.up[axis] * dy)
    } else {
        [at.0, at.1, item.at[2]]
    }
}

#[derive(Clone, Copy)]
struct GlyphMetrics {
    bounds: [f64; 4],
    texture_baseline: f64,
}

/// Actual glyph and rule bounds in em, relative to a block's baseline: left, bottom, right, top.
fn block_ink_center(block: &Block) -> Option<(f64, f64)> {
    let mut bounds = [
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    let mut include = |left: f64, bottom: f64, right: f64, top: f64| {
        bounds[0] = bounds[0].min(left);
        bounds[1] = bounds[1].min(bottom);
        bounds[2] = bounds[2].max(right);
        bounds[3] = bounds[3].max(top);
    };
    for run in &block.runs {
        if let Some(metrics) = glyph_metrics(&run.text) {
            let [left, bottom, right, top] = metrics.bounds;
            include(
                run.x + left * run.size,
                run.baseline + bottom * run.size,
                run.x + right * run.size,
                run.baseline + top * run.size,
            );
        }
    }
    for rule in &block.lines {
        include(
            rule.from[0].min(rule.to[0]) - rule.width * 0.5,
            rule.from[1].min(rule.to[1]) - rule.width * 0.5,
            rule.from[0].max(rule.to[0]) + rule.width * 0.5,
            rule.from[1].max(rule.to[1]) + rule.width * 0.5,
        );
    }
    (bounds.iter().all(|value| value.is_finite()))
        .then_some(((bounds[0] + bounds[2]) * 0.5, (bounds[1] + bounds[3]) * 0.5))
}

fn glyph_metrics(text: &str) -> Option<GlyphMetrics> {
    if let Some(metrics) = GLYPH_METRICS.with_borrow(|cache| cache.get(text).copied()) {
        return metrics;
    }
    let measured = MEASURER.with_borrow_mut(|slot| {
        let document = slot.get_or_insert_with(TextDocument::new);
        let label = TextLabel {
            id: MATH_SERIF_ID,
            object: None,
            text: text.to_string(),
            font_size: 18.0,
            line_height: 26.0,
            color: [0, 0, 0, 255],
            outline: 0.0,
            placement: TextPlacement::Screen {
                left: 0.0,
                top: 0.0,
            },
            clip: None,
        };
        document.set_labels(vec![label]).ok()?;
        let texture_baseline = document.runs[0]
            .buffer
            .layout_runs()
            .next()
            .map_or(1.0, |line| f64::from(line.line_y) / 18.0);
        let mut raster = glyphon::SwashCache::new();
        let scale = 256.0 / 18.0;
        let mut bounds = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for line in document.runs[0].buffer.layout_runs() {
            for glyph in line.glyphs {
                let physical = glyph.physical((0.0, 0.0), scale);
                let Some(image) = raster.get_image(&mut document.fonts, physical.cache_key) else {
                    continue;
                };
                let left = physical.x + image.placement.left;
                let top = physical.y - image.placement.top;
                let right = left + image.placement.width as i32;
                let bottom = top + image.placement.height as i32;
                bounds[0] = bounds[0].min(f64::from(left) / 256.0);
                bounds[1] = bounds[1].min(-f64::from(bottom) / 256.0);
                bounds[2] = bounds[2].max(f64::from(right) / 256.0);
                bounds[3] = bounds[3].max(-f64::from(top) / 256.0);
            }
        }
        bounds
            .iter()
            .all(|value| value.is_finite())
            .then_some(GlyphMetrics {
                bounds,
                texture_baseline,
            })
    });
    GLYPH_METRICS.with_borrow_mut(|cache| {
        if cache.len() >= 4096 {
            cache.clear();
        }
        cache.insert(text.to_owned(), measured);
    });
    measured
}

#[derive(Clone, Copy, Debug)]
struct LabelBox {
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
}

const LABEL_MARGIN: f64 = 5.0;
const SLIDE_PADDING: f64 = 10.0;

/// Actual typeset ink bounds in em, including scripts and vector fraction rules.
fn math_bounds(item: &TextItem) -> Option<[f64; 4]> {
    INK_BOUNDS.with_borrow_mut(|cache| {
        let key = (item.text.clone(), item.center, item.center_ink);
        if let Some(bounds) = cache.get(&key) {
            return *bounds;
        }
        let mut local = item.clone();
        local.at = [0.0; 3];
        local.height = 1.0;
        local.camera_facing = false;
        local.spatial = false;
        let output = typeset(&[local]);
        let mut bounds = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for run in output.texts {
            let Some(metrics) = glyph_metrics(&run.text) else {
                continue;
            };
            bounds[0] = bounds[0].min(run.at[0] + metrics.bounds[0] * run.height);
            bounds[1] = bounds[1]
                .min(run.at[1] + (metrics.bounds[1] - metrics.texture_baseline) * run.height);
            bounds[2] = bounds[2].max(run.at[0] + metrics.bounds[2] * run.height);
            bounds[3] = bounds[3]
                .max(run.at[1] + (metrics.bounds[3] - metrics.texture_baseline) * run.height);
        }
        for line in output
            .documents
            .iter()
            .flat_map(|doc| &doc.session.objects.lines)
        {
            for point in [line.start(), line.end()] {
                let half = f64::from(line.width) * 0.5;
                bounds[0] = bounds[0].min(point[0]);
                bounds[1] = bounds[1].min(point[1] - half);
                bounds[2] = bounds[2].max(point[0]);
                bounds[3] = bounds[3].max(point[1] + half);
            }
        }
        let bounds = bounds
            .iter()
            .all(|value| value.is_finite())
            .then_some(bounds);
        cache.insert(key, bounds);
        bounds
    })
}

fn label_box(item: &TextItem) -> LabelBox {
    if item.math
        && let Some(bounds) = math_bounds(item)
    {
        return LabelBox {
            left: item.at[0] + bounds[0] * item.height,
            bottom: item.at[1] + bounds[1] * item.height,
            right: item.at[0] + bounds[2] * item.height,
            top: item.at[1] + bounds[3] * item.height,
        };
    }
    let (width, ascent, depth) = if item.math {
        let block = cached_layout(&item.text);
        (
            block.width * item.height,
            block.ascent * item.height,
            block.depth * item.height,
        )
    } else {
        (
            measure(&item.text, item.height),
            item.height * 0.82,
            item.height * 0.18,
        )
    };
    if item.center {
        LabelBox {
            left: item.at[0] - width * 0.5,
            right: item.at[0] + width * 0.5,
            bottom: item.at[1] - (ascent + depth) * 0.5,
            top: item.at[1] + (ascent + depth) * 0.5,
        }
    } else {
        LabelBox {
            left: item.at[0],
            right: item.at[0] + width,
            bottom: item.at[1] - depth,
            top: item.at[1] + ascent,
        }
    }
}

fn overlap_area(a: LabelBox, b: LabelBox) -> f64 {
    let x = (a.right.min(b.right) - a.left.max(b.left) + 2.0 * LABEL_MARGIN).max(0.0);
    let y = (a.top.min(b.top) - a.bottom.max(b.bottom) + 2.0 * LABEL_MARGIN).max(0.0);
    x * y
}

fn clamp_label(item: &TextItem, mut at: [f64; 2], width: f64, height: f64) -> [f64; 2] {
    let mut candidate = item.clone();
    candidate.at[0] = at[0];
    candidate.at[1] = at[1];
    let bounds = label_box(&candidate);
    at[0] += if bounds.left < SLIDE_PADDING {
        SLIDE_PADDING - bounds.left
    } else if bounds.right > width - SLIDE_PADDING {
        width - SLIDE_PADDING - bounds.right
    } else {
        0.0
    };
    at[1] += if bounds.bottom < SLIDE_PADDING {
        SLIDE_PADDING - bounds.bottom
    } else if bounds.top > height - SLIDE_PADDING {
        height - SLIDE_PADDING - bounds.top
    } else {
        0.0
    };
    at
}

fn placement_score(
    item: &TextItem,
    at: [f64; 2],
    original: [f64; 2],
    placed: &[LabelBox],
    width: f64,
    height: f64,
) -> f64 {
    let mut candidate = item.clone();
    candidate.at[0] = at[0];
    candidate.at[1] = at[1];
    let bounds = label_box(&candidate);
    let overlap = placed
        .iter()
        .map(|other| overlap_area(bounds, *other))
        .sum::<f64>();
    let outside = (SLIDE_PADDING - bounds.left).max(0.0)
        + (bounds.right - width + SLIDE_PADDING).max(0.0)
        + (SLIDE_PADDING - bounds.bottom).max(0.0)
        + (bounds.top - height + SLIDE_PADDING).max(0.0);
    let distance = (at[0] - original[0]).powi(2) + (at[1] - original[1]).powi(2);
    overlap * 1.0e12 + outside * 1.0e9 + distance
}

/// Place labels at their authored size, minimizing overlap first and displacement second.
/// Returns the number of overlapping pairs left after deterministic anchored-first placement.
pub fn optimize_layout(items: &mut [TextItem], width: f64, height: f64) -> usize {
    let originals: Vec<[f64; 2]> = items.iter().map(|item| [item.at[0], item.at[1]]).collect();
    let initial: Vec<LabelBox> = items.iter().map(label_box).collect();
    let mut movable: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| !item.fixed)
        .map(|(index, _)| index)
        .collect();
    movable.sort_by(|a, b| {
        let conflicts = |index: usize| {
            initial
                .iter()
                .enumerate()
                .filter(|(other, bounds)| {
                    *other != index && overlap_area(initial[index], **bounds) > 0.0
                })
                .count()
        };
        conflicts(*b)
            .cmp(&conflicts(*a))
            .then_with(|| items[*a].name.cmp(&items[*b].name))
    });
    let mut placed: Vec<LabelBox> = items
        .iter()
        .filter(|item| item.fixed)
        .map(label_box)
        .collect();
    const DIRECTIONS: [[f64; 2]; 8] = [
        [1.0, 0.0],
        [-1.0, 0.0],
        [0.0, 1.0],
        [0.0, -1.0],
        [1.0, 1.0],
        [1.0, -1.0],
        [-1.0, 1.0],
        [-1.0, -1.0],
    ];
    for index in movable {
        let item = &items[index];
        let original = originals[index];
        let mut best_at = clamp_label(item, original, width, height);
        let mut best_score = placement_score(item, best_at, original, &placed, width, height);
        for radius in [16.0, 32.0, 64.0, 128.0, 256.0] {
            for direction in DIRECTIONS {
                let at = clamp_label(
                    item,
                    [
                        original[0] + direction[0] * radius,
                        original[1] + direction[1] * radius,
                    ],
                    width,
                    height,
                );
                let score = placement_score(item, at, original, &placed, width, height);
                if score < best_score {
                    best_at = at;
                    best_score = score;
                }
            }
        }
        if best_score >= 1.0e12 {
            let bounds = label_box(item);
            let label_width = bounds.right - bounds.left;
            let label_height = bounds.top - bounds.bottom;
            let mut y = SLIDE_PADDING;
            while y + label_height <= height - SLIDE_PADDING {
                let mut x = SLIDE_PADDING;
                while x + label_width <= width - SLIDE_PADDING {
                    let at = [item.at[0] + x - bounds.left, item.at[1] + y - bounds.bottom];
                    let score = placement_score(item, at, original, &placed, width, height);
                    if score < best_score {
                        best_at = at;
                        best_score = score;
                    }
                    x += 16.0;
                }
                y += 16.0;
            }
        }
        items[index].at[0] = best_at[0];
        items[index].at[1] = best_at[1];
        placed.push(label_box(&items[index]));
    }
    let bounds: Vec<LabelBox> = items.iter().map(label_box).collect();
    let mut overlaps = 0;
    for (index, a) in bounds.iter().enumerate() {
        for (other, b) in bounds.iter().enumerate().skip(index + 1) {
            if overlap_area(*a, *b) > 0.0 {
                overlaps += 1;
                log::warn!(
                    "text overlap: '{}' and '{}'",
                    items[index].name,
                    items[other].name
                );
            }
        }
    }
    overlaps
}

struct SpatialLabel {
    index: usize,
    name: String,
    anchor: [f64; 2],
    at: [f64; 2],
    bounds: LabelBox,
    w: f64,
}

/// Projected geometry that a spatial label must not touch.
pub enum SpatialObstacle {
    Segment([f64; 2], [f64; 2], f64),
    Disc([f64; 2], f64),
}

pub(crate) fn screen_projection(
    projection: &Xform,
    anchor: &session_rust::Point,
    world: [f64; 3],
    size: [f64; 2],
) -> Option<([f64; 2], f64)> {
    let point = [
        world[0] - anchor[0],
        world[1] - anchor[1],
        world[2] - anchor[2],
        1.0,
    ];
    let clip: [f64; 4] = std::array::from_fn(|row| {
        (0..4)
            .map(|column| projection.m[column * 4 + row] * point[column])
            .sum()
    });
    if clip[3] <= 0.0 || !(0.0..=1.0).contains(&(clip[2] / clip[3])) {
        return None;
    }
    Some((
        [
            (clip[0] / clip[3] * 0.5 + 0.5) * size[0],
            (0.5 - clip[1] / clip[3] * 0.5) * size[1],
        ],
        clip[3],
    ))
}

fn shifted_box(bounds: LabelBox, delta: [f64; 2]) -> LabelBox {
    LabelBox {
        left: bounds.left + delta[0],
        right: bounds.right + delta[0],
        bottom: bounds.bottom + delta[1],
        top: bounds.top + delta[1],
    }
}

fn spatial_candidate(
    label: &SpatialLabel,
    at: [f64; 2],
    width: f64,
    height: f64,
) -> ([f64; 2], LabelBox) {
    let mut at = at;
    let bounds = shifted_box(
        label.bounds,
        [at[0] - label.anchor[0], at[1] - label.anchor[1]],
    );
    at[0] += if bounds.left < SLIDE_PADDING {
        SLIDE_PADDING - bounds.left
    } else if bounds.right > width - SLIDE_PADDING {
        width - SLIDE_PADDING - bounds.right
    } else {
        0.0
    };
    at[1] += if bounds.bottom < SLIDE_PADDING {
        SLIDE_PADDING - bounds.bottom
    } else if bounds.top > height - SLIDE_PADDING {
        height - SLIDE_PADDING - bounds.top
    } else {
        0.0
    };
    let bounds = shifted_box(
        label.bounds,
        [at[0] - label.anchor[0], at[1] - label.anchor[1]],
    );
    (at, bounds)
}

fn point_segment_distance(point: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let ab = [b[0] - a[0], b[1] - a[1]];
    let length = ab[0] * ab[0] + ab[1] * ab[1];
    let t = if length > 0.0 {
        (((point[0] - a[0]) * ab[0] + (point[1] - a[1]) * ab[1]) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (point[0] - a[0] - ab[0] * t).hypot(point[1] - a[1] - ab[1] * t)
}

fn point_box_distance(point: [f64; 2], bounds: LabelBox) -> f64 {
    let dx = (bounds.left - point[0])
        .max(0.0)
        .max(point[0] - bounds.right);
    let dy = (bounds.bottom - point[1])
        .max(0.0)
        .max(point[1] - bounds.top);
    dx.hypot(dy)
}

fn segments_cross(a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2]) -> bool {
    let cross = |a: [f64; 2], b: [f64; 2], c: [f64; 2]| {
        (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    };
    let ab_c = cross(a, b, c);
    let ab_d = cross(a, b, d);
    let cd_a = cross(c, d, a);
    let cd_b = cross(c, d, b);
    (ab_c == 0.0 && point_segment_distance(c, a, b) == 0.0)
        || (ab_d == 0.0 && point_segment_distance(d, a, b) == 0.0)
        || (cd_a == 0.0 && point_segment_distance(a, c, d) == 0.0)
        || (cd_b == 0.0 && point_segment_distance(b, c, d) == 0.0)
        || (ab_c.signum() != ab_d.signum() && cd_a.signum() != cd_b.signum())
}

fn segment_box_distance(a: [f64; 2], b: [f64; 2], bounds: LabelBox) -> f64 {
    if point_box_distance(a, bounds) == 0.0 || point_box_distance(b, bounds) == 0.0 {
        return 0.0;
    }
    let corners = [
        [bounds.left, bounds.bottom],
        [bounds.right, bounds.bottom],
        [bounds.right, bounds.top],
        [bounds.left, bounds.top],
    ];
    let mut distance = point_box_distance(a, bounds).min(point_box_distance(b, bounds));
    for index in 0..4 {
        let c = corners[index];
        let d = corners[(index + 1) % 4];
        if segments_cross(a, b, c, d) {
            return 0.0;
        }
        distance = distance.min(point_segment_distance(c, a, b));
    }
    distance
}

fn spatial_geometry_cost(bounds: LabelBox, obstacles: &[SpatialObstacle]) -> f64 {
    obstacles
        .iter()
        .map(|obstacle| {
            let distance = match obstacle {
                SpatialObstacle::Segment(a, b, margin) => {
                    segment_box_distance(*a, *b, bounds) - margin
                }
                SpatialObstacle::Disc(center, radius) => {
                    point_box_distance(*center, bounds) - radius
                }
            };
            (-distance).max(0.0).powi(2)
        })
        .sum()
}

fn spatial_score(
    label: &SpatialLabel,
    bounds: LabelBox,
    at: [f64; 2],
    placed: &[LabelBox],
    obstacles: &[SpatialObstacle],
    width: f64,
    height: f64,
) -> f64 {
    let overlap = placed
        .iter()
        .map(|other| overlap_area(bounds, *other))
        .sum::<f64>()
        + spatial_geometry_cost(bounds, obstacles);
    let outside = (SLIDE_PADDING - bounds.left).max(0.0)
        + (bounds.right - width + SLIDE_PADDING).max(0.0)
        + (SLIDE_PADDING - bounds.bottom).max(0.0)
        + (bounds.top - height + SLIDE_PADDING).max(0.0);
    let distance = (at[0] - label.anchor[0]).powi(2) + (at[1] - label.anchor[1]).powi(2);
    overlap * 1.0e12 + outside * 1.0e9 + distance
}

fn optimize_spatial_labels(
    labels: &mut [SpatialLabel],
    obstacles: &[SpatialObstacle],
    width: f64,
    height: f64,
) -> usize {
    let mut order: Vec<usize> = (0..labels.len()).collect();
    order.sort_by(|a, b| {
        let conflicts = |index: usize| {
            labels
                .iter()
                .enumerate()
                .filter(|(other, label)| {
                    *other != index && overlap_area(label.bounds, labels[index].bounds) > 0.0
                })
                .count()
        };
        conflicts(*b)
            .cmp(&conflicts(*a))
            .then_with(|| labels[*a].name.cmp(&labels[*b].name))
    });
    let mut placed = Vec::new();
    for index in order {
        let (mut best_at, mut best_bounds) =
            spatial_candidate(&labels[index], labels[index].anchor, width, height);
        let mut best_score = spatial_score(
            &labels[index],
            best_bounds,
            best_at,
            &placed,
            obstacles,
            width,
            height,
        );
        for radius in [24.0, 40.0, 64.0, 96.0, 144.0, 224.0, 320.0] {
            for step in 0..16 {
                let angle = std::f64::consts::TAU * f64::from(step) / 16.0;
                let direction = [angle.cos(), angle.sin()];
                let (at, bounds) = spatial_candidate(
                    &labels[index],
                    [
                        labels[index].anchor[0] + direction[0] * radius,
                        labels[index].anchor[1] + direction[1] * radius,
                    ],
                    width,
                    height,
                );
                let score = spatial_score(
                    &labels[index],
                    bounds,
                    at,
                    &placed,
                    obstacles,
                    width,
                    height,
                );
                if score < best_score {
                    best_at = at;
                    best_bounds = bounds;
                    best_score = score;
                }
            }
        }
        if best_score >= 1.0e12 {
            let bounds = labels[index].bounds;
            let label_width = bounds.right - bounds.left;
            let label_height = bounds.top - bounds.bottom;
            let mut y = SLIDE_PADDING;
            while y + label_height <= height - SLIDE_PADDING {
                let mut x = SLIDE_PADDING;
                while x + label_width <= width - SLIDE_PADDING {
                    let (at, bounds) = spatial_candidate(
                        &labels[index],
                        [
                            labels[index].anchor[0] + x - bounds.left,
                            labels[index].anchor[1] + y - bounds.bottom,
                        ],
                        width,
                        height,
                    );
                    let score = spatial_score(
                        &labels[index],
                        bounds,
                        at,
                        &placed,
                        obstacles,
                        width,
                        height,
                    );
                    if score < best_score {
                        best_at = at;
                        best_bounds = bounds;
                        best_score = score;
                    }
                    x += 16.0;
                }
                y += 16.0;
            }
        }
        labels[index].at = best_at;
        labels[index].bounds = best_bounds;
        placed.push(best_bounds);
    }
    let mut overlaps = 0;
    for (index, label) in labels.iter().enumerate() {
        if spatial_geometry_cost(label.bounds, obstacles) > 0.0 {
            overlaps += 1;
            log::warn!("spatial text overlaps geometry: '{}'", label.name);
        }
        for other in labels.iter().skip(index + 1) {
            if overlap_area(label.bounds, other.bounds) > 0.0 {
                overlaps += 1;
                log::warn!(
                    "spatial text overlap: '{}' and '{}'",
                    label.name,
                    other.name
                );
            }
        }
    }
    overlaps
}

/// Resolve billboard-label collisions on screen, then put their anchors back into the 3D scene.
pub fn spatial_layout(
    items: &mut [TextItem],
    obstacles: &[SpatialObstacle],
    camera: &crate::camera::Camera,
    aspect: f64,
    split: f64,
    anchor: session_rust::Point,
    width: f64,
    height: f64,
    pixel_scale: f64,
) -> usize {
    let projection = camera.view_proj_anchored(aspect, &anchor);
    let projection_y =
        (projection.m[1].powi(2) + projection.m[5].powi(2) + projection.m[9].powi(2))
            .sqrt()
            .max(1e-12);
    let right = camera
        .orientation
        .rotate_vector(session_rust::Vector::x_axis());
    let up = session_rust::Vector::new(camera.up[0], camera.up[1], camera.up[2]);
    let mut labels = Vec::new();
    for (index, original) in items.iter().enumerate().filter(|(_, item)| item.spatial) {
        let mut item = original.clone();
        if item.camera_facing {
            item.right = [right[0], right[1], right[2]];
            item.up = [up[0], up[1], up[2]];
        }
        let Some((anchor_at, w)) =
            screen_projection(&projection, &anchor, original.at, [width, height])
        else {
            continue;
        };
        let mut bounds = LabelBox {
            left: f64::INFINITY,
            right: f64::NEG_INFINITY,
            bottom: f64::INFINITY,
            top: f64::NEG_INFINITY,
        };
        let output = typeset(&[item]);
        let mut include = |candidate: LabelBox| {
            bounds.left = bounds.left.min(candidate.left);
            bounds.right = bounds.right.max(candidate.right);
            bounds.bottom = bounds.bottom.min(candidate.bottom);
            bounds.top = bounds.top.max(candidate.top);
        };
        for run in output.texts {
            let Some(metrics) = glyph_metrics(&run.text) else {
                continue;
            };
            let (at, em) = if original.fixed && original.camera_facing {
                (
                    [
                        anchor_at[0] + f64::from(run.offset[0]) * pixel_scale,
                        anchor_at[1] + f64::from(run.offset[1]) * pixel_scale,
                    ],
                    run.height * SPATIAL_FONT_SCALE * pixel_scale,
                )
            } else {
                let Some((at, w)) =
                    screen_projection(&projection, &anchor, run.at, [width, height])
                else {
                    continue;
                };
                (at, run.height * projection_y * height / (2.0 * w))
            };
            include(LabelBox {
                left: at[0] + metrics.bounds[0] * em,
                right: at[0] + metrics.bounds[2] * em,
                bottom: at[1] + (metrics.bounds[1] - metrics.texture_baseline) * em,
                top: at[1] + (metrics.bounds[3] - metrics.texture_baseline) * em,
            });
        }
        for line in output
            .documents
            .iter()
            .flat_map(|doc| &doc.session.objects.lines)
        {
            for point in [line.start(), line.end()] {
                if let Some((at, _)) = screen_projection(
                    &projection,
                    &anchor,
                    [point[0], point[1], point[2]],
                    [width, height],
                ) {
                    include(LabelBox {
                        left: at[0] - 1.0,
                        right: at[0] + 1.0,
                        bottom: at[1] - 1.0,
                        top: at[1] + 1.0,
                    });
                }
            }
        }
        if bounds.left.is_finite() {
            labels.push(SpatialLabel {
                index,
                name: original.name.clone(),
                anchor: anchor_at,
                at: anchor_at,
                bounds,
                w,
            });
        }
    }
    let overlaps = optimize_spatial_labels(&mut labels, obstacles, width * split, height);
    for label in labels {
        let world_per_pixel = 2.0 * label.w / projection_y / height;
        let dx = (label.at[0] - label.anchor[0]) * world_per_pixel;
        let dy = -(label.at[1] - label.anchor[1]) * world_per_pixel;
        for axis in 0..3 {
            items[label.index].at[axis] += right[axis] * dx + up[axis] * dy;
        }
    }
    overlaps
}

#[derive(Clone, Copy, PartialEq)]
enum Style {
    Text,
    Script,
    ScriptScript,
}

impl Style {
    fn scale(self) -> f64 {
        match self {
            Self::Text => 1.0,
            Self::Script => 0.7,
            Self::ScriptScript => 0.5,
        }
    }

    fn smaller(self) -> Self {
        match self {
            Self::Text => Self::Script,
            Self::Script | Self::ScriptScript => Self::ScriptScript,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum FontStyle {
    Math,
    Roman,
    Bold,
    Text,
}

#[derive(Clone)]
enum Node {
    Run(String, FontStyle),
    Sequence(Vec<Node>),
    Fraction(Box<Node>, Box<Node>),
    Sqrt {
        root: Option<Box<Node>>,
        body: Box<Node>,
    },
    Scripts {
        base: Box<Node>,
        sup: Option<Box<Node>>,
        sub: Option<Box<Node>>,
    },
    Font(FontStyle, Box<Node>),
    Style(Style, Box<Node>),
    Color([u8; 4], Box<Node>),
    Matrix(Vec<Vec<Node>>, Option<(char, char)>),
    Space(f64),
    CellBreak,
    RowBreak,
    End,
}

#[derive(Clone)]
struct Run {
    text: String,
    source: String,
    x: f64,
    baseline: f64,
    size: f64,
    color: Option<[u8; 4]>,
}

#[derive(Clone)]
struct Rule {
    from: [f64; 2],
    to: [f64; 2],
    width: f64,
}

#[derive(Clone, Default)]
struct Block {
    runs: Vec<Run>,
    lines: Vec<Rule>,
    width: f64,
    ascent: f64,
    depth: f64,
}

impl Block {
    fn shifted(mut self, dx: f64, dy: f64) -> Self {
        for run in &mut self.runs {
            run.x += dx;
            run.baseline += dy;
        }
        for rule in &mut self.lines {
            rule.from[0] += dx;
            rule.from[1] += dy;
            rule.to[0] += dx;
            rule.to[1] += dy;
        }
        self
    }

    fn scaled(mut self, factor: f64) -> Self {
        for run in &mut self.runs {
            run.x *= factor;
            run.baseline *= factor;
            run.size *= factor;
        }
        for rule in &mut self.lines {
            rule.from = rule.from.map(|value| value * factor);
            rule.to = rule.to.map(|value| value * factor);
            rule.width *= factor;
        }
        self.width *= factor;
        self.ascent *= factor;
        self.depth *= factor;
        self
    }
}

fn layout(node: &Node, style: Style) -> Block {
    match node {
        Node::Run(text, font) => {
            let size = style.scale();
            let source = text.clone();
            let text = styled_text(text, *font);
            let width = measure(&text, size);
            Block {
                runs: vec![Run {
                    text,
                    source,
                    x: 0.0,
                    baseline: 0.0,
                    size,
                    color: None,
                }],
                lines: Vec::new(),
                width,
                ascent: size * 0.82,
                depth: size * 0.18,
            }
        }
        Node::Space(width) => Block {
            width: width * style.scale(),
            ..Block::default()
        },
        Node::Sequence(nodes) => {
            let mut out = Block::default();
            for node in nodes {
                let next = layout(node, style).shifted(out.width, 0.0);
                out.width += next.width;
                out.ascent = out.ascent.max(next.ascent);
                out.depth = out.depth.max(next.depth);
                out.runs.extend(next.runs);
                out.lines.extend(next.lines);
            }
            out
        }
        Node::Font(font, body) => {
            let mut out = layout(body, style);
            let mut shift = 0.0;
            for run in &mut out.runs {
                let before = measure(&run.text, run.size);
                let text = styled_text(&run.source, *font);
                let after = measure(&text, run.size);
                run.x += shift;
                run.text = text;
                shift += after - before;
            }
            out.width += shift;
            out
        }
        Node::Style(next, body) => layout(body, *next),
        Node::Scripts { base, sup, sub } => {
            let limits =
                matches!(base.as_ref(), Node::Run(text, _) if matches!(text.as_str(), "∑" | "∏"));
            let base = layout(base, style);
            let base = if limits { base.scaled(1.45) } else { base };
            let sup = sup.as_ref().map(|node| layout(node, style.smaller()));
            let sub = sub.as_ref().map(|node| layout(node, style.smaller()));
            let script_width = sup
                .as_ref()
                .map_or(0.0, |block| block.width)
                .max(sub.as_ref().map_or(0.0, |block| block.width));
            let raise = 0.78 * style.scale();
            let drop = 0.36 * style.scale();
            let script_x = base.width + 0.06 * style.scale();
            let mut out = Block {
                width: script_x + script_width,
                ascent: base.ascent,
                depth: base.depth,
                runs: base.runs,
                lines: base.lines,
            };
            if limits {
                let width = base.width.max(script_width);
                out.width = width;
                out.runs = out
                    .runs
                    .into_iter()
                    .map(|run| Run {
                        x: run.x + (width - base.width) * 0.5,
                        ..run
                    })
                    .collect();
                if let Some(sup) = sup {
                    let baseline = base.ascent + 0.12 * style.scale() + sup.depth;
                    let x = (width - sup.width) * 0.5;
                    out.ascent = out.ascent.max(baseline + sup.ascent);
                    out.runs.extend(sup.shifted(x, baseline).runs);
                }
                if let Some(sub) = sub {
                    let baseline = -base.depth - 0.16 * style.scale() - sub.ascent;
                    let x = (width - sub.width) * 0.5;
                    out.depth = out.depth.max(-baseline + sub.depth);
                    out.runs.extend(sub.shifted(x, baseline).runs);
                }
                return out;
            }
            if let Some(sup) = sup {
                out.ascent = out.ascent.max(raise + sup.ascent);
                let sup = sup.shifted(script_x, raise);
                out.runs.extend(sup.runs);
                out.lines.extend(sup.lines);
            }
            if let Some(sub) = sub {
                out.depth = out.depth.max(drop + sub.depth);
                let sub = sub.shifted(script_x, -drop);
                out.runs.extend(sub.runs);
                out.lines.extend(sub.lines);
            }
            out
        }
        Node::Fraction(numerator, denominator) => {
            let scale = style.scale();
            let numerator = layout(numerator, style);
            let denominator = layout(denominator, style);
            let width = numerator.width.max(denominator.width) + 0.24 * scale;
            let axis = 0.28 * scale;
            let gap = 0.16 * scale;
            let numerator_x = (width - numerator.width) * 0.5;
            let numerator_y = axis + gap + numerator.depth;
            let denominator_x = (width - denominator.width) * 0.5;
            let denominator_y = axis - gap - denominator.ascent;
            let numerator = numerator.shifted(numerator_x, numerator_y);
            let denominator = denominator.shifted(denominator_x, denominator_y);
            let mut out = Block {
                width,
                ascent: numerator_y + numerator.ascent,
                depth: -denominator_y + denominator.depth,
                runs: numerator.runs,
                lines: vec![Rule {
                    from: [0.0, axis],
                    to: [width, axis],
                    width: 0.055 * scale,
                }],
            };
            out.runs.extend(denominator.runs);
            out.lines.extend(numerator.lines);
            out.lines.extend(denominator.lines);
            out
        }
        Node::Sqrt { root, body } => {
            let body = layout(body, style);
            let scale = style.scale();
            let height = (body.ascent + body.depth + 0.16 * scale).max(scale);
            let bottom = -body.depth;
            let top = body.ascent + 0.10 * scale;
            let radical_width = 0.48 * height + 0.22 * scale;
            let body_x = radical_width + 0.08 * scale;
            let hook = [
                [0.0, bottom + 0.55 * height],
                [0.14 * radical_width, bottom + 0.42 * height],
                [0.34 * radical_width, bottom + 0.12 * height],
                [radical_width, top],
            ];
            let pen = 0.05 * scale;
            let mut out = Block {
                width: body_x + body.width + 0.08 * scale,
                ascent: top + 0.06 * scale,
                depth: body.depth,
                runs: Vec::new(),
                lines: vec![
                    Rule {
                        from: hook[0],
                        to: hook[1],
                        width: pen,
                    },
                    Rule {
                        from: hook[1],
                        to: hook[2],
                        width: pen,
                    },
                    Rule {
                        from: hook[2],
                        to: hook[3],
                        width: pen,
                    },
                    Rule {
                        from: hook[3],
                        to: [body_x + body.width + 0.08 * scale, top],
                        width: pen,
                    },
                ],
            };
            let body = body.shifted(body_x, 0.0);
            out.runs.extend(body.runs);
            out.lines.extend(body.lines);
            if let Some(root) = root {
                let root = layout(root, style.smaller());
                let root_x = -root.width * 0.65;
                out.runs.extend(root.shifted(root_x, 0.45 * scale).runs);
                out.width = out.width.max(body_x + body.width);
            }
            out
        }
        Node::Color(rgb, body) => {
            let mut out = layout(body, style);
            for run in &mut out.runs {
                run.color = Some(*rgb);
            }
            out
        }
        Node::Matrix(rows, brackets) => {
            let scale = style.scale();
            let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
            let cells: Vec<Vec<Block>> = rows
                .iter()
                .map(|row| row.iter().map(|cell| layout(cell, style)).collect())
                .collect();
            let mut widths = vec![0.0_f64; columns];
            for row in &cells {
                for (column, cell) in row.iter().enumerate() {
                    widths[column] = widths[column].max(cell.width);
                }
            }
            let gap = 0.28 * scale;
            let mut out = Block::default();
            let mut baseline = 0.0;
            for (row_index, row) in cells.into_iter().enumerate() {
                let row_ascent = row
                    .iter()
                    .fold(0.0_f64, |value, cell| value.max(cell.ascent));
                let row_depth = row
                    .iter()
                    .fold(0.0_f64, |value, cell| value.max(cell.depth));
                let mut x = 0.0;
                for (column, cell) in row.into_iter().enumerate() {
                    let offset = (widths[column] - cell.width) * 0.5;
                    let mut cell = cell.shifted(x + offset, baseline);
                    cell.width = widths[column];
                    out.runs.extend(cell.runs);
                    out.lines.extend(cell.lines);
                    x += widths[column] + gap;
                }
                out.ascent = out.ascent.max(row_ascent);
                baseline -= row_ascent + row_depth + gap;
                if row_index + 1 == rows.len() {
                    out.depth = -baseline - gap;
                }
            }
            out.width = widths.iter().sum::<f64>() + gap * columns.saturating_sub(1) as f64;
            if let Some((left, right)) = brackets {
                let height = out.ascent + out.depth + 0.35 * scale;
                let center = (out.ascent - out.depth) * 0.5;
                let left_width = measure(&left.to_string(), height);
                let right_width = measure(&right.to_string(), height);
                out.runs.push(Run {
                    text: left.to_string(),
                    source: left.to_string(),
                    x: -left_width - 0.12 * scale,
                    baseline: center - height * 0.5,
                    size: height,
                    color: None,
                });
                out.runs.push(Run {
                    text: right.to_string(),
                    source: right.to_string(),
                    x: out.width + 0.12 * scale,
                    baseline: center - height * 0.5,
                    size: height,
                    color: None,
                });
                out.width += left_width + right_width + 0.24 * scale;
            }
            out
        }
        Node::CellBreak | Node::RowBreak | Node::End => Block::default(),
    }
}

fn cached_layout(text: &str) -> Rc<Block> {
    LAYOUTS.with_borrow_mut(|cache| {
        if let Some(block) = cache.get(text) {
            return Rc::clone(block);
        }
        if cache.len() >= 4096 {
            cache.clear();
        }
        let block = Rc::new(layout(&Parser::new(text).parse(), Style::Text));
        cache.insert(text.to_owned(), Rc::clone(&block));
        block
    })
}

thread_local! {
    static MEASURER: RefCell<Option<TextDocument>> = const { RefCell::new(None) };
    static LAYOUTS: RefCell<std::collections::HashMap<String, Rc<Block>>> =
        RefCell::new(std::collections::HashMap::new());
    static GLYPH_METRICS: RefCell<std::collections::HashMap<String, Option<GlyphMetrics>>> =
        RefCell::new(std::collections::HashMap::new());
    static INK_BOUNDS: RefCell<std::collections::HashMap<(String, bool, bool), Option<[f64; 4]>>> =
        RefCell::new(std::collections::HashMap::new());
}

/// Fit one centered matrix value inside its cell without changing its authored data.
pub fn matrix_value_height(text: &str, cell: f64) -> f64 {
    let width = measure(text, 1.0).max(0.01);
    (cell * 0.44).min(cell * 0.76 / width)
}

/// Measure the exact shaped advance of a math run with the bundled OFL fonts.
fn measure(text: &str, em: f64) -> f64 {
    if text.is_empty() {
        return 0.0;
    }
    MEASURER.with_borrow_mut(|slot| {
        let document = slot.get_or_insert_with(|| {
            let mut document = TextDocument::new();
            document
                .replace_fonts(vec![
                    FONT_BYTES.to_vec(),
                    MATH_FONT_BYTES.to_vec(),
                    STIX_REGULAR_BYTES.to_vec(),
                    STIX_ITALIC_BYTES.to_vec(),
                    FALLBACK_BYTES.to_vec(),
                    SYMBOL_BYTES.to_vec(),
                ])
                .expect("bundled math fonts");
            document
        });
        let font_size = (em * 18.0).clamp(1.0, 256.0) as f32;
        let label = TextLabel {
            id: MATH_SERIF_ID,
            object: None,
            text: text.to_string(),
            font_size,
            line_height: font_size * 1.3,
            color: [0, 0, 0, 255],
            outline: 0.0,
            placement: TextPlacement::Screen {
                left: 0.0,
                top: 0.0,
            },
            clip: None,
        };
        if document.set_labels(vec![label]).is_err() {
            return 0.0;
        }
        document.runs[0]
            .buffer
            .layout_runs()
            .map(|line| f64::from(line.line_w) / 18.0)
            .sum()
    })
}

fn texture_baseline() -> f64 {
    MEASURER.with_borrow_mut(|slot| {
        let document = slot.get_or_insert_with(TextDocument::new);
        let label = TextLabel {
            id: MATH_SERIF_ID,
            object: None,
            text: "x".into(),
            font_size: 18.0,
            line_height: 18.0 * 1.3,
            color: [0, 0, 0, 255],
            outline: 0.0,
            placement: TextPlacement::Screen {
                left: 0.0,
                top: 0.0,
            },
            clip: None,
        };
        if document.set_labels(vec![label]).is_err() {
            return 1.0;
        }
        document.runs[0]
            .buffer
            .layout_runs()
            .next()
            .map_or(1.0, |line| f64::from(line.line_y) / 18.0)
    })
}

fn styled_text(text: &str, font: FontStyle) -> String {
    text.chars().map(|c| styled_char(c, font)).collect()
}

fn styled_char(c: char, font: FontStyle) -> char {
    let code = c as u32;
    let mapped = match font {
        FontStyle::Math => match c {
            'A'..='Z' => Some(0x1D434 + code - 'A' as u32),
            'a'..='z' => Some(0x1D44E + code - 'a' as u32),
            'Α'..='Ω' => Some(0x1D6E2 + code - 'Α' as u32),
            'α'..='ω' => Some(0x1D6FC + code - 'α' as u32),
            _ => None,
        },
        FontStyle::Bold => match c {
            'A'..='Z' => Some(0x1D400 + code - 'A' as u32),
            'a'..='z' => Some(0x1D41A + code - 'a' as u32),
            '0'..='9' => Some(0x1D7CE + code - '0' as u32),
            'Α'..='Ω' => Some(0x1D68E + code - 'Α' as u32),
            'α'..='ω' => Some(0x1D6C2 + code - 'α' as u32),
            _ => None,
        },
        FontStyle::Roman | FontStyle::Text => None,
    };
    mapped.and_then(char::from_u32).unwrap_or(c)
}

struct Parser {
    source: Vec<char>,
    at: usize,
}

impl Parser {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            at: 0,
        }
    }

    fn parse(mut self) -> Node {
        self.sequence(None)
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.at).copied()
    }

    fn take(&mut self) -> Option<char> {
        let next = self.peek()?;
        self.at += 1;
        Some(next)
    }

    fn sequence(&mut self, closing: Option<char>) -> Node {
        let mut nodes = Vec::new();
        while let Some(c) = self.peek() {
            if Some(c) == closing || c == '}' {
                break;
            }
            let relation = matches!(c, '=' | '<' | '>');
            let binary = matches!(c, '+' | '-');
            let unary = binary
                && nodes
                    .iter()
                    .rev()
                    .find(|node| !matches!(node, Node::Space(_)))
                    .is_none_or(|node| {
                        matches!(node, Node::CellBreak | Node::RowBreak)
                            || matches!(node, Node::Run(text, _) if matches!(text.as_str(), "=" | "+" | "-" | "(" | "[" | ","))
                    });
            if (relation || binary) && !unary {
                nodes.push(Node::Space(if relation { 0.28 } else { 0.22 }));
            }
            let mut node = self.atom();
            if matches!(node, Node::End) {
                break;
            }
            let mut sup = None;
            let mut sub = None;
            loop {
                match self.peek() {
                    Some('^') => {
                        self.at += 1;
                        sup = Some(Box::new(self.required()));
                    }
                    Some('_') => {
                        self.at += 1;
                        sub = Some(Box::new(self.required()));
                    }
                    _ => break,
                }
            }
            if sup.is_some() || sub.is_some() {
                node = Node::Scripts {
                    base: Box::new(node),
                    sup,
                    sub,
                };
            }
            nodes.push(node);
            if relation || binary {
                nodes.push(Node::Space(if relation { 0.28 } else { 0.22 }));
            } else if matches!(c, ',' | ';') {
                nodes.push(Node::Space(0.2));
            }
        }
        if closing.is_some() {
            self.take();
        }
        Node::Sequence(nodes)
    }

    fn required(&mut self) -> Node {
        if self.peek() == Some('{') {
            self.at += 1;
            self.sequence(Some('}'))
        } else {
            self.atom()
        }
    }

    fn atom(&mut self) -> Node {
        match self.take() {
            Some('{') => self.sequence(Some('}')),
            Some('\\') => self.command(),
            Some('&') => Node::CellBreak,
            Some('^') | Some('_') => Node::Run(String::new(), FontStyle::Math),
            Some(c) if c.is_whitespace() => Node::Space(0.33),
            Some(c) if c.is_ascii_alphabetic() => {
                let mut text = String::from(c);
                while self.peek().is_some_and(|c| c.is_ascii_alphabetic()) {
                    text.push(self.take().unwrap());
                }
                Node::Run(text, FontStyle::Math)
            }
            Some(c) if c.is_ascii_digit() => {
                let mut text = String::from(c);
                while self.peek().is_some_and(|c| c.is_ascii_digit() || c == '.') {
                    text.push(self.take().unwrap());
                }
                Node::Run(text, FontStyle::Math)
            }
            Some(c) => Node::Run(c.to_string(), FontStyle::Math),
            None => Node::Run(String::new(), FontStyle::Math),
        }
    }

    fn command(&mut self) -> Node {
        if self.peek() == Some('\\') {
            self.at += 1;
            return Node::RowBreak;
        }
        let mut name = String::new();
        while self.peek().is_some_and(|c| c.is_ascii_alphabetic()) {
            name.push(self.take().unwrap());
        }
        match name.as_str() {
            "color" => {
                let color = parse_hex_color(&self.required());
                match color {
                    Some(rgb) => Node::Color(rgb, Box::new(self.required())),
                    None => Node::Run("\\color".into(), FontStyle::Roman),
                }
            }
            "frac" | "dfrac" | "tfrac" => {
                let numerator = self.required();
                let denominator = self.required();
                Node::Fraction(Box::new(numerator), Box::new(denominator))
            }
            "sqrt" => {
                let root = if self.peek() == Some('[') {
                    self.at += 1;
                    let root = self.sequence(Some(']'));
                    Some(Box::new(root))
                } else {
                    None
                };
                let body = self.required();
                Node::Sqrt {
                    root,
                    body: Box::new(body),
                }
            }
            "text" | "operatorname" => Node::Font(FontStyle::Text, Box::new(self.required())),
            "mathrm" => Node::Font(FontStyle::Roman, Box::new(self.required())),
            "mathbf" | "boldsymbol" | "bm" => {
                Node::Font(FontStyle::Bold, Box::new(self.required()))
            }
            "mathit" => Node::Font(FontStyle::Math, Box::new(self.required())),
            "displaystyle" => Node::Style(Style::Text, Box::new(self.required())),
            "textstyle" => Node::Style(Style::Text, Box::new(self.required())),
            "scriptstyle" => Node::Style(Style::Script, Box::new(self.required())),
            "scriptscriptstyle" => Node::Style(Style::ScriptScript, Box::new(self.required())),
            "cos" | "sin" | "tan" | "ln" | "log" | "exp" | "min" | "max" | "lim" | "det"
            | "arg" => Node::Sequence(vec![
                Node::Space(0.18),
                Node::Run(name, FontStyle::Roman),
                Node::Space(0.18),
            ]),
            "sum" => Node::Run("∑".into(), FontStyle::Math),
            "prod" => Node::Run("∏".into(), FontStyle::Math),
            "int" => Node::Run("∫".into(), FontStyle::Math),
            "iint" => Node::Run("∬".into(), FontStyle::Math),
            "iiint" => Node::Run("∭".into(), FontStyle::Math),
            "oint" => Node::Run("∮".into(), FontStyle::Math),
            "partial" => Node::Run("∂".into(), FontStyle::Math),
            "nabla" => Node::Run("∇".into(), FontStyle::Math),
            "infty" => Node::Run("∞".into(), FontStyle::Math),
            "pm" | "mp" | "times" | "cdot" | "div" => {
                let symbol = match name.as_str() {
                    "pm" => "±",
                    "mp" => "∓",
                    "times" => "×",
                    "cdot" => "⋅",
                    _ => "÷",
                };
                Node::Sequence(vec![
                    Node::Space(0.22),
                    Node::Run(symbol.into(), FontStyle::Math),
                    Node::Space(0.22),
                ])
            }
            "le" | "leq" | "ge" | "geq" | "ne" | "neq" | "approx" | "equiv" | "sim" | "simeq"
            | "to" | "rightarrow" | "longrightarrow" | "leftarrow" => {
                let symbol = match name.as_str() {
                    "le" | "leq" => "≤",
                    "ge" | "geq" => "≥",
                    "ne" | "neq" => "≠",
                    "approx" => "≈",
                    "equiv" => "≡",
                    "sim" => "∼",
                    "simeq" => "≃",
                    "to" | "rightarrow" => "→",
                    "longrightarrow" => "⟶",
                    _ => "←",
                };
                Node::Sequence(vec![
                    Node::Space(0.28),
                    Node::Run(symbol.into(), FontStyle::Math),
                    Node::Space(0.28),
                ])
            }
            "mapsto" => Node::Run("↦".into(), FontStyle::Math),
            "in" => Node::Run("∈".into(), FontStyle::Math),
            "notin" => Node::Run("∉".into(), FontStyle::Math),
            "subset" => Node::Run("⊂".into(), FontStyle::Math),
            "subseteq" => Node::Run("⊆".into(), FontStyle::Math),
            "supset" => Node::Run("⊃".into(), FontStyle::Math),
            "supseteq" => Node::Run("⊇".into(), FontStyle::Math),
            "cup" => Node::Run("∪".into(), FontStyle::Math),
            "cap" => Node::Run("∩".into(), FontStyle::Math),
            "forall" => Node::Run("∀".into(), FontStyle::Math),
            "exists" => Node::Run("∃".into(), FontStyle::Math),
            "emptyset" => Node::Run("∅".into(), FontStyle::Math),
            "degree" => Node::Run("°".into(), FontStyle::Math),
            "alpha" => Node::Run("α".into(), FontStyle::Math),
            "beta" => Node::Run("β".into(), FontStyle::Math),
            "gamma" => Node::Run("γ".into(), FontStyle::Math),
            "delta" => Node::Run("δ".into(), FontStyle::Math),
            "epsilon" | "varepsilon" => Node::Run("ε".into(), FontStyle::Math),
            "zeta" => Node::Run("ζ".into(), FontStyle::Math),
            "eta" => Node::Run("η".into(), FontStyle::Math),
            "theta" => Node::Run("θ".into(), FontStyle::Math),
            "vartheta" => Node::Run("ϑ".into(), FontStyle::Math),
            "iota" => Node::Run("ι".into(), FontStyle::Math),
            "kappa" => Node::Run("κ".into(), FontStyle::Math),
            "lambda" => Node::Run("λ".into(), FontStyle::Math),
            "mu" => Node::Run("μ".into(), FontStyle::Math),
            "nu" => Node::Run("ν".into(), FontStyle::Math),
            "xi" => Node::Run("ξ".into(), FontStyle::Math),
            "pi" => Node::Run("π".into(), FontStyle::Math),
            "rho" => Node::Run("ρ".into(), FontStyle::Math),
            "sigma" => Node::Run("σ".into(), FontStyle::Math),
            "tau" => Node::Run("τ".into(), FontStyle::Math),
            "upsilon" => Node::Run("υ".into(), FontStyle::Math),
            "phi" | "varphi" => Node::Run("φ".into(), FontStyle::Math),
            "chi" => Node::Run("χ".into(), FontStyle::Math),
            "psi" => Node::Run("ψ".into(), FontStyle::Math),
            "omega" => Node::Run("ω".into(), FontStyle::Math),
            "Gamma" => Node::Run("Γ".into(), FontStyle::Math),
            "Delta" => Node::Run("Δ".into(), FontStyle::Math),
            "Theta" => Node::Run("Θ".into(), FontStyle::Math),
            "Lambda" => Node::Run("Λ".into(), FontStyle::Math),
            "Xi" => Node::Run("Ξ".into(), FontStyle::Math),
            "Pi" => Node::Run("Π".into(), FontStyle::Math),
            "Sigma" => Node::Run("Σ".into(), FontStyle::Math),
            "Upsilon" => Node::Run("Υ".into(), FontStyle::Math),
            "Phi" => Node::Run("Φ".into(), FontStyle::Math),
            "Psi" => Node::Run("Ψ".into(), FontStyle::Math),
            "Omega" => Node::Run("Ω".into(), FontStyle::Math),
            "quad" => Node::Space(1.0),
            "qquad" => Node::Space(2.0),
            "," => Node::Space(0.28),
            ";" => Node::Space(0.32),
            ":" => Node::Space(0.4),
            "!" => Node::Space(-0.17),
            "left" | "right" => self.atom(),
            "begin" => {
                let environment = self.required();
                let kind = match environment {
                    Node::Sequence(nodes) => nodes.into_iter().find_map(|node| match node {
                        Node::Run(text, _) => Some(text),
                        _ => None,
                    }),
                    _ => None,
                };
                let body = self.sequence(None);
                let brackets = match kind.as_deref() {
                    Some("pmatrix") => Some(('(', ')')),
                    Some("bmatrix") => Some(('[', ']')),
                    Some("Bmatrix") => Some(('{', '}')),
                    _ => None,
                };
                let mut rows = vec![Vec::new()];
                for node in flatten(body) {
                    match node {
                        Node::CellBreak => rows.last_mut().unwrap().push(Node::RowBreak),
                        Node::RowBreak => {
                            rows.last_mut().unwrap().push(Node::CellBreak);
                            rows.push(Vec::new());
                        }
                        other => rows.last_mut().unwrap().push(other),
                    }
                }
                Node::Matrix(rows, brackets)
            }
            "end" => {
                self.required();
                Node::End
            }
            "%" => Node::Run("%".into(), FontStyle::Math),
            "_" => Node::Run("_".into(), FontStyle::Math),
            "{" => Node::Run("{".into(), FontStyle::Math),
            "}" => Node::Run("}".into(), FontStyle::Math),
            "#" => Node::Run("#".into(), FontStyle::Math),
            "&" => Node::Run("&".into(), FontStyle::Math),
            _ if name.is_empty() => Node::Space(0.33),
            _ => Node::Run(format!("\\{name}"), FontStyle::Roman),
        }
    }
}

fn parse_hex_color(node: &Node) -> Option<[u8; 4]> {
    fn text(node: &Node) -> String {
        match node {
            Node::Run(text, _) => text.clone(),
            Node::Sequence(nodes) => nodes.iter().map(text).collect(),
            Node::Font(_, body) | Node::Style(_, body) | Node::Color(_, body) => text(body),
            _ => String::new(),
        }
    }
    let text = text(node);
    let hex = text.strip_prefix('#')?;
    if !matches!(hex.len(), 6 | 8) {
        return None;
    }
    let rgba = u32::from_str_radix(hex, 16).ok()?;
    Some(if hex.len() == 6 {
        [(rgba >> 16) as u8, (rgba >> 8) as u8, rgba as u8, 255]
    } else {
        [
            (rgba >> 24) as u8,
            (rgba >> 16) as u8,
            (rgba >> 8) as u8,
            rgba as u8,
        ]
    })
}

fn flatten(node: Node) -> Vec<Node> {
    match node {
        Node::Sequence(nodes) => nodes,
        other => vec![other],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn label(name: &str, fixed: bool) -> TextItem {
        TextItem {
            text: "x_0".into(),
            math: true,
            center: false,
            center_ink: false,
            fixed,
            max_width: 0.0,
            name: name.into(),
            outline: 0.0,
            at: [100.0, 200.0, 0.0],
            offset: [0.0; 2],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            height: 28.0,
            camera_facing: false,
            spatial: false,
            plate: false,
            color: None,
            serif: false,
        }
    }

    #[test]
    fn collision_optimizer_preserves_font_size_and_anchors() {
        let mut items: Vec<TextItem> = (0..8)
            .map(|index| label(&format!("label-{index}"), index == 0))
            .collect();
        let anchor = items[0].at;
        assert_eq!(optimize_layout(&mut items, 480.0, 360.0), 0);
        assert_eq!(items[0].at, anchor);
        assert!(items.iter().all(|item| item.height == 28.0));
        assert!(items.iter().any(|item| item.at != anchor));
    }

    #[test]
    fn spatial_collision_solver_separates_labels_without_shrinking_them() {
        let mut items = vec![label("first", true), label("second", true)];
        for item in &mut items {
            item.at = [0.0, 0.0, 0.0];
            item.spatial = true;
            item.camera_facing = true;
            item.center = true;
            item.center_ink = true;
            item.height = 18.0;
        }
        let mut camera = crate::camera::Camera::new();
        camera.fit_in(
            &session_rust::AABB::from_points(
                &[
                    session_rust::Point::new(-1.0, -1.0, -1.0),
                    session_rust::Point::new(1.0, 1.0, 1.0),
                ],
                0.0,
            ),
            1.0,
            [0.0, 0.0, 0.56, 1.0],
        );
        let overlaps = spatial_layout(
            &mut items,
            &[],
            &camera,
            1.0,
            0.56,
            session_rust::Point::new(0.0, 0.0, 0.0),
            1600.0,
            760.0,
            1.0,
        );
        assert_eq!(overlaps, 0);
        assert_ne!(items[0].at, items[1].at);
        assert!(items.iter().all(|item| item.height == 18.0));
    }

    #[test]
    fn fixed_spatial_offsets_use_label_coordinates_not_rotated_world_axes() {
        let mut item = label("n0", true);
        item.at = [3.0, 4.0, 5.0];
        item.spatial = true;
        item.camera_facing = true;
        item.center = true;
        item.center_ink = true;
        item.height = 18.0;
        let horizontal = typeset(std::slice::from_ref(&item));
        item.right = [0.0, 1.0, 0.0];
        item.up = [0.0, 0.0, -1.0];
        let rotated = typeset(&[item]);
        assert_eq!(horizontal.texts.len(), rotated.texts.len());
        assert!(
            (horizontal.texts[0].offset[0] - horizontal.texts[1].offset[0]).abs() > 5.0
                && (horizontal.texts[0].offset[1] - horizontal.texts[1].offset[1]).abs() > 5.0
        );
        for (a, b) in horizontal.texts.iter().zip(&rotated.texts) {
            assert_eq!(a.offset, b.offset);
            assert_eq!(a.at, b.at);
        }
    }

    #[test]
    fn width_limits_never_shrink_math_fonts() {
        let mut item = label("long", false);
        item.text = "x_0+y_0+z_0".into();
        item.max_width = 1.0;
        let output = typeset(&[item]);
        assert!(output.texts.iter().any(|text| text.height == 28.0));
    }

    #[test]
    fn math_runs_include_the_texture_baseline_offset() {
        let item = label("baseline", false);
        let block = layout(&Parser::new(&item.text).parse(), Style::Text);
        let output = typeset(&[item.clone()]);
        let expected = item.at[1] + (block.ascent + texture_baseline()) * item.height;
        assert!((output.texts[0].at[1] - expected).abs() < 1e-9);
        assert!((0.5..=2.0).contains(&texture_baseline()));
    }

    #[test]
    fn fractions_have_rules_and_centered_numerator_and_denominator() {
        let block = layout(&Parser::new(r"\frac{a+b}{c}").parse(), Style::Text);
        assert_eq!(block.lines.len(), 1);
        assert!(block.runs.iter().any(|run| run.baseline > 0.0));
        assert!(block.runs.iter().any(|run| run.baseline < 0.0));
        assert!(block.width > 0.0);
    }

    #[test]
    fn display_sum_limits_are_centered_below_and_above() {
        let block = layout(&Parser::new(r"\sum_i^N").parse(), Style::Text);
        let sum = block.runs.iter().find(|run| run.source == "∑").unwrap();
        let lower = block.runs.iter().find(|run| run.source == "i").unwrap();
        let upper = block.runs.iter().find(|run| run.source == "N").unwrap();
        assert!(lower.baseline < sum.baseline);
        assert!(upper.baseline > sum.baseline);
        let center = |run: &Run| run.x + measure(&run.text, run.size) * 0.5;
        assert!((center(lower) - center(sum)).abs() < 1e-3);
        assert!((center(upper) - center(sum)).abs() < 1e-3);
    }

    #[test]
    fn scripts_have_smaller_glyphs_and_vertical_offsets() {
        let block = layout(&Parser::new("x_0^2").parse(), Style::Text);
        assert!(
            block
                .runs
                .iter()
                .any(|run| run.baseline > 0.5 && run.size < 1.0)
        );
        assert!(
            block
                .runs
                .iter()
                .any(|run| run.baseline < 0.0 && run.size < 1.0)
        );
    }

    #[test]
    fn scripts_have_horizontal_clearance_from_their_base() {
        let block = layout(&Parser::new("x_0^2").parse(), Style::Text);
        let base = block.runs.iter().find(|run| run.source == "x").unwrap();
        let right = base.x + measure(&base.text, base.size);
        for run in block
            .runs
            .iter()
            .filter(|run| run.source == "0" || run.source == "2")
        {
            assert!(run.x >= right + 0.05);
        }
    }

    #[test]
    fn subscript_is_below_the_base_in_world_coordinates() {
        let item = TextItem {
            text: "n_0".into(),
            math: true,
            center: false,
            center_ink: false,
            fixed: false,
            max_width: 0.0,
            name: "n0-label".into(),
            outline: 0.0,
            at: [100.0, 200.0, 0.0],
            offset: [0.0; 2],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            height: 20.0,
            camera_facing: false,
            spatial: false,
            plate: false,
            color: None,
            serif: false,
        };
        let block = layout(&Parser::new(&item.text).parse(), Style::Text);
        for run in &block.runs {
            let at = place(&item, &block, run.x, run.baseline, 20.0);
            eprintln!("{} baseline {} at {:?}", run.source, run.baseline, at);
        }
        let base = block.runs.iter().find(|run| run.source == "n").unwrap();
        let sub = block.runs.iter().find(|run| run.source == "0").unwrap();
        let base_y = place(&item, &block, base.x, base.baseline, 20.0).1;
        let sub_y = place(&item, &block, sub.x, sub.baseline, 20.0).1;
        assert!(
            sub_y < base_y,
            "subscript must be below the base in slide coordinates: base {base_y}, sub {sub_y}"
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn subscript_pixels_are_below_base() {
        use crate::engine::gpu::{FrameInput, Gpu};
        use session_rust::{AABB, Point};

        let item = TextItem {
            text: "n_0".into(),
            math: true,
            center: false,
            center_ink: false,
            fixed: false,
            max_width: 0.0,
            name: "n0-label".into(),
            outline: 0.0,
            at: [100.0, 200.0, 0.0],
            offset: [0.0; 2],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            height: 20.0,
            camera_facing: false,
            spatial: false,
            plate: false,
            color: None,
            serif: false,
        };
        let output = typeset(&[item]);
        let labels = output
            .texts
            .iter()
            .map(|text| TextLabel {
                id: text.at[0] as u32 + 1,
                object: None,
                text: text.text.clone(),
                font_size: 18.0,
                line_height: 26.0,
                color: if text.text == "0" {
                    [255, 0, 0, 255]
                } else {
                    [0, 0, 0, 255]
                },
                outline: 0.0,
                placement: TextPlacement::WorldPlane {
                    world: text.at,
                    right: text.right,
                    up: text.up,
                    world_height: text.height,
                    plate: false,
                },
                clip: None,
            })
            .collect::<Vec<_>>();
        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(256, 256)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.show_grid = false;
        gpu.view.lit = false;
        gpu.text.set_labels(labels).unwrap();
        let mut camera = crate::camera::Camera::new();
        camera.set_view(crate::camera::View::Top);
        let bounds = AABB::from_points(
            &[Point::new(95.0, 190.0, 0.0), Point::new(130.0, 230.0, 0.0)],
            0.0,
        );
        camera.fit(&bounds, 1.0);
        let pixels = gpu.render_offscreen(&FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &Point::new(0.0, 0.0, 0.0)),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        });
        let mean_y = |red: bool| {
            let mut total = 0.0;
            let mut count = 0;
            for (index, pixel) in pixels.chunks_exact(4).enumerate() {
                let matches = if red {
                    pixel[0] > 150 && pixel[1] < 100
                } else {
                    pixel[0] < 80 && pixel[1] < 80 && pixel[2] < 80
                };
                if matches {
                    total += (index / 256) as f64;
                    count += 1;
                }
            }
            (count > 0).then_some(total / f64::from(count))
        };
        let base_y = mean_y(false).expect("base glyph pixels");
        let sub_y = mean_y(true).expect("subscript pixels");
        assert!(sub_y > base_y, "base y {base_y}, subscript y {sub_y}");
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn matrix_entry_ink_is_centered_on_its_anchor() {
        use crate::engine::gpu::{FrameInput, Gpu};
        use session_rust::Xform;

        let mut item = label("matrix-entry", true);
        item.text = "-1".into();
        item.center = true;
        item.center_ink = true;
        item.at = [0.0, 0.0, 0.0];
        item.height = 0.5;
        item.color = Some([255, 255, 255, 255]);
        let output = typeset(&[item]);
        let labels = output
            .texts
            .iter()
            .enumerate()
            .map(|(index, text)| TextLabel {
                id: index as u32 + 1 | MATH_SERIF_ID,
                object: None,
                text: text.text.clone(),
                font_size: 18.0,
                line_height: 26.0,
                color: [255, 255, 255, 255],
                outline: 0.0,
                placement: TextPlacement::WorldPlane {
                    world: text.at,
                    right: text.right,
                    up: text.up,
                    world_height: text.height,
                    plate: false,
                },
                clip: None,
            })
            .collect();
        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(256, 256)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.dark = true;
        gpu.view.show_grid = false;
        gpu.view.lit = false;
        gpu.text.set_labels(labels).unwrap();
        let pixels = gpu.render_offscreen(&FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::BLACK,
            now_ms: 0.0,
        });
        let ink: Vec<(usize, usize)> = pixels
            .chunks_exact(4)
            .enumerate()
            .filter(|(_, pixel)| pixel[0] > 160 && pixel[1] > 160 && pixel[2] > 160)
            .map(|(index, _)| (index % 256, index / 256))
            .collect();
        assert!(!ink.is_empty(), "the matrix entry must be visible");
        let bounds = ink
            .iter()
            .fold([usize::MAX, usize::MAX, 0, 0], |mut bounds, &(x, y)| {
                bounds[0] = bounds[0].min(x);
                bounds[1] = bounds[1].min(y);
                bounds[2] = bounds[2].max(x);
                bounds[3] = bounds[3].max(y);
                bounds
            });
        let center = [
            (bounds[0] + bounds[2] + 1) as f64 * 0.5,
            (bounds[1] + bounds[3] + 1) as f64 * 0.5,
        ];
        assert!(
            center.iter().all(|value| (value - 128.0).abs() <= 2.0),
            "visible ink center is {center:?}, expected [128, 128]"
        );
    }

    #[test]
    fn binary_and_relation_operators_have_tex_spacing() {
        let Node::Sequence(nodes) = Parser::new("a+b=c").parse() else {
            panic!("sequence expected")
        };
        assert!(
            nodes
                .iter()
                .filter(|node| matches!(node, Node::Space(_)))
                .count()
                >= 4
        );
        let Node::Sequence(unary) = Parser::new("-a").parse() else {
            panic!("sequence expected")
        };
        assert!(!matches!(unary.first(), Some(Node::Space(_))));
    }

    #[test]
    fn operator_names_and_dots_have_surrounding_space() {
        let block = layout(&Parser::new(r"f\cos\alpha").parse(), Style::Text);
        let f = block.runs.iter().find(|run| run.source == "f").unwrap();
        let cos = block.runs.iter().find(|run| run.source == "cos").unwrap();
        let alpha = block.runs.iter().find(|run| run.source == "α").unwrap();
        assert!(cos.x > f.x + measure(&f.text, f.size));
        assert!(alpha.x > cos.x + measure(&cos.text, cos.size));
        let dot = layout(&Parser::new(r"1\cdot2").parse(), Style::Text);
        let symbol = dot.runs.iter().find(|run| run.source == "⋅").unwrap();
        assert!(symbol.x > 0.0);
        assert!(symbol.x + measure(&symbol.text, symbol.size) < dot.width);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn fraction_rule_reaches_the_gpu() {
        use crate::engine::gpu::{FrameInput, Gpu};
        use session_rust::{AABB, Point};

        let output = typeset(&[TextItem {
            text: r"\frac{a+b}{c}".into(),
            math: true,
            center: true,
            center_ink: false,
            fixed: false,
            max_width: 0.0,
            name: "fraction".into(),
            outline: 0.0,
            at: [100.0, 100.0, 0.0],
            offset: [0.0; 2],
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            height: 80.0,
            camera_facing: false,
            spatial: false,
            plate: false,
            color: Some([0, 0, 0, 255]),
            serif: false,
        }]);
        assert_eq!(output.documents.len(), 1);
        let mut scene = crate::app::scene::Scene::new();
        for document in output.documents {
            scene.add_file(document);
        }
        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(256, 256)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.show_grid = false;
        gpu.view.lit = false;
        scene.upload_to(&mut gpu);
        let mut camera = crate::camera::Camera::new();
        camera.set_view(crate::camera::View::Top);
        let bounds = AABB::from_points(
            &[Point::new(0.0, 0.0, 0.0), Point::new(200.0, 200.0, 0.0)],
            0.0,
        );
        camera.fit(&bounds, 1.0);
        let pixels = gpu.render_offscreen(&FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &Point::new(0.0, 0.0, 0.0)),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        });
        let ink = pixels
            .chunks_exact(4)
            .filter(|pixel| pixel[0] < 100 && pixel[1] < 100 && pixel[2] < 100)
            .count();
        assert!(ink > 30, "fraction rule has only {ink} dark pixels");
    }

    #[test]
    fn radicals_and_matrix_rows_are_geometry_and_glyphs_not_svg() {
        let radical = layout(&Parser::new(r"\sqrt{x_0}").parse(), Style::Text);
        assert_eq!(radical.lines.len(), 4);
        assert!(radical.runs.iter().all(|run| run.text != "√"));
        for pair in radical.lines.windows(2) {
            assert_eq!(pair[0].to, pair[1].from);
        }
        let vinculum = &radical.lines[3];
        assert_eq!(vinculum.from[1], vinculum.to[1]);
        assert!(vinculum.to[0] > vinculum.from[0]);
        let matrix = layout(
            &Parser::new(r"\begin{pmatrix}a&b\\c&d\end{pmatrix}").parse(),
            Style::Text,
        );
        assert!(matrix.runs.iter().any(|run| run.text == "("));
        assert!(matrix.runs.iter().any(|run| run.text == ")"));
        assert!(matrix.runs.iter().any(|run| run.baseline < 0.0));
    }

    #[test]
    fn billboard_math_uses_one_camera_facing_baseline() {
        let mut item = label("tag", false);
        item.text = "x_0".into();
        item.at = [4.0, 5.0, 6.0];
        item.right = [0.0, 1.0, 0.0];
        item.up = [0.0, 0.0, 1.0];
        item.camera_facing = true;
        item.spatial = true;
        item.height = 0.4;
        let output = typeset(&[item]);
        assert_eq!(output.texts.len(), 2);
        assert!(output.texts.iter().all(|text| text.at[0] == 4.0));
        assert!(output.texts.iter().any(|text| text.at[1] != 5.0));
        assert!(output.texts.iter().all(|text| text.at[2] != 6.0));
    }

    #[test]
    fn collision_bounds_measure_the_typeset_ink() {
        let mut item = label("ink", false);
        item.center = true;
        item.center_ink = true;
        item.at = [0.0, 0.0, 0.0];
        item.height = 1.0;
        let bounds = label_box(&item);
        assert!(bounds.left < 0.0 && bounds.right > 0.0);
        assert!(bounds.bottom < 0.0 && bounds.top > 0.0);
        assert!((bounds.left + bounds.right).abs() < 0.1);
        assert!((bounds.bottom + bounds.top).abs() < 0.1);
    }

    #[test]
    fn tex_colors_are_applied_to_individual_runs() {
        let mut item = label("colored", false);
        item.text = r"\color{#ef7770}{x_0}+\color{#62cc7c}{y_0}".into();
        let output = typeset(&[item]);
        assert!(
            output
                .texts
                .iter()
                .any(|text| { text.text == "𝑥" && text.color == Some([239, 119, 112, 255]) })
        );
        assert!(
            output
                .texts
                .iter()
                .any(|text| { text.text == "𝑦" && text.color == Some([98, 204, 124, 255]) })
        );
    }

    #[test]
    fn repeated_math_uses_cached_layout_and_glyph_metrics() {
        let first = cached_layout("x_0");
        let second = cached_layout("x_0");
        assert!(Rc::ptr_eq(&first, &second));
        let first = glyph_metrics("𝑥").unwrap();
        let second = glyph_metrics("𝑥").unwrap();
        assert_eq!(first.bounds, second.bounds);
        assert_eq!(first.texture_baseline, second.texture_baseline);
        assert!(GLYPH_METRICS.with_borrow(|cache| cache.contains_key("𝑥")));
    }

    #[test]
    fn math_italic_glyphs_exist_in_the_bundled_math_font() {
        let text = styled_text("xαΩ", FontStyle::Math);
        assert_ne!(text, "xαΩ");
        assert!(measure(&text, 1.0) > 0.0);
    }
}
