use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue};

pub fn choose(event: &web_sys::Event, request: Rc<Cell<u64>>) {
    let Some(input) = event.target().and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok()) else { return; };
    if input.id() != "open" { return; }
    let Some(file) = input.files().and_then(|files| files.get(0)) else { return; };
    input.set_value("");
    let id = request.get() + 1;
    request.set(id);
    if file.size() > crate::document::MAX_BYTES as f64 {
        super::browser::report("This checkpoint accepts files up to 4 MiB");
        return;
    }
    super::browser::report("Reading the selected file…");
    wasm_bindgen_futures::spawn_local(async move {
        let result = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await;
        if request.get() != id { return; }
        let result = result.and_then(deliver);
        if let Err(error) = result {
            super::browser::report(&format!("Cannot read file: {error:?}"));
        }
    });
}

fn deliver(buffer: JsValue) -> Result<(), JsValue> {
    let detail = web_sys::CustomEventInit::new();
    detail.set_detail(&js_sys::Uint8Array::new(&buffer));
    let event = web_sys::CustomEvent::new_with_event_init_dict("viewer-file", &detail)?;
    web_sys::window().ok_or("No browser window")?.dispatch_event(&event)?;
    Ok(())
}

pub fn bytes(event: &web_sys::Event) -> Option<Vec<u8>> {
    let event = event.dyn_ref::<web_sys::CustomEvent>()?;
    let array = event.detail().dyn_into::<js_sys::Uint8Array>().ok()?;
    if array.length() as usize > crate::document::MAX_BYTES { return None; }
    Some(array.to_vec())
}
