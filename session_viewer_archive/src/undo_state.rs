use std::collections::HashMap;
use session_rust::session::Geometry;
use session_rust::{NurbsCurve, NurbsSurface};

/// Geometry state captured at drag-start so Transform undo can restore exact position
/// for types where commit_object_transform bakes the matrix into coordinates.
pub enum GeomSnapshot {
    Geom(Geometry),
    Nurbs(NurbsSurface),
}

/// Full geometry clone captured before/after a sub-object (control-point) edit.
/// Covers all three stores: lookup geometry, the nurbssurfaces list, and the
/// nurbscurves list. Undo/redo restore the snapshot and rebuild the GPU mirror.
#[derive(Clone)]
pub enum EditSnapshot {
    Geom(Geometry),
    Nurbs(NurbsSurface),
    Curve(NurbsCurve),
}

/// One undoable/redoable operation recorded on the history stacks.
pub enum UndoAction {
    /// CLI add command (box/sphere/cyl/point/line/poly): stored in session.lookup.
    /// Undo = remove guid from lookup + GPU. Redo = re-add geom to lookup + GPU.
    AddLookup { guid: String, geom: Geometry },

    /// CLI add command producing a NurbsSurface (cone/torus).
    /// Undo = remove from nurbssurfaces + GPU. Redo = re-add to nurbssurfaces + GPU.
    AddNurbs { ns: NurbsSurface },

    /// Interactive `curve` tool producing a NurbsCurve (lives in objects.nurbscurves).
    /// Undo = remove from nurbscurves + GPU. Redo = re-add.
    AddNurbsCurve { nc: NurbsCurve },

    /// CLI del command: batch remove of selected objects. `objects` are lookup
    /// geometry; `nurbs` are NurbsSurfaces (cone/torus) and `curves` are NurbsCurves,
    /// both of which live in session.objects, not lookup.
    /// Undo = re-add all. Redo = remove all again.
    RemoveObjects { objects: Vec<(String, Geometry)>, nurbs: Vec<NurbsSurface>, curves: Vec<NurbsCurve> },

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

    /// Sub-object (control-point / vertex) edit committed in edit mode.
    /// Absolute snapshots on both sides, like Transform: `before` is the undo
    /// target, `after` the redo target. Apply re-imports the snapshot into its
    /// store and rebuilds the GPU mirror.
    EditGeom {
        guid: String,
        before: EditSnapshot,
        after: EditSnapshot,
    },
}

impl UndoAction {
    pub fn snap_geom(g: Geometry) -> GeomSnapshot { GeomSnapshot::Geom(g) }
    pub fn snap_nurbs(ns: NurbsSurface) -> GeomSnapshot { GeomSnapshot::Nurbs(ns) }
}

/// Insert `ns` into the scene's nurbssurfaces list, replacing any existing entry
/// with the same guid instead of blindly pushing. The GPU side is already deduped
/// (add_nurbssurface removes first), but the CPU Vec is the tree's source of truth,
/// so an un-guarded push would show a cone/torus twice in the tree after a stray
/// double-add. Keeps the Vec invariant: at most one entry per guid.
pub(crate) fn replace_or_push_nurbs(list: &mut Vec<NurbsSurface>, ns: NurbsSurface) {
    let guid = ns.guid().to_string();
    list.retain(|n| n.guid() != guid);
    list.push(ns);
}

/// Same invariant for the nurbscurves list: at most one entry per guid.
pub(crate) fn replace_or_push_nurbscurve(list: &mut Vec<NurbsCurve>, nc: NurbsCurve) {
    let guid = nc.guid().to_string();
    list.retain(|n| n.guid() != guid);
    list.push(nc);
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
