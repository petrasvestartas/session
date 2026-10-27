use crate::{browser_reload::Shared, listeners::Listeners, read_gate::ReadGate};
use std::{cell::RefCell, rc::Rc};

struct Runtime {
    _listeners: Listeners,
    frame: crate::browser_frame::Frame,
    _publication: crate::browser_publication::Publication,
    request: Rc<RefCell<ReadGate>>,
    reload: Shared,
    fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared,
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.frame.close();
        crate::stream::reset_range_gate();
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
    }
}

thread_local! { static ACTIVE: RefCell<Option<Runtime>> = const { RefCell::new(None) }; }

pub fn install(listeners: Listeners, request: Rc<RefCell<ReadGate>>, reload: Shared, fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared, publication: crate::browser_publication::Publication, frame: crate::browser_frame::Frame,
) {
    ACTIVE.with(|slot| slot.replace(Some(Runtime { frame, _publication: publication, _listeners: listeners, request, reload, fault, document })));
}

pub fn document(fault: &crate::gpu_fault::Fault) -> Option<crate::drawing_document::Shared> {
    ACTIVE.with(|slot| slot.borrow().as_ref().filter(|runtime| runtime.fault.same_device(fault))
        .map(|runtime| Rc::clone(&runtime.document)))
}

pub fn owns(fault: &crate::gpu_fault::Fault) -> bool {
    ACTIVE.with(|slot| slot.borrow().as_ref().is_some_and(|runtime| runtime.fault.same_device(fault)))
}

pub fn complete_frame(fault: &crate::gpu_fault::Fault, ticket: u64) {
    let frame = ACTIVE.with(|slot| slot.borrow().as_ref()
        .filter(|runtime| runtime.fault.same_device(fault)).map(|runtime| runtime.frame.clone()));
    if let Some(frame) = frame { frame.complete(ticket); }
}

pub fn stop_if(fault: &crate::gpu_fault::Fault) -> bool {
    let owner = ACTIVE.with(|slot| {
        let mut current = slot.borrow_mut();
        if current.as_ref().is_some_and(|runtime| runtime.fault.same_device(fault)) { current.take() }
        else { None }
    });
    owner.is_some()
}

pub fn stop() -> bool {
    ACTIVE.with(|slot| slot.borrow_mut().take()).is_some()
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn runtime_running() -> bool { ACTIVE.with(|slot| slot.borrow().is_some()) }

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn reload_scene(url:Option<String>)->Result<(),wasm_bindgen::JsValue>{
    let control=ACTIVE.with(|slot| {
        let mut active=slot.borrow_mut();let runtime=active.as_mut().ok_or("Viewer runtime is not running")?;
        let publication=match url {Some(path)=>crate::browser_publication::Publication::at(runtime.document.clone(),runtime.fault.clone(),&path)?,None=>crate::browser_publication::Publication::new(runtime.document.clone(),runtime.fault.clone())?};
        if !publication.has_route(){return Ok(None);}
        runtime.request.borrow_mut().cancel();runtime.reload.borrow_mut().cancel();
        runtime._publication.replace(publication);let control=runtime._publication.control();
        Ok::<_,wasm_bindgen::JsValue>(Some(control))
    })?;
    if let Some(control)=control{control.begin();}Ok(())
}

pub fn refresh_frame(allowed: bool) {
    let frame = ACTIVE.with(|slot| slot.borrow().as_ref().map(|runtime| runtime.frame.clone()));
    if let Some(frame) = frame { frame.refresh(allowed); }
}
