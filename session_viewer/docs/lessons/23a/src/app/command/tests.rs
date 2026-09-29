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
