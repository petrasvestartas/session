use crate::{listeners::Listeners, startup::Startup};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

pub struct Guard {
    authority: Startup,
    _listeners: Listeners,
}

impl Guard {
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No browser window")?;
        let authority = Startup::default(); let closing = authority.clone();
        let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let cached = event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted());
            if !cached { closing.revoke(); let _ = crate::browser_report::close_report(); }
        });
        let mut listeners = Listeners::new(callback);
        listeners.listen(window.as_ref(), "pagehide")?;
        Ok(Self { authority, _listeners: listeners })
    }

    pub fn permits(&self) -> bool { self.authority.permits() }
}
