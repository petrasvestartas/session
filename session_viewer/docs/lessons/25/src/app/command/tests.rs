use super::*;

// --8<-- [start:23-test-parsed]
/// What a parsed line becomes, for comparing in tests.
fn parsed(line: &str) -> Result<String, String> {
    parse(line).map(|action| format!("{action:?}"))
}
// --8<-- [end:23-test-parsed]

// --8<-- [start:23-test-every_verb_is_offered_in_alphabetical_order]
/// Every registered name is offered once, alphabetically.
#[test]
fn every_verb_is_offered_in_alphabetical_order() {
    let offered = completions("");
    let mut names: Vec<_> = REGISTRY
        .iter()
        .flat_map(|verb| verb.spec().names)
        .copied()
        .collect();
    names.sort_by_key(|name| name.to_ascii_lowercase());
    assert_eq!(offered, names);
    // strictly increasing, so no name is offered twice
    assert!(
        offered
            .windows(2)
            .all(|pair| pair[0].to_ascii_lowercase() < pair[1].to_ascii_lowercase())
    );
}
// --8<-- [end:23-test-every_verb_is_offered_in_alphabetical_order]

// --8<-- [start:23-test-every_name_and_option_is_title_case_words]
/// Every shown name is Title Case words: no underscores, each word capitalised or a number.
#[test]
fn every_name_and_option_is_title_case_words() {
    for verb in REGISTRY {
        let spec = verb.spec();

        for name in spec.names.iter().chain(spec.options) {
            assert!(!name.contains('_'), "{name}");
            assert!(
                name.split_whitespace()
                    .next()
                    .is_some_and(|word| word.starts_with(|c: char| c.is_ascii_uppercase())),
                "{name}"
            );
        }

        for name in spec.names {
            assert!(
                name.split_whitespace().all(|word| {
                    word.starts_with(|c: char| c.is_ascii_uppercase() || c.is_ascii_digit())
                }),
                "{name}"
            );
        }
    }
}
// --8<-- [end:23-test-every_name_and_option_is_title_case_words]

// --8<-- [start:23-test-the_verbs_and_their_short_forms]
/// Short forms parse like the full verb.
#[test]
fn the_verbs_and_their_short_forms() {
    assert_eq!(parsed("delete"), Ok("Delete".into()));
    assert_eq!(parsed("del"), Ok("Delete".into()));
    assert_eq!(parsed("  UNDO "), Ok("Undo".into()));
    assert_eq!(parsed("fit"), Ok("Fit".into()));
}
// --8<-- [end:23-test-the_verbs_and_their_short_forms]

// --8<-- [start:23a-test-move_reads_the_same_coordinates_as_the_rest_of_the_viewer]
/// Move accepts every coordinate form.
#[test]
fn move_reads_the_same_coordinates_as_the_rest_of_the_viewer() {
    assert_eq!(parsed("move 10 0 0"), Ok("Move([10.0, 0.0, 0.0])".into()));
    assert_eq!(parsed("m @0 5"), Ok("Move([0.0, 5.0, 0.0])".into()));
    let polar = offset(&["10<90"]).expect("a polar offset is an offset");
    assert!((polar[1] - 10.0).abs() < 1e-9, "90 degrees is +y");
}
// --8<-- [end:23a-test-move_reads_the_same_coordinates_as_the_rest_of_the_viewer]

// --8<-- [start:23a-test-a_line_it_cannot_do_says_so]
/// A bad line gets a message naming the problem.
#[test]
fn a_line_it_cannot_do_says_so() {
    assert_eq!(parsed("fly 3"), Err("no command `fly`".into()));
    assert_eq!(parsed(""), Err("nothing typed".into()));
    assert!(
        parsed("scale 0").is_err(),
        "a zero scale collapses the object"
    );
    assert!(parsed("rotate 90").is_err(), "no axis");
    assert!(parsed("rotate x").is_err(), "no angle");
    assert!(parsed("move sideways").is_err());
}
// --8<-- [end:23a-test-a_line_it_cannot_do_says_so]

// --8<-- [start:23a-test-modeling_commands_validate_arity_and_coordinates]
/// Modeling verbs check their point counts.
#[test]
fn modeling_commands_validate_arity_and_coordinates() {
    assert_eq!(
        parsed("point 1,2,3"),
        Ok("Create(Point, [[1.0, 2.0, 3.0]])".into())
    );
    assert_eq!(
        parsed("arrow 0,0,0 1,0,0"),
        Ok("Create(Arrow, [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]])".into())
    );
    assert_eq!(parsed("trim 0.2 0.8"), Ok("Edit(Trim(0.2, 0.8))".into()));
    assert_eq!(parsed("explode"), Ok("Explode".into()));

    for line in [
        "point @1,2,3",
        "line 0,0,0",
        "arrow 0,0,0",
        "trim 0 1 extra",
        "explode extra",
        "scale 2 extra",
        "delete extra",
    ] {
        assert!(parsed(line).is_err(), "{line}");
    }

    assert!(parsed(&"x".repeat(65537)).is_err());
}
// --8<-- [end:23a-test-modeling_commands_validate_arity_and_coordinates]

// --8<-- [start:23a-test-rotation_names_its_axis]
/// Rotate carries its axis.
#[test]
fn rotation_names_its_axis() {
    assert_eq!(
        parsed("rotate z 45"),
        Ok("Rotate { axis: Z, degrees: 45.0 }".into())
    );
}
// --8<-- [end:23a-test-rotation_names_its_axis]

