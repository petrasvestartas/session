    #[cfg(target_arch = "wasm32")] // register:commands
    super::ui::command_line::STATE // register:commands
        .with_borrow_mut(|model| model.status = message.chars().take(256).collect()); // register:commands
