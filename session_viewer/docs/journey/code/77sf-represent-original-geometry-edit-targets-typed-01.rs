use crate::source_controls::{ControlId,Controls,SelectionMode};
use session_rust::{Geometry, Mesh, NurbsSurface, Point, Xform};
use std::collections::HashSet;
use std::rc::Rc;

/// The part of a geometry an edit applies to.
#[derive(Clone, Copy, Debug)]
pub enum Target {
    Control(ControlId), // one control point or vertex
    Edge(u32),          // one edge by index
    Face(usize),        // one face by key
}

impl Target {
    /// The target the current selection names, if any.
    pub fn selected(mode: &SelectionMode) -> Option<Self> {
        match *mode {
            SelectionMode::Controls {
                selected: Some(id), ..
            } => Some(Self::Control(id)),
            SelectionMode::Edge { edge, .. } => Some(Self::Edge(edge)),
            SelectionMode::Face { face, .. } => Some(Self::Face(face)),
            _ => None,
        }
    }
}

