//! A hidden `<input>` that raises the phone keyboard for the command line and every other text field.

use super::ui::MODEL;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use winit::event_loop::EventLoopProxy;

const AGENT: &str = "command-agent"; // id of the hidden input

thread_local! { static COMPOSING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; } // the keyboard is composing a word
thread_local! { static RAISED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; } // the tap being handled raised the keyboard

/// One message from the hidden input.
pub enum AgentEvent {
    Text(String),   // the whole typed text
    Key(egui::Key), // Enter, Tab, Escape or an arrow
}

/// The hidden input and its listeners.
pub struct CommandAgent {
    input: web_sys::HtmlInputElement,             // the hidden input
    canvas: web_sys::HtmlCanvasElement,           // the viewer canvas
    on_input: Closure<dyn FnMut(web_sys::Event)>, // text changed
    on_key: Closure<dyn FnMut(web_sys::KeyboardEvent)>, // editing key pressed
    on_compose: Closure<dyn FnMut(web_sys::Event)>, // composition started or ended
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
                    // some phone keyboards name Enter only by its code
                    let key = match event.key().as_str() {
                        _ if event.key_code() == 13 => egui::Key::Enter,
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
        // while a word is composed the input must not be rewritten
        let on_compose = {
            let proxy = proxy.clone();
            let input = input.clone();
            Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
                let started = event.type_() == "compositionstart";
                COMPOSING.with(|composing| composing.set(started));

                if !started {
                    let _ = proxy.send_event(crate::Msg::Agent(AgentEvent::Text(input.value())));
                }
            })
        };
        // a tap on a text field, or an item opening one, focuses the input, raising the keyboard
        let on_pointer = {
            let input = input.clone();
            Closure::<dyn FnMut(web_sys::PointerEvent)>::new(move |event: web_sys::PointerEvent| {
                if !matches!(event.pointer_type().as_str(), "touch" | "pen") {
                    return;
                }

                // the tap opened a field and raised the keyboard already
                if RAISED.with(|raised| raised.replace(false)) {
                    return;
                }

                let point = egui::pos2(event.offset_x() as f32, event.offset_y() as f32);
                let (line, popup) = MODEL.with_borrow(|m| {
                    (
                        m.command_rect.is_some_and(|r| r.contains(point))
                            || m.number_rect.is_some_and(|r| r.contains(point))
                            || m.keyboard_rects.iter().any(|r| r.contains(point)),
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

        for kind in ["compositionstart", "compositionend"] {
            input.add_event_listener_with_callback(kind, on_compose.as_ref().unchecked_ref())?;
        }

        // pointerdown keeps the keyboard up, pointerup is the gesture phones require to raise it
        for kind in ["pointerdown", "pointerup"] {
            canvas.add_event_listener_with_callback(kind, on_pointer.as_ref().unchecked_ref())?;
        }

        Ok(Self {
            input,
            canvas,
            on_input,
            on_key,
            on_compose,
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

        for kind in ["compositionstart", "compositionend"] {
            let _ = self.input.remove_event_listener_with_callback(
                kind,
                self.on_compose.as_ref().unchecked_ref(),
            );
        }

        for kind in ["pointerdown", "pointerup"] {
            let _ = self.canvas.remove_event_listener_with_callback(
                kind,
                self.on_pointer.as_ref().unchecked_ref(),
            );
        }
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

/// Put a field's text into the hidden input, selected while it has the keyboard, so typing replaces it.
pub fn edit(text: &str) {
    let Some(input) = element() else {
        return;
    };
    let focused = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element())
        .is_some_and(|active| active == **input);
    input.set_value(text);

    // select() would also take the keys from the canvas
    if focused {
        input.select();
    }
}

/// Raise the phone keyboard over an empty input; phones allow it only while a tap is handled.
pub fn raise() {
    if let Some(input) = element() {
        input.set_value("");
        let _ = input.focus();
        RAISED.with(|raised| raised.set(true));
    }
}

/// Lower the phone keyboard.
pub fn blur() {
    if let Some(input) = element() {
        let _ = input.blur();
    }
}

/// True while the phone keyboard composes a word.
pub fn composing() -> bool {
    COMPOSING.with(|composing| composing.get())
}
