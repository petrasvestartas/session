    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press_with(id, pointer.button(), position,pointer.ctrl_key()||pointer.meta_key()) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
