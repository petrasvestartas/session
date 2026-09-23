//! What is selected: a whole object, or one edge inside it, which the outline pass and the commands both read.

/// What is selected inside one object.
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
    /// The object row of a sub-selection.
    pub fn parent(&self) -> Option<u32> {
        match self {
            Self::Object => None,
            Self::Edge { parent, .. } => Some(*parent),
        }
    }

    /// Select one edge.
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
