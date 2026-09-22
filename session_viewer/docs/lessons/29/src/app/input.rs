use super::touch::{Act, Touches};
use crate::State;
use crate::camera::View;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::keyboard::{Key, NamedKey};

/// A press moving less than this many pixels is a click.
const CLICK_SLOP: f64 = 4.0;

/// Mouse, keyboard and finger state between events.
pub struct Input {
    orbiting: bool,                          // right button held
    panning: bool,                           // middle button held
    ctrl: bool,                              // Ctrl held
    shift: bool,                             // Shift held
    gizmo_drag: bool,                        // a gizmo handle is being dragged
    control_drag: bool,                      // a control point is being dragged
    last_cursor: (f64, f64),                 // last pointer position in pixels
    left_down: Option<(f64, f64)>,           // where the left button went down
    touch: Touches,                          // camera finger gestures
    // --8<-- [start:step-5a]
    touch_edit: Option<u64>,                 // finger dragging a handle or control
    fingers: std::collections::HashSet<u64>, // fingers on the screen
    touch_cancelled: bool,                   // waiting for all fingers to lift
    // --8<-- [end:step-5a]
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
            gizmo_drag: false,
            control_drag: false,
            last_cursor: (0.0, 0.0),
            left_down: None,
            touch: Touches::new(),
            // --8<-- [start:step-5b]
            touch_edit: None,
            fingers: std::collections::HashSet::new(),
            touch_cancelled: false,
            // --8<-- [end:step-5b]
        }
    }

    /// One key press; true when the frame must be redrawn.
    pub fn key(&mut self, state: &mut State, key: Key<&str>) -> bool {
        match key {
            Key::Named(NamedKey::Space) => state
                .camera
                .toggle_projection_framed(&state.gpu.bounds, state.aspect()),
            Key::Named(NamedKey::Escape) => state.escape_selection(),
            Key::Named(NamedKey::F10) => state.enable_controls(),
            Key::Named(NamedKey::Delete) => state.delete_selected(),
            // colon opens the command line
            Key::Character(":") => {
                crate::app::feedback::command_line(true);
            }
            Key::Character("l" | "L") => state.toggle_layers_panel(),
            // Ctrl+Z undo, Ctrl+Shift+Z redo
            Key::Character("z" | "Z") if self.ctrl => {
                if self.shift {
                    state.redo()
                } else {
                    state.undo()
                }
            }
            Key::Character("y" | "Y") if self.ctrl => state.redo(),
            Key::Character("1") => state.camera.set_view(View::Front),
            Key::Character("2") => state.camera.set_view(View::Back),
            Key::Character("3") => state.camera.set_view(View::Left),
            Key::Character("4") => state.camera.set_view(View::Right),
            Key::Character("5") => state.camera.set_view(View::Top),
            Key::Character("6") => state.camera.set_view(View::Bottom),
            Key::Character("7") => state.camera.set_view(View::Iso),
            Key::Character("c" | "C") => state.camera.reset(),
            Key::Character("f" | "F") => state.fit_selected_or_all(),
            Key::Character("q" | "Q") => state.gpu.view.show_points = !state.gpu.view.show_points,
            Key::Character("w" | "W") => state.gpu.view.show_lines = !state.gpu.view.show_lines,
            Key::Character("e" | "E") => {
                state.gpu.view.show_mesh_edges = !state.gpu.view.show_mesh_edges
            }
            Key::Character("o" | "O") => {
                state.gpu.view.show_outlines = !state.gpu.view.show_outlines
            }
            Key::Character("d" | "D") => state.gpu.view.lit = !state.gpu.view.lit,
            Key::Character("h" | "H") => state.hide_selected(),
            Key::Character("s" | "S") => state.show_all(),
            Key::Character("t" | "T") => state.toggle_selected_names(),
            Key::Character("b" | "B") => state.gpu.view.backface = !state.gpu.view.backface,
            Key::Character("p" | "P") => state.toggle_xray(),
            Key::Character("[") => state.set_cloud_size(state.gpu.view.cloud_size - 0.25),
            Key::Character("]") => state.set_cloud_size(state.gpu.view.cloud_size + 0.25),
            _ => return false,
        }

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
                let position =
                    winit::dpi::PhysicalPosition::new(position.x * scale, position.y * scale);

                if self.control_drag {
                    self.last_cursor = (position.x, position.y);
                    return state.drag_control(position.x, position.y);
                }

                if self.gizmo_drag {
                    self.last_cursor = (position.x, position.y);
                    return state.drag_gizmo(position.x, position.y);
                }

                let dragging = self.orbiting || self.panning;

                // camera moves in CSS pixels
                if dragging {
                    let dx = ((position.x - self.last_cursor.0) / device_pixel_ratio()) as f32;
                    let dy = ((position.y - self.last_cursor.1) / device_pixel_ratio()) as f32;

                    if self.panning || self.ctrl {
                        state.camera.pan(dx, dy);
                    } else {
                        state.camera.orbit(dx, dy);
                    }
                }

                self.last_cursor = (position.x, position.y);
                dragging || state.hover_gizmo(position.x, position.y)
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
// --8<-- [start:step-5c]

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
                    self.gizmo_drag = false;
                    self.control_drag = false;
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

                // a first finger may grab a control or a handle
                if t.phase == TouchPhase::Started && self.fingers.len() == 1 {
                    self.last_cursor = (t.location.x, t.location.y);
                    self.control_drag = state.begin_control_drag(t.location.x, t.location.y);
                    self.gizmo_drag =
                        !self.control_drag && state.begin_gizmo_touch(t.location.x, t.location.y);

                    if self.control_drag || self.gizmo_drag {
                        self.touch_edit = Some(t.id);
                    }
                }

                // the editing finger
                if self.touch_edit == Some(t.id) {
                    self.last_cursor = (t.location.x, t.location.y);

                    match t.phase {
                        TouchPhase::Moved => {
                            if self.control_drag {
                                state.drag_control(t.location.x, t.location.y);
                            } else {
                                state.drag_gizmo(t.location.x, t.location.y);
                            }
                        }
                        TouchPhase::Ended => {
                            if self.control_drag {
                                state.end_control_drag(t.location.x, t.location.y);
                            } else {
                                state.end_gizmo(t.location.x, t.location.y);
                            }
                        }
                        TouchPhase::Cancelled => state.cancel_gesture(),
                        TouchPhase::Started => {}
                    }

                    if matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                        self.fingers.remove(&t.id);
                        self.touch_edit = None;
                        self.control_drag = false;
                        self.gizmo_drag = false;
                        self.touch = Touches::new();
                    }

                    state.interacting = self.touch_edit.is_some();
                    return true;
                }

                if matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                    self.fingers.remove(&t.id);
                }

