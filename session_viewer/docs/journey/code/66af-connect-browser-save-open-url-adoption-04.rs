}

pub fn action(event: &web_sys::Event, delivery: &Delivery, scene: &crate::scene::Scene) -> Result<Option<crate::editor::Action>, JsValue> {
    let Some(event) = event.dyn_ref::<web_sys::CustomEvent>() else { return Ok(None); };
    let Ok(payload) = event.detail().dyn_into::<js_sys::Array>() else { return Ok(None); };
