            super::ui::command_line::STATE // register:commands
                .with_borrow_mut(|model| model.status = message.to_string()); // register:commands
