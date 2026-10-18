#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ticket { epoch: u64, request: u64 }

#[derive(Default)]
pub struct Gate { epoch: u64, request: u64, latest: Option<Ticket>, closed: bool }

impl Gate {
    // A ticket names one request in one scene/view epoch; both must still match before its asynchronous result can change selection.
    pub fn issue(&mut self) -> Result<Ticket, &'static str> {
        if self.closed { return Err("Picking owner is closed"); }
        let Some(request) = self.request.checked_add(1) else { self.close(); return Err("Pick requests exhausted"); };
        self.request = request;
        let ticket = Ticket { epoch: self.epoch, request }; self.latest = Some(ticket); Ok(ticket)
    }

    pub fn invalidate(&mut self) {
        self.latest = None;
        match self.epoch.checked_add(1) { Some(epoch) => self.epoch = epoch, None => self.close() }
    }

    pub fn accept(&mut self, ticket: Ticket) -> bool {
        if self.closed || self.latest != Some(ticket) { return false; }
        self.latest = None; true
    }

    pub fn close(&mut self) { self.closed = true; self.latest = None; }
}

#[cfg(test)]
#[path = "pick_gate_tests.rs"]
mod tests;
