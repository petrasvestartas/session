pub use viewer_core::decode::{ByteSource, session_from_body_with};

#[cfg(not(target_arch = "wasm32"))]
pub use viewer_core::decode::{Body, session_from_body, session_from_bytes};

#[cfg(target_arch = "wasm32")]
pub struct JsBytes(pub js_sys::Uint8Array);

#[cfg(target_arch = "wasm32")]
impl ByteSource for JsBytes {
    fn len_bytes(&self) -> u64 {
        u64::from(self.0.length())
    }

    fn copy_range(&self, from: u64, to: u64, dst: &mut [u8]) {
        self.0.subarray(from as u32, to as u32).copy_to(dst);
    }

    fn to_vec_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// A session file; a browser body stays outside linear memory and is read a window at a time.
#[cfg(target_arch = "wasm32")]
pub enum Body {
    Bytes(Vec<u8>),
    Js(js_sys::Uint8Array),
}

#[cfg(target_arch = "wasm32")]
impl Body {
    /// Bytes in the file.
    pub fn size(&self) -> u64 {
        match self {
            Body::Bytes(bytes) => bytes.len() as u64,
            Body::Js(array) => u64::from(array.length()),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn session_from_bytes(
    url: &str,
    bytes: Vec<u8>,
) -> Result<session_rust::Session, String> {
    viewer_core::decode::session_from_bytes_yielding(url, bytes, super::yielding::next_tick).await
}

#[cfg(target_arch = "wasm32")]
pub async fn session_from_body(
    url: &str,
    body: Body,
    vertices: bool,
) -> Result<session_rust::Session, String> {
    let core_body = match body {
        Body::Bytes(bytes) => viewer_core::decode::Body::Bytes(bytes),
        Body::Js(array) => viewer_core::decode::Body::Js(JsBytes(array)),
    };
    viewer_core::decode::session_from_body_with(
        url,
        core_body,
        vertices,
        super::yielding::next_tick,
    )
    .await
}
