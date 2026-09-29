
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
