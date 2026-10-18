        renderer.refresh_text_bounds(editor.display_scene_mut());
        let origin_frame=match crate::browser_rebase::prepare(&mut renderer,&mut editor){