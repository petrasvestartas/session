pub fn download(bytes: &[u8]) -> Result<(), JsValue> { download_named(bytes, "viewer.session") }

pub fn download_named(bytes: &[u8], name: &str) -> Result<(), JsValue> {
    let window