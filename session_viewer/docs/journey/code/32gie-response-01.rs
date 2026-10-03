impl Intent {
    pub fn command(self) -> &'static str {
        match self { Self::Move { .. } => "Move", Self::Delete { .. } => "Delete", Self::Save => "Save" }
    }

