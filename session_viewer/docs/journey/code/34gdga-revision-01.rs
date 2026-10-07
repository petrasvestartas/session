pub fn versions(scene: &crate::scene::Scene) -> Vec<String> {
    let mut versions: Vec<_> = scene.objects().iter().filter_map(|object|
        object.origin().map(|origin| origin.version.hex())).collect();
    versions.sort(); versions.dedup(); versions
}

#[cfg(test)]
mod tests {
    use crate::{editor::{Action, Editor}, origin::FileVersion};
    #[test]
    fn revisions_describe_distinct_active_sources_without_owning_them() {
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        editor.apply(Action::Import(bytes.clone())).unwrap();
        let weak = std::rc::Rc::downgrade(&editor.scene.objects().last().unwrap().source().unwrap().origin);
        let versions = super::versions(&editor.scene);
        assert_eq!(versions, vec![FileVersion::of(&bytes).hex()]);
        let replacement = crate::specimen::precise_bytes();
        editor.apply(Action::Replace(replacement.clone())).unwrap();
        assert_eq!(super::versions(&editor.scene), vec![FileVersion::of(&replacement).hex()]);
        editor.apply(Action::Close).unwrap(); assert!(weak.upgrade().is_none());
        assert!(super::versions(&editor.scene).is_empty()); assert_eq!(versions.len(), 1);
    }
}
