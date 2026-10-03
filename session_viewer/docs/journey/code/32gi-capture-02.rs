use crate::{editor::Action, scene::ObjectId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intent {
    Move { id: ObjectId, offset: [f64; 3] },
    Delete { id: ObjectId },
    Save,
}

impl Intent {
    pub fn capture(action: &Action, selected: Option<ObjectId>) -> Result<Option<Self>, &'static str> {
        Ok(match action {
            Action::Translate(offset) => {
                let id = selected.ok_or("Select an object before Move")?;
                if !offset.iter().all(|value| value.is_finite()) { return Err("Move needs finite coordinates"); }
                if offset.iter().all(|&value| value == 0.0) { None }
                else { Some(Self::Move { id, offset: *offset }) }
            }
            Action::Delete => selected.map(|id| Self::Delete { id }),
            _ => None,
        })
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor::Editor, reload_url::ReloadUrl};
    use std::rc::Rc;

    fn cold() -> Editor {
        let mut editor = Editor::default();
        editor.apply(Action::ReplaceAt(crate::specimen::bytes(), Rc::new(ReloadUrl::new("test:intent".into())))).unwrap();
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::UnloadSources).unwrap();
        editor
    }

    #[test]
    fn move_keeps_original_target_and_arguments_after_selection_changes() {
        let mut editor = cold(); let id = editor.selected.unwrap();
        let origin = Rc::downgrade(editor.scene.objects()[0].origin().unwrap());
        let intent = Intent::capture(&Action::Translate([0.125, -0.25, 0.5]), editor.selected).unwrap().unwrap();
        editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Orbit(0.1, 0.2)).unwrap();
        assert_ne!(editor.selected, Some(id));
        assert_eq!(intent, Intent::Move { id, offset: [0.125, -0.25, 0.5] });
        editor.apply(Action::Close).unwrap();
        assert!(origin.upgrade().is_none(), "intent owns no source or URL");
    }

    #[test]
    fn delete_keeps_its_original_target() {
        let mut editor = cold(); let id = editor.selected.unwrap();
        let intent = Intent::capture(&Action::Delete, editor.selected).unwrap().unwrap();
        editor.apply(Action::SelectNext).unwrap();
        assert_eq!(intent, Intent::Delete { id });
        assert_ne!(editor.selected, Some(id));
    }

    #[test]
    fn empty_edits_do_not_request_io_and_invalid_moves_are_rejected() {
        assert_eq!(Intent::capture(&Action::Delete, None).unwrap(), None);
        assert!(Intent::capture(&Action::Translate([0.0; 3]), None).is_err());
        let editor = Editor::default(); let id = editor.scene.objects()[0].id;
        assert_eq!(Intent::capture(&Action::Translate([0.0; 3]), Some(id)).unwrap(), None);
        assert!(Intent::capture(&Action::Translate([f64::NAN, 0.0, 0.0]), Some(id)).is_err());
        assert_eq!(Intent::capture(&Action::Fit, Some(id)).unwrap(), None);
    }
}
