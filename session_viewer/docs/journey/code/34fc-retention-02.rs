        let now = js_sys::Date::now(); let mut old = Vec::new();
        for key in keys.iter().filter(|key| *key != &self.key) {
            let Ok(Some(text)) = storage.get_item(key) else { return false; };
            if let Some(time) = crate::report_store::retention_time(&text, now, js_sys::Date::parse) {
                old.push((time, key.clone()));
            }
        }
        old.sort_by(|a, b| b.0.total_cmp(&a.0));
        let keep: Vec<_> = old.iter().take(2).map(|(_, key)| key).collect();