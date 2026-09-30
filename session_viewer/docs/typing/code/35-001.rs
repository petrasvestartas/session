
/// Tab completes the verb, then its option.
#[test]
fn partial_entries_accept_commands_then_options() {
    assert_eq!(completions("Element F"), vec!["Element Features"]);
    assert_eq!(accept("Element F"), ("Element Features ".into(), false));
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
    assert_eq!(accept("Lay"), ("Layers ".into(), false));
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
    assert_eq!(accept("elementf"), ("Element Features ".into(), false));
    assert_eq!(browse("clip")[0], "Clipping Plane");
    assert_eq!(browse("clippingplane o"), options("Clipping Plane"));
    assert_eq!(hint("clippingplane"), hint("Clipping Plane"));
    assert_eq!(option_label("Clipping Plane Fill Hatch"), "Fill Hatch");
    assert!(choosing_option("clippingplane "));
    assert!(!choosing_option("clipping"));
}
