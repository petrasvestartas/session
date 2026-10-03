use crate::State;
use winit::keyboard::{Key, NamedKey};

/// Enter completes a command; Escape cancels it. Feature requests are typed commands.
pub fn run(state: &mut State, key: &Key<&str>) -> bool {
    match key {
        Key::Named(NamedKey::Escape) => state.escape(),
        Key::Named(NamedKey::Enter) => state.enter(),
        _ => return false,
    }
    true
}
