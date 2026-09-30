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
