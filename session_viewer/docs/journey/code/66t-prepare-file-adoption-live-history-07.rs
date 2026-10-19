            Self::Move { id, offset } => editor.move_object(id, offset).map(Reply::Changed),
            Self::Delete { id } => editor.delete_object(id).map(Reply::Changed),
            Self::Save => crate::scene_archive::save(&editor.scene).map(Reply::Saved).map_err(|_|"Cannot save viewer session; reload editable sources and try again"),
        }
    }
