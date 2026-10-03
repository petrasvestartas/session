    pub fn write(&self, report: &Report) -> bool {
        if !report.valid() || report.tab != self.tab { return false; }
        let Some(storage) = &self.storage else { return false; };
        let Ok(length) = storage.length() else { return false; };
        if length > 256 { return false; }
        let keys: Result<Vec<_>, _> = (0..length).map(|i| storage.key(i)).collect();
        let Ok(keys) = keys else { return false; };
        let keys: Vec<_> = keys.into_iter().flatten().filter(|key| key.starts_with(PREFIX)).collect();
        if keys.len() > 32 { return false; }
        let mut old = self.reports(); old.retain(|(key, _)| key != &self.key);
        old.retain(|(_, report)| js_sys::Date::parse(&report.last_seen).is_finite());
        old.sort_by(|a, b| js_sys::Date::parse(&b.1.last_seen).total_cmp(&js_sys::Date::parse(&a.1.last_seen)));
        let keep: Vec<_> = old.iter().take(2).map(|(key, _)| key).collect();
        let Ok(text) = serde_json::to_string(report) else { return false; };
        if text.len() > crate::report_store::MAX_BYTES || storage.set_item(&self.key, &text).is_err() { return false; }
        let mut success = true;
        for key in keys {
            if key != self.key && !keep.contains(&&key) { success &= storage.remove_item(&key).is_ok(); }
        }
        success
    }

    pub fn previous(&self) -> Option<Report> {