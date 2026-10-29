            renderer.set_gizmo(&editor.camera,editor.gizmo_origin(),editor.gizmo_active());
            let pending=if editor.draft.is_some() || editor.gizmo_edit.is_some() {math_view.invalidate(editor.display_scene());false} else {math_view.refresh(&mut renderer,editor.display_scene(),&editor.camera,[config.width,config.height],editor.whole_selection())};
