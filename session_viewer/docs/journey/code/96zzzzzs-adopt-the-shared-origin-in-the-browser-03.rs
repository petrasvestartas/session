        panel.update(None,&canvas)?;
        crate::browser_rebase::prepare(&mut renderer,&editor).map(|frame|{if frame.moved {math_view.invalidate(editor.display_scene());}frame})?;
        canvas.set_attribute("data-origin-pending","false")?;
