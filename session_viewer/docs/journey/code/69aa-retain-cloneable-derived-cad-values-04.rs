use session_rust::Xform;

#[derive(Clone)]
pub struct Boundary {pub edge:usize,pub points:Vec<[f64;3]>}
#[derive(Clone)]
pub struct Prepared {pub source:Rc<session_rust::BRep>,pub edges:Vec<Boundary>,pub samples:Vec<crate::cad_face::Sample>,pub face_count:usize,pub edge_ids:Vec<usize>}

