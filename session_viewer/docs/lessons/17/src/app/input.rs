use super::touch::{Act, Touches};
use crate::State;
use crate::camera::View;
// --8<-- [start:step-8a]
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};
// --8<-- [end:step-8a]
use winit::keyboard::{Key, NamedKey};

/// A press moving less than this many pixels is a click.
const CLICK_SLOP: f64 = 4.0;

/// Mouse, keyboard and finger state between events.
pub struct Input {
    orbiting: bool,                          // right button held
    panning: bool,                           // middle button held
    ctrl: bool,                              // Ctrl held
    // --8<-- [start:step-8b]
    shift: bool,                             // Shift held
    // --8<-- [end:step-8b]
    last_cursor: (f64, f64),                 // last pointer position in pixels
    left_down: Option<(f64, f64)>,           // where the left button went down
    touch: Touches,                          // camera finger gestures
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
            // --8<-- [start:step-8c]
            shift: false,
            // --8<-- [end:step-8c]
            last_cursor: (0.0, 0.0),
            left_down: None,
            touch: Touches::new(),
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
            // --8<-- [start:step-8d]
            Key::Character("o" | "O") => {
                state.gpu.view.show_outlines = !state.gpu.view.show_outlines
            }
            // --8<-- [end:step-8d]
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
                // --8<-- [start:step-8e]
                state.interacting = self.orbiting || self.panning;
                // --8<-- [end:step-8e]
                false
            }
            WindowEvent::MouseInput {
                state: btn,
                button: MouseButton::Middle,
                ..
            } => {
                self.panning = *btn == ElementState::Pressed;
                // --8<-- [start:step-8f]
                state.interacting = self.orbiting || self.panning;
                // --8<-- [end:step-8f]
                false
            }
            WindowEvent::MouseInput {
                state: btn,
                button: MouseButton::Left,
                ..
            } => self.left(state, *btn),
            WindowEvent::CursorMoved { position, .. } => {
                // --8<-- [start:step-28]
                let scale = crate::engine::gpu::view::surface_per_physical(); // window to canvas pixels
                let position =
                    winit::dpi::PhysicalPosition::new(position.x * scale, position.y * scale);
                    // --8<-- [end:step-28]
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
                dragging
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
                // --8<-- [start:step-37a]
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
                state.interacting = matches!(t.phase, TouchPhase::Started | TouchPhase::Moved);

                // otherwise the fingers move the camera
// --8<-- [end:step-37a]
                match self
                    .touch
                    .event(&mut state.camera, t, viewport, device_pixel_ratio())
                {
                    Act::None => false,
                    Act::Moved => true,
                    Act::Tap(at) => {
                        // --8<-- [start:step-37b]
                        state.request_selection(at.0 as u32, at.1 as u32, false, false);
                        // --8<-- [end:step-37b]
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
        // --8<-- [start:step-37c]
        self.shift = false;
        // --8<-- [end:step-37c]
        self.left_down = None;
        self.touch = Touches::new();
    }

    /// Left button: control drag, then gizmo drag, then a click.
    fn left(&mut self, state: &mut State, btn: ElementState) -> bool {
        match btn {
            ElementState::Pressed => {
                self.left_down = Some(self.last_cursor);
                false
            }
            ElementState::Released => {
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
                    // --8<-- [start:step-37d]
                    self.ctrl && self.shift,
                    // --8<-- [end:step-37d]
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
        proxy: winit::event_loop::EventLoopProxy<crate::Msg>,
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

// --8<-- [start:step-37e]
/// Physical pixels per CSS pixel.
fn device_pixel_ratio() -> f64 {
    crate::engine::gpu::view::device_pixel_ratio()
    // --8<-- [end:step-37e]
}
