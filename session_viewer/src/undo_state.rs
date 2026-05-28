use session_rust::session::Geometry;
use session_rust::NurbsSurface;

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
    /// Undo = apply before_model. Redo = apply after_model.
    Transform { objects: Vec<(String, [[f32; 4]; 4], [[f32; 4]; 4])> },
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
