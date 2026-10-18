    assert_eq!(key("f", false, false, false, false, true, true), None);
    assert_eq!(key("f", true, false, false, false, false, true), None);
    assert_eq!(key("h", false, false, false, false, false, true), Some(Shortcut::Hide));
    assert_eq!(key("s", false, false, false, false, false, true), Some(Shortcut::Show));
}

#[test]
fn canvas_delete_hide_show_keep_focused_text_and_modifiers_owned_by_the_field() {
    for (text,shortcut) in [("Delete",Shortcut::Delete),("H",Shortcut::Hide),("S",Shortcut::Show)] {
        assert_eq!(key(text,false,false,false,false,false,true),Some(shortcut));
        for (command,alt,composing,focused) in [(true,false,false,false),(false,true,false,false),(false,false,true,false),(false,false,false,true)] {assert_eq!(key(text,command,false,alt,composing,focused,true),None);}
    }
}
