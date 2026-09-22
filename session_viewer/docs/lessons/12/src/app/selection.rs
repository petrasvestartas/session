/// What is selected inside one object.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub enum SelectionMode {
    #[default]
    Object, // whole objects only
    Edge {
        parent: u32, // object row
        edge: u32,   // edge index
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
