#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode { Append, Replace }

pub fn choose(event: &web_sys::Event, request: Rc<RefCell<crate::read_gate::ReadGate>>, mode: Mode) {
