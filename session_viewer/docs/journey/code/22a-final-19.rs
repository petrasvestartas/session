use crate::shortcuts::{Shortcut, key};

#[test]
fn history_keys_respect_text_ownership_and_modifiers() {
    assert_eq!(key("z", true, false, false, false, false, true), Some(Shortcut::Undo));
    assert_eq!(key("Z", true, true, false, false, true, true), Some(Shortcut::Redo));
    assert_eq!(key("y", true, false, false, false, false, false), Some(Shortcut::Redo));
    assert_eq!(key("z", true, false, false, false, true, false), None);
    assert_eq!(key("z", false, false, false, false, false, true), None);
    assert_eq!(key("z", true, false, true, false, false, true), None);
    assert_eq!(key("z", true, false, false, true, false, true), None);
}

#[test]
fn f_fits_only_when_the_canvas_owns_the_letter() {
    assert_eq!(key("f", false, false, false, false, false, true), Some(Shortcut::Fit));
    assert_eq!(key("F", false, true, false, false, false, true), Some(Shortcut::Fit));
    assert_eq!(key("f", false, false, false, false, true, true), None);
    assert_eq!(key("f", true, false, false, false, false, true), None);
    assert_eq!(key("h", false, false, false, false, false, true), None);
    assert_eq!(key("s", false, false, false, false, false, true), None);
}
