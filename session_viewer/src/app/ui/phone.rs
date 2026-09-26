use super::command_line::command_cursor_end;
use super::{MODEL, Model, Ui};

/// A text field the phone keyboard types into, besides the command line.
struct TextField {
    id: &'static str,                            // the egui id of its text edit
    text: fn(&mut Model) -> Option<&mut String>, // its text, None while it is not shown
}

/// The fields that take the phone keyboard before the command line, the first open one.
const FIELDS: &[TextField] = &[
    // register:layer-rename
    TextField {
        id: "layer-rename",
        text: |model| model.renaming.as_mut().map(|rename| &mut rename.text),
    },
    // register:number-box
    TextField {
        id: "number-input",
        text: |model| model.number_prompt.is_some().then_some(&mut model.number),
    },
];

/// The open field the phone keyboard types into, None for the command line.
fn open_field(model: &mut Model) -> Option<&'static TextField> {
    FIELDS.iter().find(|field| (field.text)(model).is_some())
}

impl Ui {
    /// Feed the hidden input's typing into the field; returns keys for the viewport.
    pub fn agent(&mut self, event: crate::app::agent::AgentEvent) -> Vec<String> {
        use crate::app::agent::AgentEvent;
        let id = egui::Id::new("command-input");

        // a layer name or the number box takes the typing before the command line
        if let Some(field) = MODEL.with_borrow_mut(open_field) {
            self.type_into(field, event);
            return Vec::new();
        }

        // an empty line while drawing still finishes the shape
        let (open, empty) = MODEL.with_borrow(|m| {
            (
                m.command_open,
                m.command.is_empty() && m.drawing_prompt.is_empty(),
            )
        });

        match &event {
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
        let events = &mut self.input.egui_input_mut().events;

        match event {
            // the input's whole text replaces the field, so keyboard composition cannot duplicate it
            AgentEvent::Text(value) => {
                if value == self.agent_value {
                    return Vec::new();
                }

                let deleted = value.chars().count() < self.agent_value.chars().count();
                self.agent_value.clone_from(&value);
                MODEL.with_borrow_mut(|model| {
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

    /// Feed the hidden input's typing into a field other than the command line.
    fn type_into(&mut self, field: &TextField, event: crate::app::agent::AgentEvent) {
        use crate::app::agent::AgentEvent;
        let id = egui::Id::new(field.id);

        match event {
            // the input's whole text replaces the field's, as for the command line
            AgentEvent::Text(value) => {
                command_cursor_end(&self.context, id, &value);
                MODEL.with_borrow_mut(|model| {
                    if let Some(text) = (field.text)(model) {
                        text.clone_from(&value);
                    }
                });
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

    /// The open field changed on its own: the hidden input follows.
    pub(super) fn follow_field(&mut self) {
        MODEL.with_borrow_mut(|model| {
            let field = open_field(model).map(|field| field.id);

            // a field that just opened takes the input, selected so typing replaces it
            if field != self.field {
                // a field that closed lowers the keyboard, unless the command line took it
                if field.is_none() && !model.command_open {
                    crate::app::agent::blur();
                }

                self.field = field;

                if let Some(text) = open_field(model).and_then(|field| (field.text)(model)) {
                    self.agent_value.clone_from(text);
                    crate::app::agent::edit(text);
                }
            }

            // the input keeps what was typed: no completion suffix, no rewrite mid-word
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
