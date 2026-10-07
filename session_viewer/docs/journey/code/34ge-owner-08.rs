pub fn install(listeners: Listeners, request: Rc<RefCell<ReadGate>>, reload: Shared, fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared,
) {
    ACTIVE.with(|slot| slot.replace(Some(Runtime { _listeners: listeners, request, reload, fault, document })));
}

pub fn document(fault: &crate::gpu_fault::Fault) -> Option<crate::drawing_document::Shared> {
    ACTIVE.with(|slot| slot.borrow().as_ref().filter(|runtime| runtime.fault.same_device(fault))
        .map(|runtime| Rc::clone(&runtime.document)))
}