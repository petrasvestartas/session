pub enum Mode { Append, Replace }

pub type Delivery = Rc<RefCell<Option<web_sys::File>>>;
