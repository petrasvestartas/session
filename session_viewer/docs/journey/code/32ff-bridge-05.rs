    *delivery.borrow_mut() = Some(file);
    let result = web_sys::window().ok_or_else(|| JsValue::from_str("No browser window"))
        .and_then(|window| window.dispatch_event(&event));
    delivery.borrow_mut().take();
    result.map(|_| ())
