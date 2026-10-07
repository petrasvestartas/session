        view::prepare(ui);
        model.dock_top = Some(ui.max_rect().top());
        history(ui, model, controls);
