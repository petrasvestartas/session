
/// Creation verbs take a leading option, then typed answers; anything else is refused.
#[test]
fn creation_verbs_parse_options_and_answers() {
    assert_eq!(
        parsed("Box Mesh 0,0,0 100 50 30"),
        Ok(r#"Start { shape: Box, option: 1, words: ["0,0,0", "100", "50", "30"] }"#.into())
    );
    assert_eq!(
        parsed("nurbscurvearc 3 points 0,0,0 10,0,0 5,5,0"),
        Ok(r#"Start { shape: Nurbs Curve Arc, option: 1, words: ["0,0,0", "10,0,0", "5,5,0"] }"#.into())
    );
    assert_eq!(
        parsed("sphere"),
        Ok("Start { shape: Sphere, option: 0, words: [] }".into())
    );
    assert!(parsed("Quad Sphere Brep").is_err());
    assert!(parsed("Sphere 0,0,0 wide").is_err());
    assert_eq!(accept("Bo"), ("Box".into(), true));
    assert_eq!(accept("Box "), ("Box Brep".into(), true));
    assert_eq!(accept("quad"), ("Quad Sphere".into(), true));
    assert_eq!(
        canonical("blockwithhole mesh 0,0,0"),
        "Block With Hole Mesh 0,0,0"
    );
    assert_eq!(
        canonical("nurbscurvearc 3 points"),
        "Nurbs Curve Arc 3 Points"
    );
}

/// Surface verbs take options and values typed after them; the old snake names still work.
#[test]
fn the_surface_verbs_parse_their_typed_forms() {
    for line in [
        "Loft",
        "Loft Closed",
        "Loft Open Closed",
        "Nurbs Surface Loft Straight",
        "nurbssurface_loft straight",
        "nurbsnurbs_network",
        "nurbssurfacenetwork",
        "Nurbs Surface 4 Points 0,0,0 10,0,0 10,10,3 0,10,0",
        "Nurbs Surface 4 Points",
        "Nurbs Surface Revolve 0,0,0 0,0,1 90",
        "Nurbs Surface Revolve",
        "Nurbs Surface Sweep1",
        "Nurbs Surface Sweep2",
        "Extrude 10",
        "Extrude 0,0,5",
        "Extrude Cap Off 10",
        "extrude cap on",
    ] {
        assert!(parsed(line).is_ok(), "{line}");
    }

    for line in [
        "Nurbs Surface 4 Points 0,0,0 1,0,0 1,1,0",
        "Nurbs Surface Revolve 0,0,0 0,0,1 400",
        "Extrude sideways",
        "Loft 5",
    ] {
        assert!(parsed(line).is_err(), "{line}");
    }

    assert_eq!(name_of("nurbsnurbs_network"), "Nurbs Surface Network");
    assert_eq!(
        canonical("nurbssurface_revolve 0,0,0 0,0,1"),
        "Nurbs Surface Revolve 0,0,0 0,0,1"
    );
    assert_eq!(
        completions("nurbs s"),
        vec![
            "Nurbs Surface 4 Points",
            "Nurbs Surface Loft",
            "Nurbs Surface Network",
            "Nurbs Surface Revolve",
            "Nurbs Surface Sweep1",
            "Nurbs Surface Sweep2",
        ]
    );
    assert_eq!(
        completions("Extrude cap "),
        vec!["Extrude Cap On", "Extrude Cap Off"]
    );
}
