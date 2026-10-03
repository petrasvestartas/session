use std::rc::Rc;

#[derive(Clone)]
pub enum EditSource {
    Loaded { geometry: Rc<session_rust::Mesh>, source: Option<crate::document::Source> },
    Released { origin: Rc<crate::origin::Origin>, epoch: u64 },
}
