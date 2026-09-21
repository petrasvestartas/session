//! The hidden text agent: a 1x1 `<input>` a phone can type into. Keys reach egui only through
//! the focused canvas, and no mobile browser raises its keyboard for a canvas; a tap on the
//! command line focuses this input instead, and its value is replayed into the egui field.

use super::ui::MODEL;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use winit::event_loop::EventLoopProxy;

const AGENT: &str = "command-agent";

/// What the agent reports: its whole value after every edit, or one of the editing keys the
/// command line answers itself.
pub enum AgentEvent {
    Text(String),
    Key(egui::Key),
}

pub struct CommandAgent {
    input: web_sys::HtmlInputElement,
    canvas: web_sys::HtmlCanvasElement,
    on_input: Closure<dyn FnMut(web_sys::Event)>,
    on_key: Closure<dyn FnMut(web_sys::KeyboardEvent)>,
    on_pointer: Closure<dyn FnMut(web_sys::PointerEvent)>,
}

/// The agent input, when the page has one.
fn element() -> Option<web_sys::HtmlInputElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id(AGENT)?
        .dyn_into()
        .ok()
}

impl CommandAgent {
    /// Install once for the canvas lifetime; each callback only forwards a message.
    pub fn new(
        canvas: web_sys::HtmlCanvasElement,
        proxy: EventLoopProxy<crate::Msg>,
    ) -> Result<Self, wasm_bindgen::JsValue> {
        let input =
            element().ok_or_else(|| wasm_bindgen::JsValue::from_str("no #command-agent"))?;
        let on_input = {
            let proxy = proxy.clone();
            let input = input.clone();
            Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                let _ = proxy.send_event(crate::Msg::Agent(AgentEvent::Text(input.value())));
            })
        };
        let on_key = {
            let proxy = proxy.clone();
            Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
                move |event: web_sys::KeyboardEvent| {
                    let key = match event.key().as_str() {
                        "Enter" => egui::Key::Enter,
                        "Tab" => egui::Key::Tab,
                        "Escape" => egui::Key::Escape,
                        "ArrowUp" => egui::Key::ArrowUp,
                        "ArrowDown" => egui::Key::ArrowDown,
                        _ => return,
                    };
                    event.prevent_default();
                    let _ = proxy.send_event(crate::Msg::Agent(AgentEvent::Key(key)));
                },
            )
        };
        // Only a focus inside the gesture raises the keyboard, so the tap is answered here,
        // not on the next frame.
        let on_pointer = {
            let input = input.clone();
            Closure::<dyn FnMut(web_sys::PointerEvent)>::new(move |event: web_sys::PointerEvent| {
                if !matches!(event.pointer_type().as_str(), "touch" | "pen") {
                    return;
                }

                let point = egui::pos2(event.offset_x() as f32, event.offset_y() as f32);
                let (line, popup) = MODEL.with_borrow(|m| {
                    (
                        m.command_rect.is_some_and(|r| r.contains(point)),
                        m.completion_rect.is_some_and(|r| r.contains(point)),
                    )
                });

                if line {
                    let _ = input.focus();
                } else if !popup {
                    let _ = input.blur();
                }
            })
        };
        input.add_event_listener_with_callback("input", on_input.as_ref().unchecked_ref())?;
        input.add_event_listener_with_callback("keydown", on_key.as_ref().unchecked_ref())?;
        canvas
            .add_event_listener_with_callback("pointerdown", on_pointer.as_ref().unchecked_ref())?;
        Ok(Self {
            input,
            canvas,
            on_input,
            on_key,
            on_pointer,
        })
    }
}

impl Drop for CommandAgent {
    /// Detach before dropping the wasm callbacks so JavaScript cannot retain invalid handles.
    fn drop(&mut self) {
        let _ = self
            .input
            .remove_event_listener_with_callback("input", self.on_input.as_ref().unchecked_ref());
        let _ = self
            .input
            .remove_event_listener_with_callback("keydown", self.on_key.as_ref().unchecked_ref());
        let _ = self.canvas.remove_event_listener_with_callback(
            "pointerdown",
            self.on_pointer.as_ref().unchecked_ref(),
        );
    }
}

/// Set the agent's value to the command line's; a programmatic set raises no `input` event.
pub fn sync(command: &str) {
    if let Some(input) = element()
        && input.value() != command
    {
        input.set_value(command);
    }
}
