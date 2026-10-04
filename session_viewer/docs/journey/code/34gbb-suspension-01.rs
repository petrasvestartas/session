#[derive(Clone, Copy)]
pub enum Reason { Hidden, Frozen, Cached }

#[derive(Default)]
pub struct Suspension {
    hidden: bool,
    frozen: bool,
    cached: bool,
    closed: bool,
}

impl Suspension {
    pub fn new(hidden: bool) -> Self { Self { hidden, ..Self::default() } }

    // Each browser event clears only its own reason; scheduling resumes after every remaining reason has cleared.
    pub fn change(&mut self, reason: Reason, paused: bool) -> bool {
        match reason {
            Reason::Hidden => self.hidden = paused,
            Reason::Frozen => self.frozen = paused,
            Reason::Cached => self.cached = paused,
        }
        self.permits()
    }

    pub fn permits(&self) -> bool { !self.hidden && !self.frozen && !self.cached && !self.closed }

    pub fn close(&mut self) { self.closed = true; }
}
