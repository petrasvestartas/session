pub type Shared = std::rc::Rc<std::cell::RefCell<crate::editor::Editor>>;

pub fn new() -> Shared { std::rc::Rc::new(std::cell::RefCell::new(Default::default())) }
