use crate::editor::{Action, Editor};

#[test]
fn accepted_gpu_picks_resolve_current_rows_without_consuming_document_history() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(Action::AddPoint).unwrap();
    let id = editor.scene.points()[0].id; editor.apply(Action::Picked(Some(id))).unwrap(); assert_eq!(editor.selected, Some(id));
    let intent = crate::edit_intent::Intent::capture(&Action::Delete, editor.selected).unwrap().unwrap();
    assert!(intent.keys(&editor).unwrap().is_empty(), "Original ink needs no mesh reload");
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.points().is_empty()); assert!(editor.selected.is_none());
    editor.apply(Action::Picked(Some(id))).unwrap(); assert!(editor.selected.is_none());
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.points()[0].id, id);
    editor.apply(Action::Picked(Some(id))).unwrap(); editor.apply(Action::Delete).unwrap(); assert!(editor.scene.points().is_empty());
    editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.points()[0].id, id);
    editor.apply(Action::Picked(None)).unwrap(); assert!(editor.selected.is_none());
}

#[test]
fn original_lines_paths_arrows_and_control_parents_share_selection_and_delete_history() {
    for action in [Action::AddLine, Action::AddPolyline, Action::AddArrow, Action::AddCurve] {
        let mut editor = Editor::default(); editor.apply(Action::Close).unwrap(); editor.apply(action).unwrap();
        let id = editor.scene.lines().first().map(|r| r.id).or_else(|| editor.scene.paths().first().map(|r| r.id)).unwrap();
        editor.apply(Action::Picked(Some(id))).unwrap(); assert_eq!(editor.selected, Some(id));
        let intent = crate::edit_intent::Intent::capture(&Action::Delete, editor.selected).unwrap().unwrap();
        assert!(intent.keys(&editor).unwrap().is_empty());
        editor.apply(Action::Delete).unwrap(); assert!(!editor.scene.contains(id));
        editor.apply(Action::Undo).unwrap(); assert!(editor.scene.contains(id));
        editor.apply(Action::Redo).unwrap(); assert!(!editor.scene.contains(id));
        editor.apply(Action::Close).unwrap(); editor.apply(Action::Picked(Some(id))).unwrap(); assert!(editor.selected.is_none());
    }
}
