    fn finish(&mut self, ticket: u64, failed: bool) -> Option<(Vec<ReloadKey>, Option<crate::edit_intent::Intent>)> {
        let keys = self.job.finish_with(ticket)?;