use super::command_line::STATE;
use super::{Ui, hit, keys_taken};
use winit::window::Window;

impl Ui {
    /// Offer one event to the panels; (consumed, needs repaint).
    pub fn event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> (bool, bool) {
        let response = self.input.on_window_event(window, event);
        let escape = matches!(event, winit::event::WindowEvent::KeyboardInput { event, .. }
            if event.logical_key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape))
            && STATE.with_borrow(|model| model.command_open);
        // use the current pointer, not last frame's hover
        use winit::event::{ElementState, TouchPhase, WindowEvent};
        let ratio = window.scale_factor() as f32;
        let mut consumed = response.consumed;
        let mut scene_rect = self.scene_rect;
        scene_rect.max.y -= 5.0;
        let context = self.context.clone();
        // the completion list, the number box, a context menu or a colour menu
        let in_popup = |point| {
            hit(point).1
                || context
                    .layer_id_at(point)
                    .is_some_and(|layer| layer.order != egui::Order::Background)
        };

        match event {
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

    /// A press on the field opens it, anywhere else but the list closes it.
    fn press(&mut self) {
        let id = egui::Id::new("command-input");
        let (input, popup) = STATE.with_borrow(|m| {
            (
                m.command_rect.is_some_and(|r| r.contains(self.pointer)),
                m.completion_rect.is_some_and(|r| r.contains(self.pointer)),
            )
        });

        // focus now so the first key is not lost
        if input {
            self.context.memory_mut(|memory| memory.request_focus(id));
            STATE.with_borrow_mut(|model| model.command_open = true);
        } else if !popup {
            self.context.memory_mut(|memory| memory.surrender_focus(id));
            STATE.with_borrow_mut(|model| model.command_open = false);
        }
    }
}
