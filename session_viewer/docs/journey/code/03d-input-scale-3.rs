            focused: true,
            ..Default::default()
        };
        self.consumed = false;
        if let Some(event) = event {
            if let Some(mouse) = event.dyn_ref::<web_sys::MouseEvent>() {
                let pos = egui::pos2(
                    (mouse.client_x() as f64 - rect.left()) as f32,
                    (mouse.client_y() as f64 - rect.top()) as f32,
                );
                let over = pos.y >= self.top
                    || self.model.completion_rect.is_some_and(|r| r.contains(pos));
                self.consumed = over || self.pointer_owned;
                if event.type_() == "pointerdown" {
                    self.pointer_owned = over;
                    if !over {
                        self.model.command_open = false;
                        self.model.focus_command = false;
                        self.context.memory_mut(|memory| {
                            memory.surrender_focus(egui::Id::new("command-input"))
                        });
                    }
                }
                if matches!(event.type_().as_str(), "pointerup" | "pointercancel") {
                    self.pointer_owned = false;
                }
            } else if event.dyn_ref::<web_sys::KeyboardEvent>().is_some() {
                self.consumed = self.context.egui_wants_keyboard_input();
            }

            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                let pos = egui::pos2(
                    (pointer.client_x() as f64 - rect.left()) as f32,
                    (pointer.client_y() as f64 - rect.top()) as f32,
                );
                input.events.push(egui::Event::PointerMoved(pos));
                if matches!(event.type_().as_str(), "pointerdown" | "pointerup") {
                    let button = match pointer.button() {
                        0 => egui::PointerButton::Primary,
                        1 => egui::PointerButton::Middle,
                        _ => egui::PointerButton::Secondary,
                    };
                    input.events.push(egui::Event::PointerButton {
                        pos,
                        button,
                        pressed: event.type_() == "pointerdown",
                        modifiers: Default::default(),
                    });
                    if event.type_() == "pointerdown" {
                        let options = web_sys::FocusOptions::new();
                        options.set_prevent_scroll(true);
                        canvas.focus_with_options(&options)?;
                        event.prevent_default();
                    }
                }
            } else if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                if !key.is_composing() {
                    input.modifiers = egui::Modifiers {
                        alt: key.alt_key(),
                        ctrl: key.ctrl_key(),
                        shift: key.shift_key(),
                        mac_cmd: key.meta_key(),
                        command: key.ctrl_key() || key.meta_key(),
                    };
                    if let Some(code) = egui::Key::from_name(&key.key()) {
                        input.events.push(egui::Event::Key {
                            key: code,
                            physical_key: None,
                            pressed: event.type_() == "keydown",
                            repeat: key.repeat(),
                            modifiers: input.modifiers,
                        });
                    }
                    if event.type_() == "keydown"
                        && key.key().chars().count() == 1
                        && !input.modifiers.command
                        && !input.modifiers.alt
                    {
                        input.events.push(egui::Event::Text(key.key()));
                    }
                    if self.context.egui_wants_keyboard_input() {
                        event.prevent_default();
                    }
                }
            } else if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() {
                input.events.push(egui::Event::MouseWheel {
                    unit: match wheel.delta_mode() {
                        1 => egui::MouseWheelUnit::Line,
                        2 => egui::MouseWheelUnit::Page,
                        _ => egui::MouseWheelUnit::Point,
                    },
                    phase: egui::TouchPhase::Move,
                    delta: egui::vec2(-wheel.delta_x() as f32, -wheel.delta_y() as f32),
                    modifiers: Default::default(),
                });
                if self.context.egui_wants_pointer_input() {
                    event.prevent_default();
                }
            }
        }
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / size.x;
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point =
