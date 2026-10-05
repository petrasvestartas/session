use super::*;

/// What a parsed line becomes, for comparing in tests.
fn parsed(line: &str) -> Result<String, String> {
    parse(line).map(|action| format!("{action:?}"))
}

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

/// Short forms parse like the full verb.
#[test]
fn the_verbs_and_their_short_forms() {
    assert_eq!(parsed("delete"), Ok("Delete".into()));
    assert_eq!(parsed("del"), Ok("Delete".into()));
    assert_eq!(parsed("  UNDO "), Ok("Undo".into()));
    assert_eq!(parsed("fit"), Ok("Fit".into()));
}

/// Move accepts every coordinate form.
#[test]
fn move_reads_the_same_coordinates_as_the_rest_of_the_viewer() {
    assert_eq!(parsed("move 10 0 0"), Ok("Move([10.0, 0.0, 0.0])".into()));
    assert_eq!(parsed("m @0 5"), Ok("Move([0.0, 5.0, 0.0])".into()));
    let polar = offset(&["10<90"]).expect("a polar offset is an offset");
    assert!((polar[1] - 10.0).abs() < 1e-9, "90 degrees is +y");
}

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

/// Rotate carries its axis.
#[test]
fn rotation_names_its_axis() {
    assert_eq!(
        parsed("rotate z 45"),
        Ok("Rotate { axis: Z, degrees: 45.0 }".into())
    );
}

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
    assert_eq!(accept("projectt"), ("Project To Plane".into(), true));
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

/// One Enter on a partial name runs the verb, or opens its options when it needs one.
#[test]
fn one_enter_runs_a_partial_verb_or_opens_its_options() {
    assert_eq!(accept("Del"), ("Delete".into(), true));
    assert_eq!(accept("clo"), ("Close".into(), true));
    assert_eq!(accept(" clo"), ("Close".into(), true));
    assert_eq!(accept("cur"), ("Curve".into(), true));
    assert_eq!(accept("Sn"), ("Snap".into(), true));
    assert_eq!(accept("Snap ne"), ("Snap Near".into(), true));
    assert_eq!(accept("Snap pe"), ("Snap Perp".into(), true));
}

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

/// Add Group takes an optional name; Add Edge takes nothing.
#[test]
fn session_structure_verbs_parse() {
    assert_eq!(parsed("Add Group"), Ok("AddGroup(\"\")".into()));
    assert_eq!(
        parsed("addgroup North wall"),
        Ok("AddGroup(\"North wall\")".into())
    );
    assert!(parsed("Add Group a/b").is_err());
    assert_eq!(parsed("add edge"), Ok("AddEdge".into()));
    assert!(parsed("Add Edge x").is_err());
    assert_eq!(accept("addg"), ("Add Group".into(), true));
    assert_eq!(completions("add"), vec!["Add Edge", "Add Group"]);
    assert_eq!(canonical("addedge"), "Add Edge");
}

/// Tab completes the verb, then its option.
#[test]
fn partial_entries_accept_commands_then_options() {
    assert_eq!(completions("Element F"), vec!["Element Features"]);
    assert_eq!(accept("Element F"), ("Element Features".into(), true));
    assert_eq!(
        accept("Element Features "),
        ("Element Features On".into(), true)
    );
    assert_eq!(
        accept("Element Features of"),
        ("Element Features Off".into(), true)
    );
    assert_eq!(
        browse("Element Features Off"),
        vec!["Element Features On", "Element Features Off"]
    );
    assert_eq!(option_label("Element Features Off"), "Off");
    assert_eq!(accept("Lay"), ("Layers".into(), true));
    assert_eq!(accept("Layers "), ("Layers On".into(), true));
    assert_eq!(accept("Layers of"), ("Layers Off".into(), true));
    assert_eq!(accept("Lin"), ("Line".into(), true));
    assert_eq!(accept("Point"), ("Point".into(), true));
    assert_eq!(accept("Polyline"), ("Polyline".into(), true));
    assert_eq!(accept("Polyline rec"), ("Polyline Rectangle".into(), true));
    assert_eq!(accept("Polyline pol"), ("Polyline Polygon".into(), true));
    assert_eq!(accept("Rotate "), ("Rotate x ".into(), false));
    assert_eq!(accept("Rotate x 45"), ("Rotate x 45".into(), true));
    assert_eq!(accept(""), (String::new(), true));
    assert_eq!(browse("la")[0], "Layers");
    assert_eq!(browse("la").len(), completions("").len());
    assert_eq!(browse("forgot"), completions(""));
    assert_eq!(browse("Layers o"), vec!["Layers On", "Layers Off"]);
}

