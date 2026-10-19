pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Element Beam Variable",||Action::ElementBegin{kind:crate::element_input::Kind::Variable,text:String::new()}),
    ("Element Beam",||Action::ElementBegin{kind:crate::element_input::Kind::Beam,text:String::new()}),
    ("Element Column",||Action::ElementBegin{kind:crate::element_input::Kind::Column,text:String::new()}),
    ("Element Plate",||Action::ElementBegin{kind:crate::element_input::Kind::Plate,text:String::new()}),
    ("Element Support",||Action::ElementBegin{kind:crate::element_input::Kind::Support,text:String::new()}),
    ("Element Block",||Action::ElementBegin{kind:crate::element_input::Kind::Block,text:String::new()}),
    ("Element Attributes",||Action::ElementAttributes(None)),
    ("Element Interactions",||Action::ElementInteractions(None)),
    ("View Outline",||Action::ViewOutline(None)),
    ("Arctic",||Action::Arctic(None)),
