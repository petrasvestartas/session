use crate::{edit_intent::Intent, editor::{Change, Editor}};

#[derive(Debug, PartialEq)]
pub enum Reply { Changed(Change), Saved(Vec<u8>) }

impl Intent {
    pub fn replay(self, editor: &mut Editor) -> Result<Reply, &'static str> {
        match self {
            Self::Move { id, offset } => editor.move_object(id, offset).map(Reply::Changed),
            Self::Delete { id } => editor.delete_object(id).map(Reply::Changed),
            Self::Save => crate::document::snapshot(&editor.scene).map(Reply::Saved),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Action, reload_url::ReloadUrl};
    use prost::Message;
    use std::rc::Rc;

    #[test]
    fn save_reply_keeps_precision_and_does_not_consume_move_undo() {
        let mut e = Editor::default();
        e.apply(Action::Replace(crate::specimen::bytes())).unwrap();
        let id = e.scene.objects()[0].id; let original = e.scene.objects()[0].geometry().unwrap().to_proto();
        let later = e.scene.objects()[1].id; e.selected = Some(later); let camera = e.camera.uniform();
        let model = e.scene.objects()[0].model.m;
        assert_eq!(Intent::Move { id, offset: [0.25, 0.0, 0.0] }.replay(&mut e).unwrap(), Reply::Changed(Change::Scene));
        let Reply::Saved(bytes) = Intent::Save.replay(&mut e).unwrap() else { panic!("Save reply") };
        let saved = session_rust::proto::Session::decode(bytes.as_slice()).unwrap();
        assert_eq!(saved.objects.unwrap().meshes[0].vertices, original.vertices);
        e.apply(Action::Undo).unwrap(); assert_eq!(e.scene.objects()[0].model.m, model);
        assert_eq!(e.selected, Some(later)); assert_eq!(e.camera.uniform(), camera);
        assert_eq!(Intent::Delete { id }.replay(&mut e).unwrap(), Reply::Changed(Change::Scene));
        assert!(!e.scene.contains(id)); assert_eq!(e.selected, Some(later));
        e.apply(Action::Undo).unwrap(); assert!(e.scene.contains(id));
    }

    #[test]
    fn cold_save_reply_fails_without_losing_redo() {
        let mut e = Editor::default();
        e.apply(Action::ReplaceAt(crate::specimen::bytes(), Rc::new(ReloadUrl::new("test:reply".into())))).unwrap();
        e.apply(Action::UnloadSources).unwrap();
        e.apply(Action::AddBox).unwrap(); let id = e.scene.objects().last().unwrap().id;
        e.apply(Action::Undo).unwrap(); assert!(Intent::Save.replay(&mut e).is_err());
        e.apply(Action::Redo).unwrap(); assert!(e.scene.contains(id));
    }
}
