// --8<-- [start:phone-field]
// One hidden input serves every text field: it types into whichever panel field is open, else the command line.
use super::command_line::{STATE, command_cursor_end};
use super::{PANELS, Panel, Ui};

/// The open field the phone keyboard types into, None for the command line; a later panel wins, so a layer rename before the number box.
fn open_field() -> Option<(&'static dyn Panel, &'static str)> {
    PANELS
        .iter()
        .rev()
        // `&mut |_| {}` = a closure that leaves the text alone: here `field` is only asked for its id
        .find_map(|panel| panel.field(&mut |_| {}).map(|id| (*panel, id)))
}
// --8<-- [end:phone-field]

// --8<-- [start:phone-agent]
impl Ui {
    /// Feed the hidden input's typing into the field; returns keys for the viewport.
    pub fn agent(&mut self, event: crate::app::agent::AgentEvent) -> Vec<String> {
        use crate::app::agent::AgentEvent;
        let id = egui::Id::new("command-input");

        // a layer name or the number box takes the typing before the command line
        if let Some(field) = open_field() {
            self.type_into(field, event);
            return Vec::new();
        }

        // an empty line while drawing still finishes the shape
        let (open, empty) = STATE.with_borrow(|m| {
            (
                m.command_open,
                m.command.is_empty() && m.drawing_prompt.is_empty(),
            )
        });

        match &event {
            // command line closed: the typed letters are viewer shortcuts, so only the new ones go out as keys
            AgentEvent::Text(value) if !open => {
                let shared = self
                    .agent_value
                    .chars()
                    .zip(value.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                let keys = value.chars().skip(shared).map(String::from).collect();
                self.agent_value.clear();
                crate::app::agent::sync("");
                return keys;
            }
            AgentEvent::Key(egui::Key::Enter) if open && empty => {
                crate::app::feedback::command_line(false);
                self.context.memory_mut(|memory| memory.surrender_focus(id));
                crate::app::feedback::status("Keys go to the viewer · type : for the command line");
                return Vec::new();
            }
            _ => {}
        }

        let key = |key, pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        // keys join the events egui reads next frame, as if the real keyboard had pressed and released them
        let events = &mut self.input.egui_input_mut().events;

        match event {
            // the input's whole text replaces the field, so keyboard composition cannot duplicate it
            AgentEvent::Text(value) => {
                if value == self.agent_value {
                    return Vec::new();
                }

                let deleted = value.chars().count() < self.agent_value.chars().count();
                self.agent_value.clone_from(&value);
                STATE.with_borrow_mut(|model| {
                    model.command = value;
                    model.agent_edit = Some(deleted);
                    model.command_open = true;
                    model.focus_command = true;
                });
                self.context.memory_mut(|memory| memory.request_focus(id));
            }
            AgentEvent::Key(k) => {
                events.push(key(k, true));
                events.push(key(k, false));
            }
        }

        Vec::new()
    }
    // --8<-- [end:phone-agent]

    // --8<-- [start:phone-type-into]
    /// Feed the hidden input's typing into a field other than the command line.
    fn type_into(
        &mut self,
        (panel, field): (&'static dyn Panel, &'static str),
        event: crate::app::agent::AgentEvent,
    ) {
        use crate::app::agent::AgentEvent;
        let id = egui::Id::new(field);

        match event {
            // the input's whole text replaces the field's, as for the command line
            AgentEvent::Text(value) => {
                command_cursor_end(&self.context, id, &value);
                panel.field(&mut |text| text.clone_from(&value));
                self.agent_value = value;
                self.context.memory_mut(|memory| memory.request_focus(id));
            }
            AgentEvent::Key(key) => {
                // Escape drops the text and lowers the keyboard; Enter lowers it once the field closes
                if key == egui::Key::Escape {
                    crate::app::agent::blur();
                }

                for pressed in [true, false] {
                    self.input.egui_input_mut().events.push(egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    });
                }
            }
        }
    }
    // --8<-- [end:phone-type-into]

    // --8<-- [start:phone-follow]
    /// The open field changed on its own: the hidden input follows.
    pub(super) fn follow_field(&mut self) {
        let open = open_field();
        let field = open.map(|(_, id)| id);

        // a field that just opened takes the input, selected so typing replaces it
        if field != self.field {
            // a field that closed lowers the keyboard, unless the command line took it
            if field.is_none() && !STATE.with_borrow(|model| model.command_open) {
                crate::app::agent::blur();
            }

            self.field = field;

            if let Some((panel, _)) = open {
                panel.field(&mut |text| {
                    self.agent_value.clone_from(text);
                    crate::app::agent::edit(text);
                });
            }
        }

        // the input keeps what was typed: no completion suffix, no rewrite mid-word
        STATE.with_borrow(|model| {
            if self.field.is_none()
                && self.agent_value != model.command
                && !model.inline_suffix
                && !crate::app::agent::composing()
            {
                self.agent_value.clone_from(&model.command);
                crate::app::agent::sync(&model.command);
            }
        });
    }
}
// --8<-- [end:phone-follow]
