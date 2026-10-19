    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn label(text: &str) -> TextLabel {
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

    fn has_glyph(glyph: &GlyphDiagnostic) -> bool {
        glyph.glyph != 0
    }

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

#[cfg(test)]
mod commands_tests {
    use super::tests::label;
    use super::*;

    #[test]
    fn bundled_subsets_cover_the_viewer_and_whole_fonts_the_rest() {
        for name in crate::commands::names() {assert!(covers(name),"{name}");}

        for text in [
            "Ąą Čč Ęę Ėė Įį Šš Ųų Ūū Žž",
            "Äußere Wände, Größe, Straße ẞ, „Zitat“ »Guillemets«",
            "Ø 12 ± 0,5 mm, 45°, m² m³, µm",
            "→ ■ ⏵ ◻ ⚙ ⏳ ⌘",
            "+ –",
        ] {
            assert!(covers(text), "{text}");
        }

        assert!(!covers("Fixed Ω cube"));
        let whole: Vec<Vec<u8>> = FULL_FONTS
            .iter()
            .map(|name| std::fs::read(format!("assets/text/{name}")).unwrap())
            .collect();
        let mut doc = TextDocument::new();
        doc.replace_fonts(whole).unwrap();
        doc.set_labels(vec![label("Fixed Ω cube")]).unwrap();
        assert!(doc.diagnostics().iter().all(|glyph| glyph.glyph != 0));
    }
}