// --8<-- [start:23b-test-bare_transform_verbs_start_picking]
/// A bare transform verb starts picking points; typed values still act at once.
#[test]
fn bare_transform_verbs_start_picking() {
    for (line, tool) in [
        ("move", "Moving"),
        ("rotate", "Rotating"),
        ("scale", "Scaling { mode: 3 }"),
        ("copy", "Copying"),
        ("Orient 3 Points", "Orienting"),
        ("orient3points", "Orienting"),
        ("orient3pt", "Orienting"),
    ] {
        assert_eq!(parsed(line), Ok(tool.into()), "{line}");
    }

    assert_eq!(parsed("move 10 0 0"), Ok("Move([10.0, 0.0, 0.0])".into()));
    assert_eq!(parsed("m @0 5"), Ok("Move([0.0, 5.0, 0.0])".into()));
    assert_eq!(
        parsed("rotate z 45"),
        Ok("Rotate { axis: Z, degrees: 45.0 }".into())
    );
    assert_eq!(parsed("scale 2"), Ok("Scale(2.0)".into()));
    assert_eq!(parsed("copy 0,5,0"), Ok("Copy([0.0, 5.0, 0.0])".into()));
    assert!(parsed("orient3pt 0,0,0 1,0,0 0,1,0 5,5,0 5,6,0 4,5,0").is_ok());

    for line in [
        "scale 0",
        "rotate 90",
        "rotate x",
        "scale 2 extra",
        "rotate z 45 extra",
        "orient3pt 0,0,0",
    ] {
        assert!(parsed(line).is_err(), "{line}");
    }

    assert_eq!(accept("Rot"), ("Rotate".into(), true));
    assert_eq!(accept("Rotate "), ("Rotate x ".into(), false));
    assert_eq!(accept("ori"), ("Orient 3 Points".into(), true));
    assert_eq!(completions("Co"), vec!["Cone", "Controls", "Copy"]);
    assert_eq!(name_of("m 10 0 0"), "Move");
}
// --8<-- [end:23b-test-bare_transform_verbs_start_picking]

// --8<-- [start:23c-test-creation_verbs_parse_options_and_answers]
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
// --8<-- [end:23c-test-creation_verbs_parse_options_and_answers]

// --8<-- [start:23c-test-the_surface_verbs_parse_their_typed_forms]
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
// --8<-- [end:23c-test-the_surface_verbs_parse_their_typed_forms]

// --8<-- [start:23d-test-text_waits_for_its_words_and_keeps_them]
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
// --8<-- [end:23d-test-text_waits_for_its_words_and_keeps_them]

// --8<-- [start:23d-test-project_to_plane_offers_five_planes]
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
// --8<-- [end:23d-test-project_to_plane_offers_five_planes]

// --8<-- [start:24-test-one_enter_runs_a_partial_verb_or_opens_its_options]
/// One Enter on a partial name runs the verb, or opens its options when it needs one.
#[test]
fn one_enter_runs_a_partial_verb_or_opens_its_options() {
    assert_eq!(accept("Del"), ("Delete".into(), true));
    assert_eq!(accept("clo"), ("Close".into(), true));
    assert_eq!(accept(" clo"), ("Close".into(), true));
    assert_eq!(accept("cur"), ("Curve".into(), true));
    assert_eq!(accept("Sn"), ("Snap ".into(), false));
    assert_eq!(accept("Snap ne"), ("Snap Near".into(), true));
    assert_eq!(accept("Snap pe"), ("Snap Perp".into(), true));
}
// --8<-- [end:24-test-one_enter_runs_a_partial_verb_or_opens_its_options]

// --8<-- [start:24-test-snap_takes_a_switch_or_a_kind]
/// Snap turns snapping on or off, or toggles one kind.
#[test]
fn snap_takes_a_switch_or_a_kind() {
    assert_eq!(
        parsed("Snap Off"),
        Ok("Snap { on: Some(false), mode: 0 }".into())
    );
    assert_eq!(parsed("snap"), Ok("Snap { on: None, mode: 0 }".into()));
    assert_eq!(parsed("Snap near"), Ok("Snap { on: None, mode: 2 }".into()));
    assert_eq!(
        parsed("Snap Center"),
        Ok("Snap { on: None, mode: 8 }".into())
    );
    assert!(parsed("Snap sideways").is_err());
    assert_eq!(parsed("Close"), Ok("Close".into()));
    assert!(parsed("Close 3").is_err());
}
// --8<-- [end:24-test-snap_takes_a_switch_or_a_kind]

// --8<-- [start:24-test-canonical_spells_the_command_as_shown]
/// The shown spelling replaces what was typed, for history lines.
#[test]
fn canonical_spells_the_command_as_shown() {
    assert_eq!(
        canonical("clippingplane xy 0,0,1"),
        "Clipping Plane XY 0,0,1"
    );
    assert_eq!(
        canonical("clipping plane fill hatch"),
        "Clipping Plane Fill Hatch"
    );
    assert_eq!(canonical("  snap   near "), "Snap Near");
    assert_eq!(canonical("m 10 0 0"), "Move 10 0 0");
    assert_eq!(canonical("0,0,0"), "0,0,0");
    assert_eq!(canonical(""), "");
}
// --8<-- [end:24-test-canonical_spells_the_command_as_shown]
