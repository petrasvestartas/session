use std::{cell::Cell, rc::Rc};

#[derive(Clone)]
pub struct Startup(Rc<Cell<bool>>);

impl Default for Startup {
    fn default() -> Self { Self(Rc::new(Cell::new(true))) }
}

impl Startup {
    // Cloned tickets share one revocation flag, so a delayed result cannot regain permission after the page closes.
    pub fn permits(&self) -> bool { self.0.get() }

    pub fn revoke(&self) { self.0.set(false); }
}
