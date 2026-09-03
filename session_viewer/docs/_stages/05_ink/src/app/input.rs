//! Every binding: RMB orbits, MMB (or Ctrl+RMB) pans, the wheel zooms toward the cursor;
//! 1-7 named views, Space projection, C reset, F fit, Q/W/E lane toggles,
//! L line style. Fingers go to `touch.rs`.
//! Every handler says whether the frame must be redrawn.

use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, NamedKey};
use crate::camera::View;
use crate::State;
use super::touch::{Act, Touches};

/// What the mouse is doing between events, plus the fingers.
pub struct Input {
    orbiting: bool,
    panning: bool,
    ctrl: bool,
    last_cursor: (f64, f64),
    touch: Touches,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    /// Nothing held, cursor at the origin.
    pub fn new() -> Self {
        Self { orbiting: false, panning: false, ctrl: false, last_cursor: (0.0, 0.0), touch: Touches::new() }
    }

    /// One key press (the caller filters repeats). True when the frame must be redrawn.
    pub fn key(&mut self, state: &mut State, key: Key<&str>) -> bool {
        match key {
            Key::Named(NamedKey::Space) => state.camera.toggle_projection_framed(&state.gpu.bounds, state.aspect()),
            Key::Character("1") => state.camera.set_view(View::Front),
            Key::Character("2") => state.camera.set_view(View::Back),
            Key::Character("3") => state.camera.set_view(View::Left),
            Key::Character("4") => state.camera.set_view(View::Right),
            Key::Character("5") => state.camera.set_view(View::Top),
            Key::Character("6") => state.camera.set_view(View::Bottom),
            Key::Character("7") => state.camera.set_view(View::Iso),
            Key::Character("c" | "C") => state.camera.reset(),
            Key::Character("f" | "F") => state.fit_all(),
            Key::Character("q" | "Q") => state.gpu.view.show_points = !state.gpu.view.show_points,
            Key::Character("w" | "W") => state.gpu.view.show_lines = !state.gpu.view.show_lines,
            Key::Character("e" | "E") => state.gpu.view.show_mesh_edges = !state.gpu.view.show_mesh_edges,
            Key::Character("l" | "L") => state.gpu.view.toggle_line_style(),
            _ => return false,
        }
        true
    }

    /// Buttons, motion, wheel, modifiers and fingers. True when the frame must be redrawn.
    pub fn mouse(&mut self, state: &mut State, event: &WindowEvent) -> bool {
        let viewport = state.viewport();
        match event {
            WindowEvent::MouseInput { state: btn, button: MouseButton::Right, .. } => {
                self.orbiting = *btn == ElementState::Pressed;
                false
            }
            WindowEvent::MouseInput { state: btn, button: MouseButton::Middle, .. } => {
                self.panning = *btn == ElementState::Pressed;
                false
            }
            WindowEvent::CursorMoved { position, .. } => {
                let dragging = self.orbiting || self.panning;
                if dragging {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
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
                false
            }
            WindowEvent::Touch(t) => match self.touch.event(&mut state.camera, t, viewport, device_pixel_ratio()) {
                Act::None => false,
                Act::Moved => true,
                Act::Fit => {
                    state.fit_all();
                    true
                }
            },
            _ => false,
        }
    }
}

/// Physical pixels per CSS pixel: 1 on a desktop monitor, 2-4 on a phone.
#[cfg(target_arch = "wasm32")]
fn device_pixel_ratio() -> f64 {
    web_sys::window().map(|w| w.device_pixel_ratio()).filter(|d| *d > 0.0).unwrap_or(1.0)
}

/// Native windows report logical pixels already.
#[cfg(not(target_arch = "wasm32"))]
fn device_pixel_ratio() -> f64 {
    1.0
}
