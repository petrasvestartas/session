//! Every binding: RMB orbits, MMB (or Ctrl+RMB) pans, the wheel zooms toward the cursor, a
//! left click picks the object, Ctrl+click an edge and Ctrl+Shift+click a face, and a left
//! drag on a gizmo handle moves the selection; Delete removes it, Ctrl+Z undoes and
//! Ctrl+Shift+Z or Ctrl+Y redoes;
//! 1-7 named views, Space projection, C reset, F fits the selection (or
//! everything with none selected), Q/W/E lane toggles, O silhouettes, D face lighting,
//! B the back-face flag, L the layers panel, : the command line,
//! H hides the selection and S shows everything back, T toggles selected names,
//! P toggles x-ray (faces gone, edges stay), F10 shows the selected object's source controls,
//! [ ] cloud size, Escape clears the selection. Fingers go to `touch.rs`.
//! Every handler says whether the frame must be redrawn.

use super::touch::{Act, Touches};
use crate::State;
use crate::camera::View;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::keyboard::{Key, NamedKey};

/// A press that moves less than this (CSS px) before release is a click.
const CLICK_SLOP: f64 = 4.0;

/// What the mouse is doing between events, plus the fingers.
pub struct Input {
    orbiting: bool,
    panning: bool,
    ctrl: bool,
    shift: bool,
    /// A gizmo handle is being dragged, so the pointer belongs to the widget and neither the
    /// camera nor the picker sees it until it is let go.
    gizmo_drag: bool,
    /// A control point is being dragged: the same press, a different gesture.
    control_drag: bool,
    last_cursor: (f64, f64),
    left_down: Option<(f64, f64)>,
    touch: Touches,
}

impl Default for Input {
    /// Start with the same inactive gesture state as the explicit constructor.
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
        }
    }

    /// One key press (the caller filters repeats). True when the frame must be redrawn.
    pub fn key(&mut self, state: &mut State, key: Key<&str>) -> bool {
        match key {
            Key::Named(NamedKey::Space) => state
                .camera
                .toggle_projection_framed(&state.gpu.bounds, state.aspect()),
            Key::Named(NamedKey::Escape) => state.escape_selection(),
            Key::Named(NamedKey::F10) => state.enable_controls(),
            Key::Named(NamedKey::Delete) => state.delete_selected(),
            // The colon opens the command box, the way a modal editor does. The box then holds
            // the keyboard, so the letters typed into it never reach these bindings.
            Key::Character(":") => {
                crate::app::feedback::command_line(true);
            }
            Key::Character("l" | "L") => state.toggle_layers_panel(),
            // Ctrl+Z back, Ctrl+Shift+Z or Ctrl+Y forward: the two spellings every editor takes.
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

    /// Buttons, motion, wheel, modifiers and fingers. True when the frame must be redrawn.
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
                let scale = crate::engine::gpu::view::surface_per_physical();
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
                state.interacting = matches!(t.phase, TouchPhase::Started | TouchPhase::Moved);
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

    /// Focus loss or pointer cancellation retires every incomplete mouse/touch gesture.
    pub fn cancel(&mut self) {
        self.orbiting = false;
        self.panning = false;
        self.ctrl = false;
        self.shift = false;
        self.gizmo_drag = false;
        self.control_drag = false;
        self.left_down = None;
        self.touch = Touches::new();
    }

    /// The left button. A press is offered to the control drag, then to the gizmo, then kept
    /// as the start of a click: both widgets sit over the object they move, so a press that
    /// lands on one is never also a pick of what is behind it.
    ///
    /// Ctrl reserves the press for sub-selection, even when a handle overlaps the source.
    /// A release within the slop is a click and asks the GPU what is under it. The picture is
    /// unchanged until the answer lands, so a click never redraws by itself.
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
                    return false;
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

/// Winit handles touch cancellation; this owned listener also covers mouse and pen cancellation.
#[cfg(target_arch = "wasm32")]
pub struct PointerCancellation {
    canvas: web_sys::HtmlCanvasElement,
    callback: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>,
}

#[cfg(target_arch = "wasm32")]
impl PointerCancellation {
    /// Install once for the canvas lifetime; the required callback only forwards a message.
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
    /// Detach before dropping the wasm callback so JavaScript cannot retain an invalid handle.
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;
        let _ = self.canvas.remove_event_listener_with_callback(
            "pointercancel",
            self.callback.as_ref().unchecked_ref(),
        );
    }
}

/// Pointer cancellation goes through the same event-loop owner as every other input change.
#[cfg(target_arch = "wasm32")]
fn cancel_pointer(proxy: &winit::event_loop::EventLoopProxy<crate::Msg>) {
    let _ = proxy.send_event(crate::Msg::CancelPointer);
}

/// Physical pixels per CSS pixel: 1 on a desktop monitor, 2-4 on a phone, the same capped
/// ratio the canvas is rendered at. Native windows report logical pixels already.
fn device_pixel_ratio() -> f64 {
    crate::engine::gpu::view::device_pixel_ratio()
}
