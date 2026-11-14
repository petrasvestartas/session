#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shortcut { Undo, Redo, Fit }

impl Shortcut {
    pub fn command(self) -> &'static str {
        match self { Self::Undo => "Undo", Self::Redo => "Redo", Self::Fit => "Fit" }
    }
}

pub fn key(text: &str, command: bool, shift: bool, alt: bool, composing: bool,
    focused: bool, text_empty: bool) -> Option<Shortcut> {
    if alt || composing { return None; }
    if command {
        if focused && !text_empty { return None; }
        match text.to_ascii_lowercase().as_str() {
            "z" if shift => Some(Shortcut::Redo),
            "z" => Some(Shortcut::Undo),
            "y" => Some(Shortcut::Redo),
            _ => None,
        }
    } else if !focused && text.eq_ignore_ascii_case("f") { Some(Shortcut::Fit) }
    else { None }
}
