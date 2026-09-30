    #[cfg(target_arch = "wasm32")] // register:phone
    Agent(app::agent::AgentEvent), // a phone keyboard key; register:phone
    SavedScene(Box<app::scene::Scene>), // a saved session loaded; register:commands