/// A several-word name parses with or without its spaces, in any case.
#[test]
fn several_word_names_ignore_case_and_spaces() {
    for line in [
        "Clipping Plane",
        "clipping plane",
        "ClippingPlane",
        "clippingplane",
        "CLIPPING PLANE",
        "  clipping   plane ",
    ] {
        assert_eq!(parsed(line), Ok("Pick(Normal)".into()), "{line}");
    }

    assert_eq!(parsed("clippingplane off"), Ok("Switch(false)".into()));
    assert_eq!(
        parsed("Clipping Plane Fill Solid"),
        Ok("Fill(Some(true))".into())
    );
    assert_eq!(
        parsed("elementfeatures on"),
        Ok("ElementFeatures(Some(true))".into())
    );
    assert_eq!(parsed("poly line"), parsed("Polyline"));
    assert_eq!(parsed("clipping"), Err("no command `clipping`".into()));
    assert_eq!(
        parsed("clipping_plane"),
        Err("no command `clipping_plane`".into())
    );
    assert!(
        parsed("Clipping Plane sideways").is_err(),
        "the words after the name are its options"
    );
}

/// Completion and one-Enter accept work on several-word names typed any way.
#[test]
fn several_word_names_complete() {
    assert_eq!(completions("clip"), vec!["Clipping Plane"]);
    assert_eq!(completions("clipping p"), vec!["Clipping Plane"]);
    assert_eq!(completions("ClippingP"), vec!["Clipping Plane"]);
    assert_eq!(completions("elementf"), vec!["Element Features"]);
    assert_eq!(accept("clip"), ("Clipping Plane".into(), true));
    assert_eq!(accept("clippingplane"), ("Clipping Plane".into(), true));
    assert_eq!(
        accept("clippingplane x"),
        ("Clipping Plane XY".into(), true)
    );
    assert_eq!(
        accept("clipping plane fi"),
        ("Clipping Plane Fill".into(), true)
    );
    assert_eq!(
        completions("clippingplane fill "),
        vec!["Clipping Plane Fill Hatch", "Clipping Plane Fill Solid"]
    );
    assert_eq!(accept("elementf"), ("Element Features".into(), true));
    assert_eq!(browse("clip")[0], "Clipping Plane");
    assert_eq!(browse("clippingplane o"), options("Clipping Plane"));
    assert_eq!(hint("clippingplane"), hint("Clipping Plane"));
    assert_eq!(option_label("Clipping Plane Fill Hatch"), "Fill Hatch");
    assert!(choosing_option("clippingplane "));
    assert!(!choosing_option("clipping"));
}

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

#[test]
fn view_commands_replace_viewport_shortcuts() {
    for name in ["Top", "Side", "Front", "Back", "Left", "Right", "Bottom", "Isometric", "Reset", "Perspective", "Orthographic", "Show Edges", "Hide Edges", "Show Lines", "Hide Lines", "Show Points", "Hide Points", "Outline", "Lighting", "Backfaces", "Xray", "Names", "Hide Selected", "Show All"] {
        assert!(parse(&format!("View {name}")).is_ok(), "{name}");
    }
    assert_eq!(parsed("view top"), Ok("Camera(Top)".into()));
    assert_eq!(parsed("View Side"), parsed("View Right"));
    assert_eq!(parsed("View Outline Off"), Ok("Show(Outline, Some(false))".into()));
    assert!(completions("View ").contains(&"View Show Edges"));
    for line in ["View", "View Top extra", "View Outline maybe", "View Point Size 0", "View Point Size NaN", "View Plane Size 0", "View Plane Size -5"] {
        assert!(parse(line).is_err(), "{line}");
    }
    assert!(parse("View Point Size 1.5").is_ok());
    assert_eq!(parsed("View Plane Size 250"), Ok("PlaneSize(250.0)".into()));
}

#[test]
fn repeat_restarts_tools_without_reusing_coordinates() {
    assert_eq!(repeat("m 10,0,0"), Some("Move".into()));
    assert_eq!(repeat("Rotate 0,0,0 90"), Some("Rotate".into()));
    assert_eq!(repeat("Line 0,0,0 10,0,0"), Some("Line".into()));
    assert_eq!(repeat("Polyline Rectangle 0,0,0 10,10,0"), Some("Polyline Rectangle".into()));
    assert_eq!(repeat("View Top"), Some("View Top".into()));
    assert_eq!(repeat("Opacity 0.95"), Some("Opacity 0.95".into()));
    for line in ["", "0,0,0", "@0,10,0", "5", "Close", "Escape", "unknown"] {
        assert_eq!(repeat(line), None, "{line}");
    }
}
