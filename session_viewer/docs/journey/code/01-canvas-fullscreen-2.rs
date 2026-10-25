use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document.get_element_by_id("canvas").ok_or("Missing canvas")?.dyn_into()?;
    let density = window.device_pixel_ratio();
    let density = if density.is_finite() && density > 0.0 { density } else { 1.0 };
    canvas.set_width((window.inner_width()?.as_f64().ok_or("No window width")? * density).round().max(1.0) as u32);
    canvas.set_height((window.inner_height()?.as_f64().ok_or("No window height")? * density).round().max(1.0) as u32);
    let status = document.get_element_by_id("status").ok_or("Missing status")?;
    status.set_text_content(Some("Rust is running. The canvas is ready."));
    Ok(())
}
