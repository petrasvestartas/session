// --8<-- [start:command-line-state]
use super::{Control, Output, record};
use crate::State;
use std::cell::RefCell;
use std::collections::VecDeque;

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub(crate) struct CommandLine {
    pub(crate) command_open: bool,     // command line shown
    pub(crate) command: String,        // text in the command field
    pub(crate) drawing_prompt: String, // prompt while drawing
    drawing_options: &'static [(&'static str, &'static str)], // buttons while drawing: (label, line)
    drawing_chosen: Option<&'static str>,                     // the option button shown as chosen
    pub(crate) focus_command: bool,                           // give the field focus next frame
    pub(crate) status: String,                                // status line text
    history: VecDeque<String>,                                // past commands and answers
    command_expanded: bool,                                   // history shown above the field
    completion: usize,                                        // highlighted completion index
    completion_prefix: String,                                // text the completions match
    pub(crate) inline_suffix: bool, // completion suffix shown in the field
    completion_visible: bool,       // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>, // where the field is, for taps
    snap_bar: bool,                 // snap toolbar under the field
    snap_modes: u8,                 // snap kinds switched on
    pub(crate) agent_edit: Option<bool>, // phone keyboard set the text, true on delete
}

thread_local! { pub(crate) static STATE: RefCell<CommandLine> = RefCell::default(); } // kept between frames

/// Add a line to the history, keeping the last 200.
pub(super) fn remember(line: String) {
    STATE.with_borrow_mut(|model| {
        if model.history.len() == 200 {
            model.history.pop_front();
        }

        model.history.push_back(line);
    });
}
// --8<-- [end:command-line-state]

// --8<-- [start:command-line-panel]
/// The command line, registered in PANELS.
pub(super) struct Hooks;

impl super::Panel for Hooks {
    fn closes_on_escape(&self) -> bool {
        STATE.with_borrow(|model| model.command_open)
    }

    /// A press on the field opens it, anywhere else but the list closes it.
    fn press(&self, context: &egui::Context, pointer: egui::Pos2) {
        let id = egui::Id::new("command-input");
        let (input, popup) = STATE.with_borrow(|m| {
            (
                m.command_rect.is_some_and(|r| r.contains(pointer)),
                m.completion_rect.is_some_and(|r| r.contains(pointer)),
            )
        });

        // focus now so the first key is not lost
        if input {
            context.memory_mut(|memory| memory.request_focus(id));
            STATE.with_borrow_mut(|model| model.command_open = true);
        } else if !popup {
            context.memory_mut(|memory| memory.surrender_focus(id));
            STATE.with_borrow_mut(|model| model.command_open = false);
        }
    }

    fn fill(&self, state: &mut State) {
        STATE.with_borrow_mut(|model| {
            model.drawing_prompt = state.drawing_prompt();
            model.drawing_options = state.drawing_options();
            model.drawing_chosen = state.drawing_chosen();
            model.snap_bar = state.features.snap.bar;
            model.snap_modes = state.features.snap.modes;
        });
    }

    fn show(&self, root: &mut egui::Ui, controls: &mut Option<Vec<Control>>, out: &mut Output) {
        STATE.with_borrow_mut(|model| draw(root, model, controls, out));
    }

    fn keys_taken(&self) -> bool {
        STATE.with_borrow(|model| model.command_open)
    }

    fn hit(&self, point: egui::Pos2) -> (bool, bool) {
        STATE.with_borrow(|model| {
            let inside = |rect: Option<egui::Rect>| rect.is_some_and(|r| r.contains(point));
            (inside(model.command_rect), inside(model.completion_rect))
        })
    }

    fn snapshot(&self, json: &mut serde_json::Map<String, serde_json::Value>) {
        STATE.with_borrow(|model| {
            let placeholder =
                placeholder(&model.drawing_prompt, &model.status, model.command_expanded);
            json.insert(
                "completion_rect".into(),
                super::corners(model.completion_rect),
            );
            json.insert("command_open".into(), model.command_open.into());
            json.insert("command".into(), model.command.clone().into());
            json.insert("history".into(), serde_json::json!(model.history));
            json.insert(
                "hint".into(),
                crate::app::command::hint(&model.command).into(),
            );
            json.insert("placeholder".into(), placeholder.into());
        });
    }
}
// --8<-- [end:command-line-panel]

