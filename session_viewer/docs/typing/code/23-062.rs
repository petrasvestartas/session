    #[cfg(target_arch = "wasm32")] // register:phone
    agent_value: String, // last text taken from the hidden input; register:phone
    #[cfg(target_arch = "wasm32")] // register:phone
    field: Option<&'static str>, // the field the hidden input fed last frame, None for the command line; register:phone
