    }

    pub fn repeat_line(&self)->Option<String> {
        if !self.model.command.trim().is_empty() {return Some(self.model.command.clone());}
        self.model.history.iter().rev().find_map(|line|line.strip_prefix("> ").and_then(|line|line.lines().next()).map(str::to_owned))
    }

    pub fn command(&self) -> &str { &self.model.command }

