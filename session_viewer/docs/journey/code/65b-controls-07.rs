    }
}

#[test]
fn f10_controls_only_when_the_canvas_owns_the_key() {
    use crate::shortcuts::{key,Shortcut};
    assert_eq!(key("F10",false,false,false,false,false,true),Some(Shortcut::Controls));
    for (command,alt,composing,focused) in [(true,false,false,false),(false,true,false,false),(false,false,true,false),(false,false,false,true)] {assert_eq!(key("F10",command,false,alt,composing,focused,true),None);}
    assert_eq!(Shortcut::Controls.command(),"Controls");
}
