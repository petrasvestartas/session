use session_rust::brep::BRepOrientation;

/// One use of an edge by a face.
pub struct EdgeUse {
    pub edge: usize,                  // edge index
    pub face: usize,                  // face index
    pub orientation: BRepOrientation, // which way the face runs it
}

/// The mesh vertices one BRep edge runs along.
pub struct EdgeChain {
    pub edge: usize,          // BRep edge index
    pub face: usize,          // face mesh the keys belong to
    pub keys: Vec<usize>,     // vertex keys along the edge
    pub other: Option<usize>, // the face on the other side
}
