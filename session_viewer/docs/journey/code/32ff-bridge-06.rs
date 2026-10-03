pub fn action(event: &web_sys::Event, delivery: &Delivery) -> Result<Option<crate::editor::Action>, JsValue> {
    let Some(event) = event.dyn_ref::<web_sys::CustomEvent>() else { return Ok(None); };
    let Ok(payload) = event.detail().dyn_into::<js_sys::Array>() else { return Ok(None); };
    if payload.length() != 2 { return Ok(None); }
    let Some(replace) = payload.get(0).as_bool() else { return Ok(None); };
    let Ok(array) = payload.get(1).dyn_into::<js_sys::Uint8Array>() else { return Ok(None); };
    if array.length() as usize > crate::document::MAX_BYTES { return Ok(None); }
    let Some(file) = delivery.borrow_mut().take() else { return Ok(None); };
    let location = Rc::new(crate::reload_url::ReloadUrl::from_file(&file)?);
    Ok(Some(if replace { crate::editor::Action::ReplaceAt(array.to_vec(), location) }
        else { crate::editor::Action::ImportAt(array.to_vec(), location) }))
}
