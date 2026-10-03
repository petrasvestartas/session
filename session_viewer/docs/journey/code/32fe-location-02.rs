pub struct ReloadUrl(String);

impl ReloadUrl {
    pub fn new(value: String) -> Self { Self(value) }

    pub fn value(&self) -> &str { &self.0 }

    #[cfg(target_arch = "wasm32")]
    pub fn from_file(file: &web_sys::File) -> Result<Self, wasm_bindgen::JsValue> {
        web_sys::Url::create_object_url_with_blob(file).map(Self)
    }
}

impl Drop for ReloadUrl {
    fn drop(&mut self) {
        #[cfg(target_arch = "wasm32")]
        let _ = web_sys::Url::revoke_object_url(&self.0);
    }
}
