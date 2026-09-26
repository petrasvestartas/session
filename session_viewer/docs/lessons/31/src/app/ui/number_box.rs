use super::{Control, Output, record};
use crate::State;
use crate::app::gizmo::Handle;
use crate::state::number_box::NumberPrompt;
use std::cell::RefCell;

/// What the gumball number box shows and took this frame.
#[derive(Default)]
pub(crate) struct NumberBox {
    number_prompt: Option<NumberPrompt>, // the box, when open
    number_handle: Option<Handle>,       // the handle the box was opened for
    number: String,                      // text typed into the box
    number_error: String,                // why the typed value was refused
    number_rect: Option<egui::Rect>,     // where the box is, for taps
    typed: Option<String>,               // a value Enter took this frame
    closed: bool,                        // closed without a value this frame
}

thread_local! { static STATE: RefCell<NumberBox> = RefCell::default(); } // kept between frames

/// The number box, registered in PANELS.
pub(super) struct Hooks;

impl super::Panel for Hooks {
    fn fill(&self, state: &mut State) {
        let prompt = state.number_prompt();

        // a box whose handle went behind the eye closes, so no unseen field keeps the keys
        if prompt.is_none() {
            state.close_number_box();
        }

        STATE.with_borrow_mut(|model| model.number_prompt = prompt);
    }

    fn show(&self, root: &mut egui::Ui, controls: &mut Option<Vec<Control>>, _out: &mut Output) {
        STATE.with_borrow_mut(|model| draw(root, model, controls));
    }

    fn apply(&self, state: &mut State) -> bool {
        let (typed, closed) =
            STATE.with_borrow_mut(|model| (model.typed.take(), std::mem::take(&mut model.closed)));

        if closed {
            state.close_number_box();
        }

        // Enter: one undo step, or the reason under the field
        if let Some(text) = &typed {
            match state.type_number(text) {
                Ok(Some(done)) => {
                    crate::app::feedback::status(&done);
                    super::command_line::remember(format!("> {done}")); // register:commands
                }
                Ok(None) => {}
                Err(error) => STATE.with_borrow_mut(|model| model.number_error = error),
            }
        }

        typed.is_some() || closed
    }

    fn keys_taken(&self) -> bool {
        STATE.with_borrow(|model| model.number_prompt.is_some())
    }

    fn field(&self, edit: &mut dyn FnMut(&mut String)) -> Option<&'static str> {
        STATE.with_borrow_mut(|model| {
            model.number_prompt.is_some().then(|| {
                edit(&mut model.number);
                "number-input"
            })
        })
    }

    fn hit(&self, point: egui::Pos2) -> (bool, bool) {
        let inside =
            STATE.with_borrow(|model| model.number_rect.is_some_and(|r| r.contains(point)));
        (inside, inside)
    }

    fn snapshot(&self, json: &mut serde_json::Map<String, serde_json::Value>) {
        STATE.with_borrow(|model| {
            json.insert("number".into(), model.number.clone().into());
            json.insert("number_error".into(), model.number_error.clone().into());
            json.insert("number_rect".into(), super::corners(model.number_rect));
        });
    }
}

/// The gumball number box beside its handle; Enter keeps the text in `typed`, Escape sets `closed`.
fn draw(
    root: &mut egui::Ui,                 // the panel area
    model: &mut NumberBox,               // the panel state
    controls: &mut Option<Vec<Control>>, // placed controls to draw
) {
    let Some(prompt) = model.number_prompt.as_ref() else {
        model.number_handle = None;
        model.number_rect = None;
        return;
    };
    let id = egui::Id::new("number-input");
    let opened = model.number_handle != Some(prompt.handle);

    // a new box starts empty, with the keys
    if opened {
        model.number_handle = Some(prompt.handle);
        model.number.clear();
        model.number_error.clear();
        root.memory_mut(|memory| memory.request_focus(id));
    }

    let focused = root.memory(|memory| memory.focused());

    // Escape, or another field taking the keys, closes it
    if root.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        || focused.is_some_and(|other| other != id)
    {
        model.closed = true;
        return;
    }

    // a click in the scene dropped the focus: the box keeps the keys while it is open
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
                        record(controls, "number/input", &prompt.title, &edit);
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
