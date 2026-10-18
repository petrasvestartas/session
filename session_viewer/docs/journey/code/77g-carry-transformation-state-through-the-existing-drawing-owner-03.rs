
// A draft owns unfinished points and a cursor, so preview motion cannot become saved geometry or an Undo transaction.
pub struct Draft { pub tool: Box<dyn Tool>, pub points: Vec<[f64; 3]>, pub cursor: Option<[f64; 3]>, pub snapped: Option<crate::snap::Kind>, pub frame: crate::cplane::CPlane, pub plane_points: Option<Vec<[f64;3]>>, pub primitive:Option<crate::primitive_draft::Draft>, pub transform:Option<crate::transform_draft::Draft> }

impl Draft {
    pub fn new(shape: Shape) -> Self { Self { tool: Box::new(shape), points: Vec::new(), cursor: None, snapped: None, frame: crate::cplane::CPlane::Xy, plane_points: None, primitive:if let Shape::Primitive(shape)=shape {Some(crate::primitive_draft::Draft::new(shape,crate::cplane::CPlane::Xy,0).expect("Registered primitive option"))}else{None},transform:if let Shape::Transform(kind)=shape {Some(crate::transform_draft::Draft::new(kind,crate::cplane::CPlane::Xy))}else{None} } }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {if let Some(transform)=&self.transform {return transform.prompt();}}
        if self.plane_points.is_none() {if let Some(primitive)=&self.primitive {return primitive.prompt();}}
        let mut prompt=if let Some(points)=&self.plane_points {format!("CPlane 3 Point · point {} of 3 · Esc cancels",points.len()+1)}
