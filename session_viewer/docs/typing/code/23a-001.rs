
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
