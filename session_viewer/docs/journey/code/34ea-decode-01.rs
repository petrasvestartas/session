use crate::diagnostic::Report;

pub const MAX_BYTES: usize = 1024 * 1024;

pub fn decode(text: &str) -> Option<Report> {
    if text.len() > MAX_BYTES { return None; }
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let fields = ["version", "tab", "started", "lastSeen", "outcome", "page", "browser", "secureContext",
        "webgpu", "viewport", "canvas", "devicePixelRatio", "events", "failure"];
    if value.as_object()?.keys().any(|key| !fields.contains(&key.as_str())) { return None; }
    let report: Report = serde_json::from_value(value).ok()?;
    report.valid().then_some(report)
}
