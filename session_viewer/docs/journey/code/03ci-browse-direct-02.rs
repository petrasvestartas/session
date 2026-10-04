
pub(crate) fn browse(
    ui: &egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    commands: &impl Commands,
    response: &egui::Response,
    keys: &Keys,
    id: egui::Id,
) -> Option<&'static str> {
    let choices = commands.browse(&model.completion_prefix);
    let opening = !model.completion_visible;
    if keys.browse != 0 || keys.tab {
        model.completion_visible = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if !focused || !model.completion_visible || choices.is_empty() {
        return None;
    }
    model.completion = model.completion.min(choices.len() - 1);
    if commands.choosing_option(&model.completion_prefix)
        && let Some(index) = choices
            .iter()
            .position(|name| name.eq_ignore_ascii_case(model.command.trim()))
    {
        model.completion = index;
    }
    if keys.browse != 0 {
        model.completion = if opening && !commands.choosing_option(&model.completion_prefix) {
            if keys.browse < 0 {
                choices.len() - 1
            } else {
                0
            }
        } else {
            (model.completion as isize + keys.browse).rem_euclid(choices.len() as isize) as usize
        };
        model.command = choices[model.completion].into();
        let start = if model
            .command
            .to_ascii_lowercase()
            .starts_with(&model.completion_prefix.to_ascii_lowercase())
        {
            model.completion_prefix.chars().count()
        } else {
            0
        };
        command_cursor_select(ui.ctx(), id, start, model.command.chars().count());
        model.inline_suffix = true;
        ui.ctx().request_repaint();
    }
    let mut complete = if keys.tab {
        Some(choices[model.completion])
    } else {
        None
    };
    if !commands.choosing_option(&model.completion_prefix) {
        complete = popup(ui, model, controls, &choices, response, keys.browse).or(complete);
    }
    complete
}

pub(crate) fn finish(
