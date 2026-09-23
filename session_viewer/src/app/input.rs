use super::gesture::{self, Gesture};
use super::touch::{Act, TAP_SLOP, Touches};
use crate::State;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::keyboard::Key;

/// A press moving less than this many pixels is a click.
const CLICK_SLOP: f64 = 4.0;

/// How far from a handle a mouse press still takes it, CSS pixels.
const MOUSE_REACH: f64 = 8.0;

/// How far from a handle a finger still takes it, CSS pixels.
const TOUCH_REACH: f64 = 18.0;

/// Mouse, keyboard and finger state between events.
pub struct Input {
    orbiting: bool,                          // right button held
    panning: bool,                           // middle button held
    ctrl: bool,                              // Ctrl held
    shift: bool,                             // Shift held
    gesture: Option<&'static Gesture>,       // the left-button tool in charge
    last_cursor: (f64, f64),                 // last pointer position in pixels
    left_down: Option<(f64, f64)>,           // where the left button went down
    plain: bool,             // that press had no Ctrl or Shift and may still start a tool
    dragged: bool,           // that press, or the editing finger, left its slop
    touch: Touches,          // camera finger gestures
    touch_edit: Option<u64>, // finger running a tool
    touch_down: (f64, f64),  // where that finger landed
    fingers: std::collections::HashSet<u64>, // fingers on the screen
    touch_cancelled: bool,   // waiting for all fingers to lift
}

impl Default for Input {
    /// Same as `new`.
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    /// Nothing held, cursor at the origin.
    pub fn new() -> Self {
        Self {
            orbiting: false,
            panning: false,
            ctrl: false,
            shift: false,
            gesture: None,
            last_cursor: (0.0, 0.0),
            left_down: None,
            plain: false,
            dragged: false,
            touch: Touches::new(),
            touch_edit: None,
            touch_down: (0.0, 0.0),
            fingers: std::collections::HashSet::new(),
            touch_cancelled: false,
        }
    }

    /// One key press; true when the frame must be redrawn.
    pub fn key(&mut self, state: &mut State, key: Key<&str>) -> bool {
        let Some(binding) = super::keys::binding(&key, self.ctrl, self.shift) else {
            return false;
        };
        (binding.run)(state);
        true
    }

    /// One mouse or touch event; true when the frame must be redrawn.
    pub fn mouse(&mut self, state: &mut State, event: &WindowEvent) -> bool {
        let viewport = state.viewport();

        match event {
            WindowEvent::MouseInput {
                state: btn,
                button: MouseButton::Right,
                ..
            } => {
                self.orbiting = *btn == ElementState::Pressed;
                state.interacting = self.orbiting || self.panning;
                false
            }
            WindowEvent::MouseInput {
                state: btn,
                button: MouseButton::Middle,
                ..
            } => {
                self.panning = *btn == ElementState::Pressed;
                state.interacting = self.orbiting || self.panning;
                false
            }
            WindowEvent::MouseInput {
                state: btn,
                button: MouseButton::Left,
                ..
            } => self.left(state, *btn),
            WindowEvent::CursorMoved { position, .. } => {
                let scale = crate::engine::gpu::view::surface_per_physical(); // window to canvas pixels
                let at = (position.x * scale, position.y * scale);

                // leaving the click slop turns the press into a drag
                if let Some(down) = self.left_down
                    && (at.0 - down.0).abs().max((at.1 - down.1).abs())
                        > CLICK_SLOP * device_pixel_ratio()
                {
                    self.dragged = true;
                }

                // a plain press dragged past the slop may start a tool, once
                if self.gesture.is_none()
                    && self.plain
                    && self.dragged
                    && let Some(down) = self.left_down
                {
                    self.plain = false;
                    self.gesture = gesture::start(state, down, at);
                }

                if let Some(active) = self.gesture {
                    self.last_cursor = at;
                    return (active.drag)(state, at);
                }

                let dragging = self.orbiting || self.panning;

                // camera moves in CSS pixels
                if dragging {
                    let dx = ((at.0 - self.last_cursor.0) / device_pixel_ratio()) as f32;
                    let dy = ((at.1 - self.last_cursor.1) / device_pixel_ratio()) as f32;

                    if self.panning || self.ctrl {
                        state.camera.pan(dx, dy);
                    } else {
                        state.camera.orbit(dx, dy);
                    }
                }

                self.last_cursor = at;
                dragging || state.hover_drawing(at.0, at.1) || state.hover_gizmo(at.0, at.1)
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 100.0,
                };
                state.camera.zoom_at(amount, self.last_cursor, viewport);
                true
            }
            WindowEvent::ModifiersChanged(mods) => {
                self.ctrl = mods.state().control_key();
                self.shift = mods.state().shift_key();
                false
            }
            WindowEvent::Focused(false) => {
                self.cancel();
                state.cancel_gesture();
                state.interacting = false;
                true
            }
            WindowEvent::Touch(t) => {
                let scale = crate::engine::gpu::view::surface_per_physical();
                let t = &winit::event::Touch {
                    location: winit::dpi::PhysicalPosition::new(
                        t.location.x * scale,
                        t.location.y * scale,
                    ),
                    ..*t
                };
                let at = (t.location.x, t.location.y);

                if t.phase == TouchPhase::Started {
                    self.fingers.insert(t.id);
                }

                // a second finger cancels a one-finger edit
                if self.touch_edit.is_some()
                    && t.phase == TouchPhase::Started
                    && self.fingers.len() > 1
                {
                    state.cancel_gesture();
                    self.touch_edit = None;
                    self.gesture = None;
                    self.touch_cancelled = true;
                }

                // ignore everything until every finger lifts
                if self.touch_cancelled {
                    if matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                        self.fingers.remove(&t.id);
                    }

                    if self.fingers.is_empty() {
                        self.touch_cancelled = false;
                        self.touch = Touches::new();
                    }

                    state.interacting = false;
                    return true;
                }

                // a first finger closes the number box and may grab a control or a handle
                if t.phase == TouchPhase::Started && self.fingers.len() == 1 {
                    state.close_number_box();
                    self.last_cursor = at;
                    self.touch_down = at;
                    self.dragged = false;
                    self.gesture = gesture::press(state, at, TOUCH_REACH);

                    if self.gesture.is_some() {
                        self.touch_edit = Some(t.id);
                    }
                }

                // the editing finger
                if self.touch_edit == Some(t.id) {
                    self.last_cursor = at;
                    let moved = (at.0 - self.touch_down.0).hypot(at.1 - self.touch_down.1);
                    self.dragged |= moved / device_pixel_ratio() > TAP_SLOP; // once away, a drag even if it comes back

                    match (t.phase, self.gesture) {
                        (TouchPhase::Moved, Some(active)) => {
                            (active.drag)(state, at);
                        }
                        (TouchPhase::Ended, Some(active)) => {
                            // a finger that stayed put is a tap
                            let tap = !self.dragged;
                            (active.release)(state, at, tap);

                            // a tap that opened a number box raises the phone keyboard
                            if tap && state.number_box_open() {
                                crate::app::feedback::raise_keyboard();
                            }
                        }
                        (TouchPhase::Cancelled, _) => state.cancel_gesture(),
                        _ => {}
                    }

                    if matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                        self.fingers.remove(&t.id);
                        self.touch_edit = None;
                        self.gesture = None;
                        self.touch = Touches::new();
                    }

                    state.interacting = self.touch_edit.is_some();
                    return true;
                }

