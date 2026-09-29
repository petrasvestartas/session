    for name in ["pointerdown", "pointermove", "pointerup", "pointercancel", "lostpointercapture", "contextmenu"] {
        canvas.add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    controls.remove_attribute("disabled")?;
