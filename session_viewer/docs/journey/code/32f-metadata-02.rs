#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RowMetadata {
    pub source_guid: String,
    pub name: String,
    pub is_visible: bool,
    pub is_locked: bool,
}

impl RowMetadata {
    pub fn from_mesh(mesh: &session_rust::Mesh) -> Self {
        Self { source_guid: mesh.guid().to_owned(), name: mesh.name.clone(),
            is_visible: mesh.is_visible, is_locked: mesh.is_locked }
    }
}

#[cfg(test)]
mod tests {
    use crate::editor::{Action, Editor};
    use std::rc::Rc;

    #[test]
    fn metadata_does_not_keep_a_closed_kernel_source_alive() {
        let mut editor = Editor::default();
        let mut mesh = session_rust::Mesh::create_box(1.0, 1.0, 1.0);
        mesh.name = "Released beam".into(); mesh.is_visible = false; mesh.is_locked = true;
        let id = editor.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();
        let row = editor.scene.objects().iter().find(|row| row.id == id).unwrap();
        let weak = Rc::downgrade(&row.geometry);
        let metadata = Rc::clone(&row.metadata);
        editor.apply(Action::Close).unwrap();
        assert!(weak.upgrade().is_none());
        assert_eq!(metadata.name, "Released beam");
        assert!(!metadata.is_visible && metadata.is_locked);
        assert!(!metadata.source_guid.is_empty());
    }

    #[test]
    fn duplicate_sources_keep_metadata_through_move_and_undo() {
        let mut editor = Editor::default();
        let bytes = crate::specimen::bytes();
        editor.apply(Action::Replace(bytes.clone())).unwrap();
        editor.apply(Action::Import(bytes)).unwrap();
        let original = &editor.scene.objects()[0]; let copy = &editor.scene.objects()[3];
        assert_eq!(original.metadata, copy.metadata); assert_ne!(original.guid, copy.guid);
        let metadata = Rc::clone(&original.metadata);
        editor.apply(Action::SelectNext).unwrap();
        editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
        assert!(Rc::ptr_eq(&metadata, &editor.scene.objects()[0].metadata));
        editor.apply(Action::Undo).unwrap();
        assert!(Rc::ptr_eq(&metadata, &editor.scene.objects()[0].metadata));
    }
}
