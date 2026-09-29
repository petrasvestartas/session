use std::collections::VecDeque;

pub(crate) mod theme;
pub(crate) mod view;

/// The command vocabulary supplied by the application using the dock.
pub trait Commands {
    fn canonical(&self, line: &str) -> String;
    fn choosing_option(&self, line: &str) -> bool;
    fn draws(&self, line: &str) -> bool;
    fn options(&self, line: &str) -> &'static [&'static str];
    fn option_label<'a>(&self, line: &'a str) -> &'a str;
    fn accept(&self, line: &str) -> (String, bool);
    fn completions(&self, line: &str) -> Vec<&'static str>;
    fn browse(&self, line: &str) -> Vec<&'static str>;
}

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub struct CommandLine {
    pub(crate) command_open: bool,     // command line shown
    pub(crate) command: String,        // text in the command field
    pub(crate) drawing_prompt: String, // prompt while drawing
    pub(crate) drawing_options: &'static [(&'static str, &'static str)], // buttons while drawing: (label, line)
    pub(crate) drawing_chosen: Option<&'static str>, // the option button shown as chosen
    pub(crate) focus_command: bool,                  // give the field focus next frame
    pub(crate) status: String,                       // status line text
    pub(crate) history: VecDeque<String>,            // past commands and answers
    pub(crate) command_expanded: bool,               // history shown above the field
    pub(crate) completion: usize,                    // highlighted completion index
    pub(crate) completion_prefix: String,            // text the completions match
    pub(crate) inline_suffix: bool,                  // completion suffix shown in the field
    pub(crate) completion_visible: bool,             // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>,  // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>,     // where the field is, for taps
    pub(crate) snap_bar: bool,                       // snap toolbar under the field
    pub(crate) snap_modes: u8,                       // snap kinds switched on
    pub(crate) agent_edit: Option<bool>,             // phone keyboard set the text, true on delete
}

/// One clickable control and where it was drawn, for browser tests.
#[derive(serde::Serialize)]
pub struct Control {
    pub(crate) key: String,    // what it does
    pub(crate) label: String,  // text shown
    pub(crate) rect: [f32; 4], // left, top, right, bottom
}

/// Remember one control's rectangle, when inspecting.
pub(crate) fn record(
    controls: &mut Option<Vec<Control>>,
    key: &str,
    label: &str,
    response: &egui::Response,
) {
    let Some(controls) = controls.as_mut() else {
        return;
    };
    let r = response.rect;
    controls.push(Control {
        key: key.to_string(),
        label: label.to_string(),
        rect: [r.min.x, r.min.y, r.max.x, r.max.y],
    });
}

