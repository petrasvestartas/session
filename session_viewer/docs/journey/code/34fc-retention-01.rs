pub const MAX_BYTES: usize = 1024 * 1024;

pub fn retention_time(text: &str, now: f64, parse: impl Fn(&str) -> f64) -> Option<f64> {
    if text.len() > MAX_BYTES || !now.is_finite() || now < 0.0 { return None; }
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    if value.get("version")?.as_u64()? == 0 { return None; }
    let seen = value.get("lastSeen")?.as_str()?;
    if seen.is_empty() || seen.chars().count() > 64 { return None; }
    let time = parse(seen);
    (time.is_finite() && time <= now).then_some(time)
}
