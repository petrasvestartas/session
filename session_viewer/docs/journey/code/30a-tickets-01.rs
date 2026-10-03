#[derive(Default)]
pub struct ReadGate {
    issued: u64,
    pending: Option<u64>,
}

impl ReadGate {
    pub fn begin(&mut self) -> Result<u64, &'static str> {
        self.pending = None;
        let ticket = self.issued.checked_add(1).ok_or("Read tickets exhausted; reload the viewer")?;
        self.issued = ticket;
        self.pending = Some(ticket);
        Ok(ticket)
    }

    pub fn pending(&self) -> Option<u64> {
        self.pending
    }

    pub fn finish(&mut self, ticket: u64) -> bool {
        if self.pending != Some(ticket) { return false; }
        self.pending = None;
        true
    }

    pub fn cancel(&mut self) {
        self.pending = None;
    }
}

#[cfg(test)]
mod tests {
    use super::ReadGate;

    #[test]
    fn stale_and_duplicate_completions_cannot_consume_current_work() {
        let mut gate = ReadGate::default();
        let first = gate.begin().unwrap();
        let second = gate.begin().unwrap();
        assert!(second > first);
        assert!(!gate.finish(first));
        assert_eq!(gate.pending(), Some(second));
        assert!(gate.finish(second));
        assert_eq!(gate.pending(), None);
        assert!(!gate.finish(second));
    }

    #[test]
    fn cancellation_revokes_the_pending_ticket() {
        let mut gate = ReadGate::default();
        let ticket = gate.begin().unwrap();
        gate.cancel();
        assert!(!gate.finish(ticket));
        assert!(gate.begin().unwrap() > ticket);
    }

    #[test]
    fn exhausted_ids_do_not_wrap_or_leave_old_work_pending() {
        let mut gate = ReadGate { issued: u64::MAX, pending: Some(u64::MAX) };
        assert!(gate.begin().is_err());
        assert_eq!(gate.pending(), None);
    }
}
