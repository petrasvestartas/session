use crate::{editor::{Action, Change, Editor}, tool::Shape};

#[test]
fn preview_and_cancellation_preserve_original_geometry_and_redo() {
    let mut editor = Editor::default(); editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Translate([1., 0., 0.])).unwrap(); editor.apply(Action::Undo).unwrap();
    let source = editor.scene.objects()[0].mesh.clone(); let before = editor.scene.objects()[0].model.m;
    assert_eq!(editor.apply(Action::BeginTool(Shape::Line)).unwrap(), Change::View);
    assert_eq!(editor.apply(Action::ToolPoint([0., 0., 0.])).unwrap(), Change::View);
    for x in 0..100 { editor.apply(Action::ToolHover(Some([x as f64/100., 0., 0.]))).unwrap(); }
    assert!(editor.scene.lines().is_empty()); assert_eq!(editor.draft.as_ref().unwrap().points.len(), 1);
    assert!(editor.apply(Action::ToolPoint([0., 0., 0.])).is_err());
    assert!(editor.apply(Action::ToolPoint([f64::NAN, 0., 0.])).is_err());
    assert_eq!(editor.draft.as_ref().unwrap().points.len(), 1);
    editor.apply(editor.tool_line("Escape").unwrap().unwrap()).unwrap(); assert!(editor.draft.is_none());
    assert_eq!(editor.scene.objects()[0].model.m, before); assert!(std::rc::Rc::ptr_eq(&source, &editor.scene.objects()[0].mesh));
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m[12], before[12]+1.);
}

#[test]
fn completing_a_line_commits_once_and_preserves_double_coordinates() {
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    editor.apply(Action::BeginTool(Shape::Line)).unwrap();
    let a = [0.123456789012345, -0.25, 0.]; let b = [1.23456789012345, 0.5, 0.];
    editor.apply(Action::ToolPoint(a)).unwrap();
    assert_eq!(editor.apply(Action::ToolPoint(b)).unwrap(), Change::Scene);
    assert!(editor.draft.is_none()); assert_eq!(editor.scene.lines().len(), 1);
    let line = &editor.scene.lines()[0].prepared.source;
    for i in 0..3 { assert_eq!(line.start()[i], a[i]); assert_eq!(line.end()[i], b[i]); }
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.lines().is_empty());
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.lines().len(), 1);
    editor.apply(Action::BeginTool(Shape::Point)).unwrap();
    editor.apply(Action::ToolPoint([3., 2., 1.])).unwrap(); assert_eq!(editor.scene.points().len(), 1);
    editor.apply(Action::Undo).unwrap(); assert!(editor.scene.points().is_empty()); assert_eq!(editor.scene.lines().len(), 1);
}

#[test]
fn absolute_points_validate_before_draft_changes_and_empty_enter_cancels() {
    let mut editor = Editor::default(); editor.apply(Action::BeginTool(Shape::Line)).unwrap();
    assert!(editor.tool_line("Fit").is_none()); assert!(editor.tool_line("NaN,2,3").unwrap().is_err());
    editor.apply(editor.tool_line("1,2,3").unwrap().unwrap()).unwrap();
    editor.apply(editor.tool_line("").unwrap().unwrap()).unwrap(); assert!(editor.draft.is_none());
    editor.apply(Action::BeginTool(Shape::Line)).unwrap(); editor.apply(Action::Undo).unwrap(); assert!(editor.draft.is_none());
    editor.apply(Action::BeginTool(Shape::Line)).unwrap(); editor.apply(Action::Close).unwrap(); assert!(editor.draft.is_none());
}

