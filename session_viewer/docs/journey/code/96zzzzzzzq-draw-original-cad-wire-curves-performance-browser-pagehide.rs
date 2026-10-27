        if event.type_() == "pagehide" {
            frames.refresh(false);
            if !event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) {
                frames.close();
                let owner = active_fault.clone();
                wasm_bindgen_futures::spawn_local(async move { crate::browser_runtime::stop_if(&owner); });
                return;
            }
            gesture.cancel();touch.cancel();let _=editor.apply(Action::CanvasCancel); let _=editor.apply(Action::GizmoCancel);
            crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());renderer.set_gizmo(&editor.camera,None,-1.);
            return;
        }