/// The command dock; an executed line goes to `command`.
pub fn draw(
    root: &mut egui::Ui,                 // the panel area
    model: &mut CommandLine,             // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    command: &mut Option<String>,
    commands: &impl Commands,
    snap_modes: &[(&str, u8)],
    escape_allowed: bool,
) -> bool {
    let mut focus_canvas = false;
    let previous_popup = model.completion_rect.take();
    let drawing_options = !model.drawing_options.is_empty();
    // each button row adds this much
    let extra = 28.0 * (usize::from(drawing_options) + usize::from(model.snap_bar)) as f32;
    let panel_response = view::panel(root, model.command_expanded, extra).show_inside(root, |ui| {
        view::prepare(ui);
        // the history above the field
        if model.command_expanded {
            egui::ScrollArea::vertical()
                .id_salt("command-history")
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .min_scrolled_height(0.0)
                .max_height((ui.available_height() - 38.0 - extra).max(0.0))
                .show(ui, |ui| {
                    ui.set_max_width(ui.available_width());
                    for text in &model.history {
                        ui.add(egui::Label::new(text).wrap());
                    }
                    if !model.status.is_empty()
                        && !model
                            .history
                            .back()
                            .is_some_and(|text| text.ends_with(&model.status))
                    {
                        ui.label(&model.status);
                    }
                    if !model.drawing_prompt.is_empty() {
                        let response = ui.add(egui::Label::new(&model.drawing_prompt).wrap());
                        record(controls, "command/hint", &model.drawing_prompt, &response);
                    }
                });
            // divider between history and the field
            let (rect, _) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
            ui.painter().hline(
                ui.max_rect().x_range().expand(6.0),
                rect.center().y,
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(210)),
            );
        }
        // construction or command buttons while drawing
        if drawing_options {
            let choices = model.drawing_options;
            ui.horizontal_wrapped(|ui| {
                for (label, text) in choices {
                    let option = match model.drawing_chosen {
                        Some(chosen) => ui.selectable_label(chosen == *label, *label),
                        None => ui.button(*label),
                    };
                    record(controls, &format!("command/option/{label}"), label, &option);
                    if option.clicked() {
                        *command = Some((*text).into());
                        model.command.clear();
                        model.completion_visible = false;
                        model.focus_command = true;
                    }
                }
            });
        }
        ui.horizontal(|ui| {
            // keep clear of the docs corner
            ui.set_max_width((ui.available_width() - 26.0).max(80.0));
            ui.label("Command:");
            let id = egui::Id::new("command-input");
            if model.focus_command || model.command_open {
                ui.memory_mut(|memory| memory.request_focus(id));
                if model.focus_command {
                    command_cursor_end(ui.ctx(), id, &model.command);
                    model.focus_command = false;
                }
                model.command_open = true;
            }
            // mouse wheel over the field browses the completions
            let wheel = ui.input_mut(|i| {
                let over = i.pointer.hover_pos().is_some_and(|p| {
                    model.command_rect.is_some_and(|r| r.contains(p))
                        || previous_popup.is_some_and(|r| r.contains(p))
                });
                if !over {
                    return 0;
                }
                let delta: f32 = i
                    .events
                    .iter()
                    .filter_map(|e| match e {
                        egui::Event::MouseWheel { delta, .. } => Some(delta.y),
                        _ => None,
                    })
                    .sum();
                i.smooth_scroll_delta = egui::Vec2::ZERO;
                if delta > 0.0 {
                    -1
                } else if delta < 0.0 {
                    1
                } else {
                    0
                }
            });
            if wheel != 0 {
                ui.memory_mut(|memory| memory.request_focus(id));
                model.command_open = true;
            }
            let has_focus = ui.memory(|memory| memory.has_focus(id));
            // wheel or arrow keys: -1 up, +1 down
            let browse = if wheel != 0 {
                wheel
            } else if has_focus {
                ui.input_mut(|i| {
                    if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                        1
                    } else if i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                        -1
                    } else {
                        0
                    }
                })
            } else {
                0
            };
            let enter = has_focus
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)); // run the line
            let tab =
                has_focus && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab)); // accept the completion
            let deletes = ui
                .input(|i| i.key_pressed(egui::Key::Backspace) || i.key_pressed(egui::Key::Delete));
            // space accepts the completion, unless the name goes on with one: `Orient` + space is `Orient 3 Points` typed on
            if model.inline_suffix
                && has_focus
                && model
                    .command
                    .chars()
                    .nth(spelled(&model.command, &model.completion_prefix))
                    != Some(' ')
                && ui.input(|i| {
                    i.events
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
            let option_prefix = if model.inline_suffix {
                &model.completion_prefix
            } else {
                &model.command
            };
            // a drawing verb shows its options as buttons while drawing
            let inline_options =
                if commands.choosing_option(option_prefix) && !commands.draws(option_prefix) {
                    commands.options(option_prefix)
                } else {
                    &[]
                };
            let option_width: f32 = inline_options
                .iter()
                .map(|name| {
                    let label = commands.option_label(name);
                    ui.painter()
                        .layout_no_wrap(
                            label.into(),
                            egui::FontId::proportional(14.0),
                            egui::Color32::BLACK,
                        )
                        .size()
                        .x
                        + 16.0
                })
                .sum();
            // options before the field
            for name in inline_options {
                let label = commands.option_label(name);
                let selected = model.command.trim().eq_ignore_ascii_case(name)
                    || (model.command.ends_with(' ')
                        && !commands.choosing_option(model.command.trim_end())
                        && Some(name) == inline_options.first());
                let option = ui.selectable_label(selected, label);
                record(controls, &format!("command/option/{label}"), label, &option);
                if option.clicked() {
                    let (text, run) = commands.accept(name);
                    if run {
                        *command = Some(text);
                        model.command.clear();
                    } else {
                        model.command = text;
                    }
                    model.completion_visible = false;
                    model.inline_suffix = false;
                    model.focus_command = true;
                }
            }
            let response = view::field(
                ui,
                &mut model.command,
                placeholder(&model.drawing_prompt, &model.status, model.command_expanded),
                option_width,
                model.command_open,
            );
            model.command_rect = Some(response.rect);
            record(controls, "command/input", "Command", &response);
            let focused = model.command_open || response.has_focus() || response.lost_focus();
            if response.gained_focus() {
                model.command_open = true;
            }
            let agent_edit = model.agent_edit.take();
            // the : that opened the line is not part of the command
            if (response.changed() || agent_edit.is_some()) && model.command.starts_with(':') {
                model.command.remove(0);
            }
            let deletes = deletes || agent_edit == Some(true);
            if response.changed() || agent_edit.is_some() {
                model.completion = 0;
                model.completion_visible = !model.command.is_empty();
                model.completion_prefix.clone_from(&model.command);
                model.inline_suffix = false;
                // complete only when typing at the end
                let at_end = egui::TextEdit::load_state(ui.ctx(), id)
                    .and_then(|state| state.cursor.char_range())
                    .is_some_and(|range| {
                        range.is_empty() && range.primary.index == model.command.chars().count()
                    });
                if !deletes
                    && at_end
                    && !model.command.is_empty()
                    && !model.command.ends_with(' ')
                    && let Some(name) = commands.completions(&model.command).first()
                {
                    let prefix = spelled(name, &commands.canonical(&model.command));
                    if name.chars().count() > prefix {
                        model.command = (*name).into();
                        command_cursor_select(ui.ctx(), id, prefix, model.command.chars().count());
                        model.inline_suffix = true;
                        ui.ctx().request_repaint();
                    }
                }
            } else if !model.inline_suffix {
                model.completion_prefix.clone_from(&model.command);
            }
            // the completion list
            let choices = commands.browse(&model.completion_prefix);
            let mut complete = None; // completion chosen this frame
            let opening_list = !model.completion_visible;
            if browse != 0 || tab {
                model.completion_visible = true;
            }
            if focused && model.completion_visible && !choices.is_empty() {
                model.completion = model.completion.min(choices.len() - 1);
                if commands.choosing_option(&model.completion_prefix)
                    && let Some(index) = choices
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case(model.command.trim()))
                {
                    model.completion = index;
                }
                if browse != 0 {
                    model.completion =
                        if opening_list && !commands.choosing_option(&model.completion_prefix) {
                            if browse < 0 { choices.len() - 1 } else { 0 }
                        } else {
                            (model.completion as isize + browse).rem_euclid(choices.len() as isize)
                                as usize
                        };
                    model.command = choices[model.completion].into();
                    command_cursor_select(
                        ui.ctx(),
                        id,
                        if model
                            .command
                            .to_ascii_lowercase()
                            .starts_with(&model.completion_prefix.to_ascii_lowercase())
                        {
                            model.completion_prefix.chars().count()
                        } else {
                            0
                        },
                        model.command.chars().count(),
                    );
                    model.inline_suffix = true;
                    ui.ctx().request_repaint();
                }
                if tab {
                    complete = Some(choices[model.completion]);
                }
                if !commands.choosing_option(&model.completion_prefix) {
                    let popup_width =
                        (ui.ctx().content_rect().right() - response.rect.left() - 12.0)
                            .clamp(60.0, 220.0);
                    let popup_height = (response.rect.top() - 12.0).clamp(22.0, 220.0);
                    let popup = egui::Area::new(egui::Id::new("command-completions"))
                        .pivot(egui::Align2::LEFT_BOTTOM)
                        .fixed_pos(egui::pos2(response.rect.left(), response.rect.top()))
                        .order(egui::Order::Foreground)
                        .show(ui.ctx(), |ui| {
                            egui::Frame::new()
                                .fill(egui::Color32::WHITE)
                                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_gray(215)))
                                .inner_margin(5)
                                .show(ui, |ui| {
                                    ui.set_width(popup_width);
                                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                                    egui::ScrollArea::vertical()
                                        .id_salt("command-choices")
                                        .max_height(popup_height)
                                        .show(ui, |ui| {
                                            for (index, name) in choices.iter().enumerate() {
                                                let item = ui.selectable_label(
                                                    index == model.completion,
                                                    *name,
                                                );
                                                record(
                                                    controls,
                                                    &format!("command/completion/{name}"),
                                                    name,
                                                    &item,
                                                );
                                                if index == model.completion && browse != 0 {
                                                    item.scroll_to_me(None);
                                                }
                                                if item.clicked() {
                                                    complete = Some(*name);
                                                }
                                            }
                                        });
                                });
                        });
                    model.completion_rect = Some(popup.response.rect);
                }
            }
            // a chosen completion fills the field, maybe runs it
            if let Some(name) = complete {
                let (text, run) = commands.accept(name);
                if run && !tab {
                    *command = Some(text);
                    model.command.clear();
                } else {
                    model.command = if run { format!("{text} ") } else { text };
                }
                model.completion_visible = false;
                model.inline_suffix = false;
                model.focus_command = true;
            }
            // Enter runs the line
            if enter && (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty()) {
                let (text, run) = commands.accept(&std::mem::take(&mut model.command));
                if run {
                    *command = Some(text);
                } else {
                    model.command = text;
                }
                model.completion_visible = false;
                model.inline_suffix = false;
                model.focus_command = true;
            }
            // Escape in the field clears it, unless it closes a layer menu or cancels a rename; the scene's Esc is keys.rs's
            if focused && ui.input(|i| i.key_pressed(egui::Key::Escape)) && escape_allowed {
                model.command.clear();
                model.completion_visible = false;
                model.inline_suffix = false;
                model.command_open = false;
                model.focus_command = false;
                response.surrender_focus();
                *command = Some("Escape".into());
                focus_canvas = true;
            }
            // the +/– button folds the history
            let collapse = ui
                .button(if model.command_expanded { "–" } else { "+" })
                .on_hover_text("Collapse or expand history");
            record(
                controls,
                "command/collapse",
                "Collapse or expand history",
                &collapse,
            );
            if collapse.clicked() {
                model.command_expanded = !model.command_expanded;
                ui.ctx().request_repaint();
            }
        });
        // one toggle per snap kind, under the field
        if model.snap_bar {
            ui.horizontal_wrapped(|ui| {
                for &(label, bit) in snap_modes {
                    let toggle = ui.selectable_label(model.snap_modes & bit != 0, label);
                    record(controls, &format!("snap/{label}"), label, &toggle);
                    if toggle.clicked() {
                        *command = Some(format!("Snap {label}"));
                        model.focus_command = true;
                    }
                }
            });
        }
    });
    if let Some(controls) = controls {
        let rect = panel_response.response.rect;
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

/// Put the caret at the end of the field.
pub(crate) fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

/// The characters of `name` that spell `typed`, spaces aside: `orient3p` is `Orient 3 P`.
pub(crate) fn spelled(name: &str, typed: &str) -> usize {
    let typed = typed.chars().filter(|c| !c.is_whitespace()).count();
    let (mut prefix, mut letters) = (0, 0);

    for c in name.chars() {
        if letters == typed {
            break;
        }

        prefix += 1;
        letters += usize::from(!c.is_whitespace());
    }

    prefix
}

/// The grey text of the empty field: the prompt, else the last answer when the history is folded away.
pub(crate) fn placeholder<'a>(prompt: &'a str, status: &'a str, expanded: bool) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}

/// Select `start..end` in the field.
pub(crate) fn command_cursor_select(
    context: &egui::Context,
    id: egui::Id,
    start: usize,
    end: usize,
) {
    if let Some(mut state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }
}
