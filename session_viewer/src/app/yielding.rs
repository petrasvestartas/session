// --8<-- [start:browser-yield]
use wasm_bindgen_futures::JsFuture;

/// Call `resolve` after `ms` milliseconds.
fn schedule(resolve: js_sys::Function, ms: i32) {
    if let Some(w) = web_sys::window() {
        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
    }
}

/// Wait `ms` milliseconds.
pub async fn sleep_ms(ms: i32) {
    let p = js_sys::Promise::new(&mut |resolve, _| schedule(resolve, ms));
    let _ = JsFuture::from(p).await;
}

/// Let the browser paint before continuing.
pub async fn next_tick() {
    sleep_ms(0).await;
}
// --8<-- [end:browser-yield]
