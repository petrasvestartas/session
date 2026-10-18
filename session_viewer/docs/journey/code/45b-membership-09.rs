        if let Some(action) = action {
            highlight_changed |= matches!(&action, Action::SelectAll);
