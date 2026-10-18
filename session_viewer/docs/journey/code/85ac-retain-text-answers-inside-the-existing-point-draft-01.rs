
// A draft owns unfinished points and a cursor, so preview motion cannot become saved geometry or an Undo transaction.
pub struct Draft {pub text:Option<crate::text_draft::Draft>, pub tool: Box<dyn Tool>, pub points: Vec<[f64; 3]>, pub cursor: Option<[f64; 3]>, pub snapped: Option<crate::snap::Kind>, pub frame: crate::cplane::CPlane, pub plane_points: Option<Vec<[f64;3]>>, pub primitive:Option<crate::primitive_draft::Draft>, pub transform:Option<crate::transform_draft::Draft>,pub gather:Option<crate::gather_draft::Draft>,pub project:Option<crate::project_draft::Draft>,pub extend:Option<crate::extend_draft::Draft> }

impl Draft {
    pub fn new(shape: Shape) -> Self { Self { text:None,project:None,extend:None,gather:None,tool: Box::new(shape), points: Vec::new(), cursor: None, snapped: None, frame: crate::cplane::CPlane::Xy, plane_points: None, primitive:if let Shape::Primitive(shape)=shape {Some(crate::primitive_draft::Draft::new(shape,crate::cplane::CPlane::Xy,0).expect("Registered primitive option"))}else{None},transform:if let Shape::Transform(kind)=shape {Some(crate::transform_draft::Draft::new(kind,crate::cplane::CPlane::Xy))}else{None} } }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none(){if let Some(text)=&self.text{return text.prompt()}}
        if self.plane_points.is_none(){if let Some(p)=&self.project{return p.prompt()}if let Some(e)=&self.extend{return e.prompt()}}
        if self.plane_points.is_none(){if let Some(gather)=&self.gather{return gather.prompt()}}
