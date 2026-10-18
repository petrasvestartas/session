    let Some(replace) = payload.get(0).as_bool() else { return Ok(None); };
    let Ok(array) = payload.get(1).dyn_into::<js_sys::Uint8Array>() else { return Ok(None); };
    if array.length() as usize > crate::archive::LIMIT { return Ok(None); }
    let Some(file) = delivery.borrow_mut().take() else { return Ok(None); };
    let location = Rc::new(crate::reload_url::ReloadUrl::from_file(&file)?);
    let bytes=array.to_vec();let replace=replace||bytes.starts_with(crate::archive::MAGIC);
    let ready=crate::publication::prepare(scene,&bytes,replace,Some(location)).map_err(|error|JsValue::from_str(&error))?;
    Ok(Some(crate::editor::Action::Adopt {scene:Box::new(ready),replace}))
}
