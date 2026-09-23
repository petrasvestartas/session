use session_rust::brep::BRepOrientation;

/// A BRep edge is shared: each of the two faces meeting there uses the same edge, once from each side.
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
