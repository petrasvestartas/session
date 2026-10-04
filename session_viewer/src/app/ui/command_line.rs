use super::{Control, Output};
use crate::State;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::command_dock::command_cursor_end;
pub(crate) use crate::command_dock::command_cursor_select;
#[cfg(test)]
use crate::command_dock::spelled;
use crate::command_dock::{CommandLine, placeholder};
use std::cell::RefCell;

thread_local! { pub(crate) static STATE: RefCell<CommandLine> = RefCell::default(); } // kept between frames

/// Add a line to the history, keeping the last 200.
pub(crate) fn remember(line: String) {
    STATE.with_borrow_mut(|model| model.remember(line));
}

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
        });
    }

    fn show(&self, root: &mut egui::Ui, controls: &mut Option<Vec<Control>>, out: &mut Output) {
        STATE.with_borrow_mut(|model| draw(root, model, controls, out));
    }

    fn keys_taken(&self) -> bool {
        STATE.with_borrow(|model| model.command_open)
    }

    fn holds_history(&self) -> bool {
        STATE.with_borrow(|model| model.command_open && !model.command.is_empty())
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

struct Commands;

impl crate::command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        crate::app::command::canonical(line)
    }

    fn choosing_option(&self, line: &str) -> bool {
        crate::app::command::choosing_option(line)
    }

    fn accept(&self, line: &str) -> (String, bool) {
        crate::app::command::accept(line)
    }

    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::app::command::completions(line)
    }

    fn browse(&self, line: &str) -> Vec<&'static str> {
        crate::app::command::browse(line)
    }
}

fn draw(
    root: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    out: &mut Output,
) {
    let focus = crate::command_dock::draw(
        root,
        model,
        controls,
        &mut out.command,
        &Commands,
        !super::menu_open() && !super::escape_held(),
    );
    if focus {
        crate::app::feedback::focus_canvas();
    }
}

#[cfg(test)]
mod tests;
