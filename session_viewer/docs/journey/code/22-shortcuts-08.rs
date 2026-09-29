    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    for name in ["keydown", "blur"] {
        canvas.add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel", update.as_ref().unchecked_ref(), &options,
    )?;
