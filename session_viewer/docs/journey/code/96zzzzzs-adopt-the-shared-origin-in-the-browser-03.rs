        panel.update(None,&canvas)?;
        crate::browser_rebase::prepare(&mut renderer,&editor)?;
        canvas.set_attribute("data-origin-pending","false")?;
        inspect(&editor,&renderer)?;