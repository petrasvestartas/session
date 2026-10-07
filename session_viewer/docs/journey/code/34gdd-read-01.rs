        let started = crate::browser_phase::now();
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        let accepted = request.borrow_mut().finish(id);
        if !accepted { return; }
        let bytes = result.as_ref().map_or(0, |buffer| js_sys::Uint8Array::new(buffer).length() as u64);
        crate::browser_phase::finish(if result.is_ok() { "file read" } else { "file read failed" },
            started, bytes, &file.name());
