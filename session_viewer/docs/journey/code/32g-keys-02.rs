use std::rc::Rc;

#[derive(Clone)]
pub struct ReloadKey {
    pub origin: Rc<crate::origin::Origin>,
    pub epoch: u64,
}

impl ReloadKey {
    pub fn of(row: &crate::scene::Object) -> Option<Self> {
        Some(Self { origin: Rc::clone(row.origin()?), epoch: row.release_epoch()? })
    }

    pub fn matches(&self, row: &crate::scene::Object) -> bool {
        row.release_epoch() == Some(self.epoch)
            && row.origin().is_some_and(|origin| Rc::ptr_eq(origin, &self.origin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::{Action, Editor};

    #[test]
    fn a_reload_key_matches_only_its_current_import_release() {
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:key".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap();
        assert!(ReloadKey::of(&editor.scene.objects()[0]).is_none());
        editor.apply(Action::UnloadSources).unwrap();
        let key = ReloadKey::of(&editor.scene.objects()[0]).unwrap();
        assert!(editor.scene.objects().iter().all(|row| key.matches(row)));
        let later = ReloadKey { origin: Rc::clone(&key.origin), epoch: key.epoch + 1 };
        assert!(!later.matches(&editor.scene.objects()[0]));
        editor.apply(Action::Close).unwrap();
        editor.apply(Action::ImportAt(bytes, url)).unwrap(); editor.apply(Action::UnloadSources).unwrap();
        assert!(!key.matches(&editor.scene.objects()[0]));
    }
}
