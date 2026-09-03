//! The gesture state machine and every key binding: RMB orbits, MMB (or Ctrl+RMB) pans, the
//! wheel zooms toward the cursor; 1-7 named views, Space projection, C reset, F fit, Q/W/E/L
//! lane toggles, [ ] cloud size. Mutates `camera` and `gpu.view` and says whether to redraw.

use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, NamedKey};
use crate::camera::View;
use crate::engine::gpu::LineStyle;
use crate::State;

/// What the mouse is doing between events.
pub struct Input {
    pub orbiting: bool,
    pub panning: bool,
    pub last_cursor: (f64, f64),
    pub ctrl: bool,
}

impl Input {
    /// Nothing held, cursor at the origin.
    pub fn new() -> Self {
        Self { orbiting: false, panning: false, last_cursor: (0.0, 0.0), ctrl: false }
    }

    /// One key press (the caller filters repeats). True when the frame must be redrawn.
    pub fn key(&mut self, state: &mut State, key: Key<&str>) -> bool {
        match key {
            Key::Named(NamedKey::Space) => {
                let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                state.camera.toggle_projection_framed(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
            }
            Key::Character("1") => state.camera.set_view(View::Front),
            Key::Character("2") => state.camera.set_view(View::Back),
            Key::Character("3") => state.camera.set_view(View::Left),
            Key::Character("4") => state.camera.set_view(View::Right),
            Key::Character("5") => state.camera.set_view(View::Top),
            Key::Character("6") => state.camera.set_view(View::Bottom),
            Key::Character("7") => state.camera.set_view(View::Iso),
            Key::Character("c" | "C") => state.camera.reset(),
            // Q / W / E hide a whole KIND of thing so an overlap can be taken apart by eye; L
            // draws the SOLID lane's edges as tubes or as flat quads - same table, a free A/B.
            Key::Character("q" | "Q") => {
                state.gpu.view.show_points = !state.gpu.view.show_points;
                log::info!("points: {}", state.gpu.view.show_points);
            }
            Key::Character("w" | "W") => {
                state.gpu.view.show_lines = !state.gpu.view.show_lines;
                log::info!("lines: {}", state.gpu.view.show_lines);
            }
            Key::Character("e" | "E") => {
                state.gpu.view.show_mesh_edges = !state.gpu.view.show_mesh_edges;
                log::info!("mesh edges: {}", state.gpu.view.show_mesh_edges);
            }
            Key::Character("l" | "L") => {
                state.gpu.view.line_style = match state.gpu.view.line_style {
                    LineStyle::Tubes => LineStyle::Flat,
                    LineStyle::Flat => LineStyle::Tubes,
                };
                log::info!("line style: {:?}", state.gpu.view.line_style);
            }
            // live cloud point size
            Key::Character("[") => {
                state.gpu.view.cloud_size = (state.gpu.view.cloud_size - 0.25).max(0.25);
                log::info!("cloud size scale: x{}", state.gpu.view.cloud_size);
            }
            Key::Character("]") => {
                state.gpu.view.cloud_size = (state.gpu.view.cloud_size + 0.25).min(8.0);
                log::info!("cloud size scale: x{}", state.gpu.view.cloud_size);
            }
            Key::Character("f" | "F") => {
                let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                state.camera.fit(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
            }
            _ => return false,
        }
        true
    }

    /// Buttons, motion, wheel and modifiers. True when the camera moved.
    pub fn mouse(&mut self, state: &mut State, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput { state: btn, button: MouseButton::Right, .. } => {
                self.orbiting = *btn == ElementState::Pressed; // hold RMB to orbit
                false
            }
            WindowEvent::MouseInput { state: btn, button: MouseButton::Middle, .. } => {
                self.panning = *btn == ElementState::Pressed; // hold MMB to pan (CAD standard)
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
                        state.camera.orbit(dx, dy)
                    };
                }
                self.last_cursor = (position.x, position.y);
                dragging
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 100.0,
                };
                // Zoom toward the cursor - the point under the mouse stays put
                let vp = (state.gpu.config.width as f64, state.gpu.config.height as f64);
                state.camera.zoom_at(amount, self.last_cursor, vp);
                true
            }
            WindowEvent::ModifiersChanged(mods) => {
                self.ctrl = mods.state().control_key();
                false
            }
            _ => false,
        }
    }
}
