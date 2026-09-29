
#[cfg(target_arch = "wasm32")]
impl App {
    /// The egui panels and their GPU painter.
    fn adopt_panels(&mut self, state: &mut State) {
        self.ui = Some(app::ui::Ui::new(&state.window, state.logical_size()[0]));
        state.gpu.ui = Some(engine::gpu::ui::Ui::new(
            &state.gpu.ctx,
            state.gpu.config.format,
        ));
    }

    /// The panels take the fonts too.
    fn panel_fonts(&mut self, faces: [&'static [u8]; 3]) {
        if let Some(ui) = self.ui.as_mut() {
            ui.use_fonts(faces);
        }
    }

    /// The panels get the event first; true when they took it.
    fn panels_take(&mut self, event: &WindowEvent) -> bool {
        let Some(state) = &mut self.state else {
            return false;
        };
        let Some(ui) = self.ui.as_mut() else {
            return false;
        };
        let (mut consumed, repaint) = ui.event(&state.window, event);

        // keys reach the viewer unless a text field or a menu has them; the number box from its click on
        if matches!(event, WindowEvent::KeyboardInput { .. }) {
            consumed = app::ui::keys_taken() || state.number_box_open();
        }

        if repaint {
            state.request_frame();
        }

        // a command following a left drag, e.g. a lasso, keeps the pointer over panels too
        let held = self.input.tool_held()
            && matches!(
                event,
                WindowEvent::CursorMoved { .. }
                    | WindowEvent::MouseInput {
                        button: winit::event::MouseButton::Left,
                        ..
                    }
            );

        if consumed && !held {
            // a release inside a panel ends any viewer drag
            if matches!(
                event,
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    ..
                } | WindowEvent::Touch(winit::event::Touch {
                    phase: winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled,
                    ..
                })
            ) {
                self.input.cancel();
                state.cancel_gesture();
            }

            return true;
        }

        false
    }
}

/// The panels asked for another frame.
#[cfg(target_arch = "wasm32")]
fn repaint_if(state: &mut State, repaint: bool) {
    if repaint {
        state.request_frame();
    }
}
