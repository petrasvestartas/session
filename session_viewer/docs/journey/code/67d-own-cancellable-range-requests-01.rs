use std::{cell::{Cell,RefCell},collections::BTreeMap,rc::Rc};
use wasm_bindgen::{JsCast,JsValue,closure::Closure};
use wasm_bindgen_futures::JsFuture;
pub use crate::range_rules::PROBE_BYTES;

thread_local! {
    static REQUESTS:RefCell<BTreeMap<u64,web_sys::AbortController>>=const {RefCell::new(BTreeMap::new())};
    static NEXT:Cell<u64>=const {Cell::new(0)};
}

pub fn cancel() {
    for controller in REQUESTS.with(|requests|std::mem::take(&mut *requests.borrow_mut())).into_values() {controller.abort();}
}

struct Read {
    key:u64,
    controller:web_sys::AbortController,
    timer:i32,
    stalled:Rc<Cell<bool>>,
    callback:Closure<dyn FnMut()>,
}
