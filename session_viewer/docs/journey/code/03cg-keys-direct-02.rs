
pub(crate) fn suffix_space(ui: &egui::Ui, model: &mut CommandLine, id: egui::Id, focused: bool) {
    if model.inline_suffix
        && focused
        && model
            .command
            .chars()
            .nth(spelled(&model.command, &model.completion_prefix))
            != Some(' ')
        && ui.input(|input| {
            input
                .events
                .iter()
                .find_map(|event| match event {
                    egui::Event::Text(text) => Some(text),
                    _ => None,
                })
                .is_some_and(|text| text.starts_with(' '))
        })
    {
        command_cursor_end(ui.ctx(), id, &model.command);
        model.inline_suffix = false;
    }
}

pub(crate) fn keys(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    previous: Option<egui::Rect>,
    id: egui::Id,
) -> Keys {
    if model.focus_command || model.command_open {
        ui.memory_mut(|memory| memory.request_focus(id));
        if model.focus_command {
            command_cursor_end(ui.ctx(), id, &model.command);
            model.focus_command = false;
        }
        model.command_open = true;
    }
    let focused = ui.memory(|memory| memory.has_focus(id));
    let browse = if focused {
        ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                1
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                -1
            } else {
                0
            }
        })
    } else {
        0
    };
    let enter =
        focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    let tab =
        focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Tab));
    let deletes = ui.input(|input| {
        input.key_pressed(egui::Key::Backspace) || input.key_pressed(egui::Key::Delete)
    });
    suffix_space(ui, model, id, focused);
    Keys {
        browse,
        enter,
        tab,
        deletes,
    }
}

pub(crate) fn refresh(
