use crate::diagnostic::{Context, Report};
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue};

thread_local! { static REPORT: RefCell<Option<Report>> = const { RefCell::new(None) }; }

fn context() -> Result<Context, JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document.get_element_by_id("canvas").ok_or("No canvas")?.dyn_into()?;
    let location = window.location(); let navigator = window.navigator();
    Ok(Context {
        page: format!("{}{}", location.origin()?, location.pathname()?),
        browser: navigator.user_agent()?, secure_context: window.is_secure_context(),
        webgpu: js_sys::Reflect::get(navigator.as_ref(), &"gpu".into())?.is_object(),
        viewport: [window.inner_width()?.as_f64().ok_or("No width")? as u32,
            window.inner_height()?.as_f64().ok_or("No height")? as u32],
        canvas: [canvas.width(), canvas.height()], device_pixel_ratio: window.device_pixel_ratio(),
    })
}

pub fn start() -> Result<(), JsValue> {
    let report = Report::new(uuid::Uuid::new_v4().to_string(), js_sys::Date::new_0().to_iso_string().into(), context()?);
    REPORT.with(|slot| slot.replace(Some(report)));
    Ok(())
}

pub fn observe(kind: &str, message: &str) -> Result<(), JsValue> {
    let context = context()?;
    let elapsed = web_sys::window().and_then(|window| window.performance()).ok_or("No clock")?.now();
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut(); let report = slot.as_mut().ok_or("No report")?;
        report.context = context;
        report.record(js_sys::Date::new_0().to_iso_string().into(), elapsed, kind, message).map_err(JsValue::from_str)
    })
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn diagnostic_snapshot() -> Result<String, JsValue> {
    let context = context()?;
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut(); let report = slot.as_mut().ok_or("No report")?;
        report.context = context;
        serde_json::to_string_pretty(report).map_err(|error| JsValue::from_str(&error.to_string()))
    })
}
