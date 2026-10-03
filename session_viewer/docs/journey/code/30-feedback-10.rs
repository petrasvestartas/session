    wasm_bindgen_futures::spawn_local(async move {
        if request.get() != id { return; }
        if file.size() > crate::document::MAX_BYTES as f64 {
            failure("This checkpoint accepts files up to 4 MiB");
            return;
        }
