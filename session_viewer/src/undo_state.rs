use std::collections::HashMap;
use session_rust::session::Geometry;
use session_rust::NurbsSurface;

/// Geometry state captured at drag-start so Transform undo can restore exact position
/// for types where commit_object_transform bakes the matrix into coordinates.
pub enum GeomSnapshot {
    Geom(Geometry),
    Nurbs(NurbsSurface),
}

/// One undoable/redoable operation recorded on the history stacks.
pub enum UndoAction {
    /// CLI add command (box/sphere/cyl/point/line/poly): stored in session.lookup.
    /// Undo = remove guid from lookup + GPU. Redo = re-add geom to lookup + GPU.
    AddLookup { guid: String, geom: Geometry },

    /// CLI add command producing a NurbsSurface (cone/torus).
    /// Undo = remove from nurbssurfaces + GPU. Redo = re-add to nurbssurfaces + GPU.
    AddNurbs { ns: NurbsSurface },

    /// CLI del command: batch remove of selected objects (lookup only).
    /// Undo = re-add all. Redo = remove all again.
    RemoveObjects { objects: Vec<(String, Geometry)> },

    /// Gumball drag committed via commit_object_transform.
    /// - BRep: snapshots empty → undo/redo via model matrix (before/after).
    /// - non-BRep/NURBS: commit bakes the matrix into CPU coords, so we snapshot
    ///   absolute geometry state on BOTH sides: `snapshots` = pre-drag (undo target),
    ///   `snapshots_after` = post-drag (redo target). Both directions are absolute
    ///   restores so repeated undo/redo never accumulates the delta.
    Transform {
        objects: Vec<(String, [[f32; 4]; 4], [[f32; 4]; 4])>,
        snapshots: HashMap<String, GeomSnapshot>,
        snapshots_after: HashMap<String, GeomSnapshot>,
    },
}

impl UndoAction {
    pub fn snap_geom(g: Geometry) -> GeomSnapshot { GeomSnapshot::Geom(g) }
    pub fn snap_nurbs(ns: NurbsSurface) -> GeomSnapshot { GeomSnapshot::Nurbs(ns) }
}

pub struct UndoState {
    pub undo_stack: Vec<UndoAction>,
    pub redo_stack: Vec<UndoAction>,
}

impl UndoState {
    pub fn new() -> Self {
        Self { undo_stack: Vec::new(), redo_stack: Vec::new() }
    }

    /// Push a new action onto the undo stack and clear the redo stack.
    pub fn push(&mut self, action: UndoAction) {
        self.undo_stack.push(action);
        self.redo_stack.clear();
        if self.undo_stack.len() > 64 {
            self.undo_stack.remove(0);
        }
    }
}
