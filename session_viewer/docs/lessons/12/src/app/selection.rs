//! The picked object lives in Scene; this says what inside that object is picked, if anything.

/// `#[default]` marks the variant that `Default::default()` returns.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub enum SelectionMode {
    #[default]
    Object,
    Edge {
        parent: u32, // object row
        edge: u32,
    },
}

impl SelectionMode {
    /// The object holding the sub-selection; None when the whole object is selected.
    pub fn parent(&self) -> Option<u32> {
        match self {
            Self::Object => None,
            Self::Edge { parent, .. } => Some(*parent),
        }
    }

    pub fn select_edge(&mut self, parent: u32, edge: u32) {
        *self = Self::Edge { parent, edge };
    }

    /// Back to object selection; returns the parent.
    pub fn escape(&mut self) -> Option<u32> {
        let parent = self.parent();
        *self = Self::Object;
        parent
    }
}
