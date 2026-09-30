
/// Text waits for its words and keeps them, spaced once.
#[test]
fn text_waits_for_its_words_and_keeps_them() {
    assert_eq!(accept("Tex"), ("Text ".into(), false));
    assert_eq!(accept("Text "), ("Text ".into(), false));
    assert_eq!(
        parsed("text Hello   world"),
        Ok("Text(\"Hello world\")".into())
    );
    assert_eq!(canonical("text Hello   World"), "Text Hello World");
    assert!(parsed("text").is_err());
    assert!(parsed(&format!("text {}", "x".repeat(81))).is_err());
    assert!(!hint("text x").is_empty());
}

/// Project To Plane offers its five planes, CPlane first.
#[test]
fn project_to_plane_offers_five_planes() {
    assert_eq!(
        completions("Project To Plane "),
        vec![
            "Project To Plane CPlane",
            "Project To Plane XY",
            "Project To Plane YZ",
            "Project To Plane ZX",
            "Project To Plane 3Point",
        ]
    );
    assert_eq!(accept("projectt"), ("Project To Plane ".into(), false));
    assert_eq!(
        accept("Project To Plane "),
        ("Project To Plane CPlane".into(), true)
    );
    assert_eq!(canonical("projecttoplane xy"), "Project To Plane XY");
    assert!(parsed("project to plane xz").unwrap().contains("ZX"));
    assert!(parsed("Project To Plane 3Point 0,0,0 1,0,0 0,1,0").is_ok());

    for line in [
        "Project To Plane 3Point 0,0,0 1,0,0",
        "Project To Plane sideways",
        "Project To Plane XY extra",
    ] {
        assert!(parsed(line).is_err(), "{line}");
    }
}
