        crate::browser_rebase::prepare(&mut renderer,&mut editor).map(|frame|{if frame.moved {math_view.invalidate(editor.display_scene());}frame})?;
