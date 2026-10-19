    pub fn set(&mut self,prompt:Option<NumberPrompt>) {self.number_prompt=prompt;if self.number_prompt.is_none() {self.number_handle=None;self.number_rect=None;}}
}

#[cfg(any(test, target_arch = "wasm32"))]
pub fn draw(
    root: &mut egui::Ui,                 // the panel area
    model: &mut NumberBox,               // the panel state
    controls: &mut Option<Vec<crate::command_dock::Control>>, // placed controls to draw
) {
    let Some(prompt) = model.number_prompt.as_ref() else {
        model.number_handle = None;
        model.number_rect = None;
        return;
    };
    let id = egui::Id::new("number-input");
    let opened = model.number_handle != Some(prompt.handle);

    if opened {
        model.number_handle = Some(prompt.handle);
        model.number.clear();
        model.number_error.clear();
        root.memory_mut(|memory| memory.request_focus(id));
    }

    let focused = root.memory(|memory| memory.focused());

    if root.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        || focused.is_some_and(|other| other != id)
    {
        model.closed = true;
        return;
    }

    if focused.is_none() {
        root.memory_mut(|memory| memory.request_focus(id));
    }

    let enter = root.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    let area = egui::Area::new(egui::Id::new("number-box"))
        .order(egui::Order::Foreground)
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(prompt.at[0] + 12.0, prompt.at[1] - 12.0))
        .show(root.ctx(), |ui| {
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_gray(215)))
                .inner_margin(5)
                .show(ui, |ui| {
                    ui.style_mut().override_font_id = Some(egui::FontId::proportional(14.0));
                    ui.horizontal(|ui| {
                        ui.label(&prompt.title);
                        let edit = ui.add_sized(
                            [72.0, 22.0],
                            egui::TextEdit::singleline(&mut model.number)
                                .id(id)
                                .hint_text(prompt.hint)
                                .char_limit(64),
                        );
                        crate::command_dock::record(controls, "number/input", &prompt.title, &edit);
                        ui.label(prompt.unit);
                    });

                    if !model.number_error.is_empty() {
                        ui.colored_label(egui::Color32::from_rgb(170, 30, 30), &model.number_error);
                    }
                });
        });
    model.number_rect = Some(area.response.rect);

    if enter {
        model.typed = Some(model.number.clone());
    }
}
