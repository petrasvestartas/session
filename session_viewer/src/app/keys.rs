use crate::State;
use winit::keyboard::{Key, NamedKey};

/// Enter completes a command; Escape cancels it; Ctrl+Z and Ctrl+Y step the history; F fits, H hides, S shows. Feature requests are typed commands.
pub fn run(state: &mut State, key: &Key<&str>, shortcut: bool, shift: bool) -> bool {
    let typed = match key {
        Key::Character(text) if !shortcut => letter(text),
        _ => None,
    };
    match key {
        Key::Named(NamedKey::Escape) => state.escape(),
        Key::Named(NamedKey::Enter) => state.enter(),
        _ => match history(key, shortcut, shift).or(typed) {
            Some(line) => state.run_echoed(line),
            None => return false,
        },
    }
    true
}

/// The command a letter typed on the canvas runs instead of opening the command line: F fits the selection, or everything; H hides the selection; S shows everything.
pub fn letter(text: &str) -> Option<&'static str> {
    match text {
        "f" | "F" => Some("Fit"),
        "h" | "H" => Some("Hide"),
        "s" | "S" => Some("Show"),
        _ => None,
    }
}

/// The history command a key runs: Ctrl+Z undoes, Ctrl+Y and Ctrl+Shift+Z redo; Cmd counts as Ctrl.
pub fn history(key: &Key<&str>, shortcut: bool, shift: bool) -> Option<&'static str> {
    let Key::Character(text) = key else {
        return None;
    };
    if !shortcut {
        return None;
    }

    match text.to_lowercase().as_str() {
        "z" if shift => Some("Redo"),
        "z" => Some("Undo"),
        "y" => Some("Redo"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::history;
    use winit::keyboard::Key;

    #[test]
    fn undo_and_redo_shortcuts() {
        assert_eq!(history(&Key::Character("z"), true, false), Some("Undo"));
        assert_eq!(history(&Key::Character("Z"), true, true), Some("Redo"));
        assert_eq!(history(&Key::Character("y"), true, false), Some("Redo"));
        assert_eq!(history(&Key::Character("z"), false, false), None);
        assert_eq!(history(&Key::Character("x"), true, false), None);
    }

    #[test]
    fn f_fits_h_hides_s_shows() {
        assert_eq!(super::letter("f"), Some("Fit"));
        assert_eq!(super::letter("F"), Some("Fit"));
        assert_eq!(super::letter("h"), Some("Hide"));
        assert_eq!(super::letter("S"), Some("Show"));
        assert_eq!(super::letter("g"), None);
    }
}
