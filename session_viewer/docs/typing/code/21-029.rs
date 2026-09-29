    named(NamedKey::Delete, |s| s.delete_selected()), // register:delete
    ctrl(&["z", "Z"], Some(true), |s| s.redo()), // register:redo-shift
    ctrl(&["z", "Z"], Some(false), |s| s.undo()), // register:undo
    ctrl(&["y", "Y"], None, |s| s.redo()), // register:redo
