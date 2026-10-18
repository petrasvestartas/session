use crate::{
    pick_sources::{Part, Target},
    scene::{ObjectId, Scene},
    sheet_entity::{EntityMeta, SheetTable},
    sheet_source::Prepared,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};
struct State {
    gate: crate::sheet_lookup::Gate,
    selected: Option<(ObjectId, u32, usize)>,
    abort: Option<web_sys::AbortController>,
    tables: HashMap<usize, (Weak<Prepared>, SheetTable)>,
    reply: Option<Result<EntityMeta, String>>,
    closed: bool,
}
pub struct Control(Rc<RefCell<State>>);
impl Default for Control {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(State {
            gate: Default::default(),
            selected: None,
            abort: None,
            tables: HashMap::new(),
            reply: None,
            closed: false,
        })))
    }
}
impl Drop for Control {
    fn drop(&mut self) {
        let mut owner = self.0.borrow_mut();
        owner.closed = true;
        owner.gate.cancel();
        if let Some(abort) = owner.abort.take() {
            abort.abort();
        }
    }
}
impl Control {
    pub fn take(&self) -> Option<Result<EntityMeta, String>> {
        self.0.borrow_mut().reply.take()
    }
}

fn complete(
    weak: Weak<RefCell<State>>,
    fault: crate::gpu_fault::Fault,
    ticket: u64,
    source: Weak<Prepared>,
    source_key: usize,
    result: Result<(EntityMeta, SheetTable), String>,
) {
    let Some(state) = weak.upgrade() else {
        return;
    };
    let Some(document) = crate::browser_runtime::document(&fault) else {
        return;
    };
    let document = document.borrow();
    let mut owner = state.borrow_mut();
    if owner.closed
        || fault.message().is_some()
        || !owner
            .gate
            .finish(ticket, &document.scene, document.selection())
    {
        return;
    }
    owner.abort = None;
    owner.reply = Some(result.map(|(entity, table)| {
        owner.tables.insert(source_key, (source, table));
        entity
    }));
    drop(owner);
    drop(document);
    if let Some(window) = web_sys::window() {
        if let Ok(event) = web_sys::Event::new("viewer-sheet-entity") {
            let _ = window.dispatch_event(&event);
        }
    }
}
