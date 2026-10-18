    wasm_bindgen_futures::spawn_local(async move {
        if request.borrow().pending() != Some(id) { return; }
        if file.size() > crate::archive::LIMIT as f64 {
            let accepted = request.borrow_mut().finish(id);
            if accepted { failure("Session exceeds the 512 MiB file limit"); }
            return;
        }
