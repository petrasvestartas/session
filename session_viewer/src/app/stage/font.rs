use std::cell::RefCell;
use std::collections::HashMap;
use ttf_parser::{Face, GlyphId};

/// Which face a glyph comes from.
pub type FontId = u8;
pub const MATH: FontId = 0;
pub const TEXT: FontId = 1;
pub const BOLD: FontId = 2;
pub const MONO: FontId = 3;

/// Font files beside the viewer, in `FontId` order.
pub const FILES: [&str; 4] = [
    "FiraMath-Regular.otf",
    "FiraSans-Regular.ttf",
    "FiraSans-SemiBold.ttf",
    "FiraMono-Regular.ttf",
];

/// MATH table constants in em.
#[derive(Clone, Copy, Default, Debug)]
pub struct Constants {
    pub script: f32,
    pub script_script: f32,
    pub delimited_min: f32,
    pub display_op_min: f32,
    pub axis: f32,
    pub accent_base: f32,
    pub sub_down: f32,
    pub sub_top_max: f32,
    pub sub_drop_min: f32,
    pub sup_up: f32,
    pub sup_up_cramped: f32,
    pub sup_bottom_min: f32,
    pub sup_drop_max: f32,
    pub sub_sup_gap: f32,
    pub sup_bottom_max_with_sub: f32,
    pub space_after_script: f32,
    pub upper_gap: f32,
    pub upper_rise: f32,
    pub lower_gap: f32,
    pub lower_drop: f32,
    pub num_up: f32,
    pub num_up_display: f32,
    pub den_down: f32,
    pub den_down_display: f32,
    pub num_gap: f32,
    pub num_gap_display: f32,
    pub rule: f32,
    pub den_gap: f32,
    pub den_gap_display: f32,
    pub overbar_gap: f32,
    pub overbar_rule: f32,
    pub overbar_extra: f32,
    pub underbar_gap: f32,
    pub underbar_rule: f32,
    pub underbar_extra: f32,
    pub radical_gap: f32,
    pub radical_gap_display: f32,
    pub radical_rule: f32,
    pub radical_extra: f32,
    pub stretch_gap_above: f32,
    pub stretch_gap_below: f32,
}

/// Ink box and advance of one glyph, em.
#[derive(Clone, Copy, Default, Debug)]
pub struct Metrics {
    pub advance: f32,
    pub left: f32,
    pub right: f32,
    pub height: f32,
    pub depth: f32,
    pub italic: f32,
}

/// One piece of a stretched glyph, em.
#[derive(Clone, Copy, Debug)]
pub struct Part {
    pub gid: u16,
    pub start: f32,
    pub end: f32,
    pub advance: f32,
    pub extender: bool,
}

/// Size variants and the assembly of a stretchy glyph.
#[derive(Clone, Debug, Default)]
pub struct Construction {
    pub variants: Vec<(u16, f32)>,
    pub parts: Vec<Part>,
}

/// One parsed face: the bytes stay alive for the whole page.
pub struct Font {
    pub data: &'static [u8],
    face: Face<'static>,
    upem: f32,
}

/// The three faces and the math constants.
pub struct Fonts {
    pub faces: Vec<Font>,
    pub c: Constants,
    pub overlap: f32,
    metrics: RefCell<HashMap<(FontId, u16), Metrics>>,
}

impl Font {
    fn new(bytes: Vec<u8>) -> Result<Self, String> {
        let data: &'static [u8] = Box::leak(bytes.into_boxed_slice());
        let face = Face::parse(data, 0).map_err(|error| format!("font: {error}"))?;
        let upem = face.units_per_em() as f32;
        Ok(Self { data, face, upem })
    }
}

