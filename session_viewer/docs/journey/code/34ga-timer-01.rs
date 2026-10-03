use wasm_bindgen::{closure::Closure, JsCast, JsValue};

pub struct Timer {
    window: web_sys::Window,
    id: i32,
    _callback: Closure<dyn FnMut()>,
}

impl Timer {
    pub fn every(milliseconds: i32, tick: impl FnMut() + 'static) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let callback = Closure::wrap(Box::new(tick) as Box<dyn FnMut()>);
        let id = window.set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), milliseconds)?;
        Ok(Self { window, id, _callback: callback })
    }
}

impl Drop for Timer {
    fn drop(&mut self) { self.window.clear_interval_with_handle(self.id); }
}
