use crate::{commands::{self, Command, Host}, editor::{Action, Editor}};

#[test]
fn longest_names_and_compact_spelling_keep_host_intent() {
    for text in ["Open Replace", "openreplace", " OPEN  replace "] {
        assert!(matches!(commands::parse(text).unwrap(), Command::Host(Host::Replace)));
        assert_eq!(commands::canonical(text), "Open Replace");
    }
    assert!(matches!(commands::parse("Report Previous").unwrap(), Command::Host(Host::PreviousReport)));
    assert!(matches!(commands::parse("SelectAll").unwrap(), Command::Editor(Action::SelectAll)));
    assert!(matches!(commands::parse("View Normals Off").unwrap(), Command::Editor(Action::NormalView(false))));
    assert_eq!(commands::canonical("mOvE 1E-12, 2, 3"), "Move 1E-12, 2, 3");
}

#[test]
fn invalid_commands_leave_original_rows_and_redo_untouched() {
    let mut editor = Editor::default(); editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Translate([1., 0., 0.])).unwrap(); editor.apply(Action::Undo).unwrap();
    let source = editor.scene.objects()[0].mesh.clone(); let placement = editor.scene.objects()[0].model.m;
    for text in ["", "Unknown", "Delete all", "Open Replace extra", "Fit Selected extra", "Move", "Move 1 2", "Move 1,,2", "Move NaN,0,0", "Move 1,2,inf", "Move 1,2,3,4", "Move 1 2,3,4"] {
        assert!(commands::parse(text).is_err(), "{text}");
        assert_eq!(editor.scene.objects()[0].model.m, placement);
        assert!(std::rc::Rc::ptr_eq(&source, &editor.scene.objects()[0].mesh));
    }
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m[12], placement[12]+1.);
}

#[test]
fn typed_offsets_execute_once_and_one_undo_restores_them() {
    let mut editor = Editor::default(); editor.apply(Action::SelectNext).unwrap();
    let before = editor.scene.objects()[0].model.m;
    for text in ["Move 1 -2 3e-2", "move 1, -2, 0.03", "MOVE 1 -2 .03"] {
        let Command::Editor(action) = commands::parse(text).unwrap() else { panic!("Move must reach the editor"); };
        editor.apply(action).unwrap(); assert_eq!(editor.scene.objects()[0].model.m[12], before[12]+1.);
        assert_eq!(editor.scene.objects()[0].model.m[13], before[13]-2.);
        editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m, before);
    }
}
