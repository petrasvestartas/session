#[cfg(any(target_arch = "wasm32", test))]
#[derive(Default)]
pub(crate) struct MetadataWindow {
    at: u64,        // file position of `bytes[0]`
    bytes: Vec<u8>, // the cached bytes
