// --8<-- [start:pointer-event]
// A child module sees its parent's private items, so this file reads Ui's fields as if it were mod.rs.
use super::{PANELS, Ui, hit, keys_taken};
use winit::window::Window;

// Routing = deciding who gets each event: a press on a panel stays in egui, a press on the scene reaches the viewer.
impl Ui {
    /// Offer one event to the panels; (consumed, needs repaint).
    pub fn event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> (bool, bool) {
        // egui always sees the event; `consumed` then says whether the viewer should not
        let response = self.input.on_window_event(window, event);
        let escape = matches!(event, winit::event::WindowEvent::KeyboardInput { event, .. }
            if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape))
            && PANELS.iter().any(|panel| panel.closes_on_escape());
        // a `use` inside a function names these types for this function only
        use winit::event::{ElementState, TouchPhase, WindowEvent};
        // winit reports device pixels, egui works in points: at ratio 2.0, x = 400 is point 200
        let ratio = window.scale_factor() as f32;
        let mut consumed = response.consumed;
        let mut scene_rect = self.scene_rect;
        scene_rect.max.y -= 5.0; // the lowest 5 points of the scene count as panel: the edge of what is docked below
        // the closure below borrows this clone, leaving `self` free to change inside the match
        let context = self.context.clone();
        // the completion list, the number box, a context menu or a colour menu
        let in_popup = |point| {
            hit(point).1
                || context
                    .layer_id_at(point)
                    .is_some_and(|layer| layer.order != egui::Order::Background)
        };

        match event {
            // the current pointer decides, not last frame's hover: a fast move may cross a panel edge between frames
            WindowEvent::CursorMoved { position, .. } => {
                self.pointer = egui::pos2(position.x as f32 / ratio, position.y as f32 / ratio);
                self.over_panel = in_popup(self.pointer) || !scene_rect.contains(self.pointer);
                consumed = self.ui_drag || self.over_panel;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *state == ElementState::Pressed {
                    self.ui_drag = in_popup(self.pointer) || !scene_rect.contains(self.pointer);
                    self.over_panel = self.ui_drag;
                    self.press();
                }

                consumed = self.ui_drag;

                // a left drag from the scene let go over a panel is dropped, not applied
                if *state == ElementState::Released {
                    consumed |= *button == winit::event::MouseButton::Left && self.over_panel;
                    self.ui_drag = false;
                }
            }
            WindowEvent::MouseWheel { .. } => {
                consumed = in_popup(self.pointer) || !scene_rect.contains(self.pointer)
            }
            // the first finger decides for the whole gesture: a pinch that starts on the scene stays on the scene
            WindowEvent::Touch(touch) => {
                self.pointer = egui::pos2(
                    touch.location.x as f32 / ratio,
                    touch.location.y as f32 / ratio,
                );

                if touch.phase == TouchPhase::Started {
                    if self.touches.is_empty() {
                        self.ui_drag = in_popup(self.pointer) || !scene_rect.contains(self.pointer);
                        self.press();
                    }

                    self.touches.insert(touch.id);
                }

                consumed = self.ui_drag;

                if matches!(touch.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                    self.touches.remove(&touch.id);

                    if self.touches.is_empty() {
                        self.ui_drag = false;
                    }
                }
            }
            WindowEvent::Focused(false) => {
                self.ui_drag = false;
                self.touches.clear();
            }
            _ => {}
        }

        // an open menu or number box keeps the keys too, so Escape only closes it
        if matches!(event, WindowEvent::KeyboardInput { .. }) && keys_taken() {
            consumed = true;
        }
        (consumed || escape, response.repaint || escape)
    }
    // --8<-- [end:pointer-event]

    // --8<-- [start:pointer-press]
    /// A press: the panel whose field it lands on opens it.
    fn press(&mut self) {
        for panel in PANELS {
            panel.press(&self.context, self.pointer);
        }
    }
}
// --8<-- [end:pointer-press]
