        ui.ctx().request_repaint();
    }
}

pub(crate) fn row(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
    commands: &impl Commands,
    previous: Option<egui::Rect>,
    escape_allowed: bool,
) -> bool {
    let mut focus_canvas = false;
    ui.horizontal(|ui| {
        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        let response = view::field(
            ui,
            &mut model.command,
            placeholder(&model.drawing_prompt, &model.status, model.command_expanded),
            0.0,
            model.command_open,
        );
        model.command_rect = Some(response.rect);
        record(controls, "command/input", "Command", &response);
        refresh(ui, model, &response, commands, id, keys.deletes);
        let complete = browse(ui, model, controls, commands, &response, &keys, id);
        focus_canvas = finish(
            ui,
            model,
            &response,
            command,
            commands,
            complete,
            &keys,
            escape_allowed,
        );
        collapse(ui, model, controls);
    });
    focus_canvas
}

pub fn draw(
    root: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
    commands: &impl Commands,
    escape_allowed: bool,
) -> bool {
    let previous = model.completion_rect.take();
    let mut focus_canvas = false;
    let panel = view::panel(root, model.command_expanded, 0.0).show_inside(root, |ui| {
        view::prepare(ui);
        history(ui, model, controls);
        focus_canvas = row(
            ui,
            model,
            controls,
            command,
            commands,
            previous,
            escape_allowed,
        );
    });
    if let Some(controls) = controls {
        let rect = panel.response.rect;
        controls.push(Control {
            key: "command/resize".into(),
            label: "Drag to resize command history".into(),
            rect: [
                rect.left(),
                rect.top() - 4.0,
                rect.right(),
                rect.top() + 4.0,
            ],
        });
    }
    focus_canvas
}
