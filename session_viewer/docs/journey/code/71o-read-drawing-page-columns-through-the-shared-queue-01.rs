    Some((start, bytes))
}
#[cfg(target_arch = "wasm32")]
impl Source {
    pub async fn from_probe(
