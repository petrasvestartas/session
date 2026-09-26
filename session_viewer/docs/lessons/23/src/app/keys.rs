use crate::State;
use crate::camera::View;
use winit::keyboard::{Key, NamedKey};

/// What a key press runs, and what must be held with it.
pub struct Binding {
    pub trigger: Trigger,    // the key itself
    pub ctrl: bool,          // Ctrl must be held
    pub shift: Option<bool>, // Shift must be held, or must not
    pub run: fn(&mut State), // what it does
}

/// The key a binding answers to.
pub enum Trigger {
    Chars(&'static [&'static str]), // typed characters, upper and lower
    Named(NamedKey),                // a named key such as Enter
}

impl Binding {
    /// True when this press, with these modifiers, is the one.
    fn matches(&self, key: &Key<&str>, ctrl: bool, shift: bool) -> bool {
        if (self.ctrl && !ctrl) || self.shift.is_some_and(|want| want != shift) {
            return false;
        }

        match (&self.trigger, key) {
            (Trigger::Named(named), Key::Named(pressed)) => named == pressed,
            (Trigger::Chars(chars), Key::Character(typed)) => chars.iter().any(|c| c == typed),
            _ => false,
        }
    }
}

/// The binding for one press, if any key claims it.
pub fn binding(key: &Key<&str>, ctrl: bool, shift: bool) -> Option<&'static Binding> {
    KEYS.iter()
        .find(|binding| binding.matches(key, ctrl, shift))
}

/// A press with no modifier requirement.
const fn plain(chars: &'static [&'static str], run: fn(&mut State)) -> Binding {
    Binding {
        trigger: Trigger::Chars(chars),
        ctrl: false,
        shift: None,
        run,
    }
}

/// A press of a named key, such as Enter.
const fn named(key: NamedKey, run: fn(&mut State)) -> Binding {
    Binding {
        trigger: Trigger::Named(key),
        ctrl: false,
        shift: None,
        run,
    }
}

/// Every keyboard shortcut, first match wins.
pub const KEYS: &[Binding] = &[
    // register:projection
    named(NamedKey::Space, |s| {
        s.refresh_bounds();
        s.camera.toggle_projection_framed(&s.gpu.bounds, s.aspect())
    }),
    // the first Esc cancels the command and keeps the selection, the next one clears it
    named(NamedKey::Escape, |s| s.escape()),
    named(NamedKey::F10, |s| s.enable_controls()), // register:controls
    named(NamedKey::Delete, |s| s.delete_selected()), // register:delete
    plain(&[":"], |_| crate::app::feedback::command_line(true)), // register:command-line
    ctrl(&["z", "Z"], Some(true), |s| s.redo()), // register:redo-shift
    ctrl(&["z", "Z"], Some(false), |s| s.undo()), // register:undo
    ctrl(&["y", "Y"], None, |s| s.redo()), // register:redo
    // register:view-front
    plain(&["1"], |s| s.camera.set_view(View::Front)),
    // register:view-back
    plain(&["2"], |s| s.camera.set_view(View::Back)),
    // register:view-left
    plain(&["3"], |s| s.camera.set_view(View::Left)),
    // register:view-right
    plain(&["4"], |s| s.camera.set_view(View::Right)),
    // register:view-top
    plain(&["5"], |s| s.camera.set_view(View::Top)),
    // register:view-bottom
    plain(&["6"], |s| s.camera.set_view(View::Bottom)),
    // register:view-iso
    plain(&["7"], |s| s.camera.set_view(View::Iso)),
    // register:camera-reset
    plain(&["c", "C"], |s| s.camera.reset()),
    // register:fit
    plain(&["f", "F"], |s| s.fit_selected_or_all()),
    // register:show-points
    plain(&["q", "Q"], |s| {
        s.gpu.view.show_points = !s.gpu.view.show_points
    }),
    // register:show-lines
    plain(&["w", "W"], |s| {
        s.gpu.view.show_lines = !s.gpu.view.show_lines
    }),
    // register:show-mesh-edges
    plain(&["e", "E"], |s| {
        s.gpu.view.show_mesh_edges = !s.gpu.view.show_mesh_edges
    }),
    // register:show-outlines
    plain(&["o", "O"], |s| {
        s.gpu.view.show_outlines = !s.gpu.view.show_outlines
    }),
    // register:ssao
    plain(&["g", "G"], |s| s.gpu.view.set_arctic(!s.gpu.view.ssao)),
    // register:lit
    plain(&["d", "D"], |s| s.gpu.view.lit = !s.gpu.view.lit),
    // register:hide
    plain(&["h", "H"], |s| s.hide_selected()),
    // register:show-all
    plain(&["s", "S"], |s| s.show_all()),
    // register:names
    plain(&["t", "T"], |s| s.toggle_selected_names()),
    // register:backface
    plain(&["b", "B"], |s| s.gpu.view.backface = !s.gpu.view.backface),
    // register:xray
    plain(&["p", "P"], |s| s.toggle_xray()),
    // register:cloud-smaller
    plain(&["["], |s| s.set_cloud_size(s.gpu.view.cloud_size - 0.25)),
    // register:cloud-bigger
    plain(&["]"], |s| s.set_cloud_size(s.gpu.view.cloud_size + 0.25)),
];

// --8<-- [start:21]
/// A press with Ctrl held, and Shift held, not held or either.
const fn ctrl(chars: &'static [&'static str], shift: Option<bool>, run: fn(&mut State)) -> Binding {
    Binding {
        trigger: Trigger::Chars(chars),
        ctrl: true,
        shift,
        run,
    }
}
// --8<-- [end:21]
