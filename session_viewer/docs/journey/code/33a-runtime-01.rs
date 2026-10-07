use crate::{browser_reload::Shared, listeners::Listeners, read_gate::ReadGate};
use std::{cell::RefCell, rc::Rc};

struct Runtime {
    _listeners: Listeners,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
    }
}

thread_local! { static ACTIVE: RefCell<Option<Runtime>> = const { RefCell::new(None) }; }

pub fn install(listeners: Listeners, request: Rc<RefCell<ReadGate>>, reload: Shared) {
    ACTIVE.with(|slot| slot.replace(Some(Runtime { _listeners: listeners, request, reload })));
}

pub fn stop() -> bool {
    ACTIVE.with(|slot| slot.borrow_mut().take()).is_some()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn runtime_running() -> bool { ACTIVE.with(|slot| slot.borrow().is_some()) }
