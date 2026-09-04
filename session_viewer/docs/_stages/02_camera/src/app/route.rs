//! ONE query parser (`query`) serves every knob.

/// The `?name=` value of this page's query string, percent-decoded.
pub fn query(name: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    let raw = search.strip_prefix('?')?;
    let prefix = format!("{name}=");
    for pair in raw.split('&') {
        if let Some(v) = pair.strip_prefix(prefix.as_str()) {
            return js_sys::decode_uri_component(v).ok()?.as_string();
        }
        if pair == name {
            return Some(String::new());
        }
    }
    None
}
