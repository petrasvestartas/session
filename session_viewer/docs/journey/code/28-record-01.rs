use crate::{document::Source, mesh::Mesh};
use std::rc::Rc;

pub struct PreparedMesh {
    pub geometry: Rc<session_rust::Mesh>,
    pub display: Mesh,
    pub source: Option<Source>,
}

impl PreparedMesh {
    pub fn new(geometry: session_rust::Mesh) -> Result<Self, &'static str> {
        Self::from_shared(Rc::new(geometry))
    }

    pub fn from_shared(geometry: Rc<session_rust::Mesh>) -> Result<Self, &'static str> {
        // The kernel creates a GUID lazily; establish identity before sharing.
        let _ = geometry.guid();
        let display = Mesh::from_kernel(&geometry)?;
        Ok(Self { geometry, display, source: None })
    }
}
