        let rect=pointer_canvas.get_bounding_client_rect();
        let number=editor.number_prompt([rect.width(),rect.height()]);
        if editor.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some()) && number.is_none() {
            if editor.apply(Action::GizmoCancel).is_ok() {highlight_changed=true;math_view.upload(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());}
        }
        panel.number(number);
        if let Err(error) = renderer.set_preview(editor.draft.as_ref()) { panel.result(error); }
