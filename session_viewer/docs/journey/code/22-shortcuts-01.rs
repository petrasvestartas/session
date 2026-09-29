use crate::editor::Action;

pub fn wheel(delta: f64, mode: u32, height: f64) -> Option<Action> {
    if !delta.is_finite() || delta == 0.0 || !height.is_finite() || height <= 0.0 {
        return None;
    }
    let unit = match mode {
        0 => 1.0,
        1 => 16.0,
        2 => height,
        _ => return None,
    };
    let pixels = (delta * unit).clamp(-600.0, 600.0);
    Some(Action::Zoom((-0.002 * pixels).exp() as f32))
}

pub fn key(name: &str, primary: bool, shift: bool, repeat: bool) -> Option<Action> {
    if primary {
        if repeat { return None; }
        return match name {
            "z" | "Z" if shift => Some(Action::Redo),
            "z" | "Z" => Some(Action::Undo),
            "y" | "Y" if !shift => Some(Action::Redo),
            _ => None,
        };
    }
    match name {
        "+" | "=" => Some(Action::Zoom(1.1)),
        "-" => Some(Action::Zoom(1.0 / 1.1)),
        "ArrowLeft" => Some(Action::Pan(-0.15, 0.0)),
        "ArrowRight" => Some(Action::Pan(0.15, 0.0)),
        "ArrowUp" => Some(Action::Pan(0.0, 0.15)),
        "ArrowDown" => Some(Action::Pan(0.0, -0.15)),
        "Home" if !repeat => Some(Action::ResetView),
        "Delete" if !repeat => Some(Action::Delete),
        _ => None,
    }
}
