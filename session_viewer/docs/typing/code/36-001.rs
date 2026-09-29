
/// Completion and parsing ignore case.
#[test]
fn discovery_and_layer_options_are_case_insensitive() {
    assert_eq!(completions("la"), vec!["Layers"]);
    assert_eq!(completions("Layers "), vec!["Layers On", "Layers Off"]);
    assert!(completions("").contains(&"Controls"));
    assert_eq!(parsed("Layers OFF"), Ok("Layers(Some(false))".into()));
    assert_eq!(
        parsed("Element Features off"),
        Ok("ElementFeatures(Some(false))".into())
    );
    assert_eq!(
        parsed("Element Features"),
        Ok("ElementFeatures(None)".into())
    );
    assert_eq!(parsed("Opacity 0.5"), Ok("Opacity(0.5)".into()));
    assert!(parsed("Opacity 2").is_err());
    assert!(parsed("Opacity").is_err());
    assert!(parsed("Layers maybe").is_err());
}
