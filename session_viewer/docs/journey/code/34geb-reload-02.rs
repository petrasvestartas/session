use std::cell::RefCell;
use wasm_bindgen::JsValue;

thread_local! { static POLICY: RefCell<crate::recovery::Recovery> = RefCell::new(Default::default()); }

pub fn density(value: f64) -> f64 {
    POLICY.with(|policy| if policy.borrow().reduced() { value.min(1.0) } else { value })
}

pub fn adopt() -> Result<Option<String>, JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let url = web_sys::Url::new(&window.location().href()?)?;
    let params = url.search_params();
    let Some(reason) = params.get("recovered") else { return Ok(None); };
    POLICY.with(|policy| policy.replace(crate::recovery::Recovery::adopted()));
    params.delete("recovered");
    let _ = window.history()?.replace_state_with_url(&JsValue::NULL, "", Some(&url.href()));
    Ok(Some(format!("{}; drawing at device scale 1 until the next reload",
        reason.chars().take(200).collect::<String>())))
}

pub fn request(message: &str) -> bool {
    if !POLICY.with(|policy| policy.borrow_mut().request(message.contains("device lost"))) { return false; }
    let navigate = || -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let url = web_sys::Url::new(&window.location().href()?)?;
        url.search_params().set("recovered", &message.chars().take(200).collect::<String>());
        window.location().replace(&url.href())
    };
    navigate().is_ok()
}