                if matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                    self.fingers.remove(&t.id);
                }

                state.interacting = matches!(t.phase, TouchPhase::Started | TouchPhase::Moved);

                // otherwise the fingers move the camera
                match self
                    .touch
                    .event(&mut state.camera, t, viewport, device_pixel_ratio())
                {
                    Act::None => false,
                    Act::Moved => true,
                    Act::Tap(at) => {
                        state.request_selection(at.0 as u32, at.1 as u32, false, false);
                        false
                    }
                    Act::Fit => {
                        state.fit_all();
                        true
                    }
                }
            }
            _ => false,
        }
    }

    /// Forget every gesture in progress.
    pub fn cancel(&mut self) {
        self.orbiting = false;
        self.panning = false;
        self.ctrl = false;
        self.shift = false;
        self.gesture = None;
        self.left_down = None;
        self.plain = false;
        self.dragged = false;
        self.touch = Touches::new();
        self.touch_edit = None;
        self.fingers.clear();
        self.touch_cancelled = false;
    }

    /// Left button: a tool from the registry, else a click.
    fn left(&mut self, state: &mut State, btn: ElementState) -> bool {
        match btn {
            ElementState::Pressed => {
                let closed = state.close_number_box(); // a press in the scene closes the number box
                self.left_down = Some(self.last_cursor);
                self.dragged = false;
                // while drawing, a press is only a click
                self.plain = !self.ctrl && !self.shift && state.draft.is_none();

                if self.plain {
                    self.gesture = gesture::press(state, self.last_cursor, MOUSE_REACH);
                }

                closed
            }
            ElementState::Released => {
                let down = self.left_down.take();
                self.plain = false;

                // the tool in charge takes the release; a press that never left the slop is a click
                if let Some(active) = self.gesture.take() {
                    return (active.release)(state, self.last_cursor, !self.dragged);
                }

                let Some(down) = down else {
                    return false;
                };
                let moved = (self.last_cursor.0 - down.0)
                    .abs()
                    .max((self.last_cursor.1 - down.1).abs());

                if moved > CLICK_SLOP * device_pixel_ratio() {
                    return false; // a drag, not a click
                }

                if state.draft.is_some() {
                    return state.click_drawing(self.last_cursor.0, self.last_cursor.1);
                }
                state.additive_selection = self.shift && !self.ctrl; // Shift adds to the selection
                state.request_selection(
                    self.last_cursor.0 as u32,
                    self.last_cursor.1 as u32,
                    self.ctrl,
                    self.ctrl && self.shift,
                );
                false
            }
        }
    }
}

/// A `pointercancel` listener on the canvas.
#[cfg(target_arch = "wasm32")]
pub struct PointerCancellation {
    canvas: web_sys::HtmlCanvasElement, // the canvas listened to
    callback: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>, // the JS callback
}

#[cfg(target_arch = "wasm32")]
impl PointerCancellation {
    /// Install the listener; it sends one message per event.
    pub fn new(
        canvas: web_sys::HtmlCanvasElement,
        proxy: winit::event_loop::EventLoopProxy<crate::Msg>, // sends messages to the app
    ) -> Result<Self, wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let callback =
            wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                cancel_pointer(&proxy)
            });
        canvas
            .add_event_listener_with_callback("pointercancel", callback.as_ref().unchecked_ref())?;
        Ok(Self { canvas, callback })
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for PointerCancellation {
    /// Remove the listener.
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;
        let _ = self.canvas.remove_event_listener_with_callback(
            "pointercancel",
            self.callback.as_ref().unchecked_ref(),
        );
    }
}

/// Send the cancel message to the event loop.
#[cfg(target_arch = "wasm32")]
fn cancel_pointer(proxy: &winit::event_loop::EventLoopProxy<crate::Msg>) {
    let _ = proxy.send_event(crate::Msg::CancelPointer);
}

/// Physical pixels per CSS pixel.
fn device_pixel_ratio() -> f64 {
    crate::engine::gpu::view::device_pixel_ratio()
}
