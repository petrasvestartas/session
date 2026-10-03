    pub fn finish_with(&mut self, ticket: u64) -> Option<(Vec<ReloadKey>, P)> {
        if self.pending() != Some(ticket) { return None; }
        self.pending.take().map(|pending| (pending.keys, pending.payload))
    }
