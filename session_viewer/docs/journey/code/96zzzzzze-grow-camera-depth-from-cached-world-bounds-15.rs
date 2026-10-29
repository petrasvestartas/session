        renderer.refresh_text_bounds(editor.display_scene_mut());
        let origin_frame=match crate::browser_rebase::prepare(&mut renderer,&mut editor).map(|frame|{if frame.moved {math_view.invalidate(editor.display_scene());}frame}){Ok(frame)=>frame,Err(error)=>{report(&format!("Cannot prepare view: {error:?}"));return;}};
