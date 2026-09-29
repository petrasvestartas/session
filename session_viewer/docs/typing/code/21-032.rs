
/// Fetch and decode a released document again; the answer comes back as `Msg::Hydrated`.
pub fn spawn_hydrate(doc: usize, url: String, token: u64) {
    wasm_bindgen_futures::spawn_local(async move {
        let t0 = now_ms();
        let session = match fetch_buffer(&url).await {
            Ok(array) => session_from_body(&url, Body::Js(array), true).await,
            Err(error) => Err(error),
        };
        post(Msg::Hydrated(Box::new(Hydrated {
            doc,
            token,
            session,
            ms: now_ms() - t0,
        })));
    });
}

use super::scene::Hydrated;
