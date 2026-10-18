#[derive(Clone)]
pub struct Object {
    pub original:Option<session_rust::Geometry>,
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
