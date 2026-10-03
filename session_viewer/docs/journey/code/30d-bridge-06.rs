    let payload = js_sys::Array::new();
    payload.push(&JsValue::from_bool(mode == Mode::Replace));
    payload.push(&js_sys::Uint8Array::new(&buffer));
    detail.set_detail(&payload);
