// --8<-- [start:002-knobs]
/// One setting's text: `?query=` in the browser, `ENV` natively.
pub fn knob(env: &str, query: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = env;
        self::query(query)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = query;
        std::env::var(env).ok()
    }
}

/// The `?name=` value of the page URL.
#[cfg(target_arch = "wasm32")]
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
// --8<-- [end:002-knobs]
