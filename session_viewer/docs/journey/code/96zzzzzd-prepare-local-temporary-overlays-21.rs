        let guides=editor.guide_overlay_at(renderer.anchor());
        if let Err(error)=renderer.set_guides(&guides){panel.result(error);}