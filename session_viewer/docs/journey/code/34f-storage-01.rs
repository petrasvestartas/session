use crate::diagnostic::Report;
use web_sys::Storage;

pub const PREFIX: &str = "viewer-journey-report:";

pub struct Store {
    pub tab: String,
    key: String,
    storage: Option<Storage>,
}

impl Store {
    pub fn open() -> Self {
        let window = web_sys::window();
        let session = window.as_ref().and_then(|w| w.session_storage().ok().flatten());
        let tab = session.as_ref().and_then(|s| s.get_item(PREFIX).ok().flatten())
            .filter(|tab| !tab.is_empty() && tab.chars().count() <= 128)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if let Some(session) = session { let _ = session.set_item(PREFIX, &tab); }
        Self { tab, key: format!("{PREFIX}{}", uuid::Uuid::new_v4()),
            storage: window.and_then(|w| w.local_storage().ok().flatten()) }
    }

    pub fn reports(&self) -> Vec<(String, Report)> {
        let Some(storage) = &self.storage else { return Vec::new(); };
        (0..storage.length().unwrap_or(0).min(256))
            .filter_map(|i| storage.key(i).ok().flatten()).filter(|key| key.starts_with(PREFIX)).take(32)
            .filter_map(|key| {
                let text = storage.get_item(&key).ok().flatten()?;
                Some((key, crate::report_store::decode(&text)?))
            }).collect()
    }

    pub fn previous(&self) -> Option<Report> {
        let now = js_sys::Date::now();
        self.reports().into_iter().filter_map(|(_, report)| {
            let score = crate::report_recency::eligible(&report, now, &self.tab, js_sys::Date::parse)?;
            Some((score, report))
        }).max_by(|a, b| a.0.total_cmp(&b.0)).map(|(_, report)| report)
    }
}

#[cfg(debug_assertions)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn previous_storage_probe() -> String {
    let store = Store::open();
    serde_json::json!({"tab": store.tab, "key": store.key, "previous": store.previous()}).to_string()
}
