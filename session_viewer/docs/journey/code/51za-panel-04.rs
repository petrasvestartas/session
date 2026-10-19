        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            let number=self.number.number_prompt.is_some();
            if !self.context.egui_wants_keyboard_input() && !number { self.model.focus_command=true; }
            self.context.memory_mut(|memory|memory.request_focus(egui::Id::new(if number {"number-input"} else {"command-input"})));
            if !number {self.model.command_open=true;}
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
