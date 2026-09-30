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
