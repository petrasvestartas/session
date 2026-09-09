//! Source-text layout, identity and physical placement. Fonts and shaped runs live here;
//! the GPU lane owns Glyphon's atlas and draw resources. PDF outlines remain mesh data.

use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap, fontdb};
use serde::Serialize;

pub const FONT_FAMILY: &str = "Noto Sans";
pub const FONT_BYTES: &[u8] = include_bytes!("../../assets/text/NotoSans-Regular.ttf");
pub const SYMBOL_BYTES: &[u8] = include_bytes!("../../assets/text/NotoSansSymbols-Regular.ttf");
pub const FALLBACK_BYTES: &[u8] = include_bytes!("../../assets/text/NotoSansSymbols2-Regular.ttf");
/// Bound a submitted text document independently of available GPU memory.
const MAX_TEXT_BYTES: usize = 256 * 1024;

/// Text origin, orientation and size policy; physical labels use scene depth.
#[derive(Clone, Debug, PartialEq)]
pub enum TextPlacement {
    Screen {
        left: f32,
        top: f32,
    },
    Anchor {
        world: [f64; 3],
        offset: [f32; 2],
    },
    /// Center a fixed-CSS label and black plate on a world anchor. This annotation overlays
    /// the scene, so a selected solid's interior bounds center cannot hide its own name.
    Nameplate {
        world: [f64; 3],
        /// Horizontal and vertical inset around the shaped line box, in CSS pixels.
        padding: [f32; 2],
        /// Round all corners to half the shorter plate side when true.
        rounded: bool,
    },
    /// Fixed world plane: top-left line origin, orthonormal right/up axes, and world em height.
    WorldPlane {
        world: [f64; 3],
        right: [f64; 3],
        up: [f64; 3],
        world_height: f64,
    },
    /// Camera-facing text whose em height is measured in scene world units.
    WorldBillboard {
        world: [f64; 3],
        world_height: f64,
    },
}

/// Authored text participates in ordinary object selection; annotations have no owner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextObject {
    pub row: u32,
    pub selected: bool,
}

/// One source label. Sizes, line height, offsets and optional clipping use CSS pixels.
#[derive(Clone, Debug, PartialEq)]
pub struct TextLabel {
    pub id: u32,
    pub object: Option<TextObject>,
    pub text: String,
    pub font_size: f32,
    pub line_height: f32,
    pub color: [u8; 4],
    pub placement: TextPlacement,
    /// Left, top, right, bottom in canvas CSS coordinates; never inferred from glyph bounds.
    pub clip: Option<[f32; 4]>,
}

/// Retained logical glyph coordinates, independent of camera motion and framebuffer scale.
pub struct TextRun {
    pub label: TextLabel,
    pub buffer: Buffer,
}

/// All explicit font resources and shaped runs for one viewer, without DOM/GPU ownership.
pub struct TextDocument {
    pub fonts: FontSystem,
    pub runs: Vec<TextRun>,
    pub revision: u64,
    pub font_revision: u64,
    pub shape_count: u64,
    /// Accumulated shaping time; the GPU lane reports combined preparation time separately.
    pub shaping_ms: f64,
}

impl TextDocument {
    /// Load bundled OFL fonts explicitly; never scan the native machine's font directories.
    pub fn new() -> Self {
        Self {
            fonts: bundled_fonts(),
            runs: Vec::new(),
            revision: 0,
            font_revision: 1,
            shape_count: 0,
            shaping_ms: 0.0,
        }
    }