impl Fonts {
    /// Parse the faces in `FILES` order; the first must carry a MATH table.
    pub fn new(files: Vec<Vec<u8>>) -> Result<Self, String> {
        let faces = files
            .into_iter()
            .map(Font::new)
            .collect::<Result<Vec<_>, _>>()?;
        let math = faces.first().ok_or("no math font")?;
        let table = math
            .face
            .tables()
            .math
            .ok_or("the math font has no MATH table")?;
        let consts = table.constants.ok_or("the MATH table has no constants")?;
        let upem = math.upem;
        let em = |value: ttf_parser::math::MathValue| value.value as f32 / upem;
        let c = Constants {
            script: consts.script_percent_scale_down() as f32 / 100.0,
            script_script: consts.script_script_percent_scale_down() as f32 / 100.0,
            delimited_min: consts.delimited_sub_formula_min_height() as f32 / upem,
            display_op_min: consts.display_operator_min_height() as f32 / upem,
            axis: em(consts.axis_height()),
            accent_base: em(consts.accent_base_height()),
            sub_down: em(consts.subscript_shift_down()),
            sub_top_max: em(consts.subscript_top_max()),
            sub_drop_min: em(consts.subscript_baseline_drop_min()),
            sup_up: em(consts.superscript_shift_up()),
            sup_up_cramped: em(consts.superscript_shift_up_cramped()),
            sup_bottom_min: em(consts.superscript_bottom_min()),
            sup_drop_max: em(consts.superscript_baseline_drop_max()),
            sub_sup_gap: em(consts.sub_superscript_gap_min()),
            sup_bottom_max_with_sub: em(consts.superscript_bottom_max_with_subscript()),
            space_after_script: em(consts.space_after_script()),
            upper_gap: em(consts.upper_limit_gap_min()),
            upper_rise: em(consts.upper_limit_baseline_rise_min()),
            lower_gap: em(consts.lower_limit_gap_min()),
            lower_drop: em(consts.lower_limit_baseline_drop_min()),
            num_up: em(consts.fraction_numerator_shift_up()),
            num_up_display: em(consts.fraction_numerator_display_style_shift_up()),
            den_down: em(consts.fraction_denominator_shift_down()),
            den_down_display: em(consts.fraction_denominator_display_style_shift_down()),
            num_gap: em(consts.fraction_numerator_gap_min()),
            num_gap_display: em(consts.fraction_num_display_style_gap_min()),
            rule: em(consts.fraction_rule_thickness()),
            den_gap: em(consts.fraction_denominator_gap_min()),
            den_gap_display: em(consts.fraction_denom_display_style_gap_min()),
            overbar_gap: em(consts.overbar_vertical_gap()),
            overbar_rule: em(consts.overbar_rule_thickness()),
            overbar_extra: em(consts.overbar_extra_ascender()),
            underbar_gap: em(consts.underbar_vertical_gap()),
            underbar_rule: em(consts.underbar_rule_thickness()),
            underbar_extra: em(consts.underbar_extra_descender()),
            radical_gap: em(consts.radical_vertical_gap()),
            radical_gap_display: em(consts.radical_display_style_vertical_gap()),
            radical_rule: em(consts.radical_rule_thickness()),
            radical_extra: em(consts.radical_extra_ascender()),
            stretch_gap_above: em(consts.stretch_stack_gap_above_min()),
            stretch_gap_below: em(consts.stretch_stack_gap_below_min()),
        };
        let overlap = table
            .variants
            .map_or(0.0, |variants| variants.min_connector_overlap as f32 / upem);
        Ok(Self {
            faces,
            c,
            overlap,
            metrics: RefCell::default(),
        })
    }

    /// The glyph for `c` in `font`, falling back to the math face.
    pub fn glyph(&self, font: FontId, c: char) -> Option<(FontId, u16)> {
        if let Some(face) = self.faces.get(font as usize)
            && let Some(id) = face.face.glyph_index(c)
        {
            return Some((font, id.0));
        }
        self.faces[MATH as usize]
            .face
            .glyph_index(c)
            .map(|id| (MATH, id.0))
    }

    /// Advance, ink box and italic correction of a glyph, em.
    pub fn metrics(&self, font: FontId, gid: u16) -> Metrics {
        if let Some(m) = self.metrics.borrow().get(&(font, gid)) {
            return *m;
        }
        let f = &self.faces[font as usize];
        let id = GlyphId(gid);
        let u = f.upem;
        let advance = f.face.glyph_hor_advance(id).unwrap_or(0) as f32 / u;
        let ink = f.face.glyph_bounding_box(id);
        let italic = if font == MATH {
            f.face
                .tables()
                .math
                .and_then(|t| t.glyph_info)
                .and_then(|g| g.italic_corrections)
                .and_then(|i| i.get(id))
                .map_or(0.0, |v| v.value as f32 / u)
        } else {
            0.0
        };
        let m = match ink {
            Some(b) => Metrics {
                advance,
                left: b.x_min as f32 / u,
                right: b.x_max as f32 / u,
                height: b.y_max as f32 / u,
                depth: -(b.y_min as f32) / u,
                italic,
            },
            None => Metrics {
                advance,
                italic,
                ..Metrics::default()
            },
        };
        self.metrics.borrow_mut().insert((font, gid), m);
        m
    }

    /// Top accent attachment of a math glyph, em; None uses the ink center.
    pub fn accent_attach(&self, gid: u16) -> Option<f32> {
        let f = &self.faces[MATH as usize];
        f.face
            .tables()
            .math
            .and_then(|t| t.glyph_info)
            .and_then(|g| g.top_accent_attachments)
            .and_then(|a| a.get(GlyphId(gid)))
            .map(|v| v.value as f32 / f.upem)
    }

    /// Size variants and assembly of a math glyph, vertical or horizontal.
    pub fn construction(&self, gid: u16, horizontal: bool) -> Construction {
        let f = &self.faces[MATH as usize];
        let u = f.upem;
        let Some(variants) = f.face.tables().math.and_then(|t| t.variants) else {
            return Construction::default();
        };
        let set = if horizontal {
            variants.horizontal_constructions
        } else {
            variants.vertical_constructions
        };
        let Some(c) = set.get(GlyphId(gid)) else {
            return Construction::default();
        };
        Construction {
            variants: c
                .variants
                .into_iter()
                .map(|v| (v.variant_glyph.0, v.advance_measurement as f32 / u))
                .collect(),
            parts: c
                .assembly
                .map(|a| {
                    a.parts
                        .into_iter()
                        .map(|p| Part {
                            gid: p.glyph_id.0,
                            start: p.start_connector_length as f32 / u,
                            end: p.end_connector_length as f32 / u,
                            advance: p.full_advance as f32 / u,
                            extender: p.part_flags.extender(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}
