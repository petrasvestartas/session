
pub enum Action {
    ElementBegin{kind:crate::element_input::Kind,text:String},ElementText(String),ElementPick(Option<ObjectId>),ElementAttributes(Option<bool>),ElementInteractions(Option<bool>),
    Arctic(Option<bool>),
    ViewOutline(Option<bool>),
