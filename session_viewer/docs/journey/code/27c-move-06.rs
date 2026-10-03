            .any(|name| name.eq_ignore_ascii_case(&line)
                || (*name == "Move" && line.to_ascii_lowercase().starts_with("move ")))
