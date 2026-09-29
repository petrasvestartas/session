
#[cfg(test)]
mod commands_tests {
    use super::tests::label;
    use super::*;

    /// The bundled subsets draw the viewer's own strings and the specimen; the whole fonts the rest.
    #[test]
    fn bundled_subsets_cover_the_viewer_and_whole_fonts_the_rest() {
        for spec in crate::app::command::REGISTRY.iter().map(|verb| verb.spec()) {
            for text in spec.names.iter().chain(spec.aliases).chain(spec.options) {
                assert!(covers(text), "{text}");
            }

            assert!(covers(spec.hint), "{}", spec.hint);
        }

        for text in [
            "Ąą Čč Ęę Ėė Įį Šš Ųų Ūū Žž",
            "Äußere Wände, Größe, Straße ẞ, „Zitat“ »Guillemets«",
            "Ø 12 ± 0,5 mm, 45°, m² m³, µm",
            "→ ■ ⏵ ◻ ⚙ ⏳ ⌘",
            "+ –", // the panels' fold buttons
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
