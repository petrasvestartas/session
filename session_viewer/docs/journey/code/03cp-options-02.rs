        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        // a command with options reads as its prompt with the options in it; the field takes the answer
        let hint = if model.options.is_empty() {
            placeholder(&model.drawing_prompt, &model.status, model.command_expanded)
        } else {
            options(ui, model, controls, command);
            ""
        };
        let response = view::field(ui, &mut model.command, hint, 0.0, model.command_open);
        model.command_rect = Some(response.rect);
        record(controls, "command/input", "Command", &response);
        refresh(ui, model, &response, commands, id, keys.deletes);