// --8<-- [start:dock-frame]
/// The command dock at the bottom: one row when folded, the history above the field when unfolded; an executed line goes to `out.command`.
fn draw(
    root: &mut egui::Ui,                 // the panel area
    model: &mut CommandLine,             // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
    out: &mut Output,
) {
    let previous_popup = model.completion_rect.take();
    let drawing_options = !model.drawing_options.is_empty();
    // each button row adds this much
    let extra = 28.0 * (usize::from(drawing_options) + usize::from(model.snap_bar)) as f32;
    // Folded it is exactly one row, 30 px plus a row per button bar; unfolded the person drags its top edge.
    let panel = if !model.command_expanded {
        egui::Panel::bottom("command-line-collapsed").exact_size(30.0 + extra)
    } else {
        egui::Panel::bottom("command-line")
            .default_size(104.0)
            .resizable(true)
            .size_range((64.0 + extra)..=(root.available_height() * 0.75).max(104.0 + extra))
    };
    let panel_response = panel
        .show_separator_line(false)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .inner_margin(egui::Margin::symmetric(6, 4)),
        )
        .show_inside(root, |ui| {
            ui.set_min_height(ui.max_rect().height());
            ui.painter().hline(
                ui.max_rect().x_range().expand(6.0),
                ui.max_rect().top() - 4.0,
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(110)),
            );
            ui.set_clip_rect(ui.max_rect().expand(6.0));
            ui.style_mut().override_font_id = Some(egui::FontId::proportional(14.0));
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
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
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 1.0),
                    egui::Sense::hover(),
                );
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
                            out.command = Some((*text).into());
                            model.command.clear();
                            model.completion_visible = false;
                            model.focus_command = true;
                        }
                    }
                });
            }
            // --8<-- [end:dock-frame]
            // --8<-- [start:dock-input]
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
                    && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)); // consume_key takes the key out of egui's input, so no other widget sees it too
                let tab = has_focus
                    && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab)); // accept the completion
                let deletes = ui.input(|i| {
                    i.key_pressed(egui::Key::Backspace) || i.key_pressed(egui::Key::Delete)
                });
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
                // --8<-- [end:dock-input]
                // --8<-- [start:dock-field]
                let option_prefix = if model.inline_suffix {
                    &model.completion_prefix
                } else {
                    &model.command
                };
                // a drawing verb shows its options as buttons while drawing
                let inline_options = if crate::app::command::choosing_option(option_prefix)
                    && !crate::app::command::draws(option_prefix)
                {
                    crate::app::command::options(option_prefix)
                } else {
                    &[]
                };
                let option_width: f32 = inline_options
                    .iter()
                    .map(|name| {
                        let label = crate::app::command::option_label(name);
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
                    let label = crate::app::command::option_label(name);
                    let selected = model.command.trim().eq_ignore_ascii_case(name)
                        || (model.command.ends_with(' ')
                            && !crate::app::command::choosing_option(model.command.trim_end())
                            && Some(name) == inline_options.first());
                    let option = ui.selectable_label(selected, label);
                    record(controls, &format!("command/option/{label}"), label, &option);
                    if option.clicked() {
                        let (text, run) = crate::app::command::accept(name);
                        if run {
                            out.command = Some(text);
                            model.command.clear();
                        } else {
                            model.command = text;
                        }
                        model.completion_visible = false;
                        model.inline_suffix = false;
                        model.focus_command = true;
                    }
                }
                let response = ui.add_sized(
                    [(ui.available_width() - option_width - 28.0).max(40.0), 22.0],
                    egui::TextEdit::singleline(&mut model.command)
                        .id(id)
                        .font(egui::FontId::proportional(14.0))
                        .vertical_align(egui::Align::Center)
                        .frame(egui::Frame::NONE)
                        .clip_text(true)
                        .char_limit(2048)
                        .hint_text(placeholder(
                            &model.drawing_prompt,
                            &model.status,
                            model.command_expanded,
                        )),
                );
                // caret visible on an empty field
                if model.command_open && model.command.is_empty() {
                    let y = response.rect.center().y;
                    ui.painter().vline(
                        response.rect.left(),
                        y - 7.0..=y + 7.0,
                        ui.visuals().text_cursor.stroke,
                    );
                }
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
                        && let Some(name) = crate::app::command::completions(&model.command).first()
                    {
                        let prefix = spelled(name, &crate::app::command::canonical(&model.command));
                        if name.chars().count() > prefix {
                            model.command = (*name).into();
                            // the completed tail stays selected, so the next letter replaces it: `Lin` shows `Line` with `e` selected
                            command_cursor_select(
                                ui.ctx(),
                                id,
                                prefix,
                                model.command.chars().count(),
                            );
                            model.inline_suffix = true;
                            ui.ctx().request_repaint();
                        }
                    }
                } else if !model.inline_suffix {
                    model.completion_prefix.clone_from(&model.command);
                }
                // --8<-- [end:dock-field]
                // --8<-- [start:dock-completions]
                // the completion list
                let choices = crate::app::command::browse(&model.completion_prefix);
                let mut complete = None; // completion chosen this frame
                let opening_list = !model.completion_visible;
                if browse != 0 || tab {
                    model.completion_visible = true;
                }
                if focused && model.completion_visible && !choices.is_empty() {
                    model.completion = model.completion.min(choices.len() - 1);
                    if crate::app::command::choosing_option(&model.completion_prefix)
                        && let Some(index) = choices
                            .iter()
                            .position(|name| name.eq_ignore_ascii_case(model.command.trim()))
                    {
                        model.completion = index;
                    }
                    if browse != 0 {
                        model.completion = if opening_list
                            && !crate::app::command::choosing_option(&model.completion_prefix)
                        {
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
                    if !crate::app::command::choosing_option(&model.completion_prefix) {
                        let popup_width =
                            (ui.ctx().content_rect().right() - response.rect.left() - 12.0)
                                .clamp(60.0, 220.0);
                        let popup_height = (response.rect.top() - 12.0).clamp(22.0, 220.0);
                        // An Area floats above the panels; pivot LEFT_BOTTOM puts its bottom-left corner on the field's top-left.
                        let popup = egui::Area::new(egui::Id::new("command-completions"))
                            .pivot(egui::Align2::LEFT_BOTTOM)
                            .fixed_pos(egui::pos2(response.rect.left(), response.rect.top()))
                            .order(egui::Order::Foreground)
                            .show(ui.ctx(), |ui| {
                                egui::Frame::new()
                                    .fill(egui::Color32::WHITE)
                                    .stroke(egui::Stroke::new(
                                        1.0_f32,
                                        egui::Color32::from_gray(215),
                                    ))
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
                // --8<-- [end:dock-completions]
                // --8<-- [start:dock-run]
                // a chosen completion fills the field, maybe runs it
                if let Some(name) = complete {
                    let (text, run) = crate::app::command::accept(name);
                    if run && !tab {
                        out.command = Some(text);
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
                    let (text, run) =
                        crate::app::command::accept(&std::mem::take(&mut model.command));
                    if run {
                        out.command = Some(text);
                    } else {
                        model.command = text;
                    }
                    model.completion_visible = false;
                    model.inline_suffix = false;
                    model.focus_command = true;
                }
                // Escape in the field clears it, unless it closes a layer menu or cancels a rename; the scene's Esc is keys.rs's
                if focused
                    && ui.input(|i| i.key_pressed(egui::Key::Escape))
                    && !super::menu_open()
                    && !super::escape_held()
                {
                    model.command.clear();
                    model.completion_visible = false;
                    model.inline_suffix = false;
                    model.command_open = false;
                    model.focus_command = false;
                    response.surrender_focus();
                    out.command = Some("Escape".into());
                    crate::app::feedback::focus_canvas();
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
                    for (label, bit) in crate::app::snap::MODES {
                        let toggle = ui.selectable_label(model.snap_modes & bit != 0, label);
                        record(controls, &format!("snap/{label}"), label, &toggle);
                        if toggle.clicked() {
                            out.command = Some(format!("Snap {label}"));
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
}
// --8<-- [end:dock-run]

// --8<-- [start:command-line-helpers]
/// Put the caret at the end of the field.
pub(super) fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

/// The characters of `name` that spell `typed`, spaces aside: `orient3p` is `Orient 3 P`.
fn spelled(name: &str, typed: &str) -> usize {
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
pub(super) fn placeholder<'a>(prompt: &'a str, status: &'a str, expanded: bool) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}

/// Select `start..end` in the field.
pub(super) fn command_cursor_select(
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
// --8<-- [end:command-line-helpers]

// --8<-- [start:command-line-tests]
#[cfg(test)]
mod tests {
    use super::super::theme::{BUNDLED, fonts};
    use super::*;

    fn frame(
        context: &egui::Context,
        model: &mut CommandLine,
        key: Option<egui::Key>,
    ) -> Option<String> {
        let mut input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 700.0),
            )),
            ..Default::default()
        };
        if let Some(key) = key {
            for pressed in [true, false] {
                input.events.push(egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                });
            }
        }
        let mut out = Output::default();
        let _ = context.run_ui(input, |ui| draw(ui, model, &mut None, &mut out));
        out.command
    }

    #[test]
    fn up_down_cycles_command_options_and_enter_accepts() {
        for name in ["Element Features", "Layers", "Arctic", "Outline", "Snap"] {
            let context = egui::Context::default();
            context.set_fonts(fonts(BUNDLED));
            context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
            let mut model = CommandLine {
                command: name.into(),
                focus_command: true,
                ..Default::default()
            };
            frame(&context, &mut model, None);
            assert_eq!(frame(&context, &mut model, Some(egui::Key::Enter)), None);
            assert_eq!(model.command, format!("{name} "));
            let options = crate::app::command::options(&model.command);
            for key in [egui::Key::ArrowLeft, egui::Key::ArrowRight] {
                frame(&context, &mut model, Some(key));
                assert_eq!(model.command, format!("{name} "));
            }
            frame(&context, &mut model, Some(egui::Key::ArrowDown));
            assert_eq!(model.command, options[1]);
            frame(&context, &mut model, Some(egui::Key::ArrowDown));
            assert_eq!(model.command, options[2 % options.len()]);
            frame(&context, &mut model, Some(egui::Key::ArrowUp));
            assert_eq!(model.command, options[1]);
            let (expected, run) = crate::app::command::accept(options[1]);
            let command = frame(&context, &mut model, Some(egui::Key::Enter));
            assert_eq!(command, run.then_some(expected.clone()));
            if !run {
                assert_eq!(model.command, expected);
            }
        }
    }

    #[test]
    fn a_space_the_name_spells_types_on() {
        assert_eq!(spelled("Orient 3 Points", "orient3p"), 10);
        assert_eq!(spelled("Orient 3 Points", "Orient"), 6);
        let context = egui::Context::default();
        context.set_fonts(fonts(BUNDLED));
        let mut model = CommandLine {
            focus_command: true,
            ..Default::default()
        };
        frame(&context, &mut model, None);

        // one key per frame, like a person typing
        for c in "Orient 3 Points".chars() {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 700.0),
                )),
                events: vec![egui::Event::Text(c.to_string())],
                ..Default::default()
            };
            let _ = context.run_ui(input, |ui| {
                draw(ui, &mut model, &mut None, &mut Output::default())
            });
        }

        assert_eq!(model.command, "Orient 3 Points");
    }

    #[test]
    fn browsing_starts_from_the_typed_option() {
        let context = egui::Context::default();
        context.set_fonts(fonts(BUNDLED));
        let mut model = CommandLine {
            command: "Snap Off".into(),
            focus_command: true,
            ..Default::default()
        };
        frame(&context, &mut model, None);
        frame(&context, &mut model, Some(egui::Key::ArrowDown));
        assert_eq!(model.command, "Snap End");
    }
}
// --8<-- [end:command-line-tests]
