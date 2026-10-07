use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub struct Listeners {
    callback: Closure<dyn FnMut(web_sys::Event)>,
    bindings: Vec<(web_sys::EventTarget, &'static str)>,
}

impl Listeners {
    pub fn new(callback: Closure<dyn FnMut(web_sys::Event)>) -> Self {
        Self { callback, bindings: Vec::new() }
    }

    pub fn listen(&mut self, target: &web_sys::EventTarget, name: &'static str) -> Result<(), JsValue> {
        let options = web_sys::AddEventListenerOptions::new(); options.set_passive(false);
        target.add_event_listener_with_callback_and_add_event_listener_options(
            name, self.callback.as_ref().unchecked_ref(), &options)?;
        self.bindings.push((target.clone(), name));
        Ok(())
    }
}

impl Drop for Listeners {
    fn drop(&mut self) {
        for (target, name) in &self.bindings {
            let _ = target.remove_event_listener_with_callback(name, self.callback.as_ref().unchecked_ref());
        }
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn listener_probe() -> Result<Vec<u32>, JsValue> {
    use std::{cell::Cell, rc::Rc};
    let first = web_sys::EventTarget::new()?; let second = web_sys::EventTarget::new()?;
    let hits = Rc::new(Cell::new(0)); let counted = Rc::clone(&hits);
    let captured = Rc::new(()); let observer = Rc::downgrade(&captured);
    let callback = Closure::new(move |_: web_sys::Event| {
        assert_eq!(Rc::strong_count(&captured), 1);
        counted.set(counted.get() + 1);
    });
    let mut listeners = Listeners::new(callback);
    listeners.listen(&first, "proof")?; listeners.listen(&second, "proof")?;
    let event = web_sys::Event::new("proof")?;
    first.dispatch_event(&event)?; second.dispatch_event(&event)?; let before = hits.get();
    drop(listeners);
    first.dispatch_event(&event)?; second.dispatch_event(&event)?;
    Ok(vec![before, hits.get(), observer.strong_count() as u32])
}
