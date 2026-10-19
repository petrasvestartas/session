        if focused && !text_empty { return None; }
        match text.to_ascii_lowercase().as_str() {
            "a" => Some(Shortcut::SelectAll),
            "z" if shift => Some(Shortcut::Redo),
            "z" => Some(Shortcut::Undo),
