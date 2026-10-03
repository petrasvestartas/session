    pub fn cancel(&mut self) -> bool { self.pending.take().is_some() }
}

impl ReloadJob<()> {
    pub fn begin(&mut self, keys: Vec<ReloadKey>) -> Result<Request, &'static str> {
        self.begin_with(keys, ())
    }
    pub fn finish(&mut self, ticket: u64) -> Option<Vec<ReloadKey>> {
        self.finish_with(ticket).map(|(keys, ())| keys)
    }
}
