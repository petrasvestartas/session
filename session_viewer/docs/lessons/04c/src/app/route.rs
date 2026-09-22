/// The first URL query value for `key`.
pub fn query(key: &str) -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;

    for pair in search.trim_start_matches('?').split('&') {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));

        if name == key {
            return Some(js_sys::decode_uri_component(value).ok()?.into());
        }
    }

    None
}
