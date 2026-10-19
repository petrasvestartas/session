type Slot=RefCell<Option<Rc<RefCell<State>>>>;
pub struct Publication(Rc<Slot>);