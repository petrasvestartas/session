use session_rust::element::ElementGeometry;
use session_rust::{Geometry, NurbsCurve, NurbsSurface, Point};

/// One control point of a geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub enum ControlId {
    Vertex(usize),                                  // mesh, polyline or BRep vertex
    Curve { curve: usize, point: usize },           // curve control point
    Surface { surface: usize, u: usize, v: usize }, // surface control point
    Point(u32),                                     // cloud point
}

/// What is selected inside one object.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub enum SelectionMode {
    #[default]
    Object, // whole objects only
    Edge {
        parent: u32, // object row
        edge: u32,   // edge index
    },
    Face {
        parent: u32, // object row
        face: usize, // face key
    },
    Controls {
        parent: u32,                 // object row
        selected: Option<ControlId>, // the picked control
        cloud: bool,                 // parent is a point cloud
    },
}

impl SelectionMode {
    /// The object row of a sub-selection.
    pub fn parent(&self) -> Option<u32> {
        match self {
            Self::Object => None,
            Self::Edge { parent, .. }
            | Self::Face { parent, .. }
            | Self::Controls { parent, .. } => Some(*parent),
        }
    }

    /// Select one edge.
    pub fn select_edge(&mut self, parent: u32, edge: u32) {
        *self = Self::Edge { parent, edge };
    }

    /// Show controls of `parent`; false when nothing changed.
    pub fn enable_controls(&mut self, parent: Option<u32>, cloud: bool) -> bool {
        let Some(parent) = parent else { return false };

        if matches!(self, Self::Controls { parent: active, .. } if *active == parent) {
            return false;
        }

        *self = Self::Controls {
            parent,
            selected: None,
            cloud,
        };
        true
    }

    /// Back to object selection; returns the parent.
    pub fn escape(&mut self) -> Option<u32> {
        let parent = self.parent();
        *self = Self::Object;
        parent
    }
}

/// What a click picks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionTool {
    #[default]
    Object, // whole objects
    Edge, // edges
    Face, // faces
}
