impl Intent {
    pub fn keys(&self, editor: &Editor) -> Result<Vec<ReloadKey>, &'static str> {
        match self {
            Self::Save => Ok(editor.reload_keys()),
            Self::Move { id, .. } | Self::Delete { id } => {
                let row = editor.scene.objects().iter().find(|row| row.id == *id).ok_or("Object not found")?;
                Ok(ReloadKey::of(row).into_iter().collect())
            }
        }
    }