    /// Validate the whole replacement first, then reuse unchanged source/style buffers by ID.
    /// Placement and selection-color edits do not invoke shaping.
    pub fn set_labels(&mut self, labels: Vec<TextLabel>) -> anyhow::Result<()> {
        let mut bytes = 0usize;
        let mut ids = std::collections::HashSet::new();
        for label in &labels {
            bytes = bytes.saturating_add(label.text.len());
            anyhow::ensure!(bytes <= MAX_TEXT_BYTES, "text document exceeds 256 KiB");
            anyhow::ensure!(ids.insert(label.id), "duplicate text label ID {}", label.id);
            validate_label(label)?;
        }
        if self.runs.len() == labels.len() {
            let mut unchanged = true;
            for (run, label) in self.runs.iter().zip(&labels) {
                unchanged &= run.label == *label;
            }
            if unchanged {
                return Ok(());
            }
        }
        let previous = std::mem::take(&mut self.runs);
        let mut previous_by_id = std::collections::HashMap::new();
        for run in previous {
            previous_by_id.insert(run.label.id, run);
        }
        for label in labels {
            let old = previous_by_id.remove(&label.id);
            let buffer = match old {
                Some(run) if same_layout(&run.label, &label) => run.buffer,
                _ => {
                    self.shape_count += 1;
                    let started = crate::engine::performance::now_ms();
                    let buffer = shape(&mut self.fonts, &label);
                    self.shaping_ms += crate::engine::performance::now_ms() - started;
                    buffer
                }
            };
            self.runs.push(TextRun { label, buffer });
        }
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    /// Replace explicitly supplied fonts atomically. Invalid data leaves existing text intact.
    /// The caller must supply the primary family and any desired fallback faces together.
    pub fn replace_fonts(&mut self, sources: Vec<Vec<u8>>) -> anyhow::Result<()> {
        let mut db = fontdb::Database::new();
        for bytes in sources {
            let before = db.faces().count();
            db.load_font_data(bytes);
            anyhow::ensure!(
                db.faces().count() > before,
                "font data contains no usable face"
            );
        }
        anyhow::ensure!(db.faces().count() > 0, "font set is empty");
        db.set_sans_serif_family(FONT_FAMILY);
        self.fonts = FontSystem::new_with_locale_and_db("en-US".into(), db);
        for run in &mut self.runs {
            let started = crate::engine::performance::now_ms();
            run.buffer = shape(&mut self.fonts, &run.label);
            self.shaping_ms += crate::engine::performance::now_ms() - started;
            self.shape_count += 1;
        }
        self.font_revision = self.font_revision.wrapping_add(1);
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    /// Drop source strings and shape buffers on scene replacement.
    pub fn clear(&mut self) {
        self.runs.clear();
        self.runs.shrink_to_fit();
        self.revision = self.revision.wrapping_add(1);
    }

    /// Export source clusters and logical metrics before rasterization for regression tools.
    pub fn diagnostics(&self) -> Vec<GlyphDiagnostic> {
        let mut out = Vec::new();
        for run in &self.runs {
            for line in run.buffer.layout_runs() {
                for glyph in line.glyphs {
                    out.push(GlyphDiagnostic {
                        label: run.label.id,
                        line: line.line_i,
                        cluster: [glyph.start, glyph.end],
                        glyph: glyph.glyph_id,
                        font: format!("{:?}", glyph.font_id),
                        origin: [glyph.x, glyph.y],
                        advance: glyph.w,
                        offset: [
                            glyph.x_offset * glyph.font_size,
                            glyph.y_offset * glyph.font_size,
                        ],
                        baseline: line.line_y,
                        line_width: line.line_w,
                    });
                }
            }
        }
        out
    }
}

impl Default for TextDocument {
    /// Construct the explicit bundled-font document.
    fn default() -> Self {
        Self::new()
    }
}

/// Shaper output, with byte clusters into the original line rather than character indices.
#[derive(Clone, Debug, Serialize)]
pub struct GlyphDiagnostic {
    pub label: u32,
    pub line: usize,
    pub cluster: [usize; 2],
    pub glyph: u16,
    pub font: String,
    pub origin: [f32; 2],
    pub advance: f32,
    pub offset: [f32; 2],
    pub baseline: f32,
    pub line_width: f32,
}

/// Build a predictable font database on both WASM and native test targets.
fn bundled_fonts() -> FontSystem {
    let mut db = fontdb::Database::new();
    db.load_font_data(FONT_BYTES.to_vec());
    db.load_font_data(FALLBACK_BYTES.to_vec());
    db.load_font_data(SYMBOL_BYTES.to_vec());
    db.set_sans_serif_family(FONT_FAMILY);
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}

/// Reject non-finite dimensions before they reach the rasterizer or integer clip conversion.
fn validate_label(label: &TextLabel) -> anyhow::Result<()> {
    anyhow::ensure!(
        label.font_size.is_finite() && (1.0..=256.0).contains(&label.font_size),
        "font size must be 1..256 CSS px"
    );
    anyhow::ensure!(
        label.line_height.is_finite()
            && label.line_height >= label.font_size
            && label.line_height <= 1024.0,
        "invalid text line height"
    );
    let valid_placement = match label.placement {
        TextPlacement::Screen { left, top } => left.is_finite() && top.is_finite(),
        TextPlacement::Anchor { world, offset } => {
            world.iter().all(finite_f64) && offset.iter().all(finite_f32)
        }
        TextPlacement::Nameplate { world, padding, .. } => {
            world.iter().all(finite_f64) && padding.iter().all(valid_padding)
        }
        TextPlacement::WorldPlane {
            world,
            right,
            up,
            world_height,
        } => {
            world.iter().all(finite_f64)
                && valid_plane_axes(right, up)
                && world_height.is_finite()
                && world_height > 0.0
        }
        TextPlacement::WorldBillboard {
            world,
            world_height,
        } => world.iter().all(finite_f64) && world_height.is_finite() && world_height > 0.0,
    };
    anyhow::ensure!(valid_placement, "invalid text placement");
    if let Some(c) = label.clip {
        anyhow::ensure!(
            c.iter().all(finite_f32) && c[2] >= c[0] && c[3] >= c[1],
            "invalid text clip rectangle"
        );
    }
    Ok(())
}

/// Reject degenerate or scaled plane axes so world em height has one unambiguous meaning.
fn valid_plane_axes(right: [f64; 3], up: [f64; 3]) -> bool {
    let mut right_length = 0.0;
    let mut up_length = 0.0;
    let mut dot = 0.0;
    for axis in 0..3 {
        right_length += right[axis] * right[axis];
        up_length += up[axis] * up[axis];
        dot += right[axis] * up[axis];
    }
    (right_length.sqrt() - 1.0).abs() <= 1e-6
        && (up_length.sqrt() - 1.0).abs() <= 1e-6
        && dot.abs() <= 1e-6
}

/// Iterator adapter for finite coordinate validation.
fn finite_f32(value: &f32) -> bool {
    value.is_finite()
}
/// Iterator adapter for finite world coordinate validation.
fn finite_f64(value: &f64) -> bool {
    value.is_finite()
}

/// Keep annotation padding finite and bounded independently of document text length.
fn valid_padding(value: &f32) -> bool {
    value.is_finite() && (0.0..=256.0).contains(value)
}

/// Only fields affecting glyph selection and layout participate in shaping invalidation.
fn same_layout(a: &TextLabel, b: &TextLabel) -> bool {
    a.text == b.text && a.font_size == b.font_size && a.line_height == b.line_height
}

/// Advanced shaping handles kerning, ligatures, Unicode clusters and fallback once.
fn shape(fonts: &mut FontSystem, label: &TextLabel) -> Buffer {
    let mut buffer = Buffer::new(fonts, Metrics::new(label.font_size, label.line_height));
    buffer.set_size(fonts, None, None);
    buffer.set_wrap(fonts, Wrap::None);
    buffer.set_text(
        fonts,
        &label.text,
        &Attrs::new()
            .family(Family::Name(FONT_FAMILY))
            .metadata(label.id as usize),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(fonts, false);
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Keep source-text tests on the same explicit screen placement and font metrics.
    fn label(text: &str) -> TextLabel {
        TextLabel {
            object: None,
            id: 1,
            text: text.into(),
            font_size: 16.0,
            line_height: 24.0,
            color: [255; 4],
            placement: TextPlacement::Screen {
                left: 0.0,
                top: 0.0,
            },
            clip: None,
        }
    }

    #[test]
    fn ligatures_accents_spaces_and_symbols_are_shaped() {
        let mut doc = TextDocument::new();
        doc.set_labels(vec![label(
            "office ffi fl ÄÖÜ ĄČĘĖĮŠŲŪŽ Ø ± 90° m² e\u{301}",
        )])
        .unwrap();
        let glyphs = doc.diagnostics();
        assert!(glyphs.iter().all(has_glyph));
        assert!(glyphs.iter().any(multi_byte_cluster));
        doc.set_labels(vec![label("AV")]).unwrap();
        let pair_width = doc.diagnostics()[0].line_width;
        doc.set_labels(vec![label("A V")]).unwrap();
        assert!(doc.diagnostics()[0].line_width > pair_width);
        doc.set_labels(vec![label("ffi")]).unwrap();
        assert!(
            doc.diagnostics().len() < 3,
            "font's ffi ligature must preserve a multi-character cluster"
        );
    }

    /// Reject the missing-glyph sentinel in shaped output.
    fn has_glyph(glyph: &GlyphDiagnostic) -> bool {
        glyph.glyph != 0
    }
    /// Detect source clusters spanning multiple UTF-8 bytes.
    fn multi_byte_cluster(glyph: &GlyphDiagnostic) -> bool {
        glyph.cluster[1] - glyph.cluster[0] > 1
    }

    #[test]
    fn placement_and_color_do_not_reshape_but_font_reload_does() {
        let mut doc = TextDocument::new();
        let mut item = label("AVATAR");
        doc.set_labels(vec![item.clone()]).unwrap();
        let before = doc.diagnostics()[0].line_width;
        item.color = [255, 255, 0, 255];
        item.placement = TextPlacement::Screen {
            left: 0.375,
            top: 1.25,
        };
        doc.set_labels(vec![item]).unwrap();
        assert_eq!(doc.shape_count, 1);
        assert_eq!(doc.diagnostics()[0].line_width, before);
        assert!(doc.replace_fonts(vec![vec![0; 8]]).is_err());
        assert_eq!(doc.shape_count, 1);
        doc.replace_fonts(vec![
            FONT_BYTES.to_vec(),
            FALLBACK_BYTES.to_vec(),
            SYMBOL_BYTES.to_vec(),
        ])
        .unwrap();
        assert_eq!(doc.shape_count, 2);
        assert_eq!(doc.diagnostics()[0].line_width, before);
    }

    #[test]
    fn composed_decomposed_accents_and_multiline_baselines_match() {
        let mut doc = TextDocument::new();
        doc.set_labels(vec![label("é\ne\u{301}")]).unwrap();
        let glyphs = doc.diagnostics();
        assert_eq!(glyphs.len(), 2);
        assert_eq!(glyphs[0].glyph, glyphs[1].glyph);
        assert_eq!(glyphs[0].advance, glyphs[1].advance);
        assert!((glyphs[1].baseline - glyphs[0].baseline - 24.0).abs() < 0.001);
    }

    #[test]
    fn bundled_fallback_covers_symbols_without_system_fonts() {
        let mut doc = TextDocument::new();
        doc.set_labels(vec![label("CAD ⚙ ⏳ ⌘")]).unwrap();
        let glyphs = doc.diagnostics();
        assert!(glyphs.iter().all(has_glyph));
        let mut faces = std::collections::HashSet::new();
        for glyph in glyphs {
            faces.insert(glyph.font);
        }
        assert!(
            faces.len() >= 2,
            "sample must exercise explicit font fallback"
        );
    }

    #[test]
    fn invalid_replacement_preserves_current_document() {
        let mut doc = TextDocument::new();
        doc.set_labels(vec![label("Keep")]).unwrap();
        let mut invalid = label("Reject");
        invalid.font_size = f32::NAN;
        assert!(doc.set_labels(vec![invalid]).is_err());
        assert_eq!(doc.runs[0].label.text, "Keep");
    }
}
