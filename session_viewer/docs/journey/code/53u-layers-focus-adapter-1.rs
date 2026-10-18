    if model.focus_command {
        ui.memory_mut(|memory| memory.request_focus(id));