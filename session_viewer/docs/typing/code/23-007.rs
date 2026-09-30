
/// Replace the scene with a saved one.
#[cfg(target_arch = "wasm32")]
fn open_saved(state: &mut State, scene: Box<app::scene::Scene>) {
    state.clear();
    state.scene = *scene;
    state.scene.upload_to(&mut state.gpu);
    state.scene.restore_text_visibility(&mut state.gpu); // register:scene_text
    state.update_label(); // the saved texts reach the GPU; register:scene_text
    state.fit_all();
    state.touch();
    app::feedback::status("Session opened");
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Listen to the hidden input that raises the phone keyboard.
    fn listen_agent(&mut self, canvas: web_sys::HtmlCanvasElement, proxy: &EventLoopProxy<Msg>) {
        match app::agent::CommandAgent::new(canvas, proxy.clone()) {
            Ok(agent) => self.agent = Some(agent),
            Err(error) => log::warn!("Cannot register the command agent: {error:?}"),
        }
    }

    /// Phone keys become key presses.
    fn agent_keys(&mut self, event: app::agent::AgentEvent) {
        let Some(state) = &mut self.state else { return };

        if let Some(ui) = self.ui.as_mut() {
            for key in ui.agent(event) {
                self.input
                    .key(state, winit::keyboard::Key::Character(key.as_str()));
            }
        }

        state.touch();
    }
}
