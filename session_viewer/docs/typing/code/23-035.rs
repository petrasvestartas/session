
/// Open or close the command line.
#[cfg(target_arch = "wasm32")]
pub fn command_line(open: bool) {
    super::ui::command_line::STATE.with_borrow_mut(|model| {
        model.command_open = open;
        model.focus_command = open;

        if open {
            model.command.clear();
        }
    });
}

/// No command line on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn command_line(_open: bool) {}

/// Raise the phone keyboard over an empty field; works while a tap is handled.
#[cfg(target_arch = "wasm32")]
pub fn raise_keyboard() {
    super::agent::raise();
}

/// No phone keyboard on native.
#[cfg(not(target_arch = "wasm32"))]
pub fn raise_keyboard() {}