// --8<-- [end:step-5c]
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
        self.gizmo_drag = false;
        self.control_drag = false;
        self.left_down = None;
        self.touch = Touches::new();
        // --8<-- [start:step-5d]
        self.touch_edit = None;
        self.fingers.clear();
        self.touch_cancelled = false;
        // --8<-- [end:step-5d]
    }

    /// Left button: control drag, then gizmo drag, then a click.
    fn left(&mut self, state: &mut State, btn: ElementState) -> bool {
        match btn {
            ElementState::Pressed => {
                if !self.ctrl && state.begin_control_drag(self.last_cursor.0, self.last_cursor.1) {
                    self.control_drag = true;
                    return false;
                }

                if !self.ctrl && state.begin_gizmo(self.last_cursor.0, self.last_cursor.1) {
                    self.gizmo_drag = true;
                    return false;
                }

                self.left_down = Some(self.last_cursor);
                false
            }
            ElementState::Released => {
                if self.control_drag {
                    self.control_drag = false;
                    state.end_control_drag(self.last_cursor.0, self.last_cursor.1);
                    return true;
                }

                if self.gizmo_drag {
                    self.gizmo_drag = false;
                    state.end_gizmo(self.last_cursor.0, self.last_cursor.1);
                    return true;
                }

                let Some(down) = self.left_down.take() else {
                    return false;
                };
                let moved = (self.last_cursor.0 - down.0)
                    .abs()
                    .max((self.last_cursor.1 - down.1).abs());

                if moved > CLICK_SLOP * device_pixel_ratio() {
                    return false; // a drag, not a click
                }

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
    canvas: web_sys::HtmlCanvasElement,                                 // the canvas listened to
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
