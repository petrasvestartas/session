// --8<-- [start:step-4]
//! A phone raises its keyboard only for a focused DOM input, never a canvas, so this hidden `<input>` stands in.

use super::ui::MODEL;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use winit::event_loop::EventLoopProxy;

const AGENT: &str = "command-agent"; // id of the hidden input

/// One message from the hidden input.
pub enum AgentEvent {
    Text(String),   // the whole typed text
    Key(egui::Key), // Enter, Tab, Escape or an arrow
}

/// The hidden input and its listeners.
pub struct CommandAgent {
    input: web_sys::HtmlInputElement,                     // the hidden input
    canvas: web_sys::HtmlCanvasElement,                   // the viewer canvas
    on_input: Closure<dyn FnMut(web_sys::Event)>,         // text changed
    on_key: Closure<dyn FnMut(web_sys::KeyboardEvent)>,   // editing key pressed
    on_pointer: Closure<dyn FnMut(web_sys::PointerEvent)>, // tap on the canvas
}

/// The hidden input element, if the page has one.
fn element() -> Option<web_sys::HtmlInputElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id(AGENT)?
        .dyn_into()
        .ok()
}

impl CommandAgent {
    /// Install the listeners; each one sends a message.
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
        // a tap on the command line focuses the input, raising the keyboard
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
    /// Remove the listeners.
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

/// Copy the command line text into the hidden input.
pub fn sync(command: &str) {
    if let Some(input) = element()
        && input.value() != command
    {
        input.set_value(command);
    }
}
// --8<-- [end:step-4]
