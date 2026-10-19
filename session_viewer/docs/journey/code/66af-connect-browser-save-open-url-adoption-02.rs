
pub fn install(listeners: Listeners, request: Rc<RefCell<ReadGate>>, reload: Shared, fault: crate::gpu_fault::Fault,
    document: crate::drawing_document::Shared, publication: crate::browser_publication::Publication,
) {
    ACTIVE.with(|slot| slot.replace(Some(Runtime { _publication: publication, _listeners: listeners, request, reload, fault, document })));
}

