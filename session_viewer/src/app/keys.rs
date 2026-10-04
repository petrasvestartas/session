use crate::State;
use winit::keyboard::{Key, NamedKey};

/// Enter completes a command; Escape cancels it; Ctrl+Z and Ctrl+Y step the history. Feature requests are typed commands.
pub fn run(state: &mut State, key: &Key<&str>, shortcut: bool, shift: bool) -> bool {
    match key {
        Key::Named(NamedKey::Escape) => state.escape(),
        Key::Named(NamedKey::Enter) => state.enter(),
        _ => match history(key, shortcut, shift) {
            Some(line) => state.run_echoed(line),
            None => return false,
        },
    }
    true
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
}
