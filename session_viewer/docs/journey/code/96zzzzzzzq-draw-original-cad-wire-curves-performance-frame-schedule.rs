pub struct Schedule {
    dirty: bool,
    allowed: bool,
    closed: bool,
    next: u64,
    pending: Option<u64>,
    pub flight: Option<u64>,
}

impl Schedule {
    pub fn after_submit(allowed: bool) -> Self {
        Self { dirty: false, allowed, closed: false, next: 0, pending: None, flight: Some(0) }
    }

    fn schedule(&mut self) -> Option<u64> {
        if self.closed || !self.allowed || !self.dirty || self.pending.is_some() || self.flight.is_some() { return None; }
        let Some(ticket) = self.next.checked_add(1) else { self.close(); return None; };
        self.next = ticket;
        self.pending = Some(ticket);
        Some(ticket)
    }

    pub fn request(&mut self) -> Option<u64> {
        if self.closed { return None; }
        self.dirty = true;
        self.schedule()
    }

    pub fn begin(&mut self, ticket: u64) -> bool {
        if self.closed || !self.allowed || self.pending != Some(ticket) { return false; }
        self.pending = None;
        self.dirty = false;
        self.flight = Some(ticket);
        true
    }

    pub fn complete(&mut self, ticket: u64) -> Option<u64> {
        if self.flight != Some(ticket) { return None; }
        self.flight = None;
        self.schedule()
    }

    pub fn allow(&mut self, allowed: bool) -> Option<u64> {
        self.allowed = allowed;
        if !allowed { self.pending = None; }
        self.schedule()
    }

    pub fn close(&mut self) {
        self.closed = true;
        self.dirty = false;
        self.pending = None;
        self.flight = None;
    }
}

#[cfg(test)]
#[path="frame_schedule_tests.rs"]
mod tests;
